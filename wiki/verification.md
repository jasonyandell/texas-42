# Verification Scripts and Fresh-Run Results

[Home](Home.md) · owns: every verifier and receipt — ingest Python, rob Rust
(slices 01+02 and the player track, all twelve receipts), exchange program runs ·
Sources: `ingest/*/verification/`, `rob/receipts/`, `rob/ci/check.sh`,
`exchange/adjudication/`, [exchange/README.md](../exchange/README.md). Fresh ingest
runs 2026-07-26; re-timed 2026-09-13 from a copy (never in place). Related:
[rob](rob.md), [claim-ledger](claim-ledger.md), [lean](lean.md).

Three reminders before any number below is used. A `PASS` line is a **finite
verification receipt**: it supports the finite statement the program exhausts and
nothing wider (Math §0). Receipts are never promoted — an ingest verifier, a rob
receipt and an exchange program are three different tiers of evidence and this page
labels each. And the `ingest/` tree is immutable: to re-run anything, copy it out
first (caveat 1).

## Status: everything passes

| Script | Package(s) | Result | Matches committed output? |
|---|---|---|---|
| `verify_foundation.py` (1,850 ln, byte-identical in both) | both | **PASS** | yes — exact |
| `verify_minimality_and_reachability.py` | both (differ only in docstring + "profiles"/"certificates" wording) | **PASS** | yes — exact |
| `verify_reduced_kernel.py` (871 ln) | rec only | **PASS** | yes — exact |
| `audit_package.py` | rec only | **PASS on a clean tree**; FAILS after a verifier has run in the same tree (caveat 1) | yes — exact, on a clean copy |

"Exact" means the committed `VERIFICATION_OUTPUT.txt` is the concatenation of the
actual stdout plus `=== script name ===` header lines — no numeric or textual drift.

### Measured walls (2026-09-13, this machine)

Run from a copy of each package under the job's scratch directory, Python 3.12.13,
Apple M5 Max; each command wrapped in a 60 s timeout. The full byte-diff against the
committed `VERIFICATION_OUTPUT.txt` was repeated in this pass for all five runs: the
fresh stdout is identical to the committed transcript once the `=== script ===`
header lines and the blank separator after each section are removed (rec: 35 + 16 +
10 lines; v0.7: 35 + 16), and the clean-copy `audit_package.py` stdout is identical
to `AUDIT_OUTPUT.txt`.

| Script | Copy | Wall | Exit | Note |
|---|---|---|---|---|
| `audit_package.py` | rec, clean copy, **before** any verifier | 0.03 s | 0 | 14 files / 14,390 lines; 255 claim IDs; 17 markers; byte-identical to `AUDIT_OUTPUT.txt` |
| `verify_foundation.py` | rec | 4.54 s | 0 | 35 stdout lines; 90-world witness lines as committed |
| `verify_minimality_and_reachability.py` | rec | 3.28 s | 0 | 16 lines; "proved standalone reachable-support interval: 26..46 bits" |
| `verify_reduced_kernel.py` | rec | 8.08 s | 0 | 10 lines; 5,898 machines / 17,560 pairs |
| `audit_package.py` | rec, same copy, **after** the three verifiers | 0.03 s | **1** | `AssertionError: transient Python files present: ['verification/__pycache__', …]` — the D15 trap, reproduced on purpose |
| `verify_foundation.py` | v0.7 (`python3 -B`) | 4.59 s | 0 | identical to rec's output; no `__pycache__` written |
| `verify_minimality_and_reachability.py` | v0.7 (`python3 -B`) | 3.22 s | 0 | identical to rec's output |

Total for the four rec scripts: about 16 s. Earlier passes on the same machine
measured 4.8 / 3.3 / 8.2 / 0.03 s (2026-09-07) and 4.84 / 3.41 / 8.52 / 0.02 s
(2026-09-12); load noise accounts for the differences.

## What each script exhausts

### `verify_foundation.py` (both packages)
28 dominoes; 9 declarations; ordered-deal count 472,518,347,558,400; 737,100
unique-winner cases agreeing with an independent prose-rule resolver; auction census
for caps 1..7 (2380…3214) with caps 5/6/7 identical; 66,968 tiny Hall systems;
fiber-count coefficient/recurrence agreement; 22,620 exact uniform sampler
probabilities; 785,736 marginal-edge checks (world projection vs Hall); 512 native
capacity-DP profiles with the 512/1,533/1,344/48 bounds attained; 14,412+56,460+56,460
typed transitions on tiny systems; 972 reachable cell/deal parity prefixes; 864 typed
support transitions (648 hidden nonincrease, 216 viewer equality); count/trick/pip
transport facts; and the full 90-world history witness (fiber size, anchor Q-tables,
both posteriors, expected Q, make probabilities, opposite best leads).

