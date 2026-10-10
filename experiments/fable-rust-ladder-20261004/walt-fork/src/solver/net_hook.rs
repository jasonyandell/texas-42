//! FORK-ONLY additions (not in pinned walt): a logger for every level-0
//! modeled-mind call and an optional net that answers those calls instead of
//! the 8-world dice bundle. Installed through process-global state so the
//! unchanged recursion needs no new parameters. Everything else in this crate
//! is byte-identical to the pinned source except the two hook lines in `pi`.
use super::Key;
use crate::rules::Seat;
use std::collections::HashSet;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, RwLock};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Call {
    pub seat: u8,
    pub hand: u32,
    pub played: u32,
    pub leader: u8,
    pub plays: Vec<u8>,
    pub banked_t1: u8,
    pub banked_t0: u8,
    pub legal: u32,
    pub voids: Option<[u32; 4]>,
}

pub trait NetPolicy: Send + Sync {
    /// Must return a tile inside `legal`; the caller does not re-check.
    fn choose(&self, key: &Key, seat: Seat, hand: u32, legal: u32) -> u8;
}

static NET: RwLock<Option<Arc<dyn NetPolicy>>> = RwLock::new(None);
static LOG: Mutex<Option<(Vec<Call>, HashSet<Call>)>> = Mutex::new(None);
static CALLS: AtomicU64 = AtomicU64::new(0);
static NET_CALLS: AtomicU64 = AtomicU64::new(0);

pub fn install_net(net: Option<Arc<dyn NetPolicy>>) {
    *NET.write().expect("net lock") = net;
}
pub fn net_installed() -> bool {
    NET.read().expect("net lock").is_some()
}
pub fn start_log() {
    *LOG.lock().expect("log lock") = Some((Vec::new(), HashSet::new()));
}
/// Unique calls in first-seen order, plus the total (including repeats).
pub fn take_log() -> (Vec<Call>, u64) {
    let unique = LOG.lock().expect("log lock").take().map(|(v, _)| v).unwrap_or_default();
    (unique, CALLS.swap(0, Ordering::Relaxed))
}
pub fn net_calls() -> u64 {
    NET_CALLS.swap(0, Ordering::Relaxed)
}

/// Called at the top of `Solver::pi` for level 0, before any cache or fast path.
pub(crate) fn on_pi0(key: &Key, seat: Seat, hand: u32, legal: u32) -> Option<u8> {
    if let Some((list, seen)) = LOG.lock().expect("log lock").as_mut() {
        CALLS.fetch_add(1, Ordering::Relaxed);
        let call = Call { seat: seat.index() as u8, hand, played: key.played, leader: key.leader, plays: key.plays.clone(), banked_t1: key.banked_t1, banked_t0: key.banked_t0, legal, voids: key.voids };
        if seen.insert(call.clone()) {
            list.push(call);
        }
    }
    let guard = NET.read().expect("net lock");
    let net = guard.as_ref()?;
    NET_CALLS.fetch_add(1, Ordering::Relaxed);
    Some(net.choose(key, seat, hand, legal))
}
