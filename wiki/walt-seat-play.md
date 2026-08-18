# walt — the seat plays: the scenario-player era

[Home](Home.md) · owns: the scenario-player line of the `walt/` build
(2026-08-17 onward) — the sampling-stack seat, its first games, the arena
match against the E[Q] champion, the level-2 stack, and divergence mining ·
Sources: [`walt/SCENARIO-PLAYER.md`](../walt/SCENARIO-PLAYER.md) (the spec),
result files under [`walt/walt-m3-probe/`](../walt/walt-m3-probe/). Related:
[walt](walt.md) (hub and fence), [walt-program](walt-program.md),
[walt-instruments](walt-instruments.md), [rob](rob.md).

> **Epistemic tier: EXPLORATORY — the hub's fence applies unchanged.** Every
> number on this page is a sampled ESTIMATE or an arena outcome; arena
> outcomes are external receipts about play, never statements about exact
> values; nothing here is quotable above the Ideas tier. The spec's
> obligations ledger (`SCENARIO-PLAYER.md` §10) is the graduation path.

## What happened

After the 2026-08-10 reset froze the m4/S6 compression search, the program
pivoted to Jason's standing direction ([walt-program](walt-program.md)): stop
compressing truth, **build the seat that plays**. In two days
(2026-08-17/18) the sampling-stack player went from first lawful hand to:

- **A playing seat.** Level-1 walt: exact best response over a sampled
  belief (void-conditioned fiber worlds) against a field of modeled level-0
  minds bottoming out in dice. Objective: P(make the bid) — Boolean, exact
  rationals, no floats. Playable at a web table (`webtable.rs`, dominoes,
  clockwise, trump deliberation visible); Jason has played it at length.
  His verdict after a sacrifice he first read as a blunder: "that's a good
  player buddy. a good player."
- **The match.** Via a bridge speaking the rob_bridge protocol (zero arena
  changes; ~15k decisions rules-cross-checked, zero divergences), level-1
  walt played the standing mk5 champion — the E[Q] n=10 lens that had
  beaten rob 6.5σ — under the same dropped-30, 3×384-game protocol.
  **Pooled: walt 630/1152 games (54.7%), McNemar z = +6.28 over 6,015
  paired contracts; every seed's mark-margin CI excludes zero.** The
  signature: walt loses ~4.7 points/hand and wins the marks — the pmake
  objective visible in data. Full numbers, parity audits, and honesty notes
  (a 4-game pilot that pointed the opposite way; one unresolved forensic):
  `arena_results_2026-08-17.txt`.
- **Level-2.** The field model is a parameter: a level-k mind best-responds
  to level-(k−1) minds. Level-2 — the first level whose modeled partner
  coordinates back — was laddered on the frozen carrier: it agrees with
  level-1 at every rung, and its opening lead (5-5, pulling trumps with the
  master) is unique at every sample size that separates ties, perfect at
  n=3200. Cost ≈ 25–50× level-1 per decision. `level2_results_2026-08-17.txt`.
- **The correction the parallel port caught.** Making level-2 parallel
  (≈5.6×, byte-identical across thread counts) exposed a purity bug: the
  modeled-mind cache key omitted the banked totals its objective conditions
  on. Serial runs had masked it as deterministic first-come aliasing. Fixed
  everywhere (post-match, so the 3×384 pool stays internally consistent on
  the pre-fix binary; post-fix play is a new baseline). The incident is the
  standing argument for the spec: a stated invariant, checked at review,
  catches drift that testing hides. `SCENARIO-PLAYER.md` §3.4.
- **Divergence mining.** Rather than a head-to-head to detect where
  partnership modeling matters, 900 self-played hands with a level-2 shadow
  at one seat: 4,156 shadowed decisions, divergence concentrated in tricks
  1–4 and — at large value gaps — in the partner-bid and defense regimes
  (≈2× the self-bid rate at ≥1500bp), matching the prediction that level-2
  bites where "my partner will X" carries the decision. Top mined case:
  level-2 drops a five-count on the partner's winning trump pull where
  level-1 hoards it. Self-graded (level-2's own table); refereeing is an
  open design question. `divergence_results_2026-08-18.txt`, corpus under
  `mined/`.

## Where this sits in the program

The seat is real and it wins — and every one of its numbers is a sampled
estimate against a *modeled* field, not an exact value and not an
equilibrium claim (`SCENARIO-PLAYER.md` §7). The program's next moves, per
Jason 2026-08-18: pay down the spec debt (the obligations ledger), baseline
bidding (the bid level already parameterizes the solver), the webgame
(level-1 compiles to WASM-shaped Rust; phone-viable timings), then variants.
[rob](rob.md) remains the exact-truth solver; walt remains the seat; neither
impersonates the other.
