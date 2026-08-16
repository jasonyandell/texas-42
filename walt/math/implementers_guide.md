# Implementer's Guide to the walt Mathematical Basis

**Status:** derived companion document, exploratory tier — NOT authoritative; on any
conflict the v0.4 basis (frozen) and the v0.5 amendment win; every statement below
carries its § citation
**Date:** 2026-08-10
**Sources:** `unified_information_geometry_v0.4.md` (cited as `§n`) and
`equivariant_lumpability_v0.5.md` (cited as `§12.6A (v0.5)`)

This guide answers one question: *what does an implementer need to know?* It states
the objects a program must represent, the results it may build on, the equalities it
must satisfy, and the assumptions it must not make. It reproduces **no proofs and no
proof sketches** — for those, read the basis. Everything in `walt/` sits at the
project's exploratory tier, below every evidentiary tier; that is said once, here,
and not repeated per line.

---

# 1. The model

The objects in dependency order. §15 is the compact formal interface; each object
below expands it with meaning from its home section.

## 1.1 Pips, dominoes, seats, teams — §1.1, §15.1

Pips are `Fin 7`; the domino universe $\mathcal D=\operatorname{Sym}^2(\mathbb P)$ is the
28 unordered two-end multisets, equivalently the edge set of $K_7$ with a loop at each
vertex (mixed tiles are edges, doubles are loops). The written form $h\!:\!l$ with $h\ge l$
is a **naming convention, not a physical orientation**. Seats are $\mathbb Z/4\mathbb Z$
with successor $s^+=s+1$; teams are $T_0=\{0,2\}$, $T_1=\{1,3\}$, $\theta(s)=s\bmod 2$. Each
pip's natural incidence set $\sigma_p$ has seven tiles — a double is in one, a mixed tile in
two (§1.1).

*Shape note:* a domino identity is stable forever, but it is **not** a stable strategic
type — declaration, live context, holder relations, follow obligation, and continuation
position determine its current role (§0, conclusion 1).

## 1.2 Declarations and effective contexts — §1.2

Nine declarations, $\Delta_{\mathrm{Str}}=\mathbb P\cup\{\mathrm{DT},\mathrm{NT}\}$, with
called set $\kappa_\delta$ = $\sigma_p$ for a pip trump, the doubles for DT, empty for NT;
every nonempty called set is powered, so $\pi_\delta=\kappa_\delta$. Contexts are `Fin 8`:
pips 0–6 plus context **7 = the called suit**. The effective incidence family moves called
tiles out of natural suits into context 7 —
$\widehat\sigma_p^\delta=\sigma_p\setminus\kappa_\delta$,
$\widehat\sigma_7^\delta=\kappa_\delta$ — and follow is
$F_\delta(d,q)=\mathbf 1[d\in\widehat\sigma_q^\delta]$, with led context $\ell_\delta(d)$
equal to 7 for a called tile and $\operatorname{high}(d)$ otherwise (§1.2).

*Shape note, load-bearing:* the effective family is a **covering, not a partition** — an
uncalled mixed tile is generally in two effective contexts (§1.2). Do not model the
declaration as an `Option Pip` surrogate; the nine-declaration algebra is the foundation
object (§3.1).

## 1.3 Trick order: tier, rank, key — §1.3

Relative to led context $q$ a tile has tier 2 if trump, 1 if it follows, 0 otherwise;
$r_\delta$ is the declaration-relative rank (under DT doubles rank by pip value; otherwise
a natural double is top of its effective natural suit, and a mixed tile ranks by pip sum
inside a nonzero tier). The trick key is the lexicographic pair
$\tau_\delta(d,q)=(\operatorname{tier}_\delta(d,q),r_\delta(d))$, and **[INHERITED]** for
any distinct four-tile trick with a specified lead the maximum key is unique, so every
legal trick has exactly one winner. Two derived sets an implementation will want:
$\operatorname{BEATS}_\delta(q,d)$ and the when-led threat set
$\operatorname{THREAT}_\delta(d)=\operatorname{BEATS}_\delta(\ell_\delta(d),d)$ (§1.3).

## 1.4 Count as decoration — §1.4

$c(d)$ is 5 on $\{5\!:\!0,4\!:\!1,3\!:\!2\}$, 10 on $\{6\!:\!4,5\!:\!5\}$, 0 elsewhere;
$\sum_d c(d)=35$, and with one trick point per trick a hand is 42 points (§1.4).

*Shape note:* count is a **decoration on a tile, one sparse specialization** of a general
tile valuation $w$, and changing $w$ does not alter legal play unless the change is declared
a common-knowledge rule change (§1.4, §8.2, §8.7). Build the valuation as a parameter from
the start; do not hardcode 5/10.

## 1.5 Deals and the graded DAG — §1.5

A complete deal is an ordered partition of $\mathcal D$ into four seven-tile hands. The
leader plays anything; a follower must play a member of the led effective context when able
and may slough otherwise; the winner leads next (§1.5).

*Shape note:* every legal play removes exactly one tile from one hand, so the
post-declaration game is a **finite graded DAG** — grade by live-tile count — and backward
induction is exact once utility and the optimization operator are fixed. **Boundary:** the
full pre-contract match process can contain unbounded repeated all-pass attempts, about
which finite-hand backward induction proves nothing (§1.5).

## 1.6 Fibers and capacity cells — §2.1

Fix a viewer $m$, a legal public history, and the viewer's private record: $H_m$ is the
viewer's known hand, $U$ the hidden live pool, $k_s$ the remaining capacity of hidden seat
$s$, and $P_s\subseteq U$ the tiles still locally possible at $s$ after all rule-derived
exclusions (public sloughs induce exact voids). The current-remainder fiber
$\Phi(\mathbf C)$ is the set of assignments $(H_s)_{s\ne m}$ with $H_s\subseteq P_s$,
$|H_s|=k_s$, pairwise disjoint, partitioning $U$, where
$\mathbf C=(U;(P_s,k_s)_{s\ne m})$ is the dependent capacity-cell system (§2.1).

**Inherited:** in the Straight cell-theorem scope $\Phi(\mathbf C)$ is *exactly* the set of
hidden current remainders compatible with the viewer's hand and the actor-attributed legal
public prefix — not merely a local Hall relaxation. One member plus the viewer's hand plus
the public residue reconstructs one current objective world (§2.1).

## 1.7 Support normal form — §2.2

Different cell presentations can decode the same fiber, so support is quotiented by
extensional equality. **Inherited:** every feasible capacity-cell system has a canonical
exact support normal form $N=\mathcal N(\mathbf C)$ decoding exactly $\Phi(\mathbf C)$,
whose native representation separates tiles certainly held by each hidden seat, a residual
ambiguous pool, residual capacities, and the matching-supported binary or ternary ambiguity
core. Two feasible systems share a normal form exactly when they decode the same support
set, so $N$ is the coarsest exact deterministic semantic representation of the support
(§2.2).

**Boundary (two, both load-bearing):** feasibility and exact decoding do **not** imply legal
Straight reachability — a support object used as a current state must be inherited from
legal construction or carry an accepted reachability witness, and reachability is **not** a
Boolean field inside the normal form. And the minimality claim is semantic/state-count
minimality for exact support only — not bits, runtime, cache locality, or strategic
sufficiency (§2.2).

## 1.8 The kernel $K$ — §2.3

