---
number: TBD (unassigned — draft)
slug: l2-coupling-theorems
channel: new-chat
status: "DRAFT — NOT DISPATCHED. Authorization: none. Batch quota: TBD with Jason."
deliverable: a finite-game counterexample to any of L2-T1..T5 (explicit tables, exact rationals), or step-certification with a model-checking program over a declared family of small games
---
STATUS: DRAFT — NOT DISPATCHED. Authorization: none. Batch quota: TBD with Jason.

You are performing adversarial mathematical review for a games-mathematics
project. You see ONLY this text: no repository, no prior conversation, no
outside sources. Everything you need is defined below. The claims under review
are the project's own **exploratory-tier working mathematics** — five short
coupling/stability theorems the project intends to lean on heavily. Short
proofs deserve the hardest push. Your response will be adjudicated mechanically
by reviewers holding the full corpus: programs executed, witnesses re-run,
proofs step-checked. Hedged or unverifiable claims score zero.

## 1. Setting (complete and self-contained)

A finite, perfect-recall, deterministic-transition extensive-form game with
one distinguished **focal seat** and one or more **non-focal seats**. A
**world** `ω` is the hidden initial state (e.g. the deal of hands); it is
drawn from a finite set with rational probabilities `β` ("the belief"). Given
ω and one action-choosing function per seat, play is fully deterministic and
terminates; the focal seat receives a Boolean payoff.

**Information states.** At each decision point, a seat sees exactly: its own
private component of ω, and the public record of all actions played so far
(perfect public monitoring of actions; no seat sees another's private
component). A focal information state is denoted `I`; a non-focal information
state is denoted `J`.

**Policies.** An information-consistent focal policy `ρ` is a function from
focal information states to legal actions. For a fixed legal root action `a`,
`Π_a` is the (finite, nonempty) set of information-consistent focal policies
whose action at the root information state is `a`. A **field** `σ` assigns to
every non-focal information state `J` a legal action; fields are
deterministic functions of `J` only. Two fields `σ_0`, `σ_1` are fixed.

**Values.** For physical world ω, let `u_f(ρ, ω) ∈ {0,1}` be the terminal
payoff when the focal seat follows ρ and all non-focal seats follow `σ_f`
(`f ∈ {0,1}`). Define

    V_f(ρ)   = E_{ω∼β}[ u_f(ρ, ω) ]                (fixed-policy value)
    Q_a^(f)  = sup_{ρ∈Π_a} V_f(ρ)                  (optimized root-action value)

(the sup is a max over the finite set; sup is kept for proof neutrality).

**Frontier and coupling.** The field-disagreement frontier is

    F_{0,1} = { J : σ_0(J) ≠ σ_1(J) }.

For one fixed `(ρ, ω)` run two executions from the same root: execution 0
under `(ρ, σ_0)` and execution 1 under `(ρ, σ_1)`. Define `D_ρ(ω) = 1` if the
coupled execution reaches a non-focal information state in `F_{0,1}` before
termination (in either execution, along the shared prefix), else 0 — the
**field-exposure event**. Write `d_ρ = Pr_{ω∼β}(D_ρ = 1)` and

    R_a = sup_{ρ∈Π_a} d_ρ                          (maximum field exposure).

## 2. The five claims under attack

### L2-T1 — first-disagreement localization

For every information-consistent ρ and every ω:
`D_ρ(ω) = 0 ⟹ u_0(ρ,ω) = u_1(ρ,ω)`; equivalently
`|u_1(ρ,ω) − u_0(ρ,ω)| ≤ D_ρ(ω)`.

*Claimed proof.* Induct on play steps. While public histories agree: the
focal seat has the same information state in both executions, so
information consistency forces the same action; a non-focal seat outside
`F_{0,1}` chooses the same action under both fields; transitions are
deterministic. If no frontier state is reached the induction runs to
terminal, giving equal payoffs. ∎

Corollaries claimed: `|V_1(ρ) − V_0(ρ)| ≤ E[|C_ρ|] ≤ d_ρ` where
`C_ρ(ω) = u_1(ρ,ω) − u_0(ρ,ω)`; and for two frozen policies,
`|Λ_{a,b}| ≤ d_{ρ_a} + d_{ρ_b}` where
`Λ_{a,b} = (V_1(ρ_a)−V_1(ρ_b)) − (V_0(ρ_a)−V_0(ρ_b))`.

