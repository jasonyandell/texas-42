# GPU-native trick-1 M2 rebrief

**Version:** 0.1  
**Date:** 2026-08-16  
**Status:** pre-freeze implementation rebrief; proposed adjudication, not yet authority  
**Target gate:** M2 Metal arithmetic and opening-projector parity only  
**Parent authority:** freeze 55 at GT1-A9  

This rebrief is required because the historical Gate-0 observation has changed,
because M2 needs buffer, work-unit, failure, compiler, binding, carrier and
persistence decisions that freeze 55 deliberately left open, and because the
portable source manifest cannot truthfully continue to describe a growing M2
tree.  It does not reopen or rewrite any freeze-55 object.

The governing discipline is still: exact arithmetic, CPU/reference semantics as
authority, no strategy fusion, no silent truncation, and no result promoted
beyond the gate that actually produced it.  M2 is a correctness instrument.  It
does not yet attempt a trick-1 action, construct an information net, or measure a
production speedup.

---

## 1. Historical objects that remain immutable

The following are historical inputs, not files to be regenerated under changed
sources:

```text
parent source commit
  3b4c6d60fef371e3050de151ccf9eaefbc2d2da7

freeze-55 M0/M1 manifest/build identity
  eccf0a3742e2cfc50cad158292db7ad8c6145da8aa7958b7aa2ed07a1566f2ad

freeze-55 descriptor digest
  9b181092045b003893cae7c09cc7b7c8b57f75c3c5c4cf7043b8d428df738efa

opening envelope
  1127d3868d7da07c26a7b8bc031ac8a63ba84a9068df786b67a413ea6af5f517

grade-5 declared stop
  7e8dfecf1cac314ae6e71b406eb268b29d4157206ce5e64d1c50d1aa94d43bdf

M0/M1 receipt summary
  51a162ea933801f05b852ec2a454c48a31c7d292ee8273ba683d0a7fec340b12

binding v0.3 contract
  6190e740a0579b6b5196e086e52c8022d4cddcd0f746ecbd9226f87bbc0e4790

historical Gate-0 NO-GO receipt
  b57f7077e5aa0aa1d8030a76a3399076810b71b1623ad83e001aee2b4aaeb215
```

The old manifest is verified against blobs at the parent source commit.  It is
never recut and is no longer checked against live paths after M2 changes those
paths.  The old receipts remain byte-for-byte comparands.  Current validators
may replay them as compatibility tests, but current source does not become their
historical producing source.

The historical verifier also requires the current `CENSUS-RULINGS.md` bytes to
begin with the exact parent-commit bytes.  Later rulings are append-only suffixes;
an edit anywhere in the parent prefix is a hard provenance failure.

M2 receives a new cumulative source manifest,
`math/gpu_native_trick1_m0_m2_sources_v1.sha256`.  It includes the unchanged old
manifest and artifacts among its inputs and excludes itself and generated M2
receipts.  `M2BuildIdentityV1` is SHA-256 over the exact new manifest bytes.
No manifest-covered file hardcodes the resulting M2 build identity, final receipt
digest, build-dependent semantic identity or final commit.  Receipt comparand and
checksum files remain outside the source manifest, preventing an identity cycle.

---

## 2. The host precondition is open; canonical Rust Gate 0 remains pending

The old Gate-0 receipt remains true about its observation.  It is superseded for
the present host by these newly observed facts:

```text
host                         Apple M5 Max, 40 GPU cores, 48 GB unified memory
macOS                        26.5.1, build 25F80
Xcode                        26.6, build 17F113
selected developer dir       /Applications/Xcode.app/Contents/Developer
MetalToolchain component     build 17F109
Metal toolchain id           com.apple.dt.toolchain.Metal.32023.883
Metal compiler               Apple metal version 32023.883 (metalfe-32023.883)
SDK                          26.5
xctrace                      16.0 (17F113)
```

The downloaded Metal component must be selected explicitly:

```text
xcrun --toolchain com.apple.dt.toolchain.Metal.32023.883 metal
xcrun --toolchain com.apple.dt.toolchain.Metal.32023.883 metallib
xcrun --toolchain com.apple.dt.toolchain.Metal.32023.883 metal-ar
```

The default `xcrun -sdk macosx metal` shim is not evidence that the component is
absent.

An identical Metal.framework diagnostic binary returned no default device in the
normal Codex sandbox and, when executed with elevated host access, created the
Apple M5 Max device, queue, pipeline and command buffer and completed the command
buffer with status 4.  The strict integer diagnostic passed twice over eight
carry, overflow and multiply-by-420 cases.  Observed limits were:

```text
unified memory                         true
max buffer length                      30,150,672,384 bytes
recommended working set                40,200,896,512 bytes
device max threads                     1024 x 1024 x 1024
device threadgroup memory              32,768 bytes
pipeline execution width               32
pipeline max threads                   1,024
pipeline static threadgroup memory     0
```

This opens the host/toolchain precondition but is not yet canonical M2 Gate-0
evidence; the checked-in Rust gate must reproduce it.  It proves
compiler/runtime connectivity, not M2 arithmetic or projector parity.  Official
GPU execution requires elevated host access; a sandbox no-device
result is a named environment failure and may not be converted into a skip or a
green result.  No application restart or macOS privacy-setting change is part of
the gate.

The temporary Swift source, AIR, metallib, module cache and executables under
`.gate0-diagnostic` are noncanonical orientation material and are not committed.

