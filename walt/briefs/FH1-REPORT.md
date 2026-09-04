# FH1-REPORT — the focal-horizon engine, k ∈ {0, 1, 2}, as built

**Slice:** FH1, the parent's §28 generic fixed-field engine of
`walt/math/focal_horizon_sandwich_v0.1.md` (cited by title; the object is
the FOCAL-HORIZON HIERARCHY, FH-A2) as narrowed by its companion and
rulings FH-A1..A11, under `walt/briefs/BRIEF-FH1.md`. Authorized
2026-09-04 by Jason. **Status: COMPLETE as an affordable-or-refuse
engine (FH-A11).** New module `solver/focal_horizon.rs`, new probe
`bin/focalreport.rs`, 10 gates in `walt/walt/tests/solver_focal_horizon.rs`
(326 s release), record `walt/probes/factor_belief/focal_run0.txt` (T4 at
the receipt contract, k = 0, 1, 2, σ0 tail). Two visibility-only edits
outside new files: `horizon.rs`'s `doom_over_belief` and its counting
field decorator became `pub(crate)` so the engine's God upper tail IS the
doom census's line walk (gate H2 still checks it there, gate FH1 here).
Freeze 58 untouched; `ingest/` untouched.

**EXPLORATORY tier throughout.** Every number below is exploratory; the
live default player is untouched; no arena claim (FH-A10).

---

## THE CHANGE IN ONE PARAGRAPH

Three existing instruments — the fixed-policy evaluator
(`viewer_success_mass`), the God-gap census's undoomed count, and the
in-solve horizon census's ply cut — are now the endpoints of ONE object:
the focal-horizon interval `[L_k(B), U_k(B)]`, refined by focal decisions
rather than plies. At `k = 0` the lower is the tail's value and the upper
is the world-revealed count; each further `k` replaces one layer of
clairvoyance on the upper side and one layer of the tail on the lower side
with a lawful max over EVERY legal action at ONE public information state,
hidden branches summing at the same `k`. The root reports action
intervals, a bar, a survivor set, a verdict (`Settled` / `Equivalent` /
`Unresolved`), the materialized policy `π_k` as a total `SlicePolicy`
(argmax table on the DAG, the tail off it — FH-A7), and the certified
regret `Γ_k`. A frontier node above the fiber cap refuses the whole root,
typed; nothing partial, no trivial upper installed as a fact (FH-A3).

## WHAT WAS BUILT

- `focal_horizon(oracle, root, position, lower_tail, field, spec) ->
  Result<FocalHorizonResult, FocalRefusal>`: the §7/§8 recursion in
  exact-mass form (§22), composed from `decided_success`, `legal_plays`,
  the oracle's `branch_masses`/`condition`, `viewer_success_mass` (lower
  tail) and `doom_over_belief` (upper tail). Argmax under
  `TieRule::LowestTileIndex` by strictly-greater replacement over
  ascending tile iteration; forced focal nodes consume a unit (FH-A6).
- `FocalHorizonResult { identity (root, field, contract, tail id, k),
  actions: Vec<ActionInterval>, bar_mass, survivors, verdict, policy:
  FocalChoices, executable_lower_mass, global_upper_mass,
  certified_regret, spend }`. `FocalChoices::with_tail(&dyn SlicePolicy)
  -> FocalPolicy` is the total policy; the choice table is not itself a
  policy because off-DAG it must be the tail, not the lowest tile.
- `FocalSpend`: field reads and tail reads (separately), conditionings,
  focal/hidden nodes, decided early/terminal, lower- and upper-tail
  evaluations, FORCED tail evaluations, worlds enumerated, line-walk
  nodes, and the ply histogram of the k-th focal frontier.
- `focal_depth(oracle, belief, field) -> usize`: the §6 walk, independent
  of the engine, same decided predicate, forced nodes counting.
- `focalreport scout <hand> <trick> <k> [contract] [cap] [sigma0|lowest]
  [exact]` and `scout-corpus <out>`; the report-of-record mode is FH3's.

## THE GATES (`solver_focal_horizon.rs`, 10, 326 s release wall)

