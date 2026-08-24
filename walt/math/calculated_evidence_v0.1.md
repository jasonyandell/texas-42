# HANDOFF — Calculated Evidence for Unified Walt

## Anytime-valid adaptive settlement, exact-fiber escalation, and the end of magic sample counts

**Status:** EXPLORATORY mathematical proposal. Intended for verbatim intake under the project’s `walt/math/` convention. Nothing is promoted by this document’s existence.

**Date:** 2026-08-24

**Repository snapshot reviewed:** `jasonyandell/texas-42` main at `4231cb248e21aaea809bd188f79ffafedba32123`.

**Current engineering basis:** one unified `walt` crate with the modules `rules`, `kernel`, `geom`, `strat`, `spec`, `carrier`, and `solver`; a thin `walt-wasm`; and the deliberately separate GPU trio. The unified `kernel` already owns exact fiber counting, lazy full-fiber enumeration, and exact uniform sampling. The live `solver` still contains the sampling-stack player and its older outer sampling path.

**Mathematical basis:** this document extends the signed-pivotal treatment in `walt/math/signed_pivotal_geometry_v0.1.md`, read through its maintained intake companion and the binding SP-A1..SP-A12 rulings. It also reads `SCENARIO-PLAYER.md`, `TILT-AUDIT.md`, and `LEVEL2-PROBE.md` as current exploratory project documents. It does not silently promote any of them.

**Vocabulary:** the SP-A names govern: **frozen policy**, **pivotal mass** `q`, **tilt** `τ`, **gap** `g`, **pivotal cover**, and **pivotal win share**. The word *certificate* remains reserved elsewhere and is not used for the objects introduced here. A probabilistic result is a **δ-settlement**. An exhaustive result is **exact**.

---

## 0. Executive ruling

The fixed sample counts currently visible in Walt—`40`, `160`, `200`, `800`, `16×`, and similar—must leave the correctness path.

They may remain as:

- replay fixtures;
- performance defaults in an explicitly heuristic fallback;
- historical experiment coordinates;
- batch sizes that affect throughput but not semantics.

They may not determine when a mathematical result is declared settled.

The replacement has three layers.

### Layer A — exact anytime-valid evidence

For every pair of **fixed frozen policies**, evaluate them on one common stream of exactly uniform worlds. Maintain an exact-rational evidence process. Stop only when the evidence crosses a threshold calculated from a declared error budget `δ`.

There is no fixed `n`.

An easy decision may stop after a handful of pivotal worlds. A hard decision may continue. A true tie may remain unresolved. That is correct behavior.

### Layer B — calculated cost and routing

For every unresolved pair, continuously report:

- current exact evidence;
- exact evidence threshold;
- pivotal mass estimate `q̂`;
- tilt estimate `τ̂`;
- gap estimate `ĝ`;
- information-rate estimate;
- best-case additional pivotal observations;
- forecast raw worlds to settlement;
- projected sampled cost;
- projected exact-enumeration cost.

This replaces “200 is rough and 2,000 is usually good” with a per-decision calculation.

### Layer C — monotone escalation to exactness

The unified kernel knows the exact fiber size and can enumerate the full fiber. Walt should therefore compare the projected adaptive-sampling cost with the measured cost of exact full-fiber evaluation.

When exact evaluation is cheaper, switch to it.

The resulting architecture is:

> **sample only while sampling is the cheaper truthful computation; otherwise enumerate and know.**

This is the central engineering consequence of the reorganization. The exact kernel and the live player now inhabit one crate. They should become one decision procedure.

---

## A. Current-state findings after the reorganization

The engineering recenter succeeded. The next session should not reopen the crate-layout question.

### A.1 One live Walt now exists

The workspace has one authoritative player crate, `walt/walt`, divided into the seven modules named in the header. `walt-wasm` is a thin delivery boundary, and the GPU crates remain separate for sound build reasons. The old crate proliferation is now provenance, not architecture.

### A.2 The exact and sampled machinery are finally adjacent

The unified `kernel` already provides:

- exact construction of the lawful current-remainder fiber;
- exact `|Φ(C)|` by integer dynamic programming;
- exact uniform one-world sampling driven by the same completion counts;
- lazy exhaustive enumeration.

The unified `solver` contains the live sampling-stack player, `level1_evaluate`, the faithful replay-race experiment, and the block-race/race-refine experiments.

This is the desired starting point: no new solver family is needed.

### A.3 One semantic duplication remains

The live solver still carries its older `sample_belief` shuffle-and-reject path even though the canonical kernel now owns an exact count-driven sampler. The code was unified physically before these two meanings were unified semantically.

The adaptive correctness path should close that seam by constructing one `Kernel` and reusing its `FiberDp` for count, sample, and eventual enumeration. The old sampler can remain temporarily as a regression oracle or heuristic compatibility path.

### A.4 The current mathematical gate is correctly named

The kanban item `adaptive-sampling-intake` already names the right next move: replace fixed-`n` behavior with decision-specific evidence and make that the gate for the level-2 field-swap probe. `LEVEL2-PROBE.md` likewise waits on this mathematics.

### A.5 The old racer should not be “fixed in place” into authority

The faithful Boolean replay race is the right mathematical object and needs cheap frozen-policy execution. The block sign racer is useful exploratory code but has the wrong target and test for a correctness claim. The clean unified architecture now allows both facts to coexist without another rewrite:

- preserve the heuristic for play and comparison;
- build the proof-path evidence beside it;
- share rules, fiber, policy, and replay authorities;
- keep the result types distinct.

### A.6 Immediate orientation for the implementer

The first fresh session should do **math intake and evidence infrastructure**, not another arena run, not another magic-`n` calibration, and not another crate movement.

The minimum coherent vertical slice is:

1. canonical `Kernel` adapter for one live root;
2. exact pivotal evidence arithmetic;
3. one frozen-policy pair on one common indexed world stream;
4. `DeltaSettled` versus `Unresolved`;
5. exact full-fiber comparison on a small root.

That slice proves the architecture before it touches live defaults.

---

## 1. The result types must be explicit

Walt currently has several mathematically different accomplishments that can all look like “picked tile 6-4.” They must not share one unlabeled result type.

This document proposes the following semantic ladder.

### 1.1 `ExactFiberRoot`

The complete outer fiber was evaluated under the declared field model, and the information-consistent focal optimization was solved over that complete fiber.

This is an exact model-relative root result, subject to the standing solver-correctness obligations.

### 1.2 `ExactFrozenSet`

Every named frozen candidate policy was evaluated over the complete outer fiber, and the best member of that fixed set was selected exactly.

This closes the measure and response questions for the candidate set. It does not prove that no omitted continuation policy is better.

### 1.3 `DeltaSettled`

A named frozen candidate set was evaluated on an independent common random world stream. The reported winner is best in that fixed set except on an event whose total declared probability is at most `δ` under the sampling law.

This is probabilistic, not exact.

### 1.4 `EpsilonEquivalent`

The available evidence establishes that all surviving candidates are within a declared utility tolerance `ε`, with declared error probability at most `δ`.

This is the correct response to a practically irrelevant modeled tie. It is not an exact tie statement.

### 1.5 `Unresolved`