### L2-T2 — root-action field Lipschitz bound

For every root action `a`:  `|Q_a^(1) − Q_a^(0)| ≤ R_a`.

*Claimed proof (the sup / optimization-lock step — push here).* For every
`ρ ∈ Π_a`, L2-T1 gives `V_1(ρ) ≤ V_0(ρ) + Pr(D_ρ=1) ≤ V_0(ρ) + R_a`. Taking
the supremum over `ρ ∈ Π_a`: `Q_a^(1) ≤ Q_a^(0) + R_a`. Interchanging fields
0 and 1 gives the reverse. Combine. ∎

Note what makes this delicate: the maximizers of `V_1` and `V_0` over `Π_a`
may be DIFFERENT policies, and `R_a` is itself a sup over the same index set.
The claimed inequality bounds a difference of suprema by a sup of
differences' bounds.

### L2-T3 — root winner stability

If `a` is optimal under `σ_0` and for every rival `b`:
`Q_a^(0) − Q_b^(0) > R_a + R_b`, then `a` is strictly optimal under `σ_1`.

*Claimed proof.* `Q_a^(1) ≥ Q_a^(0) − R_a > Q_b^(0) + R_b ≥ Q_b^(1)`. ∎

### L2-T4 — safe action screening (the bar argument — push here)

Suppose valid bounds are given: `Q_a^(0) ∈ [L_a^(0), U_a^(0)]` and
`R_a ≤ R_a^U` for every legal root action `a`. Define

    L_a^(1) = L_a^(0) − R_a^U,     U_a^(1) = U_a^(0) + R_a^U,
    B = max_a L_a^(1),
    A_1 = { a : U_a^(1) ≥ B }      (the admissible set).

**Claim:** every action excluded from `A_1` is incapable of being optimal
under `σ_1`, on the event that all input bounds are valid.

*Claimed proof.* If `a ∉ A_1` then `Q_a^(1) ≤ U_a^(1) < B`. Some action `c`
attains `L_c^(1) = B`, so `Q_c^(1) ≥ B > Q_a^(1)`; thus `a` is not optimal. ∎

Also claimed: `A_1` is never empty; loose bounds can only ADMIT too many
actions, never exclude a truly optimal one; if `|A_1| = 1` the root is
field-stable with no further work.

### L2-T5 — eventual periodicity

Let `S` be a finite set of deterministic field-policy profiles and
`B: S → S` any deterministic map ("best-response construction"). Then
`σ_{k+1} = B(σ_k)` is eventually periodic.

*Claimed proof.* Pigeonhole gives `σ_i = σ_j` for some `i < j`; determinism
propagates `σ_{i+t} = σ_{j+t}` for all `t ≥ 0`. ∎  (The period may be 1 but
nothing forces it.) The intended USE is typed narrowly: a "level-2" result is
a best response to the fixed field `σ_1` only — never a claim of
equilibrium, convergence, or monotone improvement across the tower.

## 3. THE TASK — refute if you can

Full credit for any ONE of the following with the required artifact; full
credit for certification of all five with load-bearing steps named.