$K=(\delta,H_m,N,\tau,\alpha_{\mathcal A})$, where $\tau$ is the trick-boundary leader or
the exact folded unresolved-trick residue and $\alpha_{\mathcal A}$ is an accumulator
sufficient for a declared purpose family (possibly trivial). Its exact rule worlds are
$\Phi(K):=[\![N]\!]$; a concrete situation is $x=K\oplus\omega$ for $\omega\in\Phi(K)$; the
live carrier is $L(K)=H_m\sqcup U(N)$, on which $\operatorname{Holds}_x(s,d)$ is exactly
total. Played or captured tiles keep their identity but have no current holder (§2.3).

*Shape note, load-bearing:* the chart — live set, hidden pool, current led context, current
winner, derived standings — is a **deterministic view of the kernel, not an independent
authority** (§2.3). This is the project's derived-views rule, stated inside the basis.

## 1.9 Evidence, latent state, belief, and $B=(K,e,\beta)$ — §2.4

$e$ is the retained viewer-known evidence beyond $K$ that a continuation field, learner, or
utility can still use — possibly empty, a proved sufficient summary, or a slice of the full
perfect-recall record. $\mathcal Z$ is the latent continuation-state space the selected
field needs (opponent policy type, persistent random tape, correlated hidden state), and the
admissible augmented latent domain $\Xi(K,e)\subseteq\Phi(K)\times\mathcal Z$ contains the
pairs satisfying every hard reconstruction and field-compatibility constraint. The belief
$\beta\in\Delta(\Xi(K,e))$ gives weights and may have *smaller* positive-mass support than
the rule support, because of chance-law zeros, policy-likelihood zeros, or earlier
conditioning (§2.4).

The exact current decision state — relative to fixed rules, field class, utility family, and
allowed focal strategy class — is $B=(K,e,\beta)$ (§2.4).

## 1.10 Objects that must never be conflated — §2.5

The noncollapse ledger, restated as a table of things an implementation must keep in
distinct types (§2.5):

| Object | Exact role | Must not be typed as |
|---|---|---|
| $\omega_0$ | one complete initial deal | a current remainder |
| $K$ | exact physical/support kernel | a perfect-recall information state |
| $\Phi(K)$ | rule-compatible current remainders | a probability measure |
| $e$ | retained viewer-known evidence | hidden field state |
| $\Xi(K,e)$ | admissible augmented latent domain | a probability law |
| $\beta$ | belief on that domain | legality |
| $\sigma$ or $\Pi$ | continuation field | a rule |
| $U$ | utility lens | mechanics |
| $V,Q$ | derived values | physical state fields |

A single public action can simultaneously do three different things — objective physical
transition, rule-support restriction and retyping, and likelihood reweighting — and none
may silently substitute for another (§2.5).

There are exact projections *full augmented continuation process* → *physical/support
kernel process* → *support-normal-form process*, and these merge branches (§2.6). So the
support graph is a load-bearing **factor** of the plan graph, never a substitute: exact
support compression solves the legality-domain problem and nothing else (§2.6).
**Inherited/boundary:** a mechanical kernel can be exactly sufficient for objective play
while failing to be the perfect-recall information state; any coarser strategic key needs
its own value- or policy-preservation theorem (§2.6).

## 1.11 Role schemas, output interfaces, Scheme cases, Fixes — §3

A **role schema** is $\Sigma=(N_Q,N_C,N_D)$ — names for effective-context, chair, and
domino roles — interpreted by $\iota=(\iota_Q,\iota_C,\iota_D)$ into $Q,S,\mathcal D$
(§3.2). An **output interface** is a designated subschema $O\subseteq\Sigma$: names in $O$
are returned, names outside are internal existential witnesses. That distinction is
load-bearing — internal proof choices must not become extra referents, probability mass,
tracked identities, valued objects, or public observations (§3.2). The corners are
$O=\varnothing$ (Boolean event query), $O=\{e\}$ (one-output role query), $O=\Sigma$ (full
witnessed realization).

The **structural signature** is a registry of typed atoms — `Live`, `Holds`, `In`,
`Double`, `Beats`, `Boss`, `Void`, `Quota`, $\doteq$, `ChairIs`, `ContextIs`, `Team`, plus
event predicates `Leader`, `NextActor`, `LedContext`, `CurrentWinner`, `Played` — every one
interpreted as a **function of the exact kernel and world**, with derived predicates such
as mastery being registered computations, not independent mutable facts (§3.3). Bounded
local-continuation relations (companion, forced-follower, beater-chain, mobility) may be
added **only with explicit horizon, information access, and semantics** (§3.3).

**Equality patterns:** for each sort take a partition $\pi$ of role names, quotient, then
interpret the quotient injectively (§3.4). A **Scheme case** is $S=(\pi,\varphi)$ with
$\varphi$ a finite conjunction of atoms over the quotient schema; a **Fix** is a finite
disjunction $F=S_1\vee\cdots\vee S_r$ over one common schema and output interface. The
empty Fix is false, contained branches may be removed, and a cut adds a conjunct
branchwise and drops unsatisfiable branches (§3.5).

*Shape note:* one positive conjunctive case is not a Boolean lattice — the semantic
property space is a Boolean algebra and a Fix is a chosen finite disjunctive fragment.
Fixes are extensionally complete only at the expensive limit and only under an explicit
hypothesis: the registry can anchor the ambient kernel (or the semantics is one
fixed-kernel slice) and can name every live holder and output referent (§3.5). The research
problem is compression, not expressibility.

## 1.12 Answer relations and the certainty hierarchy — §4

The answer relation $\operatorname{Ans}^O_{\mathfrak B}(F)$ collects triples
$(K,\omega,\rho)$ where $\rho$ interprets the output names and *some* $\iota\supseteq\rho$
satisfies a case of $F$; per world the output-answer fiber is
$W_F^O(K,\omega)=\{\rho:(K,\omega,\rho)\in\operatorname{Ans}^O\}$, and the Boolean
extension is the projection $\{(K,\omega):W_F^O\ne\varnothing\}$ (§4.1). **A role query
denotes a finite bundle of answer sets over worlds** — that is its type. Two equivalences
follow, not one: $\equiv_{\mathrm{bool}}$ (equal extensions) is strictly weaker than
$\equiv_{\mathrm{ans}}$ (equal answer relations, up to declared output relabeling) (§4.2).

**The certainty hierarchy** — six distinct levels for one fixed $K$, which must not share
a single word such as "known" (§4.4):

1. **event possibility** — some world has a nonempty answer fiber;
2. **event certainty** — every world does;
3. **constant multiplicity** — $|W_F^O(K,\omega)|=k$ for all $\omega$;
4. **constant answer set** — $W_F^O(K,\omega)=W_0$ for all $\omega$;
5. **world-functional reference** — $|W_F^O(K,\omega)|=1$ for all $\omega$;
6. **identity certainty** — $W_F^O(K,\omega)=\{\rho_0\}$ for all $\omega$.

Belief-almost-sure versions replace "every $\omega$" with "$\beta$-almost every latent
state" (§4.4).

## 1.13 Selectors and aggregation lenses — §5

The world marginal is $\mu=(\operatorname{pr}_\omega)_\#\beta$ (§5.1). A **selector** is a
conditional kernel $\chi_{K,\omega}\in\Delta(W_F^O(K,\omega))$ on every world with a
nonempty fiber; it induces $\widehat\mu_\chi(\omega,\rho)=\mu(\omega)\chi_{K,\omega}(\rho)$
and preserves the world marginal (§5.2). A selector may be canonical by a stable order,
probabilistic, adversarial, generated by another latent variable, observed by the focal
player, or hidden from them — **these are different information structures, and none is
supplied by existential satisfaction alone** (§5.2).

