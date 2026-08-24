---
number: TBD (unassigned — draft)
slug: ce-evidence-process
channel: new-chat
status: "DRAFT — NOT DISPATCHED. Authorization: none. Batch quota: TBD with Jason."
deliverable: exact-rational program reproducing the CE-T1/T2/T3 identities on a declared grid or exhibiting a numeric counterexample as exact rationals; step-certification or refutation of the anytime-validity claims
---
STATUS: DRAFT — NOT DISPATCHED. Authorization: none. Batch quota: TBD with Jason.

You are performing adversarial mathematical review for a games-mathematics
project. You see ONLY this text: no repository, no prior conversation, no
outside sources. Everything you need is defined below. The claims under review
are the project's own **exploratory-tier working mathematics** — unconfirmed by
any independent process — and your job is to refute them if you can. Your
response will be adjudicated mechanically by reviewers holding the full corpus:
programs executed, witnesses re-run, proofs step-checked. Hedged or
unverifiable claims score zero. An honest partial result with a complete proof
outranks a full claim with a gap.

## 1. Setting

A solver must compare fixed decision policies by evaluating them on a stream of
randomly sampled hidden states ("worlds") and stopping at a **data-dependent
time** without invalidating its error statement. The proposed instrument is an
exact-rational nonnegative-supermartingale evidence process with an
anytime-valid crossing rule (a Ville-type bound). No floating point appears in
any correctness statement: all quantities are exact rationals.

Throughout, `Pr` and `E` are with respect to the declared sampling law.
"Anytime-valid at level α" means: the probability, under every law satisfying
the null, that the evidence process EVER reaches `1/α` — at any data-dependent
stopping time, with arbitrary peeking — is at most α.

## 2. The claims under attack (stated verbatim-with-context)

### 2.1 CE-T1 — upper-threshold evidence process

Let `B_1, B_2, … ∈ {0,1}` be independent Bernoulli observations with success
probability `p`. (The parent document also asserts a generalization: the proof
is claimed to permit a *predictable sequence* whose conditional success
probabilities all obey the tested null; that generalization is attacked in a
separate brief and is not your primary target here, though you may address it.)
Fix a rational threshold `c ∈ (0,1)`. Test `H_0: p ≤ c` against `p > c`.

After `s` successes and `f` failures, define

    E>_{s,f}(c) = (1/(1-c)) · ∫_c^1 (r/c)^s · ((1-r)/(1-c))^f dr.

**Claim A (supermartingale).** Under every law with `p ≤ c`, the process
obtained by updating this value after every observation is a nonnegative
supermartingale beginning at 1.

**Claim B (anytime crossing).** For every `α ∈ (0,1)`,

    Pr_{p≤c}( sup_n E>_{S_n,F_n}(c) ≥ 1/α ) ≤ α,

where `S_n, F_n` are the success/failure counts after `n` observations.
Consequently the experimenter may examine the evidence after every observation
and stop the first time `E>_{s,f}(c) ≥ 1/α`, with no post-hoc peeking
correction.

**Claimed proof.** For fixed `r ≥ c` define the one-step likelihood factor

    L_r(B) = (r/c)^B · ((1-r)/(1-c))^{1-B}.

If the (conditional) success probability is `p ≤ c`, then

    E[L_r(B)] = p·r/c + (1-p)(1-r)/(1-c) = 1 + (p-c)(r-c)/(c(1-c)) ≤ 1.

So the running product of factors is a nonnegative supermartingale for each
fixed `r`; `E>` is the uniform mixture of those products over `r ∈ [c,1]`; a
nonnegative mixture of supermartingales is a supermartingale; Ville's
inequality gives the crossing bound.

**Claim C (exact rational form).** With `R = (1-c)/c` and the substitution
`r = c + (1-c)t`,

    E>_{s,f}(c) = ∫_0^1 (1+Rt)^s (1-t)^f dt
                = Σ_{i=0}^{s} C(s,i) · R^i · i!·f!/(i+f+1)!.

For rational `c` every term is rational; a threshold comparison is an integer
cross-multiplication.

### 2.2 CE-T2 — lower-threshold evidence process

