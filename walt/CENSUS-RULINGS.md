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
