# BRIEF-FH2 — the focal-horizon ladder: budget honesty, interruption and resume, proof-state facts, exact suffix reuse

**Authorized:** 2026-09-04 (Jason, "take this to the finish line").
**Binding theory:** `walt/math/focal_horizon_sandwich_v0.1.md` §19
(certified regret and "preserved facts"), §23 (sound interruption), §24
(gap measurements), §25 (continuation substitution), §XV FH7, **as
narrowed by** `walt/math/focal_horizon_sandwich_v0.1_intake.md` (P8,
P11, P12 — the proofs) and rulings **FH-A3, FH-A9, FH-A11** with
**Proposition FH-int** in `walt/CENSUS-RULINGS.md` (last section). The
companion governs where it differs from the parent. Read
`walt/briefs/FH1-REPORT.md` and `solver/focal_horizon.rs` (718 lines —
the whole thing) before anything else; then `solver/proof_state.rs`
(`Fact::Bound`, `BoundFact::lower/upper`, `ProofProducer`,
`ProofState::open`, closure views), `solver/opening.rs` (the §67.5
"resume ≡ uninterrupted bytewise" gate — reproduce its discipline),
`solver/frontier.rs` (typed refusals, the §41 census law), and
`CLAUDE.md` (the "Agents" section and the gate-sizing rule).

**EXPLORATORY tier throughout.** Non-goals of §XVIII binding (FH-A10):
no glue-coalition selection, no Ω×Θ lift, no live default change, no
arena claim. Vocabulary per FH-A2 (focal-horizon hierarchy / interval,
never "sandwich" as an object name).

## What FH1 shipped, and what this slice adds

FH1's `focal_horizon(...) -> Result<FocalHorizonResult, FocalRefusal>`
is affordable-or-refuse: one frontier node over the fiber cap refuses
the whole root (FH-A11 required this BEFORE Proposition FH-int's
discipline existed in code). `Engine::walk(belief, k) -> Result<Node,
FocalRefusal>` propagates that refusal from the single `k = 0` frontier
site; `Node { lower, upper, choices }` is one subtree's exact-mass
interval with `π_k`'s choice table through it; `FocalChoices` is
content-addressed and `with_tail(..)` makes it a total `SlicePolicy`;
`FocalSpend` counts field and tail reads separately and holds the ply
histogram. This slice makes the engine ANYTIME under FH-int — a ladder
over `k` at one root that may be interrupted by a budget and resumed —
and lets the proof state consume it. Three items, in this order.

### 1. The ladder and the interruption rule (§23, FH-int, gate FH7)

Build a per-root **ladder** (a new module — `solver/focal_ladder.rs` is
a fine name — or an extension of `focal_horizon.rs`; your call,
justified): an append-only store of **node facts** over one root under
one identity (FH1's five coordinates minus the horizon). A node fact is
`[L(C), U(C)]` in mass form with **the policy that attains `L(C)` stored
with it** (a `FocalChoices`-shaped sub-table rooted at `C`, plus the
tail — every lower fact carries its witness, FH-A9) and the horizon it
was established at. Facts are installed by **intersection** (lower =
max with the prior, keeping the winning policy; upper = min with the
prior), never replacement; a node never priced holds NO upper (the
trivial 1 is a placeholder, never a fact, FH-A3) and the lower `0` with
policy = the tail.

`advance(ladder, k, budget) -> Outcome` runs FH1's recursion at horizon
`k` under a **work budget** (field + tail reads as the unit — exact and
reproducible; plus the node fiber cap). When the budget ends
mid-walk the run STOPS (deterministically — reads are deterministic)
and the result is the sound partial state of Proposition FH-int:
completed nodes get their new facts intersected in; unfinished nodes
keep their prior facts (from the `k − 1` pass, or the placeholder);
composition upward is `[max_a L(Ba), max_a U(Ba)]` over EVERY legal
action at focal nodes — the upper is absent if any child's is — and
`[Σ_t L(B_t), Σ_t U(B_t)]` over EVERY positive-mass branch at hidden
nodes. `Outcome` is `Completed { result }` or `Interrupted {
residual_frontier: the unfinished nodes with their retained facts,
reads_spent, ceiling, stopping node }` — a typed boundary, never a
truncated number (§41(7)). The root's action intervals, bar, survivors,
verdict and `Γ` are DERIVED VIEWS of the fact set (never stored twice);
an action whose upper is absent has no interval and blocks `Settled`.

**Resume:** `advance` again at the same `k` with more budget continues
from the residual frontier; `resume` then `completion` must equal the
uninterrupted run EXACTLY on every derived view and on the fact set
(bytewise render, the §67.5 discipline). The spend is compared as a
sum, not bytewise.

### 2. Proof-state facts

A `FocalHorizonProducer: ProofProducer` over a ladder emits
`Fact::Bound` per root action: lowers with authority
`focal-horizon:<tail id>:k=<k>:lower`, `executable = true` ONLY when the
stored policy re-prices to the value through `viewer_success_mass`
(FH1's FH5 makes this hold for completed runs; a partial run's retained
lowers carry their own witness — check it, do not assume it); uppers
with authority `focal-horizon:god:k=<k>:upper`, `executable = false`.
Facts from an interrupted pass carry the RETAINED values (valid by
FH-int) — never a partial max. `Γ_{k+1} ≤ Γ_k` must hold on the ladder
unconditionally because nothing is discarded (P8(ii)).

### 3. Exact suffix reuse (§25, P12)

Within one root's ladder, a node whose fact has **collapsed** (`L = U`,
hence `= Q`) is exact for every later `k`: the walk at any higher
horizon returns the fact at that node instead of descending. The memo
is keyed by the belief's FULL identity — `FactorBelief`'s componentwise
equality already binds root, position (incl. contract), history, field
id and factors (the posterior with weights) — never a looser key
(record alone is the PiKey defect, CBS-A6/FH-A9). Count hits; the
identity-mismatch fixture (`horizon::with_contract`) consults zero
receipts.

