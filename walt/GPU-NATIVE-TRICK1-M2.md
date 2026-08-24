# GPU-native trick-1 — binding M2 Metal contract v1

**Status:** pre-freeze binding candidate; becomes authority only when its exact
SHA-256 is named by the append-only freeze-56 ruling.  Until then it authorizes
no implementation claim.  **Target:** M2 arithmetic and opening-projector Metal
parity.  **Parent:** freeze 55 through GT1-A9.

This contract incorporates, as normative text, the complete 44,079-byte
`math/gpu_native_trick1_m2_rebrief_v0.1.md` whose SHA-256 is
`9183132529a42289a104a73d8f7e196eb95058ac2edda60bb42c715f1f8a139a`.
Where this file is more specific it closes that rebrief; it never widens it.
The received v0.2 guide and v0.3 M0/M1 contract remain preserved inputs, but
freeze 56 and this file govern M2.

The only successful M2 claim is the exact UTF-8 sentence

```text
M2 METAL PROJECTOR PARITY COMPLETE under freeze 56
```

It means that every conjunct below passed.  It does not mean that Walt chose a
lead, valued an action, crossed K-OPEN4+, built an information net, measured a
speedup, or became a player.

---

## 1. Authority and immutable ancestry

The implementation verifies these historical identities before M2 work:

```text
parent source commit                 3b4c6d60fef371e3050de151ccf9eaefbc2d2da7
M0/M1 manifest and build identity    eccf0a3742e2cfc50cad158292db7ad8c6145da8aa7958b7aa2ed07a1566f2ad
freeze-55 descriptor                 9b181092045b003893cae7c09cc7b7c8b57f75c3c5c4cf7043b8d428df738efa
opening envelope                     1127d3868d7da07c26a7b8bc031ac8a63ba84a9068df786b67a413ea6af5f517
grade-5 declared stop                7e8dfecf1cac314ae6e71b406eb268b29d4157206ce5e64d1c50d1aa94d43bdf
M0/M1 receipt summary                51a162ea933801f05b852ec2a454c48a31c7d292ee8273ba683d0a7fec340b12
v0.3 contract                        6190e740a0579b6b5196e086e52c8022d4cddcd0f746ecbd9226f87bbc0e4790
historical Gate-0 NO-GO              b57f7077e5aa0aa1d8030a76a3399076810b71b1623ad83e001aee2b4aaeb215
received v0.2 guide                  ee2e78da20eb7d087fb121f467a56bafc0179a45fb692ca0b938f4c4210b6a44
M2 rebrief                           9183132529a42289a104a73d8f7e196eb95058ac2edda60bb42c715f1f8a139a
```

The old manifest is checked against blobs from the parent commit, not current
paths.  The current `CENSUS-RULINGS.md` must begin byte-for-byte with the parent
blob.  The old receipts and their checksum are immutable comparands.  A replay
with current validators is labelled compatibility replay and is not represented
as historical production.

Freeze 56 contains this contract's digest and parent identities.  This contract
does not contain its own digest, the freeze-56 descriptor digest, the M2 build
identity, an M2 semantic identity, a receipt digest, or a final commit.  Thus the
identity graph is acyclic.

---

## 2. Build, platform and shader identity

The official target is little-endian `aarch64-apple-darwin`, Rust 1.95.0, release
profile, overflow checks enabled and Cargo `--locked`.  The checked lockfile fixes:

```text
objc2                 0.6.4
objc2-core-graphics   0.3.2, default features off, linkage only
objc2-foundation      0.3.2, default features off; NSError NSObject NSString
dispatch2             0.3.1, default features off; alloc block2 objc2
objc2-metal           0.3.2, default features off; std dispatch2 MTLAllocation
                      MTLBuffer MTLCommandBuffer MTLCommandEncoder
                      MTLCommandQueue MTLComputeCommandEncoder
                      MTLComputePipeline MTLDevice MTLLibrary MTLResource MTLTypes
```

`walt-metal` imports `objc2_core_graphics as _` for linkage.  It uses no
deprecated `metal-rs`, `wgpu`, handwritten Objective-C surface or build script.
Ambient `RUSTFLAGS`, `RUSTDOCFLAGS`, `CARGO_ENCODED_RUSTFLAGS`,
`RUSTC_WRAPPER`, `RUSTC_WORKSPACE_WRAPPER`, `MACOSX_DEPLOYMENT_TARGET`,
`SDKROOT`, `DEVELOPER_DIR` and `TOOLCHAINS` must be absent at runner entry; the
checked script sets only the explicit developer/toolchain selection it records.

The canonical shader sources are, in this lexical order:

```text
walt-metal/shaders/00_u256.metal
walt-metal/shaders/01_opening_projector.metal
```

Each is compiled separately.  The two exact normalized argument lists after
`metal` are:

```text
-std=metal3.2 -mmacosx-version-min=26.0 -fmetal-math-mode=safe
-fno-fast-math -Wall -Wextra -Werror -c
<SOURCE_DIR>/00_u256.metal -o <AIR_DIR>/00_u256.air

-std=metal3.2 -mmacosx-version-min=26.0 -fmetal-math-mode=safe
-fno-fast-math -Wall -Wextra -Werror -c
<SOURCE_DIR>/01_opening_projector.metal -o <AIR_DIR>/01_opening_projector.air
```

The AIR inputs are passed to `metallib` in the same lexical order followed by
the exact normalized arguments
`<AIR_DIR>/00_u256.air <AIR_DIR>/01_opening_projector.air -o <OUTPUT>`.
Whitespace above is presentation only: the compile lists each contain eleven
arguments and the link list contains four.  `<SOURCE_DIR>` is a fresh directory
holding byte-identical copies with exactly those basenames; `<AIR_DIR>` is a
fresh output directory; `<OUTPUT>` is that run's final library path.  Source
records use repository-root-relative paths
`walt/walt-metal/shaders/00_u256.metal` and
`walt/walt-metal/shaders/01_opening_projector.metal`; placeholder expansion is
never serialized.  The executable is resolved through
`xcrun --toolchain com.apple.dt.toolchain.Metal.32023.883`; the component is
build 17F109, compiler and AIR linker are version 32023.883, Xcode is 26.6 build
17F113 and SDK is 26.5.  The receipt records only the normalized logical argv
with the exact `<SOURCE_DIR>`, `<AIR_DIR>` and `<OUTPUT>` placeholders above,
never expanded temporary paths.

