# walt — the decision-sparse program

[Home](Home.md) · owns: the decision-sparse exact-solving architecture — its thesis, its objects, its theorem
inventory, its audit history, and the standing state of its experiment program · Sources:
`walt/math/decision_sparse_exact_solving_v0.1.md` (the received parent, verbatim) and
`walt/math/decision_sparse_exact_solving_v0.1_errata.md` (the durable repaired mathematics, DS-A17);
`walt/CENSUS-RULINGS.md` (DS-A1..DS-A36, SEP-A1..SEP-A19, J-A1..J-A18, PG-A1..PG-A18, R-A1..R-A24,
N4-A1..N4-A20, EC-A1..EC-A14, RW-A1..RW-A8, **FT-A1..FT-A28**);
`walt/SEPARATION-PROBE.md`, `walt/DEADNESS-PROBE.md`; `walt/walt-factory/results/deadness_2026-08-12.txt`,
`separation_2026-08-13.txt`, `separation_n4_2026-08-14.txt`, `rule_economy_n4_2026-08-14.txt`,
`fusion_tax_2026-08-14.txt`; and the received `exchange/inbox/016-decision-sparse-nonanticipativity-taxes.md`
(adjudicated, never imported as an axiom). Related: [walt](walt.md) (hub), [walt-s6-era](walt-s6-era.md) (the sessions that produced
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
authority, and a loaded entry is re-priced before anything is reported; **identity transport only in v1**, opened at v2 to the declaration fold and nothing further (EC-A8);
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
| **D** | Adaptive gluing on one tense grade-3 root: relaxed solve, exposed-face test, emit and generalise cuts, re-solve, stop at exactness. | **OPEN AND FED.** Its two blockers are gone: **freeze 38 is filled** (v1, scoped, FT-A17), and its inputs are no longer empty — S6h's four exact negatives are exactly the failing pairs D consumes. S6k ran the first rung of the ladder over five of them; the residual second-layer gap Δ² is what a further cut must reach. |
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
imported as an axiom. Adjudication (FT-A1..FT-A28) accepted it **in large part**: the central identity and the
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
[the reference map](walt-math-reference.md#addendum-2026-08-14--what-walt-math-11-inherits).

**The next open target has a name: Δ²** — the second-layer tax, which is exactly the part of the fusion gap the
first layer provably cannot reach at the ten tied pairs.

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
