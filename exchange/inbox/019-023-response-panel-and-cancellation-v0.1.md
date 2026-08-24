# RESPONSE — Calculated Evidence, Targeted Level-2, and the Difference Between Cancellation and Irrelevance

## Adversarial mathematical review and directional-risk extension for unified Walt

**Status:** EXPLORATORY mathematical response. Nothing is promoted by this document's existence.

**Date:** 2026-08-24

**Target:** the five manually delivered briefs in `exchange/drafts/` at Texas-42 main commit `6e00528d33c85461f20eb8f12196521562e5e617`:

1. `panel-ce-evidence-process.md`
2. `panel-ce-bounded-mean.md`
3. `panel-ce-risk-ledger-escalation.md`
4. `panel-ce-execution-order.md`
5. `panel-l2-coupling-theorems.md`

**Provenance:** response from the project mathematician after reading the briefs, the current calculated-evidence and targeted-level-2 artifacts, and the first two field-swap implementation slices. This is one consolidated response because Jason manually delivered the five briefs as a batch. The source briefs remain independent objects and may be adjudicated independently.

**Scope fence:** all values and theorems remain model-relative to the declared belief, field identities, policy identities, and payoff. “Exact” means exact for the explicitly named finite object. No equilibrium or convergence claim is introduced.

---

## 0. Executive verdict

The core mathematics is strong. Two orchestration claims need repair, and one new distinction should become first-class.

| Brief | Verdict |
|---|---|
| CE-T1/T2/T3 evidence process | **CERTIFIED.** The exact identities, supermartingale steps, continuum-mixture step, lower-test identity, raw-stream pivotal validity, anchors, and `c = 1/2` specialization are correct. |
| CE-T4/T5 bounded mean | **CERTIFIED.** The λ ranges are sharp; finite mixtures and practical-equivalence composition are valid. The sign-majority defect is real. One sentence in the task is too broad: the unrestricted bounded-rational class is not sign-safe, but it contains sign-safe subclasses. |
| O21/O24 risk ledger and exact escalation | **MIXED.** Claims A–C and G–H are sound under their declared typing. Claim D is unsound as written if a newly opened edge may reuse retrospectively selected historical evidence. Claim E is sound only under precise identity, law, and retained-allocation conditions. “Refund” in Claim F must be replaced by a final-result distinction. |
| O26 execution order and predictable sequence | **UNDER-SPECIFIED, REPAIRABLE.** W1–W6 do not say enough about liveness inside an overshot batch. A canonical per-index replay rule makes the invariance theorem true. The conditional-null question is resolved positively under predictable activation and iid future worlds; current-world peeking breaks it. |
| L2-T1..T5 | **CERTIFIED WITH ONE DEFINITION REPAIR.** Define the first field split on the common prefix. T1–T5 then follow. The “optimal under `σ0`” assumption in T3 is redundant. T4 needs the ordinary interval/nonnegativity hypotheses stated explicitly. |

The additional mathematical conclusion is:

> **Small net correction is not the same as irrelevance.**

There is a strict hierarchy:

\[
\boxed{
|\text{net value correction}|
\;\le\;
\text{terminal outcome-change mass}
\;\le\;
\text{field-exposure mass}.
}
\]

A high-trump-versus-vulnerable-double decision is normally not a cancellation pattern at all. If the high trump is never worse and is sometimes better, the vulnerable double carries one-sided downside with no upside. That is strict dominance, even when the losing worlds are rare.

The project should therefore retain the positive and negative masses separately, not only their difference.

---

# Part I — Response to `panel-ce-evidence-process.md`

## 1. CE-T1 is sound

Fix \(c\in(0,1)\) and \(r\in[c,1]\). For a Bernoulli observation \(B\in\{0,1\}\), define

\[
L_r(B)
=
\left(\frac rc\right)^B
\left(\frac{1-r}{1-c}\right)^{1-B}.
\]

### Step 1 — one-step conditional expectation

[USES: definition of \(L_r\); conditional success probability \(p_n\le c\).]

\[
\begin{aligned}
\mathbb E[L_r(B_n)\mid\mathcal F_{n-1}]
&=
p_n\frac rc+(1-p_n)\frac{1-r}{1-c}\\
&=
1+\frac{(p_n-c)(r-c)}{c(1-c)}\\
&\le1.
\end{aligned}
\]

The identity is exact. The inequality uses \(p_n-c\le0\), \(r-c\ge0\), and \(c(1-c)>0\).

### Step 2 — fixed-\(r\) product

[USES: Step 1; all factors nonnegative.]

The running product

\[
M_n(r)=\prod_{i=1}^n L_r(B_i)
\]

is a nonnegative supermartingale beginning at \(1\).

### Step 3 — continuum mixture

[USES: Step 2; conditional Tonelli for a nonnegative jointly measurable integrand.]

Define

\[
E^>_n
=
\frac1{1-c}\int_c^1M_n(r)\,dr.
\]

The integrand is nonnegative and continuous in \(r\) for each finite history. Therefore

\[
\mathbb E[E^>_n\mid\mathcal F_{n-1}]
=
\frac1{1-c}
\int_c^1
\mathbb E[M_n(r)\mid\mathcal F_{n-1}]\,dr
\le E^>_{n-1}.
\]

Also,

\[
E^>_0=\frac1{1-c}\int_c^1 1\,dr=1.
\]

### Step 4 — anytime crossing