AIR is transient and noncanonical because it embeds source paths.  Before this
contract was frozen, two fresh-directory compiles with identical source bytes
produced different AIR bytes but identical 6,877-byte metallibs with SHA-256
`7ee317704bd06685bee227bfb208cfc53a1e9b143a23c0b1aa1deda69c8760be`.
That digest is a recipe smoke result, not the digest of the final library.  The
final gate repeats the two-directory build and requires both final libraries and
the checked-in `walt-metal/shaders/walt_m2.metallib` to be byte-identical.

The tool descriptor records version strings and SHA-256 for the resolved
`metal`, `metallib`, `metal-ar`, `xcodebuild` and `xctrace` executables.  It
records the sanitized device name, unified-memory flag, maximum buffer length,
recommended working-set size, device maximum thread dimensions and threadgroup
memory, plus each pipeline's execution width, maximum threads and static
threadgroup memory.  It excludes registry IDs, serials, paths and timestamps.
Driver-reported allocated length may exceed a logical length but never reduces
the logical byte range, changes hashing, or admits access beyond that range.

---

## 3. Crate and trust boundary

`walt-gpu-ref` remains portable.  Its private `m2` implementation owns the
canonical carrier, scalar comparands, choose-table extraction, task keys, pure
slot validation, binding intents and receipt encoders.  It exposes no public
constructor for an official task, checked payload, binding or accepted result.

The pure validator returns a non-`Clone`, non-serializable
`CheckedM2ProjectionPayloadV1`.  Its payload bytes are private.  Its name and
type make no GPU provenance claim: CPU bytes can satisfy a pure equality check.
The token contains task ordinal and key digest, context, response/slot/cell
counts, total mass, raw-stream digest, payload digest and length.

`walt-metal` alone joins a completed retained command with that checked token.
`AcceptedMetalOpeningTaskV1` and `AcceptedMetalArithmeticV1` are move-only,
privately constructed tokens.  The opening token immediately consumes the
task's private binding intent.  No raw arena crosses the public `walt-metal`
boundary and no API accepts a caller-supplied root, action or binding kind.

There is no public `from_words_unchecked`, `assume_completed`, malformed-task
constructor, `OpeningCell::new`, `OpeningProjection::from_cells`, checked-token
constructor or accepted-token constructor.  The thirteen malformed inputs in
each domain exist only through closed private negative-control generators.

The only `repr(C)` ABI structures are private to `walt-metal` and contain one
field, `words: [u32; N]`.  Compile-time assertions require sizes/alignment
`(32,4)`, `(64,4)`, `(80,4)`, `(64,4)` for task, slot, arithmetic input and
arithmetic output.  Hashing always appends `u32::to_le_bytes`; it never hashes
Rust struct memory.

The crate forbids unsafe code except in one private module.  That module owns
exactly three unsafe operation classes: validated host-word copy into a shared
buffer, completed shared-buffer copy into owned words, and validated buffer/
offset-zero encoder binding.  Each wrapper checks multiplication, logical and
reported buffer length, four-byte alignment, non-null contents, ownership and
completion provenance.  No pointer or borrowed buffer storage escapes.

No Walt Rust/MSL proof-path type or operation is floating point.  Generated but
unused binding declarations outside the selected feature/use path do not count
as Walt operations.  Rust compilation denies float arithmetic; source gates
reject `f32`, `f64`, suffixes and inferred decimal/exponent literals.  MSL gates
reject scalar/vector/matrix/packed/simdgroup half, float, double and bfloat types
and decimal/exponent/inf/nan numeric tokens after stripping comments.

---

## 4. Metal entry points and dispatch

The exact signatures are:

```metal
kernel void u256_parity_v1(
    device const ArithmeticInputV1* inputs [[buffer(0)]],
    device ArithmeticOutputV1* outputs [[buffer(1)]],
    uint gid [[thread_position_in_grid]]);

kernel void opening_project_v1(
    device const OpeningTaskV1* task [[buffer(0)]],
    device const OpeningChooseTableV1* choose [[buffer(1)]],
    device OpeningSlotV1* slots [[buffer(2)]],
    uint q [[thread_position_in_grid]]);
```

Every MSL ABI struct is exactly `uint words[N]`.  `task[0]` and `choose[0]` are
the only opening input records.  Buffers use shared storage, indices shown above
and offset zero.  The opening logical lengths are 32, 1,936 and 5,107,328 bytes.
The arithmetic logical lengths are 1,310,720 and 1,048,704 bytes.  All are
allocated before the first official dispatch.

Both kernels use `dispatchThreads:threadsPerThreadgroup:` with threadgroup
`(32,1,1)` and never round a logical grid.  Official arithmetic uses grid
`(16,384,1,1)`, a 16,384-record input range and a 16,386-record output/guard
range.  Arithmetic negative control uses grid `(13,1,1)`, 13 input records
(1,040 bytes) and 15 output/guard records (960 bytes).  An official opening uses
grid `(response_triple_count,1,1)` and the fixed 79,802-record output range.  Each
opening negative control uses grid `(1,1,1)` and its private twelve-record output
range (768 bytes: ten owned records and two guards).  The Gate-0 empty command
has no encoder, grid or buffer range.  Pipeline maximum threads must be at least
32.  One command buffer contains at most one task; commands are committed and
completed sequentially in official ordinal order.  There are no atomics,
reductions, scans, indirect dispatches, slabs or concurrent command buffers.

One opening thread owns response ordinal `q` and slots `10*q..10*q+9`.  Its
first action initializes all sixteen words of all ten slots to canonical SKIP.
An out-of-grid defensive `q` returns before writing because it owns no storage.
Every in-range ordinal must decode; failure after descriptor validation is
`HARD_BAD_RESPONSE_ORDINAL`.  An unreachable checked helper failure not assigned
another hard status is `HARD_INTERNAL` for arithmetic and a run-fatal host
internal error for opening; the opening status registry is not silently widened.

---

## 5. Frozen ABIs and statuses

The complete `OpeningTaskV1`, `OpeningSlotV1`, `ArithmeticInputV1` and
`ArithmeticOutputV1` word layouts, status values, canonical SKIP/VALID/HARD
forms and validation precedence are exactly rebrief sections 7, 8 and 11.
The following clauses close possible implementation latitude:

- integer words are little-endian at every byte boundary;
- poison is `0xA5A55A5A` in every word;
- every output arena has exactly two logical guard records;
- SKIP/HARD preserve only status, task/case id and slot/opcode as specified and
  zero every semantic/reserved word;
- opening thread hard failure rewrites all ten owned slots to one reason;
- arithmetic malformed input is HARD, checked numeric overflow/underflow is
  `CHECKED_UNDEFINED`, and neither is accepted arithmetic evidence;
- unknown values or noncanonical combinations void the whole run;
- before/after hashes cover task, choose and arithmetic input bytes around every
  command and must match exactly.

Opening descriptor precedence is ABI, high mask bits, subset, grade, pool
popcount/count/`3*grade`, matching cap, response formula, slot formula.
Arithmetic precedence is ABI, opcode, operation-specific unused fields, exponent.
The exact thirteen controls in each domain and their order are rebrief sections 8
and 11.  The opening private bypass is memory-safe only after independently
requiring a one-thread grid, ten writable records and two guards; production
callers cannot invoke it.

---

## 6. Arithmetic algorithm and corpus

`U256` is eight least-significant-first `u32` limbs.  Addition visits limbs
0..7 using `ulong(lhs)+rhs+carry`; the low 32 bits are written and the next carry
is the high 32 bits.  Carry after limb 7 is undefined overflow.  Subtraction
visits limbs 0..7, subtracts rhs then borrow without unsigned wrap, writes the
low difference and propagates one borrow; borrow after limb 7 is undefined
underflow.  Comparison visits limbs 7..0 and returns at the first difference.

`mul_small` visits limbs 0..7 with `ulong(lhs[i])*factor+carry`; high 32 bits
become carry and nonzero carry after limb 7 is undefined.  `mul_pow_420` copies
lhs and applies that same checked multiplication exactly exponent times in
ascending iteration order; exponent zero is identity.  Undefined outputs are
all-zero and never expose a partial limb prefix.

`U256MetalCorpusV1` is the exact 16,384-case generator and ordering in rebrief
section 11: the 32-value edge list, 4,288-case prefix, then 12,096 SplitMix64
cases with fixed seed, update/mix sequence and nine-output consumption per case.
The independent oracle is `num_bigint::BigUint`; it shares no U256 limb
operation.  Oracle records use the 64-byte output ABI and the same canonical
undefined encoding.  Input, oracle and GPU streams each receive SHA-256.  The
complete oracle/GPU byte streams must match.

---

## 7. Choose table and projector algorithm

`OpeningChooseTableV1` is an opaque 484-word row-major `choose(n,k)` table for
`0<=n,k<=21`, zero when `k>n`, exactly 1,936 bytes.  Its sole production
constructor extracts the choose section from unchanged
`SemanticTablesCanonicalV2` and checks every entry against `BigUint`.  It is a
new identity; it never claims to be or replace the full semantic table.

For a task with sorted physical pool indices, response unranking and
matching-vector order are exactly rebrief sections 8 and 10.  The shader loops
physical bits 0..27 to select positions.  It does not apply trailing-zero count
to zero or shift by 32.  Matching vectors loop `e1`, then `e2`, then `e3`, each
ascending from zero through `grade-1`, retain only follower/void-valid vectors
whose sum is the remaining matching count, and write them consecutively.  More
than ten is hard failure.

For each retained vector, support is computed with ordered choose products
equivalent to

```text
A_g = |M'|! / product_followers(e_s!)
      * |N'|! / product_all((g-1-e_s)!)
```

by sequentially choosing each seat's matching count from the remaining matching
pool, then each seat's nonmatching count from the remaining nonmatching pool;
the third seat is checked against the remainder.  Multiplications occur in seat
order and are checked after every step.  Coefficient starts at one and, in seat
order, multiplies `420/(e_s+1)` for a follower and `420/g` for a void, requiring
exact division.  Mass is the checked product `support*coefficient`.  Zero support
is SKIP.  The asserted upper bounds and cap values are exactly rebrief section 10
and are checked rather than used as permission to wrap.

The independent raw renderer does not duplicate this algorithm.  It groups the
already validated scalar `project_closed_form` cells by response, independently
unranks every response ordinal, and places the group's existing lexicographic
cells into slots 0..9 followed by SKIP.  It consumes every scalar cell exactly
once and emits exactly `candidate_slot_count` records.

After successful completion the runner compares the entire in-range raw stream
word-for-word to that renderer; checks every tail and guard word for poison;
stable-compacts actual VALID records; and encodes the existing 50-byte
`W42M1R01` header plus 26-byte cell records directly from the GPU fields.  It
checks strict key order, duplicates, support, `support*coefficient=mass`, cell
and response totals, mass conservation, cap and byte equality with the scalar
payload.  It never constructs `OpeningCell` or `OpeningProjection` from GPU
bytes.  The raw arena is then discarded.

---

## 8. Carrier, task key and binding identities

The three carrier generators, exact roots, fixture rules, action rules, direct
admission/stops, same-context duplicate geometry and canonical order are exactly
rebrief section 12.  Generated assertions fix:

```text
tasks 614; reduced bindings 103; physical bindings 1,015
direct parity 73; direct stops 541; all binding instances 1,118
```

All 64 Reduced tasks and GradeMatching grades 1..6 (39 tasks) produce one
`ReducedEvidenceBindingV1`.  GradeMatching grade 7 produces seven physical
bindings.  Each SameContext task produces two physical bindings.  No other
classification is accepted.

Every ordinal is zero-based.  Global task ordinals are `0..613`.  Arm ordinals
restart at zero and are `0..63`, `0..45` and `0..503` for Reduced,
GradeMatching and SameContext.  Each Reduced root selector is zero or one and
its coordinate ordinal restarts at zero in that root's existing generator order.
Reduced binding ordinals restart at `0..102`; physical binding ordinals restart
at `0..1,014`.  Each section is ordered by referenced global task and then
endpoint.  GradeMatching physical endpoint is zero.  SameContext endpoint zero
is the lower physical domino index and endpoint one the higher.  Consequently
the evidence-instance coordinate is exactly `(arm, arm_ordinal, endpoint)` and
is not a semantic-identity salt.