To test `H_0: p ≥ c` against `p < c`, define

    E<_{s,f}(c) = E>_{f,s}(1-c).

**Claim D.** `Pr_{p≥c}( sup_n E<_{S_n,F_n}(c) ≥ 1/α ) ≤ α`.

**Claim D′ (identity).** `E<_{s,f}(c)` equals the independently constructed
natural lower-test uniform mixture `(1/c)·∫_0^c (r/c)^s((1-r)/(1-c))^f dr` —
i.e. the definition is the genuine mixture object for the lower test, not
merely a notational symmetry. (State it carefully: the mixture for the lower
test uses alternatives `r ≤ c` with the same likelihood-ratio kernel against
the boundary null `p = c`.)

### 2.3 CE-T3 — exact pivotal-direction evidence

Two fixed policies `a` and `b` are evaluated on the SAME world; each produces a
Boolean success indicator; the signed outcome is

    Y ∈ {-1, 0, +1}   (Y = u_a - u_b).

Write `p_+ = Pr(Y=+1)`, `p_- = Pr(Y=-1)`, `q = p_+ + p_-` (pivotal mass),
`g = E[Y] = p_+ - p_-` (gap), and, when `q > 0`,
`θ = Pr(Y=+1 | |Y|=1) = p_+/q`. Test `H_0: g ≤ 0` (equivalently `θ ≤ 1/2` on
the pivotal component).

Let `a_n = #{i ≤ n : Y_i = +1}` and `b_n = #{i ≤ n : Y_i = -1}`. Define

    E+_{a,b} = ∫_0^1 (1+t)^a (1-t)^b dt      (a = a_n, b = b_n).

A nonpivotal observation (`Y=0`) leaves the evidence unchanged.

**Claim E (raw-stream validity).** `E+` is a valid anytime evidence process on
the RAW world stream, not merely on a retrospectively selected pivotal
subsample. The claimed mechanism: for fixed mixture component `r ≥ 1/2`, the
raw-world multiplier

    L_r(Y) = 2r        if Y = +1,
             2(1-r)    if Y = -1,
             1         if Y = 0

satisfies, under `g ≤ 0` (θ ≤ 1/2),

    E[L_r(Y)] = 1 - q + q·(2θr + 2(1-θ)(1-r)) ≤ 1,

so waiting through arbitrarily many nonpivotal worlds is safe: they cost time
but create no directional evidence. (`E+` is the uniform mixture of the
products of `L_r` over `r ∈ [1/2, 1]`, normalized so `E+_{0,0} = 1`.)

**Claim F (closed integer form).** With `k = a+b`,

    E+_{a,b} = ( Σ_{x=0}^{a} C(k+1, x) ) / ( (k+1) · C(k,a) ).

**Claimed anchors.**

    E+_{0,0}=1   E+_{1,0}=3/2   E+_{0,1}=1/2
    E+_{2,0}=7/3  E+_{1,1}=2/3  E+_{2,1}=11/12  E+_{3,0}=15/4

**Claim G (all-favorable closed form).** `E+_{a,0} = (2^{a+1}-1)/(a+1)`.

**Claim H (calculated pivotal requirement).** For a one-sided test at
`α = 1/128`: nine consecutive favorable pivots are insufficient
(`E+_{9,0} = 1023/10 < 128`) while ten are sufficient
(`E+_{10,0} = 2047/11 > 128`).

**Claim I (consistency).** Claim F's closed form equals the CE-T1 finite sum of
Claim C evaluated at `c = 1/2` with `(s,f) = (a,b)`, i.e. the pivotal engine is
the `c = 1/2` specialization of the general Bernoulli engine.

## 3. THE TASK — refute if you can

Any ONE of the following, delivered with a machine-checkable artifact, scores
full credit. Certifying all of them, with every load-bearing step named, also
scores full credit. Partial rigorous results score proportionally.

(A) **Identity counterexample.** Exhibit any rational `c ∈ (0,1)` and integers
`s,f ≥ 0` for which the three expressions in Claim C (defining mixture
integral, substituted integral, finite sum) are not all equal; or any
`a,b ≥ 0` for which Claim F's closed form differs from the defining integral
`∫_0^1 (1+t)^a(1-t)^b dt`, or for which Claim I fails, or any anchor in the
list that is wrong, or any `a` for which Claim G fails. All comparisons in
exact rational arithmetic.