An **aggregation lens** $A_x:\mathcal P_{\mathrm{fin}}(\text{answers})\times\mathcal C\to R$
turns an answer *set* into a reward: sum all answers, value one canonical answer, average
under a selector, take the minimum, reward capture of at least one, reward capture of
exactly one (§5.5). Linear selector-based lenses often reduce to ordinary tile valuations;
set-level predicates such as "at least one" depend on the joint capture event and
generally need the full terminal law (§5.5). Timing matters: in general
$\min_\rho\mathbb E[X_\rho]\ne\mathbb E[\min_\rho X_\rho]$ — selecting before continuation
randomness and selecting after the outcome are different games (§5.5).

## 1.14 Typed transitions, rigid transport, persistence/extinction/birth — §6

A **fully typed public observation token** $o$ — actor, played tile,
lead/follow/slough classification, required public context — induces a partial
deterministic map $T_o:D_o\subseteq X_n\to X_{n-1}$ on concrete situations with $n$ live
tiles, whose domain is exactly the predecessors where the action is legal and
actor-correct; $T_{o!}$ and $T_o^*$ are its image and preimage operators (§6.1).

**Exact support update:** for a hidden actor, *force holder edge → delete slough-forbidden
edges → contract played tile → matching-supported reduction*; for a viewer play the hidden
remainder assignment is unchanged although the kernel and live carrier change (§6.2).
**Belief update** is by legality, modeled likelihood, latent transition, and physical
pushforward — the physical marginal on current remainders alone is generally insufficient
when worlds with the same remainder carry different field state or history likelihood
(§6.3). A forced action contributes no discretionary likelihood ratio once the actor's
information state is fixed; the viewer's own action is an intervention when its
randomization is independent of their hidden-deal uncertainty (§6.3).

**Rigid transport** $r_o\rho$ carries an output assignment through $o$: tile identities
persist, chair and context roles follow declared transport rules, and a role may go dead
or stop satisfying its predicate without changing identity. The lifted transition is
$\widetilde T_o(x,\rho)=(T_o(x),r_o\rho)$ and
$\operatorname{Transport}_o(R)=\widetilde T_{o!}(R)$ (§6.4). A **fresh query** $R'$ is
evaluated independently on the successor. The three set operations separating identity
from satisfaction are $\operatorname{Persistent}_o=\operatorname{Transport}_o(R)\cap R'$,
$\operatorname{Extinct}_o=\operatorname{Transport}_o(R)\setminus R'$, and
$\operatorname{Born}_o=R'\setminus\operatorname{Transport}_o(R)$; they answer four
different questions, and **fresh re-query alone cannot answer "did the same physical
object survive"** (§6.5).

**Mastery** is the worked example: $\operatorname{Master}_K(d)$ iff $d\in L(K)$ and
$\operatorname{THREAT}_\delta(d)\cap L(K)=\varnothing$ (§6.6). Four distinct predicates
share the word — absolute live-set mastery, world-relative sure-winning lead,
belief-support sure-winning lead, field-relative winning probability — respectively
structural, control, epistemic, probabilistic; strategic optimality is a fifth layer
(§13.1).

**Hindsight anchoring** is the backward preimage
$\operatorname{AnchorBack}_o(R,A)=R\cap\widetilde T_o^*(A)$ (§6.7). **Conditioning is not
revelation:** analyst conditioning keeps the policy class fixed, player revelation
produces $B^E=(K,e\oplus E,\beta(\cdot\mid E))$ and may enlarge it (§6.8). And **two
recursions are orthogonal** — learning runs forward over $(K_t,e_t,\beta_t,R_t)$, planning
runs backward from terminal universal outcomes to $Q,V$; they meet at decision states and
are not one recursion in opposite textual order (§6.9).

## 1.15 Universal continuation before valuation — §7

$\mathcal C$ is the finite set of terminal continuation outcomes, defined as the image of
legal terminal histories, so conservation and capture/trick consistency hold by
construction. A universal outcome records at least the winner partnership of every
remaining trick, the captor partnership of every relevant tile, banked universal features
retained at the root, and every terminal residue the selected utility family needs (§7.1).

An **information-consistent policy** is $\rho_s:\mathcal I_s\to\mathcal A$ choosing one
legal action at every reachable perfect-recall information state. It may branch on
observations received; it may **not** branch on the hidden world unless the world was
revealed in the declared information structure (§7.2).

The **universal outcome kernel**
$\mathcal O_{K,e}:\Xi(K,e)\times\mathcal R(K,e)\to\Delta(\mathcal C)$ is valuation- and
utility-free, and the belief-integrated game form
$\Gamma_B(\rho)=\int\mathcal O_{K,e}(\xi,\rho)\,\beta(d\xi)$ is the central universal
continuation object (§7.3). **Fixed-field specialization — this is walt:** fixing focal
player $m$ and a continuation field $\sigma_{-m}$,
$\Gamma_B^\sigma(\rho_m)=\Gamma_B(\rho_m,\sigma_{-m})$ assigns one terminal law to every
information-set-consistent focal policy, **count-blind and utility-blind** (§7.4). $J$,
$Q_B(a;w,U)$, and $V_B(w,U)$ follow by integration and finite maximization (§7.5).

**Solution-operator boundary:** the same universal layer supports perfect-information
max/min induction, fixed stochastic field evaluation, fixed-field information-set best
response, teacher/benchmark evaluation, and a later equilibrium operator — and theorems for
one **do not** transfer to another (§7.7).

## 1.16 Additive features, the free monoid, the gauge — §8

For focal partnership $T$ and terminal outcome $c$, $t_T(c)$ counts remaining tricks won
and $x_{T,d}(c)$ indicates capture of tile $d$; the universal additive feature
$\phi_T(c)=(t_T(c),(x_{T,d}(c))_d)$ is a 29-coordinate vector obeying the conservation law
$\sum_d x_{T,d}(c)=4\,t_T(c)$ for any continuation measured consistently from its root
(§8.1). It lives in the free commutative monoid $M=\mathbb N^{(\{\star\}\sqcup\mathcal D)}$,
through which every additive terminal score with trick coefficient $b$ and tile valuation
$w$ factors uniquely as $b\,t_T(c)+\sum_d x_{T,d}(c)w(d)$ (§8.2). Straight count is one
sparse $w$; no-count is $w=0$; one specially valued tile is $w=\lambda e_d$.

**The gauge:** because $\sum_d x_d=4t$, the pair $(b,w)$ is not identifiable —
$(b,w)\sim(b-4c,w+c\mathbf 1)$ — so valuation lives in the 28-dimensional quotient
$(\mathbb R\times\mathbb R^{\mathcal D})/\langle(-4,\mathbf 1)\rangle$ (§8.3). Splitting
$w=u\mathbf 1+\eta$ with $\sum_d\eta(d)=0$ gives one symmetric trick mode plus 27
anisotropy coordinates (§8.4).

**Mid-hand modes:** future-increment mode revalues only future captures (the banked past is
an action-independent constant under the original utility); full-hand valuation-universal
mode retains a banked feature $\alpha_T$ whose contribution $\langle(b,w),\alpha_T\rangle$
is action-independent, so action order is unchanged. **A scalar Straight-count bank is not
sufficient for arbitrary revaluation of past tile identities** (§8.5).

