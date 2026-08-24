use walt::rules::{Context, Decl, Domino, DominoSet, Pip, Seat};
use walt::spec::{sha256, SemanticTables};
use walt_gpu_ref::{
    canonical_m1_grade5_declared_stop_bytes_v1, project_closed_form,
    validate_m1_grade5_declared_stop_v1, validate_opening_run_envelope_v1, BuildIdentityV1,
    OpeningContractV1, OpeningEnvelopeError, OpeningRootV1, ReducedOpeningCarrierV1,
    GPU_NATIVE_TRICK1_GUIDE_V02_SHA256, GT1_FREEZE_SET_DESCRIPTOR_V1, GT1_FREEZE_SET_SHA256_V1,
    M1_GRADE5_STOP_HEADER_BYTES, M1_GRADE5_STOP_SEMANTIC_IDENTITY_SHA256_OFFSET,
    M1_GRADE5_STOP_WORLD_COUNT_V1, OPENING_ENVELOPE_HEADER_BYTES,
    OPENING_ENVELOPE_SEMANTIC_IDENTITY_SHA256_OFFSET, OPENING_RECEIPT_CELL_BYTES,
    OPENING_RECEIPT_HEADER_BYTES, OPENING_ROOT_KEY_BYTES,
};

const BUILD_A_BYTES: [u8; 32] = [0xa5; 32];
const BUILD_B_BYTES: [u8; 32] = [0x5a; 32];

fn hand(names: [&str; 7]) -> DominoSet {
    names
        .into_iter()
        .map(|name| name.parse::<Domino>().expect("explicit root tile"))
        .collect()
}

fn fixture_root() -> OpeningRootV1 {
    OpeningRootV1::new(
        Decl::NoTrump,
        Seat::S0,
        hand(["6-0", "6-1", "6-2", "6-3", "6-4", "6-5", "5-5"]),
        OpeningContractV1::point_bid(30).expect("point contract"),
    )
    .expect("fixture root")
}

fn build_a() -> BuildIdentityV1 {
    BuildIdentityV1::new(BUILD_A_BYTES).expect("nonzero build A")
}

fn build_b() -> BuildIdentityV1 {
    BuildIdentityV1::new(BUILD_B_BYTES).expect("nonzero build B")
}

fn action(name: &str) -> Domino {
    name.parse().expect("explicit action")
}