### `verify_minimality_and_reachability.py` (both)
Normal-form decode equality and SCC compilation on the tiny corpus (14,578 feasible;
2,151 essential exclusions; rank/unrank on 22,620 worlds); native ternary signature
census (136,514 / 1,667,666 matrices / max 114; S₃: 23,842 orbits, 296,721 matrices,
279,048 stabilizer orbits, max 103); the full-schema census
(det 8,102,258,940,222,814; bin 11,495,078,055,913,018,482; ter
1,830,955,704,129,296,418,354,864; total 1,830,967,207,309,611,271,596,161 ⇒ 81 bits);
50 reachable capacity profiles; 7 lead contexts with fiber sizes 1..7; projected
schedule counts A_j/T_{j,1}/T_{j,2}; the outer-profile counts
(7,124,838,074,989/declaration; 64,123,542,674,901 total ⇒ ≤46 bits; context ceilings
43/43/40); the 44,352,165 no-void floor ⇒ ≥26 bits; the feasible-but-unreachable
witness (450 generators, 2 matches, all lead tiles hidden); interval 26..46;
0 supplemental bits given a certified mechanical state.

### `verify_reduced_kernel.py` (rec only)
Looped-K7/count-antidiagonal identity; 49 ordered unscored pip-trump transports with
307,328 contextual order comparisons and 3 mechanics classes; folded trick over all
737,100 trick cases + 2,211,300 sequential updates (max competitive chain 13; pending
counts 6); 84 legal open-trick actor/capacity shapes; 3,132 score-recovery prefixes;
support-normal dynamics on 1,331 feasible supports (170,058 typed observations,
157,809 nonempty successors, 1,406,592 holder-edge monotonicity checks); 108 symbolic
complete hands / 3,024 transitions / 6,804 = 108·63 edge deletions; 8 oriented
dihedral frames; future-equivalence minimization on 5,898 machines / 17,560 state
pairs.

### `audit_package.py` (rec only)
Structural hygiene: required files present; UTF-8/no control chars (14 files, 14,390
lines); local Markdown link integrity (14 links); 255 unique claim IDs; 8
project-neutral docs free of forbidden implementation names (Atlas, Walt, Hoyt, Forge,
mk5-main, CoordinateV1); 17 cross-document kernel markers; **no transient Python
files**.

## rob (Rust) — independent reproduction, slice 01

Recorded 2026-07-27. **rob** is this repository's exact Texas 42 engine
([rob/BRIEF.md](../rob/BRIEF.md) — the binding slice-01 assignment: rec's mathematics
under v0.7's type discipline, extended through the support normal form + capacity
DP). `rob/ci/check.sh` is green end-to-end (fmt; clippy `-D warnings
-D float_arithmetic`; no-float grep; vocabulary grep; full release test suite); the
four stage receipts under `rob/receipts/` are byte-diffed in CI against fresh runs.

**Status label: Finite verification receipt / conformance evidence — not a new
mathematical status.** Per the corpus's own claim discipline (TRUST-01 and §0 of
either Math package), these runs re-derive and confirm the committed ingest receipt
numbers in an independent implementation. They strengthen confidence in the finite
claims and in rob's conformance; they change no claim's status and are not
proof-assistant kernel proofs.

### What was reproduced

| Receipt | Stage | Headline exact integers |
|---|---|---|
| `verify_algebra` | S1 declaration algebra | 737,100 unique-winner and 737,100 prose-agreement cases; counts 35/7/42; 56,448 `BEATS` equivalences; 5,040 → 2 count-preserving permutations (`2↔3` a game-order isomorphism only between layers 2 and 3); 49 unscored transports / 307,328 order comparisons; 3 mechanics classes; competitive-ordinal max 13 |
| `verify_objective` | S2 objective machine | 472,518,347,558,400 ordered deals; 399,072,960 hidden assignments; auction census (2380, 3060, 3196, 3213, 3214, 3214, 3214) with maxima (1, 2, 3, 4, 5, 5, 5) and caps 5/6/7 identical; conservation 42 with `P_D = 42` ⇔ seven-trick sweep |
| `verify_support` | S3 cells + losslessness | initial cells 21/7/63; typed-update algebra 14,412 leads + 56,460 follows + 56,460 sloughs; 972 parity prefixes with exact fiber ↔ replayed-deal set equality; transitions 864 (648 hidden nonincrease, 216 viewer equality); 90-world support witness via both auction histories |
| `verify_normal_form` | S4 NF + capacity DP | tiny corpus 66,968 / 14,578 feasible / 22,620 worlds; three counting routes ≡ enumeration; DP bounds 512/1,533/1,344/48 exactly attained, max count 399,072,960; 785,736 marginal edges; 22,620 SCC compilations; 2,151 essential exclusions; 22,620 rank/unrank; ternary census 136,514 signatures / 1,667,666 matrices / max 114 with S₃ quotient 23,842 / 296,721 / 279,048 / max 103; 81-bit census (total 1,830,967,207,309,611,271,596,161, computed from the §7.12.5 formulas); 50 capacity profiles, each realized by an explicit legal witness prefix; 44,352,165 no-void floor (≥ 26 bits); 0 supplemental support bits |

### Independence conditions

