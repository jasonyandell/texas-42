# BRIEF-FH1 — the focal-horizon engine, k ∈ {0,1,2}, and its soundness gates

**Authorized:** 2026-09-04, Jason ("the implementations, each an
independent fable agent"; "you're the engineer in charge" — this brief
is the orchestrator's task breakdown, not the parent's). **Binding
theory:** `walt/math/focal_horizon_sandwich_v0.1.md` (verbatim,
checksum-pinned) Parts II–X and XIV–XV, **as narrowed by its companion**
`walt/math/focal_horizon_sandwich_v0.1_intake.md` and the rulings
**FH-A1..An** in `walt/CENSUS-RULINGS.md` (section "The focal-horizon
adjudication (2026-09-04)"). Where parent and companion differ, the
companion governs (DS-A17). Read the companion and the rulings BEFORE
the parent's engineering parts; read `walt/FACTOR-BELIEF.md`'s head
(status) and its last three paragraphs (UP0, UP1a, U0b), and
`walt/briefs/U0B-REPORT.md`, before writing code.

**EXPLORATORY tier throughout.** Nothing here is a play-strength claim;
the live default player is untouched; arena and defaults stay on
Jason's word. The parent's §XVIII non-goals are binding (FH-A ruling):
no glue-coalition selection, no Ω×Θ lift, no residual behaviour type,
no partnership prescriptions, no new rules engine, no live default
change, no arena claim.

## Mission

Build ONE generic fixed-field focal-horizon engine — the parent's §28
`focal_horizon(...)` — as a new in-crate module
`walt/walt/src/solver/focal_horizon.rs`, with the parent's gates FH1–FH6
in `walt/walt/tests/solver_focal_horizon.rs`, and a scout probe. Budget
honesty (§23, gate FH7), proof-state facts, and exact suffix reuse (§25)
are slice FH2; the report of record and the FH8 anchors are slice FH3.
Do not build those here beyond what the engine's types must already
leave room for (a typed refusal that carries no number).

### The object (§7–§8, exact-mass form §22)

For a belief `B` (a `FactorBelief`), horizon `k`, lower tail `π` (a
`SlicePolicy` driving the VIEWER seat), field `σ` (the belief's declared
field), and the God upper tail, compute exact integer masses

    M^L_k(B) = Z(B)·L_k(B),   M^U_k(B) = Z(B)·U_k(B)

by structural recursion:

- **decided** (the §5 arithmetic, `decided_success`): both equal `u·Z`;
- **focal, k = 0:** `M^L_0 = viewer_success_mass(oracle, B, π, σ)` (the
  Slice D fixed-policy recursion — the independent evaluator, reused,
  not copied) and `M^U_0 = Z − doomed(B)` (the world-revealed
  continuation: per-world make check over the worlds `B` represents —
  the machinery `solver/horizon.rs::doom_over_belief` already has;
  factor it into a shared `pub(crate)` helper rather than copying it,
  and keep `horizon.rs` calling the same helper so gate H2 still checks
  it);
- **focal, k ≥ 1:** `max_a` over the legal set of the child values at
  `k − 1` (every child shares `Z`, a focal play changes no factor);
  record the argmax under the declared lowest-tile-index tie rule as
  the materialized policy `π_k`'s choice at this public state;
- **hidden (modeled seat):** `Σ_t` over `oracle.branch_masses` of the
  conditioned child at the SAME `k` — public observations consume no
  horizon (§6; this is the design's essential choice and the correctness
  failure §41(5) if violated).

Root: the parent's §16 action-indexed form — for each legal root action
`a`, `L_{a,k}`, `U_{a,k}` on `B_0 a` where `k` counts ADDITIONAL focal
layers after the root action; `B_k = max_a L_{a,k}`; survivors
`S_k = {a : U_{a,k} ≥ B_k}`; the exact-action criterion
`L_{b,k} > max_{a≠b} U_{a,k}`; exact tie set when the relevant endpoints
collapse; `π_k` as a total `SlicePolicy` (the existing `ExtractedPolicy`
shape — a choice table keyed by post-root history with lowest-legal
off-DAG completion — reuse it; add a public constructor if its `new` is
private, or build the sibling type, your call, justified in the
report); `L_exec = V(π_k)`, `U*_k = max_a U_{a,k}`, `Γ_k = U*_k − L_exec`.

Also expose the remaining focal depth `h_f(B)` (§6) as an INDEPENDENT
walk (`focal_depth`), never derived from the engine's own recursion, and
its cheap bound: every focal decision plays one viewer tile, so
`h_f(B) ≤` the viewer's remaining tiles, with equality iff some
continuation stays undecided through the viewer's last tile. (Verify;
if the companion rules differently, the companion governs.)

### Surface (names indicative — L2-A3's naming latitude applies)

    FocalSpec { horizon: usize, node_fiber_cap: u128 }
    focal_horizon(oracle, root, position, lower_tail, field, spec)
        -> FocalHorizonResult
    FocalHorizonResult {
        identity (root_id, field_id, contract, lower tail id, k),
        actions: Vec<ActionInterval { action, lower_mass, upper_mass: Option<u128>,
                                      root_mass, lower: BigRational, upper: Option<BigRational> }>,
        survivors, verdict (Settled{action} | Equivalent{actions,value} | Unresolved{survivors}),
        policy: the materialized π_k, executable_lower, global_upper: Option<..>,
        certified_regret: Option<..>, spend, refusals,
    }
    focal_depth(oracle, belief, field) -> usize

