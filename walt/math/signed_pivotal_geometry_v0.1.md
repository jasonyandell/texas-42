# HANDOFF — Signed Pivotal Geometry for Walt

## Cost prediction, exactness escalation, and amortized belief/dynamics experiments

**Status:** EXPLORATORY mathematical proposal. Nothing is promoted by this document's existence.

**Date:** 2026-08-18

**Provenance:** house-mathematician pass over `HANDOFF-plan-geometry-and-names.md`, prompted by the empirical fact that small scenario samples are already producing strong and enjoyable Texas 42 play. This document is intended to stand alone. It sharpens and partly replaces the imported handoff's treatment of pivotal sets, flip thresholds, count-powered sampling, the plan census, and the first experiment. It does not adjudicate or promote that handoff's external literature-name mappings.

**Scope:** fixed field model, fixed bid, Boolean `pmake` utility, and information-consistent focal plans. The root auction, equilibrium claims, and cross-model optimality remain out of scope. All sampled quantities are estimates unless an exact-count or proof condition is stated explicitly.

---

## 0. Executive conclusion

The imported handoff found the right coordinate system, but its central object should be made **signed**, its statistical protocol should separate **plan discovery** from **plan evaluation**, and its exactness story should distinguish three independent locks:

1. **measure exactness** — how much belief mass lies in each relevant world region;
2. **response exactness** — what the frozen plans do there, including the random tape and modeled field dynamics;
3. **optimization exactness** — whether the frozen plans are truly the best information-consistent continuations available under each root action.

For two frozen plans, all pairwise decision mathematics collapses to two numbers:

\[
q = \Pr(\text{the plans disagree}),
\qquad
\tau = \mathbb E[\operatorname{sign}(\text{winner})\mid\text{they disagree}].
\]

Here \(q\in[0,1]\) is the **pivotal mass** and \(\tau\in[-1,1]\) is the **signed pivotal tilt**. Their product is the exact value gap:

\[
\boxed{g=q\tau.}
\]

The per-scenario variance of the paired comparison is

\[
\boxed{\operatorname{Var}(Y)=q-g^2=q-q^2\tau^2.}
\]

A natural fixed-pair difficulty scale is therefore

\[
\boxed{
H=\frac{q-g^2}{g^2}
  =\frac{1}{q\tau^2}-1,
}
\]

with the usual caveat that this is a planning scale, not by itself a finite-sample confidence guarantee.

This gives a precise candidate explanation for why small samples work:

> **Most scenarios either cancel, or the scenarios that do not cancel have a strong directional tilt.**

Small pivotal mass alone is not sufficient. A rare but almost perfectly balanced pivotal set is genuinely hard. The pair \((q,\tau)\), not \(q\) alone, is the mathematical object to measure.

The immediate necessary experiment is therefore not “build a stratified sampler.” It is:

> **Discover plans on one sample, freeze them, replay them together on a large independent common scenario panel, and measure the signed pivotal profile, plan instability, and world-versus-tape decomposition.**

That experiment can tell the project which of four roads is real:

- **sampling road:** fixed plans are stable and \(H\) predicts observed cost;
- **counted-boundary road:** small structural envelopes capture nearly all pivotal mass;
- **plan-library road:** active plans or signed boundaries recur across positions and beliefs;
- **search-instability road:** the dominant error is not evaluation sampling at all, but unstable continuation-plan selection.

The hoped-for exact road is also clearer. Exact structural counts can remove the measure lock. Structural theorems or exact conditional dynamics can remove the response lock. Search bounds or exhaustive active-plan discovery must remove the optimization lock. Pivotal geometry does not magically open all three locks, but it can confine the expensive work to the regions and contenders that can still change the move.

---

## 1. Mathematical setting

Fix:

- an outer information state \(B=(K,e,\beta)\);
- a fixed field model \(\sigma\), including partner behavior;
- a fixed bid and Boolean `pmake` utility;
- an information-consistent focal plan \(\rho\), which branches only on observations available to the focal seat.

A determinized scenario is

\[
\xi=(\omega,r),
\]

where \(\omega\) is the physical deal/world and \(r\) is the persistent random tape used by the modeled field. Under frozen \((\rho,\sigma)\), every scenario produces one terminal Boolean outcome

\[
u_\rho(\xi)\in\{0,1\}.
\]

The exact value of the plan is

\[
V_\rho(\beta)=\mathbb E_{\xi\sim\beta}[u_\rho(\xi)].
\]

On a finite common scenario panel, the plan is equivalently represented by its Boolean outcome vector or **make-set**. That representation is useful because all pairwise comparisons become set operations and paired statistics.

### 1.1 Frozen plans are the unit of linear geometry

For a root action \(a\), let \(\Pi_a\) be the information-consistent continuation plans whose first action is \(a\). The optimized root-action value is

\[
Q_a(\beta)=\max_{\rho\in\Pi_a}V_\rho(\beta).
\]

A single frozen plan has linear value in \(\beta\). A root action is an upper envelope of many such linear values. This distinction is load-bearing:

- between two **frozen plans**, the tie set is one hyperplane;
- between two **optimized root actions**, the active continuation plans may switch, so the tie boundary is generally piecewise linear and may have multiple crossings along a one-dimensional belief sweep.

Accordingly, every pivotal statement below is exact for a named frozen plan pair. It is only locally descriptive of optimized root actions while those two plans remain active.