- No ingest Python was read or translated. Only the committed
  `VERIFICATION_OUTPUT.txt` numbers and corpus-shape parameters were consumed, as
  rob/BRIEF §5's guardrails permit; the reproduction is Rust (pinned 1.95.0,
  runtime dependencies `num-*` only), so "never copy the verifiers" is structurally
  true.
- The independent prose-rule resolver lives in `rob-verify` and shares nothing with
  the algebra implementation beyond domino identity, the declaration enum, and seat
  labels (BRIEF §1 D4); it agrees on winner **and** trick points in all 737,100
  cases.
- rob's S3 corpus generator is its own deterministic construction; only the corpus
  *shape* (972 = 9·12·9) is an ingest-anchored assertion.

### Enforcement mechanisms

Each of INV-1..INV-10 (rob/BRIEF §5) has a named test, lint, or CI check:
`inv_derived_coherence` (derived-not-stored), `inv_projected_equality`,
`inv_no_reachability_field` (proof-irrelevant reachability, D1),
no-float grep + clippy `float_arithmetic` deny + overflow checks in every profile
(exact arithmetic), committed receipts byte-diffed in CI (counts-are-CI),
`inv_reachable_implies_feasible` (including a `should_panic` certified path),
`inv_id_not_rank` plus a `compile_fail` doctest that `DominoId` has no `Ord`,
`inv_event_replay` (one source of truth), a `compile_fail` doctest that `DealWorld`
does not convert to `RemainderWorld` (type-distinct domains), and a vocabulary grep
forbidding certificate-style identifiers (D3: "necessary outer profile", never
"certificate").

### Frozen generator-specific value

The with-voids parity count is **970** (`FROZEN_WITH_VOIDS` in
`rob/crates/verify/src/s3.rs`). This is a property of rob's own deterministic
generator, frozen at first green run per BRIEF §8. It coincides numerically with the
ingest generator's 970 but is **explicitly not an ingest-corpus number** and carries
no cross-implementation meaning; only the 972 corpus shape does.

Slice 02 (support dynamics: TRANS-08..14, REACH-06..16) is scoped in rob/BRIEF §4
and is recorded below.

## rob (Rust) — independent reproduction, slice 02

Recorded 2026-07-27. Slice 02 extends the slice-01 reproduction into **the dynamic
support layer and symbolic reachability**
([rob/BRIEF_SLICE_02.md](../rob/BRIEF_SLICE_02.md) — the binding second assignment;
BRIEF.md §§1,2,5,9,11 remain binding verbatim). The five new stage binaries
(`verify_dynamics`, `verify_symbolic`, `verify_outer`, `verify_unreachable`,
`verify_transport`) are green and byte-diffed in `rob/ci/check.sh` against the fresh
receipts under `rob/receipts/`; the full slice-01 CI (fmt; clippy `-D warnings
-D float_arithmetic`; no-float grep; vocabulary grep; release test suite) stays green.

**Status label: same as slice 01** — finite verification receipt / conformance
evidence, never a new mathematical status. Slice 02 adds a second, explicitly
labeled evidentiary layer: receipt lines whose expected value originates from an
exchange-adjudicated result (§"Exchange-adjudicated program runs" below;
[exchange/README.md](../exchange/README.md) ledger) carry an `x-` prefix and their
binary's header cites the ledger entry. **Where a green `x-` line reproduces an
exchange number in Rust, rob's receipt is independent cross-language evidence for that
number — it still ranks below ingest and is never definitional** (BRIEF_SLICE_02 §1.1).

### What was reproduced — corpus-anchored (slice-01 tier)

| Receipt | Stage | Headline exact integers |
|---|---|---|
| `verify_dynamics` | S5 matching-minor calculus | dynamics corpus **66,969** systems / **14,579** feasible / **1,331** distinct feasible NFs (the S4 corpus extended by n = 0, quotiented by NF equality); **170,058** typed observations with matching-minor update ≡ NF of extensional conditioning + pushforward (agreeing on `Empty` both routes), **157,809** nonempty successors; **1,406,592** holder-edge inclusion checks + 157,809 rank checks (monotonicity, never reactivating); **864** game-typed transitions (**648** hidden, **216** viewer) along the S3 parity corpus; **972** native-sampler agreements through the offset↔`DominoId` bijection |
| `verify_symbolic` | S6 symbolic trace validator | **108**-hand deterministic corpus (9 declarations × 12 hands), **3,024** transitions with symbolic support / S5 dynamics / `derive_rule_cells` NF agreeing **3,024** three ways; deletion budget **6,804** = 108·63 with every hand's ledger totalling 63, no edge reappearing, ≤2 edges lost per live tile (INV-11); **324** = 3·108 mutated symbolic traces each rejected with the expected typed reason |
| `verify_outer` | S7 necessary outer language | projected schedule censuses `A_j` = (1, 50, 1079, 13084, 97119, 450066, 1273609, 2097152), `T_{j,1}` = (8, 323, 5524, 51759, 286770, 947017, 1817216, 2097152), `T_{j,2}` = (22, 743, 10844, 88159, 428562, 1244937, 2080768, 2097152); `B_{n,u}` lead-witness table **176** = 22·8 entries agreeing by two independent routes (inclusion–exclusion vs polynomial convolution); **7,124,838,074,989** per declaration (< 2⁴³), **64,123,542,674,901** total (< 2⁴⁶), **839,220,930,919** max single-profile block (< 2⁴⁰; see the provenance note below), ceilings **46 / 43 / 43 / 40 bits** (standalone / declaration-supplied / capacities-supplied / both); interval line held at **26..46 bits** (rob prints only what rob computed); all big values computed from the Math §7.13.3/§7.13.6 formulas in exact `BigUint`, never hard-coded |
| `verify_unreachable` | S8 REACH-10 regression | **450** static generators at capacities (6,6,6), exactly **2** decode to the REACH-10 witness (zeros-trump called context, NT context 0; each only hidden seat 1 void), lead-fiber sizes **(7, 1)**, both with the entire lead fiber inside the hidden pool ⇒ lead-witness necessity fails |
| `verify_transport` | S9 transport quotient | `reachable_census_class` constant on the 7 pip trumps yielding exactly **3** classes, agreeing with S1's `unscored_mechanics_class` partition |

