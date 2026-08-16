# The predictive-rank probe, part one: macro-kernel certification and the
# outcome-rank census at grades 1–3
# (design for adjudication)

Status: DESIGN, awaiting walt-math rulings (R-Q1..R-Q7) in
`walt/CENSUS-RULINGS.md`. One-author rule unchanged. Standing rulings inherit:
F1–F7, r3 Q1–Q5, Y1–Y3, P-A1..P-A21, X-A1..X-A19, E-A1..E-A21, S-A1..S-A21,
Lemmas V, X, E, S, S-fold, S-det, Corollary S-rigid. Tier: exploratory.

New primary source: `walt/math/predictive_algebra_v0.6.md` (the v0.6 track,
filed 2026-08-12, Jason's authorization). Its internal theorems (rank
minimality §5.3, residual span §6.3, filtering §7.4, optimality §8.4,
outcome-algebra lift §9.2, monitor preservation §10.6, equivariant
representative independence §12.3) are design guidance at exploratory tier;
nothing here promotes them.

## Why this probe, and why now

Corollary S-rigid closed the partition route at the first play: the seat-side
structural quotient is the identity, COUNT 1 = C(28,7) = 1,184,040, because
structural compression is bought with deadness and nothing is dead at the
first play. v0.6 §5.4 states the escape hierarchy:

  linear rank ≤ positive realization size ≤ partition-lump size ≤ |X|.

Every behavioral row may be distinct (what S-rigid proved at the top) while
the rank of the continuation matrix stays small. The bar question moves from
"how many opening hands are isomorphic?" (answered: all distinct) to "what is
the exact controlled outcome rank of the opening decision problem?" (v0.6 §0).

Part one is deliberately the smallest decisive measurement: v0.6
Experiments 0 and 1 only, on suffix domains we can already enumerate, with
the trick-count contract first. Gate B (is the future low-rank?) gets a
direct number at grades 1–3 before anything else is built. Both outcomes are
results (F7): slow rank growth = the architecture lives; fast rank growth =
a named bottleneck carried back to the math, better informed.

Out of scope for part one (declared, not silent): the score contract
(count re-entry, E-A2 interactions), Scheme-basis synthesis (Experiment 3),
moment compilation (Experiment 5 / Gate D), policy-dual geometry
(Experiment 2), monitors (control return — rides in part two), nonfocal-lead
interfaces (R-Q2c), and any grade above 3.

## Design questions