---

## 2. The signed pivotal theorem

Fix two frozen plans \(\rho_a\) and \(\rho_b\). Define the paired outcome

\[
Y_{a,b}(\xi)=u_{\rho_a}(\xi)-u_{\rho_b}(\xi)\in\{-1,0,+1\}.
\]

Define the signed pivotal regions

\[
\Delta^+_{a,b}
 =\{\xi:u_{\rho_a}(\xi)=1,\ u_{\rho_b}(\xi)=0\},
\]

\[
\Delta^-_{a,b}
 =\{\xi:u_{\rho_a}(\xi)=0,\ u_{\rho_b}(\xi)=1\},
\]

and

\[
\Delta_{a,b}=\Delta^+_{a,b}\ \dot\cup\ \Delta^-_{a,b}.
\]

Everything outside \(\Delta_{a,b}\) contributes zero to the comparison, whether both plans make or both fail.

Let

\[
p_+=\beta(\Delta^+_{a,b}),
\qquad
p_-=\beta(\Delta^-_{a,b}).
\]

Define

\[
q=p_++p_-
\]

and, when \(q>0\),

\[
\tau=\frac{p_+-p_-}{p_++p_-}\in[-1,1].
\]

Then:

\[
\boxed{g(a,b)=V_{\rho_a}(\beta)-V_{\rho_b}(\beta)=p_+-p_-=q\tau.}
\]

This factorization is the central result.

- \(q\) says **how often the choice matters**.
- \(\tau\) says **which plan wins when it matters, and how consistently**.
- \(g\) says **how much the choice is worth in expectation**.

An equivalent parameterization uses

\[
\theta=\Pr(Y=+1\mid |Y|=1)=\frac{1+\tau}{2},
\]

so

\[
g=q(2\theta-1).
\]

### 2.1 Exact sampling variance

Because \(Y^2=1\) exactly on the pivotal set,

\[
\mathbb E[Y]=g,
\qquad
\mathbb E[Y^2]=q.
\]

Therefore

\[
\boxed{\operatorname{Var}(Y)=q-g^2=q-q^2\tau^2.}
\]

For \(n\) independent paired scenarios,

\[
\boxed{\operatorname{Var}(\widehat g)=\frac{q-g^2}{n}.}
\]

This is strictly sharper than treating the two make rates as unrelated estimates. The common scenarios are not a nuisance correlation; they are the mechanism that exposes cancellation.

### 2.2 The multinomial decomposition

For a fixed pair and a fixed evaluation panel, let

\[
N_+=\#\{Y=+1\},\quad
N_-=\#\{Y=-1\},\quad
N_0=\#\{Y=0\}.
\]

Then

\[
(N_+,N_-,N_0)
\sim
\operatorname{Multinomial}
\left(n;\ p_+,p_-,1-q\right).
\]

Let

\[
K=N_++N_-.
\]

Then

\[
K\sim\operatorname{Binomial}(n,q),
\]

and, conditional on \(K\),

\[
N_+\mid K\sim\operatorname{Binomial}(K,\theta).
\]

This decomposition cleanly separates two costs:

1. **finding informative scenarios**, governed by \(q\);
2. **deciding the sign once informative scenarios are found**, governed by \(|\tau|=|2\theta-1|\).

For action selection, the sign of \(g\) is the sign of \(\tau\) whenever \(q>0\). Exact knowledge of \(q\) is needed to know the magnitude of the advantage, but not its sign.

### 2.3 A fixed-pair cost scale

A variance-to-squared-margin scale is

\[
\boxed{
H(a,b)
  =\frac{\operatorname{Var}(Y)}{g^2}
  =\frac{q-g^2}{g^2}
  =\frac{1}{q\tau^2}-1.
}
\]

For ordinary asymptotic confidence calculations, required sample count is proportional to \(H\), multiplied by the confidence constant. A finite-confidence sign test based on pivotal observations has the same qualitative scaling:

\[
K_{\text{pivotal}}
 =O\!\left(\frac{\log(1/\delta)}{\tau^2}\right),
\qquad
\mathbb E[n_{\text{raw}}]
 =O\!\left(\frac{\log(1/\delta)}{q\tau^2}\right).
\]

The constants must come from the chosen fixed-sample interval or anytime-valid confidence sequence. The formulas above are cost scales, not permission to stop under an ordinary fixed-sample interval after repeatedly peeking. A simple valid implementation is to inspect only at predeclared pivotal-count checkpoints \(K_j\), assign error budgets \(\delta_j\) with \(\sum_j\delta_j\le\delta\), and stop when the corresponding binomial interval for \(\theta\) excludes \(1/2\). For multiple fixed contenders, divide the total error budget across the pairwise comparisons or use a simultaneous procedure.

### 2.4 Four qualitatively different ties

The pivotal coordinates diagnose cases that an absolute make percentage hides.

#### Case A — no pivotal mass

\[
q=0.
\]

The two frozen plans agree almost surely under the modeled belief. More samples from the same distribution cannot reveal a value difference. The plans may still differ syntactically or on zero-belief/support-excluded scenarios.

#### Case B — sparse but decisive

\[
q\ll1,
\qquad
|\tau|\approx1.
\]

Most scenarios cancel, but the rare disagreements point almost entirely one way. Uniform sampling wastes work finding disagreements. This is the ideal regime for structural envelopes or direct pivotal generation.

