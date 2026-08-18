use walt_gpu_ref::m2_receipt::*;
use walt_gpu_ref::M2OpeningParityCarrierV1;

fn repeated(byte: u8) -> Digest {
    [byte; 32]
}

fn hex_digest(value: &str) -> Digest {
    assert_eq!(value.len(), 64);
    let mut out = [0; 32];
    for (index, chunk) in value.as_bytes().chunks_exact(2).enumerate() {
        let nibble = |byte: u8| match byte {
            b'0'..=b'9' => byte - b'0',
            b'a'..=b'f' => byte - b'a' + 10,
            _ => panic!("bad test hex"),
        };
        out[index] = (nibble(chunk[0]) << 4) | nibble(chunk[1]);
    }
    out
}

fn canonical_root_key() -> [u8; 37] {
    let mut root = [0; 37];
    root[..8].copy_from_slice(b"W42RTK01");
    root[8..10].copy_from_slice(&1u16.to_le_bytes());
    root[10] = 8;
    root[11..15].fill(0);
    let hand = 0x0fe0_0000u32; // Physical indices 21..27.
    assert_eq!(hand.count_ones(), 7);
    root[15..19].copy_from_slice(&hand.to_le_bytes());
    root[19] = 7;
    root[20] = 1;
    root[21] = 30;
    root[22] = 12;
    for (offset, value) in [(23, 1u16), (25, 1), (27, 1), (29, 2), (31, 1)] {
        root[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
    }
    root[33] = 1;
    validate_root_key(&root).unwrap();
    root
}

fn invocation_arguments(source: &str, air: &str) -> Vec<String> {
    [
        "-std=metal3.2",
        "-mmacosx-version-min=26.0",
        "-fmetal-math-mode=safe",
        "-fno-fast-math",
        "-Wall",
        "-Wextra",
        "-Werror",
        "-c",
        source,
        "-o",
        air,
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

fn package(name: &str, version: &str, default_feature: bool, features: &[&str]) -> PackageRecord {
    let mut activated_features: Vec<String> =
        features.iter().map(|value| (*value).to_owned()).collect();
    activated_features.sort();
    PackageRecord {
        name: name.to_owned(),
        version: version.to_owned(),
        checksum: hex_digest(match name {
            "dispatch2" => "1e0e367e4e7da84520dedcac1901e4da967309406d1e51017ae1abfb97adbd38",
            "objc2" => "3a12a8ed07aefc768292f076dc3ac8c48f3781c8f2d5851dd3d98950e8c5a89f",
            "objc2-core-graphics" => {
                "e022c9d066895efa1345f8e33e584b9f958da2fd4cd116792e15e07e4720a807"
            }
            "objc2-foundation" => {
                "e3e0adef53c21f888deb4fa59fc59f7eb17404926ee8a6f59f5df0fd7f9f3272"
            }
            "objc2-metal" => "a0125f776a10d00af4152d74616409f0d4a2053a6f57fa5b7d6aa2854ac04794",
            _ => panic!("unknown test package"),
        }),
        default_feature,
        activated_features,
    }
}

fn authority(build_identity: Digest) -> AuthoritySection {
    let static_hashes = [
        "eccf0a3742e2cfc50cad158292db7ad8c6145da8aa7958b7aa2ed07a1566f2ad",
        "9b181092045b003893cae7c09cc7b7c8b57f75c3c5c4cf7043b8d428df738efa",
        "ee2e78da20eb7d087fb121f467a56bafc0179a45fb692ca0b938f4c4210b6a44",
        "6190e740a0579b6b5196e086e52c8022d4cddcd0f746ecbd9226f87bbc0e4790",
        "1127d3868d7da07c26a7b8bc031ac8a63ba84a9068df786b67a413ea6af5f517",
        "7e8dfecf1cac314ae6e71b406eb268b29d4157206ce5e64d1c50d1aa94d43bdf",
        "51a162ea933801f05b852ec2a454c48a31c7d292ee8273ba683d0a7fec340b12",
        "b57f7077e5aa0aa1d8030a76a3399076810b71b1623ad83e001aee2b4aaeb215",
    ];
    let lengths = [
        18_750,
        944,
        82_740,
        29_607,
        321_167,
        16_177,
        1_644,
        1_171,
        777,
        CONTRACT_BYTES,
        999,
        899,
        921_481,
    ];
    let mut identities = Vec::with_capacity(13);
    for index in 0..13usize {
        let tag = ArtifactTag::try_from(u32::try_from(index + 1).unwrap()).unwrap();
        let digest = match index {
            0..=7 => hex_digest(static_hashes[index]),
            8 => build_identity,
            9 => CONTRACT_SHA256,
            10 => repeated(0xc1),
            11 => FREEZE56_DESCRIPTOR_SHA256,
            12 => PARENT_CENSUS_SHA256,
            _ => unreachable!(),
        };
        identities.push(ArtifactIdentity {
            tag,
            byte_length: lengths[index],
            digest,
        });
    }
    AuthoritySection {
        parent_commit: PARENT_COMMIT_SHA1,
        identities,
        freeze56_descriptor: FREEZE56_DESCRIPTOR.to_vec(),
    }
}

fn sample_receipt() -> SuccessReceipt {
    let build_identity = repeated(0xb1);
    let CanonicalScalarObservationSectionsV1 {
        arithmetic,
        context_tasks,
        global_observation_digests,
    } = canonical_scalar_observation_sections_v1().unwrap();
    let (reduced_bindings, physical_bindings) = canonical_binding_sections(&context_tasks).unwrap();

    let metallib_digest = repeated(0x77);
    let mut receipt = SuccessReceipt {
        build_identity,
        freeze56_descriptor_digest: FREEZE56_DESCRIPTOR_SHA256,
        sections: ReceiptSections {
            authority: authority(build_identity),
            toolchain: ToolchainSection {
                texts: vec![
                    "1.95.0",
                    "aarch64-apple-darwin",
                    "1.95.0",
                    "aarch64-apple-darwin",
                    "",
                    "",
                    "",
                    "",
                    "26.6",
                    "17F113",
                    "com.apple.dt.toolchain.Metal.32023.883",
                    "17F109",
                    "32023.883",
                    "26.5",
                    "25F70",
                    "26.0",
                    "16.0 (17F113)",
                    "u256_parity_v1",
                    "opening_project_v1",
                ]
                .into_iter()
                .map(str::to_owned)
                .collect(),
                packages: vec![
                    package("dispatch2", "0.3.1", false, &["alloc", "block2", "objc2"]),
                    package("objc2", "0.6.4", true, &["alloc", "default", "std"]),
                    package("objc2-core-graphics", "0.3.2", false, &[]),
                    package(
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
                    package(
                        "objc2-metal",
                        "0.3.2",
                        false,
                        &[
                            "std",
                            "dispatch2",
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
                        ],
                    ),
                ],
                tools: (1..=5u32)
                    .map(|id| ToolRecord {
                        id: ToolId::try_from(id).unwrap(),
                        executable_bytes: 1_000 + u64::from(id),
                        digest: repeated(u8::try_from(id).unwrap()),
                    })
                    .collect(),
                sources: vec![
                    SourceRecord {
                        kind: SourceKind::TranslationUnit,
                        byte_length: 111,
                        digest: repeated(0x31),
                        path: "walt/walt-metal/shaders/00_u256.metal".to_owned(),
                    },
                    SourceRecord {
                        kind: SourceKind::TranslationUnit,
                        byte_length: 222,
                        digest: repeated(0x32),
                        path: "walt/walt-metal/shaders/01_opening_projector.metal".to_owned(),
                    },
                ],
                invocations: vec![
                    InvocationRecord {
                        kind: InvocationKind::Compile,
                        source_index: 0,
                        arguments: invocation_arguments(
                            "<SOURCE_DIR>/00_u256.metal",
                            "<AIR_DIR>/00_u256.air",
                        ),
                    },
                    InvocationRecord {
                        kind: InvocationKind::Compile,
                        source_index: 1,
                        arguments: invocation_arguments(
                            "<SOURCE_DIR>/01_opening_projector.metal",
                            "<AIR_DIR>/01_opening_projector.air",
                        ),
                    },
                    InvocationRecord {
                        kind: InvocationKind::Link,
                        source_index: u32::MAX,
                        arguments: [
                            "<AIR_DIR>/00_u256.air",
                            "<AIR_DIR>/01_opening_projector.air",
                            "-o",
                            "<OUTPUT>",
                        ]
                        .into_iter()
                        .map(str::to_owned)
                        .collect(),
                    },
                ],
                metallib_bytes: 9_999,
                committed_metallib_digest: metallib_digest,
                fresh_build_1_digest: metallib_digest,
                fresh_build_2_digest: metallib_digest,
                committed_repro_digest: metallib_digest,
            },
            device: DeviceSection {
                texts: ["26.6", "25G1", "Apple M5 Max"]
                    .into_iter()
                    .map(str::to_owned)
                    .collect(),
                max_buffer_length: 1 << 30,
                recommended_working_set: 1 << 29,
                max_threads: [1_024, 1_024, 64],
                max_threadgroup_memory: 32_768,
                pipelines: vec![
                    PipelineRecord {
                        kernel: KernelId::Arithmetic,
                        execution_width: 32,
                        maximum_threads: 1_024,
                        static_group_memory: 0,
                    },
                    PipelineRecord {
                        kernel: KernelId::Projector,
                        execution_width: 32,
                        maximum_threads: 1_024,
                        static_group_memory: 0,
                    },
                ],
            },
            tables_and_abi: TablesAndAbiSection {
                semantic_table: TableRecord {
                    tag: 1,
                    format_version: 2,
                    rows: 0,
                    columns: 0,
                    byte_length: SEMANTIC_TABLE_BYTES,
                    digest: SEMANTIC_TABLE_SHA256,
                },
                choose_table: TableRecord {
                    tag: 2,
                    format_version: 1,
                    rows: 22,
                    columns: 22,
                    byte_length: 1_936,
                    digest: CHOOSE_TABLE_SHA256,
                },
            },
            arithmetic,
            carrier: CarrierSection {
                accepted_payload_bytes: 0,
                task_key_stream_digest: ZERO_DIGEST,
                task_input_hash_chain_digest: repeated(0x51),
                choose_input_hash_chain_digest: repeated(0x52),
            },
            context_tasks,
            reduced_bindings,
            physical_bindings,
            global: GlobalSection {
                digests: [
                    global_observation_digests[0],
                    global_observation_digests[1],
                    global_observation_digests[2],
                    global_observation_digests[3],
                    global_observation_digests[4],
                    global_observation_digests[5],
                    global_observation_digests[6],
                    ZERO_DIGEST,
                    ZERO_DIGEST,
                    ZERO_DIGEST,
                ],
            },
        },
    };
    receipt.canonicalize().unwrap();
    receipt.validate().unwrap();
    receipt
}

#[test]
fn frozen_constants_and_all_fixed_layouts() {
    assert_eq!(CONTRACT_BYTES, 46_133);
    assert_eq!(sha256(FREEZE56_DESCRIPTOR), FREEZE56_DESCRIPTOR_SHA256);
    assert_eq!(SUCCESS_CLAIM.len(), 50);
    assert_eq!(SUCCESS_HEADER_BYTES, 128 + 10 * DIRECTORY_ENTRY_BYTES);
    assert_eq!(AUTHORITY_PREFIX_BYTES, 48);
    assert_eq!(TOOLCHAIN_PREFIX_BYTES, 80);
    assert_eq!(DEVICE_PREFIX_BYTES, 56);
    assert_eq!(TABLES_AND_ABI_BYTES, 72 + 2 * TABLE_RECORD_BYTES);
    assert_eq!(ARITHMETIC_BYTES, 48 + 2 * ARITHMETIC_RUN_RECORD_BYTES);
    assert_eq!(CARRIER_BYTES, 160);
    assert_eq!(CONTEXT_TASK_RECORD_BYTES, 384);
    assert_eq!(REDUCED_BINDING_RECORD_BYTES, 160);
    assert_eq!(PHYSICAL_BINDING_RECORD_BYTES, 160);
    assert_eq!(GLOBAL_BYTES, 64 + 10 * 32 + 4 + 50);
    assert_eq!(FAILURE_BYTES, 256);
    assert_eq!(SMOKE_BYTES, 32);
    assert_eq!(PROTECTED_CHAIN_RECORD_BYTES, 48);
    assert_eq!(RESPONSE_AGGREGATE_RECORD_BYTES, 32);

    let carrier = M2OpeningParityCarrierV1::canonical().unwrap();
    let key = TaskKey::decode(&carrier.tasks()[64].task_key().to_le_bytes()).unwrap();
    let encoded = key.encode();
    assert_eq!(&encoded[0..4], &1u32.to_le_bytes());
    assert_eq!(&encoded[4..8], &64u32.to_le_bytes());
    assert_eq!(&encoded[8..12], &2u32.to_le_bytes());
    assert_eq!(&encoded[60..64], &0u32.to_le_bytes());
    assert_eq!(TaskKey::decode(&encoded).unwrap(), key);

    let root = canonical_root_key();
    assert_eq!(&root[0..8], b"W42RTK01");
    assert_eq!(&root[8..10], &1u16.to_le_bytes());
    assert_eq!(root[36], 0);
}

#[test]
fn digest_domains_match_explicit_preimages() {
    let payload = b"three exact records";
    let mut stream_preimage = Vec::new();
    stream_preimage.extend_from_slice(b"W42M2DG1");
    stream_preimage.extend_from_slice(&15u32.to_le_bytes());
    stream_preimage.extend_from_slice(&1u32.to_le_bytes());
    stream_preimage.extend_from_slice(&3u64.to_le_bytes());
    stream_preimage.extend_from_slice(&u64::try_from(payload.len()).unwrap().to_le_bytes());
    stream_preimage.extend_from_slice(payload);
    assert_eq!(
        stream_digest(StreamPurpose::GlobalProtectedChain, 3, payload).unwrap(),
        sha256(&stream_preimage)
    );

    let mut section_preimage = Vec::new();
    section_preimage.extend_from_slice(b"W42M2SC1");
    section_preimage.extend_from_slice(&7u32.to_le_bytes());
    section_preimage.extend_from_slice(&1u32.to_le_bytes());
    section_preimage.extend_from_slice(&614u64.to_le_bytes());
    section_preimage.extend_from_slice(&u64::try_from(payload.len()).unwrap().to_le_bytes());
    section_preimage.extend_from_slice(payload);
    assert_eq!(
        section_digest(SectionTag::ContextTasks, 614, payload).unwrap(),
        sha256(&section_preimage)
    );
    assert_ne!(
        stream_digest(StreamPurpose::GlobalProtectedChain, 3, payload).unwrap(),
        stream_digest(StreamPurpose::ContextPayload, 3, payload).unwrap()
    );
}

#[test]
fn complete_success_roundtrip_and_directory_are_canonical() {
    let receipt = sample_receipt();
    let bytes = receipt.encode().unwrap();
    let decoded = SuccessReceipt::decode(&bytes).unwrap();
    assert_eq!(decoded, receipt);
    assert_eq!(decoded.encode().unwrap(), bytes);
    assert_eq!(&bytes[..8], &SUCCESS_MAGIC);
    assert_eq!(
        u64::from_le_bytes(bytes[16..24].try_into().unwrap()) as usize,
        bytes.len()
    );

    let mut expected_offset = SUCCESS_HEADER_BYTES as u64;
    for index in 0..10usize {
        let start = 128 + index * DIRECTORY_ENTRY_BYTES;
        let entry =
            SectionDirectoryEntry::decode(&bytes[start..start + DIRECTORY_ENTRY_BYTES]).unwrap();
        assert_eq!(u16::from(entry.tag), u16::try_from(index + 1).unwrap());
        assert_eq!(entry.offset, expected_offset);
        let section_start = usize::try_from(entry.offset).unwrap();
        let section_end = section_start + usize::try_from(entry.length).unwrap();
        assert_eq!(
            entry.digest,
            section_digest(
                entry.tag,
                entry.record_count,
                &bytes[section_start..section_end]
            )
            .unwrap()
        );
        expected_offset += entry.length;
    }
    assert_eq!(usize::try_from(expected_offset).unwrap(), bytes.len());
}

#[test]
fn canonical_input_hash_chains_are_independently_validated() {
    let mut task_changed = sample_receipt();
    task_changed.sections.carrier.task_input_hash_chain_digest[0] ^= 1;
    let task_error = task_changed.validate().unwrap_err().to_string();
    assert!(task_error.contains("canonical task input hash-chain digest"));

    let mut choose_changed = sample_receipt();
    choose_changed
        .sections
        .carrier
        .choose_input_hash_chain_digest[0] ^= 1;
    let choose_error = choose_changed.validate().unwrap_err().to_string();
    assert!(choose_error.contains("canonical choose input hash-chain digest"));
}

#[test]
fn canonical_scalar_oracle_rejects_equal_but_fabricated_observations() {
    let canonical = canonical_scalar_observation_sections_v1().unwrap();
    assert_eq!(canonical.arithmetic.official.success_count, 8_141);
    assert_eq!(canonical.arithmetic.official.checked_undefined_count, 8_243);
    assert_eq!(canonical.arithmetic.negative.hard_count, 13);
    assert_eq!(canonical.arithmetic.negative.allocated_input_bytes, 1_040);
    assert_eq!(canonical.arithmetic.negative.allocated_output_bytes, 960);

    let mut arithmetic_pair = sample_receipt();
    arithmetic_pair
        .sections
        .arithmetic
        .official
        .input_pre_digest[0] ^= 1;
    arithmetic_pair
        .sections
        .arithmetic
        .official
        .input_post_digest[0] ^= 1;
    assert!(arithmetic_pair.canonicalize().is_err());
    assert!(arithmetic_pair.validate().is_err());

    let mut arithmetic_allocation = sample_receipt();
    arithmetic_allocation
        .sections
        .arithmetic
        .negative
        .allocated_input_bytes += 80;
    assert!(arithmetic_allocation.canonicalize().is_err());
    assert!(arithmetic_allocation.validate().is_err());

    let mut context_pair = sample_receipt();
    context_pair.sections.context_tasks.records[0].cpu_slot_digest[0] ^= 1;
    context_pair.sections.context_tasks.records[0].gpu_slot_digest[0] ^= 1;
    context_pair.canonicalize().unwrap();
    assert!(context_pair.validate().is_err());

    let mut context_mass = sample_receipt();
    context_mass.sections.context_tasks.records[0].total_scaled_mass[0] ^= 1;
    context_mass.canonicalize().unwrap();
    assert!(context_mass.validate().is_err());

    let mut global_pair = sample_receipt();
    global_pair.sections.global.digests[0][0] ^= 1;
    global_pair.sections.global.digests[1][0] ^= 1;
    global_pair.canonicalize().unwrap();
    assert!(global_pair.validate().is_err());

    let mut protected_chain = sample_receipt();
    protected_chain.sections.global.digests[6][0] ^= 1;
    protected_chain.canonicalize().unwrap();
    assert!(protected_chain.validate().is_err());
}

#[test]
fn success_parser_rejects_header_directory_section_and_truncation_mutations() {
    let bytes = sample_receipt().encode().unwrap();
    let header_offsets = [0, 8, 10, 12, 16, 24, 28, 32, 48, 52, 56, 64, 96];
    for offset in header_offsets {
        let mut changed = bytes.clone();
        changed[offset] ^= 1;
        assert!(
            SuccessReceipt::decode(&changed).is_err(),
            "accepted header mutation at {offset}"
        );
    }
    let directory_offsets = [128, 130, 132, 136, 144, 152, 160];
    for offset in directory_offsets {
        let mut changed = bytes.clone();
        changed[offset] ^= 1;
        assert!(
            SuccessReceipt::decode(&changed).is_err(),
            "accepted directory mutation at {offset}"
        );
    }
    for index in 0..10usize {
        let entry_offset = 128 + index * DIRECTORY_ENTRY_BYTES;
        let entry = SectionDirectoryEntry::decode(
            &bytes[entry_offset..entry_offset + DIRECTORY_ENTRY_BYTES],
        )
        .unwrap();
        let mut changed = bytes.clone();
        changed[usize::try_from(entry.offset).unwrap()] ^= 1;
        assert!(
            SuccessReceipt::decode(&changed).is_err(),
            "accepted section-{index} mutation"
        );
    }
    for len in [0, 1, 767, 768, bytes.len() - 1] {
        assert!(
            SuccessReceipt::decode(&bytes[..len]).is_err(),
            "accepted truncation {len}"
        );
    }
    let mut trailing = bytes.clone();
    trailing.push(0);
    assert!(SuccessReceipt::decode(&trailing).is_err());
}

#[test]
fn cross_section_binding_and_authority_mutations_fail_after_recanonicalization() {
    let mut receipt = sample_receipt();
    receipt.sections.reduced_bindings.records[0].derived_context ^= 1;
    receipt.canonicalize().unwrap();
    assert!(receipt.validate().is_err());

    let mut receipt = sample_receipt();
    receipt.sections.physical_bindings.records[0].derived_context ^= 1;
    receipt.canonicalize().unwrap();
    assert!(receipt.validate().is_err());

    let mut receipt = sample_receipt();
    receipt.sections.reduced_bindings.records[0].selected_action ^= 1;
    receipt.canonicalize().unwrap();
    assert!(receipt.validate().is_err());

    let mut receipt = sample_receipt();
    receipt.sections.reduced_bindings.records[0].root_key[10] =
        (receipt.sections.reduced_bindings.records[0].root_key[10] + 1) % 9;
    receipt.canonicalize().unwrap();
    assert!(receipt.validate().is_err());

    let mut receipt = sample_receipt();
    receipt.sections.physical_bindings.records[8].selected_action ^= 1;
    receipt.canonicalize().unwrap();
    assert!(receipt.validate().is_err());

    let mut receipt = sample_receipt();
    receipt.sections.context_tasks.records[64].key.matching_mask = 0x7f;
    assert!(receipt.canonicalize().is_err());

    let mut receipt = sample_receipt();
    receipt.sections.context_tasks.records[64].key.generator_c ^= 1;
    receipt.canonicalize().unwrap();
    assert!(receipt.validate().is_err());

    let mut receipt = sample_receipt();
    receipt.sections.authority.identities[12].digest[0] ^= 1;
    assert!(receipt.canonicalize().is_err());

    let mut receipt = sample_receipt();
    receipt.sections.authority.freeze56_descriptor[0] ^= 1;
    assert!(receipt.canonicalize().is_err());

    let mut receipt = sample_receipt();
    receipt.sections.toolchain.texts[14] = "different".to_owned();
    assert!(receipt.canonicalize().is_err());

    let mut receipt = sample_receipt();
    receipt.sections.toolchain.packages[1]
        .activated_features
        .pop();
    assert!(receipt.canonicalize().is_err());

    let mut receipt = sample_receipt();
    receipt.sections.tables_and_abi.semantic_table.digest[0] ^= 1;
    assert!(receipt.canonicalize().is_err());

    let mut receipt = sample_receipt();
    receipt.sections.tables_and_abi.choose_table.digest[0] ^= 1;
    assert!(receipt.canonicalize().is_err());
}

#[test]
fn root_key_validator_covers_the_complete_normal_form() {
    let root = canonical_root_key();
    assert_eq!(root_hand_mask(&root).unwrap(), 0x0fe0_0000);
    validate_root_action(&root, 8, 21, 6).unwrap();
    assert_eq!(least_root_action_for_context(&root, 6).unwrap(), 21);
    assert!(validate_root_action(&root, 7, 21, 6).is_err());
    assert!(validate_root_action(&root, 8, 20, 5).is_err());
    for offset in [
        0, 8, 10, 11, 12, 15, 19, 20, 21, 22, 23, 25, 27, 29, 31, 33, 34, 36,
    ] {
        let mut changed = root;
        changed[offset] ^= 0x80;
        assert!(
            validate_root_key(&changed).is_err(),
            "accepted root mutation at {offset}"
        );
    }
}

#[test]
fn failure_smoke_and_frames_are_exact_and_cycle_free() {
    let child_failure = FailureReceipt {
        phase: FailurePhase::ProjectorTask,
        code: FailureCode::ProjectorMismatch,
        task_ordinal: 613,
        subordinal: u32::MAX,
        child_exit: i32::MIN,
        native_status: u32::MAX,
        observed_mismatch: 0,
        build_identity: repeated(0xb1),
        freeze56_digest: FREEZE56_DESCRIPTOR_SHA256,
        child_failure_frame_digest: ZERO_DIGEST,
    };
    let failure_bytes = child_failure.encode_child_zeroed().unwrap();
    assert_eq!(failure_bytes.len(), FAILURE_BYTES);
    assert_eq!(
        FailureReceipt::decode(&failure_bytes).unwrap(),
        child_failure
    );
    let mut wrong_freeze = child_failure.clone();
    wrong_freeze.freeze56_digest[0] ^= 1;
    assert!(FailureReceipt::decode(&wrong_freeze.encode()).is_err());
    for len in 0..FAILURE_BYTES {
        assert!(FailureReceipt::decode(&failure_bytes[..len]).is_err());
    }

    let frame = WireFrame {
        kind: FrameKind::Failure,
        phase_or_command_ordinal: 0,
        unit_or_terminal_code: 0,
        detail: failure_bytes.to_vec(),
    };
    let frame_bytes = frame.encode().unwrap();
    assert_eq!(WireFrame::decode(&frame_bytes).unwrap(), frame);
    assert_eq!(
        u32::from_le_bytes(frame_bytes[..4].try_into().unwrap()) as usize,
        16 + FAILURE_BYTES
    );
    assert_eq!(frame_bytes.len(), 20 + FAILURE_BYTES);

    let parent = rerender_parent_failure(&frame_bytes, 1).unwrap();
    assert_eq!(parent.child_failure_frame_digest, sha256(&frame_bytes));
    assert_ne!(parent.child_failure_frame_digest, ZERO_DIGEST);
    assert_ne!(parent.encode(), failure_bytes);
    assert_eq!(FailureReceipt::decode(&parent.encode()).unwrap(), parent);

    let mut nonchild_failure = child_failure.clone();
    nonchild_failure.child_exit = 1;
    let nonchild_frame = WireFrame {
        detail: nonchild_failure.encode().to_vec(),
        ..frame.clone()
    }
    .encode()
    .unwrap();
    assert!(rerender_parent_failure(&nonchild_frame, 1).is_err());

    let mut direct_timeout = child_failure.clone();
    direct_timeout.code = FailureCode::Timeout;
    let direct_timeout_frame = WireFrame {
        detail: direct_timeout.encode().to_vec(),
        ..frame.clone()
    }
    .encode()
    .unwrap();
    assert!(rerender_parent_failure(&direct_timeout_frame, 124).is_err());

    let mut out_of_range_task = child_failure.clone();
    out_of_range_task.task_ordinal = 614;
    assert!(FailureReceipt::decode(&out_of_range_task.encode()).is_err());

    let mut recursive_child = frame_bytes.clone();
    recursive_child[20 + 144] = 1;
    assert!(WireFrame::decode(&recursive_child).is_err());
    assert!(rerender_parent_failure(&recursive_child, 1).is_err());

    let mut contradictory_huge_detail = Vec::with_capacity(FRAME_HEADER_BYTES);
    contradictory_huge_detail.extend_from_slice(&(FRAME_FIXED_PAYLOAD_BYTES as u32).to_le_bytes());
    contradictory_huge_detail.extend_from_slice(&1u16.to_le_bytes());
    contradictory_huge_detail.extend_from_slice(&u16::from(FrameKind::Success).to_le_bytes());
    contradictory_huge_detail.extend_from_slice(&0u32.to_le_bytes());
    contradictory_huge_detail.extend_from_slice(&0u32.to_le_bytes());
    contradictory_huge_detail.extend_from_slice(&u32::MAX.to_le_bytes());
    assert!(WireFrame::decode(&contradictory_huge_detail).is_err());

    let smoke = SmokeReport.encode();
    assert_eq!(smoke.len(), SMOKE_BYTES);
    assert_eq!(SmokeReport::decode(&smoke).unwrap(), SmokeReport);
    for len in 0..SMOKE_BYTES {
        assert!(SmokeReport::decode(&smoke[..len]).is_err());
    }
    let smoke_frame = WireFrame {
        kind: FrameKind::Success,
        phase_or_command_ordinal: 0,
        unit_or_terminal_code: 0,
        detail: smoke.to_vec(),
    };
    assert_eq!(
        WireFrame::decode(&smoke_frame.encode().unwrap()).unwrap(),
        smoke_frame
    );
    assert!(WireFrame {
        kind: FrameKind::Preparing,
        phase_or_command_ordinal: 1,
        unit_or_terminal_code: 0,
        detail: vec![1],
    }
    .encode()
    .is_err());
    assert!(WireFrame {
        kind: FrameKind::Terminal,
        phase_or_command_ordinal: 0,
        unit_or_terminal_code: 8,
        detail: Vec::new(),
    }
    .encode()
    .is_err());
}

#[test]
fn fixed_auxiliary_records_roundtrip_and_protected_chain_is_closed() {
    let aggregate = ResponseAggregateRecord {
        response: [1, 2, 3],
        support: 7,
        mass: 11,
    };
    assert_eq!(
        ResponseAggregateRecord::decode(&aggregate.encode()).unwrap(),
        aggregate
    );
    let framed = TaskFramedRecord {
        task_ordinal: 9,
        payload: vec![1, 2, 3, 4],
    };
    assert_eq!(
        TaskFramedRecord::decode(&framed.encode().unwrap()).unwrap(),
        framed
    );

    let mut protected = Vec::with_capacity(629);
    protected.push(ProtectedChainRecord {
        domain: 1,
        ordinal: 0,
        first_protected_record: 16_384,
        protected_count: 2,
        digest: repeated(1),
    });
    protected.push(ProtectedChainRecord {
        domain: 2,
        ordinal: 0,
        first_protected_record: 13,
        protected_count: 2,
        digest: repeated(2),
    });
    for ordinal in 0..13u32 {
        protected.push(ProtectedChainRecord {
            domain: 3,
            ordinal,
            first_protected_record: 0,
            protected_count: 12,
            digest: repeated(3),
        });
    }
    for ordinal in 0..614u32 {
        protected.push(ProtectedChainRecord {
            domain: 4,
            ordinal,
            first_protected_record: 79_800,
            protected_count: 2,
            digest: repeated(4),
        });
    }
    let digest = protected_chain_digest(&protected).unwrap();
    let mut payload = Vec::new();
    for record in &protected {
        payload.extend_from_slice(&record.encode());
    }
    assert_eq!(
        digest,
        stream_digest(StreamPurpose::GlobalProtectedChain, 629, &payload).unwrap()
    );
    protected[15].ordinal = 1;
    assert!(protected_chain_digest(&protected).is_err());
}
