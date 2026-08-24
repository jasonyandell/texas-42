---
number: 021
slug: ce-risk-ledger-escalation
channel: new-chat
status: "DISPATCHED 2026-08-24 (hand-ferried by Jason, batch of five, quota cleared by his delivery). Response received same day: exchange/inbox/019-023-response-panel-and-cancellation-v0.1.md."
deliverable: an explicit adaptive adversary (scheduler/candidate-set strategy) that overspends the risk ledger or double-counts a world across the sample-to-enumeration switch, with exact arithmetic — or a verdict that the accounting is airtight with the load-bearing steps named
---
STATUS: DRAFT — NOT DISPATCHED. Authorization: none. Batch quota: TBD with Jason.

You are performing adversarial mathematical review for a games-mathematics
project. You see ONLY this text: no repository, no prior conversation, no
outside sources. Everything you need is defined below. The claims under review
are the project's own **exploratory-tier working mathematics** — unconfirmed by
any independent process — and your job is to break the ACCOUNTING: find an
adaptive strategy that spends more error probability than declared, or a
schedule that corrupts the exact endpoint. Your response will be adjudicated
mechanically by reviewers holding the full corpus: programs executed,
witnesses re-run, proofs step-checked. Hedged or unverifiable claims score
zero.

## 1. Setting and the assumed per-edge primitive

A solver compares `m` fixed ("frozen") decision policies by evaluating all of
them on one common stream of worlds sampled uniformly WITH REPLACEMENT from a
finite population of size `N` (the "fiber"; every world can also be
enumerated). For each ordered pair `(i,j)` of policies there is a directed
evidence process `E_{i→j}(n)`, a function of the paired outcomes on the first
`n` stream worlds.

**Assumed primitive (do not attack here; it has its own brief).** For each
ordered pair and each `α ∈ (0,1)`: if policy `j`'s true value is ≥ policy
`i`'s (i.e. the directed claim "i beats j" is FALSE or tied), then

    Pr( sup_n E_{i→j}(n) ≥ 1/α ) ≤ α,

anytime-valid: the bound holds under arbitrary data-dependent stopping and
peeking. Evidence processes for different pairs are driven by the same world
stream and are arbitrarily dependent on each other. You attack everything
built ON TOP of this primitive.

"Value" of a policy is its exact mean success over the uniform population:
`V_ρ = #{ω : u_ρ(ω) = 1} / N`, where `u_ρ(ω) ∈ {0,1}` is the deterministic
outcome of frozen policy ρ on world ω.

## 2. The claims under attack (O21 — risk-ledger completeness)

### 2.1 All-pairs allocation

Fix a decision-level error budget `δ_dec ∈ (0,1)`. Allocate

    α_{i→j} = δ_dec / (m(m-1))

to every ordered pair (there are `m(m-1)` of them), giving the common directed
edge threshold

    T_edge = m(m-1) / δ_dec.

Draw a settled edge `i → j` the first time `E_{i→j} ≥ T_edge`.

**Claim A.** By the union bound and the per-edge anytime property,
`Pr(any false directed edge is EVER drawn) ≤ δ_dec` — and this remains true
when: pair examinations are adaptive; candidates are eliminated as evidence
arrives; worlds arrive in arbitrary batch sizes; the provisional leader
changes; the computation is paused and resumed. The only stated requirement:
the candidate set stays fixed during the evidence epoch.

### 2.2 Safe elimination

**Claim B.** A candidate may be removed when any live candidate has a settled
edge into it. On the event that no false edge is ever drawn, a truly best
candidate cannot be eliminated by a worse one; when one candidate remains it
is a true maximizer of the fixed candidate set; exactly-tied maximizers may
survive forever, which is accepted behavior.

### 2.3 Telescoping run ledger

For a run comprising an unknown, unbounded number of decision events, with
total budget `δ_run`, the `d`-th decision event receives

    δ_d = δ_run / (d(d+1)),      Σ_{d=1}^{∞} δ_d = δ_run.

Within decision `d`, `δ_d` is subdivided among directed comparisons, optional
equivalence tests, rediscovery epochs, and recursively sampled inner decisions
included in the same claim.

**Claim C.** This supports an unknown number of adaptively chosen decisions
with total false-settlement probability ≤ `δ_run`, PROVIDED the allocation is
declared and serialized; a δ without its scope is meaningless.

### 2.4 Sequential edge-opening variant

**Claim D.** A variant that opens directed comparisons one at a time, giving
the `ℓ`-th NEWLY OPENED test `α_ℓ = δ_dec/(ℓ(ℓ+1))`, is also sound (spends
risk only on opened edges), even when the choice of which edge to open next
depends on all evidence so far.

### 2.5 Candidate-set mutation

**Claim E.** If the candidate set mutates (new policy, or a policy's identity
changes), old evidence does not apply to the new set; the sound options are:
new epoch on fresh worlds; retain old pair processes only for literally
unchanged policy identities while new pairs get separately accounted risk; or
discard the epoch. (Attack: is option 2 actually sound — retained processes
have already consumed stream prefix and peeks — or does re-USE of old evidence
under a new candidate set create a selection effect the ledger does not pay
for?)

### 2.6 Exact results spend no risk

**Claim F.** Full-population exact evaluation consumes no sampling-error
budget; a run that escalates to exactness may close the corresponding ledger
entry without spending its remaining allocation — and this refund is sound
even when the DECISION to escalate was made adaptively after peeking at the
evidence.

## 3. The claims under attack (O24 — exact-escalation bookkeeping)

The escalation mechanism:

1. Worlds are sampled uniformly with replacement; the sampled stream may
   contain DUPLICATES of the same physical world.