#### Case C — genuinely knife-edge

\[
q>0,
\qquad
|\tau|\approx0.
\]

The plans disagree, but the disagreement is almost balanced. This is not primarily a sampling-engine defect. The modeled decision is genuinely close and requires many pivotal observations unless additional structure proves the balance or breaks it.

#### Case D — broad and decisive

\[
q\text{ large},
\qquad
|\tau|\text{ large}.
\]

The winner should become obvious quickly even under uniform sampling.

The project's current empirical success is consistent with B, D, or a mixture. The proposed audit is needed to determine which.

---

## 3. The major statistical boundary: discovery is not evaluation

Paired evaluation solves the variance problem for a **fixed pair**. It does not, by itself, solve adaptive plan-selection bias.

Suppose Walt uses a scenario sample to choose

\[
\widehat\rho_a
 =\arg\max_{\rho\in\Pi_a}\widehat V_\rho
\]

and similarly for \(b\). Then the pair itself was selected because it looked good on that sample. The difference

\[
\widehat V_{\widehat\rho_a}
-
\widehat V_{\widehat\rho_b}
\]

is a difference of adaptively selected maxima. Common random numbers do not make that object unbiased.

### 3.1 Required separation

The clean experimental protocol is:

1. **discover** candidate continuation plans on a training scenario set;
2. **freeze** the exact executable plans or policy DAGs;
3. **evaluate** every frozen plan on a disjoint common scenario panel;
4. select among that fixed candidate set using simultaneous or sequential-valid paired inference;
5. report absolute values either with selection-aware intervals or on a further reporting holdout.

For the operational player, the evaluation panel can still choose the root action among the frozen candidate set. What must not happen silently is using evaluation outcomes to mutate the plans while retaining fixed-plan confidence claims.

### 3.2 Plan instability is a separate error source

Independent discovery samples may produce different continuation plans even when the root action is stable. This should be measured, not folded into the pivotal variance.

Syntactic plan fingerprints are useful for debugging, but they can overstate meaningful instability because plans may differ only on unreachable or negligible branches. The stronger comparison is behavioral:

- replay independently discovered plans on one common reference panel;
- measure their outcome disagreement mass;
- measure root-action selection frequency across discovery seeds;
- measure held-out value spread.

If plan instability dominates fixed-plan evaluation noise, more holdout scenarios are the wrong expenditure. The project should spend on better plan discovery, larger discovery samples, regularization, reusable plan libraries, or search bounds.

### 3.3 Root-action flip thresholds are local

For fixed plans, the tie condition is the single hyperplane

\[
\langle\beta,\alpha_{\rho_a}-\alpha_{\rho_b}\rangle=0.
\]

For optimized root actions,

\[
Q_a(\beta)-Q_b(\beta)
=
\max_{\rho\in\Pi_a}\langle\beta,\alpha_\rho\rangle
-
\max_{\pi\in\Pi_b}\langle\beta,\alpha_\pi\rangle.
\]

Along a one-parameter belief path, this is piecewise linear, not necessarily one line. It can have multiple plan switches and multiple crossings.

Accordingly, a printed “flip threshold” must name:

- the frozen plan pair;
- the belief perturbation family;
- the interval over which those plans remain active.

Without those conditions, the correct phrase is **local flip surface**, not exact global threshold.

---

## 4. Scenario geometry versus world geometry

The exact count engine acts on physical worlds \(\omega\). The pivotal set is naturally defined on scenarios \(\xi=(\omega,r)\). Those are not interchangeable.

Assume the tape law is independent of the physical-world draw under the modeled scenario distribution. Define, for each physical world,

\[
d(\omega)=\mathbb E_r[Y(\omega,r)],
\]

and

\[
s(\omega)=\mathbb E_r[Y(\omega,r)^2]
          =\Pr_r(|Y|=1\mid\omega).
\]

Then

\[
\boxed{g=\mathbb E_\omega[d(\omega)]}
\]

and

\[
\boxed{q=\mathbb E_\omega[s(\omega)].}
\]

This projection identifies whether disagreement is primarily structural in the deal or stochastic in the modeled dynamics.

### 4.1 Tape-stable and tape-sensitive worlds

A world is **tape-stable for the pair** when the sign and occurrence of the comparison do not depend on the tape. Examples include:

- always zero: the plans agree for every tape;
- always positive: \(a\) wins for every tape;
- always negative: \(b\) wins for every tape.

A world is **tape-sensitive** when different tapes produce different pair outcomes.

This distinction matters to the exact road:

- if most pivotal worlds are tape-stable, structural predicates and exact world counts may nearly solve the response geometry;
- if most pivotal mass is tape-sensitive, exact world counting alone cannot finish the comparison. The dynamics/tape expectation remains a separate object to integrate or bound.

The first experiment should therefore include a small repeated-tape panel per selected world, not only one tape per world.

---

## 5. Counted structural strata

Let \(A_1,\ldots,A_J\) be a partition of the physical-world fiber into structural atoms or strata for which the engine provides exact weights

\[
w_j=\beta_\omega(A_j).
\]

Under a uniform fiber belief these weights may be exact count ratios. Under a nonuniform belief, cardinalities alone are insufficient unless the belief is constant within each atom or its mass can otherwise be summed exactly.

For a frozen plan pair define

\[
\mu_j=\mathbb E[Y\mid \omega\in A_j]
\]