**Provenance of 839,220,930,919.** The max single-profile block is *not* a line of
either package's `VERIFICATION_OUTPUT.txt` (grep of `ingest/*/verification/*.txt`,
2026-09-13: no hit). Its source is the Math text — v0.7
`20_MATHEMATICAL_FOUNDATION.md` §7.13.6, the displayed
`max_k C(k) = 839,220,930,919 < 2^40` (line 3543; rec line 3681) — and the REACH-11A
row that cites that section. Two independent computations reproduce it: rob's
`r_out_profiles` line (from the §7.13.6 formulas) and exchange 005. Cite it as a
Math-§7.13.6 number reproduced by rob and by x:005, never as a verifier-output
number.

### What was reproduced — exchange-adjudicated tier (`x-` receipt lines)

Each line below carries the `x-` prefix in the committed receipt and its binary's
header names the ledger entry. These are the exchange numbers rob now backs with an
independent Rust receipt:

- **x:002 — the four-check outer language is not tight** (`verify_unreachable`,
  header `# exchange: 002`). The (NT, capacities (6,6,6), V₁ = {6}, 18-tile pool)
  witness passes **all four** classic outer checks through rob's *own* S7 validators
  (capacity shape, schedule admissibility, lead witness, Hall) and is Hall-feasible /
  already reduced (`x-r_unr_002_outer: 4/4`); exactly **3** of the 450 generators
  decode to it, one (doubles-trump) already killed by lead-witness necessity
  (`x-r_unr_002_static: 3; 1 kill`); complete shallow-prefix exhaustion over the
  surviving matches runs **425,520** candidates with **0** realizers
  (`x-r_unr_002_traces`) — the witness is unreachable under every declaration; and the
  fifth **follower-supply** check rejects it (`x-r_unr_002_supply: 1 < 2` for both
  surviving candidates, since `σ₆ ∖ U = {6:6}` supplies one follower where two are
  needed). The 425,520 shallow traces run through rob's own symbolic validator and NF
  pipeline; the witness pool/NF was transcribed from the inbox/002 JSON as data.
- **x:004 — transport commutes with reachability** (`verify_transport`, header
  `# exchange: 004`). Over the 12 S3 hands of each source pip trump under all 49
  ordered transports: **588** = 49·12 transported hands legal on the S2 machine,
  **16,464** = 588·28 transitions each accepted by the S6 symbolic validator, and
  **17,052** = 588·29 depth-wise NF equalities `f(N_t(prefix)) = N_u(f(prefix))`
  (`x-r_tra_corpus_commutation`). Conformance evidence for `f_{t,u}(R_t) = R_u`, not a
  proof of it; the 3-class quotient (corpus-anchored line above) cites the ledger entry
  in its doc comment. Step-17 boundary enforced by a `compile_fail` doctest: nothing
  score-conditioned transports.
- **x:005 — Burnside supplement** (`verify_outer`, header `# exchange: 002, 005`).
  The S₃ fixed-signature counts **136,514** (identity) / **2,156** (each transposition)
  / **35** (each 3-cycle) with Burnside average (136,514 + 3·2,156 + 2·35)/6 =
  **23,842**, reproducing the S4 orbit census (`x-r_out_burnside`).

### Independence conditions

- No ingest Python and no exchange Python (`exchange/adjudication/programs/`, inbox
  code blocks) was read or translated as implementation source. rob re-implements each
  construction from its prose proof and from JSON witness/tables; the BRIEF_SLICE_02 §5
  clarification permits transcribing *witness data, corpus-shape parameters, and
  expected numbers* exactly as for ingest receipts, and forbids transcribing program
  logic. The x:002 witness pool/NF and any table values are consumed as data only.
