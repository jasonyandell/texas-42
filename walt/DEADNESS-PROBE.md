# The decision-deadness probe: cheap one-sided detectors for
# choice-irrelevant branches
# (design for adjudication)

Status: DESIGN, awaiting walt-math rulings (J-Q1..J-Q6) in
`walt/CENSUS-RULINGS.md`. One-author rule unchanged. Standing rulings
inherit everything through PG-A1..PG-A18, Proposition G-flat, Lemma G.
Tier: exploratory.

## Jason's framing (2026-08-12) and the binding constraint

"Junk everywhere is a category of hands... 'I can't beat anything and
nothing I play will change any outcome' — that's a situation we could
eliminate from exploring in fibers if we could detect it very cheaply."
The S6b/policy_inspect specimens ground it: 6 of 7 singleton roots
collapsed by INDIFFERENCE (every free decision value-ties), and deadness
plausibly claims whole subtrees mid-playout wherever it claims a root.

**BINDING (Jason, verbatim intent): count vetoes elimination.** "If you
have count in your hand at all let's not eliminate the branch. Count
changes outcomes a lot, even if we are blind to it at the moment we are
evaluating." Every detector below carries the guard
`hand ∩ COUNT = ∅` (COUNT = the five count tiles) as a conjunct, not an
option. Conjecture for adjudication: the guard is precisely what makes a
deadness verdict sound under the trick-plus-count valuation as well — a
seat that can win nothing captures nothing, and with no count in hand its
sloughs move zero points — so guarded verdicts survive count re-entry
rather than dying by E-A2.

## The object

Decision-deadness at a node (per declared contract and field): every
information-consistent policy from this node has the IDENTICAL value
function on the node's fiber. Consequences when it holds: H solves lose
their max nodes below (pure expectation rollup, no argmax bookkeeping);
states distinguished only by focal options merge; fiber exploration
prunes the focal branching to 1. This is Lemma X's excision shape aimed
at the seat's agency instead of the world set — S5i excised worlds and
found no bite on strong declarer hands; the S6b specimens show the
agency-side bite is large exactly where the world-side bite was absent.

## Evidence that shapes the detector family (from policy_inspect)

The six tie-roots are NOT "can never win a trick" hands: e.g. hand
[2-1, 2-2, 6-3] under 0-trump — the 2-1 lead wins with positive field
probability in many worlds, yet every free decision ties. The tie's
source is ORDER-EXCHANGEABILITY: the free choice at grade 3 is only the
order of the last two tiles, and for these hands the outcome totals are
invariant under the swap. So "no possible winner" (D0 below) is sound but
misses the measured volume; the sharper family (D1) is where the
specimens live. Both are proposed; the probe measures each one's recall
against exact ground truth.

## Design questions

J-Q1 (the definition and the lemma). Define decision-dead(node) per
contract as above. State and prove Lemma J: if a detector in the accepted
family fires at a node, then every information-consistent policy from
that node has the identical value function on the node's fiber, under
(a) the count-free trick valuation, and (b) — via the count guard — the
trick-plus-count valuation. Sub-question for (b): the conjectured
argument is (i) firing implies focal wins no trick in any world along any
continuation, so focal captures nothing; (ii) the guard implies focal's
played tiles carry zero points into anyone's trick; (iii) focal's choices
do not alter any other seat's legal sets or the winner of any trick —
does (iii) actually hold, and does the exchangeability variant D1 need a
different route to (b)?

J-Q2 (the detector family; each one-sided: fires ⟹ dead; UNKNOWN is
always lawful). Proposed candidates, all bitset-level, never a solve
(S5j's lesson: detection cost ate the tablebase's dividend — a detector
that costs a solve saves nothing):
  D0 (no-possible-winner + guard): for every focal tile t and every
      context in which t could be played over the remaining play, a
      live in-context beater of t exists that cannot be exhausted before
      t's last possible play — the exhaustion margin is the part needing
      exact statement; propose the simplest sound counting form.
  D1 (order-exchangeability + guard): at nodes where the focal's
      remaining hand is two tiles (the entire free layer at grade 3 by
      Proposition G-flat), a sufficient condition on (t1, t2, live set,
      contexts) for the swap to be value-invariant. What is the weakest
      cheaply-checkable such condition — and does it decompose as "the
      two tiles never contest the same winnable trick" plus symmetric
      loss, or something else?
  D2 (dead-below-depth): nodewise re-application of D0/D1 during a walk
      — the detector is cheap enough to run at every node, so soundness
      may be claimed node-locally rather than globally.
Which members are accepted, and in what exact form?

J-Q3 (ground truth and the recall measurement). The policy_inspect
tie-classifier is exact: per free state, TIE vs MATTERS by one-deviation
evaluation. Proposed measurements per accepted detector: (a) RECALL —
of the exactly-tied states/roots, what fraction does the detector
certify (its misses are lawful, it is one-sided; the number is its
usefulness); (b) SOUNDNESS RECEIPT — on every fired state, the exact
tie-classification must agree (a single disagreement is stop-and-report:
the lemma or the implementation is wrong); (c) COST — nanoseconds per
call, measured against the E-A9-style closed-form control discipline.

J-Q4 (the harvest measurement). With the detector short-circuiting H
(fired nodes roll up by pure expectation): measure the solve-cost ratio
on the S5h rungs (n = 4, 5 where cold H is the measured baseline of
7–17 s) and on the S6b coordinates. Declared comparison arms in the
P-Q2/three-arm style: H-plain vs H-with-detector, same solver, same
budget unit; the dividend is the ratio and the hit counts. What arms and
rungs are lawful, and is the S5h treatment-H baseline the right control?

J-Q5 (coverage census). Two decimated counts (P-A15 discipline): the
fraction of ROOT coordinates the detectors certify dead, per grade; and
the fraction of TREE NODES certified dead along full pooled walks (the
mid-playout claim — deadness grows as winners leave hands). Stratification
is deliberately NOT proposed (no authored features); the coordinate
population and decimation are the S6a freezes.

J-Q6 (scope, freezes, results discipline). Count-free contract first
with the (b)-soundness claim recorded if proved; grade ≤ 3 plus the S5h
n=4/5 rungs for the harvest arm; new freezes for the detector encodings
and any exhaustion-margin constants, numbering continuing after 31; one
results file `walt/walt-factory/results/deadness_2026-08-12.txt`;
P-A20 lineage; the fence in the R-A23/PG-A17 style, plus: a detector
verdict is never a similarity claim, never a partition, and UNKNOWN is
never evidence of liveness.