- **FH1** endpoint parity on ten roots × two contracts: `L_{a,0}` equals
  `viewer_success_mass` under BOTH tails; `U_{a,0}` equals `Z − doomed`
  from `doom_enumeration` — the harmonicity of `G` through hidden nodes
  (the companion's W2(g) concern) is checked on real roots.
- **FH1b** record parity (FH-cut): on h3/h4/h8-t4 × {receipt, 36},
  `horizon_census(cut 4)` per-action readings equal `U_{a,0}` and cut-8
  readings equal `U_{a,1}`, live; the companion's Q6 quotations (h8-t4
  bid 36 cut 8; h4-t4 bid 39 cut 4) reproduced as rationals.
- **FH2** nesting for every action, consecutive k, both tails; strict
  rises and falls seen; bar, global upper and `Γ` monotone.
- **FH3** collapse at k = 6 − T with EVERY tail consultation at a forced
  node; zero consultations at k = 7 − T = h_f; `h_f = 7 − T` after the
  root action at every undecided root, 0 at decided ones, ≤ viewer
  tiles; not collapsed one layer earlier on some coordinate per trick;
  intervals constant beyond `h_f` (t5/t6 at k = h_f + 1).
- **FH4** `L ≤ Q ≤ U`; exact argmax ⊆ `S_k`; `S_{k+1} ⊆ S_k`; `Settled`
  names the unique exact maximizer; `Equivalent` lists exactly the
  maximizers with `Q* = B_k` (FH-tie); both verdicts seen.
- **FH5** `V(π_k)` through the independent evaluator equals `B_k` at the
  root and `L_{a,k}` at every root child, every k, both tails (the
  lower-side no-fusion gate); `Γ ≥ 0`, `Q* − L_exec ≤ Γ`; off the DAG
  the σ0-tail policy returns σ0's choice, discriminated from the lowest
  tile on real states.
- **FH6** a test-local fused k = 1 upper (per-world max then sum) is ≥
  `U_{a,1}` everywhere, strictly above on specimens, and EQUALS
  `U_{a,0}` (it is the world-revealed value, FH-God); `U_{a,1}` equals
  the salvation-mask upper computed test-locally (Theorem 5) — four
  (root, contract) coordinates, every action.
- **FH-A8** at h8-t4 bids 36 and 39: `U_{5-5,1} = 303/400 > Q_{2-1} =
  3/4`, k = 1 is not `Settled{2-1}`, 5-5 survives; k = 2 is
  `Settled{2-1}` with every action collapsed. Both tails.
- **FH-R** cap 8 at h8-t4 → one `UpperUnaffordable { history, fiber, cap }`
  naming a viewer node whose rebuilt fiber matches; the ample cap
  completes. **FH-D** determinism.

## THE SCOUT FINDINGS (`focal_run0.txt`, T4, receipt contract 30, σ0 tail)

| root | Z | k | survivors | verdict | B_k ‰ | U* ‰ | Γ_k ‰ | reads (field + tail) |
|---|---:|---|---|---|---:|---:|---:|---:|
| h3-t4 | 11,550 | 0 | 3-1 4-4 6-4 | UNRESOLVED (π plays 4-4) | 288 | 365 | 76 | 1.78M |
| | | 1 | 3-1 | SETTLED 3-1 | 338 | 350 | 12 | 2.63M |
| | | 2 | 3-1 | SETTLED 3-1 (collapse) | 350 | 350 | 0 | 2.83M |
| h4-t4 | 34,650 | 0 | 6-5 | SETTLED 6-5 | 964 | 994 | 30 | 5.13M |
| | | 1 | 6-5 | SETTLED 6-5 | 970 | 981 | 10 | 8.61M |
| | | 2 | 6-5 | SETTLED 6-5 (collapse) | 980 | 980 | 0 | 10.2M |
| h8-t4 | 1,200 | 0 | all four | UNRESOLVED (π plays 3-3) | 885 | 985 | 100 | 0.33M |
| | | 1 | 2-1 3-3 5-5 | UNRESOLVED (π plays 2-1) | 932 | 971 | 39 | 0.58M |
| | | 2 | 3-3 | SETTLED 3-3 (collapse) | 969 | 969 | 0 | 0.66M |
| h12-t4 | 34,650 | 0–2 | all four | EQUIVALENT at 0 (decided root) | 0 | 0 | 0 | 0 |