R-Q1 (filing, tier, vocabulary). (a) Confirm the filing:
`walt/math/predictive_algebra_v0.6.md` sits alongside v0.4 (frozen basis) and
v0.5 (§12.6A track) as the v0.6 track, exploratory tier, consumed as design
guidance under our discipline. (b) Vocabulary ruling needed: v0.6 says
"certificate" throughout for machine-checkable finite receipts (§16). The
project bans "certificate" in the reachability sense (D3: "necessary outer
profile"; reachability is proof-irrelevant, no identity-bearing witnesses).
These are different senses — rule on the local term. Proposal: walt files say
"receipt" wherever v0.6 says "certificate," and the reachability fence is
restated in the rulings so the two senses can never blur. (c) v0.6 introduces
"interface," "local controller," "continuation test," "predictive rank,"
"closure matrix" — confirm these enter walt vocabulary as-is or rename.

R-Q2 (interfaces and the measured domain). Proposed typed interface for part
one: (declaration; focal = declarer; focal remaining hand; leader = focal;
voids = ∅; grade n) — the void-free capacity kernel construction of P-A2/P-A4
reused verbatim, so the fiber over an interface is the exact capacity fiber
(6 / 90 / 1,680 worlds at n = 1 / 2 / 3). Questions: (a) is this tuple the
lawful instance of v0.6 §3.1's typed focal information interface for the
void-free case, and is the fiber the correct X_i? (b) coordinate selection:
propose deterministic decimation in the established style (P-A9) over
(declaration × focal-hand) coordinates at each grade — how many coordinates
per grade constitute the census claim, and what does per-coordinate
variation license? (c) focal-lead only: at focal-lead boundaries the
controller alphabet is the ≤ n led tiles (v0.6 §3.2's restriction case);
at nonfocal-lead boundaries controllers are maps from observed prefixes to
plays and the alphabet can blow up. Confirm deferral of nonfocal-lead
interfaces is a lawful scope fence for part one and state the fence sentence.

R-Q3 (the declared continuation operator). Proposal: the fixed stochastic
field for nonfocal seats = uniform over legal plays; focal seat controlled.
This is exactly the operator the existing m3 solver evaluates (max at focal
turns, uniform mean at nonfocal turns — P-A6's aggregate), so the concrete
side of every end-to-end check is the solver we already trust. Confirm:
(a) this is the declared field of v0.6 §3 for part one; (b) the concrete
ground truth for V/Q checks is the m3 dag-v1 solver under this aggregate;
(c) any belief statement in part one is the declared uniform field only —
belief adapters are out of scope.

R-Q4 (Experiment 0 — macro-kernel certification, focal-lead). Compile the
one-trick macro-kernel K_{i,u} per (interface, led tile): enumerate every
primitive four-play branch under the field; canonicalize each completed
trick; record (γ, r, o, successor interface) with exact rational weight.
Checks (v0.6 Experiment 0): total mass 1 per (i, u, ξ); exact equality of
primitive and folded path laws per successor bucket. The load-bearing
recorded result: does γ alone determine the normalized operator, or is a
typed boundary tag required (v0.6 §3.4)? Questions: (a) which label plays γ —
the 64-class level-1 alphabet, or the finer r1 canonical form (E-A8's two
cardinalities)? Proposal: 64-class as the alphabet, with the finer form
recorded as a diagnostic column, since v0.6 §3.4 predicts the alphabet alone
under-determines the operator and the experiment should measure the gap.
(b) the observation contract o for part one: propose o = the full public
trick (all four plays visible), so the player can reconstruct γ; state the
analyst/player firewall sentence (v0.6 §3.3, §18.6) that keeps r and any
analyst-only labels out of policy branching anyway. (c) the increment
alphabet for part one: count-free — focal-team trick taken ∈ {0,1} — confirm.

R-Q5 (Experiment 1 — the rank census). Residual closure per v0.6 §6: at
terminal interfaces V = span{1, terminal readouts}; backward, adjoin every
residual Pre_{i,u,e}(f) for legal u, typed event e, successor basis f; exact
row reduction over the fiber domain; r_i = dim V_i. Contracts in
order: (i) expected focal-team tricks; (ii) the full focal trick-count
distribution; (iii) (ii) plus the next-leader-offset predicate (the control
alphabet the monitor will need in part two). Report per interface and grade:
|X|; the number of distinct behavioral rows (computable exactly at these
sizes); the partition-lump size; the rank r; basis and closure-matrix
sparsity. The Gate-B headline is the three-way comparison
|X| vs #behavioral rows vs r. Questions: (a) confirm the contract ladder and
that the score contract is lawfully deferred; (b) confirm the report schema
and that behavioral rows are computed from exact test evaluation (not
sampled); (c) end-to-end check: for each measured coordinate, the predictive
V/Q from the closure module must equal the m3 solver's concrete V/Q exactly
(v0.6 §16.7's equality block, receipt-style) — confirm this is the
correctness gate for part one; (d) relation to the existing class machinery:
proposal — clean slate; the r3/class store does not participate (v0.6 §0:
the 64 classes are a transition alphabet here, not a state partition), and
S5h's negative (the class DAG is never a build accelerator) stands untouched.

R-Q6 (arithmetic and freezes). Exact row reduction over long products will
overflow Ratio<i128>. Proposal: the closure module uses arbitrary-precision
rationals (num-bigint BigRational) — still exact, no floats, the clippy/grep
rules untouched; boundary conversions to walt_geom::Q only where a value
crosses into existing machinery, with overflow = stop-and-report, never
truncate. New freezes proposed: (18) the interface encoding and enumeration
order; (19) the basis discipline — deterministic pivot selection (first
nonzero in declared enumeration order), basis vectors stored in declared
order, f_0 = 1 always index 0; (20) the event-label encoding (γ, r, o);
(21) the decimation constants per grade. Numbering: S-A19 proposed 18–21 for
the parked seat-census receipt build — rule on the collision (that build is
parked; may these numbers be reassigned, or do we continue from 22?).

R-Q7 (results discipline and the claim fence). One results file
`walt/walt-factory/results/predictive_rank_2026-08-12.txt`; P-A20 lineage
boilerplate; integers first; per-coordinate tables then per-grade summary;
both-outcomes framing stated in the header. The fence, proposed: rank
numbers license NO runtime claims (Gate D — moment compilation — is a
separate unmeasured experiment); NO promotion of any v0.6 theorem; results
are carrier-relative to the measured coordinates and the declared field;
the concrete solver remains the authority wherever the two disagree, and a
disagreement is a stop-and-report bug, never reconciled by adjustment.
Confirm the fence sentence.

## Implementation note (post-rulings)

New example `walt/walt-factory/examples/predictive_rank.rs` (its own runner —
this probe shares no arms with fiber_probe). Build begins only after
R-A rulings land. Gate: `walt/ci/check.sh` green before the probe is called
done, receipts style per R-Q7.

## Addendum (2026-08-12, post-adjudication): the declared Gate-B criterion

Per R-A20 (Y2's Q2 discipline), fixed here before any number exists. The
measured object is dim V^val (Lemma R(3)) at root information interfaces,
per coordinate, at grades n = 1, 2, 3. Statistics per grade: the full
integer multiset with min / median / max; no mean (R-A6). Let D(n) denote
the per-grade MAX of dim V^val over the declared coordinates, and let
X(n) = |X| at grade n (6 / 90 / 1,680), so the fiber growth ratios are
X(2)/X(1) = 15 and X(3)/X(2) = 56/3 ≈ 18.67.

- **Payoff CONFIRMED** if, at BOTH steps n=1→2 and n=2→3,
  D(n+1)/D(n) ≤ (1/3) · X(n+1)/X(n) — i.e. the dimension growth ratio is at
  most one third of the fiber growth ratio (equivalently ≤ 5 and ≤ 56/9).
- **Payoff REFUTED** if, at EITHER step, D(n+1)/D(n) ≥ (2/3) · X(n+1)/X(n)
  — dimension growth of the same order as fiber growth.
- Anything between is **UNRESOLVED** and is reported as exactly that.

Ratios are exact rationals over the integer maxima; a grade whose
correctness gate (R-A18) is unmet contributes no ratio and forces
UNRESOLVED at its steps. P-A21 governs everything beyond: three rungs are
not a law, no implied grade-7 dimension exists, and no number here is
quoted for the opening.
