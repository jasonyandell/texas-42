//! Fail-closed observation of the freeze-56 authority, host toolchain, and
//! Metal device.  This module observes evidence; it does not issue a receipt or
//! retain any Metal result.

use std::collections::BTreeMap;
use std::env;
use std::ffi::{OsStr, OsString};
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use walt_gpu_ref::m2_receipt::{
    sha256, ArtifactIdentity, ArtifactTag, AuthoritySection, CodecError, DeviceSection, Digest,
    InvocationKind, InvocationRecord, KernelId, PackageRecord, PipelineRecord, SourceKind,
    SourceRecord, SuccessReceipt, ToolId, ToolRecord, ToolchainSection, CONTRACT_BYTES,
    CONTRACT_SHA256, FREEZE56_DESCRIPTOR, FREEZE56_DESCRIPTOR_SHA256, PARENT_CENSUS_SHA256,
    PARENT_COMMIT_SHA1,
};
use walt_gpu_ref::{GT1_FREEZE_SET_DESCRIPTOR_V1, GT1_FREEZE_SET_SHA256_V1};
use walt_metal::{AllocationHighWater, DeviceProfile, PipelineLimits};

pub const CHECKED_DESCRIPTOR_PATH: &str =
    "walt/walt-metal/toolchain/m2_host_tool_descriptor_v1.txt";
pub const M2_SOURCE_MANIFEST_PATH: &str = "walt/math/gpu_native_trick1_m0_m2_sources_v1.sha256";
const COMPILED_M2_SOURCE_MANIFEST: &[u8] =
    include_bytes!("../../math/gpu_native_trick1_m0_m2_sources_v1.sha256");

const TARGET: &str = "aarch64-apple-darwin";
const RUST_RELEASE: &str = "1.95.0";
const XCODE_VERSION: &str = "26.6";
const XCODE_BUILD: &str = "17F113";
const METAL_COMPONENT_ID: &str = "com.apple.dt.toolchain.Metal.32023.883";
const METAL_COMPONENT_BUILD: &str = "17F109";
const METAL_VERSION: &str = "32023.883";
const SDK_VERSION: &str = "26.5";
const SDK_BUILD: &str = "25F70";
const DEPLOYMENT_TARGET: &str = "26.0";
const XCTRACE_VERSION: &str = "16.0 (17F113)";
const MACOS_VERSION: &str = "26.5.1";
const MACOS_BUILD: &str = "25F80";
const DEVICE_NAME: &str = "Apple M5 Max";
const ARITHMETIC_KERNEL: &str = "u256_parity_v1";
const PROJECTOR_KERNEL: &str = "opening_project_v1";
const PROJECTOR_LOGICAL_BYTES: u64 = 5_109_296;
const ARITHMETIC_LOGICAL_BYTES: u64 = 2_359_424;
const THREADGROUP_WIDTH: u32 = 32;
const PARENT_CENSUS_BYTES: usize = 921_481;
const PARENT_MANIFEST_ENTRIES: usize = 184;

pub const RUNNER_AMBIENT_VARIABLES: [&str; 29] = [
    "RUSTFLAGS",
    "RUSTDOCFLAGS",
    "CARGO_ENCODED_RUSTFLAGS",
    "RUSTC_WRAPPER",
    "RUSTC_WORKSPACE_WRAPPER",
    "MACOSX_DEPLOYMENT_TARGET",
    "SDKROOT",
    "DEVELOPER_DIR",
    "TOOLCHAINS",
    "BASH_ENV",
    "ENV",
    "CDPATH",
    "SHELLOPTS",
    "BASHOPTS",
    "PERL5OPT",
    "PERL5LIB",
    "PYTHONHOME",
    "PYTHONPATH",
    "RUBYOPT",
    "GEM_HOME",
    "GEM_PATH",
    "DYLD_INSERT_LIBRARIES",
    "DYLD_LIBRARY_PATH",
    "DYLD_FRAMEWORK_PATH",
    "DYLD_FALLBACK_LIBRARY_PATH",
    "DYLD_FALLBACK_FRAMEWORK_PATH",
    "DYLD_ROOT_PATH",
    "DYLD_IMAGE_SUFFIX",
    "LD_PRELOAD",
];

const TOOL_PROFILES: [(&str, &str, bool, &[&str]); 5] = [
    ("dispatch2", "0.3.1", false, &["alloc", "block2", "objc2"]),
    ("objc2", "0.6.4", true, &["alloc", "default", "std"]),
    ("objc2-core-graphics", "0.3.2", false, &[]),
    (
        "objc2-foundation",
        "0.3.2",
        false,
        &[
            "NSArray",
            "NSBundle",
            "NSDictionary",
            "NSEnumerator",
            "NSError",
            "NSObject",
            "NSRange",
            "NSString",
            "NSURL",
            "alloc",
            "bitflags",
        ],
    ),
    (
        "objc2-metal",
        "0.3.2",
        false,
        &[
            "MTLAllocation",
            "MTLBuffer",
            "MTLCommandBuffer",
            "MTLCommandEncoder",
            "MTLCommandQueue",
            "MTLComputeCommandEncoder",
            "MTLComputePipeline",
            "MTLDevice",
            "MTLGPUAddress",
            "MTLLibrary",
            "MTLResource",
            "MTLTypes",
            "alloc",
            "bitflags",
            "dispatch2",
            "std",
        ],
    ),
];

/// Unforgeable marker that the frozen ambient variables were absent when the
/// official runner entered.  Construct this before creating a runtime or
/// invoking any verifier.
#[derive(Debug)]
pub struct CleanRunnerEnvironment {
    _private: (),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorityObservation {
    pub section: AuthoritySection,
    pub build_identity: Digest,
    pub freeze56_digest: Digest,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToolchainObservation {
    pub section: ToolchainSection,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeviceObservation {
    pub section: DeviceSection,
    pub allocations: AllocationHighWater,
}

#[derive(Debug)]
pub enum ObservationError {
    Io {
        operation: &'static str,
        path: PathBuf,
        source: io::Error,
    },
    Command {
        label: &'static str,
        status: Option<i32>,
        stderr: String,
    },
    InvalidUtf8(&'static str),
    Invalid(&'static str),
    Mismatch {
        field: &'static str,
        expected: String,
        actual: String,
    },
    Codec(CodecError),
}

impl fmt::Display for ObservationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io {
                operation,
                path,
                source,
            } => write!(
                formatter,
                "{operation} failed for {}: {source}",
                path.display()
            ),
            Self::Command {
                label,
                status,
                stderr,
            } => write!(
                formatter,
                "{label} failed with status {status:?}: {}",
                stderr.trim_end()
            ),
            Self::InvalidUtf8(label) => write!(formatter, "{label} was not exact UTF-8"),
            Self::Invalid(label) => write!(formatter, "invalid {label}"),
            Self::Mismatch {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "{field} mismatch: expected {expected:?}, observed {actual:?}"
            ),
            Self::Codec(error) => write!(formatter, "receipt value rejected: {error}"),
        }
    }
}

impl std::error::Error for ObservationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            Self::Codec(source) => Some(source),
            _ => None,
        }
    }
}

impl From<CodecError> for ObservationError {
    fn from(value: CodecError) -> Self {
        Self::Codec(value)
    }
}

pub type Result<T> = std::result::Result<T, ObservationError>;

/// Must be the first observation made by the official runner.
pub fn verify_runner_entry_environment() -> Result<CleanRunnerEnvironment> {
    verify_ambient_values(RUNNER_AMBIENT_VARIABLES.map(|name| (name, env::var_os(name))))?;
    Ok(CleanRunnerEnvironment { _private: () })
}

/// Hash the exact regular, non-symlink M2 manifest bytes that bind both
/// success and failure evidence.  This deliberately does not run any other
/// authority check: callers need the identity even when a later verifier is
/// the operation that fails.
pub fn observe_m2_manifest_build_identity(repository_root: &Path) -> Result<Digest> {
    let bytes = read_regular(&repository_root.join(M2_SOURCE_MANIFEST_PATH))?;
    Ok(sha256(&bytes))
}

/// Bind runtime authority to the manifest bytes compiled into this exact
/// executable, preventing a source/manifest swap between build and execution.
pub fn verify_compiled_m2_manifest(repository_root: &Path) -> Result<Digest> {
    let bytes = read_regular(&repository_root.join(M2_SOURCE_MANIFEST_PATH))?;
    if bytes != COMPILED_M2_SOURCE_MANIFEST {
        return Err(ObservationError::Invalid(
            "compiled/live M2 source manifest equality",
        ));
    }
    Ok(sha256(&bytes))
}