- The dynamics corpus, symbolic 108-hand corpus, and transport corpus are rob's own
  deterministic constructions; only the corpus *shapes* (66,969/14,579/1,331 derived
  from the S4 corpus; 108 = 9·12; 588 = 49·12) are ingest- or arithmetic-anchored.

### Enforcement mechanisms

INV-11 through INV-14 (rob/BRIEF_SLICE_02 §5) each have a named test, lint, or
compile-time check:

- **INV-11 EDGE-BUDGET** — `inv_edge_budget` over the S6 corpus asserts 6,804 = 108·63
  with zero edge reappearances, plus a per-successor `debug_assert!` that every holder
  set is a subset of its predecessor.
- **INV-12 MONOTONE-AMBIGUITY** — `r_dyn_monotone` (157,809 cases) plus the S6 corpus
  assert tags move only `Ternary → Binary → Determinate` and inactive seats never
  reactivate; `debug_assert!` on every nonempty successor.
- **INV-13 TYPED-TRANSITION-ONLY** — the type system forbids an untyped transition on
  standalone support; a `compile_fail` doctest shows `nf.transition(domino)` does not
  exist; the abstract calculus consumes only a `TypedHiddenObservation`.
- **INV-14 FIVE-CHECKS-STILL-NECESSARY-ONLY** — the outer validator returns profile
  membership only, no path to a certified/reachable type compiles, and the x:002
  witness is the permanent regression (`r_unr_002_*` green forever) that passing every
  implemented outer check does not imply reachability; the INV-10 vocabulary grep still
  forbids certificate-style identifiers.

### Notable: no escape hatch used

The TRANS-13 counts 170,058 / 157,809 / 1,406,592 — which the ingest claim ledger
itself labels "stated by verifier" — were reproduced from rob's principled enumeration
of the closed-form observation space **without** invoking BRIEF_SLICE_02 §11.1's
frozen-count escape hatch. They matched exactly, so no
`ambiguity_trans13_observation_space` note was filed.

### S10 stretch — completed (recorded 2026-07-27)

S10 is **green and committed** (`verify_floor`, byte-diffed receipt;
header `# exchange: 001`). rob re-implemented the x:001 floor construction from the
inbox/001 prose proof (steps 1–14) and JSON tables — never its Python: the
eight-star admissible-module language (**119** four-groups per star; **3,808**
declaration/group/desired-winner cases exhausted through `resolve_trick`), the
`pair`/`m1`/`m2` fragment templates with the step-5 `|X| ≤ k_u` ranges, and an
exact 28-dimension subset-zeta upward-closure counter. Every anchor reproduced
exactly: all six no-void witness-language/coverage numbers, all 17 called-suit
per-category counts (K₈ symmetry verified on two stars), and all 54 natural-suit
per-fiber-size counts (omitted-edge exclusion; 7 ordered contexts per fiber size).
Family totals `x-r_flo_families`: **559,316,142** no-void / **8,387,350,664**
called-suit void / **8,721,399,239** natural-suit void; grand total
`x-r_flo_total`: **17,668,066,045 > 2³⁴** ⇒ floor **35 bits** — rob's receipt is
independent Rust evidence for the exchange-adjudicated REACH-17 family; the
corpus-proved interval statement (26..46, REACH-11/12) is unchanged and any
evidentiary reframing belongs to the wiki. One rob-frozen value: the step-5
one-context verification exhausts a principled **369-profile superset** of x:001's
216 tabled profiles, all satisfying the step-5 marginal descriptions (noted in the
receipt and named test).

Two dated notes on that receipt. (i) The wiki-level interval has since tightened to
**[36,45]**: floor via the disjoint REACH-18 family, exchange 006 (dispatched
2026-07-27T14:53:19Z, harvested 17:40:53Z, `programs/006.py`, 16/16 PASS 17.3 s);
ceiling 45 via the REACH-19 filtered census, exchange 007 (dispatched
2026-07-27T18:15:07Z, harvested 19:01:13Z, `programs/007.py`, 17/17 PASS 44.1 s) —
both later than S10's recording on the same day. rob has reproduced neither, nor
REACH-20 (exchange 008); they are the named slice-03 targets
([rob-slices](rob-slices.md)), and slice 03 has not begun. (ii) rob's
`x-r_flo_families` no-void figure **559,316,142** is x:001's grammar-subfamily count,
which x:008 later showed to be a proper subfamily of the exact no-void slice
624,892,870 (undercount 65,576,728; [discrepancies D17](discrepancies.md)). rob's
receipt intentionally still reproduces x:001's construction; do not diff it against
x:008.

### The eleventh receipt: the baseline (evening player v0, demoted 2026-07-28)