and

\[
\sigma_j^2=\operatorname{Var}(Y\mid \omega\in A_j).
\]

Then

\[
\boxed{g=\sum_{j=1}^J w_j\mu_j.}
\]

If \(n_j\) independent conditional scenarios are drawn in stratum \(j\), the stratified estimator

\[
\widehat g=\sum_j w_j\widehat\mu_j
\]

has variance

\[
\boxed{
\operatorname{Var}(\widehat g)
  =\sum_j\frac{w_j^2\sigma_j^2}{n_j}.
}
\]

For equal per-sample costs, the classical variance-minimizing allocation is

\[
n_j\propto w_j\sigma_j.
\]

If a conditional sample in stratum \(j\) costs \(c_j\), the cost-aware allocation is

\[
\boxed{
n_j\propto\frac{w_j\sigma_j}{\sqrt{c_j}}.}
\]

The \(\sigma_j\) values can be estimated by a pilot, with exploration floors so a falsely quiet stratum is not abandoned prematurely.

### 5.1 Exact intervals compose linearly

Suppose each stratum has a valid interval

\[
\mu_j\in[L_j,U_j].
\]

Because the weights are nonnegative and exact,

\[
\boxed{
 g\in
 \left[
   \sum_j w_jL_j,
   \sum_j w_jU_j
 \right].
}
\]

This gives an anytime refinement mechanism. The pair is settled once the aggregate interval excludes zero.

In proof-oriented mode, prioritize a stratum by its contribution to unresolved interval width,

\[
w_j(U_j-L_j),
\]

relative to the cost of tightening it.

In variance-oriented mode, the approximate marginal variance reduction from one more sample in stratum \(j\) is

\[
\Delta_j
\approx
\frac{w_j^2\widehat\sigma_j^2}
     {n_j(n_j+1)}.
\]

Per unit cost, spend next where

\[
\boxed{
\frac{w_j^2\widehat\sigma_j^2}
     {c_j n_j(n_j+1)}
}
\]

is largest, subject to validity floors and the fact that only unresolved top-candidate comparisons matter at the root.

This is a concrete answer to “where should the next unit of compute go?”

---

## 6. Pivotal envelopes and their exact cost effect

A structural predicate \(P(\omega)\) is a **proved pivotal envelope** for a frozen pair when

\[
Y(\omega,r)\neq0
\quad\Longrightarrow\quad
P(\omega)
\]

for every tape \(r\).

Let

\[
w=\beta_\omega(P).
\]

Then \(Y=0\) outside \(P\), so

\[
g=w\,\mathbb E[Y\mid P].
\]

A conditional estimator based only on scenarios generated inside \(P\) is unbiased after multiplication by \(w\).

### 6.1 Envelope hardness

The conditional estimator has variance

\[
\operatorname{Var}(w\overline Y_P)
=\frac{wq-g^2}{n_P}.
\]

Its variance-to-margin scale is

\[
\boxed{
H_P
 =\frac{wq-g^2}{g^2}
 =\frac{w}{q\tau^2}-1.
}
\]

Three cases are important:

- uniform sampling: \(w=1\), so \(H_P=1/(q\tau^2)-1\);
- exact pivotal envelope \(P=\Delta\): \(w=q\), so \(H_P=1/\tau^2-1\);
- loose envelope: the gain decreases linearly with its exact mass \(w\).

For the simpler “collect \(K\) pivotal signs” protocol:

- uniform scenarios produce a pivotal observation at rate \(q\);
- envelope-conditional scenarios produce one at rate \(q/w\).

Thus the expected scenario-count speedup is approximately

\[
\boxed{\frac{1}{w}.}
\]

The wall-clock speedup is approximately

\[
\boxed{
\frac{c_{\text{uniform}}}
     {w\,c_P},
}
\]

where \(c_P\) is the measured cost of generating and replaying a conditional scenario. A tiny envelope is not useful if conditional generation is proportionally more expensive.

### 6.2 Approximate envelopes require a complement floor

A learned or mined predicate \(\widehat P\) is not safe for pivotal-only sampling unless containment is proved. The unbiased two-stratum identity is

\[
g
=w\,\mathbb E[Y\mid\widehat P]
 +(1-w)\,\mathbb E[Y\mid\neg\widehat P].
\]

Until the complement is proved zero, it must receive samples or a rigorous bound. Observing no pivots outside \(\widehat P\) is evidence of coverage, not proof of coverage.

### 6.3 Exact counting and exact generation are separate capabilities

The engine may know \(w\) exactly while lacking an efficient uniform generator inside \(P\). Rejection from the full fiber costs roughly \(1/w\) proposals and can erase the theoretical gain. The experiment must measure:

- exact count correctness;
- conditional generator uniformity;
- conditional generation cost;
- pivotal density inside the envelope;
- observed pivotal leakage outside it.

---

## 7. The Walt refinement signature

A single global sample number such as “200 is rough; 2,000 is good” hides distinct failure modes. For each unresolved top-candidate pair, log a **refinement signature** with five groups.

### 7.1 Decision geometry

\[
(g,q,\tau,H).
\]

These answer: current margin, disagreement frequency, disagreement direction, and fixed-pair evaluation difficulty.

### 7.2 Structural geometry

\[
(w_P,\ r_P),
\qquad
r_P=\frac{q}{w_P}.
\]