fn verify_ambient_values<I>(values: I) -> Result<()>
where
    I: IntoIterator<Item = (&'static str, Option<OsString>)>,
{
    for (name, value) in values {
        if value.is_some() {
            return Err(ObservationError::Mismatch {
                field: "runner-entry ambient environment",
                expected: format!("{name} absent"),
                actual: format!("{name} present"),
            });
        }
    }
    Ok(())
}

pub fn observe_authority(
    _environment: &CleanRunnerEnvironment,
    repository_root: &Path,
) -> Result<AuthorityObservation> {
    let root = canonical_repository_root(repository_root)?;
    let _ = verify_compiled_m2_manifest(&root)?;
    run_checked(
        command_at(
            "/bin/bash",
            [
                OsString::from("-p"),
                root.join("walt/ci/verify_m2_history.sh").into_os_string(),
            ],
            &root,
        ),
        "historical verifier",
    )?;
    run_checked(
        command_at(
            "/bin/bash",
            [
                OsString::from("-p"),
                root.join("walt/ci/verify_m2_sources.sh").into_os_string(),
            ],
            &root,
        ),
        "M2 source verifier",
    )?;

    let parent_census = git_blob(
        &root,
        "3b4c6d60fef371e3050de151ccf9eaefbc2d2da7:walt/CENSUS-RULINGS.md",
    )?;
    require_usize(
        "parent CENSUS byte length",
        PARENT_CENSUS_BYTES,
        parent_census.len(),
    )?;
    require_digest(
        "parent CENSUS digest",
        &PARENT_CENSUS_SHA256,
        &sha256(&parent_census),
    )?;
    let current_census = read_regular(&root.join("walt/CENSUS-RULINGS.md"))?;
    if current_census.get(..PARENT_CENSUS_BYTES) != Some(parent_census.as_slice()) {
        return Err(ObservationError::Invalid("current parent CENSUS prefix"));
    }
    verify_freeze56_census_fence(&current_census)?;

    require_usize(
        "freeze-55 descriptor byte length",
        944,
        GT1_FREEZE_SET_DESCRIPTOR_V1.len(),
    )?;
    require_digest(
        "freeze-55 descriptor digest",
        &GT1_FREEZE_SET_SHA256_V1,
        &sha256(GT1_FREEZE_SET_DESCRIPTOR_V1),
    )?;
    require_usize(
        "freeze-56 descriptor byte length",
        899,
        FREEZE56_DESCRIPTOR.len(),
    )?;
    require_digest(
        "freeze-56 descriptor digest",
        &FREEZE56_DESCRIPTOR_SHA256,
        &sha256(FREEZE56_DESCRIPTOR),
    )?;

    let artifact_bytes = [
        read_regular(&root.join("walt/math/gpu_native_trick1_m0_m1_sources_v1.sha256"))?,
        GT1_FREEZE_SET_DESCRIPTOR_V1.to_vec(),
        read_regular(&root.join("walt/math/gpu_native_trick1_implementers_guide_v0.2.md"))?,
        read_regular(&root.join("walt/GPU-NATIVE-TRICK1.md"))?,
        read_regular(
            &root.join("walt/receipts/gpu_native_trick1_m0_m1_v1/opening_max_cell_envelope_v1.bin"),
        )?,
        read_regular(
            &root.join("walt/receipts/gpu_native_trick1_m0_m1_v1/grade5_declared_stop_v1.bin"),
        )?,
        read_regular(
            &root.join("walt/receipts/gpu_native_trick1_m0_m1_v1/receipt_summary_v1.txt"),
        )?,
        read_regular(&root.join("walt/receipts/gpu_native_trick1_gate0_2026-08-16.txt"))?,
        read_regular(&root.join(M2_SOURCE_MANIFEST_PATH))?,
        read_regular(&root.join("walt/GPU-NATIVE-TRICK1-M2.md"))?,
        read_regular(&root.join("walt/Cargo.lock"))?,
        FREEZE56_DESCRIPTOR.to_vec(),
        parent_census,
    ];
    let old_manifest_text = exact_utf8("parent M0/M1 source manifest", &artifact_bytes[0])?;
    let old_manifest_entries = old_manifest_text
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .count();
    require_usize(
        "parent source manifest entry count",
        PARENT_MANIFEST_ENTRIES,
        old_manifest_entries,
    )?;
    let tags = [
        ArtifactTag::ParentSourceManifest,
        ArtifactTag::Freeze55Descriptor,
        ArtifactTag::ReceivedGuide,
        ArtifactTag::M0M1Contract,
        ArtifactTag::OpeningEnvelope,
        ArtifactTag::Grade5Stop,
        ArtifactTag::M0M1Summary,
        ArtifactTag::HistoricalGate0,
        ArtifactTag::M2SourceManifest,
        ArtifactTag::M2Contract,
        ArtifactTag::CargoLock,
        ArtifactTag::Freeze56Descriptor,
        ArtifactTag::ParentCensus,
    ];
    let identities = tags
        .into_iter()
        .zip(artifact_bytes.iter())
        .map(|(tag, bytes)| {
            Ok(ArtifactIdentity {
                tag,
                byte_length: u64::try_from(bytes.len())
                    .map_err(|_| ObservationError::Invalid("artifact byte length"))?,
                digest: sha256(bytes),
            })
        })
        .collect::<Result<Vec<_>>>()?;
    let build_identity = identities[8].digest;
    let freeze56_digest = identities[11].digest;
    let section = AuthoritySection {
        parent_commit: PARENT_COMMIT_SHA1,
        identities,
        freeze56_descriptor: FREEZE56_DESCRIPTOR.to_vec(),
    };
    section.validate(Some(&build_identity), Some(&freeze56_digest))?;
    let _ = section.encode()?;
    require_u64("M2 contract bytes", CONTRACT_BYTES, artifact_bytes[9].len())?;
    require_digest(
        "M2 contract digest",
        &CONTRACT_SHA256,
        &sha256(&artifact_bytes[9]),
    )?;
    Ok(AuthorityObservation {
        section,
        build_identity,
        freeze56_digest,
    })
}

pub fn observe_toolchain(
    _environment: &CleanRunnerEnvironment,
    repository_root: &Path,
) -> Result<ToolchainObservation> {
    let root = canonical_repository_root(repository_root)?;
    verify_build_target()?;
    verify_release_profile(&root)?;

    let rustc_path = resolve_rust_tool(&root, "rustc")?;
    let cargo_path = resolve_rust_tool(&root, "cargo")?;

    let rustc_output = run_checked(
        command_at(&rustc_path, [OsString::from("-vV")], &root),
        "rustc version observation",
    )?;
    let rustc_text = exact_utf8("rustc version output", &rustc_output.stdout)?;
    let rustc = parse_rustc_verbose(rustc_text)?;
    require_str("rustc release", RUST_RELEASE, &rustc.release)?;
    require_str("rustc host", TARGET, &rustc.host)?;

    let cargo_output = run_checked(
        command_at(&cargo_path, [OsString::from("-V")], &root),
        "Cargo version observation",
    )?;
    let cargo_release =
        parse_cargo_release(exact_utf8("Cargo version output", &cargo_output.stdout)?)?;
    require_str("Cargo release", RUST_RELEASE, &cargo_release)?;

    let xcodebuild_path = resolve_tool(&root, None, "xcodebuild")?;
    let xctrace_path = resolve_tool(&root, None, "xctrace")?;
    let metal_path = resolve_tool(&root, Some(METAL_COMPONENT_ID), "metal")?;
    let metallib_path = resolve_tool(&root, Some(METAL_COMPONENT_ID), "metallib")?;
    let metal_ar_path = resolve_tool(&root, Some(METAL_COMPONENT_ID), "metal-ar")?;

    let xcode = run_path(
        &xcodebuild_path,
        [OsString::from("-version")],
        &root,
        "xcodebuild",
    )?;
    let (xcode_version, xcode_build) =
        parse_xcode_version(exact_utf8("xcodebuild version output", &xcode.stdout)?)?;
    require_str("Xcode version", XCODE_VERSION, &xcode_version)?;
    require_str("Xcode build", XCODE_BUILD, &xcode_build)?;

    verify_metal_component(&metal_path)?;
    verify_metal_component(&metallib_path)?;
    verify_metal_component(&metal_ar_path)?;
    let metal = run_path(&metal_path, [OsString::from("--version")], &root, "metal")?;
    let compiler_version = parse_prefixed_version(
        exact_utf8("metal version output", &metal.stdout)?,
        "Apple metal version ",
    )?;
    require_str("Metal compiler version", METAL_VERSION, &compiler_version)?;
    let metallib = run_path(
        &metallib_path,
        [OsString::from("--version")],
        &root,
        "metallib",
    )?;
    let linker_version = parse_prefixed_version(
        exact_utf8("metallib version output", &metallib.stdout)?,
        "AIR-LLD ",
    )?;
    require_str("Metal AIR linker version", METAL_VERSION, &linker_version)?;
    let metal_ar = run_path(
        &metal_ar_path,
        [OsString::from("--version")],
        &root,
        "metal-ar",
    )?;
    let metal_ar_version = parse_prefixed_version(
        exact_utf8("metal-ar version output", &metal_ar.stdout)?,
        "Apple LLVM version ",
    )?;
    require_str("Metal archive version", METAL_VERSION, &metal_ar_version)?;

    let xctrace = run_path(&xctrace_path, [OsString::from("version")], &root, "xctrace")?;
    let xctrace_version = exact_utf8("xctrace version output", &xctrace.stdout)?
        .trim_end_matches('\n')
        .strip_prefix("xctrace version ")
        .ok_or(ObservationError::Invalid("xctrace version grammar"))?
        .to_owned();
    require_str("xctrace version", XCTRACE_VERSION, &xctrace_version)?;

    let sdk_version = run_xcrun_value(&root, ["--sdk", "macosx", "--show-sdk-version"])?;
    let sdk_build = run_xcrun_value(&root, ["--sdk", "macosx", "--show-sdk-build-version"])?;
    require_str("SDK version", SDK_VERSION, &sdk_version)?;
    require_str("SDK build", SDK_BUILD, &sdk_build)?;

    let lock_bytes = read_regular(&root.join("walt/Cargo.lock"))?;
    let lock_text = exact_utf8("Cargo.lock", &lock_bytes)?;
    let locked = parse_lock_packages(lock_text)?;
    let tree = observe_cargo_feature_tree(&root, &cargo_path, &rustc_path)?;
    let packages = build_package_records(&locked, &tree)?;

    let tools = [
        (ToolId::Metal, metal_path),
        (ToolId::Metallib, metallib_path),
        (ToolId::MetalAr, metal_ar_path),
        (ToolId::Xctrace, xctrace_path),
        (ToolId::Xcodebuild, xcodebuild_path),
    ]
    .into_iter()
    .map(|(id, path)| tool_record(id, &path))
    .collect::<Result<Vec<_>>>()?;

    let sources = shader_source_records(&root)?;
    run_checked(
        command_at(
            "/usr/bin/awk",
            [
                OsString::from("-f"),
                root.join("walt/ci/check_msl_no_float.awk").into_os_string(),
                root.join("walt/walt-metal/shaders/00_u256.metal")
                    .into_os_string(),
                root.join("walt/walt-metal/shaders/01_opening_projector.metal")
                    .into_os_string(),
            ],
            &root,
        ),
        "MSL no-float source gate",
    )?;

    let invocations = normalized_invocations();
    let committed_path = root.join("walt/walt-metal/shaders/walt_m2.metallib");
    let committed_bytes = read_regular(&committed_path)?;
    require_kernel_names(&committed_bytes)?;
    let committed_metallib_digest = sha256(&committed_bytes);
    let metallib_bytes = u64::try_from(committed_bytes.len())
        .map_err(|_| ObservationError::Invalid("metallib byte length"))?;

    let verifier = run_checked(
        command_at(
            "/bin/bash",
            [
                OsString::from("-p"),
                root.join("walt/walt-metal/shaders/build_metallib.sh")
                    .into_os_string(),
                OsString::from("verify"),
            ],
            &root,
        ),
        "two-build metallib verifier",
    )?;
    verify_metallib_verifier_output(&verifier.stdout, &committed_metallib_digest)?;

    let texts = vec![
        rustc.release,
        rustc.host,
        cargo_release,
        TARGET.to_owned(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        xcode_version,
        xcode_build,
        METAL_COMPONENT_ID.to_owned(),
        METAL_COMPONENT_BUILD.to_owned(),
        compiler_version,
        sdk_version,
        sdk_build,
        DEPLOYMENT_TARGET.to_owned(),
        xctrace_version,
        ARITHMETIC_KERNEL.to_owned(),
        PROJECTOR_KERNEL.to_owned(),
    ];
    let section = ToolchainSection {
        texts,
        packages,
        tools,
        sources,
        invocations,
        metallib_bytes,
        committed_metallib_digest,
        fresh_build_1_digest: committed_metallib_digest,
        fresh_build_2_digest: committed_metallib_digest,
        committed_repro_digest: committed_metallib_digest,
    };
    section.validate()?;
    let _ = section.encode()?;
    Ok(ToolchainObservation { section })
}

pub fn observe_device(
    _environment: &CleanRunnerEnvironment,
    repository_root: &Path,
    profile: &DeviceProfile,
) -> Result<DeviceObservation> {
    let root = canonical_repository_root(repository_root)?;
    let product_version = run_sw_vers(&root, "-productVersion")?;
    let build_version = run_sw_vers(&root, "-buildVersion")?;
    device_from_values(profile, &product_version, &build_version)
}

fn device_from_values(
    profile: &DeviceProfile,
    product_version: &str,
    build_version: &str,
) -> Result<DeviceObservation> {
    require_str("macOS version", MACOS_VERSION, product_version)?;
    require_str("macOS build", MACOS_BUILD, build_version)?;
    require_str(
        "sanitized Metal device name",
        DEVICE_NAME,
        &profile.sanitized_name,
    )?;
    if !profile.unified_memory {
        return Err(ObservationError::Invalid("unified-memory device flag"));
    }
    if !profile.gate0_passed {
        return Err(ObservationError::Invalid(
            "completed Rust Gate-0 observation",
        ));
    }
    if profile.maximum_buffer_length < PROJECTOR_LOGICAL_BYTES
        || profile.recommended_working_set == 0
        || profile.maximum_threads.into_iter().any(|value| value == 0)
        || profile.maximum_threads[0] < THREADGROUP_WIDTH
        || profile.maximum_threadgroup_memory == 0
    {
        return Err(ObservationError::Invalid("Metal device limits"));
    }
    verify_pipeline("arithmetic pipeline", &profile.arithmetic_pipeline, profile)?;
    verify_pipeline("projector pipeline", &profile.opening_pipeline, profile)?;
    verify_allocations(&profile.allocations)?;

    let pipelines = vec![
        PipelineRecord {
            kernel: KernelId::Arithmetic,
            execution_width: profile.arithmetic_pipeline.execution_width,
            maximum_threads: profile.arithmetic_pipeline.maximum_threads,
            static_group_memory: profile.arithmetic_pipeline.static_threadgroup_memory,
        },
        PipelineRecord {
            kernel: KernelId::Projector,
            execution_width: profile.opening_pipeline.execution_width,
            maximum_threads: profile.opening_pipeline.maximum_threads,
            static_group_memory: profile.opening_pipeline.static_threadgroup_memory,
        },
    ];
    let section = DeviceSection {
        texts: vec![
            product_version.to_owned(),
            build_version.to_owned(),
            profile.sanitized_name.clone(),
        ],
        max_buffer_length: profile.maximum_buffer_length,
        recommended_working_set: profile.recommended_working_set,
        max_threads: profile.maximum_threads,
        max_threadgroup_memory: profile.maximum_threadgroup_memory,
        pipelines,
    };
    section.validate()?;
    let _ = section.encode()?;
    Ok(DeviceObservation {
        section,
        allocations: profile.allocations.clone(),
    })
}

/// Render the stable checked host/tool/device descriptor.  The format carries
/// no absolute path, time, process identifier, registry identifier, or device
/// serial.  Repository-relative shader paths and normalized placeholders are
/// part of the frozen compiler observation.
pub fn render_host_tool_device_descriptor(
    toolchain: &ToolchainObservation,
    device: &DeviceObservation,
) -> Result<Vec<u8>> {
    toolchain.section.validate()?;
    device.section.validate()?;
    verify_allocations(&device.allocations)?;

    let mut lines = Vec::new();
    lines.push("W42-M2-HOST-TOOL-DEVICE-DESCRIPTOR-V1".to_owned());
    let text_names = [
        "rustc_release",
        "rustc_host",
        "cargo_release",
        "rust_build_target",
        "rustflags",
        "cargo_encoded_rustflags",
        "rustc_wrapper",
        "rustc_workspace_wrapper",
        "xcode_version",
        "xcode_build",
        "metal_component_id",
        "metal_component_build",
        "metal_compiler_version",
        "sdk_version",
        "sdk_build",
        "deployment_target",
        "xctrace_version",
        "arithmetic_kernel",
        "projector_kernel",
    ];
    for (name, value) in text_names.into_iter().zip(&toolchain.section.texts) {
        descriptor_line(&mut lines, name, value)?;
    }
    for package in &toolchain.section.packages {
        let prefix = format!("package.{}", package.name);
        descriptor_line(&mut lines, &format!("{prefix}.version"), &package.version)?;
        descriptor_line(
            &mut lines,
            &format!("{prefix}.checksum"),
            &hex_digest(&package.checksum),
        )?;
        descriptor_line(
            &mut lines,
            &format!("{prefix}.default_feature"),
            if package.default_feature { "1" } else { "0" },
        )?;
        descriptor_line(
            &mut lines,
            &format!("{prefix}.features"),
            &package.activated_features.join(","),
        )?;
    }
    for tool in &toolchain.section.tools {
        let name = tool_name(tool.id);
        descriptor_line(
            &mut lines,
            &format!("tool.{name}.bytes"),
            &tool.executable_bytes.to_string(),
        )?;
        descriptor_line(
            &mut lines,
            &format!("tool.{name}.sha256"),
            &hex_digest(&tool.digest),
        )?;
    }
    for (index, source) in toolchain.section.sources.iter().enumerate() {
        descriptor_line(
            &mut lines,
            &format!("source.{index}.kind"),
            source_kind_name(source.kind),
        )?;
        descriptor_line(&mut lines, &format!("source.{index}.path"), &source.path)?;
        descriptor_line(
            &mut lines,
            &format!("source.{index}.bytes"),
            &source.byte_length.to_string(),
        )?;
        descriptor_line(
            &mut lines,
            &format!("source.{index}.sha256"),
            &hex_digest(&source.digest),
        )?;
    }
    for (index, invocation) in toolchain.section.invocations.iter().enumerate() {
        descriptor_line(
            &mut lines,
            &format!("invocation.{index}.kind"),
            invocation_kind_name(invocation.kind),
        )?;
        descriptor_line(
            &mut lines,
            &format!("invocation.{index}.source_index"),
            if invocation.source_index == u32::MAX {
                "none".to_owned()
            } else {
                invocation.source_index.to_string()
            }
            .as_str(),
        )?;
        descriptor_line(
            &mut lines,
            &format!("invocation.{index}.argc"),
            &invocation.arguments.len().to_string(),
        )?;
        for (argument_index, argument) in invocation.arguments.iter().enumerate() {
            descriptor_line(
                &mut lines,
                &format!("invocation.{index}.argv.{argument_index}"),
                argument,
            )?;
        }
    }
    descriptor_line(
        &mut lines,
        "metallib_bytes",
        &toolchain.section.metallib_bytes.to_string(),
    )?;
    descriptor_line(
        &mut lines,
        "metallib_sha256",
        &hex_digest(&toolchain.section.committed_metallib_digest),
    )?;
    descriptor_line(&mut lines, "metallib_repro_builds", "2")?;

    let device_text_names = ["macos_version", "macos_build", "device_name"];
    for (name, value) in device_text_names.into_iter().zip(&device.section.texts) {
        descriptor_line(&mut lines, name, value)?;
    }
    descriptor_line(&mut lines, "device_unified_memory", "1")?;
    descriptor_line(
        &mut lines,
        "device_max_buffer_length",
        &device.section.max_buffer_length.to_string(),
    )?;
    descriptor_line(
        &mut lines,
        "device_recommended_working_set",
        &device.section.recommended_working_set.to_string(),
    )?;
    descriptor_line(
        &mut lines,
        "device_max_threads_x",
        &device.section.max_threads[0].to_string(),
    )?;
    descriptor_line(
        &mut lines,
        "device_max_threads_y",
        &device.section.max_threads[1].to_string(),
    )?;
    descriptor_line(
        &mut lines,
        "device_max_threads_z",
        &device.section.max_threads[2].to_string(),
    )?;
    descriptor_line(
        &mut lines,
        "device_max_threadgroup_memory",
        &device.section.max_threadgroup_memory.to_string(),
    )?;
    descriptor_line(&mut lines, "device_gate0_native_status", "4")?;
    for pipeline in &device.section.pipelines {
        let name = kernel_name(pipeline.kernel);
        descriptor_line(
            &mut lines,
            &format!("pipeline.{name}.execution_width"),
            &pipeline.execution_width.to_string(),
        )?;
        descriptor_line(
            &mut lines,
            &format!("pipeline.{name}.maximum_threads"),
            &pipeline.maximum_threads.to_string(),
        )?;
        descriptor_line(
            &mut lines,
            &format!("pipeline.{name}.static_threadgroup_memory"),
            &pipeline.static_group_memory.to_string(),
        )?;
    }
    descriptor_line(
        &mut lines,
        "allocation.projector.logical_bytes",
        &device.allocations.projector_logical_bytes.to_string(),
    )?;
    descriptor_line(
        &mut lines,
        "allocation.projector.reported_bytes",
        &device.allocations.projector_reported_bytes.to_string(),
    )?;
    descriptor_line(
        &mut lines,
        "allocation.arithmetic.logical_bytes",
        &device.allocations.arithmetic_logical_bytes.to_string(),
    )?;
    descriptor_line(
        &mut lines,
        "allocation.arithmetic.reported_bytes",
        &device.allocations.arithmetic_reported_bytes.to_string(),
    )?;

    let mut rendered = lines.join("\n").into_bytes();
    rendered.push(b'\n');
    if rendered.windows(2).any(|window| window == b"\r\n") || rendered.contains(&0) {
        return Err(ObservationError::Invalid("canonical descriptor bytes"));
    }
    Ok(rendered)
}

pub fn verify_checked_descriptor(
    repository_root: &Path,
    toolchain: &ToolchainObservation,
    device: &DeviceObservation,
) -> Result<()> {
    let root = canonical_repository_root(repository_root)?;
    let expected = read_regular(&root.join(CHECKED_DESCRIPTOR_PATH))?;
    let observed = render_host_tool_device_descriptor(toolchain, device)?;
    if expected != observed {
        return Err(ObservationError::Invalid(
            "checked host/tool/device descriptor comparison",
        ));
    }
    Ok(())
}

/// Binds every observational field admitted by a success receipt to the
/// checked repository artifacts without creating a second Metal runtime.  The
/// live child already performed Gate-0; this comparison prevents a
/// self-consistent but fabricated receipt from substituting different host,
/// tool, shader, metallib, device, source-manifest, or Cargo.lock facts.
pub fn verify_success_receipt_observations(
    repository_root: &Path,
    receipt: &SuccessReceipt,
) -> Result<()> {
    let root = canonical_repository_root(repository_root)?;
    let manifest = read_regular(&root.join(M2_SOURCE_MANIFEST_PATH))?;
    let lock = read_regular(&root.join("walt/Cargo.lock"))?;
    let identities = &receipt.sections.authority.identities;
    if identities.len() != 13
        || identities[8].byte_length
            != u64::try_from(manifest.len())
                .map_err(|_| ObservationError::Invalid("source manifest byte length"))?
        || identities[8].digest != sha256(&manifest)
        || identities[10].byte_length
            != u64::try_from(lock.len())
                .map_err(|_| ObservationError::Invalid("Cargo.lock byte length"))?
        || identities[10].digest != sha256(&lock)
    {
        return Err(ObservationError::Invalid(
            "receipt dynamic authority artifact identity",
        ));
    }

    let toolchain = ToolchainObservation {
        section: receipt.sections.toolchain.clone(),
    };
    let device = DeviceObservation {
        section: receipt.sections.device.clone(),
        allocations: AllocationHighWater {
            projector_logical_bytes: PROJECTOR_LOGICAL_BYTES,
            projector_reported_bytes: PROJECTOR_LOGICAL_BYTES,
            arithmetic_logical_bytes: ARITHMETIC_LOGICAL_BYTES,
            arithmetic_reported_bytes: ARITHMETIC_LOGICAL_BYTES,
        },
    };
    verify_checked_descriptor(&root, &toolchain, &device)
}

fn canonical_repository_root(path: &Path) -> Result<PathBuf> {
    let root = fs::canonicalize(path).map_err(|source| ObservationError::Io {
        operation: "canonicalize repository root",
        path: path.to_path_buf(),
        source,
    })?;
    for required in ["CLAUDE.md", "QUICKSTART.md", "walt/CENSUS-RULINGS.md"] {
        let candidate = root.join(required);
        let metadata = fs::symlink_metadata(&candidate).map_err(|source| ObservationError::Io {
            operation: "inspect repository authority path",
            path: candidate.clone(),
            source,
        })?;
        if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
            return Err(ObservationError::Invalid(
                "repository root authority layout",
            ));
        }
    }
    Ok(root)
}

fn read_regular(path: &Path) -> Result<Vec<u8>> {
    let metadata = fs::symlink_metadata(path).map_err(|source| ObservationError::Io {
        operation: "inspect regular file",
        path: path.to_path_buf(),
        source,
    })?;
    if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
        return Err(ObservationError::Invalid("regular non-symlink artifact"));
    }
    fs::read(path).map_err(|source| ObservationError::Io {
        operation: "read artifact",
        path: path.to_path_buf(),
        source,
    })
}

