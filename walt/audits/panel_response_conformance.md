# Panel-response conformance audits — Claim-D repair + W7–W11, and the τ coupling

**Tier: session evidence.** This is an audit note about code at a named
commit — base `51eac3f` (the tree the audit branch is cut from), gates
added on the audit branch. It is evidence, never a status change; it
promotes nothing, and nothing above exploratory tier may cite it as more
than an audit record. Rulings audited against: PANEL-A3, PANEL-A5,
PANEL-A6 (`walt/CENSUS-RULINGS.md`, "The panel-response adjudication
(2026-08-24)"); card `kanban/done/panel-response-audits.md`; intake
`walt/math/response_walt_panel_and_cancellation_v0.1_intake.md`; parent
response `exchange/inbox/019-023-response-panel-and-cancellation-v0.1.md`
(§§18–24). Mechanical gates:
`walt/walt/tests/solver_panel_conformance.rs` (eight tests, all green at
audit time). Thread labels per the standing framing: audit 1 is **CE**,
audit 2 is **L2**.

---

## Audit 1 — controller vs the Claim-D repair and W7–W11 (CE thread)

### 1a. No retrospective edge-risk assignment — CONFORMS (preallocation)

Ruling audited against (PANEL-A3): *"Edge risk may never be assigned
retrospectively to already-consumed evidence. Sound forms: future-only
opening (reset to 1 at a predictable time, future worlds only) or
preallocation before any evidence enters the process. … The shipped
controller is believed compliant by construction (all-pairs preallocation
on one stream); that belief is carded for audit, not assumed."*

The shipped sound form is **preallocation**, and it is total:

- Every directed edge's α and the common threshold
  `T_edge = m(m-1)/δ_edges` are fixed in `Controller::new`, before any
  world is folded: `walt/walt/src/solver/controller.rs:762-785` computes
  `edge_alpha`, `edge_threshold`, and the complete `DecisionLedger`;
  `controller.rs:786-815` builds the full pair vector (all `m(m-1)/2`
  unordered pairs, both directions served by one state) including the
  optional equivalence processes. The pair vector never grows afterwards
  — the only `pairs.push` in the module is inside `new`.
- No opening path exists at all: the §5.2 one-at-a-time allocation is
  explicitly not built (`controller.rs:15-18` module doc). There is no
  API that adds a pair, changes a threshold, or accepts prior counts or
  evidence into a controller — evidence state is `Controller`-local and
  born empty.
- The O21 sums are enforced at construction: `RiskPlan::strict` /
  `with_equivalence` / `under_run` assert allocations within the decision
  budget and the decision budget within its run allocation
  (`controller.rs:243-293`); `DecisionLedger::allocated_total`
  (`controller.rs:334-350`) is the exact rational sum the gate compares.
- Candidate-set mutation cannot reinterpret old evidence: the epoch is a
  content address over (root identity, sorted PolicyIds, δ scope+value,
  sampler declaration) (`controller.rs:157-183`), and the epoch is folded
  into every world-stream seed (`walt/walt/src/solver/adaptive.rs:264-272`),
  so a new set draws a disjoint stream and old pair counts are
  unreachable by construction (§5.3, `controller.rs:19-24`).
- The pair path is the same shape: `evaluate_pair` fixes its threshold
  before the stream loop (`adaptive.rs:701`), m = 2 preallocation.
- `solver::act` builds a fresh strict plan per decision at
  `δ_d = δ_run/(d(d+1))` with `d` derived statelessly from the public
  record — a predictable ordinal, fixed before any of that decision's
  evidence (`walt/walt/src/solver/act.rs:295-347`); each `act` call is a
  fresh epoch with empty evidence state.
- The exact endpoints reuse cached **outcomes**, never evidence
  (`controller.rs:1004-1067`; each physical world counted exactly once,
  O24) — exactly the §19 license: "Pure policy/world outcomes may remain
  in the cache. They simply do not enter that epoch's evidence."

Gates: `panel_a3_edge_risk_is_fully_preallocated_and_sums_to_the_declared_budget`
(strict preallocation exhausts the budget exactly — there is no
unallocated remainder a retrospective opening could claim),
`panel_a3_a_mutated_candidate_set_is_a_new_epoch_with_a_disjoint_stream`.

### 1b. W8 canonical per-index liveness, W9 speculative isolation — CONFORMS

Ruling audited against (PANEL-A5 / response §22): *"W8 — canonical
per-index liveness replay: batch reconstruction reproduces L_n for every
index, not only first-crossing timestamps; W9 — speculative isolation:
speculative outcomes cannot enter evidence before canonical replay."*

- The controller's batching is loop chunking only: `evaluate_set` calls
  `fold_world(i)` per index in stream order
  (`controller.rs:1158-1210`, comment at 1163-1165), and `fold_world`
  reads `self.live` at fold time — outcomes are computed only for
  candidates in `L_n`, pair updates only for pairs with both endpoints in
  `L_n`, and elimination runs at the END of the index
  (`controller.rs:853-977`). The batch-start-liveness reading that O26
  witnesses against has no code path. W7 (predictable activation) holds
  by the same structure: `L_n` is a function of worlds `0..n-1` only, and
  the escalation check runs at a declared cadence deliberately decoupled
  from the batch size (`controller.rs:390-395`, `1203-1207`).
- The one speculative computation in scope is `evaluate_pair`'s batch of
  outcomes (`adaptive.rs:711-720`): computed ahead, then folded strictly
  in stream order; on a first crossing the function returns and the
  batch's remaining speculative outcomes never touch `a`, `b`, or the
  trace (`adaptive.rs:721-764`). With m = 2 there is no liveness to
  misread. W9 conforms.
- **The O26 batch-boundary ambiguity fixture** (the standing divergence
  witness): it exists in the scratch-tier panel-response verifier
  (`exchange/inbox/verify_walt_panel_response_v0_1.py`,
  `verify_execution_order` — the 5-world 3-candidate `ambiguity_table`
  with canonical vs batch-start engines) and did **not** previously exist
  as a Rust test. This audit ports it:
  `w8_the_o26_ambiguity_fixture_diverges_between_canonical_and_batch_start_liveness`
  pins `E⁺(1,0) = 3/2`, `E⁺(4,1) = 19/10`, the canonical result
  (live {0,2}, edges 0→1@0 and 2→1@0), the naive reading's dead-candidate
  crossings at index 4, and the live-set divergence.
- The existing V8 gate
  (`walt/walt/tests/solver_controller.rs:508-543`) already asserts batch
  sizes 1/16/64 change nothing observable at m = 3; the rerun-determinism
  gate below completes the pair.

### 1c. W10 same-index crossing semantics — CONFORMS, with a caveat that must travel

Ruling audited against (PANEL-A5 / response §19): *"All first crossings
at one index are applied simultaneously. On the no-false-edge event,
settled edges follow strict value order and cannot form a directed cycle.
If a cycle or 'eliminate everyone' condition occurs, the implementation
should emit a typed `InconsistentEvidence` result rather than silently
choose an order."*

- **Deterministic same-index rule: yes.** All pair updates at index `n`
  complete before elimination runs (`controller.rs:872-950` then
  `951-976`), so every crossing at `n` is on the table when elimination
  starts; the elimination is the declared §5.1 live-remover rule run to a
  fixed point in a fixed scan order over a deterministically ordered edge
  list — no randomness, no schedule dependence, and it sits inside
  `fold_world`, so batching cannot reach it. Gate:
  `w10_same_index_resolution_and_the_whole_record_are_deterministic_across_reruns`.
- **Typed `InconsistentEvidence`: not implemented.** No such type exists
  in the codebase. The two inconsistency conditions are instead
  structurally unreachable in the shipped controller:
  1. *Same-pair double crossing* (both directions at once) is guarded by
     a panic assert (`controller.rs:896-899`) and is arithmetically
     impossible: a crossing requires `E⁺ ≥ T` with `T = m(m-1)/δ > 1`,
     and `E⁺(a,b) ≤ 1` whenever `a ≤ b` — so both directions crossing
     would need `a > b` and `b > a`.
  2. *Same-index settled-edge cycles* cannot form: all pairs among
     simultaneously live candidates fold the SAME common worlds
     (§17.2 common random worlds), so the directed count imbalances
     around any candidate cycle telescope to zero pathwise —
     `Σ (u_x − u_y)` around a cycle is identically zero — and by the
     lemma above every settled edge carries a strictly positive
     imbalance. All edges of a cycle strictly positive is a
     contradiction. Cross-index cycles in the edge list are temporal,
     not simultaneous, and the live-remover rule handles them without
     ambiguity; *eliminate-everyone* is impossible because every removal
     requires a currently-live remover, which survives it.
  The lemma half is gated mechanically:
  `w10_crossing_requires_strict_count_majority_so_settled_edge_cycles_cannot_form`
  (`E⁺(a,b) ≤ 1` on the full grid `a ≤ b ≤ 60`; every edge threshold
  exceeds one). The structural half is the common-worlds argument above.
- **The caveat that must travel with this verdict:** conformance is
  *vacuous* — the branch W10 legislates for cannot trigger in the
  shipped single-common-stream all-pairs design. Any future variant that
  breaks the common-worlds structure (per-pair streams, subset
  activation, evidence folded from different world sets per pair, or
  importing crossings computed elsewhere) loses the telescoping argument
  and MUST implement the typed `InconsistentEvidence` result before it
  ships. This obligation is recorded here rather than dead code being
  added now.

### 1d. W11 complete pause state — CONFORMS

Ruling audited against (PANEL-A5 / response §22): *"W11 — complete pause
state: next canonical index, live set, pair counts, first crossings,
policy IDs, risk ledger, and epoch are serialized."*

The controller has no mid-decision suspend API; its pause artifact is the
terminal `Unresolved` evaluation at the world cap, and resuming is
re-evaluation at a higher cap — sound because the stream is counter-based
(§17.1: world `i` is a pure function of (root id, epoch, `i`), so "batch
size, thread count, elimination, and resume boundaries cannot change
which world occupies index `i`", `adaptive.rs:255-272`; the response's
§20 invariance covers exactly this pause/resume pattern).

Every W11 field is present on the pause artifact and serialized where
decisions are recorded:

- In memory (`SetEvaluation` + `SetResult::Unresolved`,
  `controller.rs:520-561`, `651-665`): next canonical index =
  `consumed`; live set = `survivors`; pair counts = `pair_counts` (every
  pair, frozen at elimination); first crossings = `edges` (with their
  stream indices); policy IDs = per open pair in `refinements`
  (`policy_i`/`policy_j`) plus the candidate roster held by the caller
  and folded into the epoch; risk ledger and epoch = `ledger`
  (plan, per-edge α, threshold, epoch, stream identity).
- Serialized: the shadow surface writes all of them per decision record —
  `walt/walt/src/bin/shadow.rs:269-298` (edges, eliminations,
  pair_counts), `:308-350` (the full §8.5 refinement vectors),
  `:355-370` (candidate roster with PolicyIds, survivors), `:372-392`
  (`consumed` and the complete ledger string, which carries the epoch
  and stream identity).
- Noted, not a defect: the compact one-line `Display` of
  `SetResult::Unresolved` alone (`controller.rs:626-637`) omits pair
  counts and first crossings; the complete serialized pause record is
  the evaluation record as the shadow surface writes it, not the
  one-line result tag. Consumers pausing on `Unresolved` must persist
  the evaluation record (as `shadow.rs` does), not the `Display` line.

Gate: `w11_the_unresolved_pause_state_is_complete_and_resume_consistent`
— at a cap provably below any settlement, the artifact carries all
fields; a higher-cap re-evaluation of the same spec never retro-dates a
crossing or elimination into the paused prefix, only narrows the live
set, and extends every pair's counts monotonically.

---

## Audit 2 — slice-2 exposure walk vs the τ coupling definition (L2 thread)

### 2. The first-split fork is exactly τ — CONFORMS

Ruling audited against (PANEL-A6 / response §24): *"τ = inf{ t : h⁰_t =
h¹_t, the actor is non-focal, σ0(J_t) ≠ σ1(J_t) }; if no such t exists,
τ = ∞; D_ρ(ω) = 1{τ < ∞}. The executions are coupled identically until
τ, then may fork."*