`M2TaskKeyV1` is exactly 64 bytes, sixteen little-endian `u32` words:

```text
0   key version = 1
1   global task ordinal
2   arm: 1 Reduced, 2 GradeMatching, 3 SameContextPair
3   arm ordinal
4   declaration: pip 0..6, doubles 7, no-trump 8
5   led context: natural 0..6, called 7
6   grade
7   pool mask
8   matching mask
9   pool count
10  response-triple count
11  candidate-slot count
12  generator coordinate A
13  generator coordinate B
14  generator coordinate C
15  reserved = 0
```

Coordinates A/B/C are: Reduced root selector, coordinate ordinal within that
root, zero; GradeMatching grade, matching count, zero; SameContext declaration
code, first physical domino index, second physical domino index.  Every
redundant field is regenerated and compared.  The complete task-key stream is
the exact concatenation of the 614 key records, 39,296 bytes.  Its digest is the
`StreamDigestV1` TASK_KEYS digest defined below.

`ReducedEvidenceBindingV1` and `PhysicalActionBindingV1` are each fixed 160-byte
records whose exact layouts appear in section 10.  Their semantic identities
use these exact preimages:

```text
reduced:
  "W42M2RED" || u32(1) || M2BuildIdentityV1 || freeze56_sha256 ||
  semantic_table_sha256 || choose_table_sha256 || u32(37) || root_key[37] ||
  selected_action:u32 || arm:u32 || generator_a:u32 || generator_b:u32 ||
  generator_c:u32 || declaration:u32 || led:u32 || grade:u32 ||
  pool_mask:u32 || matching_mask:u32 || payload_length:u64 || payload_sha256

physical:
  "W42M2PHY" || u32(1) || M2BuildIdentityV1 || freeze56_sha256 ||
  semantic_table_sha256 || choose_table_sha256 || u32(37) || root_key[37] ||
  selected_action:u32 || derived_context:u32 || context_pool_mask:u32 ||
  payload_length:u64 || payload_sha256
```

Binding, task, arm, pair and endpoint ordinals are deliberately excluded from
physical semantic identity.  Thus equal physical semantics across different
pair instances remain equal.  Within each same-context pair the two actions,
and therefore identities, must differ.  Reduced validation reconstructs its
exact carrier coordinate and subset relation without claiming a physical
grade-7 context.  Physical validation requires grade 7 and exact action-derived
context.

---

## 9. Runner protocol and no-partial rule

The official parent and child use little-endian frames:

```text
u32 frame_payload_length
u16 protocol_version = 1
u16 kind: 1 PREPARING, 2 COMMITTED, 3 TERMINAL, 4 FINALIZING,
          5 SUCCESS, 6 FAILURE
u32 phase_or_command_ordinal
u32 unit_or_terminal_code
u32 detail_length
detail bytes
```

Progress details are empty.  SUCCESS and FAILURE details are the profile-specific
closed objects below.  The serialized
`frame_payload_length` equals `16 + detail_length`; the complete wire frame
including its four-byte length prefix is `20 + detail_length`.  The payload
field is at most `0x7fffffff`, so the complete frame is at most
`0x80000003` bytes.  Terminal codes are 1 COMPLETED, 2 ERROR,
3 TIMEOUT, 4 NOT_ENQUEUED, 5 NOT_COMMITTED, 6 SCHEDULED, 7 UNKNOWN.  Only
COMPLETED with no command error proceeds.

The discarded maximum smoke is a separate supervised child, outside receipt
evidence.  Its command census is Gate-0 empty command 0 followed by the one
grade-7/m=6 projector command 1.  After that child exits, each official receipt
run starts a new child and recreates device, queue, library, pipelines, buffers,
carrier, comparands and accumulator.  Its exact command ordinals are Gate-0
empty command 0, arithmetic negative controls 1, official arithmetic corpus 2,
the thirteen opening negative controls 3..15, then official projector tasks
16..629.  Thus every malformed opening task has its own one-thread command,
consistent with the one-projector-task-per-command rule.  Controls and smoke do
not enter accepted counts or payload digests; control summaries do enter their
closed receipt fields.  A smoke command or result is never reused.

All protocol ordinals are zero-based.  COMMITTED uses the command ordinal and
unit zero.  TERMINAL uses the same command ordinal and one closed terminal code.
Official PREPARING phases are numeric and ordered: 1 AUTHORITY unit 0;
2 TOOLCHAIN unit 0; 3 ARITHMETIC_AND_TABLES unit 0; 4 CARRIER units `0..613`;
5 SCALAR_COMPARANDS units `0..613`; 6 METAL_INIT unit 0.  After each TERMINAL,
phase 9 POST_COMMAND emits the just-validated command ordinal before another
commit.  After command 629, FINALIZING phase 7 BINDINGS emits units `0..1,117`
and phase 8 RECEIPT_SECTIONS emits units `0..9`, followed by SUCCESS with phase
and unit zero.  The parent rejects a missing, repeated, reordered or out-of-range
unit.  The smoke profile instead uses PREPARING phases 2, 3, 5 and 6 with unit
zero, commands 0..1 and matching phase-9 units, then SUCCESS containing exactly:

```text
magic[8]="W42M2SM1"; version:u32=1; command_count:u32=2
completed_count:u32=2; accepted_count:u32=0; payload_bytes:u64=0
```

That 32-byte control report is noncanonical and cannot parse as a receipt.
Official SUCCESS detail is exactly one complete success receipt.  FAILURE detail
is exactly the child-zeroed failure receipt defined below in either profile.

The accepted state machine and two watchdogs are exactly rebrief section 13.
Deadlines use `std::time::Instant`, never wall clock.  Poll intervals are at most
10 milliseconds.  A command's 120,000ms child deadline begins immediately after
successful commit; the 125,000ms parent deadline begins upon its flushed
COMMITTED frame and no other frame extends it.  On child timeout it flushes the
matching TERMINAL/TIMEOUT frame and exits 124 without unwinding.  Parent timeout,
crash, signal, malformed protocol or nonzero exit kills/reaps the child and
constructs the same zero-accepted failure.  No partial child success bytes are
retained.

---

## 10. Receipt encoding