fn command_at<I, S, P>(program: P, arguments: I, current_dir: &Path) -> Command
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
    P: AsRef<OsStr>,
{
    let mut command = Command::new(program);
    command
        .args(arguments)
        .current_dir(current_dir)
        .env_clear()
        .env("HOME", env::var_os("HOME").unwrap_or_default())
        .env("PATH", "/usr/bin:/bin:/usr/sbin:/sbin")
        .env("LC_ALL", "C")
        .env("TMPDIR", "/tmp");
    command
}

fn run_checked(mut command: Command, label: &'static str) -> Result<Output> {
    let output = command.output().map_err(|source| ObservationError::Io {
        operation: "execute observation command",
        path: PathBuf::from(command.get_program()),
        source,
    })?;
    if !output.status.success() {
        return Err(ObservationError::Command {
            label,
            status: output.status.code(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        });
    }
    Ok(output)
}

fn run_path<I, S>(
    program: &Path,
    arguments: I,
    current_dir: &Path,
    label: &'static str,
) -> Result<Output>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    run_checked(command_at(program, arguments, current_dir), label)
}

fn exact_utf8<'a>(label: &'static str, bytes: &'a [u8]) -> Result<&'a str> {
    std::str::from_utf8(bytes).map_err(|_| ObservationError::InvalidUtf8(label))
}

