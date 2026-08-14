# Census fork rulings — walt-math

**Adjudicator:** walt-math. **Date:** 2026-08-10. **Tier:** exploratory, below every
project evidentiary tier. **Basis:** `walt/math/equivariant_lumpability_v0.5.md`
(§12.6A, the law) and `walt/math/unified_information_geometry_v0.4.md` (frozen).
Citations `v0.4 §x` / `v0.5` below. These rulings bind the census build.

## F1 — carrier. RULING: ADOPT with one scope amendment.

**Reason.** §12.6A works on a latent space X with belief β; conclusion 1 pushes β
through d, so quotienting world-level concrete situations (x = K ⊕ ω joined to the
public residue) is the correct instantiation — not a support quotient, not a belief
quotient (v0.4 §11.2). Pooling all 13 kernels into one ambient frame is lawful
because transports are declared per descriptor class and are "not asserted to be
global symmetries of Straight 42" (v0.5, transports section; v0.4 §11.1): a
cross-kernel merge needs only a lawful Θ for that class, which canonicalization
supplies. Per-kernel uniform beliefs are supported inside each kernel's states;
the theorem quantifies over all β on X, so pooling costs nothing. This is not
worldwise-PI classing: the equivalence tested is (ECL) dynamics equivalence, and
v0.4 §12.4/§17.5 forbid only importing R_PI(ω) fibers as hidden-decision classes —
no PI response ever enters d.

**Amendment (scope).** All 13 receipt trick-six kernels are pip-trump (v0.4 §14.7:
pip declarations 0,1,3,4,5,6; "No doubles-trump or no-trump transfer claim was
made"). The census MUST declare pip-trump-only scope. DT/NT pooling is not blocked
by the theorem, but DT/NT have structurally different context signatures (κ_DT =
the doubles, κ_NT = ∅, so NT has no tier-2 context at all, v0.4 §1.2–1.3) and no
receipt corpus; including them would implicitly claim cross-declaration-type
transfer, which v0.4 §17.5 explicitly does not claim (§17.4 open problem 7 is the
sanctioned later route). Within scope, trump→trump matching forces pip↔pip and
cross-pip pooling is lawful.

**Builder consequence.** Carrier = all reachable states of the 13 pip-trump
kernels under the primitive-step model; write "scope: pip-trump" into the design
header and the results file. Banked increment: see F5 amendment (not state).

## F2 — invariant list. RULING: ADOPT WITH FOUR AMENDMENTS.

**Reason and amendments.**

**A1 (gap — unresolved-trick tiles enter the comparison structure).** "BEATS
restricted to live tiles and live contexts" is too small at mid-trick states.
Future plays to the current trick are compared against the table tiles already
played (in particular the current winning tile) under the current led context —
the trick winner is the maximum trick key over all four tiles (v0.4 §1.3). Restrict
comparisons to: live tiles ∪ unresolved-trick tiles, in the live contexts plus the
current led context. Tiles in resolved tricks are correctly excluded: a captured
tile has no holder (v0.4 §2.3), can never be played again, and future trick keys
are evaluated only on tiles actually played to future tricks, which come from live
hands (v0.4 §1.3, §1.5). Dead-tile-relative ranks do NOT belong in the structure.

**A2 (gap — the led-context map).** Follow membership does not determine which
context a tile leads: an uncalled mixed tile is a member of two effective contexts
(the covering, not partition, v0.4 §1.2) but leads only ℓ_δ(d) = high(d). A
context bijection consistent with all listed relations can still swap a mixed
tile's led context, and the dynamics then diverge (led context controls follow
legality and the BEATS index, v0.4 §1.2–1.3). Add: the matching preserves the
led-context assignment — Θ^C(ℓ(d)) = ℓ(Θ^D(d)) for every live tile d.

**A3 (definition — "live context").** Define: a context q is live iff some live
tile leads it (q ∈ ℓ[live]). Membership in a context that can never be led again
is dynamically inert; erasing it is lawful and is part of what makes the quotient
nontrivial. The current trick's led context is additionally in force for A1's
comparisons even if no live tile leads it.

**A4 (seat matching is a forced rotation — no reflection).** The matching is the
unique successor-preserving map aligning current actors: seat(actor_x + i) ↔
seat(actor_y + i). This preserves partnership (partner = +2) automatically;
leader-offset equality is implied by matching the table play order; focal-offset
equality is a match precondition (focal ↔ focal), not a choice. Reflected
matchings are FORBIDDEN: reflection does not commute with the fixed clockwise
successor and is not an automorphism of the oriented game (v0.4 §11.7); the census
adjoins no orientation variable η, so only rotations of the relative-seat cycle
are lawful.

**Confirmations.** Trump→trump, non-trump→non-trump with free bijection: correct
in pip-trump scope; pips enter only relationally (count-free core; v0.4 §8.9 —
no context-free domino value is presupposed). The double flag is not forced by
pip-trump dynamics (doubles act only through rank relations, already captured by
BEATS), but retaining it is lawful and consistent with the deliberate
finest-candidate design; it and the beaten (non-winning) table tiles are the
identified first coarsening candidates for later declared rounds — not this run.
The DT/NT "called structure" question is mooted by F1's scope amendment.

**Builder consequence.** Canonical form is computed over the object: live tiles ∪
unresolved-trick tiles; holders by relative seat (rotation-aligned per A4); trump
and non-trump live contexts; follow membership; led-context map ℓ; pairwise trick
keys per A1; double flags; table order and current winner. Bank excluded (F5).

## F3 — transports by canonicalization. RULING: ADOPT.

**Reason.** Θ_xy = c_y⁻¹ ∘ c_x factors through the canonical representative, so
Θ_xx = id, Θ_yx = Θ_xy⁻¹, and Θ_xz = Θ_yz ∘ Θ_xy hold identically — for Θ^D, Θ^C,
Θ^Q and for the derived Θ^A and Θ^obs alike, which discharges v0.5 Appendix A
note 1 (coherence scopes over all declared transports) by construction. Θ^A =
Θ^D restricted to the actor's legal tiles is a declared candidate bijection; that
it IS a bijection A(x) ≃ A(y) is exactly ECL condition 1, which the census checks
rather than assumes — a failure there is a recorded counterexample, not a crash.
Per v0.5 Appendix A note 3, the observation language names tiles beyond any
represented role interface, so declare explicitly: Θ^D is defined on all live and
unresolved-trick tiles (the canonical labeling already provides this).

**Builder consequence.** No separate coherence check is needed at runtime;
implement Θ only as re-labeling through canonical form. Declare the Θ^D domain
(live ∪ table tiles) in the design text.

## F4 — probability model. RULING: ADOPT, with the operational reading fixed as follows.

**Reason.** v0.4 §7.4 fixes a field σ_{-m} for all three non-focal seats
(including the focal partner) and leaves the focal seat optimizing; the probe
convention is the uniform-random legal field (v0.4 §14.2, §14.6). The correct
§12.6A reading for a turn-taking game — the builder implements this verbatim:

- The actor is a function of the state. There is one kernel per primitive step;
  (ECL) never mixes focal choice with hidden chance in one step.
- **Focal-to-act state x:** A(x) = the focal seat's legal tiles; for each a,
  K_a(x; ·) is the Dirac point mass at the determined (k, o, x'). ECL condition 1
  checks A(y) = Θ^A(A(x)); ECL condition 2 is the commutation check per action.
- **Non-focal-to-act state x:** A(x) is a singleton no-op (the focal seat has no
  choice); K(x; ·) puts mass exactly 1/|L| on each legal move of the hidden actor
  (L its legal set), emitting that move's (k, o, x'). ECL compares the full joint
  law. Because the token names the played tile, the law is uniform over |L|
  distinct tokens, so ECL silently enforces hidden-actor legality preservation
  (|L_x| = |L_y| with tokenwise Θ^obs correspondence) — no separate check needed.
- All probabilities are exact rationals attached per primitive hidden play
  (products over steps arise only inside the theorem, never as stored floats).

**Builder consequence.** Implement two node types keyed off the actor; never
attach a probability to a focal action; i128 rationals with exact equality.

## F5 — primitive steps and mid-trick states. RULING: ADOPT with one amendment.

**Reason.** §12.6A's kernel K_a(x; k, o, x') is a primitive play-step object:
"At a primitive play step this is normally {0, e⋆}: zero unless the step
completes a trick won by the focal partnership." So mid-trick states MUST be in
the carrier (the kernel steps through them), trick-level macro steps are the
wrong granularity, and k attaches at the trick-completing fourth play — including
trick 7's final play, where the recursion closes at hand end. e⋆ is the trick
coordinate of the transported focal partnership (v0.5), which A4's focal↔focal
matching guarantees. The observation token (relative seat of actor, transported
tile, lead/follow/slough class) matches v0.4 §6.1's typed token; the led context
is derivable from the tile via ℓ and the context matching, so the token is
sufficient. Emit a token for every play, focal included, uniformly.

**Amendment (bank is emission, not state).** The count-free increment is emitted
by the kernel and accumulated by the theorem (Σᵢ kᵢ, v0.5 conclusion 2); it is
not part of the latent state's future dynamics (legality and the field read only
hands and table). Storing it in the carrier and descriptor duplicates a derived
quantity (forbidden: derived views, never stored state) and needlessly splits
trick-7 classes by trick-6 outcome, destroying real merges. Remove the banked
increment from state identity and from d; the runner tracks it as bookkeeping
outside the descriptor if it wants per-path sums.

**Builder consequence.** State = hands + table + actor (no bank field in the
canonicalized object); k computed at the resolving play; terminal after trick 7.

## F6 — baseline and quotable statistics. RULING: ADOPT.

**Reason.** v0.5's closing corollary states §12.6 is recovered exactly by
identity transports, so the identity-interface class count on the same carrier is
the canonical control, and (class count under identity) / (class count under
relabeling) is precisely the equivariance dividend. On a pooled cross-kernel
carrier the identity baseline will almost never merge across kernels — that is
the point, not a defect. Quotable form: every number is exploratory tier; a class
count is quotable only alongside its ECL verdict (a count without PASS is a
relabeling census, not a lawful compression measurement); report root-only and
full-carrier counts separately (the 10^5 bar is about situations, and root counts
are the comparable figure), per-kernel and pooled, plus the singleton-class count
(classes where ECL is vacuous) so the check's actual coverage is visible.
Promotion beyond exploratory happens only by receipt amendment, never here.

**Builder consequence.** Add to the results file: singleton/vacuous-class counts
and the explicit pairing of every count with the ECL verdict line.

## F7 — failure protocol. RULING: ADOPT.

**Reason.** Recording counterexample pairs and stopping is exactly v0.4 §12.9
steps 4–5 with steps 6–7 (new atoms, retest) deferred to later declared
candidates — the sanctioned counterexample-guided method under NO-RESCUE. Both
outcomes are results: v0.5's claim ledger lists existence of a nontrivial (d, Θ)
satisfying (ECL) as OPEN, and this census is its designated first measurement.
Drafting a 5.6 Pro dispatch is permissible; only Jason authorizes submission.

**Builder consequence.** A FAIL emits (canonical form, divergent statistic, both
concrete witnesses, exact rationals) and continues to the next class; no
descriptor edit, no re-run with altered invariants, in this session.

## Extra item — coherence and the empty output interface. RULING: LAWFUL as designed.

**Reason.** Coherence including Θ^A and Θ^obs: automatic, see F3 (v0.5 App. A
notes 1–2 discharged by factoring through the canonical representative).
O_Σ = ∅: ρ_x is the empty function, which is trivially functionally instantiated
(the requirement excludes merely-existential multi-answer fibers; the empty
interface has nothing to instantiate and vacuously satisfies it). The rigid
square holds vacuously, and v0.5 conclusions 3–5 degenerate: the preserved
terminal data is exactly the count-free outcome t_T·e⋆ (conclusion 4) — which is
the count-free core by design. Role re-entry and the valuation gauge corollary
require a later declared nonempty O_D; nothing in this census forecloses that.

**Builder consequence.** No role bookkeeping anywhere in the census code; the
only preserved outcome channel is the e⋆ accumulator.

# r3 — retrograde coarsest quotient (adjudicated 2026-08-10, walt-math)

Context: r1's finest structural quotient passed ECL exhaustively (15,253
situations → 11,949 classes, 0 of 647 roots merged; results file). r3 replaces
candidate-guessing with the target construction: backward induction over the
graded carrier computing the coarsest equivariantly lumpable quotient.

## Q1 — sound and coarsest. RULING: ADOPT WITH TWO AMENDMENTS (coherence; preamble).

**(a) Transport domain — reading CONFIRMED.** v0.5: transports "need only be
defined on the declared represented interface unless a larger transport is
required by the action or observation language." With O_Σ = ∅ the represented
interface is empty, and the primitive-step action/observation language names
only the acting seat's playable tiles. So the per-pair Θ^D domain is exactly the
actor's legal set. Later steps never force a larger domain: the theorem's
induction compares successors by CLASS and uses the successor pairs' own
transports; fresh objects are "re-evaluated at the successor" (v0.5), and the
rigid square is vacuous (empty interface). Consequently r3 may lawfully merge
states related by NO full-structure tile bijection — §12.6A never asks Θ to
extend to a state isomorphism ("not asserted to be global symmetries"). This is
the equivariance gain, and it is why r1 must refine r3 (see Q5 assertion).

**(b) Amendment 1 — coherence forbids arbitrary tie-breaking.** The coherence
laws (Θ_xx = id, Θ_yx = Θ_xy⁻¹, Θ_xz = Θ_yz ∘ Θ_xy) scope over Θ^A and Θ^obs
(v0.5 App. A note 1, adopted in F3), and per-pair matchings with arbitrary ties
can violate composition (a↦b, b↦c, but a↦c′), making the abstract action and
observation classes [x,a], [x,o] ill-defined. Fix exactly as in F3: fix one
canonical move ORDER per state — sort moves by the full signature tuple (Q3),
ties broken by a deterministic per-state rule (e.g. the state's concrete tile
order) — and declare Θ^A/Θ^obs as position matching through that order. All
transports then factor through sorted positions; coherence is automatic. Ties
(moves with identical tuples) emit identical statistics, so tie order never
changes any law; abstract actions/observations ARE the sorted positions.

**(c) Coarsest — CONFIRMED, with a relativity caveat.** By induction on grade:
at grade 0 r3 is the one-class partition, trivially coarsest. At grade g, if
(d,Θ) is any lawful equivariantly lumpable pair (under the same interface
typing, Q3) and d(x) = d(y), then ECL's bijections match moves with equal k,
preserved classification and actor offset, and equal d-successor-classes; by
the induction hypothesis d-classes refine r3-classes at grade g−1, so the
matched moves have equal r3-signature tuples, hence sig(x) = sig(y). So every
lawful (d,Θ) refines r3: r3 is the coarsest. The claim is RELATIVE to: this
carrier, the uniform-legal field, the count-free contract, the primitive-step
model, and the token typing of Q3 — a differently-typed observation interface
could admit coarser quotients; do not quote "coarsest" without this scope.

## Q2 — successor-class equality. RULING: CONFIRMED.

(ECL) sums K over x′ with d(x′) = z: only the successor's class enters, never a
successor matching through a restricted transport. With deterministic per-move
successors the sum is an indicator, so the multiset of per-move tuples IS the
joint law of (k, ō, z) up to the declared bijections (uniform masses at hidden
steps; action-indexed Dirac family at focal steps — the "multiset is the law"
phrasing applies only to hidden steps, but the equality test is the same). One
backward pass closes because every primitive play removes exactly one live
tile, so every successor has grade g−1: the carrier is strictly graded (v0.4
§1.5) and no fixpoint iteration is needed. Builder: assert grade(succ) = g−1.

## Q3 — signature tuple. RULING: as proposed, plus actor OFFSET in the preamble.

Θ^obs is not a free bijection on whole tokens: transports are "determined ...
by role-name correspondence" over the three sorts (N_Q, N_C, N_D), extended
only as the observation language requires — still sort-by-sort. The v0.4 §6.1
token is typed (actor: Q-sort, tile: D-sort, classification: bare label), so
Θ^obs acts componentwise as (Θ^Q, Θ^D, id): the classification has no transport
sort and is preserved LITERALLY — it belongs in the tuple. The actor component
is transported by Θ^Q, which "must carry the declared partnership and
orientation convention" (v0.5); with no orientation variable adjoined (§11.7,
F2 A4), the lawful chair correspondence preserves the seat's offset from focal
— focal↔focal AND partner↔partner AND left↔left, not merely focal-vs-hidden.
Ruling: **preamble = (grade, actor offset from focal ∈ {0,1,2,3})**; offset 0 =
focal choice node, offsets 1–3 = hidden chance nodes. **Per-move tuple =
(k ∈ {0, e⋆}, play classification ∈ {lead, follow, slough}, successor r3-class)**.
Tile identity and led context are deliberately absent (transported per-move —
the point of r3). Remark (purpose-level, not the legal basis): literal
classification also keeps the abstract observation record from gluing
focal-distinguishable histories, protecting the abstract-policy class that
v0.5's BOUNDARY scopes the value claim to.

## Q4 — interpretation and stability. RULING: intrinsic classes; carrier-relative counts.

Encode signatures hereditarily and content-addressed (a state's encoding is a
function of its future cone only, through the canonical move order). Then class
identity is INTRINSIC to the continuation: adding states or kernels to the
carrier can only add classes or add members to existing classes — it can never
split or re-merge existing ones, and the partition restricted to the old
carrier is unchanged. Counts remain carrier-relative (they count only sampled
states). Mandated results-file caveat, verbatim or equivalent: "Classes are
dynamics-equivalence classes under §12.6A on this carrier, uniform-legal field,
count-free contract, per-step interface typing (r3 ruling Q3); they need not be
closed under any tile relabeling and carry no structural description — the
compact-description question (v0.4 §12.7) is separate and open. Coarsest is
relative to that scope. Class identities are intrinsic to continuations; counts
are carrier-relative; carrier growth adds classes, never splits existing ones.
Exploratory tier. ECL holds by construction; see verification lines." These are
not hidden-decision PI classes (v0.4 §12.4): the equivalence is dynamics, not
response equality — the r1 caveat carries over.

## Q5 — remaining defects. RULING: three mandatory items, none blocking.

1. **Refinement assertion.** r1's structural transports are componentwise and
   classification/offset-preserving, so r1 is a lawful (d,Θ) under Q3's typing
   and MUST refine r3: assert in-run that each of the 11,949 r1 classes lands
   inside exactly one r3 class (cheap, and the strongest implementation check
   available). Failure = implementation bug or a math error in this ruling —
   stop and report per NO-RESCUE; never patch.
2. **Verification discipline.** "By construction" is not a receipt: run an
   independent ECL re-check over the r3 partition with the declared
   position-matching transports (same checker shape as r1) and pair every
   quoted count with that verdict line (F6).
3. **Determinism.** The per-state canonical move order and the content-
   addressed encoding are determinism freezes — fix them in code and note them
   in the results header; class counts must be reproducible bit-for-bit.

# The railyard factoring — shaping (walt-math, 2026-08-10; design shaping, not a build order)

Indexing convention (fix this to kill an off-by-one): level j = tricks
remaining; at a trick boundary every seat holds exactly j tiles; A_j = the set
of level-j boundary r3 classes; A_0 = the one terminal class. Level-j dynamics
instantiate at A_{j−1} (one fewer trick remaining), not A_{j+1}.

## Y1 — one-trick contract and stacking. RULING: ADOPT WITH ONE CORRECTION; stacking = r3 CONFIRMED.

**Correction (unsound as posed).** "Outcome = (increment, handoff class)" read
as a trick-level MACRO step is unlawful — F5 already ruled macro steps the
wrong granularity for §12.6A, and a black-box trick erases the mid-trick
observations and focal choices the contract must preserve. The lawful contract:
the one-trick machine is the FOUR-primitive-step machine with the Q3 per-step
interface typing (offset preamble; per-move (ε, classification, succ) with
ε ∈ {0, e⋆} emittable only at the fourth step), whose terminal symbols are the
handoff classes. This is a declared output contract in §11.3's sense (selected
outputs: typed step emissions + class-valued handoff), so the truncated-horizon
quotient is lawful.

**Stacking.** r3's recursion IS this stacking. The carrier is strictly graded
with exactly 4 plays per trick, so grouping grades in blocks of four factors
the recursion at trick boundaries: a level-j boundary state's hereditary
signature unfolds to exactly a depth-4 nested multiset tree whose decorations
are (ε, classification, offset) and whose leaves are level-(j−1) boundary
classes. Hence A_j is, definitionally, the realized subset of Tree₄(A_{j−1})
(the depth-4 signature-tree algebra over the alphabet); mid-trick classes are
the interior subtrees. The yard is therefore a REFACTORING of r3's equivalence
— not a new equivalence — provided the handoff alphabet is exactly A_{j−1}
(the r3 classes; any coarser handoff summary forfeits the identity), and it
inherits r3's ECL receipts ON THE MEASURED CARRIER only. Extending the yard to
the full game is new territory; receipts never promote (project tier rule).

## Y2 — periodicity. RULING: split the claim; half is a theorem obligation, half is a payoff measurement.

**(P1 — grade-free uniformity; PROVE, do not measure.)** The signature-tree
functor Tree₄ and the map tree ↦ abstract one-trick kernel are grade-free:
(i) the primitive-step rules (legality, tier, winner, ε — v0.4 §1.2–1.3) take
no trick index — legality and trick keys are functions of state content only;
(ii) by r3's construction the class IS the signature and the signature's root
multiset IS the transition law, so a yard state's abstract dynamics are
determined by its tree, with no residual level dependence. Consequence: the
yard transducer is AUTOMATICALLY lawful and grade-free; "every trick is the
same object" is exactly P1 and is provable, essentially from §1.3's silence
about trick index plus the r3 construction. Obligation list: (a) step/emission
functions have no grade argument; (b) within-trick preamble/offset evolution
is level-free (leader rotation, fixed clockwise successor); (c) tree
determines kernel (immediate from Q1–Q3 of the r3 ruling).

**(P2 — self-similarity of the realized image; MEASURE, cannot be proved.)**
What varies per level is which trees are realized: hands of size j support
richer leader-choice multisets than hands of size j−1. Define a tree's SHAPE
as the tree with leaves abstracted to their equality pattern (which leaves
coincide — cf. v0.4 §3.4, v0.5's equality-pattern quotient; leaf identity
matters exactly through coincidence). The compression payoff claim is:
realized shapes substantially recur across levels, so the yard's shape
inventory grows far slower than the ~370×/trick class growth. This is the
refutable claim. Note the reclassification: the proposed refuter (slough
availability varying with remaining-hand shape) CANNOT break soundness — under
P1 hand-shape variety is fully recorded in the tree — it can only appear as
shape growth, i.e., it threatens the payoff, not the mathematics.

**Finite tests on existing data.**
1. (Verifies P1 / the refactoring.) Recompute A_j from A_{j−1} for every level
   present in the t5/t6/t7 class DAGs using ONE shared grade-free routine;
   byte-compare against r3's directly computed classes. Any mismatch refutes
   the factoring implementation or exposes a hidden grade dependence.
2. (Measures P2.) Count distinct realized shapes per level and the cross-level
   shape overlap (same equality-pattern tree up to alphabet bijection). Payoff
   confirmed if shape counts are small/overlapping across levels; refuted if
   shape growth tracks class growth (the ~370× then lives in shapes and the
   yard buys little).
3. (The lead's isomorphism test, scoped correctly.) Where an alphabet
   bijection matches sub-alphabets of two levels, the induced within-trick
   sub-DAGs MUST be isomorphic — by P1 this is a consequence, so run it as a
   cross-check of the implementation, not as the periodicity experiment; a
   failure here is a bug or a P1 proof error, never a new result.

## Y3 — pruning operator. RULING: CONFIRMED, with the vocabulary discipline mandatory.

For a seat (kernel K, fiber Φ(K), belief β): map each root world to its root
class, take the class image plus abstract-kernel reachability closure — under
the uniform-legal field every legal transition has positive mass, so closure =
DAG reachability, and at focal choice nodes ALL legal abstract actions are
included. Searching this live sub-DAG with K̄ is exact by v0.5 conclusions 1–2
(β̄ = d_#β filters exactly by K̄ alone; abstract policies lift to lawful
concrete policies with the same joint law) and conclusion 7 (V/Q equality).

The seat MAY conclude: exact abstract filtering over classes; exact count-free
value and per-action Q, over the transported abstract-policy class; support
facts (a class outside the sub-DAG is inconsistent with the seat's
information). The seat may NOT: (a) present the sub-DAG as belief — it is a
SUPPORT object; classes in it can carry zero pushforward mass (support ≠
belief); (b) claim the unrestricted concrete optimum — v0.5's BOUNDARY leaves
open whether it is attained in the transported abstract-policy class; quote
values as "exact over the lifted policy class" until a sufficiency theorem
closes that gap; (c) read any count/valuation conclusion from the count-free
DAG — valuation re-enters only through declared roles (v0.5 role re-entry and
the fixed-valuation stabilizer boundary), and O_Σ = ∅ here; (d) prune by
sampling — the operator is exclusion-complete closure or it is not exact.

**Unsound-as-posed summary:** the macro-step reading of the one-trick contract
(Y1 correction); the A_{k+1} alphabet indexing (direction fixed above); and
treating the slough-availability observation as a periodicity refuter (it is a
payoff refuter; P1 makes soundness immune to it).

# Shape notion v2 (walt-math, 2026-08-10; repaired instrument — a new declared measurement, never a mutation of the committed run)

Diagnosis first, because it dictates the repair: the arity leak is NOT
root-local. At every node where the actor is unconstrained the menu is the
whole hand — a leader "may play any remaining tile" and a follower unable to
follow "may slough" any tile (v0.4 §1.5) — so lead AND slough nodes have arity
exactly j at level j, by rule. Candidate (a) (abstract the root arity into a
multiset of option-shapes) therefore only relocates the blindness to slough
nodes and is REJECTED as the primary repair; candidate (b) (root sections)
contains the same slough leak one level down; candidate (c) (mid-trick
anchoring) helps only at forced-follow nodes. The correct object, from the
structure of Tree₄ itself, is candidate (d):

## Definition — the suffix library, in two declared variants

Cut every level-j signature tree at every node. A depth-d SUFFIX (d = 1..4) is
the decorated subtree below a node: node decorations (actor offset, constraint
type ∈ {lead, forced-follow, slough}), per-move (ε, classification), leaves =
holes with the equality pattern RECOMPUTED LOCALLY within the suffix (leaf
coincidences crossing the cut are dropped — without this the library is
ill-defined). All suffixes are content-addressed (hash-consed).

- **v2-strict:** option multisets kept everywhere (arity is honest data).
  Lib_d^s(j) = the set of distinct depth-d suffixes realized at level j.
- **v2-open:** at UNCONSTRAINED nodes only (lead, slough) the option multiset
  is replaced by the SET of distinct option-shapes; forced-follow nodes keep
  their multisets (their arities are suit-split facts that can coincide across
  levels). Lib_d^o(j) analogous.

Justification for the open abstraction: at an unconstrained node the menu SIZE
is the level, definitionally; the menu CONTENT (distinct option types) is the
machinery. v2-open quotients out exactly the rule-forced coordinate and
nothing else; the strict/open gap isolates the forced-arity contribution
instead of hiding it. Both variants are INSTRUMENTS, not carriers: neither
satisfies (ECL) (v2-open even alters chance arities), no value or class claim
may ever be read from a shape count, and shapes sit below every tier.

## Q2 — refutation criterion and what recurrence buys

Metrics per variant and depth: library growth g_d = |Lib_d(j+1)|/|Lib_d(j)|
against class growth |A_{j+1}|/|A_j|; cross-level overlap ω_d(j,j′) =
|Lib_d(j) ∩ Lib_d(j′)| / |Lib_d(min)| — normalize by the smaller level, since
a suffix containing any arity > j′ cannot occur at level j′ (the overlap is
structurally asymmetric; report it as such).

- **Payoff CONFIRMED** if, for d ≤ 3, v2-open library growth is far below
  class growth and ω_d is high: the parts recur; only menus over shared parts
  grow. This certifies the SHARED-MACHINERY payoff: A_j representable over a
  cross-level hash-cons store, per-level cost ≈ new library entries + menu
  multiplicity tables over shared option types.
- **Payoff REFUTED** if v2-open depth-≤3 growth is the same order as class
  growth — the diversity then lives in genuinely new sub-parts per level and
  the yard's state-inventory reuse is small.
- v2-strict overlap additionally certifies LITERAL state sharing; expect
  lead/slough-containing strict suffixes to be level-pinned by construction —
  quantify that fraction rather than lamenting it.

What no recurrence result buys: shared MENUS (provably level-pinned — lead
arity = j is a rule, not a finding) and any lawful equivalence (instrument
tier). What stays true regardless of outcome: P1 — the yard as ONE grade-free
transition program is a theorem; v1/v2 measure only whether the state
inventory also compresses. Both outcomes are results.

## Q3 — sanity ruling: YES, bracket the recurrence

The committed within-level numbers (23,592 → 10,978 shapes, 2.1:1; shape
growth 171× vs class growth 368×, same order) already show the pure
equality-pattern abstraction discards all leaf identity yet buys barely 2× —
the diversity is STRUCTURAL, not leaf-labeling, so no leaf-side abstraction
alone can carry the payoff, and v1's whole-tree recurrence rows would likely
have disappointed even without the arity artifact. Say this plainly in the
results. Therefore measure v2 alongside ONE declared refinement rung to locate
where recurrence lives between shapes and classes: the HEREDITARY-SHAPE rung —
replace each leaf not by a hole but by the shape (same variant) of that leaf
class's own tree one level down, recursing to the terminal class. This rung is
the natural self-similarity instrument for a claimed-periodic machine (if the
game is one machine iterated, hereditary shapes are what should stabilize) and
sits strictly between shapes and classes. Report the compression ladder per
level: classes → hereditary shapes → shapes; the rung where the big ratio jump
occurs is where the recurrence actually lives.

Procedure discipline: v1 rows stay as committed (runs are never mutated); v2
and the hereditary rung are new declared measurements in a new results file,
headed by the instrument-tier caveat above and paired with the run's
determinism freezes (content-addressing is already frozen per r3 Q5.3).

## Fiber-probe rulings (P-Q1..P-Q6) — 2026-08-11

**Adjudicator:** walt-math. **Tier:** exploratory throughout, with one declared
INSTRUMENT (the fold weighting, P-Q3); nothing below changes the status of any
class or value claim. **Basis:** v0.5 §12.6A and its BOUNDARY; v0.4 §2.1–2.6,
§5.5, §7.4–7.7, §9.9, §10.3, §10.8, §11.7, §12.4, §12.6, §17.5. F1–F7, r3
Q1–Q5, yard Y1–Y3 and shape v2 are inherited unchanged. Amendments are numbered
P-A1.. and are builder obligations; a probe run that omits one is not the
adjudicated probe.

### Lemma V (value descends to r3 classes) — used by P-Q2 and P-Q3

Let an operator assign to each state a value by a node rule that reads only
(i) the actor offset and (ii) the canonically ordered family of per-move pairs
(k, value of successor). Then the value is constant on r3 classes. *Proof.* By
induction on grade (the carrier is strictly graded, r3 Q2). Grade 0: one class,
value 0. Grade g: r3-equal states have equal preamble (grade, actor offset) and
equal canonical-ordered per-move tuples (k, classification, successor class)
(r3 Q3); by the induction hypothesis the successor classes determine successor
values; the node rule applied to identical inputs returns identical values. ∎

Lemma V covers the world-informed focal-max / hidden-uniform-expectation
operator (F4's uniform-legal field; treatments C and F of v0.4 §10.3) and
the PI minimax operator of §9.9 (max at offsets 0,2; min at 1,3). It does NOT
cover treatment H, the actual hidden-information solve (§10.3): H's value at an
information state is a function of the bag and its weighting, not of any single
state's class. H is governed by Y3 instead — search the live sub-DAG with K̄ —
and there a change of weighting is a RE-SOLVE over a fixed DAG, never a re-fold.
This distinction is the spine of P-Q2 and P-Q3 and must not be blurred.

### P-Q1 — coordinate and fiber. RULING: ACCEPT WITH FOUR AMENDMENTS.

**Lawful as posed, on two independent grounds.** (a) *History-forgetting at the
class layer is not a restriction at all.* The r3 carrier is hands + table +
actor (F5) and a class identity is a function of the future cone alone (r3 Q4);
no r3 object ever reads history. So a coordinate that carries the focal hand,
the declaration, the leader and the live set determines every class in its
fiber, exactly. (b) *The proposed fiber is a well-formed support object.* It is
Φ(**C**₀) for the capacity-cell system **C**₀ = (U; P_s = U, k_s = n) of v0.4
§2.1 — a feasible cell system, hence carrying a canonical exact support normal
form (§2.2), and |Φ(**C**₀)| = (3n)!/(n!)³ = C(3n,n)·C(2n,n), the figures the
design quotes.

**But the dropped void inferences are a DECLARED SCOPE RESTRICTION, not a
non-issue, and are a flaw if left undeclared.** §2.1 fixes the seat's real
fiber as Φ(**C**) with P_s the tiles locally possible after all rule-derived
exclusions — public sloughs induce exact void constraints — so Φ(**C**) ⊆
Φ(**C**₀), generally strictly (the committed pruned census already shows this:
trick-six kernels with fibers of 36 and 27 against the void-free 90;
`results/census_pruned_2026-08-10.txt`). §2.2's BOUNDARY then bites: feasibility
and exact normal-form decoding do not imply legal Straight reachability, and a
support object used as a current game state must be inherited from legal
construction or carry a reachability witness. The coordinate itself is
inherited from the receipt; **C**₀ is not.

- **P-A1 (vocabulary, mandatory).** Members of Φ(**C**₀) are FEASIBLE worlds,
  never "reachable" and never "the seat's fiber" (feasible ≠ reachable is a
  typed distinction). The results file names the object the *void-free capacity
  fiber* and states verbatim: "This is Φ(**C**₀), the void-free capacity-cell
  fiber, a declared superset of the seat's actual support Φ(**C**) (v0.4 §2.1);
  the void constraints derivable from the actual play history are deliberately
  dropped. It is a declared cost domain. No support fact about any seat may be
  read from it, and Y3's exclusion conclusions do not apply to it."
- **P-A2 (report the gap).** Per coordinate, print |Φ(**C**₀)| and, where the
  kernel builder already supplies P_s, |Φ(**C**)| and the ratio; where it does
  not at that rung, print that it was unavailable. Silence about the gap is not
  permitted.
- **P-A3 (monotonicity is one-sided — the ratio is not a bound).** Both arm
  costs on Φ(**C**₀) are upper bounds for the same arm on Φ(**C**) (raw: fewer
  worlds, same per-world cost; memoized: a subcarrier adds no classes and
  splits none, r3 Q4). The CRUSH FACTOR is a ratio of two upper bounds and is
  therefore a bound in neither direction; label it "measured on the void-free
  capacity fiber" and never transport it to the seat's real fiber.
- **P-A4 (declare the conventions the matching does not fix).** F2 A4 fixes
  focal↔focal but not who focal is; declare "focal = the declaring seat" in the
  header. Declare the leader as an offset from focal ∈ {0,1,2,3} (r3 Q3), not by
  an orientation-flavoured word — §11.7 and F2 A4 forbid reflection-dependent
  seat language.

At a trick boundary there are no unresolved-trick tiles, so F2 A1's extension is
inert here and each rung n is exactly a level-j = n boundary fiber in the
railyard indexing (Y1). Nothing further is required for well-formedness.

### P-Q2 — the raw baseline. RULING: ACCEPT-WITH-AMENDMENT; the baseline as posed is a strawman and is replaced by a three-arm ladder.

**The object must be named with the project's own operator vocabulary, not
invented.** "Solve every world independently" is treatment C/F (world revealed;
v0.4 §10.3, `walt-strat/src/revealed.rs`) or the PI operator (§9.9,
`walt-strat/src/pi.rs`) — it is NOT treatment H, the seat's actual hidden solve
(`walt-strat/src/hidden.rs`). §7.6 (strategy fusion) and §7.7 (operator
boundary) make the difference a difference of games, and §12.4/§17.5 record the
nonclaim that worldwise perfect-information classes are the correct
hidden-decision classes.

- **P-A5 (declare the operator).** The header names the operator by §10.3
  letter (or "PI, §9.9") and states: "the quantity computed per world is
  world-informed; it is not a seat value. No V or Q of any seat is claimed."
  Recommended choice: the F4 uniform-legal field with focal informed, because
  the r3 classes were built and ECL-verified under exactly that kernel; the PI
  operator is permitted but then §10.8's caution is reproduced.
- **P-A6 (count-free contract).** Valuation is `q_trick` only (each trick ±1,
  affine in v0.5 conclusion 4's t_T·e⋆). `q_points` is FORBIDDEN in this probe:
  it is tile-count-bearing, leaves the count-free contract under which the
  classes were verified, and would require declared role re-entry (v0.5 role
  re-entry; Y3(c)). Assert the valuation in-run. Exact i128 integers and
  rationals throughout (F4); no floats anywhere, including timings and ratios
  (P-A19).

**The honest baseline is not a cacheless tree walk.** `walt-strat/src/scalar.rs`
already implements "one scalar PI solver with a boundary cache shared across
every world and root it is asked about," keyed on the packed semantic state
(hands, leader) — i.e. the identity-transport control that F6 already mandates
("§12.6 is recovered exactly by identity transports; the identity-interface
count on the same carrier is the canonical control"), and its own comment
records that worlds of one fiber share suffix states heavily. Measuring the
class DAG against a cacheless tree would credit the equivariance machinery with
ordinary transposition memoisation that the project has already banked.

- **P-A7 (three arms, mandatory).** A0 = per-world backward induction, no cache
  (tree). A1 = identity-key content-addressed DAG, cache shared across the whole
  fiber (F6's control; the `scalar.rs` key). B = r3-signature content-addressed
  DAG. Report:
  A1/A0 = memoisation dividend; **B/A1 = the equivariance dividend proper (the
  headline)**; B/A0 = total dividend, labelled "includes ordinary memoisation".
  Quoting B/A0 as "the crush" is forbidden.
- **P-A8 (raw pays no class cost; memoized pays all of its own).** A0 and A1
  must not canonicalise, hash a signature, or allocate a class object — class
  membership is not needed for their output. B's canonicalisation, hashing and
  cache probes are inside B's measured wall-clock. Move generation, fiber
  enumeration (same order), the leaf rule and the final aggregation are shared
  code across all three arms.
- **P-A9 (same-object receipt).** For every world evaluated by more than one
  arm, assert bit-exact equality of the per-world value across arms, and assert
  value(world) = value(root class of that world) for arm B (this is Lemma V's
  in-run check and the F6/Q5.2 "by construction is not a receipt" discipline).
  Any mismatch: stop and report per NO-RESCUE; never patch. Without this
  receipt the comparison is void, because unequal outputs make the ratio
  meaningless.
- **P-A10 (no pruning in any arm).** No alpha-beta, no bound-based cutoff, no
  move ordering that changes the visited set. Node counts are then
  order-independent and the arms are comparable. This also disposes of the
  design's M1/M2/M3 framing: **M2 ("exact pruning — infeasible branches die at
  classification") is REJECTED as a mechanism.** There are no infeasible
  branches in a legal-move enumeration; what M2 gestures at is either the
  root-world→root-class collapse (already measured WEAK, and reportable as its
  own line, not as a mechanism) or a sampling-style cut, which Y3(d) forbids
  outright. Delete M2 or restate it as the root-collapse line.
- **P-A11 (node counts never stand alone).** A class-DAG node costs more than an
  identity-cache probe, so a node-count ratio is not a cost ratio. Every arm
  reports both integer node counts and integer wall-clock, and every quoted
  crush factor is paired with its wall-clock counterpart.

### P-Q3 — the re-weighting. RULING: REJECT the name; ACCEPT the object under the corrected name and two amendments.

"Support-side re-weighting instrument" is backwards and blurs the ledger it
claims to respect. §2.5's noncollapse ledger types Φ(K) as rule-compatible
current remainders, explicitly *not* a probability measure; the instant unequal
weights are attached to members you have a measure-shaped object, so it is not
support. It is equally not a belief: nothing normalises it, no field likelihood
or evidence produces it, and §2.4's β is the viewer's normalised belief on
Ξ(K,e). It is an aggregation argument in the sense of §5.5.

- **P-A12 (name, mandatory).** Call it the **declared fold weighting (timing
  instrument)**: a deterministic, unnormalised, integer-valued weighting over
  feasible fiber members, whose only function is to force a genuinely different
  second aggregation so the re-fold can be timed. The results file states:
  "The fold weighting is neither support nor belief (v0.4 §2.5); it is an
  aggregation argument (§5.5) chosen to time a second fold. No number produced
  under it is a value claim, a belief, or a support fact." Tier: INSTRUMENT,
  below every tier, cited by nothing above it. Publishing its wall-clock ratio
  makes no value claim — CONFIRMED, given this labelling.
- **P-A13 (state the weighting orientation-freely and freeze it).** "Number of
  trumps the world assigns to the focal seat's left opponent" is
  orientation-flavoured language (§11.7, F2 A4). Declare it as offset from focal
  ∈ {1,2,3} — e.g. weight(ω) = 1 + |trumps assigned to offset 1|, exact
  integer — and freeze the definition (freeze 9, P-A18).

**The amortisation claim (M3) must be narrowed, and this is the ruling that
changes what the probe buys.** "Support-level values are weighting-independent,
so a second evaluation is a re-fold over a fixed spine" is TRUE for the
per-world operator (Lemma V: per-world values are class functions and the
weighting enters only the final aggregation) and FALSE for treatment H, the
seat's actual object: H's solve depends on the bag weighting, so a re-weighted
H is a re-solve over the fixed class DAG, not a re-fold. The design's payoff
sentence ("beliefs/policies iterate on re-folds") is therefore about C/F, not
about the seat.

- **P-A14 (the H row).** The results file states plainly: "The measured
  amortisation is for the world-informed per-world operator. It does not
  establish amortisation for treatment H, where a change of weighting is a
  re-solve over the fixed class DAG (Y3), not a re-fold." Additionally, at the
  smallest rung where it completes inside the declared budget, run H (the
  existing `hidden_root_values` shape) cold and then over the pre-built class
  DAG under two weightings, and report the re-solve/first-build ratio; if it
  completes at no rung, print that fact with the rung attempted and the stop.
  This is the number the "platform for belief/policy iteration" claim actually
  rests on.

### P-Q4 — declared stops. RULING: REJECT the prefix; ACCEPT a declared deterministic decimation.

A prefix of a deterministic enumeration order is a biased estimator of per-world
cost, and biased in a way that is structural rather than incidental: any lex- or
colex-style enumeration of assignments of 3n tiles varies its later coordinates
fastest, so the first W worlds share most of one seat's holding. Per-world tree
size depends strongly on exactly that (trump concentration drives forced-follow
versus free-choice arity, v0.4 §1.2, §1.5), so the prefix samples a
non-representative slice of the cost distribution. "No silent cap" is satisfied
by a prefix; unbiasedness is not, and the design asks for a per-world cost
estimate.

- **P-A15 (lawful stop rule).** Replace the prefix with a fixed multiplicative
  decimation: with N = |Φ(**C**₀)| and a declared constant g with gcd(g, N) = 1,
  evaluate the worlds at enumeration indices (i·g mod N) for i = 0..W−1, with g
  and W declared in the freeze header. Deterministic, no seed, no Date, no RNG,
  and it spreads the sample across the whole order. Report the per-world node
  count as an exact rational mean (sum/W) together with min, max and the
  quartile boundaries, so the estimator's dispersion is visible. All arms that
  are compared at that rung evaluate the SAME index set.
- **P-A16 (how the n=6 ratio is stated).** Arm B runs the full fiber where it
  can. The n=6 crush is then (raw mean × N) / (B full-fiber total), with the
  numerator flagged ESTIMATED and the estimator (g, W, dispersion) printed
  beside it. If B also stops, print worlds processed and the index of the last
  world evaluated, so the run is reproducible and resumable. Every stop printed,
  no silent caps — CONFIRMED as the design already requires.

### P-Q5 — cache persistence. RULING: ACCEPT WITH ONE AMENDMENT.

Sound: class identity is intrinsic to the continuation (r3 Q4), so a class
computed at level j is literally the same object when met again after a played
trick; carrier growth adds classes and never splits them. A warm cache is
therefore exact, not an approximation.

- **P-A17 (report the informative number).** Y1's stacking makes interior
  survival near-certain — the level-j DAG already contains the level-(j−1)
  classes as its interior — so a warm/cold wall-clock ratio alone mostly
  restates the stacking theorem. Report the ROOT-side figure: of the advanced
  coordinate's distinct root classes, the fraction already present in the
  level-j store. The advanced coordinate must use the same void-free scope
  (P-A1) or the two rungs are not comparable, and a warm cache may never be
  inherited across a change of freeze set.

### P-Q6 — results discipline and freezes. RULING: ACCEPT, with the freeze list below mandatory.

Freeze numbers 1–6 are taken (r3's two; the yard's tree encoding and shape
canonical form; v2's suffix cut and open variant). The probe's freezes therefore
begin at **7**, and the header restates r3's 1–2 and the yard's 3 unchanged.

- **P-A18 (new freezes).** (7) the fiber enumeration order, stated precisely
  enough to reproduce an index → world map; (8) the decimation rule (g, W) per
  rung, per P-A15; (9) the fold weighting definition (P-A13); (10) the operator
  and valuation (P-A5, P-A6); (11) the per-arm key functions — A1's packed
  semantic-state key and B's r3 128-bit FNV-1a signature — since node counts and
  hit counts are only reproducible against them.
- **P-A19 (timing discipline).** All arms single-threaded (or one declared
  thread count for all arms) and executed in one run on one machine; record CPU
  model, core count and build profile. A wall-clock ratio assembled across
  machines, builds or thread counts is void and must not be printed. Wall-clock
  in integer nanoseconds; every ratio an exact rational rendered by integer
  division. No f32/f64 anywhere in the probe.
- **P-A20 (inherited boilerplate).** Reproduce, verbatim, r3 Q4's mandated
  caveat block; the tier lines (exploratory; the fold weighting INSTRUMENT); the
  SINGLE-IMPLEMENTATION provenance line; the regenerate line; and P-A1's
  void-free scope paragraph. Pair every class count with its ECL verdict line
  (F6) — the probe imports the r3/t5 verdicts, it does not re-earn them.
- **P-A21 (extrapolation).** Three rungs are not a law. The per-rung ratios are
  the headline; a fitted exponent may accompany them, computed in exact
  rationals, labelled "extrapolation, exploratory tier", and phrased as "the
  implied n=7 cost under the fitted law" — never as a statement about an unrun
  computation, and never as a feasibility claim. The n=7 fiber is the deal
  itself (399,072,960 void-free worlds), where the void-free scope restriction
  of P-A1 is at its widest relative to any real seat.

**Both outcomes remain results** (F7, NO-RESCUE): a weak B/A1 dividend is a
proved negative about this route to n=7 and changes nothing about the classes'
existence or their ECL receipts.

## Fiber-refinement rulings (X-Q1..X-Q7) — 2026-08-11

**Adjudicator:** walt-math. **Tier:** exploratory; every X predicate is a
DECLARED object and nothing below changes the status of any class or value
claim. **Basis:** v0.5 §12.6A and its BOUNDARY; v0.4 §2.1–2.6, §5.5, §6.7–6.8,
§7.4–7.7, §10.3, §12.4, §17.5. F1–F7, r3 Q1–Q5, Y1–Y3, shape v2 and the
fiber-probe amendments P-A1..P-A21 are inherited unchanged. Amendments are
numbered X-A1.. and are builder obligations.

### The typing that governs this whole section

Three objects that the design's language risks fusing:

- **support** — rule-derived (§2.1: Φ(**C**) from legality plus the actor-
  attributed public prefix). Excluded ⇒ *cannot occur*.
- **belief** — a normalised weighting on the support (§2.4). Down-weighted ⇒
  *unlikely*.
- **declared exclusion** — an analyst's predicate. Excluded ⇒ *neither*. It
  carries no epistemic content whatsoever: the worlds are still feasible, still
  possible, and still carry whatever belief mass they had.

§6.8 names the operation exactly: forming a remnant is **analyst conditioning**
on a declared event E, and analyst conditioning "keeps the player's policy class
fixed" — J(ρ|E) = E_{β(·|E)}[U|ρ]. Treating it as player revelation "leaks
hidden information and recreates strategy fusion" (§6.8, §7.6). So the default
rule for every predicate below is: **evaluate a fixed policy on a remnant, never
re-optimise over a remnant and call the result a seat value.** Lemma X is the
one exception, and it is one-sided.

### Lemma X (zero-contribution excision) — the mathematical content of "except X"

Let the valuation be non-negative (q_trick: the focal partnership's trick count,
P-A6). For a world ω write V*(ω) for its Lemma-V value under the frozen
world-informed operator, and U(ω,ρ) for the field-expected focal trick count in
ω under an information-consistent policy ρ. Let Z = {ω : V*(ω) = 0}.

*Claim.* For every information-consistent ρ and every ω ∈ Z, U(ω,ρ) = 0.
Consequently, for the unnormalised objective J(ρ) = Σ_ω β(ω) U(ω,ρ), deleting Z
from the bag leaves J unchanged as a function of ρ; hence the argmax set and the
unnormalised optimal value are preserved exactly.

*Proof.* Every information-consistent ρ induces some world-dependent behaviour
in ω, and V*(ω) is the maximum over all world-dependent behaviours, so
U(ω,ρ) ≤ V*(ω) = 0 (this is the pointwise form of §7.6's fusion inequality).
Non-negativity gives U(ω,ρ) = 0. The deleted terms are therefore identically
zero in ρ, so J is literally the same function on the remnant. ∎

Three consequences the builder must respect:

1. **The lemma is one-sided.** V* is an *upper* bound, so V*(ω) = 0 forces the
   contribution to zero, while V*(ω) = n forces nothing — an
   information-consistent policy may score less than n there. **X_val_max is
   therefore NOT the symmetry check the design calls it**; see X-Q2.
2. **Normalisation breaks it.** Under β(·|Z^c) the optimal value is the
   unnormalised optimum divided by (1 − β(Z)) — a different number, and not any
   seat's value. Report unnormalised sums and the excluded mass β(Z) explicitly.
3. **Policies are preserved only up to vacuity.** Deleting Z can empty an
   information state; the policy there becomes arbitrary and has no effect on J.
   Values are preserved; policy identity at emptied states is not.

### X-Q1 — naming and tier of the refined object. RULING: ACCEPT the name, with the type line and prohibition list mandatory.

**DECLARED EXCLUSION REMNANT** is confirmed, in full as *the declared exclusion
remnant of the void-free capacity fiber* — a declared search/cost domain of the
same kind as Φ(**C**₀) itself, one further step removed from the seat. Members
remain FEASIBLE worlds (P-A1's vocabulary is unchanged; feasible ≠ reachable).

- **X-A1 (mandatory type line, verbatim in the results file).** "A remnant is a
  declared exclusion remnant of the void-free capacity fiber: analyst
  conditioning (v0.4 §6.8) on a declared, non-evidential predicate. Exclusion by
  X does not mean the world cannot occur (that is support, §2.1) and does not
  mean it is improbable (that is belief, §2.4). No support fact, no belief, no
  seat value, and no reachability claim may be read from a remnant. Excluding X
  never places X's falsity into any seat's information state; doing so would be
  player revelation and would recreate strategy fusion (§6.8, §7.6)."
- **X-A2 (what a remnant may carry).** A remnant is quotable for exactly three
  things: its size, the cost of computing it, and the cost of evaluating over
  it. Any value computed on a remnant is J(ρ|E) for a FIXED ρ, or else falls
  under Lemma X and carries X-A6's conditions.

### X-Q2 — lawfulness of each predicate. RULING: ACCEPT X_reach and X_val0 with amendments; REJECT X_val_max's stated role.

**X_reach(F) — lawful, but the quantifier must be in the name.** Reachability of
F on the class DAG is a function of the future cone alone, hence well defined on
r3 classes and stable under carrier growth (r3 Q4). But "the future *can* pass
through a forbidden class" (∃ a path) and "*every* continuation ends in F" (∀
paths — confinement) have opposite soundness properties: the ∀ version is a
zero-contribution statement in Lemma X's sense when F is the zero-trick terminal
set, the ∃ version is not remotely one (the focal seat may steer away, and at
focal nodes all legal abstract actions are live, Y3).

- **X-A3.** Predicate names carry their quantifier: **X_reach∃(F)** and
  **X_conf∀(F)**. Both are lawful as declared cost-domain predicates. Only
  X_conf∀ may ever appear in a Lemma-X argument, and then only with X-A6's
  conditions. Note for the record that X_conf∀(zero-trick terminals) and X_val0
  coincide under q_trick — the probe should say so rather than report them as
  independent findings.

**X_val0 — lawful, and it is the one predicate with mathematical content
(Lemma X).** It reads Lemma-V values under the frozen operator, so it is
policy-relative and inherits the v0.5 boundary.

- **X-A4 (boundary sentence, verbatim, wherever a value predicate appears).**
  "Value equality in conclusion 7 is over the transported abstract-policy class,
  exactly as in v0.4 §12.6's conclusion 4. Whether the unrestricted concrete
  optimum is attained inside that class is a separate sufficiency question,
  deliberately not claimed here." Together with P-A5's sentence: the quantity
  read is world-informed and is not a seat value.

**X_val_max — the stated role is REJECTED; the object is retained, re-typed.**
Calling it "the dual" and "a symmetry check on the machinery" asserts a symmetry
that does not exist: Lemma X's argument runs on the upper-bound direction only.
Excluding V* = n worlds from any solve is a genuine information injection and
would change the problem.

- **X-A5.** X_val_max is retained solely as a cost-domain / bite predicate and
  is labelled "not excisable — one-sided; see Lemma X consequence 1". It is
  never used to prune an evaluation whose value is quoted. If a machinery check
  is wanted, the lawful one is the Lemma-X receipt of X-A6, not a claimed
  symmetry.

**Stamping flags onto the store — lawful cached derivation, with two
conditions.** A predicate that is a function of the future cone is well defined
per class and stable under carrier growth (r3 Q4), which is exactly what makes
caching it legitimate rather than a stored authority. But the project rule is
derived views, never stored state, so:

- **X-A6 (flag discipline).** (i) A flag is keyed by (predicate id, freeze-set
  id), never by class id alone — a store carrying flags computed under a
  superseded freeze is corrupt, not stale. (ii) The run recomputes the flag from
  the cone for a declared sample of flagged and unflagged classes and asserts
  agreement, in the P-A9 style; "cached by construction" is not a receipt.
  (iii) Any Lemma-X excision that is actually exercised carries its own receipt:
  on a declared sample, assert U(ω,ρ) = 0 for the excluded worlds under the
  evaluated policy, and assert that the unnormalised objective and argmax agree
  between the full set and the remnant. Failure: stop and report per NO-RESCUE;
  never patch.

### X-Q3 — exclusion semantics. RULING: ACCEPT world-level only; branch-level is REJECTED for this probe on two independent grounds.

World-level exclusion by a property of the root class is analyst conditioning on
a set of worlds, which is the object X-A1 types. Branch-level (mid-cone) cutting
is not a restriction of the same problem at all:

- at a **focal** node, deleting abstract actions contradicts Y3's exclusion-
  complete closure ("at focal choice nodes ALL legal abstract actions are
  included") and v0.5 conclusions 1–2, which lift abstract policies over the
  full abstract action set; what remains is a restricted-policy-class value, not
  the value;
- at a **hidden** node, deleting branches deletes mass from the uniform-legal
  field, i.e. changes the field and therefore the operator (F4, §7.7 — theorems
  for one operator do not transfer to another).

- **X-A7.** Branch-level exclusion is out of scope. If it is ever built it is
  declared as a NEW OPERATOR (a changed field and/or a restricted policy class),
  given its own freeze and its own receipts, and its numbers are never compared
  against this probe's rows.

### X-Q4 — what "bite" may claim. RULING: ACCEPT the reporting sentence with three additions.

- **X-A8 (the sentence).** "Bite is measured on the evaluated set of the
  void-free capacity fiber (P-A1, P-A3). A shrink factor is a statement about a
  declared cost domain, never about the seat's real support Φ(**C**), never
  about belief, and never about what can happen in the game."
- **X-A9 (two distinct bites, never conflated).** Report the **world bite**
  (excluded worlds / evaluated worlds) and the **class bite** (flagged classes /
  classes in the store) as separate rows. They answer different questions, and
  the class bite is carrier-relative (r3 Q4) — it is an inventory statistic of
  this store, not a property of the game.
- **X-A10 (bites from decimated sets are estimates).** Always print numerator
  and denominator as integers; a ratio may appear only beside them, as an exact
  rational, and is labelled ESTIMATED whenever the evaluated set is a decimation
  of Φ(**C**₀) (P-A15). At n=6 the evaluated set was W = 6; a bite ratio there is
  a six-point sample and must be printed as a bare count with the ratio
  suppressed or explicitly marked as such.

### X-Q5 — coordinates and freezes. RULING: ACCEPT, freezes continue at 12.

Freezes 1–3 and 7–11 are in force and restated; the refinement probe's new
freezes begin at **12**.

- **X-A11 (new freezes).** (12) each predicate's definition — F sets declared
  **intensionally** (e.g. "all terminal classes with t_T = 0"), never as a list
  of class hashes, since hashes are themselves freeze-dependent; print the
  resulting |F| beside the definition. (13) the flag keying of X-A6(i). (14) the
  store record format and its freeze-set digest (X-Q7).
- **X-A12 (sample identity).** Where arms are re-run, reuse S5h's coordinates
  and the same (g, W); where the store side uses the full evaluated set, say
  which set each row was computed on. A bite row and a cost row computed on
  different sets are not comparable and must not share a line.

### X-Q6 — results discipline. RULING: ACCEPT, with the anti-strawman amendment on multi-pass economics.

P-A20's boilerplate, the F7/NO-RESCUE both-outcomes framing, and one file
`results/fiber_refine_2026-08-11.txt` are confirmed.

- **X-A13 (pass-2 economics must not be measured against a rebuild).** "Pass 2
  over a store that pass 1 built is cheap" is true by construction and measures
  nothing. For each predicate, state whether it is decidable **without** the
  class store (X_val0 is: arm A1 produces per-world values at roughly a quarter
  of arm B's cost; X_reach∃/X_conf∀ are not, since bare semantic-state keys
  carry no cone identity — that is precisely S5h's finding) and report the
  pass-2 cost against the cheapest storeless alternative for that predicate, or
  state that none exists. A pass2 : pass1 ratio may be printed only alongside
  that comparison. This is P-A7's anti-strawman rule applied to the second pass.
- **X-A14 (carry S5h's negative forward).** The header states: "B : A1 ≈ 4.3–4.9
  at every rung — the class store is not a build accelerator; cone identity
  cannot short-circuit descent. Every payoff sought here is a pass-≥2 transport
  payoff, and the first build is a cost this probe does not recover."
- **X-A15 (remnant-evaluation row).** The design's honest possible outcome —
  that the store is already paid, so exclusion saves nothing at evaluation time
  — is a result and is reported as such (F7). State explicitly whether the
  remnant's evaluation reused the pass-1 store or rebuilt.

### X-Q7 — persistence. RULING: ACCEPT the discipline, with four additions.

Append-only content-addressing is the mathematically correct persistence mode
here, and for a stated reason: class ids are content addresses of the future cone
(r3 Q4, freeze 1), so records are immutable, and carrier growth adds classes and
never splits them (r3 Q4) — monotone append is exactly the lawful growth mode.
Wholesale invalidation on a freeze change is confirmed (P-A17: a warm cache may
never be inherited across a change of freeze set); partial reuse across freezes
is forbidden.

- **X-A16 (collision discipline crosses runs).** r3's in-run assertion that no
  two distinct signatures share a hash is weaker than what a store spanning many
  runs and coordinates needs, because the store enlarges the collision surface
  monotonically. Each record stores enough to verify identity (the signature
  bytes, or their length plus an independent second hash), and the loader
  asserts on every insert that an existing id's stored signature matches the
  incoming one. Without this the store's identity discipline is weaker than the
  in-run one and a silent collision corrupts every downstream number.
- **X-A17 (the store is a cache, never an authority).** Derived views, never
  stored state: a run must be able to rebuild any record from the carrier, and
  must assert agreement between rebuilt and loaded records on a declared sample
  before quoting anything from a warm store.
- **X-A18 (cone-intrinsic records only).** Records hold cone-intrinsic data —
  class id, successor ids, the Lemma-V value under the frozen operator,
  predicate flags per X-A6(i). World membership, coordinate identity, fiber
  indices, weightings and beliefs are coordinate-relative and must live outside
  the store; storing them would destroy the intrinsicness that licenses reuse.
- **X-A19 (warm reuse across coordinates is lawful, and re-types the counts).**
  Because classes are intrinsic, a store may be shared across coordinates — that
  is the actual payoff of persistence. But any count quoted from a warm store is
  store-relative, not carrier-relative: declare which carriers contributed to
  the store beside every such count.

**Both outcomes remain results** (F7, NO-RESCUE). A nil bite, or a bite that
saves nothing at evaluation time, is a proved negative about declared-exclusion
refinement on this route and changes nothing about the classes, their ECL
receipts, or Lemma X — which stands as a theorem about the objective regardless
of whether any predicate turns out to bite.

## Endgame-store rulings (E-Q1..E-Q7) — 2026-08-11

**Adjudicator:** walt-math. **Tier:** exploratory throughout. **Basis:** v0.5
§12.6A and its BOUNDARY; v0.4 §1.2–1.5, §2.1–2.6, §7.7, §11.1, §11.7, §12.4,
§14.7, §17.5. F1–F7, r3 Q1–Q5, Y1–Y3, shape v2, P-A1..P-A21 and X-A1..X-A19 are
inherited unchanged. Amendments are numbered E-A1.. and are builder obligations.

### Lemma E (structural isomorphism ⇒ count-free value equality) — replaces the proposed chain

The design justifies its lookup as: form-equal ⇒ r1-equal ⇒ (Q5.1) r3-equal ⇒
same value. **That route is unnecessary and its middle link is the weakest thing
in the design.** "r1 is a lawful (d,Θ)" is CHECKED, not proved — F3 said so
explicitly ("that it IS a bijection A(x) ≃ A(y) is exactly ECL condition 1,
which the census checks rather than assumes"), and Q5.1's refinement assertion
was an in-run check on a specific carrier. Routing an off-carrier lookup through
it inherits an unproved, carrier-scoped hypothesis for no gain. The direct
argument is stronger and needs neither ECL nor r3:

*Statement.* Let x, y have equal r1 canonical forms **as amended by F2 A1–A4**.
Then there is a bijection of live ∪ unresolved-trick tiles together with a
rotation of seats carrying x to y and preserving: holders by relative seat,
the trump/non-trump context split, follow membership, the led-context map ℓ,
pairwise trick keys, double flags, table order, current winner, and focal↔focal.
Every rule of the remaining game reads only those data: legality is "follow the
led context if able, else anything" (§1.2, §1.5); the trick winner is the
maximum trick key under the led context (§1.3); the next leader is the winner;
the count-free increment is e⋆ iff the winner sits in the focal partnership
(v0.5 count-free kernel, F5); the hand ends when live tiles are exhausted
(§1.5). Hence the map is an isomorphism of the remaining extensive games
carrying increments to increments and focal to focal, and every count-free value
that is a fold over that tree — in particular the frozen world-informed operator
of P-A5 and the PI operator of §9.9 — is equal at x and y. ∎

*Why the canonical form is available without descent.* The form reads tiles and
relations only, never the future cone. That is a genuine escape from S5h's
negative, and it is the ordinary mechanism of a symmetry-reduced tablebase.

- **E-A1 (attribution, mandatory).** The escape belongs to r1 — the FINEST
  STRUCTURAL quotient, i.e. relabeling symmetry — not to r3 and not to
  equivariant lumpability. Under F6 this probe measures the structural transport
  dividend, and S5h's negative about the r3 class store stands unrescued. The
  results file states: "The lookup key is the r1 structural canonical form. This
  probe measures a symmetry-reduced tablebase, not the equivariant class
  machinery; S5h's finding that cone identity cannot short-circuit descent is
  unchanged and is not rescued here."
- **E-A2 (the hard scope limit — count-free only).** Lemma E's bijection
  preserves BEATS relations, NOT pip counts. A canonical-form-keyed store is
  therefore sound only for count-free valuations (q_trick, P-A6). **If count
  ever re-enters (v0.5 role re-entry), every record in the store becomes
  unsound and the store is invalidated wholesale, never extended.** Print this
  in the store header and the results file; it is the one failure mode that
  would silently produce wrong numbers rather than a crash.
- **E-A3 (operator hypothesis).** Any operator used with the store is declared
  isomorphism-invariant. The F4 world-informed operator qualifies (legal sets
  correspond bijectively and the uniform mass depends only on |L|). An operator
  that reads tile identity does not.

### E-Q1 — the transport-lookup chain and its receipt. RULING: REJECT the proposed chain; ACCEPT the receipt, re-aimed.

Lemma E replaces the chain, so no off-carrier ECL hypothesis is in force and the
worry as posed ("an r1 transport unlawful off-carrier") dissolves: nothing here
asserts r1 satisfies (ECL) anywhere. What Lemma E *does* rest on is the
hypothesis that the implemented canonical form carries every relation in F2's
amended list. A dropped relation — the led-context map (A2), the unresolved-trick
tiles (A1), a reflected seat matching (A4, §11.7) — breaks the lemma. So the
sampled re-expansion is exactly the right receipt, aimed at the implementation
rather than at the mathematics.

- **E-A4 (the receipt).** For a declared deterministic stride over the hit
  sequence (every s-th hit, s declared per arm, plus the first and last hit of
  every coordinate — no RNG, no Date, per P-A15), expand the cone anyway and
  assert bit-exact value agreement. Expansion runs **to terminals, not to the
  floor**, so the receipt validates the composition of layers rather than one
  layer against itself. A mismatch is stop-and-report per NO-RESCUE and is a
  canonical-form implementation defect (or a defect in F2's list), not an ECL
  event; the results file must describe it that way if it ever fires.
- **E-A5 (one code path).** The floor build, the lazy insert and the forward
  evaluator call the SAME canonical-form function. Two implementations of a
  normal form is the classic way to make a store silently wrong, and no receipt
  short of full re-expansion would catch it.

### E-Q2 — the floor's completeness. RULING: ACCEPT completeness; the floor's VALUE as a mechanism is challenged and must be measured against a closed-form control.

**Completeness is sound.** Grade is a multiple of 4 exactly at trick boundaries
(every play removes one live tile, r3 Q2), so grade 4 ⟺ last-trick boundary with
exactly one tile per seat. The a1 domain — every ordered assignment of four
distinct dominoes to four seats (491,400), every leader (4), every focal seat
(4), every pip-trump declaration (7) = 55,036,800 — is closed-form countable and
is a superset of every reachable grade-4 pip-trump state. "Miss = bug" is
therefore sound, with one split:

- **E-A6 (miss taxonomy).** A miss is reported as either OUT OF DECLARED SCOPE
  (a doubles-trump or no-trump declaration — F1's scope, v0.4 §14.7/§17.5; stop
  and report a scope violation) or IN SCOPE BUT ABSENT (a genuine bug; stop and
  report). Never a fill, never a silent recompute.
- **E-A7 (floor build assertions).** Assert in-run: the enumerated total against
  the closed-form 491,400 × 4 × 4 × 7 = 55,036,800; the class total 64; and the
  anatomy rows the floor restates (16 per actor offset; 32/32 by increment;
  8 per classification pattern) against `census_a1_complete_2026-08-11.txt`. The
  a1 file is the standing record and the floor build byte-agrees with it on
  every number it restates.
- **E-A8 (two cardinalities, never conflated).** "55,036,800 situations, 64
  classes" describes the r3 alphabet. The STORE is keyed by canonical form, and
  r1 refines r3, so the store's record count is the number of distinct r1
  canonical forms at level 1 — a different and larger number that nobody has
  measured. Report it explicitly; it is what the floor's memory cost is, and
  the design's phrasing currently reads as though the floor has 64 records.

**The mechanism claim is challenged.** At a grade-4 state every seat holds one
tile, so the subtree below is a forced chain of four plays with a single leaf,
and the level-1 class is exactly (actor offset, three classifications,
increment) — i.e. the floor value is one trick resolution, computable in closed
form from the four tiles, the leader and the declaration, with no table and no
recursion. A floor lookup therefore replaces "resolve one trick" with "compute a
canonical form, then probe a hash" — the same trade S5h measured at 4.3–4.9×
against. The design's premise that "the bulk of every tree's nodes live below
the boundary" is true by node count and false by cost.

- **E-A9 (the floor's honest competitor).** T1 is measured against a CLOSED-FORM
  last-trick control — direct winner resolution at grade 4, no table, no
  expansion — not only against full expansion. Per P-A7 and X-A13, comparing the
  floor table to a four-ply forced walk is a strawman. If the closed-form control
  wins, the floor table is a negative result and is reported as one; that costs
  the probe nothing, since level 2 is where the design's real claim lives.
- **E-A10 (cost accounting per arm).** Report ns split as
  canonicalisation / lookup / recursion for every arm, in the style of arm B's
  existing carrier/r3/fold/lookup row. Without the split, a win or a loss cannot
  be attributed and the next iteration has nothing to act on.

### E-Q3 — record contents. RULING: ACCEPT the shape, with mandatory and forbidden fields fixed.

- **E-A11 (record).** MANDATORY: the canonical-form key; the count-free value
  under the frozen operator and valuation; the identifiers of the freezes that
  make the record meaningful (canonical-form encoding, operator/valuation) —
  keyed exactly as X-A6(i) keys predicate flags, since a record computed under a
  superseded freeze is corrupt, not stale. OPTIONAL: the r3 class id, and only
  when actually computed (E-A13). FORBIDDEN: anything coordinate-relative —
  world ids, fiber indices, hand ids, weights, beliefs, hit counters (X-A18);
  and any count-bearing value (E-A2).
- **E-A12 (X-A18 reading, clarified).** X-A18 said "cone-intrinsic"; the
  operative requirement is "not coordinate-relative". A structural key is
  state-intrinsic and finer than cone-intrinsic, and by Lemma E it is invariant
  under exactly the relabelings that preserve the continuation, so it satisfies
  X-A18. **Collision discipline (X-A16):** preferred at both layers is to store
  the canonical-form BYTES as the key, which makes verification automatic and
  X-A16 vacuous — endgame forms are small. If a hash is used for indexing, the
  record must still carry the bytes and the loader must compare them on every
  hit and every insert-collision.

### E-Q4 — what the lazy insert pays. RULING: ACCEPT the proposal; the retrograde signature must NOT be run.

Under Lemma E the value is licensed by the canonical form alone, so computing an
r3 signature at insert buys class identity the store does not use and pays the
descent cost the store exists to avoid. Computing the level-2 value by the A1
recursion with floor lookups beneath it is one-trick-deep and correct.

- **E-A13.** Lazy records carry no r3 class id. An absent id is recorded as
  absent, never defaulted and never inferred from the form. If a later probe
  wants class ids it computes them and says so.
- **E-A14.** Because a level-2 value is computed against the layer below it, the
  E-A4 sample expands level-2 records to terminals, independently validating the
  composition rather than trusting the floor.

### E-Q5 — the pathfinding framing. RULING: ACCEPT, with one boundary.

"Same fiber, different enumeration order" is exact: the value is a function of
the state, so evaluation order cannot change it, and bit-identical cross-arm
equality (P-A9) is the right receipt. Two guards:

- **E-A15 (order, not set).** Changing the ORDER of evaluation is lawful and is
  what this probe measures. Changing the SET — visiting only worlds that reach
  known solutions, "pathfinding to" rather than "through" — is a declared
  exclusion and falls under X-A1's typing wholesale. If any arm evaluates fewer
  worlds than T0, it is a remnant, not an ordering.
- **E-A16 (walls are stops, not findings).** A budget wall is a declared stop
  printed with what was reached (P-A16), never a statement about the game.

### E-Q6 — persistence mechanics. RULING: ACCEPT gitignored-cache, with a reproducibility amendment.

Not committing the store is correct and follows from X-A17: a cache is never an
authority, and a committed binary blob that nobody can review would become one —
receipts in this project are text, regenerable and byte-diffed, and a store is
not a receipt. Append-only content addressing remains lawful for the reason
already given (class and form identities are content addresses; carrier growth
adds and never splits, r3 Q4).

- **E-A17 (reproducibility through the cold arm).** Because the store is not
  committed, no number quoted from a warm store is reproducible from the
  repository alone. The results file therefore prints the COLD regenerate path
  that reproduces every headline number from an empty store, and any
  warm-only number is labelled as such. A warm run re-derives a declared sample
  and asserts agreement before quoting anything (X-A17).
- **E-A18 (store provenance in the results file).** Print: the freeze-set digest;
  the record count per layer; whether the run was cold or warm; the declared
  coordinate order; which coordinates contributed records (X-A19, counts are
  store-relative); and the wholesale-invalidation rule (freeze mismatch ⇒ the
  file is discarded entire, never partially reused, P-A17).
- **E-A19 (new freezes, continuing at 15).** (15) the canonical-form key
  definition and its byte encoding — previously internal to r1, now a persistent
  key and therefore a freeze; (16) the floor domain and its closed-form count;
  (17) the declared coordinate order for the warm arm and the E-A4 receipt
  stride. Freeze 14 (store record format and freeze-set digest, X-A11) is first
  implemented here and is restated, not renumbered.

### E-Q7 — results discipline. RULING: ACCEPT, with two additions.

P-A20's boilerplate, the per-arm per-coordinate rows, declared stops and the
F7/NO-RESCUE both-outcomes framing are confirmed.

- **E-A20 (saturation curves are order-relative).** Hit rate and store size
  against coordinates processed depend on the declared order; a different order
  gives different curves. Print the order and label the curves store-relative
  and order-relative — they measure this traversal, not the game. "Lots of
  convergence late game" is a claim about the corpus and the order, and is
  quotable only in those terms.
- **E-A21 (name the headline honestly).** T3 vs T0 measures cross-coordinate
  novelty against A1's within-coordinate cache, which is the right comparison
  and should be stated in those words. A weak T3 dividend is the convergence
  hypothesis measured small on this corpus — a result (F7), not a reason to
  re-run with altered arms.

**Both outcomes remain results.** Lemma E stands regardless of every number this
probe produces: it is a theorem about count-free values under structural
relabeling, and it is the first thing in this file that licenses a lookup
without any descent at all.

## Seat-census rulings (S-Q1..S-Q6) — 2026-08-11

**Adjudicator:** walt-math. **Tier:** exploratory throughout; this section is a
CENSUS adjudication — counts only, no values, no composition. **Basis:** v0.5
§12.6A and its BOUNDARY; v0.4 §1.2–1.5, §2.1–2.6, §5.5, §6.1, §6.8, §10.9,
§11.7, §12.4, §14.7, §17.5. F1–F7, r3 Q1–Q5, Y1–Y3, shape v2, P-A1..P-A21,
X-A1..X-A19, E-A1..E-A21 and Lemmas V, X, E are inherited unchanged. Amendments
are numbered S-A1.. and are builder obligations; a run that omits one is not the
adjudicated census.

**Headline of this section, stated first because everything else follows from
it.** A seat-side analogue of Lemma E exists and is proved below (Lemma S). It
is sound, it says exactly what the design hoped, and it is *empty as a
compression mechanism at the first play*: the pip-trump relational structure on
the full 28-tile live set admits no nontrivial self-transport (Corollary
S-rigid), and the seat rotation group is trivial because focal is fixed. The
only symmetry available seat-side at trick 1 is an exact 7:1 fold of the seven
pip declarations (Lemma S-fold). Hence

  COUNT 1 = C(28,7) = 1,184,040,

which is not a measurement but a theorem, and it misses the 10^5 bar by
29601/2500 ≈ 11.84×. COUNT 2's interface alphabet is likewise the raw trick-1
record space. This is the finding, and it is a proved negative in F7's sense —
not a defect of the construction and not a reason to re-cut the invariant list.

### The typing that governs this section

Three cardinalities the design's language risks fusing, all of them live here
(E-A8's lesson, stated in the seat's terms):

- **the raw seat space** — hands: C(28,7) per declaration;
- **the form count** — hands up to seat-side structural transport (Lemma S).
  This is what COUNT 1 measures;
- **any coarser censal class count** — e.g. an r3-style dynamics quotient of the
  seat's abstract problem, or a value partition. Nothing below computes one, and
  no number below may be presented as one.

At the last trick these three numbers were 55,036,800 / 32,532 / 64 (a1 file,
`endgame_floor_2026-08-11.txt`, E-A8). At the first play the first two coincide
and the third is not computed. A results file that lets a reader carry the
last-trick intuition ("a census produces a small alphabet") into the first-play
row is misreporting, and S-A19 fixes the sentence that prevents it.

### Lemma S (seat-side structural transport) — the seat analogue of Lemma E

*Setup.* Fix pip-trump declarations δ, δ′. A **first-play seat situation** is
x = (δ, H) with |H| = 7, focal = the declaring seat = the leader (P-A4), pool
U = 𝒟 ∖ H, and the capacity-cell system **C**_x = (U; P_s = U, k_s = 7) for the
three hidden offsets (§2.1). A **seat transport** φ : 𝒟 → 𝒟 from x = (δ,H) to
y = (δ′,H′) is a bijection with φ(H) = H′ carrying the δ-structure to the
δ′-structure, i.e. inducing a context bijection π (with π(7) = 7) such that φ
preserves: trump membership (φ(κ_δ) = κ_{δ′}); follow membership
(d ∈ σ̂_q^δ ⟺ φ(d) ∈ σ̂_{π(q)}^{δ′}); the led-context map on **every** live
tile (π(ℓ_δ(d)) = ℓ_{δ′}(φ(d))); the double flag; and the winner-determining
order in every context (S-A2). The seat rotation is the identity (focal↔focal
with focal fixed; rotations by 1,2,3 move focal, and reflection is forbidden by
§11.7 and F2 A4).

*Statement.* If a seat transport φ from x to y exists, then:

1. φ(U) = U′, and ω ↦ φ∘ω is a bijection Φ(**C**_x) → Φ(**C**_y) of the seats'
   fibers carrying the capacity-cell system to the capacity-cell system (§2.1)
   and the exact support normal form to the exact support normal form (§2.2);
2. for every ω ∈ Φ(**C**_x), the concrete situations x ⊕ ω and y ⊕ φ(ω) satisfy
   Lemma E's hypothesis; hence the remaining extensive games are isomorphic,
   carrying legal actions to legal actions, count-free increments to
   increments, focal to focal, and v0.4 §6.1 observation tokens to tokens
   componentwise (actor offset by identity, tile by φ, classification
   literally);
3. therefore every **count-free censal question** about the seat-facing
   situation has the same answer at x and at y: the number of legal leads; the
   set of realizable trick-1 play records and its size; the induced map records
   → landings and hence the set of realized landing forms with multiplicities;
   the void structure induced at every landing; and, for any
   isomorphism-invariant operator (E-A3), the per-world count-free values and
   any aggregate of them under a φ-transported weighting;
4. **not preserved, and never to be claimed:** any count-bearing quantity — φ
   does not preserve the count decoration c of §1.4, so **E-A2 applies verbatim
   to every seat-side form**; and any belief-relative quantity whose belief is
   not itself transported by φ (§2.4, §2.5: support ≠ belief). The uniform-legal
   field of F4 is transported; an arbitrary β is not.

*Proof.* (1) φ is a bijection of 𝒟 with φ(H) = H′, so φ(𝒟∖H) = 𝒟∖H′; at the
first play P_s = U and k_s = 7 at both, so φ carries **C**_x to **C**_y, and the
normal form is a function of the decoded fiber (§2.2), hence transports. (2)
Every datum in Lemma E's preserved list is in φ's preservation list: holders by
relative seat (identity rotation; focal↦focal; pool↦pool by (1) and the fiber
map), the trump/non-trump context split, follow membership, ℓ, the comparison
order, and the double flags; at a trick boundary there are no unresolved-trick
tiles, so F2 A1's extension is inert (P-Q1). Legality reads follow membership
and ℓ (§1.2, §1.5); the trick winner is the maximum trick key under the led
context (§1.3), which is always attained at tier ≥ 1 because the lead itself has
tier ≥ 1, so the winner-determining order suffices (S-A2); the next leader is
the winner; the count-free increment is e⋆ iff the winner sits in the focal
partnership, preserved by focal↦focal under the identity rotation; the hand ends
when live tiles are exhausted (§1.5). Lemma E then applies pointwise. (3)
Immediate from (2) and (1). (4) c is not among the preserved data; β is an
independent datum by §2.4. ∎

### Corollary S-rigid (the first-play transport group is trivial)

*Statement.* For every pip-trump δ, the group of seat transports from (δ,H) to
itself, and more generally the group of self-transports of the δ-structure on
the full 28-tile live set, is trivial. Consequently the seat-side hand form at
the first play **is the hand**, and the seat-side structural quotient at the
first play is the identity quotient.

*Proof.* Let φ be a self-transport with induced context bijection π; π(7) = 7
since φ(κ_δ) = κ_δ. (i) For non-trump d = q:r, ℓ_δ(d) = max(q,r), so
π(max(q,r)) = max(π(q),π(r)) for all q,r ∈ ℙ∖{δ}. A bijection of a finite chain
commuting with max is order-preserving (a ≤ b ⟹ π(b) = max(π(a),π(b)) ≥ π(a)),
and an order-preserving bijection of a finite chain is the identity; so
π|_{ℙ∖{δ}} = id. (ii) φ is then the identity on non-trumps: a non-trump mixed
tile q:r is the unique member of σ̂_q ∩ σ̂_r and a non-trump double q:q the
unique double in σ̂_q, and both memberships and the double flag are preserved,
so φ(q:r) = π(q):π(r) = q:r. (iii) φ permutes κ_δ preserving the order in
context 7, which on κ_δ is a strict total order (δ:δ top, then δ:r by r
descending, all distinct), so φ|_{κ_δ} = id. Hence φ = id. ∎

Note (i) and (iii) are independent routes: the led-context map alone pins the
non-trumps, and the trump ranking alone pins the trumps. Dropping either from
the invariant list does not rescue compression; it only makes the form unsound.

**Why the last trick compresses and the first play does not.** Structural
compression in this project has always been bought with *deadness*: at level 1,
twenty-four tiles are dead, most contexts are inert and erased (F2 A3), and the
surviving relations are so sparse that 55,036,800 situations carry only 32,532
forms and 64 classes. At the first play nothing is dead, no context is inert,
and the full structure is rigid. Corollary S-rigid is the precise statement of
that contrast, and it is what the design's premise did not anticipate.

### Lemma S-fold (the seven pip declarations fold exactly 7:1)

*Statement.* For pip trumps p, p′ let π_{p→p′} : ℙ → ℙ be π(p) = p′ together
with the unique order isomorphism ℙ∖{p} → ℙ∖{p′}, and let
φ_{p→p′}(a:b) = π(a):π(b). Then φ_{p→p′} is a seat transport from the δ=p
structure to the δ=p′ structure; it is the unique one; and
φ_{p′→p″} ∘ φ_{p→p′} = φ_{p→p″}. Hence the declaration orbits have exactly
seven members each and

  COUNT 1 = C(28,7) = 1,184,040 folded, = 8,288,280 / 7 unfolded.

*Proof.* Preservation: σ_p ↦ σ_{p′}; σ̂_q^p = {q:r : r ≠ p} ↦ {π(q):s : s ≠ p′}
= σ̂_{π(q)}^{p′}; ℓ is preserved because π|_{ℙ∖{p}} is order-preserving and
trumps lead context 7 at both; doubles ↦ doubles; and the winner-determining
order is preserved because within context 7 trumps are ordered by the other pip
with p:p top, within a natural context q the members are q:q (top) then q:r
ordered by r, tiers are functions of trump and follow membership, and the tier-0
bottom class maps to the tier-0 bottom class. Uniqueness follows from Corollary
S-rigid applied to φ_{p→p′}^{-1} ∘ ψ for any other transport ψ. Composition:
both sides are the unique order isomorphism ℙ∖{p} → ℙ∖{p″} extended by
p ↦ p″. ∎

*The fold depends on S-A2 and the dependence is exact.* Under the **literal**
reading of §1.3, in which tier-0 tiles are ordered among themselves by
r_δ = pip sum, the fold collapses to the single pair δ=0 ↔ δ=6 (there
π shifts every pip by one, so every non-trump pip sum shifts by exactly two and
the order survives) and nothing else: under δ=0 the tiles 4:1 and 3:2 have equal
trick keys in context 6, while their images 4:0 and 2:1 under φ_{0→3} have keys
4 and 3. Under the literal reading the declaration orbits are {0,6} and five
singletons, giving COUNT 1 = 6 × 1,184,040 = 7,104,240. **The bar's answer is
insensitive to this choice** (both 1,184,040 and 7,104,240 are far above 10^5),
and S-A2 requires the run to print which reading it froze and what the other
would have given.

*Adjudication-time verification.* Corollary S-rigid and Lemma S-fold were
checked exhaustively over all 5,040 pip permutations for all 49 ordered
declaration pairs under both readings; the counts came out 1 self-transport per
declaration under both readings, 49 cross-declaration transports (exactly one
per ordered pair) under the operative reading, and 9 under the literal reading.
That check is a check of the proofs, not their authority, and it is exploratory
tier; the proofs above stand on their own and S-A7 makes the builder reproduce
the group computation in-run.

### S-Q1 — the seat-side hand form. RULING: ACCEPT-WITH-AMENDMENT. The object and Lemma S stand; the proposed invariant list is INCOMPLETE and would produce a spuriously small COUNT 1; the compression premise is disproved by Corollary S-rigid.

The design's list — "trump membership, within-hand and hand-vs-pool beats
relations per context, the led-context map on my tiles, double flags" — has five
gaps, and the first two are unsound rather than merely incomplete:

- **(g1) pool-vs-pool comparisons are missing.** After the seat leads, the three
  hidden replies are pool tiles compared *against each other* to determine the
  winner (§1.3). Lemma E's list is over all live tiles, not over the seat's
  tiles. A form omitting pool-vs-pool merges hands facing non-isomorphic games,
  and Lemma S's conclusion is then simply false for the merged pairs.
- **(g2) the led-context map is restricted to "my tiles."** F2 A2 requires
  Θ^C(ℓ(d)) = ℓ(Θ^D(d)) for **every** live tile; a context bijection consistent
  with the recorded relations can otherwise swap a pool mixed tile's led context
  and the dynamics diverge.
- **(g3) follow membership for pool tiles is unstated** (it governs their
  legality and their tier, §1.2–1.3).
- **(g4) the capacity-cell system is unstated.** Inert at the first play
  (P_s = U, k_s = 7) but mandatory at the landing.
- **(g5) the comparison reading is unstated** — and per Lemma S-fold it decides
  the fold factor.

These gaps have teeth for the headline: a run implementing the list as written
would report a COUNT 1 far below 1,184,040 and could appear to clear the 10^5
bar. It would be clearing it by merging hands that face different games.

- **S-A1 (the form, exactly).** The seat-side hand form of (δ, H) is the
  canonical form of the **whole δ-relational structure on the live tile set with
  H distinguished**, under the holder sort {focal, pool}: trump membership; the
  effective context family and follow membership for **every** live tile; the
  led-context map ℓ on **every** live tile; the double flag; the comparison
  order of S-A2 in every live context; and the capacity-cell system of S-A3.
  Equivalently and preferably as the implementation's specification: the form is
  the orbit of H under the group of seat transports of Lemma S. No relation in
  Lemma E's amended F2 list may be dropped on the ground that the seat cannot
  observe it — the seat's *structural situation* includes relations among tiles
  it does not hold.
- **S-A2 (the comparison invariant, declared).** The form records the
  **winner-determining order**: per live context q, the strict order on
  tier-2 ∪ tier-1 tiles, with all tier-0 tiles collapsed into a single bottom
  class. This is sound for Lemma S because the maximum trick key is always
  attained at tier ≥ 1 (the lead has tier ≥ 1), so no rule ever reads the mutual
  order of tier-0 tiles. It is a DECLARED CHOICE (freeze 18), it is what
  licenses Lemma S-fold, and the results file prints both it and the literal
  reading's COUNT 1 (7,104,240) beside the frozen one.
- **S-A3 (the support side of the form).** The form carries the capacity-cell
  system **C** and, where it is nontrivial, the exact support normal form
  N = 𝒩(**C**) (§2.1–2.2) — not merely the pool as a set. At the first play this
  is (P_s = U, k_s = 7) for all three hidden offsets and carries no information;
  at the landing it is the whole observation content (S-A12) and is mandatory.
- **S-A4 (the first play really is observation-free, and that is a statement
  needing a sentence, not silence).** At the first play Φ(**C**) = Φ(**C**₀)
  exactly — there are no plays, hence no rule-derived exclusions, so P-A1's
  void-free scope restriction is **inert here by computation, not waived**, and
  the fiber is 21!/(7!)³ = 399,072,960. The results file states: "The bidding
  prefix lies outside the modelled finite-hand game (§1.5 BOUNDARY) and is
  treated as support-inert: a pass is legal for every hand, so no hand is
  excluded from any hidden seat. Any bid-derived restriction is belief (§2.4),
  never support, and none is used here." Without this sentence "nothing has been
  observed" is an unstated assumption.

**S-Q1b (folding the seven pip declarations): ACCEPT, as Lemma S-fold, exactly
7:1 and conditional on S-A2.** This is the cross-pip pooling F1 already declared
lawful, now with its transport exhibited and proved unique. Doubles-trump and
no-trump remain OUT OF DECLARED SCOPE (F1; v0.4 §14.7, §17.5) and the fold says
nothing about them — π_{p→p′} is defined only between pip declarations.

- **S-A5 (print both).** The results file prints the unfolded per-declaration
  count, the folded count, and the fold factor, and asserts in-run that
  φ_{p→p′} carries the δ=p form set bijectively onto the δ=p′ form set for all
  49 ordered pairs. Reporting only the folded number hides which symmetry was
  used.

**S-Q1c (machinery reuse): ACCEPT reuse, under one code path, with the holder
sort made a parameter — not a new implementation and not a second normal form.**
A seat situation is *not* a degenerate world: its holder function is two-valued
({focal, pool}) where the world's is four-valued, so the seat form is the
world form's **holder-coarsening**, obtained by merging the three hidden holders
before canonicalisation, never by post-processing a world form.

- **S-A6 (one code path, seat side).** E-A5 restated: the world canonicaliser
  and the seat canonicaliser are ONE function parameterised by a declared holder
  sort. Two implementations of a normal form is the classic way to make a store
  silently wrong and no receipt short of full re-expansion catches it.
- **S-A7 (the receipts that replace the absent measurement).** Because COUNT 1
  is a theorem, its enumeration is a **receipt**, and the run asserts: (i) the
  seat transport group at the first play is trivial, recomputed in-run by the
  same exhaustive check described above; (ii) distinct forms per declaration =
  1,184,040 = C(28,7); (iii) folded forms = 1,184,040 with all 49 pairwise fold
  bijections verified; (iv) world-form equality implies seat-form equality on a
  declared deterministic sample (merging holders is a coarsening, so this is a
  theorem and a genuine cross-check between the two instantiations of S-A6).
  Any failure is stop-and-report per NO-RESCUE — it is an implementation defect
  or an error in Corollary S-rigid, never a new class count.
- **S-A8 (the r1 reading diagnostic, cheap and necessary-not-sufficient).**
  Re-run the E-A7 floor build split by declaration and print the seven
  per-declaration distinct-r1-form counts. Under S-A2's reading Lemma S-fold
  forces all seven form sets to be equal, hence each equal to the union, 32,532.
  If they differ, r1's implemented comparison is declaration-rigid and the seat
  census must not inherit it implicitly. Level 1 erases most relations, so
  agreement is evidence and not proof; the seat census declares its own reading
  in freeze 18 either way.

### S-Q2 — the first-trick interface alphabet. RULING: ACCEPT-WITH-AMENDMENT. Determination HOLDS and is a theorem; the bounded alphabet it requires is the raw trick-1 record space. That is the finding.

**Lemma S-det.** The landing state is a function of (δ, H, r) where
r = (d₁,d₂,d₃,d₄) is the ordered trick-1 play record: the landing hand is
H ∖ {d₁}; the live pool is U ∖ {d₂,d₃,d₄}; for each hidden offset i the observed
void is "void in ℓ_δ(d₁)" exactly when d_{i+1} ∉ σ̂^δ_{ℓ_δ(d₁)} (a slough proves
a void in the led context and proves nothing else, §2.1); and the next leader is
the offset attaining the maximum trick key (§1.3). By Lemma S the assignment is
equivariant: φ(landing(δ,H,r)) = landing(δ′, φ(H), φ(r)). Hence (hand form,
interface element) determines the landing form. ∎

The interface element must therefore carry the four played tiles named up to the
stabiliser of the hand form — and by Corollary S-rigid that stabiliser is
trivial, so "named by role in the form" and "named by tile" are the same thing.
The design's proposed content (lead's role, replies' classifications and mutual
relations, outcome, dead tiles as roles) is exactly a re-encoding of the record;
it is lawful, and it is not a quotient of anything.

- **S-A9 (state the finding, do not bury it).** The results file states:
  "Determination holds with a bounded alphabet, and the bound is the raw space.
  The first-trick interface alphabet is the set of ordered 4-tuples of distinct
  tiles, at most 28·27·26·25 = 491,400 overall and at most 7·21·20·19 = 55,860
  above a fixed hand form; because the hand form's stabiliser is trivial
  (Corollary S-rigid), no coarser interface element determines the landing. The
  interface buys no compression at the top of the game. Both outcomes are
  results (F7); this is the negative one, and it is proved rather than
  measured."
- **S-A10 (the one honest measurement in COUNT 2 — realizability, typed as
  legality not compression).** Not every 4-tuple is realizable: a reply must be
  legal for **some** ω in the fiber. The realized count is a genuine number and
  the run may report it, labelled "a legality census of the record space, not a
  compression measurement." It is not a class count and may never be paired with
  the bar.
- **S-A11 (two cardinalities at the interface, never on one line).** Report
  |realized records| and |distinct landing forms they induce| as separate rows
  with the collapse ratio printed as an exact rational beside its numerator and
  denominator (P-A11, X-A10). These answer different questions. Prediction on
  the record: with only four tiles dead, residual self-transports of the live
  structure are expected to be essentially absent and the ratio close to 1; a
  ratio far from 1 is a bug or a defect in Corollary S-rigid before it is a
  finding, and must be investigated as such (NO-RESCUE, never patched).
- **S-A12 (the interface element is not a macro step).** Y1's correction stands:
  the four-tile record is a *counting* coordinate for a declared output contract,
  never a trick-level macro kernel. No value, no increment composition, and no
  kernel may be read off the interface alphabet (F5, Y1).

### S-Q3 — the landing alphabet. RULING: ACCEPT-WITH-AMENDMENT. Mandatory observation content fixed below; both domains may be counted; the REACHABLE one is the claim and the closed-form superset is a declared cost domain.

- **S-A13 (mandatory content, and one forbidden field).** The landing form
  carries: the live structure over the 24 live tiles with the S-A1 invariants,
  under the holder sort {focal, pool}; the exact support normal form N of the
  landing capacity-cell system, which is precisely the void content — per hidden
  offset, exclusion of the trick-1 led context's live members when that offset
  sloughed (§2.1–2.2), capacities (6,6,6); and the next leader as an **offset
  from focal ∈ {0,1,2,3}** (r3 Q3; never an orientation-flavoured word, §11.7).
  **FORBIDDEN: the trick-1 count-free increment.** F5's amendment is in force —
  the bank is emission, not state; storing it duplicates a derived quantity and
  splits landing forms by trick-1 outcome, destroying real merges. The leader
  offset already carries what the dynamics need. Dead tiles are **erased**, not
  recorded: they enter only as absences from the live structure and through the
  voids they induced (F2 A1, Lemma E).
- **S-A14 (the landing form is a kernel, not an information state — mandatory
  sentence).** "COUNT 3 counts support kernels K = (δ, H_m, N, τ) in the sense of
  §2.3, with retained evidence declared empty (e = ∅). That declaration is a
  scope restriction, not a theorem: a mechanical kernel can be an exact
  sufficient state for objective play while failing to be the perfect-recall
  information state (§2.6). Two histories with the same landing form differ in
  evidence and therefore in belief (§2.4, §2.5). No information-state count, and
  no claim about the seat's knowledge, may be read from COUNT 3."
- **S-A15 (the two domains, typed).** §2.2's BOUNDARY is decisive: feasibility
  and exact normal-form decoding do not imply legal Straight reachability, and a
  support object used as a current game state must be inherited from legal
  construction or carry an accepted reachability witness. Therefore: the
  **reachable landing alphabet** (landings enumerated through COUNT 2 from legal
  trick-1 play) is inherited from legal construction and is THE CLAIM; the
  **closed-form landing domain** (all (6|18) live structures with declared
  observation structure and leader offset) is a DECLARED SUPERSET and a cost
  domain of exactly the kind P-A1 types, whose members are FEASIBLE and never
  "reachable." Both may be counted, both must be printed with their types, the
  gap must be printed, and the superset count is an upper bound and nothing
  else.
- **S-A16 (E-A2 re-stated where it now bites hardest).** The landing form erases
  dead tiles, so it erases which count-bearing tiles fell in trick 1. Every
  landing-form-keyed number is therefore sound **only** under the count-free
  contract (q_trick; P-A6). If count re-enters (v0.5 role re-entry), every such
  number and every form-keyed record is void wholesale, never extended. Print
  this in the results file; it is the one failure mode that would silently
  produce wrong numbers rather than a crash.

### S-Q4 — the scope fence. RULING: ACCEPT-WITH-AMENDMENT; the fence as posed is right and three clauses are missing.

- **S-A17 (the fence sentence, verbatim).** "This census produces counts. It
  makes no value claim, no policy claim, no belief claim, and no support claim
  about any seat beyond the declared cell systems above. The flat stack is a
  presentation for COUNTING; it composes no interfaces into evaluations, and it
  never replaces the nested object for values — the contradiction is localised
  there and is deliberately out of scope. Three further fences: (i) COUNT 3
  counts kernels, not information states (S-A14); (ii) the closed-form landing
  domain is a declared superset whose members are feasible, never reachable
  (S-A15, §2.2 BOUNDARY); (iii) a count of situations is not a measure of
  difficulty or of information value (§10.9, §17.5) — the bar is a question about
  inventory size and is answered only as such."

### S-Q5 — enumeration and freezes. RULING: ACCEPT-WITH-AMENDMENT. COUNT 1 and COUNT 2 become receipts; COUNT 3 is the only genuine measurement and needs a declared stop.

- **S-A18 (what the build actually is now).** COUNT 1: a receipt run over
  C(28,7) hands per declaration with the S-A7 assertions; expected output known
  in advance; a discrepancy is stop-and-report. COUNT 2: a receipt plus the
  S-A10 legality census. COUNT 3: the only measurement, and it is not
  exhaustively enumerable — the reachable landing space above all hand forms is
  of order 1,184,040 × 55,860 ≈ 6.6 × 10^10. It is therefore measured on a
  **declared multiplicative decimation** in P-A15's exact form (index set
  {i·g mod N}, gcd(g,N) = 1, g and W frozen; never a prefix, no RNG, no Date),
  reported with exact-rational mean plus min, max and quartile boundaries, and
  every stop printed (P-A16, E-A16: a wall is a stop, never a finding). The
  closed-form superset count of S-A15 is computed in closed form and asserted
  against its formula in the E-A7 style.
- **S-A19 (new freezes, continuing at 18).** (18) the seat-side form: the holder
  sort {focal, pool}, the S-A1 invariant list, the S-A2 comparison reading, and
  the byte encoding; (19) the declaration-fold maps φ_{p→p′} and whether the run
  reports folded, unfolded, or both (both are mandatory); (20) the
  interface-element encoding — the ordered play record — and the record and hand
  enumeration orders; (21) the landing form's observation content: the
  support-normal-form void encoding, the leader-offset encoding, and the
  declaration e = ∅. Freezes 1–17 are in force and restated unchanged.

### S-Q6 — results discipline. RULING: ACCEPT-WITH-AMENDMENT; two mandatory paragraphs decide whether this file is read correctly.

P-A20's inherited boilerplate lineage, integers first, per-declaration splits,
a1-style anatomy sections, one file `results/seat_census_2026-08-11.txt`, and
the F7 both-outcomes framing are confirmed. Every count is paired with its type
(raw / form / class) and, where it is a theorem rather than a measurement,
labelled THEOREM (RECEIPT RUN).

- **S-A20 (the bar, typed — verbatim).** "THE BAR, TYPED. 'Is the number of
  situations facing the trick-1 leader of order 10^5' is a question about a
  censal equivalence, and the answer depends on which one. COUNT 1 answers it
  for the FINEST seat-side equivalence — structural form, i.e. relabelling by
  seat transports (Lemma S) — and the answer is NO. At the first play the
  pip-trump structure on all 28 live tiles admits no nontrivial self-transport
  (Corollary S-rigid) and the seat rotation group is trivial because focal is
  fixed, so the only symmetry available is the exact 7:1 declaration fold (Lemma
  S-fold). The seat-side structural quotient at the first play is therefore the
  identity quotient and COUNT 1 = C(28,7) = 1,184,040, exceeding 10^5 by
  29601/2500 ≈ 11.84×. This is a FORM count. It is not a count of any coarser
  censal equivalence — in particular not of an r3-style dynamics quotient of the
  seat's abstract problem, which is not computable at the first play without
  descent (S5h), and not of any value partition, which is out of scope. Whether
  some coarser lawful equivalence reaches 10^5 is OPEN and this census does not
  address it."
- **S-A21 (why the top does not compress — verbatim).** "The last-trick census
  quotients 55,036,800 situations onto 32,532 forms and 64 classes; the
  first-play census quotients 1,184,040 hands onto 1,184,040 forms. Same
  machinery, same structure; the entire difference is deadness. At level 1
  twenty-four tiles are dead, most contexts are inert and erased, and the
  surviving relations admit enormous symmetry. At the first play nothing is dead
  and nothing is inert. Structural compression in this project has always been
  bought with dead tiles and dead contexts, and the seat has neither at the
  first play."

**Both outcomes remain results** (F7, NO-RESCUE). This section's outcome is the
negative one and it is a proved negative, not a measured disappointment: it
changes nothing about Lemmas E, V or X, nothing about the r3 classes or their
ECL receipts, and nothing about the a1 alphabet. What it does change is where a
seat-side quotient could possibly come from — not from relabelling symmetry at
the top of the game, because there is none.

## Predictive-rank probe rulings (2026-08-12)

**Adjudicator:** walt-math. **Tier:** exploratory throughout. v0.6
(`walt/math/predictive_algebra_v0.6.md`) is consumed as design guidance only;
nothing below promotes any of its theorems, and no v0.6 statement is cited above
exploratory tier anywhere in the build. **Basis:** v0.6 §§3, 5–10, 12, 15–18;
v0.5 §12.6A and its BOUNDARY; v0.4 §1.2–1.5, §2.1–2.6, §5.5, §6.1, §6.8,
§7.4–7.7, §9.9, §10.3, §10.9, §11.7, §12.4, §14.7, §17.5. F1–F7, r3 Q1–Q5,
Y1–Y3, shape v2, P-A1..P-A21, X-A1..X-A19, E-A1..E-A21, S-A1..S-A21 and Lemmas
V, X, E, S, S-fold, S-det, Corollary S-rigid are inherited unchanged. Amendments
are numbered R-A1.. and are builder obligations; a run that omits one is not the
adjudicated probe.

**Headline of this section, stated first because it decides what part one may
measure.** The proposed Experiment-1 measurement is degenerate, and provably so.
In Straight 42 every tile is eventually played and every play is publicly
attributed, so a complete continuation record determines the latent world.
Consequently **any continuation closure whose terminal seed contains a nonzero
constant function has predictive dimension exactly |X_i| at every interface**
(Lemma R(c)). That covers v0.6 §6.2's closure verbatim, and it covers the
design's contracts (ii) (the focal trick-count *distribution*) and (iii)
((ii) plus a predicate): those rows are theorems reading r = |X|, not
measurements, and a run reporting them as Gate-B evidence would be reporting an
artefact of the observation structure as a fact about the game. Exactly one
object in the design's ladder escapes, and it escapes for a precise reason: the
count-free **expected** focal-trick contract has terminal readout 0, so its
value closure V^val is seeded by the zero space and never admits residuals of the
constant. Part one measures dim V^val or it measures nothing. Both outcomes
remain results (F7): a large dim V^val is a proved negative about linear
predictive compression on this route; the degenerate rows are proved negatives
already, before any code runs.

### The v0.6 proof audit (checked at adjudication time)

Verdicts on the theorems the design proposes to consume. "SOUND" means the proof
as written establishes the statement under the hypotheses named; hypotheses that
v0.6 leaves implicit are listed because they become builder obligations.

- **§5.3 finite predictive-rank minimality — SOUND.** H = FC forces rank H ≤ r,
  and any rank factorization realizes rank H. Note what it does *not* give: the
  F obtained from an arbitrary rank factorization need not be closed under
  residuals, hence need not support filtering or (POL). §6 supplies the closed
  object, and §6.4's identification r = dim V is correct because rank H is the
  dimension of the column space, i.e. of the span of the test functions.
- **§6.3 residual span = continuation-test span — SOUND under two hypotheses
  that must be declared.** (H1) the successor interface j is a function of
  (i, u, o) alone; if the successor typing depended on hidden data the residuals
  would not be well typed. (H2) tests may condition on analyst-visible event
  labels (§5.1), which is what makes single-event residuals test functions. (H2)
  is what makes the §6.2 span the *analyst-refined* one — larger, in general,
  than anything a policy can use (§8.2, §18.6). The proof itself is correct: the
  forward direction must sum over branches o with a per-o successor test, which
  the proof's "finite sum of residuals" covers.
- **§6.4 r = dim V — SOUND given §6.3.**
- **§7.4 exact predictive filtering — SOUND**, requiring positive-probability
  histories and the constant 1 in every basis. That last requirement is not
  free: by Lemma R(c) it is precisely what forces dimension |X| here, so exact
  *normalised* predictive filtering and predictive compression are incompatible
  under this game's observation structure. The unnormalised value recursion does
  not need it.
- **§8.4 exact optimality without strategy fusion — SOUND under (H3).** (H3) the
  coefficient recursion is indexed by observation history; memoising one c per
  interface silently forces histories that share an interface to share a
  continuation, which restricts the policy class and returns a restricted-class
  value under v0.5's BOUNDARY rather than the perfect-recall value. v0.6 does not
  state (H3); the proof needs it, because "one policy per observable successor
  information state" is a statement about information states, not interfaces.
- **§9.2 outcome-algebra lift — SOUND**, given that the increment is a function
  of the event (count-free r ∈ {0, e⋆} satisfies this). One corollary that is
  *not* established and must not be assumed: that a richer outcome law costs no
  extra dimension. Decoration is free; the cost lands in the terminal seed, and
  for the trick-count distribution that seed is the constant 1 (Lemma R(c)).
- **§10.6 finite monitor preservation — SOUND** (deterministic monitor product);
  out of part-one scope.
- **§12.3 equivariant representative independence — SOUND** given its coherence
  hypotheses, which are F3's conditions in linear dress; out of part-one scope
  except through Corollary R-fold below.
- **GAP G1 (contract-insensitivity).** §6.2's generator set depends on the
  outcome contract only through terminal and immediate readouts, while §15
  Experiment 1 asks for a rank "for increasingly rich contracts". Contracts that
  add no terminal readout cannot move the number. This is a genuine gap between
  §6.2 and §15, and it is a finding, not a crisis: Lemma R supplies the
  contract-generated closures that make a ladder meaningful.
- **GAP G2 (implicit hypotheses).** (H1), (H2) and (H3) above are load-bearing
  and unstated. Each becomes a builder obligation below.
- **GAP G3 (the one with teeth).** §5.4's hierarchy is honest — v0.6 explicitly
  declines to claim the rank is small — but the design reads "is the rank small?"
  as an open measurable question for every contract. For every contract whose
  terminal readout family contains a nonzero constant it is not open and not
  measurable: the answer is |X|, by Lemma R(c).

### Lemma R (three continuation closures, and the separating-observation degeneracy)

*Setup.* Fix an information interface i of the void-free rung at grade n, the F4
uniform-legal field at the three hidden offsets, the count-free valuation
(q_trick, P-A6), and an observation contract obs(·). Write Pre_{i,u,e} for v0.6
§6.1's unnormalised preexpectation, Pre_{i,u,o} = Σ_{e: obs(e)=o} Pre_{i,u,e},
and g_{i,u}(ξ) = Σ_e g(e)·K_{i,u}(ξ; e) for the expected immediate count-free
increment. Define three graded spaces of rational functions on X_i:

1. **event closure** V^ev (v0.6 §6.2 verbatim): terminal = span{1, terminal
   readouts}; nonterminal = span{1} + span{immediate readouts} +
   Σ_{u,e} Pre_{i,u,e}(V^ev_{j(o)});
2. **observation closure** V^obs: as (1) with Pre_{i,u,o} in place of
   Pre_{i,u,e};
3. **value closure** V^val: terminal = span{terminal readouts} — for the
   count-free expected-trick contract this is the zero space; nonterminal =
   span{g_{i,u} : u} + Σ_{u,o} Pre_{i,u,o}(V^val_{j(o)}). **The constant 1 is not
   a generator.**

*Claims.*

(a) V^val ⊆ V^obs ⊆ V^ev. Each is closed under the residuals used to generate
it, so each admits closure matrices satisfying (PCM) at its own refinement:
per-event M_{i,u,e} for V^ev, observation-aggregated M_{i,u,o} for V^obs and
V^val. (POL) and the root pairing use only M_{i,u,o}.

(b) Every lawful continuation policy — one local controller per interface, one
continuation per player observation (v0.6 §8.2; §6.8's no-revelation rule) — has
its value function in V^val, and (POL) runs over a basis of V^val **with no
normaliser**: J_ρ(β) = ψ(β)c_ρ, Q_B(a) = max over the action's coefficient set,
V_B = max_a Q_B(a), all in dim V^val coordinates.

(c) **(degeneracy)** Suppose the observation contract is such that a complete
continuation record determines the latent point — as it does here, for every
contract at least as fine as "the four played tiles with their seats," since
every tile is played before the hand ends and every play is publicly attributed.
Then for every interface i, V^ev_i = V^obs_i = Q^{X_i}, i.e. r^ev_i = r^obs_i =
|X_i| exactly. More generally this holds for any closure whose terminal seed
contains a nonzero constant function.

(d) **(the ladder)** Under (c), the design's contracts (ii) (full focal
trick-count distribution) and (iii) ((ii) plus the next-leader-offset predicate)
have predictive dimension exactly |X_i|, because the point-mass terminal readout
of a distribution contract *is* the constant function 1. The expected-trick
contract (i) is not covered by (c): its terminal seed is 0.

*Proofs.*

(a) Containment is immediate from Pre_{i,u,o} = Σ_{e: obs(e)=o} Pre_{i,u,e} and
g_{i,u} = Σ_e g(e)Pre_{i,u,e}(1) once 1 is present; for V^val ⊆ V^obs use
induction on grade with the same identities. Closure is by construction: each
space contains the residuals of its successor's basis, so each Pre of a
successor basis vector has an expansion in the current basis, which is exactly
(PCM) at that refinement. ∎

(b) Induction on grade. At a terminal, V_ρ is a terminal readout. At grade
g > 0, ρ picks u and, for each player observation o, one continuation ρ_o common
to every hidden event with obs(e) = o. Then

  V_ρ(ξ) = Σ_o Σ_{e: obs(e)=o} [ g(e)·K_{i,u}(ξ; e) + Pre_{i,u,e}(V_{ρ_o})(ξ) ]
         = g_{i,u}(ξ) + Σ_o Pre_{i,u,o}(V_{ρ_o})(ξ),

where the second equality uses precisely that ρ_o does not depend on e. By the
induction hypothesis V_{ρ_o} ∈ V^val_{j(o)}, so both terms are generators of
V^val_i. Uniqueness of c_ρ is basis independence. No normaliser appears because
the recursion is unnormalised throughout; a normaliser is needed only to convert
ψ̃ into a conditional moment (§7.3), which the pairing does not require. ∎

(c) By downward induction the closure contains every composition
Pre_{u_1,o_1} ∘ … ∘ Pre_{u_k,o_k}(1) along a single observation path with any
legal controllers, i.e. the function ξ ↦ Pr_ξ(o_1 … o_k | controllers). Take
k = n, a complete record p. Given ξ, the field's masses are products of unit
fractions, so Pr_ξ(p) = w_p·1[ξ consistent with p] with w_p > 0; and a complete
record names every hidden tile together with the seat that played it, so at most
one ξ ∈ X_i is consistent with p. Hence Pr_·(p) is a nonzero multiple of the
indicator of a single world, and every ξ ∈ X_i is consistent with some complete
record (every world can be played out under the uniform-legal field). So V
contains every singleton indicator and equals Q^{X_i}. The same argument runs
from any nonzero constant in the terminal seed. ∎

(d) A distribution contract's terminal readout is the point mass at the empty
continuation, whose coefficient function is the constant 1; apply (c). For (iii),
the predicate is added on top of (ii). For (i), the terminal readout is 0 and no
constant is ever seeded, so (c) does not apply — and nothing here proves
dim V^val is small. That is the measurement. ∎

*Two consequences the builder must respect.* First, the repair is on the
test-family side, never on the observation side: coarsening o to escape (c) would
change the information model and therefore the operator (v0.4 §7.7 — theorems for
one operator do not transfer to another) and would change which policies are
lawful. Second, (c) is not a defect of v0.6; §5.4 declines to claim small rank.
It is a fact about this game's observation structure, and it is the predictive
analogue of Corollary S-rigid: the compression the design hoped to buy is absent
for a structural reason, and the reason is exactly identifiable.

### Corollary R-fold (the predictive dimension is declaration-fold invariant)

*Statement.* Let φ = φ_{p→p′} be the declaration transport of Lemma S-fold,
restricted to the live tile set L of a rung coordinate x = (δ = p, L, H). Then
x′ = (δ = p′, φL, φH) is a coordinate, φ induces a bijection X_i → X_{i′}
carrying legal sets to legal sets, uniform field masses to uniform field masses,
observations to observations, count-free increments to increments and focal to
focal; hence H_{i′} is H_i with rows and columns permuted and

  dim V_i = dim V_{i′} for each of the three closures, with corresponding
  behavioural-row counts and corresponding policy values.

*Proof.* φ is a bijection of 𝒟 preserving trump membership, follow membership,
the led-context map, double flags and the winner-determining order (Lemma
S-fold), so its restriction to L preserves every datum in Lemma E's list on the
live set; F2 A1's unresolved-trick extension is inert at a boundary. The seat
rotation is the identity and focal ↦ focal, so increments correspond. Legality
and |L_actor| correspond, so the uniform-legal field's masses correspond
(F4). Controllers, observations and tests therefore correspond bijectively, and
h_{t}(ξ) = h_{φt}(φξ) for every test. Row and column permutation preserves rank.
∎

*What this licenses and what it does not.* The dimension, the behavioural-row
count and every policy value correspond. The **basis and the closure matrices do
not**: they are chosen by a deterministic pivot rule over a declared enumeration
order, which is not φ-equivariant. So the in-run assertion is equality of
dimensions and of values, never byte-equality of matrices, and every
sparsity figure is freeze-relative (R-A21).

### R-Q1 — filing, tier, vocabulary. RULING: ACCEPT (a); ACCEPT (b) as proposed; AMEND (c).

- **R-A1 (filing).** ACCEPT. v0.6 sits alongside v0.4 (frozen basis) and v0.5
  (§12.6A track) as a third track at exploratory tier, consumed as design
  guidance. The audit above is the extent of its adjudication: its theorems are
  usable as *definitions and design guidance* with the hypotheses (H1)–(H3)
  discharged by the builder, and G1–G3 recorded. No v0.6 theorem may be cited in
  support of any claim at any tier, and "by construction" remains not a receipt
  (r3 Q5.2): every (PCM) identity quoted is recomputed independently over the
  finite domain, both sides, in exact arithmetic (v0.6 §16.3's checker shape).
- **R-A2 ("certificate").** ACCEPT the proposal. walt files say **receipt**
  wherever v0.6 says "certificate"; the word "certificate" appears only inside an
  explicit verbatim quotation of v0.6, bracketed as such. The fence, restated so
  the two senses can never blur and mandatory in the results header: "No object
  produced by this probe is an identity-bearing witness of anything.
  Reachability is a proof-irrelevant proposition (project rule; D3's 'necessary
  outer profile'); a (PCM) receipt asserts a linear identity over a declared
  finite domain and asserts nothing about whether any state of that domain
  arises in play — the domain is the void-free capacity fiber, whose members are
  FEASIBLE and never reachable (P-A1)." Note also that a walt probe receipt is
  exploratory tier and is **not** a rob CI receipt; it never promotes anything
  (TRUST-01).
- **R-A3 (the rest of the vocabulary).** AMEND. "Continuation test", "local
  controller" and "closure matrix" enter as-is; note for the record that a local
  controller is a policy fragment and not a "control" in v0.4 §12.5's skeleton
  sense. Two renames are mandatory. (i) **"interface" is always written
  "information interface"** — the bare word already denotes the declared
  role/output interface in this project, and O_Σ = ∅ is in force here (the Extra
  item ruling); a file that says "interface" unqualified is ambiguous about which
  object carries the empty declaration. (ii) **the bare word "rank" is
  forbidden.** It collides with the settled standings vocabulary (constellations,
  never "rank") and with r_δ, the trick-key order of v0.4 §1.3. Write
  **"predictive dimension"**, or the fully qualified "predictive rank (rank_Q of
  the continuation matrix)" on first use in a file and "predictive dimension"
  thereafter. (iii) ψ carries a type line wherever it appears: "ψ is a declared
  aggregation statistic of a declared belief (v0.4 §5.5); it is not support, not
  a belief, and its coordinates are not probabilities of anything — a
  minimum-dimension basis may be signed (v0.6 §5.4, §18.5). P-A12's typing
  governs it."

### R-Q2 — information interfaces and the measured domain. RULING: ACCEPT-WITH-AMENDMENT (a); AMEND (b); AMEND (c) — the fence as posed is not closed under the recursion.

**(a) The proposed tuple is not a lawful instance of §3.1: it drops the data that
determines the fiber.** §3.1 requires the "exact support/evidence interface
needed to reconstruct the latent fiber," and (declaration; focal; focal hand;
leader; voids = ∅; grade) does not: without the live pool the fiber is not
determined, and S-A3 already fixed the seat-side form's support side as the
capacity-cell system, not the pool-as-a-set.

- **R-A4 (the information interface, exactly).** The interface is
  (δ; focal = the declaring seat, P-A4; the live tile set L; the focal hand
  H ⊆ L; the capacity-cell system **C** = (P_s, k_s) for each hidden offset and,
  where nontrivial, the exact support normal form N = 𝒩(**C**) (v0.4 §2.1–2.2,
  S-A3); the leader as an **offset from focal ∈ {0,1,2,3}** (r3 Q3; never an
  orientation-flavoured word, §11.7, F2 A4); grade = tricks remaining, in Y1's
  indexing). Declared empty and stated as declared: the output-role interface
  (O_Σ = ∅) and the outcome-monitor state (none in part one). **FORBIDDEN in the
  interface: any accumulated outcome.** F5's amendment is in force — the bank is
  emission, not state; carrying it duplicates a derived quantity and splits
  interfaces by past outcome, destroying real merges (S-A13). §3.1's
  "selected outcome-monitor state" is the door through which it would enter, and
  that door is declared shut for part one.
- **R-A5 (the void-free typing, and the successor typing).** The fiber over an
  interface is Φ(**C**₀), the void-free capacity fiber, and P-A1's paragraph is
  reproduced verbatim in the results file. Two additions. (i) At a rung root the
  void-free typing is a **declared restriction, not an inert one**: unlike the
  first play (S-A4, where Φ(**C**) = Φ(**C**₀) by computation) a grade-3 boundary
  in any real hand follows four tricks of play and generally carries voids. The
  results file states: "These are fabricated void-free boundaries. Their members
  are feasible; no boundary measured here is asserted to arise in play (v0.4 §2.2
  BOUNDARY, S-A15)." (ii) Successor interfaces **carry the induced voids**
  (Lemma S-det fixes them exactly: a slough proves a void in the led context and
  proves nothing else). Typing successors void-free is also lawful as a declared
  cost domain, but then |X_j| is inflated and every dimension measured at a
  successor is an upper bound; whichever is chosen is declared per interface and
  labelled, and the two are never mixed on one row (S-A15's discipline).
- **R-A6 (coordinates, decimation, and what per-coordinate variation
  licenses).** The design cites P-A9 for decimation; the governing amendment is
  **P-A15** (P-A9 is the same-object receipt). Requirements: the coordinate
  population size N per grade is printed whether or not it is enumerated;
  enumeration is exhaustive where the population fits the declared budget and
  otherwise is the P-A15 multiplicative decimation ({i·g mod N}, gcd(g,N) = 1,
  g and W frozen — never a prefix, no RNG, no Date); where S5g/S5h coordinates
  exist at a rung they are reused with the same (g, W) so the dimension rows can
  sit beside the existing cost rows (X-A12). **A predictive dimension is not an
  averageable statistic:** the run prints the full integer multiset (or its
  histogram) of per-coordinate dimensions with min, median and max, and **no mean
  dimension may be printed or quoted**. What per-coordinate variation licenses:
  nothing about the game and nothing about unsampled coordinates. The census
  claim is "over the W declared coordinates at grade n, dim V^val ranged
  [min, max]", never "the dimension at grade n". Counts and dimensions are
  coordinate-relative exactly as class counts are carrier-relative (r3 Q4,
  X-A19).
- **R-A7 (the fold receipt).** The coordinate sample is closed under the
  declaration fold, or the run explicitly constructs and evaluates the images
  φ_{p→p′}(x) even when they fall outside the sample. It then asserts Corollary
  R-fold in-run: equal dimensions, equal behavioural-row counts and equal policy
  values along all 49 ordered declaration pairs. Basis vectors and closure
  matrices are **not** compared (they are freeze-relative). A failure is
  stop-and-report per NO-RESCUE: it is an implementation defect or an error in
  Corollary R-fold, never a finding about the game. This is the cheapest strong
  receipt available here and it replaces, at the coordinate level, what S-A7 did
  for the seat census.

**(c) The focal-lead fence is not closed under the recursion, and as posed it
would silently truncate the closure.** The next leader is the trick winner, so
most successors of a focal-lead boundary are nonfocal-lead. A run that visits
only focal-lead interfaces does not compute V — it computes a proper subspace and
under-reports the dimension. Separately, the controller alphabet U(i) at an
interior nonfocal-lead interface is a map from observed prefixes to plays and is
doubly exponential (at grade 3, hundreds of distinct prefixes over a nine-tile
pool); enumerating it is infeasible and unnecessary.

- **R-A8 (root-only fence; primitive-step closure).** The fence is restated:
  "**Only the root information interface of each coordinate is focal-lead.**
  Interior interfaces are whatever the dynamics produce, of every leadership, and
  are all visited; nonfocal-lead *roots* are deferred to part two. The deferral
  is a scope restriction on root selection, not a theorem, and no dimension
  measured here is quoted for boundaries at large." And the closure is computed
  at **primitive-step granularity** (F5, Y1's correction), not by enumerating
  U(i): a one-trick local controller is exactly a choice function over the focal
  seat's within-trick information states, and the macro residual Pre_{i,u,e}
  factors as the composition of the four primitive residuals with the focal steps
  restricted by u. Hence the primitive closure contains every macro residual, and
  every primitive composition through a complete trick is realised by some u, so
  the two spans agree at trick-boundary interfaces while the primitive
  construction never materialises U(i). The macro one-trick kernel survives only
  as the compiled presentation certified at focal-lead roots (R-A13), where
  |U(i)| = |H|.

### R-Q3 — the declared continuation operator. RULING: ACCEPT (a); REJECT the identification in (b) and re-aim it; AMEND (c).

**(b) is the trap in this design, and it is P-Q2's trap in a new dress.** The
design names two different operators in one sentence. The m3 dag-v1 solver
(`walt-strat/src/hidden_scalar.rs`, the scalar sibling of
`walt-strat/src/hidden.rs`) is **treatment H**: "every viewer choice below the
root is made once per pooled information state against the whole particle set,"
under the §7.4 fixed uniform-legal field and the uniform fiber weighting. P-A6's
aggregate (`walt/walt-factory/examples/fiber_probe.rs:215`, `aggregate`) is the
**world-informed** focal-max / hidden-uniform node rule — treatment C/F, the
object Lemma V covers and P-A5 requires to be labelled "not a seat value." The
two differ exactly by strategy fusion (v0.4 §7.6, §7.7), so equating them makes
the proposed correctness gate compare an information-consistent optimum against
its own fusion upper bound.

- **R-A9 (field and belief are two objects, both uniform, never one sentence).**
  The **field** is the §7.4 fixed uniform-legal profile at the three hidden
  offsets (F4). The **belief** is the uniform weighting over Φ(**C**₀), the
  declared convention of v0.4 §14.2/§14.6 and the H solver's declared root
  weighting. Naming them together as "the declared uniform field" fuses §7.4 with
  §2.4. Both are declared in the header, separately, and the belief additionally
  carries P-A12's typing: on a fabricated void-free kernel it is a declared
  aggregation argument, not any seat's actual belief.
- **R-A10 (the concrete authority is H; the P-A6 aggregate is a one-sided
  diagnostic).** ACCEPT (a): the field of §3 for part one is the uniform-legal
  field with the focal seat controlled. The concrete ground truth for V/Q is
  **treatment H at the same field and belief**, not the P-A6 aggregate. The
  results file states verbatim: "The concrete authority for V and Q is treatment
  H (v0.4 §10.3), the information-consistent solve under the §7.4 uniform-legal
  field and the uniform fiber weighting. The P-A6 world-informed aggregate is a
  different operator: it maximises per world and is the strategy-fusion upper
  bound (§7.6, §7.7). It is recorded only as the one-sided diagnostic
  V^pred ≤ E_β[V*], and a gap between them is expected behaviour of two correct
  programs, never a defect." That inequality is Lemma X's pointwise fusion
  inequality averaged under β and may be asserted in-run in that direction only.
- **R-A11 (the observation models must be the same model).** The probe's
  observation contract must be the one `walt-strat/src/info.rs` implements for
  the H walk. If the two differ, the two programs optimise over different policy
  classes and the end-to-end gate is void — the same defect P-A9 guards against
  for values. The run asserts the correspondence explicitly and declares it in
  freeze 26; it does not assume it.
- **R-A12 (c).** ACCEPT: no belief adapters in part one, with the R-A9 split in
  force.

### R-Q4 — Experiment 0, macro-kernel certification. RULING: ACCEPT the receipt; AMEND (a); the "does γ determine the operator" question is REJECTED as posed — it is answered NO a priori.

- **R-A13 (γ, defined).** AMEND (a). "The 64-class level-1 alphabet" is
  ambiguous in exactly E-A8's way, and the ambiguity matters: 64 is the count of
  **level-1 r3 classes** (last-trick boundary *states*), and by E-Q2 a level-1
  class is the tuple (actor offset, three classifications, increment) =
  4 × 8 × 2 = 64, matching E-A7's anatomy rows. Read as that tuple it is a
  count-free label of a **completed trick** and is well typed at every grade;
  read as "the r3 class of the pre-trick state" it is a level-1 object and using
  it at grades 2–3 is a category error (at level j the unconstrained nodes have
  arity j — shape v2 — so a level-j boundary state is not a member of A_1).
  Ruling: **γ := the count-free completed-trick token (leader offset from focal;
  the three followers' classifications ∈ {follow, slough}; the count-free
  increment ∈ {0, e⋆})**, declared as that tuple in the freeze, with the
  identification with A_1 stated as holding at level 1 only and for the stated
  reason. The finer diagnostic column is permitted but must be defined here —
  the canonical form of the completed trick record under the declared transport —
  and never borrowed from E-A8's 32,532, which counts level-1 *state* forms.
- **R-A14 (Experiment 0 is a receipt; its headline question is answered before
  the run).** Under the proposed contract o = the full public trick, γ and the
  increment are functions of (i, o) and the successor state is determined by
  (ξ, o). Therefore: (i) the event triple (γ, r, o) is redundant — **declare
  e := o and record γ as a derived label**, so the event alphabet does not
  pretend to be finer than the observation; (ii) γ names no tile, so it cannot
  determine the successor information interface, and "does γ alone determine the
  normalized operator" is answered NO by construction, not by measurement — the
  same phenomenon Corollary S-rigid recorded at the seat level, where "named by
  role" and "named by tile" coincide because the stabiliser is trivial (S-A9);
  (iii) the analyst/player firewall of §3.3/§18.6 is **vacuous here** (nothing is
  analyst-only) and is stated as a standing rule that becomes live the moment a
  coarser observation contract is declared, not as a property this probe
  exercises. What survives as genuine content, and is ACCEPTED: the mass check
  Σ K = 1 per (i, u, ξ) and exact equality of primitive and folded path laws per
  successor bucket — implementation receipts in S-A18's sense, run at focal-lead
  roots only, where |U(i)| = |H|. The replacement measurable, which the run
  reports in place of the rejected question: the **operator-multiplicity census**
  — the number of distinct closure matrices encountered, grouped by γ, and after
  the declaration-fold quotient (R-A7). It is labelled freeze-relative (matrices
  depend on the basis, R-A21) and it is the honest fragment of §12.2/Experiment 4
  that part one can afford.
- **R-A15 (increment alphabet).** ACCEPT count-free, focal-team trick ∈ {0, 1},
  asserted in-run (P-A6). **E-A2 is restated where it now bites:** every number
  in this probe is sound only under the count-free contract; if count re-enters
  (v0.5 role re-entry) every basis, closure matrix and dimension here is void
  wholesale, never extended. v0.6 §10.2 agrees from its own side (the count
  schedule is not pip-symmetric; a score decoration breaks the fold of Corollary
  R-fold). Print this in the results file: it is the failure mode that would
  silently produce wrong numbers rather than a crash.

### R-Q5 — Experiment 1, the dimension census. RULING: AMEND (a) — the ladder as posed is two theorems and one measurement; AMEND (b); ACCEPT (c) as re-aimed by R-A10; ACCEPT (d).

- **R-A16 (what is measured).** The measured object is **dim V^val for the
  count-free expected-focal-trick contract** (Lemma R(3)), computed by the
  primitive-step closure of R-A8, seeded at terminals by the **zero** space, with
  observation-aggregated residuals and the immediate generators g_{i,u}. The
  distribution contract (ii) and the predicate-on-top contract (iii) are
  **reported as THEOREM rows reading r = |X_i|** with Lemma R(c)–(d) cited, and
  are not run. The lawful enrichment that is not degenerate, and which the run
  MAY measure at the same cost, is contract **(i′): (i) plus expected
  next-leader-offset readouts as immediate generators** — it seeds no constant
  and is the control alphabet part two will need. If the builder nevertheless
  runs §6.2's closure verbatim, it asserts the identity r = |X_i| as a receipt
  and reports it as such; a §6.2 run returning anything else is a stop-and-report
  bug (NO-RESCUE), because Lemma R(c) says what it must return.
- **R-A17 (report schema, and two cardinalities that are one).** Per information
  interface and grade, integers first (P-A11): |X_i|; dim V^val; the number of
  distinct behavioural rows; |U(i)|; the observation count; basis and
  closure-matrix sparsity. Four corrections. (i) **Behavioural rows are defined
  and computed as the distinct rows of the |X_i| × dim V^val basis-evaluation
  matrix** — exact by construction, never sampled, and no test enumeration is
  needed, since two states agree on every test iff they agree on a basis. (ii)
  **"Partition-lump size" is not a second number:** under v0.6 §5.3's own
  definition a one-hot positive realization is a partition on whose blocks every
  test is constant, so the minimal one is exactly the row-equality partition.
  Report it once, as the behavioural-row count, and state the identity;
  presenting it as an independent row would imply a measurement that was never
  made (E-A8's lesson). (iii) **The row-equality partition is a response-equality
  object, not a dynamics quotient.** v0.4 §12.4's caution therefore applies to it
  in the direction opposite to r3's caveat: it must never be used as a state
  partition for a solver, and it is not an r3-style class count and may never be
  compared with one. (iv) Sparsity figures are freeze-relative (R-A21); the
  dimension itself is not. Root interfaces and interior interfaces are reported
  separately (F6's root-only/full-carrier discipline).
- **R-A18 (the correctness gate).** ACCEPT (c) with R-A10's substitution: for
  every measured coordinate, the predictive V and per-action Q computed from
  V^val must equal **treatment H's** V and Q exactly, in exact arithmetic, at the
  declared count-free valuation (v0.6 §16.7's equality block, receipt-style,
  P-A9's discipline). A mismatch is stop-and-report per NO-RESCUE; never patched,
  never reconciled by adjustment. If H does not complete at a rung within its
  declared budget, that is a declared stop printed with what was reached (P-A16,
  E-A16), and every dimension row at that rung is printed with "correctness gate
  unmet" beside it — never silently. The fusion diagnostic of R-A10 is reported
  alongside but is never the gate.
- **R-A19 (clean slate, with one inheritance).** ACCEPT (d): the r3 class store
  does not participate, the 64 labels enter only as the transition alphabet of
  R-A13, and S5h's negative (cone identity cannot short-circuit descent; B : A1 ≈
  4.3–4.9) stands untouched and is restated in the header (X-A14). The
  inheritance the phrase "clean slate" must not erase: the void-free capacity
  kernel construction and freeze 7's fiber enumeration order are **reused, not
  reinvented**, or the coordinates are not comparable with the S5g/S5h rows.
- **R-A20 (declare the Gate-B criterion before the run).** Y2's Q2 discipline:
  the refutation criterion is fixed in the design text in advance, in the form
  "payoff CONFIRMED if dim V^val grows materially slower than |X| across the
  measured grades — the reported comparison being the growth ratios
  dim V^val(n+1)/dim V^val(n) against |X_{n+1}|/|X_n| = 15 and 56/3 — REFUTED if
  the growth ratios are of the same order," with the thresholds written down
  before any number exists. P-A21 governs everything beyond: three rungs are not
  a law, an implied grade-7 dimension is an extrapolation at exploratory tier and
  is never a statement about an unrun computation, and no dimension at any grade
  is quoted for the opening.

### R-Q6 — arithmetic and freezes. RULING: ACCEPT the arithmetic; REJECT the renumbering.

- **R-A21 (arithmetic).** ACCEPT arbitrary-precision rationals for the closure
  module: exact, no floats, the clippy `float_arithmetic` deny and the f32/f64
  grep untouched. `walt_geom::Q` is `Ratio<i128>`
  (`walt/walt-geom/src/rat.rs:6`), so every conversion at the boundary is checked
  and an overflow is a **stop-and-report**, never a truncation, never a
  saturation, and never a number that reaches a results line. Two properties to
  state in the header because they are easy to confuse: **the predictive
  dimension is freeze-independent** (it is a mathematical invariant of the space,
  not of the pivot rule), while **the basis, the closure matrices and every
  sparsity figure are freeze-dependent**; a results file that labels the second
  group as reproducible facts without naming the freeze is misreporting.
- **R-A22 (freeze numbering: continue at 22).** REJECT the reassignment of
  18–21. Those numbers are **spent**: S-A19 assigned them, and S-A2 cites
  "freeze 18" inside the text of Lemma S-fold's licensing argument, so reusing
  them would corrupt an adjudicated ruling that is binding precedent regardless
  of whether the parked build ever runs. Freeze numbers are a global monotone
  registry; a parked build does not return its numbers. New freezes: **(22)** the
  information-interface encoding of R-A4 — live set, capacity-cell system,
  leader offset, grade, the declarations O_Σ = ∅ and monitor = none, the absence
  of any accumulated outcome — together with the coordinate enumeration order
  (restating freeze 7 wherever it is reused); **(23)** the closure discipline —
  primitive-step granularity, the per-contract terminal seed sets, the
  deterministic pivot rule (first nonzero in declared order), basis storage
  order, and f_0's index convention where a constant is present at all;
  **(24)** the observation-label encoding, with e := o and γ as the derived
  R-A13 tuple; **(25)** the decimation constants (g, W) per grade in P-A15's
  exact form; **(26)** the concrete-authority identification — the H solver and
  its version, its budget, its valuation, its fiber weighting, and its
  observation model (R-A11). Freezes 1–21 are in force and restated unchanged.

### R-Q7 — results discipline and the claim fence. RULING: ACCEPT-WITH-AMENDMENT.

- **R-A23 (the fence, verbatim).** P-A20's boilerplate lineage, integers first,
  per-coordinate tables then per-grade summaries, one file
  `walt/walt-factory/results/predictive_rank_2026-08-12.txt`, and the F7
  both-outcomes framing are confirmed. The fence sentence, verbatim: "THE FENCE.
  A predictive dimension is a statement about the linear span of a declared
  family of continuation tests over a declared coordinate's void-free capacity
  fiber, under the declared field, belief, count-free contract, observation
  contract and grade. It licenses NO runtime or tractability claim of any kind:
  moment compilation (v0.6 Gate D) is a separate, unmeasured experiment, and a
  small dimension whose moments require enumerating the fiber solves nothing
  (v0.6 §18.3). It is not a count of states, not a class count, not an r3-style
  dynamics quotient, and not a value partition. It promotes no v0.6 theorem. The
  numbers are coordinate-relative and are never quoted for the opening or for any
  grade not measured. The concrete authority is treatment H; a disagreement with
  it is a stop-and-report bug, never reconciled by adjustment; a divergence from
  the world-informed P-A6 aggregate is not a disagreement at all but the expected
  strategy-fusion gap (R-A10)."
- **R-A24 (how a degenerate row is reported).** A row reading dim V = |X| for
  contracts (ii)/(iii), or for a §6.2 run, is printed as **THEOREM (Lemma R(c),
  RECEIPT RUN)** in S-A18's style, never as a measurement and never as Gate-B
  evidence. The accompanying sentence, verbatim: "This row is forced by the
  observation structure of the game, not by the game's strategic complexity:
  every tile is eventually played and every play is publicly attributed, so a
  complete record determines the world, and any closure seeded with a nonzero
  constant contains every singleton indicator. It is not evidence that the
  decision problem is high-dimensional, and it must not be repaired by coarsening
  the observation contract, which would change the information model and
  therefore the operator (§7.7)."

**Both outcomes remain results** (F7, NO-RESCUE). Lemma R stands regardless of
every number this probe produces: it identifies the exact object part one may
measure, proves that the other two rungs of the proposed ladder are predetermined,
and supplies the value closure that makes the pairing J_ρ(β) = ψ(β)c_ρ lawful
without a normaliser. What it does not do is promise that dim V^val is small —
that is the measurement, and this section is careful to leave it genuinely open.

## Policy-geometry probe rulings (2026-08-12)

**Adjudicator:** walt-math. **Tier:** exploratory throughout; v0.6 §8.5 /
Experiment 2 is design guidance and nothing below promotes it. **Basis:** v0.6
§§5, 8, 11; v0.4 §2.1–2.6, §5.5, §6.8, §7.2, §7.4–7.7, §10.1, §10.3, §12.4;
v0.5 §12.6A and its BOUNDARY. F1–F7, r3 Q1–Q5, Y1–Y3, shape v2, P-A1..P-A21,
X-A1..X-A19, E-A1..E-A21, S-A1..S-A21, R-A1..R-A24 and Lemmas V, X, E, S,
S-fold, S-det, R, Corollaries S-rigid, R-fold are inherited unchanged.
Amendments are numbered PG-A1.. and are builder obligations; a run that omits
one is not the adjudicated probe. The S6a run
(`results/predictive_rank_2026-08-12.txt`) is the standing record this probe
extends; its freezes 22–26 are in force and restated, not renumbered.

**Headline, stated first, because it decides what this probe can and cannot
read.** In the measured domain the decision side does not exist below grade 3.
At a focal-lead root of grade n the focal seat plays exactly one tile per trick
and the final trick is forced, so grade 1 has one policy and grade 2 has exactly
one policy per root action (Proposition G-flat). **Grade 3 is the only grade with
a policy set at all**, and there the free choice layer is exactly trick 2, so
N_pol(a) = 2^{k(a)} with k(a) the number of trick-1 records leaving two legal
tiles. Two consequences bind the design. First, **PG-Q6's proposed
growth-ratio criterion cannot be evaluated** — a ratio across grades has one
usable data point and two forced 1s; the reading must be an absolute-magnitude
criterion at grade 3, fixed below (PG-A15). Second, a results file that reported
"N_par = 1, 1, 40 across grades 1–3, a collapse of many orders" would be
reporting Proposition G-flat as a finding; the strawman guard is mandatory
(PG-A16). Two further traps are ruled on below: **N_vec is destroyed by the very
pruning that makes N_par feasible** (the four cardinalities cannot all come from
one run at one grade, PG-A7), and **N_exp as defined in the design is not
preserved by Pareto pruning** — the design's own justification of N_par says
"full support" while its definition of N_exp says "some belief", and the gap
between those two is exactly a set of vectors that pruning silently deletes
(PG-A4). Fixed by adopting unique optimality as the definition, which is both
what the LP computes and what pruning provably preserves.

### Proposition G-flat (grades 1 and 2 carry no policy geometry)

*Statement.* At a focal-lead root information interface of grade n in the S6a
domain: the focal seat has exactly n plays, one per trick, and at the final trick
exactly one tile remains, so that decision point is forced. Hence

- **n = 1:** the root action set is a singleton and
  N_pol = N_vec = N_par = N_exp = 1;
- **n = 2:** for each root action a the whole continuation is forced and
  N_pol(a) = N_vec(a) = N_par(a) = N_exp(a) = 1;
- **n = 3:** the only free choice layer is trick 2, whose focal information
  states are indexed by the trick-1 public record o (the focal's hand after a is
  determined by a, so the record is the whole information state), and each such
  state leaves one or two legal tiles. Therefore

    N_pol(a) = Π_o |legal(o)| = 2^{k(a)},   k(a) = #{o : |legal(o)| = 2}.

*Proof.* The focal plays one tile per trick (v0.4 §1.2), so after the root action
its hand has n−1 tiles and at the last trick exactly one, which is legal by
§1.5 (follow if able, else anything) and is the only tile it holds. At n = 2 the
only non-root decision point is that forced one. At n = 3 the trick-2 decision
faces a two-tile hand, whose legal subset is the follow-set of the led context if
nonempty and otherwise both tiles (§1.2, §1.5), so |legal| ∈ {1, 2}; the trick-3
decision is forced. Policies are functions of the observation record (R-A11), and
under a fixed root action the record determines the information state, so the
choices at distinct records are independent. ∎

*Three consequences the builder must carry.* (i) The grade-1 and grade-2 rows are
**receipts, not measurements** (S-A18's typing): their expected values are known
in advance and a discrepancy is stop-and-report. (ii) The plan/reduced-strategy
distinction (PG-A3) is **inert at n ≤ 3** — every trick-2 information state is
realizable given a, and the trick-3 factor is 1 — so N_pol as a flat product is
correct here and would stop being correct at n ≥ 4. (iii) The whole of PG-Q2's
"astronomically large policy set" is the single Minkowski fold over the k(a)
free records at grade 3; that is where backward pruning earns its place, and it
is the only place.

### Lemma G (backward pruning: what it preserves and what it destroys)

*Setup.* Fix a root information interface i under root action a, with fiber X_i
and value vectors in Q^{X_i}. By Lemma R(b) the composition is

  V_ρ = g_{i,u} + Σ_o Pre_{i,u,o}(V_{ρ_o}),

with each Pre_{i,u,o} a **positive** linear operator (its kernel entries are
products of the field's unit fractions, all ≥ 0) and the continuation chosen
independently per observation o. Write S_j for a set of continuation vectors at
interface j, ≤ for the pointwise order, max(S) for the Pareto-maximal elements,
and for a finite S ⊂ Q^{X}

  Exp(S) = { v ∈ S : v is the **unique** maximiser of E_β[·] over S for some
             β ∈ Δ(X) }.

*Claims.*

1. **(monotone composition)** If v ≤ w pointwise then Pre_{i,u,o}(v) ≤
   Pre_{i,u,o}(w) pointwise, and the immediate term g_{i,u} does not depend on
   the continuation. Hence the composition is monotone in every argument.
2. **(frontier preservation, ties included)** Let R be the set of root vectors
   built from the full successor sets and R′ the set built from max(S_j) at every
   successor. Then max(R) ⊆ R′ and max(R′) = max(R). So backward Pareto pruning
   preserves the root Pareto frontier **exactly**, as a set, not merely in
   cardinality.
3. **(incremental pruning is exact, and is mandatory)** Pruning after each
   partial sum is exact: max((max(A ⊕ B)) ⊕ C) = max(A ⊕ B ⊕ C), where ⊕ is the
   elementwise sum of two vector sets. Without it the fold materialises Π_o|S_o|
   combinations — 2^{k(a)} at grade 3 — before any pruning can occur, so
   per-interface pruning alone does **not** make the computation feasible.
4. **(exposure)** Exp(S) is the unique minimal subset of S whose upper envelope
   equals that of S; Exp(S) ⊆ max(S); and Exp(max(S)) = Exp(S). Hence Pareto
   pruning preserves the exposed set exactly, over the **whole** simplex,
   boundary beliefs included. The **weak** variant — "attains the maximum for
   some β, ties allowed" — is a strictly larger count in general and is **not**
   preserved by pruning.
5. **(convex dominance)** v ∉ Exp(S) iff there are convex weights λ on S∖{v}
   with Σλ_w w ≥ v pointwise. Pruning by this rule preserves Exp exactly, and it
   is strictly stronger than Pareto pruning — therefore it **destroys N_par**.
6. **(the negative)** No pruning rule preserves N_vec. A run that prunes cannot
   report N_vec.

*Proofs.* (1) K ≥ 0 entrywise, so v ≤ w gives Σ_{ξ′}K(ξ; o, ξ′)v(ξ′) ≤
Σ_{ξ′}K(ξ; o, ξ′)w(ξ′) for every ξ. (2) Let C(v_1,…,v_m) denote the composition.
Take C(v) ∈ max(R); for each o pick w_o ∈ max(S_o) with v_o ≤ w_o (finite poset,
so a maximal element above every element exists). By (1) C(v) ≤ C(w) ∈ R;
maximality of C(v) forces C(v) = C(w) ∈ R′, so max(R) ⊆ R′. Elements of max(R)
are then maximal in R′ ⊆ R a fortiori. Conversely if x ∈ max(R′) were not
maximal in R, some y ∈ R with y > x exists; take y′ ∈ max(R) with y ≤ y′; then
y′ ∈ R′ and y′ > x, contradicting maximality in R′. Hence max(R′) = max(R). (3)
Identical argument with C = ⊕. (4) *Minimality and sufficiency:* for distinct
v, w the set {β : E_β[v] = E_β[w]} is the zero set of a nonzero linear functional
and so has empty interior in the affine hull of Δ; the union over the finitely
many pairs is therefore closed with empty interior, so on a dense open subset of
Δ the maximiser is unique and lies in Exp(S). The envelope f(β) = max_{v∈S}E_β[v]
is continuous, so f agrees with the envelope of Exp(S) on a dense set and hence
everywhere. Each v ∈ Exp(S) is the unique maximiser on a nonempty open set, so any
S″ ⊆ S with the same envelope must contain v; Exp(S) is therefore the unique
minimal representation. *Exp ⊆ max:* let v be uniquely maximal at β and suppose
v ≤ w, v ≠ w, w ∈ S. If the strict coordinates carry β-mass then E_β[w] > E_β[v],
contradicting optimality; if they do not, then E_β[w] = E_β[v], contradicting
uniqueness. *Stability:* max(S) has the same envelope as S (every element is ≤ a
maximal one), so by minimality Exp(max(S)) = Exp(S). (5) The equivalence is the
minimax/LP-duality form of δ* ≤ 0 in the exposure programme of PG-A9; removing
such a v leaves the envelope unchanged (both bounds are immediate), so by the
uniqueness in (4) it leaves Exp unchanged; and Σλ_w w ≥ v with |supp λ| > 1 does
not imply v ≤ w for any single w, so v can lie in max(S). (6) Immediate: pruning
discards vectors, and the discarded ones are counted by N_vec. ∎

*The one thing Lemma G does not license.* Exp is preserved for the **value
function**, not for the identity of every optimal policy: at a belief on a face,
a Pareto-dominated vector can tie for the maximum and is then optimal without
being exposed. A seat whose belief has support strictly inside Φ(**C**₀) — which
is the real seat's situation, Φ(**C**) ⊊ Φ(**C**₀), P-A1 — is exactly that case.
N_exp is therefore a statement about the declared cost domain's value function and
never a count of the strategies any seat needs (PG-A4).

### PG-Q1 — the lawful policy set. RULING: ACCEPT the definition with two amendments; the exclusion of mixed policies is PROVED here, not asserted.

- **PG-A1 (definition and vocabulary).** ACCEPT: a policy is a deterministic
  information-consistent policy (v0.4 §7.2, §10.1) — one choice per focal
  observation record, the record being the one `walt-strat/src/info.rs`
  implements (R-A11, freeze 26) — extending the root action a. Two vocabulary
  corrections, both load-bearing. (i) The design's "reachable focal information
  states" must read **"records realizable in the declared fiber"**: feasible ≠
  reachable is a typed distinction and the domain is the void-free capacity fiber
  (P-A1); nothing here is asserted to arise in play. (ii) The design calls the
  uniform belief "the point belief"; it is a single **point of Δ(X)**, not a point
  mass, and the results file says so.
- **PG-A2 (mixed policies excluded — with the proof, since the design asked).**
  For a single optimising seat against a fixed field, the value in each world is
  linear in the mixture over pure policies: V_μ = Σ_ρ μ(ρ)V_ρ. So the achievable
  vector set is conv{V_ρ}, and a linear functional E_β[·] attains its maximum
  over a polytope at a vertex, hence at some deterministic V_ρ. Mixed policies
  therefore never enlarge the exposed set and never change any envelope value;
  they can only tie. (The focal seat has perfect recall — its own plays are
  public and in the record — so behavioural and mixed randomisation coincide and
  the same conclusion covers behavioural policies.) Excluding them is lawful; the
  results file states the reason in one sentence rather than asserting the fact.
- **PG-A3 (N_pol counts plans; declare it, and compute the reduced count where
  they differ).** A flat product over information states counts **plans of
  action**, including choices at states the plan's own earlier choices make
  unrealizable; the behaviourally distinct count is the **reduced** one, given by
  the recursion "number of reduced continuations at a state = Σ over legal
  actions of Π over the information states realizable **given that action**".
  By Proposition G-flat(ii) the two coincide at n ≤ 3, so the design's product is
  correct here — but the definition of record must be the one that keeps it
  correct, and the results file states which count is reported and why they
  coincide. N_pol is an exact big integer (2^{k(a)} at grade 3); it is printed
  exactly, never as a float and never as an order of magnitude alone, and k(a)
  is printed beside it since k(a) is the quantity the thresholds use (PG-A15).

### PG-Q2 — dominance pruning. RULING: the lemma is DELIVERED (Lemma G); (a) AMEND the definition of N_exp; (b) ACCEPT backward pruning, with the composition granularity and the incremental fold made mandatory.

- **PG-A4 (the definition of exposure, amended).** The design is internally
  inconsistent: N_par's justification says a dominated vector "can never be
  exposed for any belief **with full support**", while N_exp is defined as
  attaining the maximum "for **SOME** belief β on the fiber". On a face of the
  simplex a Pareto-dominated vector can tie for the maximum, so under the
  design's own N_exp definition pruning deletes exposed vectors — the silent
  shrink. **N_exp is redefined as |Exp|, the number of vectors that are the
  UNIQUE maximiser of E_β[·] for some β ∈ Δ(X).** This is what the exposure
  programme of PG-A9 decides, it is what Lemma G(4) proves pruning preserves,
  and Lemma G(4) also proves it is the unique minimal representation of the value
  function over the entire simplex. The weak variant (ties admitted) is a
  different, larger cardinality: it may be reported only as its own labelled row
  and never as N_exp (E-A8's rule). Mandatory type line, verbatim: "N_exp counts
  the vectors needed to represent the value function of the declared cost domain
  over its whole belief simplex. It is not a count of strategies any seat needs:
  at a belief whose support is smaller than Φ(**C**₀) — the real seat's case,
  P-A1 — dominated vectors tie for the optimum and are optimal without being
  exposed. Beliefs here are declared aggregation arguments (P-A12), not any
  seat's belief."
- **PG-A5 (backward pruning: ACCEPT, per Lemma G(1)–(3), with two mandatory
  implementation clauses).** (i) **Granularity.** The recursion runs at
  primitive-step granularity per R-A8, never over an enumerated one-trick
  controller alphabet: at a focal primitive step the set is the **union** over
  legal tiles of the successor sets (no maximisation — this enumerates policies,
  it does not optimise); at a hidden primitive step it is the positive-weighted
  Minkowski sum over the branches; increments enter at trick completion.
  (ii) **Incremental fold.** The sum over branches is folded one branch at a time
  with a Pareto prune after each partial sum (Lemma G(3)). Without it the run
  materialises 2^{k(a)} combinations and the "linchpin" does not hold; with it
  the claim in PG-Q2(b) is exactly Lemma G(2) and is proved.
- **PG-A6 (convex-dominance pruning: lawful for N_exp, forbidden when N_par is
  reported).** By Lemma G(5) pruning a vector dominated by a convex combination
  of the others preserves Exp exactly and is strictly stronger than Pareto
  pruning. It therefore **destroys N_par**. A run may use it only in a pass whose
  reported outputs are N_exp alone, declared as such; N_par may never be read
  from a convex-pruned set. Mixing the two rules in one pass and reporting both
  counts is the unlawful pruning step this probe is most likely to commit.
- **PG-A7 (the four cardinalities cannot come from one run; say which grades give
  which).** N_vec is destroyed by every pruning rule (Lemma G(6)). So: N_vec is
  reported only where the **unpruned** vector set is enumerated to completion
  within the declared budget, and its absence elsewhere is a declared stop, not a
  finding. N_par comes from the Pareto-pruned run. N_exp comes from PG-A9's
  programme applied to that frontier. N_pol is closed-form. Each row states which
  pass produced it; a table that presents all four for a grade where the unpruned
  set was never enumerated is misreporting.
- **PG-A8 (the receipts, since "by construction" is not a receipt).** Mandatory,
  in the P-A9/R-A18 style. (i) **Lemma-G receipt:** at grade 1, grade 2 and at
  one declared grade-3 coordinate with the smallest k(a), compute the frontier
  both ways — prune-then-compose and compose-then-prune over the full unpruned
  set — and assert set equality, not merely equal cardinality. If the unpruned
  set is out of budget at every grade-3 coordinate, say so and run the receipt at
  grades 1–2 only, printing that limitation. (ii) **Authority receipt:** for
  every coordinate and root action, assert max over the frontier of E_{β₀}[V]
  equals treatment H's Q(a) exactly, and the max over actions equals H's V
  (freeze 26, R-A18). This ties the new machinery to the concrete authority and
  is the strongest available check that the composition is the same object S6a
  measured. (iii) Any failure is stop-and-report per NO-RESCUE; never patched.

### PG-Q3 — the exposure computation. RULING: ACCEPT arms 1 and 2 with the method NAMED and frozen; AMEND arm 3 — grade-3 N_exp is a budget stop, not a method impossibility.

- **PG-A9 (the method, named).** The exposure test is the standard
  useful-vector linear programme (Lark's programme, as used for α-vector pruning
  in exact POMDP incremental pruning): for v ∈ S,

    maximise δ  subject to  Σ_ξ β(ξ)(v(ξ) − w(ξ)) ≥ δ for every w ∈ S∖{v},
                            Σ_ξ β(ξ) = 1,  β ≥ 0,

  and v ∈ Exp(S) iff δ* > 0. Solver: **exact-rational primal simplex with
  Bland's rule** — Bland's rule for guaranteed termination without cycling, exact
  rationals throughout (no floats anywhere, P-A19), arbitrary precision per
  R-A21 with overflow a stop-and-report. Both the programme and the pivot rule
  are frozen (freeze 29). Note the two constraints this discharges: the LP is
  run against the **Pareto frontier**, which by Lemma G(4) gives the same answer
  as against the full set, and by the redundancy fact — v ≤ w implies
  β·(u−v) ≥ β·(u−w) for β ≥ 0, so a dominated vector's constraint is implied by
  its dominator's — pruning does not even relax the feasible region.
- **PG-A10 (witnesses both ways; never the word "certificate", R-A2).** Every
  vector reported exposed carries an exact rational witness belief β with
  E_β[v] > E_β[w] for all w ≠ v, re-checked by an independent evaluation. Every
  vector reported not exposed carries the dual witness — convex weights λ with
  Σλ_w w ≥ v pointwise (Lemma G(5)) — also re-checked. Witness checking is
  independent of the LP code path; an LP that reports a status it cannot witness
  is a stop-and-report.
- **PG-A11 (arm 3 is a declared budget stop, and a stopped run still yields an
  exact bound).** REJECT the phrasing "not measured, method infeasible at this
  dimension": that is a claim about the method, and the binding quantity is the
  frontier size, which is unknown before the run. Grade-3 N_exp **is attempted**
  under a declared budget; if the budget is exhausted the run prints the stop with
  the number of vectors tested and the number verified exposed (P-A16, E-A16: a
  wall is a stop, never a finding). Because every exposed verdict carries a
  witness, a stopped run reports an exact **lower bound** on N_exp, labelled as a
  bound — never an approximation, never a sample.

### PG-Q4 — the uniform-belief argmax diagnostic. RULING: ACCEPT-WITH-AMENDMENT.

- **PG-A12.** Lawful to report alongside, re-typed: (i) the number of distinct
  root actions attaining V_B, as an exact tie count in Q; (ii) the number of
  distinct **vectors** on the frontier attaining max E_{β₀} (well defined and
  pruning-safe: a β₀-optimal vector is Pareto-undominated, since β₀ has full
  support). (iii) The design's "count of policies attaining the optimum" is
  REJECTED as posed — it is a plan count over a set the pruned run does not
  hold; it may be reported only at a grade where the unpruned enumeration
  completed, and is then labelled a plan count (PG-A3). Mandatory fence,
  restating R-A17(iii): "The argmax sets here are response-equality objects.
  v0.4 §12.4 applies: they are not a dynamics quotient, they are not an r3-style
  class count, and they may never be used as a solver's state partition. No
  partition claim is made or implied."

### PG-Q5 — feasibility declarations and freezes. RULING: ACCEPT with three additions.

- **PG-A13 (caps, stops, and what a capped coordinate may report).** ACCEPT the
  cap discipline. Additions: the cap applies to the running frontier size **at
  each incremental partial sum**, not only per interface, and the stop prints the
  coordinate, the root action, the branch index reached and the frontier size at
  the stop. **A capped coordinate reports no N_par at all** — not a partial count
  and not a bound: the Pareto-maximal elements of a partially processed set need
  not be maximal in the whole, so an interrupted frontier bounds nothing in
  either direction. (This is the opposite of PG-A11's N_exp case, where each
  verdict is independently witnessed and a stop does bound. The asymmetry is
  exact and must not be smoothed over.) The grade order 1 → 2 → 3 and the
  declared-in-advance grade-3 conditionality are confirmed, printed either way.
- **PG-A14 (new freezes, continuing at 27).** Freezes 1–26 are in force and
  restated. **(27)** the vector encoding — world order = the S6a kernel world
  order of freeze 23, exact rationals — and the dedup order; **(28)** the
  dominance-check order and the incremental-fold order over observation branches,
  which is a determinism freeze because the stop point depends on it;
  **(29)** the exposure programme of PG-A9 and its pivot rule; **(30)** the caps
  (per-interface frontier cap, per-partial-sum cap, per-coordinate budget) and
  the grade-3 conditionality rule; **(31)** the policy-counting convention —
  plans versus reduced, per PG-A3.

### PG-Q6 — results discipline and the reading. RULING: AMEND — the proposed growth criterion is unavailable; absolute thresholds are fixed here, before any number exists.

- **PG-A15 (the pre-declared reading, concrete).** By Proposition G-flat the
  growth-ratio criterion has one usable data point and is REJECTED. The reading
  is fixed now, over grade 3 only, from quantities the run prints: let
  k(a) = #{records with two legal tiles} under root action a, K = max over
  measured (coordinate, action) of k(a), P = max over measured (coordinate,
  action) of N_par(a), and E the same for N_exp where measured. Then:
  - **STRONG COLLAPSE** iff P ≤ K + 1 — the frontier is at most linear in the
    number of decision points, against N_pol = 2^K;
  - **COLLAPSE** iff K + 1 < P ≤ |X_3| = 1680 — the frontier stays at or below
    the world count, the same anchor Gate B was read against;
  - **REFUTED** iff P > 1680;
  - **STOPPED, NO VERDICT** if any measured coordinate hits a cap (PG-A13).
  The identical bands are applied to E and reported as a **separate** verdict
  line; the N_exp verdict never inherits the N_par verdict and vice versa, and
  where N_exp is only bounded below (PG-A11) its line reads "lower bound L;
  verdict withheld" unless L alone already forces REFUTED.
- **PG-A16 (the two anti-strawman lines, mandatory).** (i) "N_par/N_pol is
  printed as bookkeeping and is not a criterion: N_pol = 2^{k(a)} counts plans
  and is astronomically large by construction, so any frontier at all is 'orders
  below' it. The criterion is PG-A15's absolute bands." (ii) "The grade-1 and
  grade-2 rows are 1 by Proposition G-flat — the focal seat has no choice there —
  and are receipts, not evidence of collapse. No cross-grade ratio in this probe
  is a measurement."
- **PG-A17 (the fence, verbatim, extending R-A23).** "THE FENCE. This probe
  counts exact objects over a declared coordinate's void-free capacity fiber
  under the declared field and belief (R-A9), the count-free expected-focal-trick
  valuation, the R-A11 observation contract and the S6a freezes. Four
  cardinalities are reported and never conflated: N_pol (plans), N_vec (distinct
  vectors, only where the unpruned set was enumerated), N_par (Pareto frontier),
  N_exp (uniquely-optimal-for-some-belief, PG-A4). **No similarity claim and no
  tolerance claim of any kind is made or supported.** 'Playing this domino means
  I am likely to get 32 one way or the other' is a statement about score
  distributions under a tolerance, and this probe measures neither: score is out
  of scope (E-A2, and by Lemma R(c)–(d) the distribution contract has predictive
  dimension |X|), and δ-similarity is future mathematics requiring its own typed
  rulings. A vector here is an expected-trick profile over the declared fiber, not
  an outcome law and not 'an outcome'. No partition claim (PG-A12), no runtime or
  tractability claim (v0.6 §18.3), no promotion of any v0.6 theorem, no number
  quoted for the opening or for any grade not measured. The concrete authority
  remains treatment H; a disagreement with it is a stop-and-report bug."
- **PG-A18 (both outcomes, and what each one means).** Confirmed (F7,
  NO-RESCUE). A collapse verdict says the decision side of the declared cost
  domain has a small exact representation at grade 3 — it does not rescue Gate B,
  does not transfer to the opening, and does not establish anything about
  similarity. A refutation adds one named entry to the bottleneck list and is
  equally a result. A STOPPED verdict is neither and is never presented as a
  weak version of either.

**Both outcomes remain results.** Lemma G stands regardless of every number this
probe produces: it proves the pruning the design needs is exact, identifies the
one definition of exposure under which that is true, and names the two counts
(N_vec, and N_par under convex pruning) that the same pruning silently destroys.
Proposition G-flat stands likewise, and it is the reason this probe has exactly
one measurement in it.

## Decision-deadness probe rulings (2026-08-12)

**Adjudicator:** walt-math. **Tier:** exploratory throughout. **Basis:** v0.4
§1.2–1.5, §2.1–2.6, §6.8, §7.4–7.7, §10.3, §12.4; v0.5 count-free kernel and
BOUNDARY. F1–F7, r3 Q1–Q5, Y1–Y3, shape v2, P-A1..P-A21, X-A1..X-A19,
E-A1..E-A21, S-A1..S-A21, R-A1..R-A24, PG-A1..PG-A18 and Lemmas V, X, E, S,
S-fold, S-det, R, G, Corollaries S-rigid, R-fold, Proposition G-flat are
inherited unchanged. Amendments are numbered J-A1.. and are builder obligations.

**Headline, stated first.** Four rulings decide this design.

1. **The exhaustion margin is unnecessary.** D0 needs no counting argument and no
   margin against adversarial orderings, because the guaranteed beater is already
   in every trick: the leader always plays a tile of the led context. If focal's
   tiles are below every potential leader's tile in every context they can be
   played to, focal cannot win, full stop (Proposition J-0). D0 becomes three
   bitset tests, exactly sound, with no constant to freeze.
2. **The design's reason for demoting D0 is a category error.** "The tie-roots
   are not no-possible-winner hands" is a statement about the ROOT HAND; D0 is a
   NODE-LOCAL condition evaluated at the decision node, after the root action is
   spent and against the record actually observed. Whether focal is shut out at
   the trick-2 decision nodes of hand [2-1, 2-2, 6-3] is unmeasured, and the
   design has pre-emptively ranked D0 below D1 on an invalid inference. D0's
   recall must be measured node-locally before any such ordering is asserted.
3. **The count guard is proved, twice, and refuted once.** Jason's conjecture
   holds for D0 (Lemma J(c)) and, by a second and independent route, for the
   transposition form of D1 — where the guard is exactly what lifts E-A2's
   count-free restriction on a structural transport, because the transport moves
   only two zero-point tiles and fixes every other tile (Proposition J-1). It
   **fails** for the win-both form: when focal wins tricks, the tiles the other
   seats contribute to each trick change with focal's choices even though focal's
   own tiles are pointless, so count is not invariant (Proposition J-win).
4. **"Order-exchangeability" is not a detector.** It is the conclusion —
   value-invariance under the swap — restated. Accepting it as a detector would
   let the implementation substitute an unproved test for a proof. It is REJECTED
   under that name; two exactly stated members replace it, and **no cheap
   sufficient structural condition is known at adjudication time for the six
   specimens' ties**. That is a finding, not a gap to be filled by guessing.

### The typing that governs this section (three node properties, never fused)

- **forced** — |legal| = 1: no decision exists. Free to detect, and worth
  nothing; counting forced nodes as harvested deadness inflates every coverage
  figure for free (J-A15).
- **decision-dead** — every information-consistent policy from the node has the
  identical value function on the node's fiber: N_vec = 1 in PG's sense. This is
  the design's object and the only one that licenses collapsing the subtree.
- **dominant** — one Pareto-undominated vector: N_par = N_exp = 1. **S6b's seven
  singleton roots are this, not deadness**, which is exactly why policy_inspect
  found only six of them indifferent and the seventh resolving 108 decisions to
  "play 1-1 over 1-0." Dominance licenses fixing a choice only if the dominant
  choice can be identified cheaply, which is the work itself; harvesting it is a
  separate direction and is out of scope here.

(N_par = 1 ⟺ N_exp = 1: if one vector is optimal at every belief, taking point
masses gives pointwise dominance. So the ladder is forced ⊂ dead ⊂ dominant, with
both inclusions strict on the S6b evidence.)

### Lemma J (non-interference ⇒ decision-deadness, and when count survives)

*Hypothesis (NI) at node i.* For every world ω ∈ X_i, every information-consistent
focal policy and every legal continuation: (i) focal is not the leader of the
current trick and never becomes the leader of a later one; (ii) no tile focal
plays is ever the maximal trick key of its trick.

*Conclusion.* (a) The joint law of (each non-focal seat's play sequence, each
trick's winner) is identical under every focal policy — for the declared
uniform-legal field and for any field whose per-seat play distribution depends
only on that seat's own hand, the led context and its position in the trick.
(b) Hence the count-free value V_ρ is identical for every ρ: the node is
decision-dead. (c) If moreover H ∩ COUNT = ∅ for focal's remaining hand H, the
value is identical for every valuation that reads the play only through the trick
winners and the point values of the tiles falling in each trick — in particular
the trick-plus-count valuation.

*[GENERALISED 2026-08-13 under DS-A24. (c) is sound as filed: "the point values"
means the count schedule, which the guard sends to 0 on H. The sharp form is
**Lemma J(c′)** (errata §8.5(e)) — the value is identical for every tile-value
schedule CONSTANT on H, of which the guarded count schedule is the constant-0
case. An arbitrary per-tile physical valuation is not covered; see Lemma E8 and
the correction of DS-A9's cone clause.]*

*Proof.* (a) By (i) every trick from i on is led by a non-focal seat, so the led
context of every trick is chosen without reference to focal's policy. A seat's
legal set is "follow the led context if able, else anything" (v0.4 §1.2, §1.5) —
a function of that seat's own hand and the led context only; there is no
must-beat rule, so it does not depend on the tiles already on the table, and in
particular not on focal's tile. Hands other than focal's are unaffected by
focal's plays. By induction along the fixed table order, the joint law of the
non-focal plays is one fixed distribution, independent of focal's policy. (b) By
(ii) focal's tile is never the maximum, so the winner is the argmax over the
other three tiles — the maximum is unique because the lead has tier ≥ 1 and
distinct tiles have distinct keys in the led context (S-A2) — hence a function of
the non-focal plays alone. The count-free increment is e⋆ iff the winner sits in
the focal partnership (F5, Lemma E), so the increment sequence is identical. (c)
The points captured in a trick are the sum of the point values of its four tiles;
three are non-focal and identically distributed by (a), and the fourth is one of
focal's tiles, of value 0 by the guard. So each trick's point total and each
trick's winner are policy-independent, and so is the partnership's total. ∎

*Answer to the sub-question J-Q1(b)(iii), which is where this design was most at
risk.* Focal's remaining hand does change which contexts focal can follow — but
that is focal-internal. It touches the other seats through exactly one channel,
the led context, and only the leader sets that. So (iii) **holds under (NI)(i)
and fails without it**: at a node where focal is on lead, its choice of lead
changes the other seats' legal sets (the follow obligation), hence the field's
masses, hence which tiles they still hold in every later trick. **Any detector
that omits the not-on-lead conjunct is unsound**, and no "focal never wins"
hypothesis repairs it.

*Scope of the verdict (mandatory in the results file).* Deadness here is relative
to a field that does not condition on focal's tile identity. Against an opponent
who draws inferences from what focal discards, the choice carries information and
the verdict does not transfer; this is the same typed boundary as v0.4 §7.7 —
theorems for one operator do not transfer to another. Two things the verdict is
**not** relative to, and this is unusual enough in this file to state plainly:
the conditions below are functions of the focal information state (hand plus the
public played record) and quantify over the whole live set, so a verdict is
**independent of the world, of any belief, and of any support**. It therefore
survives the Φ(**C**) ⊊ Φ(**C**₀) gap of P-A1 rather than being fenced by it, and
it is lawful at a pooled node without ever touching a particle.

### Proposition J-0 (D0, exact — and no margin is needed)

*Statement.* At node i let H be focal's remaining hand, T the unresolved-trick
tiles (F2 A1), L the tiles still in hands, κ_δ the trumps. Suppose:

- **(a)** focal is not the leader of the current trick (nor of the next, if a
  trick has just resolved);
- **(b)** H ∩ κ_δ = ∅;
- **(c)** for every t ∈ H, every context q with t ∈ σ̂_q, and every tile
  d ∈ ((L ∖ H) ∪ T) with ℓ(d) = q: d beats t in q. (For a trick already in
  progress, d is the tile actually led.)

Then (NI) holds at i; so by Lemma J the node is decision-dead under the count-free
contract, and with H ∩ COUNT = ∅ also under trick-plus-count.

*Proof.* Take any trick from i on and suppose inductively it is led by a non-focal
seat (true for the current trick by (a)). Let its led tile be d, so q := ℓ(d) is
the led context, d ∈ σ̂_q, and d ∈ (L ∖ H) ∪ T as evaluated at i — every tile
played from i onward was in hands at i, and the current trick's played tiles are
in T. If q is the trump context, focal holds no trump by (b), so focal is void in
q and its play is a tier-0 slough, beaten by d (tier 2). If q is a natural
context and H ∩ σ̂_q ≠ ∅, focal must follow with some t ∈ H ∩ σ̂_q, and d beats t
by (c). If H ∩ σ̂_q = ∅, focal's play follows neither q nor is trump by (b), so it
is tier 0 and d beats it. In every case focal's tile is not the maximum, so focal
does not win the trick and the next leader is again non-focal. The induction
closes, giving (NI)(i) and (ii). ∎

*Why no exhaustion margin exists to get wrong.* The design proposed to certify
that a live beater "cannot be exhausted before t's last possible play," which
would need a counting argument sound against adversarial play orderings. It is
unnecessary: the beater is not some tile that must survive in someone's hand, it
is **the led tile of the very trick focal is playing to**, which by definition is
in the led context and is present by construction. Nothing can exhaust it. The
only quantification left is over *potential* leaders — tiles d with ℓ(d) = q —
and if no such tile exists outside H then q can never be led and the clause is
vacuously true, correctly.

*Cost.* (b) is one bitset AND. (c) is, per context, one comparison of the maximum
of H ∩ σ̂_q against the minimum over potential leaders of q, on precomputed
per-context masks: seven contexts, no allocation, no world, no solve.

### Proposition J-1 (D1-sym — the transposition form, and where the guard earns its keep)

*Statement.* Let H be focal's remaining hand containing t₁ ≠ t₂, and let
τ = (t₁ t₂) act on the live-plus-table structure, fixing every other tile. Call a
context q **still leadable** if some tile outside H with ℓ = q remains in
(L ∖ H) ∪ T, or q is the current led context (F2 A3's live-context discipline —
membership in a context that can never be led again is inert and erasable).
Suppose τ preserves: trump membership; follow membership in every still-leadable
context; the winner-determining order (S-A2) in every still-leadable context;
the double flag as it enters that order; and, unless Proposition J-0 already
shows focal never leads, the led-context map ℓ on t₁ and t₂. Then playing t₁ now
and playing t₂ now have equal value in every world, for every isomorphism-invariant
valuation (E-A3), so the choice between them is dead. If moreover
H ∩ COUNT = ∅, the verdict holds under trick-plus-count as well.

*Proof.* τ fixes every non-focal seat's holdings and preserves every relation in
Lemma E's amended F2 list on the relations any rule can still read, so it is an
isomorphism of the remaining extensive game carrying legal actions to legal
actions, increments to increments and focal to focal (Lemma E). It maps the
policy "play t₁ now, then σ" to "play t₂ now, then τσ", which is again
information-consistent, with equal value. For count: E-A2 bars count-bearing
readouts under structural transports because a general transport does not
preserve the count decoration c (v0.4 §1.4). Here τ moves only t₁ and t₂ and
fixes every other tile, so c ∘ τ = c holds iff c(t₁) = c(t₂) — and the guard
gives c(t₁) = c(t₂) = 0. The decoration is preserved and the verdict lifts. ∎

*Remark (what it costs and what it will not reach).* All clauses are bitset and
per-context-order comparisons on public data. But the conditions are demanding:
two distinct tiles have different pip pairs, so they can agree on follow
membership only when their differing contexts are already inert, and they must be
adjacent in the surviving order. The specimen pair {2-2, 6-3} fails on the double
flag alone. Expect low recall and report it as measured (J-A8).

### Proposition J-win (D1-win — count-free only, and this is where the guard fails)

*Statement.* If, at node i, focal is certain to win every remaining trick under
every legal continuation and every order of its own tiles — a cheap sufficient
form being: every trump outside H is dead; for every still-leadable context q,
every t ∈ H lies in σ̂_q and beats every tile of ((L ∖ H) ∪ T) ∩ σ̂_q; and each
t ∈ H beats every tile of ((L∖H) ∪ T) ∩ σ̂_{ℓ(t)} — then the count-free value from
i is |H| under every policy, so the node is decision-dead under the count-free
contract.

*The guard does not rescue it under count, and the reason is exact.* Focal now
wins tricks, so (NI) fails and Lemma J does not apply. Different orders lead
different contexts, the other seats' follow obligations differ, and therefore
*which* of their tiles — including their count tiles — fall into the tricks focal
wins differs by focal's choice. The guard bounds only focal's own contribution.
Hence: **a D1-win verdict is void the instant count re-enters (E-A2, wholesale,
never extended)**, and a solve that pruned on it may not be quoted for any
count-bearing valuation. Jason's conjecture is therefore true for D0 and D1-sym
and false in general; the honest statement is that the guard rescues exactly the
verdicts whose soundness runs through non-interference or through a
count-preserving transport.

### J-Q1 — definition and lemma. RULING: ACCEPT the definition with the three-way typing mandatory; Lemma J DELIVERED; (b) proved for two members and refuted for the third.

- **J-A1 (definition, and the distinction the motivation blurs).** ACCEPT:
  decision-dead(node) per declared contract and field means every
  information-consistent policy from that node has the identical value function
  on the node's fiber. The results file carries the forced/dead/dominant typing
  above verbatim and states that **S6b's singleton frontiers are dominance, not
  deadness** — the seventh specimen is the proof that the two differ. No sentence
  may present a singleton-frontier count as a deadness count.
- **J-A2 (Lemma J and the not-on-lead conjunct).** Lemma J is delivered above and
  answers J-Q1(a) for every detector that establishes (NI). Sub-question (iii) is
  answered: focal's choices touch the other seats through the led context and
  through nothing else, so the claim holds under (NI)(i) and **fails at any node
  where focal is on lead**. Every accepted detector carries the not-on-lead
  conjunct or derives it (J-A4, J-A6).
- **J-A3 (the count guard, ruled).** The guard is CONFIRMED as binding and is
  proved sufficient for D0 (Lemma J(c)) and for D1-sym (Proposition J-1),
  including the pleasing fact that it is precisely what lifts E-A2 for the
  transposition transport. It is REFUTED as a general principle: Proposition
  J-win exhibits the mechanism by which a count-free tie fails under count while
  the guard holds. Consequently every verdict is tagged with the valuations it
  survives — **D0: count-free and trick-plus-count; D1-sym: count-free and
  trick-plus-count; D1-win: count-free only** — and a harvest run states which
  tags its pruning relied on. Untagged verdicts are not adjudicated verdicts.

### J-Q2 — the detector family. RULING: D0 ACCEPT-WITH-AMENDMENT (exact form, no margin); D1 as posed REJECTED, two exact members accepted in its place; D2 ACCEPT.

- **J-A4 (D0, in Proposition J-0's exact form).** ACCEPT with the statement
  above replacing the design's. No exhaustion margin, no counting condition, no
  constant to freeze. The clause quantifying over *potential leaders* (ℓ(d) = q)
  rather than over all q-members is the sharper and still exactly sound form, and
  the builder implements that one; the current trick uses the tile actually led.
- **J-A5 (D0's recall must be measured node-locally; the design's dismissal is
  struck).** The design's sentence "D0 is sound but misses the measured volume"
  is REJECTED as unsupported: it reasons from the root hand's winning chances to
  a node-local condition. The probe measures D0's recall at the decision nodes,
  against the record, before any ranking of D0 against D1 is stated anywhere.
  If D0 turns out to carry the volume, that is the probe's best possible outcome
  and the design's premise was simply wrong.
- **J-A6 (D1 as posed: REJECTED; D1-sym accepted).** "Order-exchangeability" is
  the conclusion restated, not a checkable condition; naming it a detector would
  license an unproved test. REJECTED under that name. **D1-sym** is accepted in
  Proposition J-1's exact form, with the not-on-lead conjunct or the ℓ-preservation
  clause as stated. The design's proposed decomposition — "the two tiles never
  contest the same winnable trick, plus symmetric loss" — is NOT accepted: the
  first half is not sound on its own (two tiles that never contest the same trick
  can still differ in which trick each wins, changing nothing count-free only by
  accident), and the second half is D0.
- **J-A7 (D1-win accepted, narrow and count-free).** ACCEPT in Proposition
  J-win's form, tagged count-free only, ranked third: it fires only in
  boss-endgame configurations and its clauses are the most restrictive of the
  three.
- **J-A8 (what remains, ranked honestly).** No cheap sufficient structural
  condition is known at adjudication time for the six specimens' ties, and none
  is invented here. Ranked, for the record: (1) D0 — exact, cheapest, count-safe,
  recall unmeasured and possibly large; (2) D1-sym — exact, cheap, count-safe,
  expected recall low; (3) D1-win — exact, cheap, count-free only, expected
  recall very low; (4) the specimens' mechanism — UNIDENTIFIED. A full
  one-deviation evaluation is a solve and is therefore not a detector (S5j's
  lesson, which this design correctly names and must not then violate). If the
  three accepted members leave the specimens uncovered, the run records the
  residual as a named open question with its witnesses, and does not ship a
  fourth detector without a proof of the shape given here.
- **J-A9 (D2, nodewise re-application).** ACCEPT: soundness is node-local, and a
  hit at a node licenses collapsing that node **and its entire subtree**, since
  Propositions J-0/J-1 quantify over all continuations from the node. Two notes
  the builder needs: re-checking below a hit is wasted work, and the syntactic
  conditions may cease to hold at a descendant without invalidating the hit (the
  semantic property was proved for the whole subtree). Forced nodes are excluded
  from the detector's call sites entirely (J-A13).

### J-Q3 — ground truth and recall. RULING: ACCEPT-WITH-AMENDMENT; the denominator is a conflation risk and must be typed.

- **J-A10 (two denominators, never one).** Decision-deadness is "all policies
  tie". A one-deviation classifier certifies something weaker unless it quantifies
  over every policy: single deviations from one reference policy give
  **argmax-indifference**, and the set of argmax-indifferent states is a
  **superset** of the decision-dead states. (The two coincide only if every
  one-deviation from *every* policy ties, since any two policies are connected by
  a finite chain of single deviations.) So: recall is reported against both
  denominators where both are computable — the exact dead set (PG's N_vec = 1 at
  the node, available wherever the unpruned enumeration completes, PG-A7) and the
  one-deviation tie set — each labelled, never summed, never averaged. A recall
  quoted against the larger denominator understates the detector and must say so.
- **J-A11 (the soundness receipt, strengthened).** ACCEPT (b) and strengthen: on
  every fired node, assert the exact dead-set membership (N_vec = 1) where
  computable, and the one-deviation tie otherwise. A single disagreement is
  stop-and-report per NO-RESCUE — Propositions J-0/J-1 are theorems, so a
  disagreement is an implementation defect or an error in them, never a new
  finding, and never patched by weakening the detector.
- **J-A12 (cost).** ACCEPT (c) with the E-A9 discipline named: the detector's
  per-call cost is measured against the cheapest thing it replaces, and the
  forced-node check (|legal| = 1) is present in **both** arms so the detector is
  never credited for it. Integer nanoseconds, exact rationals for ratios, no
  floats (P-A19).

### J-Q4 — the harvest. RULING: ACCEPT the arms with the call-site rule, the charging rule and one mandatory receipt.

- **J-A13 (where the detector may run and how it is charged).** Lawful call
  sites: **focal decision nodes with ≥ 2 legal plays, and nowhere else** —
  never at hidden nodes, never at forced nodes, never as a pre-pass over states
  the walk would not visit. The detector's wall-clock and its verdict-cache
  probes are charged **inside** the harvest arm (P-A8: an arm pays for its own
  machinery); verdict caching is lawful as a derived view keyed by (predicate id,
  freeze-set id) (X-A6(i)) and is recomputed on a declared sample before anything
  is quoted. This is the S5j guard: a detector whose cost is charged outside the
  arm has not been measured.
- **J-A14 (the arms, the control, and the receipt that makes pruning safe).**
  ACCEPT H-plain versus H-with-detector on the S5h n = 4/5 rungs and the S6b
  coordinates, same solver, same budget unit, same coordinates, one machine, one
  build, one declared thread count (P-A7, P-A19); the S5h cold-H baseline is the
  right control because it is the same operator, field and valuation (freeze 26).
  Mandatory: **every harvest run asserts bit-exact equality of V and every root Q
  against the plain arm on every coordinate** (P-A9). Mandatory: **the detector
  prunes focal branching only** — deleting or reweighting any hidden branch
  changes the field and therefore the operator (X-A7, Y3(d), F4) and is
  forbidden outright. A budget wall is a declared stop printed with what was
  reached (P-A16, E-A16), never a finding.

### J-Q5 — coverage census. RULING: ACCEPT-WITH-AMENDMENT.

- **J-A15.** ACCEPT the two decimated counts under P-A15 (declared multiplicative
  decimation, S6a freezes, no prefix, no RNG), with three additions. (i) **Forced
  nodes are a separate column, never inside the dead fraction** — they are free
  to detect and worth nothing, and folding them in inflates the harvest for free.
  (ii) A node fraction is **traversal-relative and memoization-relative**: which
  nodes exist to be counted depends on the declared walk and its caching (E-A20's
  order-relativity, restated), so the census is an inventory statistic of this
  traversal and not a property of the game. (iii) A node fraction is **not a cost
  fraction** (P-A11): the harvest ratio of J-A14 is the cost statement, the census
  is the inventory statement, and they never share a line. The hypothesis that
  deadness grows mid-playout as winners leave hands is exactly what (ii) measures;
  it is reported as measured on this traversal, both directions being results.

### J-Q6 — scope, freezes, results discipline. RULING: ACCEPT-WITH-AMENDMENT.

- **J-A16 (freezes, continuing at 32).** Freezes 1–31 are in force and restated.
  **(32)** the detector predicates — D0 in Proposition J-0's form including the
  potential-leader quantifier and the still-leadable-context definition, D1-sym in
  Proposition J-1's form, D1-win in Proposition J-win's form — and their bitset
  encodings; note explicitly that **no exhaustion-margin constant exists to
  freeze** (J-A4). **(33)** the detector call sites and the charging rule
  (J-A13). **(34)** the ground-truth classifier: which denominator, computed how,
  per J-A10. **(35)** the harvest arms, rungs, coordinates, budget unit and the
  control's solver identification.
- **J-A17 (the fence, verbatim, extending R-A23/PG-A17).** "THE FENCE. A deadness
  verdict says: from this node, under the declared field, every
  information-consistent focal policy has the identical value. It is not a
  similarity claim and not a tolerance claim — nothing here supports 'about the
  same' for any tolerance, and δ-similarity remains future mathematics with its
  own rulings pending. It is not a partition: the dead/live split is a
  response-equality object and v0.4 §12.4 bars using it as a solver's state
  partition. UNKNOWN is never evidence of liveness — the detectors are one-sided
  by construction and their misses are lawful. Each verdict carries the valuations
  it survives (J-A3); a D1-win verdict is void wholesale the instant count
  re-enters (E-A2). Deadness is relative to a field that does not condition on
  focal's tile identity: against an opponent who reads discards, the choice
  signals and the verdict does not transfer (§7.7). It is **not** relative to any
  world, belief or support — the conditions are functions of the focal information
  state and quantify over the whole live set — so it is one of the few objects in
  this file that crosses the Φ(**C**) ⊊ Φ(**C**₀) gap intact. No runtime claim
  follows from a coverage fraction; the harvest ratio is the only cost statement
  and it is coordinate- and traversal-relative."
- **J-A18 (both outcomes).** Confirmed (F7, NO-RESCUE). A large D0 recall is the
  best outcome and would retire the design's premise; a small recall for all three
  members, with the specimens uncovered, is a proved statement about what cheap
  structural detection can reach and is reported as such — not as a reason to ship
  an unproved fourth detector. A harvest dividend that the detector's own cost
  eats is S5j measured again, and is equally a result.

**Both outcomes remain results.** Propositions J-0, J-1 and J-win stand
regardless of every number this probe produces: they replace an unstated
exhaustion margin with an exact three-test condition, they identify the one
transport form under which the count guard lifts E-A2, and they exhibit the
mechanism by which the guard fails when focal wins tricks — which is the boundary
Jason's binding constraint was reaching for and now has a proof on both sides of.

## Decision-sparse intake audit (2026-08-13)

**Adjudicator:** walt-math. **Object:** `walt/math/decision_sparse_exact_solving_v0.1.md`
(filed 2026-08-13, commit 8ee1c9e). **Tier:** exploratory; this is an intake
audit in the style of the v0.6 audit (R-A1's shape). Nothing in the document is
promoted by being audited, and no theorem below may be cited above exploratory
tier. **Basis:** the document itself; v0.4, v0.5, v0.6; F1–F7, r3 Q1–Q5, Y1–Y3,
P-A1..P-A21, X-A1..X-A19, E-A1..E-A21, S-A1..S-A21, R-A1..R-A24, PG-A1..PG-A18,
J-A1..J-A18 and Lemmas V, X, E, S, S-fold, S-det, R, G, J, Propositions G-flat,
J-0, J-1, J-win, Corollaries S-rigid, R-fold. Amendments are numbered DS-A1..
and bind any design that consumes this document.

**Headline, stated first — four findings decide how this document may be used.**

1. **§7.1 (commuting-kernel exchangeability) is UNSOUND as written.** Under a
   literal reading its hypothesis is unsatisfiable in this game — the public
   trace records which tile focal played first, so the two orders never produce
   equal traces — and under the intended transport reading its proof step
   "compose both sides with the same lawful successor policy" is invalid, because
   under perfect recall the continuation policy is a function of the trace and
   the two traces are related by the transport, not equal. It is repairable
   exactly, and the repair is Proposition J-1's shape (DS-A6). This matters
   because the document proposes promoting D1 to a general layer on the strength
   of this theorem.
2. **§10.3 (finite Scheme-mass closure) is SOUND but degenerate in this game.**
   Its hypotheses require, at every reachable interface, that the preexpectation
   of every successor atom indicator be measurable on the current atoms, seeded
   by 1 at terminals. That is Lemma R(c)'s closure in Boolean-algebra clothing:
   iterating those residuals produces the complete-record path probabilities,
   which separate worlds, so the coarsest family satisfying the hypotheses is the
   discrete one and dim SF = |X| at every interface. The atom formulation
   therefore buys nothing here, and §10.4's shared-circuit route is the only live
   one (DS-A5). The document's filtering claim is precisely the degenerate part;
   its fixed-policy evaluation claim is the salvageable part.
3. **The treatment-C relaxation is a lawful upper witness — my R-A10 never barred
   that — but it must be ACTION-CONDITIONED, and the quantity the branch has
   already measured is not.** The P-A6 aggregate (and the `fusion_gap` column of
   the S6a receipts) maximises over root actions inside the world, giving
   E_β[max_a V*_a]; the certificate of §8.4 needs max over policies of the
   action-fixed world-informed value, E_β[V*_a], per action. Reusing the existing
   aggregate is not unsound — it is still an upper bound — but it is the same
   number for every action and makes the certificate vacuous (DS-A7).
4. **The candidate-policy library that Jason wants to extend has exactly one
   fatal naive failure mode, and it is a soundness failure, not an efficiency
   one.** A lower witness must be the exact value of a *fixed* lawful policy
   integrated under the declared belief. If a library candidate is priced with
   any evaluator that maximises inside the world — the P-A6 aggregate, `revealed`,
   a per-world best response — the result is an UPPER bound presented as L, the
   sandwich inverts, and §8.4 will certify the wrong action with a proof that
   looks valid. Everything else about the extension is lawful and one part of it
   is unusually robust (DS-A14..DS-A16).

### Proof audit of the eleven theorems of §16.2

Verdicts are per theorem; "obligation" marks a hypothesis the document leaves
implicit that becomes a builder obligation.

- **T1 §4.2 envelope sufficiency — SOUND, and definitional.** The proof is the
  definition restated, which is honest but means the theorem carries no content
  beyond fixing the object. Two things it does fix and one bridge it should
  state: the identification Q^H_β(a) = max_ρ ⟨β,α_ρ⟩ is Lemma R(b) plus PG-A2's
  linearity and is already adjudicated; and **at the full simplex the minimal
  envelope is exactly my Exp set, so W_all(B,a) = N_exp(a) of PG-A4** — the same
  number under two names, which E-A8's rule forbids reporting as two
  measurements (DS-A4). *Obligation:* on a proper belief subfamily the minimising
  envelope need not be unique (Lemma G(4)'s uniqueness argument uses the full
  simplex), so W_reach is a well-defined number but "the" reachable envelope is
  not a well-defined set; nothing may be keyed to it.
- **T2 §4.3 width monotonicity — SOUND**, trivially. *Obligation (typing, and it
  is not cosmetic):* the reachable-belief family is defined as posteriors, which
  live on successor interfaces with smaller fibers, while the α-vectors live on
  X_B. The comparison is well-typed only in the deal-level view where a world is
  a full deal that never changes and observation refines its belief. Our
  interface machinery (R-A4) uses shrinking fibers, so any run computing W_reach
  must declare the identification between the two views and hold to it.
- **T3 §5 advantage invariance — SOUND.** The common-translation argument is
  correct and the document's own root-comparison caution is the right one.
  *Gap (definitional, must be repaired):* d_adv(E) = dim span{α − α_0 : α ∈ E}
  depends on whether α_0 ∈ E — with an external reference the number can be one
  larger, and the claim "a singleton envelope has d_adv = 0" holds only when α_0
  is that singleton. **AMEND: define d_adv(E) as the affine dimension,
  dim span{α − α′ : α, α′ ∈ E}**, which is reference-free and makes the singleton
  statement unconditional (DS-A4).
- **T4 §6.2 pointwise dominance — SOUND**, and its tie caveat ("may still tie at
  beliefs assigning zero mass to every strict coordinate") independently agrees
  with PG-A4. Nothing to add.
- **T5 §6.3 dominance under positive composition — SOUND, but incomplete for the
  use it is put to.** It proves monotonicity, which is Lemma G(1). Backward
  pruning additionally needs that the pruned composition reproduces the frontier
  *exactly, as a set, ties included* (Lemma G(2)) and that the fold may be done
  incrementally (Lemma G(3)). The document asserts the consequence — "this is the
  mathematical basis for exact backward Pareto pruning" — without those two
  steps. They are already binding and supply what is missing; a design consuming
  §6.3 cites Lemma G, not §6.3 alone.
- **T6 §7.1 commuting-kernel exchangeability — UNSOUND as written; repairable.**
  See DS-A6 for the repair and the conditions.
- **T7 §8.3 value sandwich — SOUND.** *Obligations:* (i) every candidate must be
  information-consistent at the node, not merely "a policy"; (ii) the relaxation
  must contain the exact class **with the same α-map** — same field, same
  mechanics, same utility, same belief — so that the three maxima are of one
  objective; (iii) L and U must be computed at the same β. Add one that the
  document does not state and that our own probe discipline makes live:
  **neither L nor U may be computed on a decimated world set.** Decimation
  (P-A15) is lawful for choosing which coordinates to certify and is not lawful
  anywhere inside an L or a U — a sampled mean is neither a lower nor an upper
  bound, and a certificate built on one is void.
- **T8 §8.4 root-action certificate — SOUND.** *Obligation:* it certifies a
  member of the optimal set, not the set; where ties matter the certificate must
  be run per action or the argmax reported as "contains a⋆".
- **T9 §9.2 finite adaptive gluing — SOUND, with the termination bound honestly
  worthless and two implicit obligations.** Items 1–4 are correct set-inclusion
  arguments. Item 5's finiteness is real but the bound is |R_0|, doubly
  exponential; it must never be read as a complexity claim. *Obligations:*
  (i) each iteration's relaxed solve must be exact, or at minimum must return a
  proved upper bound on max over R_k — anything weaker breaks U's validity, and
  this must be proved per solver, not assumed; (ii) the stopping criterion of
  §9.3 (Opt(R_k,β) ∩ R_H ≠ ∅) is a search, not a lookup: a negative answer
  requires a proof over the whole optimal face, whereas "the returned optimiser
  is unlawful" licenses only adding a cut. A run that treats one unlawful
  optimiser as a proof that the face contains no lawful policy has skipped the
  theorem's actual hypothesis.
- **T10 §10.3 finite Scheme-mass closure — SOUND as an implication, DEGENERATE
  under its own hypotheses in this game.** See DS-A5.
- **T11 §12.1 combined factorization — SOUND**, being §§8–9 assembled, and the
  document correctly disclaims any smallness. *Obligation:* its condition 3 ("the
  Scheme circuit evaluator returns exact expectations under β") is the entire
  unproved engineering core; every reported L and U carries the receipt that its
  evaluator agreed exactly with an enumerated authority on the declared small
  carrier, or it is not a witness.

### DS-A1..DS-A3 — vocabulary

- **DS-A1 ("certificate": no third term is needed; "witness" is binding).** The
  document's objects are optimality witnesses, not verification artifacts, and
  the distinction is real — but walt already has the word. PG-A10 fixed
  **"witness"** for a mathematical object exhibited to prove a claim (there: the
  belief β exhibiting exposure, the convex weights λ exhibiting non-exposure).
  Binding: **"receipt"** = a machine-checked verification artifact regenerated by
  a run; **"witness"** = a mathematical object exhibited to prove a claim; the
  word "certificate" does not appear in walt artifacts at all (D3), and quotations
  of this document that use it are bracketed as quotations. The §8.1 candidate is
  a **primal witness**, the §8.2 relaxation is an **upper witness**, and the §8.4
  object is the **root-action separation** (L_{a⋆} ≥ U_a for all a ≠ a⋆). The
  reachability fence of R-A2 is restated wherever a witness is reported: no object
  here is identity-bearing and none asserts that any state arises in play.
- **DS-A2 ("decision width" W).** ACCEPT the term, with the bridge mandatory:
  **W_all = N_exp** (PG-A4) — identical objects, so a results file reports one
  number under one name with the other named as a synonym, never two rows. W_reach
  is a genuinely new and smaller quantity; it is reported only with its declared
  belief family, its deal-level typing (T2), and the note that its minimising set
  is not unique. The ladder that must appear whenever more than one is quoted:
  forced ⊂ dead (N_vec = 1) ⊂ dominant (N_par = 1), and N_pol ≥ N_vec ≥ N_par ≥
  W_all ≥ W_reach ≥ 1 (PG-A7, J-A1).
- **DS-A3 ("advantage dimension", "gluing cut").** d_adv: ACCEPT under the affine
  repair of T3. "Gluing cut": ACCEPT the term with one typing clause that must
  never be relaxed — **a cut constrains the RELAXATION, never the lawful policy
  class and never the fiber.** A "cut" applied to worlds is a declared exclusion
  remnant and falls under X-A1's typing wholesale, with none of §9's guarantees.
  Validity ("satisfied by every H-policy") is proved per cut, per §9.5's three
  conditions; asserted validity is not validity.

### DS-A4..DS-A9 — interaction with standing rulings

- **DS-A4 (cardinalities; answers the E-A8 exposure).** The document introduces
  W_all, W_reach and d_adv alongside the existing N_pol/N_vec/N_par/N_exp. Ruling:
  W_all is N_exp renamed (DS-A2); d_adv is repaired to the affine dimension (T3);
  and any design quoting more than one of the seven states the ladder and the
  pass that produced each (PG-A7). A run that reports W_all and N_exp as separate
  measurements has measured one thing twice.
- **DS-A5 (§10.3 is Lemma R(c) again; the circuit route is the live one).** The
  hypotheses of §10.3, required at every reachable interface, force the discrete
  algebra: seeded by 1 at terminals and closed under preexpectations of successor
  atom indicators, the family contains ξ ↦ Pr_ξ(complete record), and a complete
  record names every hidden tile with its seat, so those functions are nonzero
  multiples of singleton indicators (Lemma R(c)). Since the span of atom
  indicators has dimension equal to the number of atoms, the atoms are singletons.
  Consequences, all binding: (i) **no compression claim may be read from §10.3**,
  and a design that reports "the Scheme algebra closed at N atoms" with N < |X|
  has either violated hypothesis 3 or is measuring a purpose-relative object that
  must be declared as such; (ii) the salvageable formulation is the one already
  ruled in R-A16 — seed with the contract's readouts, not with the constant, and
  close under observation-aggregated residuals; (iii) exact *normalised* filtering
  is what forces the constant back in, so filtering and compression remain
  incompatible under this observation contract, exactly as Lemma R's consequences
  state; (iv) §10.4's shared arithmetic/Boolean DAG and §10.5's weighted model
  counting are untouched by this and are the real proposals. State the reframe
  precisely when it is used: the escape is not that circuits beat rank — rank
  remains a lower bound on linear factorisations — it is that the target changed
  from representing every value function to evaluating finitely many inner
  products. That is a legitimate change of target and it is the document's best
  idea.
- **DS-A6 (§7.1 repaired; what promoting D1 requires — answers 3(c)).** The
  theorem must be restated as: *let Θ be a declared involution of the remaining
  game exchanging the two focal action blocks and fixing every non-focal
  holding; suppose the kernels intertwine, 𝖪_b ⋆ 𝖪_a = Θ_*(𝖪_a ⋆ 𝖪_b), on
  (public trace, accumulated outcome, successor interface); suppose the lawful
  policy class is Θ-closed. Then the two orders achieve the same SET of terminal
  laws, hence the same optimal continuation value.* Three clauses are
  load-bearing and none may be dropped. (i) The conclusion is about the achievable
  set and the optimum, **not** about a fixed continuation policy: under perfect
  recall a policy is a function of the trace, the traces differ by Θ, and "the
  same policy" is ill-typed across the two orders. (Equality for each fixed
  continuation is recoverable only by restricting to interface-measurable
  policies, which is a different and smaller class and must be declared if used.)
  (ii) The outcome monoid must be commutative for the accumulated outcome to be
  order-insensitive; ℕ·e⋆ is, and the document's own count boundary is the right
  fence for richer ones. (iii) The intertwining is checked against the
  *still-leadable* structure only (F2 A3, Proposition J-1) — checking it against
  inert contexts costs recall, not soundness. With those, §7.1 is exactly
  Proposition J-1 stated kernel-wise, and the promotion of D1 to a
  trace-equivalence layer is lawful; without them it is not a theorem. Note also
  that Proposition J-1 already supplies what §7.1's count boundary asks for: the
  transposition preserves the count decoration precisely when both swapped tiles
  are pointless, which is Jason's guard.
- **DS-A7 (treatment C as an upper witness — answers 3(a)).** C ≥ H is SOUND and
  its promotion to a load-bearing upper bound is lawful: the inequality is the
  pointwise fusion inequality of Lemma X's proof (α_ρ(ξ) ≤ V*_a(ξ) for every
  information-consistent ρ with root action a), averaged under β. R-A10 barred
  the P-A6 aggregate from being an *equality gate* against H; it never barred the
  inequality, which is exactly the one-sided use. Four conditions attach.
  (i) **Action-conditioned.** U_a := E_β[V*_a], the world-informed value with the
  root action held at a. The existing aggregate computes E_β[max_a V*_a], which
  is ≥ U_a for every a: still a valid upper bound, hence not unsound, but
  identical across actions and therefore useless for §8.4. The S6a `fusion_gap`
  column is that root-level object and is **not** a per-action price.
  (ii) Same field, same belief, same valuation, same world set as L (T7).
  (iii) The document's C fixes the root action and reveals the world for later
  focal decisions; the implementation's world-informed evaluator maximises at
  every focal node including the root, so the action-conditioned variant is a
  small change to an existing path (`walt-strat/src/revealed.rs`,
  `fiber_probe.rs:215`) and must be built, not assumed. *[CORRECTED 2026-08-13 by
  SEP-A7: the sentence naming `walt-strat/src/revealed.rs` as needing the change
  is WRONG. `revealed::revealed_summary().q_c[a]` has been the
  action-conditioned U_a = E_β[V*_a] since S3; the root-maximising objects are
  `revealed_summary().v_f` (= U^agg = V^F, same file), `fiber_probe.rs::aggregate`
  and `predictive_rank.rs::fused` (which produced the S6a `fusion_gap` column).
  What remained to build was the harness and the receipts, not the evaluator. The
  rest of clause (iii), and the whole of DS-A7, stands; the durable statement is
  SEP-A7 with freeze 37 at SEP-A6.]* (iv) PI minimax is a
  different operator and is not a valid upper bound for the fixed stochastic
  field — the document says this and it is correct (§8.2, §15.4). One positive
  worth recording: at several S6a coordinates the measured fusion gap is exactly
  0, so at those coordinates the C-relaxation is already tight and the sandwich
  would close on the first iteration; that is real prior evidence for Experiment
  E and it costs nothing to reuse.
- **DS-A8 (deadness vs dominance — answers 3(b)).** §6.4's two definitions are
  correct and match J-A1's typing exactly; the document's "the completed S6b
  cases certify dominance, the S6c inspection suggests deadness" is the right
  word in both places, and is consistent with J-A10: the one-deviation classifier
  decides argmax-indifference, a superset of deadness, so it *suggests* and does
  not certify. Binding on Experiment A's report: each of its bullet counts is
  tagged with the denominator that produced it (J-A10), the "universally dominant
  but non-dead" line requires both classifications on the same node set, and the
  forced column stays separate (J-A15).
- **DS-A9 (cone dominance vs the count guard — answers 3(d); and the firewall —
  3(e)).** §11.3 does **not** subsume Jason's guard, and the relation is worth
  stating because it strengthens the guard rather than replacing it. Cone
  dominance is a criterion on already-computed feature laws μ_ρ: it is exact and
  general and it pays for the feature law. The guard is a cheap sufficient
  condition for the feature *difference to vanish identically* — under
  Proposition J-0 or J-1 with H ∩ COUNT = ∅ we have μ_{ρ}(ξ) = μ_{ρ′}(ξ) for all
  ξ and all lawful ρ, ρ′ — which is the degenerate case of §11.3 and holds for
  **every** cone at once, with no feature law computed. *[CORRECTED 2026-08-13 by
  DS-A24: the clause from "we have μ_ρ(ξ) = μ_{ρ′}(ξ)" through "every cone at
  once" is WRONG. The per-tile capture coordinates do not agree — focal's own
  tiles land in different tricks under different policies — so the equality holds
  only for valuation directions constant on the exchanged tiles. Lemma E8 (errata
  §8.5) is the corrected statement; everything else in this ruling, and all of
  Lemma J and Propositions J-0/J-1/J-win, stands.]* So: the guard stays a
  separate conjunct, it is the only form of §11.3 available without paying the
  feature cost, and Proposition J-win marks its exact limit (when focal wins
  tricks the difference does not vanish and no cone conclusion is available
  cheaply). On 3(e): §10.6's firewall is the same principle as v0.6 §3.3/§18.6,
  R-A14 and v0.4 §6.8/§15.2, with one permission stated more sharply than we had
  it — **a hidden-world formula may EVALUATE a lawful policy, it may only not
  SELECT one** — which is correct and useful. Two obligations: the observability
  judgment ⊢ F : observable is proved against the information partition
  `walt-strat/src/info.rs` implements (R-A11), never against an informal one; and
  a guard reading the *support* of the current fiber is observable and therefore
  lawful, since the support is a function of the public record — worth stating
  because it is the useful case and an over-cautious reading would forbid it.

### DS-A10..DS-A13 — which of Experiments B–H are lawfully designable now

- **DS-A10 (designable now, under existing rulings plus this section).**
  **B (tense-root anatomy)** — with two clauses: PG-A13 stands, so a frontier may
  be quoted only at a node where the frontier computation *completed*, and the
  "first split" must be identified by a complete computation at that node, not
  inferred from where the capped run stopped; and the synthesised Scheme
  separators are INSTRUMENTS in shape-v2's sense, below every tier, cited by
  nothing, with failed separators recorded as counterexample pairs (the document
  already asks for this). **E (root-action certification)** — with DS-A7's
  action-conditioning, T7's no-decimation clause, and one receipt: wherever H
  completes at the same coordinate, assert L ≤ Q^H ≤ U exactly and assert the
  certified action lies in H's argmax (R-A18's discipline extended to the
  witness). *[SUPPLEMENTED 2026-08-13 by SEP-A12: the clause stands, but as
  written the L ≤ Q^H half is satisfied as equality by construction whenever the
  candidate is H-argmax-seeded (Corollary E4.1(2)) and the U ≥ Q^H half is already
  asserted inside `price.rs::information_prices`, so neither is a receipt in
  PG-A8's sense. The five receipts that carry content for Experiment E are fixed
  at SEP-A12 (R1)–(R5).]* **F (reachable envelope)** — with DS-A2's ladder and DS-A4; W_reach
  requires exact enumeration of the reachable posteriors, feasible only at the
  small grades, and a stop is a stop (PG-A13).
- **DS-A11 (designable after the object is fixed).** **C (Scheme-weighted
  filtering)** must not be designed against §10.3's atom formulation (DS-A5). Its
  lawful form measures **circuit size against world count** for a *declared
  purpose* — one likelihood, one posterior moment family, one fixed lawful
  policy's value — with bit-exact agreement against enumeration, and states in
  its header that no algebra-compression claim is made and that exact normalised
  filtering re-enters Lemma R(c). Stated that way it is the branch's first real
  attempt at Gate D (moment compilation), which R-A23 named as separate and
  unmeasured; it should say so. **D (adaptive gluing)** is designable given T9's
  two obligations, DS-A3's cut typing and DS-A7's action-conditioning.
- **DS-A12 (needs its own adjudication first).** **G (count and score lift)** is
  where E-A2 bites hardest and it touches Jason's binding constraint, the
  valuation gauge (§11.1), capture completeness (§11.2 step 5) and the stabilizer
  question. It is NOT lawfully designable from this document alone: it needs a
  design and its own rulings, minimally covering the declared cone, the feature
  law's construction, which count-free verdicts are re-derived versus inherited
  (Propositions J-0/J-1 survive; J-win does not), and what happens to every
  form-keyed record (E-A2: void wholesale, never extended). **H (grade climb)** is
  premature by its own terms and, when it comes, is governed by P-A21: three
  rungs are not a law and no growth rate measured at grades ≤ 4 is quoted for the
  opening.
- **DS-A13 (freezes the next design will need, continuing at 36).** Freezes 1–35
  are in force and restated. **(36)** the candidate-policy library: entry format,
  the canonical form each entry is keyed by, and the declared transport used to
  move an entry between coordinates. **(37)** the relaxation family: the
  action-conditioned C evaluator and its solver identification (DS-A7).
  **(38)** the gluing-cut language, the validity-proof obligation, and the cut
  ordering (the sequence of relaxations is a determinism freeze because the stop
  point depends on it). **(39)** the circuit representation and its evaluation
  order, since exact rational arithmetic is order-insensitive in value but the
  reported node and operation counts are not. **(40)** the reachable-belief family
  defining W_reach, with its deal-level typing (T2).

### DS-A14..DS-A16 — the candidate-policy library (Jason's extension), and how it goes wrong

- **DS-A14 (the fatal naive failure, stated first because it is a soundness
  failure).** A primal witness is the exact value of a **fixed** lawful policy,
  integrated under the declared belief and field: no maximisation anywhere inside
  the world. If a candidate is priced by any world-informed evaluator — the P-A6
  aggregate, the revealed/C path, a per-world best response — the number returned
  is an **upper** bound on that policy's value, and installing it as L inverts the
  sandwich. §8.4 will then certify an action with a proof that is formally
  well-formed and wrong. The document flags the shape of this in §15.3; this
  ruling makes it a hard obligation: **every L is produced by a fixed-policy
  evaluator that contains no max node below the root, and the run asserts that
  property structurally (not by inspection), plus L ≤ Q^H against H wherever H
  completes.**
- **DS-A15 (what may and may not be carried across coordinates).** A library
  entry is a policy, and a policy is a function of information states, so moving
  one between coordinates requires a declared transport (Lemma E, Lemma S,
  Corollary R-fold) and a canonical key; without it the entry is not even
  well-defined at the new coordinate. **Dominance never travels.** "This policy
  was dominant at coordinate X" has no status at coordinate Y — the library
  carries policies, not verdicts, and an entry's contribution is exactly its
  lawfulness (which makes L valid) and its measured quality at the coordinate
  where it was measured. A run that reported an action as certified because a
  seeded policy had been dominant elsewhere would have asserted a transported
  optimality claim that no theorem in this file supports. Seeds are heuristics
  for *finding* witnesses; witnesses are validated by exact evaluation, always.
- **DS-A16 (one genuinely robust property, worth banking).** Lower-bound validity
  depends only on a policy's lawfulness, not on the valuation under which it was
  discovered. So the library survives count re-entry **as a witness source**:
  every entry remains a valid primal witness under trick-plus-count, evaluated
  under that valuation, even though its count-free dominance or deadness status
  does not survive (E-A2, Proposition J-win). This is the one place in the branch
  where count re-entry does not destroy prior work, and it is a reason to build
  the library in the count-free layer now rather than waiting. It is also the
  precise sense in which Jason's intuition that the playbook "extends if we do it
  carefully" is correct: the policies extend, the verdicts do not.

**Summary of the audit.** Nine of the eleven theorems are sound as stated, with
the implicit hypotheses above becoming builder obligations; §6.3 is sound but
incomplete for the pruning it is invoked to justify, and Lemma G supplies the
rest; §7.1 is unsound as written and repairable exactly as Proposition J-1;
§10.3 is sound but degenerate in this game by Lemma R(c), which moves the
document's weight onto §10.4's circuits — where, in my judgement, it belongs.
Nothing here is promoted, and both outcomes of every experiment above remain
results (F7, NO-RESCUE).

### DS-A17..DS-A18 — the errata filing (2026-08-13)

- **DS-A17 (the durable home, and the citation rule).** The repairs of
  DS-A1..DS-A16 are filed as first-class mathematics in
  **`walt/math/decision_sparse_exact_solving_v0.1_errata.md`**, with full
  statements and proofs: **Theorem E1** with Corollaries E1.1–E1.2 (the §7.1
  repair — declared involution Θ, intertwining
  𝖪_b ⋆ 𝖪_a = Θ_*(𝖪_a ⋆ 𝖪_b), Θ-closed policy class, conclusion on the
  achievable law-set and the optimum, with the unsoundness argument for the
  original and the commutativity condition located correctly as a *necessary*
  condition for the hypothesis rather than an extra hypothesis of the proof);
  **Definition E2** and **Proposition E2.1** (the affine repair of d_adv, with
  the exact off-by-one); **Lemma E3** and **Remark E3.1** (the
  action-conditioned upper witness and the action-constant aggregate);
  **Lemma E4** and **Non-theorem E4′** (the primal witness and the inversion,
  with a two-world witness showing the certified action can be strictly worse
  than the rejected one); **Proposition E5** with Corollaries E5.1–E5.2 (the
  §10.3 degeneracy and the reframe); and **Theorems E6.1–E6.5** (the parent's
  §§4.3, 6.3, 8.3, 8.4, 9.2 restated with their load-bearing hypotheses inside
  the statements). Citation rule, binding on every design and results file: cite
  the **errata theorem number** for the mathematics and the **DS-A ruling** for
  its provenance; where parent and errata differ, the errata governs. This
  rulings file remains the adjudication record; it is no longer the home of the
  repaired mathematics.
- **DS-A18 (the parent stays verbatim).** No correction is ever written into
  `decision_sparse_exact_solving_v0.1.md`; the handoff is a received document and
  is preserved as filed, exactly as ingest packages are (the reason is the same:
  a corrected source destroys the record of what was corrected). If a v0.2 is
  ever filed, the errata is **re-audited against it and never silently
  inherited** — a repair that a revision has absorbed must be checked to have
  been absorbed, and a repair the revision contradicts is a new finding.

## Second-audit adjudication (2026-08-13)

**Adjudicator:** walt-math. **Object:**
`walt/math/decision_sparse_second_audit_v0.1.md` (filed 2026-08-13, commit
314ea65), §2's nine proposed amendments. Its §1 validates the repairs of the
errata and its §§3–8 are its own synthesis; neither is adjudicated here beyond
the checks noted. **Tier:** exploratory throughout. Amendments continue at
DS-A19; accepted mathematics is filed in
`walt/math/decision_sparse_exact_solving_v0.1_errata.md` §8 under DS-A17's
citation rule.

**Headline.** All nine are substantively correct and all nine are ACCEPTED, four
of them in a binding form sharper than proposed. Two bear on rulings of mine:
**§2.6 is a correct challenge and DS-A9's cone clause is wrong** — the
correction is filed as Lemma E8 with a sharper condition than the second audit
proposed, and Lemma J is unaffected (its clause (c) is sound as filed and is
generalised, not repaired, by Lemma J(c′)); and **§2.5 revises my DS-A10**,
though the demotion it asks for is broader than the defect warrants (DS-A23).

- **DS-A19 (§2.1, transported successor states). ACCEPT, with two hypotheses
  added.** The generalisation is lawful and is filed as **Theorem E1′** (errata
  §8.1), with Theorem E1 as its Θ_X = Θ_M = id specialisation. Two hypotheses
  the proposal does not name are load-bearing and are in the filed statement:
  **(H4′) continuation equivariance** — transporting successor states without
  transporting the process that runs from them is not enough, and the composed
  law then fails to correspond; and **(H5′) utility invariance under Θ_M**. The
  second is worth noticing: (H5′) is the *same condition* as the valuation
  restriction of DS-A24, seen from the kernel side, so the generalised theorem
  and the cone correction are one fact in two coordinates. **On the
  characterisation of my (D1): AMEND.** The second audit reads it as "an
  underspecification warning, not the decisive defect". Both defects stand as
  filed. (D1) is not a stylistic complaint: under the literal reading the two
  kernels have disjoint support in the trace coordinate, so the hypothesis holds
  for no pair of distinct blocks and the statement is *vacuous* — a proved
  property of the text, and the project's ambiguity protocol is to record the
  conflict rather than adopt the plausible reading. That said, the second audit
  is right that (D2) alone forces the repair, and the repair is agreed; the
  disagreement is about labelling a defect, not about any mathematics.
- **DS-A20 (§2.2, latent separation). ACCEPT.** Hypothesis **(S)** is added to
  Proposition E5's statement, and **Lemma E5.0** is filed showing (S) holds on
  the measured carrier — latent = the hidden current remainder, memoryless
  uniform-legal field — by the naming argument. So E5's conclusion stands
  unconditionally for S6a/S6b/S6c; (S) is future-proofing against an augmented
  latent ξ = (ω,z), where the argument would yield indicators of
  complete-record classes rather than singletons, which is a weaker and possibly
  compressive conclusion and would then be worth measuring. The naming point is
  ACCEPTED with it: **treatment C reveals ω; revealing (ω,z) is C⁺**, still a
  valid upper witness by the same proof, never called C unqualified.
- **DS-A21 (§2.3, narrow the filtering conclusion). ACCEPT.** My wording
  over-reached in two places and both are narrowed in place: the binding negative
  is "**atom-mass linear filtering is noncompressive on this carrier**", not
  "filtering and compression are incompatible"; and predictive rank lower-bounds
  the corresponding **linear factorisation target**, not unrestricted nonlinear
  circuit size. Nothing here asserts that no lower-bound technique applies to
  circuits — only that no result in this branch supplies one.
- **DS-A22 (§2.4, zero-global-gap corollary). ACCEPT, and it is stronger than
  stated.** Filed as **Corollary E3.2** with its proof, the three non-implications
  the second audit lists, and one consequence it does not draw: since
  U_a ≤ U^agg = V^H for *every* action, a primal witness attaining V^H separates
  non-strictly from every competitor with **no gluing iteration at all**. At a
  zero-fusion-gap coordinate the whole remaining difficulty is primal. The
  separation is non-strict, so it certifies membership in the optimal set and
  never uniqueness (E6.4's caveat).
- **DS-A23 (§2.5, retype reachable width). ACCEPT the retyping; AMEND the
  consequence for Experiment F.** **Definition E9** files the interface-local
  W^loc_reach(I,a) as the primary object, with the three-way naming discipline
  (interface-local width; a global summary such as the max over interfaces; the
  size of a transported library — three quantities, three names) and the
  root-level W_reach(B,a) retained as a fourth, well typed in the deal-level view
  and answering a question no seat asks. **The demotion is narrower than
  proposed:** Experiment F's other rows — N_pol, N_par, W_all = N_exp, d_adv —
  were designable under DS-A10 and remain so; only the W_reach row moves to
  pending-until-retyped, and it is now retyped, so what actually remains before
  it can run is an enumeration of B_reach(I) on a carrier small enough to
  enumerate exactly. DS-A10 is revised to that extent and otherwise stands.
- **DS-A24 (§2.6, the cone claim). ACCEPT the challenge: DS-A9 is WRONG in the
  named clause, and the correction is filed.** DS-A9 said the feature difference
  vanishes identically under J-0/J-1 with the guard and therefore holds "for
  every cone at once". It does not: focal's own tiles land in different tricks
  under different policies, so the per-tile capture coordinates differ even when
  every count-bearing quantity agrees. The correction is filed as **Lemma E8**
  (errata §8.5) and is sharper than the proposal in two ways. (i) The exact
  condition is **w constant on the exchanged tiles**, not w = 0 and not only
  w ∘ Θ = w: the feature difference lies in the sum-zero subspace supported on
  the focal hand (four tiles per won trick is a conservation law), so a constant
  contributes nothing. w ∘ Θ = w is the correct condition for J-1's transposition
  and is the special case; J-0's deadness is not proved through any single
  transport, so the transport-invariance formulation does not reach it, and the
  constancy formulation does. (ii) The condition is **gauge-stable** under the
  parent's §11.1 gauge, so it is a condition on the valuation class. Everything
  the branch has claimed survives: count-free trick value, and ordinary Straight
  count under the guard. Lemma J is unaffected — its clause (c) quantifies over
  valuations reading *the point values*, which the guard sends to 0 on H, so it
  is sound as filed; **Lemma J(c′)** generalises it from vanishing to constant,
  with the same proof, and is filed alongside. Propositions J-0, J-1, J-win and
  every J-A ruling stand unchanged. The superseded clause of DS-A9 carries a
  pointer marker at its site; its text is not rewritten.
- **DS-A25 (§2.7, dominance transport). ACCEPT as a narrowing of DS-A15,** filed
  as **Lemma E7** with proof, in a form more precise than the proposed prose:
  pointwise dominance, universal dominance and decision-deadness transport under
  an exhibited value-order isomorphism α_{Tρ}(Tξ) = α_ρ(ξ) with T a bijection of
  both worlds and lawful policy classes — no belief needed, since these are
  pointwise notions; **belief-relative** verdicts (optimality at β, exposure,
  any width) transport only if the belief is transported too, β′ = T_*β. DS-A15's
  rule stands in the binding form: a policy transport establishes lawfulness,
  which is all a primal witness needs; transporting a *verdict* needs the
  isomorphism above. Corollary R-fold is an instance and may be cited as one;
  nothing else in the branch currently is.
- **DS-A26 (§2.8, pruning wording). ACCEPT.** E6.2(c) is narrowed in place:
  Pareto and convex pruning do not preserve N_vec in general, and a pruned run
  may not report N_vec unless it maintains a separate complete unpruned
  accounting — which is the thing the pruning was adopted to avoid, so the escape
  is logical rather than practical. My "no pruning rule preserves it" was too
  strong: rules that discard only duplicates preserve it trivially.
- **DS-A27 (§2.9, evaluator invariant). ACCEPT the semantic form as binding, and
  keep the syntactic form as a sufficient receipt.** The invariant is: every
  later focal action is supplied by the candidate policy, and no optimiser selects
  a focal action using hidden-state information. Nonfocal expectation, chance
  summation, deterministic singleton choices and implementation maxima over
  singleton sets are harmless. "No max node below the root" remains an accepted
  **sufficient** implementation form — cheap to assert structurally and
  impossible to game — but it is a receipt for the invariant, not the invariant,
  and an implementation that satisfies the semantics by other means is lawful if
  it asserts that structurally. Amended in place at errata §4.2.
- **DS-A28 (how a superseded ruling is handled).** This file is the adjudication
  record and is **append-only**: no adjudicated ruling's text is ever rewritten,
  because the record of what was ruled — and later corrected — is itself
  evidence. Three obligations follow, and DS-A24 is their first exercise. (i) A
  superseded or corrected clause receives a **bracketed pointer marker at its
  site** naming the correcting ruling and the durable statement that replaces it;
  a pointer is navigation, not a rewrite, and its absence is drift. (ii) The
  corrected mathematics is filed in the errata with a full statement and proof,
  never as prose in the correcting ruling alone. (iii) The errata, unlike the
  received parent and the received second audit, is a **maintained** document:
  hypotheses may be added and language narrowed in place, each with a dated
  provenance marker naming its ruling, so that no reader meets a superseded form
  without its correction. Received documents stay verbatim; my own documents stay
  correct.

**On §§6–7 of the second audit, for the record and not as rulings.** Its
experimental sequence agrees with DS-A10..DS-A12 in substance; the two
divergences are both improvements and are adopted — the action-conditioned C
evaluator is the most immediate missing component (it is what makes every
per-action upper witness non-vacuous, DS-A7), and the reachable-width experiment
waits on DS-A23. Its §6.4 independently restates PG-A13 (do not infer the first
frontier split from where a capped run stopped), which is correct. Its §7
"closed routes" list is its judgement, not a ruling; I checked it against the
standing results and found nothing false in it, and note only that "closed"
there means "unlikely to repay further work", never that a proved negative has
become a theorem about the game.

## S6c runner: resumption and parallelism (2026-08-13)

**Adjudicator:** walt-math. **Scope:** execution scheduling and persistence for
the adjudicated deadness probe. **No mathematics changes:** the detectors, arms,
ground-truth classification, receipts and report content are exactly as ruled in
J-A1..J-A18, and nothing below alters what is measured. **Basis:** P-A15, P-A16,
P-A17, P-A19, X-A6, X-A16, X-A17, E-A16, E-A17, E-A18, E-A20, J-A11..J-A15.
Amendments continue at DS-A29; freezes continue at 41.

**Headline.** Both changes are lawful, and two things in the proposal are not.
(i) The load-invariance claim on which the whole plan rests is **conditional**,
and the conditions must be asserted rather than assumed (DS-A29) — one
wall-clock-dependent stop or one shared mutable cache would make the *counts*
load-dependent and void the run. (ii) The contended per-unit plain:detector ratio
is not merely "indicative": it is **biased in the flattering direction**, for a
structural reason, so it may be recorded but never quoted as the harvest
dividend (DS-A32). The sequential rung of proposal 2(c) is the right instrument
and is accepted with its selection rule tightened.

- **DS-A29 (the precondition, asserted in-run, not assumed).** Everything the
  proposal claims to be load-invariant — counts, receipts, recall, census
  fractions, exact V/Q — is load-invariant **iff** all four of these hold, and
  the runner asserts each: (a) **every stop criterion is in deterministic budget
  units** (the H solver's particle-step budget), never wall-clock: a time-based
  cap would make a unit's *content* depend on machine load and would destroy
  load-invariance outright; (b) no `Date`, clock, RNG or environment value enters
  any decision (P-A15 already forbids this for sampling; here it is a
  correctness precondition); (c) **workers share no mutable state on which any
  reported number depends** — per-worker caches only, or a shared cache whose
  hit/miss statistics are not reported, since scheduling would otherwise make
  those counts nondeterministic (E-A20's order-relativity, arriving through a new
  door); (d) exact-rational arithmetic throughout, which is what makes any
  reduction order safe (P-A19's no-float rule paying off directly). If any of (a)
  – (d) fails, the parallel plan is void and the failure is stop-and-report, not
  a caveat.
- **DS-A30 (checkpoints: yes to a freeze, with the store discipline that comes
  with it).** A checkpoint is a persisted artifact that a later run **loads and
  trusts**, which is exactly the case the store rulings govern. Binding:
  (i) **freeze 41** is the checkpoint record format, and every record carries the
  **freeze-set digest**; (ii) a record whose digest differs from the running
  freeze set is **corrupt, not stale** — the cache is discarded **entire**, never
  partially reused (X-A6(i), P-A17, E-A18); (iii) the cache is a cache and never
  an authority (X-A17): a resumed run **re-runs a declared sample of loaded units
  and asserts byte-identical non-timing output** before quoting anything, and one
  unit is enough for the sample to be meaningful; (iv) records are written
  **atomically** (temp file, then rename) and carry their own digest — the
  failure mode that motivated this change is being killed at an arbitrary
  instant, and a torn record that the loader trusts is worse than no record;
  (v) unit granularity only — no partial-unit checkpoints, so every record is a
  completed adjudicated unit; (vi) a unit's declared stop (cap or budget
  exhaustion) is itself a checkpointed outcome, and a resumed run reproduces it
  rather than re-running into a different one, which (a) of DS-A29 guarantees.
- **DS-A31 (provenance: yes to a RESUMED line, and three more).** The results
  file prints: (i) **FRESH or RESUMED**, in E-A18's cold/warm style; (ii) the
  freeze-set digest and the number of units loaded versus computed; (iii) the
  **cold regenerate path** that reproduces every headline number from an empty
  cache directory, since the cache is gitignored and nothing quoted from a
  resumed run is otherwise reproducible from the repository alone (E-A17);
  (iv) for **every timing quantity**, the identity of the process that produced
  it. That last one is not bookkeeping: timings assembled from checkpoints
  written by different processes, on a differently loaded machine, are not one
  measurement, and P-A19 voids a wall-clock ratio assembled across runs. A
  resumed run therefore inherits counts and receipts freely and inherits
  **no quotable timing at all**.
- **DS-A32 (parallel timings: recordable, not quotable, and biased in a named
  direction).** Running units W-way parallel with both arms of a unit sequential
  in one worker is lawful, and the per-unit ratio is *closer* to common-mode than
  a cross-worker comparison would be. It is still not a measurement, and the
  error is not symmetric. The two arms have different memory profiles: the plain
  arm is a memory-heavy H solve, the detector arm is the same solve minus pruned
  subtrees **plus cache-resident bitset work**. Under contention the solve
  inflates by some factor c_solve and the detector work by c_det ≤ c_solve, so
  the measured ratio
  (c_solve·S_A)/(c_solve·S_B + c_det·D) ≥ S_A/(S_B + D),
  the uncontended ratio — contention **discounts the detector's own cost and
  flatters the dividend**, which is precisely the S5j failure mode J-A12 exists
  to prevent. Binding: contended timings are labelled `CONTENDED(W=n)`, are
  never compared with any sequential figure, and carry the sentence "these
  timings are biased in favour of the detector arm and are not the dividend."
  The detector's ns/call (J-A12) is under the same bar.
- **DS-A33 (the sequential rung is the only quotable timing, with its control
  re-measured).** ACCEPT proposal 2(c), tightened. The rung is (i) selected **by
  a rule declared in freeze 43 before the parallel pass runs** — a P-A15-style
  deterministic choice over the canonical unit order, never by result, and never
  "the two most interesting units"; (ii) run at **W = 1 in a single
  uninterrupted process**, both arms, from checkpointed inputs only if those
  inputs are non-timing; (iii) the control is the **plain arm re-run in that same
  pass** — not S5h's recorded numbers. S5h's 7–17 s figures are *context*, not a
  control: P-A19 voids a ratio assembled across machines or builds, so those
  numbers may be quoted beside the rung only if the machine, core count and build
  profile are asserted identical, and otherwise are cited as background only.
  Since the rung re-runs both arms anyway, nothing is lost by treating them that
  way.
- **DS-A34 (W is recorded, not frozen — confirmed).** Freezes exist to make
  quoted numbers reproducible. All non-timing output is W-invariant by canonical
  assembly (DS-A29), and all timing output is explicitly load-relative
  (DS-A32), so W determines nothing that is quotable and is an execution
  parameter like the nice level. It is **recorded** alongside the CPU model, core
  count and build profile that P-A19 already requires. The one exception is the
  rung's W = 1, which is part of its declaration and is frozen with it (freeze
  43).
- **DS-A35 (single process only — confirmed).** Multi-process copies sharing one
  checkpoint directory are ruled OUT for this probe, on three independent
  grounds: two processes can write the same record (a torn or interleaved write
  is exactly the silent corruption X-A16 guards against); cross-process timings
  can never be one measurement (P-A19, DS-A31(iv)); and on one machine it buys
  nothing over W threads, since the cores are the same cores. If it is ever
  wanted, the conditions are stated in advance: exclusive-create claim files per
  unit, atomic rename for every record, and per-process timing provenance with no
  cross-process ratio ever printed.
- **DS-A36 (results-file structure, and the receipt it makes free).** The file is
  written in two clearly separated blocks: a **deterministic block** — every
  count, receipt outcome, recall figure, census fraction and exact V/Q, in
  canonical unit order — and a **timing block**. The deterministic block must be
  **byte-identical** across fresh, resumed and any W, which makes it
  byte-diffable in the style of rob's receipts one tier down, and which yields
  the cheapest possible validation of this entire mechanism: run one small
  configuration fresh, run it again resumed from checkpoints, and byte-compare
  the deterministic block. A difference is stop-and-report — it is a defect in
  the checkpointing or in DS-A29's preconditions, never a finding about the game.
  New freezes: **(41)** the checkpoint record format and its freeze-set digest;
  **(42)** the unit identity and the canonical assembly order; **(43)** the
  sequential timing rung's selection rule and its W = 1 requirement. Freezes
  1–40 are in force and restated unchanged.

**Nothing above changes what is measured.** J-A1..J-A18 stand entire; this
section governs only when work happens, where it is stored, and which clock
readings may be quoted.

## Experiment E adjudication: the separation probe (2026-08-13)

**Adjudicator:** walt-math. **Object:** `walt/SEPARATION-PROBE.md` — the
root-action separation probe by primal and upper witnesses — together with the
reserved content of **freezes 36 and 37** (DS-A13). **Tier:** exploratory
throughout. Nothing below is promoted; no number this probe produces becomes
quotable except by brief amendment adding it to a verifier receipt. **Basis:**
DS-A1..DS-A36 and every ruling they inherit (F1–F7, r3 Q1–Q5, Y1–Y3, P-A, X-A,
E-A, S-A, R-A, PG-A, J-A and Lemmas V, X, E, S, S-fold, S-det, R, G, J,
Propositions G-flat, J-0, J-1, J-win, Corollaries S-rigid, R-fold); the errata
`walt/math/decision_sparse_exact_solving_v0.1_errata.md` under DS-A17's citation
rule — Lemma E3 and the four conditions of §3.4, Remark E3.1, Lemma E4 and
Non-theorem E4′ with DS-A27's semantic obligation, Corollary E3.2, Lemma E7,
Theorems E6.3–E6.5; R-A18, PG-A8, PG-A12, PG-A13, P-A19, X-A17, E-A17, E-A18,
DS-A29..DS-A36; the S6a receipts
(`walt-factory/results/predictive_rank_2026-08-12.txt`) and the S6b receipts
(`policy_geometry_2026-08-12.txt`); and first-hand reading of
`walt-strat/src/{info,hidden,hidden_scalar,revealed,price,direction,scalar}.rs`,
`walt-geom/src/envelope.rs`, `walt-core/src/rules.rs` and
`walt-factory/examples/{predictive_rank,policy_geometry,policy_inspect,fiber_probe}.rs`.
Amendments are numbered **SEP-A1..SEP-A18** and bind the build.

**Question labels.** The design numbers its questions E-Q1..E-Q8. That prefix is
already spent: the endgame-store section of 2026-08-11 owns E-Q1..E-Q7, and a
later reader searching "E-Q4" would be sent to two different questions in one
file. The design's questions are renumbered **SEP-Q1..SEP-Q8**, one-to-one and
in order with its E-Q1..E-Q8, and the design text is updated to match before the
run. The ruling prefixes `E-A` and `DS-A` are likewise spent; this section uses
**SEP-A**.

**Headline — five findings, stated before the rulings.**

1. **The upper-witness side needs nothing built.** The design's identification is
   correct and understates itself: `revealed.rs::revealed_summary().q_c` is
   exactly U_a = E_β[V*_a] per root action (Lemma E3, errata §3.1–3.2), and
   `price.rs::information_prices()` already forms **and asserts** the per-action
   price U_a − Q^H(a) ≥ 0 in `g_cont_by_root`. DS-A7(iii)'s premise — that the
   action-conditioned evaluator "must be built" — is **wrong on the `revealed.rs`
   path** and is corrected at SEP-A7, with a pointer marker at the DS-A7 site
   (DS-A28(i)).
2. **The design's two-solver bridge assert is WRONG and would assert a false
   identity.** The envelope path (`hidden::hidden_root_values` at
   `Direction::trick_diff`) and the scalar authority (`ScalarHidden` at
   `ScalarValuation::trick_only`) are **both** focal-minus-opponent trick
   differentials — the same units. Asserting `Q_envelope = 2·Q_scalar − grade`
   between them asserts a false identity at every value that is not that map's
   fixed point. The freeze-26 bridge is the count↔differential bridge; it belongs
   at the S6a cross-check and at the extraction solve, and nowhere else (SEP-A5,
   SEP-A14).
3. **The max-freedom receipt the design names is vacuous at the declared
   direction.** `Envelope::is_affine()` is `pieces.len() == 1`;
   `Direction::trick_diff()` has `delta = 0`, so every `Line` on the ray has zero
   slope and every envelope built from them has exactly one piece — no matter what
   the expand callback returns, the full-legal-set H solve included. The assertion
   cannot fail and therefore is not a receipt (PG-A8: "by construction" is not a
   receipt). The structural obligation of DS-A14/DS-A27 must be discharged by a
   singleton assertion at the callback with a counted receipt (SEP-A13).
4. **The primal side of this experiment carries no information, and the design
   must pre-declare that.** With the H-argmax seed, L_{a⋆} = Q^H(a⋆) exactly and
   necessarily — it is the ceiling of Lemma E4, not a measurement (Corollary E4.1,
   SEP-A2). Two consequences: every separation verdict here is decided **entirely
   by the U side**, and a NOT-SEPARATED pair is not a failure of this run's
   candidates but an exact proof that **no candidate set whatsoever** separates
   that pair under relaxation C at that coordinate. The second is a stronger
   result than the design claims for its own negative outcome.
5. **No economy claim, of any kind, from this run.** The receipts of DS-A10
   require treatment H at every action at every coordinate, so the run completes
   the exact solve it would have to avoid for the parent §8.4 economy claim ["the
   solver does **not** need an exact solution for every action"] to be exercised.
   What the run does test is that the Pareto-frontier enumeration S6b could not
   complete at the tense leads is unnecessary for the root decision. That is a
   real result and it is not the economy claim; the two are separated in the
   results file (SEP-A15).

### Corollary E4.1 (the primal ceiling, and the exact negative) — delivered here

*Extends Lemma E4 (errata §4.1; DS-A14). To be filed in the errata as §4.3 at the
next errata amendment, per DS-A28(ii); until then this statement and proof are
the durable form.*

Fix a coordinate B, a belief β, a field, a valuation, and the canonical
perfect-recall information partition of the future focal decision nodes after
root action a. Let 𝒞_a ⊆ ℛ_H(B,a) be a finite candidate set and
L_a = max_{ρ∈𝒞_a} ⟨β, α_ρ⟩ as in Lemma E4.

1. **(Ceiling.)** L_a ≤ Q^H_B(a) for every 𝒞_a, with equality iff some candidate
   attains the maximum over ℛ_H(B,a).
2. **(Attainment.)** Let the H operator be exact backward induction on the
   information tree — value at a state = max over its legal labels of the child
   value — and let π_a assign to every state **any** action attaining that max.
   Then π_a ∈ ℛ_H(B,a) and ⟨β, α_{π_a}⟩ = Q^H_B(a). Hence L_a = Q^H_B(a) whenever
   π_a ∈ 𝒞_a, **for every tie-breaking rule**.
3. **(The exact negative.)** If Q^H_B(a⋆) < U_a for some competitor a, then no
   finite candidate set whatsoever separates a⋆ from a under that relaxation at
   that coordinate: L_{a⋆} ≤ Q^H_B(a⋆) < U_a for every 𝒞_{a⋆}. The remaining
   lever is a tighter relaxation — a gluing cut (Theorem E6.5, errata §6; DS-A3's
   cut typing) — and never a better candidate.

*Proof.* (1) is Lemma E4. (2): each observation record is visited at most once per
walk, so the reachable focal information states after a form a finite tree of
bounded depth ordered by record extension. Induct on remaining depth. At a
terminal the fixed-policy value and the H value coincide. At a field node both are
the same nonnegative linear aggregation of child values, so the induction passes
through. At a focal state s the H value is the max over legal labels of the child
H value, and by hypothesis π_a(s) attains it; by the induction hypothesis the
fixed-policy value of the subtree under π_a(s) equals its H value; hence the
fixed-policy value at s equals the H value at s. Taking s to be the root state
after a and pairing with β gives ⟨β, α_{π_a}⟩ = Q^H_B(a). π_a is a total function
of the information state, hence information-consistent. Tie-breaking is irrelevant
because every maximiser satisfies the hypothesis. (3): compose (1) with the
assumed strict inequality. ∎

**What (2) buys and what it costs.** It buys the sharpest primal witness
available, so a separation failure is maximally informative (clause 3). It costs
the experiment its primal content: L is no longer a measurement of the library but
a receipt tying two evaluators together (SEP-A11, SEP-A12).

### SEP-A1..SEP-A2 — typing and vocabulary for this section

- **SEP-A1 (vocabulary, restating DS-A1 where it bites here).** **Primal witness**
  = L_a, the exact value of a fixed lawful information-consistent policy
  integrated under the declared belief; **upper witness** = U_a, the
  action-conditioned treatment-C value; **root-action separation** = the relation
  L_{a⋆} ≥ U_a for every a ≠ a⋆; **receipt** = a machine-checked verification
  artifact regenerated by a run. The word "certificate" does not appear in this
  probe's design, code, results file or wiki text; where the parent v0.1 document
  is quoted it is bracketed as a quotation (D3, DS-A1). The design already
  observes this fence and is CONFIRMED on it. The R-A2 reachability fence is
  restated verbatim wherever a witness is reported: the fiber is the void-free
  capacity fiber, its members are FEASIBLE and never reachable (P-A1), and no
  object here is identity-bearing.
- **SEP-A2 (Corollary E4.1 is binding on the design's typing).** The three clauses
  above bind every reported verdict. In particular the design's phrase "maximised
  over a finite declared candidate set" is formally correct and practically empty
  at v1: the candidate set at each H-optimal action is the singleton {π_a}, and L
  is at its ceiling. The results file states this in its header, before any number,
  in the form: *"the primal witness at each H-optimal action is an H-optimal policy
  re-priced by the fixed-policy evaluator, so L = Q^H by Corollary E4.1(2); the
  separation verdict at this coordinate is determined entirely by the upper
  witness."*

### SEP-A3..SEP-A4 — SEP-Q1: freeze 36

- **SEP-A3 (freeze 36 as specified: ACCEPT-WITH-AMENDMENT, six amendments).** The
  entry format, the identity-only transport and the DS-A16 header note are right.
  Six things are wrong or missing and each is binding.
  (i) **`InfoStateId` may never appear in a stored entry.** It is an index assigned
  in `InfoPartition::build` traversal order — an in-process handle, not a canonical
  key — and storing it makes the entry a second authority over an ordering the
  partition already derives (the project's derived-views rule, and DS-A15's
  canonical-key requirement). The stored form is keyed on the **observation
  record**: the sequence of plays since the kernel decision point, root action
  first, exactly `walk`'s `obs`. The design's own serialisation sentence already
  says this; the ruling makes it exclusive.
  (ii) **The coordinate key's `pip` is derived, not independent.**
  `coordinate(grade, index)` computes `pip = index / (live_c · hand_c)`, so
  (grade, base index, declaration) determines pip. Pip is a printed field for human
  reading and never a key component; a stored pip that disagrees with the unranking
  is corruption, asserted at load.
  (iii) **The extraction tie-break is not freeze 36's to declare.** Freeze 26
  already fixes it — "tie rule for the extracted policy: least domino index among
  the argmax" — and re-declaring it inside freeze 36 creates two authorities for
  one constant, which is the DS-A4 defect in freeze clothing. Freeze 36 **cites**
  freeze 26.
  (iv) **An entry stores no value and no verdict — and no rank and no dominance
  status.** The design says "never verdicts"; add "never numbers". A stored L would
  be a number a later run could quote without re-running the evaluator that makes
  it a witness. A library file is a cache and never an authority (X-A17); a loaded
  entry is re-priced by `policy_value` before anything is reported.
  (v) **Every entry carries the freeze-set digest** and the frame it was built
  against — observation contract, field, belief, fiber enumeration order and |X|.
  A digest mismatch makes the file **corrupt, not stale**, and it is discarded
  entire, never partially reused (DS-A30(i)–(ii), X-A6(i), P-A17, E-A18). The
  information partition is a function of the observation contract, so an entry
  built under a different contract is not merely stale — it is not well-defined at
  the coordinate at all.
  (vi) **Transport cites Lemma E7, not only DS-A15.** Identity transport is lawful
  and trivially satisfies both halves of the rule; the ruling to record is the
  general one (errata §8.3, DS-A25): a policy transport establishes lawfulness,
  which is all a primal witness needs, and transporting a *verdict* additionally
  requires an exhibited value-order isomorphism α_{Tρ}(Tξ) = α_ρ(ξ) with the belief
  transported for belief-relative verdicts. Dominance does not travel with a policy
  alone.
- **SEP-A4 (FREEZE 36 — the candidate-policy library, v1, frozen content).**
  **(a) Key:** (grade, base index, declaration ∈ {0..6}, root action) under the S6a
  unranking `coordinate(grade, index)` (freezes 22–25); pip derived per
  SEP-A3(ii). **(b) Body:** a total map from observation record to chosen tile over
  `InfoPartition::build(kernel, root)`, serialised as the list of (observation
  record, chosen tile) pairs sorted lexicographically by record under the canonical
  ascending domino-index order, the record being the plays since the kernel
  decision point with the root action first; `InfoStateId` never appears.
  **(c) Frame, mandatory on every entry:** observation contract (the full public
  record, R-A11, freeze 26), field (v0.4 §7.4 uniform-legal, F4), belief (uniform
  over the enumerated void-free capacity fiber in the freeze-7/23 enumeration
  order), |X|, and the freeze-set digest. **(d) Stored content is a policy and its
  provenance only** — no value, rank, verdict or dominance status; the file is a
  cache, never an authority; a loaded entry is re-priced before use; a digest
  mismatch is corruption and the file is discarded entire. **(e) Transport:**
  identity only in v1, per SEP-A3(vi); cross-coordinate transport re-enters with
  its own adjudication. *[AMENDED 2026-08-13 by EC-A8: freeze 36(e) v2
  additionally admits the declaration fold φ_{p→p′} of Lemma S-fold — image key
  per freeze 46(b), R9 receipts asserted in-run, values licensed by Corollary
  S-fold-val; any further transport still re-enters with its own adjudication.]*
  **(f) Seed rule:** the seed is the argmax-recording pooled
  H solve over the same information partition, **unmemoized** (SEP-A11(ii)), with
  the tie rule of freeze 26 cited, not restated; the seed contributes no number to
  any reported L (DS-A14, DS-A15). **(g) Header note (DS-A16):** entries remain
  valid primal-witness sources under count re-entry, evaluated under the richer
  valuation; their count-free quality verdicts do not survive (E-A2, Proposition
  J-win).

### SEP-A5..SEP-A7 — SEP-Q2: freeze 37 and the DS-A7(iii) correction

- **SEP-A5 (the U-side identification: ACCEPT; the bridge assert: REJECT).** The
  identification of `revealed.rs::revealed_world_root_values` /
  `revealed_summary().q_c` as U_a is verified and CORRECT. The function removes the
  root action from the viewer's hand, solves on a one-particle bag with the world
  revealed and the full legal set expanded at every later focal node, and leaves
  the other three seats on the fixed uniform-legal field; `revealed_summary` sums
  per-world envelopes and scales by 1/|X| over the full enumerated fiber. At a
  fixed direction the support of a weighted Minkowski sum is the weighted sum of
  supports, so `q_c` read on the ray is exactly E_β[V*_a] = U_a of errata §3.1, and
  treatment C's field is untouched, so (C4) holds by construction.
  The **bridge assert is REJECTED and must be removed**: `hidden_root_values` at
  `Direction::trick_diff` (base = unit trick, delta = 0) and
  `ScalarHidden::action_values_dag` at `ScalarValuation::trick_only` (trick = 1,
  tiles = 0, negated when the winner is not focal) are both focal-minus-opponent
  trick differentials. **They are asserted equal exactly, with no bridge.** The
  freeze-26 bridge Q_diff = 2·Q_count − grade converts between the count convention
  (S6a's filed Q, `policy_inspect`'s extraction solve, `predictive_rank`'s `fused`)
  and the differential convention (both H solvers, `revealed`, `price`,
  `policy_value`); it applies wherever those two conventions meet and nowhere else.
  It is exact and its slope is 2 > 0, so every inequality, argmax and separation
  verdict is invariant under it — the run may report in either convention provided
  it declares which and asserts the bridge at the boundary. **Reporting
  convention: the count convention**, so the rows are directly comparable with the
  S6a receipts.
  Two further clauses. (i) The probe **asserts the root is trick-leading** (empty
  prefix): `hidden_root_values` and `revealed_world_root_values` iterate
  `kernel.viewer_hand()` while `ScalarHidden` iterates
  `legal_plays(decl, hand, led)`, and these coincide only when `led` is `None`.
  Without that assertion the two solvers can be compared over different action
  lists. (ii) The per-action price is
  `price.rs::information_prices().g_cont_by_root`, which already exists and already
  asserts U_a − Q^H(a) ≥ 0 envelope-wise; the probe reads it rather than
  recomputing it, and prints it, because **that column is the measurement**.
- **SEP-A6 (FREEZE 37 — the action-conditioned upper witness and its solver
  identification, frozen content).** **(a) Evaluator:** U_a :=
  `walt_strat::revealed::revealed_summary(kernel, focal, dir).q_c[a]` read at the
  declared direction, identified as E_β[V*_a] of errata §3.1 (Lemma E3, §3.2;
  DS-A7). **(b) Relaxation name:** treatment **C**, not C⁺ — on this carrier the
  latent is ξ = ω and the two coincide, and the results file says so (DS-A20).
  **(c) Direction and convention:** `Direction::trick_diff()` (base = unit trick,
  delta = 0), the count-free focal trick differential; the reporting convention is
  S6a's count convention and the freeze-26 bridge Q_diff = 2·Q_count − grade is
  asserted exactly at the reporting boundary; the bridge is affine with positive
  slope, so verdicts are convention-invariant. **(d) Belief and world set:**
  uniform over `kernel.worlds()`, the full enumerated void-free capacity fiber,
  identical on both sides of the sandwich; no decimated world set from any probe —
  `fiber_probe`'s W = 240 sets included — appears inside any L or U ((C2) of errata
  §3.4; T7). **(e) Conditions:** (C1)–(C4) of errata §3.4 asserted in-run — same
  field, same belief and world set, same utility and count contract on both sides,
  PI minimax never substituted. **(f) Per-action price:** U_a − Q^H(a) is
  `price.rs::information_prices().g_cont_by_root[a]`, asserted nonnegative; the
  aggregate siblings in the same struct are named once and never confused with it —
  V^C = max_a U_a, V^F = U^agg = E_β[max_a V*_a], g_total = V^F − V^H = the S6a
  `fusion_gap` column. **(g) Solver identification:** the envelope path
  `hidden::hidden_root_values` at `trick_diff` and the scalar authority
  `ScalarHidden::action_values_dag` at `trick_only` with `AUTHORITY_BUDGET`
  (freeze 26) are two independently built solvers in the **same** units and are
  asserted **equal exactly, per action, with no bridge**; the probe asserts the
  root is trick-leading so their action lists coincide. **(h) Budget honesty:** the
  scalar authority is budgeted and its exhaustion is a declared stop printed
  R-A18-style with "correctness gate unmet" beside every row it voids;
  `hidden.rs`, `revealed.rs`, `price.rs` and `policy_value` carry **no budget and
  no stop**, and the results file states that in place rather than implying a
  uniform stop discipline across evaluators.
- **SEP-A7 (the DS-A7(iii) premise is corrected; recorded as a ruling, with a
  pointer marker at the site).** DS-A7(iii) reads: "the implementation's
  world-informed evaluator maximises at every focal node including the root, so the
  action-conditioned variant is a small change to an existing path
  (`walt-strat/src/revealed.rs`, `fiber_probe.rs:215`) and must be built, not
  assumed." Verified against the code, that premise is **wrong on the `revealed.rs`
  path**: `revealed.rs` has contained the action-conditioned evaluator since S3.
  The design is right to raise this and its own statement is incomplete; the
  durable form is:
  **(1)** `revealed::revealed_summary().q_c[a]` is the **action-conditioned**
  object U_a = E_β[V*_a] — root action held, world revealed for later focal
  decisions only.
  **(2)** `revealed::revealed_summary().v_f` is the **root-maximising** object
  U^agg = V^F = E_β[max_a V*_a] — in the same file, one field away, which is why
  the confusion was easy to make.
  **(3)** `fiber_probe.rs::aggregate` (max at actor offset 0, mean at offsets 1–3)
  is the P-A6 root-maximising operator, and
  **(4)** `predictive_rank.rs::fused` is the root-maximising evaluator that
  actually produced the S6a `fusion_gap` column the design quotes.
  DS-A7(iii)'s substance is untouched — the ruling that U must be
  action-conditioned, that E_β[max_a V*_a] is action-constant and makes the
  separation vacuous, and that the `fusion_gap` column is not a per-action price,
  all stand and are restated in Remark E3.1 (errata §3.3). What changes is one
  engineering premise: **what remained to build was the harness and the receipts,
  not the evaluator.** A bracketed dated pointer marker is placed at the DS-A7(iii)
  site per DS-A28(i); its text is not rewritten.

### SEP-A8..SEP-A9 — SEP-Q3 and SEP-Q4

- **SEP-A8 (SEP-Q3, the argmax tie-break: ACCEPT the rule, REJECT re-freezing it,
  and it is weaker than the design thinks).** The rule — least domino index among
  the argmax — is right and is **already freeze 26**; freeze 36 cites it
  (SEP-A3(iii)). The design says the tie-break "affects which witness is exhibited,
  never validity". True, and understated: by Corollary E4.1(2) it does not affect
  **the value of L either**, because every argmax selection at every information
  state yields a policy attaining Q^H(a). The tie-break is a determinism freeze for
  the exhibited policy — which matters for a library entry and for byte-diffable
  output — and is not a soundness clause.
  One further fact must be stated wherever the two conventions meet, because the
  extraction solve (`policy_inspect.rs::Ctx::solve`) runs in the **count**
  convention while the pricing runs in the **differential** convention: at any node
  the number of tricks remaining is the same for every legal action, so
  V_diff = 2·V_count − (tricks remaining) with an action-independent offset; hence
  the argmax sets, and therefore the least-tile selection, are **identical** under
  the two conventions. Without that observation the two-convention split in the
  extraction is unjustified; with it, it is exact.
- **SEP-A9 (SEP-Q4, the tied optimum at idx = 2599418: LAWFUL as stated, with one
  completion and one simplification).** The protocol — attempt each tied action
  symmetrically, report which if either separates, and report the exhaustion
  statement as treatment H's fact and never the witnesses' — is lawful and is
  exactly Theorem E6.4's member-not-set caveat honoured (errata §6; DS-A22's
  restatement). Two corrections to the design's reading 3.
  (i) **Incomplete as written.** Certifying a⋆ = 11 requires L_{11} ≥ U_a for
  **both** competitors: the tied 22 **and** 42. The design's table has the 42
  headroom (1/21) but its prose names only the tie. The binding form: with
  V^H = 15/14, action 11 separates iff U_{22} ≤ 15/14 **and** U_{42} ≤ 15/14;
  action 22 separates iff U_{11} ≤ 15/14 and U_{42} ≤ 15/14 (count convention).
  (ii) **One conjunct is automatic.** The design writes that separation "closes iff
  the tied competitor's action-conditioned gap is exactly 0 AND the primal witness
  attains V^H". By Corollary E4.1(2) the second conjunct holds by construction, so
  the condition is exactly: the tied competitor's per-action price U − Q^H is
  **exactly zero**, and the 42 price is at most 1/21. Since U_a ≥ Q^H(a) always,
  "at most zero" means "exactly zero", and by Theorem E6.3 that forces
  L = Q^H = U at the tied competitor — the sandwich collapses there. This
  coordinate is therefore a test of whether the C-relaxation is **exactly tight**
  at a tied H-optimal action, which is the sharpest question the three coordinates
  pose.

### SEP-A10..SEP-A11 — SEP-Q5 and SEP-Q6

- **SEP-A10 (SEP-Q5, the n = 4 rung: REJECTED for v1).** Not in scope, on three
  independent grounds, any one of which suffices.
  (i) **The cost model is unstated, not merely large.** The design asks whether
  "34650 one-world solves per action" is acceptable and supplies no estimate of
  what a one-world grade-4 solve costs. A rung admitted on an unestimated cost is a
  stop written into v1 by construction — the design's own rule is that a unit
  exceeding ten minutes returns here for a runner adjudication. *[RETIRED AS A
  GATE 2026-08-14 by N4-A14: the ten-minute threshold was never frozen and was
  mistyped — a wall-clock quantity that decides whether a unit is computed
  makes the set of computed units load-relative. It is replaced by a
  run-owner-declared whole-pass budget T_pass of M_max's type, compared before
  the pass and gating no content. The demand for a measured cost model, which
  is the substance of this clause, is discharged by the §5 rung and by
  Corollary N-1.]*
  (ii) **The U-side evaluator cannot declare a stop.** `revealed_summary` takes no
  budget and returns no `Option`; there is no stop to declare and no partial result
  to print. At grade 3 that is harmless because the run completes; at grade 4 it is
  the difference between a declared stop (R-A18, P-A16, E-A16) and a run that must
  be killed, which is precisely the failure mode DS-A30 exists to handle and which
  this design explicitly does not engage.
  (iii) **The primary receipt is likely unavailable.** DS-A10's receipt for
  Experiment E is conditioned on H completing at the same coordinate. Whether
  `ScalarHidden::action_values_dag` completes at grade 4 within `AUTHORITY_BUDGET`
  is unmeasured. Where it does not, every row prints "correctness gate unmet" and
  the coordinate contributes a separation verdict with no authority cross-check —
  mathematically still valid under Theorem E6.4, but not what DS-A10 authorised.
  **The repair, if the rung is wanted:** it is its own design with its own
  adjudication, minimally carrying (a) a measured single-world grade-4 timing rung
  before any full pass, (b) a declared per-(coordinate, action) budget in
  **deterministic units** — particle-steps, never wall-clock (DS-A29(a)) — on every
  evaluator including the revealed path, which means adding a budget to
  `revealed_summary`, and (c) a declaration of what a coordinate reports when H
  does not complete. Nothing about the n = 4 rung is unlawful; it is unspecified.
- **SEP-A11 (SEP-Q6, L < Q^H for an H-argmax seed: the typing is CORRECT, and the
  reason is stronger than the design gives).** Stop-and-report is right, and it is
  right because Corollary E4.1(2) makes L = Q^H a **theorem** about the pipeline,
  not an expectation about the seed: a strict inequality proves a defect in the
  probe, not a fact about the game. NO-RESCUE applies in full — never patched,
  never reconciled by adjustment (F7, R-A18). Two clauses attach.
  (i) **The three defects it can indicate are named in the stop message**, so the
  stop is diagnostic rather than a bare failure: the extraction did not produce an
  argmax at every state; the extraction's information partition disagrees with
  `InfoPartition::build`; or the two H authorities disagree (which the SEP-A6(g)
  identification would have caught first).
  (ii) **One anticipated non-bug cause is excluded by construction, not by
  inspection.** `ScalarHidden::action_values_dag` cannot supply the seed: it returns
  root action values only, its `node_dag` records no argmax, and its trick-boundary
  memo returns whole subtrees from the cache without expanding them — so a policy
  harvested from it would be **partial**, missing an action at every information
  state below a cache hit, and `Policy::build` would either panic or receive a
  fabricated choice. The seed therefore comes from an **unmemoized**
  argmax-recording pooled H solve over the same partition;
  `policy_inspect.rs::Ctx::solve` is exactly that object and already implements the
  freeze-26 tie rule. If a memoized extraction is ever wanted, the conditions are
  stated in advance: the memo stores the argmax with the value, and the run asserts
  the assembled map is **total** on `InfoPartition::build`'s state set before
  pricing.

### SEP-A12..SEP-A14 — SEP-Q7 and SEP-Q8: the receipts

- **SEP-A12 (SEP-Q7, are step 7's receipts sufficient? AMEND — as written two of
  the three are near-vacuous, and the informative receipts are elsewhere).** DS-A10
  asks for "R-A18's discipline extended to the witness". R-A18's discipline is not
  two assertions: it is (a) exact equality against the concrete authority, (b)
  stop-and-report on mismatch with no adjustment, and (c) a declared stop printed
  with what was reached, with "correctness gate unmet" beside every row it voids,
  never silently. Measured against that, the design's step 7 is incomplete. The
  chain assertion `L ≤ Q^H` holds as **equality by construction** (Corollary
  E4.1(2)), and `U ≥ Q^H` is already asserted inside
  `price.rs::information_prices`; neither can fail in a way that indicates anything
  about the game, and PG-A8 forbids presenting "by construction" as a receipt.
  **The five receipts that carry content, all mandatory:**
  **(R1) solver identification** — envelope H equals scalar-authority H exactly,
  per action, no bridge, root asserted trick-leading (SEP-A6(g));
  **(R2) the primal receipt** — L = Q^H **exactly**, per H-optimal action, tying
  `policy_value` to the two agreeing H authorities through a third, structurally
  max-free code path; this is the receipt the design mislabels a measurement;
  **(R3) the measurement** — the per-action price U_a − Q^H(a) printed as an exact
  rational for **every** action, from `g_cont_by_root`, with the sign assertion it
  already carries;
  **(R4) the S6a cross-check** — SEP-A14;
  **(R5) the max-freedom receipt** — SEP-A13.
  Plus, unchanged from the design and correct: the certified action asserted to lie
  in H's argmax, and Theorem E6.4's member-not-set caveat printed **verbatim beside
  every SEPARATED verdict**, never in a footnote.
  One typing clause that must be printed and is easy to lose: **the separation's
  validity does not cite H, but this run's witnesses were produced with H's help.**
  L's seed is an H solve and (R1)–(R4) are all H cross-checks. The logic of Theorem
  E6.4 is H-free; the provenance of these particular witnesses is not. A results
  file that blurs those two will be read as a claim this run does not support.
- **SEP-A13 (the max-freedom receipt: the named assertion is VACUOUS; the repair is
  mandatory).** DS-A14 requires the no-maximisation property to be asserted
  **structurally, not by inspection**, and DS-A27 fixes the semantic invariant with
  "no max node below the root" as an accepted sufficient implementation form. The
  design offers three things. Two are sound: the focal callback in `policy_value`
  returns `DominoSet::single(policy.action(id))`, and `Policy` is a total function
  on the information partition — which is the **right** reason world-peeking is
  unconstructible, and a stronger reason than the design's "opaque `InfoStateId`":
  a policy assigns one action per information state, so it cannot depend on the
  hidden world whatever its constructor knew. The third is **vacuous**:
  `env.is_affine()` is `pieces.len() == 1`, and at `Direction::trick_diff()` the
  delta is zero, so every line has zero slope, no two lines cross, and every
  envelope on the ray has exactly one piece — including the full-legal-set H
  envelope. The assertion cannot fail and is not a receipt.
  **Repair, binding:** the L path asserts, at every focal callback invocation, that
  the returned set is a **singleton** (`chosen.len() == 1`), and the run prints a
  counted receipt — the number of focal decision states evaluated and the number of
  singleton expansions — asserted equal to each other and to the count of states
  the walk reaches in `InfoPartition`. *[DISAMBIGUATED 2026-08-13 by SEP-A19: the
  third quantity is the number of DISTINCT partition states the L walk actually
  reaches, never `InfoPartition::len()`, which a fixed policy cannot reach and
  which no correct run can satisfy.]* `is_affine()` may remain in the code as a
  cheap invariant; it may not be **reported** as the max-freedom receipt, and the
  results file does not name it as one.
- **SEP-A14 (SEP-Q8, the S6a values: ASSERTED, not printed side-by-side).** Exact
  equality, asserted, stop-and-report on mismatch. Printing side by side would let
  a drift in the frozen unranking, the kernel construction or the enumeration order
  pass silently, and R-A18 already binds: treatment H is the concrete authority and
  two runs of that authority at one frozen coordinate that disagree are a bug,
  never a finding. Three clauses. (i) The comparison is made in the **count
  convention** after the freeze-26 bridge, since the S6a filed values (53/21,
  355/168, 16319/6720; 1, 43/42, 127/126; 15/14, 15/14, 43/42) are count values and
  the probe's evaluators are differential. (ii) The S6a values enter the probe as a
  **frozen table in the probe source** carrying the provenance line "quoted from
  `predictive_rank_2026-08-12.txt`, S6a, exploratory tier" — not re-parsed from the
  results text, which is not a machine-readable interface. (iii) The **coordinate
  identity** is asserted first: same grade, same base index, same declaration, same
  |X| = 1680, same fiber enumeration order; an equality of Q values at coordinates
  that were not shown to be the same coordinate is not a cross-check.
  I have re-derived the design's prior-data table from the S6a receipts and it is
  **arithmetically correct in every entry**: at idx = 0, 53/21 − 355/168 = 23/56
  and 53/21 − 16319/6720 = 641/6720; at idx = 1299709, 43/42 − 1 = 1/42 and
  43/42 − 127/126 = 1/63; at idx = 2599418, 15/14 − 43/42 = 1/21 with the tied
  competitor at 0; and the three `fusion_gap` values 9301/120960, 2663/181440 and
  23/420 are quoted exactly, in the same count convention as the Q rows. The argmax
  attributions are correct. No discrepancy.

### SEP-A15..SEP-A18 — the reading, the results discipline, and the freezes

- **SEP-A15 (the pre-declared reading: AMEND in three places).**
  (i) **The aggregate-gap column is a one-sided screen and licenses nothing else.**
  Placing "headroom per competitor" and "aggregate fusion gap" in adjacent columns
  invites the inference *gap < headroom, therefore separation is likely*. That
  inference is invalid. The only implication available is Corollary E3.2's: if
  U^agg − V^H = 0 then U_a ≤ V^H for every a and, with L at its ceiling, every
  H-optimal action separates with no gluing iteration at all. When the gap is
  nonzero, U_a ≤ V^H + gap is all that follows, which never establishes U_a ≤ V^H.
  The numeric coincidence that the gap is smaller than the tightest headroom at two
  of the three coordinates (9301/120960 against 11538/120960 at idx = 0;
  2663/181440 against 2880/181440 at idx = 1299709) and larger at the third
  (23/420 against 20/420 at idx = 2599418) is **not evidence in either direction**,
  and the results file says so where the column appears.
  (ii) **The thesis scope is narrower than reading 1 claims.** What the run can
  demonstrate at idx = 0 is that the object S6b could not complete — the Pareto
  frontier at leads 10 and 11, capped at 16384 (PG-A13) — is **not needed** for the
  root decision. That is a genuine result. What it cannot demonstrate is the parent
  §8.4 economy claim, quoted bracketed: ["the solver does **not** need an exact
  solution for every action"]. This run computes an exact H solve at every action
  at every coordinate, because DS-A10's receipts require it. Both sentences are
  printed; neither is allowed to stand for the other.
  (iii) **No cost, timing, runtime or tractability claim of any kind.** Since the
  run performs the full exact solve it would have to avoid, any timing comparison
  is void by construction, and P-A19/DS-A32's discipline applies a fortiori. The
  probe prints wall-clock only as provenance, never as a dividend.
  Otherwise the pre-declared reading is ACCEPTED as written, including the three
  named coordinate roles, F7/NO-RESCUE on both outcomes, and the R-A2 fence.
- **SEP-A16 (what a NOT-SEPARATED verdict says, and it is more than the design
  claims).** By Corollary E4.1(3), a failing pair (a⋆, a) at these coordinates is
  the exact statement **Q^H(a⋆) < U_a**, and therefore a proof that no candidate
  policy set whatsoever separates a⋆ from a under relaxation C at that coordinate.
  The results file prints it in that form — the failing gap U_a − Q^H(a⋆) as an
  exact rational, with the sentence that no primal witness can close it — and not
  as "this run's candidates were not strong enough". This is the input Experiment D
  needs: the failing pairs are exactly where a gluing cut would have to bite
  (Theorem E6.5; DS-A3's cut typing — a cut constrains the relaxation, never the
  lawful policy class and never the fiber). It remains exploratory tier and remains
  coordinate-relative: nothing here is a statement about any coordinate not run,
  about the opening, or about reachability.
- **SEP-A17 (the successor experiment, named now so this one is not read as it).**
  The experiment that would test the parent's economy claim is: seed L from a
  source that is **not** an exact solve at a⋆ — a transported library entry, a
  hand-authored playbook, a cheap heuristic — and ask whether the sandwich still
  closes. It is out of scope here for a reason the design states correctly: six of
  the seven completed S6b singleton roots collapse by indifference, so this run's
  harvest is one informative entry (idx = 0, lead 00, the 108-decision playbook)
  *[DISAMBIGUATED 2026-08-13 by EC-A12: "108" counts the strictly-mattering
  subset — 384 free two-tile decision states minus 276 one-deviation ties (S6c,
  `deadness_2026-08-12.txt`) — a derived difference of two measured counts,
  present in no receipt; the entry's receipt-backed decision count is 384
  (`separation_2026-08-13.txt`, S6b k = 384). See EC-A12.]*
  plus lawful-but-vacuous entries, and there is nothing yet to transport. That
  successor needs freeze 36's transport clause opened, which needs Lemma E7's
  isomorphism exhibited, which is its own adjudication (SEP-A3(vi), DS-A25).
  Recording it here prevents the standing question "did walt ever test the economy
  claim?" from being answered by pointing at this run.
- **SEP-A18 (freezes).** Freezes 1–35 are in force and restated unchanged.
  **(36)** is fixed at SEP-A4 — the candidate-policy library v1: key, body, frame,
  no-values rule, identity-only transport, seed rule and the DS-A16 header note.
  **(37)** is fixed at SEP-A6 — the action-conditioned upper witness: the
  `revealed_summary().q_c` evaluator, the treatment-C naming clause, the declared
  direction and reporting convention with the freeze-26 bridge at the boundary
  only, the belief and world set with the no-decimation clause, conditions
  (C1)–(C4), the per-action price object, the two-solver identification with **no
  bridge**, and the budget-honesty clause. **(38)–(40) remain reserved and
  untouched** — the gluing-cut language and cut ordering, the circuit
  representation and evaluation order, and the reachable-belief family for W_reach
  (DS-A13, as revised by DS-A23). **(41)–(43)** stand as ruled at DS-A36. No number
  is reused.

**What must change in the design before it is built.** Six items, in order of
severity: the bridge assert between the two H solvers is removed and replaced by
an exact equality (SEP-A5); the `is_affine` receipt is replaced by the singleton
assertion and its counted receipt (SEP-A13); the seed is taken from an unmemoized
argmax-recording solve and never from `action_values_dag` (SEP-A11(ii)); the
n = 4 rung is removed from v1 (SEP-A10); the tie-break and the scalar authority
are cited to freeze 26 rather than re-declared (SEP-A3(iii), SEP-A6); and the
pre-declared reading gains the primal-ceiling paragraph, the one-sided-screen
paragraph and the two thesis sentences (SEP-A2, SEP-A15). Everything else in the
design is sound and is bound as written.

### SEP-A19 — the SEP-A13 counted receipt, disambiguated at build time (2026-08-13)

**Raised by:** the build, from a first implementation that read SEP-A13's third
quantity as `InfoPartition::len()` and tripped at idx = 0 lead 00 — 22,920 states
reached against 50,712 in the partition, with the extraction totality receipt
`choices.len() == partition.len() == 50,712` HELD and (R1), (R2), (R4) HELD. The
trip was correct behaviour by a wrong receipt, which is the good failure mode.
A pointer marker is placed at the SEP-A13 site per DS-A28(i).

- **SEP-A19 (the third quantity is the reached set, and equality with
  `InfoPartition::len()` is unsatisfiable).** The build's proposed reading is
  CONFIRMED. The receipt is
  **focal callback invocations == singleton expansions == distinct partition
  states reached**, the third counted as a set of `InfoStateId`s accumulated at
  the callback. It is a genuine receipt and not a tautology: the first equality is
  the DS-A14/DS-A27 structural obligation asserted per call; the second additionally
  witnesses that no observation record is visited twice in one walk — the property
  `info.rs` claims in prose and on which Corollary E4.1(2)'s tree induction
  depends — and that every visited record is a partition state.
  **Equality with `InfoPartition::len()` is not merely unmet here; it is
  unsatisfiable for any policy that ever prunes.** The partition enumerates the
  states reachable after the root action under *some* lawful continuation; a fixed
  deterministic policy selects one action at each state it reaches, so the L walk
  descends only the policy-consistent subtree. A run in which the two counts agreed
  would be a run whose "policy" expanded counterfactual focal branches, which is
  exactly the max node DS-A14 forbids. The two receipts are therefore complementary
  and both are mandatory: **totality** on the extraction side
  (`choices.len() == InfoPartition::len()` — the seed must supply an action at
  every state, since the walk's reached set is not known before the walk) and
  **reachedness** on the pricing side.
- **SEP-A19(b) (how the pair of numbers is reported, and one fence).** The results
  file prints `reached X of partition Y` with both numbers, as the build proposes.
  Typing, mandatory beside it: X is an exact computational observable of the
  **exhibited witness**, in the same class as `InfoPartition::len()` = E_B(a)
  (§10.9) — not an information value, not a decision width, not a policy count, and
  never a term in the DS-A2 ladder. It is additionally **tie-break-relative**: at a
  state where the H-argmax set is not a singleton, freeze 26's least-tile rule picks
  one action and a different rule would descend a different subtree and reach a
  different count. Y is the tie-break-free quantity; X is not. A results file that
  quotes X without that sentence has published a determinism-freeze artifact as a
  fact about the coordinate. The ratio X/Y is not a measurement of anything and is
  not printed.

**Nothing in SEP-A1..SEP-A18 changes.** SEP-A13's repair stands as ruled; only the
identity of its third counted quantity is disambiguated, and the design text and
probe may proceed on the build's reading.

## The n = 4 separation rung adjudication (2026-08-13)

**Adjudicator:** walt-math. **Object:** `walt/SEPARATION-RUNG-N4.md` (commit
42d83d9) — the n = 4 separation rung, the successor SEP-A10 prescribed,
answering its three-part repair spec — together with the reserved content of
**freezes 44 and 45**. **Tier:** exploratory throughout. Nothing below is
promoted; no number the rung produces becomes quotable except by brief amendment
adding it to a verifier receipt. **Basis:** SEP-A1..SEP-A19 and everything they
inherit; the errata under DS-A17's citation rule (Lemma E3 and (C1)–(C4) of
§3.4, Lemma E4 and Non-theorem E4′ with DS-A27's semantic obligation, Corollary
E4.1 — pending its errata filing as §4.3 — Corollary E3.2, Theorems E6.3–E6.5);
DS-A29..DS-A36; R-A18, PG-A8, PG-A13, P-A19, P-A21, X-A17, F7; and first-hand
re-verification, at adjudication time, of every receipt number and code claim
the design quotes: `walt-factory/results/fiber_probe_h_2026-08-11.txt` (the
nine in-scope hands h0, h1, h2, h4, h5, h6, h8, h9, h12 and the out-of-scope
four; the `tree-v0` range 1,855,419,966 (h6) to 16,211,488,002 (h9); the
`dag-v1` range 78,359,234 (h2) to 191,841,542 (h5)),
`fiber_probe_2026-08-11.txt` (the P-A2 void-filtered sizes: ratio 1 at
h0/h1/h4/h6/h9/h12; 23,100 at h2; 14,700 at h5; 1,200 at h8),
`separation_2026-08-13.txt` (partition sizes 50,712/134,190/109,788/98,628;
wall-clock 3,942 ms), `store/candidate_library.txt` (393,333 lines, four
entries), and `walt-strat/src/{info,hidden,hidden_scalar,revealed}.rs`,
`walt-factory/examples/{fiber_probe,separation_probe}.rs` (the unbudgeted
`walk`; `hidden_scalar`'s `cost = parts.len()` charge at both `node` and
`node_dag` entry; `hidden_root_values` making **one `walk` per root action**, so
the design's B/4B split is consistent; `void_free_kernel`'s
`voids: ContextSet::EMPTY` beside the maintained `voids_before_trick`). Every
checked number and claim agreed with its source. Rulings are numbered
**N4-A1..N4-A12**, one per design question in order.

**Question and ruling prefixes.** The design numbers its questions
N4-Q1..N4-Q12; the prefix is unused in this file (checked by grep at
adjudication time, as at authoring time) and is retained. Rulings use **N4-A**,
likewise unused and grep-checked; the ruling prefix mirrors the question prefix
so a reader searching a question lands beside its ruling — the SEP-A precedent.
E-A, E-Q, DS-A and SEP-A remain spent.

**One correction to both designs' inheritance headers, binding here and on the
economy successor.** Both headers recite "X-A1..X-A17, E-A1..E-A20,
S-A1..S-A18"; the families actually run X-A1..**X-A19**, E-A1..**E-A21**,
S-A1..**S-A21**. Standing rulings inherit **as whole families, by name, never
as ranges**; the full corpus binds regardless of a header's recited range, and
the recited ranges are typos in the designs, not scope declarations.

**Headline — four findings, stated before the rulings.**

1. **The design is ACCEPTED with amendments; the SEP-A10 repair spec is
   answered in full.** Ground (i) by §5's measured rung, ground (ii) by §3's
   budgeted-walk contract on every evaluator including the revealed path,
   ground (iii) by §6's three-tier regime. Nothing was found that rejects the
   rung.
2. **(R0) as designed contradicts itself.** It demands byte-identical
   reproduction of `separation_2026-08-13.txt` "except the wall-clock line",
   but that file's header carries freeze 37(h)'s sentence
   "hidden/revealed/price/policy_value carry no budget and no stop", which the
   freeze-44 refactor makes false of the code that regenerates it. A
   regenerated receipt must tell the truth at regeneration time. Repaired at
   N4-A10: exactly two enumerated permitted differences.
3. **The §5 timing rung's decimation constant g is "declared" nowhere.** The
   design fixes the pattern (i·g mod 34,650, gcd asserted in-run) and omits the
   value. Fixed at freeze 44(e): **g = 15,485,863**, a fresh prime —
   deliberately not 7,919, 104,729 or 1,299,709, which are freeze-25 constants
   of another track (the freeze-25 no-cross-wiring clause applies).
4. **One internal inconsistency, non-binding:** §4.3 derives the partition
   growth factor 185 (truncating division, 191,841,542 / 1,033,720) and §4.4
   calls it "the factor 186". Both are cost-model inputs licensing nothing; the
   results file prints the arithmetic it actually uses; the design document is
   the authors' record of what was proposed and stands as filed.

- **N4-A1 (N4-Q1: freeze 44 — ACCEPT-WITH-AMENDMENT; FREEZE 44 fixed below).**
  The §3 content is right and complete, with three clarifications.
  (i) **One unit, several traversals, named counts.** The walk-step and
  `hidden_scalar`'s particle-step are the **same unit** — one unit per
  (particle, node) visit, charged as the bag size at node entry before any
  child — and the identification is sound. What it licenses is bounded: counts
  of **different declared traversals** (the envelope walk, the revealed
  per-world walks, the scalar tree walk, the scalar dag walk) are different
  observables **in the same unit**; arithmetic across traversals — §4.1's
  derivation of B from the quoted `tree-v0` column included — is a
  **cost-model input** under DS-A32/DS-A33's typing and never an identity, a
  receipt, or a prediction. *[NARROWED 2026-08-14 by Lemma N: where two
  traversals are **exhibited** to visit the same nodes under the same charge
  rule — as the partition build and the envelope H walk are, being one call
  with one expansion — their counts are equal as a theorem, and a comparison of
  them is a traversal against itself. The fence stands entire for every pair
  without such an exhibit, §4.1's derivation of B included.]* Every printed count names its traversal. The only
  cross-run *assertion* on step counts is (R6), which compares a traversal's
  count with the same traversal's count (N4-A7).
  (ii) The no-partial-fold propagation rule is CONFIRMED as a correctness rule,
  for the design's stated reason — an interrupted fold bounds nothing in either
  direction, PG-A13's asymmetry being the precedent — and (C2) of errata §3.4
  is the reason no partial fiber sum survives `revealed_summary`'s `None`;
  §3.3 is bound verbatim.
  (iii) `revealed_summary`'s whole-call budget scope is CONFIRMED with the
  design's own argument (a per-world budget would let a call exhaust the budget
  |X| times over); the per-action walk-step subtotals are typed in SEP-A19(b)'s
  class and are never an information value, a width, a cost claim, or a DS-A2
  term.
  **FREEZE 44 — the walk-step unit and the budgeted-walk contract, frozen
  content.** **(a) Unit and charge rule:** one walk-step per (particle, node)
  visit; at each entry to `walk` the charge is `bag.len()`, taken before any
  child call — the same rule as `hidden_scalar`'s `cost = parts.len()`; one
  unit, per-traversal counts named by traversal. **(b) Contract:** `walk` takes
  `budget: &mut u64` and returns `Option<Envelope>`; charge-then-descend; on
  exhaustion, `None`; a `None` from any child propagates immediately and **no
  partial fold of any kind is retained**. The stop point is a function of
  (kernel, budget) alone. **(c)** The `Option` contract on all six evaluators
  of the design's §3.2 — `hidden_root_values`; `revealed_world_root_values` and
  `revealed_summary`; `InfoPartition::build`; `policy_value_receipt`; the
  probe's unmemoized argmax-recording extraction solve; and
  `action_values_dag`, already budgeted under freeze 26 and unchanged —
  `information_prices` composing the first two and returning `Option`, its
  assertions firing only on complete results. **(d)** `revealed_summary`: one
  budget for the whole call (all worlds × all root actions), decremented
  monotonically across both loops, never per-world and never per-action; on
  `None` all partial state is discarded, and the stop prints the coordinate
  identity, the action and world index reached, the steps charged and the
  declared budget — counts of the run, never statements about the coordinate;
  per-action walk-step subtotals printed as exact integers with SEP-A19(b)'s
  typing sentence. **(e) Constants:** B = 10,000,000,000 walk-steps per
  (coordinate, action) for each evaluator whose traversal is per action
  (`hidden_root_values`' per-action walks, `InfoPartition::build`, the
  extraction solve, `policy_value_receipt`); 4B whole-call for
  `revealed_summary`; P_max = 32,000,000 partition states per (coordinate,
  action), checked at each insertion, PG-A13 governing exceedance *[AMENDED
  2026-08-14 by N4-A16(vi) — FREEZE 44 v2, clause (e) only: P_max becomes
  **192,000,000** and is applied to the count-only pass's completed count
  before any map is allocated; the insertion check survives as a defensive
  stop and is never reported as a receipt. The rest of clause (e) is
  unchanged.]*; the §5
  rung's world sample: fiber indices (i·g mod 34,650) for i = 0..15 with
  **g = 15,485,863**, gcd(g, 34,650) = 1 asserted in-run, W = 1, selection as
  §5 declares (first coordinate h0, first root action ascending, never by
  result). **(f) Canonical unit order:** coordinates h0, h1, h2, h4, h5, h6,
  h8, h9, h12; within a coordinate, root actions ascending by domino index;
  36 units. **(g)** The reduced-rung fallback set and rule of N4-A12.
- **N4-A2 (N4-Q2: B is fixed now; the rung-derived alternative is REJECTED).**
  DS-A33(i)'s pattern governs selection rules for timing rungs, not budget
  ceilings, and a ceiling fixed by the rung it gates would let a measurement
  move its own gate — tuning in miniature, the thing F7 exists to forbid.
  B = 10,000,000,000 is declared now, from a quoted exploratory receipt with
  the margin stated; §4.1's two honesty clauses (ceiling-not-prediction; source
  named) are mandatory and printed. The §5 gate, not B's provenance, is what
  guards the pass.
- **N4-A3 (N4-Q3: Route C CONFIRMED; Route A stays closed; FREEZE 45 fixed
  below).** The decline of the seat-rotation transport is the DS-A15-correct
  move, and all three Route-C reasons stand independently; the decline of the
  freeze-36 key extension is SEP-A3(i)'s own ground correctly applied. Route A
  is not opened here: exhibiting the cyclic-rotation transport — a
  Lemma-E7-style value-order isomorphism plus a canonical ranking — is its own
  adjudication, and nothing in this rung needs it; a rung should not buy a key
  with a new theorem when provenance lines suffice. A later design that wants
  receipt-corpus coordinates inside the S6a index space brings the exhibit.
  **FREEZE 45 — the n = 4 coordinate identity, frozen content.** The printed
  and asserted identity is: grade = 4; declaration pip; the viewer's hand and
  the pool as canonical ascending-domino-index tile lists; the leader offset
  from focal, asserted **0**; |X| = 34,650 asserted against `kernel.count()`;
  the fiber enumeration order (freeze 7/23). The corpus hand id and trick
  number are printed as provenance only, on their own line, never as identity
  components. The kernel is rebuilt in-run from the printed identity and
  asserted equal to `void_free_kernel`'s. **No library entry is written at any
  n = 4 coordinate** (Route C); freeze 36's key is untouched.
- **N4-A4 (N4-Q4: M_max is a gate input, not a freeze; P_max is not derived
  from it).** M_max is a property of the machine, exactly as the design says,
  and walt-math fixes the **rule**, not the constant: M_max is declared by the
  run owner before the §5 rung, printed in the header beside P-A19's CPU
  model, core count and build profile, as provenance; a rung run without a
  declared M_max is not run. The gate uses it exactly as §5 declares — checked
  before the full pass and **never during it**; a mid-pass memory stop would be
  a load-relative stop, DS-A29(a)'s violation arriving through another door.
  P_max is NOT derived from M_max: P_max is a deterministic stop, a function of
  (kernel, cap) alone, and deriving it from a machine property would make the
  stop machine-relative. P_max stays as the freeze-44 declared constant; its
  provenance (an explicitly-labelled estimate) is printed and licenses nothing.
  *[SUPERSEDED IN PART 2026-08-14 by N4-A16: the estimate behind P_max v1 —
  24,825,150 states — is measured wrong, the §5 rung having exceeded 32,000,000
  states at (h0, first action). The **rule** stated here is untouched and binds
  the successor: P_max v2 is declared in an adjudication, never derived at run
  time from M_max, and the pass contains no memory-derived stop.]*
- **N4-A5 (N4-Q5: the trade is NOT taken in v1; the fallback and its
  compensating receipt are pre-declared here).** The design is right not to
  weaken a receipt for memory by default. If — and only if — the §5 rung shows
  the two-map form fails the gate *[CONDITION MET 2026-08-14, measured — the
  rung's first map alone exceeded the cap; the fallback is ACTIVATED at
  N4-A15, and its count-only partition pass, named below, is what measures the
  state count at O(1) memory]*, the accepted fallback is: pricing runs
  against the extraction map alone, and the SEP-A19 totality receipt's domain
  comparison is replaced by the **streaming set-digest receipt**: both passes —
  the count-only partition pass and the extraction — fold, per record key, the
  128-bit FNV-1a hash of the key's canonical byte encoding into a commutative
  exact accumulator (wrapping addition mod 2^128), and the run asserts both
  digests **and** both counts equal. Typing, mandatory wherever the receipt is
  cited: this is a **hash-level domain receipt** — strictly stronger than the
  cardinality comparison (two different equal-sized state sets fail it except
  under hash collision), strictly weaker than the held-map domain comparison,
  and in the same identity-by-hash class as freeze 1. The results file names
  the weakening in place. Taking the fallback with this receipt needs no
  further adjudication; anything weaker returns here.
- **N4-A6 (N4-Q6: the three-tier regime — ACCEPT, three clauses).**
  (i) Tier 2 is right: (R2) is correctly characterised as the receipt that
  survives the authority gate — it ties a third, structurally max-free code
  path to the envelope H through the independently written extraction — and
  the "VERDICT UNCROSSCHECKED" language is ACCEPTED verbatim, with one
  addition to the row: the L = Q^H equality at Tier 2 is asserted against the
  **envelope H only**, the sole authority at that coordinate, and the row says
  so.
  (ii) Tier 3's asymmetry is CONFIRMED — it is PG-A11-versus-PG-A13's
  asymmetry correctly transposed, and "a stop can complete a negative and can
  never complete a positive" is bound as the printed sentence. Made explicit:
  a NOT-SEPARATED **pair** verdict requires the whole primal pipeline at a⋆
  (partition, extraction, L walk) **and** that competitor's U to have
  completed. The Corollary E4.1(3) exact-negative sentence may be printed at
  Tiers 2 and 3, but its provenance there is an uncrosschecked H, so the tier
  language attaches to the exact-negative sentence too, not only to SEPARATED
  verdicts.
  (iii) Tier-2 rows are **carried with their label, never excluded and never
  silently included** — in the results file and in any wiki text derived from
  it. Dissents and caveats travel with results verbatim; excluding a computed
  outcome would be NO-RESCUE's violation in the other direction. What DS-A10
  authorised is a receipt set, not a speech ban: the row is outside that
  receipt set and says so, exactly as §6.2's language has it.
- **N4-A7 (N4-Q7: (R6) is lawful as typed, with a declared-cause clause).** A
  step-determinism check on an unchanged code path is a check on the runner
  and on DS-A29(a)–(b), and a mismatch with no declared cause is a
  load-invariance failure: stop-and-report, never a finding — CONFIRMED. Scope
  clause: (R6) compares only counts of a code path **unchanged since the
  quoted receipt** — the scalar authority is untouched by the freeze-44
  refactor, and (R0) is what proves the envelope-side refactor changed nothing
  it should not have. If a later adjudicated change to the scalar path
  intervenes before the rung runs, the (R6) comparison at the affected counts
  is void-with-cause and the file says so in place; it is the *undeclared*
  mismatch that stops the run. (R6) never checks a value.
- **N4-A8 (N4-Q8: keep all nine; the fence is sufficient, with one
  strengthening).** Exclusion of h2, h5 and h8 is REJECTED: all nine are
  equally lawful coordinates of the declared carrier, the divergence column is
  provenance, and h8 — the widest divergence — is precisely the instructive
  case for teaching the support-is-not-belief fence with a number attached. A
  separately-typed heading is REJECTED as redundant. The strengthening,
  mandatory: at h2, h5 and h8 the **verdict line itself** carries an inline
  marker — "real-deal fence applies: void-filtered fiber N of 34,650" — so
  that no quotation of a verdict row can detach it from the fence. §2.2's two
  grounds, the per-coordinate printed fence, and the licenses-nothing sentence
  on the void-filtered column are bound verbatim.
- **N4-A9 (N4-Q9: checkpointing — ACCEPT, two clauses).** The (coordinate,
  action) granularity, 36 units, and the prohibition of any sub-world
  checkpoint inside `revealed_summary` are CONFIRMED — the design's (C2)
  argument is exactly right, and DS-A30(v)'s no-partial-unit rule plus §3.2's
  determinism give DS-A30(vi) for free. Two clauses.
  (i) The per-coordinate authority-gate outcome carried denormalised on every
  unit record: ACCEPT, with a load-time consistency assertion — all loaded
  unit records of one coordinate must carry the same gate outcome; a
  disagreement is corruption, and the cache is discarded entire (freeze 41's
  discipline).
  (ii) **The shared-call clause.** `revealed_summary` (and the coordinate's
  scalar authority solve) spans a coordinate's four units in one call. A
  resumed run holding some but not all units of a coordinate re-runs the whole
  call under the same declared budget and asserts the loaded units' values
  equal the recomputed ones — an X-A17 re-run assertion, mandatory at every
  partially-resumed coordinate, in addition to the declared
  first-loaded-unit sample.
- **N4-A10 (N4-Q10: (R0) is a BLOCKING precondition, with its byte-identity
  contract repaired).** (R0) blocks: no n = 4 unit runs until it has passed;
  it is also printed with the n = 4 results. The repair of headline finding 2:
  the regenerated grade-3 file is permitted **exactly two** differences from
  `separation_2026-08-13.txt`, enumerated in advance — the wall-clock
  provenance line, and the freeze-37(h) budget-honesty header sentence, which
  is replaced by the freeze-44 form ("the `walk`-based evaluators carry
  declared budgets under freeze 44; in this run every declared budget was
  asserted non-binding and every residual asserted strictly positive"). Any
  other byte difference is stop-and-report — a defect in the refactor, never a
  finding (DS-A36's discipline). Freeze 37 itself is **not amended**: its (h)
  clause remains true of the grade-3 receipt as filed, and freeze 44 carries
  the budget contract from here on; the design's §9 header sentence says
  exactly this and is ACCEPTED. Additionally: (R0) asserts the candidate
  library file byte-identical after the re-run — its four entries are
  deterministic content under freezes 22–26/36–37, and a re-run that mutates
  the file (duplicate append included) is a defect to fix before (R0) can
  pass.
- **N4-A11 (N4-Q11: both halves).** The bridge is implemented once, as a
  function of the coordinate's declared grade — no grade literal appears
  anywhere in bridge code — **and** the probe asserts the substituted grade
  equals the coordinate identity's grade, exactly as §7 requires, because a
  correct function fed a wrong argument reproduces the defect the assertion
  exists to catch. The design's observation that a silently reused grade-3
  constant produces well-typed wrong numbers is correct and is why both halves
  are mandatory.
- **N4-A12 (N4-Q12: the fallback is GRANTED, conditioned; ruled together with
  N4-Q8 as the design asks).** Pre-declaring the reduced rung now is the only
  rule-shaped route; the design is right that a fallback selected after a gate
  failure would be selected by result. Binding: (a) a gate failure is filed
  first, as a result, whether or not the fallback runs — it is the measured
  cost model SEP-A10(i) said was missing (F7); (b) the fallback set is fixed
  now *[SUPERSEDED 2026-08-14 by N4-A19: this fallback's own gate was the
  per-unit wall threshold, retired at N4-A14, and it failed that arithmetic at
  h8. The route is the full pass over all nine coordinates under per-unit
  admission — a rule declared in advance, result-independent, and strictly
  containing {h6, h4, h8}, so this clause's protection is preserved a fortiori.
  Clause (a) is discharged: the rung is filed and stands.]* by the declared rule "the three cheapest in-scope coordinates by quoted
  S5h `tree-v0` steps, ascending": **{h6, h4, h8}** (1,855,419,966;
  2,442,873,158; 3,016,730,096 — re-verified against the receipt at
  adjudication time); (c) the fallback runs only if the §5 gate arithmetic
  passes for those three coordinates' units; a second gate failure is a return
  to this file, with no second fallback; (d) the results file's header labels
  the pass REDUCED RUNG, GATE FAILURE FILED; (e) the N4-Q8 interaction
  resolves by N4-A8: all nine stay in scope, so the fallback set needs no
  re-derivation, and h8 carries its inline fence marker in the fallback
  exactly as in the full pass; (f) the twelve fallback units run in the
  freeze-44 unit order restricted to the three coordinates.

**Freezes.** Freezes 1–43 are in force and restated unchanged; 38–40 remain
reserved and untouched — nothing in this design instantiates the gluing-cut
language, the circuit representation, or the reachable-belief family, so the
design's proposed numbering stands: **44** and **45** are fixed above as new
numbers. No number is reused.

**What must change in the design before it is built.** In order of severity:
the (R0) byte-identity contract gains its two enumerated permitted differences
and the library byte-check (N4-A10); the §5 rung's decimation constant is
g = 15,485,863 (freeze 44(e)); the Tier-2/Tier-3 clauses of N4-A6(i)–(ii) —
envelope-only provenance on the Tier-2 equality, tier language on the
exact-negative sentence, the pair-completion requirement made explicit; the
verdict-line fence markers at h2/h5/h8 (N4-A8); the partially-resumed-coordinate
re-run assertion and the gate-outcome consistency assertion (N4-A9); M_max
declared by the run owner before the rung, as provenance (N4-A4); the
reduced-rung fallback block (N4-A12); per-traversal naming of every printed
walk-step count (N4-A1(i)); and the inheritance-header ranges corrected to
whole families. Everything else is sound and is bound as written.

## The economy-successor adjudication (2026-08-13)

**Adjudicator:** walt-math. **Object:** `walt/ECONOMY-SUCCESSOR.md` (commit
42d83d9) — the successor SEP-A17 names: seed L from a source that is **not** an
exact solve, at coordinates where treatment H completes — together with the
reserved content of **freeze 46** and the requested re-entry of **freeze
36(e)**. **Tier:** exploratory throughout; nothing below is promoted, and no
number this run produces becomes quotable except by brief amendment adding it
to a verifier receipt. **Basis:** SEP-A1..SEP-A19 and everything they inherit;
the errata under DS-A17's citation rule (Lemma E3 and (C1)–(C4) of §3.4, Lemma
E4 and Non-theorem E4′ with DS-A27's semantic obligation, Corollary E4.1 —
pending its errata filing as §4.3 — Corollary E3.2, Lemma E7 at §8.3, Theorems
E6.3–E6.5); Lemma S-fold and Corollary R-fold with S-A2's declared reading; and
first-hand re-verification at adjudication time: the §1.2 slack table
re-derived to the last rational from `separation_2026-08-13.txt` (margins
449/1120 and 59/2240 at idx = 0; 1/42 and 1/63 at idx = 1299709; 0 and 1/21 at
idx = 2599418 for both tied a⋆; slacks therefore 59/2240, 1/63, 0 — every
entry agrees), `store/candidate_library.txt` (four entries, all `PipTrump(0)`,
the digest line as quoted), `policy_geometry_2026-08-12.txt` (k = 384 at
idx = 0 lead 00), `deadness_2026-08-12.txt` (384 classified, 276 tied, 50,328
forced at that coordinate and lead), commit ca2c178 (the `policy_inspect`
diagnostic, self-labelled exploratory, no results file), and
`predictive_rank.rs::coordinate` (the unranking arithmetic §2.1 recites,
verified; no ranking inverse exists yet — building one is part of this
design's build). Rulings are numbered **EC-A1..EC-A14**; EC-A1..EC-A13 answer
EC-Q1..EC-Q13 in order, and EC-A14 fixes the receipt set.

**Prefix.** EC-A, unused in this file, checked by grep at adjudication time;
it mirrors the design's EC-Q numbering per the SEP-A precedent. The
inheritance-header range correction recorded at the head of the n = 4 section
binds this design identically.

**Headline — five findings, stated before the rulings.**

1. **The central instrument is CONFIRMED and is the right one.** The
   economy-gap / certification-slack decomposition and the R8 identity are
   exact (EC-A3), the retyping of the primal side is correct (EC-A2), and the
   zero-slack control is correctly pre-declared as a theorem (EC-A10,
   renamed). The design is ACCEPTED with amendments.
2. **The T-diagnostic is REJECTED as typed, and the rejection is good news.**
   The S-A2 conditionality it proposes to measure is **form-level, not
   value-level**: no rule of the game ever reads the mutual order of tier-0
   tiles (S-A2's own soundness clause), so the declaration fold transports
   values for **all 49 ordered pairs** under either reading, and a value
   mismatch at any image is a defect, never a measurement. Delivered below as
   **Corollary S-fold-val**; the arm is retyped as receipts (EC-A4), and the
   claim to discharge S-A2's print-both obligation is withdrawn — that
   obligation belongs to the (parked) seat-census build and is untouched here.
3. **The freeze-46 arm list omits an arm its own reading requires.** §5.2's
   CERTIFIED-EXACT-ONLY compares the cheap arms against "the exact seed
   (recomputed in this run)", but the §5.1 run order contains no exact-seed
   arm. **Arm X** is added (EC-A1).
4. **P1's citation is corrected.** Freeze 26's tie rule selects among the
   **argmax**; P1 selects among **all legal tiles**. The shared object is the
   canonical ascending domino-index order, not the tie rule; "freeze 26 read
   as a global policy" would conflate a tie-break with a policy (EC-A1(a)).
5. **The 108/384 discrepancy is RESOLVED: 108 counts something real, at a
   tier below any receipt** — the strictly-mattering subset, 384 − 276
   (EC-A12). A DISAMBIGUATED pointer marker is placed at the SEP-A17 site per
   DS-A28(i).

### Corollary S-fold-val (value transport along the declaration fold is reading-independent) — delivered here

*Extends Lemma S-fold and Corollary R-fold; S-series mathematics, filed in
this record where the S-series lives.*

**Statement.** Let φ = φ_{p→p′} be the declaration transport of Lemma S-fold,
restricted to the live set of a rung coordinate as in Corollary R-fold. Then
for **every** ordered pair (p, p′) of pip declarations: the induced bijection
of fibers satisfies α_{Tρ}(Tξ) = α_ρ(ξ) for every lawful ρ and every ξ; hence
Q^H per corresponding action, every fixed-policy value L, and every
treatment-C value U_a correspond exactly along φ — **independently of which
S-A2 comparison reading is adopted**. The reading-dependence recorded at Lemma
S-fold ("orbits {0,6} and five singletons under the literal reading") is a
statement about transports of the recorded relational **form**; it has no
value-level content.

**Proof.** Dynamics read only legality (follow membership and the led-context
map), the double flag, trump membership, and the winner-determining
comparison; by S-A2's soundness clause the maximum trick key is always
attained at tier ≥ 1, so the mutual order of tier-0 tiles is read by no rule.
Lemma S-fold's preservation argument shows φ preserves every datum in that
list, for every ordered pair — the literal reading's counterexample (4:1
versus 3:2 in context 6 under δ = 0) concerns two tier-0 tiles, exactly the
relation dynamics never read. Corollary R-fold's proof then gives the
bijections of legal sets, uniform field masses, observations and count-free
increments with focal fixed, hence h_t(ξ) = h_{φt}(φξ) for every test and
α_{Tρ}(Tξ) = α_ρ(ξ). Values are exact maxima and expectations of
corresponding quantities under corresponding masses; treatment C's per-world
values correspond world-wise under the fiber bijection, and the uniform belief
pushes forward to the uniform belief. ∎

**What it licenses and what it does not.** It licenses R9's value equalities
as **receipts at every image** p′ ∈ {1..6}, and it is what makes arm T's
verdict transport lawful under Lemma E7 (the exhibited isomorphism, with
β′ = T_*β). It does **not** decide which reading the seat-census form
comparator realises — both readings predict identical values, so no value
observable can distinguish them — and it does not touch the form-level fold
factor (7:1 versus {0,6} plus singletons), which remains S-A2-conditional
exactly as Lemma S-fold records.

- **EC-A1 (EC-Q1: FREEZE 46 fixed, with arm X added and the list CLOSED).**
  **(a) Arms and exact definitions.** **X (exact control):** the H-argmax seed
  by freeze 36(f)'s rule, recomputed in-pass; its rows are receipts, not
  measurements — g = 0 by Corollary E4.1(2) — and they are what
  CERTIFIED-EXACT-ONLY compares against. **T (transport):** the four library
  entries transported by φ to p′ = 6, and the idx = 0 entry additionally to
  p′ ∈ {1..5}; every image row is a receipt under Corollary S-fold-val; g = 0
  by theorem; the §2.1 honesty headline mandatory. **P1 (least-tile):** the
  least legal tile by the canonical ascending domino-index order — cited to
  that order as a standing convention, NOT to freeze 26, whose tie rule
  selects among the argmax and is a different object. **P2 (greatest-tile):**
  the greatest legal tile by the same order. **P3 (beat-if-able):** if the
  viewer is not the trick leader and some legal tile strictly beats the best
  tile so far in the current trick under the declaration's winner-determining
  order, the least such tile; otherwise the least legal tile; the §2.2
  information-consistency argument printed with it. **P4 (trump-hoard):** the
  least legal non-trump if one exists, otherwise the least legal trump.
  **R (heuristic re-key):** the idx = 0 root-00 entry moved to each other base
  coordinate at each of that coordinate's H-optimal a⋆ by the
  **rank-within-live-set positional correspondence** — the root position of a
  target record maps to the target a⋆ by declaration, and each subsequent
  play maps to the tile of equal rank in the source coordinate's canonical
  ascending live-set order; if the mapped record is absent from the source
  entry, or the mapped choice is illegal at the target state, the fallback is
  P1 at that state; the number of fallback states is counted and printed as an
  exact integer beside L^R; the label HEURISTIC RE-KEY (NOT A TRANSPORT) on
  every row. **(b)** The transport φ and the image-key construction as §2.1
  (unrank; apply φ tilewise; re-rank by a ranking inverse of `unrank_comb`;
  assemble index′), with the round-trip receipts of R9. **(c) Canonical run
  order:** coordinates ascending by S6a index, each base coordinate before its
  images and images ascending by image index; within a coordinate, a⋆
  ascending by domino index; arms in the order **X, T(p′ = 6),
  T(p′ ∈ {1..5}, idx = 0 only), P1, P2, P3, P4, R**. Image coordinates carry
  arm-T rows and the R9 block only. **(d)** The results-file column set: per
  (coordinate, a⋆, arm) — L^seed, R7's economy gap g, the pairwise comparison
  against every competitor's U, the verdict, and the R2′/R5/R6 outcomes; R9 on
  transport rows; the §5.1 header items plus this section's amendments.
  **(e) CLOSED.** The rule family is frozen as written. An open arm list is
  not a freeze; a later arm is a freeze-46 v2 fixed by a later adjudication
  (numbers are never reused; content is versioned by ruling, as freeze 36 v2
  at EC-A8 demonstrates).
- **EC-A2 (EC-Q2: the retyping — ACCEPT; the header is arm-scoped; R2′ is a
  genuine receipt).** The REPLACEMENT — not amendment, not coexistence — of
  SEP-A2's header sentence is correct: a file printing both would assert L at
  and below its ceiling at once. One amendment: the replacement sentence is
  **arm-scoped** — as written it says "the primal witness at each seeded
  action is a NON-EXACT lawful policy", which is false of arms X and T.
  Binding form: the header sentence carries the clause "this applies to arms
  P1–P4 and R; arm X's rows are at the ceiling by Corollary E4.1(2) and carry
  SEP-A2's sentence as row typing; arm T's rows are exact relabellings and
  carry the §2.1 honesty headline." R2′'s promotion from by-construction
  equality to genuine receipt is CONFIRMED: with a non-exact seed, L ≤ Q^H
  can fail only through Non-theorem E4′'s inversion or a lawfulness defect,
  so the assertion now carries information and PG-A8 is satisfied; the
  three-defect stop message is ACCEPTED; NO-RESCUE binds. The complement: on
  arm-X rows the same assertion reverts to by-construction equality and is
  reported there as a receipt of the pipeline (R2's grade-3 sense), never as
  this run's measurement.
- **EC-A3 (EC-Q3: R8 CONFIRMED; the table verified; one no-clamp clause).**
  The identity — SEPARATED iff g(a⋆, seed) ≤ s(a⋆) — is correct as stated and
  proved; the one-line equivalence is exact, with non-strict inequalities
  matching Theorem E6.4's non-strict form. Reading s off the adjudicated
  run's margin column is correct because L = Q^H there makes every margin
  equal Q^H(a⋆) − U_a; the §1.2 table has been re-derived from
  `separation_2026-08-13.txt` and every entry is exact (slacks 59/2240, 1/63,
  0, as quoted in the basis line). Asserting the identity against the
  independently computed pairwise verdicts is the right receipt form — two
  computations, one stop on divergence. One clause: s(a⋆) ≥ 0 is a fact of
  these coordinates, not an invariant — the in-run recomputation must neither
  clamp s at zero nor assert s ≥ 0; at a coordinate with a failing pair,
  s < 0 is exactly Corollary E4.1(3)'s signature. g ≥ 0 per arm IS an
  invariant, and it is R2′.
- **EC-A4 (EC-Q4: REJECT the T-diagnostic typing; the arm is retyped as
  receipts; the S-A2 discharge claim is withdrawn).** By Corollary S-fold-val
  the R9 value equalities hold at every image p′ ∈ {1..6} regardless of which
  comparison reading `walt-core` realises. The design's premise — "under the
  literal reading these fail" — misreads the adjudication-time verification
  of Lemma S-fold, which counted transports of the recorded **form** (49
  under the operative reading, 9 under the literal), not value
  correspondences. No value observable distinguishes the readings, so the
  proposed measurement measures nothing. Retype, binding: all six images are
  receipt rows; a mismatch at any image is stop-and-report — a defect in
  `walt-core`'s rules, in the fold implementation, or in the key
  correspondence, never a finding about the game. The p′ ∈ {1..5} images of
  the idx = 0 entry are RETAINED as additional receipt coverage — they
  exercise non-adjacent order isomorphisms and are cheap at grade 3 —
  explicitly not a diagnostic. The claim that this run discharges S-A2's
  print-both obligation is WITHDRAWN: that obligation attaches to a run that
  computes the seat-side **form** (freeze 18's content), which nothing here
  does; it stays with the parked seat-census build. The design's instinct
  that its one non-stop-and-report assertion was "uncomfortable enough to
  want an explicit ruling" was exactly right; the discomfort marked a
  mistyping, and with the retype it disappears.
- **EC-A5 (EC-Q5: arm R RUNS, unconditionally, fenced, with the repair
  count).** Dropping is REJECTED — the arm is lawful, its g is a fact about
  the coordinate, and it is the branch's cheapest direct test of precisely
  the over-reading DS-A15 forbids: better measured under a fence than left to
  intuition. Running it **conditionally on P1–P4 failing** is also REJECTED,
  on discipline: a result-dependent arm list is selection by result, and the
  arm list is declared in advance or not at all. It runs always, with: the
  HEURISTIC RE-KEY (NOT A TRANSPORT) label on every row; the exact
  correspondence rule of freeze 46(a) — the design's "matching records
  positionally" is too loose to freeze; and the fallback-state count printed
  beside L^R, so every row is honest about how much of the re-keyed policy is
  actually P1.
- **EC-A6 (EC-Q6: the decline of arm B is CONFIRMED, with its until-clause
  recorded).** No lawful count-free surrogate exists: a bid's content is a
  points-and-marks claim; stating any tricks-implication of it requires the
  count feature law, which is Experiment G's declared cone and not lawfully
  designable yet (DS-A12); inventing a conversion here would be adopting a
  plausible reading under the ambiguity protocol. The decline is **until-G,
  not forever**: under DS-A16, once a count-bearing carrier exists with its
  own adjudication, a bid-derived seed becomes a designable arm there — a
  lawful policy is a valid primal witness under any valuation — and this
  paragraph is where "did walt ever try the contagion seed?" resolves:
  declined with reasons, 2026-08-13, pending Experiment G.
- **EC-A7 (EC-Q7: arm T is kept, as retyped).** The structure-proving
  disposition is CONFIRMED: the arm opens freeze 36(e) with the branch's only
  exhibited non-identity value-order isomorphism, proves the key
  correspondence and transport machinery in running code, and its g ≡ 0 by
  theorem is printed as the honesty headline requires. Deferral until "a
  non-isomorphic lawful correspondence exists" is REJECTED as incoherent for
  verdict transport — Lemma E7's hypotheses are what lawful verdict transport
  means, and the non-isomorphic case is arm R's, already covered under its
  fence. One dividend is struck from the design as filed: the S-A2 discharge
  (EC-A4).
- **EC-A8 (EC-Q8: freeze 36(e) re-entry GRANTED, scoped and NAMED; FREEZE 36
  v2).** Freeze 36(e) is amended by this ruling to: **"Transport: identity,
  and the declaration fold φ_{p→p′} of Lemma S-fold — image key computed by
  the freeze-46(b) construction, R9 receipts asserted in-run, values licensed
  by Corollary S-fold-val and verdict transport by Lemma E7 with β′ = T_*β.
  Any further transport re-enters with its own adjudication."** The class
  formulation — admitting "transports with an exhibited isomorphism" as a
  class — is REJECTED: a freeze is a constant, not a rule, and a class clause
  would delegate future adjudications to the freeze. Conditions: transported
  candidates are in-process objects; **no image entry is written to the
  library file in this run** — the file stays at its four entries, and
  writing image entries is a separate decision with its own identity and
  dedup questions; a transported policy is re-priced before anything is
  reported (36(d), X-A17, unchanged). A pointer marker is placed at the
  SEP-A4(e) site per DS-A28(i).
- **EC-A9 (EC-Q9: the carrier — ACCEPT).** The three base coordinates plus
  the transport images are right, for the design's four reasons; SEP-A17's
  checkability scope is honoured; the structural unavailability of outcome 3
  is printed in the header before any row, so its absence is never read as
  evidence about seeds. No coordinate exists in the branch where the exact
  seed failed to separate, so the alternative is unbuildable today; if the
  n = 4 rung produces failing pairs, those coordinates are the natural second
  carrier, ported by re-declaration exactly as §3 says.
- **EC-A10 (EC-Q10: the control typing — ACCEPT; the verdict name — REJECT
  and replace).** idx = 2599418 is correctly typed: s = 0 at both tied a⋆ is
  a quoted-and-recomputed fact, and "a seed separates there iff g = 0" is the
  R8 identity's specialisation, pre-declared as a theorem. The name
  "ZERO-SLACK COINCIDENCE" is REJECTED: "coincidence" is a probability word,
  and possible ≠ probable is a typed distinction this file does not blur —
  whether a least-tile rule attaining the optimum is *likely* is exactly the
  kind of claim no run here supports. Binding name: **ZERO-SLACK: SEED
  EXACTLY OPTIMAL (NOT ECONOMY)**, printed with the design's explanatory
  sentence amended to "the seed exactly attains the optimal value at this
  action; this is not economy" — attainment in value, which is what g = 0
  says; it does not say the seed is the H-argmax policy.
- **EC-A11 (EC-Q11: the wording is strong enough; the class-level object is a
  separate experiment).** The primal/upper asymmetry sentence is bound
  verbatim where §5.2 places it. CERTIFIED-EXACT-ONLY records the per-arm
  gaps and nothing else: a lower bound on achievable L over a declared
  candidate class is a class-capacity statement — a different object with its
  own definitional questions — and gesturing at it inside this verdict would
  blur candidate-failure with class-failure, the exact distinction the
  outcome exists to keep.
- **EC-A12 (EC-Q12: RESOLVED — "108" counts the strictly-mattering subset;
  retyped, with a DISAMBIGUATED marker at SEP-A17).** The arithmetic,
  verified against the files at adjudication time: the idx = 0 lead-00
  extraction has 50,712 states, of which 384 carry genuine two-tile choice
  (`separation_2026-08-13.txt`; S6b's k = 384 agrees); the S6c ground-truth
  classifier reports 384 classified and 276 tied under one deviation
  (`deadness_2026-08-12.txt`); 384 − 276 = **108** states where the choice
  strictly matters, and 50,328 forced + 384 = 50,712. The figure's provenance
  is commit ca2c178's `policy_inspect` diagnostic — self-labelled
  exploratory, below every tier, with no results file — which is why 108
  appears in no receipt. Ruling: (i) SEP-A17's phrase "the 108-decision
  playbook" is **DISAMBIGUATED, not corrected** — the pointer marker is
  placed at its site; (ii) binding description henceforth in any walt
  artifact: the entry is named by **384**, its receipt-backed free-decision
  count; where 108 is mentioned it is typed as *the strictly-mattering subset
  under the S6c one-deviation classifier — a derived difference of two
  measured counts (384 carrying the S6b/extraction free-state typing; 276
  carrying J-A10's classifier-denominator typing), inheriting both scope
  fences, not an independent measurement, present in no receipt*; (iii) 108
  becomes quotable as a result only by brief amendment adding it to a
  verifier receipt, like every other number here. The design's preference to
  have the discrepancy ruled rather than quietly using one number is the
  right instinct, and the answer is: use 384; 108 is real, lower-tier, and
  derived.
- **EC-A13 (EC-Q13: the scope sentence is right; the standing question is
  answered "the primal half").** Outcome 1's statement and fence are bound
  verbatim. From this design forward, "did walt ever test the economy claim?"
  is answered: **the primal half** — whether the *witness* at a⋆ must be an
  exact solve — is tested here, at coordinates where every claim is
  checkable; the **full** parent sentence, a solver that avoids exact solves,
  additionally requires the U side cheapened — a relaxation coarser than C,
  run down Theorem E6.5's ladder, Experiment D's territory with freeze 38
  still reserved — and remains untested. A results file or wiki sentence that
  says "the economy claim was tested" without the word "primal" has
  over-claimed.
- **EC-A14 (the receipt set, as amended).** R1, R3, R4 and R5 inherited
  unchanged: ACCEPT. R2′ per EC-A2. R6 (seed lawfulness and totality):
  ACCEPT — for arms P and R it is the contentful receipt exactly as argued,
  and PG-A8 is respected by the design's own note that arm-P totality *by
  construction* is not the receipt; the receipt is the assertion against
  `InfoPartition::build`'s state set. R7: ACCEPT as THE measurement, with
  §5.2's per-source column fences verbatim. R8 per EC-A3. R9: ACCEPT with one
  strengthening — the exhibited-isomorphism equalities assert **Q^H, L, and
  U_a per corresponding action** at source and image; U transports by
  Corollary S-fold-val's world-wise correspondence, and asserting it closes
  the loop on the upper witness at the images for free; value equalities
  only, never per-world byte-equality, exactly as §2.1's caveat states; every
  image row stops-and-reports on mismatch (EC-A4). The SEP-A12
  provenance-typing sentence as sharpened in §4's closing paragraph is
  ACCEPTED verbatim.

**Freezes.** Freezes 1–45 are in force (44 and 45 fixed in the preceding
section); 38–40 remain reserved and untouched — nothing here instantiates
their reserved content. **46** is fixed at EC-A1 as a new number; **36** is
amended to v2 at EC-A8, its number unchanged and its v1 text preserved at
SEP-A4 with a pointer marker. No number is reused.

**Standing obligation, carried forward explicitly.** Corollary E4.1's filing
as errata §4.3 (DS-A28(ii)) remains due at the next errata amendment. This
adjudication amends no errata content and does not discharge it; both designs'
citations "pending its errata filing" remain accurate, and the durable form
remains the statement and proof at the head of the Experiment E section.
Corollary S-fold-val, delivered above, is S-series mathematics and lives in
this record with its series — it corrects nothing in the decision-sparse
corpus and creates no errata obligation.

**What must change in the design before it is built.** In order of severity:
the T-diagnostic sub-arm is retyped as receipts and the S-A2-discharge claim
removed (EC-A4); arm X is added to the arm list and run order (EC-A1); the
replaced header sentence is arm-scoped (EC-A2); the zero-slack verdict is
renamed (EC-A10); P1's definition cites the canonical order, not freeze 26
(EC-A1(a)); arm R's correspondence rule is the freeze-46(a) rank-within-live-set
rule with the fallback count printed (EC-A1(a), EC-A5); R9 gains the U-column
equalities (EC-A14); no image library entries are written (EC-A8); the in-run
slack recomputation carries no s ≥ 0 assumption (EC-A3); and the
inheritance-header ranges are corrected to whole families (the n = 4 section's
correction binds here). Everything else is sound and is bound as written.

## The n = 4 rung return: the overnight pass (2026-08-14)

**Adjudicator:** walt-math. **Object:** the N4-A12(c) return —
`walt/walt-factory/results/separation_n4_rung_2026-08-14.txt` (the §5 measured
rung, filed NO-GO, run after the (R0) blocking regression passed at commit
c4ae306) together with the run owner's changed declarations of 2026-08-14: the
machine given over for the night, M_max = 40 GiB standing, and W-parallelism
across coordinates offered. **Tier:** exploratory throughout. Nothing below is
promoted; no number this section or the pass it authorises produces becomes
quotable except by brief amendment adding it to a verifier receipt. **Basis:**
N4-A1..N4-A12 and everything they inherit; DS-A29..DS-A36; SEP-A10..SEP-A19;
EC-A1..EC-A14; F7, R-A18, PG-A8, PG-A13, P-A19, P-A21, X-A17; and first-hand
reading, at adjudication time, of `walt-strat/src/info.rs` (`walk`,
`root_bag`, `InfoPartition::build`, `policy_value_receipt`), `hidden.rs`
(`hidden_root_values`), `revealed.rs` (`revealed_world_root_values`,
`revealed_summary`), `hidden_scalar.rs` (`action_values`, `node`, `node_dag`)
and `walt-factory/examples/separation_probe.rs` (`n4_rung_main`,
`n4_compute_coordinate`), plus re-verification of the `tree-v0` column of
`walt-factory/results/fiber_probe_h_2026-08-11.txt`. Rulings are numbered
**N4-A13..N4-A20**, continuing the family. **Lemma N** is delivered below.
**Freeze 44 is amended to v2**, clause (e) only, at N4-A16.

**Headline — five findings, stated before the rulings.**

1. **The filed rung's partition line is misread, and the number in it is an
   artifact.** The stop was the cap, as the return says — but
   `InfoPartition::build` implements the cap by *poisoning the budget cell*
   (`cell.set(0)`), so the printed "walk-steps 10000000000" is the poison, not
   a measurement. The steps the partition traversal actually charged before the
   cap are destroyed by the stop and are unknown. N4-A13.
2. **The rung's unit-wall estimate estimates nothing.** `est h0 unit wall
   800884 ms` is the revealed extrapolation plus the wall of a *truncated*
   partition build, plus zero for an extraction and an L walk that never ran.
   It is neither an estimate nor a bound of a completed unit, and the h9 figure
   scaled from it inherits the defect. N4-A13.
3. **Lemma N: the pooled traversals and the revealed traversal are one cost,
   and that cost is already measured exactly for all nine coordinates.** The
   partition build and the envelope H walk are the same traversal; a pooled
   walk's charge equals the sum over the fiber of the one-world revealed
   charges. The 16-world extrapolation is therefore a sample of a quantity the
   S5h `tree-v0` column measures exactly: 1,855,419,966 (h6) to 16,211,488,002
   (h9), sum 56,631,363,840. **Every step budget in freeze 44(e) is met, at
   every in-scope coordinate, from a quoted receipt rather than a sample.**
4. **The ten-minute wall threshold was the corpus's only load-relative gate on
   content, and the run owner's declaration retires it with nothing
   mathematical lost.** N4-A4 had already ruled that a mid-pass memory stop
   would be "DS-A29(a)'s violation arriving through another door"; a wall
   threshold that decides whether a unit is computed at all is the same door.
   Wall-clock is provenance, it is compared against the run owner's declared
   budget, and it gates no content. N4-A14.
5. **The binding resource is the partition state count, and the instrument that
   measures it is already adjudicated.** N4-A5's fallback — whose activating
   condition ("the §5 rung shows the two-map form fails the gate") the rung has
   now met — contains a **count-only partition pass**. Because `walk` is a tree
   walk whose focal callback fires once per observation record, that pass
   yields the exact state count at O(1) memory, before any map is allocated.
   The cap becomes an admission threshold on a measured count rather than a
   truncation, and PG-A13's "no bound from a stop" worry disappears with it.
   N4-A15, N4-A16.

### Lemma N (the pooled-cost decomposition) — delivered here

Fix a kernel `K` with fiber `X`, `|X| = N`, and a root action `a`; charges are
freeze 44(a)'s walk-steps (`bag.len()` at each `walk` entry, before any child).
Write `T_a` for the traversal `info::walk` performs from `root_bag(K, a)` at
`k = 1` under the identity expansion `|_, legal, _| legal`, and `T_a^ω` for the
same traversal from the one-particle bag of world `ω`. Write `c(·)` for total
charge.

**(a) The partition build and the envelope H walk are the same traversal.**
`InfoPartition::build(K, a, ·)` and `hidden_root_values`' walk at `(K, a)` call
`info::walk` with the same `WalkCtx`, the same `root_bag(K, a)`, the same
`root_tiles(a)`, the same `k = 1`, and an expansion that returns `legal`
unchanged at every focal state; the partition build's callback differs only in
side effects — recording, and the cap poison — which do not enter the set it
returns. Hence, absent a cap stop, `c(partition build) = c(envelope H walk)`
**exactly**, at every `(coordinate, action)`. This is an identity between two
counts of one traversal, not arithmetic across traversals, and it is assertable
in-run by comparing residuals.

**(b) `c(T_a) = Σ_{ω ∈ X} c(T_a^ω)`.** Both sides count the pairs `(n, ω)` with
`n` a node of `T_a` and `ω ∈ bag(n)`: the left because the charge at `n` is
`|bag(n)|`, the right because every node of `T_a^ω` carries one particle and so
charges 1. It suffices to exhibit, for each `ω`, a bijection between
`{n ∈ T_a : ω ∈ bag(n)}` and the nodes of `T_a^ω`. Nodes of either traversal are
labelled by their play prefix and the label determines the node, so it is enough
that the label sets coincide; induct on depth. Both start at the label `(a)`. At
a focal seat the expansion is `legal_plays(decl, hand, led)` with `hand` the
viewer's remaining hand, which is constant on any bag by construction of the
fiber (`walk` asserts it) and equals `ω`'s, so both branch over the same set and
`ω` survives into every child. At a field seat `T_a` branches over the union of
the bag's legal sets and retains in child `d` exactly the particles whose legal
set contains `d`, while `T_a^ω` branches over `ω`'s legal set alone; so `ω`
survives into child `d` of a node containing `ω` iff `d ∈ legal(ω)`, which is
exactly `T_a^ω`'s branch set at that label. At a completed trick (`k = 4`) both
recurse on the whole bag with the same winner, a function of the tiles and the
declaration. No node has an empty bag, since every `d` in a union is legal for
some particle. ∎

**(c)** Summing (b) over the viewer's root actions: `Σ_a c(T_a)` is the
whole-fiber revealed charge — the quantity `revealed_summary` consumes over its
whole call, and the quantity the §5 rung extrapolated from 16 worlds.

**Remark N(d) — the scalar correspondence, exhibited but asserted, never
assumed.** `ScalarHidden::node` and `info::walk` were read side by side at
adjudication time: both charge `parts.len()`/`bag.len()` at entry before any
child; both branch a focal seat over the full `legal_plays` set and a field seat
over the union of the bag's legal sets with the same particle filter; both
recurse on the whole bag at `k = 4` under the same winner rule; and
`ScalarHidden::action_values` builds the same post-action bag of `|X|` particles
and enters at `k = 1`, exactly as `root_bag` does. On that exhibit the `tree-v0`
column of `fiber_probe_h_2026-08-11.txt` — itself a derived count, the tree cost
the dag solver propagates through its boundary hits — is `Σ_a c(T_a)` at that
coordinate. **The exhibit licenses a comparison, not an assumption:** the pass
prints its exact whole-fiber revealed total per coordinate and asserts it equal
to the quoted `tree-v0` (receipt (R7), N4-A18). This is not vacuous in PG-A8's
sense — it checks two independently written solvers against each other *and*
checks the dag solver's tree-cost propagation against a real unmemoized walk —
and a mismatch is a declared-cause stop in (R6)'s class (N4-A7), never a finding
about the game.

**Corollary N-1 (every freeze-44(e) step budget is met, exactly, from a quoted
receipt).** The nine in-scope `tree-v0` values are h6 1,855,419,966; h4
2,442,873,158; h8 3,016,730,096; h12 3,666,808,044; h0 3,727,724,856; h2
3,918,922,312; h5 6,305,108,794; h1 15,486,288,612; h9 16,211,488,002 —
**sum 56,631,363,840**. Each is the whole-call revealed charge at its
coordinate, so each is compared with `4B = 40,000,000,000`: the largest, h9,
passes with a factor of **2.467**. Each coordinate's per-action average,
`tree-v0 / 4`, is compared with `B = 10,000,000,000`: the largest, h9's
4,052,872,001, passes with the same factor 2.467. **What is not licensed:** `B`
binds a *single action*, and only the coordinate average is known, so `B` still
binds at h9 or h1 if one action there takes more than 2.467× its coordinate's
average. That stays a measured question — the per-unit residual is printed and
a stop is declared under freeze 44(b)–(d) — and is not assumed away here.

**Corollary N-2 (the rung's U-side number is superseded as evidence, and its
error is diagnosed).** The rung's 4,327,256,587 is a 16-draw extrapolation of
h0's exact 3,727,724,856, an overshoot of 16.1 %; the sample mean 124,884.75
against the exact mean ≈ 107,582 is well inside sampling error for 16 draws from
a population whose observed spread is 41,727..321,206. It is neither evidence
against Remark N(d) nor needed any longer: the exact column replaces it for all
nine coordinates. The rung's own typing sentence already said its numbers are
cost-model inputs; this corollary retires one of them in favour of a better
measurement of the same quantity, and promotes nothing.

**Corollary N-3 (what is left binding).** With every step budget met by
Corollary N-1 and wall-clock retired as a gate by N4-A14, the only resource that
can still bar a unit is the **partition state count** — and the count-only pass
of N4-A15 measures it exactly at O(1) memory. The n = 4 rung's cost question is
therefore closed except for one number per unit, and that number is measured
inside the unit rather than estimated in advance.

- **N4-A13 (the filed rung, as read: two corrections and one instrument defect;
  the NO-GO itself STANDS).** The gate failure is a result and stays filed (F7);
  nothing here rescues it and no number in it is promoted. Three corrections to
  how it is read, all binding on any citation of that file.
  (i) **The partition line's step count is a poison artifact, not a
  measurement.** `InfoPartition::build` stops the walk on cap exceedance by
  setting the budget cell to 0, so the caller's `B_WALK - pb` reads
  10,000,000,000 whatever the traversal had actually charged. The printed figure
  is not an exact deterministic observable and must never be quoted as one; the
  true charge at that stop is **unknown and unrecoverable from the run**. The
  return's diagnosis is otherwise CONFIRMED: `cap_hit` is set only in the
  callback's `legal_by_id.len() >= state_cap` branch, so the stop was the cap
  and not the budget.
  (ii) **The instrument is repaired, not reinterpreted.** The build records the
  residual *before* poisoning and prints, at a cap stop, both the charge
  actually consumed and the sentence "cap stop: budget residual poisoned; the
  printed charge is the charge to the cap." A stop must print counts of the run
  (freeze 44(d)); a stop that prints a constant instead is a defect in the
  instrument, in DS-A36's stop-and-report class, and is fixed before the pass.
  (iii) **`est h0 unit wall` estimates nothing.** It sums the revealed
  extrapolation, the wall of a *truncated* partition build, and zero for an
  extraction and an L walk that never ran. It is not an estimate of a completed
  unit and not a bound in either direction, and the h9 figure scaled from it
  inherits the defect entire. Both figures are struck as cost-model inputs;
  N4-A14 replaces what they were for.
  (iv) The rung's remaining content stands: the 16 per-world revealed charges
  and their spread are exact observables of a completed traversal; the resident
  size is load-relative provenance; the typing paragraph is correct as printed.
- **N4-A14 (R1: the per-unit wall threshold is RE-DECLARED as a run-owner gate
  input, and wall-clock is retired as a gate on content).** GRANTED, and on a
  ground stronger than the run owner's convenience.
  (i) **The constant was never frozen.** Freeze 44 contains B, 4B, P_max, g and
  the rung's sample; the ten minutes appear only in SEP-A10(i)'s citation of the
  design's own rule and in §5's gate text. Nothing is being unfrozen.
  (ii) **Its type was wrong.** A threshold in wall-clock that decides whether a
  unit is computed makes the *set of computed units* a function of machine load
  — the same defect N4-A4 named when it forbade M_max from being checked during
  a pass ("a mid-pass memory stop would be a load-relative stop, DS-A29(a)'s
  violation arriving through another door"). DS-A32 and DS-A31(iii)–(iv) already
  hold that no wall-clock figure in this corpus is quotable as a measurement. A
  quantity quotable as nothing may not gate anything. **Binding: wall-clock
  gates no content anywhere in the n = 4 pass.**
  (iii) **What replaces it.** A whole-pass wall budget **T_pass**, declared by
  the run owner before the pass, typed exactly as M_max is by N4-A4: printed in
  the header beside M_max, W, and P-A19's CPU model, core count and build
  profile; **provenance, never a freeze**; and *a pass run without a declared
  T_pass is not run* (the M_MAX_GIB precedent — the build invents no default).
  T_pass is compared against a printed estimate **before** the pass and never
  during it. **Exceeding T_pass is not a bar.** The pass is checkpointed at
  (coordinate, action) granularity under DS-A30 and freeze 41/42; a run that
  does not finish in one night resumes in canonical unit order, and the run
  owner may kill it at any instant. A killed run is not a declared stop, states
  nothing about the game, and loses at most one unit's work.
  (iv) **The roster is provenance, the content is not.** Which units a given
  night completed is load-relative and licenses nothing — in particular the
  absence of a unit is never read as a property of that coordinate. Each
  completed unit's content is a function of (kernel, freeze-44 budgets, P_max)
  alone and is byte-identical across fresh, resumed and any W (DS-A36).
  (v) **Per-unit wall is printed as provenance**, per DS-A31(iv) with the
  identity of the producing process, and no per-unit wall figure gates, stops or
  is compared against any other arm.
  (vi) **The exact gate form the pass must print** is fixed at N4-A18.
- **N4-A15 (R2, first half: N4-A5's fallback is ACTIVATED, and its count-only
  pass is the instrument the cap question needed).**
  (i) **The activating condition is met, measured.** N4-A5 pre-adjudicated the
  fallback "if — and only if — the §5 rung shows the two-map form fails the
  gate". The rung shows the *first* of the two maps alone exceeding the cap. The
  condition is met a fortiori, the fallback is taken, and N4-A5's own sentence
  governs: "Taking the fallback with this receipt needs no further
  adjudication." Its typing paragraph is mandatory wherever the receipt is
  cited, verbatim, including that the digest receipt is **strictly weaker** than
  the held-map domain comparison it replaces, and the results file names the
  weakening in place.
  (ii) **The build's reading is CONFIRMED and sharpened.** The digest fallback
  cannot lower a unit's state COUNT below any cap and therefore does not by
  itself rescue a `> P_max` unit — correct. What it does buy is stated exactly,
  because it is what makes the pass affordable: it removes the
  `BTreeMap<Vec<Domino>, InfoStateId>` index and the per-state `legal` and
  `nodes` vectors from residence, leaving one map over the record key space, so
  it roughly **halves the resident bytes per state** and thereby raises the
  count a given M_max admits. It is a memory instrument, not a cap instrument.
  (iii) **The count-only pass yields the exact count at O(1) memory, and this is
  the finding that changes the shape of the problem.** `info::walk` is a tree
  walk whose node label *is* its play prefix; the focal callback therefore fires
  exactly once per observation record (`InfoPartition::build`'s
  `assert!(prev.is_none(), ...)` is that property, checked). A pass that only
  increments a counter in the callback therefore returns the **exact** partition
  state count while holding nothing. Two honesty clauses: the count-only pass
  **loses the `prev.is_none()` dedup check** — uniqueness there rests on the
  by-construction argument just given, which is stated in one printed sentence
  beside the count — and the count-only pass and the map build are the same
  traversal by Lemma N(a), so where both run the run asserts their counts equal
  and types that as **a code-level equality check, not a receipt in PG-A8's
  sense**.
  (iv) The count-only pass carries its own budget `B` under freeze 44(b)–(c) and
  its own declared stop; if it exhausts `B` the unit reports a budget stop with
  no count and no bound, and Corollary N-1 says that outcome would itself be
  news.
- **N4-A16 (R2, second half: P_max v2 = 192,000,000, with the arithmetic; FREEZE
  44 v2, clause (e) only).** P_max is raised, and its role is changed from a
  truncation to an admission threshold on a measured count.
  (i) **The memory arithmetic, printed with the constant.** The only measured
  point is the rung's post-discard resident size, 2,797,840 KiB against
  32,000,000 states: **≈ 89.5 bytes per state, gross of the process base**, for
  the index map plus the `legal` and `nodes` vectors. It is load-relative
  provenance and licenses nothing on its own; it is used here exactly as N4-A4
  permits P_max's provenance to be used — as an explicitly-labelled estimate
  behind a declared constant. Structurally it agrees with the object: a
  `Vec<Domino>` key is 24 bytes in-node over a ≤ 32-byte heap record, a
  `BTreeMap` pair carries its node's fill factor, and the two side vectors add
  12. Under the N4-A15 fallback the resident form is one map with a 2-byte
  value and no side vectors, ≈ 77 bytes per state by the same arithmetic.
  **Declared figure: 128 bytes per state**, a 1.66× margin over the arithmetic.
  (ii) **The constant.** **P_max v2 = 192,000,000 partition states per
  (coordinate, action).** Arithmetic, printed: 192,000,000 × 128 B = 24,576,000,000
  bytes = 22.89 GiB ≤ **M_budget = 24 GiB**, the declared aggregate residence for
  pricing structures; and at 192 bytes per state — two and a half times the
  arithmetic figure — a single such unit would still occupy 34.3 GiB, inside
  M_max = 40 GiB. The margin is deliberate and is not an estimate of anything.
  M_budget, like M_max and T_pass, is a run-owner gate input: provenance, never
  a freeze.
  (iii) **N4-A4's rule is untouched.** P_max v2 is **declared here, in this
  adjudication**, as a constant; it is not computed at run time from M_max, and
  the pass contains no memory-derived stop. The stop remains a function of
  (kernel, cap) alone. A build that reads M_max and derives a cap from it is in
  breach of N4-A4 and stops.
  (iv) **What a unit whose count exceeds the cap reports, and why it is more
  than PG-A13 could give before.** The unit is **NOT PRICED**: no L, no
  separation row, no verdict, no partial partition — PG-A13 entire. But its
  **exact state count is printed**, because that count comes from a *completed*
  count-only traversal and not from a truncation; PG-A13 forbids a count read
  off a stop, and there is no longer one. The line reads "partition states N
  (count-only pass, COMPLETED); NOT PRICED — N > P_max v2 = 192,000,000; no
  verdict, PG-A13" and is a **result** under F7, in the deterministic block, in
  canonical order. Typing, printed beside it: the count is an exact
  computational observable of the declared traversal in SEP-A19(b)'s class —
  never an information value, a decision width, a cost claim, or a DS-A2 term.
  (v) **The insertion check survives as a defensive stop, never a receipt.**
  With admission made on the measured count, an insertion-time cap can no longer
  fire on a correct run; it is retained against coding error, is **not reported
  as a receipt** (PG-A8), and if it ever fires the run stops and reports a
  defect.
  (vi) **FREEZE 44 v2 — clause (e) only.** `P_max` becomes **192,000,000**
  partition states per (coordinate, action), applied to the count-only pass's
  completed count **before any map is allocated**, with the insertion check
  retained as (v)'s defensive stop. Every other constant of clause (e) is
  unchanged and restated: B = 10,000,000,000; 4B whole-call for
  `revealed_summary`; g = 15,485,863 with `gcd(g, 34650) = 1` asserted; the §5
  rung's sample and its W = 1. Clauses (a)–(d), (f) and (g) of freeze 44 are
  unchanged. The number 44 is not reused and v1's text stands at N4-A1 with a
  pointer marker — the freeze-36 → v2 pattern (EC-A8), cited as the precedent.
- **N4-A17 (R3: W-parallelism ACROSS COORDINATES is lawful, with a coordinate
  claim, an admission rule, and four assertions the run must add).** LAWFUL
  under DS-A29(a)–(d), DS-A34, DS-A35 and N4-A9 as built. The N4-A9(ii)
  shared-call clause binds *within* a coordinate — `revealed_summary` and the
  scalar authority solve span that coordinate's four units in one call — and
  coordinates share no state: different kernels, per-call caches, no clock, no
  RNG, exact rationals throughout. Binding:
  (a) **One coordinate, one worker, for its whole life.** A coordinate is
  claimed by exactly one worker until all its units are written. Splitting a
  coordinate across workers would either duplicate the shared call or let two
  workers write one unit record — DS-A35's torn-write ground arriving through a
  new door. Asserted, not assumed.
  (b) **Admission by measured count, throttled by waiting and never by
  skipping.** Before allocating any map, a worker holds its unit's count-only
  result and requests admission for `count × 128 B` against M_budget = 24 GiB;
  it **waits** until the budget admits it. A unit is never skipped, deferred out
  of the run, or reordered out of the deterministic block for memory reasons —
  waiting changes execution order only, and order is not content (DS-A36,
  freeze 42). A unit whose count exceeds P_max v2 is not "denied admission": it
  is N4-A16(iv)'s measured stop, decided by the count alone.
  (c) **The concurrency arithmetic, printed before the pass:** `W × 1 GiB`
  (declared per-worker U-side working-set allowance — the revealed pass's
  per-world envelope accumulation dominates it) `+ M_budget (24 GiB) ≤ M_max −
  4 GiB`, giving **W ≤ 12** at M_max = 40 GiB. W is run-owner declared within
  that bound, **recorded, not frozen** (DS-A34), printed with CPU model, core
  count and build profile (P-A19).
  (d) **Four assertions the run must add**, beyond N4-A9's two: (1) no unit
  record contains a worker id, a thread id, or any timing field — a record that
  did would make the deterministic block W-dependent; (2) every timing quantity
  carries the identity of the process that produced it (DS-A31(iv)) and every
  timing line under `W ≥ 2` is labelled `CONTENDED(W=n)` with DS-A32's sentence,
  which here bites nothing because the pass forms no ratio and quotes no
  dividend; (3) the deterministic block is assembled at the end, in canonical
  unit order, from records — never in completion order; (4) DS-A36's cheap
  validation is run once for this pass: one coordinate fresh, the same
  coordinate resumed from checkpoints, deterministic blocks byte-compared, a
  difference being stop-and-report.
  (e) DS-A35 stands: W threads in **one process**, never multi-process copies
  over one checkpoint directory.
- **N4-A18 (R4: no third rung is required; the gate arithmetic re-runs inside
  the pass, and here is its exact printed form).** The pass may proceed on the
  filed rung as corrected by N4-A13, plus these amendments. The reasoning is
  that every quantity the old gate estimated is now either exact from a quoted
  receipt (the step budgets, Corollary N-1), retired as a gate (wall-clock,
  N4-A14), or measured inside the unit that needs it (the state count, N4-A15).
  A rung whose only remaining job is to estimate what the pass measures is not
  worth a night. Binding:
  (i) **Per coordinate, in canonical order (freeze 44(f)):** the scalar
  authority and the tier fix (§6); the envelope H and the revealed pass, which
  together determine the H-optimal actions; then, **at H-optimal actions only**
  — the primal pipeline runs nowhere else, so the memory-heavy work is a handful
  of units, not 36 — the count-only pass, the admission decision, and, if
  admitted, the extraction and the L walk under the N4-A15 fallback.
  (ii) **This is not N4-A2's forbidden pattern.** N4-A2 forbids a measurement
  fixing the budget that gates it. P_max v2 is declared in this file, in
  advance, by an adjudicator; the count-only pass measures a coordinate property
  and compares it against that constant. A declared gate applied to a
  measurement is what a gate is.
  (iii) **The gate arithmetic, in the exact form the pass prints**, under a
  heading naming it a cost-model input licensing nothing:

  ```
  GATE ARITHMETIC (cost-model inputs, licensing nothing; N4-A14/A16/A17)
  run-owner gate inputs (provenance, never freezes): M_max = <G> GiB;
      M_budget = 24 GiB; T_pass = <H> h; W = <n> (recorded, not frozen)
  [A] step budgets, EXACT from a quoted receipt (Lemma N, Corollary N-1):
      whole-fiber revealed charge at <coord> = tree-v0 = <exact> vs 4B = 40,000,000,000
      per-action coordinate average = tree-v0/4 = <exact> vs B = 10,000,000,000
      caveat printed in place: only the coordinate average is known in advance;
      B binds a single action, and the per-unit residual is measured
  [B] memory admission, constants only, checked BEFORE the pass and never
      during it (N4-A4): P_max v2 = 192,000,000 states; declared 128 bytes/state;
      192,000,000 x 128 B = 22.89 GiB <= M_budget = 24 GiB;
      W x 1 GiB + M_budget <= M_max - 4 GiB  =>  W <= 12
  [C] per-unit admission, deterministic, a function of (kernel, P_max v2)
      alone: count-only states N(unit) <= P_max v2; and
      sum of N x 128 B over concurrently pricing units <= M_budget (wait, never skip)
  [D] whole-pass wall estimate, provenance, GATES NOTHING: T_est = <..> h vs
      T_pass = <..> h. "Wall-clock gates no content (N4-A14). Exceeding T_pass
      is not a bar: the pass is checkpointed at (coordinate, action) granularity
      and resumes in canonical unit order. Which units a night completed is
      load-relative provenance and licenses nothing."
  ```

  (iv) **Receipt (R7), new, mandatory per coordinate:** the exact whole-fiber
  revealed charge asserted equal to the quoted `tree-v0` value for that
  coordinate. Licensed as a same-traversal comparison by Lemma N(b)–(c) and
  Remark N(d); non-vacuous under PG-A8 for the two reasons Remark N(d) gives; a
  mismatch is declared-cause stop-and-report in (R6)'s class (N4-A7), never a
  finding about the game. **Receipt (R8), new, per priced unit:** the count-only
  count asserted equal to the built map's `len()` — typed, in place, as a
  code-level equality check and **not** a receipt in PG-A8's sense (N4-A15(iii)).
  (v) **Cost-model inputs the pass may print for the run owner**, each labelled
  an estimate and licensing nothing, all derived from the rung's own provenance
  and the quoted column: one-particle charge rate ≈ 5.83 × 10⁶ steps/s
  (1,998,156 steps in 343 ms); revealed side over all nine coordinates ≈ Σ
  tree-v0 = 5.66 × 10¹⁰ charges ≈ 2.7 h at W = 1; map insertion ≤ 1.82 µs/state
  (32,000,000 insertions in 58,075 ms). P-A21 is printed beside them: three
  rungs are not a law and no growth rate measured at grades ≤ 4 is quoted for
  the opening.
- **N4-A19 (N4-A12's reduced-rung fallback is SUPERSEDED by a strictly more
  inclusive rule; no second fallback, and no selection by result).** The
  fallback set {h6, h4, h8} was pre-declared to keep a post-failure choice from
  being made by result, and it failed its own arithmetic at h8 (≈ 648 s against
  600 s) — against a threshold N4-A14 has now retired. Ruling: **the route is
  the full pass over all nine in-scope coordinates under per-unit admission.**
  N4-A12's protection is preserved a fortiori: the new rule is declared here,
  before any further number exists; it is a rule, not a selection; it is
  result-independent; and it strictly contains {h6, h4, h8}. N4-A12(a) (a gate
  failure is filed first, as a result) is discharged — the rung is filed and
  stands. N4-A12(d)'s "REDUCED RUNG" header is not used; N4-A8's inline
  real-deal fence markers at h2, h5 and h8 apply in the full pass exactly as
  they would have in the fallback, and all nine coordinates keep their scope.
  **No third return is needed for the foreseeable branch:** if some units are
  not admitted, the pass prices the admitted ones in canonical order and prints
  the rest as N4-A16(iv) measured stops, and that mixed outcome **is** the
  result (F7, NO-RESCUE — both outcomes of every gate are results). A return to
  this file is owed only for something new: a count-only pass exhausting B, an
  (R6) or (R7) mismatch, an L ≠ Q^H strict inequality (SEP-A11), a measured
  bytes-per-state figure above the declared 128 by more than the M_max margin
  absorbs, or any stop-and-report class above.
- **N4-A20 (freezes, markers, and what the results file must carry).** Freezes
  1–46 are in force and restated unchanged except **44**, amended to **v2** at
  N4-A16(vi), clause (e) only, its number unchanged and its v1 text preserved at
  N4-A1 with a pointer marker; 38–40 remain reserved and untouched — nothing
  here instantiates the gluing-cut language, the circuit representation or the
  reachable-belief family. **No new freeze number is created**, because nothing
  new here is a determinism constant: T_pass, M_budget, the 128-bytes-per-state
  figure, the 1 GiB per-worker allowance and W are all run-owner or provenance
  quantities of M_max's type (N4-A4), printed and licensing nothing. No number
  is reused. **Six DS-A28 pointer markers** are placed at their sites in this
  file: freeze 44(e)'s P_max text at N4-A1; N4-A4's P_max-provenance sentence;
  N4-A5's activating condition; N4-A12(b)'s fallback set; N4-A1(i)'s
  cross-traversal fence (narrowed by Lemma N, and only where an exhibit exists);
  and SEP-A10(i)'s ten-minute citation. **The results file additionally
  carries**, beyond §9's header list: N4-A13(i)'s cap-stop sentence at any cap
  stop; N4-A15(i)'s N4-A5 typing paragraph verbatim, naming the digest receipt's
  weakening in place; N4-A15(iii)'s uniqueness sentence beside every count-only
  count; N4-A16(iv)'s NOT PRICED line form with its SEP-A19(b) typing;
  N4-A17(c)'s concurrency arithmetic; N4-A17(d)'s labels; N4-A18(iii)'s gate
  block; and N4-A14(iv)'s roster sentence. Every one of them is exploratory, and
  nothing in this section or the pass it authorises is quotable as a result
  about the game except by brief amendment adding it to a verifier receipt.

**What must change in the build before the pass runs.** In order of severity:
the cap-stop residual is recorded before the poison and printed with N4-A13(ii)'s
sentence; the N4-A5 fallback is implemented with its streaming set-digest
receipt and its typing paragraph (N4-A15(i)); the count-only pass is added and
admission is made on its completed count, before any map is allocated
(N4-A15(iii), N4-A16); `P_MAX` becomes 192,000,000 under freeze 44 v2; T_pass,
M_budget and W join M_max as run-owner declarations with no invented defaults
(N4-A14(iii), N4-A17(c)); the coordinate claim and the wait-never-skip admission
are implemented and asserted (N4-A17(a)–(b)); the four N4-A17(d) assertions and
the DS-A36 fresh-versus-resumed byte comparison are added; receipts (R7) and (R8)
are added with their typings; the gate block of N4-A18(iii) replaces §5's gate
text in the results file; and the NOT PRICED line form replaces any cap-stop
verdict row. Any of this that touches code the grade-3 path executes re-triggers
(R0) as a blocking precondition (N4-A10); changes confined to the n = 4 path do
not. Everything else in the design as adjudicated stands as written.

## The trick-1 witness: the bounded sandwich, refuted and replaced (2026-08-14)

**Adjudicator:** walt-math. **Object:** the build's bounded-sandwich proposal for
a certified first-trick play, relayed 2026-08-14 — per-world sound bounds
(adversarial-field lower, cooperative-field upper) summed exactly over the
trick-1 fiber |X| = C(21,7)·C(14,7) = 399,072,960 — together with the run
owner's directive that a proved first-trick play is the target and that rigour
is not negotiable. **Tier:** exploratory throughout, without exception; nothing
below is promoted, and no statement here is quotable in a brief, a dispatch,
[FINDINGS](FINDINGS.md) or any claim-tier page except by brief amendment adding
it to a verifier receipt. **Basis:** the errata under DS-A17 (Lemma E3, Lemma E4
and Non-theorem E4′, Corollary E4.1, Theorem E6.4 with its member-not-set
caveat, (C1)–(C4) of §3.4); SEP-A1..SEP-A19; N4-A1..N4-A20; DS-A1, DS-A15,
DS-A16, DS-A27; E-A2, R-A2, P-A1, P-A21, PG-A8, PG-A13, F7; and first-hand
reading, at adjudication time, of `walt-core/src/rules.rs` (`Tier`, `Rank`,
`DOUBLE_TOP`, `called_set`, `effective_incidence`, `led_context`, `trick_key`,
`beats`, `threat`, `Trick::winner`, `legal_plays`),
`walt-strat/src/direction.rs` (`trick_diff`, `trick_line`),
`walt-strat/src/info.rs` (`walk`'s uniform field share
`p.weight *= q(1, legal.len())`), and `rob/receipts/verify_player.txt` (13
hands; the bidder leads trick 1 in each). Rulings **T1-A1..T1-A12**; two lemmas,
two propositions, one theorem and one corollary delivered below; **freeze 47**
fixed at T1-A11. The prefixes `T1-A`/`T1-Q` and every name below were
grep-checked unused at adjudication time. **`Lemma X` is spent** (zero-contribution
excision, X-A section) — nothing here is named with a bare letter for that reason.

**The rule facts everything below rests on**, read from the code and stated once
so no proof re-derives them. Trick keys are lexicographic `(tier, rank)` with
`Tier::Slough(0) < Tier::Follows(1) < Tier::Called(2)`, and `Trick::winner`
returns the unique maximum. For `Decl::PipTrump(p)` the called set is `NATURAL[p]`
— the seven tiles bearing pip p. `rank` is `DOUBLE_TOP = 12` for a natural
double and the pip sum otherwise, whose maximum over mixed tiles is 6 + 5 = 11;
so **a natural double is the strict top of its effective natural context**.
`led_context` of a non-called tile is `Natural(hi)`, and `effective_incidence`
subtracts the called set, so context s comprises the tiles bearing s and not p.
`legal_plays` compels following the led effective context when able and permits
anything otherwise. The valuation is `Direction::trick_diff`: **the focal team's
trick differential**, +1 per trick the focal team takes and −1 per trick it
concedes, so at grade g the value lies in [−g, +g] and **+g is attained exactly
when the focal team takes every remaining trick**. The declared field is uniform
over each seat's legal set, per world, independently.

**Headline — five findings, stated before the rulings.**

1. **Both corner bounds are sound, and the build's reading of (C4) is correct.**
   A field-adversarial value of a fixed lawful policy is a pointwise lower bound
   on that policy's fixed-field value (a minimum is at most a mean), and a
   cooperative-field maximum is a pointwise upper bound on the world-informed
   value (a mean is at most a maximum). Perfect-information **minimax** is
   neither: the adversarial field pushes below the fixed-field value while the
   information relaxation pushes above it, and the composite bounds nothing in
   either direction. T1-A2.
2. **The proposal cannot do what it was built for, and this is proved, not
   predicted.** Proposition **T1-blind**: a lower witness that is valid at every
   root action — which every hand-only counting guarantee is — can never
   strictly exclude any competitor, because U_a ≥ Q^H(a) ≥ L for that same a.
   Proposition **T1-corner**: the corner sandwich closes at trick 1 only when
   the focal holds the entire trump suit. A night spent scanning 399,072,960
   worlds would have measured exactly this. T1-A4.
3. **What replaces it needs no relaxation at all.** Theorem **T1-draw**: on a
   closed, fully enumerated family of **294 declared trick-1 coordinates**, the
   focal seat takes all seven tricks against **every** field behaviour in
   **every** world, so `Q^H(a) = +7`, the maximum of the valuation, for every
   trump lead. The upper witness needed for membership is the trivial `U_a ≤ 7`.
   T1-A5.
4. **On 287 of the 294 a competitor is strictly excluded, so the optimal
   opening-lead set is determined exactly, not sandwiched.** Corollary
   **T1-ruff** prices the double lead by the ruff it invites. At the flagship
   coordinate — declaration `PipTrump(6)`, focal hand {6:6, 6:5, 6:4, 6:3, 6:2,
   6:1, 5:5} — `Q^H(trump) = 7` and `Q^H(5:5) = 7 − 143/5814`, exact rationals,
   so `Opt^H` is exactly the six trump leads. T1-A5, T1-A6.
5. **The corpus arm survives as the measurement, and it is the one that
   specifies the frontier.** The 13 real trick-1 coordinates are expected to
   satisfy none of T1-draw's hypotheses; what they yield is the exact corner gap
   `7 − k − E_β[f]` per coordinate, computed by exhaustive fiber sums with no
   decimation — the exact specification of what a tighter relaxation (Theorem
   E6.5's gluing, freeze 38, still reserved) must beat. T1-A7, T1-A10.

### Lemma T1-run (the trump-run guarantee) — delivered here

Let δ = `PipTrump(p)`, let T be the trump tiles still in play at a kernel, and
suppose the focal seat's hand H contains the **top k tiles of T** under δ's rank
order. Then in every world of the fiber, under every information-consistent
focal policy, every field behaviour and every belief, the focal **seat** takes at
least k tricks.

*Proof.* Every tile in play is played exactly once, and a seat plays exactly one
tile per trick, so the k tiles occupy k distinct tricks. Fix one, of rank r
within T. Every other tile played on that trick is either not a trump — trick key
tier `Slough` or `Follows`, both below `Called` — or is a trump of rank below r,
since every trump of rank above r lies in H and H's other tiles are on other
tricks. The focal tile therefore carries the unique maximum trick key and
`Trick::winner` names the focal seat. ∎

The strength and the weakness are the same fact: **it is a property of the hand
alone**. No policy, no belief, no field model and no root action enters, which is
exactly why Proposition T1-blind applies to it.

### Lemma T1-force (the prefix-forcing upper bound) — delivered here

Let S be a **prefix** of the rank order on T (the m highest trumps in play) and
suppose every tile of S is held by seats of one side. Let c be the largest number
of S-tiles held by a single seat of that side. Then that side takes at least c
tricks in every complete play, and the other side takes at most (tricks
remaining) − c.

*Proof.* The c tiles of that seat occupy c distinct tricks. On each, the highest
trump played lies in S — any trump ranking above an S-tile is itself in S — and
S is held entirely by that side, so the winner is on that side. ∎

Two corollaries used below: with the two seats of a side splitting a prefix of
size m, that side is forced at least ⌈m/2⌉ tricks; and forced focal tricks and
forced opponent tricks are **disjoint**, so at grade g their counts sum to at
most g.

### Proposition T1-blind (an action-blind lower witness can never exclude) — delivered here

Let L be a lower witness valid at every root action, i.e. L ≤ Q^H(a) for all a.
Then for every a and every valid upper witness U_a ≥ Q^H(a) we have U_a ≥ L.
Hence **no competitor is ever strictly excluded**, and the strongest verdict such
a pair can produce is membership certified simultaneously for every action —
which distinguishes a⋆ from nothing.

*Proof.* U_a ≥ Q^H(a) ≥ L. ∎

*Corollary.* Lemma T1-run's guarantee, and every other hand-only counting
guarantee, is action-blind by construction and cannot found a non-trivial
trick-1 verdict. **A non-trivial witness requires a lower witness whose validity
is action-conditioned — one that is false for the competitors.** This is the
primal-side twin of Remark E3.1's action-constant upper aggregate, and it is why
this section abandons the corners rather than sharpening them.

### Proposition T1-corner (the corner sandwich closes only degenerately at trick 1) — delivered here

At a trick-1 coordinate let k be the length of the focal seat's own top run in T
and let f(ξ) be the opponents' forced tricks under Lemma T1-force. Take
L = 2k − 7 (Lemma T1-run, converted to the differential by D = 2·tricks − 7) and
U = 7 − 2·E_β[f] (Lemma T1-force, same conversion). Then L ≥ U iff
k + E_β[f] ≥ 7, and this holds **iff k = 7**.

*Proof.* The conversion D = 2·tricks − 7 is affine with positive slope, so it
carries bounds and preserves verdicts — the freeze-37(c) argument, applied to a
different pair of conventions. Forced focal and forced opponent tricks are
disjoint, so k + f(ξ) ≤ 7 pointwise and the inequality requires f(ξ) = 7 − k for
β-almost every ξ. At trick 1 nothing has been played, so no seat is known void
and the void-free capacity fiber is the **complete** set of splits of the 21
unseen tiles; the partner therefore holds the trump of rank k + 1 with positive
probability, and in every such world the opponents hold no prefix of T ∖ H at
all, giving f(ξ) = 0 < 7 − k whenever k < 7. Hence E_β[f] < 7 − k strictly for
k < 7, and L < U. For k = 7 the focal holds the whole trump suit, f ≡ 0, and
L = U = 7. ∎

The degenerate case is the hand in which the focal takes every trick under every
play — where the sandwich closes because there is nothing to decide.

### Theorem T1-draw (the drawing-hand collapse) — delivered here

Let a kernel have declaration δ = `PipTrump(p)`, the focal seat **on lead**
(leader offset from focal 0), focal hand H with |H| = g, and remaining trump set
T; write t = |H ∩ T|. Suppose:

- **(Z1)** H ∩ T is the top t of T under δ's rank order;
- **(Z2)** 2t ≥ |T| — equivalently t ≥ |T| − t, the outstanding trump count;
- **(Z3)** every tile of H ∖ T is a **natural double**.

Let ρ_draw be the policy: *while any trump remains outside H, lead the lowest
trump in hand; thereafter lead the remaining tiles in the declared canonical
order.* Then ρ_draw takes **all g tricks** for the focal seat, in every world of
the fiber and against every field behaviour. Consequently `Q^H(a) = +g` for every
a ∈ H ∩ T, `V^H = +g`, and `Opt^H ⊇ H ∩ T`.

*Proof.* (i) *The draw terminates inside the hand.* When the focal leads a trump,
`legal_plays` compels every seat holding a trump to play one, so after j trump
leads a seat holding c outstanding trumps has surrendered min(j, c). Each seat
holds at most |T| − t of them, which by (Z2) is at most t, and the focal has t
trumps to lead. (ii) *Every trump the focal leads wins.* By (Z1) every
outstanding trump ranks below every tile of H ∩ T, and `Tier::Called` dominates
both other tiers, so the focal's led trump carries the unique maximum trick key;
the focal therefore retains the lead throughout. (iii) *After the draw, everything
in hand wins when led.* No trump remains outside H, and the focal plays one tile
per trick, so a trick the focal leads with a trump contains no other trump and is
won by tier. A natural double J ∈ H ∖ T led at that point has led context
`Natural(hi(J))` and rank `DOUBLE_TOP` = 12; every other tile in play carries
either `(Follows, ≤ 11)` or `(Slough, ·)`, so J carries the unique maximum key
and wins — this is precisely the assertion `threat(J) ⊆ called_set`, and (Z3)
plus "no trump outside H" discharges it. (iv) The focal takes every trick, so the
differential is +g, which is the maximum of `trick_diff` at grade g; hence
`Q^H(a) = +g` for every root action that begins ρ_draw, i.e. every a ∈ H ∩ T,
and no action can exceed it. ∎

**Note what the membership half does not use:** no belief, no field model, no
relaxation, no seed, no transport, no library entry, and no upper witness beyond
the trivial `U_a ≤ g`. The sandwich frame still describes it — L = U = g at a⋆ —
but the content is a pointwise structural fact about the rule algebra.

### Corollary T1-ruff (the double lead, strictly priced) — delivered here

In Theorem T1-draw's setting with |T| > t, let J ∈ H ∖ T and let q(J) be the
probability, under the declared uniform belief on the fiber and the declared
uniform-random legal field, that an **opponent** seat takes the trick 1 that J is
led to. Then

  `Q^H(J) ≤ g − 2·q(J)`, with equality when |H ∖ T| = 1,

so **q(J) > 0 strictly excludes J from Opt^H**.

*Proof.* The differential is g − 2·(tricks the opponents take). If the opponents
take the J trick they take at least one, so the realised differential is at most
g − 2; otherwise it is at most g. Taking expectations gives the inequality. For
equality when |H ∖ T| = 1: after the J trick the focal's hand is exactly its
t = g − 1 top trumps, so by Lemma T1-run the focal takes every one of the
remaining g − 1 tricks and the opponents take at most the J trick; the realised
differential is exactly g or exactly g − 2. ∎

**The flagship member, computed exactly.** δ = `PipTrump(6)`;
H = {6:6, 6:5, 6:4, 6:3, 6:2, 6:1, 5:5}; g = 7; T = the seven 6-tiles; t = 6, the
outstanding trump being 6:0, the lowest; H ∖ T = {5:5}, a natural double. (Z1),
(Z2) (12 ≥ 7) and (Z3) hold. Context 5 is `NATURAL[5] ∖ NATURAL[6]` =
{5:5, 5:4, 5:3, 5:2, 5:1, 5:0}, of which the focal holds 5:5 and five are unseen.
The 5:5 trick is lost exactly when 6:0 lies in an opponent's hand — probability
2/3, the three unseen seats being symmetric — **and** that opponent holds none of
the five remaining context-5 tiles, its other six tiles being drawn from the
twenty others of which five are context-5, probability C(15,6)/C(20,6) =
5005/38760 = 1001/7752 — **and** the field's uniform choice over its seven legal
tiles selects 6:0, probability 1/7. The two opponents' events are disjoint
because a single seat holds 6:0. Hence

  `q(5:5) = (2/3)·(1001/7752)·(1/7) = 143/11628`, and
  `Q^H(5:5) = 7 − 286/11628 = 7 − 143/5814 ≈ 6.97541`,

against `Q^H(a) = 7` for each of the six trump leads. **`Opt^H` is exactly the six
trump leads.** Theorem E6.4's member-not-set caveat is **discharged at this
coordinate** — not waived — because both sides are exact values rather than
bounds, and the argmax set is therefore determined rather than intersected.

- **T1-A1 (typing, tier, vocabulary, and what this section is).** The build's
  proposal is **ACCEPTED IN PART**: its soundness reasoning is correct and is
  ratified at T1-A2; its purpose is unattainable by the route proposed and that
  is proved, not asserted, at T1-A4; and a different route to the same goal is
  delivered at T1-A5. Everything is exploratory. DS-A1 binds: *witness* and
  *receipt*, never the forbidden word; a proved statement here is proved
  relative to walt's own declared basis and to the rule algebra as implemented,
  and remains exploratory. Both outcomes of every gate below are results (F7),
  and a receipt failure is stop-and-report, never a patch (NO-RESCUE).
- **T1-A2 (the corner directions: the build's (C4) reading CONFIRMED, with the
  proofs, and one correction of vocabulary).**
  (i) **Lower side, sound.** For a fixed information-consistent policy ρ and a
  world ξ, `min over field behaviours of v(ξ, ρ, ·) ≤ E_field[v(ξ, ρ, ·)] =
  α_ρ(ξ)`, a minimum being at most a mean; taking E_β preserves it, and
  E_β[α_ρ] = L_ρ ≤ Q^H(a) by Lemma E4 with DS-A27's semantic obligation. Sound.
  (ii) **Upper side, sound.** `V*_a(ξ) = max over focal policies of
  E_field[v] ≤ max over focal policies and field behaviours of v`, the
  cooperative-field maximum; taking E_β gives U_a ≤ E_β[b(·,a)], and
  U_a ≥ Q^H(a) by Lemma E3. Sound.
  (iii) **(C4) CONFIRMED, and the reason is sharper than the one offered.**
  Perfect-information minimax is barred as an upper bound, and not merely
  because the adversarial field can push below the fixed-field value: it is
  barred because it moves **both** dials at once, relaxing the focal's
  information upward and the field's behaviour downward, so it is neither an
  upper nor a lower bound on Q^H(a) and belongs on neither side. The corners
  are the only two directions in which a single dial moves.
  (iv) **Vocabulary, corrected.** "Minimin" names nothing here: with ρ fixed
  there is exactly one minimisation, over field behaviours. The object is *the
  field-adversarial value of a fixed policy*, and the results file uses that
  name. Note in place that the **partner is part of the field**: the lower
  corner assumes an adversarial partner and the upper corner a cooperative one.
  Both are sound; the first is why the lower corner is weak.
- **T1-A3 (the two cheap bound families, DELIVERED, with their costs typed).**
  Lemma T1-run and Lemma T1-force are the tree-free counting bounds the proposal
  asked for, and they **dominate the corners in cost while remaining sound**:
  T1-run ≤ the field-adversarial value of any policy, and 7 − (T1-force) ≥ the
  cooperative maximum, so each is a valid replacement on its side. Their cost
  typing is the useful part. **The lower bound is fiber-constant**: at trick 1
  the focal's hand is known and identical in every world, so k is a single
  integer and E_β[·] over 399,072,960 worlds is not computed at all. **The upper
  bound depends on ξ only through the split of the trumps among the three unseen
  seats**, so its fiber expectation is an exact rational obtained either in
  closed form or by an exhaustive integer count; either way no decimation
  appears anywhere inside a witness and (C2) is satisfied without argument.
  The budget the proposal reserved — 399M worlds × 7 actions × microseconds — is
  **not needed for the bounds at all**.
- **T1-A4 (the proposal's purpose is REFUTED, and the refutation is the first
  result of this section).** Propositions T1-blind and T1-corner are delivered
  above. Together they establish that the proposed sandwich, with any hand-only
  counting bound on the lower side, certifies membership for every action
  simultaneously or nothing at all, and closes only on the hand that holds the
  entire trump suit. This is filed as a **result** under F7, exactly as a gate
  failure is: it is what the proposed measurement would have measured, obtained
  by proof instead of by a night of compute. Nothing about the bounds is
  unsound; the defect is that they discriminate nothing. **The binding lesson,
  stated for the successor:** at trick 1 the lower witness is the scarce object,
  and it must be action-conditioned — the upper side had a trivial valid witness
  (`U_a ≤ 7`) all along.
- **T1-A5 (the replacement route is GRANTED: Theorem T1-draw and Corollary
  T1-ruff).** The route is admitted because it violates no standing ruling and
  needs no relaxation: the membership half is a pointwise structural fact
  discharged by the rule algebra, and the exclusion half is an exact expectation
  over the declared model, computed by exhaustive integer counting. Three
  clauses.
  (i) **The policy ρ_draw is exhibited, not seeded.** DS-A15's seeds-versus-
  witnesses distinction is not engaged: nothing here is harvested from an
  observed play or a heuristic, and no grade-3 verdict travels. Freeze 36's
  library is untouched and **no library entry is written at any trick-1
  coordinate** — freeze 45's Route-C discipline, transposed.
  (ii) **The exclusion half is model-dependent and says so.** q(J) > 0 requires
  a field model under which an opponent void in J's context and holding a trump
  plays it with positive probability. The declared uniform-random legal field
  supplies this; the statement is printed with that hypothesis attached, never
  as a claim about how anybody plays.
  (iii) **Grades other than 7 are in scope for the same theorem.** T1-draw is
  stated for an arbitrary kernel, which is what makes the authority cross-check
  of T1-A9(ii) an instance of the same statement rather than an analogy.
- **T1-A6 (the carrier, arm A: the constructed family is LAWFUL, and it is
  closed and exhaustively enumerated, so nothing is selected at all).** The
  family is **defined by T1-draw's hypotheses**, not chosen among candidates:
  for each pip p, each t ∈ {4, 5, 6, 7} — the range 2t ≥ |T| = 7 admits — and
  each choice of 7 − t of the six non-trump doubles. That is
  1 + 6 + 15 + 20 = 42 hands per declaration and **294 coordinates**, every one
  of which is run. A closed family run in full cannot be selected by result, and
  this is a stronger guarantee than the declared-in-advance rule the brief asked
  for. The seven t = 7 members are the degenerate all-trump hands: they are run,
  and their rows are labelled **TRIVIAL — every action takes every trick**, so
  that a membership verdict there can never be paraded as a decision. The
  remaining **287** carry at least one double competitor.
- **T1-A7 (the carrier, arm B: the 13 corpus trick-1 coordinates, and what the
  real-deal fence becomes at trick 1).** The corpus arm is `verify_player.txt`'s
  13 hands at trick 1, all in scope because the bidder leads the first trick in
  every one — the N4-A8 scope restriction is automatically satisfied rather than
  imposed. Two clauses.
  (i) **N4-A8's real-deal fence transposes with its arithmetic vacuous and its
  substance intact.** At trick 1 nothing has been played, so no void is known
  and the void-free capacity fiber is the complete set of splits: the
  void-filtered ratio is 1 **by construction at every coordinate**, and the
  inline marker N4-A8 mandates at h2/h5/h8 has nothing to mark. What does not
  weaken is the belief fence: uniform over that fiber is **nobody's belief**,
  and no seat at the table holds it. The results file prints both halves
  together so the vacuity of the first is never read as a strengthening of the
  second.
  (ii) **What arm B measures**: per coordinate, k (fiber-constant), E_β[f] by
  exhaustive integer count over all 399,072,960 worlds, the corner gap
  7 − k − E_β[f], and the T1-draw hypothesis check. The expected outcome is that
  no corpus hand is a drawing hand and every gap is strictly positive; that
  outcome is a result and is filed as one.
- **T1-A8 (the fences, verbatim on every certified row).** A certified verdict at
  a trick-1 coordinate says exactly this and no more.
  (i) **The membership half is belief-free and field-free**, holding pointwise in
  every world of the fiber and against every field behaviour. Because the trick-1
  fiber is the complete set of deals consistent with the focal's hand, that half
  is a statement about the rules, not about the declared model. **This is the one
  place in walt where R-A2's feasible-versus-reachable fence does not bind a
  verdict** — and the reason is not that the fence was relaxed but that the
  quantified statement ranges over every world, so reachability is irrelevant to
  it. The fence binds everything else in the row.
  (ii) **The exclusion half is model-relative**: q(J) and `Q^H(J)` are
  expectations under the declared uniform belief and the declared uniform-random
  legal field, and both are named in place. No row may let (i)'s strength leak
  onto (ii).
  (iii) **Not claimed, printed in place:** nothing about points or marks (the
  valuation is `trick_diff`, count-free — E-A2's boundary, and a count re-entry
  voids every form-keyed record wholesale); nothing about bidding or about
  whether this hand should be bid; nothing about how real opponents play;
  nothing about any coordinate outside the declared carrier; no growth law and
  no opening claim from any grade (P-A21); and, for arm A, nothing about deals —
  a constructed coordinate is a coordinate, and the family is a construction, not
  a corpus.
  (iv) **The honest characterisation of arm A, mandatory in the results file and
  in any wiki text derived from it:** *a drawing hand is a hand that plays
  itself. The theorem certifies a first-trick play at a coordinate where no
  search is needed to find it, and it says nothing whatever about hands that
  require judgement.* Dissents and caveats travel with results verbatim.
- **T1-A9 (the receipt set for the H-free regime: what replaces (R1), (R2) and
  (R3)).** At trick 1 the concrete authority does not merely exhaust its budget —
  it is **structurally absent**, which is a different situation from N4-A6's
  Tier 2 and must not borrow its language. The separation design's (R1) (envelope
  H equals the scalar authority), (R2) (L equals Q^H at the seed) and (R3) (the
  per-action price) have no referent at grade 7 and are **not printed as unmet;
  they are printed as inapplicable, with the reason**. Five receipts replace
  them, each with its PG-A8 typing.
  (i) **(T1-R1) the rule-algebra discharge.** Per coordinate, machine-verify
  (Z1) by comparing H ∩ T against the top t of `called_set` under `trick_key`;
  (Z2) by integer arithmetic; and (Z3) as the exact assertion
  `threat(J) ⊆ called_set` for every J ∈ H ∖ T, using the existing `threat`
  function. **Contentful**: each can fail on a mis-specified coordinate.
  (ii) **(T1-R2) the grade-reduced authority cross-check — the receipt that
  earns the trick-1 claim.** Construct the drawing family at a grade where
  `ScalarHidden::action_values_dag` completes within `AUTHORITY_BUDGET`, run it,
  and assert the solver's exact action values equal the theorem's prediction:
  +g at every trump lead and strictly less at every double lead, matching the
  closed form for q. Grades 2, 3 and 4 are declared in scope — grade 4's fiber is
  34,650 worlds, the very size the n = 4 work solves in seconds — and grade 5 is
  attempted with its budget stop declared and printed either way (R-A18's
  discipline). **A disagreement is a bug in the theorem or in the construction
  and is stop-and-report**; it is the single most informative outcome available
  tonight and it is pre-declared as such.
  (iii) **(T1-R3) the exhaustive fiber count.** q(J) and E_β[f] are computed by
  enumerating **all 399,072,960 worlds** with a per-world integer predicate — no
  decimation anywhere, (C2) satisfied outright — and asserted equal to the closed
  form where one is derived. All arithmetic is integer until a single final
  rational; no float appears (P-A19). **Contentful**: the closed form and the
  count are independently written.
  (iv) **(T1-R4) an exhibited world witness.** For each excluded J, print one
  explicit world and one field realisation in which the J lead concedes trick 1 —
  a witness in DS-A1's sense, machine-checkable, exact. This is what establishes
  q(J) > 0 per coordinate; positivity is never asserted from the general
  argument.
  (v) **(T1-R5) the exclusion arithmetic**, printed as exact rationals with the
  margin `Q^H(a⋆) − Q^H(J)`, and the `Opt^H` set stated with T1-ruff's
  discharge-not-waiver sentence beside it.
  (vi) **What is NOT a receipt.** The membership half's `U_a ≤ g` is true by the
  range of the valuation and is not evidence of anything; it is printed as an
  arithmetic remark. PG-A8 governs: an assertion that cannot fail is not a
  receipt.
- **T1-A10 (both outcomes pre-declared, before any number exists).**
  (a) **Arm A discharges** — the hypotheses check, (T1-R2) agrees at every
  reduced grade, the counts match the closed form: then 287 coordinates carry a
  proved exact optimal opening-lead set and 7 carry a labelled trivial one. That
  is a first-trick play proved, with T1-A8(iv)'s characterisation attached.
  (b) **Arm A fails a receipt** — most sharply, (T1-R2) disagreeing with the
  concrete authority at grade ≤ 4: then the theorem or the construction is
  wrong, **nothing is claimed**, and the failure is reported with the
  disagreeing values. No patch, no adjustment, no re-derivation to fit
  (F7, R-A18).
  (c) **Arm B certifies nothing**, as expected: then the filed object is the
  per-coordinate corner gap `7 − k − E_β[f]`, exact, over all 13 coordinates —
  **the exact specification of what a tighter relaxation must beat**, which is
  the intended content of the next dispatch. Typed as a cost-model and
  specification quantity: it licenses no claim about the game, and by P-A21 no
  gap measured here is quoted for any other grade or for the opening in general.
  (d) A corpus hand unexpectedly satisfying T1-draw's hypotheses is reported as
  what it is — a real deal that plays itself — and is not promoted by being
  real.
- **T1-A11 (freezes).** Freezes 1–46 are in force and restated unchanged; 44
  stands at v2 (N4-A16), 36 at v2 (EC-A8); 38–40 remain reserved and untouched —
  in particular **nothing here instantiates freeze 38's gluing-cut language**,
  and Theorem E6.5 is named at T1-A10(c) only as the destination of a future
  dispatch. **FREEZE 47 — the trick-1 carrier, frozen content. (a)** Arm A: the
  drawing family as defined at T1-A6, all 294 coordinates, in the canonical order
  *declaration pip ascending; then t descending; then the non-trump doubles'
  pips ascending lexicographically*, with the coordinate identity printed in
  freeze-45's form (declaration, focal hand as canonical ascending domino-index
  tiles, leader offset from focal asserted 0, |X| asserted against
  `kernel.count()`, the freeze-7/23 enumeration order) and the kernel rebuilt
  in-run from the printed identity and asserted equal. **(b)** Arm B: the 13
  `verify_player.txt` hands at trick 1, by corpus index ascending, with the
  bidder asserted to be the trick-1 leader at each. **(c)** The reduced-grade
  cross-check ladder of (T1-R2): grades 2, 3, 4 mandatory, grade 5 attempted with
  a declared stop. **(d)** No library entry is written at any coordinate of
  either arm. The belief and field models are **not** re-declared here: they are
  freeze 26 and freeze 37(d), cited and unchanged. No number is reused.
- **T1-A12 (results discipline, and the one thing that would make this section
  wrong).** The results file carries: the tier line; T1-A8's four fence clauses
  verbatim; the inapplicability notice of T1-A9 for (R1)–(R3) with its reason;
  the TRIVIAL labels of T1-A6; T1-A8(iv)'s characterisation sentence; the exact
  rationals of every q and every value; and, per arm, the declared regenerate
  path. **The load-bearing risk, named so it is watched:** every statement in
  Theorem T1-draw is a claim about the rule algebra **as implemented**, read at
  adjudication time from `rules.rs`. If the implementation and the rules corpus
  disagree — a tier order, `DOUBLE_TOP`, the effective-incidence subtraction, or
  the compelled follow — then this section is wrong in a way no receipt inside it
  can detect, because every receipt is computed by the same implementation.
  (T1-R2) is the partial guard: it checks the theorem against an independently
  written solver, though not against the corpus. **The corpus check is therefore
  mandatory before any of this is cited outside walt**: the three rule facts of
  this section's preamble are to be verified against the rules package by a
  reader, and until that is done every statement here is exploratory in the
  strong sense — proved relative to walt's implementation of the rules, and not
  yet relative to the rules.

## Lay downs: the characterization, and the four-laydown question (2026-08-14)

**Adjudicator:** walt-math. **Object:** the run owner's conversational directive
of 2026-08-14 — formalise the family term *lay down*, and settle whether a single
deal can contain four of them, a conjecture his family reached by hand
enumeration and remembered as **at most three**. **Tier:** exploratory
throughout. **Basis:** the trick-1 section immediately above (T1-A1..T1-A12,
Theorem T1-draw, Corollary T1-ruff, and **T1-A12's implementation-versus-corpus
risk, which carries here in full**); F1's pip-trump scope; DS-A1's vocabulary;
PG-A8; F7; and the same first-hand reading of `walt-core/src/rules.rs`. Rulings
**LD-A1..LD-A10**; **Theorem LD** and five corollaries delivered below;
**freeze 48** fixed at LD-A9. The prefix `LD-A` and every name below were
grep-checked unused. The rule facts of the trick-1 section's preamble are used
here without restatement.

**Headline — four findings.**

1. **"Lay down" has an exact characterization, and it is two cheap bitset
   tests.** Theorem LD: a hand is a lay down iff (L1) it holds at least |O|
   trumps outranking every trump it lacks, and (L2) every non-trump it holds has
   `threat(d) ⊆ T ∪ H`. Both are decidable per hand in microseconds, so the
   complete catalogue of lay downs is enumerable by brute force over all
   C(28,7) = 1,184,040 hands × 7 declarations.
2. **Every lay down holds at least four trumps** — Corollary LD-four, the bound
   the brief hoped for, proved rather than assumed. It falls out of the same
   worst-case world that drives everything else: one seat may hold every
   outstanding trump, so drawing them costs one lead each.
3. **Theorem T1-draw's 42-hand family is a strict inner class, and (Z1) is not
   necessary** — the brief's witness is settled: {6:6, 6:4, 6:3, 6:2, 6:1, 6:0}
   plus a natural double **is** a lay down, because the single outstanding trump
   6:5 sits alone in its holder's hand and the compelled follow extracts it on
   the first lead. The banking worry is real in general and is exactly what (L1)
   measures; it cannot arise when |O| = 1.
4. **The four-laydown question is reduced to a finite exact search over the
   catalogue, and I do not pre-judge it.** Five structural corollaries cut it
   hard — the four declarations must be distinct pips, at most one hand may hold
   a whole suit, exactly one must, and the other three each carry exactly one
   double of a non-declared pip with all their non-trumps in that single context.
   My own hand-worked constructions all failed, and one failed for a reason worth
   recording (LD-A7), but **a failed search by hand is not a theorem** and the
   family's conjecture stays a hypothesis until the probe runs.

### Theorem LD (the lay-down characterization) — delivered here

**Definition (lay down).** A 7-tile hand H is a **lay down** under δ =
`PipTrump(p)` if, with its holder on lead at trick 1, some plan takes all seven
tricks in **every** world of the complete trick-1 fiber against **every** field
behaviour. This is the field-adversarial guarantee — T1-A2's lower corner
demanded at its maximum — so it is belief-free and field-free exactly as
T1-draw's membership half is, and it is **not** the fixed-field Q^H.

Write T for the seven trump tiles, t = |H ∩ T|, O = T ∖ H, and — ranking T by
`trick_key` — let **r** be the rank position of the highest trump not in H
(r = 8 when O = ∅), so H contains exactly ranks 1..r−1 of T.

**Theorem LD.** H is a lay down under δ iff

- **(L1)** **r − 1 ≥ |O|**, and
- **(L2)** for every non-trump d ∈ H, **`threat(d) ⊆ T ∪ H`** — every tile that
  beats d when d is led is either a trump or held by the holder.

*Proof of sufficiency.* The plan: lead the top |O| trumps, then everything else
in any order. Each of those leads beats every outstanding trump by (L1) and by
`Tier::Called` dominating both other tiers, so each wins and the holder keeps the
lead. Each lead compels every seat holding a trump to play one, and no seat holds
more than |O| of them, so after |O| leads no trump remains outside H. Thereafter a
trump the holder leads is the only trump on its trick and wins by tier; and a
non-trump d wins because, by (L2), every tile beating it in its led context is a
trump — none of which remain outside H — or is in H and therefore in no other
seat's hand. Seven tricks. ∎

*Proof of necessity.* The holder must win every trick, hence leads every trick,
hence leads all seven of its tiles.

*(L1).* Suppose r − 1 < |O|, so H holds fewer than |O| trumps above max(O). Take
the world in which one seat A holds all of O; A plays its **lowest** trump on
each trump lead, so max(O) survives A's first |O| − 1 trump plays. Any trump the
holder leads while max(O) is outstanding must outrank it, and it has only r − 1
such trumps; so within the first |O| leads the holder must either lead a trump
ranked below max(O) — which A beats — or lead a non-trump, which A ruffs if A is
void in that context. A can be made void in every context the holder can lead:
the holder's non-trump tiles occupy a set P of led pips with |P| ≤ 7 − t, so A's
other 7 − |O| = t tiles need only avoid P, and the non-trump tiles on the
remaining 6 − |P| ≥ t − 1 pips number (6−|P|)(7−|P|)/2 ≥ t for every t ≥ 3.
Either way A takes a trick.

*(L2).* Suppose some non-trump d ∈ H has a non-trump e ∉ H beating d in
c = `led_context(d)`. Let h_c be the number of holder tiles led into context c;
h_c ≤ 7 − t ≤ 3 by Corollary LD-four below. Give e to a seat together with
h_c − 1 further context-c tiles, which is possible because the context holds six
tiles of which the holder leads h_c ≤ 3, leaving 6 − h_c ≥ 3 outstanding. That
seat follows every earlier context-c lead with a spare and plays e when d is led,
taking the trick. e cannot be extracted otherwise: extraction into context c
happens only on the holder's context-c leads, since a seat void in c discards its
own choice and adversarially never a context-c tile. ∎

**Corollary LD-four (every lay down holds at least four trumps).** t ≥ 4.
*Proof.* If O = ∅ then t = 7. Otherwise ranks 1..r−1 lie in H, so t ≥ r − 1, and
(L1) gives r − 1 ≥ |O| = 7 − t; hence t ≥ 7 − t. ∎

**Corollary LD-top (every lay down holds its declaration's double).** Rank 1 of T
is `p:p`. If it were absent, r = 1 and (L1) would demand 0 ≥ |O| = 7 − t, forcing
t = 7 and hence H ⊇ T ∋ p:p, a contradiction. ∎

**Corollary LD-closed (the non-trump part is a union of context prefixes, and
each context's double is held).** (L2) applied to d says every context-c tile
above d is in H; the top of every context is its natural double (rank
`DOUBLE_TOP` = 12), so `c:c ∈ H` for every context c the holder leads into,
unless d is itself that double. Since t ≥ 4, at most three non-trumps exist, so at
most three contexts are led into and each carries its double. ∎

**Corollary LD-nobank (the banking worry cannot arise at |O| = 1).** A seat
holding the single outstanding trump holds no other trump, so the compelled
follow makes it play that trump on the holder's first trump lead. Hence
{p:p} ∪ (any five further trumps) with (L2)-closed non-trumps is a lay down
whatever the rank of the one missing trump, **provided r − 1 ≥ 1** — which
Corollary LD-top already gives. This settles the brief's witness affirmatively
and shows (Z1) of Theorem T1-draw is **sufficient, never necessary**. ∎

**Corollary LD-extract (extraction never buys anything).** The necessity proof's
alternative — the holder leading its high context-c tiles to drag out a superior
before leading a lower one — requires h_c > r_c − 1 and r_c − 1 ≥ 6 − h_c, whence
h_c > 3. But h_c ≤ 7 − t ≤ 3 by Corollary LD-four. **So the extraction branch is
empty and (L2) is the whole of the non-trump condition.** ∎

- **LD-A1 (the definition is ACCEPTED as stated, with its typing fixed).** The
  brief's formalisation is exactly right and is bound: a lay down is the
  **field-adversarial guarantee at its maximum** — some plan takes all seven
  tricks in every world against every field behaviour. Four typing clauses, all
  printed wherever the term is used. (i) It is **not** Q^H: it quantifies over
  field behaviours instead of averaging over the declared uniform-random legal
  field, so it is belief-free and field-free in T1-A8(i)'s sense and, the trick-1
  fiber being the complete deal set, it is a statement about the rules rather
  than about walt's declared model. (ii) The holder is the **trick-1 leader**,
  which in a dealt hand means the bidder. (iii) The declaration is the holder's
  **own**, so "H is a lay down" is always elliptical for "under δ", and a hand may
  be a lay down under one pip and not another. (iv) Scope is **pip-trump only**
  (F1): `DoublesTrump` and `NoTrump` are out of scope here and their rank
  algebra differs (`rank` branches on `Decl::DoublesTrump`), so nothing below is
  quoted for them.
- **LD-A2 (Theorem LD is DELIVERED, and with it a decision procedure that is two
  bitset tests).** (L1) is a scan of the seven trumps in `trick_key` order; (L2)
  is `threat(d).difference(called_set.union(H)).is_empty()` for each non-trump
  d ∈ H, using the `threat` function that already exists. Cost per (hand,
  declaration) is a handful of bitset operations, so **the complete catalogue is
  enumerable by brute force**: C(28,7) = 1,184,040 hands × 7 declarations =
  8,288,280 tests. No search, no tree, no fiber. This is the answer to the
  brief's "characterization **or** cheap decision procedure": it is both, and
  they are the same object.
- **LD-A3 (the brief's witness is SETTLED, affirmatively, and (Z1) is demoted).**
  {6:6, 6:4, 6:3, 6:2, 6:1, 6:0} plus a natural double **is** a lay down under
  `PipTrump(6)`: |O| = 1 with O = {6:5}, r = 2, and (L1) reads 1 ≥ 1. The
  compelled follow does the work — a seat holding the one outstanding trump holds
  no other, so it cannot bank it behind a lower one, which is Corollary
  LD-nobank. **The banking worry the brief raised is real in general**: it is
  precisely what (L1) measures, and it is why (L1) counts the holder's trumps
  *above max(O)* rather than its trumps in total. Consequently Theorem T1-draw's
  hypothesis (Z1) is **sufficient and not necessary**, and T1-draw's 42-hand
  family per declaration is a **strict inner class** of the lay downs. Nothing in
  the trick-1 section is weakened: T1-draw remains true as stated, and its family
  remains the closed carrier freeze 47 fixes.
- **LD-A4 (Corollary LD-four is the structural result the brief asked for).**
  Every lay down holds **at least four trumps**, and the proof is one line from
  (L1). What it cuts: the non-trump part of any lay down has at most three tiles,
  which is what makes Corollary LD-extract vacuous, which is what makes (L2) —
  rather than some recursive extraction condition — the exact non-trump test. The
  bound is therefore not merely a search pruner; **it is what makes the
  characterization finite in the first place**, and it is worth having
  independently exactly as the brief anticipated.
- **LD-A5 (Corollary LD-sweep, and the one place count enters).** A lay down's
  holder wins every trick, so every tile falls to the holder's team and the
  opposing team captures nothing: the hand takes **all seven tricks and all
  forty-two points**. This is the family's own sense of "can't lose", and it is
  reached by arithmetic — every trick is won, so every tile is captured — not by
  a transport. **E-A2 is not engaged and must not be cited as if it were**: no
  form-keyed record is created here, no count-free verdict is extended, and
  nothing count-valued is stored. The sentence is a consequence of the sweep and
  travels only with it; a hand that takes six tricks is outside it entirely.
- **LD-A6 (the four-laydown question: six structural facts, proved).** Let a deal
  be four 7-tile hands, hand i a lay down under δ_i = `PipTrump(p_i)`. Then:
  **(D1)** the four pips are **distinct** — by Corollary LD-top each hand holds
  `p_i:p_i`, and one tile sits in one hand. (D1 is the brief's presumption,
  now proved rather than presumed.)
  **(D2)** the four trump suits meet pairwise in exactly one tile, so their union
  is 4·7 − C(4,2) = **22** tiles, and each contributes at most one to Σt_i;
  hence Σ t_i ≤ 22 and Σ n_i ≥ 6, where n_i = 7 − t_i.
  **(D3)** every non-trump tile d in hand i has its **led pip among the three
  non-declared pips**: `hi(d) = p_j` would force `p_j:p_j ∈ H_i` by Corollary
  LD-closed against `p_j:p_j ∈ H_j` by LD-top, and `hi(d) = p_i` would make d a
  trump.
  **(D4)** only three doubles of non-declared pips exist and each hand with a
  non-trump needs at least one, so **at least one hand has n_i = 0**, i.e. holds
  its whole trump suit.
  **(D5)** **at most one** hand can hold a whole suit, since two whole suits
  would both contain their shared cross tile. With (D4): **exactly one hand holds
  a full suit, and the other three each hold exactly one non-declared double and
  have all their non-trumps in that single context.**
  **(D6)** writing m_i for the number of hand i's own-suit cross tiles it does
  not hold, each cross tile is held by at most one of its two suits' hands, so
  Σ m_i ≥ 6; and (L1) gives a_i = r_i − 1 ≥ n_i ≥ m_i, so each hand's held top
  prefix is at least as long as its cross-tile deficit.
  These reduce the question to a small finite search, and LD-A8 specifies it.
- **LD-A7 (my own hand search FAILED to find a four-laydown deal, and that is
  NOT a result).** Recorded as motivation and nothing more, per the standing rule
  that exploratory reasoning is cited by nothing above it. Two configurations
  were worked by hand and both died. With declarations {6, 5, …} the two hands
  contend for `6:5`, which is rank 2 of suit 5 and rank 2 of suit 6; whoever
  loses it has a_i = 1 and therefore needs t_i = 6, which its remaining
  availability cannot supply. With declarations {0, 1, 2, 3} and the full suit at
  0, the six tiles left over are exactly the six on the non-declared pips
  {4, 5, 6}, whose context-closed 2-subsets are {4:4, 6:4}, {5:5, 6:5} and
  {6:6, 6:5} — the last two collide on `6:5`, so only two of the three hands can
  be supplied. **Both are dead ends in a search, not obstructions in a proof**:
  neither argument closes over the choice of declared pips, the assignment of the
  six cross tiles, or the possibility that a hand's non-trumps come from another
  hand's suit. The family's ≤ 3 conjecture remains a **hypothesis**. I decline to
  present a partial case analysis as an impossibility theorem, and the brief's
  own instruction — both outcomes are results — is what the probe is for.
- **LD-A8 (the probe, in two phases, with its receipts and both outcomes
  pre-declared).** This needs a build, not more proof.
  **Phase 1 — the catalogue.** For each of the 7 pip declarations and all
  C(28,7) hands, evaluate (L1) ∧ (L2) and emit the catalogue plus the per-
  declaration count. Receipts: **(LD-R1)** the count for `PipTrump(6)` asserted
  against the closed form of LD-A9(ii) — contentful, since the closed form and
  the enumeration are independently derived; **(LD-R2)** every member of freeze
  47's T1-draw family asserted to be in the catalogue under its own declaration,
  and the containment asserted **strict** — contentful, and it is the check that
  LD-A3's demotion of (Z1) is real rather than a misreading; **(LD-R3)** for a
  declared deterministic sample of catalogue members — the first in canonical
  order at each value of t — the sweep asserted directly by exhaustive play of
  the LD plan against **all** field behaviours in a **declared reduced-grade**
  analogue where that enumeration is finite, in the style of (T1-R2). Not a
  receipt, and printed as such (PG-A8): the fact that (L1) and (L2) hold on a
  hand the catalogue selected *because* they hold.
  **Phase 2 — the four-laydown search.** Over the catalogue, search for four
  pairwise-disjoint hands with distinct declarations whose union is all 28 tiles.
  Pruned by LD-A6: begin from the seven full-suit hands, one per pip, since (D5)
  makes exactly one of them present; extend by (D1) distinctness and disjointness.
  The search is **exhaustive and exact** — it terminates with a witness or with a
  proof of non-existence over the catalogue, and the catalogue is complete by
  Theorem LD. Report additionally the **maximum number of pairwise-disjoint lay
  downs completable to a deal**, which is the quantity the family's memory is
  actually about ("three plus a strong fourth, yes").
  **Both outcomes.** A witness deal is printed tile-by-tile with a per-hand
  discharge of (L1) and (L2) and is a **counterexample to the family's
  conjecture**. Non-existence over the catalogue is a **proof of the conjecture**
  relative to Theorem LD and to the implementation caveat of LD-A10 — and it is
  filed as the result it is (F7), not as a null.
- **LD-A9 (the taxonomy, banked; FREEZE 48).** (i) **What the catalogue is for**:
  it is the outer boundary the S6g results were missing — T1-draw's family is the
  inner class walt can certify by a single theorem, and the catalogue is
  everything the property actually admits. Both are exploratory and neither is
  quoted for the other. (ii) **A prediction, declared before the run and typed as
  a prediction, not a result**: for `PipTrump(6)` I count **301** lay downs, from
  1 + 36 + 160 + 104 over non-trump-part sizes 0, 1, 2, 3 — the non-trump parts
  being 1, 6, 16 and 26 context-closed sets and the trump parts 1, 6, 10 and 4
  admissible (L1)-configurations at t = 7, 6, 5, 4. Against T1-draw's 42 per
  declaration, the inner class would be about one seventh of the whole. A
  mismatch is a defect in this arithmetic, not in the enumeration, and (LD-R1)
  exists to catch it. Counts for the other six pips are not predicted here: the
  context-closure count depends on which pip is trump, and hand-deriving seven of
  them would multiply the chance of exactly the error (LD-R1) is meant to find.
  (iii) **FREEZE 48 — the lay-down catalogue.** The hand enumeration order
  (ascending canonical domino index, lexicographic), the declaration order (pip
  ascending), the catalogue record format with the freeze-set digest, and the
  phase-2 search order (full-suit hand by pip ascending, then extensions in
  catalogue order). No number is reused; freezes 1–47 stand, 44 at v2, 36 at v2,
  38–40 reserved and untouched.
- **LD-A10 (fences and results discipline).** (i) **The four-laydown question is
  combinatorial, not a situation.** In a dealt hand only the bid winner declares
  and only one seat leads trick 1, so four lay downs can never be *realised*
  together; the question asks whether the 28 tiles can be partitioned so that
  each hand **would** sweep if it were the one to declare and lead. Every row
  says so. (ii) **T1-A12's risk carries in full and is if anything sharper here**,
  because Theorem LD is a claim about `rules.rs`'s rank, tier, follow and
  compelled-follow semantics and the probe computes its own evidence from that
  same implementation. The three rule facts of the trick-1 preamble, plus
  `threat`'s definition, must be checked against the rules package before any of
  this is cited outside walt; until then everything here is proved relative to
  walt's implementation of the rules and not relative to the rules. (iii) No
  promotion: not for bidding, not for real opponents, not for `DoublesTrump` or
  `NoTrump`, and no count-keyed record anywhere despite LD-A5. (iv) The term
  **lay down** is the family's, is used here as a defined technical term with
  LD-A1's typing attached, and is never used loosely — a hand that "looks like a
  lay down" is a hand that has not been tested.

**What the build owes this section.** The two-phase probe of LD-A8, with the
three receipts, freeze 48's orders, and the counts printed per declaration.
Everything else here is proof and needs no code. If phase 2 returns a witness, it
is checked tile-by-tile before anything is said; if it returns none, that is the
family's conjecture proved, with the LD-A10(ii) caveat attached to it verbatim.

### Closing note: the probe returned (2026-08-14, after the run)

**Object:** `walt-factory/examples/laydown_probe.rs` and
`results/laydown_2026-08-14.txt`, `results/laydown_catalogue_2026-08-14.txt`
(commits 702c866, 104f2b4). All three declared receipts **HELD**: (LD-R1) the
`PipTrump(6)` count is exactly **301**, discharging LD-A9(ii)'s pre-declared
closed form; (LD-R2) all 294 freeze-47 members are present under their own
declarations and the containment is **strict at every declaration**, 42 of 301;
(LD-R3) the LD plan swept every trick against every field behaviour at the
declared grade-3 analogues for each t ∈ {4, 5, 6, 7}, zero tricks lost over
120,960–362,880 adversarial leaves per sample. Phase 2 returned **no
four-laydown deal**, exhaustively from every full-suit anchor per (D5), every
declaration triple, every disjoint catalogue pair, and the forced fourth hand
under every remaining declaration. Wall-clock 235 ms, provenance only.

- **LD-A11 (the four-laydown question is SETTLED in the negative; the family's
  conjecture is PROVED, and here is the exact form of the statement).** The proof
  chain is: the rule algebra as implemented → Theorem LD → the catalogue is
  **complete**, not merely large → the phase-2 search is exhaustive over a
  complete catalogue → no partition of the 28 tiles makes all four hands lay
  downs. Every link is proved; the one soft link is LD-A10(ii), the
  implementation-versus-corpus check, which remains pending and travels with the
  statement. **The sentence that may be written, and no stronger one:** *no deal
  of the 28 tiles admits four lay downs, one per hand under each hand's own
  declaration — proved at the exploratory tier, relative to walt's implementation
  of the rules, by exhaustive search over a catalogue that Theorem LD proves
  complete.* Filed as a result under F7 exactly as a witness would have been.
  Two clauses. (i) **The maximum is three, and it is exhibited**: the first
  witness in freeze-48 order is `PipTrump(0)` {0:0, 1:0, 1:1, 2:0, 3:0, 4:0,
  5:0}, `PipTrump(2)` {2:1, 2:2, 3:2, 3:3, 4:2, 4:4, 6:2}, `PipTrump(5)` {5:1,
  5:2, 5:3, 5:4, 5:5, 6:5, 6:6}, with the leftover fourth hand {3:1, 4:1, 4:3,
  6:0, 6:1, 6:3, 6:4}. Re-verified at adjudication time: the four hands partition
  all 28 tiles with no repetition, and each of the three carries t ∈ {5, 6} with
  every non-trump a natural double, so (L1) and (L2) discharge by inspection —
  (L1) reads 1 ≥ 1, 2 ≥ 2 and 6 ≥ 1 respectively. **This is precisely the shape
  the family remembered**: three lay downs and a strong fourth hand. That the
  remembered bound and the remembered near-miss both land exactly is worth
  saying, and is not evidence of anything beyond itself. (ii) LD-A7's hand
  searches are **retired**: they were dead ends in a search and the search has now
  been done properly. They stay on the record as filed, because a corrected
  record needs the correction visible, not the error erased.
- **LD-A12 (the seven-fold constancy: RECEIPT IT — it is a corollary, not a
  regularity, and Corollary LD-fold is delivered here).** The build's observation
  that every declaration counts 301 is not a coincidence to be noted and left;
  it is a theorem, and leaving a theorem as an observed regularity is the
  failure mode this record exists to prevent.

  **Corollary LD-fold (the lay-down predicate is declaration-fold invariant).**
  For pips p, p′ let σ = σ_{p→p′} be the bijection of {0..6} sending p ↦ p′ and
  the k-th smallest pip of {0..6} ∖ {p} to the k-th smallest of {0..6} ∖ {p′},
  extended to tiles by a:b ↦ σ(a):σ(b). Then H is a lay down under
  `PipTrump(p)` iff σ(H) is a lay down under `PipTrump(p′)`; consequently the
  catalogue counts are equal across all seven declarations.
  *Proof.* σ is an isomorphism of the declaration-relative rule algebra. Called-
  ness: a tile contains p iff its image contains p′. Contexts: σ restricted to
  the non-declared pips is order-preserving by construction, so `hi` commutes
  with σ on non-trump tiles and `led_context` transports. Effective incidence: e
  follows context c iff e bears c and not p, iff σ(e) bears σ(c) and not p′.
  Within-tier order: inside the called tier the order is the double first, then
  by the other pip descending; inside the follows tier for context c the ranks
  are the pip sums c + y, which compare by y alone, and the natural double's
  `DOUBLE_TOP` = 12 exceeds every mixed sum (max 11) — so **both orders are
  functions of the order of the non-declared pips only**, which σ preserves.
  Legality and the compelled follow are defined from effective incidence, so
  they transport. Hence O, r, `threat` and every quantity in (L1) and (L2)
  transport, and the predicate is invariant. ∎
  **Note what is and is not being cited.** This is a direct exhibition, in the
  style Lemma E7 requires of a transport, and it is what licenses the equality.
  Lemma S-fold and Corollary S-fold-val are the **precedent** that the
  declaration fold is the right object and that value transport along it is
  reading-independent; they are named as precedent and are not the licensing
  authority for this predicate, which is count-free, form-level, and proved
  directly above.
  **Binding: add (LD-R4)** — all seven per-declaration counts asserted equal, and
  the `PipTrump(6)` count asserted equal to 301 as (LD-R1) already does.
  Contentful under PG-A8: it fails if σ is not an isomorphism, if the
  enumeration is asymmetric, or if the rank algebra is not what Corollary LD-fold
  reads it to be — which makes it, additionally, the **cheapest available probe
  of the LD-A10(ii) risk**, since an implementation whose ranks were not pip-order
  functions would break the equality. The catalogue totals **301 per declaration,
  2,107 (hand, declaration) pairs** may then be printed as a receipted count
  rather than an observation.
- **LD-A13 (what is now closed, and what is not).** Closed: the definition, the
  characterization, the t ≥ 4 bound, the (Z1) demotion with its quantification
  (42 of 301), the catalogue, the four-laydown question, and the maximum. Open
  and unchanged: LD-A10(ii)'s corpus check, which (LD-R4) now probes but does not
  discharge — a reader must still verify tier order, `DOUBLE_TOP`, the
  effective-incidence subtraction, the compelled follow and `threat` against the
  rules package before any of this leaves walt. **No further build is owed on this
  section beyond (LD-R4).** Nothing here is promoted: the catalogue, the counts,
  the witness deal and the negative are all exploratory, cited by nothing above
  this tier, and quotable as results only by brief amendment adding them to a
  verifier receipt.
