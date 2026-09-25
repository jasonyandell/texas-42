# First Contact — rob meets the champion (2026-07-30)

[Field Home](Home.md) · owns: the record of rob's first head-to-head encounters
with E[Q] n=10, the mk5 champion. **Field-measurement tier** (see the Home banner):
nothing here is a receipt or a claim.

> **Headline, stated at exact calibration:** over the full hand, the champion
> defeated rob decisively (≈6.5σ). From rob's exact window onward — identical
> mid-hand positions, both making and defending — the two are **statistically
> indistinguishable**. Together the two results localize the entire full-hand
> deficit to the first tricks, which rob plays with an **exact one-trick window
> and a banked-points leaf** — exact but myopic by budget, not a heuristic
> (see "What the pair of results pins down" and [directions](directions.md)). This is **not
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
tricks 1–2(–3). What rob plays there is not a heuristic: it is **an exact
one-trick window with a banked-points leaf**. Under the normative budget
B = 2²⁸ (BRIEF_PLAYER_01 §7, amended 2026-07-28) the window depth is H = 1 at
trick 1 and H ∈ {1, 2, 3} at trick 2 (the receipt's t=1 histogram is 85/23/0,
`r_pos_schedule`), full depth from the third trick on; within the window every
fiber world is visited, σ is applied exactly, and the frontier leaf is the banked
team points and nothing else (INV-P7 — one unit variant, no tunable term). The
solve is exact; only its horizon is short. This is the *window rent* the brief
registered up front ([rob](../rob.md) §7), characterised by the B/2 and 2B
ablation row rather than tuned. Nothing the mathematics certifies was outplayed.

*Correction of the 2026-07-30 wording (2026-09-12).* This page and
[directions](directions.md) originally called the opening a "convenience
stand-in" and "a fast evaluator jammed into the opening", "slated for
replacement, not tuning". The first phrase misdescribes the mechanism (it is
the counting engine at window 1, exact by construction); the second never came
true — after the 2026-08-17 pivot nobody replaced rob's opening, because
[walt](../walt.md) superseded rob as the player instead (see the sequel below).
The original claims are kept in the version history; the direction they pointed
is mapped at the foot of [directions](directions.md).

## The sequel — the same champion, the same protocol, walt (2026-08-17)

Field tier, like everything on this page, and owned by [walt-seat-play](../walt-seat-play.md);
recorded here so the two encounters are read beside their consequence. Eighteen days after
first contact, `walt_bridge` — speaking the same line protocol as `rob_bridge`, zero arena
changes, ~15k decisions rules-cross-checked with zero divergences — seated the level-1 walt
against E[Q] n=10 under the same dropped-30, 3 × 384-game protocol. **walt 630/1152 games
(54.7 %), McNemar z = +6.28 over 6,015 paired contracts; every seed's mark-margin CI excludes
zero.** The signature was the opposite of rob's Points-lens play: walt loses ~4.7 points per
hand and wins the marks — its pmake objective visible in data. Record:
`walt/probes/m3/arena_results_2026-08-17.txt` (with its own honesty notes: a 4-game pilot that
pointed the other way; one unresolved forensic). Exploratory arena outcome; the wall's
"demonstrated strategic reason" is deliberately not ruled ([lineage](../lineage.md)).

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
- The "~180,000+ decisions, zero divergences" conformance figure is recorded
  only in this area's prose; no log or artifact in this repository pins it, and
  the mk5-side raw results directories were not located in the 2026-09-12 check
  ([Field Home](Home.md), provenance convention).
