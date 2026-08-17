//! Command-line boundary for the supervised freeze-56 M2 Metal gate.

#![forbid(unsafe_code)]

use core::convert::Infallible;
use std::env;
use std::ffi::OsString;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};

use walt_gpu_ref::m2_receipt::{
    sha256, Digest, FailureCode, FailurePhase, FailureReceipt, SmokeReport, SuccessReceipt,
    FREEZE56_DESCRIPTOR_SHA256, ZERO_DIGEST,
};
use walt_m2_runner::child::{run_child, ChildProfile};
use walt_m2_runner::observation::{
    observe_device, observe_m2_manifest_build_identity, observe_toolchain,
    render_host_tool_device_descriptor, verify_checked_descriptor, verify_compiled_m2_manifest,
    verify_runner_entry_environment, verify_success_receipt_observations,
};
use walt_m2_runner::protocol::{supervise_child, RunProfile, SupervisedOutcome, SupervisedSuccess};
use walt_metal::{CommandEvent, CommandTerminal, MetalRuntime};

const UNAVAILABLE_ORDINAL: u32 = u32::MAX;
const UNAVAILABLE_NATIVE_STATUS: u32 = u32::MAX;
const UNAVAILABLE_CHILD_EXIT: i32 = i32::MIN;

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(error) => {
            eprintln!("walt-m2-runner: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<ExitCode, String> {
    let mut arguments = env::args_os();
    let _program = arguments.next();
    let command = next_utf8(&mut arguments, "command")?;
    match command.as_str() {
        "descriptor-render" => {
            let root = next_path(&mut arguments, "repository root")?;
            require_end(&mut arguments)?;
            let bytes = observe_descriptor(&root, false)?;
            let mut stdout = io::stdout().lock();
            stdout
                .write_all(&bytes)
                .and_then(|()| stdout.flush())
                .map_err(|error| format!("write descriptor: {error}"))?;
            Ok(ExitCode::SUCCESS)
        }
        "descriptor-verify" => {
            let root = next_path(&mut arguments, "repository root")?;
            require_end(&mut arguments)?;
            let _ = observe_descriptor(&root, true)?;
            Ok(ExitCode::SUCCESS)
        }
        "child-smoke" | "child-official" => {
            let root = next_path(&mut arguments, "repository root")?;
            require_end(&mut arguments)?;
            let profile = if command == "child-smoke" {
                ChildProfile::Smoke
            } else {
                ChildProfile::Official
            };
            let mut stdout = io::stdout().lock();
            let code = run_child(&root, &mut stdout, profile);
            Ok(ExitCode::from(u8::try_from(code).map_err(|_| {
                "child exit code does not fit u8".to_owned()
            })?))
        }
        "run-smoke" | "run-official" => {
            let root = next_path(&mut arguments, "repository root")?;
            let output = next_path(&mut arguments, "output receipt")?;
            require_end(&mut arguments)?;
            let profile = if command == "run-smoke" {
                RunProfile::Smoke
            } else {
                RunProfile::Official
            };
            run_parent(&root, &output, profile)
        }
        "validate-smoke" => {
            let path = next_path(&mut arguments, "smoke report")?;
            require_end(&mut arguments)?;
            let bytes = read_regular(&path)?;
            SmokeReport::decode(&bytes).map_err(|error| error.to_string())?;
            Ok(ExitCode::SUCCESS)
        }
        "validate-receipt" => {
            let root = next_path(&mut arguments, "repository root")?;
            let path = next_path(&mut arguments, "M2 receipt")?;
            require_end(&mut arguments)?;
            validate_success_receipt(&root, &path)?;
            Ok(ExitCode::SUCCESS)
        }
        "validate-failure" => {
            let path = next_path(&mut arguments, "M2 failure receipt")?;
            require_end(&mut arguments)?;
            let bytes = read_regular(&path)?;
            FailureReceipt::decode(&bytes).map_err(|error| error.to_string())?;
            Ok(ExitCode::SUCCESS)
        }
        "adjudicate-receipts" => {
            let root = next_path(&mut arguments, "repository root")?;
            let first = next_path(&mut arguments, "first regenerated receipt")?;
            let second = next_path(&mut arguments, "second regenerated receipt")?;
            let committed = next_path(&mut arguments, "committed receipt")?;
            let checksum = next_path(&mut arguments, "committed checksum")?;
            let failure = next_path(&mut arguments, "failure receipt output")?;
            require_end(&mut arguments)?;
            adjudicate_receipts(&root, &first, &second, &committed, &checksum, &failure)
        }
        _ => Err(usage()),
    }
}

fn run_parent(
    repository_root: &Path,
    output: &Path,
    profile: RunProfile,
) -> Result<ExitCode, String> {
    // Absence is an entry condition, not something a parent may manufacture
    // by laundering a dirty invocation into a clean child environment.
    let environment = verify_runner_entry_environment();
    let manifest_identity = observe_m2_manifest_build_identity(repository_root);
    if environment.is_err() {
        return persist_parent_failure(
            output,
            FailurePhase::RustBuild,
            FailureCode::ToolchainMismatch,
            manifest_identity.unwrap_or(ZERO_DIGEST),
        );
    }
    let build_identity = match manifest_identity {
        Ok(identity) => identity,
        Err(_) => {
            return persist_parent_failure(
                output,
                FailurePhase::SourceManifest,
                FailureCode::IdentityMismatch,
                ZERO_DIGEST,
            );
        }
    };
    if !matches!(
        verify_compiled_m2_manifest(repository_root),
        Ok(identity) if identity == build_identity
    ) {
        return persist_parent_failure(
            output,
            FailurePhase::SourceManifest,
            FailureCode::IdentityMismatch,
            build_identity,
        );
    }
    let child_command = match profile {
        RunProfile::Smoke => "child-smoke",
        RunProfile::Official => "child-official",
    };
    let executable = match env::current_exe() {
        Ok(executable) => executable,
        Err(_) => {
            return persist_parent_failure(
                output,
                FailurePhase::RustBuild,
                FailureCode::InternalFailure,
                build_identity,
            );
        }
    };
    let mut command = Command::new(executable);
    command
        .arg(child_command)
        .arg(repository_root)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .env_clear()
        .env("HOME", env::var_os("HOME").unwrap_or_default())
        .env("PATH", "/usr/bin:/bin:/usr/sbin:/sbin")
        .env("LC_ALL", "C")
        .env("TMPDIR", "/tmp");
    let child = match command.spawn() {
        Ok(child) => child,
        Err(_) => {
            return persist_parent_failure(
                output,
                FailurePhase::RustBuild,
                FailureCode::InternalFailure,
                build_identity,
            );
        }
    };
    let outcome = match supervise_child(child, profile, build_identity) {
        Ok(outcome) => outcome,
        Err(_) => {
            return persist_parent_failure(
                output,
                FailurePhase::ChildProtocol,
                FailureCode::InternalFailure,
                build_identity,
            );
        }
    };
    let (bytes, exit) = match outcome {
        SupervisedOutcome::Success(SupervisedSuccess::Smoke(report))
            if profile == RunProfile::Smoke =>
        {
            (report.encode().to_vec(), ExitCode::SUCCESS)
        }
        SupervisedOutcome::Success(SupervisedSuccess::Official(receipt))
            if profile == RunProfile::Official =>
        {
            if verify_success_receipt_observations(repository_root, &receipt).is_err() {
                return persist_parent_failure(
                    output,
                    FailurePhase::ReceiptRender,
                    FailureCode::MalformedOutput,
                    build_identity,
                );
            }
            (
                receipt.encode().map_err(|error| error.to_string())?,
                ExitCode::SUCCESS,
            )
        }
        SupervisedOutcome::Success(_) => {
            return persist_parent_failure(
                output,
                FailurePhase::ChildProtocol,
                FailureCode::ChildProtocolFailure,
                build_identity,
            );
        }
        SupervisedOutcome::Failure(failure) => (failure.encode().to_vec(), ExitCode::FAILURE),
    };
    write_new_artifact(output, &bytes)?;
    Ok(exit)
}

fn observe_descriptor(repository_root: &Path, verify: bool) -> Result<Vec<u8>, String> {
    verify_compiled_m2_manifest(repository_root).map_err(|error| error.to_string())?;
    let environment = verify_runner_entry_environment().map_err(|error| error.to_string())?;
    let toolchain =
        observe_toolchain(&environment, repository_root).map_err(|error| error.to_string())?;
    let mut runtime = MetalRuntime::new().map_err(|error| error.to_string())?;
    let mut events = Vec::with_capacity(2);
    let mut observer = |event| events.push(event);
    let mut on_timeout = |state| -> Infallible {
        eprintln!("descriptor Gate-0 timed out in {state:?}");
        std::process::exit(124)
    };
    runtime
        .run_gate0(&mut observer, &mut on_timeout)
        .map_err(|error| error.to_string())?;
    if events
        != [
            CommandEvent::Committed,
            CommandEvent::Terminal(CommandTerminal::Completed),
        ]
    {
        return Err("descriptor Gate-0 emitted the wrong command events".to_owned());
    }
    let device = observe_device(&environment, repository_root, runtime.device_profile())
        .map_err(|error| error.to_string())?;
    if verify {
        verify_checked_descriptor(repository_root, &toolchain, &device)
            .map_err(|error| error.to_string())?;
    }
    render_host_tool_device_descriptor(&toolchain, &device).map_err(|error| error.to_string())
}

fn validate_success_receipt(repository_root: &Path, path: &Path) -> Result<(), String> {
    let bytes = read_regular(path)?;
    let receipt = SuccessReceipt::decode(&bytes).map_err(|error| error.to_string())?;
    let expected_build = source_build_identity(repository_root)?;
    if receipt.build_identity != expected_build {
        return Err("receipt/source build identity mismatch".to_owned());
    }
    verify_success_receipt_observations(repository_root, &receipt)
        .map_err(|error| error.to_string())?;
    let rerendered = receipt.encode().map_err(|error| error.to_string())?;
    if rerendered != bytes {
        return Err("receipt decode/re-encode bytes changed".to_owned());
    }
    Ok(())
}

fn source_build_identity(repository_root: &Path) -> Result<Digest, String> {
    verify_compiled_m2_manifest(repository_root).map_err(|error| error.to_string())
}

fn adjudicate_receipts(
    repository_root: &Path,
    first: &Path,
    second: &Path,
    committed: &Path,
    checksum: &Path,
    failure_output: &Path,
) -> Result<ExitCode, String> {
    // Keep the same first-observation rule as every official parent entry.
    let environment = verify_runner_entry_environment();
    let manifest_identity = observe_m2_manifest_build_identity(repository_root);
    if environment.is_err() {
        return persist_parent_failure(
            failure_output,
            FailurePhase::RustBuild,
            FailureCode::ToolchainMismatch,
            manifest_identity.unwrap_or(ZERO_DIGEST),
        );
    }
    let build_identity = match manifest_identity {
        Ok(identity) => identity,
        Err(_) => {
            return persist_parent_failure(
                failure_output,
                FailurePhase::SourceManifest,
                FailureCode::IdentityMismatch,
                ZERO_DIGEST,
            );
        }
    };
    if !matches!(
        verify_compiled_m2_manifest(repository_root),
        Ok(identity) if identity == build_identity
    ) {
        return persist_parent_failure(
            failure_output,
            FailurePhase::SourceManifest,
            FailureCode::IdentityMismatch,
            build_identity,
        );
    }

    let first_bytes = match read_canonical_success(repository_root, first, build_identity) {
        Ok(bytes) => bytes,
        Err(()) => {
            return persist_parent_failure(
                failure_output,
                FailurePhase::ReceiptRegeneration,
                FailureCode::MalformedOutput,
                build_identity,
            );
        }
    };
    let second_bytes = match read_canonical_success(repository_root, second, build_identity) {
        Ok(bytes) => bytes,
        Err(()) => {
            return persist_parent_failure(
                failure_output,
                FailurePhase::ReceiptRegeneration,
                FailureCode::MalformedOutput,
                build_identity,
            );
        }
    };
    if first_bytes != second_bytes {
        return persist_parent_failure(
            failure_output,
            FailurePhase::ReceiptRegeneration,
            FailureCode::ReceiptNondeterministic,
            build_identity,
        );
    }

    let committed_bytes = match read_canonical_success(repository_root, committed, build_identity) {
        Ok(bytes) => bytes,
        Err(()) => {
            return persist_parent_failure(
                failure_output,
                FailurePhase::ReceiptRegeneration,
                FailureCode::ReceiptComparandMismatch,
                build_identity,
            );
        }
    };
    let checksum_bytes = match read_regular(checksum) {
        Ok(bytes) => bytes,
        Err(_) => {
            return persist_parent_failure(
                failure_output,
                FailurePhase::ReceiptRegeneration,
                FailureCode::ReceiptComparandMismatch,
                build_identity,
            );
        }
    };
    if first_bytes != committed_bytes
        || checksum_bytes != canonical_receipt_checksum(&committed_bytes)
    {
        return persist_parent_failure(
            failure_output,
            FailurePhase::ReceiptRegeneration,
            FailureCode::ReceiptComparandMismatch,
            build_identity,
        );
    }
    Ok(ExitCode::SUCCESS)
}

fn read_canonical_success(
    repository_root: &Path,
    path: &Path,
    build_identity: Digest,
) -> Result<Vec<u8>, ()> {
    let bytes = read_regular(path).map_err(|_| ())?;
    let receipt = SuccessReceipt::decode(&bytes).map_err(|_| ())?;
    if receipt.build_identity != build_identity
        || verify_success_receipt_observations(repository_root, &receipt).is_err()
        || receipt.encode().map_err(|_| ())? != bytes
    {
        return Err(());
    }
    Ok(bytes)
}

fn canonical_receipt_checksum(receipt: &[u8]) -> Vec<u8> {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    const SUFFIX: &[u8] = b"  m2_metal_parity_v1.bin\n";
    let digest = sha256(receipt);
    let mut bytes = Vec::with_capacity(64 + SUFFIX.len());
    for value in digest {
        bytes.push(HEX[usize::from(value >> 4)]);
        bytes.push(HEX[usize::from(value & 0x0f)]);
    }
    bytes.extend_from_slice(SUFFIX);
    bytes
}

fn persist_parent_failure(
    output: &Path,
    phase: FailurePhase,
    code: FailureCode,
    build_identity: Digest,
) -> Result<ExitCode, String> {
    let receipt = FailureReceipt {
        phase,
        code,
        task_ordinal: UNAVAILABLE_ORDINAL,
        subordinal: UNAVAILABLE_ORDINAL,
        child_exit: UNAVAILABLE_CHILD_EXIT,
        native_status: UNAVAILABLE_NATIVE_STATUS,
        observed_mismatch: 1,
        build_identity,
        freeze56_digest: FREEZE56_DESCRIPTOR_SHA256,
        child_failure_frame_digest: ZERO_DIGEST,
    };
    let bytes = receipt.encode();
    FailureReceipt::decode(&bytes).map_err(|error| error.to_string())?;
    write_new_artifact(output, &bytes)?;
    Ok(ExitCode::FAILURE)
}

fn read_regular(path: &Path) -> Result<Vec<u8>, String> {
    let metadata = fs::symlink_metadata(path)
        .map_err(|error| format!("inspect {}: {error}", path.display()))?;
    if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
        return Err(format!(
            "{} is not a regular non-symlink file",
            path.display()
        ));
    }
    fs::read(path).map_err(|error| format!("read {}: {error}", path.display()))
}