---

## 3. Exact M2 scope and claim fence

M2 contains exactly two official tasks:

```text
U256MetalParityV1
OpeningProjectorMetalParityV1
```

It includes checked integer arithmetic, one extracted choose table, a fixed
projector ABI, CPU/GPU cell-byte parity, domain-separated reduced-evidence and
physical-action binding records, deterministic receipts and fail-closed negative
controls.

It excludes:

- action resolution and any root action value;
- loss-budget pruning and contract settlement;
- K-OPEN4+ continuation;
- GPU prefix scans, reductions, sorting, atomics and indirect dispatch;
- task slabs, concurrent command buffers and performance crossover;
- wavefronts, Scheme/Fix search, information nets and controllers;
- a selected lead, an optimal set, a player, or any strategy-strength claim.

The only success sentence admitted by this gate is:

```text
M2 METAL PROJECTOR PARITY COMPLETE under freeze 56
```

Timing, captures, counters and thermal observations are noncanonical appendix
material and cannot enter that conjunction.

---

## 4. Rust binding and unsafe boundary

Use the current `objc2` family directly, with exact package versions and a
minimal explicit feature closure:

```toml
objc2 = "=0.6.4"
objc2-core-graphics = { version = "=0.3.2", default-features = false }
objc2-foundation = { version = "=0.3.2", default-features = false,
  features = ["NSError", "NSObject", "NSString"] }
dispatch2 = { version = "=0.3.1", default-features = false,
  features = ["alloc", "block2", "objc2"] }
objc2-metal = { version = "=0.3.2", default-features = false, features = [
  "std", "dispatch2", "MTLAllocation", "MTLBuffer", "MTLCommandBuffer",
  "MTLCommandEncoder", "MTLCommandQueue", "MTLComputeCommandEncoder",
  "MTLComputePipeline", "MTLDevice", "MTLLibrary", "MTLResource", "MTLTypes"
] }
```

`objc2-core-graphics` is a linkage-only direct dependency required by
`MTLCreateSystemDefaultDevice`; `walt-metal` imports it as `_` and enables none of
its generated API features.  This avoids a project-owned empty `extern` linkage
block and does not add a fourth unsafe operation class.

Cargo.lock registry checksums are part of the M2 source identity.  Do not use
deprecated `metal-rs`, `wgpu`, a portability layer, a handwritten Objective-C
surface, or `build.rs` shader compilation in this gate.

The `walt-metal` crate denies unsafe code at crate scope and allows it only in one
private reviewed module.  That module owns exactly three unsafe operation classes:

1. copy validated host words into a shared `MTLBuffer`;
2. copy words from a completed shared `MTLBuffer` into owned host storage;
3. bind a validated buffer and offset to a compute encoder.

Its public API is safe.  `unsafe_op_in_unsafe_fn` remains denied; lengths,
alignment, offsets, command completion and ownership are checked before each
operation.  No pointer escapes the module.

No Walt Rust type, value, arithmetic operation, buffer field, MSL type,
expression, compiler option or invoked Metal API in the proof path uses floating
point.  The pinned third-party binding contains generated declarations for APIs
outside the selected feature/use path, including declarations mentioning floating
types.  Those uncalled declarations are not Walt proof-path types or operations.
This interpretation is accepted explicitly; rejecting unused generated
declarations would require a separately adjudicated integer-only C shim and is not
silently substituted.

---

## 5. Compiler and library identity

Check in the final MSL sources and the final metallib.  Compile no shader from
`build.rs`.  The canonical source order is lexical filename order and the strict
compiler profile is:

```text
-std=metal3.2
-mmacosx-version-min=26.0
-fmetal-math-mode=safe
-fno-fast-math
-Wall
-Wextra
-Werror
-c
```

Use the named downloaded toolchain without `--sdk`; record the resolved SDK
identity separately.  Link the resulting AIR inputs in the same lexical source
order.  `metal-ar` availability and identity are recorded, but it is not inserted
into the build if the library has no archive stage.

AIR is transient diagnostic material.  It can embed an absolute source path and
is neither checked in nor required to match across directories.  The gate instead
builds the final checked-in sources in two fresh directories and requires the two
final metallibs and the checked-in metallib to be byte-identical.  Source bytes,
normalized invocations, Xcode build, toolchain component id/build, reported
compiler version, SDK version, tool executable digests and final metallib digest
are bound in the source manifest and receipt.  Absolute component paths are
observations, not semantic identities.

---

## 6. One work unit and why scheduling is intentionally simple

One projector work unit is one complete validated `OpeningContext`.  It is
context-bound, not action-bound.  Its raw GPU cells are nonpersistable.

One command buffer contains exactly one projector task.  Command buffers are
committed and completed sequentially in canonical carrier order.  One GPU thread
owns one ordered response-triple ordinal and writes exactly ten fixed slots.  No
two threads write the same word.  There are no atomics, GPU prefix scans,
reductions, indirect dispatches, slabs or adaptive batch sizes.

The frozen kernel entry points and buffer indices are:

```text
u256_parity_v1       buffer(0) ArithmeticInputV1 array
                     buffer(1) ArithmeticOutputV1 array

opening_project_v1   buffer(0) one OpeningTaskV1
                     buffer(1) OpeningChooseTableV1
                     buffer(2) OpeningSlotV1 array
```

