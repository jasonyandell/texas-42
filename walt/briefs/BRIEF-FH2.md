# BRIEF-FH2 — budget honesty, interruption and resume, proof-state facts, exact suffix reuse

**Status: DRAFT until FH1 lands** — the orchestrator finalizes the type
names against FH1's shipped surface before launch. **Authorized:**
2026-09-04 (Jason, "take this to the finish line"). **Binding theory:**
`walt/math/focal_horizon_sandwich_v0.1.md` §23 (sound interruption),
§24 (gap measurements), §25 (continuation substitution — the exact
suffix theorem and its identity list), §XV FH7, as narrowed by
`walt/math/focal_horizon_sandwich_v0.1_intake.md` and the FH-A rulings
in `walt/CENSUS-RULINGS.md`. Read `walt/briefs/FH1-REPORT.md` and
`solver/focal_horizon.rs` before anything else; then
`solver/proof_state.rs` (the append-only proof state, `Fact::Bound`,
`ProofProducer`), `solver/opening.rs` (§67.5's "resume ≡ uninterrupted
bytewise" gate — the discipline to reproduce), `solver/frontier.rs`
(typed refusals, §41 census law).

**EXPLORATORY tier throughout.** Non-goals of §XVIII binding.

## Mission

Make the engine anytime-honest and let the proof state consume it.
Three items, in this order.

1. **Budget honesty (§23, gate FH7).** A declared work budget (field-
   read ceiling as the unit — reads are exact and reproducible; plus
   the node fiber cap) may stop a horizon-`k` run before every child
   completes. The result must then be a SOUND partially refined state:
   completed child → its new `[L, U]`; unfinished child → its prior
   valid `[L, U]` (from the completed run at `k − 1` at that node, or
   from a completed `k = 0` tail, or the trivial `[0, Z]`/`[0,1]` when
   nothing prior exists — say which in the type); focal parent
   `[max_a L_a, max_a U_a]`; hidden parent `[Σ L_t, Σ U_t]`. The
   refusal names the actual boundary (reads spent vs ceiling; the node
   where it stopped). A residual frontier — the unfinished nodes with
   their retained intervals — is part of the result and is RESUMABLE:
   resuming from it with the remaining budget and finishing must equal
   the uninterrupted run exactly (bytewise render, the §67.5
   discipline). Nothing dropped; no truncated number installed as if
   complete (§41(7)).
2. **Proof-state facts.** A `FocalHorizonProducer: ProofProducer`
   emitting `Fact::Bound` per root action per horizon: lowers with
   authority `focal-horizon:<tail id>:k=<k>:lower` and `executable =
   true` ONLY when the materialized `π_k` re-priced through
   `viewer_success_mass` witnesses the value (FH1's FH5 makes that
   always true for completed runs; a partial run's retained lowers keep
   their own executability); uppers with authority
   `focal-horizon:god:k=<k>:upper`, `executable = false`. Identity: the
   `SemanticsIdentity` coordinates plus the lower-tail id and `k` in the
   authority string. Facts from a refused/partial run carry the
   retained values (they are valid) — never a partial max.
3. **Exact suffix reuse (§25).** Within one root's ladder over `k`, a
   node whose interval has COLLAPSED (`L = U`, hence `= Q`) is exact
   for every later `k` — install it as a terminal: a memo keyed by the
   belief's full identity (the `FactorBelief`'s componentwise equality
   already binds root, position incl. contract, history, field id,
   factors — use it, never a looser key). Count hits. Reusing under a
   mismatched identity is correctness failure §41(8): a gate must show
   a receipt from one contract is never consulted under another
   (`horizon::with_contract` gives the fixture).

## Gates (`walt/walt/tests/solver_focal_budget.rs`)

- **FH7 budget honesty** (the parent's five bullets, each asserted):
  under a read ceiling too small to finish `k = 1` on an affordable
  root — no child dropped (the residual frontier plus completed
  children cover every root child); unfinished children carry the
  `k = 0` intervals (equal to the uncapped `k = 0` run's); the root
  interval contains the independent exact `Q` for every action; the
  refusal names reads-spent, ceiling and the stopping node; resume
  with the remaining budget then completion renders bytewise equal to
  the uninterrupted `k = 1` run.
- **FH7b monotone under interruption.** Interval endpoints installed
  by a partial run never move the wrong way against the `k − 1` run
  (`L` never falls, `U` never rises) — §41(3)/(4) under budgets.
- **PS1 facts install and close.** The producer's facts install into
  a `ProofState::open` root with zero rejections; the state's closure
  survivors equal the engine's `S_k`; its executable-bar witness value
  equals `L_exec`; the closure's certified regret equals `Γ_k`; a
  second `produce` at `k + 1` only tightens (closure monotone).
- **PS2 executability is honest.** No upper fact is executable; every
  executable lower's policy re-prices to its value through the
  independent evaluator.
- **SR1 suffix reuse is invisible in value.** With the memo on vs off,
  every action interval, survivor set, verdict and `π_k` choice table
  are identical on the corpus; hits > 0 somewhere (else the gate
  proves nothing); the identity-mismatch fixture consults zero
  receipts.
- **SR2 a reused receipt is a §25 identity.** The memo key includes
  field id, contract, history and factors; a belief differing in any
  one coordinate misses.
- Determinism.

## Probe

Extend `focalreport` with a `budget` mode (one root, one ceiling ladder:
reads spent, refusal, residual frontier size/mass, retained-vs-new
interval counts, suffix hits per `k`). Commit `focal_budget_run1.txt`
over T4 at the receipt contract.

## Discipline

As FH1's: `walt/ci/check.sh` green; `ingest/` and freeze 58 untouched;
compose never copy; exact integers; no stored duplicate authority (the
residual frontier is the one place a retained interval lives, and it is
a FACT of a prior run, not a cache of the current one); vocabulary per
FH-A; ambiguity protocol. Write `walt/briefs/FH2-REPORT.md`; append the
FH2 paragraph to `walt/FACTOR-BELIEF.md`; README entry. Commit with
`walt FH2:`; no push, no PR. Report back with: the budget-ladder table,
suffix-hit counts per root, every deviation, and anything FH3 must know.
