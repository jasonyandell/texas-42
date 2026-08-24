---
number: TBD (unassigned — draft)
slug: ce-execution-order
channel: new-chat
status: "DRAFT — NOT DISPATCHED. Authorization: none. Batch quota: TBD with Jason."
deliverable: a scheduling counterexample (two lawful schedules on one fixed world stream with different reported settlements), or a proof sketch of execution-order invariance PLUS a precise resolution of the conditional-null / predictable-sequence question
---
STATUS: DRAFT — NOT DISPATCHED. Authorization: none. Batch quota: TBD with Jason.

You are performing adversarial mathematical review for a games-mathematics
project. You see ONLY this text: no repository, no prior conversation, no
outside sources. Everything you need is defined below. The claims under review
are the project's own **exploratory-tier working mathematics** — unconfirmed by
any independent process. Two targets: (1) an execution-order-invariance claim
about an evidence engine; (2) an open filtration question the project has
flagged for exactly this review and answered only provisionally. Your response
will be adjudicated mechanically by reviewers holding the full corpus:
programs executed, witnesses re-run, proofs step-checked. Hedged or
unverifiable claims score zero.

## 1. The engine (complete specification)

A solver compares a FROZEN finite set of deterministic policies
`{ρ_1, …, ρ_m}` (fixed for the whole evidence epoch) on a common stream of
worlds. All objects below are exact (integers/rationals); no floating point.

**W1 — counter-based world identity.** World `i` (for `i = 0, 1, 2, …`) is
generated from a seed derived deterministically from
`(root identity, evaluation epoch, i)`. Hence which world occupies index `i`
is independent of batch size, thread count, elimination decisions, and
pause/resume boundaries.

**W2 — common random worlds.** Every LIVE candidate is evaluated on world `i`
before pair evidence for index `i` is updated. Policy outcomes are
deterministic Booleans `u_ρ(ω_i) ∈ {0,1}`; the pair observation for ordered
pair `(ρ_a, ρ_b)` at index `i` is `Y_i = u_a(ω_i) − u_b(ω_i) ∈ {−1,0,+1}`.

**W3 — evidence.** For each ordered pair there is an evidence value that is a
deterministic function of the multiset-ordered sequence of that pair's
observations up to index `n` (concretely, of the counts
`a_n = #{i ≤ n : Y_i = +1}` and `b_n = #{i ≤ n : Y_i = −1}`; the concrete
formula `E+_{a,b} = ∫_0^1 (1+t)^a (1−t)^b dt`, an exact rational, may be taken
as given — its statistical validity has its own brief). A pair SETTLES at the
first index `n` where its evidence reaches a fixed rational threshold `T`.

**W4 — elimination.** A candidate is eliminated when any live candidate has a
settled edge into it. Eliminated candidates stop consuming FUTURE worlds;
previously accumulated pair counts remain, aligned by world index.

**W5 — batching.** The engine may process worlds in batches of any size
(1, 8, 64, 4096, …). A batch may overshoot the first crossing for throughput
reasons, but the REPORTED settlement index is the first crossing index inside
the batch. Evidence is conceptually updated in stream order.

**W6 — pause/resume.** The computation may be paused at any point and resumed;
resume re-derives worlds from W1 and continues.

## 2. The invariance claim under attack (O26)

**Claim INV.** For a fixed world stream (fixed root identity and epoch), a
frozen candidate set, and a fixed threshold/risk configuration, EVERY lawful
execution — any batch partition, any parallel schedule, any pause/resume
pattern — produces identical:

1. world IDs at every index;
2. per-pair counts `(a_n, b_n)` at every stream index `n` (for indices where
   both candidates were live);
3. first evidence-crossing index for every pair that settles;
4. elimination graph (who eliminated whom, at which index);
5. final result kind and selected move.

Timing and duplicated work may differ; nothing semantic may.

Note what the claim does NOT say: it does not claim invariance across
different world streams, different candidate sets, or different thresholds.

## 3. The flagged open question (state of the project's own doubt)

The evidence process's anytime validity is proved for iid observations, with a
claimed extension to "a predictable sequence whose conditional success
probabilities all obey the tested null." The project's internal review flagged
the following and answered it only provisionally; you are asked to settle it.

**Question Q (verbatim task).** State precisely the filtration and the
conditional-null condition under which the Bernoulli-threshold and
signed-pivotal evidence processes remain anytime-valid for non-iid streams —
and determine whether the common-world design of §1 satisfies that condition
when candidates are eliminated ADAPTIVELY (W4), given that elimination
decisions depend on past evidence, which depends on past worlds, which are
drawn from the same stream that generates future observations.

**The project's provisional answer (attack or certify it).** "The
common-stream design evaluates every live candidate on world `i` before
evidence updates, and elimination depends only on PAST evidence, so the
conditional null is inherited; this should be stated as an invariant to
assert" — i.e.: let `F_n` be the σ-algebra generated by worlds
`ω_0, …, ω_{n−1}` (and any independent scheduling randomness); worlds are iid
uniform and independent of scheduling; every data-dependent choice made before
index `n` (elimination, batch boundaries, pausing, which pairs are examined)
is `F_n`-measurable; therefore the conditional law of `Y_n` given `F_n` equals
its unconditional law, and the null hypothesis transfers conditionally, which
is exactly the hypothesis of the supermartingale argument. Potential holes you
should probe: scheduling randomness correlated with the stream; the fact that
a pair's observation SEQUENCE (restricted to indices where both candidates
are live) is a data-dependent subsequence of the stream — does selecting
indices by an `F_n`-measurable rule preserve the conditional null for the
SELECTED subsequence?; parallel schedules where evidence from world `i+k` is
computed before the elimination decision notionally due at index `i` has been
applied (W5 overshoot) — does the retroactive first-crossing reconstruction
fully repair this?

