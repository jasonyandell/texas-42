# Texas 42: Integrate the Worlds, Compress the Information Price

**Version:** 0.1  
**Date:** 2026-09-05  
**Status:** exploratory research note with finite proofs and independent executable checks. Not a replacement for the rules, foundation, or unified review. Not an opening solver or a gameplay-performance report.

## 0. The proposal

Stop demanding one representation that simultaneously compresses hidden worlds, reproduces observations, chooses lawful actions, and certifies optimality.

Two narrower objects may be much more economical:

1. An **exact integration circuit** for the posterior, exploiting fixed hand capacities rather than identifying strategically equivalent worlds.
2. A **small information-price certificate**, penalizing the advantage of seeing hidden information without requiring a uniformly accurate model of every continuation value.

These can meet through hand-separable features whose conditional expectations are exact contractions.

The immediate concrete result is a capacity-saturated covering-convolution kernel. On a full opening-sized universe of 21 unseen tiles, all 116,280 acting-hand completion weights matched an independent enumeration of all 399,072,960 legal ordered deals, including nonuniform synthetic hand factors. This validates a counting kernel, not the recursive game solve.

The broader hypothesis is that useful information-price certificates have much lower representational and discovery cost than the full response system. That is not established here.

## 1. What is inherited and what this note adds

The counted-belief manuscript already proposes seat-local posterior factors and an exact-cover interface, including ranked subset convolution as a candidate backend. The August 14 second-rung manuscript already proves a multistage centered-penalty dual and proposes small structural feature bases. The unification already centers the research program on action-indexed proof complexity. [P1–P4]

This note does not claim to invent those ideas. It supplies:

- A specialization from ranked subset convolution to ordinary covering convolution when hand capacities exhaust the unseen pool, with full-size independent integer verification.
- An explicit conditional-span stability theorem for continuation-potential penalties: successor-common value error cancels, even when its absolute magnitude is large.
- A fixed-feature convex synthesis objective and an exact algebraic connection between separable information-price features and counted beliefs.
- A Boolean first-irreversible-loss reformulation, useful for interpreting when information must arrive.

The transform identities belong to established zeta/Möbius convolution mathematics; the specialization is the relevant observation for this target, not a claim of a new general algorithm. Information-relaxation duality is also established research. [E1, E2]

# Part I. Capacity-saturated exact integration

## 2. Setup

Let the unseen universe be \(E\), with \(|E|=n\). The three hidden seats hold \(k_1,k_2,k_3\) tiles, where

\[
k_1+k_2+k_3=n.
\]

Extend each hand factor \(f_s:2^E\to\mathbb Q_{\ge0}\) by zero outside sets of size \(k_s\). The unnormalized posterior is

\[
W(H_1,H_2,H_3)=
\mathbf1\{H_1\mathbin{\dot\cup}H_2\mathbin{\dot\cup}H_3=E\}
\prod_{s=1}^3 f_s(H_s).
\]

This includes arbitrary local hand weights, not just uniform deals or tile-independent weights. It does **not** represent every possible joint belief. Correlated latent types, shared random tapes, or nonfactorizing priors require additional factors or explicit mixture components. The hands themselves are not independent: exact cover couples them. [P1, P2]

The same identity can use fixed root hands throughout a hand, or remaining hands on a shrinking universe, provided the representation and likelihood factors are proved equivalent to the intended posterior.

## 3. Saturation lemma

**Lemma.** If \(\sum_s|H_s|=|E|\), then

\[
H_1\cup H_2\cup H_3=E
\quad\Longleftrightarrow\quad
H_1\mathbin{\dot\cup}H_2\mathbin{\dot\cup}H_3=E.
\]

**Proof.** The union condition puts every tile into at least one hand. There are exactly \(|E|\) total tile slots. Every tile must therefore occur exactly once. The reverse implication is immediate. ∎

This is what removes the cardinality-rank bookkeeping for the particular contraction below. We are not computing arbitrary disjoint subset convolution at every output cardinality.

