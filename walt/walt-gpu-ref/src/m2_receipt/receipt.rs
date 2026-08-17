use super::records::*;
use super::wire::{
    checked_usize, put_digest, put_u16, put_u32, put_u64, require_eq, section_digest, sha256,
    stream_digest, Arm, ArtifactTag, CodecError, Digest, InvocationKind, KernelId, Reader, Result,
    SectionTag, SourceKind, StreamPurpose, ToolId, POISON_WORD, ZERO_DIGEST,
};
use super::{CONTRACT_BYTES, CONTRACT_SHA256};
use crate::m2::{
    canonical_arithmetic_receipt_facts_v1, canonical_opening_receipt_facts_v1,
    CanonicalBindingFactV1, M2CanonicalArithmeticRunReceiptFactsV1,
    M2CanonicalOpeningReceiptFactsV1, M2CarrierArmV1, M2OpeningNegativeControlsV1,
    M2OpeningParityCarrierV1, OpeningChooseTableV1,
};
use crate::{canonical_opening_root_key_bytes_v1, DirectPreflightV1};

pub const SUCCESS_MAGIC: [u8; 8] = *b"W42M2R01";
pub const SUCCESS_HEADER_BYTES: usize = 768;
pub const SUCCESS_SECTION_COUNT: usize = 10;
pub const SUCCESS_CLAIM: &[u8; 50] = b"M2 METAL PROJECTOR PARITY COMPLETE under freeze 56";
pub const FREEZE56_DESCRIPTOR: &[u8; 899] = b"GT1-M2-FREEZE-SET-V1|authority=GPU-NATIVE-TRICK1-M2-v1@aacb6df5e9106b3b6bf00ccfb496c71f762c0fb4644c13a17f76d2ac2f0326e3+GT1-A10..GT1-A17+freeze56|parent=freeze55@9b181092045b003893cae7c09cc7b7c8b57f75c3c5c4cf7043b8d428df738efa;commit=3b4c6d60fef371e3050de151ccf9eaefbc2d2da7|guide=ee2e78da20eb7d087fb121f467a56bafc0179a45fb692ca0b938f4c4210b6a44|rebrief=9183132529a42289a104a73d8f7e196eb95058ac2edda60bb42c715f1f8a139a|tasks=U256MetalParityV1,OpeningProjectorMetalParityV1|arithmetic=U256MassV1,U256MetalCorpusV1|projector=M2MetalAbiV1,OpeningChooseTableV1|carrier=M2OpeningParityCarrierV1|bindings=ReducedEvidenceBindingV1,PhysicalActionBindingV1|runner=M2SequentialRunnerV1|compiler=M2MetalCompilerProfileV1|receipt=M2MetalParityReceiptV1|manifest=M2SourceManifestV1|proof=Texas42.Trick1MetalFoundation|reserved=39,40|excluded=action-value,selected-lead,information-net,K-OPEN4+,performance,player";
pub const FREEZE56_DESCRIPTOR_SHA256: Digest = [
    0x7b, 0xdc, 0x5e, 0x05, 0x51, 0x3f, 0xd1, 0xd7, 0xe7, 0xb6, 0xc2, 0x68, 0x70, 0xcf, 0x9b, 0xd4,
    0xa1, 0x69, 0x66, 0xc5, 0xda, 0xf4, 0x89, 0x63, 0x72, 0x9d, 0x99, 0x9c, 0x4b, 0x6b, 0x28, 0xcf,
];
pub const PARENT_CENSUS_SHA256: Digest = [
    0x51, 0x8a, 0xb5, 0x40, 0x35, 0x8f, 0x8d, 0x74, 0xea, 0x09, 0x1a, 0x5e, 0x0d, 0x9d, 0xd2, 0x69,
    0xd6, 0xe6, 0x4e, 0xc5, 0x5a, 0x4b, 0x54, 0xc7, 0xb2, 0xa1, 0x0f, 0x2d, 0x3d, 0x20, 0x3e, 0x45,
];
pub const SEMANTIC_TABLE_BYTES: u64 = 14_884;
pub const SEMANTIC_TABLE_SHA256: Digest = [
    0x65, 0x95, 0xfa, 0xdb, 0x8c, 0x2a, 0xcf, 0x17, 0x4f, 0x10, 0x67, 0x00, 0xc6, 0x58, 0xa9, 0x9a,
    0x8a, 0x66, 0xb7, 0xd6, 0x3c, 0x53, 0x49, 0xdd, 0xda, 0x81, 0xe7, 0xab, 0xd2, 0x08, 0xb6, 0x6b,
];
pub const CHOOSE_TABLE_SHA256: Digest = [
    0x6c, 0xa8, 0x43, 0x2f, 0xa9, 0xa6, 0x42, 0x15, 0xa2, 0x4e, 0x77, 0xb8, 0xe4, 0xe4, 0x4b, 0xa3,
    0xe3, 0xdc, 0x66, 0x90, 0x78, 0x94, 0x3f, 0xfd, 0x1e, 0x96, 0xdf, 0x06, 0x69, 0x5b, 0x4d, 0x7a,
];

pub const AUTHORITY_PREFIX_BYTES: usize = 48;
pub const TOOLCHAIN_PREFIX_BYTES: usize = 80;
pub const DEVICE_PREFIX_BYTES: usize = 56;
pub const TABLES_AND_ABI_BYTES: usize = 184;
pub const ARITHMETIC_BYTES: usize = 624;
pub const CARRIER_BYTES: usize = 160;
pub const CONTEXT_TASKS_PREFIX_BYTES: usize = 16;
pub const REDUCED_BINDINGS_PREFIX_BYTES: usize = 16;
pub const PHYSICAL_BINDINGS_PREFIX_BYTES: usize = 16;
pub const GLOBAL_PREFIX_BYTES: usize = 64;
pub const GLOBAL_BYTES: usize = 438;

pub const CONTEXT_TASK_COUNT: usize = 614;
pub const REDUCED_BINDING_COUNT: usize = 103;
pub const PHYSICAL_BINDING_COUNT: usize = 1_015;
pub const ACCEPTED_ARITHMETIC_COUNT: u32 = 16_384;
pub const PROJECTOR_CAPACITY: u64 = 5_109_296;
pub const ARITHMETIC_CAPACITY: u64 = 2_359_424;

pub const PARENT_COMMIT_SHA1: [u8; 20] = [
    0x3b, 0x4c, 0x6d, 0x60, 0xfe, 0xf3, 0x71, 0xe3, 0x05, 0x0d, 0xe1, 0x51, 0xcc, 0xf9, 0xea, 0xef,
    0xbc, 0x2d, 0x2d, 0xa7,
];

const SECTION_RECORD_COUNTS: [u64; SUCCESS_SECTION_COUNT] = [13, 1, 1, 2, 2, 1, 614, 103, 1_015, 1];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthoritySection {
    pub parent_commit: [u8; 20],
    pub identities: Vec<ArtifactIdentity>,
    pub freeze56_descriptor: Vec<u8>,
}