## 4. THE TASK

Full credit for any ONE of the following; full credit for a complete
certification with load-bearing steps named.

(A) **Scheduling counterexample to Claim INV.** Exhibit two lawful executions
(both satisfying W1–W6) on one fixed world stream and candidate set whose
reported settlements differ in any of items 1–5 — as an explicit small
instance: a world outcome table (e.g. 3 policies, ≤ 30 worlds, Boolean
outcomes you choose), two schedules described precisely (batch partitions,
elimination application points, pause points), and the divergent results
computed exactly. The interesting attack surface: interactions between W4 and
W5 — e.g. a batch that overshoots a crossing which, once reconstructed,
eliminates a candidate whose observations INSIDE that same batch were already
consumed by other pairs; races between two pairs settling in the same batch;
an elimination at index `n` discovered only at batch end while another
schedule with batch size 1 applies it immediately — do per-pair counts at
indices `n+1, …` then differ between the schedules, violating item 2? If the
specification as written is AMBIGUOUS about such a case (i.e. two readings
give different results), exhibiting the ambiguity precisely is itself a
full-credit finding: state both readings and the divergent outcomes.

(B) **Invariance proof sketch.** Prove Claim INV as a determinism statement:
define the canonical sequential execution (batch size 1, immediate
elimination), show every lawful execution's REPORTED artifacts equal the
canonical one's — the key lemmas being (i) worlds are schedule-independent
(W1), (ii) per-pair counts at index `n` are functions of the stream prefix
and the liveness sets, (iii) liveness sets are themselves functions of the
stream prefix by induction via reconstructed first crossings, and (iv) the
overshoot-reconstruction rule of W5 makes batch boundaries unobservable in
the artifacts. State exactly what W5's reconstruction must recover for (iv)
to hold (first crossing only? or full per-index liveness?) — if you find the
rule as stated under-specified for (iv), say precisely what must be added;
that is a finding, not a failure.

(C) **Resolution of Question Q.** A precise theorem: filtration, conditional
null, and a proof that the supermartingale property (hence Ville's crossing
bound) survives — covering the data-dependent subsequence selection issue —
OR a counterexample: a lawful W1–W6 configuration where adaptive elimination
or scheduling makes some pair's evidence process fail anytime validity at its
declared level (exact finite-horizon computation of the crossing probability
required, no simulation). Address specifically whether optional stopping /
optional selection by predictable rules is innocuous here, and whether the
independence of the world stream from all scheduling choices is load-bearing
(construct the failure when it is dropped, if you can — e.g. a scheduler that
peeks at world `i` before deciding whether index `i` counts).

(D) **Sharpen the specification.** If your analysis under (A)–(C) shows W1–W6
sound only under additional stated invariants (e.g. "elimination applied only
at batch boundaries whose index is a deterministic function of settlement
indices", "scheduling randomness independent of the stream", "the engine must
recompute liveness per index during reconstruction"), enumerate the minimal
additional invariants and prove sufficiency of the completed set.

Zero credit: floating-point or simulation-only demonstrations; "obviously
deterministic" without the induction; restating the provisional answer without
addressing the subsequence-selection and overshoot probes named above.

## 5. DELIVERABLE CONTRACT

End your response with a section titled exactly `MACHINE-CHECKABLE ARTIFACTS`:

1. A line `FINAL ANSWER: COUNTEREXAMPLE (INV)` or
   `FINAL ANSWER: COUNTEREXAMPLE (Q)` or
   `FINAL ANSWER: INVARIANT (proof) + Q RESOLVED (<condition>)` or
   `FINAL ANSWER: UNDER-SPECIFIED (<exact gap>)` or
   `FINAL ANSWER: PARTIAL (<scope>)`.
2. Numbered proof steps labeled `[USES: …]`.
3. One self-contained program (any language with exact rational arithmetic in
   its standard library — e.g. Python 3 `fractions`; single fenced block;
   deterministic; no network/file I/O; under 30 minutes one core) that
   implements a miniature evidence engine per W1–W6 (pluggable scheduler:
   batch partition + elimination-application policy + pause points; small
   declared world tables; the `E+` closed form
   `(Σ_{x=0}^{a} C(k+1,x)) / ((k+1)·C(k,a))`, `k = a+b`, as the evidence
   function) and:
   (a) for a counterexample: runs your two schedules and prints the divergent
   artifacts side by side;
   (b) for an invariance verdict: runs at least 200 distinct schedules
   (varied batch partitions including adversarial overshoots, pause points,
   thread-order permutations) on at least 3 world tables chosen to force
   same-batch settlement races, and verifies items 1–5 identical across all
   schedules per table;
   (c) for Question Q with a counterexample: computes the exact crossing
   probability over your finite tree as a rational and compares to the
   declared level;
   printing `PASS <check>` / `FAIL <check> <detail>` lines, exit 0 iff all
   checks pass (for counterexample claims, "pass" means the divergence or
   overspend is confirmed by the program).

A response whose program fails any of its own checks scores zero.
