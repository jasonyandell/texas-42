# The GPU side track: exact arithmetic on Metal, and why it never became a player

[Home](Home.md) · owns: the GPU-native trick-1 track — its authority chain,
narrow opening-root scope, the exact objects it fixed, what each rung earned
and what each rung excludes, the frozen-but-unbuilt M3 net, the freeze-56 v2
re-issue at the unified layout, and the debts and cards it left open · Sources:
[`gpu_native_trick1_implementers_guide_v0.2.md`](../walt/math/gpu_native_trick1_implementers_guide_v0.2.md)
(received, verbatim),
[`GPU-NATIVE-TRICK1.md`](../walt/GPU-NATIVE-TRICK1.md) (adjudicated v0.3),
[`gpu_native_trick1_m2_rebrief_v0.1.md`](../walt/math/gpu_native_trick1_m2_rebrief_v0.1.md),
[`GPU-NATIVE-TRICK1-M2.md`](../walt/GPU-NATIVE-TRICK1-M2.md) (v1 + §13),
[`gpu_native_trick1_m3_rebrief_v0.1.md`](../walt/math/gpu_native_trick1_m3_rebrief_v0.1.md),
[`GPU-NATIVE-TRICK1-M3.md`](../walt/GPU-NATIVE-TRICK1-M3.md) (v1);
[`walt/CENSUS-RULINGS.md`](../walt/CENSUS-RULINGS.md) GT1-A1..A24 and
FZ-A1..A6; the three source manifests under `walt/math/gpu_native_trick1_*_sources_*.sha256`;
the receipts under [`walt/receipts/`](../walt/receipts/)
(`gpu_native_trick1_gate0_2026-08-16.txt`, `gpu_native_trick1_m0_m1_v1/`,
`gpu_native_trick1_m2_v1/`);
[`Trick1Foundation.lean`](../lean/Texas42/Trick1Foundation.lean),
[`Trick1MetalFoundation.lean`](../lean/Texas42/Trick1MetalFoundation.lean),
[`Trick1PerfectRecallNet.lean`](../lean/Texas42/Trick1PerfectRecallNet.lean)
and its eight submodules; the code — `walt::spec` and `walt::carrier` inside
[`walt/walt/`](../walt/walt/), the GPU trio `walt-gpu-ref`, `walt-metal`,
`walt-m2-runner`, the gate scripts under [`walt/ci/`](../walt/ci/); the
exploratory probe records under [`walt/probes/m3/`](../walt/probes/m3/);
the kanban cards [[gpu-level2]], [[m2-receipt-reearn]], [[m2-runner-trace]].
Related: [walt hub](walt.md), [freeze register](walt-math-freezes.md),
[received artifacts](walt-math-intakes.md), [instruments](walt-instruments.md),
[the kernel](lean.md), [counted-belief era](walt-counted-belief-era.md),
[how walt plays](walt-seat-play.md), [rob](rob.md),
[proof-assistant plan](proof-assistant-plan.md), [decision-sparse](walt-decision-sparse.md).