Here \(w_P\) is envelope mass and \(r_P\) is pivotal density inside it. These answer whether the count/query engine can buy a real variance reduction.

### 7.3 Search stability

Record:

- root-action stability across independent discovery samples;
- behavioral disagreement among independently discovered continuation plans;
- held-out value spread among those plans;
- any available root-action search upper-bound slack.

These answer whether the uncertainty is plan search rather than plan evaluation.

### 7.4 Particle and depth health

Record, at minimum:

- minimum alive-set size along material branches;
- effective sample size where weights are nonuniform;
- maximum contribution of any one scenario to the root estimate;
- depth at which survivor collapse occurs.

These answer whether the outer sample looks large while deep conditional decisions rest on almost no support.

### 7.5 Measured cost

Record:

- world generation cost;
- conditional generation cost by stratum;
- frozen-plan replay cost;
- plan-discovery/search cost;
- tape multiplicity cost.

These turn mathematical sample scales into wall-clock predictions.

### 7.6 Spend routing

The signature determines the next action.

| Observed pattern | Interpretation | Spend next on |
|---|---|---|
| Confidence interval for \(g\) excludes zero; plans stable; search slack closed | Decision settled at current tier | Stop |
| Small \(q\), large \(|\tau|\), stable plans | Sparse decisive boundary; uniform sampling wastes scenarios | Structural envelope / conditional generation |
| Moderate or large \(q\), small \(|\tau|\) | Genuine modeled near-tie | More paired pivotal signs, or declare practical indifference |
| High plan instability, low fixed-pair evaluation error | Continuation discovery is the bottleneck | Larger discovery sample, better search, regularization, plan reuse |
| Large search upper-bound slack | An unseen continuation may beat the frozen plan | Search expansion / branch bounds, not more replay |
| Deep alive-set collapse | Conditional beliefs are under-resolved | Node regeneration, depth-scaled samples, or exact conditioned generation |
| Envelope mass large or pivotal density low | Structural predicate is not buying concentration | Improve the predicate or stay uniform |
| Tape sensitivity dominates within worlds | Count-only exactness cannot finish the response lock | Exact/controlled dynamics integration, repeated tapes, or control variates |

This table is the operational form of the mathematics.

---

## 8. The three locks on exact Walt

“Exact counts” and “exact decision” must not be conflated. A fully exact root decision needs all three locks closed.

### 8.1 Measure lock

For every region used in the comparison, know its exact belief mass under the actual target belief, not merely its cardinality under a different fiber.

Possible routes:

- exact uniform fiber counts;
- exact weighted counts for a structured nonuniform belief;
- a proved sufficient partition on which belief mass is exactly available.

### 8.2 Response lock

For each frozen plan and relevant region, know the exact expected Boolean outcome under the modeled field and tape.

Possible routes:

- deterministic response on the region;
- finite exact enumeration of tape branches;
- dynamic programming over field randomness;
- rigorous upper/lower response bounds tightened only on pivotal atoms.

### 8.3 Optimization lock

Prove that no omitted information-consistent continuation plan under a competing root action has greater value.

Possible routes:

- exhaustive plan enumeration at the relevant grade;
- branch-and-bound over partial policy DAGs;
- admissible upper bounds on unresolved continuations;
- an exact active-plan/facet library over the target belief region.

Pivotal geometry directly attacks the measure and response locks for a frozen pair. It helps the optimization lock only indirectly by identifying which root contenders and partial-plan branches can still change the result.

### 8.4 Exactness escalation ladder

A practical ladder is:

1. **Uniform sampled solve.** Existing Walt behavior.
2. **Frozen-plan independent evaluation.** Honest fixed-pair gap and instability measurements.
3. **Exact stratum weights.** Sample response within counted atoms; combine without measure noise.
4. **Proved pivotal envelope.** Stop sampling structurally irrelevant worlds.
5. **Signed atom refinement.** Prove or tightly bound \(\mu_j\) atom by atom; aggregate exact weights.
6. **Exact signed pivotal regions.** Count \(\Delta^+\) and \(\Delta^-\), or exact tape-integrated analogues, so the frozen-pair gap is exact.
7. **Search-bound closure.** Prove the active plans are globally optimal under their root actions, making the root action exact.

Every rung is useful. Failure to reach rung 7 does not invalidate the earlier speed and confidence gains.

---

## 9. The necessary experiment: E0 frozen-plan signed-pivotal audit

This experiment should precede implementation of a new stratified sampler. It tests the mathematical mechanism and tells the project what to build.

### 9.1 Questions E0 must answer

1. Are current strong decisions easy because \(q\) is small, \(|\tau|\) is large, or both?
2. Does \(H\) predict the empirical sample count needed to recover the reference winner?
3. Are continuation plans stable enough that fixed-plan evaluation is the relevant bottleneck?
4. Can small structural world predicates capture most pivotal mass?
5. Is pivotal behavior mostly tape-stable or tape-sensitive?
6. Do signed boundary patterns recur across positions strongly enough to support a reusable boundary library?

### 9.2 Corpus

Use at least three groups:

1. the historical level-2 trick-1 saturation/tie episode;
2. positions where the new first-play champion beats the previous champion;
3. a broader stratified corpus of early-trick decisions, including ordinary easy decisions and known possessed/guarded-count positions.

The single historical tie is a regression anchor, not sufficient evidence by itself.

### 9.3 Phase A — discovery replicates