`rob/receipts/verify_player.txt` freezes a complete self-play match transcript of the
player that `rob/crates/player` shipped on 2026-07-27 under the name **evening player
v0**: fixed-field Monte Carlo best response (Math §11.4) — rollout policy fixed before
worlds are drawn, world identity unrepresentable in its view — over exact uniform
fiber sampling (rejection, no modulo bias), exact integer/rational values, 12 worlds
per decision, player seed 7, match seed 42: 13 hands, T0 7–6 T1. **Naming law**
(`rob/BRIEF_PLAYER_01.md` §1, 2026-07-28): the player specified by that brief *is*
rob; the slice-01 player continues as the **baseline / receipt mode** — it keeps its
receipts (`verify_player.txt`), its docs say "baseline", and its receipt text does not
change. Every later rob number "against the baseline" (the +718 below) names this
player, not σ. Like `FROZEN_WITH_VOIDS` above, the transcript is a **determinism
freeze of rob's own construction**, not an ingest-corpus number. The HTML inspector
(`rob/inspector/`) renders the same trace per-seat with exact fiber counts, marginals,
decision values, trump display, and shareable URL-hash state; the JS recomputes no game
logic — everything is emitted from Rust. Two named invariant tests guard it:
`inv_rollout_conservation` and `inv_sampled_world_voids`
(`rob/crates/player/tests/inv_player.rs`).

### The twelfth receipt: the player track (`verify_rob`)

**Count correction, 2026-08-13.** This page and three others said "eleven
receipts". `rob/receipts/` holds **twelve** files, and `rob/ci/check.sh` byte-diffs
all of them (`for expected in receipts/verify_*.txt`), so the twelfth is gated
exactly like the rest. The uncounted one is `rob/receipts/verify_rob.txt`.

It is a single file carrying all five player-track stages, each with its own PASS
line: P1 self-play and determinism (108 hands, 3,024 plays, 42×108 conserved; 108
traces byte-equal); P2 the position corpus and its fiber bounds (756 positions;
the 399,072,960 / 17,153,136 / 756,756 / 34,650 / 1,680 / 90 / 6 ladder;
44,722,908,161 census); P3 the solver agreements (known-world, brute-force,
undominated, conservation over 58,609,267 nodes, double-solve byte-equality,
cross-engine); P4 the match rig — including `r_mat_paired`, **200 hands, net
+718**, the number several pages already quote without naming its receipt — and
the window ablation; P5 plan-book round-trip and trace embedding.

Tier, unchanged: this is a **rob conformance receipt**, evidence and never a status
change, and its self-play and match figures are determinism freezes of rob's own
construction rather than ingest-corpus numbers — the same standing as
`FROZEN_WITH_VOIDS` and the baseline transcript above. One tier inversion is
worth recording: before this correction, `verify_rob.txt`'s only mention anywhere in
the wiki was in [idea-hierarchical-fibers](idea-hierarchical-fibers.md), an
**ideas**-tier page — a tier-4 receipt documented only below every tier. It is
inventoried here now, which is where it belongs.

### Enforcement of the player invariants (INV-P1..P7)

`rob/BRIEF_PLAYER_01.md` §5 names seven player invariants. Their enforcement, as it
actually stands at c00717d1 (grep 2026-09-13), is uneven and should be read
precisely:

- **INV-P1 PLAN-NOT-TILE** and **INV-P7 FRONTIER-LEAF-IS-LAW** are enforced by a CI
  grep: `rob/ci/check.sh` step "vocabulary grep (INV-P1, INV-P7)" fails on any
  forbidden identifier ("no per-domino scalar value API; no tunable leaf"). INV-P1 is
  additionally cited in `p5.rs` (the plan-book round-trip, `r_book_roundtrip`).
- **INV-P2..P6 have no test named for them.** They are asserted *inside* the P-stage
  receipt code and surface as receipt lines: INV-P4 FIELD-FIXED in `p1.rs`
  (`r_sig_deterministic`, 108 traces byte-equal); INV-P6 WINDOW-EXACTNESS in `p2.rs`
  (`r_pos_schedule`) and `p4.rs` (`r_mat_rolling`, 2,800 window agreements); INV-P5
  BUNDLE-CONSERVATION in `p3.rs` (`r_sol_conservation`, 58,609,267 nodes); INV-P3
  EXACT-NO-SAMPLING in `p3.rs` (`r_sol_deterministic`, 756 double-solves byte-equal).
  **INV-P2 ONE-INFOSET-ONE-ACTION has neither a named test nor an in-source citation
  in `p1.rs`–`p5.rs`.**
- The brief itself names three tests — `inv_p2_partition` (line 157),
  `inv_p4_determinism` (line 170), `inv_p5_conservation` (line 178) — **that do not
  exist under those names anywhere in `rob/crates`**. The obligations they describe
  are covered by the receipt lines above; the names are a documentation debt
  ([rob](rob.md) "Known documentation drift"). No `ambiguity_*` test exists anywhere
  in rob: the ambiguity protocol was never triggered.

## Exchange-adjudicated program runs (external evidentiary tier)