The correctness path reached its wall-clock, world, or memory limit without satisfying an exact or probabilistic stopping condition.

`Unresolved` is a successful honest output. It must not silently become an index tie-break.

### 1.6 `HeuristicFallback`

An explicitly named fallback chose a move after the correctness path returned `Unresolved`. The fallback may use the current sampled solver, a fixed budget, a plan library, or another empirical player.

The result carries no exact or `δ`-settled claim.

### 1.7 The distinction is permanent

A UI may display all six as a tile choice. Logs, APIs, reports, and experiments must preserve the result kind.

In particular:

> **A sample cap is a resource limit, never a proof rule.**

---

## 2. Mathematical setting

Fix one outer information state `I`, declaration, bid, Boolean `pmake` utility, and declared field model.

Fix two deterministic information-consistent frozen policies `ρ_a` and `ρ_b`. Under the current deterministic modeled field, one physical world determines one terminal outcome for each policy. Under a future stochastic field, append the persistent tape to the world; every statement below then applies to scenarios rather than bare physical worlds.

Let

\[
 u_a(\xi),u_b(\xi)\in\{0,1\}
\]

be their terminal make indicators and define

\[
 Y(\xi)=u_a(\xi)-u_b(\xi)\in\{-1,0,+1\}.
\]

Write

\[
 p_+=\Pr(Y=+1),\qquad
 p_-=\Pr(Y=-1),\qquad
 p_0=\Pr(Y=0).
\]

Then

\[
 q=p_++p_-
\]

is pivotal mass,

\[
 \tau=\frac{p_+-p_-}{p_++p_-}
\]

when `q>0` is tilt, and

\[
 g=\mathbb E[Y]=p_+-p_-=q\tau
\]

is the value gap.

The old fixed-pair hardness coordinate remains

\[
 H=\frac{q-g^2}{g^2}
  =\frac{1}{q\tau^2}-1,
\]

when `g≠0`.

The new question is not merely how to estimate these quantities. It is:

> **How can Walt stop at a data-dependent time without invalidating the error statement?**

The answer is an exact evidence process.

---

## 3. Exact Bernoulli-threshold evidence

The pivotal decision is one instance of a more general object that Walt also needs for bidding, wake-up detection, and probability thresholds.

Let

\[
 B_1,B_2,\ldots\in\{0,1\}
\]

be independent Bernoulli observations with success probability `p`. More generally, the proof below permits a predictable sequence whose conditional success probabilities all obey the tested null.

Fix a rational threshold

\[
 c\in(0,1).
\]

We first test

\[
 H_0:p\le c
\]

against `p>c`.

### CE-T1 — upper-threshold evidence process

After `s` successes and `f` failures, define

\[
 \boxed{
 E^{>}_{s,f}(c)
 =\frac{1}{1-c}
   \int_c^1
   \left(\frac{r}{c}\right)^s
   \left(\frac{1-r}{1-c}\right)^f
   \,dr.
 }
\]

Then, under every law satisfying `p≤c`, the process obtained by updating this value after every observation is a nonnegative supermartingale beginning at one.

Consequently, for every `α∈(0,1)`,

\[
 \Pr_{p\le c}
 \left(
   \sup_n E^{>}_{S_n,F_n}(c)\ge\frac1\alpha
 \right)
 \le\alpha.
\]

Therefore Walt may examine the evidence after every world, every block, or every preemption point and stop the first time

\[
 E^{>}_{s,f}(c)\ge\frac1\alpha.
\]

No peeking correction is added afterward. Sequential validity is already built into the process.

### Proof

Fix one alternative `r≥c` and define the one-step likelihood factor

\[
 L_r(B)
 =\left(\frac{r}{c}\right)^B
  \left(\frac{1-r}{1-c}\right)^{1-B}.
\]

If the conditional success probability is `p≤c`, then

\[
 \mathbb E[L_r(B)]
 =\frac{pr}{c}
  +\frac{(1-p)(1-r)}{1-c}
 =1+\frac{(p-c)(r-c)}{c(1-c)}
 \le1.
\]

Thus the product of the factors is a nonnegative supermartingale. The displayed `E^>` is the uniform mixture of those products over `r∈[c,1]`; a nonnegative mixture of supermartingales is a supermartingale. Ville’s inequality gives the crossing bound.

No asymptotics and no floating-point approximation enter the correctness statement.

### CE-T2 — lower-threshold evidence process

To test

\[
 H_0:p\ge c
\]

against `p<c`, define

\[
 \boxed{
 E^{<}_{s,f}(c)
 =E^{>}_{f,s}(1-c).
 }
\]

The same argument gives

\[
 \Pr_{p\ge c}
 \left(
   \sup_n E^{<}_{S_n,F_n}(c)\ge\frac1\alpha
 \right)
 \le\alpha.
\]

### 3.1 Exact rational form

The integral is not a request for numerical quadrature.

Put

\[
 R=\frac{1-c}{c}.
\]

Using `r=c+(1-c)t`,

\[
 E^{>}_{s,f}(c)
 =\int_0^1(1+Rt)^s(1-t)^f\,dt.
\]

Expanding the first factor gives the exact finite sum

\[
 \boxed{
 E^{>}_{s,f}(c)
 =\sum_{i=0}^{s}
   \binom{s}{i}R^i
   \frac{i!f!}{(i+f+1)!}.
 }
\]

For rational `c`, every term is rational. `BigInt`/`BigRational` arithmetic suffices. A threshold comparison is an integer cross-multiplication.

There is no float in the value path.

### 3.2 What the threshold means

The evidence threshold is calculated from the declared error allocation:

\[
 T=\frac1\alpha.
\]

It is not a tuned sample count.

The observations determine how long reaching `T` takes.

---

## 4. Signed-pivotal specialization

For a frozen pair, observe the common-world outcome

\[
 Y_i\in\{-1,0,+1\}.
\]

Let

\[
 A_n=\#\{i\le n:Y_i=+1\},
\]

\[
 B_n=\#\{i\le n:Y_i=-1\},
\]

and let the remaining observations be nonpivotal.

Conditional on a pivotal observation,

\[
 \theta
 =\Pr(Y=+1\mid |Y|=1)
 =\frac{1+\tau}{2}.
\]

The sign of `g` is the sign of `θ-1/2` whenever `q>0`.

### CE-T3 — exact pivotal-direction evidence

To establish that policy `a` is better than policy `b`, test

\[
 H_0:g\le0,
\]

which is equivalent to `θ≤1/2` on the pivotal component.

Define

\[
 \boxed{
 E^+_{a,b}
 =\int_0^1(1+t)^a(1-t)^b\,dt.
 }
\]

Here `a=A_n` and `b=B_n`. A nonpivotal observation leaves the evidence unchanged.

This is nevertheless a valid process on the **raw world stream**, not merely on a retrospectively selected pivotal subsample. For a fixed mixture component `r≥1/2`, use the raw-world multiplier

\[
 L_r(Y)=
 \begin{cases}
  2r,&Y=+1,\\
  2(1-r),&Y=-1,\\
  1,&Y=0.
 \end{cases}
\]

Under `g≤0`, or equivalently `θ≤1/2`,

\[
 \mathbb E[L_r(Y)]
 =1-q+q\bigl(2\theta r+2(1-\theta)(1-r)\bigr)
 \le1.
\]