## 4. Exact normalizer by inclusion–exclusion

Define the subset zeta transform

\[
F_s(S)=(\zeta f_s)(S)=\sum_{H\subseteq S}f_s(H).
\]

Then the exact normalizer is

\[
\boxed{
Z=\sum_{S\subseteq E}(-1)^{n-|S|}F_1(S)F_2(S)F_3(S).
}
\]

**Proof.** Expanding the product, a triple \((H_1,H_2,H_3)\) receives coefficient

\[
\sum_{S\supseteq H_1\cup H_2\cup H_3}(-1)^{n-|S|}.
\]

This coefficient is one if the union is \(E\), and zero otherwise. The saturation lemma converts the covering condition into exact cover. ∎

The arithmetic can be performed over a commutative ring; subtraction is essential. Probabilistic interpretation uses nonnegative input factors and a positive normalizer.

## 5. All acting-hand completion weights in one contraction

For seat 1, define

\[
c_1(H)=\sum_{B\mathbin{\dot\cup}C=E\setminus H} f_2(B)f_3(C).
\]

Compute the ordinary covering, or OR, convolution

\[
g=\mu\big((\zeta f_2)(\zeta f_3)\big),
\]

where \(\mu\) is the inverse subset zeta transform. Expanding and inverting gives

\[
g(T)=\sum_{B\cup C=T}f_2(B)f_3(C).
\]

For \(|H|=k_1\), the complement has size \(k_2+k_3\). Every contributing pair has those total capacities, so covering that complement again implies disjointness. Hence

\[
\boxed{c_1(H)=g(E\setminus H).}
\]

Two forward transforms, pointwise multiplication, and one inverse transform produce all completion weights. The cost is \(O(n2^n)\) ring operations and \(O(2^n)\) working-array space, with several arrays in a straightforward implementation. A zeta transform or inverse uses \(n2^{n-1}\) additions or subtractions.

This is a specialization of standard covering convolution. Generic ranked subset convolution solves a broader query and has the familiar \(O(n^2 2^n)\) ring-operation bound. [E1]

Now

\[
Z=\sum_H f_1(H)c_1(H),
\qquad
\Pr(H_1=H)=\frac{f_1(H)c_1(H)}Z.
\]

For a deterministic seat-local field action \(\sigma_1(H,h)\),

\[
\Pr(a\mid h)=
\frac{\sum_{H:\sigma_1(H,h)=a} f_1(H)c_1(H)}Z.
\]

A stochastic field uses its action likelihood instead of an indicator.

Thus exact action masses require acting-hand classification and weighted buckets, not a replay of every complete deal. The field can still be expensive to classify. Repeating this operation at a huge number of distinct histories remains expensive.

### Reuse and derivatives

Completion weights \(c_1\) do not depend on \(f_1\). At one information state they support every action-likelihood bucket for seat 1. Zeta transforms of unchanged seat factors can also be cached under exact factor identities.

Because the partition function is multilinear,

\[
\frac{\partial Z}{\partial f_s(H)}=c_s(H).
\]

This is an exact polynomial identity, not an independence assumption or a neural approximation.

### Optional XOR version

Capacity saturation also means that \(H_1\mathbin\triangle H_2\mathbin\triangle H_3=E\) implies exact cover: every tile occurs an odd positive number of times, and the total number of occurrences is only \(n\). Consequently, an XOR convolution can recover the same saturated coefficients. The small verifier checks this independently with a Walsh transform. The OR transform is the main proposal because it avoids the inverse transform's division and uses fewer additions in this implementation.

## 6. Full-size check performed here

`cover_contraction.cpp` is independent of the project's engine, field code, and existing counting implementation.

It uses \(n=21\), \(k_s=7\), arrays of length \(2^{21}=2,097,152\), and checks all \(\binom{21}{7}=116,280\) first-seat hands.

**Uniform test.** Every completion weight is 3,432; the normalizer is 399,072,960. This is mainly a sanity test, since uniform counting has an elementary closed form.