fn write_new_artifact(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let metadata = fs::symlink_metadata(parent)
        .map_err(|error| format!("inspect output directory {}: {error}", parent.display()))?;
    if !metadata.is_dir() || metadata.file_type().is_symlink() {
        return Err(format!(
            "output parent {} is not a regular directory",
            parent.display()
        ));
    }
    match fs::symlink_metadata(path) {
        Ok(_) => return Err(format!("output already exists: {}", path.display())),
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => return Err(format!("inspect output {}: {error}", path.display())),
    }
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| "output filename is not canonical UTF-8".to_owned())?;
    let mut temporary = None;
    for attempt in 0..100u32 {
        let candidate = parent.join(format!(
            ".{file_name}.m2tmp.{}.{}",
            std::process::id(),
            attempt
        ));
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&candidate)
        {
            Ok(file) => {
                temporary = Some((candidate, file));
                break;
            }
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {}
            Err(error) => {
                return Err(format!(
                    "create temporary output {}: {error}",
                    candidate.display()
                ));
            }
        }
    }
    let (temporary_path, mut temporary_file) =
        temporary.ok_or_else(|| "temporary output namespace exhausted".to_owned())?;
    if let Err(error) = temporary_file
        .write_all(bytes)
        .and_then(|()| temporary_file.sync_all())
    {
        let _ = fs::remove_file(&temporary_path);
        return Err(format!("persist temporary output: {error}"));
    }
    drop(temporary_file);
    if let Err(error) = fs::hard_link(&temporary_path, path) {
        let _ = fs::remove_file(&temporary_path);
        return Err(format!("publish output {}: {error}", path.display()));
    }
    fs::remove_file(&temporary_path)
        .map_err(|error| format!("remove temporary output: {error}"))?;
    fs::File::open(parent)
        .and_then(|directory| directory.sync_all())
        .map_err(|error| format!("sync output directory {}: {error}", parent.display()))
}