Arithmetic dispatches one thread per input case.  Projector dispatches exactly
`response_triple_count` threads, never a rounded-up logical grid; a defensive
out-of-range thread returns without writing because it owns no safe slot region.
Both use `dispatchThreads` with threadgroup width 32.  The unsafe host boundary
checks the exact grid count, threadgroup limit, buffer lengths, bindings and
offset zero before encoding.

The host scans fixed output offsets in ascending response ordinal and then
ascending local-slot ordinal.  Consequently hardware execution and completion
order cannot change the canonical payload.  This is a correctness choice for M2,
not a production scheduling recommendation.

The complete physical root, selected physical lead and all closed root profiles
exist only in the accepted typed binding record.  A binding record may incorporate a
validated context payload only when the host reconstructs the declaration and
selected led context from that root and action and proves that they equal the
task declaration and led context.  For grade 7, the complete action-derived
`OpeningContext` must equal the task.  For a reduced grade, the task pool must be
a subset of the root hidden pool and must be exactly the pool produced by the
named carrier rule at the bound coordinate; reduced capacities are test
instrument metadata, not a false physical grade-7 root.  A raw context payload
alone cannot be persisted or cited.

---

## 7. Opening task ABI

`OpeningTaskV1` is eight little-endian `u32` words, exactly 32 bytes:

```text
word 0   abi_version = 1
word 1   task_ordinal
word 2   grade
word 3   pool_mask
word 4   matching_mask
word 5   pool_count
word 6   response_triple_count
word 7   candidate_slot_count
```

Before allocation or dispatch, the host validates all of the following:

- ABI version and task ordinal;
- `grade in 1..=7`;
- masks have no bits above physical domino index 27;
- `matching_mask` is a subset of `pool_mask`;
- `popcount(pool_mask) = pool_count = 3 * grade`;
- `popcount(matching_mask) <= 6`;
- `response_triple_count = pool_count * (pool_count-1) * (pool_count-2)`;
- `candidate_slot_count = response_triple_count * 10 <= 79,800`;
- the declaration and led context reconstructed from the bound root/action agree
  with the task, the named carrier rule reconstructs the task pool, and
  `pool_mask & effective_incidence(declaration, led)` reconstructs exactly the
  supplied matching mask rather than trusting it as a second rules authority.

The shader repeats range and count checks defensively.  A production host
preflight failure dispatches nothing.  The private negative-control harness may
bypass only semantic descriptor preflight after independently proving its exact
one-thread grid and twelve-slot buffer memory-safe; it is unavailable through the
public production API.  A shader-side impossible descriptor produces a
hard-error status and voids a production run.

---

## 8. Opening output ABI, poison and compaction

`OpeningSlotV1` is sixteen little-endian `u32` words, exactly 64 bytes:

```text
word 0       status
word 1       task_ordinal
word 2       slot_ordinal
words 3..5   response domino indices for seats 1, 2, 3
words 6..8   remaining matching counts for seats 1, 2, 3
word 9       support count
words 10..11 per-world likelihood coefficient, low then high
words 12..13 scaled cell mass, low then high
words 14..15 reserved, always zero when written
```

The Rust and MSL buffer definitions use scalar `u32`/`uint` words or
`uint words[N]`, never `uint3`, packed vectors, enums, booleans or native
pointers.  Both sides assert sizes 32, 64, 80 and 64 bytes for OpeningTask,
OpeningSlot, ArithmeticInput and ArithmeticOutput respectively.  `ulong` is
allowed only for checked shader arithmetic and is split explicitly at the ABI.

Status classes are:

```text
0             SKIP
1             VALID
0x80000001    HARD_BAD_ABI
0x80000002    HARD_BAD_MASK
0x80000003    HARD_BAD_COUNT
0x80000004    HARD_BAD_RESPONSE_ORDINAL (validated in-range decode invariant)
0x80000005    HARD_TOO_MANY_STRATA
0x80000006    HARD_CHOOSE_INDEX
0x80000007    HARD_SUPPORT_OVERFLOW
0x80000008    HARD_COEFFICIENT_OVERFLOW
0x80000009    HARD_MASS_OVERFLOW
```

Every dispatched thread first overwrites every word of all ten owned slots with
canonical SKIP, computes and validates its response, then fills consecutive
VALID slots.  If any thread-local hard condition occurs, it rewrites all ten
owned slots to the same canonical HARD reason.  SKIP and HARD slots zero every
semantic and reserved word while still writing the exact task and slot ordinals.
VALID writes all fields and both zero reserved words.  For response ordinal `q`
and local slot `s`, `slot_ordinal = 10*q+s`.
Unknown statuses, nonzero reserved words, malformed status/field combinations,
wrong ordinals or duplicate/out-of-order valid keys fail the whole run.

The output arena always has 79,800 task slots plus two guard slots.  Before each
dispatch every word is initialized to poison `0xA5A55A5A`.  Every complete
in-range record must equal the independently rendered CPU-expected VALID/SKIP
record; validation does not incorrectly require every individual data word to be
different from the poison value.  Every tail slot beyond `candidate_slot_count`
and both guard slots must retain poison.  The host scans actual slots, discards
only canonical SKIP records, and encodes accepted cells directly from the
validated GPU-produced fields.  It does not regenerate accepted payload bytes
from CPU cells after an equality check.

