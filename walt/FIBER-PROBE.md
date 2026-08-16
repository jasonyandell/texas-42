# The fiber-crush probe — design for adjudication

Status: DESIGN, awaiting walt-math rulings. Nothing here is built until the
rulings land in `walt/CENSUS-RULINGS.md` (one-author rule: only walt-math
writes rulings; this file is the orchestrator's design and stays the
orchestrator's).

Tier: exploratory throughout; one declared instrument (the re-weighting, P-Q3
below). Basis: v0.4 (frozen) + v0.5 §12.6A; census rulings F1–F7, r3 Q1–Q5,
railyard Y1–Y3 all inherited unchanged. Scope: pip-trump only, receipt corpus
`rob/receipts/verify_player.txt` hands 0–12, per F1.

## The question

Jason's frame (2026-08-11): raw fiber enumeration is quick at 4 tricks
remaining, untenable at 6, intractable at 7. Raw fiber sizes are
C(3n,n)·C(2n,n): 34,650 (n=4) / 756,756 (n=5) / 17,153,136 (n=6) /
399,072,960 (n=7) — growth ×22–23 per trick. If class-memoized evaluation
makes n=4 manyfold cheaper, n=7 may become merely slow or better. That is the
platform for belief/policy iteration, where countless re-evaluations are the
workload and each must not mean re-enumeration.

Root-class merging is already measured WEAK at the root stratum (647→306 at
t6; 16,112→12,924 at t5, ratio 1.25:1 and weakening). The probe therefore
measures the three mechanisms where the manyfold is hypothesized to live:

  M1 shared interior — one content-addressed DAG under the whole fiber;
     evaluation cost driven by distinct-class count, not world count.
  M2 exact pruning — infeasible branches die at classification.
  M3 amortization — support-level values are weighting-independent; a second
     evaluation under a different weighting is a re-fold over a fixed spine,
     never a re-enumeration.

## The ladder

For each rung n ∈ {4, 5, 6} (tricks remaining), for each selected coordinate:

  (a) RAW baseline: enumerate the fiber; solve every world independently —
      full backward induction on that world's own game tree, NO cross-world
      sharing, no content addressing. Same objective as (b), same move
      generation, same integers. Record wall-clock and total tree nodes
      visited.
  (b) MEMOIZED: same fiber, one shared content-addressed r3-signature cache
      (the census machinery, unchanged freezes). Record wall-clock, distinct
      classes created, cache hits, DAG node count.
  (c) CRUSH FACTORS: worlds / distinct root classes (expected weak) and raw
      tree-nodes / distinct DAG nodes (expected strong — this is M1's
      number).
  (d) SECOND EVALUATION: re-fold the fixed DAG under a different declared
      weighting (P-Q3). Record wall-clock ratio second-eval / first-build.
      This is M3's number.

Extrapolation: fit growth of (a) and (b) across rungs; report both exponents
and the implied n=7 costs, clearly labeled extrapolation, exploratory tier,
never a claim about an unrun computation.

## Design questions for walt-math

P-Q1 (coordinates). Proposal: for each rung n, the coordinate is (focal
seat's actual n-trick hand from receipt hand h, the declaration, the leader at
that trick) taken at the start of trick 8−n, for every receipt hand h ∈ 0–12
and focal = the declaring seat (matching F2's focal convention). The fiber is
ALL assignments of the 3n unseen tiles to the other three seats — the
coordinate deliberately forgets the actual opponents' holdings and the play
history that produced the position, EXCEPT what the coordinate legitimately
carries (tiles gone, whose lead). Is history-forgetting here lawful for a
per-coordinate probe, given r3 classes are futures-only? Any additional
context the coordinate must carry to keep the fiber well-formed (e.g. void
inferences from the actual history are NOT available at this tier — confirm
they are cleanly out of scope rather than silently dropped)?

P-Q2 (the raw baseline's fairness). The comparison is only meaningful if (a)
and (b) compute the same object: the count-free trick-swing value vector over
the transported abstract-policy class (v0.5 BOUNDARY), exact integers, no
floats. Confirm the baseline solver must share move generation and objective
with the census machinery and differ ONLY in the absence of sharing — and
whether a per-world plain minimax (no classes at all) is the honest baseline,
or whether it must also compute class membership to be comparable.

P-Q3 (the re-weighting instrument). Proposal: the second evaluation weights
worlds by a declared deterministic non-uniform integer weighting (e.g. weight
= 1 + number of trumps the world assigns to the focal seat's left opponent,
exact integers). NAME: "support-side re-weighting instrument" — it is NOT a
belief (no belief machinery exists; support ≠ belief is typed). Confirm the
name, the tier (instrument), and that publishing its wall-clock ratio makes no
value claim.

P-Q4 (declared stops). Raw at n=6 is 17.2M solves per coordinate — likely
over the fast-iteration budget (5 minutes per check, Jason's standing
instruction). Proposal: raw runs a declared deterministic PREFIX of the fiber
enumeration order (first W worlds, W declared up front, e.g. 100,000) and
reports per-world cost from that prefix; memoized runs the FULL fiber at every
rung it can within budget, with any stop declared in the results file. Never a
silent cap; every stop printed. Acceptable?

P-Q5 (cache persistence, optional rung). If budget allows: after (b) at rung
n, advance the coordinate by one actual played trick (from the receipt
history) and re-run memoized evaluation WARM vs COLD. Reports the
across-plays cache survival. Optional — cut first if budget forces.

P-Q6 (results discipline). One results file
`walt/walt-factory/results/fiber_probe_2026-08-11.txt`, regenerate line, all
freezes restated, tier labels on every number, extrapolations labeled as
such. New code in walt-skeleton (probe module) + census_run subcommand
`fiber`. Anything else the rulings require?

## What either outcome buys

Strong crush (b ≪ a, d ≪ b): the level-1 seat platform is real — enumeration
paid once per coordinate, beliefs/policies iterate on re-folds. Weak crush:
a proved negative on THIS route to n=7, carried back to the math per
NO-RESCUE — the classes exist regardless; their utility for fiber evaluation
would be the thing measured and found wanting.