(A) **Counterexample to L2-T1 or its corollaries.** An explicit finite game
(tables: worlds with rational probabilities, per-seat information partitions,
legal actions, deterministic transitions, Boolean payoffs), a policy ρ, two
fields, and a world with `D_ρ(ω) = 0` yet different payoffs. The induction's
soft spots to probe: does `D_ρ` as DEFINED (frontier reached "along the
shared prefix") actually cover the case where the executions diverge for a
reason other than a frontier state — e.g. can the focal seat's information
states differ between executions BEFORE any frontier state is reached, under
some reading of the coupling? If the definition of `D_ρ` is doing silent work
(it is defined on the coupled execution, which is only well-defined while
histories agree), state precisely what the airtight definition must be, and
whether the theorem survives it.

(B) **Counterexample to the L2-T2 sup step.** A finite instance where
`|Q_a^(1) − Q_a^(0)| > R_a`. Since the pointwise step looks tight, the attack
surface is the interaction of the three sups (`Q^(0)`, `Q^(1)`, `R_a`) over
`Π_a`: verify by brute force over small games whether a gap is possible; if
your search over the declared family (see §4) finds none, either promote the
search into a proof (the two-line argument above, certified step by step —
including that sup of a sum ≤ sum of sups is applied in the valid direction)
or exhibit the exact hypothesis (finiteness? shared index set? validity of
the same `R_a` for both directions?) without which it fails.

(C) **Counterexample to L2-T4.** An instance with valid input bounds where an
action excluded from `A_1` IS optimal under `σ_1`; or where `A_1 = ∅`; or
show the nonemptiness claim requires `L_a^(0) ≤ U_a^(0)` and `R_a^U ≥ 0`
(state the minimal hypotheses; note `U_c^(1) ≥ L_c^(1)` for the argmax `c`
puts `c ∈ A_1`). Probe the strictness bookkeeping: the exclusion uses
`U_a^(1) < B` (strict), membership uses `≥` — certify there is no boundary
case where an optimal action sits at `U_a^(1) = B` and is wrongly excluded,
and none where ties at `B` break the "incapable of being optimal" phrasing
(optimal means: attains `max_b Q_b^(1)`).

(D) **Attack L2-T3 / L2-T5 or the typing.** For T3: the strict/non-strict
inequality chain, and whether "optimal under σ_0" is even needed as a
hypothesis. For T5: the statement is elementary — certify it, then attack
the USE: exhibit a 3-profile example with period 3 (rock-paper-scissors
style) demonstrating that period-1 convergence claims would be false, thereby
confirming the project's narrow typing is necessary, or show the narrow
typing still leaks somewhere in the stated corollaries.

(E) **Certify all five** with numbered steps, an airtight restatement of the
coupled-execution/`D_ρ` definition (the one place the prose is loosest), and
the minimal hypothesis list for each theorem (finiteness, determinism,
information consistency, perfect public monitoring, field measurability —
which are load-bearing where).

Zero credit: counterexamples violating the stated setting (stochastic fields,
imperfect recall, non-deterministic transitions) unless you show the project's
setting fails to exclude them; floating-point arithmetic; unproved
assertions.

## 4. DELIVERABLE CONTRACT

End your response with a section titled exactly `MACHINE-CHECKABLE ARTIFACTS`:

1. A line `FINAL ANSWER: COUNTEREXAMPLE (<theorem>)` or
   `FINAL ANSWER: CERTIFIED (L2-T1..T5)` or
   `FINAL ANSWER: DEFINITION GAP (<exact repair>)` or
   `FINAL ANSWER: PARTIAL (<scope>)`.
2. Numbered proof steps labeled `[USES: …]`; any repaired definition stated
   in full.
3. One self-contained program (any language with exact rational arithmetic in
   its standard library — e.g. Python 3 `fractions`; single fenced block;
   deterministic; no network/file I/O; under 60 minutes one core) that:
   (a) implements a small extensive-form game engine per §1 (declared
   explicit family: e.g. 2 seats, 2–3 stages, 2–3 actions per state, worlds
   with 2–6 states, exhaustively or deterministically enumerated — at least
   1,000 distinct (game, σ_0, σ_1) instances; state your family precisely);
   (b) for every instance, enumerates ALL information-consistent focal
   policies and both fields' coupled executions, computes `u_f`, `V_f`,
   `Q_a^(f)`, `d_ρ`, `R_a` exactly, and model-checks L2-T1 (per world),
   L2-T2, L2-T3, and L2-T4 (with exact bounds and with deliberately loosened
   bounds), plus L2-T5 on the profile graph;
   (c) for a counterexample: prints the full game tables and the violated
   inequality with both exact sides;
   printing `PASS <check>` / `FAIL <check> <detail>` lines and instance
   counts, exit 0 iff all pass (for counterexample claims, "pass" confirms
   the violation).

A response whose program fails any of its own checks scores zero. An honest
"the theorems hold but definition X needed repair Y" is a high-value outcome.