**Sufficiency hierarchy** (§8.6), smallest sufficient object first per evaluation class:
terminal feature → expected feature → terminal feature law → full universal terminal law →
policy-indexed game form. No smaller quotient is universal without a theorem for the
selected utility family.

**Two meanings of "change the tile values"** (§8.7): *payoff relabeling* holds history,
belief, field, and information structure fixed and replaces only $w\mapsto U_w$ — the clean
parametric experiment; *common-knowledge scoring change* lets the field itself depend on
$w$, so likelihoods and posteriors change and off-path assessments may be required. Payoff
relabeling does not filter rule support — $\Phi(K)$ is unchanged — though a role-value
constraint can filter *role interpretations* as a query restriction (§8.8).

**Inherited:** no scalar $v(d)$ depending only on physical identity equals exact strategic
action value in every information state, belief, field, and utility; the foundation holds a
legal witness with opposite exact values for the same action and mechanical endpoint under
different hidden worlds (§8.9). $w(d)$ is a payoff coefficient, never $Q(B,d)$.

## 1.17 Policy polytopes and support functions — §9

For each deterministic information-consistent focal policy $\rho$, its expected feature
vector is $\mu_\rho=\mathbb E_{c\sim\Gamma_B^\sigma(\rho)}[\phi_T(c)]\in\mathbb R^{29}$
(§9.1). The action polytope is $P_{B,a}=\operatorname{conv}\{\mu_\rho:\rho(B)=a\}$ and the
root polytope is $P_B=\operatorname{conv}\bigcup_a P_{B,a}$; independent focal randomization
adds no point outside them (§9.1).

Values are support functions: $Q_B(a;v)=h_{P_{B,a}}(v)$ and $V_B(v)=h_{P_B}(v)$ (§9.2). The
polytope is the **complete** finite signature of an action for every additive valuation
direction (§9.3) — for a restricted valuation family $W$ only $h_P|_W$ matters, so a smaller
quotient may be exact.

Along a one-tile ray, each policy contributes a line $A_\rho+B_\rho\lambda$ and $Q$ is a
continuous convex piecewise-affine function with **rational breakpoints when the field,
belief, and feature data are rational** (§9.4). The normal fan of $P_{B,a}$ partitions
valuation space into regions exposing the same policy face (§9.4). The directional
derivative is $D^+h_P(v;u)=h_{F_P(v)}(u)$, so a one-tile defect first interrogates the
baseline-optimal face and can resolve a baseline tie at $0^+$ with no positive interior
crossing (§9.5).

**Cone dominance** prunes relative to a permitted valuation cone $C$ and is purpose-relative
— a point dominated on "more points" directions may be exposed under negative or
threshold-sensitive valuations (§9.6). **Activity** of a direction is likewise relative to
action, information structure, field, baseline, and permitted parameter domain — never an
intrinsic property of a physical tile (§9.7).

## 1.18 Information treatments H / C / F and prices — §10

An **information structure** $\mathcal I$ is an equivalence relation on the finite set
$D_m$ of future focal decision nodes; only valid classes are admitted (same acting player
and same legal action labels, or a declared bijective action transport). A deterministic
policy is a legal map constant on every class (§10.1). Refinement $\mathcal I\preceq\mathcal I'$
imposes fewer action-equality constraints (§10.1).

Three treatments over the same belief, field, utility, and root actions (§10.3):

- **H** — actual hidden information: the focal player sees only the original record.
- **C** — common root, continuation revealed: the root action is common across worlds, then
  the complete world is revealed before any later focal decision. Its polytope is the
  weighted Minkowski sum $P_a^C=\sum_\omega\mu(\omega)P_{\omega,a}$.
- **F** — world revealed before the root: $P^F=\sum_\omega\mu(\omega)P_\omega^{\mathrm{root}}$.

The prices (§10.5): $G_a^{\mathrm{cont}}(v)=h_{P_a^C}(v)-h_{P_a^H}(v)$,
$G^{\mathrm{cont}}=V^C-V^H$, $G^{\mathrm{root}}=V^F-V^C$, $G^{\mathrm{total}}=V^F-V^H$. Every
price is nonnegative by the polytope inclusions (§10.5).

**C is the controlled causal comparison for information value** — per-world
perfect-information minimax changes both the information *and* the continuation operator,
while C changes only focal information and keeps the field fixed (§10.8).

## 1.19 Constellations and carriers — §11

Three objects that must not share one unqualified definition (§11.1):

- **orbit constellation** — $\operatorname{Orb}_G(x)$ for a *proved* transformation group,
  with declaration, seats, partnerships, orientation, evidence, belief, field, output roles,
  and utility orientation all transported; arbitrary chair or pip permutations are not
  automatically symmetries;
- **Scheme cell** — the world set / answer relation / observable fiber a Fix denotes; it can
  contain part of one orbit, several orbits, or several strategic classes;
- **purpose constellation** — $x\equiv_{\mathcal P}y\iff R_{\mathcal P}(x)=R_{\mathcal P}(y)$
  for a chosen carrier $X$ and exact response map. **There is no purpose-free canonical
  constellation.**

The carrier may be an objective physical state, an exact support kernel, a concrete hidden
world, a decision state $B$, a world–output-answer tuple, an information structure with its
polytope, or a dynamic predictive state (§11.2). A world-level perfect-information response
class is not automatically a hidden-information decision class; a support quotient is not
automatically a belief quotient; a value class is not automatically a policy class (§11.2).

**Inherited:** mechanical future equivalence is a right congruence whose reachable quotient
is the unique smallest exact deterministic machine for a *selected deterministic output
contract* — it does not automatically minimize a stochastic belief process or an
imperfect-information policy problem (§11.3).

**Exact seat gauges:** seat rotations form an exact $C_4$ symmetry under complete transport,
with odd rotations exchanging team labels so utility orientation must be transported.
Reflection is **not** an automorphism of the oriented extensive game; it becomes a coordinate
gauge only after adjoining an orientation variable $\eta\in\{+1,-1\}$ with
$\operatorname{next}_\eta(s)=s+\eta$, giving a $D_4$ gauge on the oriented family (§11.7).

## 1.20 Descriptors, soundness, factorization, lumpability — §12

A descriptor $D:X\to Z$ is **purpose-sound** for target $R^*:X\to Y$ when
$D(x)=D(y)\Rightarrow R^*(x)=R^*(y)$, and **purpose-exact** when descriptor fibers equal
response fibers (§12.1). Soundness is equivalent to a unique factorization $R^*=\bar R\circ D$
(§12.1) — the exact static compression target.

**Three distinct compression questions** (§12.2): extensional world compression
$|X|/|\operatorname{im}D|$; intensional relational compression (size and generality of the
program/Fix/grammar, with no canonical scalar absent a language and cost model); and
decision-geometry compression (distinct feature points, vertices, exposed faces, ray
segments, action regions). Exact support-normal-form compression is a fourth, already
established layer beneath these (§12.2).

A **dynamic control skeleton** is a typed relational state with update
$D_{t+1}=\delta_D(D_t,a_t,o_{t+1})$, or a finite stochastic kernel over descriptor states
(§12.5). Its roles may include current led-context strength, actual next actor and relative
seat position, rigidly transported valued-tile and companion roles, forced-follow and slough
mobility, beater/overtake chains in the context actually led, newly observed void and
possession facts, and support-normal-form summaries needed for exact transition (§12.5). It
should instantiate successor-context roles **when they become causally relevant**, not encode
every possible future suit at the root (§12.5).

