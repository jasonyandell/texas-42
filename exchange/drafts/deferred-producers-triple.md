---
slug: deferred-producers-triple
channel: new-chat
status: "DRAFT — NOT DISPATCHED. No number assigned; awaiting Jason's explicit go under a future authorized batch."
deliverable: three-part design response — (1) theorem + proof of an anytime-valid upper confidence bound on a supremum of means, with validity conditions under the stated coupling and an enumeration-verifiable exact-rational worked example; (2) the weakest machine-checkable structural sufficient condition for zero hazard mass, with a worked example and an explicit non-coverage statement; (3) a motif alphabet as decidable predicates over the declared trace fields, with coverage/exclusivity analysis and per-motif aggregate typing
---
STATUS: DRAFT — NOT DISPATCHED. Authorization: none. Batch quota: TBD with Jason.

> **Internal note (removed at dispatch).** Thread: **[L2]**; Part 1 straddles
> into CE machinery via the sanctioned one-directional crossing (L2 consumes
> CE baselines; CE never consumes L2). Sources: `walt/probes/fieldswap_cancel/`
> `README.md` ("Deferred LOUDLY"), `kanban/backlog/slice3-deferred-producers.md`,
> `walt/CENSUS-RULINGS.md` (PANEL-A7/A8), inbox x:019–023 Part VI (§§31–42),
> `walt/walt/src/solver/exposure.rs` and `field_swap.rs`,
> `walt/math/targeted_level2_field_stability_v0.1.md` §7.4/§10.

You are doing adversarial design review for a games-mathematics project. You
see ONLY this text: no repository, no prior conversation, no outside sources.
Everything you need is defined below. The project deliberately deferred three
constructions rather than approximate them; each needs a design it does not
have, and this brief asks for all three. Unlike a pure refutation brief, the
project states its own positions and candidate solutions below — attack them,
improve them, or replace them, but engage them concretely. Your response will
be adjudicated mechanically by reviewers holding the full corpus: programs
executed, witnesses re-run, proofs step-checked. Hedged or unverifiable claims
score zero. An honest partial design with a complete proof outranks a full
design with a gap.

## 1. Setting (complete and self-contained)

### 1.1 Game model