fn run_xcrun_value<const N: usize>(root: &Path, arguments: [&str; N]) -> Result<String> {
    let output = run_checked(command_at("/usr/bin/xcrun", arguments, root), "xcrun")?;
    one_line("xcrun value", &output.stdout)
}

fn run_sw_vers(root: &Path, argument: &str) -> Result<String> {
    let output = run_checked(command_at("/usr/bin/sw_vers", [argument], root), "sw_vers")?;
    one_line("sw_vers value", &output.stdout)
}

fn one_line(label: &'static str, bytes: &[u8]) -> Result<String> {
    let text = exact_utf8(label, bytes)?;
    let value = text.strip_suffix('\n').unwrap_or(text);
    if value.is_empty() || value.contains(['\r', '\n', '\0']) {
        return Err(ObservationError::Invalid(label));
    }
    Ok(value.to_owned())
}

fn git_blob(root: &Path, object: &str) -> Result<Vec<u8>> {
    let mut command = command_at(
        "/usr/bin/git",
        [
            OsString::from("--no-replace-objects"),
            OsString::from("-c"),
            OsString::from("core.fsmonitor=false"),
            OsString::from("-c"),
            OsString::from("core.untrackedCache=false"),
            OsString::from("-c"),
            OsString::from("color.ui=false"),
            OsString::from("-C"),
            root.as_os_str().to_owned(),
            OsString::from("show"),
            OsString::from(object),
        ],
        root,
    );
    command
        .env("HOME", "/var/empty")
        .env("XDG_CONFIG_HOME", "/var/empty")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_NO_REPLACE_OBJECTS", "1");
    let output = run_checked(command, "read immutable parent blob")?;
    Ok(output.stdout)
}