**Nonuniform test.** Each possible hand of each seat receives a deterministic hash-generated weight in \(\{0,1,2,3\}\). The independent reference loops through all 399,072,960 legal ordered deals. Every completion weight agrees exactly. The resulting weighted normalizer is 1,345,725,937.

| Measurement | Recorded result |
|---|---:|
| Nonuniform completion weights matched | 116,280 / 116,280 |
| Legal deals visited by direct reference | 399,072,960 |
| Nonuniform transform calculation | 0.0927206 seconds |
| Nonuniform direct reference | 1.30848 seconds |
| Uniform transform sanity check | 0.102188 seconds |

These are **one uncalibrated container run**, not a benchmark of Walt or a comparison with its backends. The x86 reference used hardware parallel bit deposit to enumerate subsets efficiently. The source includes a portable fallback. Setup, factor classification, game-tree expansion, learned-feature evaluation, and proof search are not represented by these numbers.

The reported input bounds permit signed 64-bit arithmetic, including a conservative check on Möbius-inversion intermediates. Real posterior likelihood denominators and integer growth require a separate arithmetic-width strategy. Ring-operation complexity is not bit complexity.

Dense transforms should compete with sparse hand tables and the existing tile-pattern dynamic program, not replace them unconditionally. The only engine-side integration proposed here is another backend for the existing exact-cover interface, behind parity tests. No current repository implementation was audited for this optimization in this pass.

# Part II. Price information instead of predicting everything

## 7. Fix the actual target

Fix a finite root information state, root belief, terminal utility, and information-pure nonfocal behavioral field. Include the partner in that field. Let a complete scenario \(\xi\) contain the hidden deal and any persistent type or random tape needed to make all transitions deterministic.

Giving an evaluator access to \(\xi\) does not give the player access to it. Likewise, revealing \(\xi\) to the focal relaxed optimizer must not silently change what the other seats know or how their fixed field behaves. An old full-information minimax solver is not automatically this relaxation. [P1]

Use focal decision epochs \(t=0,\ldots,T-1\), with intervening field moves folded into transitions. Absorbing dummy transitions handle early termination. Terminal utility \(U\in[0,1]\) is measurable from the terminal lawful information state.

Let \(I_t\) retain the full lawful information, including relevant action history. For a fixed legal action \(a\), define

\[
P_t\psi(I,a)
=\mathbb E[\psi(I_{t+1})\mid I_t=I,\operatorname{do}(a)].
\]

Here \(\operatorname{do}(a)\) means to impose that action and use the target's lawful posterior and transition law. Do not infer hidden information from the fact that a clairvoyant optimizer selected it. Arrival probabilities may depend on earlier actions; complete information histories retain that dependence.

## 8. Any exactly centered potential gives a valid upper

Choose arbitrary real functions \(\psi_t\) on lawful information states, with terminal anchor \(\psi_T=U\). Define the summed innovation penalty

\[
M^\psi(\xi,\tau)
=\sum_{t=0}^{T-1}
\left[
\psi_{t+1}(I_{t+1})-P_t\psi_{t+1}(I_t,a_t)
\right].
\]

Every lawful policy has \(\mathbb E M^\psi=0\), by conditioning on its current information and selected action. The penalty is not nonnegative on each lawful path; zero mean is the property used.

For a fixed root action \(a\), define

\[
\boxed{
D_a(\psi)=
\mathbb E_\xi\max_{\tau:\tau_0=a}
\left[U(\xi,\tau)-M^\psi(\xi,\tau)\right],
}
\]

where the maximum covers all mechanically feasible focal action paths, including paths selected with knowledge of the complete scenario, while holding the nonfocal field fixed.

**Weak duality.** \(Q_a\le D_a(\psi)\).

**Proof.** A lawful policy beginning with \(a\) has the same expected penalized and unpenalized payoff. Its realized path is included in the relaxed maximum. Maximize over lawful policies. ∎

The important separation is:

> Accuracy of the predictor affects tightness. Correct centering and a sound relaxed upper determine validity.

This is a finite version of established information-relaxation duality, already developed in the project's second-rung note. [P3, E2]

## 9. Perfect penalties and the error that matters

Let \(V_t^*\) be the exact lawful optimal value at time \(t\). The Bellman equation is

\[
V_t^*(I)=\max_a P_tV_{t+1}^*(I,a).
\]

For any relaxed path, telescoping gives

\[
U-M^{V^*}
=V_0^*(I_0)+\sum_t
\left[P_tV_{t+1}^*(I_t,a_t)-V_t^*(I_t)\right].
\]

Every summand is nonpositive. With the first action fixed to \(a\), the first term and first summand combine to \(Q_a\); all subsequent summands remain nonpositive. A lawful optimal continuation makes them zero. Thus

\[
D_a(V^*)=Q_a.
\]

This establishes existence, not cheapness. Computing \(V^*\) would solve the original problem.

### Conditional-span stability

Write \(e_t=\psi_t-V_t^*\), with \(e_T=0\). Suppose that, for every reachable state/action at stage \(t\),

\[
\max_{I'\in\operatorname{supp}P_t(\cdot\mid I,a)}e_{t+1}(I')
-
\min_{I'\in\operatorname{supp}P_t(\cdot\mid I,a)}e_{t+1}(I')
\le\varepsilon_t.
\]

Then

\[
\boxed{0\le D_a(\psi)-Q_a\le\sum_t\varepsilon_t.}
\]

**Proof.** The difference from the perfect-penalty path payoff is

\[
\sum_t\left[P_te_{t+1}(I_t,a_t)-e_{t+1}(I_{t+1})\right].
\]

Each summand is at most \(\varepsilon_t\), because a conditional mean lies between the minimum and maximum on its support. The perfect-penalty payoff is at most \(Q_a\) for each path beginning with \(a\). Take the path maximum and outer expectation; weak duality gives the lower inequality. ∎

Consequently, errors constant across the possible successors of each state/action contribute nothing to the penalty error. The absolute prediction error can be very large while the information price remains exact.

There is a variance consequence. Write \(R_a^\psi(\xi)\) for the inner relaxed maximum and \(E=\sum_t\varepsilon_t\). The perfect-penalty maximum equals \(Q_a\) in every positive-mass scenario: Bellman inequalities bound all paths, and a lawful optimal continuation attains equality in each scenario. The same pathwise comparison in both directions therefore gives

\[
|R_a^\psi(\xi)-Q_a|\le E,
\qquad
\operatorname{Var}(R_a^\psi)\le E^2.
\]

The variance inequality follows from \(\operatorname{Var}(X)\le\mathbb E[(X-Q_a)^2]\). A sufficient price approximation can thus improve both upper-bound tightness and outer-sampling stability. This does not turn a sample variance estimate into a certified error bound; the same unknown-error and selection obligations remain.

This is not a claim that typical inaccurate predictors have the required structure. It is a statement of the correct invariance and an argument for studying centered error rather than only absolute value error. The theorem references unknown \(V^*\); using it as a numerical certificate requires independent error bounds.

## 10. Centering errors and optimization errors must be paid

An inaccurate value function can still generate a valid upper. An inaccurately centered penalty need not.

If approximate centers obey

\[
|\widetilde P_t\psi_{t+1}-P_t\psi_{t+1}|\le\eta_t
\]

uniformly over the relevant states/actions, and \(\widetilde D_a\) is the corresponding relaxed value, then

\[
\boxed{Q_a\le\widetilde D_a+\sum_t\eta_t.}
\]

This follows by comparing the two penalized path payoffs. A pointwise upper on cumulative center error can replace the uniform sum.

The smallest counterexample has no uncertainty: reward and terminal potential both equal one, while the alleged center equals zero. The penalty is one and the purported upper is zero, below the true value one. The missing center allowance is one.

