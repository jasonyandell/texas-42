# First Contact — rob meets the champion (2026-07-30)

[Field Home](Home.md) · owns: the record of rob's first head-to-head encounters
with E[Q] n=10, the mk5 champion. **Field-measurement tier** (see the Home banner):
nothing here is a receipt or a claim.

> **Headline, stated at exact calibration:** over the full hand, the champion
> defeated rob decisively (≈6.5σ). From rob's exact window onward — identical
> mid-hand positions, both making and defending — the two are **statistically
> indistinguishable**. Together the two results localize the entire full-hand
> deficit to the first tricks, which rob currently plays with scaffolding that
> was never intended to survive (see [directions](directions.md)). This is **not
> a wall-clearing** — [lineage](../lineage.md) requires *beating* the champion
> for a demonstrated strategic reason. It is first contact, and the exact solver
> did not lose it.

## The seating

rob played via `rob_bridge` (texas-42 commit `6f7d346`): a dependency-free
subprocess line protocol of plain integers, an 8-worker pool inside mk5's arena
(`arena/rob_play.py`, mk5 worktree branch `rob-vs-eq`). Every bridge reply carries
rob's independently derived trick leader and team points, asserted against the mk5
engine's every decision — so the entire campaign doubled as a rules-conformance
cross-check between the two independent rules implementations. **~180,000+
decisions, zero divergences.** (Cross-check evidence, not a receipt row.)

Opponent: `LensPlay(utility="ev", n_samples=10)` — E[Q] n=10, the undefeated
champion ([lineage](../lineage.md)), verified at source to be the original
E[Q] mathematics (mean of oracle Q over 10 sampled consistent worlds, argmax),
vectorized. rob played its exact-fiber Points lens, bid- and score-blind.

## Encounter 1 — full hand ("dropped-30"): the champion wins, ≈6.5σ

Protocol: no auction — every hand forced to a 30 bid, declaration = best pip
trump of the forced bidder's hand; deterministic from the deal, so every deal is
played twice with teams swapped (mirrored pairs); 1 mark per hand; games race to
7 marks. Three seeds × 384 games = 1,152 games, 12,866 hands.

- rob won **525/1152 games (45.6%)**, ≈ −0.38 marks/game, ≈ −0.85 pts/hand.
- Paired on 6,028 mirrored deal-pairs: rob made **32.4%** of contracts, E[Q]
  made **36.1%** *of the same deals*. Discordant pairs 459 (rob-only) vs 679
  (E[Q]-only); McNemar z = **−6.52**. Make-rate edge −3.65pp [−4.75, −2.55].
- Negative in **all seven declarations**, and on raw points — rob's own lens
  objective. Per-seed marks/game: −0.27, −0.57 (CI excludes zero), −0.30.
- A 24-game probe the previous day (+0.46 rob) was noise with the wrong sign.

Artifacts (mk5 side, local): `arena/results/dropped30_384{,_s2,_s3}/`,
writeup `arena/evidence/DROPPED30_RESULTS.md` (mk5 commit `3193295f`).

## Encounter 2 — mid-hand takeover: dead heat from the exact window

Same dropped-30 hands; a fast deterministic heuristic (JudPlay, argmax over a
small value net, no oracle) plays **all four seats** through the prefix; the
position is frozen and played out twice from the byte-identical state — rob's
team holding the contract vs rob's team defending, E[Q] in the other seats.
Paired offense-vs-offense on identical positions. 256 positions per takeover
trick (plus the original 16-position run at trick 4: **16/16 concordant**).

| takeover | rob offense made | E[Q] offense made | discordant (rob/eq) | offense pts/hand |
|---|---|---|---|---|
| trick 3 | 93/256 | 91/256 | 16 / 14 | 23.6 / 23.7 |
| trick 4 | 84/256 | 87/256 | 10 / 13 | 22.8 / 23.2 |
| trick 5 | 89/256 | 91/256 | 4 / 6 | 22.8 / 22.8 |

Every row |z| ≤ 0.6; pooled make-rate gap across 768 pairs **−0.4pp ± 1.7pp**;
points agree to tenths. Contrast encounter 1's −3.65pp at 6.5σ.

Artifacts (mk5 side, local): driver `arena/midhand_eval.py` (mk5 commit
`2d7375bc`), results `arena/results/midhand_t{3,4,5}_256/`, writeup
`arena/evidence/MIDHAND_RESULTS.md` (mk5 commit `594ee5e9`).

## What the pair of results pins down

The full-hand deficit does **not** come from the exact solve: from trick 3
onward, best-response-over-the-exact-fiber matches the champion on identical
positions, on offense and defense alike. The deficit accumulates entirely in
tricks 1–2(–3) — which rob currently plays with a convenience stand-in (a fast
evaluator jammed into the opening to make full games tractable) that is
explicitly **not** part of rob-proper and is slated for replacement, not tuning
([directions](directions.md)). Nothing the mathematics certifies was outplayed.

## Caveats, all of them

- Dropped-30 is a forced-contract, random-deal distribution with pip trumps
  only; realistic auctions select very different contracts.
- rob played the Points lens at window budget B=2^28 with counting-engine
  overflow routing; ContractSuccess lens untested.
- Mid-hand positions descend from a shared heuristic prefix; in the full-hand
  games each player steers its own opening, so trajectories diverge by trick 4.
  The mid-hand rig measures skill from common positions; compounding along
  divergent early trajectories is not separated out.
- E[Q] is a strong baseline, not optimality: a dead heat bounds neither player's
  distance from optimal. "Exact" here means exact best response *given rob's
  opponent/evaluation model* — the encounters measure that model as much as the
  solve.
- All numbers computed by mk5 arena code, not by certified Rust. Field tier.
