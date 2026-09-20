//! Optional, process-local QoS setup for the native timing experiment.
//!
//! Call `initialize` from the benchmark main thread after the first game's
//! `full_game_start` timestamp and before any Rayon use. It creates the global
//! Rayon pool once, preserving Rayon's `RAYON_NUM_THREADS` behavior. No other
//! process, thread pool, or system scheduling setting is modified.

#[cfg(not(target_os = "macos"))]
compile_error!("mac-qos is available only on macOS");

use std::ffi::c_int;
use std::sync::{Arc, Mutex};

// Verified in the installed macOS SDK's <sys/qos.h> and <pthread/qos.h>:
// qos_class_t is an unsigned-int enum, USER_INTERACTIVE = 0x21, and this
// function returns zero or an errno value. Relative priority zero requests
// the top of the class's permitted range.
const QOS_CLASS_USER_INTERACTIVE: u32 = 0x21;
const RELATIVE_PRIORITY: c_int = 0;

unsafe extern "C" {
    fn pthread_set_qos_class_self_np(qos_class: u32, relative_priority: c_int) -> c_int;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MacQosDiagnostic {
    pub qos_class: u32,
    pub relative_priority: i32,
    pub main_thread_ok: bool,
    pub rayon_workers: usize,
    pub workers_configured: usize,
}

fn set_current_thread() -> Result<(), c_int> {
    // SAFETY: The SDK function takes only two scalar values and changes the
    // calling thread's own QoS. Both values are valid per the installed SDK.
    let code =
        unsafe { pthread_set_qos_class_self_np(QOS_CLASS_USER_INTERACTIVE, RELATIVE_PRIORITY) };
    if code == 0 {
        Ok(())
    } else {
        Err(code)
    }
}

/// Configure this calling thread and each worker in the runner's global pool.
/// Any failed pthread call or preexisting Rayon pool is returned as an error.
pub fn initialize() -> Result<MacQosDiagnostic, String> {
    set_current_thread()
        .map_err(|errno| format!("main thread QoS setup failed with errno {errno}"))?;

    let started: Arc<Mutex<Vec<(usize, c_int)>>> = Arc::new(Mutex::new(Vec::new()));
    let worker_results = Arc::clone(&started);
    rayon::ThreadPoolBuilder::new()
        .start_handler(move |index| {
            let code = set_current_thread().err().unwrap_or(0);
            worker_results
                .lock()
                .expect("QoS results mutex poisoned")
                .push((index, code));
        })
        .build_global()
        .map_err(|error| format!("Rayon global pool QoS setup failed: {error}"))?;

    // `build_global` waits for workers to be primed, but Rayon invokes its
    // start handler immediately after priming. Broadcast waits until every
    // worker has passed that handler before we inspect the results.
    let mut reached = rayon::broadcast(|context| context.index());
    let workers = rayon::current_num_threads();
    reached.sort_unstable();
    if reached != (0..workers).collect::<Vec<_>>() {
        return Err(format!(
            "Rayon QoS verification reached {:?} of {workers} workers",
            reached
        ));
    }

    let mut outcomes = started
        .lock()
        .map_err(|_| "QoS results mutex poisoned")?
        .clone();
    outcomes.sort_unstable_by_key(|(index, _)| *index);
    if outcomes.len() != workers
        || outcomes
            .iter()
            .enumerate()
            .any(|(expected, (actual, _))| *actual != expected)
    {
        return Err(format!(
            "Rayon QoS start handler reported {:?} of {workers} workers",
            outcomes
        ));
    }
    if let Some((index, errno)) = outcomes.into_iter().find(|(_, code)| *code != 0) {
        return Err(format!(
            "Rayon worker {index} QoS setup failed with errno {errno}"
        ));
    }
    Ok(MacQosDiagnostic {
        qos_class: QOS_CLASS_USER_INTERACTIVE,
        relative_priority: RELATIVE_PRIORITY,
        main_thread_ok: true,
        rayon_workers: workers,
        workers_configured: workers,
    })
}
