# Thesis and Scope

## 1. Statement of the object

Texas 42 is modeled as a declaration-indexed physical game and, from each
player's position, as an imperfect-information game over hidden deals and
current hidden remainders.

Its exact viewer-relative factorization is:

\[
\text{declaration algebra}
\;+
\text{owned marked hand}
\;+
\text{minimal exact hidden support}
\;+
\text{folded physical play residue}
\;+
\text{utility accumulator}
\;+
\text{retained evidence}
\;+
\text{augmented belief},
\]

with continuation field, utility, and allowed decision strategy supplied as
typed parameters to value. A complete objective world remains the latent
physical witness underneath this viewer-relative object. These are related
mathematical objects, not interchangeable names for one state vector.

> **Thesis.** A physical domino is a stable node, not a stable strategic type.
> Declaration selects the relational world in which that node acts. A hand is
> a controlled marked region embedded in that world and coupled to an exact
> hidden complement. A player's information determines an intensional fiber
> of compatible objective worlds; public history can place unequal weight on
> those worlds without changing the fiber. A move spends and relocates one
> controlled node, transforming both the physical and epistemic situation.
> Its value is therefore derived from that whole transition, not intrinsic to
> the domino.

## 2. Primary formal object

The complete rules profile in this package is **straight points-and-marks
Texas 42**:

- four seats;
- fixed opposite-seat partnerships;
- one double-six domino set;
- seven dominoes per seat;
- one clockwise auction action per seat;
- point bids and straight mark bids;
- a configured finite maximum mark bid;
- seven pip-trump declarations, doubles-trump, and no-trump/follow-me;
- ordinary follow-if-possible play;
- seven complete tricks;
- count points plus one point per trick;
- contract settlement in marks;
- matches to a configured finite target.

This profile is called **Straight 42** below. Claims proved here have Straight
42 scope unless a broader scope is stated explicitly.

## 3. Excluded rules

The following are outside the present formal object:

- nello / Nel-O;
- plunge;
- splash / crash;
- sevens;
- partner sitting out;
- exposed-hand or forced-exposure rules;
- private side signals;
- altered trick objectives;
- hand-content bid eligibility;
- illegal-play adjudication and renege penalties;
- physical tournament procedures that do not alter the abstract legal game.

An excluded contract can change declaration semantics, active-player count,
private information, or the shape of exact support. No theorem in this package
is inherited automatically by such a contract.

## 4. Full hands are primitive

A contracted hand contains all seven tricks and all 28 plays.

A computation may select or stop at states satisfying an explicit predicate.
That operation does not redefine the game's terminal states. A shortened
terminal game requires a separately stated quotient theorem for a named
utility and continuation model.

## 5. Configuration and finiteness

A rules configuration contains at least

\[
\Gamma=(m_{\max},T,\chi_{\mathrm{pass}}),
\]

where:

- \(m_{\max}\in\mathbb N^+\) is the configured maximum mark bid;
- \(T\in\mathbb N^+\) is the match target in marks;
- \(\chi_{\mathrm{pass}}\) is the all-pass rule.

This profile uses reshake/redeal with the successor of the current shaker
becoming shaker after four passes. No numerical cap is privileged by the
mathematics. The customary match target is \(T=7\).

The auction progression itself imposes a sharper reachable ceiling. There are
four one-time bidding actions; the first mark bid is at most two marks; and
each later mark raise is exactly one. Therefore

\[
m_{\mathrm{reachable}}=\min(m_{\max},5).
\]

Every configured cap at least five induces the same legal Straight-auction
tree. A cap of seven is a valid configuration value but makes bids six and
seven unreachable under this profile. Reaching them would require changing an
auction rule, not merely increasing the cap.

One auction attempt is finite because it has exactly four actions and only
finitely many legal actions at each node. This conclusion would remain true
even without a global cap; the present profile nevertheless requires a finite
configured cap. Each contracted hand has 28 plays. Repeated all-pass attempts
are unbounded in number unless a termination assumption or external bound is
added.

## 6. Exact layer boundary