(B) **Anytime-validity counterexample.** Exhibit a distribution satisfying the
null (`p ≤ c` for CE-T1; `p ≥ c` for CE-T2; `g ≤ 0` for CE-T3) and an adapted
stopping/peeking strategy under which the probability of the evidence process
ever reaching `1/α` exceeds α — or a demonstration that the process fails to be
a supermartingale under some null law (a single one-step conditional
expectation exceeding 1 suffices; give it as exact rationals). Note the exact
boundary cases: `c` at which denominators vanish, `q = 0` streams, and laws
mixing pivotal and nonpivotal outcomes adversarially.

(C) **One-step algebra.** Either certify the identity
`E[L_r(B)] = 1 + (p-c)(r-c)/(c(1-c))` and the inequality chain of Claim E as
correct for all `p ≤ c ≤ r` in `(0,1)` (resp. `θ ≤ 1/2 ≤ r`), naming every
step, or produce a rational triple `(p,c,r)` (resp. `(q,θ,r)`) violating them.

(D) **Mixture/Ville step.** The proof leans on: (i) a nonnegative mixture of
nonnegative supermartingales, each started at 1, is a nonnegative
supermartingale started at 1 (including the continuum uniform mixture over
`r`, with the implicit Fubini/Tonelli step); (ii) Ville's inequality. Certify
or refute these steps AS USED — in particular whether the interchange of
integral (over `r`) and conditional expectation is licit here, and whether
`E>_{0,0}(c) = 1` and `E+_{0,0} = 1` hold exactly so the processes truly start
at 1.

(E) **Claim D′.** Prove or refute that `E<_{s,f}(c) = E>_{f,s}(1-c)` is the
genuine lower-test uniform mixture and not merely definitionally convenient;
if the natural lower-test mixture object differs, exhibit `(c,s,f)` where the
two differ, as exact rationals.

Zero credit: floating-point demonstrations, asymptotic arguments standing in
for exact claims, or "the literature says" without a self-contained proof.
These formulas resemble known safe-testing/e-process constructions; you may
recognize them, but your certification must stand on the algebra in this brief
alone.

## 4. DELIVERABLE CONTRACT

End your response with a section titled exactly `MACHINE-CHECKABLE ARTIFACTS`:

1. A line `FINAL ANSWER: COUNTEREXAMPLE (<which claim>)` or
   `FINAL ANSWER: CERTIFIED (claims A–I)` or `FINAL ANSWER: PARTIAL (<scope>)`.
2. Your proof or refutation as numbered steps, each labeled `[USES: …]`
   (definitions above, prior steps, or standard named theorems with a
   self-contained statement of what is used).
3. One self-contained program (any language with exact rational arithmetic in
   its standard library — e.g. Python 3 with `fractions`; single fenced block;
   deterministic; no network or file I/O; under 30 minutes on one core) that:
   (a) implements the defining integrals of `E>` and `E+` by exact polynomial
   integration (expand, integrate term-by-term with rational coefficients —
   no quadrature), the finite sum of Claim C, and the closed form of Claim F,
   entirely in exact rationals;
   (b) verifies Claims C, F, G, I and all anchors on the declared grid
   `c ∈ {1/10, 1/3, 1/2, 11/16, 2/3, 9/10}`, `0 ≤ s,f ≤ 12`, and
   `0 ≤ a,b ≤ 40`, plus Claim H — OR prints your counterexample as exact
   rationals with both conflicting values;
   (c) verifies the one-step identities of Task (C) on a full rational grid
   `p,c,r ∈ {1/8, 1/4, …, 7/8}` restricted to the null-and-alternative
   ordering — OR prints the violating triple;
   printing one `PASS <check>` / `FAIL <check> <detail>` line per check, exit 0
   iff all pass.
4. For Task (B), if you claim an anytime-validity counterexample: an exact
   computation (finite-horizon exhaustive tree over your declared law, exact
   rationals) showing the crossing probability exceeding α, not a simulation.

A response whose program fails any of its own checks scores zero.