fn grade5_coordinate() -> walt_gpu_ref::ReducedOpeningCoordinateV1 {
    ReducedOpeningCarrierV1::from_root(fixture_root())
        .expect("fixture carrier")
        .coordinates()
        .iter()
        .copied()
        .find(|coordinate| coordinate.grade() == 5)
        .expect("grade-5 coordinate")
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[test]
fn frozen_descriptor_guide_and_table_identities_are_exact() {
    assert_eq!(GT1_FREEZE_SET_DESCRIPTOR_V1.len(), 944);
    assert_eq!(
        sha256(GT1_FREEZE_SET_DESCRIPTOR_V1),
        GT1_FREEZE_SET_SHA256_V1
    );
    let guide = include_bytes!("../../math/gpu_native_trick1_implementers_guide_v0.2.md");
    assert_eq!(guide.len(), 82_740);
    assert_eq!(sha256(guide), GPU_NATIVE_TRICK1_GUIDE_V02_SHA256);
    let tables = SemanticTables::from_walt_core().canonical_bytes();
    assert_eq!(tables.len(), 14_884);
    assert_eq!(
        hex(&sha256(&tables)),
        "6595fadb8c2acf174f106700c658a99a8a66b7d63c5349ddda81e7abd208b66b"
    );
}

#[test]
fn opening_envelope_is_bound_replayable_and_action_distinct() {
    let root = fixture_root();
    let led = Context::Natural(Pip::new(6).expect("six"));
    let first_projection = project_closed_form(root.opening_context(led).expect("root context"))
        .expect("first projection");
    let second_projection = project_closed_form(root.opening_context(led).expect("root context"))
        .expect("second projection");

    let action_60 = action("6-0");
    let action_61 = action("6-1");
    let first = first_projection
        .canonical_run_envelope_bytes(root, action_60, build_a())
        .expect("first envelope");
    let fresh = second_projection
        .canonical_run_envelope_bytes(root, action_60, build_a())
        .expect("fresh envelope");
    let same_context_other_action = first_projection
        .canonical_run_envelope_bytes(root, action_61, build_a())
        .expect("same-context envelope");

    assert_eq!(first, fresh);
    assert_ne!(first, same_context_other_action);
    let verified = validate_opening_run_envelope_v1(&first, build_a()).expect("bound envelope");
    assert_eq!(verified.root(), root);
    assert_eq!(verified.selected_action(), action_60);
    assert_eq!(verified.projection_context(), led);
    assert_eq!(verified.projector_payload_len(), 1_140 * 26 + 50);

    let table_len = SemanticTables::canonical_byte_len();
    let payload_offset = OPENING_ENVELOPE_HEADER_BYTES
        + GT1_FREEZE_SET_DESCRIPTOR_V1.len()
        + OPENING_ROOT_KEY_BYTES
        + table_len;
    assert_eq!(
        &first[payload_offset..],
        &same_context_other_action[payload_offset..]
    );
    assert_eq!(
        verified.projector_payload_len() as usize,
        OPENING_RECEIPT_HEADER_BYTES + 1_140 * OPENING_RECEIPT_CELL_BYTES
    );
    assert_eq!(
        first.len(),
        payload_offset + verified.projector_payload_len() as usize
    );

    let semantic_digest = &first[OPENING_ENVELOPE_SEMANTIC_IDENTITY_SHA256_OFFSET
        ..OPENING_ENVELOPE_SEMANTIC_IDENTITY_SHA256_OFFSET + 32];
    assert_eq!(semantic_digest, verified.semantic_identity_sha256());
    assert_eq!(
        hex(&verified.semantic_identity_sha256()),
        "d1cad0e0aeab18a962b2f818e4e4ea4b1767b67b0ab6ed0b842376e85501344c"
    );
    assert_eq!(
        hex(&verified.projector_payload_sha256()),
        "65b42e237c2c2c251fb38ad6a9faa2eb6584ac706d9174950ce64cd1a501928a"
    );
    assert_eq!(first.len(), 45_827);
    assert_eq!(
        hex(&sha256(&first)),
        "e8f9b5fd8cb9c0345004b4aaf9da06d6a859420438290425697cc20a61338705"
    );
}

#[test]
fn opening_envelope_generation_and_load_fail_closed() {
    assert_eq!(
        BuildIdentityV1::new([0; 32]),
        Err(OpeningEnvelopeError::ZeroBuildIdentity)
    );

    let root = fixture_root();
    let led = Context::Natural(Pip::new(6).expect("six"));
    let projection =
        project_closed_form(root.opening_context(led).expect("root context")).expect("projection");
    assert!(matches!(
        projection.canonical_run_envelope_bytes(root, action("0-0"), build_a()),
        Err(OpeningEnvelopeError::IllegalRootAction { .. })
    ));
    assert!(matches!(
        projection.canonical_run_envelope_bytes(root, action("5-5"), build_a()),
        Err(OpeningEnvelopeError::ActionContextMismatch { .. })
    ));

    let reduced = grade5_coordinate()
        .opening_context()
        .expect("reduced context");
    let reduced_projection = project_closed_form(reduced).expect("reduced projection");
    assert_eq!(
        reduced_projection.canonical_run_envelope_bytes(root, action("6-0"), build_a()),
        Err(OpeningEnvelopeError::RootProjectionMismatch)
    );

    let envelope = projection
        .canonical_run_envelope_bytes(root, action("6-0"), build_a())
        .expect("envelope");
    assert_eq!(
        validate_opening_run_envelope_v1(&envelope, build_b()),
        Err(OpeningEnvelopeError::BuildIdentityMismatch)
    );
    let other_build = projection
        .canonical_run_envelope_bytes(root, action("6-0"), build_b())
        .expect("self-consistent other-build envelope");
    assert_eq!(
        validate_opening_run_envelope_v1(&other_build, build_a()),
        Err(OpeningEnvelopeError::BuildIdentityMismatch)
    );

    let mut zero_build = envelope.clone();
    zero_build[32..64].fill(0);
    assert_eq!(
        validate_opening_run_envelope_v1(&zero_build, build_a()),
        Err(OpeningEnvelopeError::ZeroBuildIdentity)
    );
    let mut unknown_version = envelope.clone();
    unknown_version[8] = 2;
    assert!(matches!(
        validate_opening_run_envelope_v1(&unknown_version, build_a()),
        Err(OpeningEnvelopeError::UnknownIdentity("envelope version"))
    ));
    let mut corrupt_payload = envelope.clone();
    *corrupt_payload.last_mut().expect("payload byte") ^= 1;
    assert!(matches!(
        validate_opening_run_envelope_v1(&corrupt_payload, build_a()),
        Err(OpeningEnvelopeError::UnknownIdentity(
            "projector payload SHA-256"
        ))
    ));
    assert!(matches!(
        validate_opening_run_envelope_v1(&envelope[..OPENING_ENVELOPE_HEADER_BYTES - 1], build_a()),
        Err(OpeningEnvelopeError::Truncated("envelope header"))
    ));
}

#[test]
fn grade5_declared_stop_is_exact_bound_and_has_no_partial_output() {
    let root = fixture_root();
    let coordinate = grade5_coordinate();
    let first = canonical_m1_grade5_declared_stop_bytes_v1(root, coordinate, build_a())
        .expect("grade-5 declared stop");
    let fresh = canonical_m1_grade5_declared_stop_bytes_v1(root, coordinate, build_a())
        .expect("fresh grade-5 declared stop");
    assert_eq!(first, fresh);
    let verified = validate_m1_grade5_declared_stop_v1(&first, build_a()).expect("validated stop");
    assert_eq!(verified.root(), root);
    assert_eq!(verified.grade(), 5);
    assert_eq!(verified.world_count(), M1_GRADE5_STOP_WORLD_COUNT_V1);
    assert_eq!(verified.emitted_worlds(), 0);
    assert_eq!(verified.emitted_cells(), 0);
    assert_eq!(verified.payload_len(), 0);
    assert_eq!(verified.led(), coordinate.led());
    assert_eq!(verified.matching_count(), coordinate.matching_count());
    assert_eq!(verified.pool(), coordinate.pool());
    assert_eq!(
        &first[M1_GRADE5_STOP_SEMANTIC_IDENTITY_SHA256_OFFSET
            ..M1_GRADE5_STOP_SEMANTIC_IDENTITY_SHA256_OFFSET + 32],
        verified.semantic_identity_sha256()
    );
    assert_eq!(
        first.len(),
        M1_GRADE5_STOP_HEADER_BYTES
            + GT1_FREEZE_SET_DESCRIPTOR_V1.len()
            + OPENING_ROOT_KEY_BYTES
            + SemanticTables::canonical_byte_len()
    );
    assert_eq!(first.len(), 16_177);
    assert_eq!(
        hex(&verified.semantic_identity_sha256()),
        "ab824e6e202e2133c2fe1ff81c488dfdca0a21de85498b598c1b760db3f57b4b"
    );
    assert_eq!(
        hex(&sha256(&first)),
        "72ccde05e0dd0cc6296637adbbc67ff684a8f35e8d0516b3574840697dfca020"
    );
}

#[test]
fn grade5_declared_stop_rejects_corruption_partial_output_and_other_build() {
    let root = fixture_root();
    let coordinate = grade5_coordinate();
    let stop = canonical_m1_grade5_declared_stop_bytes_v1(root, coordinate, build_a())
        .expect("grade-5 declared stop");

    let other_build = canonical_m1_grade5_declared_stop_bytes_v1(root, coordinate, build_b())
        .expect("self-consistent other-build stop");
    assert_eq!(
        validate_m1_grade5_declared_stop_v1(&other_build, build_a()),
        Err(OpeningEnvelopeError::BuildIdentityMismatch)
    );

    for offset in [280usize, 288, 292] {
        let mut partial = stop.clone();
        partial[offset] = 1;
        assert_eq!(
            validate_m1_grade5_declared_stop_v1(&partial, build_a()),
            Err(OpeningEnvelopeError::PartialStopOutput)
        );
    }
    let mut corrupt_world_count = stop.clone();
    corrupt_world_count[264] ^= 1;
    assert!(matches!(
        validate_m1_grade5_declared_stop_v1(&corrupt_world_count, build_a()),
        Err(OpeningEnvelopeError::UnknownIdentity("grade-5 world count"))
    ));
    let mut corrupt_table = stop.clone();
    *corrupt_table.last_mut().expect("table byte") ^= 1;
    assert!(matches!(
        validate_m1_grade5_declared_stop_v1(&corrupt_table, build_a()),
        Err(OpeningEnvelopeError::FrozenIdentityMismatch(
            "semantic table bytes"
        ))
    ));
    assert!(matches!(
        validate_m1_grade5_declared_stop_v1(&stop[..M1_GRADE5_STOP_HEADER_BYTES - 1], build_a()),
        Err(OpeningEnvelopeError::Truncated("grade-5 stop header"))
    ));
}
