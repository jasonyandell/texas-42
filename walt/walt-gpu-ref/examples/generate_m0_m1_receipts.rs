use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fmt::{self, Write as _};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use walt::rules::{Context, Decl, Domino, DominoSet, Pip, Seat};
use walt::spec::{sha256, SemanticTables, TABLE_FORMAT_VERSION};
use walt_gpu_ref::{
    canonical_m1_grade5_declared_stop_bytes_v1, project_closed_form,
    validate_m1_grade5_declared_stop_v1, validate_opening_run_envelope_v1, BuildIdentityV1,
    OpeningContractV1, OpeningRootV1, ReducedOpeningCarrierV1, GPU_NATIVE_TRICK1_GUIDE_V02_SHA256,
    GT1_FREEZE_SET_DESCRIPTOR_V1, GT1_FREEZE_SET_SHA256_V1, M1_DIRECT_WORLD_CAP_V1,
    M1_GRADE5_STOP_WORLD_COUNT_V1, MAX_OPENING_CELLS_V1,
};

const SOURCE_MANIFEST_BYTES: &[u8] =
    include_bytes!("../../math/gpu_native_trick1_m0_m1_sources_v1.sha256");
const PROVISIONAL_MANIFEST_HEADER: &[u8] = b"# STATUS: PROVISIONAL M0/M1 SOURCE MANIFEST V1\n";
const FINAL_MANIFEST_HEADER: &[u8] = b"# STATUS: FINAL M0/M1 SOURCE MANIFEST V1\n";

const ENVELOPE_FILE: &str = "opening_max_cell_envelope_v1.bin";
const STOP_FILE: &str = "grade5_declared_stop_v1.bin";
const SUMMARY_FILE: &str = "receipt_summary_v1.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let output_dir = output_directory()?;
    fs::create_dir_all(&output_dir)?;

    let manifest_status = source_manifest_status()?;
    let build_identity = BuildIdentityV1::new(sha256(SOURCE_MANIFEST_BYTES))?;
    let root = max_cell_root()?;
    let selected_action = domino("6-6")?;
    let led = Context::Natural(Pip::new(6).ok_or_else(|| invalid("pip 6"))?);
    if root.decl().led_context(selected_action) != led {
        return Err(invalid("max-cell action/context drift").into());
    }
    let context = root.opening_context(led)?;
    if context.matching_pool().len() != 6 {
        return Err(invalid("max-cell hidden matching count drift").into());
    }
    let projection = project_closed_form(context)?;
    if projection.cells().len() != MAX_OPENING_CELLS_V1 {
        return Err(invalid("max-cell projection cell-count drift").into());
    }
    let envelope =
        projection.canonical_run_envelope_bytes(root, selected_action, build_identity)?;
    let verified_envelope = validate_opening_run_envelope_v1(&envelope, build_identity)?;

    let grade5 = ReducedOpeningCarrierV1::from_root(root)?
        .coordinates()
        .iter()
        .copied()
        .find(|coordinate| {
            coordinate.grade() == 5 && coordinate.led() == led && coordinate.matching_count() == 6
        })
        .ok_or_else(|| invalid("grade-5 max-matching carrier coordinate missing"))?;
    let stop = canonical_m1_grade5_declared_stop_bytes_v1(root, grade5, build_identity)?;
    let verified_stop = validate_m1_grade5_declared_stop_v1(&stop, build_identity)?;

    let semantic_tables = SemanticTables::from_walt_core().canonical_bytes();
    let summary = render_summary(
        manifest_status,
        build_identity,
        root,
        selected_action,
        &envelope,
        verified_envelope,
        &stop,
        verified_stop,
        &semantic_tables,
    )?;

    fs::write(output_dir.join(ENVELOPE_FILE), envelope)?;
    fs::write(output_dir.join(STOP_FILE), stop)?;
    fs::write(output_dir.join(SUMMARY_FILE), summary.as_bytes())?;
    Ok(())
}

fn source_manifest_status() -> Result<&'static str, io::Error> {
    if SOURCE_MANIFEST_BYTES.starts_with(PROVISIONAL_MANIFEST_HEADER) {
        Ok("PROVISIONAL_SOURCE_MANIFEST")
    } else if SOURCE_MANIFEST_BYTES.starts_with(FINAL_MANIFEST_HEADER) {
        Ok("FINAL_SOURCE_MANIFEST")
    } else {
        Err(invalid("unknown source-manifest status header"))
    }
}

fn output_directory() -> Result<PathBuf, Box<dyn Error>> {
    let mut args = env::args_os();
    let program = args
        .next()
        .unwrap_or_else(|| OsString::from("generate_m0_m1_receipts"));
    let Some(output) = args.next() else {
        return Err(invalid(&format!(
            "usage: {} OUTPUT_DIRECTORY",
            Path::new(&program).display()
        ))
        .into());
    };
    if args.next().is_some() {
        return Err(invalid("expected exactly one output directory").into());
    }
    Ok(PathBuf::from(output))
}