A finite, perfect-recall, deterministic-transition extensive-form game with
one distinguished **focal seat** and one or more **non-focal seats**. A
**world** `ω` is the hidden initial state, drawn from a finite set with
rational probabilities `β` (the belief; uniform over a "fiber" of worlds in
current probes). Each seat sees its own private component of ω plus the
public record of all actions played (perfect public monitoring; no seat sees
another's private component). Focal information states are `I`, non-focal
are `J`. Play from a fixed root is fully deterministic given ω and one
action-choosing function per seat; the focal seat receives a Boolean payoff
`u ∈ {0,1}` ("make").

An information-consistent focal policy `ρ` maps focal information states to
legal actions; for a legal root action `a`, `Π_a` is the finite nonempty set
of such policies playing `a` at the root. A **field** `σ` maps every
non-focal information state to a legal action; two fields `σ_0, σ_1` are
fixed. `u_f(ρ,ω) ∈ {0,1}` is the payoff under `(ρ, σ_f)`; `V_f(ρ) =
E_{ω∼β}[u_f(ρ,ω)]`; `Q_a^(f) = max_{ρ∈Π_a} V_f(ρ)`.

The concrete game is Texas 42, straight points-and-marks: 4 seats in two
partnerships, 28 dominoes dealt 7 per seat, 7 tricks, follow-suit
constraints, a declared trump suit, and a bid threshold making the payoff
Boolean (contract made or not). Parts 1–2 deliverables may be stated over
the abstract model above; Part 3 additionally asks for the trick-taking
reading.

### 1.2 Coupling, exposure, and the cancellation ladder

For fixed `(ρ, ω)` run two executions from the same root, one per field,
with public histories `h_t^0, h_t^1` before action `t`. Define the stopping
time

    τ = inf{ t : h_t^0 = h_t^1, the actor is non-focal, σ_0(J_t) ≠ σ_1(J_t) },

with `τ = ∞` if no such `t` exists, and the field-exposure event
`D_ρ(ω) = 1{τ < ∞}`. The executions are coupled identically until τ, then
may fork. Known and not under review here (they were independently
certified): `|u_1(ρ,ω) − u_0(ρ,ω)| ≤ D_ρ(ω)` per world; with
`O_ρ = 1{u_1 ≠ u_0}`, `C_ρ^+ = 1{u_1=1, u_0=0}`, `C_ρ^- = 1{u_1=0, u_0=1}`,
and expectations
`d_ρ = E[D_ρ]`, `r_ρ = E[O_ρ]`, `c_ρ^± = E[C_ρ^±]`, `c_ρ = c_ρ^+ − c_ρ^-`,
the ladder `|c_ρ| ≤ r_ρ = c_ρ^+ + c_ρ^- ≤ d_ρ` holds with three distinct
zeros (behavioral irrelevance, terminal-outcome irrelevance, value
neutrality).

### 1.3 Pairwise masses and dominance

For two frozen policies `a, b` under one field: `B(a|b) = Pr(u_a=1, u_b=0)`
(benefit), `H(a|b) = Pr(u_a=0, u_b=1)` (hazard), `g = B − H`,
`q = B + H`. For Boolean payoff, `a` weakly dominates `b` almost surely iff
`H(a|b) = 0`; strictly in expected value if additionally `B(a|b) > 0`. The
project's adopted ruling admits the label `Dominated` only via **exact
enumeration of `H = 0 ∧ B > 0`, or a valid bound**; a finite sample with
zero observed hazards never proves `H = 0`. The sampled-masses type in the
implementation deliberately has NO dominance method — a compile-time lock,
not a convention — and only the exact-enumeration producer exists today.

### 1.4 Directional bounds, exposure rungs, and the screen

Directional root-action correction bounds:

    R_a^+ = max_{ρ∈Π_a} Pr(u_1=1, u_0=0),    R_a^- = max_{ρ∈Π_a} Pr(u_1=0, u_0=1),

giving `Q_a^(0) − R_a^- ≤ Q_a^(1) ≤ Q_a^(0) + R_a^+`. Maximum field exposure
is `R_a = max_{ρ∈Π_a} d_ρ`, with the rung ladder `R_a^± ≤ R_a^outcome ≤
R_a^exposure`. Exposure upper bounds come in rungs: **E0** exact field
equality (bound 0), **E1** counted structural covers, **E2** clairvoyant
split-reach (per-world: does ANY focal continuation reach the frontier —
"strategy fusion" is allowed per world, so it over-approximates), **E3**
information-consistent split-reach solve on a sampled route, **E4** the
exact split-reach solve over the whole fiber (`R_a` exactly).

Safe screening (the bar construction): given valid intervals `Q_a^(0) ∈
[L_a^(0), U_a^(0)]` and valid directional uppers `R_a^± ≤ (R_a^±)^U` for
every legal root action, set `L_a^(1) = L_a^(0) − (R_a^-)^U`, `U_a^(1) =
U_a^(0) + (R_a^+)^U`, `B = max_a L_a^(1)`, admissible set `A_1 = {a :
U_a^(1) ≥ B}`. Excluded actions cannot be `σ_1`-optimal, on the event all
input bounds are valid.

**The admissibility rule (load-bearing, binding on Part 1):** the screen
consumes ONLY (i) exact values, (ii) admissible upper bounds — meaning
bounds valid with an explicitly declared, ledgered error probability δ —
or (iii) valid structural over-approximations. A sampled lower witness to a
supremum is none of these. The implementation enforces this by type: the
existing sampled split-reach record is an ESTIMATE tier that cannot feed a
screen.

### 1.5 The evidence engine (what "δ-valid" means here)

The project's sampling instrument is an exact-rational, anytime-valid
e-process family over MEANS. For a Bernoulli stream `B_1, B_2, …` whose
conditional success probabilities obey `p ≤ c` (rational `c ∈ (0,1)`), the
per-observation factor

    L_r(B) = (r/c)^B ((1−r)/(1−c))^(1−B),    r ∈ [c,1],

has one-step conditional expectation `1 + (p−c)(r−c)/(c(1−c)) ≤ 1`; the
uniform mixture over `r ∈ [c,1]` of the running products is a nonnegative
supermartingale starting at 1 with the closed form (after `s` successes,
`f` failures, `R = (1−c)/c`)

    E>_{s,f}(c) = Σ_{i=0}^{s} C(s,i) R^i · i! f! / (i+f+1)!,

and Ville's inequality gives `Pr(sup_n E>_n ≥ 1/δ) ≤ δ` with arbitrary
peeking and data-dependent stopping. The exact mirror tests `p ≥ c`.
Inverting the mirror family over `c` yields an anytime-valid UPPER
confidence sequence for a mean. All error probabilities are drawn from
declared δ ledgers with explicit allocations; all arithmetic is exact
rational. Worlds arrive as a declared frozen stream `ω_1, ω_2, …` modeled
as i.i.d. draws from β; a stream prefix `0..n` at a declared epoch is part
of any claim's identity.

This engine has instruments for means and for differences routed through
per-world pivotal variables. It has NO instrument for a supremum. That gap
is Part 1.

### 1.6 House constraints (binding on all three parts)

- Exact integers and rationals only; no floating point in any correctness
  statement or deliverable program.
- All sampling risk lives under declared δ ledgers; "valid" always means
  "valid at a declared δ, anytime, under data-dependent stopping" — or
  deterministically valid with δ = 0.
- Deliverables must be machine-checkable. Reference programs (Python 3
  stdlib, `fractions`) are welcome; they will only ever be re-run as
  evidence, never imported as source.
- Results are consumed at exploratory tier until adjudicated.
- Vocabulary: if a design requires a checkable witness object, call it a
  **witness** or a **necessary outer profile** — never a bare
  "certificate"; that word is reserved against by project convention.
- Do not assume repo access; if something you need is not defined here,
  state the gap explicitly rather than inventing it.

---

## 2. Part 1 — a δ-valid admissible-upper E3 producer

### 2.1 The problem

Rung E3 must deliver an upper bound on `R_a = max_{ρ∈Π_a} d_ρ` (and, for
the directional rungs, on `R_a^± = max_{ρ∈Π_a} c_ρ^±`) from a sampled
world stream, admissible under §1.4's rule: valid at a declared δ. The
object is a **maximum over finitely many coupled branch values, each branch
value itself a mean over worlds**. Two plug-in routes both fail:

- **Explored-branch max.** Any concrete policy or finite family of policies
  you evaluate yields lower witnesses to the max; taking the max of
  estimates over an explored subset under-covers the supremum. A sampled
  lower witness to `R_a` is not an upper bound — this sentence is
  load-bearing in the parent mathematics.
- **Full-class empirical max.** The implemented estimate solves the
  split-reach objective exactly over the empirical measure on the sampled
  prefix — `max_{ρ∈Π_a} (1/n) Σ_i D_ρ(ω_i)` — which overfits the sample:
  its expectation is ≥ `R_a` (max of means ≤ mean of max), but with no
  declared-δ statement in either direction. On a committed specimen it read
  `1` on a 64-world prefix where exact `R_a` was `39/40` and `197/200` —
  informative, inadmissible.

Neither is a defect to patch by convention; the typing that keeps these out
of the screen is deliberate. What is missing is the theorem.

### 2.2 What we want

An **anytime-valid upper confidence bound on a maximum of finitely many
means**, meeting all of: exact-rational computability; validity at a
declared δ under data-dependent stopping on the declared stream; explicit
hypotheses about the coupling (all branch values are evaluated on the SAME
worlds — for each sampled ω the per-branch indicators are jointly
computable from one coupled execution tree, so the empirical branch means
are positively dependent by construction); and a stated relationship to the
rung ladder (a valid E3 upper may exceed E4's exact value; it must never be
claimed tighter than it is).

### 2.3 Our positions and candidate routes (engage these concretely)

**(0) Our baseline — the one δ-valid route we can already prove.** Compose
the E2 structural over-approximation with the mean instrument. Per world,
the clairvoyant indicator `D*_a(ω) = 1{some focal continuation in Π_a
reaches the frontier on ω}` is computable exactly by a per-world walk, and

    R_a = max_ρ E[D_ρ] ≤ E[max over the per-world fused choices] = E[D*_a],

where `E[D*_a]` IS a mean of a per-world Boolean. Apply the §1.5 upper
confidence sequence to the stream `D*_a(ω_1), D*_a(ω_2), …` at declared δ:
an anytime-valid upper bound on `E[D*_a]`, hence on `R_a`. This is
admissible today. Its defect is the fusion gap: `E[D*_a] − R_a` can be
large exactly where E3 was supposed to beat E2. Our question is not whether
this works (we believe it does — certify or refute it as stated, including
the composition of a deterministic over-approximation with a δ-valid mean
bound) but whether you can do materially better. Note the directional
analogues need care: the natural per-world fused over-approximation of
`c_ρ^±` requires running fused branches to decided terminals; state exactly
what per-world Boolean over-approximates `max_ρ C_ρ^±` validly.

**(a) Per-branch e-processes + union bound.** Decompose `Π_a` by a finite
branch structure (e.g. the information-consistent backward-induction
decomposition of the coupled public tree: at each focal information state a
max over actions, at each chance/partition level a β-weighted sum of
conditional means). Run one upper e-process per branch at `δ/m` over the
`m` branches; the max of valid uppers is a valid upper for the max. This is
valid but possibly loose twice over: the `δ/m` split, and the conditional
means' shrinking effective sample sizes down the tree. Questions we want
answered with proofs: is the m-way split the right price, or does the
positive coupling (shared worlds across branches) admit a strictly better
combination — e.g. a single mixture supermartingale over the branch family,
or an e-process whose one-step factor is built from the vector of branch
indicators? What exactly is `m` for the backward-induction decomposition —
per focal information state, per (public history, infoset) node, per
root-child — and which choice makes the union bound priceable?

**(b) A single e-process on the running max's exceedance events.** For a
candidate level `x`, the null `R_a ≤ x` implies `d_ρ ≤ x` for EVERY ρ,
hence every branch's exceedance is controlled simultaneously. Is there a
one-process construction — betting against exceedance of `x` by any branch,
inverted over `x` into an upper confidence sequence for the max — that
avoids the explicit δ split? If yes, state the supermartingale, its one-step
factor, and the inversion; if no, prove the obstruction (we suspect the
issue is that "some branch exceeds" is a union event and the process must
pay for it somewhere — show where, and whether the payment can be adaptive
in the observed branch dependence).

**(c) A route native to the coupled structure that we have not seen.**
The split-reach objective is an optimal-control value with Boolean payoff
over a coupled tree, not an arbitrary finite max: branches share prefixes,
the frontier is an absorbing event on the common prefix, and `D_ρ` is
monotone in a precise sense (more frontier states, more exposure). If any
of that structure buys a tighter admissible upper than (0)/(a)/(b) — e.g.
an optimistic per-node bound propagated by backward induction with
per-node confidence sequences and a proof that the propagated root bound is
anytime-valid at the summed δ — construct it. This shape (optimism over a
tree of conditional means) is well-trodden in other fields; what we need is
the exact-rational, anytime-valid, ledger-compatible statement with proof,
not folklore.

### 2.4 Required deliverable for Part 1

The theorem with proof, for at least one route strictly better than (0) or
a proof that none of (a)–(c) can beat (0)'s composition in the stated
regime; its exact validity conditions under the τ coupling and the i.i.d.
declared-stream model (state which hypotheses are load-bearing: finiteness,
common worlds across branches, Boolean payoffs, predictable stopping); and
a finite worked example — small enough that we can re-verify every number
by exhaustive enumeration, all quantities exact rationals — in which your
producer's upper bound at a declared δ is computed step by step alongside
exact `R_a` (and `R_a^±` if your construction covers the directional
rungs), demonstrating both validity and the claimed tightness advantage.

---

## 3. Part 2 — the dominance valid-bound route

### 3.1 The problem

§1.3's ruling admits `Dominated` via exact enumeration OR a valid bound.
Exact enumeration exists and is the only producer. The valid-bound route
was deliberately not stubbed: a structural hazard-bound type with no
producer behind it invites misuse, and the type-level lock (sampled masses
cannot claim dominance at all) is only sound while no half-built bound
route sits beside it. The design question: what may a valid bound on
`H(a|b)` legitimately BE?

### 3.2 Our position

A valid hazard bound must be **structural** — proven from game structure
over ALL information-consistent continuations, never estimated. Sampling is
categorically excluded for this label: `H = 0` is a probability-zero claim,
and no δ-ledgered instrument certifies an exact zero (a confidence upper
bound `H ≤ ε` with `ε > 0` does not meet the `Dominated` definition, and we
do not want an "approximately dominated" label — it would blur a typed
distinction the vocabulary exists to keep). So the route we seek is a
rules-level argument that **no information-consistent continuation reaches
a terminal where the hazard event occurs** — the shape of the motivating
example: with the highest remaining trump in hand against a vulnerable
counting double, playing the high trump is never strictly worse, because no
continuation constructs a world-and-line where holding it back wins a world
that playing it loses. Deterministic, δ = 0, checkable per root.

### 3.3 What we want

The **weakest structurally-checkable sufficient condition** for
`H(a|b) = 0` (with `a, b` frozen policies differing at a declared point,
under one declared field) that is machine-verifiable per root without full
fiber enumeration. Concretely:

1. State the condition as a decision procedure over the game structure
   (root position, legal-play constraints, trump order, count placement,
   the two policies, the field) — either a direct decidable predicate, or
   a checkable **witness** object (a witness or necessary outer profile,
   per §1.6 vocabulary) whose verification is cheap even if its discovery
   is not. A per-world pointwise mapping argument (an injection or
   identity on worlds with `u_b = 1 ⇒ u_a = 1`, proven from a local
   exchange/dominance argument on the play tree rather than by evaluating
   every world) is the shape we suspect is right — but we have not proven
   that any local condition suffices, and trick-taking games are rich in
   counterexamples to naive exchange arguments (a "higher" tile can lose a
   later trick's count by winning an earlier trick). Be precise about what
   breaks naive exchange arguments and what hypothesis restores them.
2. Prove soundness: condition holds ⇒ `H(a|b) = 0` over the full fiber.
3. Give a worked example in the high-trump shape above, small enough to
   re-verify by enumeration, where the procedure certifies `H = 0` (and
   exhibits `B > 0` by any admissible means, enumeration included).
4. State explicitly what the condition does NOT cover — a non-coverage
   statement with at least one concrete instance where `H(a|b) = 0` is
   true but your condition fails to certify it, so the type stays honest
   about its reach. Completeness is not required; honesty about
   incompleteness is.
5. State whether the condition composes: if `a` dominates `b` under both
   fields σ_0 and σ_1 separately, that is two independent runs of the
   procedure — or is there a coupled statement worth having?

We explicitly do not want: any route through sampled hazards; any secondary
objective smuggled in (choosing among non-dominated exact ties by variance,
robustness, or convention is a declared-separately decision, not part of
this label); any weakening of `Dominated` to a δ-qualified cousin.

---

## 4. Part 3 — the motif vocabulary over first-split traces

### 4.1 The problem

For every world whose terminal outcome changed between fields, the
implementation persists a `FieldSplitTrace` — the per-world explanation
record. Its fields today (this is the actual struct, stated faithfully):

1. `root_id` — root information-state identity;
2. `world` — canonical world identity (four hand bitmasks, seat-indexed);
3. `action` — the focal root action (a domino);
4. `policy` — the frozen focal policy identity;
5. `field0`, `field1` — the two field identities;
6. `split` — the first field split: acting non-focal seat; 1-based trick
   number; 0-based ply within the trick; σ_0's chosen tile; σ_1's chosen
   tile; the acting seat's private hand at the split; the common public
   record since the root (which, with the root frame, is the complete
   modeled information-state key);
7. `u0`, `u1` — terminal make indicators under each field; the derived
   sign `favors = u1 − u0 ∈ {−1, +1}` on these pivotal traces.

Aggregates shipped today: exposed mass, positive/negative correction
masses, first-split seat histogram, first-split trick histogram, and the
conditional outcome difference `(c^+ − c^-)/d`. The parent design lists one
more item the trace does not carry: **structural motif tags** — a
vocabulary such that aggregates can say what KIND of divergence happened
("reveal-response": one field's tile reveals information the other's
retains, and the downstream response differs), not merely where. It was
deferred loudly: naming motifs before the vocabulary is designed would make
aggregates readable and wrong.

### 4.2 Our position

This is ubiquitous-language design, and the failure mode is seductive
labels. Binding design constraints we have already committed to:

- A motif must be a **derived predicate over the recorded trace fields**
  (items 1–7 above) — a total computable function of data already
  persisted, never a new judgment call at record time, and never a second
  stored authority beside the trace.
- The alphabet should be small; motifs mutually exclusive where possible
  (or explicitly layered: a primary partition plus optional orthogonal
  flags — if you propose layering, say which layer is the partition);
- An explicit `Other`/untagged residual so coverage is honest — a trace
  the predicates do not confidently classify must land in the residual,
  never in the nearest readable bucket.

One field we suspect the vocabulary needs and the trace may not carry: the
public observation that distinguishes the relevant downstream branches
(the parent design's own list has such an item; the current struct carries
it only implicitly, as the pair of chosen tiles plus the public record).
If your predicates need trace enrichment — e.g. the first post-split
decision of the OTHER non-focal seat, or the delta in the focal seat's
information state after the split — say so explicitly as a schema change
request with the predicate that motivates it; do not assume unrecorded
data.

### 4.3 What we want

1. A proposed motif alphabet, each motif a **formal predicate** over the
   trace fields (pseudocode or Python over a dict with exactly the §4.1
   fields), each decidable, with the residual explicit.
2. A coverage and exclusivity analysis: which pairs of motifs can co-occur
   and why that is either impossible (proof from the predicates) or
   intended (layering); what fraction of traces you would EXPECT to land
   in the residual for a trick-taking game of this shape, and what a
   high residual rate would indicate.
3. The trick-taking-theoretic reading of each motif — one paragraph each,
   grounded in the model (e.g. what "reveal-response" means in terms of
   the acting seat's information state, the tiles' suit/count structure,
   and the downstream fork), so a human report can use the label without
   lying.
4. A typing of per-motif aggregate statistics: which aggregates over
   motif-partitioned traces would be **evidence-bearing** later (exact
   masses per motif over an enumerated fiber; conditional outcome
   differences per motif) versus merely **descriptive** (histograms over
   sampled prefixes, readable but never screen-feeding) — and any
   aggregate you would refuse to publish per motif because the partition
   invites a false causal reading (the traces record the first mechanical
   divergence, which is not per se the cause of the outcome change; say
   how the vocabulary should guard that distinction).

---

## 5. DELIVERABLE CONTRACT

End your response with a section titled exactly `MACHINE-CHECKABLE
ARTIFACTS` containing:

1. One line per part:
   `PART 1 ANSWER: CONSTRUCTION (<route>)` or `PART 1 ANSWER: BASELINE (0)
   OPTIMAL (<proof scope>)` or `PART 1 ANSWER: PARTIAL (<scope>)`;
   `PART 2 ANSWER: CONDITION (<name>)` or `PART 2 ANSWER: IMPOSSIBILITY
   (<scope>)` or `PART 2 ANSWER: PARTIAL (<scope>)`;
   `PART 3 ANSWER: ALPHABET (<k> motifs + residual)` or
   `PART 3 ANSWER: PARTIAL (<scope>)`.
2. Numbered proof steps labeled `[USES: …]` for every theorem claimed in
   Parts 1–2, with the minimal hypothesis list stated per theorem.
3. One self-contained deterministic program (Python 3, standard library
   only, exact `fractions.Fraction` arithmetic, no network or file I/O,
   under 60 minutes on one core; a single fenced block, or one block per
   part) that:
   (a) for Part 1: implements a small game family per §1.1 (declare it
   precisely; worlds with 2–8 states, 2–3 seats' worth of decision
   structure suffices), computes exact `R_a` (and `R_a^±` if covered) by
   enumeration, runs your producer on declared deterministic streams at a
   declared δ, and checks validity (bound ≥ exact value on every run under
   the null construction) and reports tightness against the (0) baseline;
   (b) for Part 2: implements your decision procedure on the worked
   example, certifies `H = 0 ∧ B > 0`, re-verifies by exhaustive
   enumeration that they agree, and demonstrates the non-coverage instance
   (procedure declines; enumeration shows `H = 0` anyway);
   (c) for Part 3: implements every motif predicate over a set of at least
   20 hand-constructed trace dicts (using exactly the §4.1 fields),
   demonstrating the partition property you claim (each trace gets exactly
   one primary label, residual included) and printing the per-motif
   counts;
   printing `PASS <check>` / `FAIL <check> <detail>` lines, exit 0 iff all
   pass.

A response whose program fails any of its own checks scores zero on that
part; the three parts are scored independently — a complete answer to one
part with honest silence on the others is a valuable response. "Design X
is impossible under constraints Y, here is the proof" is a high-value
outcome for any part.
