# The policy-geometry probe (v0.6 Gate E / Experiment 2): does the
# strategy side collapse?
# (design for adjudication)

Status: DESIGN, awaiting walt-math rulings (PG-Q1..PG-Q6) in
`walt/CENSUS-RULINGS.md`. One-author rule unchanged. Standing rulings
inherit everything through R-A1..R-A24, Lemmas V, X, E, S, S-fold, S-det, R,
Corollaries S-rigid, R-fold. Tier: exploratory.

## Motivation and what this measures

S6a refuted Gate B: dim V^val saturates to |X| by grade 3 — the VALUE side
does not compress. v0.6 named a second, independent favorable possibility
(§8.5, Gate E): the DECISION side may collapse even when the predictive
dimension is full — an enormous information-consistent policy set may
induce few distinct value vectors, and fewer still that are ever optimal
for any belief. Jason's direction (2026-08-12): the hope was never exact
low dimension; it is SIMILARITY OF OUTCOMES — "playing this domino means
I'm likely to get 32 one way or the other." Gate E is the exact,
already-adjudicable fragment of that direction: how many genuinely
different outcome profiles do the lawful strategies produce, and how many
are exposed? Both outcomes are results (F7): collapse = the outcome-
similarity direction has legs at the strategy level; no collapse = the
bottleneck list grows by one named entry.

Scope fence, proposed: same domains, freezes and machinery as S6a (grades
1–3, void-free focal-lead roots, freeze 22 encoding, freeze 25 decimation,
the R-A9 field/belief split, count-free expected-trick valuation, R-A11
observation contract). NO similarity/tolerance claims of any kind in this
probe — "δ-similar" is future mathematics requiring its own typed rulings;
this probe counts exact objects only. Richer outcome laws (score, named
captures — v0.6 §9–10) deferred with their E-A2 count interactions.

## The objects (proposed, for adjudication)

For a root coordinate and root action a, with the fiber X and the uniform
belief:

  N_pol(a)   — the number of lawful deterministic information-consistent
               policies extending a (computable exactly as a product over
               reachable focal information states of |legal|, no
               enumeration: the states are enumerated by the S6a record
               machinery / the info.rs partition).
  N_vec(a)   — the number of DISTINCT value vectors {V_ρ ∈ Q^X}.
  N_par(a)   — the number of Pareto-undominated vectors (per-world ≤,
               dominated vectors can never be exposed for any belief with
               full support — proposed lemma, see PG-Q2).
  N_exp(a)   — the number of exposed vectors: those attaining the maximum
               of E_β[V_ρ] for SOME belief β on the fiber (the upper
               convex hull of the vector set over the belief simplex).

The Gate E comparison is N_pol vs N_vec vs N_par vs N_exp, per action, per
coordinate, per grade — four cardinalities that each license strictly less
than the last, never conflated (E-A8's lesson).

## Design questions

PG-Q1 (the lawful policy set). Confirm: "policy" = deterministic
information-consistent policy per v0.4 §7.2/§10.1 — one choice per
observation record, the same observation contract as the H walk (R-A11),
extending root action a. Mixed policies excluded (the exposed set of the
mixed hull equals that of the deterministic set — may this be asserted, or
must it be proved at adjudication?).

PG-Q2 (dominance pruning is lawful — the lemma this probe needs). Proposed:
a vector per-world-dominated by another (V_ρ ≤ V_ρ' pointwise, somewhere
strict) is never uniquely exposed under any belief with full support, so
backward Pareto pruning preserves N_exp and every exposed vector.
Questions: (a) state and prove the exact preservation claim (ties and
faces need care: equal-on-a-face vectors and beliefs without full
support); (b) is per-interface backward pruning lawful — i.e. does
Pareto-domination at a successor interface imply domination of every
composition through (POL), so pruning inside the recursion never removes
an exposed root vector? This is the feasibility linchpin: without
backward pruning the policy set is astronomically large; with it the run
maintains only undominated sets per interface.

PG-Q3 (the exposure computation). N_exp is a convex-hull question in
dimension |X| (up to 1,680) — exact hull enumeration is infeasible there.
Proposed arms, each exact, each with a declared stop: (arm 1) N_vec and
N_par exactly at every grade (set-dedup and pairwise dominance are
O(n²·|X|) rational comparisons); (arm 2) N_exp exactly at grade ≤ 2 only
if a lawful method is adjudicated (LP-style exposure testing per vector
uses exact rational pivoting — no floats — but the method must be named
and frozen); (arm 3) at grade 3, N_exp is NOT computed — reported as "not
measured, method infeasible at this dimension," never approximated.
Confirm the arms and the stop discipline (P-A16).

PG-Q4 (what the uniform-belief argmax adds). The point belief already used
by the R-A18 gate exposes exactly the H-optimal vectors. Proposed
diagnostic: per coordinate, the count of DISTINCT root actions that are
H-optimal, and the count of policies attaining the optimum — the
argmax-partition seed measurement (the dropped-30 direction). Confirm this
is lawful to report alongside without a partition claim.

PG-Q5 (feasibility declarations). Per-interface undominated sets can still
grow. Declared budget: a per-coordinate cap on the running undominated-set
size (number to be frozen), exceed = declared stop with the reached counts
printed (never silent, never sampled). Grade order 1 → 2 → 3 with the
grade-3 run conditional on grade-2 set sizes (declared in advance, printed
either way). New freezes needed: the vector encoding and dedup order; the
dominance-check order; the exposure method if adjudicated; the caps.
Numbering continues after 26.

PG-Q6 (results discipline). One file
`walt/walt-factory/results/policy_geometry_2026-08-12.txt`; P-A20 lineage;
integers first; the four cardinalities never conflated; the R-A23-style
fence restated with the additions: no similarity claim, no tolerance
claim, no partition claim, N_exp absence at grade 3 stated as a method
limit, not a finding about the game. Pre-declared reading criterion
(Y2 Q2): the probe REFUTES strategy-side collapse if N_par grows with the
policy set at the same order across grades; it CONFIRMS collapse if N_par
(and N_exp where measured) stays orders below N_pol and its growth ratio
across grades is materially below the |X| growth ratio — thresholds to be
fixed at adjudication before any number exists.