fn max_cell_root() -> Result<OpeningRootV1, Box<dyn Error>> {
    let hand: DominoSet = ["0-0", "1-0", "1-1", "2-0", "2-1", "2-2", "6-6"]
        .into_iter()
        .map(domino)
        .collect::<Result<_, _>>()?;
    Ok(OpeningRootV1::new(
        Decl::NoTrump,
        Seat::S0,
        hand,
        OpeningContractV1::point_bid(30)?,
    )?)
}

#[allow(clippy::too_many_arguments)]
fn render_summary(
    manifest_status: &str,
    build_identity: BuildIdentityV1,
    root: OpeningRootV1,
    selected_action: Domino,
    envelope: &[u8],
    verified_envelope: walt_gpu_ref::VerifiedOpeningEnvelopeV1,
    stop: &[u8],
    verified_stop: walt_gpu_ref::VerifiedM1Grade5DeclaredStopV1,
    semantic_tables: &[u8],
) -> Result<String, fmt::Error> {
    let mut out = String::new();
    writeln!(out, "format walt-gpu-native-trick1-m0-m1-receipts-v1")?;
    writeln!(out, "status {manifest_status}")?;
    writeln!(
        out,
        "source_manifest_file math/gpu_native_trick1_m0_m1_sources_v1.sha256"
    )?;
    writeln!(out, "source_manifest_bytes {}", SOURCE_MANIFEST_BYTES.len())?;
    writeln!(
        out,
        "build_identity_sha256 {}",
        hex(&build_identity.bytes())
    )?;
    writeln!(
        out,
        "received_v0_2_sha256 {}",
        hex(&GPU_NATIVE_TRICK1_GUIDE_V02_SHA256)
    )?;
    writeln!(
        out,
        "freeze_descriptor_bytes {}",
        GT1_FREEZE_SET_DESCRIPTOR_V1.len()
    )?;
    writeln!(
        out,
        "freeze_descriptor_sha256 {}",
        hex(&GT1_FREEZE_SET_SHA256_V1)
    )?;
    writeln!(out, "semantic_table_format {TABLE_FORMAT_VERSION}")?;
    writeln!(out, "semantic_table_bytes {}", semantic_tables.len())?;
    writeln!(
        out,
        "semantic_table_sha256 {}",
        hex(&sha256(semantic_tables))
    )?;
    writeln!(out, "root_decl {}", root.decl())?;
    writeln!(out, "root_focal {}", root.focal())?;
    writeln!(out, "root_hand_bits {:08x}", root.focal_hand().bits())?;
    writeln!(out, "root_contract PointBid30")?;
    writeln!(out, "root_loss_budget {}", root.loss_budget())?;
    writeln!(out, "selected_action {selected_action}")?;
    writeln!(
        out,
        "selected_context {}",
        verified_envelope.projection_context()
    )?;
    writeln!(out, "opening_cells {MAX_OPENING_CELLS_V1}")?;
    writeln!(out, "opening_envelope_file {ENVELOPE_FILE}")?;
    writeln!(out, "opening_envelope_bytes {}", envelope.len())?;
    writeln!(out, "opening_envelope_sha256 {}", hex(&sha256(envelope)))?;
    writeln!(
        out,
        "opening_semantic_identity_sha256 {}",
        hex(&verified_envelope.semantic_identity_sha256())
    )?;
    writeln!(
        out,
        "opening_payload_bytes {}",
        verified_envelope.projector_payload_len()
    )?;
    writeln!(
        out,
        "opening_payload_sha256 {}",
        hex(&verified_envelope.projector_payload_sha256())
    )?;
    writeln!(out, "grade5_stop_file {STOP_FILE}")?;
    writeln!(out, "grade5_stop_bytes {}", stop.len())?;
    writeln!(out, "grade5_stop_sha256 {}", hex(&sha256(stop)))?;
    writeln!(
        out,
        "grade5_stop_semantic_identity_sha256 {}",
        hex(&verified_stop.semantic_identity_sha256())
    )?;
    writeln!(out, "grade5_stop_grade {}", verified_stop.grade())?;
    writeln!(out, "grade5_stop_context {}", verified_stop.led())?;
    writeln!(
        out,
        "grade5_stop_matching_count {}",
        verified_stop.matching_count()
    )?;
    writeln!(
        out,
        "grade5_stop_pool_bits {:08x}",
        verified_stop.pool().bits()
    )?;
    writeln!(
        out,
        "grade5_stop_world_count {}",
        verified_stop.world_count()
    )?;
    writeln!(out, "grade5_stop_cap {}", M1_DIRECT_WORLD_CAP_V1)?;
    writeln!(
        out,
        "grade5_stop_emitted_worlds {}",
        verified_stop.emitted_worlds()
    )?;
    writeln!(
        out,
        "grade5_stop_emitted_cells {}",
        verified_stop.emitted_cells()
    )?;
    writeln!(
        out,
        "grade5_stop_payload_bytes {}",
        verified_stop.payload_len()
    )?;
    if verified_stop.world_count() != M1_GRADE5_STOP_WORLD_COUNT_V1 {
        return Err(fmt::Error);
    }
    Ok(out)
}

fn domino(name: &str) -> Result<Domino, io::Error> {
    name.parse().map_err(|_| invalid("invalid fixed domino"))
}

fn invalid(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

fn hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        write!(out, "{byte:02x}").expect("writing to String is infallible");
    }
    out
}