All integers are unsigned little-endian unless explicitly `i32`.  Reserved fields
and unknown flag bits are zero.  `TextV1` is `byte_length:u32 || UTF-8 bytes`,
with no padding, NUL, CR or LF.  Canonical paths are relative, `/`-separated and
contain no empty, `.` or `..` component.  File identity is ordinary SHA-256 of
exact bytes.  Logical stream identity is:

```text
SHA256("W42M2DG1" || purpose:u32 || stream_version:u32 ||
       record_count:u64 || payload_bytes:u64 || payload)
```

`stream_version` is exactly 1 for every M2 stream in this contract.

Purpose codes are 1 TASK_KEYS, 2 ARITHMETIC_INPUT, 3 ARITHMETIC_OUTPUT,
4 CONTEXT_SLOT_STREAM, 5 CONTEXT_PAYLOAD, 6 CONTEXT_RESPONSE_AGGREGATES,
7 TASK_INPUT_HASH_CHAIN, 8 CHOOSE_INPUT_HASH_CHAIN, 9 PROTECTED_RECORDS,
10 REDUCED_IDENTITIES, 11 PHYSICAL_IDENTITIES, 12 GLOBAL_SLOT_STREAM,
13 GLOBAL_PAYLOAD_STREAM, 14 GLOBAL_RESPONSE_AGGREGATES and
15 GLOBAL_PROTECTED_CHAIN.  A section digest is

```text
SHA256("W42M2SC1" || section_tag:u32 || section_version:u32 ||
       record_count:u64 || section_bytes:u64 || exact_section_bytes)
```

Digest assignment is closed as follows.  Parent/source/tool/MSL/metallib/table/
contract/descriptor file identities and semantic identities use ordinary SHA-256
over the exact bytes or identity preimages expressly named.  Directory entries
use the section digest above.  Every other digest uses `StreamDigestV1` version 1:

- task keys use purpose 1, count 614 and the 39,296 concatenated key bytes;
- arithmetic input and non-guard output use purposes 2 and 3, each run's case
  count, and exact concatenated 80- or 64-byte ABI records;
- a context raw digest uses purpose 4, its candidate-slot count and exact
  concatenated 64-byte in-range records;
- a context/binding payload digest uses purpose 5, count 1 and the complete
  `W42M1R01` payload bytes;
- a context aggregate digest uses purpose 6, its nonempty-response count and
  concatenated 32-byte aggregate records;
- task and choose input chains use purposes 7 and 8, count 614 and concatenated
  72-byte chain records; each embedded pre/post digest is ordinary SHA-256 over
  the exact 32-byte task or 1,936-byte choose buffer;
- arithmetic guards use purpose 9, count 2 and their 128 raw bytes; an official
  context's tail/guard digest uses purpose 9, count
  `79,802-candidate_slot_count`, and raw 64-byte records from the first tail slot
  through the two guards; an opening negative digest uses purpose 9, count 12
  and its complete ten HARD plus two guard records;
- reduced and physical identity streams use purposes 10 and 11 and the exact
  40-byte records defined below;
- global raw, payload and aggregate streams use purposes 12, 13 and 14, count
  614 and their exact task-framed records defined below;
- the global protected chain uses purpose 15, count 629 and the exact 48-byte
  chain records defined below.

The two arithmetic guard pre/post fields are each their purpose-9 stream digest;
pre hashes canonical poison initialization and post hashes actual completed
guard bytes.  CPU/GPU and pre/post fields that must match nevertheless retain
their separately computed digest values.  No field may substitute ordinary
SHA-256 for its assigned stream digest or vice versa.

### 10.1 Success header and directory

The success receipt has a fixed 768-byte header and exactly ten sections:

```text
0     magic[8] = "W42M2R01"
8     format_version:u16 = 1
10    header_bytes:u16 = 768
12    outcome:u32 = 1
16    total_receipt_bytes:u64
24    section_count:u32 = 10
28    reserved:u32 = 0
32    accepted_arithmetic_case_count:u32 = 16,384
36    accepted_context_task_count:u32 = 614
40    accepted_reduced_binding_count:u32 = 103
44    accepted_physical_binding_count:u32 = 1,015
48    mismatch_count:u32 = 0
52    partial_result_retained:u32 = 0
56    accepted_payload_bytes:u64
64    M2BuildIdentityV1[32]
96    freeze56_descriptor_sha256[32]
128   ten SectionDirectoryEntryV1 records
```

Each 64-byte directory entry is section tag u16, version u16=1, flags u32=0,
offset u64, length u64, record count u64 and section digest[32].  Entries are
ordered 1 AUTHORITY, 2 TOOLCHAIN, 3 DEVICE, 4 TABLES_AND_ABI, 5 ARITHMETIC,
6 CARRIER, 7 CONTEXT_TASKS, 8 REDUCED_BINDINGS, 9 PHYSICAL_BINDINGS, 10 GLOBAL.
Sections begin at byte 768, are contiguous without padding, and end exactly at
`total_receipt_bytes`.  Unknown, missing, repeated or out-of-order sections,
bad lengths/digests, trailing bytes and nonzero reserved data fail validation.
The directory record counts for tags 1..10 are respectively
`13, 1, 1, 2, 2, 1, 614, 103, 1,015, 1`.

### 10.2 AUTHORITY

The 48-byte prefix is:

```text
authority_version:u32 = 1
verification_flags:u32 = 0x0000000f
parent_commit_sha1[20]
parent_manifest_entry_count:u32 = 184
identity_record_count:u32 = 13
freeze56_identity_record_tag:u32 = 12
reserved:u64 = 0
```

The flag bits assert parent commit availability with replacement objects
disabled, all 184 parent-manifest entries verified against parent blobs, all old
artifacts matched, and exact parent CENSUS prefix.  Thirteen 48-byte records
follow: tag u32, hash kind u32=1, byte length u64 and digest[32].  Their tags/order
are exactly: 1 parent M0/M1 source manifest, 2 freeze-55 descriptor, 3 received
guide, 4 v0.3 contract, 5 opening envelope, 6 grade-5 stop, 7 M0/M1 summary,
8 historical Gate-0, 9 M2 source manifest, 10 this M2 contract, 11 Cargo.lock,
12 freeze-56 descriptor and 13 parent CENSUS blob.  Exact freeze-56 descriptor
bytes follow record 13; their length and
digest equal record 12 and header offset 96.  The M2 manifest digest equals the
header build identity.

