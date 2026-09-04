# BRIEF-FH1 — the focal-horizon engine, k ∈ {0,1,2}, and its soundness gates

**Authorized:** 2026-09-04, Jason ("the implementations, each an
independent fable agent"; "you're the engineer in charge" — this brief
is the orchestrator's task breakdown, not the parent's). **Binding
theory:** `walt/math/focal_horizon_sandwich_v0.1.md` (verbatim,
checksum-pinned) Parts II–X and XIV–XV, **as narrowed by its companion**
`walt/math/focal_horizon_sandwich_v0.1_intake.md` and the rulings
**FH-A1..A11** with the delivered propositions FH-God, FH-int, FH-tie,
FH-cut, FH-last in `walt/CENSUS-RULINGS.md` (section "The focal-horizon
adjudication (2026-09-04)" — the last section of the file). Where
parent and companion differ, the companion governs (DS-A17). Read the
rulings section and the companion's "eight questions" and "Proofs"
parts BEFORE the parent's engineering parts; read `walt/FACTOR-BELIEF.md`'s
head (status) and its last three paragraphs (UP0, UP1a, U0b), and
`walt/briefs/U0B-REPORT.md`, before writing code.

**EXPLORATORY tier throughout.** Nothing here is a play-strength claim;
the live default player is untouched; arena and defaults stay on
Jason's word. The parent's §XVIII non-goals are binding (FH-A10): no
glue-coalition selection, no Ω×Θ lift, no residual behaviour type, no
partnership prescriptions, no new rules engine, no live default change,
no arena claim.

**Vocabulary (FH-A2, binding):** focal-horizon hierarchy (the
construction), focal-horizon interval `[L_k(B), U_k(B)]`, action
interval `[L_{a,k}, U_{a,k}]`, bar `B_k`, survivor set `S_k`, focal
depth `h_f`. "Sandwich" appears only when citing the parent's title.
"Necessary outer profile", never "certificate"; "certified regret"
stays the APS term of art.

## Mission

Build ONE generic fixed-field focal-horizon engine — the parent's §28
`focal_horizon(...)` — as a new in-crate module
`walt/walt/src/solver/focal_horizon.rs`, with the parent's gates FH1–FH6
(plus the free parity gates FH-A11 names) in
`walt/walt/tests/solver_focal_horizon.rs`, and a scout probe. **FH1 is
an affordable-or-refuse engine (FH-A11, binding):** a root either
completes at the requested horizon or returns one typed whole-root
refusal naming the boundary — no partial intervals, no lower-only
intervals, no trivial upper installed as a fact (FH-A3). Budget
honesty with retained intervals (§23, gate FH7), proof-state facts, and
exact suffix reuse (§25) are slice FH2, and they are gated on
Proposition FH-int's intersection-and-witness discipline, which is why
FH1 may not report partial results. The report of record and the FH8
anchors are slice FH3.

### The object (§7–§8, exact-mass form §22)

For a belief `B` (a `FactorBelief`), horizon `k`, lower tail `π` (a
`SlicePolicy` driving the VIEWER seat), field `σ` (the belief's declared
field), and the God upper tail, compute exact integer masses

    M^L_k(B) = Z(B)·L_k(B),   M^U_k(B) = Z(B)·U_k(B)

by structural recursion:

- **decided** (the §5 arithmetic through `decided_success` — the SAME
  predicate everywhere, FH-A6): both equal `u·Z`;
- **focal, k = 0:** `M^L_0 = viewer_success_mass(oracle, B, π, σ)` (the
  Slice D fixed-policy recursion — the independent evaluator, reused,
  never copied) and `M^U_0 = Z − doomed(B)` = the count of undoomed
  worlds (Proposition FH-God: this IS `G`; the per-world make check
  over the worlds `B` represents is `solver/horizon.rs::doom_over_belief`
  — factor it into a shared `pub(crate)` helper rather than copying it,
  and keep `horizon.rs` calling the same helper so gate H2 still checks
  it);
- **focal, k ≥ 1:** `max_a` over the FULL legal set of the child values
  at `k − 1` (every child shares `Z`, a focal play changes no factor;
  every legal action, always — §41, FH-int); record the argmax under
  `TieRule::LowestTileIndex` as the materialized policy `π_k`'s choice
  at this public state. A FORCED focal node (singleton legal set)
  consumes a unit of horizon like any other (FH-A6, binding);
- **hidden (modeled seat):** `Σ_t` over every positive-mass branch of
  `oracle.branch_masses` of the conditioned child at the SAME `k` —
  public observations consume no horizon (§6; correctness failure
  §41(5) if violated).

Root: the parent's §16 action-indexed form — for each legal root action
`a`, `L_{a,k}`, `U_{a,k}` on `B_0 a` where `k` counts ADDITIONAL focal
layers after the root action; bar `B_k = max_a L_{a,k}`; survivor set
`S_k = {a : U_{a,k} ≥ B_k}`; verdict: `Settled{b}` iff
`L_{b,k} > max_{a≠b} U_{a,k}` (§18); `Equivalent{actions, value}` iff
every survivor is collapsed (`L = U`) — the exact optimal set is then
`{a ∈ S_k : Q_a = B_k}` (Proposition FH-tie); else
`Unresolved{survivors}`. `π_k` as a total `SlicePolicy`: the choice
table keyed by post-root history for the first `k` layers, and **the
tail π below and off-DAG** (FH-A7, binding — the existing
`ExtractedPolicy` completes off-DAG by lowest tile, which is right only
for the `lowest_first` tail; build a sibling type holding the table plus
a tail reference, or generalize the extractor — your call, justified).
`L_exec = V(π_k)` at the root (= `B_k` by construction, and FH5 checks
it), `U*_k = max_a U_{a,k}`, `Γ_k = U*_k − L_exec`.

Identity (FH-A4): the σ0 tail reads the bid, so a lower is per (root,
contract, field, tail id, k) and is never projected across contracts;
carry the tail id and `k` in the result's identity.

Also expose the focal depth `h_f(B)` (§6, FH-A6) as an INDEPENDENT walk
(`focal_depth`), never derived from the engine's own recursion, using
the same `decided_success` cutoff, forced nodes counting. Its
independent checks (FH-A6): `h_f ≤` the viewer's remaining tiles;
`h_f = 7 − T` after the root action at an undecided viewer-lead
trick-`T` uniform root; and **tail consultations = 0 whenever
`k ≥ h_f`** — count lower-tail and upper-tail evaluations in the spend
so this is mechanical. Proposition FH-last: trick 7 is forced, so
trick-6 roots collapse at k = 0, trick-5 at k = 1, trick-4 at k = 2 —
which makes k = 2 on the trick-4 corpus a COLLAPSE gate and the h8-t3
root the real k = 2 test (FH3's).

### Surface (names indicative — L2-A3's naming latitude applies)

    FocalSpec { horizon: usize, node_fiber_cap: u128 }
    focal_horizon(oracle, root, position, lower_tail, field, spec)
        -> Result<FocalHorizonResult, FocalRefusal>   // or an enum
    FocalHorizonResult {
        identity (root_id, field_id, contract, tail id, k),
        actions: Vec<ActionInterval { action, lower_mass, upper_mass, root_mass,
                                      lower: BigRational, upper: BigRational }>,
        bar, survivors, verdict, policy: π_k, executable_lower, global_upper,
        certified_regret, spend,
    }
    FocalRefusal { UpperUnaffordable { history, fiber, cap }, ... }
    focal_depth(oracle, belief, field) -> usize

A frontier node whose God enumeration is unaffordable (`Z >
node_fiber_cap`) refuses the WHOLE root with the boundary named
(history, fiber, cap); nothing is dropped silently, nothing partial is
returned (FH-A11). Never a truncated number (§41(7)).

### Spend

Exact integer counters: field consultations (wrap the field in the
counting decorator `horizon.rs` uses), conditionings, focal nodes,
hidden nodes, lower-tail evaluations, upper-tail evaluations, worlds
enumerated, line-walk nodes, decided-early / decided-terminal, and
**the ply distribution of tail consultations** (a histogram by
post-root depth — FH-A11's "ply distribution of the k-th focal
frontier"). Wall is reported by the probe only and is the one
approximate number.

## Gates (`walt/walt/tests/solver_focal_horizon.rs`)

Corpus and fixtures as `tests/solver_horizon.rs` (copy its helpers:
receipt roots `T4 = {(3,4),(4,4),(8,4),(12,4)}`, `T56 = {(8,5),(3,5),
(12,6),(10,6),(5,6),(4,6)}`, σ0 = `FieldModel::new(level0 n0=2)`,
contracts {receipt, 36} via `horizon::with_contract`, `SupportOracle`).
Tails (FH-A4): σ0 driving the viewer seat is PRIMARY; `FixedPreference::
lowest_first` is the gate-only second tail — run FH2/FH4/FH5 under
BOTH. Exact integer comparison everywhere. Horizons: k = 0, 1, 2 on T4;
k = 0, 1, 2 on t5 and t6 too (values beyond `h_f` must be CONSTANT —
assert it).

- **FH1 endpoint parity.** For every root action at k = 0:
  `lower_mass == viewer_success_mass(focal_play(a), π)` computed
  independently in the test; `upper_mass == Z − doomed` where doomed is
  `doom_enumeration`'s per-world truth for that coordinate (the U0
  census's `GodUpper` — the same check H2 makes). Ten roots × two
  contracts.
- **FH1b record parity (free oracles, FH-A5/A11).** On the viewer-lead
  trick-4 roots h3-t4/h4-t4/h8-t4 at the receipt contract and 36: the
  engine's `U_{a,0}` equals `horizon_census(cut_plays = 4)`'s per-action
  cut reading and `U_{a,1}` equals the `cut_plays = 8` reading, computed
  live in the test (Proposition FH-cut). Optionally also byte-check
  against the committed `horizon_run1.txt` values quoted in the
  companion's Q6 table.
- **FH2 nesting.** For every root action and consecutive k:
  `L_{a,k} ≤ L_{a,k+1} ≤ Q_a ≤ U_{a,k+1} ≤ U_{a,k}`, with `Q_a` from
  `response_success_mass` computed independently. Assert at least one
  strict `L` rise and one strict `U` fall somewhere on the corpus (else
  the gate proves nothing).
- **FH3 exact collapse (FH-last, as corrected 2026-09-04 after the
  builder's blocked test).** At t6 roots k = 0, t5 roots k = 1, t4 roots
  k = 2 (= `6 − T`): `L_k = Q = U_k` for every action — the forced
  trick-7 layer still CONSULTS the tails there (FH-A6 counts it, so
  `h_f = 7 − T`), it just cannot move the value. Tail consultations = 0
  and `focal_depth = 7 − T` are the checks at `k = 7 − T`; the
  viewer-tile bound holds at both. Also assert collapse is NOT reached at `k − 1` on at
  least one coordinate per trick (strict), so the depth is load-bearing.
- **FH4 action containment and survivor monotonicity.** For every k:
  `L_{a,k} ≤ Q_a ≤ U_{a,k}`; the exact argmax set (from the independent
  `Q_a`) is a subset of `S_k`; `S_{k+1} ⊆ S_k`; a `Settled{b}` verdict
  has `b` the unique exact maximizer; an `Equivalent` verdict's actions
  are exactly the exact maximizers (FH-tie).
- **FH5 executable lower witness — the lower-side no-strategy-fusion
  gate (FH-A7).** For every k and both tails: replay `π_k` through
  `viewer_success_mass` (viewer policy = `π_k`, same field) at the root
  and at every root child, and require equality with `L_{a,k}` /
  `B_k`. Also `Γ_k ≥ 0` and `Q* − L_exec ≤ Γ_k`. For the σ0 tail, assert
  the off-DAG continuation is σ0 (a state off the table returns σ0's
  choice, not the lowest tile).
- **FH6 merge before max.** A test-local FUSED implementation of the
  k = 1 upper (per-world max over the next focal action, then sum —
  the strategy-fusion order) must be `≥` the engine's `U_{a,1}` on every
  coordinate and STRICTLY above on at least one (the specimen exists
  wherever `U_{a,0} > U_{a,1}`; FH-A8 names h8-t4 at bid 36/39, 5-5 vs
  2-1, as one place to look). The engine's `U_{a,1}` must equal the
  salvation-mask upper — `max` over next actions of the mass of worlds
  individually salvageable after the common action — computed
  test-locally (Theorem 5).
- **FH-R refusal shape.** Under a tiny `node_fiber_cap`: one typed
  whole-root refusal naming history/fiber/cap; no intervals, no
  verdict, no regret; and the same root under an ample cap completes.
- **FH-D determinism.** Two runs render identically.

Keep the gate file under ~6 minutes in release. Note that FH-A8's law
statements are gates too if cheap: at h8-t4 bids 36/39 the verdict at
k = 1 is NOT `Settled{2-1}` (because `U_{5-5,1} = 757‰ > Q_{2-1} =
750‰`), and at h8-t4 k = 2 it collapses.

## Scout probe (`walt/walt/src/bin/focalreport.rs`, mode `scout`)

`scout <hand> <trick> <k> [contract] [node-cap] [tail]` prints, per
root action: `L_{a,k}`, `U_{a,k}`, width, survivor mark; then the root
verdict, `π_k` id, `L_exec`, `U*`, `Γ_k`, `focal_depth`, and the spend
line including the tail-consultation ply histogram. Commit one scout
record `walt/probes/factor_belief/focal_run0.txt` over T4 at the receipt
contract for k = 0, 1, 2 under the σ0 tail (a `scout-corpus` mode is
fine). The report-of-record mode is FH3's; leave the binary open for it.

## Discipline

- **Never end a turn with background work pending** (CLAUDE.md, Agents):
  run the gate file, `check.sh` and the record in the foreground under the
  600 s tool timeout, split or polled in a foreground loop — a yielded
  agent is not woken when its background job finishes.
- `walt/ci/check.sh` green (fmt, clippy `-D warnings -D float_arithmetic`,
  no-float greps, vocabulary greps, release tests, Lean) before calling
  the slice done — it takes ~6 minutes.
- `ingest/` untouched; freeze 58 (`solver/refine.rs`) untouched;
  `horizon.rs`, `doom.rs`, `godgap.rs` touched for VISIBILITY/FACTORING
  only, with their gates unchanged and green (H1–H5, G1–G5, doom gates).
- Compose, never copy: `viewer_success_mass`, `response_success_mass`
  (gates only), `decided_success`, `legal_plays`, `ExactCoverOracle`,
  the doom line-walk. No second authority for any of them.
- Exact integers and rationals only; no floats; no stored state that
  duplicates a derived view (the posterior is a function of root +
  history).
- Ambiguity protocol: a spec conflict gets a blocked test and an exact
  citation, never a plausible reading.
- Write `walt/briefs/FH1-REPORT.md` (the shape of `U0B-REPORT.md`: the
  change in one paragraph, what was built, the gates, the scout
  findings with numbers, deviations, flags). Append the FH1 status
  paragraph to `walt/FACTOR-BELIEF.md` after U0b's and extend its status
  line. Add the instrument and gate file to
  `walt/probes/factor_belief/README.md`.
- Commit on the current branch with a message starting `walt FH1:`; do
  not push, do not open a PR — the orchestrating session reviews and
  lands it. Report back with: the scout table for T4 at k = 0, 1, 2 (per
  root: survivors by k, verdict, `Γ_k`, reads), every deviation from
  this brief, the wall of the gate file, and anything FH2/FH3 must know
  about the types you shipped. Keep the report under about 900 words.
