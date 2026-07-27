# Thesis and Scope

## 1. Statement of the object

Texas 42 is modeled as a declaration-indexed physical game and, from each
player's position, as an imperfect-information game over hidden deals and
current hidden remainders.

Its native factorization is:

\[
\text{declaration-indexed physics}
\;+
\text{objective marked world}
\;+
\text{player information}
\;+
\text{exact support fiber}
\;+
\text{belief measure}
\;+
\text{policy model}
\;+
\text{utility}.
\]

These are related mathematical objects. They are not interchangeable names for
one state vector.

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
| Physics | declaration algebra and objective transition | legality, trick order, scoring, conservation |
| Information | private-observation record and public history | what a player has observed and remembers |
| Rule support | compatible complete deals, current remainder fiber, and legal-prefix reachability | which hidden worlds remain possible by rule and which exact support states legal play can generate |
| Belief | probability measure on compatible latent worlds | relative weights from chance and modeled action likelihood |
| Field | behavioral or correlated action law | discretionary action probabilities and required latent continuation state |
| Value | expectation for a named utility and continuation law | derived continuation and action values |

The current remainder fiber is not a probability distribution. Fiber
feasibility is not legal-prefix reachability. Belief is not a physical
coordinate. Policy is not legality. Value is not stored game state.

## 7. Mathematical content

The foundation develops:

1. the finite domino universe and natural incidence covering;
2. declaration as selection of a relational algebra;
3. exact effective-suit, follow, tier, rank, and trick-order relations;
4. a unique-winner theorem;
5. deal, auction, contract, full-play hand, and match objects;
6. full location states and reduced physical Markov states;
7. deal-local and match-global perfect-recall information records;
8. compatible complete-deal support and the current-remainder map;
9. dependent capacity cells and the exact intensional remainder fiber;
10. a proof that the cell representation is lossless for Straight 42 rule
    support within its stated scope;
11. Hall/max-flow feasibility;
12. exact fiber cardinality by a generating-function coefficient, deletion
    recurrence, and bounded native capacity dynamic program;
13. the distinction between local holder allowance and actual marginal holder
    support, with a contractive, idempotent canonical fixed-schema reduction;
14. the globally representation-minimal exact support quotient, its
    determinate/binary/ternary normal form, and exact compiled counting and
    sampling forms;
15. strict Straight reachability as the image of legal prefixes, with exact
    witness certification, reachability-derived capacity and lead-context
    reductions, a feasible-but-unreachable counterexample, and honest standalone
    bit bounds;
16. a proof that support alone selects no probability law;
17. exact holder marginals and a count-ratio sampler after the uniform fiber
    law is explicitly selected;
18. typed support transitions for hidden-player and viewer actions;
19. posterior belief on augmented current-attempt worlds, with inherited
    match information carried by the prior and latent state, and its
    pushforward to the current fiber;
20. Bayesian filtering under public and viewer-private observations;
21. the marked hand embedded in its ambient relational world;
22. strategic sufficiency for an exact state consisting of mechanical/support
    residue, the required retained continuation record, and an augmented belief;
23. an exact legal 90-world counterexample in which identical mechanical
    support and identical posterior support still require opposite leads;
24. exact gauges, analytically scoped transports, and invalid symmetries;
25. monotone rule-support refinement within one fixed deal attempt, stated on
    both complete-deal and typed remainder domains;
26. the finite-DAG basis for exact perfect-information continuation values
    after contract and declaration, once the decision operator and exact value
    representation are named.

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