Thus waiting through an arbitrary number of nonpivotal worlds is safe. Those worlds cost time but do not create fake directional evidence.

The evidence that `b` is better than `a` is

\[
 E^-_{a,b}=E^+_{b,a}.
\]

### 4.1 Closed integer form

Let

\[
 k=a+b.
\]

Then

\[
 \boxed{
 E^+_{a,b}
 =
 \frac{
   \displaystyle\sum_{x=0}^{a}\binom{k+1}{x}
 }{
   (k+1)\binom{k}{a}
 }.
 }
\]

This is the preferred implementation for the Boolean pivotal engine.

It needs only exact binomial coefficients, an exact integer prefix sum, and one rational comparison.

Useful anchors for intake verification are:

\[
 E^+_{0,0}=1,
\qquad
 E^+_{1,0}=\frac32,
\qquad
 E^+_{0,1}=\frac12,
\]

\[
 E^+_{2,0}=\frac73,
\qquad
 E^+_{1,1}=\frac23,
\qquad
 E^+_{2,1}=\frac{11}{12},
\]

\[
 E^+_{3,0}=\frac{15}{4}.
\]

When every pivotal observation favors `a`,

\[
 \boxed{
 E^+_{a,0}=\frac{2^{a+1}-1}{a+1}.
 }
\]

For a single one-sided test at `α=1/128`, nine consecutive favorable pivots are insufficient because

\[
 E^+_{9,0}=\frac{1023}{10}<128,
\]

while ten are sufficient because

\[
 E^+_{10,0}=\frac{2047}{11}>128.
\]

That is a **calculated pivotal requirement** in the strongest possible tilt regime. The raw-world requirement still depends on how often pivots occur.

### 4.2 The nonpivotal count is still useful

Although `N_0` does not enter directional evidence, it estimates pivotal mass:

\[
 \widehat q=\frac{a+b}{n}.
\]

The pair’s empirical coordinates remain

\[
 \widehat\tau=\frac{a-b}{a+b}
\]

when `a+b>0`, and

\[
 \widehat g=\frac{a-b}{n}.
\]

These coordinates predict cost and diagnose the decision. They do not replace the evidence threshold.

---

## 5. Multiple candidates: a decision-level risk ledger

Suppose the frozen candidate set contains `m` policies.

There are

\[
 M=\binom{m}{2}
\]

unordered pairs and

\[
 2M=m(m-1)
\]

directed claims.

Fix a decision-level error budget

\[
 \delta_{\mathrm{dec}}\in(0,1).
\]

The simplest auditable allocation is

\[
 \alpha_{i>j}
 =\frac{\delta_{\mathrm{dec}}}{m(m-1)}
\]

for every ordered pair `i≠j`.

The common directed-edge evidence threshold is therefore

\[
 \boxed{
 T_{\mathrm{edge}}
 =\frac{m(m-1)}{\delta_{\mathrm{dec}}}.
 }
\]

Run every opened pair on the same ordered world stream. Draw a settled edge

\[
 i\longrightarrow j
\]

when

\[
 E^+_{i,j}\ge T_{\mathrm{edge}}.
\]

By the union bound and the anytime crossing theorem,

\[
 \Pr(\text{any false directed edge is ever drawn})
 \le\delta_{\mathrm{dec}}.
\]

This remains true when:

- pair examinations are adaptive;
- candidates are eliminated as evidence arrives;
- worlds arrive in arbitrary batch sizes;
- the current provisional leader changes;
- the computation is paused and resumed.

The candidate set itself must remain fixed during the evidence epoch.

### 5.1 Safe elimination

A candidate may be removed when any live candidate has a settled edge into it.

On the event that no false edge is ever drawn, a truly best candidate cannot be eliminated by a worse candidate. When one candidate remains, it is a true maximizer of the fixed candidate set. If several policies have exactly equal maximum value, the procedure may leave several survivors forever; that is honest.

### 5.2 A sharper later allocation

The equal all-pairs allocation is deliberately conservative and easy to audit.

A later version may open directed comparisons one at a time and give the `ℓ`-th newly opened test

\[
 \alpha_\ell
 =\frac{\delta_{\mathrm{dec}}}{\ell(\ell+1)},
\]

because

\[
 \sum_{\ell=1}^{\infty}
 \frac{1}{\ell(\ell+1)}=1.
\]

That spends risk only on edges actually opened. It should not be the first implementation; deterministic all-pairs allocation is the cleaner intake target.

### 5.3 Candidate-set mutation starts a new epoch

If discovery produces a new policy, or an existing policy’s freeze tuple changes, old evidence does not apply to the new candidate set.

The safe choices are:

1. start a new evaluation epoch on fresh worlds;
2. retain old pair processes only for policy identities that are literally unchanged, while opening fresh processes for every new pair under a separately accounted risk allocation;
3. discard the old epoch entirely.

The implementation must not silently reinterpret old outcomes as observations of a newly optimized policy.

---

## 6. Risk across a hand or a research run

A decision-level `δ` is not enough when many adaptive decisions are made.

Let

\[
 \delta_{\mathrm{run}}
\]

be the declared total sampling-error budget for a hand, match, experiment, or other named run.

For the `d`-th decision event, allocate

\[
 \boxed{
 \delta_d
 =\frac{\delta_{\mathrm{run}}}{d(d+1)}.
 }
\]

Then

\[
 \sum_{d=1}^{\infty}\delta_d
 =\delta_{\mathrm{run}}.
\]

This supports an unknown number of decisions without a magic maximum.

Within decision `d`, divide `δ_d` among:

- directed candidate comparisons;
- optional practical-equivalence tests;
- any rediscovery epochs;
- any recursively sampled inner-policy decisions whose errors are included in the same claim.

The allocation is configuration and must be serialized into the result. A `δ` value without its scope is meaningless.

### 6.1 Exact results spend no risk

Full-fiber exact evaluation consumes no sampling-error budget. A run that escalates to exactness may close the corresponding risk ledger entry without using its remaining allocation.

---

## 7. The information rate: what actually predicts raw worlds

The old hardness coordinate

\[
 H=\frac{1}{q\tau^2}-1
\]

is a useful small-gap scale. The exact evidence process reveals the more fundamental asymptotic coordinate.

Let

\[
 \theta=\frac{1+\tau}{2}.
\]

Define the Bernoulli divergence from a fair pivotal sign:

\[
 D_{1/2}(\theta)
 =D_{\mathrm{Ber}}(\theta\Vert 1/2)
\]

\[
 =\theta\ln(2\theta)
  +(1-\theta)\ln(2(1-\theta)).
\]

In terms of tilt,

\[
 \boxed{
 D_{1/2}(\tau)
 =\frac{1+\tau}{2}\ln(1+\tau)
  +\frac{1-\tau}{2}\ln(1-\tau),
 }
\]

with `0 ln 0` interpreted as zero.

The **raw-world pivotal information rate** is

\[
 \boxed{
 \mathcal I=qD_{1/2}(\tau).
 }
\]

For a true positive gap, the logarithm of the mixture evidence grows at leading order like

\[
 n\mathcal I.
\]

Thus the leading-order raw-world forecast to reach threshold `T` is