Parity has two distinct byte domains.  First compare the complete
`candidate_slot_count * 64` in-range GPU slot stream with an independently
rendered CPU expected slot stream, byte-for-byte.  Then stable-compact VALID GPU
records and encode their fields into the existing 26-byte-per-cell canonical
projector payload, comparing that complete payload byte-for-byte with the scalar
payload.  Tail and guard poison are a third, separate check.

Response triples are the ordered distinct triples of the pool's physical domino
indices, lexicographic by seats 1, 2, 3.  Decode response ordinal `q` with
`n = pool_count`, `A = (n-1)(n-2)`, `i = q/A`, remainder `r = q mod A`,
`j_rank = r/(n-2)`, and `k_rank = r mod (n-2)`.  Map `j_rank` into the original
pool positions while skipping `i`; map `k_rank` while skipping the lower and
then higher of `i,j`; finally select the corresponding set bits by a fixed
physical-bit loop over 0..27.  No count-trailing-zeros operation on zero and no
shift by the word width is admitted.

Within one response, feasible remaining
matching-count vectors are enumerated lexicographically by seats 1, 2, 3 and
written to the first consecutive local slots; the rest are SKIP.  More than ten
feasible vectors is a hard error.  The compacted VALID stream must match GT1-A4
cell order exactly.

The fixed maximum projector arena is therefore:

```text
OpeningTaskV1                         32 bytes
OpeningChooseTableV1               1,936 bytes
(79,800 + 2) OpeningSlotV1      5,107,328 bytes
                                  ---------
total                            5,109,296 bytes
```

All three buffers use shared storage.  Host reads occur only after successful
command completion.

`OpeningNegativeControlsV1` separately exercises shader-side descriptor defense.
Its valid base is ABI 1, grade 1, `pool_mask=0x00000007`,
`matching_mask=0`, pool count 3, response count 6 and slot count 60.  It runs
exactly thirteen single-fault tasks in this order:

```text
0   ABI 0                                      HARD_BAD_ABI
1   ABI 2                                      HARD_BAD_ABI
2   base pool mask OR 0x10000000                HARD_BAD_MASK
3   matching mask 0x10000000                   HARD_BAD_MASK
4   matching mask 0x00000008                   HARD_BAD_MASK
5   grade 0                                    HARD_BAD_COUNT
6   grade 8                                    HARD_BAD_COUNT
7   declared pool count 4                      HARD_BAD_COUNT
8   pool mask 0x0000000f, declared count 3      HARD_BAD_COUNT
9   grade 2 with the base three-tile pool       HARD_BAD_COUNT
10  grade 3, pool mask 0x000001ff, pool count 9,
    matching mask 0x0000007f, response count
    504 and slot count 5,040                   HARD_BAD_COUNT
11  declared response count 7                  HARD_BAD_COUNT
12  declared slot count 61                     HARD_BAD_COUNT
```

Descriptor-validation precedence is ABI, high mask bits, matching subset,
grade range, pool popcount/count/three-times-grade relation, matching-count cap,
response-count formula, then slot-count formula.  Each negative task dispatches
exactly one logical thread into ten slots plus two guards; all ten slots must
carry the same expected HARD reason and both guards remain poison.  These are
expected-failure controls, not accepted carrier tasks.

---

## 9. Extracted choose table

The projector receives `OpeningChooseTableV1`, a 22 by 22 row-major table of
little-endian `u32` values `choose(n,k)` for `0 <= n,k <= 21`, exactly 1,936
bytes.  Values with `k > n` are zero.

This is a new identity and version.  It does not mutate, replace, or masquerade
as `SemanticTablesCanonicalV2`.  Production bytes are extracted from the choose
section of the still-unchanged v2 table.  Every entry is compared with an
independent arbitrary-precision binomial oracle before any GPU allocation.  Its
version, byte length and digest are carried by the task-run descriptor and M2
receipt.

---

## 10. Projector arithmetic and failure semantics

For grade `g`, let remaining seat capacity be `r = g-1`.  For each response
triple, the shader derives follower/void roles from the matching mask.  A void
seat has `e_s = 0`; follower counts satisfy `0 <= e_s <= r` and
`sum e_s = m-|F|`.  The shader enumerates those vectors and computes the frozen
reduced-grade generalization of the GT1-A4 quantities:

```text
A_g(e,x) = |M'|! / product_{s in F}(e_s!)
             * |N'|! / product_{all s}((r-e_s)!)

C_g(e,x) = product_{s in F}(420/(e_s+1))
             * product_{s in Z}(420/g)

W_g(e,x) = A_g(e,x) * C_g(e,x)
```

Thus the familiar void factor 60 is specific to physical grade 7.  The shader
implements `A_g` through the ordered 22-by-22 choose table, matching the current
scalar implementation rather than through an unchecked factorial division.

The ten-slot bound is structural: for `f=0` or `f=1` there is at most one vector;
for `f=2` there are at most five; for `f=3`, `m-f <= 3` and there are at most
`choose(5,2)=10` weak compositions before capacity filtering.  The maximum ten
is attained for `g>=4,m=6,f=3`.

Every multiply, add, subtraction, table index, count conversion and 64-bit split
is checked against its frozen bound.  A zero support vector is SKIP, never a
VALID zero-support cell.  Arithmetic overflow is a hard error.  The host checks
per-cell bytes, canonical key order, cell count, response aggregates and complete
mass conservation against the scalar projector.