Two further obligations remain independent. A sampled mean of relaxed values is not automatically a certified population upper, especially after feature selection or adaptive minimization. And a good feasible solution of the relaxed inner optimization is a **lower** on that inner maximum, not an upper. Use exact maximization or a verified inner upper, and appropriate outer statistical or exact-integration authority.

# Part III. The point of contact: separable information prices

## 11. Exact moments of separable structural features

Suppose a feature has the representation

\[
g(H_1,H_2,H_3)=\sum_{j=1}^r
 g_{j1}(H_1)g_{j2}(H_2)g_{j3}(H_3).
\]

Then

\[
\boxed{
\mathbb E_\beta g
=\frac1{Z(f_1,f_2,f_3)}
\sum_{j=1}^r
Z(f_1g_{j1},f_2g_{j2},f_3g_{j3}).
}
\]

**Proof.** Substitute the separable expansion in the exact-cover expectation, distribute the finite sums, and absorb each local multiplier into its seat factor. ∎

The products inside the partition function may be signed if the features are signed; the algebra remains valid though those modified inputs are not probability distributions.

Thus low separable feature complexity has an explicit computational payoff: it reduces exact centering to a controlled number of existing contractions. Neither arbitrary continuation potentials nor every useful Scheme predicate is guaranteed to have such a representation. A relational feature may require a sum over many cases, and that cost must be measured.

A one-seat event has one local nontrivial factor. A conjunction of specified one-seat properties factors similarly. More complicated holder/control/entry relationships can be tested for small verified expansions. Rich latent type structure can add indices or invalidate the three-hand form altogether.

## 12. Convex optimization over a fixed price basis

For declared features \(g_{tj}(\xi,I,a)\), define

\[
\lambda_t^\theta(\xi,I,a)
=\sum_j\theta_j
\left[g_{tj}(\xi,I,a)-\mathbb E(g_{tj}\mid I,\operatorname{do}(a))\right].
\]

Every coefficient vector gives zero-mean penalties under every lawful policy. For general hidden-state features, zero mean suffices for weak duality; the increments need not themselves be observable at the next public epoch. The potential construction in Part II is an adapted innovation process.

Let \(G_j(\xi,\tau)\) denote each feature's summed centered exposure along a path. Then

\[
D_a(\theta)=
\mathbb E_\xi\max_{\tau:\tau_0=a}
\left[U(\xi,\tau)-\theta^\top G(\xi,\tau)\right].
\]

**Proposition.** For a fixed finite target and feature basis, \(D_a\) is convex and piecewise linear in \(\theta\).

**Proof.** For each scenario it is a finite maximum of affine functions. A finite nonnegative weighted sum preserves convexity and piecewise linearity. ∎

For an exact maximizing path, \(-G(\xi,\tau)\) is a scenario-level subgradient; weighted sums produce a subgradient of the exact objective. This gives a direct synthesis target: lower a competing root action's certified upper, rather than fit all values indiscriminately.

This is a convex coefficient problem **around a potentially difficult combinatorial oracle**. Convexity does not make feature discovery, expectation calculation, or inner maximization inexpensive. Restricting the basis may leave a permanent dual gap. A nonlinear search over feature definitions also need not be convex.

The inner oracle's transposition structure may deteriorate when prices depend on full history. We should regard preserving cheap price evaluation and sufficient cache identity as part of the representation problem, not as a later engineering detail.

## 13. What to ask Scheme to describe

Instead of demanding that a Scheme descriptor reproduce all future observations, ask it to describe a certified information-price feature:

- the hidden-holder condition that makes an entry expendable;
- an incompatibility between preserving a count tile and retaining control;
- a follower-supply event that changes which commitment is safe;
- a publicly triggered point at which a formerly irrelevant distinction becomes consequential.

The feature may inspect hidden hands inside the verifier or relaxed evaluator. The actual player does not receive that information. The executed lower witness remains a lawful policy.

Individual feature usefulness is not enough: two features can matter only jointly. The prior XOR glue example already prevents a generic greedy/diminishing-returns assumption. [P1]

