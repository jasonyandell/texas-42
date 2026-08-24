---
number: TBD (unassigned — draft)
slug: ce-bounded-mean
channel: new-chat
status: "DRAFT — NOT DISPATCHED. Authorization: none. Batch quota: TBD with Jason."
deliverable: exact-rational program verifying CE-T4/T5 factor nonnegativity, λ-ranges, and one-step inequalities on a declared grid — or a rational counterexample; verdict on the sign-majority unsoundness claim with a characterization or refutation
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

A solver compares two evaluation pipelines by paired observations on common
random inputs. Each observation is a bounded rational value. The project's
older habit was to count which pipeline produced the larger value per block
("sign majority"). The mathematics under review (i) replaces that with a
bounded-mean betting process claimed to be anytime-valid, and (ii) claims the
sign-majority habit is UNSOUND for concluding mean order in this payoff class.
Both the replacement and the unsoundness claim are your targets.

"Anytime-valid at level α": under every law satisfying the null, the
probability that the evidence process EVER reaches `1/α` — at any
data-dependent stopping time, with arbitrary peeking — is at most α. The
standard machinery assumed: a nonnegative supermartingale starting at 1
satisfies Ville's inequality `Pr(sup_n M_n ≥ 1/α) ≤ α`.

## 2. The claims under attack (stated verbatim-with-context)

Let `X_1, X_2, … ` be observations with values in a known interval `[L, U]`
(`L < U`, rational endpoints), adapted to a filtration `(F_i)`; "independent or
conditionally mean-controlled" means the null below constrains the conditional
mean given the past.

### 2.1 CE-T4 — positive bounded-mean betting process

To test

    H_0 : E[X_i | F_{i-1}] ≤ c        (c rational, L < c < U)

choose any rational λ with

    0 ≤ λ ≤ 1/(c - L)

and define

    M_n^+(λ; c) = Π_{i=1}^{n} ( 1 + λ(X_i - c) ).

**Claim A (nonnegativity).** Every factor is nonnegative: for all
`X ∈ [L, U]` and λ in the stated range, `1 + λ(X - c) ≥ 0`.

**Claim B (supermartingale).** Under the null,
`E[ 1 + λ(X_i - c) | F_{i-1} ] ≤ 1`, hence `M_n^+` is a nonnegative
supermartingale starting at 1, hence anytime-valid at level α with threshold
`1/α` by Ville.

**Claim C (mixtures).** For a finite rational mixture `{(w_j, λ_j)}_{j=1..J}`,
`w_j ≥ 0`, `Σ_j w_j = 1`, each `λ_j` in the valid range,

    M_n^+(c) = Σ_j w_j · M_n^+(λ_j; c)

is an exact-rational anytime-valid evidence process (nonnegative
supermartingale starting at 1 under the null).

### 2.2 CE-T5 — negative bounded-mean betting process

To test `H_0 : E[X_i | F_{i-1}] ≥ c`, choose rational

    0 ≤ λ ≤ 1/(U - c)

and use

    M_n^-(λ; c) = Π_{i=1}^{n} ( 1 - λ(X_i - c) ),

with finite rational mixtures valid exactly as above (Claims A′/B′/C′, the
mirror images of A/B/C).

### 2.3 The practical-equivalence composition (context)

The two engines are composed: to establish `|g| < ε` for a mean `g` of
observations in `[-1, 1]` (rational tolerance `ε > 0`), apply CE-T5 at `c = ε`
to reject `g ≥ ε`, and CE-T4 at `c = -ε` to reject `g ≤ -ε`, each at its own
declared risk, both reading the SAME observation stream; when both are
settled, the conclusion `|g| < ε` is claimed to hold with error probability at
most the SUM of the two risks (a union bound over the two one-sided errors).

### 2.4 The §10.1 sign-majority defect (the unsoundness claim)

Suppose one fixed evaluation algorithm produces iid paired block values
`V_{a,j}, V_{b,j} ∈ [0,1]` and define `X_j = V_{a,j} - V_{b,j} ∈ [-1,1]`.
CE-T4 with `c = 0` tests whether the MEAN block difference is positive. The
project claims this is mathematically different from comparing the frequencies
of `X_j > 0` versus `X_j < 0`, because **sign frequency does not determine
mean order**, exhibiting:

    X = +1/8  with probability 3/4,
        -1/2  with probability 1/4.

Then `Pr(X > 0) = 3/4` yet
`E[X] = (3/4)(1/8) - (1/4)(1/2) = 3/32 - 4/32 = -1/32 < 0`.

**Claim D (defect).** A sign test on this law will eventually become confident
in the WRONG mean ordering (the positive sign wins 75% of blocks while the
mean is negative), so any component that treats block-sign majority as proof
of positive mean is unsound for this payoff class (bounded signed rational
differences); the bounded-mean process, which uses magnitude, does not have
this defect.

## 3. THE TASK — refute if you can

Any ONE of the following with a machine-checkable artifact scores full credit;
so does a certification of all of them with every load-bearing step named.

