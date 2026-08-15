# walt — the decision-sparse program

[Home](Home.md) · owns: the decision-sparse exact-solving architecture — its thesis, its objects, its theorem
inventory, its audit history, and the standing state of its experiment program · Sources:
`walt/math/decision_sparse_exact_solving_v0.1.md` (the received parent, verbatim) and
`walt/math/decision_sparse_exact_solving_v0.1_errata.md` (the durable repaired mathematics, DS-A17);
`walt/CENSUS-RULINGS.md` (DS-A1..DS-A36, SEP-A1..SEP-A19, J-A1..J-A18, PG-A1..PG-A18, R-A1..R-A24,
N4-A1..N4-A20, EC-A1..EC-A14, RW-A1..RW-A8, **FT-A1..FT-A29**, **SR-A1..SR-A37**, **FF-A1..FF-A33**, **FC-A1..FC-A22**);
`walt/SEPARATION-PROBE.md`, `walt/DEADNESS-PROBE.md`; `walt/walt-factory/results/deadness_2026-08-12.txt`,
`separation_2026-08-13.txt`, `separation_n4_2026-08-14.txt`, `rule_economy_n4_2026-08-14.txt`,
`fusion_tax_2026-08-14.txt`, `second_rung_2026-08-14.txt`, `feature_fee_2026-08-14.txt`, its corrected
re-run `feature_fee_v11_2026-08-14.txt`, and `fc_correlation_2026-08-14.txt`; and the received
`exchange/inbox/016-decision-sparse-nonanticipativity-taxes.md` and `017-second-rung-gluing.md`
(both adjudicated, neither imported as an axiom). Related: [walt](walt.md) (hub), [walt-s6-era](walt-s6-era.md) (the
sessions that produced
the evidence), [walt-math-reference](walt-math-reference.md) and
[walt-math-decision-sparse](walt-math-decision-sparse.md) (formal statements and proof provenance),
[walt-census-era](walt-census-era.md), [walt-instruments](walt-instruments.md), [walt-scheme-fix](walt-scheme-fix.md),
[walt-foundation-era](walt-foundation-era.md), [walt-factory-era](walt-factory-era.md).