Every numbered exchange response that carried a verification program had that
program extracted **unmodified** to `exchange/adjudication/programs/NNN.py` and
executed here (stdlib-only Python) as the Execute phase of the adjudication workflow
(`exchange/adjudication/workflow.js`: Extract → Execute → Verify by three referee
lenses → Verdict). The first batch ran 2026-07-27 (workflow `wf_775fe0ec`, 30
agents); the second ran 2026-08-01. Each result is **exchange-adjudicated** at the
verdict shown — a tier below corpus theorems and Lean kernel proofs and above rob
receipts, **never** imported as an axiom (TRUST-01). The verdict rule is "program
green on its own claims + no referee demonstrates a real flaw"; the referee tally is
printed for every row and the one non-unanimous panel is marked. Full result
statements and caveats live in [claim-ledger](claim-ledger.md); this table owns the
receipts. Dispatches 011/013/014/015 (the Lean thread and the informal take) had no
program — their deliverable was a `lake build` or nothing.

| # | Program (lines) | Result | Recorded adjudication run | Referee panel | Re-run 2026-09-13 (this machine, from a copy, `python3 -B`, 60 s cap) |
|---|---|---|---|---|---|
| 001 | `001.py` (1,789) | REACH-17 — floor 17,668,066,045, [35,46] | ALL_PASS, 15.9 s | 3/3 SOUND | 15.45 s, exit 0, 13 PASS lines, 0 FAIL; `PASS headline INTERVAL [35,46] bits` |
| 002 | `002.py` (965) | outer language not tight; fifth condition | 16/16 PASS, 0.9 s | 3/3 SOUND | 0.91 s, exit 0, 16 PASS, 0 FAIL; `generators=450 traces=425520` |
| 003 | `003.py` (907) | OPEN-01 COLLAPSE (bisimulation checker) | ALL_PASS 8/8, 0.4 s | 3/3 SOUND | 0.38 s, exit 0, 8 PASS, 0 FAIL; 204 / 22,848 / 1,604 / 1,280 |
| 004 | `004.py` (673) | transport theorem, 9→3 classes | ALL_PASS, 4.6 s | 3/3 SOUND | 4.28 s, exit 0, 6 PASS, 0 FAIL; 45,472 commutation checks |
| 004 | `004-cocycle.py` (146) — **in-house, Claude-authored**, not a Pro deliverable, not referee-panelled | Step-15 cocycle lemma `f_{u,v}∘f_{t,u}=f_{t,v}` over all 343 ordered pip-trump triples (+ identity and inverse legs) | ALL_PASS, 2026-07-27 | — (finite verification receipt, exchange-side) | 0.02 s, exit 0, 4 PASS, 0 FAIL; `ALL_PASS 343 ordered triples` |
| 005 | `005.py` (699) | census-integer audit, 19 integers | 19/19 PASS, ~13 s | 3/3 SOUND | 12.90 s, exit 0, 19 PASS, 0 FAIL |
| 006 | `006.py` (1,053) | REACH-18 — combined floor 36,913,384,410, [36,46] | 16/16 PASS, 17.3 s | 3/3 SOUND | 16.88 s, exit 0, 16 PASS, 0 FAIL; `PASS headline INTERVAL [36,46] bits` |
| 007 | `007.py` (975) | REACH-19 — filtered census 33,297,009,347,414, ceiling 45, [36,45] | 17/17 PASS, 44.1 s | 3/3 SOUND | 36.42 s, exit 0, 17 PASS, 0 FAIL; `FILTERED_TAGGED_OUTER=33297009347414 CEILING=45` |
| 008 | `008.py` (1,217; SHA 38fd84ea…) | REACH-20 — no-void slice exactly 624,892,870 | ALL_PASS 38/38, 71.8 s | **2/3 SOUND + 1 UNVERIFIABLE-no-defect** (dissent carried verbatim in the claim-ledger row) | **not re-run** — the recorded 71.8 s exceeds this pass's 60 s budget; last executed 2026-07-27 |
| 009 | `009.py` (383) | C1 PARTIAL; pooled-key backward commutation REFUTED | ALL_PASS 8/8, 16.3 s | 2/3 SOUND + 1 FLAWED (flaw in corroboration artifacts, not the proof chain) | 13.98 s, exit 0, 8 PASS, 0 FAIL; `fixed_partial_maps=4 full_embeddings=0 legal_embeddings=0` |
| 010 | `010.py` (1,051) | R1 — realizable = reachable at k=1; 31,197 classes | 31,830 PASS / 0 FAIL, ~19 s | 3/3 SOUND | 17.46 s, exit 0, 31,830 PASS, 0 FAIL |
| 012 | `012.py` (651) | carrier-skeleton staircase a₄=37, b₄=486, b₈=126,657 | 14/14 PASS, 18.95 s | 3/3 SOUND | 17.43 s, exit 0, 14 PASS, 0 FAIL; `a4=37 b4=486 b8=126657` |

