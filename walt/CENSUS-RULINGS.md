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