### 10.3 TOOLCHAIN and DEVICE

TOOLCHAIN begins with this 80-byte prefix:

```text
toolchain_version:u32=1; verification_flags:u32=0x00001fff; build_profile:u32=1
binding_package_count:u32=5; tool_count:u32=5; source_count:u32=2
compile_invocation_count:u32=2; link_invocation_count:u32=1
repro_build_count:u32=2; text_field_count:u32=19
metallib_bytes:u64; committed_metallib_sha256[32]
```

Its nineteen `TextV1` values are rustc release, rustc host, Cargo release, Rust
build target, empty RUSTFLAGS, empty CARGO_ENCODED_RUSTFLAGS, empty
RUSTC_WRAPPER, empty RUSTC_WORKSPACE_WRAPPER, Xcode version/build, Metal
component id/build, compiler version, SDK version/build, deployment target,
xctrace version and the two kernel names.  Five registry-package records in
bytewise name order contain name TextV1, version TextV1, decoded lockfile
checksum[32], default-feature u32, activated-feature-count u32, then exactly that
many bytewise-sorted unique feature TextV1 values.  They are
`dispatch2`, `objc2`, `objc2-core-graphics`, `objc2-foundation`, `objc2-metal`.

Five 48-byte tool records contain tool id u32, zero u32, executable bytes u64
and digest[32].  Tool ids are 1 metal, 2 metallib, 3 metal-ar, 4 xctrace and
5 xcodebuild, and records are serialized in ascending tool-id order.  Source
records in bytewise path order contain kind u32, zero u32,
bytes u64, digest[32], path TextV1; source kind 1 is translation unit and 2 is
include.  Compile records followed by the one link record contain kind u32,
source index or `u32::MAX`, argument count u32, zero u32 and argument TextV1
values; invocation kind 1 is compile and 2 is link.  Source indices are zero and
one in recorded path order; the link source index is `u32::MAX`.  Arguments are
exactly the lists in section 2 and use `<SOURCE_DIR>`, `<AIR_DIR>` and `<OUTPUT>`;
absolute paths are forbidden.  The section ends with fresh build
1, fresh build 2 and committed metallib digests[32], all equal to its prefix.
The thirteen required verification bits are clean environment, exact Rust
release, exact target/endianness/profile, exact lockfile package/checksum closure,
exact Xcode, exact component, exact tool versions/digests, exact normalized argv,
exact source identities, both fresh builds equal, committed library equal, both
kernel functions present and no-float/no-warning source gates.  Higher bits are
zero.  Both source records are translation units; v1 has no include record.

DEVICE has this 56-byte prefix:

```text
device_version:u32=1; flags:u32; pipeline_count:u32=2; text_count:u32=3
max_buffer_length:u64; recommended_working_set:u64
max_threads_x:u32; max_threads_y:u32; max_threads_z:u32
max_threadgroup_memory:u32; gate0_native_status:u32=4; reserved:u32=0
```

Flags equal `0x00000003`, requiring unified memory and a passed Rust Gate-0 device/queue/empty-command
test.  Texts are macOS version, macOS build and sanitized device name.  Two
24-byte pipeline records, arithmetic then projector, contain kernel id u32
(1 arithmetic, 2 projector),
execution width u32, maximum threads u32, zero u32 and static group memory u64.

### 10.4 TABLES_AND_ABI and ARITHMETIC

TABLES_AND_ABI is 184 bytes: section version u32=1, table count u32=2, ABI
version u32=1, poison u32, ABI sizes 32/64/80/64 as four u32, field scale u32=420,
response exponent u32=3, cell cap u32=11,730, slot cap u32=79,800, timeout
u64=120,000, projector arena u64=5,109,296 and arithmetic arena u64=2,359,424.
Two 56-byte records then contain table tag u32, format version u32, rows u32,
columns u32, byte length u64 and digest[32]: semantic table `(1,2,0,0)` and
choose table `(2,1,22,22)`.

ARITHMETIC is fixed 624 bytes.  Its 48-byte prefix is:

```text
0 version:u32=1; 4 run-record bytes:u32=288; 8 run count:u32=2
12 edge count:u32=32; 16 edge-prefix count:u32=4,288
20 tail count:u32=12,096
24 SplitMix initial state:u64=0x4d325f5532353656
32 SplitMix increment:u64=0x9e3779b97f4a7c15
40 oracle profile:u32=1; 44 reserved:u32=0
```

Two 288-byte records, official then negative, contain:

```text
0 run kind:u32; 4 profile version:u32=1; 8 case count:u32
12 accepted count:u32; 16 input bytes:u32=80; 20 output bytes:u32=64
24 guard count:u32=2; 28 poison:u32
32 input payload bytes:u64; 40 output payload bytes:u64
48 allocated input bytes:u64; 56 allocated output bytes:u64
64 completion class:u32=1; 68 native status:u32=4
72 success count:u32; 76 checked-undefined count:u32; 80 hard count:u32
84 validation flags:u32
88 input pre-digest[32]; 120 input post-digest[32]
152 CPU output digest[32]; 184 GPU output digest[32]
216 guard pre-digest[32]; 248 guard post-digest[32]
280 eight reserved zero bytes
```

Run kind 1 is the official corpus and 2 is negative controls; no other value is
valid.

Official case/accepted counts are 16,384/16,384; negative are 13/0 and hard
count 13.  Validation flags equal `0x0000003f`: completed/no error, immutable
input, every output overwritten, CPU/GPU equality, intact guards and exact
canonical status census.  Pre/post inputs, CPU/GPU outputs and canonical poison
guards match.  Higher flag bits are zero.

### 10.5 CARRIER and CONTEXT_TASKS

CARRIER is fixed 160 bytes:

```text
version:u32=1; profile:u32=1; tasks:u32=614
Reduced tasks:u32=64; GradeMatching:u32=46; SameContext:u32=504
direct parity:u32=73; direct stops:u32=541
reduced bindings:u32=103; physical bindings:u32=1,015
task-key bytes:u32=64; maximum cell high-water:u32=11,730
task-key stream bytes:u64=39,296; accepted payload bytes:u64
task-key stream digest[32]; task-input hash-chain digest[32]
choose-input hash-chain digest[32]
```

Input hash-chain records are task ordinal u32, zero u32, pre-digest[32] and
post-digest[32].  Each chain has 614 records and each pair matches canonical
task/table bytes.