2. Every physical world has a canonical identity; the terminal outcome
   `u_ρ(ω)` of every frozen policy on every DISTINCT sampled world is cached
   (outcomes are deterministic, so multiplicity does not change them).
3. At an arbitrary, adaptively chosen stream index the controller may switch
   to exact enumeration: it evaluates every not-yet-cached world of the
   population exactly once, reusing cached outcomes for worlds already seen.
4. The final exact value is `V_ρ = #{ω : u_ρ(ω)=1} / N` computed over the `N`
   UNIQUE worlds — each physical world counted exactly once regardless of its
   multiplicity in the sampled stream.

**Claim G (switch-equals-cold).** Switching to enumeration at ANY stream
index — including index 0 (cold), mid-batch, after pauses/resumes, and after
some candidates were eliminated — yields the same exact endpoint `V_ρ` for
every surviving policy as a cold full enumeration. Sampled multiplicities are
never double-counted in the exact sum.

**Claim H (typing).** The exact endpoint is exact regardless of any error in
the cost forecast that triggered the switch; the switch rule affects
performance only, never correctness.

## 4. THE TASK — break the ledger or certify it

Full credit for any ONE of the following, with the required artifact; full
credit also for a complete certification naming every load-bearing step.

(A) **Ledger-overspend adversary.** Construct an explicit adversary — a
data-dependent strategy for: examination order, elimination timing, batch
boundaries, pause/resume, decision-event scheduling under §2.3, edge-opening
order under §2.4, and/or candidate-set mutation handling under §2.5 option
2 — together with a concrete finite population and policy outcome table, such
that the probability of at least one false settlement exceeds the declared
budget (`δ_dec` or `δ_run` at the appropriate scope). The failure probability
must be computed EXACTLY (finite-horizon exhaustive tree over your declared
law, exact rationals), not simulated. If your adversary needs to violate a
stated requirement (e.g. mutate the candidate set mid-epoch while keeping
evidence), say so explicitly — that scores as "the stated requirement is
load-bearing", which is valuable, but less than an adversary that wins WITHIN
the stated rules.

(B) **Double-count adversary.** Construct an explicit schedule (sample
sequence with duplicates, cache states, switch index, partial batches,
mid-epoch eliminations) under which the DESCRIBED mechanism of §3 produces an
exact endpoint different from cold enumeration — i.e. find the bookkeeping
hole in Claim G — or a case where a world is counted with multiplicity > 1 in
the exact sum, or a world is silently omitted (e.g. cached for one policy but
not another at switch time; an eliminated candidate's cache reused unsoundly
if it re-enters via §2.5). Exhibit the arithmetic explicitly.

(C) **Airtight verdict.** Certify Claims A–H. This must not be a nod: for
each claim name the load-bearing steps — for Claim A, why adaptive
examination order and elimination spend nothing beyond the up-front union
bound over the FIXED finite edge set (each edge's supremum bound is over the
whole future, so no optional-stopping surcharge); for Claim D, why
data-dependent edge opening is covered (e.g. each edge's α is assigned by
opening ORDER, a predictable index, and Σ 1/(ℓ(ℓ+1)) = 1); for Claim E option
2, the precise measurability condition making retention sound, or a proof it
is unsound (see (A)); for Claim F, why an adaptively timed refund cannot be
laundered into extra spending elsewhere; for Claim G, the exact invariant
(e.g. "the final sum ranges over canonical world IDs, and the cache is a
function from (policy, world ID) to outcome, populated at most once") and why
partial batches and elimination cannot break it.

(D) **Boundary findings.** Anything sharp about edges of the design: `m = 1`
(no pairs, `T_edge` undefined); `δ_dec ≥ 1`; a decision event that opens zero
edges; exhaustion of the population by sampling (all N worlds cached) making
the "switch" vacuous; ties (`V_i = V_j` exactly) interacting with elimination.

## 5. DELIVERABLE CONTRACT

End your response with a section titled exactly `MACHINE-CHECKABLE ARTIFACTS`:

1. A line `FINAL ANSWER: ADVERSARY (<ledger|double-count>)` or
   `FINAL ANSWER: AIRTIGHT (claims A–H certified)` or
   `FINAL ANSWER: PARTIAL (<scope>)`.
2. Numbered proof steps labeled `[USES: …]`. For an adversary: the complete
   adversary description (state machine or pseudocode) plus its exact failure
   probability as a rational, with the arithmetic shown.
3. One self-contained program (any language with exact rational arithmetic in
   its standard library — e.g. Python 3 `fractions`; single fenced block;
   deterministic; no network/file I/O; under 30 minutes one core) that:
   (a) verifies the ledger algebra exactly: `Σ_{ℓ=1}^{K} 1/(ℓ(ℓ+1)) = K/(K+1)`
   for `K` up to 10⁴, `T_edge · α_{i→j} = 1`, and your declared sub-allocation
   sums;
   (b) implements a miniature version of the §3 escalation mechanism (small
   N, e.g. N ≤ 12, 2–4 policies with declared outcome tables) and replays
   YOUR adversarial schedule (for (B)) or a systematic family of schedules
   (for (C): all switch indices 0..stream length, several duplicate-heavy
   streams, eliminations at varied indices), comparing every run's endpoint
   to cold enumeration and printing per-run `PASS`/`FAIL`;
   (c) for a Task (A) adversary: computes the exact false-settlement
   probability over your finite tree and compares it to the declared budget;
   printing `PASS <check>` / `FAIL <check> <detail>` lines, exit 0 iff all
   checks pass (for adversary claims, "pass" means your counterexample is
   confirmed: the mechanism's output demonstrably differs from the
   specification's promise).

A response whose program fails any of its own checks scores zero. A verdict of
"airtight" with unnamed load-bearing steps scores as hedging.