For each root position:

1. Run the current Walt discovery solve at the current standard budget, initially \(n=200\).
2. Repeat across independent discovery seeds. Eight repeats are enough for a smoke test; 16–32 are preferred for a stable instability estimate.
3. Save, for every legal root action:
   - the selected executable continuation plan;
   - a canonical or content-addressed plan identifier;
   - search-node and policy-DAG size;
   - root on-training value;
   - alive-set/depth diagnostics.
4. Deduplicate plans behaviorally by replaying them later on the common panel. Do not rely only on syntax.

### 9.4 Phase B — one large common replay panel

Construct an evaluation panel disjoint from every discovery sample.

- The existing \(n=800\) panel is enough for a first smoke test.
- Because frozen-plan replay should be much cheaper than re-solving, the preferred panel is as large as practical: 10,000 or more scenarios for anchor positions, with all frozen plans evaluated on exactly the same scenarios.
- Store world ID and tape seed separately.

For every frozen plan pair that can affect the root winner, record:

\[
N_+,\quad N_-,\quad N_0,
\]

and estimate

\[
\widehat q=\frac{N_++N_-}{n},
\qquad
\widehat\tau=\frac{N_+-N_-}{N_++N_-}\quad\text{when }N_++N_->0,
\qquad
\widehat g=\frac{N_+-N_-}{n},
\]

with

\[
\widehat H=\frac{\widehat q-\widehat g^2}{\widehat g^2}
\]

when the denominator is nonzero.

Use fixed-sample intervals for fixed analyses. If the implementation repeatedly checks and extends the panel until separation, use predeclared alpha spending or an anytime-valid confidence sequence. Ordinary fixed-\(n\) intervals are not valid under unrestricted repeated peeking.

### 9.5 Phase C — sample-size calibration by subsampling

Treat the large common panel as the reference distribution for the frozen plans.

For sample sizes

\[
25,50,100,200,400,800,\ldots
\]

repeatedly subsample or replay prefixes and measure:

- probability of selecting the reference winner;
- empirical gap error;
- pivotal observations required to settle the sign;
- predicted versus observed cost from \((q,\tau,H)\).

The central plot is not make-rate error. It is **winner recovery versus predicted fixed-pair hardness**.

### 9.6 Phase D — plan-instability audit

Across discovery replicates, report:

- frequency of each root action;
- number of distinct syntactic plans;
- number of behaviorally distinct plans on the common panel;
- pairwise behavioral disagreement mass among plans for the same root action;
- held-out value spread;
- whether plan switching changes the root winner.

This separates “we need more scenarios to evaluate this plan” from “the sample changed which plan Walt invented.”

### 9.7 Phase E — world/tape decomposition

For a selected subset of physical worlds, evaluate multiple independent tapes per world. Estimate

\[
d(\omega)=\mathbb E_r[Y\mid\omega]
\]

and

\[
s(\omega)=\Pr_r(|Y|=1\mid\omega).
\]

Report the fraction of estimated pivotal mass arising from:

- tape-stable positive worlds;
- tape-stable negative worlds;
- tape-sensitive worlds.

This determines how much exact world counting can plausibly buy without a separate exact dynamics treatment.

### 9.8 Phase F — offline structural-envelope audit

Before writing a conditional sampler, mine candidate predicates \(P\) from the existing common panel. For each predicate report:

- exact or estimated belief mass \(w_P\);
- observed pivotal coverage;
- observed pivotal density \(q/w_P\);
- pivotal leakage outside \(P\);
- estimated conditional-generation cost;
- predicted wall-clock gain using \(c_{\text{uniform}}/(w_Pc_P)\).

A predicate should not be promoted to pivotal-only sampling until containment is proved. The offline audit tells whether proving or engineering it is worth the effort.

### 9.9 E0 falsifiers

The pivotal explanation is weakened if:

- \(q\) is usually large and \(|\tau|\) usually tiny, yet small samples remain stable for reasons not captured by the model;
- \(H\) fails to predict winner-recovery cost even for frozen plans;
- continuation-plan instability dominates evaluation noise;
- candidate structural envelopes have mass near one or miss material pivotal mass;
- most pivotal behavior is tape-sensitive and structurally diffuse;
- boundary signatures fail to recur across comparable positions.

It is strengthened if:

- most scenarios cancel;
- pivotal disagreements have strong tilt;
- observed sample cost tracks \(1/(q\tau^2)\);
- independent discovery runs produce behaviorally equivalent active plans;
- small counted envelopes capture nearly all disagreement;
- signed boundary motifs repeat under the existing equivariance quotient.

### 9.10 E0 decision gates

After E0:

- **Build the stratifier** only if envelope mass and conditional-generation cost predict a real wall-clock gain.
- **Build plan/facet reuse** if discovery repeatedly returns behaviorally equivalent plans or boundaries.
- **Invest in search stabilization** if plan instability dominates.
- **Invest in exact dynamics or control variates** if tape sensitivity dominates.
- **Treat some positions as honest near-ties** if \(|\tau|\) remains near zero after large pivotal evidence.

---

## 10. A better simplicity hypothesis

The imported handoff proposes counting undominated plan signatures and treating an exploding plan census as a falsifier of “a relatively simple solution exists.” That falsifier is too strong.

Many plans can exist while the decision rule remains simple. Plans may differ only on rare branches, unreachable histories, or regions that cancel in every relevant comparison. The more direct objects are the signed boundaries