fn verify_freeze56_census_fence(census: &[u8]) -> Result<()> {
    let mut needle = Vec::with_capacity(FREEZE56_DESCRIPTOR.len() + 17);
    needle.extend_from_slice(b"  ```text\n  ");
    needle.extend_from_slice(FREEZE56_DESCRIPTOR);
    needle.extend_from_slice(b"\n  ```");
    let count = census
        .windows(needle.len())
        .filter(|window| *window == needle.as_slice())
        .count();
    require_usize("freeze-56 fenced descriptor occurrence count", 1, count)
}

fn verify_build_target() -> Result<()> {
    if !cfg!(all(
        target_arch = "aarch64",
        target_os = "macos",
        target_endian = "little"
    )) {
        return Err(ObservationError::Mismatch {
            field: "Rust build target",
            expected: TARGET.to_owned(),
            actual: format!("{}-{}", env::consts::ARCH, env::consts::OS),
        });
    }
    if cfg!(debug_assertions) {
        return Err(ObservationError::Invalid("release build profile"));
    }
    Ok(())
}

fn verify_release_profile(root: &Path) -> Result<()> {
    let bytes = read_regular(&root.join("walt/Cargo.toml"))?;
    let text = exact_utf8("Walt workspace Cargo.toml", &bytes)?;
    if text.contains('\r') {
        return Err(ObservationError::Invalid("Walt Cargo.toml CR byte"));
    }
    let mut in_release = false;
    let mut release_sections = 0_usize;
    let mut overflow_settings = 0_usize;
    for raw_line in text.lines() {
        let line = raw_line.trim();
        if line.starts_with('[') {
            in_release = line == "[profile.release]";
            if in_release {
                release_sections += 1;
            }
            continue;
        }
        if in_release && line.starts_with("overflow-checks") {
            if line != "overflow-checks = true" {
                return Err(ObservationError::Invalid("release overflow-checks setting"));
            }
            overflow_settings += 1;
        }
    }
    require_usize("release profile section count", 1, release_sections)?;
    require_usize(
        "release overflow-checks setting count",
        1,
        overflow_settings,
    )
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RustcVersion {
    release: String,
    host: String,
}

fn parse_rustc_verbose(text: &str) -> Result<RustcVersion> {
    let mut release = None;
    let mut host = None;
    for line in text.lines() {
        if let Some(value) = line.strip_prefix("release: ") {
            if release.replace(value.to_owned()).is_some() {
                return Err(ObservationError::Invalid("duplicate rustc release"));
            }
        }
        if let Some(value) = line.strip_prefix("host: ") {
            if host.replace(value.to_owned()).is_some() {
                return Err(ObservationError::Invalid("duplicate rustc host"));
            }
        }
    }
    Ok(RustcVersion {
        release: release.ok_or(ObservationError::Invalid("missing rustc release"))?,
        host: host.ok_or(ObservationError::Invalid("missing rustc host"))?,
    })
}

fn parse_cargo_release(text: &str) -> Result<String> {
    let first = text
        .strip_prefix("cargo ")
        .ok_or(ObservationError::Invalid("Cargo version grammar"))?;
    let release = first
        .split_once(' ')
        .map_or(first.trim_end_matches('\n'), |(value, _)| value);
    if release.is_empty() || release.contains(['\r', '\n', '\0']) {
        return Err(ObservationError::Invalid("Cargo release"));
    }
    Ok(release.to_owned())
}

fn parse_xcode_version(text: &str) -> Result<(String, String)> {
    let mut lines = text.lines();
    let version = lines
        .next()
        .and_then(|line| line.strip_prefix("Xcode "))
        .ok_or(ObservationError::Invalid("Xcode version grammar"))?;
    let build = lines
        .next()
        .and_then(|line| line.strip_prefix("Build version "))
        .ok_or(ObservationError::Invalid("Xcode build grammar"))?;
    if lines.next().is_some() || version.is_empty() || build.is_empty() {
        return Err(ObservationError::Invalid("Xcode version line count"));
    }
    Ok((version.to_owned(), build.to_owned()))
}

fn parse_prefixed_version(text: &str, prefix: &'static str) -> Result<String> {
    let first_line = text
        .lines()
        .next()
        .ok_or(ObservationError::Invalid("empty tool version"))?;
    let rest = first_line
        .strip_prefix(prefix)
        .ok_or(ObservationError::Invalid("tool version prefix"))?;
    let version = rest.split_once(' ').map_or(rest, |(value, _)| value).trim();
    if version.is_empty() {
        return Err(ObservationError::Invalid("tool version value"));
    }
    Ok(version.to_owned())
}

fn resolve_tool(root: &Path, toolchain: Option<&str>, tool: &str) -> Result<PathBuf> {
    let mut arguments = Vec::new();
    if let Some(toolchain) = toolchain {
        arguments.push(OsString::from("--toolchain"));
        arguments.push(OsString::from(toolchain));
    }
    arguments.push(OsString::from("-f"));
    arguments.push(OsString::from(tool));
    let output = run_checked(
        command_at("/usr/bin/xcrun", arguments, root),
        "resolve Xcode tool",
    )?;
    let path_text = one_line("resolved tool path", &output.stdout)?;
    let path = PathBuf::from(path_text);
    if !path.is_absolute() {
        return Err(ObservationError::Invalid("resolved absolute tool path"));
    }
    let metadata = fs::metadata(&path).map_err(|source| ObservationError::Io {
        operation: "inspect resolved tool target",
        path: path.clone(),
        source,
    })?;
    if !metadata.is_file() || metadata.len() == 0 {
        return Err(ObservationError::Invalid("resolved tool target"));
    }
    Ok(path)
}

fn resolve_rust_tool(root: &Path, tool: &str) -> Result<PathBuf> {
    if !matches!(tool, "rustc" | "cargo") {
        return Err(ObservationError::Invalid("closed Rust tool name"));
    }
    let home = env::var_os("HOME").ok_or(ObservationError::Invalid("HOME for rustup proxy"))?;
    let home = PathBuf::from(home);
    if !home.is_absolute() {
        return Err(ObservationError::Invalid("absolute HOME for rustup proxy"));
    }
    let rustup = home.join(".cargo/bin/rustup");
    let rust_workspace = root.join("walt");
    let output = run_checked(
        command_at(
            &rustup,
            [OsString::from("which"), OsString::from(tool)],
            &rust_workspace,
        ),
        "resolve pinned Rust tool",
    )?;
    let path = PathBuf::from(one_line("resolved Rust tool path", &output.stdout)?);
    if !path.is_absolute() {
        return Err(ObservationError::Invalid(
            "resolved absolute Rust tool path",
        ));
    }
    let canonical = fs::canonicalize(&path).map_err(|source| ObservationError::Io {
        operation: "canonicalize resolved Rust tool",
        path: path.clone(),
        source,
    })?;
    let expected_suffix = PathBuf::from(format!(
        ".rustup/toolchains/{RUST_RELEASE}-{TARGET}/bin/{tool}"
    ));
    if !canonical.ends_with(&expected_suffix) {
        return Err(ObservationError::Mismatch {
            field: "resolved pinned Rust tool path",
            expected: expected_suffix.display().to_string(),
            actual: canonical.display().to_string(),
        });
    }
    let metadata = fs::metadata(&canonical).map_err(|source| ObservationError::Io {
        operation: "inspect resolved Rust tool",
        path: canonical.clone(),
        source,
    })?;
    if !metadata.is_file() || metadata.len() == 0 {
        return Err(ObservationError::Invalid("resolved Rust tool target"));
    }
    Ok(canonical)
}

fn verify_metal_component(metal_path: &Path) -> Result<()> {
    let canonical = fs::canonicalize(metal_path).map_err(|source| ObservationError::Io {
        operation: "canonicalize Metal compiler target",
        path: metal_path.to_path_buf(),
        source,
    })?;
    let toolchain_root = canonical
        .ancestors()
        .find(|ancestor| {
            ancestor
                .file_name()
                .and_then(OsStr::to_str)
                .is_some_and(|name| name == "Metal.xctoolchain")
        })
        .ok_or(ObservationError::Invalid("Metal toolchain root"))?;
    let asset_name = toolchain_root
        .parent()
        .and_then(Path::file_name)
        .and_then(OsStr::to_str)
        .ok_or(ObservationError::Invalid("Metal component asset name"))?;
    if !asset_name.starts_with("com.apple.MobileAsset.MetalToolchain-v17.6.109.0.") {
        return Err(ObservationError::Mismatch {
            field: "Metal component build asset",
            expected: METAL_COMPONENT_BUILD.to_owned(),
            actual: asset_name.to_owned(),
        });
    }
    let info = read_regular(&toolchain_root.join("ToolchainInfo.plist"))?;
    let info_text = exact_utf8("Metal ToolchainInfo.plist", &info)?;
    let identifier = format!("<string>{METAL_COMPONENT_ID}</string>");
    if !info_text.contains("<key>Identifier</key>") || !info_text.contains(&identifier) {
        return Err(ObservationError::Invalid("Metal component identifier"));
    }
    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct LockedPackage {
    name: String,
    version: String,
    source: Option<String>,
    checksum: Option<String>,
}

fn parse_lock_packages(text: &str) -> Result<Vec<LockedPackage>> {
    if text.contains('\r') {
        return Err(ObservationError::Invalid("Cargo.lock CR byte"));
    }
    let mut packages = Vec::new();
    let mut current: Option<LockedPackage> = None;
    for line in text.lines() {
        if line == "[[package]]" {
            if let Some(package) = current.take() {
                finish_lock_package(&package)?;
                packages.push(package);
            }
            current = Some(LockedPackage {
                name: String::new(),
                version: String::new(),
                source: None,
                checksum: None,
            });
            continue;
        }
        let Some(package) = current.as_mut() else {
            continue;
        };
        if let Some(value) = parse_toml_basic_string(line, "name")? {
            if !package.name.is_empty() {
                return Err(ObservationError::Invalid("duplicate lock package name"));
            }
            package.name = value;
        } else if let Some(value) = parse_toml_basic_string(line, "version")? {
            if !package.version.is_empty() {
                return Err(ObservationError::Invalid("duplicate lock package version"));
            }
            package.version = value;
        } else if let Some(value) = parse_toml_basic_string(line, "source")? {
            if package.source.replace(value).is_some() {
                return Err(ObservationError::Invalid("duplicate lock package source"));
            }
        } else if let Some(value) = parse_toml_basic_string(line, "checksum")? {
            if package.checksum.replace(value).is_some() {
                return Err(ObservationError::Invalid("duplicate lock package checksum"));
            }
        }
    }
    if let Some(package) = current {
        finish_lock_package(&package)?;
        packages.push(package);
    }
    if packages.is_empty() {
        return Err(ObservationError::Invalid("Cargo.lock package census"));
    }
    Ok(packages)
}

fn parse_toml_basic_string(line: &str, key: &str) -> Result<Option<String>> {
    let prefix = format!("{key} = \"");
    let Some(rest) = line.strip_prefix(&prefix) else {
        return Ok(None);
    };
    let value = rest
        .strip_suffix('"')
        .ok_or(ObservationError::Invalid("Cargo.lock string grammar"))?;
    if value.contains(['"', '\\', '\r', '\n', '\0']) {
        return Err(ObservationError::Invalid("Cargo.lock simple string"));
    }
    Ok(Some(value.to_owned()))
}

fn finish_lock_package(package: &LockedPackage) -> Result<()> {
    if package.name.is_empty() || package.version.is_empty() {
        return Err(ObservationError::Invalid("Cargo.lock package fields"));
    }
    Ok(())
}

fn observe_cargo_feature_tree(
    root: &Path,
    cargo_path: &Path,
    rustc_path: &Path,
) -> Result<BTreeMap<(String, String), Vec<String>>> {
    let mut command = command_at(
        cargo_path,
        [
            OsString::from("tree"),
            OsString::from("--manifest-path"),
            root.join("walt/Cargo.toml").into_os_string(),
            OsString::from("--workspace"),
            OsString::from("--locked"),
            OsString::from("--target"),
            OsString::from(TARGET),
            OsString::from("-e"),
            OsString::from("normal,build"),
            OsString::from("--prefix"),
            OsString::from("none"),
            OsString::from("--format"),
            OsString::from("{p}|{f}"),
        ],
        root,
    );
    // Cargo itself was resolved through the pinned rustup toolchain, but it
    // still launches rustc as a subprocess while computing target metadata.
    // Bind that subprocess to the independently resolved absolute compiler;
    // the deliberately minimal PATH must not select a proxy or ambient tool.
    command.env("RUSTC", rustc_path);
    let output = run_checked(command, "Cargo activated-feature closure")?;
    parse_cargo_tree(exact_utf8("cargo tree output", &output.stdout)?)
}

fn parse_cargo_tree(text: &str) -> Result<BTreeMap<(String, String), Vec<String>>> {
    let mut result: BTreeMap<(String, String), Vec<String>> = BTreeMap::new();
    for raw_line in text.lines() {
        let line = raw_line.strip_suffix(" (*)").unwrap_or(raw_line);
        let Some((package, features_text)) = line.split_once('|') else {
            continue;
        };
        let Some((name, version)) = package.rsplit_once(" v") else {
            continue;
        };
        if name.is_empty() || version.is_empty() {
            return Err(ObservationError::Invalid("cargo tree package grammar"));
        }
        let mut features = if features_text.is_empty() {
            Vec::new()
        } else {
            features_text.split(',').map(str::to_owned).collect()
        };
        features.sort_unstable();
        if features.iter().any(String::is_empty)
            || features.windows(2).any(|window| window[0] == window[1])
        {
            return Err(ObservationError::Invalid(
                "cargo activated features sorted unique",
            ));
        }
        let key = (name.to_owned(), version.to_owned());
        if let Some(previous) = result.insert(key, features.clone()) {
            if previous != features {
                return Err(ObservationError::Invalid(
                    "inconsistent cargo activated-feature closure",
                ));
            }
        }
    }
    Ok(result)
}

fn build_package_records(
    locked: &[LockedPackage],
    activated: &BTreeMap<(String, String), Vec<String>>,
) -> Result<Vec<PackageRecord>> {
    let mut records = Vec::with_capacity(TOOL_PROFILES.len());
    for (name, version, expected_default, expected_features) in TOOL_PROFILES {
        let matches = locked
            .iter()
            .filter(|package| package.name == name && package.version == version)
            .collect::<Vec<_>>();
        require_usize("unique frozen lockfile package", 1, matches.len())?;
        let package = matches[0];
        require_str(
            "registry package source",
            "registry+https://github.com/rust-lang/crates.io-index",
            package
                .source
                .as_deref()
                .ok_or(ObservationError::Invalid("missing registry package source"))?,
        )?;
        let checksum_text = package
            .checksum
            .as_deref()
            .ok_or(ObservationError::Invalid("missing registry checksum"))?;
        let checksum = decode_digest(checksum_text)?;
        let features = activated
            .get(&(name.to_owned(), version.to_owned()))
            .ok_or(ObservationError::Invalid(
                "missing activated package feature closure",
            ))?
            .clone();
        let expected = expected_features
            .iter()
            .map(|feature| (*feature).to_owned())
            .collect::<Vec<_>>();
        if features != expected {
            return Err(ObservationError::Mismatch {
                field: "activated package feature closure",
                expected: format!("{name} {version}: {expected:?}"),
                actual: format!("{name} {version}: {features:?}"),
            });
        }
        let default_feature = features.iter().any(|feature| feature == "default");
        if default_feature != expected_default {
            return Err(ObservationError::Invalid(
                "activated default-feature observation",
            ));
        }
        records.push(PackageRecord {
            name: name.to_owned(),
            version: version.to_owned(),
            checksum,
            default_feature,
            activated_features: features,
        });
    }
    Ok(records)
}

fn tool_record(id: ToolId, path: &Path) -> Result<ToolRecord> {
    let bytes = fs::read(path).map_err(|source| ObservationError::Io {
        operation: "read resolved tool target",
        path: path.to_path_buf(),
        source,
    })?;
    if bytes.is_empty() {
        return Err(ObservationError::Invalid("resolved tool bytes"));
    }
    Ok(ToolRecord {
        id,
        executable_bytes: u64::try_from(bytes.len())
            .map_err(|_| ObservationError::Invalid("tool byte length"))?,
        digest: sha256(&bytes),
    })
}

fn shader_source_records(root: &Path) -> Result<Vec<SourceRecord>> {
    [
        "walt/walt-metal/shaders/00_u256.metal",
        "walt/walt-metal/shaders/01_opening_projector.metal",
    ]
    .into_iter()
    .map(|relative| {
        let bytes = read_regular(&root.join(relative))?;
        Ok(SourceRecord {
            kind: SourceKind::TranslationUnit,
            byte_length: u64::try_from(bytes.len())
                .map_err(|_| ObservationError::Invalid("shader source byte length"))?,
            digest: sha256(&bytes),
            path: relative.to_owned(),
        })
    })
    .collect()
}

fn normalized_invocations() -> Vec<InvocationRecord> {
    let common = [
        "-std=metal3.2",
        "-mmacosx-version-min=26.0",
        "-fmetal-math-mode=safe",
        "-fno-fast-math",
        "-Wall",
        "-Wextra",
        "-Werror",
        "-c",
    ];
    let mut first = common.into_iter().map(str::to_owned).collect::<Vec<_>>();
    first.extend([
        "<SOURCE_DIR>/00_u256.metal".to_owned(),
        "-o".to_owned(),
        "<AIR_DIR>/00_u256.air".to_owned(),
    ]);
    let mut second = common.into_iter().map(str::to_owned).collect::<Vec<_>>();
    second.extend([
        "<SOURCE_DIR>/01_opening_projector.metal".to_owned(),
        "-o".to_owned(),
        "<AIR_DIR>/01_opening_projector.air".to_owned(),
    ]);
    vec![
        InvocationRecord {
            kind: InvocationKind::Compile,
            source_index: 0,
            arguments: first,
        },
        InvocationRecord {
            kind: InvocationKind::Compile,
            source_index: 1,
            arguments: second,
        },
        InvocationRecord {
            kind: InvocationKind::Link,
            source_index: u32::MAX,
            arguments: vec![
                "<AIR_DIR>/00_u256.air".to_owned(),
                "<AIR_DIR>/01_opening_projector.air".to_owned(),
                "-o".to_owned(),
                "<OUTPUT>".to_owned(),
            ],
        },
    ]
}

fn require_kernel_names(metallib: &[u8]) -> Result<()> {
    for name in [ARITHMETIC_KERNEL.as_bytes(), PROJECTOR_KERNEL.as_bytes()] {
        if !metallib.windows(name.len()).any(|window| window == name) {
            return Err(ObservationError::Invalid(
                "required kernel name in committed metallib",
            ));
        }
    }
    Ok(())
}

fn verify_metallib_verifier_output(stdout: &[u8], expected: &Digest) -> Result<()> {
    let text = exact_utf8("metallib verifier output", stdout)?;
    let last = text
        .lines()
        .last()
        .ok_or(ObservationError::Invalid("metallib verifier output"))?;
    let digest_text = last
        .split_whitespace()
        .next()
        .ok_or(ObservationError::Invalid("metallib verifier digest"))?;
    let observed = decode_digest(digest_text)?;
    require_digest("two-build metallib digest", expected, &observed)
}

fn verify_pipeline(
    label: &'static str,
    pipeline: &PipelineLimits,
    profile: &DeviceProfile,
) -> Result<()> {
    if pipeline.execution_width == 0
        || pipeline.maximum_threads < THREADGROUP_WIDTH
        || pipeline.maximum_threads > profile.maximum_threads[0]
        || pipeline.static_threadgroup_memory > u64::from(profile.maximum_threadgroup_memory)
    {
        return Err(ObservationError::Invalid(label));
    }
    Ok(())
}

fn verify_allocations(allocations: &AllocationHighWater) -> Result<()> {
    require_u64(
        "projector logical allocation",
        PROJECTOR_LOGICAL_BYTES,
        allocations.projector_logical_bytes,
    )?;
    require_u64(
        "arithmetic logical allocation",
        ARITHMETIC_LOGICAL_BYTES,
        allocations.arithmetic_logical_bytes,
    )?;
    if allocations.projector_reported_bytes < allocations.projector_logical_bytes
        || allocations.arithmetic_reported_bytes < allocations.arithmetic_logical_bytes
    {
        return Err(ObservationError::Invalid(
            "driver-reported allocation range",
        ));
    }
    Ok(())
}

fn descriptor_line(lines: &mut Vec<String>, key: &str, value: &str) -> Result<()> {
    if key.is_empty()
        || key.bytes().any(|byte| {
            !(byte.is_ascii_lowercase() || byte.is_ascii_digit() || b"._-".contains(&byte))
        })
        || value.contains(['\r', '\n', '\0'])
    {
        return Err(ObservationError::Invalid("descriptor field"));
    }
    lines.push(format!("{key}={value}"));
    Ok(())
}

const fn tool_name(id: ToolId) -> &'static str {
    match id {
        ToolId::Metal => "metal",
        ToolId::Metallib => "metallib",
        ToolId::MetalAr => "metal-ar",
        ToolId::Xctrace => "xctrace",
        ToolId::Xcodebuild => "xcodebuild",
    }
}

const fn source_kind_name(kind: SourceKind) -> &'static str {
    match kind {
        SourceKind::TranslationUnit => "translation-unit",
        SourceKind::Include => "include",
    }
}