(A) **λ-range error.** Exhibit rational `(L, U, c, λ, x)` with λ in the
CE-T4-declared range `[0, 1/(c-L)]` and `x ∈ [L,U]` such that
`1 + λ(x - c) < 0`; or the mirror for CE-T5's range `[0, 1/(U-c)]`; or prove
the declared ranges are exactly the maximal λ-intervals guaranteeing factor
nonnegativity for all `x ∈ [L,U]` (i.e. certify sharpness, not just
sufficiency). Pay attention to the endpoints `λ = 1/(c-L)`, `λ = 1/(U-c)`
(zero-valued factors are permitted for a nonnegative supermartingale — say
explicitly whether anything downstream breaks when a factor hits exactly 0 and
the process is absorbed at 0), and to the degenerate limits `c → L`, `c → U`.

(B) **Supermartingale failure.** Exhibit a null-satisfying conditional law on
`[L,U]` and a valid λ (or valid finite mixture) with a one-step conditional
expectation exceeding 1, as exact rationals — or certify Claims B/B′/C/C′,
including that mixing preserves the start-at-1 and nonnegativity properties
and that Ville applies to the mixture.

(C) **Composition error.** Attack §2.3: the two one-sided tests read the same
stream and are not independent. Either exhibit a joint law and stopping
strategy under which `Pr(both tests falsely settle ∪ either falsely settles)`
exceeds the summed risks when `|g| ≥ ε` — i.e. the union-bound composition
fails — or certify that the union bound needs no independence and the
composition is valid as stated.

(D) **Attack the unsoundness claim (Claim D).** Either:
  (i) verify the counterexample's arithmetic and STRENGTHEN the claim into a
  precise theorem — e.g. characterize exactly the payoff classes on which
  sign-majority IS sound for mean order (a natural candidate: `|X|` constant
  on the pivotal event; or symmetric magnitude distributions), with proof,
  and confirm that bounded signed rational block differences fall outside
  every such class; or
  (ii) refute Claim D's operational half — show that the specific procedure
  "sign test eventually becomes confident in the wrong mean ordering" does
  not follow from the stated law for some reasonable formalization of "sign
  test", and say precisely which formalizations are and are not indicted; or
  (iii) exhibit an error in the claim that the bounded-mean process avoids
  the defect: a bounded law and valid mixture under which CE-T4 at `c = 0`
  itself becomes confident in the wrong mean direction with probability
  exceeding its declared α.

(E) **Boundary bookkeeping.** The observations are DIFFERENCES `X = V_a - V_b`
of `[0,1]` values, so `X ∈ [-1,1]`; CE-T4 at `c = -ε` needs
`λ ≤ 1/(c - L) = 1/(-ε+1) = 1/(1-ε)`, and CE-T5 at `c = ε` needs
`λ ≤ 1/(U - c) = 1/(1-ε)`. Certify or correct these range computations and
state whether any published λ-range in this brief is wrong for the difference
payoff class.

Zero credit: floating-point demonstrations; simulation in place of exact
computation; appeals to literature without self-contained algebra. These
resemble known betting-martingale constructions; recognition is fine, but
certification must stand on this brief's algebra alone.

## 4. DELIVERABLE CONTRACT

End your response with a section titled exactly `MACHINE-CHECKABLE ARTIFACTS`:

1. A line `FINAL ANSWER: COUNTEREXAMPLE (<which claim>)` or
   `FINAL ANSWER: CERTIFIED (A–E)` or `FINAL ANSWER: PARTIAL (<scope>)`.
2. Numbered proof steps labeled `[USES: …]`.
3. One self-contained program (any language with exact rational arithmetic in
   its standard library — e.g. Python 3 `fractions`; single fenced block;
   deterministic; no network/file I/O; under 30 minutes one core) that:
   (a) for the declared grid `L = -1, U = 1`,
   `c ∈ {-1/2, -1/8, 0, 1/8, 1/2}`, λ traversing
   `{0, 1/4, 1/2, 3/4, 1} · λ_max(c)` and `x ∈ {-1, -1/2, -1/32, 0, 1/8, 1}`:
   verifies factor nonnegativity for CE-T4 and CE-T5 — or prints your
   violating tuple;
   (b) for a declared grid of two-point null laws on `[-1,1]` (state it in
   the program), verifies the one-step conditional expectation ≤ 1 for every
   valid λ on the grid and for at least one nontrivial mixture — or prints
   the violating law and λ;
   (c) verifies the §10.1 arithmetic exactly (`Pr(X>0) = 3/4`,
   `E[X] = -1/32`) and, by exact finite-horizon computation over the two-point
   law (no simulation), demonstrates your Task (D) position: e.g. computes the
   exact probability after `n` blocks that sign-majority favors `+` for a
   stated increasing sequence of `n`, and the exact behavior of a stated
   CE-T4 mixture at `c = 0` on the same law;
   printing `PASS <check>` / `FAIL <check> <detail>` lines, exit 0 iff all
   pass.
4. If you claim any anytime-validity violation: an exact finite-horizon tree
   computation of the crossing probability under your law, as rationals, not a
   simulation.

A response whose program fails any of its own checks scores zero.