CONTEXT_TASKS begins with version u32=1, record bytes u32=384, count u32=614 and
zero u32, followed by canonical-order 384-byte records:

```text
0 task key[64]
64 task status:u32=1; 68 direct status:u32 (1 PARITY, 2 DECLARED_STOP)
72 direct world count:u64; 80 direct cap:u64=100,000
88 accepted cells:u32; 92 cell cap:u32=11,730
96 in-range slot bytes:u64; 104 canonical payload bytes:u64
112 total scaled mass as eight least-significant-first u32 limbs
144 completion class:u32=1; 148 native status:u32=4
152 validation flags:u32=0x000003ff; 156 reserved:u32=0
160 CPU slot digest[32]; 192 GPU slot digest[32]
224 CPU payload digest[32]; 256 GPU payload digest[32]
288 CPU response-aggregate digest[32]; 320 GPU aggregate digest[32]
352 tail-and-guard digest[32]
```

The ten validation bits mean completed/no error, immutable task, immutable
choose, full raw parity, compact payload parity, tail poison, both guards,
aggregate parity, mass conservation and exact direct parity/stop.  Aggregate
stream records are response[3] u32, zero u32, support u64 and mass u64, only for
nonempty responses in canonical order.

### 10.6 Binding sections

REDUCED_BINDINGS begins version u32=1, record bytes u32=160, count u32=103,
zero u32, then 160-byte records:

```text
0 version:u32=1; 4 binding ordinal:u32; 8 task ordinal:u32
12 arm:u32 (1 or 2); 16 arm ordinal:u32; 20 endpoint:u32=0
24 canonical W42RTK01 root key[37]
61 selected action:u8; 62 derived context:u8; 63 grade:u8
64 matching count:u8; 65 three zero bytes; 68 reduced pool mask:u32
72 payload bytes:u64; 80 payload digest[32]
112 reduced semantic identity[32]; 144 sixteen zero bytes
```

PHYSICAL_BINDINGS begins version u32=1, record bytes u32=160, count u32=1,015,
zero u32, then 160-byte records:

```text
0 version:u32=1; 4 binding ordinal:u32; 8 task ordinal:u32
12 arm:u32 (2 or 3); 16 arm ordinal:u32
20 endpoint:u32 (grade arm 0; pair arm 0 or 1)
24 canonical W42RTK01 root key[37]
61 selected action:u8; 62 derived context:u8; 63 grade:u8=7
64 four zero bytes; 68 context pool mask:u32
72 payload bytes:u64; 80 payload digest[32]
112 physical semantic identity[32]; 144 sixteen zero bytes
```

### 10.7 GLOBAL and success conjunction

GLOBAL begins with this 64-byte prefix:

```text
version:u32=1; required validation flags:u32=0x00003fff
maximum cell high-water:u32=11,730; poison:u32=0xA5A55A5A
opening tail failures:u32=0; opening guard failures:u32=0
arithmetic guard failures:u32=0; input mutation failures:u32=0
projector capacity:u64=5,109,296; arithmetic capacity:u64=2,359,424
projector allocated high-water:u64=5,109,296
arithmetic allocated high-water:u64=2,359,424
```

Flags 0..13 respectively require complete context census, arithmetic corpus,
arithmetic negative controls, opening negative controls, task-key identity,
binding census, global raw parity, global payload parity, global aggregate
parity, mass conservation, all poison checks, all guard checks, all
input-immutability checks and no partial result.
All higher bits are zero.  Cross-run receipt equality, Lean and final manifest
verification are outer integrated-gate facts and are not falsely claimed by one
runner receipt.

It then carries ten digests[32] in order: global CPU/GPU raw slot streams,
global CPU/GPU payload streams, global CPU/GPU response aggregates, global
tail/guard chain, reduced identity stream, physical identity stream and success
conjunction.  Global CPU/GPU raw, payload and aggregate records are task ordinal
u32, zero u32, payload length u64 and payload bytes.  Identity records are
binding ordinal u32, zero u32 and identity[32].  The protected-chain payload
contains exactly 629 records of:

```text
domain:u32; ordinal:u32; first_protected_record:u32; protected_count:u32
purpose-9 digest[32]
```

Their order is arithmetic official `(domain=1, ordinal=0, first=16,384,
count=2)`, arithmetic negative `(2,0,13,2)`, the thirteen opening negatives
`(3,control_ordinal,0,12)`, then the 614 official contexts
`(4,task_ordinal,candidate_slot_count,79,802-candidate_slot_count)`.  Thus an
opening-negative digest covers all ten expected HARD records plus both guards,
while an official context digest covers every tail record and both guards.
Each opening-negative purpose-9 digest is compared with an independently
rendered expected control stream before its chain record is admitted.

The conjunction digest is

```text
SHA256("W42M2CON" || u32(1) || M2BuildIdentityV1 || freeze56_sha256 ||
       four accepted counts:u32 || accepted_payload_bytes:u64 ||
       mismatch_count:u32 || partial_result_retained:u32 ||
       section digests for sections 1..9 in directory order)
```

It ends with `u32(50)` and the exact 50 claim bytes, without NUL or newline.
Accepted payload bytes sum each `50 + 26*cell_count` context payload once across
614 tasks; framing and binding references do not count.  No raw or compact
payload bytes are persisted.  Volatile time, PID, path, registry id, temperature
and counters do not exist in the schema.

### 10.8 Failure receipt

Failure is a distinct fixed 256-byte type with no section or claim:

```text
0 magic[8]="W42M2F01"; 8 version:u16=1; 10 bytes:u16=256
12 outcome:u32=2; 16 total bytes:u64=256
24 phase:u32; 28 code:u32; 32 task ordinal:u32; 36 subordinal:u32
40 child exit:i32; 44 native status:u32
48/52/56/60 four accepted counts:u32 = 0
64 accepted payload bytes:u64=0; 72 observed mismatch:u32
76 partial retained:u32=0; 80 build identity[32]; 112 freeze56 digest[32]
144 child failure-frame digest[32]; 176 parent commit SHA-1[20]
196 sixty reserved zero bytes
```