\[
 \boxed{
 n_{\mathrm{forecast}}
 \approx
 \frac{\ln T}{qD_{1/2}(\tau)}.
 }
\]

For the equal all-pairs allocation,

\[
 \boxed{
 n_{\mathrm{forecast}}
 \approx
 \frac{
   \ln\!\bigl(m(m-1)/\delta_{\mathrm{dec}}\bigr)
 }{
   qD_{1/2}(\tau)
 }.
 }
\]

This is a forecast, not the stopping rule.

### 7.1 Relation to `H`

Near a genuine tie,

\[
 D_{1/2}(\tau)
 =\frac{\tau^2}{2}
  +\frac{\tau^4}{12}
  +\frac{\tau^6}{30}
  +\cdots.
\]

Therefore, for small `|τ|`,

\[
 n_{\mathrm{forecast}}
 \approx
 \frac{2\ln T}{q\tau^2}
 =2(H+1)\ln T.
\]

This explains why `H` worked as a first cost coordinate and sharpens it away from the knife edge.

### 7.2 Four regimes, now operational

1. **Small `q`, large `|τ|`.** The sign is easy once a pivot appears; raw cost is waiting for pivots. Structural covers or exact conditioned generation have high potential value.
2. **Large `q`, large `|τ|`.** Evidence grows quickly. Uniform sampling is fine.
3. **Any `q`, small `|τ|`.** The modeled decision is genuinely difficult. More raw worlds may be expensive because pivotal signs nearly balance.
4. **`q=0` or `τ=0`.** Strict directional evidence has zero asymptotic growth. Exact evaluation, an equivalence tolerance, or `Unresolved` is required.

### 7.3 Information, not disagreement alone

A field upgrade can increase `q` while decreasing `|τ|`. It can also decrease `q` while making the surviving pivots unanimous.

The quantity that predicts sampling cost is not `q` alone and not the absolute gap alone. It is

\[
 qD_{1/2}(\tau).
\]

This matters directly to the level-2 probe.

---

## 8. Exact evidence debt and calculated remaining work

The implementation should report more than `n` and an estimated percentage.

### 8.1 Exact evidence debt

For one directed pair, let the threshold be `T` and current exact evidence be `E`.

Define the exact debt ratio

\[
 \boxed{
 R_{\mathrm{debt}}=\frac{T}{E}.
 }
\]

Settlement occurs when `R_debt≤1`.

No logarithm is required in the correctness path. The UI may display a logarithmic score, but the engine compares exact integers or rationals.

### 8.2 Best-case additional pivots

For current pivotal counts `(a,b)`, define

\[
 h^+_{\min}(a,b;T)
 =\min\{h\ge0:E^+_{a+h,b}\ge T\}.
\]

This is the exact minimum number of additional favorable pivotal observations that could settle the positive direction.

Similarly,

\[
 h^-_{\min}(a,b;T)
 =\min\{h\ge0:E^+_{b+h,a}\ge T\}.
\]

These are lower bounds on further pivotal work. They are cheap to compute by exact recurrence or monotone search.

They support an immediate routing rule:

> If even the best-case pivotal path costs more than exact enumeration, stop sampling and enumerate.

### 8.3 Estimated raw-world burden

When `q̂>0`, a crude raw conversion is

\[
 \widehat n^+_{\min}
 =\frac{h^+_{\min}}{\widehat q}.
\]

This remains a forecast. It must be labeled as such.

### 8.4 Exact forecast dynamic program under a declared predictive law

Suppose a forecast—not a correctness claim—uses rational predictive probabilities

\[
 \widetilde p_+,
 \widetilde p_-,
 \widetilde p_0,
\qquad
 \widetilde p_++\widetilde p_-+\widetilde p_0=1.
\]

Let `F_h(a,b)` be the probability of reaching either evidence threshold within at most `h` additional raw worlds.

Set `F_h(a,b)=1` at an already settled state, `F_0(a,b)=0` otherwise, and recurse:

\[
 \boxed{
 F_h(a,b)
 =\widetilde p_+F_{h-1}(a+1,b)
  +\widetilde p_-F_{h-1}(a,b+1)
  +\widetilde p_0F_{h-1}(a,b).
 }
\]

All arithmetic is rational. The smallest `h` with

\[
 F_h(a,b)\ge\gamma
\]

is an exact computation **conditional on the declared predictive law**.

It is still only a forecast because the predictive probabilities are estimated. The `δ`-settlement remains governed solely by the evidence threshold.

### 8.5 The per-decision refinement vector

For each unresolved pair, persist:

\[
 \boxed{
 \mathcal R_{a,b}
 =(
   n,a,b,n_0,
   \widehat q,\widehat\tau,\widehat g,
   E^+,E^-,T,
   R_{\mathrm{debt}},
   \widehat{\mathcal I},
   h^+_{\min},h^-_{\min},
   \widehat n_{\mathrm{forecast}},
   C_{\mathrm{sample}},C_{\mathrm{exact}}
 ).
 }
\]

Here `n_0` is the number of nonpivotal outcomes, not an inner-mind sample size.

This vector answers both questions:

- “How much more evidence is likely required?”
- “Where should the next unit of compute go?”

---

## 9. True ties and practical equivalence

No sound sequential method can guarantee finite directional settlement when the true gap is zero.

That is not a defect. A procedure that always forces a winner eventually must sometimes manufacture one from noise.

Walt needs three declared modes.

### 9.1 Strict mode

Only exact inequality or a `δ`-settled directional edge ends the comparison.

A true or extremely close tie may return `Unresolved`.

### 9.2 Practical-equivalence mode

Fix a rational utility tolerance

\[
 \varepsilon>0.
\]

The goal is to establish

\[
 |g|<\varepsilon
\]

rather than exact equality.

Section 10 gives a general exact-rational bounded-mean evidence engine. Apply CE-T5 at `c=ε` to reject the null `g≥ε`, and apply CE-T4 at `c=-ε` to reject the null `g≤-ε`. Thus the two required settlements are

\[
 g<\varepsilon
\]

and

\[
 g>-\varepsilon.
\]

When both are settled under separately allocated risks, the pair is `EpsilonEquivalent`. The sum of those risks is charged to the decision ledger.

### 9.3 Conservative pivotal-mass equivalence

Because

\[
 |g|\le q,
\]

it is sufficient to establish

\[
 q<\varepsilon.
\]

The pivotal indicator

\[
 P_i=|Y_i|\in\{0,1\}
\]

is Bernoulli. Apply the lower-threshold evidence process `E^<` with threshold `c=ε`.

This route is conservative: it cannot recognize a large-`q`, balanced-tilt tie. But it is simple, exact-rational, and directly useful for saturation cases where the candidates almost never differ.

### 9.4 Exact zero

Sampling can support `q≤ε` for positive `ε`. It cannot establish `q=0` after finitely many ordinary random observations.

Exact `q=0` requires:

- full-fiber enumeration;
- a proved pivotal cover with exact empty count;
- another exact structural argument.

---

## 10. A general exact-rational bounded-mean engine

The Boolean pivotal engine is the preferred tool for frozen `pmake` policies. Walt also needs a safe replacement for the current habit of counting which block had the larger rational value.

Let

\[
 X_i\in[L,U]
\]

