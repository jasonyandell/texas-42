# BRIEF-FH3 — the report of record and the PR #87 anchors

**Status: DRAFT until FH2 lands** — finalized by the orchestrator
against the shipped surface. **Authorized:** 2026-09-04. **Binding
theory:** `walt/math/focal_horizon_sandwich_v0.1.md` §XV FH8, §XVI
(the report of record — every measurement listed there), §XVII (success
and falsifiers, §41 correctness failures), as narrowed by the companion
and the FH-A rulings (the anchors' coordinates are ruled at FH-A —
use those). Read `FH1-REPORT.md`, `FH2-REPORT.md`, `U0B-REPORT.md`, and
`walt/probes/factor_belief/horizon_run1.txt`'s horizon table (its tail)
first.

**EXPLORATORY tier throughout.** The parent's own rule is binding: do
not pin the focal-horizon answer in advance beyond the soundness laws.
"The experiment is to discover the smallest k that settles or
ε-settles these anchors." No success rate is assumed (§39); an honest
partial success (§40) is a success; every §41 item is a stop-and-
investigate, never a disappointing number.

## Mission

1. **The report of record** `walt/probes/factor_belief/focal_run1.txt`
   via `focalreport report <out>`: for each root × contract × k ∈
   {0,1,2} (k = 3 where affordable) and each root action, everything §38
   lists — `L_{a,k}`, `U_{a,k}`, `U − L`, `Δ^L_{a,k}`, `Δ^U_{a,k}`,
   survivor set, exact action where independently known (the record's
   exact values where a fresh `response_success_mass` is unaffordable —
   say which), `π_k` id, `L_exec`, `U*_k`, `Γ_k`, the lower-policy's
   root action, action changes by horizon, exact field reads,
   conditioned nodes, suffix receipt hits, completed focal depth,
   refused frontier mass/count, wall. Corpus: T4 × contracts
   {receipt, 33, 36, 39, 42}; T56 × {receipt, 36}; plus the anchors.
2. **The FH8 anchors**, each at k = 0, 1, 2 (3 if affordable): (i) the
   h8-t3 fixed-field root (exact `Q* = 28859/29988`, argmax 1-1, 14 min
   exact in `horizon_run1.txt`; a trick-4 ply cut flipped it to 3-3);
   (ii) h8-t4 at contracts 36 and 39, where a trick-6 ply cut of 7‰
   flipped the root action; (iii) h4-t4 across contracts, the
   contract-sensitive trick-5 specimen (13/65/85/105/33‰). Report the
   smallest k that settles (unique exact survivor), that gives an exact
   tie set, or the `Γ_k` ladder where neither happens — and whether the
   focal-horizon ladder ever selects the ply cut's wrong action (it
   must not select ANY action it cannot certify; a `Settled` verdict at
   any k must agree with the exact argmax — assert it).
3. **Gate file `walt/walt/tests/solver_focal_anchors.rs`:** the anchors
   (ii) and (iii) with `Q_a` recomputed independently — sandwich,
   nesting, containment, `Settled ⇒ exact`, and the ply-cut comparison
   (the `horizon_census` cut argmax vs the focal ladder's verdict, on
   the same root/contract). Anchor (i) is probe-only unless k ≤ 2 there
   is cheap enough to gate under ~3 minutes — measure, then decide,
   and say which in the report.
4. **`walt/briefs/FH3-REPORT.md`** — the findings, in the shape of
   `U0B-REPORT.md`: the question, what was built, the gates, the
   findings with tables (`Δ^L`/`Δ^U` by k; survivors by k; `Γ_k` by k;
   cost by k in reads), the §39/§40 verdict in the parent's own terms,
   the wall, deviations and boundaries. Then the FH3 paragraph in
   `walt/FACTOR-BELIEF.md` (and the status line), the README entries.
   The wiki era page and `walt/LOG.md` are the orchestrator's.

## Discipline

As FH1/FH2. The record is byte-diffed by nobody but must be
reproducible: every number in the report is in the record; wall is the
one approximate number. Commit with `walt FH3:`; no push, no PR. Report
back with: the anchors table (per anchor: verdict by k, `Γ_k` by k,
reads by k), the three-layer picture the parent asked for in one
paragraph, and every §41 item if any fired.