Independent exhaustive enumeration of the finite `(g,m,roles,e)` parameter
space fixes these runtime upper checks:

```text
largest choose entry       choose(21,10) = 352,716
largest cell support                       17,153,136
largest coefficient                         74,088,000 = 420^3
largest cell mass                 1,270,841,539,968,000
largest whole mass               29,566,517,460,480,000
```

Support therefore fits `u32`; coefficient, cell mass and whole mass fit their
declared `u64` fields.  These are asserted derived bounds, not permission to
wrap if a value exceeds them.

The maximum nonempty count remains 11,730.  The rectangular 79,800 count is an
address-space cap, not an accepted-cell cap.  Either cap being exceeded voids the
run; neither may truncate it.

---

## 11. U256 arithmetic ABI

`ArithmeticInputV1` is twenty little-endian `u32` words, exactly 80 bytes:

```text
word 0       abi_version = 1
word 1       case_id
word 2       operation
word 3       operand
words 4..11  lhs, eight little-endian limbs
words 12..19 rhs, eight little-endian limbs
```

The closed operation registry is:

```text
1  CHECKED_ADD       operand must be zero
2  CHECKED_SUB       operand must be zero
3  CHECKED_MUL_SMALL operand is the u32 factor; rhs must be zero
4  CHECKED_MUL_POW_420 operand is exponent 0..=21; rhs must be zero
5  COMPARE           operand must be zero
```

Unknown versions or operations, forbidden nonzero fields and exponent above 21
are malformed requests, not checked arithmetic overflow.

`ArithmeticOutputV1` is sixteen little-endian `u32` words, exactly 64 bytes:

```text
word 0       status
word 1       case_id
word 2       operation copied from input
word 3       defined
word 4       ordering
words 5..12  result, eight little-endian limbs
words 13..15 reserved, always zero
```

Ordering is `0 = NOT_APPLICABLE`, `1 = LESS`, `2 = EQUAL`, `3 = GREATER`.
Canonical outcomes are:

- exact arithmetic result: status SUCCESS, defined 1, ordering 0, exact limbs;
- exact comparison: status SUCCESS, defined 1, ordering 1..3, zero limbs;
- checked overflow or underflow: status CHECKED_UNDEFINED, defined 0, ordering 0,
  zero limbs;
- malformed request or internal failure: high-bit HARD ERROR, defined 0,
  ordering 0, zero limbs.

Every other combination fails validation.

Arithmetic status values are frozen as:

```text
1             SUCCESS
2             CHECKED_UNDEFINED
0x80000001    HARD_BAD_ABI
0x80000002    HARD_BAD_OPERATION
0x80000003    HARD_BAD_OPERAND
0x80000004    HARD_BAD_UNUSED_RHS
0x80000005    HARD_BAD_EXPONENT
0x80000006    HARD_INTERNAL
```

The official `U256MetalCorpusV1` contains exactly 16,384 validly encoded inputs.
Its checked-in generator is part of the source manifest and has two ordered
portions.

The edge list has exactly 32 `U256` values in this order:

```text
0, 1, 2,
2^32-1, 2^32,
2^64-1, 2^64,
2^96-1, 2^96,
2^128-1, 2^128,
2^160-1, 2^160,
2^192-1, 2^192,
2^224-1, 2^224,
2^256-1, 2^256-2,
0xaaaaaaaa repeated in all eight limbs,
0x55555555 repeated in all eight limbs,
limbs [0,1,2,3,4,5,6,7],
limbs [7,6,5,4,3,2,1,0],
N0 = 399,072,960,
420,
420^21,
N0*420^21,
42*N0*420^21,
2^255,
2^254,
limbs [0xffffffff,0,0xffffffff,0,0xffffffff,0,0xffffffff,0],
limbs [0,0xffffffff,0,0xffffffff,0,0xffffffff,0,0xffffffff]
```

All limb lists are least-significant limb first.  The edge prefix is then:

1. operations ADD, SUB, COMPARE in that order, each over the complete
   lexicographic `32 x 32` `(lhs,rhs)` product with lhs ordinal outer and rhs
   ordinal inner: 3,072 cases;
2. MUL_SMALL over each edge lhs, then factors
   `[0,1,2,3,6,7,10,42,60,84,105,140,210,419,420,u32::MAX]`: 512 cases;
3. MUL_POW_420 over each edge lhs, then every exponent `0..=21`: 704 cases.

The prefix therefore contains 4,288 cases.  The remaining 12,096 cases use
SplitMix64 with initial state `0x4d325f5532353656` and wrapping `u64`
arithmetic.  For every output, first set
`state = state + 0x9e3779b97f4a7c15`, then `z = state`, then apply exactly

```text
z = (z xor (z >> 30)) * 0xbf58476d1ce4e5b9
z = (z xor (z >> 27)) * 0x94d049bb133111eb
z = z xor (z >> 31)
```

For every tail case, consume four outputs for lhs and four for a provisional
rhs, taking each output's low `u32` limb before its high limb, then consume one
selector output.  Cycle operations
`[ADD,SUB,MUL_SMALL,MUL_POW_420,COMPARE]` by tail ordinal.  ADD, SUB and COMPARE
use the generated rhs and operand zero.  MUL_SMALL uses the selector low word as
factor and canonical zero rhs.  MUL_POW_420 uses selector low word modulo 22 as
exponent and canonical zero rhs.  Discarded provisional rhs words are still
consumed.  Case ids are the complete input ordinals `0..16,383`.