be independent or conditionally mean-controlled observations.

### CE-T4 — positive bounded-mean betting process

To test

\[
 H_0:\mathbb E[X_i\mid\mathcal F_{i-1}]\le c,
\]

choose any rational

\[
 0\le\lambda\le\frac{1}{c-L}
\]

and define

\[
 \boxed{
 M_n^+(\lambda;c)
 =\prod_{i=1}^n
  \left(1+\lambda(X_i-c)\right).
 }
\]

The factors are nonnegative, and under the null

\[
 \mathbb E[1+\lambda(X_i-c)\mid\mathcal F_{i-1}]
 \le1.
\]

Therefore `M_n^+` is a nonnegative supermartingale.

For a finite rational mixture

\[
 \{(w_j,\lambda_j)\}_{j=1}^J,
\qquad
 w_j\ge0,
\qquad
 \sum_jw_j=1,
\]

define

\[
 \boxed{
 M_n^+(c)
 =\sum_{j=1}^Jw_jM_n^+(\lambda_j;c).
 }
\]

It is an exact-rational evidence process.

### CE-T5 — negative bounded-mean betting process

To test

\[
 H_0:\mathbb E[X_i\mid\mathcal F_{i-1}]\ge c,
\]

choose

\[
 0\le\lambda\le\frac{1}{U-c}
\]

and use

\[
 \boxed{
 M_n^-(\lambda;c)
 =\prod_{i=1}^n
  \left(1-\lambda(X_i-c)\right).
 }
\]

Finite rational mixtures are valid exactly as above.

### 10.1 Correct use on rational block differences

Suppose one fixed evaluation algorithm produces iid paired block values

\[
 V_{a,j},V_{b,j}\in[0,1]
\]

and define

\[
 X_j=V_{a,j}-V_{b,j}\in[-1,1].
\]

Then CE-T4 with `c=0` tests whether the **mean block-value difference** is positive.

This is mathematically different from counting only

\[
 \mathbf 1\{X_j>0\}
\]

versus

\[
 \mathbf 1\{X_j<0\}.
\]

Sign frequency does not determine mean order.

For example,

\[
 X=
 \begin{cases}
  +1/8,&\text{with probability }3/4,\\
  -1/2,&\text{with probability }1/4.
 \end{cases}
\]

The positive sign wins 75% of blocks, but

\[
 \mathbb E[X]
 =\frac34\cdot\frac18
  -\frac14\cdot\frac12
 =-\frac1{32}.
\]

A sign test will eventually become confident in the wrong mean ordering.

The bounded-mean process uses magnitude and does not have that defect.

### 10.2 What this does not repair

If each block **re-optimizes a different continuation policy**, then the target itself is not a frozen-policy value. A valid mean test can settle the mean of that named block algorithm, but it cannot be relabeled as a signed-pivotal comparison of two frozen policies or as an exact root-action value.

Target typing and statistical validity are separate requirements. Both must hold.

---

## 11. The exact finite-fiber escalation

The unified kernel already provides the essential exact objects:

- exact fiber size;
- exact uniform one-world sampling;
- lazy full-fiber enumeration.

The adaptive player should make those objects authoritative.

### 11.1 One canonical outer fiber

For every root decision, build one canonical `kernel::Kernel` and one reusable `FiberDp`.

Use that same object for:

- `N = |Φ(C)|`;
- exactly uniform sampled worlds;
- world identity;
- eventual exhaustive enumeration;
- exact structural strata and counts.

The solver’s legacy shuffle-and-reject sampler may remain temporarily for regression comparison. It must not be a second semantic authority in the new correctness path.

### 11.2 Sampling law

The first implementation should sample **with replacement** from the exact kernel sampler.

Reasons:

- the evidence proofs above apply directly to iid worlds;
- duplicate worlds are mathematically valid and preserve their multiplicity in the evidence stream;
- small fibers should normally trigger exact enumeration before duplicate saturation becomes expensive.

A future without-replacement evidence process is possible, but it is not required for this intake.

### 11.3 Exact switch rule

Let

\[
 N=|\Phi(C)|
\]

be exact.

Measure or estimate:

- `c_sample`: cost of drawing and evaluating one additional sampled world across the live candidates;
- `c_enum`: cost of evaluating one not-yet-cached enumerated world;
- `n_rem`: forecast additional sampled worlds to settlement;
- `N_rem`: number of unique fiber worlds not yet cached.

Define

\[
 C_{\mathrm{sample}}
 =\widehat n_{\mathrm{rem}}\,\widehat c_{\mathrm{sample}},
\]

\[
 C_{\mathrm{exact}}
 =N_{\mathrm{rem}}\,\widehat c_{\mathrm{enum}}.
\]

Switch to exact enumeration when

\[
 \boxed{
 C_{\mathrm{exact}}
 \le
 C_{\mathrm{sample}}.
 }
\]

This decision affects performance only. A wrong cost forecast cannot make the mathematical result wrong.

### 11.4 Reuse sampled work

Assign every physical world a canonical identity. Cache the terminal outcome of every frozen policy on each distinct sampled world.

If exact enumeration begins later, reuse those cached outcomes and evaluate only the remaining unique worlds. The final exact sum counts every physical world once, regardless of how often it appeared in the sampled stream.

### 11.5 Two exact endpoints

#### Exact frozen-set endpoint

Enumerate every world and replay every named frozen policy. Count exact outcomes.

For a uniform fiber,