> **Epistemic tier: EXPLORATORY — below every tier on
> [Home](Home.md#evidentiary-tiers--never-promoted-never-blurred).** The
> received guide, the adjudicated contracts, the Rust and MSL code, the tests,
> the receipts and every number on this page sit inside walt's exploratory
> fence and are cited by nothing above it. Two exceptions are labelled where
> they occur: the three Lean modules are **proof-assistant kernel** facts about
> Lean-defined arithmetic (what they mean *for walt* stays exploratory), and
> the rob prose-rules bridge reads rob's resolver as a **dev-time oracle**, never
> as a status. Metal parity evidence is not a root verdict; a finite receipt is
> not a theorem; a green gate is evidence, never a status change.

**In one paragraph.** In August 2026 the project asked whether an exact
imperfect-information *opening* player — a seat that leads the first trick
against every one of the 399,072,960 deals compatible with its own seven tiles
— could be built on the Apple GPU. A design from the Pro channel was preserved
verbatim, repaired in four places, and turned into a gated ladder M0–M5. Two
rungs closed, each with exactly one admitted sentence: **PORTABLE M0/M1
COMPLETE under freeze 55** (2026-08-16) and **M2 METAL PROJECTOR PARITY
COMPLETE under freeze 56** (2026-08-17). Those sentences say that exact 256-bit
integer arithmetic and the *opening-response projector* — a quotient of the
399 million deals into at most 11,730 cells — are computed identically by a
portable Rust reference and by two Metal kernels, byte for byte, on a frozen
carrier. They establish **no action value, selected lead, optimal set,
information net, continuation, performance claim, or player**. The third rung,
the perfect-recall information net (M3), had its contract frozen the same day
(freeze 57) and was never built: its production crates were deleted as
unbuildable work-in-progress a week later, and the program was parked. What
survives is the discipline (verbatim source → repaired contract → numbered
ruling → numbered freeze → source manifest = build identity → receipt earned
twice → one sentence), the no-float CI gates, the `walt::spec` and
`walt::carrier` modules, three Lean modules, and the observation that the
projector-cell quotient — not the GPU — is the idea that any exact solve
earlier than trick 4 would need.

---

## 1. The wall, and the idea

Plainly: at trick 1 the seat knows its own seven tiles and nothing else. The
other 21 tiles are split 7/7/7 among three hidden seats in

```text
N0 = 21! / (7!)^3 = 399,072,960
```

ways, all equally likely under the uniform opening prior. An exact solve that
carries a posterior over those deals through the hand is the object every
walt program wants and none has reached from the opening.
(N0 is v0.3 §4.1; as a `Nat` identity it is kernel-proved in
`Trick1Foundation.openingDealCount_eq_multinomial` — **kernel as arithmetic**;
as a cardinality of a deal type it remains open in Lean, see [lean](lean.md) §7.1.)

How far exact enumeration gets was measured on the one frozen carrier this
track owns, receipt hand 8, by walking the boundary backward from trick 4
toward trick 1. **Probe record, not gate-pinned**
(`walt/probes/m3/ladder_results_2026-08-17.txt`, produced by
`walt/walt/src/bin/ladder.rs`; objective pmake; uniform boundary posterior over
the void-consistent support, the freeze-57 convention):

| Boundary | Support (worlds) | Outcome | Cost |
|---|---:|---|---|
| t = 4 | 1,200 (equals the frozen M3 carrier exactly) | solved exactly; lawful pmake lead 3-3; all four values equal the frozen first-play fractions | 8,154,532 nodes, 4,788,028 memo entries, 139,113 distinct posteriors, **4.305 s** |
| t = 3 | 59,976 (void filter 756,756 → 59,976) | **DIED** — killed at ~600 s, first lead subtree unfinished, memo growth linear in nodes, no convergence signal | 300,000,000 nodes, 162,319,107 memo entries, 4,693,012 distinct posteriors |
| t = 2 | 7,399,392 | **DIED** — wall-clock budget | 158,990,336 nodes, 81,373,795 memo entries, 120.012 s |
| t = 1 | 399,072,960 raw assignments | **DIED** at support materialization (over the 30,000,000-world cap), before any solving | — |

The file's own verdict: "exact posterior-carrying enumeration solves t=4 in
seconds and dies at t=3; the equivariant quotient (M1/M2 projector cells) is
load-bearing for every earlier trick." That quotient is the track's idea. Fix
the seat's lead. The three hidden seats respond; what matters about a deal for
the rest of the hand is *which* three tiles came back and *how many* matching
tiles each responder still holds — not which of the 399 million deals it was.
Grouping deals by that (response triple, remaining-matching-count vector) key
gives at most **11,730** nonempty cells, each carrying an exact integer mass,
and the masses of all cells sum to exactly

```text
N0 · 420^3 = 29,566,517,460,480,000
```

(§3 below has the formulas; the counts are gate-pinned in
`walt/walt-gpu-ref/tests/m1.rs:149–179` and kernel-proved in
`Trick1Foundation.openingCellCount_values`). A GPU was the proposed way to
push that quotient through the rest of the hand. The quotient was built and
parity-gated; the push never happened. The program that did reach trick 1 —
by counting 116,280 acting-seat hands instead of enumerating 399 million deals
— is the [counted-belief era](walt-counted-belief-era.md) §1, on the CPU.

---

## 2. The adjudication discipline

This is the reusable pattern. Each stage produces one immutable artifact
whose identity the next stage names.

1. **The received artifact, verbatim and checksum-gated.** The Pro-authored
   implementer's guide v0.2 (82,740 bytes) is preserved byte-for-byte at
   `walt/math/gpu_native_trick1_implementers_guide_v0.2.md`, SHA-256
   `ee2e78da20eb7d087fb121f467a56bafc0179a45fb692ca0b938f4c4210b6a44`
   (original source commit `ca18bc68…`, intake commit `c230949c`, 2026-08-16).
   Every `walt/ci/check.sh` run re-checks that hash. It records what was
   received; it is never silently repaired.
2. **The repaired contract.** `walt/GPU-NATIVE-TRICK1.md` v0.3 (29,607 bytes,
   SHA-256 `6190e740…`) is binding wherever it narrows, repairs or rejects
   v0.2. Its §8 classifies all fifteen received claims (derived / repaired /
   proof debt / rejected). The four hard repairs are §2.1 below.
3. **The numbered ruling range.** `walt/CENSUS-RULINGS.md` GT1-A1..A9
   (2026-08-16), then A10..A17, then A18..A24 (2026-08-17); each range closes
   with "RANGE (RE-)FROZEN … chapter closed", and reopening needs an explicit
   rebrief (the M2 and M3 rebriefs, 44,079 and 44,738 bytes, SHA-256
   `91831325…` and `07b3c993…`, are those rebriefs — ruling-carried hashes,
   see [received artifacts](walt-math-intakes.md) §3).
4. **The numbered freeze with a byte-exact descriptor.** Freeze 55 fixes
   `GT1_FREEZE_SET_DESCRIPTOR_V1` (944 bytes, SHA-256 `9b181092…`); freeze 56
   fixes `GT1-M2-FREEZE-SET-V1` (899 bytes, `7bdc5e05…`); freeze 57 fixes
   `GT1-M3-FREEZE-SET-V1` (962 bytes, `e5efe6ce5c293b29fc05902e7bf913fd13f04a031c2951f7a1bf5cf92137f852`).
   A descriptor names its contract's hash, its parent descriptor's hash and
   the parent commit, every profile, task, carrier and ABI it binds, and an
   `excluded=` field listing what it may never be read as claiming. The
   descriptor bytes and their hash are embedded in every receipt.
5. **The source manifest, whose bytes are the build identity.** A sorted
   `sha256  path` manifest pins every proof-path source (Rust, MSL, TOML,
   Lean, both contracts, the rulings file, the scripts, the metallib). The
   SHA-256 *of the manifest bytes* is the build identity that receipts embed:
   `eccf0a37…` (M0/M1 v1, 184 entries), `257d2fdb…` (M0–M2 v1, 381 entries),
   `8a780895…` (v2, 282 entries). No caller supplies the identity; the
   manifest excludes itself and every receipt.
6. **The receipt, earned twice from fresh state.** M0/M1's canonical envelope
   and grade-5 stop are regenerated from fresh state and byte-diffed against
   the committed comparands on every gate run; M2's binary receipt must be
   produced by two fresh complete runs that equal each other *and* the
   committed comparand *and* its external checksum.
7. **The sentence.** Each freeze admits exactly one green sentence, and a
   failed conjunct emits no sentence and keeps no partial result: a failed
   gate is a result, never permission to change the measured object.

**The identity DAG is acyclic by construction.** Contract → freeze descriptor
(names the contract's hash) → source manifest (pins the contract, the rulings
file with the descriptor in it, and the code) → build identity (hash of the
manifest) → receipt (embeds build identity and descriptor hash) → sentence.
Nothing downstream is an input to anything upstream: the contract may be
edited only *before* its digest is appended to a ruling (M2 §12 "freeze rule"),
the manifest excludes itself and all receipts (M2 §11), and a receipt is
compared, never re-read as a source. This is what makes "verify by hashing"
meaningful three weeks later (§6).

### 2.1 The four repairs

Each is a place where the received design would have been wrong in a way that
a green test could not have caught.

1. **A narrow root, not a generic public state.** The first API is exactly
   `OpeningRootV1` (§3.1): one profile each for prior, evidence, field,
   utility, horizon and scale, named in the type and the digest. Changing any
   one is a new profile that re-enters adjudication and invalidates every
   uniform-cell shortcut. (v0.3 §2.1; GT1-A2.)
2. **Count already lying in the unresolved trick counts.** The received
   decidedness test ("remaining trick points plus count on unplayed count
   tiles") is false mid-trick. The permanent blocking fixture (v0.3 §3):
   no-trump, P37 by S0, hands `S0: 00 11 22 33 44 66 54`,
   `S1: 55 60 61 62 63 64 65`, `S2: 10 21 32 43 50 51 53`,
   `S3: 20 31 42 30 41 40 52`; after six tricks led and won by S0
   (`00 60 10 20`, `11 61 21 31`, `22 62 32 42`, `33 63 43 30`, `44 64 50 41`,
   `66 65 51 40`) S0 has banked 31 and the defenders 0; at the prefix `54 55`
   the only unplayed tiles are the non-count `53 52`, so the received formula
   returns `P_live = 1 ≤ R = 5` and calls the contract safe — yet S1's `55`
   wins an 11-point trick and sets it. The repaired bound uses
   `P_unbanked = 42 − declaring_banked − defending_banked`, which includes
   count tiles already played into the current trick; the only lawful free
   tests are `defending_banked > R` (exact failure),
   `defending_banked + P_unbanked ≤ R` (exact success), otherwise live. The
   state-tied invariant `score0 + score1 + unbanked = 42` with the unresolved
   trick's tiles inside `unbanked`, preserved by every legal step, is
   kernel-proved in `Trick1Foundation` (GT1-A8) — **kernel as arithmetic**.
3. **Exact masses are role- and frame-typed.** A support count, a per-world
   likelihood coefficient, a cell mass, a conditional value and a weighted
   contribution are different roles; an additive value carries a frame (prior,
   field, utility, measure role, elapsed and full-horizon field exponent,
   task/root identity). Addition and comparison reject unequal frames. The
   contract's own example: raw numerators `1` at exponent 2 and `1` at
   exponent 3 do not add to `2`; at the common exponent 3 they add to `421`.
   (v0.3 §4.3; GT1-A3.)
4. **The information key must equal, not merely contain, the observation.**
   At every focal decision `key(x) == key(y)` iff x and y are the same lawful
   perfect-recall information state. Including the canonical observation among
   other fields is insufficient (received claim 15 rejected): take two hidden
   components in the same true information state with payoff vectors `(1,0)`
   and `(0,1)`; if an internal descriptor splits them, each segment picks its
   own action and reports value 1, while any lawful common action has value
   1/2 — strategy fusion, with the correct hand and history still present in
   both keys. A derived policy descriptor may enter the key only after a proof
   or exhaustive finite check that it is a function of the canonical key and
   cannot refine the partition. (v0.3 §6; GT1-A7; "information-key
   equivalence" is named proof debt, §8.)

### 2.2 Authority and provenance

| Surface | Identity | Authority in this track |
|---|---|---|
| Received v0.2 guide | source commit `ca18bc6807b974b31d4640786d7a2d63ae0b79fe`; intake commit `c230949c77ff7e8e22f912ed70f8206488ac9022`; SHA-256 `ee2e78da…`; 82,740 bytes | Preserved design input, byte-for-byte; checksum-gated per run. Not authority over a repair. |
| Adjudicated v0.3 contract | [`walt/GPU-NATIVE-TRICK1.md`](../walt/GPU-NATIVE-TRICK1.md), SHA-256 `6190e740…`, 29,607 bytes (unchanged since `3b4c6d60`) | Binding first-build design inside the exploratory track; governs wherever it narrows, repairs or rejects v0.2. |
| M2 rebrief and contract | rebrief SHA-256 `91831325…` (44,079 bytes); [`walt/GPU-NATIVE-TRICK1-M2.md`](../walt/GPU-NATIVE-TRICK1-M2.md) v1 **frozen at `aacb6df5…` (46,133 bytes = the blob at commit `20a9fecc`)**; the on-disk file hashes to `8695fea1…` (47,586 bytes) since the 2026-08-24 §13 append — see §6 | The mandatory bridge from freeze 55 and the exact M2 authority (GT1-A17, freeze 56). Narrows M2 to arithmetic and opening-projector parity. |
| M3 rebrief and contract | rebrief SHA-256 `07b3c993…` (44,738 bytes); [`walt/GPU-NATIVE-TRICK1-M3.md`](../walt/GPU-NATIVE-TRICK1-M3.md) v1, SHA-256 `79de73e9…`, 152,251 bytes, 2,803 lines (unchanged since `e6cd9586`) | The M3 gate authority (GT1-A24, freeze 57). Authorizes only the gate; records no result. |
| Append-only rulings and freezes | [`walt/CENSUS-RULINGS.md`](../walt/CENSUS-RULINGS.md) GT1-A1..A24 (range closed), FZ-A1..A6; [freezes 55, 56, 56 v2, 57](walt-math-freezes.md) | The adjudication and deterministic-encoding record. Freezes bind exact measured objects and prove no mathematics. |
| Executable evidence | `walt::spec` (formerly `walt-gpu-spec`), `walt-gpu-ref`, `walt-metal`, `walt-m2-runner`, `walt::carrier` (formerly `walt-m3-carrier`), their tests; receipts under `walt/receipts/` | Evidence about implemented carriers and the parity gates, at exploratory tier. The M2 receipt is **old-layout evidence** under freeze-56 v2 (FZ-A3). |
| Lean modules | `Trick1Foundation.lean`, `Trick1MetalFoundation.lean` (built and audited per gate run); `Trick1PerfectRecallNet.lean` + 8 submodules (statements committed; build unverified) | Kernel-facing finite foundations only; Rust/Lean and Metal/Rust correspondence are named debt. |

---

## 3. The exact objects

### 3.1 `OpeningRootV1` and its five closed profiles

The one model the first slice accepts (v0.3 §2.1; GT1-A2):

```text
OpeningRootV1
  declaration               one walt-core (now walt::rules) declaration
  focal/bidder/leader/actor  one seat; all four roles equal
  focal_hand                 exactly seven distinct dominoes
  contract_normal_form       PointBid(30..41) or Mark, with the bidder
  loss_budget                derived view only: 42-bid or 0, asserted <= 12
  evidence_profile           IgnoreAuctionEvidenceV1
  prior_profile              UniformCompatibleOpeningDealsV1
  field_profile              UniformRandomLegalV1
  utility_profile            DeclaringTeamMakesV1
  horizon_profile            OpeningStraightHand21FieldActionsV1
```

Preconditions: empty public record and current trick; the hidden support is
the complete ordered 7/7/7 allocation of the other 21 tiles; `loss_budget` is
derived from the normal form and can never be supplied independently; the
bidder's partnership is the declaring team; legal leads, led contexts, trick
resolution and count values come from the rules layer only. Ignoring auction
evidence is a declared model choice, not a claim that it is irrelevant — the
standing 90-world witness has two legal histories with identical hidden
support and opposite optimal leads ([lean](lean.md) §6): **support ≠ belief**
even when the set of worlds is unchanged. Every persisted object is keyed by a
digest over the freeze set, canonical table bytes, declaration, all five
profiles, information-key version, arithmetic ABI and build identity; a digest
mismatch means *corrupt for this run*, not stale-but-reusable.

### 3.2 The scale and the integer

Under `UniformRandomLegalV1` a field seat with `k` legal moves plays each with
probability `1/k`, and `k ∈ 1..7` always divides

```text
L = lcm(1..7) = 420,
```

so one field action contributes the integer factor `420/k` and `e` field
actions share the denominator `420^e`. The three opening responses use
exponent **3**; the full hand's 21 field actions use exponent **21**. With
`D = N0 · 420^21`, `D` has bit length **212** and `42·D` has **217** magnitude
bits (a signed carrier would need 218). Hence the ABI integer:

```text
U256Mass = eight little-endian u32 limbs   (32 canonical bytes, limb 0 first)
```

with exactly five closed checked operations — zero/compare, checked add,
ordered checked subtract, checked `mul_small(u32)`, and `mul_pow_420(e)` as
`e` repeated `mul_small(420)` calls — undefined overflow encoded as all-zero,
never a partial limb prefix. No wide atomics; exact values combine by
deterministic segmented reduction. A legal-set size of zero is an error, not a
zero multiplier that silently annihilates mass. (v0.3 §4.1–4.2; GT1-A3; the
212/217 windows are kernel-proved in `Trick1Foundation.rootDenominator_bit_window`
and `utilityMagnitude_bit_window` — **kernel as arithmetic**.) Role and frame
typing (§2.1 item 3) sits on top: `SupportCount`, `LikelihoodCoeff`,
`CellMass`, `ConditionalValue`, `WeightedContribution` are checked newtypes,
and a horizon lift both multiplies by the power of 420 *and* advances the
elapsed exponent.

### 3.3 The projector cells

Fix a legal root lead `a` with selected effective led context `q`. Of the 21
hidden tiles, `m = |U ∩ context(q)| ≤ 6` match the context. For an ordered
distinct response triple `x = (x1, x2, x3)`, let `F` be the seats whose
response matched, `Z` the void seats, and `e_s` the matching tiles seat
`s ∈ F` still holds after responding (`Σ e_s = m − |F|`). After the responses
every hidden seat has capacity six; with `M'`, `N'` the remaining matching and
non-matching pools:

```text
A(e,x) = |M'|! / Π e_s!  ·  |N'|! / (Π (6−e_s)! · (6!)^|Z|)        support count
C(e,x) = Π_{s∈F} (420/(e_s+1)) · 60^|Z|                             coefficient at 420^3
W(e,x) = A(e,x) · C(e,x)                                             cell mass
Σ_{x,e} W(e,x) = N0 · 420^3 = 29,566,517,460,480,000                conservation
```

(`A` is implemented as checked products of binomials; the conservation proof
double-counts (deal, response) pairs.) The nonempty cells by `m`, with the
generator `C(3,f)·falling(m,f)·falling(21−m,3−f)·C(m−1,f−1)` summed over
feasible `f = |F| ≥ 1` and `(m,f) = (0,0)` contributing `falling(21,3) = 7,980`:

| `m` | 0 | 1 | 2 | 3 | 4 | 5 | 6 |
|---:|---:|---:|---:|---:|---:|---:|---:|
| nonempty cells | 7,980 | 1,140 | 2,166 | 3,408 | 5,172 | 7,800 | **11,730** |

The received rectangular bound `7,980 × 10 = 79,800` is safe but loose; 11,730
is the sharp maximum and a **hard fail-closed cap** — exceeding it is a
receipt failure, never a truncation. Emission order is a generating rule
(response indices lexicographic by seats 1, 2, 3; then matching-count vectors
in the same seat order; impossible vectors not emitted), not a stored list.
Pins: gate `walt/walt-gpu-ref/tests/m1.rs` (lines 149, 153, 178–179: the
seven counts, the conservation total, the maximum); the committed M0/M1
envelope records `opening_cells 11730` for its root; kernel
`Trick1Foundation.openingCellCount_values` and `openingCellCount_le_11730`
(GT1-A8). The `(response,e)` *partition* itself, the `A/C/W` formulas and the
global conservation are **named Lean proof debt** (v0.3 §10).

Two boundaries travel with the cells. **Same-context reuse:** under the frozen
field, root leads that select the same led context share the response law
and therefore the projector payload — exactly **504** physical opening-lead
pairs across all declarations do (`walt/walt-gpu-ref/tests/same_context.rs:114`)
— but they remain distinct semantic actions with distinct persisted envelope
identities, because the led tile still decides the winner, the count, the next
leader and the public record. **Merge boundary:** projector computations may
be deduplicated by identical shape/query bytes, but semantic masses merge
only inside one identical full public information key and one identical scale
frame; equal physical allocation shapes reached through different
actor-attributed histories are not one state (v0.3 §5.2). Uniformity of every
`(x,e)` cell holds only under the constant opening prior; a one-mask histogram
answers one marginal query and separate marginals are never multiplied into
a joint law (§5.3).

### 3.4 Root verdicts and the freeze-26 rule

With exact action values `Q` bracketed `L(a) ≤ Q(a) ≤ U(a)` in one frame, a
root distinguishes five outcomes (v0.3 §7):

| Verdict | Condition |
|---|---|
| `OptimalMember(a)` | `L(a) ≥ U(b)` for every competitor `b` |
| `UniqueOptimal(a)` | `L(a) > U(b)` for every competitor `b` |
| `CanonicalOptimal(a)` | member, and every lower-index competitor strictly excluded |
| `ExactOptimalSet(S)` | the relevant intervals collapse and establish exactly `S` |
| `Unresolved` | none of the above |

`U(a) < max_b L(b)` removes `a`; touching or overlapping intervals never
manufacture a tie. `OptimalMember` is an internal result, never by itself a
play. Freeze 26 chooses the **least-domino-index member of the exact argmax**,
so a playable `CanonicalOptimal(a)` needs non-strict separation from every
higher-index competitor and strict separation from every lower-index one,
unless an exact optimal set is already known. Reporting precedence is
`ExactOptimalSet`, `UniqueOptimal`, `CanonicalOptimal`, `OptimalMember`,
`Unresolved`; only the first three are playable. Upper contributions of
positive posterior components sum componentwise before any focal maximum;
lower contributions sum only under one jointly lawful global continuation
(a complete program instance suffices; a bare program ID does not).
Componentwise upper summation, one-shared-policy lower summation, dominance,
and member-versus-uniqueness are kernel-proved in `Trick1Foundation`
(GT1-A7/A8); the canonical least-index verdict is proof debt. This portable
slice computed none of these verdicts — it built no `Q^H`, no interval, no
selected play (GT1-A7).

---

## 4. What each rung earned

| Milestone | Status | What it does not establish |
|---|---|---|
| **M0 — portable arithmetic and semantics** | Source-complete; closed under freeze 55. `U256Mass`, role/frame types, `SemanticTablesCanonicalV2` generated from `walt::rules` (format 2, 14,884 bytes, SHA-256 `6595fadb…`), FIPS SHA-256 anchors, an independent big-integer test oracle, the complete prose-rules bridge to rob. | No Metal parity, speed result, root value or player. |
| **M1 — scalar opening projector** | Source-complete; closed under freeze 55. Independent closed-form and direct paths; every feasible `m = 0..6` reduced coordinate at grades 2–4; full-opening counts and mass; the grade-5 zero-output stop; same-context reuse without action collapse; canonical envelope/stop validators. | No 399,072,960-world direct enumeration, no grade-5 parity, no root action, and no claim that grade 4 predicts the opening. |
| **Lean: `Trick1Foundation`** (480 lines, 25 theorems, `3b4c6d60`) | **Kernel as arithmetic**; built by `walt/ci/check.sh` each run. Loss allowance ≤ 12; 1..7 divide 420; N0 identity; 212/217 windows; current-trick-aware unbanked invariant; the seven cell counts; interval algebra. | Not the `(response,e)` partition, `A/C/W`, conservation, key equivalence, least-index verdict, or Rust/Lean correspondence. |
| **Lean: `Trick1MetalFoundation`** (141 lines, 8 theorems, `813d5e81`) | **Kernel as arithmetic**; built each run and its axiom transcript byte-diffed against `lean/trick1_metal_foundation_axioms_v1.txt` (8 entries). Slot bound ≤ 79,800; projector arena `32 + 1936 + (79800+2)·64 = 5,109,296` bytes; arithmetic arena `2,359,424`; GradeMatching covers every grade with exactly 46 tasks; ≤ 10 matching vectors; stable filter; failed conjunction accepts nothing. | Not Rust/Lean or Metal/Rust correspondence. |
| **Lean: `Trick1PerfectRecallNet`** (facade 129 lines + 8 submodules 1,426 lines; 103 `#print axioms` lines; `97ce321a`) | **Statements committed; build and audit unverified** as of 2026-09-07 ([lean](lean.md) §7.4): committed in a commit titled "does not build", built by no gate, no build artifact found, receipt `lean/trick1_perfect_recall_net_axioms_v1.txt` lists 94 of 103 names and is consumed by no script. `Types.lean` alone was shown to elaborate (measured 2026-09-12 on the lean page). | Nothing yet, for any purpose — treated as exploratory until `lake build Texas42.Trick1PerfectRecallNet` and a regenerated receipt exist (a freeze event, FZ-A5). |
| **M2 — Metal parity** | **M2 METAL PROJECTOR PARITY COMPLETE under freeze 56** (earned at `a6df853c`, closure recorded at `20a9fecc`, 2026-08-17). Receipt = old-layout evidence since FZ-A3. | No action value, selected lead, optimal set, information net, continuation, performance claim, player or Lean correspondence theorem. |
| **M3 — perfect-recall net** | Contract frozen (freeze 57, GT1-A24, `e6cd9586`); `walt::carrier` holds the h8 carrier. **Gate authorized, never built, never passed; no M3 result recorded.** Production crates deleted 2026-08-24 (§5). | No strategy-fusion repair demonstrated by this track. |
| **M4 — representation growth** | Untouched. | No compression, basis-rank, memory-growth or GPU-speed claim. |
| **M5+ — stopped controller, gluing, opening attempt** | Untouched. | No opening player, root closure, optimal action or playable verdict. |

### 4.1 Rung 1 — PORTABLE M0/M1 COMPLETE under freeze 55 (2026-08-16, `3b4c6d60`)

The admitted sentence, verbatim from GT1-A9: a green integrated gate "may
report **PORTABLE M0/M1 COMPLETE under freeze 55** without a new ruling … It
still reports no Metal result and no opening-root verdict." The gate is the
conjunction of the checked source manifest, the committed canonical envelope
and grade-5 stop, fresh byte-for-byte regeneration, the guide checksum,
formatting, warning and float denials, release workspace tests, and the Lean
target — together, or the status is `IMPLEMENTED, GATE PENDING`.

What the rung contains, each number with its pin (all exploratory):

| Object | Value | Pin |
|---|---|---|
| Reduced direct-parity carrier `ReducedOpeningCarrierV1` | grades 2, 3, 4 hold 90, 1,680, 34,650 complete physical worlds (mandatory closed-form-vs-direct parity rungs); the declared two-root carrier gives **48** root-bound parity coordinates | `walt/walt-gpu-ref/tests/root_carrier.rs:103–106`; GT1-A5 |
| Grade 5 | 756,756 worlds > `M1_DIRECT_WORLD_CAP_V1 = 100,000` ⇒ a typed **DECLARED STOP** at each of **16** coordinates before enumeration, with emitted worlds/cells/payload all zero and no partial comparison retained; the cap governs only the direct arm, never the closed-form projector | committed `grade5_declared_stop_v1.bin` (grade 5, context q6, matching 6, pool bits `07e07fc0`, world_count 756756, cap 100000, emitted 0/0/0) |
| Prose-rules bridge (T1-A12 for this slice) | rob's `prose_resolver` agrees with walt's rules over **252** led-context cases, **29,232** compelled-follow legal sets, **2,948,400** actor-attributed distinct four-tile tricks (winner and points) — complete declared finite domains, not samples | `walt/walt-gpu-ref/tests/prose_bridge.rs:108, 114, 166`; rob is a dev-dependency only; runs in `check.sh` |
| Same-context reuse | **504** physical lead pairs share payload; distinct envelope identities | `same_context.rs:114` |
| Canonical envelope (root NT, S0, hand bits `0800003f`, PointBid30, loss budget 12, lead 6-6, context q6) | 321,167 bytes, SHA-256 `1127d386…`; payload 305,030 bytes (`50 + 26·cells` with 11,730 cells); build identity `eccf0a37…`; freeze descriptor 944 bytes `9b181092…` | `walt/receipts/gpu_native_trick1_m0_m1_v1/` (three files; summary 1,644 bytes `51a162ea…`; stop 16,177 bytes `7e8dfecf…`) |
| Metal Gate 0 | **NO-GO** on 2026-08-16: macOS 26.5.1 (25F80), Apple M5 Max, 18 CPU / 40 GPU cores, 48 GB, Metal supported, but only Command Line Tools selected — `xcodebuild`, `metal`, `metallib`, `metal-ar`, `xctrace` unavailable | `walt/receipts/gpu_native_trick1_gate0_2026-08-16.txt` (1,171 bytes, `b57f7077…`) — immutable; GT1-A10 superseded only the host precondition after full Xcode 26.6 was installed, and never rewrote this receipt |

**Reproduced 2026-09-13 on this machine.** The exact `check.sh` step is
`cargo --locked run --release -p walt-gpu-ref --example generate_m0_m1_receipts -- <tmpdir>`
followed by `diff -r receipts/gpu_native_trick1_m0_m1_v1 <tmpdir>`. Invoking
the prebuilt `walt/target/release/examples/generate_m0_m1_receipts` twice into
fresh directories took **0.021 s and 0.018 s** wall; both directories were
byte-identical to the committed comparands and to each other (`diff -r` clean;
SHA-256 `7e8dfecf…`, `1127d386…`, `51a162ea…`). Repeated 2026-09-13 with the
same prebuilt example into two fresh directories: both `diff -r` clean against
the committed comparands and against each other. A replay is evidence that the
comparands regenerate; it changes no status.

### 4.2 Rung 2 — M2 METAL PROJECTOR PARITY COMPLETE under freeze 56 (2026-08-17, `a6df853c`)

The admitted sentence, verbatim from GT1-A11: "M2 computes no action value,
selected lead, optimal set, information net, continuation, performance
crossover or player. Its sole admitted green sentence is **M2 METAL PROJECTOR
PARITY COMPLETE under freeze 56**." The freeze-56 descriptor's own fence:
`excluded=action-value,selected-lead,information-net,K-OPEN4+,performance,player`.

What the sentence covers: the same U256 arithmetic and the same opening
projector, recomputed by two MSL kernels (`u256_parity_v1`,
`opening_project_v1`) on the Apple M5 Max, agree word-for-word and
byte-for-byte with the portable scalar reference over a frozen carrier, in
two fresh complete runs that equal each other and the committed receipt.

| Census item | Value | Pin |
|---|---:|---|
| Arithmetic corpus `U256MetalCorpusV1` | **16,384** cases = a 4,288-case prefix built from a 32-value edge list + 12,096 SplitMix64 cases; independent `BigUint` oracle; 13 malformed controls per domain, never accepted results | M2 §6; GT1-A12; receipt header word `accepted_arithmetic_case_count` |
| Projector carrier `M2OpeningParityCarrierV1` | **614** tasks = 64 Reduced + 46 GradeMatching + 504 SameContextPair; 103 reduced bindings; 1,015 physical bindings; 73 direct-parity tasks; 541 direct stops; 1,118 binding instances; task-key stream 39,296 bytes | M2 §8; `walt-gpu-ref/src/m2.rs` constants; `Trick1MetalFoundation` (46 tasks) |
| Arenas | projector 5,109,296 bytes; arithmetic 2,359,424 bytes (both allocated to exactly their high-water) | M2 §10.7; `Trick1MetalFoundation` |
| Receipt `W42M2R01` | **420,738** bytes, 768-byte header, 10 sections with record counts 13/1/1/2/2/1/614/103/1,015/1; **mismatch 0, partial 0**; **76,639,960** accepted payload bytes (`Σ (50 + 26·cells)` over 614 tasks); build identity `257d2fdb…` at offset 64; freeze-56 descriptor hash `7bdc5e05…` at offset 96; ends with the 50 claim bytes | `walt/receipts/gpu_native_trick1_m2_v1/m2_metal_parity_v1.bin`, SHA-256 `0aa2f6ab…` — header re-parsed 2026-09-13 and again 2026-09-13 (magic `W42M2R01`, header 768, 10 sections, u32 fields 16,384 / 614 / 103 / 1,015 / 0 / 0, u64 76,639,960 at offset 56, `257d2fdb…` at 64, `7bdc5e05…` at 96, the 50-byte claim last), every word as the contract states |
| Toolchain | Metal toolchain 32023.883 (Xcode 26.6 build 17F113, SDK 26.5), `-std=metal3.2 -fmetal-math-mode=safe -fno-fast-math -Wall -Wextra -Werror`; two fresh-directory compiles give different AIR (source paths embedded) but identical metallibs | M2 §2; `check_m2_metal.sh` two-build phase |
| Checked-in library | `walt/walt-metal/shaders/walt_m2.metallib`, 14,348 bytes, SHA-256 `2bc886eb…` (the M3 contract pins this value; a *pre-freeze* smoke produced 6,877-byte libraries at `7ee31770…` — a recipe smoke, not the final digest, by the contract's own words) | hashed 2026-09-13 |
| Runner protocol | child polls a committed command ≤ 120,000 ms; parent watchdog 125,000 ms, unextendable; CPU phases 600,000 ms liveness; timed-out child exits 124; any failure yields a distinct 256-byte `W42M2F01` receipt with zero accepted counts | M2 §9–10.8; GT1-A14; `walt-m2-runner/src/protocol.rs` (32 unit tests) |

**Why there is no speed claim.** M2 is deliberately sequential — no atomics,
reductions, scans, indirect dispatch, slabs, adaptive batching or concurrent
commands (GT1-A14); one thread owns one ordered response and ten fixed slots;
stable host scanning fixes order independently of execution order. The gate
was built to show that Metal reproduces canonical exact objects rather than
introducing its own semantics. Nothing about throughput was measured, and the
descriptor's `excluded=performance` forbids reading one in.

The three device tests in `walt/walt-metal/tests/metal_device.rs` are all
`#[ignore = "requires elevated access to the local Apple GPU; run in release
mode"]`: the portable gate never exercises Metal. Only the elevated
`walt/ci/check_m2_metal.sh` does — portable `check.sh`, then an immutable HEAD
snapshot, `--locked` release build of `walt-m2-runner`, host/tool descriptor
and the two-build metallib, Rust Gate 0 and the U256 corpus and negative
controls, protocol timeout/no-partial tests, a discarded maximum smoke child,
official child 1 and child 2 from fresh process state, committed-comparand
adjudication, the Lean build and axiom audit, and a final source-identity
check. It last ran on 2026-08-17; whether it still runs to green after the
fold is untraced ([[m2-runner-trace]]; §8).

---

## 5. Freeze 57 and the unbuilt net

### 5.1 What the gate would have been

M3 is where the track would have first joined *decisions* across hidden
worlds under a lawful perfect-recall key — the object every strategy-fusion
worry is about. Freeze 57 (GT1-A18..A24, 2026-08-17 08:36, `e6cd9586`) fixed
the contract `GPU-NATIVE-TRICK1-M3.md` v1 (152,251 bytes, `79de73e9…`) and the
962-byte descriptor `GT1-M3-FREEZE-SET-V1` (`e5efe6ce…`, pinned in code as
`walt::carrier::FREEZE57_DESCRIPTOR_SHA256`). Its scope, per GT1-A19:

- **One carrier**: receipt hand h8 immediately before trick 4. S0 shaker; S1
  bidder/declarer/viewer/leader; P30 in fives; S1 holds `[21,31,33,55]` (the
  four roots, canonical indices 4/7/9/20); hidden pool
  `[00,20,30,40,41,43,50,53,60,62,64,66]`; prefix `S1:52 S2:63 S3:51 S0:65 /
  S0:42 S1:54 S2:44 S3:32 / S1:11 S2:22 S3:10 S0:61` (winners S0/S1/S1, points
  1/6/1). The support is the 34,650 void-free assignments filtered by the
  replayed voids (S2 void in called P5 and natural P1; S3 void in natural P4)
  to exactly **1,200 worlds**, built two independent ways that must agree
  byte-for-byte. Banked T1 = 7, T0 = 1; 34 unbanked; T0's P30 allowance 12,
  remaining 11. Carrier profile `(UniformCompatibleSupportV1,
  HistoricalVoidFeasibilityOnlyV1, UniformRandomLegalV1)`; the carrier admits
  only the exact `rob/receipts/verify_player.txt` (6,650 bytes, `cf2c9dd2…`).
- **Two objectives**: M3A the future T1-minus-T0 trick differential; M3B
  whether T1 makes P30 (an independent loss automaton: each future T0 trick
  adds `1 + count` to a spend in `0..34`; make iff spend ≤ 11).
- **Two treatments**: H, lawful perfect recall — `M3PerfectRecallKeyV1` is
  exactly S1's scoped observation with complete own action-observation memory
  and no hidden-world identity; C, world-revealed — `M3WorldRevealedKeyV1`, a
  disjoint type, a per-world clairvoyant walk averaged over the support.
  Eight tasks (2 objectives × 4 roots), each H then C.
- **The conjuncts**: CPU/Metal parity of H and of C at every root; for M3A,
  **strict `C > H` at all four roots** as a control (the information price
  must be visibly positive); for M3B parity without a predeclared gap;
  objective topologies equal root by root; sum-before-max, no key
  renormalization, no cross-world C pooling, no slab-local maximum; closed
  caps (H 2,048 / C 16,384 / task 32,768 / run 524,288 commands; task/run
  frames 131,072 / 2,097,152; Metal 512 MiB, host 2 GiB, spill 16 GiB);
  exactly two reduction families (`MASS_BUCKET` epochs 0..3,
  `BACKWARD_VALUE` epochs 3..0); a 36-control registry; a 4,096-byte
  checkpoint; a 52,880-byte `W42M3R01` receipt and a 512-byte failure receipt;
  the Lean tree building and passing its axiom audit; two fresh runs
  byte-identical. Only the 14-step official order of M3 §14 may emit the
  53-byte sentence `M3 PERFECT-RECALL NET PARITY COMPLETE under freeze 57`.

Freeze 57 **authorizes the gate and records no result**; the sentence has
never been issued.

### 5.2 The honest state, as of 2026-09-07 (`c00717d1`)

- The production, oracle and Metal crates (`walt-m3-net`, `walt-m3-oracle-a`,
  `walt-m3-metal`) were committed at `97ce321a` (2026-08-17 09:45) in a commit
  titled "WIP: M3 perfect-recall net scaffolding (mid-flight, does not build)"
  — per `walt/UNIFICATION-CENSUS.md` §1 cargo refused the first two outright
  and the third compiled only in isolation — and **deleted at `ad355e93`
  (2026-08-24 01:47)** as
  closure-clean orphans in unification stage 1. They are addressable at
  `97ce321a`; no later commit rebuilt them.
- `walt/walt-metal/shaders/02_m3_wavefront.metal` (27,918 bytes) and
  `build_m3_metallib.sh` survive as **orphans**: no consumer crate, and no
  checked-in `walt_m3.metallib` (the shaders directory holds only
  `walt_m2.metallib`).
- **No M3 source manifest** (`M3SourceManifestV1`) was ever issued, so no M3
  build identity exists to embed in a receipt.
- The Lean tree `Trick1PerfectRecallNet` is kept — FZ-A4 calls it
  "kernel-audited freeze-57 mathematics, protected work, not scaffolding" —
  but as [lean](lean.md) §7.4 records, that phrase is a ruling's wording:
  no artifact on this machine shows the tree building, no gate builds it, its
  axiom receipt is 94 names against a 103-name facade and is audited by no
  script. M3 `Types.lean` elaborates; the full tree's build is unverified.
- `walt::carrier` (formerly `walt-m3-carrier`) is **live**: 7 tests in
  `walt/walt/tests/carrier.rs` pin the receipt bytes, the freeze-55/56 commit
  hashes, the freeze-57 descriptor (962 bytes, `e5efe6ce…`), the two-way
  support construction and the 128-byte root-alias KAT. It is imported by the
  fixed-carrier bins (`scenario`, `level1`, `level2`, `playout`, `ladder`,
  `m3probe`, and the report bins `modelbeliefreport`, `unifiedreport`; grep
  2026-09-13), not by the live solver modules (`grep -l carrier
  walt/walt/src/solver/*.rs` is empty).
- The M0–M2 objects are unaffected: freeze 57's descriptor names freeze 56 as
  its parent, and the M3 contract §1 pins the 929,957-byte freeze-56 CENSUS
  prefix (`d573ac68…`), the M2 contract, receipt and metallib hashes — all
  re-verified 2026-09-13.

Whether freeze 57 is a live authorization or a historical artifact is not
ruled anywhere; the [[gpu-level2]] card treats "the M0–M3 gate lineage" as the
substrate to build on when the program is unparked.

### 5.3 Probe box — the only computed values on the M3 carrier

> **EXPLORATORY PROBE — below every tier; no receipt; the files say so.**
> `walt/probes/m3/results_2026-08-17.txt`, produced by
> `walt/walt/src/bin/m3probe.rs` ("This is NOT the freeze-57 M3 gate: no Metal
> parity, no receipt, no net census, and nothing here is a trick-1 statement
> (P-A21)"). Exact integers only: every field factor is scaled by 12 (twelve
> future field moves, legal sets of size 1..4), so a node's mass is an
> integer bounded by 1,200·12^12 < 2^54; every field node conserved 12× mass
> and the root mass was exactly `1200 · 12^12`. All argmax members are
> reported; no convention selects among ties silently.
>
> | Lead | M3A: H (T1−T0 differential) | M3A: C | M3B: H (P30 make) | M3B: C |
> |---|---|---|---|---|
> | 21 | 2647711/1036800 | 21483353/8294400 | 5204203/5529600 | 15722123/16588800 |
> | 31 | 490069/276480 | 424483/230400 | 5033873/5529600 | 5094649/5529600 |
> | 33 | 448279/138240 | 6912821/2073600 | **16078667/16588800** (≈ 96.92 %) | 1355971/1382400 |
> | 55 | **6817057/2073600** (≈ 3.288) | 699919/207360 | 7770169/8294400 | 5849/6144 |
>
> The lawful perfect-recall lead is **55 under M3A** and **33 under M3B**: the
> two objectives *disagree* on the lead on this carrier — the trick-differential
> proxy and the pmake objective (ruled the objective 2026-08-17) part company
> at trick 4 of a real receipt hand. `C > H` holds at all four roots under
> M3A, as GT1-A19 would require of the gate; a probe cannot discharge a gate
> conjunct, and none of these fractions is quotable as a result. The ladder
> (§1) reproduces the four M3B H values exactly as its t = 4 rung. (The
> prebuilt `walt/target/release/m3probe`, run 2026-09-13, printed all sixteen
> fractions, both `H-optimal` markers and the `1200 * 12^12` root-mass line
> exactly as the 2026-08-17 record has them.)

---

## 6. The fold and the v2 re-issue

The 2026-08-24 unification ([walt-architecture](walt-architecture.md)) moved
`walt-core`, `walt-kernel` and `walt-gpu-spec` into the one `walt` crate as
modules `rules`, `kernel`, `spec` (THE FOLD, `d1499d43`, pure code motion,
trace-identical). Freeze 56's source closure had pinned those paths by name.
Rather than edit a frozen manifest, the freeze was **re-issued append-only as
v2** (`c92175ae`, FZ-A1..A6) — a worked case of maintaining a freeze without
touching it:

- **v1 stays; v2 sits beside it.** `gpu_native_trick1_m0_m2_sources_v1.sha256`
  (381 entries, identity `257d2fdb…`) is byte-immutable forever — it is the
  build identity the standing receipt names. `…_v2.sha256` (282 entries,
  identity `8a780895…`) pins the post-fold closure; its identity is **attested
  by no hardware run**.
- **Translation is an amendment, not a reading.** `walt/ci/verify_m2_sources.sh`
  carries an explicit 32-entry fold-translation table (14 `walt-core`, 11
  `walt-kernel`, 7 `walt-gpu-spec` paths → `walt/src/{rules,kernel,spec}`, the
  unified `walt/Cargo.toml`, prefix-renamed tests). v1 pinned paths, the paths
  moved, and the table is the auditable record of where (FZ-A2).
- **Freeze-event verification, not per-commit.** The unified crate holds the
  actively developed solver, so a per-commit full-digest closure would be red
  on every ordinary commit. Since v2 the closure check runs at freeze events
  (`/bin/bash -p walt/ci/verify_m2_sources.sh`); `check.sh` keeps the per-run
  immutable checks — M0/M1 history at its producing commit
  (`verify_m2_history.sh`: 184 blobs at `3b4c6d60`, the 921,481-byte CENSUS
  prefix, the pinned artifact hashes), the guide checksum, the M0/M1 receipt replay,
  the Lean build and axiom audit (FZ-A5). Living append-only documents can
  thus be pinned by full digest in the closure without making every append a
  CI failure.
- **The standing receipt is old-layout evidence.** `m2_metal_parity_v1.bin`
  keeps its bytes and its meaning — hardware parity for the v1 identity — and
  is never presented as attesting v2 (FZ-A3). Re-earning it under v2 (the
  614-task carrier, twice, on hardware) is [[m2-receipt-reearn]], deferred to
  the program's unparking. This is the sentence "a green receipt is evidence,
  never a status change" made concrete: the status sentence stands, the
  receipt's *scope* is what changed.
- **Drift disposition (FZ-A4).** walt-strat's two files were reverted to their
  frozen digests; `lean/Texas42.lean`'s import of the M3 tree was absorbed into
  v2 by re-pinning; ordinary drift in the rulings file, `Cargo.toml` and
  `Cargo.lock` was absorbed by re-pinning. v2's package roots are the unified
  `walt/walt`, the GPU trio and the rob oracle set; `walt-wasm` stays outside
  (FZ-A6).

**The M2 contract's two hashes — verify by hashing.** The frozen identity
`aacb6df5e9106b3b6bf00ccfb496c71f762c0fb4644c13a17f76d2ac2f0326e3`
(46,133 bytes) is the blob at commit `20a9fecc`
(`git show 20a9fecc:walt/GPU-NATIVE-TRICK1-M2.md | shasum -a 256`). Since the
2026-08-24 §13 amendment — the append-only adjudication that §12's freeze rule
permits — the on-disk file hashes to
`8695fea19012b12e35f2891babcabfa0aa80fcef8456b83edec027bd8520ce4b`
(47,586 bytes), and that is the value the v2 manifest pins (line 145); the v1
manifest pins `aacb6df5…` (line 136). Both are correct for their layout; a
reader who hashes the working file and sees `8695fea1…` is looking at a freeze
that is intact, not broken. (Both hashes re-verified 2026-09-13.)

| Identity | Bytes / entries | SHA-256 | Where it lives |
|---|---:|---|---|
| Received guide v0.2 | 82,740 | `ee2e78da…` | `walt/math/…implementers_guide_v0.2.md` + `.sha256` (per-run) |
| v0.3 contract | 29,607 | `6190e740…` | pinned in both M0–M2 manifests |
| M2 contract, frozen | 46,133 | `aacb6df5…` | blob at `20a9fecc`; freeze-56 descriptor; v1 manifest |
| M2 contract, on disk (with §13) | 47,586 | `8695fea1…` | v2 manifest |
| M3 contract | 152,251 | `79de73e9…` | freeze-57 descriptor |
| Freeze-55 descriptor | 944 | `9b181092…` | `walt-gpu-ref/src/receipt.rs`; every M0/M1 envelope |
| Freeze-56 descriptor | 899 | `7bdc5e05…` | GT1-A17; M2 receipt offset 96 |
| Freeze-57 descriptor | 962 | `e5efe6ce…` | GT1-A24; `walt::carrier::constants` |
| M0/M1 manifest v1 → build identity | 184 entries (18,750 bytes) | `eccf0a37…` | envelope header; `verify_m2_history.sh` (per run) |
| M0–M2 manifest v1 → `M2BuildIdentityV1` | 381 entries | `257d2fdb…` | M2 receipt offset 64 |
| M0–M2 manifest v2 → post-fold identity | 282 entries | `8a780895…` | freeze-event verification only; no receipt |
| M0/M1 envelope / stop / summary | 321,167 / 16,177 / 1,644 | `1127d386…` / `7e8dfecf…` / `51a162ea…` | `walt/receipts/gpu_native_trick1_m0_m1_v1/`, byte-diffed per run |
| M2 receipt | 420,738 | `0aa2f6ab…` | `walt/receipts/gpu_native_trick1_m2_v1/` + `.sha256` |
| M2 metallib | 14,348 | `2bc886eb…` | `walt/walt-metal/shaders/walt_m2.metallib` |
| Gate-0 receipt | 1,171 | `b57f7077…` | `walt/receipts/gpu_native_trick1_gate0_2026-08-16.txt` |
| `verify_player.txt` (carrier input) | 6,650 | `cf2c9dd2…` | `rob/receipts/`; pinned by `walt::carrier` and M3 §1 |

Every hash in this table was recomputed 2026-09-13 and matches its source;
an independent recomputation on 2026-09-13 (`shasum -a 256` on every on-disk
file, `git show 20a9fecc:walt/GPU-NATIVE-TRICK1-M2.md | shasum -a 256` for the
frozen blob, the two descriptor lines of `walt/CENSUS-RULINGS.md` measured at
899 and 962 bytes, and the manifest entry counts 184/381/282 by `grep -c` of
64-hex lines) matched every row again.

---

## 7. Running it

For the engineer. All commands from the repository root unless stated; the
gate scripts self-bootstrap a scrubbed environment (`PATH=/usr/bin:/bin:/usr/sbin:/sbin`,
fixed `~/.elan/bin/lake`), so run them as written.

| What | Command | Cost / notes |
|---|---|---|
| Replay the M0/M1 receipts | `cd walt && cargo --locked run --release -p walt-gpu-ref --example generate_m0_m1_receipts -- <tmpdir> && diff -r receipts/gpu_native_trick1_m0_m1_v1 <tmpdir>` | milliseconds once built (0.02 s measured 2026-09-13); the per-run `check.sh` step, failure code 23 |
| Verify the immutable M0/M1 history | `/bin/bash -p walt/ci/verify_m2_history.sh` | per run inside `check.sh` |
| Verify the full source closure (v2) | `/bin/bash -p walt/ci/verify_m2_sources.sh` | freeze events only (FZ-A5); prints `PASS (N files; M2BuildIdentityV1=…)` |
| The portable gate | `/bin/bash -p walt/ci/check.sh` | release build + clippy `-D warnings -D float_arithmetic` + no-float greps over Rust/MSL/TOML + all release tests + `lake build Texas42.Trick1Foundation Texas42.Trick1MetalFoundation` + axiom diff; never needs a GPU; never issues an M2 result. Do not run casually. |
| The native M2 gate | `/bin/bash -p walt/ci/check_m2_metal.sh` | needs Xcode 26.6 with the Metal 32023.883 component, a real Apple GPU and elevated access; runs the 614-task carrier twice; last run 2026-08-17; post-fold status untraced |
| Rebuild the metallib and compare | `/bin/bash -p walt/walt-metal/shaders/build_metallib.sh [verify\|--replace]` | two fresh builds, `cmp`, then `cmp` with the committed library |
| The runner directly | `walt-m2-runner <mode> <repository root> …` with modes `descriptor-render`, `descriptor-verify`, `child-smoke`, `child-official`, `run-smoke`, `run-official`, `validate-smoke`, `validate-receipt`, `validate-failure`, `adjudicate-receipts` | built `--locked` into a fresh target dir by `check_m2_metal.sh`; no binary in the shared target dir today |
| Lean targets | `cd lean && lake build Texas42.Trick1Foundation Texas42.Trick1MetalFoundation` then `lake env lean Texas42/Trick1MetalFoundation.lean \| diff -u trick1_metal_foundation_axioms_v1.txt -` | warm mathlib cache required; the M3 tree has no gate |
| The exploratory M3 solve | `cd walt && cargo run --release -p walt --bin m3probe` (prebuilt at `walt/target/release/m3probe`) | seconds; writes nothing; prints the §5.3 fractions |
| The exploratory ladder | `cd walt && cargo run --release -p walt --bin ladder [t] [budget_secs] [sample]` (prebuilt at `walt/target/release/ladder`) | t = 4 in ~4 s; t = 3 needs a budget you are prepared to lose |

What `check.sh` scans for floats: MSL rejects `half`/`float`/`double`/`bfloat`
families and decimal/exponent/inf/nan tokens (`check_msl_no_float.awk`); Rust
rejects `f32`/`f64` and inferred float literals after stripping comments and
strings (`check_rust_no_float.py`); TOML/JSON manifests likewise
(`check_toml_no_float.awk`). `render_m2_failure.py` writes the typed 256-byte
failure receipt when a gate fails. These gates outlived the program: they are
the same discipline the rest of the crate runs under.

---

## 8. What remains

**Lean debt, named in v0.3 §10 and GT1-A8/A23 and discharged by nothing
since.** The semantic `(response,e)` partition; the `A(e,x)`, `C(e,x)`, `W`
formulas and the global conservation identity; posterior stratification and
prior-aware seat-potential factorization; information-key equivalence and the
deterministic-descriptor corollary; the canonical least-index verdict;
sparse-DP fold and meet-in-the-middle complement join; Rust/Lean
correspondence; Metal/Rust correspondence; grade-4-to-trick-1 transport (a
standing rule: no grade-4 quantity is evidence about trick 1). Executable
parity is evidence for the last three, never a theorem that erases them. Plus
the M3 tree's own build and audit, and its stale 94-of-103 receipt (regenerating
it is a freeze event under FZ-A5).

**Three open kanban cards** (all in `kanban/backlog/` as of 2026-09-07):

| Card | What | Done when |
|---|---|---|
| [[gpu-level2]] (opened 2026-08-24) | The parked GPU program. Level 2 — a best response to a named σ1, never an equilibrium ([how walt plays](walt-seat-play.md) §4) — makes every field decision inside every rollout a fiber-sampling solve; the exact workload factors into GPU-shaped batches plus CPU search; the M0–M3 lineage is the substrate. **PARKED: not actionable until Jason's word.** | Unparked; then scoped against the level-2 field-swap results (only wake-up positions need level-2 evaluation) |
| [[m2-receipt-reearn]] (2026-08-24) | Re-earn the M2 receipt under the v2 identity: the 614-task carrier, twice, on hardware, via `walt-m2-runner` | A fresh receipt attesting `8a780895…`, filed beside (never replacing) v1, recorded by append in `GPU-NATIVE-TRICK1-M2.md` |
| [[m2-runner-trace]] (2026-08-24) | `walt-m2-runner`'s LIVE status is inferred, not confirmed; nobody has traced whether `check_m2_metal.sh` still runs green after the fold (its final `verify_m2_sources.sh` step now depends on a freeze-event-fresh v2 manifest) | Its gate status documented where the GPU branch lives, or the crate archived with a ledger entry. The survey of 2026-09-07 judged the documentation half met by [instruments](walt-instruments.md) and [walt](walt.md); the trace itself has not been run |

**The escape-hatch framing.** The repository's own words are the
[[gpu-level2]] card's: "PARKED: not actionable until Jason's word, per the
standing GPU escape-hatch protocol." The protocol's content is a session
ruling (2026-08-30; recorded in the project memory, not in a repository
document): the GPU is not forbidden, it is a ripcord — the recurring decision
is "make what we have faster versus bail to the GPU", and bailing prematurely
is the failure mode, not the GPU. The evidence that it is time would be a CPU
representation fight that is genuinely lost, not merely slow. The
counted-belief factorized backends (subset convolution; [counted-belief
era](walt-counted-belief-era.md)) are the most GPU-shaped mathematics the
project has produced — a fact to carry, not a proposal to push. `walt/MAP.md`
lists this track under "Side tracks, not in the critical path": the live
player and the counted-belief, anytime and focal-horizon programs consume
nothing from it.

**Living descendants.** `walt::spec` (the U256 integer, role/frame types,
canonical tables, SHA-256 anchors; 12 tests in `walt/walt/tests/spec_m0.rs`) —
"LIVE — load-bearing for GPU branch AND player spine" per the unification
census (`walt/UNIFICATION-CENSUS.md` crate table); `walt::carrier` (the h8 carrier; 7 tests) — the fixed carrier every
2026-08-17 seat-play probe and the ladder stand on; the no-float CI gates
(`-D float_arithmetic`, the MSL/Rust/TOML scanners) and the scrubbed-environment
verifier pattern, now the whole crate's gate; the two gated Lean modules
(33 kernel theorems about trick-1 arithmetic) and the M3 statements; and the
projector-cell quotient itself — 11,730 cells over 399,072,960 deals — which
§1's ladder names as load-bearing for any exact solve earlier than trick 4
and which no program has yet pushed past the opening response.

---

## 9. The record

| Date (2026) | Commit | Event |
|---|---|---|
| 08-16 21:12 | `c230949c` | Intake of the Pro implementer's guide v0.2, verbatim (`ee2e78da…`). |
| 08-16 23:24 | `3b4c6d60` | Portable M0/M1 closed: v0.3 contract, GT1-A1..A9, **freeze 55**, `walt-gpu-spec`/`walt-gpu-ref`, `Trick1Foundation.lean`, M0/M1 receipts, Gate-0 NO-GO receipt. **PORTABLE M0/M1 COMPLETE under freeze 55.** |
| 08-17 04:06 | `813d5e81` | Freeze-56 M2 gate implemented: `walt-metal`, `walt-m2-runner`, the two MSL kernels, `Trick1MetalFoundation.lean`, `check_m2_metal.sh`; GT1-A10..A17 after full Xcode 26.6 was installed. |
| 08-17 04:32 | `a6df853c` | "Harden M2 timeout provenance" — the immutable commit at which **M2 METAL PROJECTOR PARITY COMPLETE under freeze 56** was earned. |
| 08-17 04:54 | `20a9fecc` | Freeze-56 closure recorded; committed receipt `0aa2f6ab…`; the M2 contract blob `aacb6df5…`. |
| 08-17 08:36 | `e6cd9586` | M3 contract frozen: GT1-A18..A24, **freeze 57**, GT1 range re-frozen A1..A24 and closed. |
| 08-17 09:45 | `97ce321a` | WIP M3 scaffolding committed "mid-flight, does not build" (`walt-m3-net`, `-oracle-a`, `-metal`, `-carrier`; `02_m3_wavefront.metal` and `build_m3_metallib.sh`; the `Trick1PerfectRecallNet` tree; the body: "four walt-m3-* crates not yet wired into the workspace (cargo refuses them as-is) … Committed only so the work survives the worktree"). |
| 08-17 10:39 | `1aa409c8`, then `171cd22c` | The exploratory `walt-m3-probe` crate and `results_2026-08-17.txt` ("first lawful play — exact H-treatment solve of the frozen M3 carrier"), then the ladder and `ladder_results_2026-08-17.txt`; both records moved to `walt/probes/m3/` at THE FOLD. Same day: pmake ruled the objective; the seat player begins ([walt-program](walt-program.md)). |
| 08-24 01:47 | `ad355e93` | Unification stage 1: `walt-m3-net`, `walt-m3-oracle-a`, `walt-m3-metal` deleted as unbuildable orphans. |
| 08-24 03:31 | `d1499d43`, `c92175ae` | THE FOLD (`gpu-spec` → `walt::spec`, `m3-carrier` → `walt::carrier`; GPU trio kept separate); **freeze-56 v2** re-issued append-only (FZ-A1..A6); full `check.sh` PASS. |
| 08-24 | `e463665e` | Wiki stale-path sweep; the previous edit of this page. |
| 09-07 | `c00717d1` | Survey snapshot: no GPU-track file changed since 2026-08-24; every frozen hash re-verified; M0/M1 replay byte-identical. |
| 09-13 | — | This rewrite; hashes, receipt header and the M0/M1 replay re-verified on this machine. |
| 09-16 | — | Independent fact-check of this page: every hash, byte count, manifest entry count, descriptor length, receipt header word, gate line citation, commit date, kanban card and probe fraction on the page re-verified against its source; the M0/M1 replay and `m3probe` re-run; the `97ce321a` row corrected (the probe crate and records were `1aa409c8`/`171cd22c`, not the WIP commit). |
