# walt-math question — pmake soundness and the walk to trick 1 (2026-08-17)

Status: EXPLORATORY question document. Nothing here is a claim; everything
below the "Frozen inputs" section is asking for rulings. Two questions, sized
for one walt-math session.

## Frozen inputs (context you can trust)

- Carrier: receipt hand 8, trump fives, P30 by T1 (S1+S3), viewer S1. The
  freeze-57 M3 contract (`walt/GPU-NATIVE-TRICK1-M3.md`) froze the trick-4
  boundary: S1 holds [21,31,33,55], 1,200 uniform support worlds.
- Treatment H: S1 plays lawful perfect recall; the other three seats
  (including partner S3) play uniformly at random from their legal sets.
- First-play probe (`walt/walt-m3-probe/src/main.rs`, results committed):
  at trick 4, E[trick-differential] argmax is lead 55, but P(make) argmax is
  lead 33 (16078667/16588800 ≈ 96.93% vs 7770169/8294400 ≈ 93.68% for 55).
  The objectives disagree on the play.
- Jason's ruling 2026-08-17: **pmake is walt's objective.** The game is
  making the bid; trick differential is a proxy and is demoted to diagnostic.
- The ladder (`walt/walt-m3-probe/src/bin/ladder.rs`) re-solves the same
  boundary pmake-only under four exactness-preserving devices (below) and
  reproduces all four frozen trick-4 fractions bit-for-bit, then walks the
  boundary back toward trick 1 on the same hand.

## Question 1 — ratify the pmake pruning devices as exact

The ladder's H-recursion computes, at each information state of S1 (public
record + exact integer posterior over support worlds), the exact P(make).
Four devices claim to preserve exactness. Please prove or refute each, in the
project's own vocabulary, at whatever level of rigor a future Lean row would
want:

- **P1 (decided cutoffs).** Banked points are monotone nondecreasing and the
  hand totals 42. Claim: `banked_T1 >= 30` implies value 1 at every node
  below; `banked_T0 > 12` implies value 0. (Note 42 - 12 = 30 exactly; the
  boundary case banked_T0 = 12 must stay live.)
- **P2 (viewer early exit).** Payoff is an indicator, so node values lie in
  [0,1]; at an S1 max-node a child of value 1 ends the sibling scan. Claim:
  the returned max is still exact (and the root argmax report, which scans
  all leads without early exit, is unaffected).
- **P3 (key reduction under pmake).** Trick-count tallies are dropped from
  the memo key; only (played mask, leader, in-trick plays, banked_T1,
  banked_T0, posterior) remains. Claim: pmake values are a function of this
  reduced state. (Under trick-differential this reduction would be unsound.)
- **P4 (projective posteriors).** The posterior enters the recursion only
  through weight *ratios*: field-move likelihoods rescale all alive worlds by
  integer factors L/k_w, and node values are ratios of weighted sums. Claim:
  dividing the integer weight vector by its gcd (and interning the result, so
  proportional posteriors share one memo identity) changes no value.

Also worth a ruling: the ladder keys the posterior *exactly* (two histories
with the same tile mask but different field-likelihood profiles are different
states). Is there a theorem that under uniform-random field the posterior is
in fact a function of (public record) alone — i.e., is the exact-posterior
key redundant, or are there genuinely distinct posteriors at equal public
records? The intern table suggests the latter (hundreds of thousands of
distinct posteriors at t=3); a characterization would size the state space.

## Question 2 — what dies on the walk to trick 1, and what quotient saves it

Empirical ladder results on hand 8 (pmake only, budgets as noted):

- t=4: 1,200 worlds; solved in ~4s; 8.2M nodes, 139k distinct posteriors.
  Optimal lead 33 at 16078667/16588800.
- t=3: 59,976 worlds (void filter: 756,756 raw → 59,976); DIED — killed at
  ~600s after 300M nodes, 162M memo entries, 4.69M distinct posteriors,
  with the *first* of S1's five lead subtrees still unfinished; memo growth
  stayed linear in nodes with no convergence signal.
- t=2: 7,399,392 void-consistent worlds (one void known: S2 has no trump);
  materialized fine, then DIED at its 120s budget — 159M nodes, 81M memo
  entries, 4.03M distinct posteriors, still inside the root fan.
- t=1: 399,072,960 worlds (no voids yet — the full C(21,7)·C(14,7)
  multinomial); DIED before any solving: support materialization alone
  exceeds the 30M-world cap (and ~6GB of RAM just for the world list).

The recursion that dies is the *posterior-carrying belief recursion*: public
record × exact posterior vector. The question: **what is the correct exact
quotient that collapses it, and is the M1/M2 projector-cell machinery (the
GPU guide's equivariant quotient over constellation cells, §12.6A / ECL
track) already that quotient?** Specifically:

- The support at trick 1 is uniform over a multinomial; the viewer's hand
  partitions the 21 hidden tiles only through walt-core's follow/beat
  structure. Two hidden tiles that no reachable continuation distinguishes
  (same effective incidence pattern against every live context, same count
  value, same rank relations) should be exchangeable — worlds related by
  permuting them should be one class. Is this exchangeability exactly the
  cell structure M1/M2 already computes, or a coarser/finer relation?
- Under pmake (P3 above) the payoff needs only banked totals: does the
  quotient get *coarser* under pmake than under the full-value objective,
  and by how much on the frozen carrier (measurable: 139k posteriors at
  t=4 — how many classes)?
- If the quotient is the right object: what is the predicted class count at
  t=1 on hand 8 — is it CPU-feasible, or is this exactly where the GPU
  milestone (M4+) becomes load-bearing rather than optional?

Deliverable that would help most: a statement of the invariance lemma the
ladder should quotient by (in the §12.6A idiom), plus a back-of-envelope
class-count bound at t=1, so the next code step is "implement the quotient"
rather than "guess one."

## Caveats

Everything above the frozen contract lines is exploratory-tier probe output;
no receipt exists for the ladder numbers; nothing here is a trick-1 statement
under P-A21 until the t=1 line itself is solved and adjudicated.