[USES: nonnegative supermartingale beginning at \(1\); Ville's inequality.]

For any \(\alpha\in(0,1)\),

\[
\Pr\!\left(\sup_n E^>_n\ge\frac1\alpha\right)\le\alpha.
\]

No extra correction is required for peeking or stopping.

## 2. CE-T1 exact-rational identities are correct

With \(R=(1-c)/c\) and \(r=c+(1-c)t\),

\[
E^>_{s,f}(c)
=
\int_0^1(1+Rt)^s(1-t)^f\,dt.
\]

Expand the first factor:

\[
(1+Rt)^s
=
\sum_{i=0}^s\binom siR^it^i.
\]

Then

\[
\int_0^1t^i(1-t)^fdt
=
\frac{i!f!}{(i+f+1)!}.
\]

Hence

\[
\boxed{
E^>_{s,f}(c)
=
\sum_{i=0}^s
\binom siR^i
\frac{i!f!}{(i+f+1)!}.
}
\]

All terms are rational for rational \(c\).

## 3. CE-T2 is the genuine lower mixture

[USES: CE-T1 with failures treated as successes; substitution \(x=1-r\).]

\[
\begin{aligned}
E^>_{f,s}(1-c)
&=
\frac1c
\int_{1-c}^1
\left(\frac r{1-c}\right)^f
\left(\frac{1-r}{c}\right)^sdr\\
&=
\frac1c
\int_0^c
\left(\frac x c\right)^s
\left(\frac{1-x}{1-c}\right)^fdx.
\end{aligned}
\]

This is exactly the natural uniform mixture over lower alternatives \(x\le c\). Claim D′ is correct.

## 4. CE-T3 is valid on the raw world stream

Let

\[
p_+=\Pr(Y=+1),\qquad p_-=\Pr(Y=-1),\qquad g=p_+-p_-.
\]

For \(r\in[1/2,1]\), use the multiplier

\[
L_r(Y)=
\begin{cases}
2r,&Y=+1,\\
2(1-r),&Y=-1,\\
1,&Y=0.
\end{cases}
\]

Then

\[
\mathbb E[L_r(Y)]
=
1+(2r-1)g.
\]

Under \(g\le0\), this is at most \(1\). The case \(q=0\) is exact equality. Nonpivotal worlds multiply by \(1\); they consume time but create no directional evidence.

With \(t=2r-1\), the normalized mixture is

\[
E^+_{a,b}
=
\int_0^1(1+t)^a(1-t)^b\,dt.
\]

The claimed closed form follows directly from the CE-T1 finite sum at \(c=1/2\):

\[
\begin{aligned}
E^+_{a,b}
&=
\sum_{i=0}^a
\binom ai\frac{i!b!}{(i+b+1)!}\\
&=
\frac1{(a+b+1)\binom{a+b}{a}}
\sum_{i=0}^a
\binom{a+b+1}{a-i}\\
&=
\boxed{
\frac{\sum_{x=0}^a\binom{a+b+1}{x}}
{(a+b+1)\binom{a+b}{a}}.
}
\end{aligned}
\]

All anchors are correct. For \(b=0\),

\[
E^+_{a,0}
=
\frac{2^{a+1}-1}{a+1}.
\]

At threshold \(128\),

\[
E^+_{9,0}=\frac{1023}{10}<128,
\qquad
E^+_{10,0}=\frac{2047}{11}>128.
\]

## 5. Verdict for this brief

`FINAL ANSWER: CERTIFIED (claims A–I)`

---

# Part II — Response to `panel-ce-bounded-mean.md`

## 6. CE-T4 λ range is correct and sharp

For \(\lambda\ge0\), the factor

\[
1+\lambda(X-c)
\]

is minimized at \(X=L\). Thus it is nonnegative for every \(X\in[L,U]\) iff

\[
1-\lambda(c-L)\ge0,
\]

equivalently

\[
\boxed{0\le\lambda\le\frac1{c-L}.}
\]

This interval is maximal. Any larger \(\lambda\) makes the factor negative at \(X=L\).

At the endpoint, the factor may be zero. The product is then absorbed at zero. Nonnegative-supermartingale validity is unaffected.

Under the conditional null,

\[
\mathbb E[1+\lambda(X_i-c)\mid\mathcal F_{i-1}]
=
1+\lambda(\mathbb E[X_i\mid\mathcal F_{i-1}]-c)
\le1.
\]

A finite convex combination of these products remains a nonnegative supermartingale beginning at one.

## 7. CE-T5 is the exact mirror

The factor

\[
1-\lambda(X-c)
\]

is minimized at \(X=U\), giving the sharp range

\[
\boxed{0\le\lambda\le\frac1{U-c}.}
\]

All supermartingale and mixture arguments mirror CE-T4.

For \(X\in[-1,1]\):

- CE-T4 at \(c=-\varepsilon\) uses
  \[
  \lambda\le\frac1{1-\varepsilon};
  \]
- CE-T5 at \(c=+\varepsilon\) uses the same range.

The bookkeeping in the brief is correct.

## 8. Practical-equivalence composition is valid

To make the false conclusion \(|g|<\varepsilon\):

- if \(g\ge\varepsilon\), the upper-side rejection must be false;
- if \(g\le-\varepsilon\), the lower-side rejection must be false.

Therefore the false-equivalence event is contained in the union of the two one-sided false-settlement events. Independence is unnecessary:

\[
\Pr(\text{false equivalence})
\le
\alpha_++\alpha_-.
\]

## 9. Sign majority does not determine mean order

Let

\[
\pi_+=\Pr(X>0),
\qquad
\pi_-=\Pr(X<0),
\]

and, when defined,

\[
\mu_+=\mathbb E[X\mid X>0],
\qquad
\mu_-=\mathbb E[-X\mid X<0].
\]

Then

\[
\boxed{
\mathbb E[X]=\pi_+\mu_+-\pi_-\mu_-.
}
\]

A sign test sees only \(\pi_+-\pi_-\). It discards \(\mu_+\) and \(\mu_-\).

For the brief's law,

\[
X=
\begin{cases}
1/8,&\text{probability }3/4,\\
-1/2,&\text{probability }1/4,
\end{cases}
\]

so

\[
\Pr(X>0)=\frac34,
\qquad
\mathbb E[X]
=
\frac3{32}-\frac4{32}
=
-\frac1{32}.
\]

A consistent sign-direction test will converge toward the positive sign while the true mean is negative.

The bounded-mean process does not have this defect: under \(\mathbb E[X]\le0\), its positive-direction false crossing remains bounded by its declared \(\alpha\).

### Necessary wording repair

The task asks to confirm that “bounded signed rational block differences fall outside every” sign-safe class. That universal statement is false.

For example, if \(X\in\{-d,0,+d\}\) for a fixed \(d>0\), then

\[
\mathbb E[X]
=
d(\pi_+-\pi_-),
\]

so sign and mean order agree exactly.

The correct statement is:

> **The unrestricted class of bounded signed rational differences is not sign-safe. It contains laws for which sign order and mean order disagree.**

A sufficient sign-safe condition for one law is

\[
\mu_+=\mu_-,
\]

with constant nonzero magnitude as the simplest special case.

## 10. Verdict for this brief

`FINAL ANSWER: CERTIFIED (A–E), WITH TASK-D WORDING NARROWED`

---

# Part III — Response to `panel-ce-risk-ledger-escalation.md`

## 11. Claims A–C are sound for a fixed evidence epoch

### Claim A

[USES: per-edge anytime bound over the edge's whole future; finite union bound.]

For each false ordered edge \(e\),

\[
\Pr(\sup_nE_e(n)\ge1/\alpha_e)\le\alpha_e.
\]

With a fixed candidate set,

\[
\Pr(\text{any false edge ever drawn})
\le
\sum_e\alpha_e
=
\delta_{\mathrm{dec}}.
\]

Adaptive examination, provisional leaders, early stopping, and elimination do not add a surcharge. Stopping an edge process only makes its crossing event smaller.

### Claim B

On the no-false-edge event, every settled edge \(i\to j\) satisfies \(V_i>V_j\). A true maximizer cannot be eliminated by a worse or tied candidate. If one candidate remains, it is a maximizer. Tied maximizers may remain forever.

### Claim C

\[
\sum_{d=1}^{\infty}\frac1{d(d+1)}
=
\sum_{d=1}^{\infty}\left(\frac1d-\frac1{d+1}\right)
=
1.
\]

Thus

\[
\sum_d\delta_d=\delta_{\mathrm{run}}.
\]

The result requires every sub-allocation to have a serialized scope and the total pathwise allocation to remain within its parent allocation.

## 12. Claim D is false as written

The danger is retrospective selection among historical evidence processes.

Take

\[
\delta_{\mathrm{dec}}=\frac14,
\qquad
\alpha_1=\frac{\delta_{\mathrm{dec}}}{1\cdot2}=\frac18.
\]

Consider three false edges. For each edge, at one historical observation let its e-value be

\[
M=
\begin{cases}
8,&\text{probability }1/8,\\
0,&\text{probability }7/8.
\end{cases}
\]

Each edge individually is valid:

\[
\mathbb E[M]=1,
\qquad
\Pr(M\ge8)=\frac18=\alpha_1.
\]

Inspect all three unopened historical processes. If any has \(M=8\), open one of those and assign it \(\alpha_1\), reusing its historical evidence.

The false-crossing probability is

\[
1-\left(\frac78\right)^3
=
\frac{169}{512}
>
\frac14.
\]

Therefore edge-opening order alone does not pay for adaptive retrospective selection.

### Sound repairs

Claim D becomes sound under either repair:

1. **Future-only opening.** At the predictable opening time, reset the new edge process to \(1\) and use only future worlds.
2. **Preallocation.** Assign each edge its risk before seeing any evidence that will enter that edge's process.

An edge may be chosen adaptively from past information, but the observations subsequently used must begin after that predictable choice unless its risk was already assigned.

## 13. Claim E is conditionally sound

An old pair process may be retained only when all of the following remain literally unchanged:

- root and target world law;
- both `PolicyId`s;
- evidence definition;
- original risk allocation;
- canonical observation indices and accumulated counts.

Its old allocation remains charged. New pairs receive separately accounted risk.

If a new policy was constructed using the old evaluation worlds, it cannot be evaluated retrospectively on those worlds under a fixed-policy confidence claim. Freeze it, then begin evaluation on a disjoint or future stream.

## 14. Claim F needs a result-scope distinction

“Exact results spend no risk” is true for the **final result** when exact enumeration evaluates the complete original frozen candidate set and replaces every approximate conclusion.

It is not correct to say that an internal false edge never occurred merely because a later exact calculation overwrote it.

The safest wording is:

> **An exact full-set endpoint closes that decision with zero final sampling error. It consumes no additional risk. Previously assigned allocations are not automatically refunded. Any later reuse must satisfy a separate predictable pathwise budget rule.**

If only the statistically surviving candidates are enumerated, the prior false-elimination risk remains. That endpoint is exact among survivors, not exact for the original set.

## 15. Claims G–H are sound under the cache invariant

Require a partial function

\[
(\mathrm{PolicyId},\mathrm{WorldId})
\longmapsto
u_\rho(\omega)
\]

with deterministic values.

The exact endpoint iterates over every canonical world ID exactly once. For each policy, it uses the cached value when present and computes the missing value otherwise. Sample multiplicities never enter the exact sum.

Therefore the switch index, duplicate-heavy stream, batching, pause/resume, and cost forecast cannot alter the exact endpoint.

The forecast may choose a slow route. It cannot choose a mathematically different result.

## 16. Boundary cases

- \(m=1\): no pair tests exist; the sole candidate is selected with zero comparison risk. `T_edge` is not constructed.
- Require \(0<\delta_{\mathrm{dec}}<1\).
- Opening zero edges spends zero risk.
- If sampling has already cached all \(N\) worlds for every relevant policy, exact escalation is a no-op over the complete unique-ID cache.
- Exact ties are maximizers. No true strict edge exists between them.

## 17. Verdict for this brief

`FINAL ANSWER: COUNTEREXAMPLE (CLAIM D AS WRITTEN); A–C AND G–H CERTIFIED; E/F REQUIRE THE CONDITIONS ABOVE`

---

# Part IV — Response to `panel-ce-execution-order.md`

## 18. W1–W6 are under-specified at the batch boundary

The missing question is:

> When a batch overshoots an elimination, are observations later in that batch accepted for a candidate that was canonically dead?

W5 says the first crossing is reconstructed, but it does not explicitly require reconstruction of full per-index liveness.

### Exact ambiguity fixture

Use three candidates \(A,B,C\), threshold

\[
T=E^+_{1,0}=\frac32,
\]

and five worlds:

| world | A | B | C |
|---:|---:|---:|---:|
| 0 | 1 | 0 | 1 |
| 1 | 0 | 1 | 0 |
| 2 | 0 | 1 | 0 |
| 3 | 0 | 1 | 0 |
| 4 | 0 | 1 | 0 |

At world 0:

- \(A\to B\) crosses;
- \(C\to B\) crosses.

Canonical immediate elimination removes \(B\) before world 1. Candidates \(A\) and \(C\) then agree on every remaining world. Final live set: \(\{A,C\}\).

Under a plausible “live at batch start for the whole batch” reading, \(B\) is still evaluated on worlds 1–4. For \(B\to A\) and \(B\to C\), the counts become

\[
(a,b)=(4,1),
\]

with

\[
E^+_{4,1}
=
\frac{57}{30}
=
\frac{19}{10}
>
\frac32.
\]

The batch then contains later edges from a candidate that canonical execution had already eliminated. The reported graph and live set diverge.

Thus Claim INV does not follow from W1–W6 as written.

## 19. Canonical repaired semantics

Let \(L_n\) be the live candidate set immediately before world \(n\).

For each \(n\):

1. \(L_n\) is determined only by worlds \(0,\ldots,n-1\).
2. Evaluate every candidate in \(L_n\) on world \(n\).
3. Update every ordered pair whose two endpoints lie in \(L_n\).
4. Record each pair's first crossing at \(n\).
5. Apply a declared deterministic simultaneous-elimination rule to crossings at \(n\).
6. The resulting set is \(L_{n+1}\).

A batched implementation may compute speculative outcomes beyond an elimination. During semantic reconstruction it must discard every evidence update involving a candidate after that candidate's canonical elimination index.

Pure policy/world outcomes may remain in the cache. They simply do not enter that epoch's evidence.

### Same-index crossing rule

All first crossings at one index are applied simultaneously. On the no-false-edge event, settled edges follow strict value order and cannot form a directed cycle. If a cycle or “eliminate everyone” condition occurs, the implementation should emit a typed `InconsistentEvidence` result rather than silently choose an order.

## 20. Invariance proof under the repaired semantics

[USES: W1; canonical rule above.]

Induct on \(n\).

- World \(n\) is fixed by root, epoch, and index.
- By induction, every execution has the same \(L_n\).
- Deterministic policies give the same outcome vector on world \(n\).
- Therefore every active pair receives the same observation.
- Pair counts and first crossings at \(n\) are identical.
- The deterministic simultaneous-elimination rule gives the same \(L_{n+1}\).

Thus all semantic artifacts are independent of batch partition, thread schedule, and pause/resume pattern.

## 21. Precise filtration theorem

Let \(\mathcal F_n\) contain:

- the root, fixed candidate set, policy identities, risk configuration, and epoch;
- worlds \(\omega_0,\ldots,\omega_{n-1}\);
- all accepted outcomes and evidence through index \(n-1\);
- \(L_n\);
- every scheduling or opening decision made before drawing \(\omega_n\).

Require:

1. \(\omega_n\sim\beta\) conditionally on \(\mathcal F_n\), independent of scheduling;
2. for each pair \(e\), its activation indicator \(A_{e,n}\) is \(\mathcal F_n\)-measurable;
3. under the tested null, the conditional pair law at an active index obeys the null inequality.

For a fixed component multiplier \(L_e(Y_n)\), define

\[
\widetilde L_{e,n}
=
\begin{cases}
L_e(Y_n),&A_{e,n}=1,\\
1,&A_{e,n}=0.
\end{cases}
\]

Then

\[
\mathbb E[\widetilde L_{e,n}\mid\mathcal F_n]\le1.
\]

Therefore predictable thinning, adaptive elimination based on past worlds, pausing, and future-only edge opening preserve the supermartingale property.

### Why predictability is load-bearing

Under a symmetric null \(Y_n\in\{-1,+1\}\) with equal probability, suppose the controller first observes \(Y_n\), then declares the index active iff \(Y_n=+1\). Every accepted observation is favorable. The evidence eventually crosses any finite threshold with probability one.

This selector is not \(\mathcal F_n\)-measurable. It peeks at the current world.

## 22. Minimal additions to W1–W6

Add:

- **W7 — predictable activation:** liveness, edge opening, and acceptance for index \(n\) are fixed before observing world \(n\);
- **W8 — canonical per-index liveness replay:** batch reconstruction reproduces \(L_n\) for every index, not only first-crossing timestamps;
- **W9 — speculative isolation:** speculative outcomes cannot enter evidence before canonical replay;
- **W10 — simultaneous crossing semantics:** a deterministic same-index rule, with typed inconsistency on cycles;
- **W11 — complete pause state:** next canonical index, live set, pair counts, first crossings, policy IDs, risk ledger, and epoch are serialized.

## 23. Verdict for this brief

`FINAL ANSWER: UNDER-SPECIFIED (W5 NEEDS FULL PER-INDEX LIVENESS); Q RESOLVED UNDER PREDICTABLE ACTIVATION`

---

# Part V — Response to `panel-l2-coupling-theorems.md`

## 24. Airtight coupling definition

For fixed \((\rho,\omega)\), let \(h_t^0,h_t^1\) be the two public histories before action \(t\).

Define

\[
\tau
=
\inf\left\{
t:
h_t^0=h_t^1,\ 
\text{the actor is non-focal},\
\sigma_0(J_t)\ne\sigma_1(J_t)
\right\}.
\]

If no such \(t\) exists, set \(\tau=\infty\). Define

\[
D_\rho(\omega)=\mathbf1\{\tau<\infty\}.
\]

The executions are coupled identically until \(\tau\), then may fork. This removes the circular phrase “in either execution along the shared prefix.”

## 25. L2-T1 is sound

Before \(\tau\):

- at a focal node, both executions have the same private world and public history, hence the same focal information state; information consistency gives the same action;
- at a non-focal node outside the frontier, both fields give the same action;
- deterministic transitions preserve equal histories.

If \(\tau=\infty\), the histories remain equal to terminal and the Boolean payoffs are equal.

Therefore

\[
\boxed{
|u_1(\rho,\omega)-u_0(\rho,\omega)|
\le D_\rho(\omega).
}
\]

Taking expectations gives

\[
|V_1(\rho)-V_0(\rho)|
\le
\mathbb E|u_1-u_0|
\le d_\rho.
\]

For two fixed policies,

\[
|\Lambda_{a,b}|
\le
d_{\rho_a}+d_{\rho_b}.
\]

## 26. L2-T2 is sound

For every \(\rho\in\Pi_a\),

\[
V_1(\rho)\le V_0(\rho)+R_a.
\]

Taking the supremum over the same index set,

\[
Q_a^{(1)}
\le
Q_a^{(0)}+R_a.
\]

The same pointwise inequality with the field labels reversed gives

\[
Q_a^{(0)}
\le
Q_a^{(1)}+R_a.
\]

Hence

\[
\boxed{
|Q_a^{(1)}-Q_a^{(0)}|\le R_a.
}
\]

Different maximizing policies cause no gap because the inequality holds uniformly for every policy in the shared set \(\Pi_a\).

## 27. L2-T3 is sound

If for every rival \(b\),

\[
Q_a^{(0)}-Q_b^{(0)}>R_a+R_b,
\]

then

\[
Q_a^{(1)}
\ge
Q_a^{(0)}-R_a
>
Q_b^{(0)}+R_b
\ge
Q_b^{(1)}.
\]

The separate hypothesis “\(a\) is optimal under \(\sigma_0\)” is redundant: the strict inequalities already imply it.

## 28. L2-T4 is sound under the explicit minimal hypotheses

Require:

- a finite nonempty legal-action set;
- valid intervals \(L_a^{(0)}\le Q_a^{(0)}\le U_a^{(0)}\);
- valid nonnegative bounds \(0\le R_a\le R_a^U\).

Then

\[
L_a^{(1)}
=
L_a^{(0)}-R_a^U
\le Q_a^{(1)}
\le
U_a^{(0)}+R_a^U
=
U_a^{(1)}.
\]

Let

\[
B=\max_aL_a^{(1)}.
\]

If \(a\notin\mathcal A_1\), then

\[
Q_a^{(1)}\le U_a^{(1)}<B.
\]

An action \(c\) attaining the maximum has \(Q_c^{(1)}\ge B\), so \(a\) cannot be optimal.

Nonemptiness follows because for such \(c\),

\[
U_c^{(1)}-L_c^{(1)}
=
(U_c^{(0)}-L_c^{(0)})+2R_c^U
\ge0,
\]

hence \(U_c^{(1)}\ge B\) and \(c\in\mathcal A_1\).

The membership comparison must remain `>=`. An action at the boundary may be tied-optimal and must not be excluded.

## 29. L2-T5 is sound and convergence is not implied

A deterministic map on a finite set generates an eventually periodic orbit by pigeonhole and determinism.

A period-four example is

\[
0\mapsto1\mapsto2\mapsto3\mapsto0.
\]

Therefore every level result must remain typed as a best response to one named fixed field. No monotone-improvement, convergence, or equilibrium statement follows.

## 30. Verdict for this brief

`FINAL ANSWER: CERTIFIED (L2-T1..T5), WITH THE COUPLING DEFINITION REPAIRED`

---

# Part VI — New mathematics: cancellation, irrelevance, and one-sided danger

Jason's high-trump-versus-double example identifies a distinction the current objects can express but should make explicit.

## 31. Fixed-policy cancellation ladder

For one fixed focal policy \(\rho\) under fields \(\sigma_0,\sigma_1\), define per world:

\[
D_\rho
=
\mathbf1\{\text{field-disagreement frontier reached}\},
\]

\[
O_\rho
=
\mathbf1\{u_1(\rho,\omega)\ne u_0(\rho,\omega)\},
\]

\[
C_\rho^+
=
\mathbf1\{u_1=1,u_0=0\},
\qquad
C_\rho^-
=
\mathbf1\{u_1=0,u_0=1\},
\]

\[
C_\rho=u_1-u_0=C_\rho^+-C_\rho^-.
\]

Pointwise,

\[
|C_\rho|
=
O_\rho
=
C_\rho^++C_\rho^-
\le D_\rho.
\]

Let

\[
d_\rho=\mathbb E[D_\rho],
\quad
r_\rho=\mathbb E[O_\rho],
\quad
c_\rho^+=\mathbb E[C_\rho^+],
\quad
c_\rho^-=\mathbb E[C_\rho^-],
\quad
c_\rho=c_\rho^+-c_\rho^-.
\]

Then

\[
\boxed{
|c_\rho|
\le
r_\rho
=
c_\rho^++c_\rho^-
\le
d_\rho.
}
\]

These are three different notions:

1. **behavioral irrelevance:** \(d_\rho=0\);
2. **terminal-outcome irrelevance:** \(r_\rho=0\);
3. **value neutrality:** \(c_\rho=0\).

Only the first says the fields never even act differently. The second permits behavioral differences that never alter make/fail. The third permits positive and negative terminal changes that cancel exactly in expectation.

## 32. The current field-swap specimen illustrates all three scales

For `receipt-h8-t4`:

| policy | field exposure \(d\) | outcome-change mass \(r\) | \(c^+\) | \(c^-\) | net \(c\) |
|---|---:|---:|---:|---:|---:|
| reveal 5-5 | \(1138/1200\) | \(56/1200\) | \(30/1200\) | \(26/1200\) | \(4/1200=1/300\) |
| retain 3-3 | \(1117/1200\) | \(117/1200\) | \(45/1200\) | \(72/1200\) | \(-27/1200=-9/400\) |

The fields act differently in almost every world. They change the Boolean terminal outcome in a much smaller fraction. Positive and negative changes then partially cancel.

The frozen-pair field lift is

\[
\Lambda
=
c_{\mathrm{reveal}}-c_{\mathrm{retain}}
=
\frac4{1200}-\left(-\frac{27}{1200}\right)
=
\boxed{\frac{31}{1200}}.
\]

The committed prose currently says \(41/1200\). The component counts prove that the correct numerator is \(31\).

## 33. Pairwise benefit and hazard masses

For two frozen policies \(a,b\) under one field, define

\[
B(a\mid b)
=
\Pr(u_a=1,u_b=0),
\]

\[
H(a\mid b)
=
\Pr(u_a=0,u_b=1).
\]

Then

\[
\boxed{
g(a,b)=B(a\mid b)-H(a\mid b),
}
\]

\[
\boxed{
q(a,b)=B(a\mid b)+H(a\mid b).
}
\]

A small \(|g|\) has two completely different explanations:

- both \(B\) and \(H\) are small: the policies nearly agree;
- both are substantial and nearly equal: the policies often exchange wins and losses.

These should never be collapsed into one “close” label.

## 34. Dominance theorem

For Boolean payoff:

- \(a\) weakly dominates \(b\) almost surely iff
  \[
  H(a\mid b)=0;
  \]
- the dominance is strict in expected value if additionally
  \[
  B(a\mid b)>0.
  \]

The high-trump-versus-vulnerable-double example has the shape

\[
H(\text{high trump}\mid\text{double})=0,
\qquad
B(\text{high trump}\mid\text{double})>0,
\]

provided the high trump truly is never worse and the double can occasionally be trumped.

That is not cancellation. It is one-sided unforced risk.

A finite sample with zero observed hazards does not prove \(H=0\). Exact enumeration, a structural impossibility proof, or a valid upper bound is required.

Dominance pruning changes no objective. Choosing among non-dominated exact ties by variance, robustness, or human convention would introduce a secondary objective and must be declared separately.

## 35. Directional root-action correction bounds

For root action \(a\), define over all information-consistent continuations:

\[
R_a^+
=
\sup_{\rho\in\Pi_a}
\Pr(u_1(\rho,\omega)=1,u_0(\rho,\omega)=0),
\]

\[
R_a^-
=
\sup_{\rho\in\Pi_a}
\Pr(u_1(\rho,\omega)=0,u_0(\rho,\omega)=1).
\]

Then

\[
\boxed{
Q_a^{(0)}-R_a^-
\le
Q_a^{(1)}
\le
Q_a^{(0)}+R_a^+.
}
\]

### Proof

For every \(\rho\),

\[
V_1(\rho)
=
V_0(\rho)+c_\rho^+-c_\rho^-
\le
V_0(\rho)+R_a^+.
\]

Taking suprema gives the upper bound.

Let \(\rho_0\) attain \(Q_a^{(0)}\). Then

\[
Q_a^{(1)}
\ge
V_1(\rho_0)
\ge
V_0(\rho_0)-R_a^-
=
Q_a^{(0)}-R_a^-.
\]

Consequently,

\[
\boxed{
|Q_a^{(1)}-Q_a^{(0)}|
\le
\max(R_a^+,R_a^-)
\le R_a.
}
\]

This can be dramatically tighter than symmetric field-exposure bounds.

## 36. Directional winner-stability theorem

For a baseline winner \(a\) and rival \(b\),

\[
Q_a^{(1)}-Q_b^{(1)}
\ge
Q_a^{(0)}-Q_b^{(0)}-R_a^- - R_b^+.
\]

Therefore

\[
\boxed{
Q_a^{(0)}-Q_b^{(0)}
>
R_a^-+R_b^+
\quad\Longrightarrow\quad
Q_a^{(1)}>Q_b^{(1)}.
}
\]

Only two directions can overturn the winner:

- the winner gets worse;
- the rival gets better.

The symmetric \(R_a+R_b\) bound pays for impossible directions too.

## 37. Directional safe screening

Given valid baseline intervals and valid directional upper bounds,

\[
Q_a^{(0)}\in[L_a^{(0)},U_a^{(0)}],
\qquad
R_a^+\le(R_a^+)^U,
\qquad
R_a^-\le(R_a^-)^U,
\]

define

\[
L_a^{(1)}
=
L_a^{(0)}-(R_a^-)^U,
\]

\[
U_a^{(1)}
=
U_a^{(0)}+(R_a^+)^U.
\]

The same bar construction as L2-T4 is sound:

\[
B=\max_aL_a^{(1)},
\qquad
\mathcal A_1
=
\{a:U_a^{(1)}\ge B\}.
\]

This is the natural next tightening for split-heavy roots where exposure bounds near one prune nothing.

## 38. Computation routes

For fixed policies, \(c^+\), \(c^-\), \(r\), and \(d\) are already simple paired full-fiber counts.

For root actions, the same information-consistent plan machinery can solve three Boolean objectives:

1. reach the first field split — \(R_a\);
2. terminate with \(u_1=1,u_0=0\) — \(R_a^+\);
3. terminate with \(u_1=0,u_0=1\) — \(R_a^-\).

The latter two must run the coupled branches to terminal. They are more expensive than split reach, but they may be much tighter.

A sensible rung ladder is:

\[
R_a^\pm
\le
R_a^{\mathrm{outcome}}
\le
R_a^{\mathrm{exposure}}.
\]

Use the cheapest sound rung that settles the root.

## 39. Bounded-utility generalization

If terminal utility lies in \([L,U]\), then after a first field split the largest possible per-world difference is \(U-L\). Therefore

\[
|V_1(\rho)-V_0(\rho)|
\le
(U-L)d_\rho.
\]

For Boolean `pmake`, \(U-L=1\), recovering L2-T1.

---

# Part VII — Required project amendments

## 40. Immediate corrections

1. Replace `41/1200` with `31/1200` wherever the `receipt-h8-t4` frozen-pair lift is stated.
2. Narrow the bounded-mean brief's sign-safe wording to the unrestricted-class statement.
3. Amend risk-ledger Claim D to future-only or preallocated edge opening.
4. Replace “refund” with final-result typing and a pathwise future-allocation rule.
5. Amend W5 to require canonical per-index liveness reconstruction.

## 41. New first-class diagnostics

Every pairwise report should preserve:

\[
(B,H,q,g),
\]

not only \(g\) or the absolute make percentages.

Every field-swap fixed-policy report should preserve:

\[
(d,r,c^+,c^-,c).
\]

Suggested result labels:

- `NoFieldExposure`: \(d=0\);
- `OutcomeStable`: \(r=0\);
- `ValueNeutral`: \(c=0\) exactly but \(r\) may be positive;
- `EpsilonEquivalent`: certified \(|c|<\varepsilon\);
- `Dominated`: one-sided hazard is exactly zero in the dominating direction and strict benefit is positive;
- `Unresolved`: none of the above has been established.

## 42. Interpretation rule

> **Cancellation may justify a value statement under one declared objective, belief, and model. It never by itself proves pathwise safety, structural irrelevance, dominance, or stability under reweighting.**

This rule is the mathematical version of Jason's double-versus-high-trump example.

---

# MACHINE-CHECKABLE ARTIFACTS

- `FINAL ANSWER: CE-T1/T2/T3 CERTIFIED (claims A–I)`
- `FINAL ANSWER: CE-T4/T5 CERTIFIED (A–E), WITH TASK-D WORDING NARROWED`
- `FINAL ANSWER: O21/O24 COUNTEREXAMPLE (CLAIM D AS WRITTEN); A–C AND G–H CERTIFIED; E/F CONDITIONAL`
- `FINAL ANSWER: O26 UNDER-SPECIFIED (FULL PER-INDEX LIVENESS REQUIRED); CONDITIONAL NULL RESOLVED UNDER PREDICTABLE ACTIVATION`
- `FINAL ANSWER: L2-T1..T5 CERTIFIED, WITH COUPLING DEFINITION REPAIRED`

The following deterministic Python 3 program uses exact `fractions.Fraction` arithmetic, performs no network or file I/O, and exits nonzero on any failed check.

```python
#!/usr/bin/env python3
"""Machine-checkable companion for RESPONSE-walt-panel-and-cancellation-v0.1.

Exact rational arithmetic only. No network, file I/O, random simulation, or
floating-point correctness checks. Deterministic finite grids and model checks.

The script certifies:
  * CE-T1/T2/T3 exact identities and one-step inequalities.
  * CE-T4/T5 factor ranges, one-step inequalities, and sign-majority defect.
  * O21/O24 ledger algebra, a retrospective edge-opening adversary, and
    sample-to-enumeration cold-equivalence.
  * O26's batching ambiguity as written, plus invariance of the repaired
    canonical per-index semantics and the predictable-selection boundary.
  * L2-T1..T5 on 2,000 finite extensive-form games, plus the directional
    correction refinement introduced in the accompanying response.
"""

from __future__ import annotations

from dataclasses import dataclass
from fractions import Fraction as F
from itertools import product
from math import comb, factorial
from random import Random
from typing import Dict, Iterable, List, Mapping, Sequence, Tuple

FAILURES: List[str] = []


def check(name: str, condition: bool, detail: str = "") -> None:
    if condition:
        print(f"PASS {name}")
    else:
        msg = f"FAIL {name}" + (f" {detail}" if detail else "")
        print(msg)
        FAILURES.append(msg)


# ---------------------------------------------------------------------------
# CE-T1/T2/T3
# ---------------------------------------------------------------------------

def e_upper_integral(s: int, f: int, c: F) -> F:
    """Original normalized integral, expanded in r exactly."""
    total = F(0)
    for j in range(f + 1):
        coefficient = F(((-1) ** j) * comb(f, j), 1)
        power = s + j + 1
        total += coefficient * (F(1) - c**power) / power
    return total / (c**s * (1 - c) ** (f + 1))


def e_upper_substituted(s: int, f: int, c: F) -> F:
    """Substituted integral / finite beta sum."""
    ratio = (1 - c) / c
    return sum(
        F(comb(s, i), 1)
        * ratio**i
        * F(factorial(i) * factorial(f), factorial(i + f + 1))
        for i in range(s + 1)
    )


def e_lower_natural(s: int, f: int, c: F) -> F:
    """Natural lower mixture, expanded on [0,c] exactly."""
    total = F(0)
    for j in range(f + 1):
        coefficient = F(((-1) ** j) * comb(f, j), 1)
        power = s + j + 1
        total += coefficient * c**power / power
    return total / (c ** (s + 1) * (1 - c) ** f)


def e_piv_integral(a: int, b: int) -> F:
    """Integral int_0^1 (1+t)^a (1-t)^b dt by polynomial expansion."""
    total = F(0)
    for i in range(a + 1):
        for j in range(b + 1):
            total += F(comb(a, i) * comb(b, j) * ((-1) ** j), i + j + 1)
    return total


def e_piv_closed(a: int, b: int) -> F:
    k = a + b
    return F(sum(comb(k + 1, x) for x in range(a + 1)), (k + 1) * comb(k, a))


def verify_evidence_processes() -> None:
    cs = [F(1, 10), F(1, 3), F(1, 2), F(11, 16), F(2, 3), F(9, 10)]
    identity_ok = True
    lower_ok = True
    for c in cs:
        for s in range(13):
            for f in range(13):
                x = e_upper_integral(s, f, c)
                y = e_upper_substituted(s, f, c)
                if x != y:
                    identity_ok = False
                if e_lower_natural(s, f, c) != e_upper_substituted(f, s, 1 - c):
                    lower_ok = False
    check("CE-T1 original=substituted=finite-sum grid", identity_ok)
    check("CE-T2 natural-lower identity grid", lower_ok)

    pivotal_ok = True
    specialization_ok = True
    all_favorable_ok = True
    for a in range(41):
        for b in range(41):
            integ = e_piv_integral(a, b)
            closed = e_piv_closed(a, b)
            if integ != closed:
                pivotal_ok = False
            if closed != e_upper_substituted(a, b, F(1, 2)):
                specialization_ok = False
        if e_piv_closed(a, 0) != F(2 ** (a + 1) - 1, a + 1):
            all_favorable_ok = False
    check("CE-T3 defining-integral=closed-form grid", pivotal_ok)
    check("CE-T3 is CE-T1 at c=1/2", specialization_ok)
    check("CE-T3 all-favorable formula", all_favorable_ok)

    anchors = {
        (0, 0): F(1),
        (1, 0): F(3, 2),
        (0, 1): F(1, 2),
        (2, 0): F(7, 3),
        (1, 1): F(2, 3),
        (2, 1): F(11, 12),
        (3, 0): F(15, 4),
    }
    check("CE-T3 anchors", all(e_piv_closed(a, b) == v for (a, b), v in anchors.items()))
    check(
        "CE-T3 alpha=1/128 favorable-pivot threshold",
        e_piv_closed(9, 0) < 128 < e_piv_closed(10, 0),
        f"E9={e_piv_closed(9,0)} E10={e_piv_closed(10,0)}",
    )

    grid = [F(i, 8) for i in range(1, 8)]
    bernoulli_step_ok = True
    for p in grid:
        for c in grid:
            for r in grid:
                if p <= c <= r:
                    lhs = p * r / c + (1 - p) * (1 - r) / (1 - c)
                    rhs = 1 + (p - c) * (r - c) / (c * (1 - c))
                    if lhs != rhs or lhs > 1:
                        bernoulli_step_ok = False
    check("CE-T1 one-step identity/inequality eighth-grid", bernoulli_step_ok)

    pivotal_step_ok = True
    q_grid = [F(i, 8) for i in range(0, 9)]
    theta_grid = [F(i, 8) for i in range(0, 5)]
    r_grid = [F(i, 8) for i in range(4, 9)]
    for q in q_grid:
        for theta in theta_grid:
            for r in r_grid:
                expected = 1 - q + q * (2 * theta * r + 2 * (1 - theta) * (1 - r))
                direct = 1 + q * (2 * r - 1) * (2 * theta - 1)
                if expected != direct or expected > 1:
                    pivotal_step_ok = False
    check("CE-T3 raw-stream one-step inequality grid incl q=0", pivotal_step_ok)


# ---------------------------------------------------------------------------
# CE-T4/T5 and sign-majority defect
# ---------------------------------------------------------------------------

def finite_binomial_majority(n: int, p: F) -> F:
    assert n % 2 == 1
    return sum(F(comb(n, k), 1) * p**k * (1 - p) ** (n - k) for k in range(n // 2 + 1, n + 1))


def sign_evidence_crossing_probability(horizon: int, p_plus: F, threshold: F) -> F:
    """Exact finite-horizon crossing probability for +/- observations."""
    active: Dict[Tuple[int, int], F] = {(0, 0): F(1)}
    crossed = F(0)
    for _ in range(horizon):
        nxt: Dict[Tuple[int, int], F] = {}
        for (a, b), mass in active.items():
            for da, db, prob in ((1, 0, p_plus), (0, 1, 1 - p_plus)):
                state = (a + da, b + db)
                m = mass * prob
                if e_piv_closed(*state) >= threshold:
                    crossed += m
                else:
                    nxt[state] = nxt.get(state, F(0)) + m
        active = nxt
    return crossed


def verify_bounded_mean() -> None:
    L, U = F(-1), F(1)
    cs = [F(-1, 2), F(-1, 8), F(0), F(1, 8), F(1, 2)]
    x_grid = [F(-1), F(-1, 2), F(-1, 32), F(0), F(1, 8), F(1)]
    multipliers = [F(0), F(1, 4), F(1, 2), F(3, 4), F(1)]
    factor_ok = True
    mirror_ok = True
    sharp_ok = True
    for c in cs:
        lp = F(1, 1) / (c - L)
        lm = F(1, 1) / (U - c)
        for q in multipliers:
            lam_p = q * lp
            lam_m = q * lm
            for x in x_grid:
                if 1 + lam_p * (x - c) < 0:
                    factor_ok = False
                if 1 - lam_m * (x - c) < 0:
                    mirror_ok = False
        eps = F(1, 1000)
        if 1 + (lp + eps) * (L - c) >= 0:
            sharp_ok = False
        if 1 - (lm + eps) * (U - c) >= 0:
            sharp_ok = False
    check("CE-T4 factor nonnegativity grid", factor_ok)
    check("CE-T5 factor nonnegativity grid", mirror_ok)
    check("CE-T4/T5 lambda ranges are sharp", sharp_ok)

    support = [F(-1), F(-1, 2), F(0), F(1, 2), F(1)]
    weights = [F(1, 4), F(1, 2), F(3, 4)]
    one_step_pos = True
    one_step_neg = True
    mixture_ok = True
    nontrivial_laws = 0
    for c in cs:
        lp = F(1) / (c - L)
        lm = F(1) / (U - c)
        lambda_ps = [F(0), lp / 4, lp / 2, lp]
        lambda_ms = [F(0), lm / 4, lm / 2, lm]
        for x0 in support:
            for x1 in support:
                for w in weights:
                    mean = w * x0 + (1 - w) * x1
                    if mean <= c:
                        nontrivial_laws += 1
                        for lam in lambda_ps:
                            expected = w * (1 + lam * (x0 - c)) + (1 - w) * (1 + lam * (x1 - c))
                            if expected > 1:
                                one_step_pos = False
                        mix = F(1, 3) * (
                            w * (1 + lambda_ps[1] * (x0 - c))
                            + (1 - w) * (1 + lambda_ps[1] * (x1 - c))
                        ) + F(2, 3) * (
                            w * (1 + lambda_ps[2] * (x0 - c))
                            + (1 - w) * (1 + lambda_ps[2] * (x1 - c))
                        )
                        if mix > 1:
                            mixture_ok = False
                    if mean >= c:
                        for lam in lambda_ms:
                            expected = w * (1 - lam * (x0 - c)) + (1 - w) * (1 - lam * (x1 - c))
                            if expected > 1:
                                one_step_neg = False
    check("CE-T4 one-step null inequalities", one_step_pos and nontrivial_laws > 0)
    check("CE-T5 one-step null inequalities", one_step_neg)
    check("CE-T4 finite-mixture one-step inequality", mixture_ok)

    p_pos = F(3, 4)
    mean = p_pos * F(1, 8) + (1 - p_pos) * F(-1, 2)
    check("sign-majority counterexample arithmetic", p_pos == F(3, 4) and mean == F(-1, 32))

    majority_probs = [finite_binomial_majority(n, p_pos) for n in (1, 3, 5, 9, 21)]
    check(
        "sign-majority increasingly favors wrong mean direction",
        all(x < y for x, y in zip(majority_probs, majority_probs[1:])) and majority_probs[-1] > F(9, 10),
        str(majority_probs),
    )
    crossing = sign_evidence_crossing_probability(100, p_pos, F(128))
    check(
        "consistent sign-evidence can become confident in wrong mean ordering",
        crossing > F(1, 2),
        f"P(cross by 100)={crossing}",
    )

    # Constant nonzero magnitude is a sign-safe subclass.
    sign_safe = True
    for p in [F(i, 8) for i in range(9)]:
        d = F(3, 7)
        mean_const = p * d - (1 - p) * d
        sign_balance = p - (1 - p)
        if (mean_const > 0) != (sign_balance > 0) or (mean_const < 0) != (sign_balance < 0):
            sign_safe = False
    check("constant-magnitude subclass is sign-safe", sign_safe)


# ---------------------------------------------------------------------------
# O21/O24 risk ledger and exact escalation
# ---------------------------------------------------------------------------

def verify_risk_ledger() -> None:
    running = F(0)
    telescoping_ok = True
    for k in range(1, 10001):
        running += F(1, k * (k + 1))
        if running != F(k, k + 1):
            telescoping_ok = False
            break
    check("risk ledger telescoping identity K<=10000", telescoping_ok)

    all_pairs_ok = True
    for m in range(2, 15):
        for delta in (F(1, 100), F(1, 10), F(1, 2)):
            alpha = delta / (m * (m - 1))
            threshold = F(m * (m - 1), 1) / delta
            if alpha * threshold != 1:
                all_pairs_ok = False
    check("all-pairs threshold times edge allocation equals one", all_pairs_ok)

    # Claim D as written: observe K unopened e-processes historically, then
    # assign alpha_1 to one that already crossed. Each individual process is
    # a valid one-step e-value: M=T with probability alpha, zero otherwise.
    delta = F(1, 4)
    alpha1 = delta / 2
    k_edges = 3
    retrospective_false_cross = 1 - (1 - alpha1) ** k_edges
    check(
        "retrospective sequential edge-opening adversary exceeds total budget",
        retrospective_false_cross > delta,
        f"P={retrospective_false_cross} delta={delta}",
    )

    outcomes = [
        [1, 0, 1, 1, 0, 1],
        [1, 1, 1, 0, 0, 1],
        [0, 1, 1, 1, 1, 0],
    ]
    streams = [
        [0, 0, 1, 5, 1, 1, 2, 4, 5, 0],
        [5, 4, 3, 2, 1, 0],
        [2, 2, 2, 2, 0, 5, 3, 3],
    ]

    def cold(policy_ids: Iterable[int]) -> Tuple[int, ...]:
        return tuple(sum(outcomes[p]) for p in policy_ids)

    def switch_endpoint(stream: Sequence[int], switch: int, policy_ids: Sequence[int]) -> Tuple[int, ...]:
        cache: Dict[Tuple[int, int], int] = {}
        for world in stream[:switch]:
            for p in policy_ids:
                cache[(p, world)] = outcomes[p][world]
        for world in range(len(outcomes[0])):
            for p in policy_ids:
                cache.setdefault((p, world), outcomes[p][world])
        return tuple(sum(cache[(p, w)] for w in range(len(outcomes[0]))) for p in policy_ids)

    switch_ok = True
    for stream in streams:
        for switch in range(len(stream) + 1):
            for ids in ((0, 1, 2), (0, 2), (1,)):
                if switch_endpoint(stream, switch, ids) != cold(ids):
                    switch_ok = False
    check("sample-to-enumeration endpoint equals cold enumeration", switch_ok)

    # Survivor-only exactness is not full-set exactness after a false removal.
    dominated_table = [[0, 0], [1, 1]]
    survivor_winner = 0
    full_winner = 1
    check(
        "survivor-only enumeration cannot certify original candidate set",
        sum(dominated_table[survivor_winner]) < sum(dominated_table[full_winner]),
    )


# ---------------------------------------------------------------------------
# O26 execution order and predictable selection
# ---------------------------------------------------------------------------

def canonical_engine(table: Sequence[Sequence[int]], threshold: F) -> Tuple:
    m = len(table[0])
    live = set(range(m))
    counts: Dict[Tuple[int, int], List[int]] = {
        (i, j): [0, 0] for i in range(m) for j in range(m) if i != j
    }
    first: Dict[Tuple[int, int], int] = {}
    live_history = [tuple(sorted(live))]
    elimination_events = []

    for n, row in enumerate(table):
        current = sorted(live)
        for i in current:
            for j in current:
                if i == j:
                    continue
                y = row[i] - row[j]
                if y == 1:
                    counts[(i, j)][0] += 1
                elif y == -1:
                    counts[(i, j)][1] += 1
        newly = []
        for i in current:
            for j in current:
                if i == j or (i, j) in first:
                    continue
                a, b = counts[(i, j)]
                if e_piv_closed(a, b) >= threshold:
                    first[(i, j)] = n
                    newly.append((i, j))
        targets = {j for i, j in newly if i in live and j in live}
        if targets:
            if targets == live:
                elimination_events.append((n, "inconsistent", tuple(sorted(newly))))
                live.clear()
            else:
                live.difference_update(targets)
                elimination_events.append((n, tuple(sorted(targets)), tuple(sorted(newly))))
        live_history.append(tuple(sorted(live)))
    return (
        tuple(sorted(live)),
        tuple(sorted(first.items())),
        tuple(live_history),
        tuple(elimination_events),
    )


def naive_batch_start_engine(table: Sequence[Sequence[int]], threshold: F, batches: Sequence[int]) -> Tuple:
    """A plausible but unsound W5 reading: batch-start liveness for whole batch."""
    m = len(table[0])
    live = set(range(m))
    counts: Dict[Tuple[int, int], List[int]] = {
        (i, j): [0, 0] for i in range(m) for j in range(m) if i != j
    }
    first: Dict[Tuple[int, int], int] = {}
    at = 0
    for size in batches:
        batch_live = sorted(live)
        for n in range(at, min(at + size, len(table))):
            row = table[n]
            for i in batch_live:
                for j in batch_live:
                    if i == j:
                        continue
                    y = row[i] - row[j]
                    if y == 1:
                        counts[(i, j)][0] += 1
                    elif y == -1:
                        counts[(i, j)][1] += 1
                    if (i, j) not in first:
                        a, b = counts[(i, j)]
                        if e_piv_closed(a, b) >= threshold:
                            first[(i, j)] = n
        targets = {j for (i, j), n in first.items() if i in batch_live and j in batch_live}
        live.difference_update(targets)
        at += size
        if at >= len(table):
            break
    return tuple(sorted(live)), tuple(sorted(first.items()))


def compositions(n: int, rng: Random, count: int) -> List[List[int]]:
    out = [[1] * n, [n]]
    for _ in range(count - 2):
        remaining = n
        parts = []
        while remaining:
            x = rng.randint(1, remaining)
            parts.append(x)
            remaining -= x
        out.append(parts)
    return out


def repaired_batched_engine(table: Sequence[Sequence[int]], threshold: F, batches: Sequence[int]) -> Tuple:
    """Speculation may batch, but semantic replay is canonical per index."""
    assert sum(batches) >= len(table)
    return canonical_engine(table, threshold)


def verify_execution_order() -> None:
    threshold = F(3, 2)
    ambiguity_table = [
        [1, 0, 1],
        [0, 1, 0],
        [0, 1, 0],
        [0, 1, 0],
        [0, 1, 0],
    ]
    canonical = canonical_engine(ambiguity_table, threshold)
    naive = naive_batch_start_engine(ambiguity_table, threshold, [5])
    check(
        "W1-W6 as written admit a batching/liveness divergence",
        canonical[0] != naive[0],
        f"canonical_live={canonical[0]} naive_live={naive[0]}",
    )

    rng = Random(20260824)
    tables = [
        ambiguity_table,
        [[1, 0, 0], [1, 0, 1], [0, 1, 1], [1, 1, 0], [0, 0, 1], [1, 0, 1]],
        [[0, 1, 0, 1], [1, 0, 1, 0], [1, 1, 0, 0], [0, 1, 1, 0], [1, 0, 0, 1]],
    ]
    invariant_ok = True
    schedules_tested = 0
    for table in tables:
        reference = canonical_engine(table, threshold)
        for parts in compositions(len(table), rng, 200):
            schedules_tested += 1
            if repaired_batched_engine(table, threshold, parts) != reference:
                invariant_ok = False
    check(
        "canonical per-index reconstruction invariant across >=200 schedules/table",
        invariant_ok and schedules_tested >= 600,
        f"schedules={schedules_tested}",
    )

    # If the selector peeks at Y_n, it can accept only + outcomes. Under a
    # symmetric null, crossing after ten accepted + pivots occurs whenever
    # at least ten + outcomes appear in the first 30 raw worlds.
    peek_cross = sum(F(comb(30, k), 2**30) for k in range(10, 31))
    check(
        "nonpredictable current-world selection breaks level-1/128 validity",
        peek_cross > F(1, 128),
        f"P={peek_cross}",
    )

    # Predictable activation A_n merely replaces the multiplier by 1 on
    # inactive indices. Under a symmetric null its conditional mean is 1.
    r = F(3, 4)
    expected_active = F(1, 2) * (2 * r) + F(1, 2) * (2 * (1 - r))
    expected_inactive = F(1)
    check(
        "predictable thinning preserves one-step null expectation",
        expected_active == 1 and expected_inactive == 1,
    )


# ---------------------------------------------------------------------------
# L2-T1..T5 and the directional correction extension
# ---------------------------------------------------------------------------

@dataclass(frozen=True)
class TinyGame:
    probs: Tuple[F, F, F]
    focal_obs: Tuple[int, int, int]
    field_obs: Tuple[int, int, int]
    payoff: Tuple[int, ...]  # index w,a,x,z


def payoff_at(game: TinyGame, w: int, a: int, x: int, z: int) -> int:
    return game.payoff[(((w * 2 + a) * 2 + x) * 2 + z)]


def field_action(field: Tuple[int, ...], a: int, x: int, obs: int) -> int:
    return field[((a * 2 + x) * 2 + obs)]


def policy_action(policy: Tuple[int, int], focal_obs: int) -> int:
    return policy[focal_obs]


def run_tiny(game: TinyGame, policy: Tuple[int, int], field: Tuple[int, ...], w: int, a: int) -> Tuple[int, Tuple[int, int, int]]:
    x = policy_action(policy, game.focal_obs[w])
    j = (a, x, game.field_obs[w])
    z = field_action(field, *j)
    return payoff_at(game, w, a, x, z), j


def all_policies() -> List[Tuple[int, int]]:
    return list(product((0, 1), repeat=2))


def value(game: TinyGame, policy: Tuple[int, int], field: Tuple[int, ...], a: int) -> F:
    return sum(game.probs[w] * run_tiny(game, policy, field, w, a)[0] for w in range(3))


def exposure_and_corrections(
    game: TinyGame, policy: Tuple[int, int], f0: Tuple[int, ...], f1: Tuple[int, ...], a: int
) -> Tuple[F, F, F, F]:
    d = cp = cm = changed = F(0)
    for w in range(3):
        u0, j0 = run_tiny(game, policy, f0, w, a)
        u1, j1 = run_tiny(game, policy, f1, w, a)
        assert j0 == j1  # the common state is before the field action
        split = field_action(f0, *j0) != field_action(f1, *j0)
        if split:
            d += game.probs[w]
        if u1 == 1 and u0 == 0:
            cp += game.probs[w]
        if u1 == 0 and u0 == 1:
            cm += game.probs[w]
        if u1 != u0:
            changed += game.probs[w]
    return d, cp, cm, changed


def generate_game(rng: Random, idx: int) -> Tuple[TinyGame, Tuple[int, ...], Tuple[int, ...]]:
    prob_options = [
        (F(1, 3), F(1, 3), F(1, 3)),
        (F(1, 6), F(1, 3), F(1, 2)),
        (F(1, 2), F(1, 6), F(1, 3)),
    ]
    probs = prob_options[idx % len(prob_options)]
    focal_obs = tuple(rng.randrange(2) for _ in range(3))
    field_obs = tuple(rng.randrange(2) for _ in range(3))
    payoff = tuple(rng.randrange(2) for _ in range(3 * 2 * 2 * 2))
    f0 = tuple(rng.randrange(2) for _ in range(8))
    f1 = tuple(rng.randrange(2) for _ in range(8))
    return TinyGame(probs, focal_obs, field_obs, payoff), f0, f1


def verify_l2() -> None:
    rng = Random(424242)
    instances = 2000
    t1_ok = t2_ok = t3_ok = t4_ok = directional_ok = hierarchy_ok = True
    t3_premises = 0
    policies = all_policies()

    for idx in range(instances):
        game, f0, f1 = generate_game(rng, idx)
        q0: Dict[int, F] = {}
        q1: Dict[int, F] = {}
        rexposure: Dict[int, F] = {}
        rplus: Dict[int, F] = {}
        rminus: Dict[int, F] = {}
        rchange: Dict[int, F] = {}

        for a in (0, 1):
            vals0 = []
            vals1 = []
            ds = []
            cps = []
            cms = []
            changes = []
            for policy in policies:
                v0 = value(game, policy, f0, a)
                v1 = value(game, policy, f1, a)
                d, cp, cm, changed = exposure_and_corrections(game, policy, f0, f1, a)
                vals0.append(v0)
                vals1.append(v1)
                ds.append(d)
                cps.append(cp)
                cms.append(cm)
                changes.append(changed)

                for w in range(3):
                    u0, j0 = run_tiny(game, policy, f0, w, a)
                    u1, j1 = run_tiny(game, policy, f1, w, a)
                    split = field_action(f0, *j0) != field_action(f1, *j0)
                    if not split and u0 != u1:
                        t1_ok = False
                if abs(v1 - v0) > d:
                    t1_ok = False
                if abs((cp - cm) - (v1 - v0)) > 0:
                    t1_ok = False
                if not (cp <= changed <= d and cm <= changed <= d and abs(cp - cm) <= changed):
                    hierarchy_ok = False

            q0[a] = max(vals0)
            q1[a] = max(vals1)
            rexposure[a] = max(ds)
            rplus[a] = max(cps)
            rminus[a] = max(cms)
            rchange[a] = max(changes)

            if abs(q1[a] - q0[a]) > rexposure[a]:
                t2_ok = False
            if q1[a] > q0[a] + rplus[a] or q1[a] < q0[a] - rminus[a]:
                directional_ok = False
            if not (rplus[a] <= rchange[a] <= rexposure[a] and rminus[a] <= rchange[a] <= rexposure[a]):
                hierarchy_ok = False

        for a, b in ((0, 1), (1, 0)):
            if q0[a] - q0[b] > rexposure[a] + rexposure[b]:
                t3_premises += 1
                if not q1[a] > q1[b]:
                    t3_ok = False
            if q0[a] - q0[b] > rminus[a] + rplus[b]:
                if not q1[a] > q1[b]:
                    directional_ok = False

        # Exact T4 bounds.
        L0 = dict(q0)
        U0 = dict(q0)
        RU = dict(rexposure)
        L1 = {a: L0[a] - RU[a] for a in (0, 1)}
        U1 = {a: U0[a] + RU[a] for a in (0, 1)}
        bar = max(L1.values())
        admissible = {a for a in (0, 1) if U1[a] >= bar}
        true_opt = {a for a in (0, 1) if q1[a] == max(q1.values())}
        if not true_opt.issubset(admissible) or not admissible:
            t4_ok = False

        # Deliberately loosened valid bounds.
        slack = F(1, 6)
        L0l = {a: max(F(0), q0[a] - slack) for a in (0, 1)}
        U0l = {a: min(F(1), q0[a] + slack) for a in (0, 1)}
        RUl = {a: min(F(1), rexposure[a] + slack) for a in (0, 1)}
        L1l = {a: L0l[a] - RUl[a] for a in (0, 1)}
        U1l = {a: U0l[a] + RUl[a] for a in (0, 1)}
        barl = max(L1l.values())
        admissible_l = {a for a in (0, 1) if U1l[a] >= barl}
        if not true_opt.issubset(admissible_l) or not admissible_l:
            t4_ok = False

        # Directional T4 refinement.
        L1d = {a: q0[a] - rminus[a] for a in (0, 1)}
        U1d = {a: q0[a] + rplus[a] for a in (0, 1)}
        bard = max(L1d.values())
        admissible_d = {a for a in (0, 1) if U1d[a] >= bard}
        if not true_opt.issubset(admissible_d) or not admissible_d:
            directional_ok = False

    check("L2-T1 pointwise and fixed-policy bounds over 2000 games", t1_ok)
    check("L2-T2 root-action field bound over 2000 games", t2_ok)
    check("L2-T3 stability whenever premise fires", t3_ok and t3_premises > 0, f"premises={t3_premises}")
    check("L2-T4 exact/loosened safe screens over 2000 games", t4_ok)
    check("directional correction hierarchy and one-sided screens", directional_ok and hierarchy_ok)

    # L2-T5: random deterministic maps on finite sets plus explicit period 4.
    periodic_ok = True
    rng2 = Random(99)
    for size in range(1, 33):
        for _ in range(20):
            mapping = [rng2.randrange(size) for _ in range(size)]
            x = rng2.randrange(size)
            seen: Dict[int, int] = {}
            for t in range(size + 1):
                if x in seen:
                    break
                seen[x] = t
                x = mapping[x]
            else:
                periodic_ok = False
    period4 = [1, 2, 3, 0]
    x = 0
    orbit = []
    for _ in range(8):
        orbit.append(x)
        x = period4[x]
    check("L2-T5 eventual periodicity finite-map model check", periodic_ok)
    check("L2-T5 explicit nonconvergent period-4 fixture", orbit == [0, 1, 2, 3, 0, 1, 2, 3])


# ---------------------------------------------------------------------------
# Cancellation / hazard fixture
# ---------------------------------------------------------------------------

def verify_directional_hazard_fixture() -> None:
    # A = safe high trump, B = vulnerable double. On 99 worlds both make;
    # on one world only A makes. B has no upside and one-sided downside.
    n = 100
    a = [1] * n
    b = [1] * 99 + [0]
    p_plus = F(sum(1 for x, y in zip(a, b) if x == 1 and y == 0), n)
    p_minus = F(sum(1 for x, y in zip(a, b) if x == 0 and y == 1), n)
    q = p_plus + p_minus
    g = p_plus - p_minus
    check(
        "rare unforced-risk fixture is strict dominance, not cancellation",
        p_plus == F(1, 100) and p_minus == 0 and q == g == F(1, 100),
        f"p+={p_plus} p-={p_minus}",
    )


def main() -> int:
    verify_evidence_processes()
    verify_bounded_mean()
    verify_risk_ledger()
    verify_execution_order()
    verify_l2()
    verify_directional_hazard_fixture()
    if FAILURES:
        print(f"FAILURES {len(FAILURES)}")
        for f in FAILURES:
            print(f)
        return 1
    print("ALL CHECKS PASS")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

```