The research hypothesis is deliberately narrower than a low-rank game model:

> A small, cheaply centered, cheaply evaluated information-price basis may suffice to certify a useful action, even when the full response system has high predictive dimension.

# Part IV. A different interpretation of a hand: information deadlines

## 14. First irreversible loss of makeability

Fix a complete scenario and a focal action prefix. Let \(C_t\in\{0,1\}\) indicate whether some remaining focal continuation can make the contract when it is allowed to use that scenario, while the other seats retain the same fixed behavioral rules.

Before selecting the next action, \(C_t\) is the maximum over its possible continuations. After the action and intervening deterministic field evolution, the continuation set is restricted. Consequently,

\[
\ell_t=C_t-C_{t+1}\in\{0,1\},
\qquad
C_T=U.
\]

Telescoping yields

\[
\boxed{U=C_0-\sum_t\ell_t.}
\]

At most one \(\ell_t\) is positive along a path. If the scenario is initially makeable but the realized policy fails, exactly one focal commitment first destroys every making continuation. Therefore

\[
\mathbb E U
=\mathbb E C_0-
\Pr(\text{a first destruction of makeability occurs}).
\]

This is an exact reformulation for complete scenarios and Boolean utility, not a statement about stochastic partially revealed values without the scenario augmentation.

### What it suggests

A hand need not be understood first as a quest to identify every tile. It can be understood as a race between **observations arriving** and **options expiring**.

A probe is valuable when it resolves a consequential ambiguity before an irreversible commitment. A lead can be bad because it forces commitment before enough information arrives. A low-value-looking play can be good because it preserves different viable continuations until their distinguishing observation becomes public.

These are proposed explanatory targets, not claims that the verified 42 corpus already exhibits a cheap deadline structure.

### What it does not simplify automatically

The last individually avoidable loss is not necessarily the earliest strategic error. An earlier action can preserve a winning continuation in every physical world yet force an unresolved information conflict later. Detecting \(C_t\), finding the critical step, and reasoning about its policy-dependent occupancy may still require substantial work. Greedily minimizing only the next step's destruction risk is not justified.

The useful object to search for is a compact, verified **commitment/observation interaction**, potentially represented by a small dynamic controller, not a static list of important tiles.

# Part V. Receipts and the next experiment

## 15. Independent finite verification

`verify_improvisation.py` uses only Python integers and `fractions.Fraction`.

It constructs 256 finite three-decision hidden-scenario games, each with four positively weighted scenarios, two actions per decision, action-dependent observations, and Boolean terminal utility. It tests five potential families per game, including exact values, arbitrary values, zero internal values, small perturbations, and exact values plus large successor-common offsets.

Recorded checks:

| Check | Count |
|---|---:|
| Weighted small exact-cover cases | 128 |
| Small completion weights compared, including OR and XOR routes | 2,524 |
| Three-decision games | 256 |
| Potential families checked for upper validity and span allowance | 1,280 |
| Exact zero-mean lawful-policy checks | 10,240 |
| First-irreversible-loss path checks | 8,192 |
| Largest nonroot absolute potential error with exact dual recovery in the constructed offset family | 1,019 |

Every reported check passed. The offset family is constructed to lie in the centering operator's null directions; its behavior is not an empirical claim about ordinary learned predictors. These are finite regression checks supporting the displayed proofs, not Lean kernel proofs or Texas 42 gameplay experiments.

## 16. A decisive experiment inside the existing architecture

Use the existing target and proof-state APIs. Do not build a new player for this note.

**First establish contraction parity.** Add the saturated-cover backend as a competing exact authority. Test real posterior factors from conditioned successors, not only uniform roots. Measure arithmetic width, density, factor preparation, field classification, and amortized reuse as well as transform time.

**Then freeze a small price basis.** Mine features on training roots or cheaper sampled solves, but publish the basis and posterior-centering contracts before evaluating held-out roots. Include a modest interaction family rather than discarding every feature with no isolated effect.