## Gates (`walt/walt/tests/solver_focal_ladder.rs`) — sized to laws

Per the CLAUDE.md gate-sizing rule: one root per law plus a PINNED
strictness witness; expensive values (exact `Q_a`, uninterrupted
results) in one `LazyLock` fixture per suite; keep the file well under
~2 minutes standalone. Corpus: h8-t4 (Z = 1,200, cheap, and FH1 showed
survivors 4 → 3 → 1 there — the natural interruption witness), h3-t4
for one heavier row, one t6 root for the trivial cases. Both tails only
where the law mentions the tail.

- **FH7 budget honesty** (the parent's five bullets, each asserted): a
  read ceiling too small to finish `k = 1` at h8-t4 after `k = 0`
  completed — no root child dropped (residual frontier ∪ completed
  children = every child); unfinished children carry the `k = 0` facts
  (equal to the uncapped `k = 0` run's); every root action's interval
  contains the independent exact `Q_a`; the refusal names reads-spent,
  ceiling and the stopping node; resume + completion ≡ uninterrupted
  `k = 1` on every derived view and the fact set.
- **FH7b monotone under interruption.** Across a ladder of ceilings
  (a pinned schedule), no lower ever falls and no upper ever rises at
  any node or root action; `Γ` never rises; survivors only shrink.
- **FH7c the placeholder is not a fact.** A root whose `k = 0` pass is
  interrupted before any upper exists yields `Unresolved` with NO
  action interval and NO regret — never a number built from the
  trivial upper.
- **PS1 facts install and close.** The producer's facts install into
  `ProofState::open` with zero rejections; closure survivors equal the
  ladder's; the executable-bar witness equals `L_exec`; closure's
  certified regret equals `Γ_k`; a second `produce` at `k + 1` only
  tightens.
- **PS2 executability is honest.** No upper is executable; every
  executable lower's stored policy re-prices to its value through the
  independent evaluator, including one RETAINED lower from an
  interrupted pass.
- **SR1 suffix reuse is invisible in value** (memo on vs off: identical
  intervals, survivors, verdicts, choice tables at h8-t4 and h3-t4 over
  k = 0..2; hits > 0 pinned at a named node); **SR2** the identity is
  the full belief (the `with_contract` fixture: zero hits; a belief
  differing in factors alone: a miss).
- **FH-D** determinism of an interrupted run.

## Probe

`focalreport ladder <hand> <trick> [contract] <ceiling schedule>`: per
step — reads spent, outcome, residual frontier size/mass, retained-vs-
new fact counts, suffix hits, and the derived root views. Commit
`walt/probes/factor_belief/focal_ladder_run1.txt` over h8-t4 and h3-t4
at the receipt contract with a pinned ceiling schedule (say the schedule
in the record header).

## Discipline

- **Never end a turn with background work pending** (CLAUDE.md,
  Agents): run gate files, `check.sh` (now ~6 min under the concurrent
  runner) and the record in the foreground under the 600 s tool
  timeout, split or polled in a foreground loop.
- `ingest/` untouched; freeze 58 untouched; FH1's gates stay green and
  unweakened (if the engine's signature must change, update
  `solver_focal_horizon.rs` plumbing only — CI1 may have added a
  fixture there; keep it).
- Compose, never copy: FH1's `Engine`, `viewer_success_mass`,
  `ProofState`. Derived views never stored state: the ladder stores
  FACTS; every root view is computed from them. Exact integers; no
  floats; ambiguity protocol on any spec conflict.
- Write `walt/briefs/FH2-REPORT.md` (the FH1 shape); append the FH2
  paragraph and status-line clause to `walt/FACTOR-BELIEF.md`; README
  entries for the gate file and probe mode.
- Commit with `walt FH2:`; no push, no PR. Report back with: the
  ladder table for h8-t4 (per ceiling: outcome, survivors, `Γ`, reads,
  suffix hits), every deviation, the gate file's wall, and what FH3
  must know about the ladder's types — under about 900 words.