**Strong controlled lumpability** (§12.6): with $K_a(x;r,o,x')$ the joint kernel for
immediate universal feature increment, next observation, and successor latent state, a
descriptor $d:X\to Y$ is strongly controlled-lumpable when $d(x)=d(y)$ implies (1)
$A(x)=A(y)$ and (2) for every legal $a$, increment $r$, observation $o$, and descriptor state
$y'$, the aggregate successor probabilities agree. This induces a well-defined abstract
kernel $\bar K$. **Boundary:** strong lumpability is sufficient and deliberately stringent;
weaker belief-dependent, policy-relative, or bisimulation-style quotients may exist and
require separate theorems (§12.6).

Scheme/Fix can serve as the syntax for $d$ when output roles are explicit, rigid and fresh
semantics are distinguished, exact support stays authoritative, every derived continuation
atom declares horizon and information access, the step compiler is proved to preserve the
answer relation, and the induced descriptor transition satisfies the selected theorem
(§12.7).

## 1.21 The v0.5 equivariant version — §12.6A (v0.5)

v0.4 §12.6 compares interfaces **literally**, so two situations differing only by tile
identity can never merge; under that reading only world-reconstructing skeletons pass. v0.5
replaces literal equality with **equivariant** equality: interfaces are compared under
declared typed transports and outcomes under the count-free quotient (§12.6A preamble).

**Interface-decorated carrier.** Work *after* the active equality-pattern quotient, so
remaining output names are interpreted injectively per sort; if different equality patterns
can occur, the descriptor must retain enough to determine the resulting quotient-interface
type. Each latent state $x$ carries a declared concrete interpretation $\rho_x$ of
$O_\Sigma$, which must be **functionally instantiated** — from a unique Scheme answer, a
declared selector incorporated into the latent state, or another explicit construction. A
merely existential multi-answer fiber does not define $\rho_x$. Names outside $O_\Sigma$
stay internal witnesses and gain no persistence or transport (§12.6A).

**Declared interface transports.** Whenever $d(x)=d(y)$, a declared typed transport
$\Theta_{xy}=(\Theta^Q,\Theta^C,\Theta^D)$ relates the concrete realizations by role-name
correspondence ($\Theta(\rho_x(n))=\rho_y(n)$), plus bijections $\Theta^A_{xy}:A(x)\simeq A(y)$
on legal actions and $\Theta^{\mathrm{obs}}_{xy}$ on observation labels. Coherence:
$\Theta_{xx}=\mathrm{id}$, $\Theta_{yx}=\Theta_{xy}^{-1}$, $\Theta_{xz}=\Theta_{yz}\circ\Theta_{xy}$.
The chair transport carries the declared partnership and orientation convention, and
$e_\star$ always denotes the trick coordinate of the *transported* focal partnership
(§12.6A). Transports are defined on the declared represented interface unless the action or
observation language requires a larger one — and they are **not** asserted to be global
symmetries of Straight 42 (§12.6A).

**The rigid square.** For output roles declared rigid through a step,

$$\Theta^\Sigma_{x'y'}\circ r_o=r_{\Theta^{\mathrm{obs}}_{xy}(o)}\circ\Theta^\Sigma_{xy}.$$

Fresh successor roles are re-evaluated at the successor and make no claim of predecessor
identity; their equality is governed by successor descriptor semantics, not by the square
(§12.6A).

**Count-free kernel.** The preserved immediate outcome alphabet is only the trick component
$R_\star\subseteq\mathbb N e_\star$ — at a primitive play step normally $\{0,e_\star\}$. **No
physical-domino capture coordinate is part of the primitive lumpability contract** (§12.6A).
Condition (ECL) then requires $A(y)=\Theta^A_{xy}(A(x))$ and equality of aggregate successor
probabilities after transporting the action and observation (§12.6A).

**Role re-entry of tile features.** Tile anisotropy comes back through declared *domino
roles*, not physical coordinates: for rigid $e\in O_D$, $\bar x_{T,e}$ indicates capture of
the transported tile occupying role $e$, giving
$\bar\Phi_{T,O}=t_Te_\star+\sum_{e\in O_D}\bar x_{T,e}e_e$ and coefficients
$\lambda:O_D\to\mathbb R$ whose physical pullback is $w_x(\rho_x(e))=\lambda(e)$. Role
coefficients are invariant under change of representative even when the physical domino
occupying the role changes (§12.6A).

**Stabilizer boundary.** A *fixed physical* valuation descends through a descriptor class
only when $w(\Theta^D_{xy}(d))=w(d)$ for every valuation-relevant represented tile — i.e. when
the declared transport lies in the stabilizer of $w$. Otherwise you must transport the
valuation with the role interface, retain the distinguishing label in the descriptor, or
refine the class. **The dynamic quotient is fundamentally valuation-free**; the §8 gauge acts
only after the role-indexed valuation interface is declared (§12.6A).

---

# 2. Proved outcomes you may build on

Prose-proved is **not** machine-checked. Every v0.4 and v0.5 result below carries a prose
proof in its source and no proof-assistant term (§0 formalization intent, §17.2,
§12.6A claim ledger). [INHERITED] items come from the established foundation and are
source-authoritative (§17.1).

**Foundations you may assume (§17.1).**

1. **[INHERITED]** The 28-domino universe, seat/team structure, and the nine-declaration
   relational algebra. *Licenses:* modeling declarations as a nine-element type; forbids an
   `Option Pip` surrogate (§3.1, §17.1).
2. **[INHERITED]** Exact follow and trick-winner rules; every legal trick has exactly one
   winner. *Licenses:* a total winner function on legal tricks with no tie handling (§1.3).
3. **[INHERITED]** Finite objective post-declaration play as a graded DAG. *Licenses:*
   termination by induction on live-tile count; forbids extending that to the pre-contract
   pass-out process (§1.5).
4. **[INHERITED]** Exact current-remainder fibers: $\Phi(\mathbf C)$ is exactly the
   compatible hidden remainders in the cell-theorem scope. *Licenses:* enumerating the fiber
   as *the* world set, not an over-approximation (§2.1).
5. **[INHERITED]** Support-normal-form semantic minimality and typed support transitions.
   *Licenses:* keying hidden-state equality on $N$ alone; forbids treating $N$ as carrying
   reachability (§2.2, §6.2).
6. **[INHERITED]** Separation of support, evidence, belief, field, and value; a mechanical
   kernel can be play-sufficient without being the perfect-recall information state.
   *Forbids:* reusing one type for two of these roles (§2.5, §2.6).
7. **[INHERITED]** Exact finite fixed-field best-response existence. *Licenses:* writing the
   walt solve as a finite maximization (§17.1, §7.5).
8. **[INHERITED]** Mechanical future-equivalence minimality for a selected deterministic
   output contract. *Licenses:* Myhill–Nerode-style minimization of a deterministic machine;
   forbids applying it to a stochastic belief process (§11.3).
9. **[INHERITED]** Seat-rotation and oriented-frame gauge boundaries: $C_4$ exact, reflection
   only with an orientation variable. *Licenses:* rotational canonicalization; forbids free
   chair permutation (§11.7).
10. **[INHERITED]** No universal context-free domino value. *Forbids:* any static per-tile
    strength table used as strategic value (§8.9).

**Results proved in v0.4** — the 21 enumerated in §17.2, plus two theorems proved in the
document but absent from that enumeration: §9.8's full independent terminal separation and
§9.9's evaluation/backward-induction commutation with its piecewise-affine corollary.

11. **[PROVED v0.4]** *Equality-pattern completeness* — every interpretation of a finite role
    schema factors uniquely as a kernel partition followed by an injective interpretation of
    the quotient. *Licenses:* enumerating equality patterns as a finite disjunction and
    keeping every branch injective (§3.4).
12. **[PROVED v0.4]** *Answer-level meta-fiber decomposition* — the full witnessed realization
    relation is the disjoint union of ground fibers tagged by interpretation. *Licenses:*
    implementing output answers and Boolean extension as relational projections of one
    object (§4.3).
13. **[PROVED v0.4]** *No canonical answer lift* — if any positive-mass world has two answers,
    two distinct world–answer laws share the world marginal. *Forbids:* deriving an answer
    distribution from belief alone; a selector must be declared (§5.1).
14. **[PROVED v0.4]** *Multiplicity-bias covariance identity* —
    $\mathbb E_{\widetilde\mu_F}[X]-\mathbb E_\mu[X]=\operatorname{Cov}_\mu(X,m_F)/\mathbb E_\mu[m_F]$.
    *Forbids:* weighting worlds by answer count; also gives the exact size of the error if
    someone does (§5.3).
15. **[PROVED v0.4]** *Image/preimage adjunction and cut/step exchange* —
    $T_{o!}(A\cap T_o^*(B))=T_{o!}(A)\cap B$. *Licenses:* pushing a filter through a step in
    either order; this is the correctness spec for a `step` compiler (§6.1).
16. **[PROVED v0.4]** *Surviving-master monotonicity* — a master that stays live stays a
    master when live tiles are deleted. *Licenses:* caching mastery across deletions;
    requires recomputing for **birth** of new masters (§6.6).
17. **[PROVED v0.4]** *Universal fixed-field terminal-law sufficiency* — $\Gamma_B^\sigma$
    determines the exact fixed-field best response and every root action value for every
    bounded utility readable from $\mathcal C$; independent private randomization cannot
    improve the optimum. *Licenses:* solving once, valuing many times; restricting search to
    deterministic contingent policies (§7.5).
18. **[PROVED v0.4]** *Universal additive factorization* — every additive terminal score
    factors uniquely as $b\,t_T+\sum_d x_{T,d}w(d)$. *Licenses:* storing a 29-coordinate
    feature instead of a score (§8.2).
19. **[PROVED v0.4]** *Additive gauge invariance* — $(b,w)\sim(b-4c,w+c\mathbf 1)$; valuation
    space is a 28-dimensional quotient. *Licenses:* canonicalizing valuations; forbids
    treating $(b,w)$ as identifiable (§8.3).
20. **[PROVED v0.4]** *Action-value support-function representation* —
    $Q_B(a;v)=h_{P_{B,a}}(v)$, $V_B(v)=h_{P_B}(v)$. *Licenses:* computing $Q$ as
    $\max_{x\in S_{B,a}}\langle v,x\rangle$ over a finite feature set (§9.2, §16.1).
21. **[PROVED v0.4]** *Support-function completeness* — $h_P=h_Q$ for all $v$ iff $P=Q$.
    *Licenses:* using the polytope as the complete signature of an action across all additive
    valuations (§9.3).
22. **[PROVED v0.4]** *Support-function directional derivative* — $D^+h_P(v;u)=h_{F_P(v)}(u)$.
    *Licenses:* resolving baseline ties at $0^+$ by optimizing $u$ over the baseline-optimal
    face only (§9.5).
23. **[PROVED v0.4]** *Sufficient inertness criterion* — if $\langle u,x\rangle$ is constant on
    $P_{B,a}$ then $Q$ shifts affinely and the maximizer set is unchanged. *Licenses:*
    skipping a direction, with proof, instead of re-solving (§9.7).
24. **[PROVED v0.4]** *Monotone valuation refinement* — a larger valuation family gives a finer
    or equal purpose equivalence. *Licenses:* incremental valuation families; adding tile
    values can split classes but never merge them (§9.8).
25. **[PROVED v0.4]** *Full independent terminal separation* — equal additive value for every
    tile valuation, with the trick coefficient observable, implies equal trick-and-capture
    vectors. *Licenses:* using the feature vector as a complete terminal key (§9.8).
26. **[PROVED v0.4]** *Evaluation commutes with backward induction*, with its
    piecewise-affine corollary — for a finite complete-information continuation whose internal
    operators are finite max, finite min, or fixed valuation-independent expectations, and
    whose leaves are affine in the valuation, evaluating the symbolic expression after the
    structural recursion equals scalar backward induction under that valuation; every
    resulting $V$ and $Q$ is continuous piecewise affine. *Licenses:* solving once symbolically
    and evaluating at many valuations. *Forbids:* assuming convexity outside the fixed-field
    best-response case, or extending it to mixed equilibria recomputed per valuation (§9.9).
27. **[PROVED v0.4]** *Information-refinement policy and polytope inclusion* —
    $\mathcal R_{\mathcal I}(a)\subseteq\mathcal R_{\mathcal I'}(a)$ and
    $P_a^{\mathcal I}\subseteq P_a^{\mathcal I'}$, hence $Q_a^{\mathcal I'}\ge Q_a^{\mathcal I}$.
    *Licenses:* using a revealed solve as a valid upper bound on a hidden solve (§10.2).
28. **[PROVED v0.4]** *Revealed-continuation Minkowski formula* —
    $P_a^C=\sum_\omega\mu(\omega)P_{\omega,a}$. *Licenses:* computing treatment C per world and
    combining, instead of solving a revealed game globally (§10.3).
29. **[PROVED v0.4]** *Nested information polytopes* — $P_a^H\subseteq P_a^C$ and
    $P^H\subseteq P^C\subseteq P^F$. *Licenses:* asserting $V^F\ge V^C\ge V^H$ as an invariant
    a test can check (§10.4).
30. **[PROVED v0.4]** *Exact information-price decomposition* —
    $G^{\mathrm{total}}=G^{\mathrm{cont}}+G^{\mathrm{root}}$, every price nonnegative.
    *Licenses:* reporting the two prices separately with an exact additivity check (§10.5).
31. **[PROVED v0.4]** *Zero-information exposed-face criterion* — for $P\subseteq Q$,
    $h_Q(v)=h_P(v)$ iff $F_Q(v)\cap P\ne\varnothing$. *Licenses:* certifying zero revelation
    value by exhibiting one hidden-implementable optimal feature, without comparing whole
    polytopes (§10.6).
32. **[PROVED v0.4]** *Static descriptor factorization* — $D$ is purpose-sound iff
    $R^*=\bar R\circ D$ for a unique $\bar R$. *Licenses:* implementing a sound descriptor as
    a lookup from descriptor cell to response (§12.1).
33. **[PROVED v0.4]** *Strong controlled-lumpability value preservation* — a strongly
    controlled-lumpable $d$ gives exact abstract filtering, equal joint law of observations
    and accumulated features, equal utility values, and equal $V$/$Q$ over the same abstract
    policy class. *Licenses:* running the whole solve on descriptor states once the two
    conditions are checked (§12.6).

**The v0.5 amendment (§12.6A).**

34. **[PROVED v0.5]** *Equivariant controlled lumpability* — under (ECL) with declared
    coherent interface, action, and observation transports and a count-free increment
    alphabet, the pushed belief updates using $\bar K$ alone; lifted abstract policies are
    lawful and induce the same joint law of transported observations, accumulated $e_\star$,
    and descriptor state; the rigid output-role trace is well defined on the quotient; the
    count-free terminal outcome law, every statistic readable from the preserved trace, every
    bounded utility of those quantities, and $V$ and $Q$ all agree. *Licenses:* merging states
    that differ by tile identity — the merge v0.4 §12.6 structurally forbade (§12.6A preamble).
35. **[PROVED v0.5]** *Valuation gauge descends to the quotient* — under capture-completeness
    of the domino-role interface, $\sum_{e\in O_D}\bar x_{T,e}=4t_T$, so
    $(b,\lambda)\sim(b-4c,\lambda+c\mathbf 1)$ and valuation factors through
    $(\mathbb R\times\mathbb R^{O_D})/\langle(-4,\mathbf 1)\rangle$, with $Q$ and $V$ equal to
    the exact values per gauge class. *Licenses:* reusing the entire §8/§9 valuation and
    polytope machinery on role coordinates (§12.6A).
36. **[PROVED v0.5]** *§12.6 as the identity-interface case* — taking every transport to be
    the identity and omitting the role-indexed readout recovers v0.4 §12.6. *Licenses:* one
    code path for both, with identity transports as the default configuration (§12.6A).

---

# 3. Implementation contracts

These are the semantic equalities and preservation conditions code must satisfy. They are
the testable surface of the model.

1. **Step-compiler contract — §6.1.** Any syntactic `step` must satisfy the image/preimage
   adjunction and cut/step exchange:
   $T_{o!}(A\cap T_o^*(B))=T_{o!}(A)\cap B$. A syntactic Scheme/Fix step compiler is exact
   only after denotational equality with the lifted direct image is proved (§16.5).

2. **Exact support update — §6.2.** Successor support is *uniquely determined* by
   $(N,\delta,\text{actor},\text{played tile},\text{led context or lead boundary})$. Implement
   the hidden-actor path as force-holder → delete slough-forbidden → contract → matching
   reduction; a viewer play leaves the hidden remainder assignment unchanged (§6.2).

3. **No independent rival holder state — §6.2.** Scheme must not maintain its own holder
   store. It queries the exact kernel before and after the inherited support update.

4. **Derived views, never stored authorities — §2.3, §2.5.** The chart (live set, hidden pool,
   led context, current winner, standings) is a function of $K$. Storing it as a second
   authority is forbidden; the noncollapse ledger's nine rows must be nine distinct types.

5. **Rigid transport vs fresh query — §6.4, §6.5.** These are two separate operations and both
   must exist. Persistence/extinction/birth are computed from
   $\operatorname{Transport}_o(R)$ and $R'$; a system that only re-queries cannot answer "did
   the same physical object survive?"

6. **Hindsight anchoring is a preimage, never player knowledge — §6.7.**
   $\operatorname{AnchorBack}_o(R,A)=R\cap\widetilde T_o^*(A)$ filters prior hypotheses for an
   analyst. It must not feed the player's earlier information state or authorize a
   hindsight-informed earlier action.

7. **Conditioning ≠ revelation — §6.8.** Analyst conditioning keeps the policy class fixed;
   player revelation constructs $B^E=(K,e\oplus E,\beta(\cdot\mid E))$ and may enlarge the
   class. Conflating them leaks hidden information and recreates strategy fusion.

8. **Never weight worlds by answer count — §5.3.** The multiplicity-bias identity is the exact
   statement of the error: naive per-pair uniform mass reweights worlds by $m_F$ and shifts
   every statistic by $\operatorname{Cov}_\mu(X,m_F)/\mathbb E_\mu[m_F]$.

9. **Fix union is not a mixture — §5.4.** $\operatorname{Ans}^O(F)=\bigcup_i\operatorname{Ans}^O(S_i)$
   as a *set* union. An answer satisfying two branches is one answer; branch overlap is not
   duplicated mass. A probabilistic mixture over branches is a separate model needing explicit
   weights and a branch-selection variable.

10. **Additive gauge quotient — §8.3.** Valuations are equal when they lie on the same gauge
    line $(b,w)\sim(b-4c,w+c\mathbf 1)$. Canonicalize before comparing, hashing, or caching a
    valuation.

11. **Universal continuation before valuation — §7, §0 conclusion 6.** A count-blind solve must
    retain policy-indexed terminal laws, feature sets, symbolic game forms, or an equivalent.
    **Retaining only the action optimal at one baseline valuation destroys later valuation
    universality.**

12. **Policy legality typing — §7.2.** A policy is a map from information states to legal
    actions, constant on information classes. Branching on the hidden world must be
    unrepresentable in the type, not merely avoided by convention (§16.6, strategy-fusion
    exclusion by typing).

13. **Interfaces functionally instantiated — §12.6A (v0.5).** $\rho_x$ must come from a unique
    Scheme answer, a declared selector folded into the latent state, or another explicit
    construction. An existential multi-answer fiber does **not** define an interface; code
    must reject that configuration rather than pick a witness.

14. **Transports with coherence — §12.6A (v0.5).** Declared transports must satisfy
    $\Theta_{xx}=\mathrm{id}$, $\Theta_{yx}=\Theta_{xy}^{-1}$,
    $\Theta_{xz}=\Theta_{yz}\circ\Theta_{xy}$ within a descriptor class, for the interface,
    action, and observation transports alike. (On represented objects the role-name
    correspondence makes coherence automatic; it has independent force on any declared
    extension required by the observation language — v0.5 Appendix A, review commentary.)

15. **The rigid square — §12.6A (v0.5).**
    $\Theta^\Sigma_{x'y'}\circ r_o=r_{\Theta^{\mathrm{obs}}_{xy}(o)}\circ\Theta^\Sigma_{xy}$ on
    the rigid subinterface. This is a checkable equality per class and step, and it is what
    makes the abstract role history representative-independent.

16. **Count-free primitive kernel — §12.6A (v0.5).** The primitive lumpability contract carries
    only $R_\star\subseteq\mathbb N e_\star$ (normally $\{0,e_\star\}$ per play step). Do not
    put the 28 capture coordinates into the primitive kernel; tile anisotropy re-enters through
    declared domino roles.

17. **Stabilizer check before descending a fixed physical valuation — §12.6A (v0.5).** Before
    reusing a descriptor class under a fixed $w$, check $w(\Theta^D_{xy}(d))=w(d)$ on every
    valuation-relevant represented tile. On failure: transport the valuation with the role
    interface, retain the distinguishing label in the descriptor, or refine the class.

18. **Capture-completeness before using the quotient gauge — §12.6A (v0.5).** The role-indexed
    conservation $\sum_{e\in O_D}\bar x_{T,e}=4t_T$, and therefore the quotient gauge, holds
    only when every tile whose capture enters the selected additive outcome appears exactly
    once as a rigid domino role, with banked and unresolved-trick residue included
    consistently.

19. **No target leakage in the atom registry — §3.3.** A registered predicate may not call the
    target solver or read the response class, and every bounded-continuation atom must declare
    its horizon and information access.

20. **Exact arithmetic through the PWL layer — §9.4, §14.1, §16.8.** Breakpoints are rational
    when field, belief, and feature data are rational; endpoint ownership and interval
    invariants should be proved, not only tested (the basis records a real
    interval-endpoint defect found this way, §14.1, §16.8).

---

# 4. Boundaries and nonclaims — what you must NOT assume

A checklist. Each line is a thing the basis explicitly does not claim (§17.5) or explicitly
bounds.

- **Support does not determine belief.** $\Phi(K)$ is the allowed domain; $\beta$ carries
  weights and may have strictly smaller positive-mass support (§2.4, §17.5).
- **Feasible is not reachable.** Exact normal-form decoding does not imply legal Straight
  reachability, and reachability is not a field inside the support normal form (§2.2).
- **A Scheme is not an information state,** and existential witnesses are not probabilities
  (§17.5, §3.2).
- **Event certainty does not license "the".** A query true in every world can return several
  answers in every world; a definite description needs uniqueness, a selector, or a declared
  set-valued aggregation (§4.4, §13.3).
- **Perfect-information / worldwise response classes are NOT hidden-decision classes.** A
  descriptor pure for $R_{\mathrm{PI}}$ is sufficient for averaging oracle responses and is
  *not necessary* for an exact hidden solve; two worlds with different oracle responses can
  merge in an exact hidden model (§12.4, §17.5).
- **Information-state count is not information value.** The meaningful object is
  incompatibility among optimal actions across glued nodes, not raw node count (§10.9, §17.5).
- **More information does not preserve breakpoints.** Refinement raises support functions but
  need not preserve vertices, exposed faces, normal fans, policy breakpoints, or root-switch
  prices (§10.7, §17.5).
- **"Free versus pinned tile" is not an information-independent physical classification.**
  Activity is relative to the selected information polytope (§14.6, §9.7).
- **The universal continuation object is not one baseline-optimal policy** (§17.5, §7).
- **Expected capture vectors do not suffice for nonlinear utility** (§8.6, §17.5).
- **A scalar count bank does not suffice for arbitrary revaluation of past tile identities**
  (§8.5).
- **Theorems do not transfer between solution operators.** Piecewise-affine support-function
  results for a finite fixed-field best response say nothing about a valuation-dependent
  mixed-strategy equilibrium (§7.7).
- **Reported experiment numbers are receipts, not theorems.** Everything in §14 is
  `[EXPERIMENTAL RECEIPT — reported]`; the consolidation did not rerun the programs.
  Promotion requires adding programs, records, and a certificate checker to the project's
  verifier-receipt process (§14.1, §17.3). In particular: the four Experiment 3A atoms are not
  globally sufficient, the 33-cell descriptor is not language-independent or transferable, the
  repaired 64-template registry is not useful merely because it is sound, perfect-information
  affineness is **not** a theorem, and doubles-trump / no-trump transfer has **not** been
  validated (§17.5).
- **Strong lumpability is stringent, not canonical.** Weaker belief-dependent, policy-relative,
  or bisimulation-style quotients may exist and need separate theorems (§12.6).
- **v0.5 value equality is over the transported abstract-policy class.** Whether the
  unrestricted concrete optimum is attained inside that class is a separate sufficiency
  question, deliberately not claimed (§12.6A boundary; same shape as §12.6 conclusion 4).
- **v0.5 transports are per-class declarations, not global symmetries** of Straight 42
  (§12.6A boundary, cf. §11.1, §11.7).
- **Existence of a nontrivial $(d,\Theta)$ satisfying (ECL) on real kernels is OPEN**
  (§12.6A claim ledger).
- **Arbitrary chair or pip permutations are not automatically symmetries;** reflection is not
  an automorphism of the oriented game without an orientation variable (§11.1, §11.7).
- **Finite-hand backward induction proves nothing about the repeated-pass-out match process**
  (§1.5, §17.4 item 10).
- **The proposed Lean module plan has not been implemented** (§17.5, §16).

---

# 5. The target

Everything above serves one object. The static factorization $R^*=\bar R\circ D$ is *not* the
goal; the goal is the dynamic commuting square (§18):

$$
\begin{array}{ccc}
\text{exact latent process} & \xrightarrow{\ d\ } & \text{control-skeleton process}\\
\downarrow\ \text{action, observation, feature} & & \downarrow\ \text{induced abstract kernel}\\
\text{exact successor law} & \xrightarrow{\ d_\#\ } & \text{abstract successor law}\\[2mm]
Q_{\mathrm{abstract}}(a;w,U) & = & Q_{\mathrm{exact}}(a;w,U)
\end{array}
$$

for the declared policy, field, valuation, and utility scope. The v0.5 refinement makes the
horizontal arrows **equivariant**: the square commutes up to declared interface, action, and
observation transports, with the outcome compared count-free and tile anisotropy re-entering
through roles (§12.6A).

The deliverable is therefore three things together, never a partition alone (§12.7):

$$\text{descriptor semantics}+\text{exact update law}+\text{response-preservation proof}.$$

**The open measurement.** Find a nontrivial pair $(d,\Theta)$ satisfying (ECL) on real
Straight 42 kernels, and run the class-count census on the existing probe corpus. That census
is the designated first measurement of the v0.5 amendment, and it sharpens v0.4 open problems
3–5 (§12.6A claim ledger, §17.4).

---

# 6. Vocabulary

| Term pair | The distinction | § |
|---|---|---|
| support / belief | support is the rule-compatible domain; belief is weights on it, possibly with smaller positive support | §2.4 |
| evidence $e$ / latent state $\mathcal Z$ | $e$ is retained *viewer-known* record; $\mathcal Z$ is *hidden* field state | §2.4, §2.5 |
| conditioning / revelation | analyst conditioning keeps the policy class fixed; player revelation makes a new information state and may enlarge it | §6.8 |
| rigid transport / fresh query | transport follows the same physical identity through a step; fresh query finds what satisfies the predicate now | §6.4, §6.5 |
| event certainty / identity certainty | true in every world vs. the same single answer in every world — four levels lie between | §4.4 |
| possibility → certainty → constant multiplicity → constant answer set → world-functional → identity certainty | the six-level hierarchy; one word such as "known" must not span it | §4.4 |
| payoff relabeling / common-knowledge scoring change | replace only $U_w$ with history, belief, field, information fixed vs. change the game so the field, likelihoods, and posterior all move | §8.7 |
| purpose constellation / orbit / Scheme cell | response-equality class for a declared carrier+purpose / group orbit under proved transport / world set denoted by a Fix — three different objects | §11.1 |
| sound / exact descriptor | sound: descriptor cells refine response classes; exact: they equal them | §12.1 |
| world / intensional / decision-geometry compression | worlds merged per cell / size and generality of the defining program / exposed policies, vertices, faces, regions | §12.2 |
| capped-exclusion / sampling | exact enumeration with what-was-excluded declared vs. drawing worlds — a walt build discipline, not a v0.4 claim; the basis supplies the exact fibers it acts on | project rule; cf. §2.1 |
| H / C / F treatments | hidden / common root then world revealed for the continuation / world revealed before the root | §10.3 |
| $\operatorname{BEATS}$ / $\operatorname{THREAT}$ | beats in a given led context / beats when the tile itself is led | §1.3 |
| output role / internal witness | returned by the query and transportable / existential proof machinery with no persistence, mass, identity, or value | §3.2, §12.6A |
| $w(d)$ / $Q(B,d)$ | payoff coefficient on a tile / exact strategic action value — never equal in general | §8.9 |