Unavailable ordinals/status use `u32::MAX`; unavailable child exit uses
`i32::MIN`.  The child emits a failure receipt with an all-zero child-frame
digest and encloses it in exactly one flushed FAILURE wire frame.  The parent
requires that zero, hashes the exact complete wire-frame bytes including its
four-byte length prefix, then renders its final failure receipt with that
ordinary SHA-256.  The parent does not send or re-embed the final receipt, so no
self-hash exists.  If no complete child failure frame was received, the final
field remains zero.  A nonzero child-supplied field or multiple failure frames
is protocol failure.  Failure build identity is zero exactly when no valid M2
manifest bytes were available to hash; otherwise it is their digest.
Phases 1..16 are HISTORICAL, SOURCE_MANIFEST, RUST_BUILD, METAL_TOOLCHAIN,
SHADER_REPRODUCIBILITY, GATE0, TABLES, ARITHMETIC_NEGATIVE,
ARITHMETIC_CORPUS, CARRIER_PREFLIGHT, OPENING_NEGATIVE, PROJECTOR_TASK,
BINDINGS, RECEIPT_RENDER, CHILD_PROTOCOL and RECEIPT_REGENERATION.  Codes 1..24
are INVALID_AUTHORITY, IDENTITY_MISMATCH, TOOLCHAIN_MISMATCH,
METALLIB_MISMATCH, NO_DEVICE, ALLOCATION_FAILURE, PIPELINE_FAILURE,
ENCODER_FAILURE, COMMAND_STATE_FAILURE, COMMAND_ERROR, TIMEOUT,
MALFORMED_OUTPUT, POISON_FAILURE, GUARD_FAILURE, INPUT_MUTATION,
ARITHMETIC_MISMATCH, PROJECTOR_MISMATCH, MASS_MISMATCH, CARRIER_MISMATCH,
BINDING_MISMATCH, CHILD_PROTOCOL_FAILURE, RECEIPT_NONDETERMINISTIC,
RECEIPT_COMPARAND_MISMATCH and INTERNAL_FAILURE.  Unknown values are invalid.
Exit 124 maps to the active GATE0, ARITHMETIC_NEGATIVE, ARITHMETIC_CORPUS,
OPENING_NEGATIVE or PROJECTOR_TASK phase and TIMEOUT.  Failure bytes are
diagnostics and never satisfy the M2 success comparand.

---

## 11. Manifest and integrated gate

`walt/math/gpu_native_trick1_m0_m2_sources_v1.sha256` is a sorted two-space
`sha256  relative/path` manifest whose paths are UTF-8, relative to the repository
root, use `/`, contain neither `..` nor absolute components, and are unique.  Its
checker therefore runs from the repository root; the manifest file itself remains
under `walt/math/`.  The historical M0/M1 manifest retains its original
`walt/`-relative path grammar and is verified separately from `walt/`.
It includes Cargo manifests/lock/toolchain, all Walt and referenced Rob/Lean
proof-path sources, both contracts and rebrief, complete parent historical
manifest and artifacts, appended rulings, scripts, MSL, final metallib and Lean
M2 theorem source.  It excludes itself, `target`, AIR, caches, diagnostics,
temporary files, M2 success/failure receipts and receipt checksums.  The exact
manifest bytes hash to `M2BuildIdentityV1`; no caller supplies that identity.

The portable `ci/check.sh` verifies historical blobs and receipts, current source
manifest, formatting, warning-denied clippy, no-float gates, release workspace
tests and both Lean targets.  It never skips a Metal result into green and never
requires a GPU.  `ci/check_m2_metal.sh` first runs `ci/check.sh`, validates the
host/tool descriptor, rebuilds the metallib twice, executes negative controls
and discarded maximum smoke, then runs the complete 614-task carrier twice from
fresh process state.  The two success receipts and the committed comparand must
be byte-identical, their external SHA-256 file must match, and the source
manifest is reverified last.  Every Cargo command uses `--locked`.

Lean target `Texas42.Trick1MetalFoundation` proves all seven listed obligations
in rebrief section 15 and is audited with `#print axioms`; only expected core
axioms are allowed.  The finite projector formulas, Rust/Lean correspondence and
Metal/Rust semantic correspondence remain named proof debt.  Metal parity is
evidence for the last, not a theorem.

The final conjunction order is historical verification; source verification;
portable gates; two-build metallib equality; canonical Rust Gate 0/U256 corpus;
negative/timeout/no-partial controls; discarded grade-7/m=6 smoke; fresh
Reduced, GradeMatching and SameContext official run; second fresh complete run;
receipt equality; Lean build/axiom audit; final source verification.  Any failed
conjunct emits no success sentence and accepts no partial result.

---

## 12. Freeze rule

This file may be edited only before its digest is appended in GT1-A17.  Once
frozen, an implementation discrepancy is resolved by failing the gate or by a
new append-only adjudication and new version; the implementation never silently
changes this measured object after seeing a Metal result.

---

## 13. Append-only amendment: freeze-56 v2 (2026-08-24)

Adjudicated as the FZ-A series in `CENSUS-RULINGS.md` (Jason's
2026-08-24 unification ruling); this section is the new append-only
adjudication §12 requires — nothing above this line changed.

The one-crate unification moved walt-core, walt-kernel and
walt-gpu-spec into the unified `walt` crate (modules `rules`, `kernel`,
`spec`). Consequences for this gate's objects:

- The v1 cumulative source manifest
  (`math/gpu_native_trick1_m0_m2_sources_v1.sha256`) is byte-immutable
  and preserved; its digest remains the M2BuildIdentityV1 that the
  committed receipt `receipts/gpu_native_trick1_m2_v1/` attests to.
  That receipt is henceforth explicitly **evidence for the old
  layout**; it is never presented as attesting the post-fold sources.
- A v2 manifest (`…_v2.sha256`) pins the post-fold closure; its digest
  is a new build identity, attested by no hardware run yet. Re-earning
  the M2 Metal parity receipt under v2 (§9's 614-task carrier, twice)
  is deferred to the kanban story [[m2-receipt-reearn]].
- `ci/verify_m2_sources.sh` carries the explicit 32-entry
  fold-translation table (FZ-A2) and verifies v2; since v2 it runs at
  freeze events rather than per commit (FZ-A5), because the unified
  crate contains actively developed solver code. `ci/check.sh` keeps
  the per-run immutable checks (§ history, guide identity, M0/M1
  receipt replay, Lean axiom audit).