fn next_utf8(
    arguments: &mut impl Iterator<Item = OsString>,
    label: &'static str,
) -> Result<String, String> {
    arguments
        .next()
        .ok_or_else(usage)?
        .into_string()
        .map_err(|_| format!("{label} is not UTF-8"))
}

fn next_path(
    arguments: &mut impl Iterator<Item = OsString>,
    _label: &'static str,
) -> Result<PathBuf, String> {
    arguments.next().map(PathBuf::from).ok_or_else(usage)
}

fn require_end(arguments: &mut impl Iterator<Item = OsString>) -> Result<(), String> {
    if arguments.next().is_none() {
        Ok(())
    } else {
        Err(usage())
    }
}

fn usage() -> String {
    "usage: walt-m2-runner COMMAND ...\n\
     commands: descriptor-render ROOT | descriptor-verify ROOT | run-smoke ROOT OUTPUT | \
     run-official ROOT OUTPUT | validate-smoke FILE | validate-receipt ROOT FILE | \
     validate-failure FILE | adjudicate-receipts ROOT FIRST SECOND COMMITTED CHECKSUM FAILURE"
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static TEMP_ORDINAL: AtomicU64 = AtomicU64::new(0);

    fn temporary_directory() -> PathBuf {
        let ordinal = TEMP_ORDINAL.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "walt-m2-main-test-{}-{ordinal}",
            std::process::id()
        ));
        fs::create_dir(&path).unwrap();
        path
    }

    #[test]
    fn checksum_line_has_one_exact_lowercase_named_entry() {
        assert_eq!(
            canonical_receipt_checksum(&[]),
            b"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  m2_metal_parity_v1.bin\n"
        );
    }

    #[test]
    fn parent_failure_publication_is_typed_and_create_new() {
        let directory = temporary_directory();
        let output = directory.join("failure.bin");
        let code = persist_parent_failure(
            &output,
            FailurePhase::SourceManifest,
            FailureCode::IdentityMismatch,
            ZERO_DIGEST,
        )
        .unwrap();
        assert_eq!(code, ExitCode::FAILURE);
        let bytes = fs::read(&output).unwrap();
        let failure = FailureReceipt::decode(&bytes).unwrap();
        assert_eq!(failure.phase, FailurePhase::SourceManifest);
        assert_eq!(failure.code, FailureCode::IdentityMismatch);
        assert_eq!(failure.build_identity, ZERO_DIGEST);
        assert!(persist_parent_failure(
            &output,
            FailurePhase::SourceManifest,
            FailureCode::IdentityMismatch,
            ZERO_DIGEST,
        )
        .is_err());
        assert_eq!(fs::read(&output).unwrap(), bytes);
        fs::remove_file(output).unwrap();
        fs::remove_dir(directory).unwrap();
    }
}