> **Epistemic tier: EXPLORATORY — below every tier on
> [Home](Home.md#evidentiary-tiers--never-promoted-never-blurred).** The parent document, its errata, every theorem
> named here and every measured number cited here sit on walt's own exploratory basis. Nothing here is a corpus
> status, a kernel proof, an exchange-adjudicated result, or a rob receipt, and nothing here may be quoted in a brief,
> a dispatch, `FINDINGS.md`, or any claim-tier page. The theorems are prose proofs adjudicated by walt-math; the
> measurements are one Rust implementation on fabricated void-free capacity fibers whose members are **feasible and
> never reachable**.

## The thesis

Two compression programs asked for a compact representation of **all future distinctions**, and both ended in
negatives: the first-play structural quotient is the identity (Corollary S-rigid, 1,184,040 = C(28,7) classes on the
opening carrier), and the universal linear value closure saturates to |X| by grade three (Gate B refuted, S6a). The
decision-sparse reframe is that exact action choice needs **less than either program was buying**:

> the opening truth may be high-dimensional while the opening decision is sparse.

The target moves from *compressing truth* to **proving the root action**. For each root action a, construct a lawful
lower bound L_a and a valid relaxed upper bound U_a with L_a ≤ Q^H(a) ≤ U_a. The opening play is then proved without
solving every action exactly, as soon as some a⋆ satisfies

> **L_{a⋆} ≥ U_a for every a ≠ a⋆.**

That is the **root-action separation**, and it is the whole architecture in one inequality. Walt calls the resulting
object a *certification of the root action*; DS-A1 bars "certificate" as a name for it going forward — the ruling
binds forward and not retroactively, so older walt artifacts still contain the word (walt's own §16.11 lesson record
type keeps it legitimately) — and where the parent uses it the quotation is bracketed as the parent's word. This is a
distinct sense from D3's
**necessary outer profile** in the reachability discussion, and the two senses never blur: nothing in this program is
identity-bearing and nothing asserts that any state arises in play.

The proposed architecture around the inequality is a small lawful **candidate-policy set**, exact Scheme circuits for
candidate and upper-bound outcomes, exact weighted integration over the opening fiber, and a small set of
information-gluing or dominance arguments. **Whether any of those four is small is the experimental question, not a
corollary** — Theorem E6.5's termination bound is |R_0(a)|, doubly exponential, and must never be read as a
complexity claim.

## The objects

**The primal witness L_a** (Lemma E4). The exact value of a **fixed** lawful information-consistent policy with root
action a, integrated under the declared belief and field with **no maximisation anywhere below the root**, then
maximised over a finite declared candidate set. Always L_a ≤ Q^H(a). The binding invariant is semantic (DS-A27):
every later focal action is supplied by the candidate policy, and no optimiser selects a focal action using
hidden-state information. "No max node below the root" is an accepted **sufficient implementation form** — a receipt
for the invariant, not the invariant — and it is asserted structurally, never by inspection.

**The upper witness U_a** (Lemma E3). The action-conditioned treatment-C value E_β[V*_a]: the root action is held at
a, each world is then solved revealed, the field untouched, and the values averaged over the full fiber. Always
Q^H(a) ≤ U_a. It must be **action-conditioned**: the unconditioned world-informed aggregate U^agg is a valid upper
witness for every action but does not depend on a, so the separation test degenerates into demanding that one lawful
policy match the full strategy-fusion bound — sound and, in general, vacuous (Remark E3.1).

**The four attached conditions** (§3.4) hold jointly for the pair or the separation is void: **(C1)** the same fixed
field on both sides; **(C2)** the same belief and the *same world set* on both sides — in particular **no decimation
inside L or U**, since a sampled mean is neither bound (decimation remains lawful for choosing *which* coordinates to
certify); **(C3)** the same utility and count contract; **(C4)** perfect-information minimax is not a substitute, being
a different operator.

**The candidate library.** A cache of primal-witness sources keyed by (grade, base index, declaration, root action),
whose body is a total map from observation record to chosen tile over the information partition. Frozen at freeze 36
(SEP-A4) with four disciplines that matter: **no values, no verdicts, no ranks** — the file is a cache and never an
authority, and a loaded entry is re-priced before anything is reported; **identity transport only in v1**, opened at v2
to the declaration fold and nothing further (EC-A8);
every entry carries its frame (observation contract, field, belief, |X|, freeze digest) and a digest mismatch means
**corrupt, not stale**, discarding the entry entire; and entries remain valid primal-witness sources under count
re-entry even though their count-free quality verdicts do not (DS-A16).

**Where deadness fits.** Decision-deadness (every information-consistent policy from a node has the identical value
function on the node's fiber) and universal dominance are **different objects** and are never presented as one
(J-A1, DS-A8). Deadness collapses max nodes; dominance prunes. Both are reduction sources feeding the candidate
library, and the detector family of [walt-s6-era](walt-s6-era.md#s6c--2026-08-13-the-decision-deadness-probe) is the branch's measured instance.

## Theorem inventory

Formal statements, hypotheses and proof provenance are owned by the mathematics pages —
[walt-math-reference](walt-math-reference.md) is the map, and the E-series itself lives on
[walt-math-decision-sparse](walt-math-decision-sparse.md) with the detector mathematics on
[walt-math-deadness](walt-math-deadness.md). This table is the index of names and what each one buys. **Citation rule, binding on every design and results file (DS-A17):**
cite the **errata theorem number** for the mathematics and the **DS-A ruling** for its provenance, and where parent
and errata differ, **the errata governs**.

| Name | One-line statement | Repairs / status |
|---|---|---|
| **Theorem E1** | Order exchange holds under a *declared involution* Θ intertwining the two kernels, over a Θ-closed policy class — not under bare commutation. | Replaces parent §7.1, which is **UNSOUND as written** (DS-A6). |
| **Theorem E1′** | The same, under a fully transported involution (E1 is the Θ_X = Θ_M = id case). | New (DS-A19). |
| **Definition E2 / Prop. E2.1** | The advantage dimension d_adv, redefined affinely; the reference-based form was ill-defined, with an exact off-by-one. | Replaces parent §5's d_adv (DS-A3/DS-A4). |
| **Lemma E3 / Remark E3.1** | The action-conditioned upper witness is valid pointwise by strategy fusion; the unconditioned aggregate is valid but action-constant, hence generally vacuous. | Replaces the informal parent §8.2/§15.4 (DS-A7). |
| **Corollary E3.2** | If the global fusion gap is zero then U_{a⋆} = Q^H(a⋆) = V^H for every H-optimal a⋆, and U_a ≤ V^H for every a — so the whole remaining difficulty is on the **primal** side. | New (DS-A22). Three non-implications travel with it. |
| **Lemma E4** | A fixed lawful policy priced by a fixed-policy evaluator is a valid lower bound. | Formalises parent §8.1/§15.3 (DS-A14). |
| **Non-theorem E4′** | Pricing a candidate with any world-informed evaluator yields an **upper** bound; installing it as L inverts the sandwich and can certify a strictly worse action while every displayed step typechecks. A two-world witness shows the failure is real. | The soundness failure mode, stated as a non-theorem (DS-A14). |
| **Corollary E4.1** | With H-argmax seeds, L = Q^H necessarily — the primal witness is at its ceiling, verdicts are decided entirely by the U side, and a NOT-SEPARATED pair proves **no candidate set** separates it under relaxation C. | Ruled at SEP-A2, pending errata filing as §4.3. |
| **Proposition E5** (+ E5.0, E5.1, E5.2) | Under the parent §10.3 hypotheses at every reachable interface plus latent separation (S), the algebra's atoms are singletons: dim SF = \|X\|. Atom-mass **linear** filtering is noncompressive on this carrier. | Parent §10.3 is SOUND but **degenerate here** (DS-A5); E5.0 shows (S) holds on the measured carrier (DS-A20). |
| **Theorem E6.1** | Width monotonicity on belief families typed on a common world space; W_reach ≤ W_all, and at the full simplex W_all = N_exp — one object, never two rows. | Parent §4.3 with typing made explicit (DS-A2). |
| **Theorem E6.2** | Backward Pareto pruning is exact through positive composition (frontier reproduction), the incremental fold is **required** for feasibility, and pruning destroys the distinct-vector count. | Parent §6.3 completed (DS-A4, narrowed at DS-A26). |
| **Theorem E6.3** | The value sandwich: nested candidate ⊆ lawful ⊆ relaxed sets under one α-map, one belief, one world set give L_a ≤ Q^H(a) ≤ U_a, and equal endpoints trap the middle. | Parent §8.3 with hypotheses inside the statement (DS-A7). |
| **Theorem E6.4** | Root-action separation: L_{a⋆} ≥ U_a for all a ≠ a⋆ implies a⋆ ∈ Opt^H(B); strict inequalities give uniqueness. **Member-not-set** is in the statement, not the commentary. | Parent §8.4 (the parent's "root-action [certificate]"). |
| **Theorem E6.5** | Finite adaptive gluing terminates and its upper values decrease monotonically to Q^H — under **(G1)** each relaxed solve returning a *proved* bound and **(G2)** the exposed-face stopping test being a search over the whole optimal face. | Parent §9.2 with two obligations (DS-A3). |
| **Lemma E7** | Dominance travels only under an exhibited value-order isomorphism; a policy transport establishes lawfulness (all a primal witness needs), while transporting a **verdict** additionally needs the isomorphism and the transported belief. | New (DS-A25); narrows "dominance never travels". |
| **Definition E9** | Interface-local reachable decision width, superseding the root-level W_reach for operational use. | New (DS-A23). |
| **Lemma E8 / Lemma J(c′)** | Under J-0's hypothesis the value is identical for every lawful policy for every valuation whose tile-value schedule is **constant on the focal hand**; ordinary Straight count under the guard is the zero case. | New (DS-A24); **corrects DS-A9**, whose "every cone at once" gloss is false in fixed physical coordinates. |
| Parent §4.2, §6.2, §12.1 | Envelope sufficiency (definitional), pointwise dominance, combined factorization. | Audited **SOUND**, untouched. |

## Audit history

The parent arrived on 2026-08-13 as a received mathematical handoff and, like an ingest package, **stays verbatim**
(DS-A18): no correction is ever written into it, because a corrected source destroys the record of what was
corrected. The repairs live beside it in the errata, which is walt-math's own document and **is maintained** —
in-place amendments carry dated provenance markers naming the ruling, and genuinely new mathematics is added as a new
section rather than grafted into an old one. If a v0.2 parent is ever filed, the errata is re-audited against it and
never silently inherited.

**Intake audit, DS-A1..DS-A18.** Vocabulary fixed (DS-A1: receipt, witness, primal/upper, root-action separation; no
"certificate"), the decision-width and advantage-dimension terms typed (DS-A2, DS-A3), the eleven parent theorems
audited — nine sound once implicit hypotheses are explicit, one sound but incomplete for its use, one unsound as
written — treatment C established as an upper witness with the action-conditioning requirement (DS-A7), deadness
separated from dominance (DS-A8), the candidate library's fatal naive failure stated **first** because it is a
soundness failure (DS-A14: every L is produced by a fixed-policy evaluator, asserted structurally), transport limits
fixed (DS-A15), the one robust bankable property recorded (DS-A16), and the errata named as the durable home with its
citation rule (DS-A17).

**Second audit, DS-A19..DS-A28.** An external reviewer's nine proposed amendments were adjudicated, and **all nine
were accepted** — three as new mathematics (Theorem E1′, Corollary E3.2, Lemma E7), one as a new definition
(Definition E9), one as a correction of a walt ruling (Lemma E8 / Lemma J(c′), correcting DS-A9's cone clause), and
four as in-place narrowings (treatment C reveals ω while C⁺ reveals (ω, z); the evaluator invariant restated
semantically; filtering and rank language narrowed to exact scope; the pruning claim narrowed to the two rules
actually in use). DS-A28 fixed how a superseded ruling is handled: the rulings file is the append-only adjudication
record, and a superseded ruling is marked, never rewritten.

**Runner discipline, DS-A29..DS-A36.** Written when the S6c sequential run was killed twice. Preconditions asserted
in-run rather than assumed (every stop criterion a deterministic count, never wall-clock; no clock, RNG or
environment value entering any decision; exact rational arithmetic throughout, so every reported count is
execution-schedule-invariant); checkpoints frozen with a digest and a store discipline where a mismatch is corrupt
rather than stale; a RESUMED provenance line and three companions; **parallel timings recordable but never quotable,
with the bias direction named** (DS-A32); a sequential rung declared as the only quotable timing instrument
(DS-A33); W recorded but not frozen; single process only; and a deterministic results block that makes a byte-diff
receipt free (DS-A36).

## The experiment program and where it stands

| | Experiment | Status |
|---|---|---|
| **A** | Complete the deadness run — forced / dead / dominant-not-dead nodes, D0 and D1 hits, false positives (which must be zero), detector cost, solve-cost dividend. | **COMPLETE** via S6c: 174,250,255 detector calls, 27,980,333 fires, zero false positives, ~33% recall against a tie denominator that understates it. |
| **B** | Tense-root anatomy on the two stopped non-boss trump roots: first frontier split, complete frontier at the smallest split, pairwise advantage sign sets, d_adv, reachable posteriors, W_reach, minimal Scheme separators. | **QUEUED**, designable now under DS-A10's two clauses: a frontier may be quoted only where the computation *completed*, and the synthesised separators are instruments below every tier, with every failed separator recorded as a counterexample pair. |
| **C** | Scheme-weighted filtering on an enumerable fiber: belief, one likelihood, one posterior, one fixed-policy value function, bit-exact against enumeration. | **NOT YET DESIGNED.** DS-A11 bars designing it against §10.3's atom formulation (Proposition E5 makes that degenerate); its lawful form measures **circuit size against world count** for a *declared purpose*, states that no algebra-compression claim is made, and notes that exact normalised filtering re-enters Lemma R(c). So stated it is the branch's first real attempt at Gate D — and should say so. |
| **D** | Adaptive gluing on one tense grade-3 root: relaxed solve, exposed-face test, emit and generalise cuts, re-solve, stop at exactness. | **BOTH RUNGS RUN AT GRADE 4; THE CARRIER IS NOW EXHAUSTED.** Its two blockers are gone: **freeze 38 is filled** (v1, scoped, FT-A17), and S6h's four exact negatives supplied the failing pairs it consumes. S6k computed rung one over the five-coordinate carrier those pairs sit in and closed one pair; S6l computed rung two exactly at two of those coordinates. At grade 4 there is no rung three, and Proposition SR-degen bars grade 4 from testing closure at all — so **progress now requires a longer ladder**, not a further cut here. |
| **E** | Lower/upper root-action certification on a tractable multi-action root. | **COMPLETE** via S6d: all three grade-3 coordinates SEPARATED; seven of nine per-action prices exactly 0; the two nonzero prices at the two leads where S6b's frontier exploded. Extended one grade by S6h at all nine n = 4 coordinates: 4 SEPARATED, **4 exact negatives**, one NOT PRICED ([walt-s6-era](walt-s6-era.md#s6h--2026-08-14-overnight-the-n--4-separation-pass)). |
| **F** | Reachable envelope: compare N_pol, N_par, W_all, W_reach, d_adv without conflating them. | Designable now under DS-A10 with the DS-A2 ladder; W_reach needs exact enumeration of reachable posteriors, feasible only at the small grades, and a stop is a stop. |
| **G** | Count and score lift — the valuation cone, the gauge, which count-free verdicts are re-derived versus inherited. | **NEEDS ITS OWN ADJUDICATION** (DS-A12): this is where E-A2 bites hardest and it touches Jason's binding count constraint. Propositions J-0 and J-1 survive count re-entry; J-win does not; every form-keyed record is void wholesale, never extended. |
| **H** | Grade climb, measuring truth size, circuit size, active policy size and constraint size independently. | **PREMATURE** by its own terms (DS-A12), and governed by P-A21 when it comes: three rungs are not a law, and no growth rate measured at grades ≤ 4 is quoted for the opening. |

## The named next step

**The economy-claim successor (SEP-A17).** S6d proved a root action while computing the exact H solve at every
action, because DS-A10's receipts require it. It therefore did **not** test the parent's economy claim — "the solver
does not need an exact solution for every action" — and is never cited as if it had. The experiment that would test
it is: **seed L from a source that is not an exact solve at a⋆** — a transported library entry, a hand-authored
playbook, a cheap heuristic — and ask whether the sandwich still closes.

**Status, since EC-A1..EC-A14 (2026-08-13): designed and adjudicated; the gates are open.** The three things that
gated it have been answered. Freeze 36's transport clause is **opened at freeze 36 v2** (EC-A8) to identity plus the
declaration fold φ, with values licensed by **Corollary S-fold-val** and verdict transport by Lemma E7 with
β′ = T_*β; any further transport re-enters with its own adjudication. The arm list is fixed and **CLOSED** at
**freeze 46**: an exact control X, the transport arm T, four fixed tile rules P1–P4 (least-tile, greatest-tile,
beat-if-able, trump-hoard), and a heuristic re-key R that carries the label **HEURISTIC RE-KEY (NOT A TRANSPORT)** on
every row. The thinness of the harvest — six of seven completed S6b singleton roots collapse by indifference — is
answered by those non-exact arms rather than by waiting for something to transport.

**What it does and does not settle (EC-A13, binding).** It tests the **primal half**: whether the *witness* at a⋆
must be an exact solve. The **full** parent sentence — a solver that avoids exact solves — additionally requires the
U side cheapened, a relaxation coarser than C run down Theorem E6.5's ladder. That is Experiment D's territory, and
since S6k it is no longer closed off: freeze 38 is filled and the ladder's first rung has been computed exactly at
five coordinates. **The full claim is still untested**, because the S6k run itself prices U exactly and imports
exact Q^H values for its receipts. **A results file or wiki sentence saying "the economy claim was tested" without
the word *primal* has over-claimed.**

## The fusion-tax chapter (S6k, 2026-08-14)

Every separation up to this point had been won on the primal side: the upper witness was always treatment C,
computed exactly, and the only question was whether some candidate could reach it. Corollary E4.1(3) then proved
that at four coordinates **none ever can** — which leaves cheapening the upper side as the only remaining move, and
that half of the architecture had gone untouched since the intake audit reserved a freeze number for it. This
chapter is where it moved, and it moved because of an external note plus one night's build.

**Where it came from, and what that does and does not confer.** `exchange/inbox/016` — a received note titled
*Nonanticipativity Taxes and a Compositional Plan Calculus for Straight Texas 42* — arrived hand-ferried and
UNADJUDICATED, answering walt's own outbox request for a cheaper upper witness. The note self-classifies its claims;
**those labels are the sender's and carried no status here until adjudicated**, and an external note is never
imported as an axiom. Adjudication (FT-A1..FT-A29) accepted it **in large part**: the central identity and the
first-layer mathematics are correct, three "exact results" needed a hypothesis named that the note never states, one
prose claim was rejected and replaced by a stronger theorem, and two schema families carry obligations no probe may
skip. **No number from the note entered as evidence** — the four decimals its experiment section quotes are rounded
forms of walt's own exact rationals, re-derived here from the filed rows.

**The fusion-gap identity is the note's genuine contribution.** The gap U_a^C − Q^H(a) equals the **minimum, over
lawful policies, of the expected clairvoyance regret** — the expected amount by which the world-informed value of
holding root action a exceeds what that fixed lawful policy actually achieves. Its value is what it does to the
work: it converts "tighten the relaxation", an open-ended instruction, into "prove a lower bound on one
expectation", a finite obligation with a named object. Subtract a **proved** lower bound Γ_a from U_a^C and the
result is still a valid upper witness, so the separation test becomes Γ_a ≥ U_a^C − L_{a⋆}. A Γ that is *measured*
rather than proved — a tax computed by a run whose budget stopped — is not a Γ: freeze 44's contract already says a
stopped walk retains no partial fold, and **there is no partially-computed tax.**

**The reveal-delay ladder, and how much shorter it is than the note thought.** Between treatment C (the world
revealed before every focal decision) and treatment H (never revealed) sits a ladder of relaxations C^(k) that
withhold the world through the first k focal decisions and reveal it below. Each rung is a valid upper witness, and
the rungs are nested. **Lemma FT-trunc** sharpens where the ladder ends: a suffix of *forced* decisions truncates
it, and since the focal seat's last decision is over a one-tile hand it is always forced, so U^(N−1) = Q^H already
and the last tax is identically zero. **Corollary FT-grade4** is the operational payoff: at an n = 4 coordinate the
seat holds four tiles and leads, so the ladder has **exactly two rungs**, and
U_a^C − Q^H(a) = Δ_a^(1) + Δ_a^(2) with both endpoints already filed. **One computation of U^(1) therefore
determines the entire layer decomposition** — (Δ¹, Δ²) is complete at this carrier, not a prefix of something
longer, and the note's proposed follow-up experiment is vacuous here and was not commissioned.

**Three results a builder needs before using any of it — two fences and one licence.**

- **Proposition FT-flat — the upper twin of T1-blind.** If an upper feature does not depend on the *frontier*
  action, the bound it returns is **at least U_a^C**: the max drops out, and what is left dominates the witness
  already filed. So it can never improve on it — not "is unlikely to be selective enough", which is what the
  received note said, but **never**. Read with S6g's Proposition T1-blind, the two close the sandwich from both
  sides with one lesson: **a witness must be conditioned on the decision it is trying to price.** Note the scope
  exactly — FT-flat constrains dependence on the *frontier* action, not the root action, and U_a stays
  action-conditioned through the frontier masses regardless.
- **Lemma FT-post — the composition trap, and the sharpest one in the received note.** The belief at a frontier
  information state is **not uniform over its latent worlds**: arrival weights are products of the field's per-world
  legal-set sizes, and those sizes vary across the fiber because they are computed from the moving seat's own hand.
  So a residual witness priced as a **fresh coordinate under the branch's standard uniform belief** — which is
  exactly how every walt coordinate is built — prices the wrong measure, and the composition is **void, not merely
  loose**: neither an upper nor a lower bound on the residual value. Two forms compose and nothing else does:
  evaluate the continuation **inside the same walk** under the carried weights, or prove the residual bound
  **pointwise in every world** of the leaf, where the measure is irrelevant. Any artifact that pastes a residual
  witness prints which of the two it used, on the row.
- **Lemma FT-mix — heterogeneous upper witnesses compose.** If each competitor has *some* valid upper witness — and
  they need not come from one relaxation, one evaluator, one traversal or one run — and L_{a⋆} beats every one of
  them, then a⋆ is optimal, with uniqueness when every inequality is strict. Theorem E6.4's proof never required one
  relaxation; the lemma is stated so that a **mixed** proof is licensed by a stated result and never read as
  stretching E6.4 past what E6.4 proves. Its content is not the one-line proof but the four evidentiary conditions
  attached to it (below), which is where a composed verdict can actually go wrong.

**Freeze 38 is filled** (v1, scoped) — the gluing-cut language, the validity obligation and the cut ordering that
DS-A13 reserved the number for in the intake audit. What made it fillable is a **typing**: a block merge identifies
action variables inside one information state, removing no world and changing no world's mass, which is what
separates a cut in Theorem E6.5's sense from a declared exclusion remnant. Full clauses on
[the freeze register](walt-math-freezes.md#freeze-38--the-gluing-cut-v1-scoped-ft-a17).

### The run: twelve binding pairs, one closure

Twelve binding (a⋆, competitor) pairs over the five negative-margin n = 4 coordinates, nine units, all receipts
HELD at every unit, with the reduced-grade cross-check run as a **blocking** pre-check before any carrier number
existed.

**ONE PAIR CLOSED.** At h6 — pip 4, hand `[11 40 43 53]` — the first-layer tax shaves competitor 11 below L_{40},
**strictly**, with surplus `L − U^(1) = 4930081/479001600`. Composed with S6h's frozen treatment-C rows for the
other two competitors under Lemma FT-mix and receipt **(FT-R8)**, this gives **Opt^H(h6) = {40}** — uniqueness, not
merely membership, since no comparison is non-strict. It is the branch's first gluing-cut separation and the first
time the two-sided architecture has closed end-to-end.

> **Mandatory sentence (FT-A25(vi)), which travels with that verdict verbatim:** *this coordinate's optimal set was
> already determined by the filed `Q^H` column; what this verdict demonstrates is that the two-sided proof
> architecture now closes here, and that the lever was a gluing cut and never a better candidate — which is exactly
> what Corollary E4.1(3) proved was the only lever available.*

Without that sentence the row reads as a finding about 42, and it is not one. The closure **could** have failed — it
did at the other eleven pairs — so the run is a genuine test; the **conclusion** could not have come out otherwise,
so it is not evidence about the game. The composition is licensed only under four printed conditions: same coordinate
(the freeze-45 identity rebuilt in-run), same L (one number, one provenance, its primal receipt named), same
(C1)–(C4), and a **freeze-subset assertion** — the two runs' freeze digests differ by construction, so what is
asserted instead is that every freeze the imported numbers depend on is identical and that the freezes this run adds
touch no object those numbers depend on. (FT-R8)'s sharpest clause asserts the **competitor row set equals the legal
action set at the rebuilt kernel minus a⋆**, never a row count in a filed file: a composed verdict is a universally
quantified claim over competitors, and a competitor silently absent from an imported table would make a false
verdict look complete.

**Ten tied pairs NOT CLOSED, and the shortfall must be typed honestly.** At a coordinate where the binding
competitor is *tied* with a⋆ in Q^H, the pair closes **only if the relaxation is exact** — an all-or-nothing
threshold. It was not met at any of the ten, and the shortfall came back equal to Δ² exactly. **That equality is an
arithmetic identity, not independent evidence** (FT-A26(ii)): given the tie, U^(1) − L = U^(1) − Q^H(a) = Δ² by
definition and cannot fail. It confirms the bookkeeping, not the proposition. Nothing could have tested the
proposition, because a tied pair closing with Δ² > 0 is impossible; it earns its keep as a **fence on the reading**,
and this run is the first occasion on which that fence did work. The filed object at each of these pairs is the
exact pair (Δ¹, Δ²), which by Corollary FT-grade4 is the **complete** layer decomposition — a result under F7, not a
null, and the exact specification of what a second-layer or feature-penalty cut must beat.

**The tax is sparse, and the sparsity comes with a selection fence.** Of **281,542** frontier states across the run,
**12,639 (4.49%)** pay a positive tax. The per-coordinate spread is wildly non-uniform — from almost every state
paying to about one in fifty — and the inversion is worth seeing: the coordinate with the largest frontier has the
sparsest tax, and the one with the smallest frontier has the densest tax and the largest gap. **Three fences, all
binding.** No distribution measured at grade 4 is quoted for trick 1 or for the opening. Nothing causal is claimed.
And the sharp one: **five coordinates chosen by negative margin are a carrier, not a sample, and the selection
criterion is correlated with the quantity being described** — so the spread must never be read as a distribution
over coordinates.

**Every minimal fusion core came back binary.** The ceiling at the grade-4 frontier is three worlds, because the
focal seat holds three tiles there; the observed size is **2 everywhere**. That answers the received note's open
question at this carrier by measurement rather than conjecture, under the same fences.

**(FT-R1) held at h9, and it is the quietest good news in the file.** h9's filed U had been computed exactly once,
by the revealed traversal, at the coordinate the exact primal route could not price at all. (FT-R1) reconstructs
that same U from the frontier decomposition — a **different traversal, with different intermediate quantities** —
and agrees exactly. It is **the only independent check that number has ever received**. It does not weaken h9's NOT
PRICED label, which is about the primal pipeline: h9 has no primal receipt, so no composed verdict may be filed
there at all.

**Where the FT mathematics lives, and for how long.** Lemma FT-arrive, Lemma FT-trunc, Corollary FT-grade4,
Proposition FT-flat, Proposition FT-tie, Lemma FT-post, Corollary FT-conv and Lemma FT-mix are proved in
`walt/CENSUS-RULINGS.md`, which is **their only authority** until the next errata amendment files them as a new §9.
That filing is queued behind DS-A28(ii)'s older outstanding item and is tracked on
[the reference map](walt-math-reference.md#addendum-2026-08-14--what-a-walt-math-successor-inherits).

**The next open target has a name: Δ²** — the second-layer tax, which is exactly the part of the fusion gap the
first layer provably cannot reach at the ten tied pairs. That target was taken up immediately, and the chapter below
is what came back.

## The second-rung chapter (S6l, 2026-08-14)

Δ² was the named target, and asking Pro for it produced the second external note of the same day. The pattern of the FT
chapter repeated exactly: the mathematics came back correct, the repairs were about hypotheses nobody named, and the
run's real finding was not the identity it was built to check but a structural fact nobody had asked about.

**What was asked and what came back.** The handoff asked four things — the exact second layer with policy-dependent
arrival, the multi-stage martingale penalty dual formalised, the depth-two regret-event calculus, and a grading of
the theory against walt's five exact Δ² rationals. **All four were delivered**, and the adjudicator found **no false
theorem and no wrong inequality direction anywhere in the note**. What it did find were four places where a
hypothesis is used and not named, one where a stated justification names the wrong hypothesis, one silent weakening
of a repair the note claims to have adopted, one receipt program whose two structural assertions **cannot fail**, and
one reading of the grading table that is tautological at grade 4 and not typed as such.

**The centrepiece: the slack–tax interchange law.** Δ² decomposes state by state as
Δ² = Σ_I min_b [s_{I,b} + d_{I,b}], where **s** is the *slack* — how much rung-one value the first action gives up
against the best first action — and **d** is the *downstream tax* it incurs below. It is CONFIRMED step by step, and
it is the first object in this branch that prices **policy adjustment** rather than only conflict: paying a little
slack up front to avoid a larger tax below is a trade the rung-one picture could not express. The engineering
consequence was the single most consequential fact in the adjudication — **the entire slack column was already inside
S6k's frontier pass and had only to be printed**, since the first-frontier branch values were computed and then
discarded down to an argmax set.

**Three repairs worth carrying.** *Theorem 4.1 uses a hypothesis nobody names* — the outer per-state maximum is
licensed only if a lawful first-stage policy may choose **independently** at distinct first-frontier states, i.e.
only if those states are genuinely distinct *information states* rather than histories. True here because of freeze
26's full-record contract; delivered as **Lemma SR-coord**, which also proves that a second-frontier state has a
unique parent. *The policy-level minimum does separate*, but not for the reason the note gives — it separates because
a lawful first-stage policy ranges over a free product of the per-state action sets; mutual exclusivity and fixed
arrival are what make the two rungs decompose against **the same weights**, which is a different job
(**Proposition SR-sep**). And *the note's own repair of Lemma FT-post is silently weaker than Lemma FT-post*: it
admits a residual witness "evaluated under the actual posterior", which is true and **not receiptable** — it is
precisely the sentence an artifact would write while pricing a fresh uniform coordinate and calling it the posterior.
FT-post's operational form and its print-in-place clause **stand unamended**.

**Why the backward recursion is legal even though occupancy is not.** Occupancy is policy-dependent from rung two on,
so the global object may not be flattened — and yet the backward recursion is valid anyway, **because occupancy never
enters it**. Everything below the first frontier enters through the lawful *posterior*, which is policy-independent
by Bayes cancellation; occupancy enters exactly once, at the first frontier, where Lemma FT-arrive fixes it
(**Proposition SR-post**). That is what makes the deeper penalty route coherent at all.

**Proposition SR-degen — the grade-4 boundary, and it is the fence that shapes the whole build.** At grade 4,
U^(2) = Q^H by Corollary FT-grade4 and L = Q^H at a ceiling-attaining witness by Corollary E4.1(2). So
L ≥ U_a^(2) holds **unconditionally at every binding pair**, strictly exactly at the untied ones, with surplus
exactly Q^H(a⋆) − Q^H(a). **No grade-4 experiment can test whether the second rung closes a pair** — the answer is
fixed by two already-filed columns. The received note's three graded §13 items are therefore already-filed exact H gaps
recovered by addition, or
the earlier chapter's arithmetic identity; all three were re-derived and land to the rational. The arithmetic is still
worth having, as a
cross-check between two independently produced filed columns, but reading it as evidence about the second rung's
*power* is barred. This is FT-tie's job one rung up, and the build was re-specified around it: **no closure verdict
is reported at all**, and what the probe reports instead is the identity, the decomposition and the escape census.

### The run: ten receipts, and the finding nobody asked for

Four units — two at h2, two at h9 — **all ten receipts HELD at every unit**, with the reduced-grade cross-check
running **blocking** before any carrier number existed. **Arm 2 completed; no declared stop occurred.** The
adjudicator re-derived every quantity independently by parsing the committed rows and recomputing from the branch
rows alone: **zero deviations at all 3,300 states.**

**The identity instantiates exactly.** The depth-two reconstruction reproduces the filed Q^H at both coordinates and
the filed Δ² at both, along with the rung-one U^(0), U^(1) and Δ^(1) as by-products of a depth-two traversal. This is
the result the build was for, and it is **a result about the proof machinery, not a discovery about 42** — the exact
value column already knew both answers. What is new is that the note's rung-two law now has an exact instantiation in
this engine, and that the (s, d) decomposition of a fusion gap exists as an artifact for the first time.

**ESCAPE ACTIONS ARE PRESENT — the first measured instance of policy adjustment in the branch.** The pre-declared
open question was whether the minimising first action ever leaves the rung-one optimal face. It does:
**36 of 330 first-frontier states at h2, 498 of 1,320 at h9.** The safety rule this triggers was pre-declared and now
binds: **every future rung-two lower witness must cover every first action** — not the tie-broken optimiser, and not
even the complete optimal face. Two rules now bind at rung two and they are different rules: the earlier one bars a
tie-broken optimiser in favour of the complete face; this one says the complete face is *also* not enough.

**What it would have cost, exactly.** **Proposition SR-loc** converts the escape flag from a diagnostic into an exact
accounting identity: the local tax equals the naive optimal-face-only tax **if and only if** the state is not an
escape state, so the total error of a naive witness is the sum over the escape set **and over nothing else**. Measured:
a witness taxing only the rung-one optimal face would have reported `1543/138600` at h2 against the true
`1483/138600`, and `12667/66528` at h9 against the true `4532503/26611200` — overstatements of **4.0459%** and
**11.7881%**. Since the naive quantity is an *upper* bound on the true tax, such a witness **claims to have shaved
more than it did**; at h9, nearly an eighth more. That is what makes this a safety rule rather than a technicality.
At h2 the arithmetic closes to the unit: each of the 36 escape states overstates by exactly `1/83160`, and
36 × 1/83160 is the whole of the discrepancy.

**The counts are NOT independent observations and must never be read as a rate.** At h2 all 36 escapes carry **one
signature** — the same local tax, the same naive value, a singleton face `{33}` and a singleton argmin `{54}` (and
`{53}` in the mirror unit). At h9 the escape action is the single tile **61 at every one of the 498**, against a
singleton face, with multiplicities arriving in blocks of 6 and 12. These are **one structural phenomenon reached by
many field continuations, not many phenomena.** The honest statement is *"escape occurs, at these coordinates, with
this structure"*; the dishonest one is *"escape occurs at 37.73% of states"* read as a rate. The selection fence binds
in its sharpest form yet: **five coordinates chosen by negative binding margin are a carrier, not a sample, and the
selection criterion is correlated with the quantity being described** — two of the five are in scope here, and neither
the escape rate nor the rung-two tax density may be read as a distribution over coordinates or over hands. No
distribution measured at grade 4 is quoted for trick 1 or for the opening, and nothing causal is claimed.

Two structural observations, claimed as nothing more: **no escape state has zero local tax** — escaping *reduces* the
local tax everywhere it happens and *eliminates* it nowhere, so the escape route is never a free rescue here; and at
h9 the escape states carry 47.09% of the whole Δ² while being 37.73% of the states.

**h9's second independent reconstruction, and what it does not buy.** The depth-two decomposition reproduces h9's
filed Q^H exactly — a different traversal, different intermediate quantities, a different theorem. Together with the
FT chapter's reconstruction of h9's U, **both of h9's filed columns have now each been independently reconstructed
once, by two different routes.** And the clause that must travel with it: **h9's NOT PRICED label stands verbatim and
is not weakened by any of it.** NOT PRICED is a statement about the *primal* pipeline — its extraction map exceeded
the cap, so no primal witness is exhibited there and L = Q^H is a ceiling, not a receipted witness. Reconstructing a
value twice on the **dual** side says nothing about the primal side. **A cross-check is not a witness**; agreement
between two computations of the same quantity does not manufacture the object the pipeline could not build.

**Two things closed, one carried.** **FT-A28 is fully discharged**: the deferred per-unit frontier digest is now
carried by all four units, which closes the named across-process residual **by receipt rather than evidentially** —
one scalar per unit reaches every individual rung-one fusion gap across executions. (It reaches those and
not the depth-two rows — the digest is over the rung-one records.) That discharge rests on something the build
found in itself: its streaming SHA-256 had a buffered-length defect, caught by a published known-answer self-check
**before any carrier number existed**. The dependency is recorded rather than assumed, because a mis-buffering hash
is still *deterministic* — two runs would have agreed and the receipt would have been **green and worthless**, since
a broken compression function may be wildly non-injective and the digest's whole scope claim is a statement about the
hash function, not about the probe. The discipline it yields: *a receipt asserting an equality of digests carries a
second, silent obligation — that the digest primitive is anchored to published vectors covering the code path
actually used, streaming path included.* And the reduced-grade cross-check did work
no filed number could have done: at the grade-3 coordinate every second-frontier state is forced, and the ladder
collapse held against **the engine's own H operator**, an independent evaluator the grade-4 carrier cannot consult.
It is the only check in the build whose answer was known **by proof** rather than by a filed rational — *a build whose
strongest checks are all against filed numbers can be self-consistently wrong; this is the check that is not.*

### Where this leaves the track

**There is no rung three at grade 4.** The ladder has exactly two rungs there, both are now computed exactly, and
Proposition SR-degen says grade 4 can no longer test closure at all. The carrier that produced every result in the FT
and SR chapters is therefore exhausted as a test-bed for the ladder: it can still instantiate identities, but it can
no longer *decide* anything about the architecture's reach.

**The next real question needs a longer ladder** — a coordinate where the ladder outruns the corpus of filed answers,
so that a rung's value is not already known before it is computed. That is a grade-5-or-deeper object, and it is
where the received note's trick-1 program and its three standing obligations stop being a distant destination and
become **the binding constraint**. Those obligations are unchanged and remain BLOCKED: the exact table at trick 1
ranges over 399,072,960 worlds and nothing reduces it; the only routes that avoid it need proved pointwise upper
bounds or proved regret events, **neither of which has a single proved instance in this game yet**; and any event mass
must be counted exhaustively, with no decimation inside a witness.

Four small obligations are owed on the next second-rung emission, and **nothing is owed now**: a cosmetic case fix in
the escape column; the filed binding pairs transcribed into the probe's frozen table and **asserted**, which converts
a by-construction guarantee into a comparison against a named carrier; the companion digest carried in that same
table, which at zero cost converts into a receipt what is currently the file's strongest evidence about the
depth-two layer's cross-process determinism; and a
**re-design** — not a re-application — of the committed/companion emission split before any deeper build. The
companion here is 8.8 GB against the previous chapter's 36 MB, "regenerable" is a weaker practical guarantee at that
size when auditing the digest costs a full re-run, and the growth is not incidental: the depth-two state count scales
with the field plies between frontiers, so a longer ladder multiplies it again.

## The feature-fee chapter (S6m, 2026-08-14)

The SR chapter ended by saying the next question needs a longer ladder. This one asks a different question that the
same carrier *can* answer: not how deep the ladder goes, but **which cheap structural quantities actually price the
tax the ladder measures.** It is the experiment both the received note and the SR adjudication anticipated — measure
which structural features approximate the perfect penalties, on a carrier where the perfect answer is already filed,
**before** any counting problem is faced. It is also the cheapest experiment the branch has left at grade 4, and it
ran in under a minute.

**Where the candidate came from.** Jason derived a control-flavoured feature at the table, reasoning through one real
hand. That provenance is worth stating precisely because the chapter turns on it: **table reasoning is a perfectly
good source of a hypothesis and no kind of evidence for it.** The audition exists to price it, and a null result
costs the hypothesis its candidacy and nothing else.

**The instrument.** For a feature φ of (world, action) at a frontier state, the fee is θ·(φ − centre) and the
question is how much of the state's local tax an optimal θ removes. Three results make that computable and readable:

- **Proposition FF-blind** — an **action-blind** fee removes exactly zero, at every state and for every θ. It is the
  penalty-side twin of the two blindness results already on the books, and with them the branch now has one lesson
  proved three times in three formalisms: **a witness, a bound or a fee must be conditioned on the decision it is
  trying to price.** One of the two requested features was action-blind, so auditioning it live would have burned the
  run rediscovering a theorem — it was **repurposed as a null control** with a pre-declared exact prediction of zero,
  which is the only contentful check the harness could have.
- **Lemma FF-min** — the fee objective is convex piecewise-linear and exactly minimisable: enumerate the
  breakpoints, evaluate, take the least. **No grid, no search, no float.** It also supplies two genuine receipts,
  because the swept minimum and the filed tax come from different computations.
- **Proposition FF-oracle** — and this one governs how every number here may be read. Optimising θ **per state**
  spends one free rational per information state, which is a **lookup table, not a feature basis**. So the two
  outcomes are not logically symmetric: a **low** capture **refutes conclusively**, because no shared or coarser
  parameterisation can beat the per-state oracle; a **high** capture **establishes nothing** about a usable fee
  family and licenses exactly one thing, the shared fit. Both outcomes are results; they are not results of equal
  strength. The column is named **oracle-θ capture** everywhere, never "capture" unqualified.

A fourth, **Proposition FF-degen**, is what makes the artifact readable after the fact: **zero breakpoints is exactly
vacuity.** A capture of zero at a state with thousands of breakpoints is a *measurement*; the same zero with no
breakpoints is a *tautology*. That diagnostic is emitted per state, and it is the only reason the chapter's central
defect was catchable from the committed file instead of by re-running.

### What the first run produced, including a defect it exposed

The frozen feature list attached its no-outstanding-trump fallback to all three features, but only two of them
reference the boss-trump holder. The third is well defined with no trump outstanding — and that is when it is most
interesting. **Six of the twelve (feature, unit) cells were therefore vacuous by construction**, and one of them was
the measurement the run most needed. The adjudication types those six as **unmeasured, not zero**, and no sentence
anywhere may report them as evidence. Freeze 52 went to v1.1, v1.2 and v1.3 in response; the sequence is on
[the freeze register](walt-math-freezes.md#freeze-52s-amendment-chain-v1--v14).

Two findings survive that run intact.

**Jason's feature is REFUTED, on the only part of the carrier where it has a domain.** At h0's **574 leading
states** — the only place in this carrier where a boss trump exists at the frontier — it was genuinely swept, 23,016
breakpoints, and its oracle-θ capture over those 574 states is **3,673 ppm, about a third of one percent**. By
FF-oracle that is an *upper bound* on what any shared or coarser parameterisation could reach: **a family that
cannot break 0.37% with 574 free parameters cannot break it with one.** Filed as a result, not a null.

**The scope discovery is worth more than the refutation.** Elsewhere the feature is not refuted but **inapplicable** —
there is no outstanding trump at the frontier, so the quantity it keys on does not exist. A boss-trump feature has a
**shrinking domain precisely as the hand simplifies**, which is the opposite of where a cheap witness is wanted. And
the fence that matters most: none of this is a verdict on Jason's reading of that hand. The feature was priced as a
centred fee against one specific object; losing that job says the quantity does not linearise the Jensen gap and says
nothing about whether the relation is the right thing to be thinking about. It was also sharp enough to be killed in
47.5 seconds, which is the property one actually wants from a table intuition.

### The corrected re-run, and the number the programme wanted

The sibling feature — *can my action be beaten?* — is action-conditioned, hidden, and the most directly
control-flavoured of the three. Re-run under the repaired domain clause, it separates into **three regimes**, and
each figure is stated with the state set it ranges over because that is a binding rule of this chapter:

| where | oracle-θ capture | breakpoints | reading |
|---|---|---|---|
| h0, **574 leading** states | **76.4628%** | 33,986 over the unit's 1,332 swept states | the measurement the chapter rests on |
| h0, **758 following** states | **29.2679%** | — | same feature, same unit, same sweep |
| h2, **216 swept** states, at each of the two units | **exactly 0** | **3,126 at each unit** | **refuted conclusively** |
| h0, all **1,332 swept** states | **75.1420%** | — | the whole-unit figure, for completeness |

**h2's zero is a refutation and not an empty test, and the breakpoint count is the whole of the difference.** In the
first run the same zero came with **zero** breakpoints and was a tautology; here the fee genuinely varied across
3,126 breakpoints and the minimum sat at θ = 0 anyway. **The same number means opposite things in the two files**,
and only the diagnostic separates them.

The whole-unit figure supersedes a pre-amendment number that had been typed as a *lower bound* precisely because it
averaged the genuine leading measurement against states the defective domain clause had forced to zero. The bound
held: the measured 75.1420% exceeds it. That earlier figure now retains exactly one legitimate use — as the
historical measurement of the feature **as originally frozen**, and hence as one term of the comparison that
confirmed its own supersession — and is closed for every other purpose.

**The leading/following gap changes what an earlier ruling said.** With the domain repaired, the split is no longer
about feature *availability* — it is a large difference in feature *quality*: **76.4628% over the 574 leading
states against 29.2679% over the 758 following states**, the same feature, the same unit, the same sweep. Whether that is about leading versus following **as such**, or about what
else differs between those two state sets at this one coordinate, is **not determined**.

**THE RESULT THE CHAPTER EXISTED TO PRODUCE: the shared fit.** FF-oracle had licensed exactly one follow-on, and it
returned. Over h0's 574 leading states, **one pooled θ* = −56/45** gives a shared capture of **76.3608%** against the
per-state oracle's **76.4628%** — **about 99.87% of the oracle survives collapsing 574 free rationals to one**, with
a shortfall of 0.102% of the leading-part tax. The corroborating structure is consistent: the per-state optimum takes
only **27 distinct values**, none of them zero, over a narrow range, with 12 states already matching the pooled
value. A feature whose optimal price is nearly constant across states is precisely one whose shared fit should lose
almost nothing, and it does. **This is the first time in the branch that a *small* fee family has been shown to carry
a first-layer tax**, and it is what makes the penalty route more than a theorem.

**The single sentence worth carrying out of the chapter**, because it is the one comparison free of every
between-coordinate confound: on **one and the same set of 574 states**, with one sweep, one arithmetic, one centring
and one tie rule, the two action-conditioned candidates return **0.3673%** and **76.46%** — a ratio of about
**208×**. What that supports, at exact strength: *at h0's leading frontier states, the first-layer Jensen gap is
substantially aligned with whether the focal seat's action can be beaten, and essentially not at all with whether the
boss-trump holder can follow it.*

### What none of it establishes

The fences here are unusually load-bearing, because a capture fraction reads exactly like a rate and the entire
motive for the chapter is screening for trick 1.

- **One coordinate, one part of it.** 574 of that unit's 1,332 swept states, and h0 is one of nine n = 4
  coordinates. The two coordinates in scope were selected by negative binding margin, so they are **a carrier, not a
  sample**, and the selection criterion correlates with the quantity being described.
- **Nothing is quoted for trick 1 or for the opening** — the fence most at risk, given that trick 1 is the target.
  The three obligations blocking the trick-1 route are untouched.
- **No grade-4 verdict moved and none could**, by the second-rung degeneracy result. h0's binding pair is untied and
  already closes at rung two.
- **It is a rung-one result.** Every number here prices the first layer only. A fee capturing 100% of it would still
  leave the second-layer tax untouched.
- **A 99.87% shared/oracle ratio at one coordinate part is a licence to test the fee at a second coordinate, not a
  licence to believe in it.** No artifact of this build says the feature "works", and none does.

**One receipt-design lesson, filed alongside the chapter's other two.** The null control expects an optimal
coefficient of zero — **and so does the failure mode "the sweep is broken and always returns zero".** A null control
whose expected answer coincides with a plausible bug's answer **cannot** validate an exact zero measured elsewhere.
What licenses reading h2's zero as real is that the same feature at h0 returns a *non-zero* optimum at all 574
leading states, across 27 distinct values: the sweep demonstrably moves. **A null control is complete only when
paired with a case whose correct answer is known to be non-null** — here that pairing existed by luck of the carrier
rather than by design, and the next pre-declaration should require it.

**Nothing further is commissioned.** The obvious next experiment — the surviving feature at a third coordinate,
chosen to vary trump survival at the frontier while holding as much else fixed as possible — is deliberately *not*
inherited: it is a new carrier and wants its own freeze, its own pre-declared readings including an explicit
empty-arm branch, and the non-null pairing above. It should be asked for, not assumed.

**It was asked for, and the answer redirected it.** The next chapter declined the third coordinate in favour of
explaining h2's zero first, and that explanation retired the trump-survival criterion before a coordinate was chosen
on it — see below.

## The fee-correlation chapter (S6n, 2026-08-14)

The feature audition ended holding an exact zero it could not explain: the surviving feature captured about three
quarters of the first-layer tax at h0's leading frontier and **exactly zero** at h2, twice over, across thousands of
breakpoints that proved the fee genuinely varied. The audition had made the diagnostic optional. **This chapter
reverses that**, and the reasoning is worth keeping: an exact rational identity holding at 432 independent states is
the most informative unexplained fact in the branch, and the instrument that would explain it costs seconds.

**The third coordinate was deferred, not refused, and the deferral turned out to be load-bearing.** A third
observation taken before the mechanism is measured *enlarges* the previous chapter's confound rather than resolving
it — a single contrast cannot isolate one variable from a dozen co-varying ones, and adding a third under the same
conditions does not escape that. Had a third coordinate been run first it would have been selected on trump
survival, and the actual mechanism would still be unknown.

**The instrument. Proposition FC-drop** converts the earlier zero-test into a **quantitative lower bound on
capture**, computable with no minimisation: **capture is at least correlation times reach.** One directional slope
measures how far the feature leans on the clairvoyant choice; one breakpoint distance measures how far the fee can be
pushed before that choice starts changing; the fee collects their product at minimum. **Corollary FC-null** supplies
a null control whose exact value is fixed by theorem rather than by a filed number — which is what a control needs to
be, since a control checked against a filed rational tests the filing rather than the harness.

### The answer, and the mechanism

**The zero is TIE-DRIVEN, unanimously.** At both h2 units, over each unit's **216 swept states**, the two one-sided
slopes **strictly straddle zero at every state, with neither slope zero anywhere.** The alternative — genuine
orthogonality between feature and clairvoyant choice — is **refuted at every state of the carrier**, which is the
stronger of the two ways a pre-declared reading can fail.

**Proposition FC-width is why that is a result and not an observation.** The width of the subgradient equals the
mass-weighted spread of the feature across the clairvoyant tie. **Without ties the interval is a point**, so zero
capture would demand an exact rational identity — implausible at 216 states twice over. **With ties it has positive
width**, and zero capture needs only that zero fall inside it: robust, not coincidental. Measured: a non-singleton
clairvoyant argmax at **236,784 of the 362,880 (state, world) arrivals at each h2 unit's 216 swept states (65.25%)**
against **59,776 of 266,132 at h0's one unit over its 1,332 swept states (22.46%)**, with h2's straddle holding at 216 of 216 per unit where h0's fails at 1,252 of its 1,332 — all four counts at the
beatability feature.
Mechanism and measurement agree.

### What it does to the programme

**The h2 refutation was never about that feature.** No fee keyed on the clairvoyant choice can be expected to bite
where the face is widely non-singleton, because FC-width widens the subgradient for **any** such feature. Three
consequences, and they are the reason this chapter outranks a third data point:

- **The refutation decouples from the candidate.** It is a statement about the **fee route**, not about a feature.
- **There is now a pre-fee screening statistic** — the argmax cardinality profile. It is a property of the
  coordinate's world structure, measurable **before any fee is built**, and it gates whether building one is worth
  attempting.
- **The negative is structural rather than a failure of cleverness**, which is the most useful kind this branch
  produces.

So the programme's first question at a new coordinate is no longer *which feature* but **"is the clairvoyant choice
pinned down enough for any fee to bite"** — cheaper than building a fee, and now backed by an exact statistic. It
does not advance the trick-1 obligations and nothing here is quoted for trick 1, but **it is the first thing the
branch has that says where *not* to spend the attempt.** The third coordinate, still uncommissioned and still needing
its own freeze, is now selected on **measured multiplicity** rather than on guessed trump survival; the earlier
trump-survival criterion is demoted to a hypothesis about a correlate.

### How good the screen is, stated in the words the ruling binds

Over the **1,252 straddle-false states of h0's one unit at the beatability feature**, the bound is **attained** —
equal to the frozen captured amount — at **258 of them (20.61%)**, while the summed bound recovers **14.873%** of the
summed capture. **Proposition FC-tight** says what those 258 are: the states where the descent is a single
linear piece. Three qualifications travel with those numbers, and the wording is not decoration:

- **It is a lower bound at every state, the 258 included** — never *exact* as a property of the functional anywhere.
  Saying "exact at 258 states" invites the reading that the screen predicts capture a fifth of the time, which is
  the reading this chapter exists to prevent.
- **Which states attain it is not knowable without the captured amount**, the very quantity the bound exists to
  avoid computing. The 258 is therefore a fact about the distribution of the gap and **never a usable property of the
  instrument**: in use the screen is exactly as weak as its aggregate 14.873% and no weaker.
- The reach is measured to the nearest *candidate* breakpoint, which can fall short of a true kink, so the bound
  carries **a second and independent conservatism** on top of the descent running past it. Both shrink it, never
  enlarge it. This belongs with any citation of the 14.873%.

The bound is **one-sided**, and that is what may be said about it: **a positive bound PROVES a fee bites at that
state; a zero or small bound proves NOTHING.** No false positives, unbounded false negatives. That is shorter and
stronger than any adjective grading the instrument, and it is what the ruling settled on after two attempts —
"exact" and "usable" both being true under one reading, inviting a stronger one, and silent about which. The
substantive point underneath is unchanged: **screening and estimating are different jobs.**

The house form this sets, and it binds past this chapter: **do not grade an instrument — state what follows from a
positive reading and what follows from a negative one.** A sentence built that way cannot be excerpted into
something stronger than itself, which is the only durable protection, because escorts do not travel and sentences
do.

### The rest of the run, each figure with its full scope

The **null control** held with both slopes exactly zero at **all 1,764 swept states across the three units**,
blocking and first. The **non-null pairing receipt** — required by design because the previous chapter had got that
pairing only by luck — held over **h0's 574 leading states at the beatability feature**, slopes not both zero at
**518**. A **filed-face receipt** compared **3,528 masks over every swept state of all three units — 1,332 at h0 and 216 at
each h2 unit — 1,374 of them two-tile**, all matching a different program's committed output; those two-tile masks are 39% of the comparison and are exactly the
faces a tie-broken argmax would have collapsed. That receipt earned its keep on a narrow margin worth recording: a
collapsed face would *most likely* have failed loudly, but a collapsed value landing exactly on zero would have
reported the wrong one of two pre-declared readings **with every receipt green**.

**The graded boss-keyed feature is not refuted**: over **h0's 574 domain-nonempty swept states** its bound is
positive at **322**, so its capture is **proved positive by theorem, with no sweep at all**. The calibration attached
to that finding governs how it reads, and without it the sentence flatters: **the already-refuted binary form is
straddle-false at 374 of those same 574 states — more states than the graded form — and that binary cashed out at
0.367%.** **Proved-positive and negligible are entirely compatible, and here that is the likely reading.** The sweep
was declined; what would reopen it is a shared-parameter capture at a *second* coordinate materially different from
h0's, which would show the fee route is coordinate-robust. Nothing less.

**Where this leaves the line.** The fee route works at h0's leading frontier and **is not to be expected to bite,
robustly so**, wherever the clairvoyant face is widely non-singleton — not a fixable defect of any candidate. The
modality matters and the ruling corrected its own text for it: the width result makes zero capture **robust**, not
positive capture **impossible**. A feature whose mean slope exceeded the half-width would still bite despite
widespread ties; nothing forbids it. Every standing fence is
undiminished: three units at two coordinates chosen by negative binding margin are **a carrier and not a sample**;
**grade 4, so no verdict moved and none could**; the **first layer only**; and **nothing quoted for trick 1 or for
the opening**, binding hardest here because multiplicity is now the variable most tempting to extrapolate.

---

## Standing fences

- Every number cited on this page is exploratory, coordinate-relative, and measured over declared **void-free
  capacity fibers** — a declared cost domain and a superset of any seat's actual support. Members are **feasible**;
  none is asserted reachable, and no support fact about any seat may be read from one.
- **Support is not belief.** The beliefs in this program are declared aggregation arguments on fabricated kernels
  (P-A12), not any seat's actual belief; the field is a separate declaration from the belief and the two are never
  compressed into one sentence (R-A9).
- **No cost, timing, runtime or tractability claim** follows from any result in this program (SEP-A15(iii), R-A23).
  Termination bounds are not complexity claims; a node fraction is never a cost fraction.
- Every result in the count-free layer is **void wholesale** if count re-enters, never extended (E-A2), except where
  a ruling states the survival explicitly — Lemma J(c′) for the guarded detectors, DS-A16 for library entries as
  primal-witness sources.
- The concrete authority is treatment H wherever it completes; a disagreement with it is a **stop-and-report bug**,
  never reconciled by adjustment. A divergence from the world-informed aggregate is not a disagreement at all but
  the expected strategy-fusion gap.