`coupled_replay` (`walt/walt/src/solver/exposure.rs:332-410`) implements
this event exactly, conjunct by conjunct:

- *Equal public histories:* the two executions advance in lockstep and
  public-view equality is **asserted at every pre-split step**
  (`exposure.rs:345-349`) — the common prefix is checked, not assumed.
- *Non-focal actor:* the fork can only be reached in the `seat != viewer`
  branch (`exposure.rs:356`/`366`); at focal seats the two executions'
  actions are asserted identical (`exposure.rs:357-363`), so a focal
  disagreement is a panic (an unlawful focal policy), never a split —
  the event is not widened.
- *σ0(J) ≠ σ1(J), first such t:* at every non-focal step both fields are
  queried on the same information state (`exposure.rs:367-368`; equal
  hands asserted at `:353`); agreement advances both executions coupled,
  and the FIRST disagreement records the split and forks
  (`exposure.rs:372-394`). Because every non-focal step on the common
  prefix is tested in order, the fork index is exactly the infimum — the
  event is not narrowed.
- *D_ρ = 1{τ < ∞}:* `CoupledOutcome::exposed()` is `split.is_some()`
  (`exposure.rs:185-201`); τ = ∞ runs to terminal with equal histories
  and asserts equal payoffs (L2-T1, `exposure.rs:398-404`).

Gates: `panel_a6_identical_fields_never_split_so_the_event_is_not_wider_than_tau`
(σ0 = σ1 ⇒ the event is empty on every fiber world);
`panel_a6_the_first_split_is_the_symmetric_non_focal_stopping_time`
(τ is a symmetric function of the unordered field pair on the common
prefix: swapping σ0/σ1 yields the same stopping state with tiles and
terminals swapped; split seat always non-focal; the fixture demonstrably
splits, so the gate is non-vacuous).

---

## Disposition

Both audits: **CONFORMS** on every point (1a, 1b, 1c with its traveling
caveat, 1d, 2). No defect filed. The one judgment call is 1c: the typed
`InconsistentEvidence` result demanded by W10's letter does not exist in
the code; the verdict rests on the trigger condition being structurally
unreachable in the shipped design (lemma gated, structure argued above).
If that reading is rejected on review, 1c reopens as a defect and the
card returns to the backlog.