impl AuthoritySection {
    pub fn encode(&self) -> Result<Vec<u8>> {
        self.validate(None, None)?;
        let mut out = Vec::with_capacity(
            AUTHORITY_PREFIX_BYTES + 13 * ARTIFACT_IDENTITY_BYTES + self.freeze56_descriptor.len(),
        );
        put_u32(&mut out, 1);
        put_u32(&mut out, 0x0f);
        out.extend_from_slice(&self.parent_commit);
        put_u32(&mut out, 184);
        put_u32(&mut out, 13);
        put_u32(&mut out, 12);
        put_u64(&mut out, 0);
        for identity in &self.identities {
            out.extend_from_slice(&identity.encode());
        }
        out.extend_from_slice(&self.freeze56_descriptor);
        Ok(out)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self> {
        let mut reader = Reader::new(bytes);
        require_eq(&reader.u32()?, &1, "authority version")?;
        require_eq(&reader.u32()?, &0x0f, "authority verification flags")?;
        let parent_commit = reader.array()?;
        require_eq(&reader.u32()?, &184, "parent manifest entry count")?;
        require_eq(&reader.u32()?, &13, "authority identity count")?;
        require_eq(&reader.u32()?, &12, "freeze56 identity tag")?;
        require_eq(&reader.u64()?, &0, "authority reserved")?;
        let mut identities = Vec::with_capacity(13);
        for _ in 0..13 {
            identities.push(ArtifactIdentity::read_from(&mut reader)?);
        }
        let descriptor_len = checked_usize(identities[11].byte_length, "freeze descriptor usize")?;
        let freeze56_descriptor = reader.bytes(descriptor_len)?.to_vec();
        reader.finish()?;
        let value = Self {
            parent_commit,
            identities,
            freeze56_descriptor,
        };
        value.validate(None, None)?;
        Ok(value)
    }

    pub fn validate(
        &self,
        build_identity: Option<&Digest>,
        freeze56_digest: Option<&Digest>,
    ) -> Result<()> {
        require_eq(&self.parent_commit, &PARENT_COMMIT_SHA1, "parent commit")?;
        require_eq(&self.identities.len(), &13, "authority identity count")?;
        for (index, identity) in self.identities.iter().enumerate() {
            let expected_tag = ArtifactTag::try_from(
                u32::try_from(index + 1).map_err(|_| CodecError::LengthOverflow("artifact tag"))?,
            )?;
            require_eq(&identity.tag, &expected_tag, "artifact tag/order")?;
        }
        let static_lengths = [
            Some(18_750),
            Some(944),
            Some(82_740),
            Some(29_607),
            Some(321_167),
            Some(16_177),
            Some(1_644),
            Some(1_171),
            None,
            Some(CONTRACT_BYTES),
            None,
            Some(899),
            Some(921_481),
        ];
        for (identity, expected) in self.identities.iter().zip(static_lengths) {
            if let Some(expected) = expected {
                require_eq(
                    &identity.byte_length,
                    &expected,
                    "static authority byte length",
                )?;
            }
        }
        for index in [8usize, 10] {
            if self.identities[index].byte_length == 0
                || self.identities[index].digest == ZERO_DIGEST
            {
                return Err(CodecError::Invalid("dynamic authority artifact identity"));
            }
        }
        for (index, expected_hex) in [
            "eccf0a3742e2cfc50cad158292db7ad8c6145da8aa7958b7aa2ed07a1566f2ad",
            "9b181092045b003893cae7c09cc7b7c8b57f75c3c5c4cf7043b8d428df738efa",
            "ee2e78da20eb7d087fb121f467a56bafc0179a45fb692ca0b938f4c4210b6a44",
            "6190e740a0579b6b5196e086e52c8022d4cddcd0f746ecbd9226f87bbc0e4790",
            "1127d3868d7da07c26a7b8bc031ac8a63ba84a9068df786b67a413ea6af5f517",
            "7e8dfecf1cac314ae6e71b406eb268b29d4157206ce5e64d1c50d1aa94d43bdf",
            "51a162ea933801f05b852ec2a454c48a31c7d292ee8273ba683d0a7fec340b12",
            "b57f7077e5aa0aa1d8030a76a3399076810b71b1623ad83e001aee2b4aaeb215",
        ]
        .iter()
        .enumerate()
        {
            require_eq(
                &self.identities[index].digest,
                &decode_hex_digest(expected_hex)?,
                "historical artifact digest",
            )?;
        }
        require_eq(
            &self.identities[9].byte_length,
            &CONTRACT_BYTES,
            "M2 contract byte length",
        )?;
        require_eq(
            &self.identities[9].digest,
            &CONTRACT_SHA256,
            "M2 contract digest",
        )?;
        require_eq(
            self.freeze56_descriptor.as_slice(),
            &FREEZE56_DESCRIPTOR[..],
            "exact freeze56 descriptor bytes",
        )?;
        let descriptor_digest = sha256(&self.freeze56_descriptor);
        require_eq(
            &self.identities[11].byte_length,
            &u64::try_from(self.freeze56_descriptor.len())
                .map_err(|_| CodecError::LengthOverflow("freeze descriptor u64"))?,
            "freeze descriptor byte length",
        )?;
        require_eq(
            &self.identities[11].digest,
            &descriptor_digest,
            "freeze descriptor digest",
        )?;
        require_eq(
            &descriptor_digest,
            &FREEZE56_DESCRIPTOR_SHA256,
            "frozen freeze56 descriptor digest",
        )?;
        require_eq(
            &self.identities[12].digest,
            &PARENT_CENSUS_SHA256,
            "parent CENSUS digest",
        )?;
        if let Some(build_identity) = build_identity {
            require_eq(
                &self.identities[8].digest,
                build_identity,
                "manifest/build identity",
            )?;
        }
        if let Some(freeze56_digest) = freeze56_digest {
            require_eq(
                &descriptor_digest,
                freeze56_digest,
                "header freeze56 digest",
            )?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToolchainSection {
    pub texts: Vec<String>,
    pub packages: Vec<PackageRecord>,
    pub tools: Vec<ToolRecord>,
    pub sources: Vec<SourceRecord>,
    pub invocations: Vec<InvocationRecord>,
    pub metallib_bytes: u64,
    pub committed_metallib_digest: Digest,
    pub fresh_build_1_digest: Digest,
    pub fresh_build_2_digest: Digest,
    pub committed_repro_digest: Digest,
}

impl ToolchainSection {
    pub fn encode(&self) -> Result<Vec<u8>> {
        self.validate()?;
        let mut out = Vec::new();
        put_u32(&mut out, 1);
        put_u32(&mut out, 0x1fff);
        put_u32(&mut out, 1);
        put_u32(&mut out, 5);
        put_u32(&mut out, 5);
        put_u32(&mut out, 2);
        put_u32(&mut out, 2);
        put_u32(&mut out, 1);
        put_u32(&mut out, 2);
        put_u32(&mut out, 19);
        put_u64(&mut out, self.metallib_bytes);
        put_digest(&mut out, &self.committed_metallib_digest);
        for text in &self.texts {
            super::wire::put_text(&mut out, text)?;
        }
        for package in &self.packages {
            package.encode(&mut out)?;
        }
        for tool in &self.tools {
            out.extend_from_slice(&tool.encode());
        }
        for source in &self.sources {
            source.encode(&mut out)?;
        }
        for invocation in &self.invocations {
            invocation.encode(&mut out)?;
        }
        put_digest(&mut out, &self.fresh_build_1_digest);
        put_digest(&mut out, &self.fresh_build_2_digest);
        put_digest(&mut out, &self.committed_repro_digest);
        Ok(out)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self> {
        let mut reader = Reader::new(bytes);
        require_eq(&reader.u32()?, &1, "toolchain version")?;
        require_eq(&reader.u32()?, &0x1fff, "toolchain verification flags")?;
        require_eq(&reader.u32()?, &1, "toolchain build profile")?;
        require_eq(&reader.u32()?, &5, "toolchain package count")?;
        require_eq(&reader.u32()?, &5, "toolchain tool count")?;
        require_eq(&reader.u32()?, &2, "toolchain source count")?;
        require_eq(&reader.u32()?, &2, "toolchain compile count")?;
        require_eq(&reader.u32()?, &1, "toolchain link count")?;
        require_eq(&reader.u32()?, &2, "toolchain repro count")?;
        require_eq(&reader.u32()?, &19, "toolchain text count")?;
        let metallib_bytes = reader.u64()?;
        let committed_metallib_digest = reader.digest()?;
        let mut texts = Vec::with_capacity(19);
        for _ in 0..19 {
            texts.push(reader.text()?);
        }
        let mut packages = Vec::with_capacity(5);
        for _ in 0..5 {
            packages.push(PackageRecord::read_from(&mut reader)?);
        }
        let mut tools = Vec::with_capacity(5);
        for _ in 0..5 {
            tools.push(ToolRecord::read_from(&mut reader)?);
        }
        let mut sources = Vec::with_capacity(2);
        for _ in 0..2 {
            sources.push(SourceRecord::read_from(&mut reader)?);
        }
        let mut invocations = Vec::with_capacity(3);
        for _ in 0..3 {
            invocations.push(InvocationRecord::read_from(&mut reader)?);
        }
        let value = Self {
            texts,
            packages,
            tools,
            sources,
            invocations,
            metallib_bytes,
            committed_metallib_digest,
            fresh_build_1_digest: reader.digest()?,
            fresh_build_2_digest: reader.digest()?,
            committed_repro_digest: reader.digest()?,
        };
        reader.finish()?;
        value.validate()?;
        Ok(value)
    }

    pub fn validate(&self) -> Result<()> {
        require_eq(&self.texts.len(), &19, "toolchain text count")?;
        for text in &self.texts {
            super::wire::validate_text(text)?;
        }
        require_eq(&self.texts[0].as_str(), &"1.95.0", "rustc release")?;
        require_eq(
            &self.texts[1].as_str(),
            &"aarch64-apple-darwin",
            "rustc host",
        )?;
        require_eq(&self.texts[2].as_str(), &"1.95.0", "Cargo release")?;
        require_eq(
            &self.texts[3].as_str(),
            &"aarch64-apple-darwin",
            "Rust build target",
        )?;
        for text in &self.texts[4..8] {
            require_eq(text, &String::new(), "empty Rust environment field")?;
        }
        require_eq(&self.texts[8].as_str(), &"26.6", "Xcode version")?;
        require_eq(&self.texts[9].as_str(), &"17F113", "Xcode build")?;
        require_eq(
            &self.texts[10].as_str(),
            &"com.apple.dt.toolchain.Metal.32023.883",
            "Metal component id",
        )?;
        require_eq(&self.texts[11].as_str(), &"17F109", "Metal component build")?;
        require_eq(
            &self.texts[12].as_str(),
            &"32023.883",
            "Metal compiler version",
        )?;
        require_eq(&self.texts[13].as_str(), &"26.5", "SDK version")?;
        require_eq(&self.texts[14].as_str(), &"25F70", "SDK build")?;
        require_eq(&self.texts[15].as_str(), &"26.0", "deployment target")?;
        require_eq(
            &self.texts[16].as_str(),
            &"16.0 (17F113)",
            "xctrace version",
        )?;
        require_eq(
            &self.texts[17].as_str(),
            &"u256_parity_v1",
            "arithmetic kernel name",
        )?;
        require_eq(
            &self.texts[18].as_str(),
            &"opening_project_v1",
            "projector kernel name",
        )?;

        let package_profiles: [(&str, &str, &str, bool, &[&str]); 5] = [
            (
                "dispatch2",
                "0.3.1",
                "1e0e367e4e7da84520dedcac1901e4da967309406d1e51017ae1abfb97adbd38",
                false,
                &["alloc", "block2", "objc2"],
            ),
            (
                "objc2",
                "0.6.4",
                "3a12a8ed07aefc768292f076dc3ac8c48f3781c8f2d5851dd3d98950e8c5a89f",
                true,
                &["alloc", "default", "std"],
            ),
            (
                "objc2-core-graphics",
                "0.3.2",
                "e022c9d066895efa1345f8e33e584b9f958da2fd4cd116792e15e07e4720a807",
                false,
                &[],
            ),
            (
                "objc2-foundation",
                "0.3.2",
                "e3e0adef53c21f888deb4fa59fc59f7eb17404926ee8a6f59f5df0fd7f9f3272",
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
                "a0125f776a10d00af4152d74616409f0d4a2053a6f57fa5b7d6aa2854ac04794",
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
        require_eq(
            &self.packages.len(),
            &package_profiles.len(),
            "package count",
        )?;
        for (package, (name, version, checksum, default_feature, features)) in
            self.packages.iter().zip(package_profiles)
        {
            package.validate()?;
            require_eq(&package.name.as_str(), &name, "package order/name")?;
            require_eq(&package.version.as_str(), &version, "package version")?;
            require_eq(
                &package.checksum,
                &decode_hex_digest(checksum)?,
                "package checksum",
            )?;
            require_eq(
                &package.default_feature,
                &default_feature,
                "package default feature",
            )?;
            let expected_features: Vec<String> = features
                .iter()
                .map(|feature| (*feature).to_owned())
                .collect();
            require_eq(
                &package.activated_features,
                &expected_features,
                "package activated features",
            )?;
        }

        require_eq(&self.tools.len(), &5, "tool count")?;
        for (index, tool) in self.tools.iter().enumerate() {
            let expected = ToolId::try_from(
                u32::try_from(index + 1).map_err(|_| CodecError::LengthOverflow("tool id"))?,
            )?;
            require_eq(&tool.id, &expected, "tool id/order")?;
            if tool.executable_bytes == 0 {
                return Err(CodecError::Invalid("tool executable length"));
            }
        }

        let paths = [
            "walt/walt-metal/shaders/00_u256.metal",
            "walt/walt-metal/shaders/01_opening_projector.metal",
        ];
        require_eq(&self.sources.len(), &2, "source count")?;
        for (source, expected_path) in self.sources.iter().zip(paths) {
            require_eq(&source.kind, &SourceKind::TranslationUnit, "source kind")?;
            require_eq(&source.path.as_str(), &expected_path, "source path/order")?;
            if source.byte_length == 0 {
                return Err(CodecError::Invalid("source byte length"));
            }
        }

        require_eq(&self.invocations.len(), &3, "invocation count")?;
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
        for index in 0..2 {
            let invocation = &self.invocations[index];
            require_eq(
                &invocation.kind,
                &InvocationKind::Compile,
                "compile kind/order",
            )?;
            require_eq(
                &invocation.source_index,
                &u32::try_from(index).map_err(|_| CodecError::LengthOverflow("source index"))?,
                "compile source index",
            )?;
            let basename = if index == 0 {
                "00_u256"
            } else {
                "01_opening_projector"
            };
            let mut expected: Vec<String> =
                common.iter().map(|value| (*value).to_owned()).collect();
            expected.push(format!("<SOURCE_DIR>/{basename}.metal"));
            expected.push("-o".to_owned());
            expected.push(format!("<AIR_DIR>/{basename}.air"));
            require_eq(&invocation.arguments, &expected, "compile argv")?;
        }
        let link = &self.invocations[2];
        require_eq(&link.kind, &InvocationKind::Link, "link kind/order")?;
        require_eq(&link.source_index, &u32::MAX, "link source index")?;
        require_eq(
            &link.arguments,
            &vec![
                "<AIR_DIR>/00_u256.air".to_owned(),
                "<AIR_DIR>/01_opening_projector.air".to_owned(),
                "-o".to_owned(),
                "<OUTPUT>".to_owned(),
            ],
            "link argv",
        )?;
        if self.metallib_bytes == 0 {
            return Err(CodecError::Invalid("metallib byte length"));
        }
        require_eq(
            &self.fresh_build_1_digest,
            &self.committed_metallib_digest,
            "fresh build 1 digest",
        )?;
        require_eq(
            &self.fresh_build_2_digest,
            &self.committed_metallib_digest,
            "fresh build 2 digest",
        )?;
        require_eq(
            &self.committed_repro_digest,
            &self.committed_metallib_digest,
            "committed repro digest",
        )?;
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeviceSection {
    pub texts: Vec<String>,
    pub max_buffer_length: u64,
    pub recommended_working_set: u64,
    pub max_threads: [u32; 3],
    pub max_threadgroup_memory: u32,
    pub pipelines: Vec<PipelineRecord>,
}

impl DeviceSection {
    pub fn encode(&self) -> Result<Vec<u8>> {
        self.validate()?;
        let mut out = Vec::new();
        put_u32(&mut out, 1);
        put_u32(&mut out, 3);
        put_u32(&mut out, 2);
        put_u32(&mut out, 3);
        put_u64(&mut out, self.max_buffer_length);
        put_u64(&mut out, self.recommended_working_set);
        for value in self.max_threads {
            put_u32(&mut out, value);
        }
        put_u32(&mut out, self.max_threadgroup_memory);
        put_u32(&mut out, NATIVE_STATUS_COMPLETED);
        put_u32(&mut out, 0);
        for text in &self.texts {
            super::wire::put_text(&mut out, text)?;
        }
        for pipeline in &self.pipelines {
            out.extend_from_slice(&pipeline.encode());
        }
        Ok(out)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self> {
        let mut reader = Reader::new(bytes);
        require_eq(&reader.u32()?, &1, "device version")?;
        require_eq(&reader.u32()?, &3, "device flags")?;
        require_eq(&reader.u32()?, &2, "pipeline count")?;
        require_eq(&reader.u32()?, &3, "device text count")?;
        let max_buffer_length = reader.u64()?;
        let recommended_working_set = reader.u64()?;
        let max_threads = [reader.u32()?, reader.u32()?, reader.u32()?];
        let max_threadgroup_memory = reader.u32()?;
        require_eq(
            &reader.u32()?,
            &NATIVE_STATUS_COMPLETED,
            "Gate0 native status",
        )?;
        require_eq(&reader.u32()?, &0, "device reserved")?;
        let mut texts = Vec::with_capacity(3);
        for _ in 0..3 {
            texts.push(reader.text()?);
        }
        let mut pipelines = Vec::with_capacity(2);
        for _ in 0..2 {
            pipelines.push(PipelineRecord::read_from(&mut reader)?);
        }
        reader.finish()?;
        let value = Self {
            texts,
            max_buffer_length,
            recommended_working_set,
            max_threads,
            max_threadgroup_memory,
            pipelines,
        };
        value.validate()?;
        Ok(value)
    }

    pub fn validate(&self) -> Result<()> {
        require_eq(&self.texts.len(), &3, "device text count")?;
        if self.texts.iter().any(String::is_empty) {
            return Err(CodecError::Invalid("device text"));
        }
        for text in &self.texts {
            super::wire::validate_text(text)?;
        }
        if self.max_buffer_length < PROJECTOR_CAPACITY
            || self.max_threads[0] < 32
            || self.max_threads[1] == 0
            || self.max_threads[2] == 0
        {
            return Err(CodecError::Invalid("device limits"));
        }
        require_eq(&self.pipelines.len(), &2, "pipeline count")?;
        for (pipeline, kernel) in self
            .pipelines
            .iter()
            .zip([KernelId::Arithmetic, KernelId::Projector])
        {
            require_eq(&pipeline.kernel, &kernel, "pipeline order")?;
            if pipeline.execution_width == 0 || pipeline.maximum_threads < 32 {
                return Err(CodecError::Invalid("pipeline limits"));
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TablesAndAbiSection {
    pub semantic_table: TableRecord,
    pub choose_table: TableRecord,
}

impl TablesAndAbiSection {
    pub fn encode(&self) -> Result<Vec<u8>> {
        self.validate()?;
        let mut out = Vec::with_capacity(TABLES_AND_ABI_BYTES);
        for value in [1, 2, 1, POISON_WORD, 32, 64, 80, 64, 420, 3, 11_730, 79_800] {
            put_u32(&mut out, value);
        }
        put_u64(&mut out, 120_000);
        put_u64(&mut out, PROJECTOR_CAPACITY);
        put_u64(&mut out, ARITHMETIC_CAPACITY);
        out.extend_from_slice(&self.semantic_table.encode());
        out.extend_from_slice(&self.choose_table.encode());
        require_eq(&out.len(), &TABLES_AND_ABI_BYTES, "tables encoded length")?;
        Ok(out)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self> {
        require_eq(&bytes.len(), &TABLES_AND_ABI_BYTES, "tables section length")?;
        let mut reader = Reader::new(bytes);
        for (expected, label) in [
            (1, "tables section version"),
            (2, "table count"),
            (1, "ABI version"),
            (POISON_WORD, "ABI poison"),
            (32, "task ABI bytes"),
            (64, "slot ABI bytes"),
            (80, "arithmetic input ABI bytes"),
            (64, "arithmetic output ABI bytes"),
            (420, "field scale"),
            (3, "response exponent"),
            (11_730, "cell cap"),
            (79_800, "slot cap"),
        ] {
            require_eq(&reader.u32()?, &expected, label)?;
        }
        require_eq(&reader.u64()?, &120_000, "timeout")?;
        require_eq(&reader.u64()?, &PROJECTOR_CAPACITY, "projector capacity")?;
        require_eq(&reader.u64()?, &ARITHMETIC_CAPACITY, "arithmetic capacity")?;
        let value = Self {
            semantic_table: TableRecord::read_from(&mut reader)?,
            choose_table: TableRecord::read_from(&mut reader)?,
        };
        reader.finish()?;
        value.validate()?;
        Ok(value)
    }

    pub fn validate(&self) -> Result<()> {
        require_eq(
            &(self.semantic_table.tag, self.semantic_table.format_version),
            &(1, 2),
            "semantic table tag/version",
        )?;
        require_eq(
            &(
                self.semantic_table.rows,
                self.semantic_table.columns,
                self.semantic_table.byte_length,
                self.semantic_table.digest,
            ),
            &(0, 0, SEMANTIC_TABLE_BYTES, SEMANTIC_TABLE_SHA256),
            "semantic table identity",
        )?;
        require_eq(
            &(
                self.choose_table.tag,
                self.choose_table.format_version,
                self.choose_table.rows,
                self.choose_table.columns,
                self.choose_table.byte_length,
                self.choose_table.digest,
            ),
            &(2, 1, 22, 22, 1_936, CHOOSE_TABLE_SHA256),
            "choose table identity",
        )?;
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArithmeticSection {
    pub official: ArithmeticRunRecord,
    pub negative: ArithmeticRunRecord,
}

impl ArithmeticSection {
    pub fn encode(&self) -> Result<Vec<u8>> {
        self.validate()?;
        let mut out = Vec::with_capacity(ARITHMETIC_BYTES);
        for value in [1, 288, 2, 32, 4_288, 12_096] {
            put_u32(&mut out, value);
        }
        put_u64(&mut out, 0x4d32_5f55_3235_3656);
        put_u64(&mut out, 0x9e37_79b9_7f4a_7c15);
        put_u32(&mut out, 1);
        put_u32(&mut out, 0);
        out.extend_from_slice(&self.official.encode());
        out.extend_from_slice(&self.negative.encode());
        require_eq(&out.len(), &ARITHMETIC_BYTES, "arithmetic encoded length")?;
        Ok(out)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self> {
        require_eq(&bytes.len(), &ARITHMETIC_BYTES, "arithmetic section length")?;
        let mut reader = Reader::new(bytes);
        for (expected, label) in [
            (1, "arithmetic section version"),
            (288, "arithmetic record bytes"),
            (2, "arithmetic run count"),
            (32, "edge count"),
            (4_288, "edge prefix count"),
            (12_096, "tail count"),
        ] {
            require_eq(&reader.u32()?, &expected, label)?;
        }
        require_eq(
            &reader.u64()?,
            &0x4d32_5f55_3235_3656,
            "SplitMix initial state",
        )?;
        require_eq(&reader.u64()?, &0x9e37_79b9_7f4a_7c15, "SplitMix increment")?;
        require_eq(&reader.u32()?, &1, "oracle profile")?;
        require_eq(&reader.u32()?, &0, "arithmetic prefix reserved")?;
        let value = Self {
            official: ArithmeticRunRecord::read_from(&mut reader)?,
            negative: ArithmeticRunRecord::read_from(&mut reader)?,
        };
        reader.finish()?;
        value.validate()?;
        Ok(value)
    }

    pub fn validate(&self) -> Result<()> {
        require_eq(
            &self.official.kind,
            &super::wire::ArithmeticRunKind::Official,
            "official arithmetic order",
        )?;
        require_eq(
            &self.negative.kind,
            &super::wire::ArithmeticRunKind::Negative,
            "negative arithmetic order",
        )?;
        self.official.validate()?;
        self.negative.validate()?;
        let canonical = canonical_arithmetic_receipt_facts_v1()
            .map_err(|_| CodecError::Invalid("canonical arithmetic receipt regeneration"))?;
        validate_arithmetic_run_against_canonical(
            &self.official,
            &canonical.official,
            1_310_720,
            1_048_704,
            "canonical official arithmetic receipt",
        )?;
        validate_arithmetic_run_against_canonical(
            &self.negative,
            &canonical.negative,
            1_040,
            960,
            "canonical negative arithmetic receipt",
        )
    }
}

fn validate_arithmetic_run_against_canonical(
    record: &ArithmeticRunRecord,
    canonical: &M2CanonicalArithmeticRunReceiptFactsV1,
    allocated_input_bytes: u64,
    allocated_output_bytes: u64,
    label: &'static str,
) -> Result<()> {
    require_eq(
        &(
            record.case_count,
            record.accepted_count,
            record.input_payload_bytes,
            record.output_payload_bytes,
            record.allocated_input_bytes,
            record.allocated_output_bytes,
            record.success_count,
            record.checked_undefined_count,
            record.hard_count,
        ),
        &(
            canonical.case_count,
            canonical.accepted_count,
            canonical.input_payload_bytes,
            canonical.output_payload_bytes,
            allocated_input_bytes,
            allocated_output_bytes,
            canonical.success_count,
            canonical.checked_undefined_count,
            canonical.hard_count,
        ),
        label,
    )?;
    for (actual, expected, digest_label) in [
        (
            &record.input_pre_digest,
            &canonical.input_sha256,
            "canonical arithmetic input pre-digest",
        ),
        (
            &record.input_post_digest,
            &canonical.input_sha256,
            "canonical arithmetic input post-digest",
        ),
        (
            &record.cpu_output_digest,
            &canonical.output_sha256,
            "canonical arithmetic CPU output digest",
        ),
        (
            &record.gpu_output_digest,
            &canonical.output_sha256,
            "canonical arithmetic GPU output digest",
        ),
        (
            &record.guard_pre_digest,
            &canonical.guard_sha256,
            "canonical arithmetic guard pre-digest",
        ),
        (
            &record.guard_post_digest,
            &canonical.guard_sha256,
            "canonical arithmetic guard post-digest",
        ),
    ] {
        require_eq(actual, expected, digest_label)?;
    }
    Ok(())
}

fn decode_hex_digest(value: &str) -> Result<Digest> {
    if value.len() != 64 {
        return Err(CodecError::Invalid("embedded digest length"));
    }
    let mut digest = [0; 32];
    for (index, chunk) in value.as_bytes().chunks_exact(2).enumerate() {
        let high = hex_nibble(chunk[0])?;
        let low = hex_nibble(chunk[1])?;
        digest[index] = (high << 4) | low;
    }
    Ok(digest)
}

fn hex_nibble(value: u8) -> Result<u8> {
    match value {
        b'0'..=b'9' => Ok(value - b'0'),
        b'a'..=b'f' => Ok(value - b'a' + 10),
        _ => Err(CodecError::Invalid("embedded digest hex")),
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CarrierSection {
    pub accepted_payload_bytes: u64,
    pub task_key_stream_digest: Digest,
    pub task_input_hash_chain_digest: Digest,
    pub choose_input_hash_chain_digest: Digest,
}

impl CarrierSection {
    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut out = Vec::with_capacity(CARRIER_BYTES);
        for value in [1, 1, 614, 64, 46, 504, 73, 541, 103, 1_015, 64, 11_730] {
            put_u32(&mut out, value);
        }
        put_u64(&mut out, 39_296);
        put_u64(&mut out, self.accepted_payload_bytes);
        put_digest(&mut out, &self.task_key_stream_digest);
        put_digest(&mut out, &self.task_input_hash_chain_digest);
        put_digest(&mut out, &self.choose_input_hash_chain_digest);
        require_eq(&out.len(), &CARRIER_BYTES, "carrier encoded length")?;
        Ok(out)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self> {
        require_eq(&bytes.len(), &CARRIER_BYTES, "carrier section length")?;
        let mut reader = Reader::new(bytes);
        for (expected, label) in [
            (1, "carrier version"),
            (1, "carrier profile"),
            (614, "carrier task count"),
            (64, "Reduced task count"),
            (46, "GradeMatching task count"),
            (504, "SameContext task count"),
            (73, "direct parity count"),
            (541, "direct stop count"),
            (103, "reduced binding count"),
            (1_015, "physical binding count"),
            (64, "task-key bytes"),
            (11_730, "maximum cell high-water"),
        ] {
            require_eq(&reader.u32()?, &expected, label)?;
        }
        require_eq(&reader.u64()?, &39_296, "task-key stream bytes")?;
        let value = Self {
            accepted_payload_bytes: reader.u64()?,
            task_key_stream_digest: reader.digest()?,
            task_input_hash_chain_digest: reader.digest()?,
            choose_input_hash_chain_digest: reader.digest()?,
        };
        reader.finish()?;
        Ok(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContextTasksSection {
    pub records: Vec<ContextTaskRecord>,
}

impl ContextTasksSection {
    pub fn encode(&self) -> Result<Vec<u8>> {
        self.validate()?;
        let mut out = Vec::with_capacity(
            CONTEXT_TASKS_PREFIX_BYTES + CONTEXT_TASK_COUNT * CONTEXT_TASK_RECORD_BYTES,
        );
        put_u32(&mut out, 1);
        put_u32(&mut out, 384);
        put_u32(&mut out, 614);
        put_u32(&mut out, 0);
        for record in &self.records {
            out.extend_from_slice(&record.encode());
        }
        Ok(out)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self> {
        require_eq(
            &bytes.len(),
            &(CONTEXT_TASKS_PREFIX_BYTES + CONTEXT_TASK_COUNT * CONTEXT_TASK_RECORD_BYTES),
            "context section length",
        )?;
        let mut reader = Reader::new(bytes);
        require_eq(&reader.u32()?, &1, "context section version")?;
        require_eq(&reader.u32()?, &384, "context record bytes")?;
        require_eq(&reader.u32()?, &614, "context record count")?;
        require_eq(&reader.u32()?, &0, "context prefix reserved")?;
        let mut records = Vec::with_capacity(CONTEXT_TASK_COUNT);
        for _ in 0..CONTEXT_TASK_COUNT {
            records.push(ContextTaskRecord::read_from(&mut reader)?);
        }
        reader.finish()?;
        let value = Self { records };
        value.validate()?;
        Ok(value)
    }

    pub fn validate(&self) -> Result<()> {
        require_eq(
            &self.records.len(),
            &CONTEXT_TASK_COUNT,
            "context record count",
        )?;
        let mut parity_count = 0usize;
        let mut stop_count = 0usize;
        let mut high_water = 0u32;
        for (index, record) in self.records.iter().enumerate() {
            record.validate()?;
            let ordinal =
                u32::try_from(index).map_err(|_| CodecError::LengthOverflow("task ordinal"))?;
            require_eq(&record.key.task_ordinal, &ordinal, "global task ordinal")?;
            let (arm, arm_ordinal) = match index {
                0..=63 => (Arm::Reduced, ordinal),
                64..=109 => (Arm::GradeMatching, ordinal - 64),
                _ => (Arm::SameContextPair, ordinal - 110),
            };
            require_eq(&record.key.arm, &arm, "carrier arm order")?;
            require_eq(&record.key.arm_ordinal, &arm_ordinal, "carrier arm ordinal")?;
            let expected_direct = match record.key.arm {
                Arm::Reduced if record.key.grade <= 4 => super::wire::DirectStatus::Parity,
                Arm::GradeMatching if record.key.grade <= 4 => super::wire::DirectStatus::Parity,
                _ => super::wire::DirectStatus::DeclaredStop,
            };
            require_eq(
                &record.direct_status,
                &expected_direct,
                "carrier direct classification",
            )?;
            match record.direct_status {
                super::wire::DirectStatus::Parity => parity_count += 1,
                super::wire::DirectStatus::DeclaredStop => stop_count += 1,
            }
            high_water = high_water.max(record.accepted_cells);
        }
        require_eq(&parity_count, &73, "direct parity census")?;
        require_eq(&stop_count, &541, "direct stop census")?;
        require_eq(&high_water, &11_730, "accepted-cell high-water")?;
        Ok(())
    }

    pub fn accepted_payload_bytes(&self) -> Result<u64> {
        self.records.iter().try_fold(0u64, |total, record| {
            total
                .checked_add(record.canonical_payload_bytes)
                .ok_or(CodecError::LengthOverflow("accepted payload sum"))
        })
    }

    pub fn task_key_stream(&self) -> Vec<u8> {
        let mut stream = Vec::with_capacity(CONTEXT_TASK_COUNT * TASK_KEY_BYTES);
        for record in &self.records {
            stream.extend_from_slice(&record.key.encode());
        }
        stream
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReducedBindingsSection {
    pub records: Vec<ReducedBindingRecord>,
}

impl ReducedBindingsSection {
    pub fn encode(&self) -> Result<Vec<u8>> {
        require_eq(
            &self.records.len(),
            &REDUCED_BINDING_COUNT,
            "reduced binding count",
        )?;
        let mut out = Vec::with_capacity(
            REDUCED_BINDINGS_PREFIX_BYTES + REDUCED_BINDING_COUNT * REDUCED_BINDING_RECORD_BYTES,
        );
        put_u32(&mut out, 1);
        put_u32(&mut out, 160);
        put_u32(&mut out, 103);
        put_u32(&mut out, 0);
        for record in &self.records {
            record.validate_shape()?;
            out.extend_from_slice(&record.encode());
        }
        Ok(out)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self> {
        require_eq(
            &bytes.len(),
            &(REDUCED_BINDINGS_PREFIX_BYTES + REDUCED_BINDING_COUNT * REDUCED_BINDING_RECORD_BYTES),
            "reduced section length",
        )?;
        let mut reader = Reader::new(bytes);
        require_eq(&reader.u32()?, &1, "reduced section version")?;
        require_eq(&reader.u32()?, &160, "reduced record bytes")?;
        require_eq(&reader.u32()?, &103, "reduced section count")?;
        require_eq(&reader.u32()?, &0, "reduced prefix reserved")?;
        let mut records = Vec::with_capacity(REDUCED_BINDING_COUNT);
        for _ in 0..REDUCED_BINDING_COUNT {
            records.push(ReducedBindingRecord::read_from(&mut reader)?);
        }
        reader.finish()?;
        Ok(Self { records })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PhysicalBindingsSection {
    pub records: Vec<PhysicalBindingRecord>,
}

impl PhysicalBindingsSection {
    pub fn encode(&self) -> Result<Vec<u8>> {
        require_eq(
            &self.records.len(),
            &PHYSICAL_BINDING_COUNT,
            "physical binding count",
        )?;
        let mut out = Vec::with_capacity(
            PHYSICAL_BINDINGS_PREFIX_BYTES + PHYSICAL_BINDING_COUNT * PHYSICAL_BINDING_RECORD_BYTES,
        );
        put_u32(&mut out, 1);
        put_u32(&mut out, 160);
        put_u32(&mut out, 1_015);
        put_u32(&mut out, 0);
        for record in &self.records {
            record.validate_shape()?;
            out.extend_from_slice(&record.encode());
        }
        Ok(out)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self> {
        require_eq(
            &bytes.len(),
            &(PHYSICAL_BINDINGS_PREFIX_BYTES
                + PHYSICAL_BINDING_COUNT * PHYSICAL_BINDING_RECORD_BYTES),
            "physical section length",
        )?;
        let mut reader = Reader::new(bytes);
        require_eq(&reader.u32()?, &1, "physical section version")?;
        require_eq(&reader.u32()?, &160, "physical record bytes")?;
        require_eq(&reader.u32()?, &1_015, "physical section count")?;
        require_eq(&reader.u32()?, &0, "physical prefix reserved")?;
        let mut records = Vec::with_capacity(PHYSICAL_BINDING_COUNT);
        for _ in 0..PHYSICAL_BINDING_COUNT {
            records.push(PhysicalBindingRecord::read_from(&mut reader)?);
        }
        reader.finish()?;
        Ok(Self { records })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GlobalSection {
    /// CPU raw, GPU raw, CPU payload, GPU payload, CPU aggregate, GPU
    /// aggregate, protected chain, reduced identities, physical identities,
    /// success conjunction.
    pub digests: [Digest; 10],
}

impl GlobalSection {
    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut out = Vec::with_capacity(GLOBAL_BYTES);
        for value in [1, 0x3fff, 11_730, POISON_WORD, 0, 0, 0, 0] {
            put_u32(&mut out, value);
        }
        for value in [
            PROJECTOR_CAPACITY,
            ARITHMETIC_CAPACITY,
            PROJECTOR_CAPACITY,
            ARITHMETIC_CAPACITY,
        ] {
            put_u64(&mut out, value);
        }
        for digest in &self.digests {
            put_digest(&mut out, digest);
        }
        put_u32(&mut out, 50);
        out.extend_from_slice(SUCCESS_CLAIM);
        require_eq(&out.len(), &GLOBAL_BYTES, "global encoded length")?;
        Ok(out)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self> {
        require_eq(&bytes.len(), &GLOBAL_BYTES, "global section length")?;
        let mut reader = Reader::new(bytes);
        for (expected, label) in [
            (1, "global version"),
            (0x3fff, "global validation flags"),
            (11_730, "global cell high-water"),
            (POISON_WORD, "global poison"),
            (0, "opening tail failures"),
            (0, "opening guard failures"),
            (0, "arithmetic guard failures"),
            (0, "input mutation failures"),
        ] {
            require_eq(&reader.u32()?, &expected, label)?;
        }
        for (expected, label) in [
            (PROJECTOR_CAPACITY, "global projector capacity"),
            (ARITHMETIC_CAPACITY, "global arithmetic capacity"),
            (PROJECTOR_CAPACITY, "global projector allocated high-water"),
            (
                ARITHMETIC_CAPACITY,
                "global arithmetic allocated high-water",
            ),
        ] {
            require_eq(&reader.u64()?, &expected, label)?;
        }
        let mut digests = [[0; 32]; 10];
        for digest in &mut digests {
            *digest = reader.digest()?;
        }
        require_eq(&reader.u32()?, &50, "success claim length")?;
        require_eq(reader.bytes(50)?, SUCCESS_CLAIM, "success claim")?;
        reader.finish()?;
        let value = Self { digests };
        value.validate_pairs()?;
        Ok(value)
    }

    pub fn validate_pairs(&self) -> Result<()> {
        for (left, right, label) in [
            (0, 1, "global raw parity"),
            (2, 3, "global payload parity"),
            (4, 5, "global aggregate parity"),
        ] {
            require_eq(&self.digests[left], &self.digests[right], label)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReceiptSections {
    pub authority: AuthoritySection,
    pub toolchain: ToolchainSection,
    pub device: DeviceSection,
    pub tables_and_abi: TablesAndAbiSection,
    pub arithmetic: ArithmeticSection,
    pub carrier: CarrierSection,
    pub context_tasks: ContextTasksSection,
    pub reduced_bindings: ReducedBindingsSection,
    pub physical_bindings: PhysicalBindingsSection,
    pub global: GlobalSection,
}

/// Regenerates the exact closed-carrier binding records while copying only
/// the already-observed payload length and GPU payload digest from each task.
/// Semantic identities remain zero until [`SuccessReceipt::canonicalize`]
/// binds them to the build, freeze, and table identities.
pub fn canonical_binding_sections(
    context_tasks: &ContextTasksSection,
) -> Result<(ReducedBindingsSection, PhysicalBindingsSection)> {
    context_tasks.validate()?;
    let carrier = regenerate_canonical_carrier()?;
    validate_context_tasks_against_carrier(context_tasks, &carrier)?;
    canonical_binding_sections_from_carrier(context_tasks, &carrier)
}

fn regenerate_canonical_carrier() -> Result<M2OpeningParityCarrierV1> {
    M2OpeningParityCarrierV1::canonical()
        .map_err(|_| CodecError::Invalid("canonical M2 carrier regeneration"))
}

fn canonical_input_hash_chain_digests(
    carrier: &M2OpeningParityCarrierV1,
) -> Result<(Digest, Digest)> {
    let choose = OpeningChooseTableV1::canonical()
        .map_err(|_| CodecError::Invalid("canonical choose-table regeneration"))?;
    require_eq(
        &choose.digest(),
        &CHOOSE_TABLE_SHA256,
        "canonical choose-table digest",
    )?;
    let mut task_chain = Vec::with_capacity(carrier.tasks().len() * INPUT_HASH_CHAIN_RECORD_BYTES);
    let mut choose_chain =
        Vec::with_capacity(carrier.tasks().len() * INPUT_HASH_CHAIN_RECORD_BYTES);
    for (ordinal, task) in carrier.tasks().iter().enumerate() {
        let ordinal = u32::try_from(ordinal)
            .map_err(|_| CodecError::LengthOverflow("input-chain task ordinal"))?;
        require_eq(
            &task.ordinal(),
            &ordinal,
            "canonical input-chain task order",
        )?;
        let mut task_bytes = [0u8; 32];
        for (index, word) in task.task_words().iter().enumerate() {
            task_bytes[index * 4..index * 4 + 4].copy_from_slice(&word.to_le_bytes());
        }
        let task_digest = sha256(&task_bytes);
        task_chain.extend_from_slice(
            &InputHashChainRecord {
                task_ordinal: ordinal,
                pre_digest: task_digest,
                post_digest: task_digest,
            }
            .encode(),
        );
        choose_chain.extend_from_slice(
            &InputHashChainRecord {
                task_ordinal: ordinal,
                pre_digest: choose.digest(),
                post_digest: choose.digest(),
            }
            .encode(),
        );
    }
    Ok((
        stream_digest(StreamPurpose::TaskInputHashChain, 614, &task_chain)?,
        stream_digest(StreamPurpose::ChooseInputHashChain, 614, &choose_chain)?,
    ))
}

fn validate_context_tasks_against_carrier(
    context_tasks: &ContextTasksSection,
    carrier: &M2OpeningParityCarrierV1,
) -> Result<()> {
    require_eq(
        &context_tasks.records.len(),
        &carrier.tasks().len(),
        "canonical carrier task count",
    )?;
    let canonical = canonical_opening_receipt_facts_v1()
        .map_err(|_| CodecError::Invalid("canonical opening receipt regeneration"))?;
    require_eq(
        &canonical.contexts.len(),
        &carrier.tasks().len(),
        "canonical opening fact count",
    )?;
    for ((record, task), fact) in context_tasks
        .records
        .iter()
        .zip(carrier.tasks())
        .zip(canonical.contexts.iter())
    {
        require_eq(
            &record.key.encode(),
            &task.task_key().to_le_bytes(),
            "canonical carrier task key",
        )?;
        require_eq(
            &record.direct_world_count,
            &task.direct_preflight().world_count(),
            "canonical direct world count",
        )?;
        let expected_status = match task.direct_preflight() {
            DirectPreflightV1::Admitted { .. } => super::wire::DirectStatus::Parity,
            DirectPreflightV1::DeclaredStop { .. } => super::wire::DirectStatus::DeclaredStop,
        };
        require_eq(
            &record.direct_status,
            &expected_status,
            "canonical direct status",
        )?;
        require_eq(
            &record.key.task_ordinal,
            &fact.task_ordinal,
            "canonical opening fact ordinal",
        )?;
        require_eq(
            &record.accepted_cells,
            &fact.accepted_cells,
            "canonical accepted cell count",
        )?;
        require_eq(
            &record.canonical_payload_bytes,
            &fact.canonical_payload_bytes,
            "canonical context payload bytes",
        )?;
        require_eq(
            &record.total_scaled_mass,
            &fact.total_scaled_mass.limbs_le(),
            "canonical total scaled mass",
        )?;
        for (actual, expected, label) in [
            (
                &record.cpu_slot_digest,
                &fact.raw_sha256,
                "canonical CPU raw slot digest",
            ),
            (
                &record.gpu_slot_digest,
                &fact.raw_sha256,
                "canonical GPU raw slot digest",
            ),
            (
                &record.cpu_payload_digest,
                &fact.payload_sha256,
                "canonical CPU payload digest",
            ),
            (
                &record.gpu_payload_digest,
                &fact.payload_sha256,
                "canonical GPU payload digest",
            ),
            (
                &record.cpu_aggregate_digest,
                &fact.aggregate_sha256,
                "canonical CPU aggregate digest",
            ),
            (
                &record.gpu_aggregate_digest,
                &fact.aggregate_sha256,
                "canonical GPU aggregate digest",
            ),
            (
                &record.tail_guard_digest,
                &fact.tail_guard_sha256,
                "canonical opening tail-guard digest",
            ),
        ] {
            require_eq(actual, expected, label)?;
        }
    }
    Ok(())
}

/// Canonical scalar and digest observations used by receipt mutation tests and
/// independent offline adjudicators.  These values make no Metal-provenance
/// claim; the official runner may populate a receipt only from its move-only
/// accepted Metal evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalScalarObservationSectionsV1 {
    pub arithmetic: ArithmeticSection,
    pub context_tasks: ContextTasksSection,
    pub global_observation_digests: [Digest; 7],
}

/// Recompute the exact portable half of every scalar/digest observation that
/// cannot be inferred merely by equating two receipt fields.
pub fn canonical_scalar_observation_sections_v1() -> Result<CanonicalScalarObservationSectionsV1> {
    let carrier = regenerate_canonical_carrier()?;
    let opening = canonical_opening_receipt_facts_v1()
        .map_err(|_| CodecError::Invalid("canonical opening receipt regeneration"))?;
    require_eq(
        &opening.contexts.len(),
        &carrier.tasks().len(),
        "canonical scalar opening count",
    )?;
    let mut records = Vec::with_capacity(CONTEXT_TASK_COUNT);
    for (task, fact) in carrier.tasks().iter().zip(opening.contexts.iter()) {
        let direct_status = match task.direct_preflight() {
            DirectPreflightV1::Admitted { .. } => super::wire::DirectStatus::Parity,
            DirectPreflightV1::DeclaredStop { .. } => super::wire::DirectStatus::DeclaredStop,
        };
        records.push(ContextTaskRecord {
            key: TaskKey::decode(&task.task_key().to_le_bytes())?,
            direct_status,
            direct_world_count: task.direct_preflight().world_count(),
            accepted_cells: fact.accepted_cells,
            in_range_slot_bytes: u64::from(task.candidate_slot_count()) * 64,
            canonical_payload_bytes: fact.canonical_payload_bytes,
            total_scaled_mass: fact.total_scaled_mass.limbs_le(),
            cpu_slot_digest: fact.raw_sha256,
            gpu_slot_digest: fact.raw_sha256,
            cpu_payload_digest: fact.payload_sha256,
            gpu_payload_digest: fact.payload_sha256,
            cpu_aggregate_digest: fact.aggregate_sha256,
            gpu_aggregate_digest: fact.aggregate_sha256,
            tail_guard_digest: fact.tail_guard_sha256,
        });
    }
    let context_tasks = ContextTasksSection { records };

    let arithmetic = canonical_arithmetic_receipt_facts_v1()
        .map_err(|_| CodecError::Invalid("canonical arithmetic receipt regeneration"))?;
    let arithmetic = ArithmeticSection {
        official: arithmetic_record_from_canonical(
            super::wire::ArithmeticRunKind::Official,
            &arithmetic.official,
            1_310_720,
            1_048_704,
        ),
        negative: arithmetic_record_from_canonical(
            super::wire::ArithmeticRunKind::Negative,
            &arithmetic.negative,
            1_040,
            960,
        ),
    };
    let global_observation_digests = canonical_global_observation_digests(&carrier, &opening)?;
    context_tasks.validate()?;
    arithmetic.validate()?;
    Ok(CanonicalScalarObservationSectionsV1 {
        arithmetic,
        context_tasks,
        global_observation_digests,
    })
}

fn arithmetic_record_from_canonical(
    kind: super::wire::ArithmeticRunKind,
    canonical: &M2CanonicalArithmeticRunReceiptFactsV1,
    allocated_input_bytes: u64,
    allocated_output_bytes: u64,
) -> ArithmeticRunRecord {
    ArithmeticRunRecord {
        kind,
        case_count: canonical.case_count,
        accepted_count: canonical.accepted_count,
        input_payload_bytes: canonical.input_payload_bytes,
        output_payload_bytes: canonical.output_payload_bytes,
        allocated_input_bytes,
        allocated_output_bytes,
        success_count: canonical.success_count,
        checked_undefined_count: canonical.checked_undefined_count,
        hard_count: canonical.hard_count,
        input_pre_digest: canonical.input_sha256,
        input_post_digest: canonical.input_sha256,
        cpu_output_digest: canonical.output_sha256,
        gpu_output_digest: canonical.output_sha256,
        guard_pre_digest: canonical.guard_sha256,
        guard_post_digest: canonical.guard_sha256,
    }
}

fn canonical_global_observation_digests(
    carrier: &M2OpeningParityCarrierV1,
    opening: &M2CanonicalOpeningReceiptFactsV1,
) -> Result<[Digest; 7]> {
    require_eq(
        &opening.contexts.len(),
        &carrier.tasks().len(),
        "canonical protected opening count",
    )?;
    let arithmetic = canonical_arithmetic_receipt_facts_v1()
        .map_err(|_| CodecError::Invalid("canonical arithmetic receipt regeneration"))?;
    let controls = M2OpeningNegativeControlsV1::canonical()
        .map_err(|_| CodecError::Invalid("canonical opening negative regeneration"))?;
    let mut protected = Vec::with_capacity(629);
    protected.push(ProtectedChainRecord {
        domain: 1,
        ordinal: 0,
        first_protected_record: 16_384,
        protected_count: 2,
        digest: arithmetic.official.guard_sha256,
    });
    protected.push(ProtectedChainRecord {
        domain: 2,
        ordinal: 0,
        first_protected_record: 13,
        protected_count: 2,
        digest: arithmetic.negative.guard_sha256,
    });
    for control in controls.controls() {
        let mut bytes = Vec::with_capacity(12 * 16 * 4);
        for slot in control.expected_slots() {
            for word in slot {
                bytes.extend_from_slice(&word.to_le_bytes());
            }
        }
        protected.push(ProtectedChainRecord {
            domain: 3,
            ordinal: control.ordinal(),
            first_protected_record: 0,
            protected_count: 12,
            digest: stream_digest(StreamPurpose::ProtectedRecords, 12, &bytes)?,
        });
    }
    for (task, fact) in carrier.tasks().iter().zip(opening.contexts.iter()) {
        protected.push(ProtectedChainRecord {
            domain: 4,
            ordinal: task.ordinal(),
            first_protected_record: task.candidate_slot_count(),
            protected_count: 79_802u32
                .checked_sub(task.candidate_slot_count())
                .ok_or(CodecError::Invalid("canonical protected task range"))?,
            digest: fact.tail_guard_sha256,
        });
    }
    let protected_digest = protected_chain_digest(&protected)?;
    Ok([
        opening.global.raw_sha256,
        opening.global.raw_sha256,
        opening.global.payload_sha256,
        opening.global.payload_sha256,
        opening.global.aggregate_sha256,
        opening.global.aggregate_sha256,
        protected_digest,
    ])
}

fn canonical_binding_sections_from_carrier(
    context_tasks: &ContextTasksSection,
    carrier: &M2OpeningParityCarrierV1,
) -> Result<(ReducedBindingsSection, PhysicalBindingsSection)> {
    let mut reduced = Vec::with_capacity(REDUCED_BINDING_COUNT);
    let mut physical = Vec::with_capacity(PHYSICAL_BINDING_COUNT);
    for task in carrier.tasks() {
        for fact in task.canonical_binding_facts_v1() {
            match fact {
                CanonicalBindingFactV1::Reduced {
                    binding_ordinal,
                    task_ordinal,
                    arm,
                    arm_ordinal,
                    root,
                    selected_action,
                    context,
                    matching_count,
                } => {
                    let observation = context_tasks
                        .records
                        .get(usize::try_from(task_ordinal).map_err(|_| {
                            CodecError::LengthOverflow("canonical reduced task ordinal")
                        })?)
                        .ok_or(CodecError::Invalid("canonical reduced task ordinal"))?;
                    reduced.push(ReducedBindingRecord {
                        binding_ordinal,
                        task_ordinal,
                        arm: receipt_arm(arm),
                        arm_ordinal,
                        root_key: canonical_opening_root_key_bytes_v1(root)
                            .map_err(|_| CodecError::Invalid("canonical reduced root encoding"))?,
                        selected_action: u8::try_from(selected_action.index())
                            .map_err(|_| CodecError::LengthOverflow("canonical reduced action"))?,
                        derived_context: u8::try_from(context.led().index())
                            .map_err(|_| CodecError::LengthOverflow("canonical reduced context"))?,
                        grade: context.grade(),
                        matching_count,
                        reduced_pool_mask: context.pool().bits(),
                        payload_bytes: observation.canonical_payload_bytes,
                        payload_digest: observation.gpu_payload_digest,
                        semantic_identity: ZERO_DIGEST,
                    });
                }
                CanonicalBindingFactV1::Physical {
                    binding_ordinal,
                    task_ordinal,
                    arm,
                    arm_ordinal,
                    endpoint,
                    root,
                    selected_action,
                    context,
                } => {
                    let observation = context_tasks
                        .records
                        .get(usize::try_from(task_ordinal).map_err(|_| {
                            CodecError::LengthOverflow("canonical physical task ordinal")
                        })?)
                        .ok_or(CodecError::Invalid("canonical physical task ordinal"))?;
                    physical.push(PhysicalBindingRecord {
                        binding_ordinal,
                        task_ordinal,
                        arm: receipt_arm(arm),
                        arm_ordinal,
                        endpoint,
                        root_key: canonical_opening_root_key_bytes_v1(root)
                            .map_err(|_| CodecError::Invalid("canonical physical root encoding"))?,
                        selected_action: u8::try_from(selected_action.index())
                            .map_err(|_| CodecError::LengthOverflow("canonical physical action"))?,
                        derived_context: u8::try_from(context.led().index()).map_err(|_| {
                            CodecError::LengthOverflow("canonical physical context")
                        })?,
                        context_pool_mask: context.pool().bits(),
                        payload_bytes: observation.canonical_payload_bytes,
                        payload_digest: observation.gpu_payload_digest,
                        semantic_identity: ZERO_DIGEST,
                    });
                }
            }
        }
    }
    require_eq(
        &reduced.len(),
        &REDUCED_BINDING_COUNT,
        "canonical reduced binding count",
    )?;
    require_eq(
        &physical.len(),
        &PHYSICAL_BINDING_COUNT,
        "canonical physical binding count",
    )?;
    Ok((
        ReducedBindingsSection { records: reduced },
        PhysicalBindingsSection { records: physical },
    ))
}

const fn receipt_arm(arm: M2CarrierArmV1) -> Arm {
    match arm {
        M2CarrierArmV1::Reduced => Arm::Reduced,
        M2CarrierArmV1::GradeMatching => Arm::GradeMatching,
        M2CarrierArmV1::SameContextPair => Arm::SameContextPair,
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SuccessReceipt {
    pub build_identity: Digest,
    pub freeze56_descriptor_digest: Digest,
    pub sections: ReceiptSections,
}

impl SuccessReceipt {
    /// Recomputes every receipt value that is derivable from other persisted
    /// fields.  It never invents an observation digest.
    pub fn canonicalize(&mut self) -> Result<()> {
        require_eq(
            &self.sections.authority.identities.len(),
            &13,
            "authority identity count",
        )?;
        self.sections.authority.identities[8].digest = self.build_identity;
        self.sections.authority.identities[9].byte_length = CONTRACT_BYTES;
        self.sections.authority.identities[9].digest = CONTRACT_SHA256;
        self.freeze56_descriptor_digest = sha256(&self.sections.authority.freeze56_descriptor);
        self.sections.authority.identities[11].byte_length =
            u64::try_from(self.sections.authority.freeze56_descriptor.len())
                .map_err(|_| CodecError::LengthOverflow("freeze descriptor u64"))?;
        self.sections.authority.identities[11].digest = self.freeze56_descriptor_digest;

        let accepted_payload_bytes = self.sections.context_tasks.accepted_payload_bytes()?;
        self.sections.carrier.accepted_payload_bytes = accepted_payload_bytes;
        self.sections.carrier.task_key_stream_digest = stream_digest(
            StreamPurpose::TaskKeys,
            614,
            &self.sections.context_tasks.task_key_stream(),
        )?;
        let canonical_carrier = regenerate_canonical_carrier()?;
        let (task_input_chain, choose_input_chain) =
            canonical_input_hash_chain_digests(&canonical_carrier)?;
        self.sections.carrier.task_input_hash_chain_digest = task_input_chain;
        self.sections.carrier.choose_input_hash_chain_digest = choose_input_chain;

        let semantic_table = self.sections.tables_and_abi.semantic_table.digest;
        let choose_table = self.sections.tables_and_abi.choose_table.digest;
        for record in &mut self.sections.reduced_bindings.records {
            let context = self
                .sections
                .context_tasks
                .records
                .get(
                    usize::try_from(record.task_ordinal)
                        .map_err(|_| CodecError::LengthOverflow("reduced task ordinal usize"))?,
                )
                .ok_or(CodecError::Invalid("reduced task ordinal"))?;
            record.semantic_identity = reduced_semantic_identity(
                &self.build_identity,
                &self.freeze56_descriptor_digest,
                &semantic_table,
                &choose_table,
                record,
                &context.key,
            );
        }
        for record in &mut self.sections.physical_bindings.records {
            record.semantic_identity = physical_semantic_identity(
                &self.build_identity,
                &self.freeze56_descriptor_digest,
                &semantic_table,
                &choose_table,
                record,
            );
        }

        self.sections.global.digests[7] =
            reduced_identity_stream_digest(&self.sections.reduced_bindings.records)?;
        self.sections.global.digests[8] =
            physical_identity_stream_digest(&self.sections.physical_bindings.records)?;
        let section_bytes = self.encode_sections_1_through_9()?;
        let section_digests = section_digests_1_through_9(&section_bytes)?;
        self.sections.global.digests[9] = success_conjunction_digest(
            &self.build_identity,
            &self.freeze56_descriptor_digest,
            accepted_payload_bytes,
            &section_digests,
        );
        Ok(())
    }

    pub fn encode(&self) -> Result<Vec<u8>> {
        self.validate()?;
        let mut section_bytes = self.encode_sections_1_through_9()?;
        section_bytes.push(self.sections.global.encode()?);
        let mut directories = Vec::with_capacity(SUCCESS_SECTION_COUNT);
        let mut offset = u64::try_from(SUCCESS_HEADER_BYTES)
            .map_err(|_| CodecError::LengthOverflow("success header u64"))?;
        for (index, bytes) in section_bytes.iter().enumerate() {
            let tag = SectionTag::try_from(
                u16::try_from(index + 1).map_err(|_| CodecError::LengthOverflow("section tag"))?,
            )?;
            let length = u64::try_from(bytes.len())
                .map_err(|_| CodecError::LengthOverflow("section length u64"))?;
            let record_count = SECTION_RECORD_COUNTS[index];
            directories.push(SectionDirectoryEntry {
                tag,
                offset,
                length,
                record_count,
                digest: section_digest(tag, record_count, bytes)?,
            });
            offset = offset
                .checked_add(length)
                .ok_or(CodecError::LengthOverflow("total receipt bytes"))?;
        }

        let accepted_payload_bytes = self.sections.context_tasks.accepted_payload_bytes()?;
        let total = checked_usize(offset, "total receipt usize")?;
        let mut out = Vec::with_capacity(total);
        out.extend_from_slice(&SUCCESS_MAGIC);
        put_u16(&mut out, 1);
        put_u16(&mut out, 768);
        put_u32(&mut out, 1);
        put_u64(&mut out, offset);
        put_u32(&mut out, 10);
        put_u32(&mut out, 0);
        put_u32(&mut out, 16_384);
        put_u32(&mut out, 614);
        put_u32(&mut out, 103);
        put_u32(&mut out, 1_015);
        put_u32(&mut out, 0);
        put_u32(&mut out, 0);
        put_u64(&mut out, accepted_payload_bytes);
        put_digest(&mut out, &self.build_identity);
        put_digest(&mut out, &self.freeze56_descriptor_digest);
        for directory in &directories {
            out.extend_from_slice(&directory.encode());
        }
        require_eq(&out.len(), &SUCCESS_HEADER_BYTES, "success header length")?;
        for bytes in section_bytes {
            out.extend_from_slice(&bytes);
        }
        require_eq(&out.len(), &total, "total encoded receipt length")?;
        Ok(out)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < SUCCESS_HEADER_BYTES {
            return Err(CodecError::Invalid("success receipt minimum length"));
        }
        let mut header = Reader::new(&bytes[..SUCCESS_HEADER_BYTES]);
        require_eq(&header.array::<8>()?, &SUCCESS_MAGIC, "success magic")?;
        require_eq(&header.u16()?, &1, "success format version")?;
        require_eq(&header.u16()?, &768, "success header bytes")?;
        require_eq(&header.u32()?, &1, "success outcome")?;
        let total_receipt_bytes = checked_usize(header.u64()?, "total receipt usize")?;
        require_eq(&total_receipt_bytes, &bytes.len(), "total receipt bytes")?;
        require_eq(&header.u32()?, &10, "success section count")?;
        require_eq(&header.u32()?, &0, "success header reserved")?;
        require_eq(&header.u32()?, &16_384, "accepted arithmetic count")?;
        require_eq(&header.u32()?, &614, "accepted context count")?;
        require_eq(&header.u32()?, &103, "accepted reduced count")?;
        require_eq(&header.u32()?, &1_015, "accepted physical count")?;
        require_eq(&header.u32()?, &0, "success mismatch count")?;
        require_eq(&header.u32()?, &0, "success partial result")?;
        let header_accepted_payload_bytes = header.u64()?;
        let build_identity = header.digest()?;
        let freeze56_descriptor_digest = header.digest()?;
        let mut directories = Vec::with_capacity(SUCCESS_SECTION_COUNT);
        for _ in 0..SUCCESS_SECTION_COUNT {
            directories.push(SectionDirectoryEntry::read_from(&mut header)?);
        }
        header.finish()?;

        let mut expected_offset = SUCCESS_HEADER_BYTES as u64;
        let mut slices: Vec<&[u8]> = Vec::with_capacity(SUCCESS_SECTION_COUNT);
        for (index, directory) in directories.iter().enumerate() {
            let tag = SectionTag::try_from(
                u16::try_from(index + 1)
                    .map_err(|_| CodecError::LengthOverflow("directory tag"))?,
            )?;
            require_eq(&directory.tag, &tag, "directory tag/order")?;
            require_eq(
                &directory.offset,
                &expected_offset,
                "directory contiguous offset",
            )?;
            require_eq(
                &directory.record_count,
                &SECTION_RECORD_COUNTS[index],
                "directory record count",
            )?;
            let start = checked_usize(directory.offset, "section offset usize")?;
            let end_u64 = directory
                .offset
                .checked_add(directory.length)
                .ok_or(CodecError::LengthOverflow("section end"))?;
            let end = checked_usize(end_u64, "section end usize")?;
            if end > bytes.len() {
                return Err(CodecError::Invalid("directory section range"));
            }
            let slice = &bytes[start..end];
            require_eq(
                &section_digest(tag, directory.record_count, slice)?,
                &directory.digest,
                "directory section digest",
            )?;
            slices.push(slice);
            expected_offset = end_u64;
        }
        require_eq(
            &checked_usize(expected_offset, "receipt end usize")?,
            &bytes.len(),
            "receipt exact end",
        )?;

        let value = Self {
            build_identity,
            freeze56_descriptor_digest,
            sections: ReceiptSections {
                authority: AuthoritySection::decode(slices[0])?,
                toolchain: ToolchainSection::decode(slices[1])?,
                device: DeviceSection::decode(slices[2])?,
                tables_and_abi: TablesAndAbiSection::decode(slices[3])?,
                arithmetic: ArithmeticSection::decode(slices[4])?,
                carrier: CarrierSection::decode(slices[5])?,
                context_tasks: ContextTasksSection::decode(slices[6])?,
                reduced_bindings: ReducedBindingsSection::decode(slices[7])?,
                physical_bindings: PhysicalBindingsSection::decode(slices[8])?,
                global: GlobalSection::decode(slices[9])?,
            },
        };
        require_eq(
            &value.sections.carrier.accepted_payload_bytes,
            &header_accepted_payload_bytes,
            "header/carrier accepted payload bytes",
        )?;
        value.validate()?;
        Ok(value)
    }

    pub fn validate(&self) -> Result<()> {
        if self.build_identity == ZERO_DIGEST || self.freeze56_descriptor_digest == ZERO_DIGEST {
            return Err(CodecError::Invalid("zero success authority digest"));
        }
        self.sections.authority.validate(
            Some(&self.build_identity),
            Some(&self.freeze56_descriptor_digest),
        )?;
        self.sections.toolchain.validate()?;
        self.sections.device.validate()?;
        self.sections.tables_and_abi.validate()?;
        self.sections.arithmetic.validate()?;
        self.sections.context_tasks.validate()?;
        let canonical_carrier = regenerate_canonical_carrier()?;
        validate_context_tasks_against_carrier(&self.sections.context_tasks, &canonical_carrier)?;

        let accepted_payload_bytes = self.sections.context_tasks.accepted_payload_bytes()?;
        require_eq(
            &self.sections.carrier.accepted_payload_bytes,
            &accepted_payload_bytes,
            "carrier accepted payload bytes",
        )?;
        require_eq(
            &self.sections.carrier.task_key_stream_digest,
            &stream_digest(
                StreamPurpose::TaskKeys,
                614,
                &self.sections.context_tasks.task_key_stream(),
            )?,
            "carrier task-key stream digest",
        )?;
        require_eq(
            &self.sections.carrier.task_key_stream_digest,
            &canonical_carrier.task_key_stream_sha256(),
            "canonical carrier task-key digest",
        )?;
        let (task_input_chain, choose_input_chain) =
            canonical_input_hash_chain_digests(&canonical_carrier)?;
        require_eq(
            &self.sections.carrier.task_input_hash_chain_digest,
            &task_input_chain,
            "canonical task input hash-chain digest",
        )?;
        require_eq(
            &self.sections.carrier.choose_input_hash_chain_digest,
            &choose_input_chain,
            "canonical choose input hash-chain digest",
        )?;
        self.validate_bindings(&canonical_carrier)?;
        self.sections.global.validate_pairs()?;
        let canonical_opening = canonical_opening_receipt_facts_v1()
            .map_err(|_| CodecError::Invalid("canonical opening receipt regeneration"))?;
        let canonical_global =
            canonical_global_observation_digests(&canonical_carrier, &canonical_opening)?;
        for (actual, expected) in self.sections.global.digests[..7]
            .iter()
            .zip(canonical_global.iter())
        {
            require_eq(actual, expected, "canonical global observation digest")?;
        }
        require_eq(
            &self.sections.global.digests[7],
            &reduced_identity_stream_digest(&self.sections.reduced_bindings.records)?,
            "global reduced identity stream",
        )?;
        require_eq(
            &self.sections.global.digests[8],
            &physical_identity_stream_digest(&self.sections.physical_bindings.records)?,
            "global physical identity stream",
        )?;
        let section_bytes = self.encode_sections_1_through_9()?;
        let section_digests = section_digests_1_through_9(&section_bytes)?;
        require_eq(
            &self.sections.global.digests[9],
            &success_conjunction_digest(
                &self.build_identity,
                &self.freeze56_descriptor_digest,
                accepted_payload_bytes,
                &section_digests,
            ),
            "success conjunction",
        )?;
        Ok(())
    }

    fn encode_sections_1_through_9(&self) -> Result<Vec<Vec<u8>>> {
        Ok(vec![
            self.sections.authority.encode()?,
            self.sections.toolchain.encode()?,
            self.sections.device.encode()?,
            self.sections.tables_and_abi.encode()?,
            self.sections.arithmetic.encode()?,
            self.sections.carrier.encode()?,
            self.sections.context_tasks.encode()?,
            self.sections.reduced_bindings.encode()?,
            self.sections.physical_bindings.encode()?,
        ])
    }

    fn validate_bindings(&self, carrier: &M2OpeningParityCarrierV1) -> Result<()> {
        require_eq(
            &self.sections.reduced_bindings.records.len(),
            &REDUCED_BINDING_COUNT,
            "reduced binding count",
        )?;
        require_eq(
            &self.sections.physical_bindings.records.len(),
            &PHYSICAL_BINDING_COUNT,
            "physical binding count",
        )?;
        let semantic_table = self.sections.tables_and_abi.semantic_table.digest;
        let choose_table = self.sections.tables_and_abi.choose_table.digest;
        let (canonical_reduced, canonical_physical) =
            canonical_binding_sections_from_carrier(&self.sections.context_tasks, carrier)?;
        for (index, record) in self.sections.reduced_bindings.records.iter().enumerate() {
            record.validate_shape()?;
            let mut expected = canonical_reduced.records[index].clone();
            expected.semantic_identity = record.semantic_identity;
            require_eq(record, &expected, "canonical reduced binding")?;
            require_eq(
                &record.binding_ordinal,
                &u32::try_from(index)
                    .map_err(|_| CodecError::LengthOverflow("reduced binding ordinal"))?,
                "reduced binding ordinal",
            )?;
            require_eq(
                &record.task_ordinal,
                &u32::try_from(index)
                    .map_err(|_| CodecError::LengthOverflow("reduced task ordinal"))?,
                "reduced binding task census",
            )?;
            let context = self.context(record.task_ordinal)?;
            validate_root_action(
                &record.root_key,
                context.key.declaration,
                record.selected_action,
                record.derived_context,
            )?;
            require_eq(
                &record.selected_action,
                &least_root_action_for_context(&record.root_key, record.derived_context)?,
                "reduced least context action",
            )?;
            if context.key.pool_mask & root_hand_mask(&record.root_key)? != 0 {
                return Err(CodecError::Invalid("reduced pool intersects root hand"));
            }
            require_eq(&record.arm, &context.key.arm, "reduced binding arm")?;
            require_eq(
                &record.arm_ordinal,
                &context.key.arm_ordinal,
                "reduced binding arm ordinal",
            )?;
            require_eq(
                &u32::from(record.derived_context),
                &context.key.led,
                "reduced binding derived context",
            )?;
            require_eq(
                &u32::from(record.grade),
                &context.key.grade,
                "reduced binding grade",
            )?;
            require_eq(
                &u32::from(record.matching_count),
                &context.key.matching_mask.count_ones(),
                "reduced binding matching count",
            )?;
            require_eq(
                &record.reduced_pool_mask,
                &context.key.pool_mask,
                "reduced binding pool",
            )?;
            require_eq(
                &record.payload_bytes,
                &context.canonical_payload_bytes,
                "reduced binding payload length",
            )?;
            require_eq(
                &record.payload_digest,
                &context.gpu_payload_digest,
                "reduced binding payload digest",
            )?;
            require_eq(
                &record.semantic_identity,
                &reduced_semantic_identity(
                    &self.build_identity,
                    &self.freeze56_descriptor_digest,
                    &semantic_table,
                    &choose_table,
                    record,
                    &context.key,
                ),
                "reduced semantic identity",
            )?;
        }

        let mut grade_physical = 0usize;
        let mut pair_physical = 0usize;
        for (index, record) in self.sections.physical_bindings.records.iter().enumerate() {
            record.validate_shape()?;
            let mut expected = canonical_physical.records[index].clone();
            expected.semantic_identity = record.semantic_identity;
            require_eq(record, &expected, "canonical physical binding")?;
            require_eq(
                &record.binding_ordinal,
                &u32::try_from(index)
                    .map_err(|_| CodecError::LengthOverflow("physical binding ordinal"))?,
                "physical binding ordinal",
            )?;
            let expected_coordinate = if index < 7 {
                (
                    103 + u32::try_from(index)
                        .map_err(|_| CodecError::LengthOverflow("grade physical ordinal"))?,
                    0,
                )
            } else {
                let pair_offset = index - 7;
                (
                    110 + u32::try_from(pair_offset / 2)
                        .map_err(|_| CodecError::LengthOverflow("pair task ordinal"))?,
                    u32::try_from(pair_offset % 2)
                        .map_err(|_| CodecError::LengthOverflow("pair endpoint"))?,
                )
            };
            require_eq(
                &(record.task_ordinal, record.endpoint),
                &expected_coordinate,
                "physical binding task/endpoint census",
            )?;
            let context = self.context(record.task_ordinal)?;
            validate_root_action(
                &record.root_key,
                context.key.declaration,
                record.selected_action,
                record.derived_context,
            )?;
            require_eq(
                &record.context_pool_mask,
                &(!root_hand_mask(&record.root_key)? & 0x0fff_ffff),
                "physical context pool/root hidden pool",
            )?;
            if record.arm == Arm::GradeMatching {
                require_eq(
                    &record.selected_action,
                    &least_root_action_for_context(&record.root_key, record.derived_context)?,
                    "grade physical least context action",
                )?;
            }
            require_eq(&record.arm, &context.key.arm, "physical binding arm")?;
            require_eq(
                &record.arm_ordinal,
                &context.key.arm_ordinal,
                "physical binding arm ordinal",
            )?;
            require_eq(
                &u32::from(record.derived_context),
                &context.key.led,
                "physical binding derived context",
            )?;
            require_eq(&context.key.grade, &7, "physical binding task grade")?;
            require_eq(
                &record.context_pool_mask,
                &context.key.pool_mask,
                "physical binding pool",
            )?;
            require_eq(
                &record.payload_bytes,
                &context.canonical_payload_bytes,
                "physical binding payload length",
            )?;
            require_eq(
                &record.payload_digest,
                &context.gpu_payload_digest,
                "physical binding payload digest",
            )?;
            require_eq(
                &record.semantic_identity,
                &physical_semantic_identity(
                    &self.build_identity,
                    &self.freeze56_descriptor_digest,
                    &semantic_table,
                    &choose_table,
                    record,
                ),
                "physical semantic identity",
            )?;
            match record.arm {
                Arm::GradeMatching => grade_physical += 1,
                Arm::SameContextPair => pair_physical += 1,
                Arm::Reduced => return Err(CodecError::Invalid("physical Reduced arm")),
            }
        }
        require_eq(&grade_physical, &7, "grade-7 physical binding count")?;
        require_eq(
            &pair_physical,
            &1_008,
            "same-context physical binding count",
        )?;
        for pair in self.sections.physical_bindings.records[7..].chunks_exact(2) {
            require_eq(
                &pair[0].task_ordinal,
                &pair[1].task_ordinal,
                "pair task ordinal",
            )?;
            require_eq(&pair[0].endpoint, &0, "pair endpoint zero")?;
            require_eq(&pair[1].endpoint, &1, "pair endpoint one")?;
            require_eq(&pair[0].root_key, &pair[1].root_key, "pair fixture root")?;
            if pair[0].selected_action >= pair[1].selected_action
                || pair[0].semantic_identity == pair[1].semantic_identity
            {
                return Err(CodecError::Invalid("same-context distinct endpoints"));
            }
        }
        Ok(())
    }

    fn context(&self, ordinal: u32) -> Result<&ContextTaskRecord> {
        self.sections
            .context_tasks
            .records
            .get(
                usize::try_from(ordinal)
                    .map_err(|_| CodecError::LengthOverflow("task ordinal usize"))?,
            )
            .ok_or(CodecError::Invalid("binding task ordinal"))
    }
}

pub fn reduced_semantic_identity(
    build_identity: &Digest,
    freeze56_digest: &Digest,
    semantic_table_digest: &Digest,
    choose_table_digest: &Digest,
    record: &ReducedBindingRecord,
    key: &TaskKey,
) -> Digest {
    let mut preimage = Vec::with_capacity(289);
    preimage.extend_from_slice(b"W42M2RED");
    put_u32(&mut preimage, 1);
    put_digest(&mut preimage, build_identity);
    put_digest(&mut preimage, freeze56_digest);
    put_digest(&mut preimage, semantic_table_digest);
    put_digest(&mut preimage, choose_table_digest);
    put_u32(&mut preimage, 37);
    preimage.extend_from_slice(&record.root_key);
    for value in [
        u32::from(record.selected_action),
        key.arm.into(),
        key.generator_a,
        key.generator_b,
        key.generator_c,
        key.declaration,
        key.led,
        key.grade,
        key.pool_mask,
        key.matching_mask,
    ] {
        put_u32(&mut preimage, value);
    }
    put_u64(&mut preimage, record.payload_bytes);
    put_digest(&mut preimage, &record.payload_digest);
    sha256(&preimage)
}

pub fn physical_semantic_identity(
    build_identity: &Digest,
    freeze56_digest: &Digest,
    semantic_table_digest: &Digest,
    choose_table_digest: &Digest,
    record: &PhysicalBindingRecord,
) -> Digest {
    let mut preimage = Vec::with_capacity(265);
    preimage.extend_from_slice(b"W42M2PHY");
    put_u32(&mut preimage, 1);
    put_digest(&mut preimage, build_identity);
    put_digest(&mut preimage, freeze56_digest);
    put_digest(&mut preimage, semantic_table_digest);
    put_digest(&mut preimage, choose_table_digest);
    put_u32(&mut preimage, 37);
    preimage.extend_from_slice(&record.root_key);
    put_u32(&mut preimage, u32::from(record.selected_action));
    put_u32(&mut preimage, u32::from(record.derived_context));
    put_u32(&mut preimage, record.context_pool_mask);
    put_u64(&mut preimage, record.payload_bytes);
    put_digest(&mut preimage, &record.payload_digest);
    sha256(&preimage)
}

pub fn reduced_identity_stream_digest(records: &[ReducedBindingRecord]) -> Result<Digest> {
    let mut payload = Vec::with_capacity(records.len() * IDENTITY_STREAM_RECORD_BYTES);
    for record in records {
        payload.extend_from_slice(
            &IdentityStreamRecord {
                binding_ordinal: record.binding_ordinal,
                identity: record.semantic_identity,
            }
            .encode(),
        );
    }
    stream_digest(
        StreamPurpose::ReducedIdentities,
        u64::try_from(records.len())
            .map_err(|_| CodecError::LengthOverflow("reduced identity count"))?,
        &payload,
    )
}

pub fn physical_identity_stream_digest(records: &[PhysicalBindingRecord]) -> Result<Digest> {
    let mut payload = Vec::with_capacity(records.len() * IDENTITY_STREAM_RECORD_BYTES);
    for record in records {
        payload.extend_from_slice(
            &IdentityStreamRecord {
                binding_ordinal: record.binding_ordinal,
                identity: record.semantic_identity,
            }
            .encode(),
        );
    }
    stream_digest(
        StreamPurpose::PhysicalIdentities,
        u64::try_from(records.len())
            .map_err(|_| CodecError::LengthOverflow("physical identity count"))?,
        &payload,
    )
}

pub fn protected_chain_digest(records: &[ProtectedChainRecord]) -> Result<Digest> {
    require_eq(&records.len(), &629, "protected chain record count")?;
    let mut payload = Vec::with_capacity(records.len() * PROTECTED_CHAIN_RECORD_BYTES);
    for (index, record) in records.iter().enumerate() {
        record.validate()?;
        let expected = match index {
            0 => (1, 0),
            1 => (2, 0),
            2..=14 => (3, u32::try_from(index - 2).unwrap_or(u32::MAX)),
            _ => (4, u32::try_from(index - 15).unwrap_or(u32::MAX)),
        };
        require_eq(
            &(record.domain, record.ordinal),
            &expected,
            "protected chain order",
        )?;
        payload.extend_from_slice(&record.encode());
    }
    stream_digest(StreamPurpose::GlobalProtectedChain, 629, &payload)
}

fn section_digests_1_through_9(section_bytes: &[Vec<u8>]) -> Result<[Digest; 9]> {
    require_eq(&section_bytes.len(), &9, "section digest input count")?;
    let mut digests = [[0; 32]; 9];
    for (index, bytes) in section_bytes.iter().enumerate() {
        let tag = SectionTag::try_from(
            u16::try_from(index + 1)
                .map_err(|_| CodecError::LengthOverflow("section digest tag"))?,
        )?;
        digests[index] = section_digest(tag, SECTION_RECORD_COUNTS[index], bytes)?;
    }
    Ok(digests)
}

pub fn success_conjunction_digest(
    build_identity: &Digest,
    freeze56_digest: &Digest,
    accepted_payload_bytes: u64,
    section_digests: &[Digest; 9],
) -> Digest {
    let mut preimage = Vec::with_capacity(8 + 4 + 32 + 32 + 16 + 8 + 8 + 9 * 32);
    preimage.extend_from_slice(b"W42M2CON");
    put_u32(&mut preimage, 1);
    put_digest(&mut preimage, build_identity);
    put_digest(&mut preimage, freeze56_digest);
    for count in [16_384, 614, 103, 1_015] {
        put_u32(&mut preimage, count);
    }
    put_u64(&mut preimage, accepted_payload_bytes);
    put_u32(&mut preimage, 0);
    put_u32(&mut preimage, 0);
    for digest in section_digests {
        put_digest(&mut preimage, digest);
    }
    sha256(&preimage)
}