\[
Y_{a,b}=\alpha_{\rho_a}-\alpha_{\rho_b}.
\]

### 10.1 Three censuses, not one

Measure separately:

1. **syntactic plan count** — executable policy-DAG diversity;
2. **behavioral plan count** — distinct outcome vectors on a common canonical panel;
3. **signed boundary count** — distinct \(-1/0/+1\) comparison vectors or structural descriptions.

An exploding syntactic census kills only the small universal plan-library hypothesis. It does not kill a small boundary library, low-dimensional decision geometry, or a compact structural rule system.

### 10.2 Decision dimension

Fix a set of active frozen plans and choose one reference plan \(\rho_0\). Consider the family of signed vectors

\[
\alpha_\rho-\alpha_{\rho_0}.
\]

Their real linear span has dimension

\[
r=\operatorname{rank}\{\alpha_\rho-\alpha_{\rho_0}\}.
\]

For that plan library, every comparison depends on the belief only through \(r\) linear coordinates. Thus:

> A large plan library can still induce a low-dimensional decision problem.

On samples, measure the singular spectrum rather than only exact rank. On structural atoms, measure rank after the equivariance quotient. A rapidly decaying spectrum would be evidence that Walt's belief dependence can be compressed into a small set of counted decision coordinates.

### 10.3 Boundary motifs are the likely exact bridge

The strongest exact-road hypothesis is not “there are few plans.” It is:

> **The signed pivotal regions of active plan pairs can be expressed as a small family of structural motifs whose belief masses are exactly countable.**

If true, the final player need not replay whole plans on whole worlds. It can evaluate a compact signed boundary table:

\[
g(a,b)
=\sum_j w_j d_{a,b,j},
\]

where \(w_j\) are exact belief masses of structural atoms and \(d_{a,b,j}\) are exact or tightly bounded signed responses.

That is the bridge from “sampling usually works” to “the decision is a small exact weighted sum.”

---

## 11. Amortizing many belief and dynamics experiments

An ultra-efficient Walt is valuable not only for live play but for experimentation. The pivotal/facet view separates two kinds of reuse.

### 11.1 Belief experiments: reuse outcomes, change weights

For a fixed field model and frozen plan library, build an outcome matrix

\[
U_{\rho,i}=u_\rho(\xi_i).
\]

A new belief over the same scenario support changes only the weights. Plan values become matrix-vector products. Pairwise boundaries become dot products with signed rows.

On a counted structural partition, store

\[
M_{\rho,j}=\mathbb E[u_\rho\mid A_j].
\]

Then any belief represented by exact atom weights \(w_j\) can be evaluated as

\[
V_\rho=\sum_jw_jM_{\rho,j}.
\]

This makes large belief sweeps nearly free once the response table exists.

### 11.2 Facet discovery over a belief region

Use Walt as a plan-discovery oracle:

1. solve at selected belief points;
2. add newly active plans to a library;
3. compute the upper envelope of known plan values across the belief region;
4. query Walt where the current library is uncertain or likely incomplete;
5. repeat until held-out regret is negligible or an exact search upper bound closes the region.

Within a complete library, all local flip surfaces and plan-switch points are explicit. This is a more useful experiment engine than rerunning full Walt independently at every belief.

### 11.3 Dynamics experiments: pair the models too

For two field/dynamics models \(\sigma_0\) and \(\sigma_1\), use the same physical worlds and compatible random tapes. Estimate the difference

\[
\mathbb E[Y^{(1)}]-\mathbb E[Y^{(0)}]
=\mathbb E[Y^{(1)}-Y^{(0)}].
\]

If the models are similar, the paired difference can have much lower variance than two independent solves. A cheap lower-level model can also serve as a control variate:

\[
\mathbb E[Y^{(k)}]
=
\mathbb E[Y^{(k-1)}]
+
\mathbb E[Y^{(k)}-Y^{(k-1)}].
\]

The expensive computation is then concentrated on the correction induced by the richer dynamics. This is the dynamics analogue of pivotal cancellation.

---

## 12. Target architecture if E0 succeeds

A plausible ultra-efficient Walt has five separable components.

### 12.1 Discovery solver

Produces candidate information-consistent plan DAGs and any admissible search bounds. It is allowed to be expensive because it is invoked selectively and its output is reusable.

### 12.2 Common-panel replay kernel

Evaluates all frozen plans on the same scenario panel. Store outcome vectors as packed bits. For two plans:

- \(N_+\) is a popcount of `A AND NOT B`;
- \(N_-\) is a popcount of `B AND NOT A`;
- \(N_0\) is the remainder.

Once outcomes are stored, the pivotal profile of every pair is cheap.

### 12.3 Sequential candidate racer

Maintains valid paired intervals for unresolved contenders, eliminates a candidate when another is provably better at the chosen confidence level, and spends only on comparisons that can still change the root move. If the candidate set changes, stored scenarios are replayed for the new plan and fixed-pair evidence is relabeled correctly.

### 12.4 Counted stratifier/refiner

Maintains exact stratum weights, conditional response estimates or bounds, and allocates the next sample according to marginal decision-value reduction per unit cost.

### 12.5 Boundary/facet cache

Caches canonical active plans, signed boundary motifs, structural envelopes, and atom response tables across equivariant positions. Belief sweeps become reweighting; dynamics sweeps use paired corrections.