const fn invocation_kind_name(kind: InvocationKind) -> &'static str {
    match kind {
        InvocationKind::Compile => "compile",
        InvocationKind::Link => "link",
    }
}

const fn kernel_name(kernel: KernelId) -> &'static str {
    match kernel {
        KernelId::Arithmetic => "arithmetic",
        KernelId::Projector => "projector",
    }
}

fn hex_digest(digest: &Digest) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut text = String::with_capacity(64);
    for byte in digest {
        text.push(char::from(HEX[usize::from(byte >> 4)]));
        text.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    text
}

fn decode_digest(text: &str) -> Result<Digest> {
    if text.len() != 64 {
        return Err(ObservationError::Invalid("SHA-256 text length"));
    }
    let bytes = text.as_bytes();
    let mut digest = [0_u8; 32];
    for (index, output) in digest.iter_mut().enumerate() {
        let high = decode_hex_nibble(bytes[index * 2])?;
        let low = decode_hex_nibble(bytes[index * 2 + 1])?;
        *output = (high << 4) | low;
    }
    Ok(digest)
}

fn decode_hex_nibble(byte: u8) -> Result<u8> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        _ => Err(ObservationError::Invalid("lowercase SHA-256 text")),
    }
}

fn require_str(field: &'static str, expected: &str, actual: &str) -> Result<()> {
    if expected == actual {
        Ok(())
    } else {
        Err(ObservationError::Mismatch {
            field,
            expected: expected.to_owned(),
            actual: actual.to_owned(),
        })
    }
}