Malformed descriptors form a separate `ArithmeticNegativeControlsV1` command
and never occur in the accepted 16,384-case corpus.  It contains exactly thirteen
single-fault cases in the order below.  Unless named otherwise, ABI is 1,
operation is ADD, operand is zero, and both operands are zero; case id is the
negative-control ordinal.

```text
0   ABI 0                                      HARD_BAD_ABI
1   ABI 2                                      HARD_BAD_ABI
2   operation 0                                HARD_BAD_OPERATION
3   operation 6                                HARD_BAD_OPERATION
4   ADD operand 1                              HARD_BAD_OPERAND
5   SUB operand 1                              HARD_BAD_OPERAND
6   COMPARE operand 1                          HARD_BAD_OPERAND
7   MUL_SMALL with rhs limb 0 equal to 1       HARD_BAD_UNUSED_RHS
8   MUL_SMALL with rhs limb 7 equal to 1       HARD_BAD_UNUSED_RHS
9   MUL_POW_420 with rhs limb 0 equal to 1     HARD_BAD_UNUSED_RHS
10  MUL_POW_420 with rhs limb 7 equal to 1     HARD_BAD_UNUSED_RHS
11  MUL_POW_420 exponent 22                    HARD_BAD_EXPONENT
12  MUL_POW_420 exponent u32::MAX              HARD_BAD_EXPONENT
```

The validation precedence is ABI, operation registry, operation-specific unused
fields, then exponent range; every case above has only its named fault.  Every
slot must return the exact canonical hard-error form and two output guards must
retain poison.  A missing hard error fails the negative gate; the expected hard
statuses are not accepted arithmetic results and do not trigger a false success
or an official-run partial result.

The input byte stream, independent BigUint oracle output stream and GPU output
stream each receive a digest.  BigUint code shares no limb arithmetic with the
shader or `U256Mass`.  The two output byte streams must be identical.

The arithmetic arena holds at most 16,384 inputs and 16,384 outputs plus two
output guards:

```text
16,384 * 80-byte inputs              1,310,720 bytes
(16,384 + 2) * 64-byte outputs       1,048,704 bytes
                                      ---------
total                                2,359,424 bytes
```

Every output slot is overwritten and both output guards retain poison.

---

## 12. Canonical M2 carrier

`M2OpeningParityCarrierV1` has three ordered arms.  Generating rules govern;
counts are derived assertions.

### 12.1 `ReducedArmV1`

Promote `ReducedArmRootsV1` from the old private test helper into a canonical
generator.  Both roots use the exact ordered hand
`[6-0,6-1,6-2,6-3,6-4,6-5,5-5]`.  Root 0 is NT, S0, P30; root 1 is P6, S2,
Mark.  Use every coordinate of their complete `ReducedOpeningCarrierV1`, in root
then existing generator order.  Bind each coordinate to the least physical legal
lead whose derived led context equals the coordinate led context.

This yields 64 context tasks:

- grades 2, 3 and 4 retain scalar/direct/GPU parity;
- grade 5 retains scalar/GPU parity and the unchanged complete-direct declared
  stop before enumeration.

### 12.2 `GradeMatchingArmV1`

Use no-trump and natural-six.  For each `grade = 1..=7`, then each
`m = 0..=min(6, 3*grade)`, construct the reduced pool from the first `m`
natural-six matching tiles and then the first `3*grade-m` nonmatching tiles in
physical domino-index order.

For the outer binding at matching count `m`, construct the grade-7 fixture by the
same rule, take its exact complement as a seven-tile focal hand, use S0/P30, and
bind the reduced coordinate to the least physical lead in that hand selecting
natural-six.  The reduced pool is asserted to be a subset of that root's hidden
pool; declaration and led context must agree.  At grade 7 the complete
action-derived context must equal the task.  At lower grades the exact reduced
pool and capacity are reconstructed from the named fixture rule and are not
misrepresented as the root's physical hidden support.

This yields 46 context tasks: four at grade 1 and seven at each grade 2 through
7.  It covers every supported grade and every feasible `m` regime.  Grades 1
through 4 have scalar/direct/GPU parity.  Grades 5 through 7 have scalar/GPU
parity plus the exact complete-direct preflight stop at respectively 756,756,
17,153,136 and 399,072,960 worlds.

### 12.3 `SameContextPairArmV1`

Use the exact existing `tests/same_context.rs` generating rule: declarations in
`Decl::ALL` order; first physical domino index ascending; second index strictly
greater and ascending; retain exactly pairs with equal effective led context;
construct the root hand from the pair followed by the first five other physical
dominoes; use S0/P30.

There are 56 pairs per declaration and 504 pairs total.  Run one grade-7 GPU
context task per pair and produce two physical binding records, one for each
lead.  Both GPU-derived payload digests must equal the scalar payload digest;
the two endpoint semantic identities within that pair must differ.

The 1,008 endpoint instances contain only 946 distinct physical semantic tuples;
62 occurrences repeat an identity from another pair ordinal and the maximum
multiplicity is 33.  This is accepted generator geometry.  Equality across pair
ordinals is permitted when root/action/context are genuinely identical.  The
receipt carries a separate evidence-instance id `(arm,pair,endpoint)` and never
salts that ordinal into the physical semantic identity to manufacture uniqueness.
Complete-direct preflight must declare the grade-7 stop before enumeration.