"PASS lines" counts stdout lines containing `PASS`, which for 001 and 004 includes the
program's own ALL/headline line; the recorded columns are the adjudication's own
counts. The 2026-09-12 pass measured the same PASS/FAIL counts at 16.73 / 0.96 /
0.40 / 4.56 / 0.02 / 13.49 / 17.05 / 37.07 / 14.77 / 18.00 / 17.73 s. Witness JSON
for 001/002/003/006/007/008 and the referee-independent b₈ routes for 012
(`witnesses/012/`: `indep012*.py`, `ref012.c`, `012_out.txt`) are static under
`exchange/adjudication/witnesses/`. The programs were copied out and run with `-B`,
so the worktree stayed clean.

Three of the programs deserve a sentence beyond their row:

- **`005.py` (census-integer audit)** upgraded the status of the 19 load-bearing
  integers from single-source verifier receipts to **independently reproduced** (two
  computation routes per integer). The integers: N_det 8,102,258,940,222,814; N_bin
  11,495,078,055,913,018,482; N_ter 1,830,955,704,129,296,418,354,864; grand total
  1,830,967,207,309,611,271,596,161 (2⁸⁰ < total < 2⁸¹); outer-profile totals
  7,124,838,074,989 and 64,123,542,674,901; max C(k) 839,220,930,919; floor
  44,352,165; and the signature-census chain 136,514 / 23,842 / 1,667,666 / 114 /
  296,721 / 21,686 / 2,121 / 35 / 279,048 / 103. Referee-side foreign methods included
  a max-flow validator over all 343 triples reproducing the 136,514 criterion, an
  exact-rational EGF for N_det, and a brute-forced ternary validity criterion over
  16,712 structural cases.
- **`004.py` (transport commutation)**: all anchors reproduced (307,328 ALG-22
  comparisons; 45,472 commutation checks, 38,976 nontrivial; 6,496 prefixes; 224
  deals); 4/4 injected mutations caught (broken order preservation, unmapped
  exclusions, untransported deal, unmapped void contexts). Mechanical scope is
  **family certification per contract** (single auction shape, 224 pseudo-random
  traces); universality is carried by the prose induction, not the run. The Step-15
  cocycle gap is closed separately by the in-house `004-cocycle.py`, whose transport
  definition is copied verbatim from `004.py` rather than imported so that the
  receipt stands alone.
- **`003.py` (kernel-vs-quotient)** is a reusable synchronized-product **bisimulation
  checker** with diagonal closure plus capacitated-Hall support-fiber conditioning,
  teeth-tested via forged `r/w/z` perturbations — the standard instrument for future
  kernel-vs-quotient claims.

The two walt-side companion verifiers Pro shipped with x:019–023 and x:024
(`exchange/inbox/verify_walt_panel_response_v0_1.py`, 36/36;
`exchange/inbox/verify_deferred_producers_triple_v0_1.py`, 13/13) are **scratch
tier — session evidence, never a receipt** ([claim-ledger](claim-ledger.md) walt-tier
table); they are not in the table above because they verify nothing at the claim
tier.

## Caveats

1. **The `__pycache__` trap** ([discrepancies D15](discrepancies.md)): running any
   verifier creates `verification/__pycache__`, after which `audit_package.py` fails
   its no-transients check — reproduced on purpose 2026-09-13 (table above: exit 1,
   `transient Python files present`). The checked-in `ingest/` tree is **clean**: no
   `__pycache__` is tracked (`git ls-files ingest | grep -c pycache` = 0) or present on
   disk at c00717d1 (`find ingest -name __pycache__` = nothing), so the trap is one a
   reader sets for themself by running in place. Since `ingest/` is immutable anyway,
   the discipline is: **copy the package out, audit first, then run the verifiers**
   (or run with `python3 -B`). An earlier version of this caveat said the ingest copies
   "currently contain" `__pycache__` directories; that described a working tree after a
   run, not the repository.
2. **The two verifier entry points are not independent**: the minimality script
   imports abstract-world helpers from `verify_foundation`. v0.7 states this; rec's
   docstring wrongly claims "dependency-free" ([discrepancies D4](discrepancies.md)).
3. **Receipts are not kernel proofs** [TRUST-01, Boundary]: `PASS` output supports
   finite claims but must be re-proved or reflected inside a proof assistant
   ([proof-assistant-plan](proof-assistant-plan.md), [lean-row-index](lean-row-index.md)).
4. Both MANIFEST.sha256 files verify clean (17 and 14 entries; re-verified 2026-09-13
   from the copies with `shasum -a 256 -c`).
5. **Runtimes are seconds, not minutes** (measured 2026-09-13 on this machine, Apple
   M5 Max, Python 3.12.13, from a copy): `audit_package.py` 0.03 s,
   `verify_foundation.py` 4.54 s, `verify_minimality_and_reachability.py` 3.28 s,
   `verify_reduced_kernel.py` 8.08 s — about 16 s for all four; the v0.7 copies of the
   first two run in 4.59 s and 3.22 s. All scripts are stdlib-only Python. The
   exchange programs are likewise seconds-scale except `008.py` (71.8 s recorded) and
   `007.py` (36–44 s). rob's `rob/ci/check.sh`, by contrast, is an hours-long job whose
   wall clock has never been recorded to completion ([rob](rob.md)).