**Optimize prices and retain a lawful witness.** Maintain a materialized policy with lower value and action-indexed information-price uppers. Use exact or certified inner maximization and exact or statistically valid outer expectations. For the actual policy \(\widehat\rho\), the executable regret receipt remains

\[
Q^*-V(\widehat\rho)\le\max_a U_a-L_{\widehat\rho}.
\]

**Count the whole bill.** Include feature discovery, centering, relaxed search, inner proof generation, outer expectation, and lower-policy extraction/evaluation. Report best-action closure or a predeclared useful regret band against total work. Do not use an exact opening solve to manufacture an allegedly cheap initial witness.

A positive outcome is an early decision certificate with small total cost and a small priced feature set, even while the full posterior and response structure remain intricate. A negative outcome could be that useful price features have large separation complexity, centers fragment across histories, or penalties destroy the cheap inner solver. Those are distinct failures and should remain distinct in the record.

## 17. Final assessment

The strongest claim supported here is not that opening 42 has become easy. It is that we now have a concrete exact integration specialization, checked at the opening's full hidden-hand cardinality, plus a sharper approximation target for information-consistent upper bounds.

The question worth pursuing is:

> Can we integrate the lawful posterior efficiently and describe the costly information conflicts cheaply enough that the upper meets a good executable policy before most of the game is solved?

That asks neither for every world to become equivalent nor for a predictor to know every value accurately. It asks for the distinctions that matter to have tractable weights and tractable prices.

# Source register

**[P1]** *Texas 42: A Unified Mathematical Core*, v0.1, 2026-09-05. Sections 2, 4, 7, 10, 13, and 18. Current-conversation review.

**[P2]** *DESIGN-walt-counted-belief-sandwich-v0.1.md*. Parts V–VII, particularly §§24–26: exact-cover interface, acting-hand weights, ranked subset convolution, and backend costs. Recovered from the user's Library.

**[P3]** *Second-Rung Gluing: Policy-Dependent Occupancies, the Slack–Tax Interchange Law, and Exact Martingale Penalties for Straight Texas 42*, v0.1, 2026-08-14. §9, particularly stagewise centering, weak/strong duality, and §9.3 feature-based penalties. Recovered from the user's Library. Earlier rung-count corrections in that document govern the preceding first-rung note; this note does not use an uncorrected rung count.

**[P4]** *Decision-Sparse Exact Solving: Nonanticipativity Taxes and a Compositional Plan Calculus for Straight Texas 42*, v0.1, 2026-08-14. Historical predecessor to [P3].

**[E1]** Andreas Björklund, Thore Husfeldt, Petteri Kaski, Mikko Koivisto. *Fourier meets Möbius: fast subset convolution*. arXiv:cs/0611101, submitted 2006; STOC 2007. Primary source for the general fast subset-convolution framework and covering/partitioning context. The saturated coefficient derivation in this note is supplied explicitly.

**[E2]** David B. Brown, James E. Smith, Peng Sun. *Information Relaxations and Duality in Stochastic Dynamic Programs*. Author manuscript dated 2008-11-20; subsequently Operations Research (2010). Primary source for general information-relaxation duality and ideal penalties; the finite target, conditional-span statement, and proofs used here are written out above.

# Reproduction

Python 3.10 or later (standard library only):

```sh
python verify_improvisation.py
```

The script writes `verification_results.json` beside itself. Its fixed random seed selects a reproducible finite test corpus; all comparisons are exact.

GCC or Clang with C++17 and the supported compiler integer intrinsics:

```sh
g++ -O3 -std=c++17 -march=native cover_contraction.cpp -o cover_contraction
./cover_contraction > cover_results.json
```

`-march=native` is optional and makes the executable specific to the compilation machine. Only source is distributed. The reference enumerator uses hardware bit deposit when the compiler enables BMI2, otherwise its software fallback. Its time will vary substantially by machine and build. Assertions/checks fail with a nonzero exit status.