### 12.4 Carrier totals and order

The canonical order is Reduced, GradeMatching, SameContextPair, then the order
inside each generator above:

```text
context GPU tasks                    64 + 46 + 504 = 614
reduced evidence binding records     64 + 39       = 103
physical action binding records      7 + 1008      = 1,015
all binding record instances                       = 1,118
direct parity tasks                  48 + 25       = 73
direct declared stops                16 + 21 + 504 = 541
```

`ReducedEvidenceBindingV1` and `PhysicalActionBindingV1` have distinct magic/tag
domains.  A reduced record binds the carrier profile, root selector, physical
context-selecting action and exact reduced coordinate; it never claims that its
reduced pool/capacity is a physical `OpeningRootV1` result.  A physical record
binds the complete grade-7 root and action-derived context.  Only the latter is
an action-envelope result.

The complete canonical task-key stream is serialized and hashed before GPU
execution.  A count or order drift is a carrier failure, not a new result.

---

## 13. Completion, timeout and no partial result

The official Metal runner polls a committed command buffer for at most 120,000
milliseconds per arithmetic or projector command.  Success requires the Metal
completed status and no command-buffer error.  Timeout, not-enqueued, not-
committed, scheduled-without-completion, error and unknown statuses all fail
closed.  No timeout fallback, smaller task, CPU substitution or partial promotion
exists under the same receipt.

The receipt generator supervises the Metal runner as a child process.  The child
uses only the default retained-reference `commandBuffer()`, never an unretained
variant, and retains every bound buffer while polling.  On a nonterminal timeout
it writes and flushes the fixed matching `TERMINAL(command,TIMEOUT)` failure
frame, then calls `process::exit(124)` so no Rust destructor drops or reuses
in-flight resources; the operating system tears down the child.  The parent
accepts no partial child output and emits only the typed zero-accepted failure
receipt.  `waitUntilCompleted` is not used because it cannot implement the
timeout.

Child-to-parent control traffic is a closed length-delimited frame protocol with
exactly four progress kinds:

```text
PREPARING   CPU/toolchain/preflight work; monotone phase and unit ordinals
COMMITTED   one exact arithmetic or projector command ordinal was committed
TERMINAL    the same command ordinal reached a named terminal result
FINALIZING  post-run validation and canonical-receipt construction
```

The state machine is `PREPARING*`, then repeated
`COMMITTED(command) -> TERMINAL(same command) -> PREPARING*`, then
`FINALIZING*`, then exactly one complete success frame.  The parent rejects an
unknown kind, malformed length, skipped or repeated ordinal, invalid transition,
terminal frame for the wrong command, output before finalization, trailing bytes
or success without the complete expected command census.

The 125,000-millisecond hard command watchdog is armed only by `COMMITTED` and
can be disarmed only by the matching `TERMINAL`; no heartbeat or unrelated frame
extends it.  The child's internal nonterminal limit remains 120,000 milliseconds.
CPU-only `PREPARING` and `FINALIZING` phases instead use a separate conservative
600,000-millisecond liveness timeout since the last valid progress frame.  Their
work is divided at deterministic carrier/task/section boundaries and emits a
monotone progress frame at least once per completed bounded unit and at least
every 30,000 milliseconds whenever control returns between units.  These CPU
frames are noncanonical transport, never receipt content, and cannot satisfy a
command watchdog.

Silence past the applicable deadline, malformed progress, a stuck call before
polling begins, child crash or signal causes the parent to kill/reap the child and
emit the same typed zero-accepted failure receipt.  No child-produced success
fragment is retained.

Before the first dispatch, the host validates the complete carrier, CPU
comparands, semantic and choose-table identities, metallib, kernel functions,
pipeline limits, arena caps, every allocation and the full task-key stream.

Any of the following voids the complete official run:

- allocation, encoder, pipeline, command-buffer, timeout or completion failure;
- malformed descriptor, unknown ABI/status/opcode or hard shader status;
- changed poison, unwritten in-range word, guard write or out-of-range ordinal;
- arithmetic overflow outside a declared checked-undefined arithmetic case;
- more than ten strata for a response or more than 11,730 accepted cells;
- duplicate, missing or noncanonical cell;
- CPU/GPU field, byte, response-total or mass mismatch;
- root/action/context, carrier, compiler, library, table, build or freeze identity
  mismatch;
- receipt regeneration or fresh-metallib byte mismatch.

The official receipt then has exactly:

```text
accepted_arithmetic_case_count = 0
accepted_context_task_count = 0
accepted_reduced_binding_count = 0
accepted_physical_binding_count = 0
accepted_payload_bytes = 0
partial_result_retained = false
```

A typed diagnostic failure record may identify the first failing ordinal and
reason.  No earlier successful task or command becomes M2 evidence.

Timeout and incomplete-command behavior are tested through an injected host
completion-state source; M2 does not submit an intentionally nonterminating
kernel to the machine.  Corruption, cap, malformed ABI, poison and status
controls are similarly distinguished from the official accepted carrier.

---

## 14. Canonical M2 receipt

The binding M2 contract fixes the complete closed binary schema: magic, version,
section order, field widths, little-endian encoding, length domains, digest
domains and every success/failure status.  No implementation-added field or
debug representation enters `M2MetalParityReceiptV1`.  Its required semantic
sections are:

- freeze-56 descriptor and parent freeze-55 descriptor digest;
- parent commit, old manifest/build identity and old artifact identities;
- M2 source manifest and `M2BuildIdentityV1`;
- Rust version/target and exact binding packages, features and checksums;
- Xcode build, Metal component id/build, compiler and SDK versions, normalized
  compiler/link invocations, source and final metallib digests, and tool digests;
- sanitized device name, unified-memory fact and observed device/pipeline limits,
  excluding serial, registry and path-derived identifiers;
- `SemanticTablesCanonicalV2` identity and `OpeningChooseTableV1` identity;
- arithmetic corpus generator, input, independent-oracle output and GPU-output
  identities, counts, poison and completion status;
- carrier generator/version, complete length-delimited task-key-stream digest
  and all arm/task/binding-record counts;
- fixed arena capacities, allocated bytes, accepted-cell high-water, poison and
  guard results, timeout policy and no-partial marker;
- per context task: ordinal, arm, grade, pool and matching masks, response/slot/
  cell counts, complete context key, CPU/GPU cell ABI digests, canonical payload
  digests, exact total mass and status;
- domain-separated reduced evidence and physical action binding sections with
  their exact fields, selected action, referenced context-task ordinal, payload
  digest and semantic identity;
- global CPU/GPU payload digests, mismatch count, accepted counts and success
  sentence.

Accepted payload bytes count each validated compacted context payload exactly
once, not once per binding reference.  The success header fixes
`accepted_arithmetic_case_count=16,384`, `accepted_context_task_count=614`,
`accepted_reduced_binding_count=103`, and
`accepted_physical_binding_count=1,015`.

Task, choose-table and arithmetic input buffers are hashed before and after every
command and must be immutable.  Global stream digests are over explicit
versioned, length-delimited records, never an ambiguous concatenation.

Raw context output is never a persisted reusable value.  The canonical receipt is
generated twice from fresh process state and must be byte-identical.  Volatile
timing, process ids, absolute paths, registry ids, timestamps, temperatures and
counter samples are excluded.

---

## 15. Lean and serious-adjudication boundary

M2 does not pretend that finite Rust/Metal parity proves semantic correspondence
in Lean.  Add `Texas42.Trick1MetalFoundation` and audit its exported theorems with
`#print axioms`.  At minimum it proves the numeric facts frozen here:

- the 79,800 rectangular slot cap;
- the 5,109,296-byte projector-arena equality;
- the 2,359,424-byte arithmetic-arena equality;
- grade coverage 1 through 7 and the 46-task grade/matching count;
- the at-most-ten matching-vector bound for every response;
- stable filtering of a fixed lexicographic slot stream preserves GT1-A4 order;
- the no-partial acceptance shape: a failed conjunct yields zero accepted tasks
  and payload.

Still-open proof debt is stated, not hidden: the semantic response partition,
`A/C/W` formulas and conservation, Rust/Lean correspondence, and Metal/Rust
semantic correspondence.  M2 addresses the last item by exact carrier parity and
ABI receipts, not by theorem.

---

## 16. Blocking gate order

1. Adversarially review this rebrief and resolve every blocking ambiguity.
2. Write and adversarially review the complete binding M2 contract, including
   canonical encodings; compute its digest.
3. Append GT1-A10..GT1-A17, bind that contract digest and fix freeze 56 without
   changing old ruling bytes.
4. Verify every freeze-55 manifest entry against the parent commit and every old
   artifact against its frozen digest.
5. Implement `walt-metal`, checked-in MSL, the historical verifier, the new source
   manifest path, negative controls and the Lean target.
6. Rebuild the final metallib twice from fresh directories and compare both with
   the committed library.
7. Run the checked-in elevated Rust Gate-0/U256 corpus.
8. Run ABI, poison, guard, malformed-descriptor, timeout-classifier and no-partial
   controls.
9. Run the grade-7/m=6 maximum-cell task as a discarded preflight smoke, then run
   all 614 official tasks anew in canonical Reduced, GradeMatching,
   SameContextPair order.  Only the latter run enters the receipt.
10. Generate the complete receipt twice from fresh state and compare bytes.
11. Run historical-manifest verification, new source-manifest verification,
    formatting, clippy/warning/no-float gates, release workspace tests, Lean
    builds and the official elevated Metal gate as one conjunction.

Only step 11 green may issue the M2 success sentence.  Any failure is retained as
a typed failure and stops the claim; it is not an invitation to change the carrier
or arithmetic after seeing the result.

---

## 17. Proposed adjudication range

The append-only ruling should separate these decisions so later work can cite
them without importing unrelated implementation detail:

```text
GT1-A10  explicit rebrief and Gate-0 supersession
GT1-A11  exact M2 scope, claim fence and carrier
GT1-A12  U256 corpus and OpeningChooseTableV1
GT1-A13  context work unit, task/slot ABI and canonical order
GT1-A14  scheduler, arenas, timeout, poison and no-partial semantics
GT1-A15  Rust binding, unsafe boundary, no-float ruling and compiler identity
GT1-A16  historical/current persistence, receipt and blocking gate
GT1-A17  freeze-56 descriptor and range close
```

Freeze 56 should name the generating rules and versioned profiles, not turn
derived counts, observed limits or file hashes into parallel mathematical
authorities.  Those remain asserted consequences in the receipt.