1. **One focal layer settles two of the three live trick-4 roots under
   the σ0 tail; k = 0 already settles h4-t4.** At h4-t4 the tail is
   worth 964‰ after 6-5 against every other action's God upper (≤ 869‰),
   so the exact-action criterion fires with no search at all — the
   answer to FH-A8(iii)'s tail-quality question at bid 30. h3-t4 settles
   at k = 1 (bar 338‰ over the next upper 328‰). h8-t4 needs k = 2:
   survivors 4 → 3 → 1.
2. **The width at k = 1 is almost all policy gap, not fusion price.**
   Per action, `U − Q` is 0–3‰ at h3/h4-t4 and 0–2‰ at h8-t4 once one
   focal layer is explicit, while `Q − L` is 9–15‰ (h3/h4) and 24–41‰
   (h8). Even at k = 0 the split favors the tail as the problem: h3-t4
   3-1 is 15‰ fusion vs 69‰ policy gap; h8-t4 5-5 is 20‰ vs 131‰. On
   this corpus the glue question is smaller than the better-tail
   question (walt-math W3's two columns answer differently).
3. **The bar's argmax is not the exact argmax — and the regret bound
   knows.** At h8-t4 k = 1, `π_1` plays 2-1 (bar 932‰) while the exact
   best is 3-3 (969‰ vs 956‰); the certified regret 39‰ contains the
   true regret 13‰. §20 as a live specimen.
4. **FH-last and FH-A6 are both mechanical.** At k = 2 every trick-4
   root collapses with tail consultations 100% forced (h4-t4: 216,360 of
   216,360); at k = 3 consultations are exactly 0 and the intervals do
   not move. The forced trick-7 layer costs reads but no value.
5. **Cost.** h4-t4: 5.0M field reads at k = 0, 8.3M at k = 1, 10.0M at
   k = 2 (the exact solve), 17–27 s each; the k-th frontier's ply
   histogram is in the record (k = 0 at h4-t4: 14,533 of 23,812 frontier
   nodes at ply 7 — the viewer mostly plays last in trick 5).

## DEVIATIONS AND FLAGS

1. **The FH3 wording conflict.** The brief's first FH3 ("tail
   consultations = 0, `focal_depth ≤ 6 − T` at k = 6 − T") contradicted
   FH-A6's forced-node convention (`h_f = 7 − T`; consultations = 0
   whenever `k ≥ h_f`). Under the ambiguity protocol a BLOCKED test
   cited both; the brief was corrected against FH-A6 (commit 8efe692)
   and the blocked test deleted. The ruled reading is what FH3 gates.
2. **The full-root `focal_depth` walk is skipped at trick-4 roots** (it
   costs a second exact-size walk per contract); the after-action walks
   are the check there, and the root walk runs at t5/t6 and decided
   roots. The k = h_f run on trick-4 roots is under σ0 only (tail-
   independent by construction).
3. **`scout-corpus` prices `Q_a`** (W3's split) — additive to the brief.
4. **Policy ids embed the tail id** (`focal-k<k>-<tail>-<hex>`), long
   under σ0's content id; deliberate (FH-A4 identity).
5. **Process.** The gate file and record were run in the background
   and the builder was never woken — the FH1 wedge that became
   CLAUDE.md's "Agents" rule. Timeline (CDT, 2026-09-04): 02:06 the
   builder yielded with both jobs running; 02:08 the record finished;
   02:11 the gate suite finished (10 passed); nothing happened until the
   orchestrator's ping at about 07:15 (five hours); commit 07:31.
   Everything after the ping ran foreground.

## FOR FH2/FH3

`Engine::walk` returns `Result<Node, FocalRefusal>` and propagates a
refusal from the single k = 0 frontier site; FH2's retained intervals
and proof-state facts hook there (per-node facts under FH-int's
intersection discipline, a stored policy with every lower). `FocalChoices`
is content-addressed and serializable as-is; `FocalSpend.tail_plies` is
the FH8 ply column. The report-of-record mode belongs in `focalreport`.

---

EXPLORATORY — below every evidentiary tier; quotable only via gate
receipts.