A frontier node whose God enumeration is unaffordable (`Z > node_fiber_cap`)
is a typed refusal carrying NO number: its upper is `None`, every
ancestor's upper is `None`, the exact side (lower) is still complete, and
the refusal names the boundary (fiber, cap, history). FH2 will add
retention of previously valid intervals; here the honest answer is
absence. Never a truncated number (§41(7)).

### Spend

Exact integer counters: field consultations (wrap the field in the
counting decorator `horizon.rs` uses), conditionings, focal nodes,
hidden nodes, lower-tail evaluations, upper-tail evaluations, worlds
enumerated, line-walk nodes, decided-early / decided-terminal. Wall is
reported by the probe only and is the one approximate number.

## Gates (`walt/walt/tests/solver_focal_horizon.rs`)

Corpus and fixtures as `tests/solver_horizon.rs` (copy its helpers:
receipt roots `T4 = {(3,4),(4,4),(8,4),(12,4)}`, `T56 = {(8,5),(3,5),
(12,6),(10,6),(5,6),(4,6)}`, σ0 = `FieldModel::new(level0 n0=2)`,
contracts {receipt, 36} via `horizon::with_contract`, `SupportOracle`).
Lower tail per the FH-A ruling (expected: σ0 driving the viewer seat as
primary; `FixedPreference::lowest_first` as the gate-only second tail —
run FH2/FH4/FH5 under BOTH tails). Exact integer comparison everywhere.

- **FH1 endpoint parity.** For every root action at k = 0:
  `lower_mass == viewer_success_mass(focal_play(a), π)` computed
  independently in the test, and `upper_mass == Z − doomed` where doomed
  is `doom_enumeration`'s per-world truth for that coordinate (the U0
  census's God upper — the same check H2 makes). Ten roots × two
  contracts.
- **FH2 sandwich and nesting.** For k = 0, 1, 2 (and 3 on the t5/t6
  roots) and every root action: `L_{a,k} ≤ L_{a,k+1} ≤ Q_a ≤ U_{a,k+1}
  ≤ U_{a,k}`, with `Q_a` from `response_success_mass` computed
  independently. Assert at least one strict `L` rise and one strict `U`
  fall somewhere on the corpus (else the gate proves nothing).
- **FH3 exact collapse.** On every root where `k ≥ h_f` is affordable
  (t6 roots: `h_f ≤ 2`; t5: `≤ 3`), `L_k = Q = U_k` for every action,
  with `h_f` from the independent `focal_depth` AND checked against the
  viewer-tile bound. Also assert collapse is NOT reached at `k = h_f − 1`
  on at least one coordinate (strict), so the depth is load-bearing.
- **FH4 action containment and survivor monotonicity.** For every k:
  `L_{a,k} ≤ Q_a ≤ U_{a,k}`; the exact argmax set (from the independent
  `Q_a`) is a subset of `S_k`; `S_{k+1} ⊆ S_k`; a `Settled{b}` verdict
  has `b` an exact maximizer; an `Equivalent` verdict's actions are
  exactly the exact maximizers.
- **FH5 executable lower witness — the lower-side no-strategy-fusion
  gate.** For every k: replay `π_k` through `viewer_success_mass`
  (viewer policy = `π_k`, same field) at the root and at every root
  child, and require equality with `L_{a,k}` / `max_a L_{a,k}`. Also
  `Γ_k ≥ 0` and `Q* − L_exec ≤ Γ_k`.
- **FH6 merge before max.** A test-local FUSED implementation of the
  k = 1 upper (per-world max over the next focal action, then sum —
  the strategy-fusion order) must be `≥` the engine's `U_{a,1}` on every
  coordinate and STRICTLY above on at least one (find the specimen on
  the corpus — Theorem 5 says it is any node whose one-step glue gain is
  positive). The engine's `U_{a,1}` must equal the salvation-mask upper
  `max` over next actions of `Pr(world individually salvageable after
  the common action)` computed test-locally (Theorem 5, FH-A ruling).
- **FH-R refusal shape.** Under a tiny `node_fiber_cap`: uppers absent,
  lowers complete and equal to the uncapped run's, refusals name every
  over-cap frontier node, survivors/verdict/regret absent rather than
  computed from a partial upper.
- **FH-D determinism.** Two runs render identically.

Keep the gate file under ~5 minutes in release; put the t5/t6 k = 3
work behind the same corpus constants so FH3 can widen it.

## Scout probe (`walt/walt/src/bin/focalreport.rs`, mode `scout`)

`scout <hand> <trick> <k> [contract] [node-cap]` prints, per root action:
`L_{a,k}`, `U_{a,k}`, width, survivor mark, `π_k` id, and the spend
line; then the root verdict, `L_exec`, `U*`, `Γ_k`, `h_f` bound. Commit
one scout record `walt/probes/factor_belief/focal_run0.txt` over T4 at
the receipt contract for k = 0, 1, 2 (a `scout-corpus` mode is fine).
The report-of-record mode is FH3's; leave the binary open for it.

## Discipline

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
- Vocabulary per the FH-A ruling on "sandwich" (expected: "focal-horizon
  hierarchy / interval" in code and ledger; the word "sandwich" only
  when citing the parent's title). "Necessary outer profile", never
  "certificate". "Certified regret" stays the APS term of art.
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
  root: survivors by k, verdict, Γ_k), every deviation from this brief,
  the wall of the gate file, and anything FH2/FH3 must know about the
  types you shipped.