fn require_digest(field: &'static str, expected: &Digest, actual: &Digest) -> Result<()> {
    if expected == actual {
        Ok(())
    } else {
        Err(ObservationError::Mismatch {
            field,
            expected: hex_digest(expected),
            actual: hex_digest(actual),
        })
    }
}

fn require_usize(field: &'static str, expected: usize, actual: usize) -> Result<()> {
    if expected == actual {
        Ok(())
    } else {
        Err(ObservationError::Mismatch {
            field,
            expected: expected.to_string(),
            actual: actual.to_string(),
        })
    }
}

fn require_u64<T>(field: &'static str, expected: u64, actual: T) -> Result<()>
where
    T: TryInto<u64> + Copy,
{
    let actual = actual
        .try_into()
        .map_err(|_| ObservationError::Invalid("u64 observation conversion"))?;
    if expected == actual {
        Ok(())
    } else {
        Err(ObservationError::Mismatch {
            field,
            expected: expected.to_string(),
            actual: actual.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LOCK_FIXTURE: &str = r#"version = 4

[[package]]
name = "dispatch2"
version = "0.3.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "1e0e367e4e7da84520dedcac1901e4da967309406d1e51017ae1abfb97adbd38"

[[package]]
name = "objc2"
version = "0.6.4"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "3a12a8ed07aefc768292f076dc3ac8c48f3781c8f2d5851dd3d98950e8c5a89f"

[[package]]
name = "objc2-core-graphics"
version = "0.3.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e022c9d066895efa1345f8e33e584b9f958da2fd4cd116792e15e07e4720a807"

[[package]]
name = "objc2-foundation"
version = "0.3.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e3e0adef53c21f888deb4fa59fc59f7eb17404926ee8a6f59f5df0fd7f9f3272"

[[package]]
name = "objc2-metal"
version = "0.3.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "a0125f776a10d00af4152d74616409f0d4a2053a6f57fa5b7d6aa2854ac04794"
"#;

    const TREE_FIXTURE: &str = r#"dispatch2 v0.3.1|alloc,block2,objc2
dispatch2 v0.3.1|alloc,block2,objc2 (*)
objc2 v0.6.4|alloc,default,std
objc2-core-graphics v0.3.2|
objc2-foundation v0.3.2|NSArray,NSBundle,NSDictionary,NSEnumerator,NSError,NSObject,NSRange,NSString,NSURL,alloc,bitflags
objc2-metal v0.3.2|MTLAllocation,MTLBuffer,MTLCommandBuffer,MTLCommandEncoder,MTLCommandQueue,MTLComputeCommandEncoder,MTLComputePipeline,MTLDevice,MTLGPUAddress,MTLLibrary,MTLResource,MTLTypes,alloc,bitflags,dispatch2,std
"#;

    #[test]
    fn ambient_gate_rejects_even_empty_present_value() {
        assert!(verify_ambient_values([("RUSTFLAGS", None)]).is_ok());
        let error = verify_ambient_values([("RUSTFLAGS", Some(OsString::new()))])
            .expect_err("present empty value must fail");
        assert!(error.to_string().contains("RUSTFLAGS present"));
    }

    #[test]
    fn version_parsers_are_strict() {
        let rustc =
            parse_rustc_verbose("rustc 1.95.0\nhost: aarch64-apple-darwin\nrelease: 1.95.0\n")
                .expect("exact rustc grammar");
        assert_eq!(rustc.release, RUST_RELEASE);
        assert_eq!(rustc.host, TARGET);
        assert!(parse_rustc_verbose("release: 1.95.0\nrelease: 1.95.0\nhost: x\n").is_err());
        assert_eq!(
            parse_cargo_release("cargo 1.95.0 (revision date)\n").expect("Cargo grammar"),
            RUST_RELEASE
        );
        assert_eq!(
            parse_xcode_version("Xcode 26.6\nBuild version 17F113\n").expect("Xcode grammar"),
            (XCODE_VERSION.to_owned(), XCODE_BUILD.to_owned())
        );
        assert!(parse_xcode_version("Xcode 26.6\nBuild version 17F113\nextra\n").is_err());
    }

    #[test]
    fn lock_and_activated_feature_closure_are_independent_inputs() {
        let locked = parse_lock_packages(LOCK_FIXTURE).expect("lock parser");
        let tree = parse_cargo_tree(TREE_FIXTURE).expect("feature parser");
        let records = build_package_records(&locked, &tree).expect("exact package closure");
        assert_eq!(records.len(), 5);
        assert!(records[1].default_feature);
        assert!(!records[4].default_feature);

        let mutated =
            TREE_FIXTURE.replace("objc2 v0.6.4|alloc,default,std", "objc2 v0.6.4|alloc,std");
        let tree = parse_cargo_tree(&mutated).expect("syntactically valid mutation");
        assert!(build_package_records(&locked, &tree).is_err());

        let duplicated = format!(
            "{LOCK_FIXTURE}\n[[package]]\nname = \"objc2-metal\"\nversion = \"0.3.2\"\nsource = \"registry+https://github.com/rust-lang/crates.io-index\"\nchecksum = \"a0125f776a10d00af4152d74616409f0d4a2053a6f57fa5b7d6aa2854ac04794\"\n"
        );
        let duplicate_lock = parse_lock_packages(&duplicated).expect("valid duplicate syntax");
        let exact_tree = parse_cargo_tree(TREE_FIXTURE).expect("feature parser");
        assert!(build_package_records(&duplicate_lock, &exact_tree).is_err());
    }

    #[test]
    fn freeze_descriptor_requires_exact_fenced_census_bytes() {
        let mut census = Vec::new();
        census.extend_from_slice(b"prefix\n  ```text\n  ");
        census.extend_from_slice(FREEZE56_DESCRIPTOR);
        census.extend_from_slice(b"\n  ```\nsuffix\n");
        verify_freeze56_census_fence(&census).expect("exact fenced descriptor");
        let index = census
            .windows(FREEZE56_DESCRIPTOR.len())
            .position(|window| window == FREEZE56_DESCRIPTOR)
            .expect("descriptor offset");
        census[index] ^= 1;
        assert!(verify_freeze56_census_fence(&census).is_err());
    }

    #[test]
    fn device_observation_rejects_gate_and_allocation_mutations() {
        let profile = device_profile();
        let observed =
            device_from_values(&profile, MACOS_VERSION, MACOS_BUILD).expect("device profile");
        assert_eq!(observed.section.texts[2], DEVICE_NAME);

        let mut missing_gate = profile.clone();
        missing_gate.gate0_passed = false;
        assert!(device_from_values(&missing_gate, MACOS_VERSION, MACOS_BUILD).is_err());

        let mut short_allocation = profile;
        short_allocation.allocations.projector_reported_bytes = PROJECTOR_LOGICAL_BYTES - 1;
        assert!(device_from_values(&short_allocation, MACOS_VERSION, MACOS_BUILD).is_err());
    }

    #[test]
    fn descriptor_is_deterministic_and_contains_no_absolute_tool_path() {
        let toolchain = toolchain_observation();
        let device = device_from_values(&device_profile(), MACOS_VERSION, MACOS_BUILD)
            .expect("device profile");
        let first =
            render_host_tool_device_descriptor(&toolchain, &device).expect("descriptor render");
        let second =
            render_host_tool_device_descriptor(&toolchain, &device).expect("descriptor rerender");
        assert_eq!(first, second);
        let text = std::str::from_utf8(&first).expect("descriptor UTF-8");
        assert!(text.starts_with("W42-M2-HOST-TOOL-DEVICE-DESCRIPTOR-V1\n"));
        assert!(!text.contains("/Applications/"));
        assert!(!text.contains("/private/"));
        assert!(text.ends_with('\n'));
    }

    fn device_profile() -> DeviceProfile {
        DeviceProfile {
            sanitized_name: DEVICE_NAME.to_owned(),
            unified_memory: true,
            maximum_buffer_length: 30_150_672_384,
            recommended_working_set: 40_200_896_512,
            maximum_threads: [1_024, 1_024, 1_024],
            maximum_threadgroup_memory: 32_768,
            arithmetic_pipeline: PipelineLimits {
                execution_width: 32,
                maximum_threads: 1_024,
                static_threadgroup_memory: 0,
            },
            opening_pipeline: PipelineLimits {
                execution_width: 32,
                maximum_threads: 1_024,
                static_threadgroup_memory: 0,
            },
            allocations: AllocationHighWater {
                projector_logical_bytes: PROJECTOR_LOGICAL_BYTES,
                projector_reported_bytes: PROJECTOR_LOGICAL_BYTES,
                arithmetic_logical_bytes: ARITHMETIC_LOGICAL_BYTES,
                arithmetic_reported_bytes: ARITHMETIC_LOGICAL_BYTES,
            },
            gate0_passed: true,
        }
    }

    fn toolchain_observation() -> ToolchainObservation {
        let locked = parse_lock_packages(LOCK_FIXTURE).expect("lock parser");
        let tree = parse_cargo_tree(TREE_FIXTURE).expect("tree parser");
        let packages = build_package_records(&locked, &tree).expect("packages");
        let tools = [
            ToolId::Metal,
            ToolId::Metallib,
            ToolId::MetalAr,
            ToolId::Xctrace,
            ToolId::Xcodebuild,
        ]
        .into_iter()
        .enumerate()
        .map(|(index, id)| ToolRecord {
            id,
            executable_bytes: u64::try_from(index + 1).expect("test tool length"),
            digest: [u8::try_from(index + 1).expect("test digest byte"); 32],
        })
        .collect();
        let sources = [
            "walt/walt-metal/shaders/00_u256.metal",
            "walt/walt-metal/shaders/01_opening_projector.metal",
        ]
        .into_iter()
        .enumerate()
        .map(|(index, path)| SourceRecord {
            kind: SourceKind::TranslationUnit,
            byte_length: u64::try_from(index + 1).expect("test source length"),
            digest: [u8::try_from(index + 11).expect("test digest byte"); 32],
            path: path.to_owned(),
        })
        .collect();
        let digest = [42; 32];
        let section = ToolchainSection {
            texts: vec![
                RUST_RELEASE.to_owned(),
                TARGET.to_owned(),
                RUST_RELEASE.to_owned(),
                TARGET.to_owned(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                XCODE_VERSION.to_owned(),
                XCODE_BUILD.to_owned(),
                METAL_COMPONENT_ID.to_owned(),
                METAL_COMPONENT_BUILD.to_owned(),
                METAL_VERSION.to_owned(),
                SDK_VERSION.to_owned(),
                SDK_BUILD.to_owned(),
                DEPLOYMENT_TARGET.to_owned(),
                XCTRACE_VERSION.to_owned(),
                ARITHMETIC_KERNEL.to_owned(),
                PROJECTOR_KERNEL.to_owned(),
            ],
            packages,
            tools,
            sources,
            invocations: normalized_invocations(),
            metallib_bytes: 1,
            committed_metallib_digest: digest,
            fresh_build_1_digest: digest,
            fresh_build_2_digest: digest,
            committed_repro_digest: digest,
        };
        section.validate().expect("test toolchain section");
        ToolchainObservation { section }
    }
}