\[
 V_\rho
 =\frac{\#\{\omega:u_\rho(\omega)=1\}}{N}.
\]

This yields `ExactFrozenSet`.

#### Exact root endpoint

Feed the complete fiber to the information-consistent solver rather than a sampled subset. If the declared field model is deterministic and the solver obligations hold, the solver’s exact-on-sample optimization becomes exact-on-fiber optimization.

This yields `ExactFiberRoot`: exact best response over the complete outer belief against the declared field model.

The field model may itself be a deterministic sampled-mind policy identified by a freeze tuple. The result is exact **relative to that model**. It is not thereby an equilibrium or an exact statement about another field model.

### 11.6 Exactness remains three-dimensional

The three locks still govern:

1. **measure lock:** complete exact outer fiber or exact region masses;
2. **response lock:** exact outcomes under the declared frozen field dynamics;
3. **optimization lock:** no omitted information-consistent focal continuation can improve the root value.

`ExactFrozenSet` closes the first two for a fixed candidate set.

`ExactFiberRoot` closes all three relative to the declared field model, assuming the solver correctness obligations.

---

## 12. Frozen-policy materialization

Any fixed-policy evidence theorem is useless if “policy `a`” changes while evidence is being accumulated.

The unification makes the required repair local and tractable.

### 12.1 Freeze tuple

A frozen policy identity must include at least:

- solver semantic version or source identity;
- declaration and bid;
- field model and level;
- inner sample schedule;
- discovery world IDs or discovery stream identity;
- discovery seed schedule;
- tie-handling rule;
- any practical-equivalence parameter affecting actions;
- any policy-library version;
- any exact/heuristic mode flag that changes decisions.

Hash the complete tuple into `PolicyId`.

### 12.2 Lazy materialization

A policy need not be serialized eagerly as a full DAG.

For each `PolicyId`, maintain a memo table

\[
 \text{focal information state}
 \longmapsto
 \text{chosen legal action}.
\]

When replay first reaches an unseen information state, compute the action under the frozen discovery configuration and cache it. Every later replay uses the cached action.

This is lazy policy extraction.

### 12.3 Information consistency

The focal action key may contain only the focal seat’s information:

- its own remaining hand;
- the public record or a proved sufficient reduction;
- the freeze tuple.

It may not contain the evaluation world’s hidden hands.

The evaluation world determines which public observations occur. It must not directly select the focal action.

### 12.4 Discovery and evidence streams are disjoint

Policy discovery may use one world stream. Evidence must use another stream that did not construct or alter the policy.

If lazy action materialization itself calls a sampled solver, that solver’s worlds come from the frozen policy’s **discovery schedule**, derived only from the information state and freeze tuple—not from the evaluation stream’s hidden world.

### 12.5 Policy mutation invalidates evidence

Any cache miss may extend the representation of the same frozen policy. It may not change previously defined actions.

Changing configuration, replacing an action, increasing its discovery sample budget, or re-solving a state under a new seed creates a new `PolicyId` and a new evidence epoch.

---

## 13. Correct interpretation of the current racers

The existing racing work remains useful exploratory engineering. Its mathematical status must be narrowed before the correctness path is built.

### 13.1 Replay race

The replay race that compares terminal Boolean outcomes of named frozen policies on common worlds is the correct signed-pivotal object.

Its current performance problem—replay approximately re-solving the policy—is an implementation problem. Lazy frozen-policy materialization is the direct cure.

### 13.2 Block race

The current block race is not a signed-pivotal implementation because:

1. each block may re-optimize continuation behavior rather than replay one fixed policy;
2. it counts which rational block value is larger, discarding difference magnitude;
3. repeated fixed-threshold looks do not by themselves supply a run-level anytime error guarantee.

It may remain as `HeuristicFallback` or as an experiment.

### 13.3 A lawful block successor

A mathematically typed block successor would require all of the following:

- one fixed algorithmic target per candidate;
- iid or conditionally valid common blocks;
- the full rational block difference, not only its sign;
- CE-T4/CE-T5 or another valid bounded-mean evidence process;
- a complete risk ledger across repeated looks and candidates;
- a result label naming the block-algorithm mean as the target.

Even then it would not automatically become an exact frozen-policy or root-action result.

### 13.4 Race-refine remains outside the proof path

The legacy `race-refined` mode can stay opt-in while the new path is built. It must not feed evidence, exactness, or confidence labels into the new result types.

---

## 14. The level-2 probe needs one mathematical correction

The current level-2 probe correctly identifies field-swap comparison as the next research target. Its proposed “pivotal mass wakes up” detector should be split into three distinct objects.

For the same frozen action pair under field models `σ_0` and `σ_1`, define

\[
 (q_0,\tau_0,g_0)
\]

and

\[
 (q_1,\tau_1,g_1).
\]

### 14.1 Response wake-up

A field upgrade changes whether the policies disagree:

\[
 q_1>q_0
\]

or, operationally,

\[
 q_1-q_0>\varepsilon_q.
\]

This is a response-geometry result. It does not by itself prove a value difference.

### 14.2 Value wake-up

A field upgrade changes the signed gap:

\[
 g_1-g_0\ne0
\]

or by a declared amount.

This is the object relevant to value.

### 14.3 Decision wake-up

The field upgrade changes the selected action, or changes an unresolved/equivalent comparison into a settled one.

This is the object relevant to play.

### 14.4 Why `q` alone is insufficient

It is possible that

\[
 q_1>0
\]

but

\[
 \tau_1=0,
\qquad
 g_1=0.
\]

The upgraded field may make the candidates disagree on many worlds while those disagreements balance exactly.

Thus:

> **`q`-wake is a detector of newly active response structure, not by itself a proof that level 1 loses value.**

### 14.5 Sampling cost under each field

The correct cost comparison is

\[
 \mathcal I_f
 =q_fD_{1/2}(\tau_f),
\qquad
 f\in\{0,1\}.
\]

Level 2 is easier to sample at the root when

\[
 \mathcal I_1>\mathcal I_0,
\]

not merely when `q_1>q_0` or `H_1<H_0` in a noisy plug-in report.

### 14.6 Paired field-correction evidence

On the same world, let

\[
 Y_i^{(0)},Y_i^{(1)}\in\{-1,0,+1\}
\]

and define

\[
 Z_i=Y_i^{(1)}-Y_i^{(0)}\in\{-2,-1,0,1,2\}.
\]

Then

\[
 \mathbb E[Z]=g_1-g_0.
\]

Apply the bounded-mean engine to

\[
 X_i=Z_i/2\in[-1,1]
\]

to obtain exact-rational anytime evidence for the direction of the field correction.

### 14.7 Exact zero versus practical zero

A statement such as `q_0=0` requires exact enumeration or a structural proof. Sampling may establish only `q_0≤ε_q` at declared risk.

The level-2 probe should preserve that distinction in its output contract.

---

## 15. Auction thresholds use the same engine

Let `B_i` indicate whether one fixed declaration/bid policy makes the contract in world `i`.

For a declared auction policy threshold

\[
 \vartheta\in(0,1),
\]

use CE-T1 to establish

\[
 p>\vartheta
\]

and CE-T2 to establish

\[
 p<\vartheta.
\]

The auction threshold `ϑ` and the sampling error budget `δ` are different parameters:

- `ϑ` says how much modeled make probability the bidding policy demands;
- `δ` says how much sampling error the evidence procedure allows.

The empirical `11/16` policy threshold may remain a configurable auction choice. Adaptive evidence determines whether a fixed contract cell lies above or below it; it does not validate the policy threshold itself.

Declaration selection and bid walking are multiple adaptive comparisons. They need their own candidate identities and risk allocation. A first implementation should land play selection before changing auction semantics.

---

## 16. The unified architecture implied by the mathematics

Do not create another Walt crate.

The unified crate is now the right home.

### 16.1 `kernel` — one world authority

Owns:

- information-state kernel construction;
- exact `FiberDp`;
- exact count;
- exact uniform world stream;
- canonical world identity;
- lazy full enumeration;
- later, exact counted strata and conditional generation.

### 16.2 `solver::policy` — frozen policy authority

Owns:

- `FreezeTuple`;
- `PolicyId`;
- information-consistent action key;
- lazy policy materialization;
- immutable action cache;
- frozen-policy replay.

### 16.3 `solver::evidence` — arithmetic authority

Owns:

- exact Bernoulli-threshold evidence;
- exact pivotal closed form;
- exact bounded-mean mixture betting;
- evidence thresholds;
- risk ledgers;
- exact threshold comparisons;
- evidence-debt and best-case-pivot calculations.

This module must not know Texas 42 rules.

### 16.4 `solver::adaptive` — decision controller

Owns:

- candidate set and epoch identity;
- common world index;
- pair outcome updates;
- safe elimination;
- practical-equivalence mode;
- sample-versus-exact routing;
- resource caps and `Unresolved`;
- result typing.

### 16.5 `solver::exact` — full-fiber endpoint

Owns the adapters that:

- replay a frozen set on every kernel world;
- run the existing solver on the complete fiber;
- reuse cached sampled outcomes;
- return `ExactFrozenSet` or `ExactFiberRoot`.

This may be a submodule rather than a separate file if the implementer prefers. The semantic boundary matters more than the physical layout.

### 16.6 Thin consumers

`walt-wasm`, table binaries, the arena bridge, the tilt audit, and the level-2 probe should call the same decision API. They may choose a resource policy, but they may not reimplement evidence or sampling semantics.

---

## 17. Determinism, batching, and the world stream

A correctness result must not depend on how work was scheduled.

### 17.1 Counter-based world identity

Prefer a world stream where world `i` is selected from a seed derived from

\[
 (\text{root identity},\text{evaluation epoch},i).
\]

Then batch size, thread count, candidate elimination, and resume boundaries do not change which world occupies index `i`.

A mutable sequential RNG may still be lawful, but it makes execution-order invariance harder to audit.

### 17.2 Common random worlds

Every live candidate is evaluated on the same world index before pair evidence is updated.

A candidate may stop consuming future worlds only after lawful elimination. Previously accumulated pair counts remain aligned by world ID.

### 17.3 Batch size is not a statistical parameter

The engine may process 1, 8, 64, or 4,096 worlds per compute batch. Evidence is conceptually updated in stream order. A batch may overshoot the first crossing for throughput reasons, but the reported settlement index is the first crossing inside the batch.

Changing batch size must not change the settled winner on the same world stream.

### 17.4 Randomness semantics

A `δ` statement is relative to the declared exactly uniform sampling law. A pseudorandom implementation is a reproducible realization of that law under the project’s RNG assumption; a fixed transcript is not itself a deterministic proof of correctness.

Therefore every `DeltaSettled` result records:

- sampler identity;
- seed or seed provenance;
- world stream identity;
- sampling-with-replacement declaration;
- exact fiber identity.

Exact enumeration is the route that removes this probabilistic dependence entirely.

---

## 18. Outer sampling first; recursive inner sampling second

The current level-1 player still contains inner modeled minds with declared sample sizes.

Replacing only `n_outer` removes the outer magic number. It does not magically make every inner policy exact.

The work should proceed in two phases.

### Phase 1 — outer adaptive settlement

Hold the current frozen field model fixed. Apply calculated evidence only to the outer candidate comparison.

This is enough to:

- remove the 40-versus-160 root flip mode;
- make the level-2 field-swap probe interpretable;
- measure true outer cost;
- validate exact-fiber escalation;
- preserve current play strength as a fallback.

The result remains model-relative to inner policies identified by their freeze tuples.

### Phase 2 — recursive inner decisions

When a modeled mind at a new `PiKey` must choose among actions, give that inner decision its own adaptive controller or exact-fiber endpoint.

Its error budget must come from a declared nested ledger. One simple open-ended allocation is again

\[
 \delta_j=\frac{\delta_{\mathrm{inner,total}}}{j(j+1)}
\]

for the `j`-th newly materialized inner decision in the run.

Because `PiKey` decisions are cached, each inner decision pays its evidence cost once per freeze tuple and information state.

Until this phase lands, inner sample counts remain declared approximations. They must stay visible in every result identity.

---

## 19. Validation program

The mathematics is self-contained, but the implementation still needs adversarial validation.

### V1 — exact formula anchors

Check the pivotal closed form against direct rational integration or the finite-sum CE-T1 formula for a grid such as

\[
 0\le a,b\le100.
\]

Include the anchors in §4.1.

### V2 — one-step supermartingale identities

For rational grids of `p≤c≤r`, verify exactly that

\[
 \mathbb E[L_r(B)]
 =1+\frac{(p-c)(r-c)}{c(1-c)}
 \le1.
\]

Do the lower direction by symmetry.

### V3 — bounded-mean nonnegativity and expectation

For every configured `(w_j,λ_j)`, assert exact weight normalization, valid λ range, nonnegative factors over the full declared observation range, and the one-step expectation inequality.

### V4 — small-fiber exact oracle

Select roots whose complete fibers are cheap. For each:

1. compute `ExactFiberRoot`;
2. compute `ExactFrozenSet` for named policies;
3. run many adaptive streams only as regression evidence;
4. check that any `DeltaSettled` result disagrees with the exact target only at a frequency compatible with the declared risk.

The theorem, not the empirical frequency, carries correctness. The experiment catches implementation defects.

### V5 — the historical 40/160 flip

Reconstruct the live count-timing position where the 40-world and 160-world choices differed.

The new controller must do one of the following:

- remain unresolved at 40 and settle later;
- settle at or before 40 and never reverse on that stream;
- switch to exact enumeration and return the exact target;
- return an explicitly labeled heuristic fallback after `Unresolved`.

It must never call both 40 and 160 “settled” answers.

### V6 — fixed-pair cost calibration

For each fixed policy pair with a large reference panel or exact fiber, compare:

- observed settlement worlds;
- `q̂`, `τ̂`, `Ĥ`;
- `q̂D_{1/2}(τ̂)`;
- the leading-order forecast;
- the exact predictive DP forecast.

This is done per fixed pair, never by pooling different discovery policies into one pseudo-pair.

### V7 — block-sign counterexample

Add a unit or probe fixture reproducing the §10.1 distribution. Any component that calls sign-majority a proof of positive mean must fail the gate.

### V8 — batching and thread invariance

For one frozen root, candidate set, world stream, and risk ledger, run multiple batch sizes and thread counts. Require identical:

- world IDs;
- pair counts at each stream index;
- first evidence crossing;
- elimination graph;
- final result kind and move.

Timing and duplicate work may differ.

### V9 — exact-switch parity

Force the controller to switch to exactness at several different stream indices. Every run must return the same exact result, and the exact total must equal a cold full enumeration.

### V10 — level-2 wake-up decomposition

For each anchor pair, report separately:

- response wake-up `q`;
- value wake-up `g` or `g_1-g_0`;
- decision wake-up;
- information rate under each field;
- exact versus probabilistic status.

---

## 20. Acceptance contract for the first implementation

The first applied version is complete only when all of the following hold.

1. The correctness path contains no fixed sample count that determines settlement.
2. `δ` is explicit, rational, scoped, serialized, and propagated through a complete ledger.
3. Optional `ε` is explicit, rational, and never confused with exact equality.
4. Candidate policies are frozen and content-addressed before evaluation evidence begins.
5. Discovery and evidence use disjoint world streams.
6. Every live candidate sees the same ordered evidence worlds.
7. Pair evidence uses exact arithmetic and an anytime-valid threshold.
8. `level1_raced` and `race-refined` do not supply proof-path evidence.
9. A resource cap returns `Unresolved` or invokes a visibly labeled `HeuristicFallback`.
10. The canonical kernel supplies count, sample, world identity, and enumeration.
11. The controller calculates both projected sampled cost and projected exact cost.
12. Exact escalation reuses cached unique sampled-world outcomes.
13. Batch size and thread count do not alter the result on a fixed stream.
14. The historical 40/160 flip is handled according to V5.
15. Full-fiber small-grade tests reproduce exact cold runs.
16. The old player remains available as a fallback until arena and conformance gates justify a default change.

---

## 21. Proposed new obligations

These numbers continue the current O12–O19 line. They are proposals for intake and adjudication, not self-issued rulings.

### O20 — exact evidence theorem

The implementation of CE-T1 through CE-T5 matches the stated exact formulas and anytime-valid hypotheses.

**Route:** paper proof in this parent; exact-rational intake verification; implementation tests.

### O21 — risk-ledger completeness

Every probabilistic settlement can reconstruct the full allocation from run budget to decision, epoch, pair direction, and optional equivalence test. The sum of all allocated risks is bounded by the declared scope budget.

**Route:** invariant audit plus exact-rational ledger tests.

### O22 — frozen-policy identity

Every evidence observation names immutable `PolicyId`s. Evaluation cannot mutate, re-discover, or world-condition the focal actions while retaining old evidence.

**Route:** data-flow audit, cache immutability tests, adversarial hidden-world tests.

### O23 — canonical fiber and sampler domain

The evidence stream, exact count, and exhaustive endpoint target the same information-state fiber and belief. The unified kernel is the authority.

**Route:** construction proof, count/sample/enumeration parity tests, domain assertions.

### O24 — exact-escalation correctness

Switching from sampling to enumeration at any stream index yields the same exact endpoint as cold full enumeration. Sample multiplicities are not double-counted in the exact sum.

**Route:** V9 and a short bookkeeping proof.

### O25 — result typing and fallback separation

`ExactFiberRoot`, `ExactFrozenSet`, `DeltaSettled`, `EpsilonEquivalent`, `Unresolved`, and `HeuristicFallback` are mechanically distinct. No UI or bridge erases the type before persistence.

**Route:** API and serialization tests.

### O26 — execution-order invariance of evidence

For a fixed world stream and frozen candidate set, batching, parallel scheduling, and pause/resume do not change evidence or settlement.

**Route:** indexed world stream plus V8.

### O27 — sampling randomness semantics

The probability space supporting a `δ` claim is explicit. Sampler uniformity, replacement semantics, seed provenance, and PRNG assumptions are declared rather than hidden under determinism.

**Route:** design note plus O3 integration.

### O28 — recursive inner-risk accounting

Any later adaptive inner-mind decision obtains risk from a complete nested ledger. Outer `δ` claims do not silently ignore inner stochastic decision error.

**Route:** Phase-2 design and cache audit.

---

## 22. Recommended implementation sequence

### Step 1 — intake before code

File this parent verbatim with a checksum. Create a maintained intake companion that:

- verifies every exact identity;
- checks the pivotal closed form against CE-T1;
- checks the supermartingale algebra;
- adjudicates vocabulary and obligation numbering;
- records any current-code boundary found during implementation review.

Do not edit the parent to absorb repairs.

### Step 2 — make the kernel authoritative

Add an adapter from the live solver’s root information state to `kernel::Kernel` and `FiberDp`.

Prove by regression that the old and new samplers target the same fiber where both apply. Then route the new controller exclusively through the canonical exact sampler.

### Step 3 — implement `solver::evidence`

Land exact arithmetic and tests before integrating with play:

- general rational threshold evidence;
- pivotal closed form;
- directed-edge thresholds;
- run/decision risk ledger;
- bounded-mean finite mixtures;
- evidence debt;
- best-case pivotal counts.

### Step 4 — implement lazy frozen policies

Create `FreezeTuple`, `PolicyId`, and an immutable information-state action cache. Replay fixed policies on a supplied world without reading hidden data.

### Step 5 — build a fixed-candidate adaptive evaluator

For one root and one frozen candidate set:

- draw indexed common worlds;
- produce terminal Boolean outcome bits;
- update every live pair;
- eliminate safely;
- return `DeltaSettled`, `EpsilonEquivalent`, or `Unresolved`.

No exact switch yet.

### Step 6 — add exact endpoints

Use `Kernel::worlds()` for:

- exact frozen-set replay;
- complete-fiber solver evaluation.

Add the sampled-work cache and exact-switch parity gate.

### Step 7 — shadow the current player

Run the new controller beside the existing player. Persist:

- result kind;
- chosen move;
- settlement index;
- refinement vector;
- exact-switch decision;
- old-player choice;
- exact reference where available.

Do not change live defaults during this measurement phase.

### Step 8 — repair the historical flip and E0

Run V5 and the corrected per-fixed-pair E0 calibration. Only after these pass should the controller become an opt-in play mode.

### Step 9 — run the level-2 probe

With outer sampling noise now controlled, execute the field-swap program using the three wake-up notions and paired correction evidence in §14.

### Step 10 — recurse inward

Only after outer behavior is understood should adaptive evidence replace the inner `n_k` sample schedules.

---

## 23. What this document does not claim

1. It does not claim a `δ`-settled result is exact.
2. It does not claim a best member of a frozen candidate set is the globally optimal root action.
3. It does not claim a finite directional stopping time exists at a true tie.
4. It does not claim a cost forecast is a correctness guarantee.
5. It does not claim pivotal mass alone determines difficulty.
6. It does not claim a level-2 `q` wake-up is necessarily a value wake-up.
7. It does not claim the current inner sampled minds are exact.
8. It does not claim a fixed pseudorandom transcript is a deterministic theorem about the fiber.
9. It does not claim exact structural counts alone close the response or optimization locks.
10. It does not claim the block racer is useless; it says only that its current target and test do not support a signed-pivotal correctness label.
11. It does not require the sampled player to be deleted. It requires heuristic fallback and mathematical settlement to be typed separately.
12. It does not require exact trick-1 evaluation immediately. It builds a monotone road on which every late-grade exact result and every probabilistically settled early result has an honest meaning.

---

## 24. Final mathematical thesis

The correct replacement for a magic sample count is not another formula that emits one supposedly universal `n` before seeing the decision.

The correct replacement is a decision process whose required work is calculated from:

- the declared error scope;
- the number of fixed contenders;
- the exact evidence already observed;
- the pair’s pivotal frequency;
- the directional information carried by each pivot;
- the exact size of the remaining fiber;
- the measured cost of sampling versus enumeration.

For a frozen pair, the operative information rate is

\[
 \boxed{
 \mathcal I
 =qD_{\mathrm{Ber}}
  \left(
    \frac{1+\tau}{2}
    \middle\Vert
    \frac12
  \right).
 }
\]

The correctness rule is the exact evidence crossing

\[
 \boxed{
 E\ge\frac1\alpha.
 }
\]

The exactness escape hatch is the full fiber

\[
 \boxed{
 N=|\Phi(C)|.
 }
\]

And the controller’s central economic decision is

\[
 \boxed{
 \text{continue sampling only while truthful sampling is cheaper than knowing.}
 }
\]

That gives unified Walt what the project has been asking for:

- no arbitrary `40`, `160`, `200`, or `2,000` in the correctness rule;
- easier decisions that stop quickly;
- hard decisions that expose why they are hard;
- true near-ties that remain honest;
- exact late-game ground truth whenever the fiber permits it;
- a calculated path from probabilistic evidence to exhaustive knowledge;
- a stable substrate for many belief and dynamics experiments;
- and a mathematically clean gate into level 2.

Sampling found something real. The next step is not to retreat from that success. It is to put it back under mathematical control and make every extra world answer a calculated question.