| Layer | Object | Exact content |
|---|---|---|
| Physics | declaration algebra, objective transition, and folded play residue | legality, trick order, scoring transition, conservation |
| Rule support | compatible complete deals, current remainder fiber, minimal support normal form, and legal-prefix reachability | which hidden worlds remain possible and how exact support evolves |
| Information/evidence | private-observation record and public history | what a player has observed and what a selected model may still read |
| Belief | probability measure on compatible augmented latent worlds | relative weights from chance and modeled action likelihood |
| Field | behavioral or correlated action law | discretionary action probabilities and required latent continuation state |
| Utility/value | named utility accumulator and expectation under a continuation law | derived continuation and action values |

The current remainder fiber is not a probability distribution. Fiber
feasibility is not legal-prefix reachability. Belief is not a physical
coordinate. Policy is not legality. Value is not stored game state.

## 7. Mathematical content

The foundation develops:

1. the domino universe as \(\operatorname{Sym}^2(\mathbb P)\), equivalently
   the edges of complete looped \(K_7\), with natural suits as closed stars and
   count as the sum-five/sum-ten antidiagonal decoration;
2. declaration-indexed effective suits, follow, contextual order, and unique
   trick winner;
3. exactly three unscored declaration-mechanics classes—pip trump,
   doubles-trump, and no-trump—plus the narrower scored
   \(2\leftrightarrow3\) transport;
4. deal, auction, contract, full-play hand, and match objects;
5. complete objective states, reduced physical states, and the finite graded
   perfect-information continuation DAG after contract and declaration;
6. deal-local and match-global perfect-recall information records;
7. compatible complete-deal support and the typed current-remainder map;
8. dependent capacity cells and a proof that they generate exact Straight rule
   support within the stated scope;
9. Hall feasibility, exact counting, exact uniform sampling after a law is
   selected, and the distinction between local holder allowance and globally
   realizable holder edges;
10. the globally representation-minimal exact support quotient, its
    determinate/binary/ternary normal form, one-witness SCC compiler, and exact
    compiled forms;
11. strict Straight reachability as the image of legal prefixes, including
    exact deal witnesses, smaller symbolic-trace certificates, reachable
    capacity/context reductions, a feasible-but-unreachable support, and honest
    standalone bit bounds;
12. exact support transition directly on the minimal normal form by
    force/delete/contract/reduce matching operations;
13. monotone holder-edge deletion, downward-only ambiguity phases, and the
    exact 63-edge whole-hand refinement budget;
14. an exact current-trick fold, actor recovery from remaining capacities,
    score conservation, and utility-specific accumulators;
15. a reduced exact viewer play/support kernel that removes duplicated void and
    completed-play provenance from physical state while retaining that history
    separately when a model reads it;
16. posterior belief on augmented latent worlds, exact filtering under public
    and viewer-private observations, and the separation of support from
    probability law;
17. the native hand as an owned marked region in the full ambient relational
    world;
18. strategic sufficiency for physical/support kernel, retained evidence, and
    augmented belief relative to a fixed continuation problem;
19. an exact legal 90-world counterexample in which identical mechanical
    support and identical posterior support still require opposite leads;
20. rotations and orientation-transporting reflections as exact coordinate
    gauges with their boundaries; and
21. future equivalence as the unique globally minimal deterministic exact
    transition quotient for each named finite output contract.

## 8. Typed degrees of freedom

The foundation deliberately leaves the following as parameters rather than
silently selecting one:

- the policy model used to interpret bids and plays;
- required retained continuation-record state, field state, or latent
  correlation structure;
- the posterior induced by a chosen prior and policy model;
- any sampling law placed on a support fiber;
- the utility lens;
- the decision operator applied at each perfect-information actor;
- the exact value representation and effective operators used by a solver;
- exact or approximate computational representations of the same mathematical
  objects, kept in distinct interfaces.

Leaving these typed but unspecified is part of the factorization. It permits
them to vary without changing the rules or the support object.

## 9. Standard of honesty

A theorem states its assumptions. A finite verification states its exact
finite domain. A counterexample refutes only the claim it contradicts. A
representation is called minimal only after a minimality proof. An off-path
posterior is not called unique without an off-path belief rule. A computational
failure does not change the mathematical object.