This architecture does not assume the final exact compression exists. It earns speed immediately and produces the measurements needed to discover whether the exact compression exists.

---

## 13. Minimal implementation contract for E0

For each root computation, persist:

- root information-state/fiber identifier;
- declaration, bid, trick grade, seat, and field-model version;
- discovery sample IDs and seeds;
- evaluation world IDs and tape seeds;
- legal root actions;
- frozen plan identifiers and serialized executable plans;
- outcome bitset per plan on the common panel;
- training value and holdout value;
- plan-DAG size and search statistics;
- alive-set size or effective sample size by material node/depth;
- root winner and runner-up;
- \(N_+,N_-,N_0,q,\tau,g,H\) for unresolved pairs;
- structural predicate memberships used in envelope audits;
- measured generation, replay, and solve costs.

A report should include, at minimum:

1. winner-recovery curves by raw sample count;
2. winner-recovery curves by pivotal-observation count;
3. predicted versus observed cost using \(1/(q\tau^2)\);
4. action and plan instability across discovery seeds;
5. envelope mass, coverage, density, and wall-clock prediction;
6. tape-stable versus tape-sensitive pivotal mass;
7. boundary-signature recurrence and approximate decision rank.

---

## 14. Obligations added by this treatment

### O12 — Frozen-plan typing

Every pivotal estimate names the exact frozen plan pair. Root-action claims state whether active-plan stability has been checked.

### O13 — Discovery/evaluation separation

Fixed-plan confidence claims use scenarios not used to construct or alter those plans, unless a valid uniform-complexity argument replaces sample splitting.

### O14 — Sequential validity

Any resample-until-separated protocol uses an anytime-valid method or a predeclared checkpoint/alpha-spending schedule.

### O15 — Scenario/world domain match

Every exact structural count is over the same physical-world fiber and belief measure targeted by the conditional sampler. Tape integration remains explicit.

### O16 — Envelope containment

Pivotal-only sampling requires proof that the envelope contains all pivotal scenarios after projection to worlds. Otherwise the complement retains a sampling floor or rigorous bound.

### O17 — Conditional generator correctness and cost

Exact stratum weights do not substitute for a correct, efficient conditional generator. Uniformity and wall-clock cost are tested separately.

### O18 — Optimization-lock accounting

An exact frozen-pair result is not labeled an exact root-action result unless competing continuation-plan upper bounds are closed.

### O19 — Behavioral census

Plan simplicity claims use behavioral and signed-boundary equivalence, not syntactic plan counts alone.

---

## 15. Claims this document does not make

- It does not claim the active pivotal regions are small. E0 measures that.
- It does not claim small pivotal mass alone explains sampling success. Tilt and plan stability are equally load-bearing.
- It does not claim a structural world predicate can exactly represent a scenario-level pivotal set.
- It does not claim an exact count ratio is an exact probability under arbitrary learned beliefs.
- It does not claim fixed-plan exactness proves root-action optimality.
- It does not claim a large plan census kills all simple-solution hypotheses.
- It does not claim one global sample budget can be correct across positions.
- It does not claim a global flip threshold where active continuation plans may switch.
- It does not claim the external literature mappings in the imported handoff have been verified here.

---

## 16. Recommended order of work

1. **Instrument and run E0 using existing artifacts first.** No new sampler is required for the smoke test.
2. **Build the common-panel bitset replay and reporting path.** This is useful regardless of the hypothesis outcome.
3. **Measure fixed-pair hardness, plan instability, and tape sensitivity.** Decide which bottleneck is real.
4. **Audit structural envelopes offline.** Do not build conditional generation until predicted wall-clock gain is positive.
5. **Add sequential-valid candidate racing.** Replace global fixed sample counts with decision-specific refinement.
6. **Add exact counted strata and adaptive allocation.** Preserve a complement floor until containment is proved.
7. **Run the boundary/facet census and decision-rank measurement.** Evaluate held-out regret, not count alone.
8. **Pursue exact signed atoms where recurrence is strongest.** This is the most plausible bridge to an exact fast Walt.
9. **Only then attack optimization-lock closure.** Use the learned boundary structure to prioritize plan branches and search bounds.

---

## 17. Final mathematical thesis

The number of possible deals is not the operative complexity of a fixed Walt decision. The operative object is the signed disagreement function

\[
Y_{a,b}:\Xi\to\{-1,0,+1\}.
\]

Its first two moments are exactly

\[
\mathbb E[Y]=g,
\qquad
\mathbb E[Y^2]=q.
\]

Its decision geometry is exactly

\[
g=q\tau.
\]

Its fixed-pair sampling difficulty is controlled by

\[
q\tau^2.
\]

Its count-powered acceleration is controlled by the mass and cost of a proved structural envelope. Its exactness is controlled by three separate locks: measure, response, and optimization. Its cross-belief compressibility is controlled not only by plan count but by signed-boundary recurrence and decision dimension.

That gives the project a much stronger research program than “try 200, then 2,000.” It gives Walt:

- a per-decision cost scale;
- a reasoned stopping rule;
- a vector that says where additional compute belongs;
- a route to conditional sampling without bias;
- a route to reuse across many belief and dynamics experiments;
- and a credible bridge from empirical strength to exact counted decision geometry.

The bridge is not proved to exist. The proposed experiment is designed to discover whether it does.
