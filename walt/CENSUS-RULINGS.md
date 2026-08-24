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
  assertions firing only on complete results. *[CLARIFIED 2026-08-14 by RW-A8:
  clause (b)'s contract binds **every** `walk`-based evaluator, including ones
  built after this freeze — `policy_value_by_rule` is the first. This clause
  enumerates that design's six evaluators; it is an inventory, not a closed list
  of what may exist, and a new budgeted evaluator needs no freeze amendment.]* **(d)** `revealed_summary`: one
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

## The map-free rule walk, and what h9 already decided (2026-08-14)

**Adjudicator:** walt-math. **Object:** the proposal to price the N4-A16(iv)
NOT PRICED coordinate h9 by evaluating EC-A1's hand-authored rule arms **at the
callback**, with no extraction map — `policy_value_by_rule` — together with the
port of the economy arms to the n4 carrier. **Tier:** exploratory throughout.
**Basis:** EC-A1..EC-A14 (freeze 46, EC-A2's retyping, EC-A9's second carrier,
EC-A11's candidate-versus-class fence, EC-A13's primal-half split);
N4-A1..N4-A20 (freezes 44 v2, 45; N4-A6's tier regime; N4-A16(iv)); SEP-A11,
SEP-A13, SEP-A19; Lemma E4 and Non-theorem E4′ with DS-A27's semantic
obligation; **Corollary E4.1**; Lemma N of the n4 return; DS-A14, DS-A16, PG-A1,
PG-A8, R-A18, F7. Read first-hand at adjudication time: the filed
`results/separation_n4_2026-08-14.txt` — h9's Tier-1 block in full, its four R3
rows, its two `a⋆` NOT PRICED lines, and the `PRIMAL CEILING` header — and
`walt-strat/src/info.rs`'s callback signature. Rulings **RW-A1..RW-A8**;
**freeze 49** at RW-A8. The prefix `RW-A` was grep-checked unused.

**Headline — the target is unattainable, and h9 has already answered a different
question that nobody has filed.**

The filed block gives, at h9 (pip 4, hand [30 41 54 61], Tier 1, gate MET):
`Q^H(41) = Q^H(54) = 28422259/8870400` as the argmax, and
`U(54) = 545341/158400 = 30539096/8870400`. The binding margin at either
H-optimal action is therefore

  **Q^H(a⋆) − max_{a ≠ a⋆} U_a = −2116837/8870400 ≈ −0.23864**,

which is negative, and is exactly the price already printed on h9's own R3 row
for root 54. By **Corollary E4.1(3)**, `Q^H(a⋆) < U_a` means **no candidate set
whatsoever separates that pair** — not the exact seed, not a rule, not anything.
Three consequences, all available now and none needing a build.

1. **A rule cannot separate at h9.** Outcome (a) of the brief — "CERTIFIED
   CHEAP at the coordinate that broke enumeration" — is unreachable, and h9 is
   the *worst* of the nine coordinates on this measure, not a borderline one.
2. **h9's coordinate verdict is already determined and is NOT SEPARATED**, by an
   exact negative, from Tier-1 crosschecked numbers. The primal pipeline is not
   needed for it. The pass filed `NOT PRICED — no verdict` and stopped; that line
   is true of the *pipeline* and understates what the coordinate's own numbers
   already prove.
3. **The separation question the rule route can actually answer lives
   elsewhere.** Computed at adjudication time from the filed R3 rows, the binding
   margin is **positive at exactly four coordinates** — pip 6 [11 43 60 66]
   (+23/2970), pip 1 [21 40 51 65] (+4318547/19958400), pip 5 [31 51 55 63]
   (+8023709/43545600), pip 5 [21 31 33 55] (+5003513/43545600) — and negative at
   the other five, h9 included. Each computed margin reproduces the filed
   `margin` column and the filed verdict exactly where the coordinate was priced,
   which is the check that this arithmetic is reading the file correctly.

**None of this makes the rule walk not worth building.** It retypes it. The
economy gap `g = Q^H(a) − L_rule(a)` is a real measurement at every coordinate
including h9, and at h9 it is the *only* primal quantity obtainable at all — a
dumb instant heuristic setting a bar where enumeration cannot run is exactly the
stated interest, and it is a bar on the gap, never a separation.

- **RW-A1 (the map-free rule walk is a LAWFUL L evaluator; the rule's argument
  list is declared and closed).** Lemma E4 needs the policy to be deterministic,
  information-consistent, and to play the root action at the root. A rule
  evaluated at the callback satisfies all three, and the third for free because
  the L walk fixes the root action before the rule is ever consulted. Binding.
  (i) **The argument list is exactly `(record, legal)`.** The record is the
  focal's observation record and the legal set is a function of the focal's hand
  and the led context; both are functions of the information state, so
  information-consistency is structural. The public trick context — who led,
  what is on the table, who took the previous tricks — is a **derived view of
  (kernel identity, record)**, not additional information, and a rule may compute
  it; that is the project's derived-views-never-stored-state discipline applied
  to a policy.
  (ii) **The pooled-node count is NOT passed to the rule.** `walk`'s callback
  also carries `bag.len()`. It is a function of the information state, so reading
  it would not be *unlawful* — but it is a function of the **declared fiber**,
  and a rule that reads it is belief-dependent and is no longer the hand-authored
  object freeze 46 named. A rule reading it is a different arm requiring its own
  adjudication. The evaluator does not pass it.
  (iii) **Each arm carries a one-line information-consistency argument, printed
  beside its row**, in the style the brief calls "P3-style" — naming which
  components of (record, legal) it reads. This is a stated obligation on the
  build, not something I can discharge from a design.
  (iv) DS-A16 continues to apply: rule arms remain valid primal-witness sources
  if count re-enters; their count-free verdicts do not survive.
- **RW-A2 (the receipt set: what is by construction, what replaces totality, and
  the one blocking check).** The brief's reading is **correct and incomplete**.
  (i) **Totality is by construction and is not a receipt** — right, and PG-A8 is
  the reason: with no map there is nothing to be missing. It is printed as
  by-construction or not printed.
  (ii) **Three more collapse the same way, and each must be printed as
  by-construction rather than as evidence.** The singleton expansion, since the
  rule returns one tile by type — DS-A27's semantic invariant is discharged
  structurally, which DS-A27 itself anticipates. The equality of R5's two counted
  halves, since both counters increment in the same callback. The distinctness of
  reached states, by the tree-walk property already used at N4-A15(iii) — a
  node's label is its play prefix, so no record is reached twice — **which is
  also why the `seen` set is dropped and the walk stays O(1) in memory**.
  Legality likewise, **provided each arm is defined as a selection from `legal`**;
  the arm declarations state that they are, and any arm not so defined must
  assert legality at every callback, where the assertion is contentful.
  (iii) **What replaces totality is stronger than totality was: (RW-R1)
  `L_rule(a) ≤ Q^H(a)`, asserted exactly against the filed Q^H.** At h9 this is
  available precisely because the coordinate is Tier 1. It is Lemma E4's
  conclusion turned into a check, it can fail, and a violation is
  stop-and-report naming three defects in SEP-A11(i)'s style: the rule is not
  information-consistent (it is reading the world), the walk is wrong, or the
  authorities disagree. This is the genuine receipt of the whole route.
  (iv) **(RW-R2), BLOCKING: the two evaluators agree.** Before any h9 number is
  quoted, the *same rule* is priced twice at a coordinate where both routes fit —
  once by `policy_value_by_rule` computing the choice at the callback, once by
  materialising that rule into a map and pricing with `policy_value_by_record` —
  and the two L values are asserted **exactly equal**. This is the (R0)/(T1-R2)
  pattern: a new evaluator is not trusted until it reproduces the old one on
  shared ground. A difference is a defect in the new evaluator, never a finding.
  (v) **(RW-R3)** the L walk's reached-state count is an exact integer in
  SEP-A19(b)'s class, named by traversal, and is never an information value, a
  decision width, a cost claim or a DS-A2 term. **(vi)** freeze 44(b)'s budget
  contract applies unchanged — B walk-steps, `Option`, no partial fold retained.
  Cost-model input, labelled and licensing nothing: by Lemma N the L walk's
  charge is the pooled charge with focal branching pruned, so at h9 it is of
  order `tree-v0 / 4` divided by the focal branching product — roughly 10⁸–10⁹
  walk-steps against B = 10¹⁰, in O(1) memory.
- **RW-A3 (h9's verdict is already determined; FILE IT, and do not blur the two
  labels).** Binding, and it costs no compute.
  (i) The h9 block gains a **coordinate verdict line**: *NOT SEPARATED at either
  H-optimal action; binding margin −2116837/8870400 < 0; by Corollary E4.1(3) no
  candidate set whatsoever separates this coordinate — exact negative, from
  Q^H and U alone, at Tier 1 with the authority gate MET.*
  (ii) **This is not a NOT-SEPARATED pair verdict in N4-A6(ii)'s sense** and must
  never be printed as one. N4-A6(ii) requires the whole primal pipeline for a
  *pair* verdict; Corollary E4.1(3)'s exact negative is a different statement —
  about what no candidate can achieve — and needs only the two witnesses. The
  distinction is exactly what N4-A6(ii) exists to keep, and the line says which
  object it is.
  (iii) **The NOT PRICED line stands verbatim.** The unit remains **NOT PRICED on
  the exact route** and becomes, additionally, **RULE-EVALUATED** when the rule
  arms run. The two labels never merge, and a rule-seeded L is **never** called a
  price: "priced" means the exact primal pipeline ran at that unit.
- **RW-A4 (the port to the n4 carrier: which arms run, and how the missing one
  prints).** EC-A9's "ported by re-declaration" **covers h9**: h9 is a coordinate
  of the n4 carrier, and being not-priced is the reason to port, not a bar.
  **P1 least-tile, P2 greatest-tile, P3 beat-if-able, P4 trump-hoard run**, their
  definitions re-declared verbatim against the n4 carrier's canonical order
  (EC-A1(a): the canonical order, not freeze 26). **Arm X is STRUCTURALLY
  UNAVAILABLE at h9** and prints as such — *extraction map exceeds P_max v2 =
  192,000,000 at a measured 517,562,322 states (N4-A16(iv))* — never as a failure
  and never as a gap. What its absence costs is **a receipt, not the
  measurement**: arm X was the control witnessing that the pipeline reproduces
  Q^H (g = 0 by Corollary E4.1(2)), and the anchor for every gap is the **filed
  Q^H itself**, which exists at h9. **Arms T and R are out of scope** on this
  carrier: both key off stored library entries, and freeze 45 writes no library
  entry at any n = 4 coordinate.
- **RW-A5 (the aim, redirected; and the identity that does the work).** The
  separation condition for a rule is `g(a⋆) ≤ margin(a⋆)` where
  `margin(a⋆) = Q^H(a⋆) − max_{a≠a⋆} U_a` — R8's identity, computable **now**
  from the filed rows at every coordinate, priced or not. Consequently:
  (i) **the separation question is live only at the four positive-margin
  coordinates**, where it asks the economy thesis's real question — can a cheap
  rule match what the exact seed achieves? — and the answer is a number, not an
  opinion;
  (ii) **at the five negative-margin coordinates, h9 included, no rule can
  separate and no rule failure is informative about separation**; what those rows
  measure is the gap;
  (iii) **h9 remains the most interesting coordinate for the economy
  measurement** precisely because it is the one the exact route cannot price, so
  the gap there is the only primal number obtainable at all.
- **RW-A6 (the reading, pre-declared before any rule number exists).**
  (a) **A rule separates at a positive-margin coordinate** → the coordinate is
  certified by a candidate that cost no map and no extraction, with EC-A13's
  primal-half fence verbatim and Theorem E6.4's member-not-set caveat beside the
  verdict. It certifies membership under the declared belief, field, valuation
  and observation contract, and nothing else.
  (b) **No rule separates** → the per-arm gaps are filed, typed per EC-A11 as
  **candidate-failure, never class-failure**. A rule's L is one lower bound among
  many; its failure says nothing about what another candidate could do, and the
  exact negative is **not** obtainable from a rule failure. Where the exact
  negative does hold it comes from the filed Q^H and U, as at h9, and the row
  says which source it came from.
  (c) **At h9 specifically** → the deliverable is the gap `g` per arm, printed as
  an exact rational beside `Q^H`, with the RW-A3(iii) labels and the sentence
  that the coordinate's separation verdict was settled by Corollary E4.1(3)
  before any rule ran.
- **RW-A7 (scope: all four actions, all nine coordinates, with the negative-margin
  rows typed).** Run the rule arms at **all four root actions** — the gap
  `g(a) = Q^H(a) − L_rule(a)` is a measurement at every action, while the
  **separation verdict is computed only at H-optimal actions**, since the design
  asserts the certified action lies in `argmax_H`. Run them at **all nine
  coordinates**: the marginal cost is one budgeted walk each and the comparison
  across coordinates is the economy measurement's point. At the five
  negative-margin coordinates the **separation column is a receipt of Corollary
  E4.1(3), not a measurement** — it reports a theorem about what no candidate can
  do — while the **gap column remains a genuine measurement**; both typings print
  on the same row and are never conflated. This is the brief's own reading of the
  already-filed exact negatives, confirmed and extended to h9.
- **RW-A8 (freezes and results discipline).** **FREEZE 49 — the n4 economy
  carrier**: the coordinate list (the nine n4 coordinates in freeze-44(f) order),
  the action set (all four per coordinate, ascending), the arm subset (P1..P4,
  with X printed structurally unavailable where the cap bars it and T/R out of
  scope), the **rule argument list `(record, legal)` of RW-A1(i)–(ii)**, the
  canonical run order and the results-file column set (arm, action, L_rule, Q^H,
  gap, margin, separation cell with its typing, reached count, walk-steps,
  residual). Freezes 1–48 stand, 44 at v2, 36 at v2, 38–40 reserved and
  untouched; **no freeze is amended**, because freeze 44(b)'s contract already
  binds every `walk`-based evaluator and (c) enumerates that design's six rather
  than closing the set — a pointer marker records this at (c)'s site. Results
  discipline: every row carries its arm's information-consistency argument
  (RW-A1(iii)), the by-construction notices of RW-A2(ii), the RW-A3(iii) label
  pair, and the EC-A11 fence; and nothing here is promoted — the gaps, the
  verdicts and the h9 exact negative are exploratory and quotable as results only
  by brief amendment adding them to a verifier receipt.

**What the build owes this section.** `policy_value_by_rule` with the closed
argument list; (RW-R2) run and green **before** any h9 number is quoted; the four
arms ported with their consistency arguments; freeze 49's columns; and — first,
because it needs no code — h9's coordinate verdict line of RW-A3(i), which is a
result the pass left on the table.

## The fusion tax: inbox 016 adjudicated (2026-08-14)

**Adjudicator:** walt-math. **Object:**
`exchange/inbox/016-decision-sparse-nonanticipativity-taxes.md` — *"Decision-Sparse
Exact Solving: Nonanticipativity Taxes and a Compositional Plan Calculus for
Straight Texas 42", v0.1*, received 2026-08-14 in answer to
`exchange/outbox/016-cheap-upper-witness-handoff.md`, hand-ferried and
UNADJUDICATED on arrival. The note self-classifies its claims as *exact result*,
*[certificate] schema* or *research proposal*; those labels are the sender's and
carry no status here until confirmed below. **Tier:** exploratory throughout,
without exception. Nothing in this section is promoted, nothing is quotable in a
brief, a dispatch, [FINDINGS](FINDINGS.md) or any claim-tier page except by brief
amendment adding it to a verifier receipt, and an external note is never imported
as an axiom (TRUST-01). **Basis:** the errata under DS-A17 (Lemma E3 with its
§3.1 object and §3.4 conditions (C1)–(C4), Lemma E4 and Non-theorem E4′ with
DS-A27's semantic restatement, Corollary E4.1, Corollary E3.2, Theorems
E6.3/E6.4/E6.5, Lemma E7); DS-A1, DS-A13, DS-A14, DS-A15, DS-A16, DS-A20, DS-A28,
DS-A30; SEP-A3..SEP-A19 (freezes 36 v2 and 37); N4-A1..N4-A20 (freezes 44 v2 and
45); EC-A1..EC-A14 (freeze 46); T1-A1..T1-A12 (freeze 47, Theorem T1-draw,
Proposition T1-blind); LD-A1..LD-A13 (freeze 48, Theorem LD, Corollary LD-fold);
RW-A1..RW-A8 (freeze 49); P-A1, P-A19, P-A21, PG-A8, PG-A13, R-A2, R-A18, F7 and
NO-RESCUE; and first-hand reading, at adjudication time, of
`walt-strat/src/info.rs` (`walk`, `Particle::weight`, the field share
`p.weight *= q(1, legal.len())`, `InfoPartition`'s `BTreeMap<Vec<Domino>,
InfoStateId>` record index), `walt-core/src/rules.rs` (`legal_plays`,
`Trick::winner`), and the filed rows of
`walt-factory/results/separation_n4_2026-08-14.txt` and
`walt-factory/results/rule_economy_n4_2026-08-14.txt`. Rulings
**FT-A1..FT-A22**; four lemmas, two propositions and two corollaries delivered
below with full proofs; **reserved freeze 38 is FILLED** at FT-A17 (v1, scoped)
and **freeze 50** is fixed at FT-A18. The prefixes `FT-A`/`FT-Q` and every name
below were grep-checked unused at adjudication time.

**The engine facts everything below rests on**, stated once so no proof
re-derives them.

- **Every seat plays exactly one tile per trick**, so at a coordinate of grade g
  the focal seat has exactly g decisions, of which the last is over a
  single-tile hand and therefore **forced**. "At most" is not needed: the count
  is exact.
- **The focal information state is the complete public record.** Freeze 26's
  observation contract is the full public record, and `InfoPartition` keys states
  by `Vec<Domino>` — the plays since the kernel decision point with the root
  action first (freeze 36(b)). Two records of different length are therefore two
  different information states, and the frontier states of §6 are mutually
  exclusive **because of this contract**, not by general principle.
- **The legal-action set is a function of (focal hand, led context) alone**, both
  known to the focal seat: `legal_plays(decl, hand, led)`. Hence `A(I)` is
  constant across the latent worlds of `I`, as the note assumes.
- **The field is uniform over each seat's own legal set, per world**:
  `p.weight *= q(1, legal.len())` where `legal` is computed from *that
  particle's* hand. The arrival weight of a record is therefore world-dependent,
  and is **not** proportional to a count of worlds. Lemma FT-post below is the
  consequence, and it is the sharpest trap in the note's §13.
- **At every n = 4 coordinate the leader offset from focal is asserted 0**
  (freeze 45): the focal seat leads the root trick, holds 4 tiles, and
  |X| = 34,650 = 12!/(4!)³.
- **The declared direction is the count-free trick differential; the reporting
  convention is the count convention**, bridged by freeze 26's
  `Q_diff = 2·Q_count − grade` at the reporting boundary only (freeze 37(c)).

**Headline — seven findings, stated before the rulings.**

1. **There is no field-convention mismatch, and this was the thing most likely to
   be one.** The note's §2.1 defines `V_a^C(ω)` without naming the field, but §2
   fixes `σ_{-m}` as standing data and §2.2 evaluates lawful policies against it;
   read that way `V_a^C` **is** the errata's `V*_a` of §3.1 exactly — root action
   held, world revealed, same fixed field. Our `U` is not a revealed max over
   field behaviour, and neither is the note's. (C1) and (C4) survive intact.
   FT-A2.
2. **Every claim labelled "Exact result" is sound mathematics; three are
   incomplete as stated.** Theorem 6.1, its corollaries, and Theorem 10.1 all use
   a hypothesis the note never names — that the arrival law at the next-decision
   frontier does not depend on the focal continuation policy. It is true here,
   for a reason specific to the frontier being the *next* decision, and it is
   delivered as **Lemma FT-arrive**. FT-A7, FT-A11.
3. **The ladder is one rung shorter than the note says, at every coordinate.**
   The last focal decision is forced, so revealing the world before it buys
   nothing: **Lemma FT-trunc** gives `U_a^(N−1) = Q^H(a)`. After an opening lead
   the ladder has five taxes, not six. At grade 4 it has **exactly two**
   (**Corollary FT-grade4**), so computing `U^(1)` together with the already-filed
   `Q^H` determines the entire layer decomposition and the note's Experiment 15.4
   and open question §18.4 are **answered outright at these coordinates**. FT-A6.
4. **The note's §10.2 heuristic is rejected and replaced by a theorem that says
   something stronger.** "A hand-only or action-independent upper feature is
   unlikely to be selective enough" is a guess; **Proposition FT-flat** proves
   that a `b`-independent upper feature can never beat `U_a^C` **at all**. This is
   the exact upper-side twin of Proposition T1-blind, and it closes the pair of
   negatives that bracket the sandwich: action-blind below excludes nothing,
   action-blind above shaves nothing. FT-A11.
5. **The carrier is five coordinates, not four, and the fifth is the interesting
   one.** RW-A5 already established that the binding margin is negative at five
   of the nine n = 4 coordinates; h9 — pip 4, hand `[30 41 54 61]` — is the fifth
   and by far the largest, `−2116837/8870400`. h9 is **NOT PRICED on the exact
   route** (its extraction map measured 517,562,322 states against
   `P_max v2 = 192,000,000`), but the first-layer tax needs only the **depth-one**
   frontier partition and the revealed continuations below it, never the full
   extraction map. **h9 is therefore in scope for this probe although it is out of
   scope for the primal pipeline**, which makes it the single most informative
   unit in the carrier. FT-A18.
6. **At three of the five, the binding competitor is tied with `a⋆` in `Q^H`, so
   the first layer must carry the entire fusion gap or nothing.** Computed at
   adjudication time from the filed Tier-1 `R3` rows, the fraction of the
   competitor's own fusion gap that must be shaved to close the pair is
   **1 exactly** at pip 5 `[21 33 53 54]`, pip 0 `[20 30 40 65]` and h9 — because
   `Q^H(a) = Q^H(a⋆)` there — against `12627174/16709317` at pip 3
   `[00 21 32 53]` and `8524657/18853881` at pip 4 `[11 40 43 53]`.
   **Proposition FT-tie** states the general fact. This is a fence on the
   experiment's reading, not a prediction of its outcome. FT-A20.
7. **Freeze 38 is filled.** The note supplies exactly the three things DS-A13
   reserved the number for — a cut language, a validity obligation and a cut
   ordering — and its cuts are **correctly typed**: a block merge identifies
   action variables inside one information state and touches neither the fiber
   nor any world's mass, which is what separates a cut from a declared exclusion
   remnant. Freeze 38 v1 is scoped to the reveal-delay ladder and first-frontier
   partitions; feature penalties, multi-stage penalties and adaptive block search
   are explicitly **not** in it and re-enter as v2. FT-A17.

---

### The received claims, adjudicated one by one

Sender's label on the left, this section's verdict on the right. Every verdict is
justified in the ruling named, and no verdict promotes anything above the
exploratory tier.

| Received claim | Sender's label | Verdict | Reason (one line) |
|---|---|---|---|
| Thm 3.2 fusion-gap identity | Exact result | **CONFIRMED** | max→min exchange over a nonempty finite lawful class; `E[V^C]` is `ρ`-free. FT-A3 |
| Cor 3.3 fusion-tax upper witness | Exact result | **CONFIRMED** | Immediate; E6.4's member-not-set caveat travels with any verdict built on it. FT-A4 |
| §4.1–4.2 local partition geometry | Exact representation | **CONFIRMED-WITH-REPAIR** | Sound; two typings amended — `Ω_a` is action-independent here, and `X_I`/`μ_I` are policy-independent only at depth one. FT-A5 |
| §4.3 decision-relative distance `κ_a(T)` | (implicit) | **BLOCKED** | A definition with an undeclared `cost(·)`; unusable in a probe until the cost model is frozen. FT-A5(iii) |
| Prop 5.2 ladder monotonicity | Exact result | **CONFIRMED** | Nested admissible policy classes. FT-A6(i) |
| Prop 5.3 finite termination | Exact result | **CONFIRMED-WITH-REPAIR** | True; sharpened to `U^(N−1) = Q^H` by Lemma FT-trunc, and `N` is exact here, not an upper bound. FT-A6(ii) |
| "at most six layers after an opening lead" | Exact combinatorial bound | **CONFIRMED-WITH-REPAIR** | Five, not six: the seventh-trick decision is forced. FT-A6(iii) |
| Thm 6.1 first-layer values | Exact result | **CONFIRMED-WITH-REPAIR** | Correct once Lemma FT-arrive, the common-`A(I)` fact and the full-record contract are named as hypotheses. FT-A7 |
| Def 6.2 / `Δ^(1) = Σ_I δ_I` | Exact result | **CONFIRMED** | Frontier states are mutually exclusive under freeze 26's contract. FT-A7(iv) |
| Prop 6.3 regret form | Exact result | **CONFIRMED** | The avg-of-max term is `b`-free. FT-A7(v) |
| Cor 6.4 zero-tax criterion + tie-set warning | Exact result | **CONFIRMED** | Both directions correct; the warning is E6.5(G2)'s exposed-face criterion in local form. FT-A8 |
| Prop 7.1 binary tax | Exact result | **CONFIRMED** | Verified in both displayed forms. FT-A9(i) |
| Prop 8.2 fusion core ≤ \|A(I)\| ≤ 7 | Exact result | **CONFIRMED** | Correct; sharpened here to ≤ tiles in hand at that decision — ≤ 3 at the grade-4 frontier. FT-A9(ii) |
| Def 8.3 regret matrix | Exact result | **CONFIRMED** | Restatement of Prop 6.3. FT-A9(iii) |
| Prop 9.1 exact block-merge cost | Exact result | **CONFIRMED** | `max+max ≥ max` on disjoint mass; and the cut is correctly typed. FT-A10 |
| Thm 10.1 glued simple-function upper | [Certificate] schema | **CONFIRMED-WITH-REPAIR** | Theorem sound under Lemma FT-arrive; its *use* is a schema with a per-`B` proof obligation. FT-A11(i) |
| §10.2 "action-independent features unlikely to be selective" | (prose) | **REJECTED AS STATED — REPLACED** | Not a theorem; Proposition FT-flat proves the stronger exact fact. FT-A11(ii) |
| Thm 11.1 regret minorant | [Certificate] schema | **CONFIRMED** | Pointwise domination under a min. FT-A12(i) |
| Cor 11.2 event-form action cover | [Certificate] schema | **CONFIRMED** | Instance of Thm 11.1 with `g = η_b·1_{E_b}`. FT-A12(ii) |
| §11.3 local sandwich for a regret event | [Certificate] schema | **CONFIRMED-WITH-REPAIR** | Sound; `L_c(ω)` is a pointwise lower bound on a world-informed continuation and must never be called a primal witness. FT-A12(iii) |
| §11.4 no free summation | (prose) | **CONFIRMED** | Matches our overlap discipline exactly. FT-A12(iv) |
| Thm 12.1 zero-mean penalty | Exact result | **CONFIRMED** | Centering then pointwise max; valid for every centered `λ`. FT-A13(i) |
| Prop 12.2 exact recovery | Exact result | **CONFIRMED** | Correct — and it buys no compute, since it needs the full `q̄` table. FT-A13(ii) |
| §12.3 feature-based penalties | Research proposal | **CONFIRMED as a valid family; BLOCKED as a probe** | Validity holds for every `θ`; the centering is an *exact* equality, so no float and no sampled expectation may appear. FT-A13(iii) |
| §12.4 multi-stage martingale penalty | Research proposal | **BLOCKED** | Correctly self-labelled; the conditional induction is unwritten. FT-A13(iv) |
| Thm schema 13.1 fixed-field plan value | [Certificate] schema | **CONFIRMED-WITH-REPAIR** | It is Lemma E4 plus public-leaf pasting; the repair is Lemma FT-post, and "public" must be read as focal-information-measurable. FT-A14 |
| §13.2 composition operations | [Certificate] schema | **CONFIRMED** | "Candidate maximum outside the evaluator" is DS-A14/DS-A27 verbatim; the adversarial fold is valid because `σ_{-m}` is supported on legal plays. FT-A14(iii) |
| §13.3–13.5 partial laydowns, straight-count lift, "34 on two trumps" | Schema / proposal | **BLOCKED, with the count fence restated** | E-A2: a count re-entry voids every form-keyed record wholesale; the note's own §13.4 caution is correct and is adopted. FT-A14(v) |
| §14 combined decision proof | (synthesis) | **CONFIRMED** | Exactly Theorem E6.4, with its member-not-set caveat mandatory in the statement. FT-A15 |
| §15 Experiment 15.1 | Open experiment | **BUILDABLE — GRANTED as the FT family** | Freeze 50, receipts FT-R1..FT-R6, budgets under freeze 44(b)–(e) v2. FT-A18 |
| §15.2/15.3 explain and compile the tax | Open experiment | **GRANTED (15.2) / BLOCKED (15.3)** | 15.2 is reporting over data 15.1 already produces; 15.3 needs proved event minorants that do not exist yet. FT-A19(vii), FT-A21 |
| §16 trick-1 program | Research proposal | **BLOCKED** | Three obligations named at FT-A21; nothing here makes the 399,072,960-world `q` table reachable. |
| §17 laydown-catalogue report | Implementation-relative report | **CONFIRMED FAITHFUL, two amendments** | Numbers match LD-A11/LD-A12; (LD-R4) is still owed, and our caveat is stronger than the note's. FT-A16 |
| §18.9 mechanize the theorem layer | Research proposal | **NOTED, with a tier fence** | A kernel proof of the abstract finite model is a kernel-tier fact about *that model* and promotes nothing about the engine. FT-A22(iv) |

---

### Lemma FT-arrive (the frontier arrival law is policy-independent) — delivered here

Fix a coordinate, a root action `a`, the declared belief `β` and the declared
field `σ_{-m}`. Let `T` be the focal seat's **next** decision after the root play,
and let `I_T` be the focal information state at `T` (undefined if play terminates
first). Then the joint law of `(ω, I_T)` under `β` and `σ_{-m}` **does not depend
on the focal continuation policy**, and for every world `ω` and record `I`,

  `μ_I(ω) = β(ω) · Π_j (1 / |legal_j(ω)|)`,

the product taken over the field plays of the record in order, each `legal_j(ω)`
being that seat's legal set in `ω` at that point.

*Proof.* Between the root play and `T` the focal seat takes no action, by the
definition of `T` as its next decision. Every intervening move is a field move,
and `σ_{-m}` selects it from the moving seat's own legal set with probability
`1/|legal|`, a quantity determined by `ω` and the play so far and in no way by
the focal seat's future behaviour. The record reached is therefore a function of
`(ω, field choices)` alone, and its probability is the displayed product; `β` is
fixed. The information state at `T` is the record (freeze 26's observation
contract), so the joint law of `(ω, I_T)` is as claimed. ∎

**Why it is load-bearing and not bookkeeping.** Theorem 6.1's two displayed
formulas, Definition 6.2's additivity and Theorem 10.1 all weight frontier
quantities by one and the same `μ_I(ω)` while comparing controllers with
*different* powers — treatment `C`, treatment `C^(1)`, and an arbitrary lawful
policy. That comparison is only legitimate because all three induce the same
arrival law. **The lemma fails at the second frontier and beyond**: the mass
arriving at a depth-two state depends on what was chosen at depth one. This is
exactly why the note's §9 warns that merge costs interact, and it is why the
product-of-lattices picture of §4.2 is a description of the *constraint space*
and never a decomposition of the value.

### Lemma FT-trunc (the ladder truncates one rung early) — delivered here

Let `N` be the number of focal decisions remaining after the root play. Suppose
that at every focal decision of depth `k+1, …, N` the legal set is a singleton in
every positive-mass world. Then

  `U_a^(k) = Q^H(a)`.

In particular, because the focal seat's last decision is over a one-tile hand and
is therefore always forced,

  **`U_a^(N−1) = Q^H(a)`,  hence  `Δ_a^(N) = 0` identically.**

*Proof.* A `C^(k)`-admissible controller is required to be information-state
measurable at depths `1, …, k` and is free to condition on `ω` at depths
`k+1, …, N`. If every decision at those depths has exactly one legal action in
every positive-mass world, that freedom selects nothing: the controller's action
is determined by legality alone, so its behaviour coincides with that of the
lawful policy agreeing with it on depths `1, …, k`. The field cannot exploit the
revelation either, since `σ_{-m}` is fixed and reads only its own seat's
information. Hence the `C^(k)` class and the lawful class induce the same set of
achievable value vectors, and their maxima agree. For the second statement, the
focal seat holds one tile at its last decision, so `|A(I)| = 1` there in every
world. ∎

**Corollary FT-grade4 (at grade 4 the ladder has exactly two rungs).** At an
n = 4 coordinate the focal seat holds four tiles and leads the root trick
(freeze 45), so `N = 3` and

  `C^(0) ⊇ C^(1) ⊇ C^(2) = H`,  with  `U_a^C − Q^H(a) = Δ_a^(1) + Δ_a^(2)`.

Consequently **one computation of `U_a^(1)` determines the whole decomposition**,
because `Δ_a^(1) = U_a^C − U_a^(1)` and `Δ_a^(2) = U_a^(1) − Q^H(a)` with both
`U_a^C` and `Q^H(a)` already filed per root action in
`separation_n4_2026-08-14.txt`. The note's Experiment 15.4 — "compute `U^(2)` if
`U^(1)` is not enough" — is **vacuous at this carrier**: `U^(2)` is `Q^H`, which
we have. Its open question §18.4, how much is gained by `C^(2)` and `C^(3)`, is
answered exactly and for free once Experiment 15.1 runs. ∎

*(After an opening lead the same lemma gives `U_a^(5) = Q^H(a)`: five taxes, not
six. The note's boxed "at most six focal-decision layers" is true but never
tight.)*

### Proposition FT-flat (an action-blind upper feature can never shave) — delivered here

In Theorem 10.1's setting, suppose the upper feature does not depend on the
frontier action: `B_I(ω,b) = B_I(ω)` for every legal `b`, and `B_I(ω) ≥ q_I(ω,b)`
for every legal `b` and every positive-mass `ω`. Then the bound the theorem
returns is **at least `U_a^C`**:

  `T_a + Σ_I max_b Σ_ω μ_I(ω) B_I(ω,b) = T_a + Σ_I Σ_ω μ_I(ω) B_I(ω) ≥ U_a^C.`

*Proof.* With `B_I` independent of `b` the inner maximum is attained at every
action and the `max` drops out. The hypothesis `B_I(ω) ≥ q_I(ω,b)` for **every**
legal `b` gives `B_I(ω) ≥ max_b q_I(ω,b) = m_I(ω)` pointwise. Summing against the
nonnegative weights `μ_I(ω)` and adding `T_a` gives at least
`T_a + Σ_I Σ_ω μ_I(ω) m_I(ω)`, which is `U_a^(0) = U_a^C` by Theorem 6.1. ∎

**What it says, and why it is the twin of T1-blind.** Proposition T1-blind proved
that a lower witness valid at every root action excludes nothing, because
`U_a ≥ Q^H(a) ≥ L`. Proposition FT-flat proves the mirror image one level down: an
upper feature that does not discriminate the *frontier* action recovers none of
the first-layer tax and returns a number no better than the witness we already
have. Together they fence both sides of the sandwich with the same lesson —
**a witness must be conditioned on the decision it is trying to price.** The note's
§10.2 asserts the weaker, vaguer form of this ("unlikely to be selective
enough"); the proved form is unconditional and is what belongs in a design.

*Note the scope precisely.* The proposition constrains `B` as a function of the
**frontier** action `b`. It says nothing against an upper feature that is
`b`-dependent but crude, and nothing against the root-action conditioning that
`U_a` already carries through `μ_I` and the frontier set — that conditioning is
what makes the whole construction action-conditioned in Lemma E3's sense.

### Proposition FT-tie (a tied competitor demands an exact relaxation) — delivered here

Let `a⋆` be an H-optimal root action priced by a primal witness attaining the
lawful maximum, `L_{a⋆} = Q^H(a⋆)` (Corollary E4.1(2)), and let `a ≠ a⋆` be a
competitor with `Q^H(a) = Q^H(a⋆)` — a tie inside `Opt^H`. Let `U_a^(k)` be any
relaxation value with `Q^H(a) ≤ U_a^(k)`. Then

  `L_{a⋆} ≥ U_a^(k)  ⟺  U_a^(k) = Q^H(a)`,

that is, the pair closes **only** if the relaxation is exact at `a`; and along the
reveal-delay ladder this is `Σ_{j>k} Δ_a^(j) = 0`.

*Proof.* `L_{a⋆} = Q^H(a⋆) = Q^H(a) ≤ U_a^(k)`, so `L_{a⋆} ≥ U_a^(k)` forces
equality throughout. Conversely equality gives the test. The ladder form follows
from `U_a^(k) − Q^H(a) = Σ_{j>k} Δ_a^(j)` (Definition 5.4 with Lemma FT-trunc's
truncation). ∎

**Where it bites, computed at adjudication time from the filed `R3` rows.** Of
the five negative-margin n = 4 coordinates, the binding competitor is tied with
`a⋆` at three:

| Coordinate | `a⋆` / binding `a` | required shave `U_a − L_{a⋆}` | competitor's own gap `U_a − Q^H(a)` | fraction required |
|---|---|---|---|---|
| pip 3 `[00 21 32 53]` (h0) | 53 / 00 | `300647/2138400` | `16709317/89812800` | `12627174/16709317` |
| pip 5 `[21 33 53 54]` | 53 / 54 (and 54 / 53) | `9557/554400` | `9557/554400` | **1 (tied)** |
| pip 4 `[11 40 43 53]` (h6) | 40 / 11 | `8524657/479001600` | `6284627/159667200` | `8524657/18853881` |
| pip 0 `[20 30 40 65]` (h12) | 20, 30, 40 mutually | `364429/9979200` | `364429/9979200` | **1 (tied)** |
| pip 4 `[30 41 54 61]` (h9) | 41 / 54 (and 54 / 41) | `2116837/8870400` | `2116837/8870400` | **1 (tied)** |

Every entry was recomputed here from the filed per-root `Q^H` and `U` columns and
reproduces the filed `margin` column exactly. **The reading this forces**: at the
three tied coordinates the experiment's question is not "does the first layer
shave enough" but "is the fusion gap entirely first-order", a strictly binary
question whose negative answer is as informative as its positive one (F7). At the
two untied coordinates a partial shave can still close the pair, and the fractions
above are what it must exceed. Both readings are pre-declared here, before any
`δ_I` exists.

### Lemma FT-post (the frontier posterior is not uniform) — delivered here

At a frontier information state `I` with positive mass, the conditional belief
over its latent worlds is

  `ν_I(ω) = μ_I(ω) / p_I`,  `p_I = Σ_ω μ_I(ω)`,

with `μ_I` as in Lemma FT-arrive. This is uniform on `X_I` **only if** the field's
legal-set sizes `|legal_j(ω)|` are constant over `ω ∈ X_I` along the record.
They are not constant in general, because `legal_plays` is computed from the
moving seat's own hand, which varies across the fiber.

*Proof.* Immediate from Lemma FT-arrive's product form; the counterexample is
structural — a record in which one seat follows context `c` has `|legal|` equal
to that seat's holding in `c`, which differs between worlds of `X_I`. ∎

**The trap this closes, and it is the sharpest one in the received note.** The
note's Theorem schema 13.1 hands a residual position at a public leaf `h` to
"another [certificate] `L_h`". If `L_h` is obtained by treating the residual
position as a **fresh coordinate with the branch's standard uniform belief over
its fiber** — which is exactly how every walt coordinate is built (freeze 45,
freeze 47) — then `L_h` prices the wrong measure and the composition is **void**:
it is neither an upper nor a lower bound on the residual value under the
posterior. The composition is valid when `L_h` is (i) the value of an exhibited
lawful continuation policy evaluated **inside the same walk**, under the carried
weights, or (ii) a bound that holds **pointwise in every world of the leaf**, in
which case the measure is irrelevant. Nothing else composes. **Binding: any FT- or
plan-calculus artifact that pastes a residual witness prints which of (i) or (ii)
it used, in place, on the row.**

### Corollary FT-conv (taxes scale with the convention; verdicts do not) — delivered here

Let `v ↦ αv + c` with `α > 0` be a reconvention of the valuation — freeze 26's
bridge `Q_diff = 2·Q_count − grade` is the case `α = 2`, `c = −grade`. Then every
`q_I(ω,b)`, `m_I(ω)`, `Q^H`, `U` and `L` transforms the same way, every local tax
`δ_I` and every layer tax `Δ^(k)` transforms as `δ ↦ αδ`, and every separation
verdict is unchanged.

*Proof.* `Σ_ω μ_I(ω)(αm + c) = α Σ μ m + c·p_I` and
`max_b Σ_ω μ_I(ω)(αq + c) = α max_b Σ μ q + c·p_I`; subtracting cancels `c·p_I`
and leaves `αδ_I`. Verdict invariance is freeze 37(c)'s argument, an affine map
with positive slope preserving `≥`. ∎

**Binding consequence.** The internal evaluators are differential; the filed
margins are count. A `δ` computed internally is a **differential** tax and must be
halved — exactly, as a rational — before it is compared with a count-convention
margin. **A tax quoted in one convention against a margin in another is void**,
and the results file states the convention of every tax column in its header.

---

- **FT-A1 (typing, tier, vocabulary, and what this section is).** The received
  note is **ACCEPTED IN LARGE PART**: its central identity and its first-layer
  mathematics are correct, three of its exact results need a hypothesis named,
  one prose claim is replaced by a stronger theorem, and its two schema families
  carry obligations that no probe may skip. Everything is exploratory. DS-A1
  binds: this section says **witness**, **receipt** and **necessary outer
  profile**, never the forbidden word; the received note uses that word freely and
  it appears here only inside bracketed attribution to it. *Support ≠ belief*,
  *feasible ≠ reachable*, *possible ≠ probable* are typed distinctions and are
  kept. Both outcomes of every gate below are results (F7); a receipt failure is
  stop-and-report, never a patch (NO-RESCUE). No number in the received note
  enters walt as evidence: the four decimals its §15 quotes are rounded forms of
  our own exact rationals, they were re-derived here from the filed rows, and
  **the exact rationals are the objects** (P-A19: no float anywhere).
- **FT-A2 (the setups are the same setup; the field convention CONFIRMED, and
  one typing amended).** Three clauses.
  (i) **No `σ_{-m}` mismatch.** The note's §2 fixes a field policy `σ_{-m}` as
  standing data, and §2.2 defines `α_ρ(ω) = E[U | ω, ρ, σ_{-m}]` exactly as errata
  §4.1 does. Its §2.1 omits `σ_{-m}` from the display of `V_a^C`, but the only
  reading consistent with §2 and with the pointwise inequality it asserts is the
  one in which the field is that same fixed policy — which is errata §3.1's
  `V*_a` verbatim. **Read that way, `U_a^C` and our `U_a` are the same object**,
  and freeze 37(a)'s identification of `U_a` with the per-root-action column of
  the revealed summary is untouched. The omission is filed as an editorial defect
  in the received note, not a mathematical one, and a builder quoting §2.1 must
  restore the field.
  (ii) **What would have been a mismatch, and is not.** Had `V_a^C` been a maximum
  over field behaviours as well, the object would be the cooperative-field corner
  of T1-A2(ii) — still a valid upper bound but a strictly weaker one — and had it
  been perfect-information minimax it would be barred outright by (C4) and by
  T1-A2(iii), which moves both dials at once and bounds nothing in either
  direction. Neither is what the note wrote.
  (iii) **`Ω_a` is amended.** The note writes `Ω_a` for "the physical worlds
  compatible with `s` and root action `a`". In this engine the focal seat's own
  play removes no world: the fiber is the set of splits of the unseen tiles and is
  **identical for every root action**. Condition (C2) requires the *same world set*
  on both sides of a separation, and freeze 37(d) fixes the belief as uniform over
  the full enumerated fiber. Read `Ω_a = X_B` for every `a`; the action-conditioning
  lives in the continuation and in the frontier masses, never in the carrier.
- **FT-A3 (Theorem 3.2, the fusion-gap identity: CONFIRMED).** The proof is
  correct and every step is licensed here: `E_β[V_a^C]` does not depend on `ρ`, so
  it passes through the `max` and inverts it to a `min`; expectation is linear;
  `r_a(ω,ρ) = V_a^C(ω) − α_ρ(ω) ≥ 0` pointwise is exactly Lemma E3's pointwise
  strategy-fusion inequality, already ours. **Two hypotheses are named because
  they are used**: `R_H(a)` is nonempty (a legal action always exists), and the
  maximum defining `Q^H(a)` is attained — true because the lawful pure-policy
  class at a coordinate is finite, which is also what errata §4.1 assumes when it
  writes `max` rather than `sup`. **What the identity buys us**, and it is the
  note's genuine contribution: it converts "tighten the relaxation" — an
  open-ended instruction — into "prove a lower bound on one expectation", a
  finite obligation with a named object. The minimising `ρ` is any H-optimal
  policy, so the identity is also a statement about the H solve we already run.
- **FT-A4 (Corollary 3.3, the fusion-tax upper witness: CONFIRMED, with E6.4
  attached).** If `0 ≤ Γ_a ≤ U_a^C − Q^H(a)` is **proved**, then
  `Q^H(a) ≤ U_a^C − Γ_a` and the separation test becomes `Γ_a ≥ U_a^C − L_{a⋆}`.
  Three clauses. (i) `U_a^C − Γ_a` is a valid upper witness in Lemma E3's sense
  and inherits (C1)–(C4) unchanged; a `Γ` computed against a different field,
  belief, world set or valuation voids it. (ii) **Theorem E6.4's member-not-set
  caveat travels verbatim** with every verdict built this way: the non-strict test
  certifies a *member* of `Opt^H`, never the set. (iii) A `Γ` that is *measured*
  rather than proved — for instance a `Δ^(1)` computed by a run whose budget
  stopped — is not a `Γ`. Freeze 44's contract already says a stopped walk retains
  no partial fold, and the same rule binds here: **there is no partially-computed
  tax.**
- **FT-A5 (§4, the geometry between C and H: CONFIRMED as a representation, two
  amendments, and `κ_a(T)` BLOCKED).**
  (i) **The nonanticipativity reading is right and is the note's best idea.**
  Treatment `C`'s illegitimate power is *local*: it may give different actions to
  latent copies `a_{I,ω}` of one information state. Requiring `a_{I,ω} = a_{I,ω′}`
  at every `I` recovers exactly the lawful class, and imposing it at some `I` and
  not others interpolates. Every intermediate value is an upper bound on `Q^H(a)`
  because every lawful policy is admissible in every such relaxation. This is
  sound and it is a better starting point than partitioning the world space,
  which is the reframe the outbox asked for.
  (ii) **Amendment: the product `𝔓 = Π_I Part(X_I)` is the constraint space, not
  a value decomposition.** At depth one, `X_I` and `μ_I` are policy-independent
  (Lemma FT-arrive) and the local taxes add exactly (Definition 6.2). At depth two
  and below, which worlds reach `I` and with what mass depends on what was chosen
  above, so the lattice coordinates are not independent and **no additivity may be
  read off the product**. The note half-states this in §9 for merges within one
  `I`; it is stated here for depth, which is where a builder would be misled.
  (iii) **`κ_a(T)` is BLOCKED as a probe object.** Definition 4.3 minimises
  `cost(Π)` over relaxations meeting a target, with `cost` described as
  "representation, counting, or solve cost" and never defined. A quantity whose
  minimand is undeclared cannot be computed, compared or receipted, and a
  cost-model claim is precisely what N4-A16 and SEP-A19(b) forbid reading off a
  traversal observable. The definition may be quoted as a definition; **no number
  may be reported as a `κ`** until a cost model is frozen by its own adjudication.
- **FT-A6 (§5, the ladder: 5.2 CONFIRMED; 5.3 CONFIRMED-WITH-REPAIR; the layer
  counts corrected against this engine).**
  (i) **Proposition 5.2 is CONFIRMED.** Every `C^(k+1)`-admissible policy is
  `C^(k)`-admissible, since the latter withholds the world no longer and so
  imposes no constraint the former does not already meet; every lawful policy is
  admissible in every `C^(k)`. Nested classes, nested maxima, and the chain
  `U^(0) ≥ U^(1) ≥ … ≥ Q^H` follows.
  (ii) **Proposition 5.3 is CONFIRMED, and REPAIRED to a shorter ladder.** The
  note's argument — once revelation is withheld through every remaining focal
  decision there is no action left for it to act on — is correct, and it needs
  the additional fact that the field cannot exploit the revelation either, which
  holds because `σ_{-m}` is fixed and reads only its own seat's information. The
  repair is **Lemma FT-trunc**: the last focal decision is over a one-tile hand
  and is forced, so `U^(N−1) = Q^H` already and `Δ^(N) = 0` identically. The
  general form — any suffix of forced decisions truncates the ladder — is proved
  with it and will matter more at deeper grades, where follow-suit forces many
  late decisions.
  (iii) **The two quoted layer counts are corrected.** "At most six later focal
  decisions after an opening lead, hence `U^(6) = Q^H`" becomes **`U^(5) = Q^H`**;
  "at most three with four tiles remaining, hence `U^(3) = Q^H`" becomes
  **`U^(2) = Q^H`**. In this engine the counts are moreover **exact**, not upper
  bounds, because every seat plays exactly one tile per trick and the hand always
  runs to completion. The note's claim-ledger row "Reveal-delay monotonicity and
  finite termination — remains: engine mapping from game histories to
  focal-decision count" is hereby discharged by that sentence.
  (iv) **Corollary FT-grade4 is the operational payoff** and is delivered above:
  at grade 4 the ladder is `C^(0) ⊇ C^(1) ⊇ C^(2) = H`, so the gap is
  `Δ^(1) + Δ^(2)` with `Δ^(2) = U^(1) − Q^H(a)` free once `U^(1)` exists.
  **Experiment 15.4 is vacuous at this carrier and is not commissioned.**
- **FT-A7 (§6, the first fusion layer: CONFIRMED-WITH-REPAIR — three hypotheses
  named, none of them decoration).**
  (i) **The missing hypothesis is Lemma FT-arrive.** Theorem 6.1 weights the
  `C^(0)` value and the `C^(1)` value by the same `μ_I(ω)`. That is legitimate
  only because the arrival law at the frontier is common to both — and to every
  lawful policy, which Theorem 10.1 also needs. The note never says so. The lemma
  is delivered above with its proof and with the statement of where it fails
  (depth ≥ 2).
  (ii) **The second hypothesis is that `A(I)` is common across `X_I`.** The note
  asserts it from "the same focal hand and public record"; in this engine it is
  the code fact that `legal_plays` reads only `(decl, hand, led)`. Confirmed, and
  recorded as an engine fact rather than a general one — an engine that abstracted
  the record could violate it.
  (iii) **The third is mutual exclusivity of the frontier states**, which
  Definition 6.2's additivity needs. It holds because the focal seat is at exactly
  one record at time `T` and the information state **is** the record (freeze 26).
  Under an abstracted observation contract two arrivals of different depth could
  share a label and the taxes would not add. Named so a successor does not inherit
  the additivity without the contract.
  (iv) **With those three named, Theorem 6.1, Definition 6.2 and
  `Δ^(1) = Σ_I δ_I` are CONFIRMED.** The `C^(0)` formula is Bellman at the
  frontier of the world-informed problem; the `C^(1)` formula is the same sum with
  the max moved outside the world sum; the difference of the maxima is the
  avg-of-max minus max-of-avg Jensen gap the note names.
  (v) **Proposition 6.3 is CONFIRMED**: the avg-of-max term is `b`-free, so
  subtracting the best action value is minimising expected regret.
  (vi) **A simplification specific to this carrier, and it is not small.** `T_a`,
  the terminal-frontier mass, is **zero at every n = 4 coordinate**: the focal
  seat holds four tiles, plays one at the root, and therefore always acts again.
  The probe asserts `T_a = 0` rather than computing it, and the assertion is
  contentful — it fails if the root is ever the focal seat's last tile.
- **FT-A8 (Corollary 6.4 and its tie-set warning: CONFIRMED, and it is our own
  exposed-face criterion in local form).** Both directions check: the regret
  summands are nonnegative, so a zero weighted sum forces zero regret in every
  positive-mass world for the minimising action, which is then a common optimum;
  conversely a common optimum has zero regret everywhere. **The warning attached
  to it — that the criterion must use the complete optimal action sets, since one
  arbitrarily tie-broken treatment-`C` optimiser can manufacture a conflict that
  another optimiser on the same optimal face avoids — is not a caution, it is
  Theorem E6.5's clause (G2) in local form**: the stopping test is a search over
  the whole optimal face, an unlawful returned optimiser licenses only adding a
  cut, and a negative answer requires proof. The concordance is exact and is worth
  recording as such: an independent derivation reached our own hardest-won
  procedural rule. **Binding for any FT probe**: the `δ_I = 0` test is computed
  from complete argmax sets, never from freeze 26's least-domino-index tie rule,
  which exists to make the *authority* deterministic and is not a statement about
  the optimal face.
- **FT-A9 (§7 and §8: CONFIRMED, with two sharpenings for this carrier).**
  (i) **Proposition 7.1 (binary tax) is CONFIRMED**, both forms verified:
  `δ̂_I = E[d₊] − max(0, E[d]) = min{E[d₊], E[(−d)₊]} = ½(E|d| − |E d|)` using
  `d = d₊ − (−d)₊` and `|d| = d₊ + (−d)₊`. The reading — treatment `C` harvests
  both advantage masses, a lawful common action must surrender the smaller — is
  correct and needs no pairing of worlds.
  (ii) **Proposition 8.2 (fusion cores are small) is CONFIRMED and sharpened.**
  The proof is right: an empty full intersection yields, for each action `b`, a
  positive-mass world in which `b` is not optimal, and a minimal subset of those
  at most `|A(I)|` worlds is a core. The sharpening is that `|A(I)|` is bounded by
  the **tiles in hand at that decision**, not by 7: at the grade-4 frontier the
  focal seat holds three tiles, so **every decisive fusion core has at most three
  worlds**, and follow-suit often makes it two. At trick 1 the bound is 7. The
  note's open question §18.2 — are most cores binary or ternary — is therefore not
  open at grade 4: ternary is the ceiling.
  (iii) **Definition 8.3 (regret matrix) is CONFIRMED** as the correct
  quantitative object, and the note's remark that a graph of pairwise world
  conflicts discards genuine multi-action structure is right — its three-world
  example with `A₁ = {a,b}`, `A₂ = {b,c}`, `A₃ = {a,c}` is a valid witness that
  pairwise agreement does not imply a common optimum.
- **FT-A10 (§9, block gluing: Proposition 9.1 CONFIRMED, and the cut is CORRECTLY
  TYPED — which is the clause that lets freeze 38 be filled at all).**
  (i) **The merge cost is exact.** `Q_{B₁∪B₂}(b) = Q_{B₁}(b) + Q_{B₂}(b)` by
  disjointness, and `max Q_{B₁} + max Q_{B₂} ≥ max(Q_{B₁} + Q_{B₂})`, so
  `c_I(B₁,B₂) ≥ 0` and equals the upper-value reduction caused by requiring one
  common action across the union. The non-additivity caveat is correct: merge
  costs interact and the value must be recomputed after each merge, whereas taxes
  at mutually exclusive frontier states add exactly.
  (ii) **The typing, and it is the load-bearing ruling of this section.** A block
  merge **identifies action variables** inside one information state. It removes
  no world, changes no world's mass, and asserts nothing about reachability. It is
  therefore a *cut* in Theorem E6.5's sense — a constraint on the relaxation,
  satisfied by every lawful policy — and **not** a declared exclusion remnant,
  which is what a "cut" applied to worlds would be and which carries none of
  E6.5's guarantees. The received note never confuses the two, and its §4.1
  formulation in terms of action variables is exactly why. **Binding: every FT
  artifact prints, per cut, the sentence "this cut identifies action variables at
  one information state; the fiber and every world's mass are untouched."**
  (iii) **Validity is discharged for this whole family at once.** Every lawful
  policy chooses one action per information state, hence satisfies every block
  identification inside that state, hence satisfies every cut in the family. E6.5's
  per-cut validity obligation is met by a single argument for the ladder and the
  block partitions; **a cut family outside this one re-enters with its own proof.**
- **FT-A11 (§10, the glued simple-function upper theorem: CONFIRMED-WITH-REPAIR;
  and §10.2's prose REJECTED and REPLACED).**
  (i) **Theorem 10.1 is CONFIRMED** once Lemma FT-arrive is named: a lawful policy
  chooses one `b(I)` at `I`; its per-world continuation value is at most
  `q_I(ω,b(I))` by the pointwise strategy-fusion inequality (Lemma E3 applied at
  the frontier node) and hence at most `B_I(ω,b(I))`; summing against `μ_I` and
  bounding by the max over `b` gives the result. The note's own emphasis — the
  decisive order is `max_b Σ_ω`, not `Σ_ω max_b` — is exactly right and is the
  whole content of first-layer gluing. **Its use is a schema**: each `B_I` needs a
  proof of pointwise domination, exact and per action, before any number it
  produces is a witness.
  (ii) **§10.2's closing prose is REJECTED as stated.** "A hand-only or
  action-independent upper feature is unlikely to be selective enough" is a
  heuristic and would be quoted as a result the first time a design needed it.
  **Proposition FT-flat replaces it with a theorem**: a `b`-independent `B` returns
  a bound `≥ U_a^C` and therefore never improves on the witness already filed. Note
  what the theorem does *not* say — it constrains dependence on the **frontier**
  action, not on the root action, and `U_a` is action-conditioned through `μ_I`
  regardless.
  (iii) **The candidate feature list of §10.2 — boss loss, ruff availability, lead
  retention, trump expenditure, entry creation and destruction, control
  incompatibility, count exposure, follower supply — is a research proposal and is
  filed as one.** Each entry becomes usable only as a proved pointwise bound with
  exact expectations; the note's suggestion that such expectations may come from
  exact weighted model counting rather than world enumeration is the right shape
  and is **exactly the target DS-A21/Corollary E5.2 left open** — E5 rules out a
  small universal atom algebra, not a compact weighted-counting circuit for a
  declared purpose. Any build here is designed against a declared purpose with
  bit-exact agreement against enumeration at a reduced grade, never against the
  atom formulation (DS-A5).
- **FT-A12 (§11, regret minorants: CONFIRMED, with one vocabulary repair that is a
  soundness fence).**
  (i) **Theorem 11.1 is CONFIRMED**: `0 ≤ g_I ≤ R_I` pointwise gives
  `Σ μ g ≤ Σ μ R` for each `b`, and the minima are ordered. Composing with
  `Δ^(1) = Σ_I δ_I ≤ U_a^C − Q^H(a)` makes `Γ_a = Σ_I min_b Σ_ω μ_I(ω) g_I(ω,b)`
  a valid proved tax in Corollary 3.3's sense.
  (ii) **Corollary 11.2 is CONFIRMED** as the instance `g = η_b·1_{E_b}`, and the
  economy it advertises is real: at most `|A(I)|` obligations per information
  state — at most three at the grade-4 frontier — instead of one per world.
  (iii) **§11.3's local sandwich is CONFIRMED, with the vocabulary repaired, and
  the repair is not cosmetic.** The note asks for "a lawful or otherwise valid
  local lower witness `L_c(ω)`". In our types that object is a **pointwise lower
  bound on a world-informed continuation value** `q_I(ω,c)` — for instance the
  per-world value of a fixed lawful continuation, which is `≤ q_I(ω,c)` by Lemma
  E3 — and it is **not a primal witness**, which is a `β`-integrated fixed-policy
  value at the root. The danger is the adjacency: **the very same world-informed
  number that is legitimate here as an upper bound `B_b(ω)` is a soundness bug if
  it is installed as `L` at the root** (Non-theorem E4′, whose minimal witness
  shows the separated action can be strictly worse than the rejected one). One
  object, two slots, opposite verdicts. **Binding: every FT row naming a
  per-world bound prints its slot — `upper on q(·,b)` or `lower on q(·,c)` — and
  no per-world world-informed quantity is ever carried out of a frontier and used
  as a root `L`.**
  (iv) **§11.4's overlap discipline is CONFIRMED** and coincides with ours: two
  minorants of the same `R_I(ω,b)` may not be added unless their supports are
  disjoint, or a single combined `g` is proved pointwise below `R_I`, or they
  arise at mutually exclusive information states or at successive exact ladder
  layers. The third case is licensed here by Definition 5.4 with Lemma FT-trunc.
- **FT-A13 (§12, zero-mean penalties: CONFIRMED, with an exactness obligation, and
  §12.4 BLOCKED).**
  (i) **Theorem 12.1 is CONFIRMED.** For any `λ` with `Σ_ω μ_I(ω) λ_I(ω,b) = 0`
  for every legal `b`, centering rewrites the chosen action's value and the
  pointwise max dominates it; taking the max over common actions and summing gives
  the bound, which then bounds `Q^H(a)` because `Q^H(a) ≤ U^(1)`.
  (ii) **Proposition 12.2 is CONFIRMED**: `λ_I(ω,b) = q_I(ω,b) − q̄_I(b)` is
  centered by construction and collapses the penalised max to `p_I max_b q̄_I(b)`,
  recovering `U^(1)` exactly. **What it buys is understanding, not compute** — the
  exact `λ` requires the full `q̄` table, i.e. the thing `U^(1)` needed anyway. Its
  value is that it exhibits hard gluing and the dual penalty as one object, which
  is what makes cheaper penalty families meaningful.
  (iii) **§12.3's feature penalties are a VALID FAMILY and a BLOCKED PROBE.** Every
  `θ` gives a valid one-stage penalty, so validity is free and the coefficients may
  be optimised without risking soundness. The obligation is arithmetic and
  absolute: **the centering condition is an exact equality**, so `E_{ν_I}[φ]` must
  be an exact rational computed over the full positive-mass set. A sampled or
  decimated expectation breaks centering and voids the bound — this is (C2) in a
  new costume, and it is the failure mode a "shadow price" framing invites. No
  float, ever (P-A19).
  (iv) **§12.4 is BLOCKED**, correctly self-labelled. Global zero-mean centering is
  insufficient at multiple decisions; the increment must be a martingale
  difference relative to the focal information filtration. The note's interim
  contract — stagewise conditional centering — is accepted as the safe form, and
  **nothing multi-stage is designable until the induction theorem is written and
  adjudicated.**
- **FT-A14 (§13, the plan calculus: CONFIRMED-WITH-REPAIR; three concordances and
  one trap).**
  (i) **Theorem schema 13.1 is CONFIRMED as a repackaging of Lemma E4, and it is a
  useful one.** The composite of a lawful partial policy `π` with residual lawful
  policies pasted along **publicly distinguishable** leaves is a single lawful
  policy, because distinct public records are distinct information states under
  freeze 26; its `β`-integrated value is at least `E[g(H_τ) + L_{H_τ}]`; Lemma E4
  then gives `L(P) ≤ Q^H(a)`. The four hypotheses the note lists are exactly the
  ones the proof uses.
  (ii) **The trap is Lemma FT-post, delivered above.** A residual witness priced at
  a *fresh coordinate under uniform belief* is not a valid `L_h`, because the
  posterior at a public leaf carries the field's legal-set-size weights. Since
  every walt coordinate is built with uniform belief over its fiber, this is the
  natural mistake and it is void, not merely loose. The two admissible forms —
  evaluate inside the same walk under carried weights, or prove the residual bound
  pointwise in every world of the leaf — are binding.
  (iii) **Three concordances worth recording, because independent arrival at our
  own rules is evidence about the rules and not about the sender.** §13.2's
  *candidate maximum* clause — "compare completed lawful plan values **outside**
  the fixed-policy evaluator; do not select a plan separately in each hidden world,
  that would reintroduce strategy fusion" — is DS-A14 and DS-A27's semantic
  invariant word for word. §13.2's *public case split* — branching on observations
  the focal player actually receives, never on hidden ownership — is
  information-consistency. §13.1's guard `G` "expressed only in the focal hand,
  declaration, and public record" is our information state exactly.
  (iv) **The adversarial fold is valid here, and the reason is worth stating.**
  Replacing the field expectation by a minimum over legal field responses yields a
  number that lower-bounds the fixed-field value **because `σ_{-m}` is supported on
  legal plays** — a minimum over a set containing the support is at most the mean
  (T1-A2(i)). This is what makes Theorem T1-draw's and Theorem LD's field-free
  guarantees usable as primal witnesses under the declared field, and it is the
  one direction that survives; the cooperative corner does not (T1-A2(ii)–(iii)).
  Read "public" throughout §13 as **focal-information-measurable**: at a fixed
  coordinate the focal hand is constant across the fiber so the two coincide, and
  the distinction becomes real only for a plan quantified over hands.
  (v) **§13.3–13.5 are BLOCKED as mathematics and welcome as language.** The
  identification of a drawing hand as the terminal case with `g = 7` is faithful to
  Theorem T1-draw and Theorem LD. The partial-laydown hierarchy — guarantee `k`,
  force a small family of public residual contracts, price each — is a genuine
  target and is exactly the shape LD-A13 left open. **§13.4's own caution is
  adopted verbatim**: a count-free plan theorem does not prove its count-decorated
  analogue. E-A2 is sharper still — a count re-entry voids every form-keyed record
  **wholesale, never extended** — and DS-A16 records the single exception, that a
  library policy remains a valid witness source under re-evaluation while its
  count-free verdicts do not survive. The "34 on two trumps" programme of §13.5 is
  a research proposal about family knowledge and is filed as one, with the standing
  fence that nothing here is a claim about bidding or about how anybody plays.
- **FT-A15 (§14, the combined decision proof: CONFIRMED, and it is Theorem E6.4).**
  `L_{a⋆} ≥ U_a^C − Γ_a` for every competitor certifies `a⋆ ∈ Opt^H`; strict
  inequalities give uniqueness. This is our own separation theorem with a proved
  tax subtracted from the upper side, and it inherits every attached condition:
  (C1)–(C4), the member-not-set caveat printed **in the statement and not the
  commentary**, and DS-A14's structural assertion that `L` came from a
  fixed-policy evaluator satisfying DS-A27's invariant.
- **FT-A16 (§17, the laydown restatement: FAITHFUL, with two amendments).** The
  note reports 301 laydowns per pip declaration, 2,107 hand/declaration pairs, no
  deal in which all four hands are laydowns, and an exhibited three-laydown deal.
  **Every one matches our record**: LD-A11's closing note ((LD-R1) held at exactly
  301; phase 2 returned no four-laydown deal; the maximum is three and is
  exhibited) and LD-A12's Corollary LD-fold (the predicate is declaration-fold
  invariant, hence 301 per declaration and 2,107 pairs). Two amendments.
  (i) **The outbox's "294" and the note's "301" are different objects and must
  never be reconciled by arithmetic.** 294 = 42 × 7 is Theorem T1-draw's inner
  class, freeze 47's closed carrier; 2,107 = 301 × 7 is the full lay-down
  catalogue of Theorem LD. LD-A3 demoted (Z1) to sufficient-not-necessary and
  quantified the containment as 42 of 301, strict at every declaration. A reader
  who sees both numbers must be told which object each names.
  (ii) **The note's caveat is right and ours is stronger.** It asks that the
  catalogue definition, the producing script, the commit hash, the witness deal
  and the exhaustive no-four search be preserved before promotion — that is
  reproducibility, and it is necessary. **T1-A12 and LD-A10(ii) demand more**:
  every statement is proved relative to walt's *implementation* of the rules, and
  the tier order, `DOUBLE_TOP`, the effective-incidence subtraction, the compelled
  follow and `threat` must be checked against the rules package by a reader before
  any of it leaves walt — a check no receipt computed by that same implementation
  can perform. **(LD-R4) remains owed** and is the cheapest available probe of that
  risk, not a discharge of it. Until then "2,107" is licensed by Corollary LD-fold
  as a proof plus one receipted count, not by seven receipted counts.
- **FT-A17 (FREEZE 38 — FILLED, v1, scoped; the reservation is discharged).**
  DS-A13 reserved 38 for "the gluing-cut language, the validity-proof obligation,
  and the cut ordering", and SEP-A18, T1-A11, LD-A9 and RW-A8 each confirmed it
  reserved and untouched. The received note supplies all three, correctly typed
  (FT-A10(ii)), so the number is now spent. **FREEZE 38 — the gluing cut, v1.**
  **(a) The cut language.** A *cut* is a declared partition `Π_I` of the latent
  world set `X_I` of one named focal information state `I`, constraining the
  relaxed controller to one common action per block. It is an identification of
  action variables. It removes no world, alters no world's mass, and asserts
  nothing about reachability; a construct that excludes worlds is a **declared
  exclusion remnant** and is not a cut (Theorem E6.5's typing).
  **(b) The canonical family, v1.** (1) The reveal-delay ladder: `C^(k)` is the
  one-block partition at every focal frontier of depth `≤ k` and the singleton
  partition below, with `C^(0) = C` and `C^(N−1) = C^(N) = H` by Lemma FT-trunc.
  (2) Within the **first** frontier only, arbitrary partitions `Π_I`, priced by the
  exact merge cost `c_I(B₁,B₂) = v_I(B₁) + v_I(B₂) − v_I(B₁∪B₂)`.
  **(c) The validity obligation, and its discharge for (b).** Every cut must be an
  information equality satisfied by every lawful policy. For family (b) it is
  discharged once and for all: a lawful policy chooses one action per information
  state and therefore satisfies every block identification within that state. Any
  cut outside (b) — feature-induced, adaptive across layers, or defined on
  anything other than a partition of one `X_I` — re-enters with its own proof and
  its own freeze version.
  **(d) The cut ordering, which is why this is a determinism freeze.** Layers
  ascending, `k = 1, 2, …`; within a layer, frontier information states in
  **ascending observation-record order** (freeze 36(b)'s lexicographic order over
  the canonical ascending domino index); within a state, actions in ascending
  domino index; block merges, where used, in **descending exact merge cost** with
  ties broken by the smallest record in the block. The order is declared before the
  run and never chosen by result (DS-A33/DS-A34's discipline).
  **(e) The stop rule.** Theorem E6.5(G2)'s exposed-face criterion, instantiated as
  Corollary 6.4's zero-tax test computed from **complete** argmax sets. A single
  tie-broken optimiser is not evidence of a conflict and freeze 26's least-index
  tie rule is not used here (FT-A8).
  **(f) Arithmetic and reporting.** Exact rationals throughout, accumulated over a
  common integer denominator; no float (P-A19). Taxes are reported in the **count
  convention**, obtained from the differential evaluators by the exact inverse of
  freeze 26's bridge; by Corollary FT-conv a differential tax is **twice** its
  count-convention value, and a tax compared against a margin in the other
  convention is void.
  **(g) Scope of v1, stated so a successor knows what is not frozen.** Feature
  penalties (§12.3), multi-stage/martingale penalties (§12.4), adaptive block
  search beyond the first frontier, and any cost model for `κ_a(T)` are **NOT** in
  freeze 38 v1. They re-enter as **freeze 38 v2** fixed by a later adjudication —
  versioning content by ruling, as freeze 36 v2 and freeze 44 v2 already do. No
  number is reused: freezes 1–49 stand, 36 at v2, 44 at v2, **39 and 40 remain
  reserved and untouched**.
- **FT-A18 (Experiment 15.1: BUILDABLE AGAINST THIS ENGINE AS-IS — GRANTED as the
  FT family; FREEZE 50 fixes its carrier).** The experiment is admitted because it
  needs no new authority, no new relaxation type, and no object the engine cannot
  already produce. Six clauses.
  (i) **What it computes.** For each carrier coordinate and each *binding*
  competitor action `a`: the frontier data `μ_I(ω)` and `q_I(ω,b)`, the local taxes
  `δ_I`, the layer tax `Δ_a^(1) = Σ_I δ_I`, and `U_a^(1) = U_a^C − Δ_a^(1)`; then
  the decision question `L_{a⋆} ≥ U_a^(1)`, and — free, by Corollary FT-grade4 —
  the residual `Δ_a^(2) = U_a^(1) − Q^H(a)`.
  (ii) **Why the machinery already exists.** `walk` carries exactly the required
  objects: a bag of particles with weights, the field share
  `p.weight *= q(1, legal.len())` (which *is* `μ_I(ω)`, Lemma FT-arrive), and an
  `obs` record that is the frontier information state's key, indexed by
  `InfoPartition`'s `BTreeMap<Vec<Domino>, InfoStateId>`. The two evaluators
  needed are (1) the existing revealed per-world solve, read at the child of a
  frontier node to give `q_I(ω,b)`, and (2) a *glue-one-then-reveal* walker that
  is lawful at the first focal node after the root and world-informed below it.
  Both are derived views of the semantic state; the continuation memo, if used, is
  keyed by the **projected** state and is a cache, never an authority (freeze
  36(d)'s pattern).
  (iii) **Budgets: no new constant is fixed.** Freeze 44(b)'s contract already
  binds every `walk`-based evaluator: `B = 10,000,000,000` walk-steps per
  (coordinate, action), charge-then-descend at `bag.len()` on entry, `Option`
  return, and **on exhaustion no partial fold of any kind is retained** — which
  here means there is no partial tax and no partial `U^(1)`. The frontier partition
  is asserted against `P_max v2 = 192,000,000`; it is a depth-one object and is
  expected far below that, and the assertion is contentful.
  (iv) **h9 is IN SCOPE although it is NOT PRICED.** N4-A16(iv) and RW-A4 record
  h9's extraction map at 517,562,322 states, above `P_max v2`, so no primal
  pipeline runs there. The FT probe needs no extraction map: it needs the
  depth-one frontier and the revealed continuations below it, and `U` and `Q^H` are
  already filed at h9 for every root action with the authority gate MET. **h9's
  NOT PRICED label stands verbatim and is not weakened by this** (RW-A3(iii): the
  labels never merge); what changes is that the coordinate carrying the branch's
  largest exact negative becomes measurable on the dual side.
  (v) **Filed values enter as a frozen table, per SEP-A14(ii).** `U_a` and `Q^H(a)`
  for every carrier coordinate and root action are transcribed into the probe
  source as a frozen table carrying the provenance line *"quoted from
  `separation_n4_2026-08-14.txt`, exploratory tier"*, **never re-parsed from the
  results text**, which is not a machine-readable interface. The coordinate
  identity is asserted first in freeze 45's form — grade, declaration, hand and
  pool as canonical ascending domino-index tile lists, leader offset 0,
  `|X| = 34,650` against `kernel.count()`, freeze-7/23 enumeration order, kernel
  rebuilt in-run and asserted equal — because an equality of values at coordinates
  not shown to be the same coordinate is not a cross-check.
  (vi) **FREEZE 50 — the fusion-tax probe carrier.** **(a)** The carrier is the
  **five negative-margin n = 4 coordinates**, in this canonical order: pip 3
  `[00 21 32 53]`; pip 4 `[11 40 43 53]`; pip 5 `[21 33 53 54]`; pip 4
  `[30 41 54 61]` (h9); pip 0 `[20 30 40 65]` — declaration pip ascending, then
  hand ascending lexicographically by canonical domino index. **(b)** Per
  coordinate the run is over the **binding pairs only**: every `(a⋆, a)` whose
  filed margin is negative, `a⋆` ranging over the filed H-argmax set and `a` over
  the competitors, in ascending domino index on each side. **(c)** The frontier
  emission format: one row per `(coordinate, a, I)` carrying the record as an
  ascending-domino-index play list, `p_I`, `|X_I|`, `|A(I)|`, `δ_I` as an exact
  rational in the count convention, the complete argmax set of
  `Σ_ω μ_I(ω) q_I(ω,b)`, and — where `δ_I > 0` — one minimal fusion core with its
  worlds printed by fiber index under freeze 7/23. **(d)** The per-coordinate
  summary: `T_a` (asserted 0), `Δ_a^(1)`, `U_a^(1)`, `Δ_a^(2)`, the decision cell
  `L_{a⋆} ≥ U_a^(1)`, and the fraction-required column of Proposition FT-tie.
  **(e)** Belief and field are **not** re-declared: they are freeze 26 and freeze
  37(d), cited unchanged, uniform over the full enumerated fiber with no decimation
  anywhere inside any `L`, `U` or tax ((C2)). **(f)** **No library entry is written
  at any coordinate** (freeze 45). **(g)** The freeze-set digest travels on every
  record; a digest mismatch is corruption and the cache is discarded entire
  (freeze 41, DS-A30).
- **FT-A19 (the receipts, since "by construction" is not a receipt).** Mandatory,
  in the PG-A8 style, with the non-receipts named as such.
  (i) **(FT-R1) the reconstruction receipt — the one that earns the section.**
  Assert `T_a + Σ_I Σ_ω μ_I(ω) m_I(ω) = U_a` exactly, against the frozen filed
  `U_a` of FT-A18(v). **Contentful and strong**: it ties the frontier decomposition
  to the already-receipted revealed summary, and it fails on any error in the
  arrival weights, the record keying, the frontier detection or the continuation
  values. A mismatch is stop-and-report; it is a bug in the probe or in Theorem
  6.1's hypotheses, never a finding about the game (R-A18, NO-RESCUE).
  (ii) **(FT-R2) the mass receipt.** Assert `Σ_I Σ_ω μ_I(ω) = 1` exactly and
  `T_a = 0`, the latter by asserting the focal seat has a further decision in every
  positive-mass world. **Contentful**: it fails if any field branch is dropped or
  double-counted.
  (iii) **(FT-R3) the sandwich receipt.** Assert `U_a^(1) ≥ Q^H(a)` against the
  frozen filed `Q^H(a)`. **Contentful**: it is the only check that the glue was
  applied at the right node and in the right direction. Note in place that
  `δ_I ≥ 0` and `Δ^(1) ≥ 0` are **arithmetic remarks, not receipts** — they cannot
  fail given the formulas — unless computed by the two independent paths of
  (FT-R4).
  (iv) **(FT-R4) the two-path receipt.** Compute `U_a^(1)` twice: once as
  `U_a^C − Σ_I δ_I` from the frontier table, and once directly by the
  glue-one-then-reveal walker, and assert exact equality. **Contentful**, because
  the two computations are independently written and share only the rule algebra.
  (v) **(FT-R5) the optimal-face receipt.** For a declared deterministic sample —
  the first ten frontier states in freeze-50(c) order at each coordinate — assert
  Corollary 6.4 both ways: where `δ_I = 0`, the complete argmax sets intersect;
  where `δ_I > 0`, print a minimal fusion core of size `≤ |A(I)| ≤ 3` and assert
  its argmax intersection is empty while every proper subset's is not.
  **Contentful**, and it is the direct test of FT-A8's discipline.
  (vi) **(FT-R6) the reduced-grade cross-check.** At a declared grade-2 and
  grade-3 coordinate where treatment `H` completes cheaply, compute `U^(1)` by the
  probe and assert `U^(0) ≥ U^(1) ≥ Q^H` with all three exact, and assert
  `U^(1) = Q^H` wherever Lemma FT-trunc predicts the ladder has already collapsed
  (grade 2: `N = 1`, so `U^(0) = Q^H` and the first tax is identically zero).
  **Contentful and it tests the lemma, not just the code** — a nonzero `Δ^(1)` at
  grade 2 falsifies Lemma FT-trunc or the implementation, and either is
  stop-and-report.
  (vii) **Experiment 15.2 is GRANTED as reporting**, not as a separate build: the
  per-`I` rows of freeze 50(c) already carry `Pr(I)`, `δ_I`, the argmax set and the
  cores, and the concentration question (§18.1) is answered by sorting them. No
  claim is attached to the ranking beyond the exploratory tier, and P-A21 binds —
  **no distribution measured at grade 4 is quoted for trick 1 or for the opening.**
- **FT-A20 (both outcomes pre-declared, before any `δ_I` exists).**
  (a) **`U_a^(1) ≤ L_{a⋆}` at any binding pair** → the first exact gluing cut in
  the branch closes a pair that Corollary E4.1(3) proved no candidate could ever
  close. The verdict is filed with Theorem E6.4's member-not-set caveat verbatim,
  with the freeze-38 cut sentence of FT-A10(ii), and — at the three tied
  coordinates — with Proposition FT-tie's note that closure there means the fusion
  gap was **entirely first-order**, an exact statement about this coordinate and
  nothing wider.
  (b) **`U_a^(1) > L_{a⋆}` everywhere** → the filed object is the exact pair
  `(Δ_a^(1), Δ_a^(2))` per binding pair, which by Corollary FT-grade4 is the
  **complete** layer decomposition of the fusion gap at grade 4. That answers the
  note's §18.4 at this carrier and specifies exactly what a second-layer or
  feature-penalty cut must beat. This is a result under F7, filed as one, not a
  null.
  (c) **A budget stop** → declared stop, no partial fold retained (freeze 44), no
  partial tax reported, printed as a stop and never as a finding (R-A18).
  (d) **(FT-R1) or (FT-R4) disagreeing** → the most informative outcome available,
  and pre-declared as such: Theorem 6.1's hypotheses or the implementation are
  wrong, nothing is claimed, and the disagreeing exact rationals are printed
  (F7, NO-RESCUE).
- **FT-A21 (§16, the trick-1 program: BLOCKED, with its three obligations named).**
  The pipeline is coherent and is the right destination; it is not designable now.
  (i) The exact `q_I(ω,b)` table at trick 1 ranges over 399,072,960 worlds and up
  to seven actions per frontier state, and nothing in the note reduces it — the
  note says so itself. (ii) The only routes that avoid it are Theorem 10.1 with
  proved pointwise `B`, and Corollary 11.2 with proved regret events; **neither
  has a single proved instance in this game yet**, and Proposition FT-flat now
  proves that the cheapest instinct — a `b`-independent feature — cannot help at
  all. (iii) Exact counting of any event mass must be exhaustive: no decimation
  inside a witness, (C2), the discipline (T1-R3) already applies at trick 1.
  **What is designable at trick 1 today remains what T1-A5 granted** — Theorem
  T1-draw's membership half and Corollary T1-ruff's exclusion half, which need no
  relaxation at all — plus arm B's corner gaps, which are the specification of what
  a tighter relaxation must beat. Experiment 15.3 is BLOCKED with §16.
- **FT-A22 (fences, carried obligations, and what this section is not).**
  (i) **The R-A2 fence, mandatory in the results header**, unchanged: no object
  produced by this probe is an identity-bearing witness of anything; reachability
  is a proof-irrelevant proposition; the carrier is the void-free capacity fiber
  whose members are **FEASIBLE and never reachable** (P-A1). The N4-A8 real-deal
  fence travels with every carrier coordinate verbatim — the hands and pools come
  from rob's receipt corpus, **the belief does not**, the voids the play record had
  already revealed are deliberately discarded, and the void-filtered column
  licenses nothing.
  (ii) **Not claimed, printed in place:** nothing about points or marks (the
  valuation is count-free `trick_diff`; E-A2's boundary, and a count re-entry voids
  every form-keyed record wholesale); nothing about bidding; nothing about how real
  opponents play; no growth law and no quantity measured at grade 4 quoted for
  trick 1 or for the opening (P-A21); and no cost claim read off any traversal
  observable (SEP-A19(b), N4-A16).
  (iii) **DS-A28(ii) remains CARRIED.** Corollary E4.1's filing as errata §4.3 is
  still owed at the next errata amendment; this section does not amend the errata
  and does not discharge it. **A second obligation is added to the same queue**:
  Lemma FT-arrive, Lemma FT-trunc, Proposition FT-flat, Proposition FT-tie, Lemma
  FT-post and Corollary FT-conv, together with the confirmed first-layer
  mathematics, belong in the errata as a new §9 at that amendment. Until then they
  live here, and `walt/CENSUS-RULINGS.md` is their only authority.
  (iv) **On mechanization (§18.9), a tier fence.** The fusion-gap identity, the
  first-layer formulas, the block costs, the regret minorants and the one-stage
  penalty theorem are small finite-model statements and are legitimate Lean
  targets. **A kernel proof of the abstract model is a kernel-tier fact about that
  model and promotes nothing about walt's engine**, whose claims remain
  exploratory and remain hostage to T1-A12's implementation-versus-corpus risk.
  Tiers are never blurred by a proof of a neighbouring statement.
  (v) **The load-bearing risk, named so it is watched.** Every hypothesis this
  section discharged from the engine — one focal decision per trick, the record as
  information state, `A(I)` from `(hand, led)`, the field's per-world uniform
  share, leader offset 0 — was read from the implementation at adjudication time.
  If the implementation and the rules corpus disagree, the mathematics above is
  still correct and its application here is wrong, and **no receipt inside this
  section can detect it** because every receipt is computed by the same
  implementation. (FT-R6) is the partial guard. The corpus check of T1-A12 and
  LD-A10(ii) is owed before any of this leaves walt.

**What the build owes this section.** The FT probe of FT-A18 over freeze 50's
five coordinates and their binding pairs, with the six receipts of FT-A19, the
freeze-38(d) cut ordering, the freeze-38(f) convention rule, and both outcomes of
FT-A20 pre-printed in the results header. Everything else here is proof and needs
no code. If (FT-R1) or (FT-R4) fails, nothing is claimed and the disagreeing
rationals are reported; if the taxes come back and close a pair, it is the first
gluing-cut separation in the branch and it is checked twice before anything is
said.

### Closing note: the probe returned (2026-08-14, after the run)

**Object:** `walt-factory/examples/fusion_tax.rs` and
`walt-factory/results/fusion_tax_2026-08-14.txt`, uncommitted at adjudication
time; `ci/check.sh` PASS. Nine (coordinate, competitor) units, 12 binding pairs,
all six receipts HELD at every unit, (FT-R6) HELD at both declared reduced grades
as a blocking pre-check before any carrier number existed. **One pair CLOSED**:
h6, pip 4, hand `[11 40 43 53]`, `a⋆ = 40` against competitor `11`. Ten tied
pairs NOT CLOSED, one untied pair (h0) NOT CLOSED. Three questions were referred
back by the build and are ruled at FT-A23..FT-A25; three findings of the run are
typed at FT-A26.

**Re-verified at adjudication time, independently of the run**, from the filed
`R3` rows of `separation_n4_2026-08-14.txt` and the probe's summary lines:

- The h6 closure. `L_{40} = 541161923/239500800`, `U_11^C = 1090848503/479001600`,
  `U_11^(1) = 10260893/4561920`, `Q^H(11) = 535997311/239500800`. The closure is
  **strict**, with surplus `L − U^(1) = 4930081/479001600`. Achieved shave
  `Δ^(1) = 611579/21772800`; achieved fraction of the competitor's own fusion gap
  `13454738/18853881` against the `8524657/18853881` Proposition FT-tie required.
  Both fractions land on the same denominator, which is arithmetic and not an omen.
- **`Δ^(1) + Δ^(2)` equals the filed fusion gap exactly at every one of the 12
  pairs**, checked here at h6, h9 and h12 and by the probe at all of them.
- The three tied coordinates each returned `Δ^(2) > 0`: h2 `9557/554400 −
  Δ^(1)`, h9 `4532503/26611200`, h12 `95917/4989600`.
- The frontier row census: **281,542 rows, of which 12,639 (4.49%) carry
  `δ_I > 0`** and 268,903 carry `δ_I = 0`. Per unit, positive of total: h0
  1,332/16,136; h6 4,041/53,570; h2 216/330; h9 1,296/1,320; h12 1,414/69,512.

---

### Lemma FT-mix (heterogeneous upper witnesses compose) — delivered here

Let `a⋆` be a root action with a primal witness `L_{a⋆} ≤ Q^H(a⋆)` in Lemma E4's
sense. Suppose that for **each** competitor `a ≠ a⋆` there is *some* valid upper
witness `W_a ≥ Q^H(a)` — the `W_a` need not come from one relaxation, one
evaluator, one traversal or one run — and that `L_{a⋆} ≥ W_a` for every `a`.
Then `a⋆ ∈ Opt^H`. If every one of those inequalities is strict, then
`Opt^H = {a⋆}`.

*Proof.* Fix `a ≠ a⋆`. Then `Q^H(a⋆) ≥ L_{a⋆} ≥ W_a ≥ Q^H(a)`, so no competitor
exceeds `a⋆` and `a⋆ ∈ Opt^H`. If the middle inequality is strict then
`Q^H(a⋆) > Q^H(a)` and `a ∉ Opt^H`; if that holds for every competitor, `Opt^H`
has no other member. ∎

**Why it is stated rather than assumed.** Theorem E6.4 is written with **the**
action-conditioned upper witness `U_a`, and a reader may take its quantifier to
range over one relaxation applied uniformly. Nothing in its proof does: the
proof uses only `U_a ≥ Q^H(a)` separately for each `a`. This lemma records the
generalisation explicitly so that a **mixed** proof — some competitors closed by
treatment `C`, others by a first-layer gluing cut — is licensed by a stated
result and is never read as an extension of E6.4 past what E6.4 proves. It is a
one-line lemma; the content is entirely in the four evidentiary conditions of
FT-A25, which is where a composed verdict can actually go wrong.

---

- **FT-A23 (freeze 50(a)'s internal conflict: the ENUMERATION governs, the sort
  clause is STRUCK, the emitted order STANDS; FREEZE 50 v1.1(a)).** The builder
  reported the conflict, implemented the more specific instruction, and printed
  both passages verbatim rather than silently picking a reading. That is the
  ambiguity protocol executed correctly and it is commended in place.
  (i) **The defect is mine and is worse than a typo.** I wrote an explicit
  five-element list *and* a trailing sort rule, and the rule does not generate the
  list. Nor does anything else: the enumerated order is h0, h6, h2, h9, h12, which
  is neither pip-ascending (`0, 3, 4, 4, 5`), nor hand-lexicographic, nor the
  corpus-hand-id order of `separation_n4_2026-08-14.txt` (which would be h0, h2,
  h6, h9, h12). **It is the order in which I discussed the coordinates in the prose
  above it, and it is generated by no rule at all.**
  (ii) **The enumeration governs.** EC-A8's principle decides it: *a freeze is a
  constant, not a rule.* The five-element list **is** the constant; the sort clause
  is a broken derivation of it and is **struck**, not repaired. Repairing it — that
  is, re-sorting the carrier — would change the emitted file for zero
  informational gain.
  (iii) **The emitted order stands; no re-emit on this account.** Nothing numeric
  crosses coordinates: the units are independent, no quantity is formed across
  them, DS-A36 already requires assembly in canonical unit order rather than
  completion order, and no budget stop occurred (the largest charge was
  786,443,140 walk-steps against `B = 10,000,000,000`). The order is presentational
  here. **It would not have been** had a stop occurred, since which units complete
  is a function of the order — which is the whole reason a carrier order is frozen,
  and the reason this defect had to be ruled rather than shrugged at.
  (iv) **FREEZE 50 v1.1, clause (a) only.** The carrier is the five coordinates in
  exactly the enumerated order — pip 3 `[00 21 32 53]` (h0); pip 4
  `[11 40 43 53]` (h6); pip 5 `[21 33 53 54]` (h2); pip 4 `[30 41 54 61]` (h9);
  pip 0 `[20 30 40 65]` (h12) — with **no generating rule**. Content versioned by
  ruling, the freeze-36-v2/44-v2/50-v1.1 pattern; no number is reused.
  (v) **A standing discipline, so this cannot recur.** *A freeze clause states a
  constant **or** a generating rule, never both.* Where a future freeze wants both
  — a rule for a successor to extend and a list for this run to execute — the rule
  is authoritative, the list is printed as a **derived check**, and the two are
  **asserted equal in-run**. An unasserted redundant derivation beside a constant
  is not documentation; it is a second authority for one object, which is the
  DS-A4 defect in freeze clothing (SEP-A3(iii)'s precedent, transposed).
- **FT-A24 (the emission layout: the split is GRANTED, but cut by CONTENT and not
  by size; FREEZE 50 v1.1(c); one new receipt).**
  (i) **50(c)'s mandate is on content emitted, not on file cardinality.** A
  companion file carrying the mandated rows unchanged satisfies it in principle,
  and the proposal is not refused. But splitting a file because it is large is
  cutting along the wrong seam: it leaves the committed artifact incomplete for
  claims and the uncommitted one full of rows that carry none.
  (ii) **The measured seam, computed at adjudication time.** Of 281,542 frontier
  rows, **12,639 carry `δ_I > 0` and 268,903 carry `δ_I = 0`**. A zero row
  contributes exactly zero to `Δ^(1)`, cannot appear in Experiment 15.2's ranking,
  and has no fusion core. The positive support is 4.49% of the rows and fits in
  under 2 MB.
  (iii) **The committed named file carries**: every row with `δ_I > 0` — the
  support of the tax, which is the whole content of Experiment 15.2 — together
  with the complete (FT-R5) material, every receipt, every summary, every pair
  verdict, and every header and fence, unchanged.
  (iv) **Two accounting assertions make the omission auditable, and both are
  contentful under PG-A8.** Per unit: **(1)** the sum of `δ_I` over the *printed*
  rows equals `Δ^(1)` exactly; **(2)** printed rows + suppressed rows equals the
  frontier state count already asserted on the `P_max` admission line. Add, because
  it costs one integer per unit and is Corollary 6.4's actual content: the
  suppressed count split into **forced** states (`|A(I)| = 1`, where `δ_I = 0`
  trivially) and **unforced with a common optimum** (where the zero is E6.5(G2)'s
  exposed-face criterion doing real work).
  (v) **The full table, zero rows included, is the companion: REGENERABLE and NOT
  COMMITTED.** It is a deterministic function of committed inputs; it contains no
  row that carries a claim; and its omitted rows are accounted for exactly by (iv).
  The named file's header carries the regeneration command, the companion's
  SHA-256, and the freeze-set digest. **A 36 MB artifact in every clone forever is
  a cost with no evidentiary return**, and "reproducible from the repository alone"
  is satisfied by a deterministic regeneration command plus a digest, which is
  strictly what the freeze register exists to guarantee.
  (vi) **A declared cap, so a future run cannot smuggle a truncation.** If the
  `δ_I > 0` support ever exceeds **20,000 rows in one unit**, the file carries the
  top 20,000 by descending `δ_I`, ties by ascending record order, plus the residual
  tail's exact count and exact summed `δ` — a **declared** truncation, printed in
  place, with assertion (iv)(1) restated over the full support so the sum still
  reconciles. Never a silent one.
  (vii) **The split is produced by the probe's own emitter in a re-run, never by
  post-processing the emitted text.** Results text is not a machine-readable
  interface (SEP-A14(ii)'s principle, which bars re-parsing in the other
  direction for the same reason). The re-run is deterministic, so this costs one
  pass and buys something: **(FT-R7) the re-emission determinism receipt** —
  every per-unit summary value, every receipt status and every printed `δ_I`
  compared against the first emission and asserted identical. **Contentful**: it
  fails on thread-order leakage, on uninitialised state, on any accidental
  dependence on iteration order. Disagreement is stop-and-report, not a
  re-emission (F7, NO-RESCUE). It is the cheapest determinism probe this run
  affords and it did not exist before.
  (viii) **FREEZE 50 v1.1, clause (c) only.** Content versioned by ruling; no
  number is reused; clauses (a) as amended at FT-A23, (b) and (d)–(g) stand.
- **FT-A25 (cross-probe composition: ADMISSIBLE under four conditions; the missing
  receipt named; the h6 coordinate verdict FILED, corrected upward in strength and
  downward in meaning).** The builder was right not to print this as a verdict
  before it was ruled.
  (i) **The mathematics composes, and Lemma FT-mix above is why.** At h6 the three
  comparisons draw on two different relaxations — competitor 11 by the first-layer
  gluing cut `U^(1)`, competitors 43 and 53 by treatment `C` — and E6.4's proof
  never required one relaxation. No new mathematics is needed; what needs ruling is
  the evidentiary join.
  (ii) **The four conditions on a composed verdict.** **(C-i) Same coordinate**:
  both halves assert the same freeze-45 identity, and the FT probe already rebuilds
  the kernel in-run and asserts it equal. **(C-ii) Same `L`**: one number, one
  provenance — at h6, `541161923/239500800`, receipted in S6h by its `R2` primal
  receipt (HELD) and quoted by FT from the frozen source table under FT-A18(v).
  **(C-iii) Same (C1)–(C4)**: same fixed field, same belief, same world set, same
  utility and count contract — both cite freeze 26 and freeze 37(d) unchanged, with
  no decimation inside any `L`, `U` or tax. **(C-iv) Freeze-set compatibility**,
  and this is the sharp one. The two digests **differ by construction**, because
  this session added freezes 38 and 50. **DS-A30 does not bite**: it governs a
  *stored record* of computed state, for which a digest mismatch is corruption and
  the cache is discarded entire. A number imported as a **frozen source table with
  provenance** is a different object, and SEP-A14(ii) is the governing precedent —
  it imported S6a values computed under an older freeze set on exactly this basis.
  What is required instead is a **freeze-subset assertion**: the freezes the
  imported number depends on — 7/23 enumeration order, 26 authority and bridge and
  tie rule, 37 the upper witness, 44 v2 budgets, 45 coordinate identity — are
  **identical** in both runs, and the freezes this run adds — 38 v1 (cut language
  and ordering) and 50 (a carrier) — touch **no object the imported number depends
  on**: neither changes the kernel, the belief, the field, the valuation or the
  enumeration order. **Printed, never assumed.**
  (iii) **The missing item, named exactly: (FT-R8) the single-roof composition
  receipt.** Wherever a composed verdict is filed, one block carries: the
  freeze-45 coordinate identity, asserted once and rebuilt in-run; the single `L`
  with its provenance and its S6h `R2` status; **one row per competitor** giving
  its upper witness, that witness's **source** (`FT U^(1)` or `S6h U^C, frozen
  table`) and the comparison with its strictness; the (C-iv) freeze-subset
  assertion; and the composed verdict. **Most importantly it asserts that the
  competitor row set is exactly the legal action set at the root minus `a⋆`,
  checked against `legal_plays` at the rebuilt kernel — never against a row count
  in the filed file.** That is the dangerous failure mode and it is the reason this
  needed a receipt rather than a paragraph: **a composed verdict is a universally
  quantified claim over competitors, and a competitor silently absent from an
  imported table would make a false verdict look complete.** Contentful on every
  clause.
  (iv) **The verdict, filed.** At h6 — grade 4, `PipTrump(4)`, hand
  `[11 40 43 53]`, pool `[00 20 22 30 31 42 44 51 52 54 55 64]`, leader offset 0,
  `|X| = 34,650`, freeze-7/23 enumeration — with
  `L_{40} = Q^H(40) = 541161923/239500800`:
  `L − U_11^(1) = 4930081/479001600 > 0` (FT, first-layer gluing cut);
  `L − U_43^C = 23577691/239500800 > 0` (S6h, treatment C);
  `L − U_53^C = 53019679/239500800 > 0` (S6h, treatment C).
  All three strict, and `{11, 43, 53}` is the complete competitor set. By Lemma
  FT-mix, **`Opt^H(h6) = {40}`** — uniqueness, subject to (FT-R8) being emitted.
  (v) **Correction upward: the CLOSED pair row under-claims.** The build printed
  Theorem E6.4's MEMBER-NOT-SET caveat on it. That boilerplate is right for a
  **non-strict** comparison; here `L > U^(1)` **strictly**, and a strict pair
  comparison says something stronger and different — `Q^H(11) < Q^H(40)`, i.e.
  **competitor 11 is excluded from `Opt^H`**. The caveat is not wrong, it is
  under-claiming, and the row should carry the strict form with the exact surplus.
  MEMBER-NOT-SET stays mandatory wherever any comparison is non-strict; at h6 none
  is.
  (vi) **Correction downward, and this is the one that matters.** All four of h6's
  `Q^H` values are already filed — `2.259…`, `2.238…`, `2.146…`, `1.991…` in the
  count convention, with 40 strictly the largest — so **`Opt^H(h6) = {40}` was
  already determined by the authority column before this probe ran.** The composed
  verdict is therefore a **result about the machinery, not a discovery about the
  coordinate**. **Mandatory sentence, printed with the verdict and in any wiki text
  derived from it:** *this coordinate's optimal set was already determined by the
  filed `Q^H` column; what this verdict demonstrates is that the two-sided proof
  architecture now closes here, and that the lever was a gluing cut and never a
  better candidate — which is exactly what Corollary E4.1(3) proved was the only
  lever available.* Without that sentence the row reads as a finding about 42 and
  it is not one. The closure **could** have failed — it did at ten of twelve pairs
  — so the run is a genuine test; the **conclusion** could not have come out
  otherwise, so it is not evidence about the game.
  (vii) **No composed verdict may be filed at h9.** h9 has no `R2` primal receipt:
  its extraction map exceeded `P_max v2` and the primal pipeline never ran, so
  `L_{a⋆} = Q^H(a⋆)` there is Corollary E4.1(2)'s **ceiling**, not a receipted
  primal witness. The builder typed this correctly in the h9 rows and it is
  **ratified**; (C-ii) fails at h9 and composition is barred until a primal witness
  is receipted there.
- **FT-A26 (three findings of the run that are results in their own right, each
  with its fence).**
  (i) **Corollary FT-grade4 held on real data at all 12 pairs**:
  `Δ^(1) + Δ^(2)` equals the filed fusion gap exactly. This is not bookkeeping —
  it is Lemma FT-trunc's `U^(2) = Q^H` confirmed against two independently filed
  numbers, and it retires the received note's Experiment 15.4 at this carrier by
  measurement as well as by proof.
  (ii) **Proposition FT-tie's all-or-nothing threshold was not met at any of the
  ten tied pairs, with the shortfall equal to `Δ^(2)` exactly. Type it honestly:
  that equality is an arithmetic identity, not independent evidence** — given the
  tie and (FT-R3) it cannot fail, since `U^(1) − L = U^(1) − Q^H(a) = Δ^(2)` by
  definition. It confirms the bookkeeping, not the proposition. Nothing could have
  tested the proposition, because a tied pair closing with `Δ^(2) > 0` is
  impossible; FT-tie earns its keep as a **fence on the reading**, which is what it
  was filed as, and the run is the first occasion on which that fence did work.
  (iii) **The tax concentration is wildly non-uniform, which answers the note's
  §18.1 at this carrier.** Fraction of frontier states paying a tax: h9
  1,296/1,320; h2 216/330; h0 1,332/16,136; h6 4,041/53,570; h12 1,414/69,512 —
  from almost every state to one in fifty. The inversion is worth seeing: h12 has
  the largest frontier and the sparsest tax, h9 the smallest frontier, the densest
  tax and the largest gap. **Three fences, all binding.** P-A21: no distribution
  measured at grade 4 is quoted for trick 1 or for the opening. **Nothing causal is
  claimed.** And the selection fence, which is the sharp one — **five coordinates
  chosen by negative margin are a carrier, not a sample, and the selection
  criterion is correlated with the quantity being described**, so the spread must
  never be read as a distribution over coordinates.
  (iv) **Every minimal fusion core came back binary.** The ceiling at the grade-4
  frontier is `|A(I)| ≤ 3` (FT-A9(ii)); the observed size is 2 everywhere. This
  answers the note's §18.2 at this carrier — *decisive cores are binary here* is
  measured rather than conjectured — under the same P-A21 and selection fences, and
  it is the one place where the received note's guess and this engine's arithmetic
  can be compared at all.
  (v) **(FT-R1) held at h9, and that is the quietest good news in the file.** h9's
  filed `U` had been computed exactly once, by the revealed traversal; (FT-R1)
  reconstructs it from the frontier decomposition — a different traversal, with
  different intermediate quantities — and agrees exactly. **It is the only
  independent check `U` has ever received at the coordinate the exact route could
  not price.** It does not weaken h9's NOT PRICED label, which is about the primal
  pipeline (FT-A25(vii)).
- **FT-A27 (carried obligations, and what none of this claims).**
  (i) **DS-A28(ii) remains carried**, and the errata §9 queue now also carries
  **Lemma FT-mix**, together with the FT mathematics listed at FT-A22(iii).
  (ii) **The wiki freeze register is stale and I may not touch it.**
  `wiki/walt-math-freezes.md` calls itself the register of freezes 1–43, tabulates
  through 46, and still lists **38 as RESERVED**. Missing or wrong: 38 (filled at
  FT-A17), 47 (T1-A11), 48 (LD-A9), 49 (RW-A8), 50 with its v1.1 (FT-A18, FT-A23,
  FT-A24). That is precisely the cross-reference drift CLAUDE.md names as a bug.
  **Owed to the wiki owner, not dischargeable here**, and listed so it is not lost.
  (iii) **Not claimed**, everything at FT-A22(ii) unchanged, plus one specific to
  this run: **the h6 closure is not a claim that gluing is cheap, general or
  scalable.** One pair closed of twelve, at the untied coordinate with the smaller
  required fraction, at grade 4, on a five-coordinate carrier selected by negative
  margin — and no cost, timing or tractability claim is read off any traversal
  observable (SEP-A19(b), N4-A16). The walk-step and wall-clock columns are
  provenance.
  (iv) **T1-A12's implementation-versus-corpus risk carries here in full**, and is
  if anything sharper: every hypothesis this section discharged from the engine was
  read from the same implementation that computes the probe's evidence.

**What the build owes this section.** The freeze-50 v1.1(c) re-emission with
(FT-R7) and the four accounting integers per unit; (FT-R8) at h6 with the complete
competitor row set asserted against `legal_plays`; the h6 pair row restated in its
strict form with the exact surplus `4930081/479001600`; and FT-A25(vi)'s mandatory
sentence beside the coordinate verdict. Nothing else — the carrier order stands,
the numbers stand, and no re-derivation is owed.

- **FT-A28 (FT-R7's two-half discharge: RATIFIED, with three amendments, one
  named residual and one deferred closure. NO RE-EMISSION IS REQUIRED.)**
  **Object:** the S6k re-emission (commit `fdeeae8`), in which (FT-R7) was
  discharged in two scoped halves with the scopes printed, because the receipt as
  I wrote it could not be executed as written. The builder found this itself,
  after its run was green, and reported it rather than satisfying the letter of my
  ruling with a weaker check and saying nothing. That is the behaviour this record
  exists to reward, and it is the second time in this session — the freeze-50(a)
  conflict was the first. Both times the defect was in my text and the build found
  it.
  (i) **The defect in (FT-R7) was mine, and it is the same shape as freeze
  50(a)'s.** I wrote that the re-emission's values be *"compared against the first
  emission and asserted identical"*. The first emission's individual `δ_I` values
  exist in exactly one durable place — its results text — and a program treating
  results text as an interface is barred by SEP-A14(ii)'s principle, which I had
  myself invoked four clauses earlier in FT-A24(vii) to forbid producing the split
  by post-processing. **I specified a comparison without specifying the carrier of
  the reference value, and the only carrier available was one I had just
  prohibited.** The receipt was therefore not executable as written, and no
  faithful build could have discharged it literally.
  **The standing discipline this yields, stated so it cannot recur:** *a receipt
  that compares against a prior run must name the **carrier** of the reference
  value — a frozen table in the probe source with its provenance line, or an
  in-run recomputation — and never "the previous emission" unqualified.* A prior
  run is not an object; its results text is not an interface; only a transcribed
  constant or a recomputation is. This joins FT-A23(v)'s rule as the second
  specification defect of the same family: **naming a relation without naming its
  relata.**
  (ii) **The two halves are RATIFIED and renamed, and half 1's scope is corrected
  UPWARD.** (FT-R7) unqualified now names their conjunction; the halves are
  versioned, not renumbered.
  **(FT-R7a) the cross-run invariant receipt.** The frozen `FT_FIRST` table,
  transcribed into the probe source from FT-A24(ii) and the closing note above
  with its provenance line, asserted against the re-emission. This is exactly
  SEP-A14(ii)'s pattern — that ruling's own frozen table was itself transcribed
  out of a results file by hand, so the bar has always been on *machine
  re-parsing* and never on the values' origin. **The provenance here is if
  anything better than the precedent's**, because the summary values in the
  closing note were independently re-derived at adjudication time from the filed
  S6h `R3` rows before the re-emission existed.
  **The scope correction.** The build's printed scope says half 1 *"does not reach
  individual `δ_I`"*, which is true but understates it: `FT_FIRST` carries the
  FT-A24(ii) census, and the per-unit count `#{I : δ_I > 0}` **is a functional of
  the individual `δ_I` vector**, not of the summary. So (FT-R7a) pins **two
  independent functionals of that vector across two executions** — its sum, via
  `Δ^(1)`, and its support size, via the census. The accurate scope line is
  *"reaches `Σ_I δ_I` and `|supp δ_I|` per unit across executions; does not reach
  individual `δ_I`."* **An understated scope is never an error** — it claims less
  than is true, which is always safe — so this correction binds the **next**
  emission's wording and forces nothing now.
  **(FT-R7b) the in-run reproduction receipt.** A full second pass with fresh
  maps, accumulators and budgets, every printed `δ_I` and every row asserted
  identical. Scope: reaches every individual value; within one process. This is
  the half that carries the failure modes I named — iteration-order dependence,
  stale or reused accumulators, memoisation error, thread-order leakage — and it
  carries them at full strength, because per-unit content is a function of
  (kernel, budgets) alone and each unit runs single-threaded, so the only route by
  which the surrounding `W = 9` assembly could touch a unit's rows is shared
  mutable state, which is precisely what fresh state tests.
  (iii) **The residual, named rather than papered over.** The conjunction of
  (FT-R7a) and (FT-R7b) is **not equivalent** to what I originally specified, and
  the honest statement of the difference is short: a per-row discrepancy that
  arises **across processes**, **preserves both `Σ_I δ_I` and `|supp δ_I|` in every
  unit**, and **reproduces within each process**, would be caught by neither half.
  That is a contrived failure mode and I know of no mechanism that would produce
  it — but "contrived" is not "impossible", and a residual that is named costs
  nothing while a residual that is quietly absorbed into a HELD is exactly the
  drift this record exists to prevent. It is named here and it travels with any
  future citation of (FT-R7).
  (iv) **The closure, specified and DEFERRED: (FT-R7c) the frontier digest.** The
  residual closes completely and cheaply. Per unit, the probe emits into its
  summary line a SHA-256 over the canonical serialisation of the printed
  `(record, δ_I)` pairs in freeze-50(c) order. A digest is a fixed-width constant
  and is transcribable into `FT_FIRST` like any other frozen table value, so a
  later run asserts **one scalar per unit** and thereby reaches **every individual
  `δ_I` across executions**, with no results-text parsing anywhere. **Not owed for
  S6k**, which is committed and green and whose residual is already closed
  evidentially by (v) below; **binding on the next FT run that regenerates a
  frontier**, and cheap enough that it should simply become part of the emitter.
  Requiring a re-emission now to convert a known-true fact into a receipt would
  spend real cost on process hygiene alone, and proportionality is part of the
  discipline, not an exception to it.
  (v) **The orchestrator's byte-diff: its typing is RATIFIED, and the typing is
  the whole of its value.** The comparison of the second emission's 12,639 printed
  rows against the first emission's positive support, byte-identical, is an
  **audit note and not a receipt**. Three reasons, and each is independently
  sufficient: it is not asserted in-run; it is not reproduced by the verify path;
  and it does not survive into any future run, which is what a receipt is *for* —
  a receipt earns its keep by running every time, not by having been true once.
  **What it may be:** adjudication-time evidence, recorded as such, and it is
  recorded here — it closes (iii)'s residual **evidentially for this run**, which
  is why (FT-R7c) is deferred rather than demanded. **What it may never be:**
  cited as a receipt status, printed in a results file as HELD, or counted among
  "all N receipts HELD". **On SEP-A14(ii):** that ruling bars a *program* from
  treating results text as an interface. A one-off human or orchestrator diff is
  not that, and the operative distinction is that **nothing downstream keys off
  the audit** — the moment something did, it would be an interface and the bar
  would bite.
  (vi) **NO RE-EMISSION IS REQUIRED, stated explicitly as asked.** Every amendment
  above is either a correction to the record (i, iii), a wording change that
  claims *less* than the truth and is therefore safe as emitted (ii), a deferred
  addition (iv), or a ratification (v). S6k stands as committed at `fdeeae8`:
  the two halves discharge (FT-R7) as ruled-and-repaired, (FT-R8) HELD,
  `Opt^H(h6) = {40}` filed with FT-A25(vi)'s mandatory sentence and the strict h6
  row. Nothing is routed back to the builder.
  (vii) **What is owed, and it is small.** On the next FT run that regenerates a
  frontier: (FT-R7c)'s per-unit digest, and (FT-R7a)'s corrected scope line. On
  nobody, now: anything else. **Nothing here is promoted** — the receipts, the
  audit note, the verdict and the taxes are exploratory, cited by nothing above
  this tier, and quotable as results only by brief amendment adding them to a
  verifier receipt.

- **FT-A29 (two corrections to this section, both verified at adjudication time
  before filing; the corrected text stays visible and the errors are not erased,
  per LD-A11(ii)'s convention). Nothing downstream depends on either.**
  (i) **FT-A16(ii)'s "(LD-R4) remains owed" is WRONG and is corrected here.**
  `walt-factory/results/laydown_2026-08-14.txt` line 16 reads: *"(LD-R4): all
  seven per-declaration counts equal — Corollary LD-fold — HELD. Receipted: 301
  lay downs per declaration, 2,107 (hand, declaration) pairs."* The receipt ran
  before this section was written. **What changes:** FT-A16(ii)'s closing sentence
  — *"Until then '2,107' is licensed by Corollary LD-fold as a proof plus one
  receipted count, not by seven receipted counts"* — is **superseded**. The
  catalogue total is receipted by all seven per-declaration counts, exactly as
  LD-A12 specified, and may be quoted as a receipted count rather than as a proof
  plus a single count. **What does not change**, and must not be read as changing:
  (LD-R4) HELD remains *the cheapest available probe* of the LD-A10(ii)
  implementation-versus-corpus risk and is **not a discharge** of it — that
  sentence of FT-A16(ii) stands, LD-A13's open item is untouched, and the
  rules-package check is still owed before any of it leaves walt.
  **The error was mine and its shape is worth naming.** I read LD-A12's binding
  clause — *"Binding: add (LD-R4)"* — as an obligation **outstanding**, when it is
  an obligation **created**, and I never opened the artifact that would have told
  me it had since been discharged. **The discipline: a ruling that creates an
  obligation is not evidence that the obligation is still open; only the artifact
  is.** That is "by construction is not a receipt" (PG-A8) transposed from claims
  to obligations, and it is the third defect of mine this session found by someone
  else checking — after freeze 50(a)'s two authorities (FT-A23) and (FT-R7)'s
  unnamed carrier (FT-A28). All three are the same failure: **asserting a status
  from a text that governs it rather than from the object that carries it.**
  (ii) **FT-A25(vi)'s "it did at ten of twelve pairs" is WRONG; the count is
  ELEVEN.** Verified by census of the results file: `FT-A20(a) CLOSED` appears
  once and `FT-A20(b) NOT CLOSED` eleven times, over 12 binding pairs. The
  commentary counted the ten tied pairs and dropped h0, which is **untied and also
  failed** — indeed h0 is the more interesting of the two untied pairs, since it
  needed `12627174/16709317` of its competitor's gap and got less, whereas h6
  needed `8524657/18853881` and got `13454738/18853881`. **The corrected sentence
  reads: "The closure could have failed — it did at eleven of twelve pairs."**
  The error is localised to that one clause: this section's closing note states it
  correctly (*"Ten tied pairs NOT CLOSED, one untied pair (h0) NOT CLOSED"*), and
  FT-A26(ii) correctly scopes its claim to *"any of the ten tied pairs"*. The
  wiki restates eleven correctly and needs nothing.
  (iii) **Neither correction touches a verdict, a receipt, a freeze or a number in
  any results file**, and neither requires a re-emission or a re-run. FT-A25's
  mandatory sentence is unaffected in both letter and force — if anything (ii)
  strengthens it, since the closure failed more often than the commentary said,
  which is the direction that makes the run a sharper test of the machinery and a
  weaker claim about the game.

---

## The second rung: inbox 017 adjudicated (2026-08-14)

**Adjudicator:** walt-math-11. **Object:**
`exchange/inbox/017-second-rung-gluing.md` — *"Second-Rung Gluing:
Policy-Dependent Occupancies, the Slack–Tax Interchange Law, and Exact Martingale
Penalties for Straight Texas 42", v0.1*, received 2026-08-14 in answer to
`exchange/outbox/017-second-rung-gluing-handoff.md`, hand-ferried and
UNADJUDICATED on arrival. The note self-classifies its claims as *exact result*,
*[certificate] schema*, *experimental receipt — reported* or *open*; those labels
are the sender's and carry no status here until confirmed below. **Tier:**
exploratory throughout, without exception. Nothing in this section is promoted,
nothing is quotable in a brief, a dispatch, [FINDINGS](FINDINGS.md) or any
claim-tier page except by brief amendment adding it to a verifier receipt, and an
external note is never imported as an axiom (TRUST-01). **Basis:** the whole FT
chapter above — FT-A1..FT-A29 with Lemma FT-arrive, Lemma FT-trunc, Corollary
FT-grade4, Proposition FT-flat, Proposition FT-tie, Lemma FT-post, Corollary
FT-conv and Lemma FT-mix; freezes 26, 36 v2, 37, 38 v1, 44 v2, 45, 50 v1.1; the
errata mathematics under DS-A17 (Lemma E3 with (C1)–(C4), Lemma E4, Corollary
E4.1, Theorems E6.4 and E6.5); DS-A1, P-A1, P-A19, P-A21, R-A2, R-A18, SEP-A14(ii),
F7 and NO-RESCUE; and first-hand reading, at adjudication time, of
`walt-factory/examples/fusion_tax.rs` (the `walk` recursion at ≈ 1031–1168, the
`Arrival` fields `prefix`/`den`/`seen_focal`, `FrontierState::acc_q`, `path_a`,
`path_b`, `SCALE = 12^12`, `DEN_MU = 12^6`) and of every summary, pair and
frontier row of `walt-factory/results/fusion_tax_2026-08-14.txt`. Rulings
**SR-A1..SR-A26**; three lemmas and four propositions delivered below with full
proofs; **freeze 38 is clarified to v1.1(d)** at SR-A21 (no new content) and
**freeze 51** is fixed at SR-A22. The prefixes `SR-A`/`SR-R` and every name below
were grep-checked unused at adjudication time.

**What was asked, and what came back.** Handoff 017 asked four things: (1) the
exact second layer with policy-dependent arrival; (2) the multi-stage martingale
penalty dual, formalised; (3) the depth-two regret-event calculus with its
safe-addition schema; (4) grade the theory against our five exact `Δ^(2)`
rationals. **All four were delivered.** The mathematics is correct — I found no
false theorem and no wrong inequality direction anywhere in the note. What I did
find is four places where a hypothesis is used and not named, one place where a
stated justification names the wrong hypothesis, one silent weakening of a repair
the note claims to have adopted, one receipt program whose two structural
assertions **cannot fail**, and one reading of the grading table that is
tautological at grade 4 and is not typed as such. Those are the repairs, and they
are filed below.

**The engine facts everything below rests on**, restated so no proof re-derives
them. All were re-read from the implementation at adjudication time.

- **The focal information state is the complete public record.** Freeze 26's
  observation contract is the full record; `fusion_tax.rs` keys frontier states by
  `Vec<Domino>` — the plays since the kernel decision point with the root action
  first (freeze 36(b)). This is the fact that discharges the note's §2.2
  perfect-recall hypothesis **and** the unnamed hypothesis of its Theorem 4.1;
  see Lemma SR-coord.
- **`A(I)` is a function of (focal hand, led context) alone**, both known to the
  focal seat, and the probe asserts it constant across `X_I` in place
  (`"(FT-A7(ii)) stop-and-report: A(I) is not common across X_I"`).
- **The field is uniform over each seat's own legal set, per world.** Arrival
  weights are the products of inverse legal-set sizes along the record
  (Lemma FT-arrive); the frontier posterior is therefore **not** uniform
  (Lemma FT-post), and no aggregate uniformity of `p_I` or `|X_I|` implies
  otherwise — see SR-A18(v).
- **At grade 4 the focal seat holds four tiles and leads the root trick**
  (freeze 45), so it has exactly three further decisions, of which the last is
  forced. Hence `T_0 = 0` **and** `Θ_{I,b} = 0` identically: after the first
  frontier the focal seat always acts again. Both are contentful assertions for
  the depth-two build (SR-R2).
- **The first-frontier branch values are already computed and already unemitted.**
  `FrontierState::acc_q[j]` is exactly `Σ_ω μ_I(ω) q_I(ω,b_j)`, which is
  `F^(1)_{I,b_j}` of the note's §4 (with `Θ_{I,b} = 0`). The rung-one probe
  reduces it to an argmax set and discards the values. **The entire slack column
  `s_{I,b}` of Theorem 6.2 is therefore already inside S6k's frontier pass and has
  only to be printed.** This is the single most consequential engineering fact in
  this section.
- **The reveal-delay ladder at `k = 2` is already inside freeze 38 v1.** Freeze
  38(b)(1) declares the canonical family as *"`C^(k)` is the one-block partition
  at every focal frontier of depth `≤ k`"*, and 38(c) discharges validity for the
  whole family at once. **The depth-two cut needs no new cut authority and no
  freeze 38 v2** (SR-A21).

**Headline — eight findings, stated before the rulings.**

1. **The centrepiece is correct.** The slack–tax interchange law
   `Δ^(2) = Σ_I min_b [s_{I,b} + d_{I,b}]` (Theorem 6.2) is CONFIRMED, step by
   step, from Theorem 4.1 and the finite interchange identity of Lemma 6.1. It is
   the right answer to ask (1) and it is the first object in this branch that
   prices *policy adjustment* rather than only conflict. SR-A8.
2. **The multi-stage penalty dual is delivered and FT-A13(iv) is discharged on
   its validity half.** FT-A13(iv) BLOCKED the received §12.4 because *"the
   conditional induction is unwritten"*. Theorems 9.2 and 9.3 write it: stagewise
   conditional centering gives weak duality by the tower property, and
   backward-centered continuation values recover `U^(k)` exactly at every rung.
   Both are CONFIRMED. **It remains BLOCKED as a probe** — exact recovery needs the
   exact `q̄` table and buys no compute, exactly as FT-A13(ii) already said of the
   one-stage case. SR-A14.
3. **Theorem 4.1 uses a hypothesis nobody names, and it is the same shape as
   FT-arrive.** The outer `Σ_I max_b` is licensed only if a lawful first-stage
   policy may choose **independently** at distinct first-frontier states — i.e.
   only if `I_1` indexes distinct *information states*, not histories. The note
   names mutual exclusivity and policy-independent arrival; neither is that
   hypothesis. It is true in this engine because of freeze 26's full-record
   contract, and it is delivered as **Lemma SR-coord** with the second-frontier
   unique-parent fact proved in the same breath. SR-A4.
4. **§6.1's justification names the wrong hypothesis.** The policy-level minimum
   does separate into Theorem 6.2's local formula, but *not* "because
   first-frontier states are mutually exclusive and policy-independent" — it
   separates because a lawful first-stage policy ranges over the free product
   `∏_I A(I)`. Mutual exclusivity and fixed arrival are what make `U^(1)` and
   `U^(2)` decompose against **the same weights**; they are not what makes the
   minimum commute with the sum. Written out as **Proposition SR-sep**. SR-A9.
5. **§7's warning and §9's recursion are reconciled by a fact the note leaves
   implicit, and it is the most useful structural statement in the note.**
   Occupancy is policy-dependent from rung two on, so the global object may not be
   flattened (§7, correct); yet the backward recursion of Theorem 9.3 is valid
   anyway — **because occupancy never enters it.** Every stage below the first
   frontier enters through the lawful *posterior*, which is policy-independent by
   §9.1's Bayes cancellation; occupancy enters exactly once, at the first frontier,
   where Lemma FT-arrive fixes it. Delivered as **Proposition SR-post**. It is what
   makes the trick-1 penalty route coherent at all. SR-A11.
6. **The §12 verifier's two structural assertions cannot fail.** Both
   `assert f1 - f2 == downstream_tax` and `assert local_direct ==
   local_interchange` are identities in the verifier's own recomputed quantities —
   proved algebraically at **Proposition SR-taut** and confirmed by 20,000
   randomised inputs, zero failures. The program's only contentful checks are the
   input-sanity `delta >= 0` and the **optional** `expected_delta2` comparison.
   As a *quantity list* the schema is right and is adopted; as a receipt it is
   REJECTED, and three amendments are named that make it real. This is exactly
   "by construction is not a receipt" (PG-A8), and it is the second time in this
   chapter that a proposed check turned out to be an arithmetic remark
   (FT-A19(iii) was the first). SR-A17.
7. **Grade 4 cannot test whether rung two closes, and the note's §13.1–13.3 are
   not typed as such.** At grade 4, `U^(2) = Q^H` (Corollary FT-grade4) and
   `L_{a⋆} = Q^H(a⋆)` (Corollary E4.1(2)), so `L_{a⋆} ≥ U_a^(2)` holds
   **unconditionally** at every binding pair, strictly exactly at the untied ones.
   The note's h6 "full-`H` strict surplus" and h0 "closes strictly" are therefore
   the already-filed exact `H` gaps recovered by addition — I re-derived both and
   they land on `L − Q^H(a)` exactly. This is **Proposition SR-degen**, and it is
   the FT-A26(ii) lesson one rung up: type the identity honestly or it will be read
   as evidence. The arithmetic is a genuine cross-check between two independently
   filed columns; it is not a test of the theory. SR-A18.
8. **The note's own repair of Lemma FT-post is silently weaker than
   Lemma FT-post.** Its §1.4 admits a residual witness *"evaluated under the actual
   posterior induced at the public stopping history"*. That is true and it is not
   receiptable: it is precisely the sentence an artifact would write while pricing
   a fresh uniform coordinate and calling it the posterior. FT-post's form (i) is
   operational — *the value of an exhibited lawful continuation policy evaluated
   inside the same walk, under the carried weights* — and its binding clause
   (print which of (i) or (ii) was used, in place, on the row) **stands
   unamended**. SR-A2(iv).

Two further findings that are not defects in the note but change what the next
build may claim, both pre-declared here before any depth-two number exists:

- **Rung-two fusion cores at grade 4 are binary by arithmetic, not by
  measurement.** The focal seat holds two tiles at the second frontier, so
  `|A(J)| ≤ 2`; a positive `δ_{I,b,J}` then forces a minimal core of size exactly
  2. The note's open ledger row *"second-rung fusion cores remain binary"* is
  therefore **answered a priori and unmeasurable at this carrier** — the same shape
  as FT-A26(ii), caught before the run rather than after. SR-A24(f).
- **The escape action of §6.3 is the one genuinely open question the depth-two
  probe can decide.** Whether `argmin_b (s_{I,b} + d_{I,b})` ever leaves the
  rung-one optimal face `B*_I` is not determined by anything filed. Both answers
  are results (F7) and both are pre-declared. SR-A24(c)–(d).

---

### The received claims, adjudicated one by one

Sender's label on the left, this section's verdict on the right. Every verdict is
justified in the ruling named, and no verdict promotes anything above the
exploratory tier.

| Received claim | Sender's label | Verdict | Reason (one line) |
|---|---|---|---|
| §1.1 FT-arrive adopted as explicit hypothesis | (repair) | **CONFIRMED FAITHFUL** | Both halves present — the next-decision argument and the inverse-legal-set product form. SR-A2(i) |
| §1.2 five effective rungs; `U^(2) = Q^H` at grade 4 | (repair) | **CONFIRMED FAITHFUL, one convention owed** | Faithful to FT-trunc/FT-grade4; the ladder is indexed by *nontrivial* decisions there and by *decisions* here — equivalent by Lemma SR-forced, but the frontier-2 detector must freeze one. SR-A2(ii) |
| §1.3 action-independent upper cannot improve `C` | (repair) | **CONFIRMED FAITHFUL** | Proposition FT-flat's statement and proof, branch-local; FT-flat's scope clause must travel. SR-A2(iii) |
| §1.4 residual plans must carry the posterior | (repair) | **CONFIRMED-WITH-REPAIR** | Form (1) is true but not receiptable; FT-post's operational form (i) and its binding print-clause stand unamended. SR-A2(iv) |
| Lemma 3.1 policy-independent first arrival | Exact result | **CONFIRMED** | It is Lemma FT-arrive; the proof is ours in substance. SR-A3(i) |
| Lemma 3.2 action-conditioned second arrival | Exact result | **CONFIRMED** | Mutual exclusivity along one trajectory plus exhaustiveness with the early-terminal event. SR-A3(ii) |
| The conditioning-order box | Exact result | **CONFIRMED** | Fix `I`, fix `b`, then add over `J`; alternative `b` are counterfactual branches, not disjoint events. SR-A3(iii) |
| Theorem 4.1 nested formulas for `U^(1)`, `U^(2)` | Exact result | **CONFIRMED-WITH-REPAIR** | Correct once the free-product hypothesis is named (Lemma SR-coord); §2.2's unique-parent hypothesis holds in this engine. SR-A4 |
| §4.1 partial-policy form | Exact result | **CONFIRMED** | `max` over a product separates; `D^(2)(π_1)`'s occupancy is the selected policy's. SR-A5 |
| Prop 5.1 regret form | Exact result | **CONFIRMED** | The clairvoyant term is `c`-free. SR-A6(i) |
| Cor 5.2 complete-optimal-face criterion at depth two | Exact result | **CONFIRMED** | Both directions; complete argmax sets required, tie-broken optimiser barred — freeze 38(e), FT-A8. SR-A6(ii) |
| Prop 5.3 conditional additivity `d_{I,b} = Σ_J δ_{I,b,J}` | Exact result | **CONFIRMED** | `Θ_{I,b}` cancels; the remainder separates over conditionally exclusive `J`. SR-A6(iii) |
| Lemma 6.1 finite interchange identity | Exact result | **CONFIRMED, hypothesis unused** | An identity for arbitrary finite families over a common nonempty index; `y_b ≤ x_b` only delivers `d_b ≥ 0`. SR-A7 |
| **Theorem 6.2 the slack–tax interchange law** | Exact result | **CONFIRMED** | Theorem 4.1 + Lemma 6.1 + Prop 5.3, per first state; the centrepiece and it holds. SR-A8 |
| §6.1 policy-level form | Exact result | **CONFIRMED as an identity; REPAIRED justification** | The identity is right; the stated reason names the wrong hypothesis. Proposition SR-sep writes the separation out. SR-A9 |
| §6.2 exact zero criterion | Exact result | **CONFIRMED** | `s, d ≥ 0` makes the local tax vanish iff some `b` has both zero. SR-A10(i) |
| §6.3 taxing only the rung-one optimiser is unsafe | Exact corollary | **CONFIRMED** | Every inequality direction checked; `min_{b∈B*} d` is an **upper** bound on the true tax. SR-A10(ii) |
| §6.4 when the rung-one action stays optimal | Exact result | **CONFIRMED** | `d_{I,b*} ≤ s_{I,b} + d_{I,b}` for all `b`, re-derived. SR-A10(iii) |
| §7 recursive law for deeper rungs | Exact algebraic result | **CONFIRMED, scope sharpened** | Lemma 6.1 again; the no-flattening caveat is right and is sharpened by Proposition SR-post. SR-A11 |
| Thm 8.1 incremental second-rung penalty | Exact result | **CONFIRMED** | Centering under each `(I,b,J)` occupancy, then pointwise max. SR-A12(i) |
| Prop 8.2 exact recovery of `U^(2)` | Exact result | **CONFIRMED** | `q − λ*` is `ω`-free; substitution returns Theorem 4.1's formula. SR-A12(ii) |
| §9.1 lawful posterior well-defined and policy-free | Exact result | **CONFIRMED-WITH-REPAIR** | Bayes cancellation is right; three steps unstated — field-factor independence, lawful-reachability of relaxed histories, and Kuhn for mixed policies. SR-A13 |
| Def 9.1 stagewise conditional centering | Exact result | **CONFIRMED, one step named** | The "equivalently" uses `A_t ⊥ X_t | G_t`, which *is* lawfulness and should be stated. SR-A14(i) |
| Thm 9.2 multistage weak duality | Exact result | **CONFIRMED** | Tower property under a lawful policy; the relaxed class contains `ρ`. SR-A14(ii) |
| Thm 9.3 exact recovery by backward centering | Exact result | **CONFIRMED** | Induction step and base case both check; the penalised relaxed value collapses to `V_t(g)`, `x`-free. SR-A14(iii) |
| §9.2 explicit perfect penalties for `U^(2)` | Exact result | **CONFIRMED** | Prop 8.2 at stage 2, then the same construction at stage 1 over the glued continuation. SR-A14(iv) |
| §9.3 feature-based multistage penalties | [Certificate] schema | **CONFIRMED as a valid family; BLOCKED as a probe** | Valid for every `θ`; the centering is an *exact* equality — no float, no sampled moment (FT-A13(iii) binds unchanged). SR-A14(v) |
| §12.4 of inbox 016 (multi-stage penalty) | — | **FT-A13(iv) DISCHARGED on validity; still BLOCKED as a probe** | The conditional induction FT-A13(iv) called unwritten is now written. SR-A14(vi) |
| Thm 10.1 regret minorant under policy-dependent occupancy | [Certificate] schema | **CONFIRMED** | Pointwise domination survives the weighted sum and the `min` over a **complete** `A(J)`. SR-A15(i) |
| §10.1 event form | [Certificate] schema | **CONFIRMED** | Instance with `g = η·1_E`; combined minorants must stay pointwise below `R`. SR-A15(ii) |
| §10.2 the primitive is `(I,b,J,c)` | Exact result | **CONFIRMED** | A `b`-uniform event still needs its mass recomputed under each `μ_{I,b,J}`. SR-A15(iii) |
| Thm 11.1 two-stage action cover | [Certificate] schema | **CONFIRMED** | Monotonicity of `min` under termwise domination. SR-A16(i) |
| §11.1 cheap slack lower bounds | [Certificate] schema | **CONFIRMED** | `max{0, L_I − B_{I,b}} ≤ s_{I,b}` re-derived; equals 0 on the optimal face, as it must. SR-A16(ii) |
| §11.2 the five safe-addition rules | [Certificate] schema | **CONFIRMED, all five** | Each checked against the exclusivity fact it invokes; rules 2 and 4 are the two `min`s and both are right. SR-A16(iii) |
| Thm 11.2 safe telescoping across rungs | Exact result | **CONFIRMED** | Exact increments telescope; each amount must target its own adjacent difference. SR-A16(iv) |
| §12 aggregate receipt schema | [Certificate] schema | **CONFIRMED-WITH-REPAIR** | The quantity list is right and is adopted; three amendments — mass rows, action-set completeness, mandatory carried reference. SR-A17(ii) |
| §12.1 the Python verifier | [Certificate] schema | **REJECTED AS A RECEIPT** | Two of its three structural assertions are identities in its own recomputed quantities (Proposition SR-taut). SR-A17(i) |
| §13 the five reported `(Δ^(1), Δ^(2))` | Experimental receipt — reported | **TRANSCRIPTION CONFIRMED EXACT** | All ten rationals match `fusion_tax_2026-08-14.txt` character for character. SR-A18(i) |
| §13.1 h6 full-`H` surplus `1291153/59875200` | Exact arithmetic | **ARITHMETIC CONFIRMED; typed by SR-degen** | Re-run exactly; it equals `L_{40} − Q^H(11)`, the already-filed `H` gap. SR-A18(ii) |
| §13.2 h0 excess `4082143/89812800` | Exact arithmetic | **ARITHMETIC CONFIRMED; typed by SR-degen** | Re-run exactly; it equals `L_{53} − Q^H(00)`. Closure at rung two is unconditional at grade 4. SR-A18(iii) |
| §13.3 tied coordinates terminate at equality | Exact consequence | **CONFIRMED, and it is FT-A26(ii) verbatim** | Given the tie and `U^(2) = Q^H` the equality cannot fail; it confirms bookkeeping, not a proposition. SR-A18(iv) |
| §13.4 second-rung share percentages | (presentation) | **CONFIRMED AS DECIMALS; FENCE MISSING** | All five round correctly, and they enter no proof — but the note reads them as properties of hands. The FT-A26(iii) selection fence and P-A21 must travel. SR-A18(v) |
| §13.5 the h2 330-state branch receipt | (proposal) | **GRANTED as the next build, re-scoped** | It is the right object; scope, freezes, receipts and pre-declared outcomes are fixed at SR-A22..SR-A24. |
| §14 what rung two changes in the trick-1 program | (synthesis) | **NOTED; FT-A21 stands BLOCKED** | §14.2 names an obligation, not a method: the conditional feature moments are exactly what nobody can yet count. SR-A19 |
| §15 claim ledger | (self-assessment) | **RE-DERIVED ROW BY ROW; four labels amended** | SR-A20 |

---

### Lemma SR-coord (the two coordinate hypotheses of Theorem 4.1, discharged for this engine) — delivered here

Fix a coordinate, a root action `a`, the declared belief and field. Let `I_1` be
the set of first-frontier focal information states and, for `I ∈ I_1` and
`b ∈ A(I)`, let `I_2(I,b)` be the set of second-frontier focal information states
reached after `b`. Under freeze 26's observation contract — the focal information
state **is** the complete public record since the kernel decision point — both of
the following hold.

**(a) Free product at the first frontier.** The lawful first-stage policies are
exactly the elements of the product `∏_{I ∈ I_1} A(I)`: a lawful policy may choose
any legal action at each `I` independently of its choices at every other `I'`.

**(b) Unique parent at the second frontier.** Every `J ∈ I_2(I,b)` determines
`(I,b)` uniquely, and `I_2(I,b) ∩ I_2(I',b') = ∅` whenever `(I,b) ≠ (I',b')`.

*Proof.* (a) A lawful policy is by definition a function from focal information
states to legal actions. Distinct records are distinct information states (two
records of different length are two different states a fortiori), so no
constraint couples the value at `I` to the value at `I' ≠ I`, and the legal set
`A(I)` is nonempty and common across `X_I` because `legal_plays(decl, hand, led)`
reads only quantities the focal seat knows. The set of lawful first-stage
policies is therefore the full product.

(b) `J` is a record. The declaration is fixed, and the record's first entry is the
root action by freeze 36(b), so replaying `J` determines the seat to move at every
ply — leaders are determined by `Trick::winner` applied to completed tricks of the
record, and the focal seat leads the root trick at leader offset 0 (freeze 45).
Hence the positions of the focal seat's own plays inside `J` are determined by `J`.
The second such play is `b`, and the prefix of `J` strictly before it is exactly
the record `I`. So `(I,b)` is a function of `J`, which gives both the unique-parent
statement and the disjointness. ∎

**Why it is stated rather than assumed.** The note's Theorem 4.1 writes
`U^(1) = T_0 + Σ_I max_b F^(1)_{I,b}` and justifies the outer operation by *"the
first-frontier states `I` are mutually exclusive and have policy-independent
occupancies"*. Neither of those licenses the `max` being taken **inside** the sum.
Mutual exclusivity and fixed arrival say the two treatments decompose against the
*same weights*; independence of choice — clause (a) — is what says the controller
may optimise each summand separately. It is the same defect shape as the one
FT-arrive repaired one rung down: a hypothesis that is true here, for a reason
specific to this engine's coordinate, and nowhere named. Clause (b) is the note's
own §2.2 hypothesis, which it does name and correctly flags (*"If an
implementation uses a coarser coordinate, the parent history must be restored"*);
this engine uses the finest available coordinate, so nothing is owed. **Both
clauses fail under a coarser record**, and a successor that ever compresses the
observation contract voids Theorem 4.1 here, not merely its efficiency.

### Lemma SR-forced (gluing a forced decision is free; the two ladder indexings agree) — delivered here

Let `D` be a focal decision at which `|A(·)| = 1` in every positive-mass world of
its information state. Then the reveal-delay treatment that glues `D` and the one
that skips it have the same value, and the local tax at `D` is `0`.

Consequently, writing `U^(k)_dec` for the treatment gluing the next `k` focal
decisions **whether forced or not** and `U^(k)_ntv` for the treatment gluing the
next `k` **nontrivial** ones, we have, at every `k` and every coordinate,

  `U^(k)_dec ≥ U^(k)_ntv`,  with equality whenever the first `k` decisions along
  every positive-mass path contain no forced decision;

and at grade 4 both give `U^(2) = Q^H(a)`.

*Proof.* At `D` a lawful controller and a world-informed controller have the same
single action available, so world-contingency selects nothing and the two
treatments induce the same achievable value vectors; the field is unaffected
because `σ_{-m}` reads only its own seat's information (Lemma FT-trunc's argument,
applied at one decision). The local tax is
`δ = Σ_ω μ(ω) max_c q(ω,c) − max_c Σ_ω μ(ω) q(ω,c)` with a singleton `A`, so both
terms are the same sum and `δ = 0`. The inequality follows because gluing `k`
decisions of which some are forced constrains at most `k` real choices and at
least `0`, whereas gluing `k` nontrivial ones constrains exactly `k`. At grade 4
the focal seat has three decisions after the root, the third forced, so both
indexings glue the second and third and both equal `Q^H(a)` by Lemma FT-trunc. ∎

**Binding consequence for the build.** The received note indexes its ladder by
*nontrivial* decisions (§1.2); the S6k probe indexes by *decisions* — its own
accounting prints `FORCED |A(I)| = 1 = 3188` at h0, so forced states are frontier
states there with `δ_I = 0`. The two agree at grade 4 by the lemma, but a
frontier-**2** detector must implement one of them and not drift between them.
**Freeze 51(c) fixes the S6k convention: the second frontier is the focal seat's
next decision after `b`, forced or not; a forced `J` is a frontier state with
`|A(J)| = 1` and `δ_{I,b,J} = 0`, and it is counted, not skipped.** Consistency
with rung one is the reason, and the alternative is not wrong — it is merely a
different object, and two objects with one name is how a chapter goes bad.

### Proposition SR-sep (the policy-level minimum separates — the argument written out) — delivered here

With the note's notation, `V^(1)(π_1) = T_0 + Σ_I F^(1)_{I,π_1(I)}` and
`D^(2)(π_1) = Σ_I (F^(1)_{I,π_1(I)} − F^(2)_{I,π_1(I)})`. Then

  `U^(1) − U^(2) = min_{π_1} [ U^(1) − V^(1)(π_1) + D^(2)(π_1) ]
                 = Σ_I min_{b ∈ A(I)} [ s_{I,b} + d_{I,b} ]`.

*Proof.* The first equality is §4.1's `U^(2) = max_{π_1}[V^(1) − D^(2)]` rearranged
by `U^(1) − max = min(U^(1) − ·)`, valid because the maximum is over a finite
nonempty set. For the second, `U^(1) = T_0 + Σ_I M_I` with `M_I = max_b F^(1)_{I,b}`
(Theorem 4.1), so `T_0` cancels and

  `U^(1) − V^(1)(π_1) = Σ_I (M_I − F^(1)_{I,π_1(I)}) = Σ_I s_{I,π_1(I)}`,

whence the bracket is `Σ_I [ s_{I,π_1(I)} + d_{I,π_1(I)} ]`. **The minimum now
commutes with the sum because `π_1` ranges over the free product `∏_I A(I)`**
(Lemma SR-coord(a)): a minimiser may be chosen coordinatewise, and every
coordinatewise choice is a lawful `π_1`. ∎

**What this repairs.** The note's §6.1 asserts the separation and attributes it to
*"first-frontier states [being] mutually exclusive and policy-independent"*. Those
two facts are used **earlier** — they are what make `U^(1)` and `U^(2)` decompose
against one and the same `μ_I` in Theorem 4.1, without which neither `M_I` nor
`s_{I,b}` is even well defined. They say nothing about `min` and `Σ` commuting.
The gap is not a hole in the note's logic, because Theorem 6.2 is proved
independently of §6.1; it is a wrong reason attached to a right identity, which is
the failure mode that survives review and then gets cited. It is filed as a repair
for that reason and no other.

### Proposition SR-post (the rung ladder's backward recursion is occupancy-free) — delivered here

In the finite model, for every rung `k`, the backward construction of Theorem 9.3
uses **no occupancy measure below the first frontier**. Precisely: with
`V_t(g) = max_a Q̄_t(g,a)` and `Q̄_t(g,a) = E[Q*_t(X_t,g,a) | G_t = g]`, every
object in the recursion is a function of

  (i) the lawful posteriors `ν_t(· | G_t = g)`, which are policy-independent by
  §9.1's Bayes cancellation, and

  (ii) the field-and-chance transition kernels from `(x,g,a)`, which are
  policy-independent because `σ_{-m}` reads only the moving seat's own information;

and the only place an arrival mass appears is the outermost combination
`U^(k) = T_0 + Σ_{g ∈ I_1} p_g V_1(g)`, where `p_g` is fixed by Lemma FT-arrive.

*Proof.* Immediate by inspection of the recursion: `Q*_t` is defined by a
conditional expectation given `(X_t, G_t, A_t)` and therefore uses only (ii) plus
`V_{t+1}`, and `Q̄_t` averages `Q*_t` against `ν_t(·|g)` only. Neither depends on
`Pr(G_t = g)`. The induction of Theorem 9.3 shows `V_t(g)` is the lawful value at
`g`, and the top-level combination is Theorem 4.1's outer sum at `k = 1`. ∎

**Why this is the most useful sentence in the note that the note does not
write.** §7 warns, correctly, that from rung two on the occupancies depend on
earlier actions and *"must not be flattened with policy-independent weights"*. A
reader may conclude that no rung-two calculation decomposes. That is the wrong
conclusion. What is policy-dependent is the **unnormalised** occupancy
`μ_{I,b,J}`; what the recursion consumes is the **normalised** posterior, and the
`b` on which it depends is a *conditioning index inside the bracket*, not a free
policy variable. Conditioning on `b` is exactly what restores determinacy — which
is the content of the note's own conditioning-order box, one level up from where
it states it. This is what makes §14.2's penalty route coherent at trick-1 scale
at all: a feature moment `E[φ_{t,j}(X_t,g,a) | G_t = g]` is a **posterior**
quantity, so its exact computation never requires the depth-`t` occupancy that
nobody can enumerate. **It does not make that computation cheap** — FT-A21 stands
BLOCKED — but it removes the objection that it is ill-posed.

### Corollary SR-conv (the interchange law is convention-covariant) — delivered here

Let `v ↦ αv + c` with `α > 0` be a reconvention of the valuation (freeze 26's
bridge `Q_diff = 2·Q_count − grade` is `α = 2`, `c = −grade`). Then, for every
first state `I` and action `b`,

  `F^(1)_{I,b} ↦ α F^(1)_{I,b} + c·p_I`,  `F^(2)_{I,b} ↦ α F^(2)_{I,b} + c·p_I`,
  `s_{I,b} ↦ α s_{I,b}`,  `d_{I,b} ↦ α d_{I,b}`,  `δ_{I,b,J} ↦ α δ_{I,b,J}`,

and `Δ^(2) ↦ α Δ^(2)`. Every escape-action verdict, every zero-tax verdict and
every separation verdict is unchanged.

*Proof.* `F^(1)_{I,b} = Θ_{I,b} + Σ_J Σ_ω μ_{I,b,J}(ω) m(ω)` is a sum against
masses totalling `p_I` (Lemma 3.2's exhaustiveness), so an affine map on values
adds `c·p_I` and scales the rest by `α`; the same for `F^(2)`, whose inner
`max_c` passes the common additive `c·p_{I,b,J}` unchanged. Differences of two
such quantities at the same `I` lose `c·p_I` exactly, giving `s ↦ αs` and
`d ↦ αd`; `δ` is Corollary FT-conv's computation verbatim. `min` and `Σ` commute
with multiplication by `α > 0`. Verdicts compare quantities of the same type, and
an affine map with positive slope preserves `≥` (freeze 37(c)). ∎

**Binding consequence, restating freeze 38(f) for rung two.** The evaluators run
in the trick differential; every column of a depth-two artifact is reported in the
**count** convention, obtained by the exact inverse of freeze 26's bridge. A
differential `s`, `d`, `δ` or `Δ^(2)` is exactly twice its count-convention value.
**A rung-two tax quoted in one convention against a rung-one tax, a margin or a
filed `Δ^(2)` in the other is VOID**, and this is the failure mode most likely to
produce a near-miss that looks like a discovery.

### Proposition SR-degen (at grade 4, rung two closes every binding pair unconditionally) — delivered here

Let `a⋆` be a root action whose primal witness attains the lawful ceiling,
`L_{a⋆} = Q^H(a⋆)` (Corollary E4.1(2)), let `a ≠ a⋆` be any competitor, and work
at an n = 4 coordinate so that `U_a^(2) = Q^H(a)` (Corollary FT-grade4). Then

  `L_{a⋆} ≥ U_a^(2)` **always**, and the inequality is **strict exactly when the
  pair is untied**, with exact surplus `L_{a⋆} − U_a^(2) = Q^H(a⋆) − Q^H(a)`.

*Proof.* `a⋆` is `H`-optimal, so `Q^H(a⋆) ≥ Q^H(a)`; substitute both filed
identities. Strictness is `Q^H(a⋆) > Q^H(a)`, which is the definition of untied.
∎

**What it forces on the reading, pre-declared before any depth-two number
exists.** No grade-4 experiment can test whether the second rung closes a binding
pair — the answer is fixed by two already-filed columns and by Lemma FT-trunc.
In particular:

- the note's §13.1 h6 "full-`H` strict surplus" `1291153/59875200` **is**
  `L_{40} − Q^H(11)`, re-derived here exactly from the filed `R3` rows;
- the note's §13.2 h0 "closes strictly" and its excess `4082143/89812800` **is**
  `L_{53} − Q^H(00)`, likewise;
- the note's §13.3 tied-coordinate equality is FT-A26(ii)'s arithmetic identity.

**The arithmetic is still worth having, and it is not nothing**: each of the three
is a cross-check between two *independently produced* filed columns — the
revealed-summary `U`/`Q^H` table of `separation_n4_2026-08-14.txt` and the S6k tax
decomposition — and all three hold to the rational. What is barred is reading any
of them as evidence about the second rung's *power*. **Therefore the depth-two
build below is specified as an instantiation receipt on Theorem 6.2 and a
measurement of structure, and its results header prints this proposition.** This
is Proposition FT-tie's job, one rung up, and it earns its keep the same way:
as a fence on the reading, filed before the run.

### Proposition SR-taut (which depth-two identities cannot fail, and why the §12 verifier checks almost nothing) — delivered here

Let a depth-two aggregate receipt supply, per first state `I`, per action `b` and
per second state `J`, the numbers `Θ_{I,b}`, `C_{I,b,J}` and `{A_{I,b,J,c}}_c`,
and let a verifier **recompute**

  `δ_{I,b,J} = C_{I,b,J} − max_c A_{I,b,J,c}`,  `d_{I,b} = Σ_J δ_{I,b,J}`,
  `F^(1)_{I,b} = Θ_{I,b} + Σ_J C_{I,b,J}`,  `F^(2)_{I,b} = Θ_{I,b} + Σ_J max_c A_{I,b,J,c}`,
  `M_I = max_b F^(1)_{I,b}`,  `s_{I,b} = M_I − F^(1)_{I,b}`.

Then both of the following are **identities in the recomputed quantities** and
hold for every input whatsoever, valid or not:

  (1) `F^(1)_{I,b} − F^(2)_{I,b} = d_{I,b}`;
  (2) `M_I − max_b F^(2)_{I,b} = min_b [ s_{I,b} + d_{I,b} ]`.

*Proof.* (1) `F^(1) − F^(2) = Σ_J (C − max_c A) = Σ_J δ = d`, termwise. (2)
`s_{I,b} + d_{I,b} = (M_I − F^(1)_{I,b}) + (F^(1)_{I,b} − F^(2)_{I,b})
= M_I − F^(2)_{I,b}`; minimising over `b` gives `M_I − max_b F^(2)_{I,b}`. ∎

**Consequence for the received §12.1 program.** Its
`assert f1 - f2 == downstream_tax` is (1) and its
`assert local_direct == local_interchange` is (2); **neither can ever fire.**
Confirmed empirically at adjudication time: 20,000 randomised receipts of up to
three first states, three actions and three second states each, all
`δ ≥ 0`-admissible, produced zero assertion failures. The program's only
contentful checks are the input-sanity `assert delta >= 0` — which tests the
supplied numbers, not the algebra — and the comparison against
`expected_delta2`, which the code makes **optional** (`if "expected_delta2" in
receipt`).

**The general rule this yields, and it is the rung-two form of PG-A8's "by
construction is not a receipt":** *an assertion between two quantities the checker
itself derived from one set of inputs is an arithmetic remark; a receipt compares
a derived quantity against a value carried by a different object.* FT-A19(iii)
made this call once already, about `δ_I ≥ 0`; FT-A28(i) made the neighbouring call
about naming a relation without naming its relata. This is the third instance and
they are one discipline. Every depth-two receipt in SR-A23 is written to compare
against a **frozen filed value** or an **independently written second evaluator**,
never against the probe's own restatement of its own arithmetic.

---

- **SR-A1 (typing, tier, vocabulary, and what this section is).** The received
  note is **ACCEPTED IN LARGE PART**: its four asks are all answered, its central
  identity is correct, its multistage dual is correct and discharges an obligation
  FT-A13(iv) left open, one hypothesis needs naming, one justification is
  misattributed, one adopted repair is silently weakened, one receipt program is
  vacuous, and one reading of the grading table is tautological and untyped.
  Everything is exploratory. DS-A1 binds: this section says **witness**,
  **receipt** and **necessary outer profile**, never the forbidden word; the
  received note uses that word freely and it appears here only inside bracketed
  attribution to it. *Support ≠ belief*, *feasible ≠ reachable*, *possible ≠
  probable* are typed distinctions and are kept. Both outcomes of every gate below
  are results (F7); a receipt failure is stop-and-report, never a patch
  (NO-RESCUE). **No number of the received note enters walt as evidence.** The ten
  rationals of its §13 table are *ours* — the note says so itself and labels them
  "experimental receipt — reported" — and they were re-checked here against
  `fusion_tax_2026-08-14.txt` rather than accepted; its decimals and percentages
  are rounded forms of our exact rationals and the exact rationals are the objects
  (P-A19: no float anywhere).
- **SR-A2 (the four adopted repairs: three FAITHFUL, one silently weakened).**
  (i) **§1.1, Lemma FT-arrive: CONFIRMED FAITHFUL.** The note states the
  hypothesis explicitly and gives the correct reason — *"only because its frontier
  is the focal player's **next** decision after the fixed root action. No focal
  choice occurs before that frontier"* — and carries the second half of our lemma
  too, the inverse-legal-set product `Π_t 1/|A_t(ω,h_t)|` displayed immediately
  after its Lemma 3.1. It also states the failure at depth ≥ 2, which is the whole
  point. Nothing is owed.
  (ii) **§1.2, Lemma FT-trunc and Corollary FT-grade4: CONFIRMED FAITHFUL, with
  one convention owed.** The note's ladder `U^(0) ≥ … ≥ U^(5) = Q^H` after an
  opening lead and its boxed `U^(2) = Q^H` at grade 4 are ours exactly, and it
  supersedes its own earlier six-rung and three-rung counts in place, which is the
  right way to carry a correction. **The one difference that matters
  operationally:** the note counts *nontrivial* decisions, the S6k probe counts
  *decisions* (its h0 accounting prints 3,188 forced frontier states with
  `δ_I = 0`). Lemma SR-forced proves the two agree at grade 4 and that the general
  ladder statement survives either reading, but a frontier-2 detector must pick
  one — fixed at freeze 51(c) as the S6k convention. Its own ledger row flags this
  ("forced-action convention must be explicit"), correctly.
  (iii) **§1.3, Proposition FT-flat: CONFIRMED FAITHFUL.** The statement, the
  pointwise step `B_I ≥ max_b q_I`, and the conclusion that the glued upper is no
  smaller than the treatment-`C` branch are ours; the note states it branch-locally
  and ours states it after summing over `I` and adding `T_a`, which is the same
  claim. **FT-flat's scope clause travels unamended and must be printed wherever
  this is cited**: the proposition constrains `B` as a function of the *frontier*
  action, says nothing against a `b`-dependent-but-crude feature, and says nothing
  against the root-action conditioning that `U_a` already carries through `μ_I` and
  the frontier set.
  (iv) **§1.4, Lemma FT-post: CONFIRMED-WITH-REPAIR — the note's form (1) is a
  silent weakening and is not adopted.** The note admits a residual witness in
  either of two forms: *(1) it is evaluated under the actual posterior induced at
  the public stopping history; or (2) it is a pointwise guarantee valid in every
  compatible world.* Form (2) is FT-post's form (ii) exactly and is accepted. **Form
  (1) is true and is not receiptable.** FT-post's form (i) is *operational* — the
  value of an exhibited lawful continuation policy evaluated **inside the same
  walk**, under the carried weights — and that operational content is precisely
  what makes the condition checkable by an artifact. "Evaluated under the actual
  posterior" is a semantic condition that a coordinate built with a fresh uniform
  belief can be *asserted* to satisfy, and asserting it is how the trap closes on
  you: it is the same sentence the erroneous composition would write about itself.
  **FT-post's binding clause stands unamended and extends to rung two: any SR- or
  plan-calculus artifact that pastes a residual witness prints which of (i) or (ii)
  it used, in place, on the row.** Form (1) is admissible only where the posterior
  is **exhibited** — carried as the arrival weights of an actual walk — in which
  case it is form (i) and should be called that.
- **SR-A3 (§3, the two arrival lemmas and the conditioning-order box: all
  CONFIRMED).** Three clauses.
  (i) **Lemma 3.1 is Lemma FT-arrive.** Its proof is ours in substance — between
  the fixed root action and the first frontier only the fixed field and exogenous
  chance act, so the joint law of `(ω, I)` is determined by `β`, the rules, the
  root action, `σ_{-m}` and chance. CONFIRMED. Its follow-on remark that
  conditioning on the public path does not generally produce a uniform distribution
  over worlds is Lemma FT-post and is CONFIRMED.
  (ii) **Lemma 3.2 CONFIRMED.** Once `(I,b)` is fixed no focal action intervenes
  before the next focal frontier, so the transition law is the field-and-chance
  kernel `K_{I,b}`; distinct next public information states are mutually exclusive
  along one realised trajectory; and either one is reached or the continuation
  terminates first, which is exhaustiveness. The "in general the family changes
  with `b`" clause is right and is the whole reason this note exists: different `b`
  induce different successor hands, led contexts and field legal sets.
  (iii) **The conditioning-order box CONFIRMED**, and it is the note's sharpest
  single sentence: *fix `I`, fix `b`, then add over mutually exclusive `J`;
  alternative first actions are counterfactual branches, not disjoint events under
  one policy.* This is the exact statement whose violation would produce a
  plausible-looking and wrong rung-two additivity, and it is stated before any
  formula depends on it.
- **SR-A4 (Theorem 4.1: CONFIRMED-WITH-REPAIR; the free-product hypothesis is
  named at Lemma SR-coord).** The two displayed formulas are correct. Step-check.
  (i) **`U^(1)`.** Under `C^(1)` the controller commits one common `b` at `I` and
  the world is revealed immediately after; the value of branch `(I,b)` is then
  `Σ_ω μ_I(ω) q_I(ω,b)`, which the note re-expresses as
  `Θ_{I,b} + Σ_J Σ_ω μ_{I,b,J}(ω) m_{I,b,J}(ω)` using `μ_{I,b,J} = μ_I·K_{I,b}` and
  the exhaustiveness of Lemma 3.2. The re-expression is exact and it is what makes
  the rung-two decomposition possible at all: `F^(1)_{I,b}` is simultaneously a
  rung-one branch value and a sum over second-frontier states.
  (ii) **`U^(2)`.** Under `C^(2)` one common `c` must be chosen at each reached `J`;
  distinct `J` after `(I,b)` are mutually exclusive and, by Lemma SR-coord(b),
  distinct across branches too, so the joint optimisation over `(b, {c_J})` factors
  as `max_b [Θ_{I,b} + Σ_J max_c (·)]`. Correct.
  (iii) **The repair.** The outer `Σ_I max_b` needs Lemma SR-coord(a) — lawful
  first-stage policies form the free product `∏_I A(I)` — and the note supplies
  mutual exclusivity and policy-independence instead. Those are needed too, and
  earlier: they are what let `U^(1)` and `U^(2)` be weighted by one and the same
  `μ_I` while comparing controllers of different powers, which is FT-arrive's
  "why it is load-bearing" note repeated one rung up. The hypothesis holds in this
  engine by freeze 26's full-record contract and by nothing weaker.
  (iv) **`T_0` and `Θ_{I,b}` are handled correctly**: `T_0` is common to both
  treatments, `Θ_{I,b}` appears in `F^(1)` and `F^(2)` with the same value and
  cancels in every difference. At grade 4 both are identically zero and the probe
  asserts it (SR-R2).
- **SR-A5 (§4.1, the partial-policy form: CONFIRMED).** `U^(1) = max_{π_1} V^(1)`
  because a maximum over a product separates (Lemma SR-coord(a) again), and
  `U^(2) = max_{π_1}[V^(1)(π_1) − D^(2)(π_1)]` because `F^(2)_{I,b} = F^(1)_{I,b} −
  d_{I,b}` termwise. The note's closing sentence — *"the occupancy appearing in
  `D^(2)(π_1)` is induced by the selected first-stage policy"* — is the correct
  reading of what makes this a genuinely joint object and not a two-step
  optimisation, and it is the sentence a builder must not skip.
- **SR-A6 (§5, the exact conditional second-stage tax: all three CONFIRMED).**
  (i) **Proposition 5.1 CONFIRMED.** `Σ_ω μ m` is `c`-free, so subtracting the
  maximum common-action value is minimising expected regret. Nonnegativity of every
  `δ_{I,b,J}` follows and is an arithmetic remark, not a receipt (FT-A19(iii),
  Proposition SR-taut).
  (ii) **Corollary 5.2 CONFIRMED, both directions, with the discipline
  mandatory.** (⇐) a common `c*` in every positive-mass world's argmax makes
  `Σ μ q(·,c*) = Σ μ m`. (⇒) `δ = 0` gives some `c*` with
  `Σ_ω μ(ω)[m(ω) − q(ω,c*)] = 0`, a sum of nonnegative terms against positive
  weights, so `m(ω) = q(ω,c*)` at every positive-mass `ω`. **The note states the
  discipline itself** — *"The complete optimal action sets are required. A
  tie-broken optimizer is not a valid substitute"* — which is freeze 38(e) and
  FT-A8 verbatim, arrived at independently. Freeze 26's least-domino-index tie rule
  is **not** used anywhere in a depth-two artifact.
  (iii) **Proposition 5.3 CONFIRMED.** `Θ_{I,b}` cancels and the remainder
  separates over the conditionally exclusive `J`. The note's closing sentence —
  *"This is the full extent of free additivity at rung two"* — is exactly right and
  is the boundary that §11.2's five rules then police.
- **SR-A7 (Lemma 6.1: CONFIRMED, and its hypothesis is not used).** With
  `M = max_b x_b`, `s_b = M − x_b`, `d_b = x_b − y_b`, we get
  `y_b = M − s_b − d_b`, hence `max_b y_b = M − min_b(s_b + d_b)`, which rearranges
  to the claim. **The stated hypothesis `y_b ≤ x_b` is nowhere used**: the identity
  holds for arbitrary finite families over a common nonempty index set, and
  `y_b ≤ x_b` only delivers the side fact `d_b ≥ 0`. This is a sharpening, not a
  defect, and it matters in one place: it means the interchange law is available at
  a rung where monotonicity has not yet been established, which is worth knowing
  before someone proves monotonicity twice.
- **SR-A8 (Theorem 6.2, the slack–tax interchange law: CONFIRMED — the
  centrepiece, and it holds).** By Theorem 4.1,
  `Δ^(2) = Σ_I [max_b F^(1)_{I,b} − max_b F^(2)_{I,b}]`; apply Lemma 6.1 at each `I`
  with `x_b = F^(1)_{I,b}`, `y_b = F^(2)_{I,b}`, `d_b = d_{I,b}`; then Proposition
  5.3 expands `d_{I,b} = Σ_J δ_{I,b,J}`. Every step is licensed. Four clauses on
  what it means here.
  (i) **It is the right answer to ask (1) of handoff 017.** We asked what replaces
  the clean additivity `Δ^(1) = Σ_I δ_I` and whether there is a per-`(I_1,b_1)`
  decomposition with an exact interchange lemma. The answer is: additivity survives
  **inside** a branch and is replaced **across** branches by a minimum, and the
  minimum is over slack-plus-tax, not over tax.
  (ii) **The object it prices is new to this branch.** Every tax in the FT chapter
  prices *conflict* — worlds disagreeing about the best action. `s_{I,b}` prices
  *adjustment* — the controller buying its way out of a downstream conflict by
  playing worse now. Nothing in inbox 016 or in the FT rulings has a term of that
  type, and its absence is exactly why the first-rung formula could not be iterated.
  (iii) **It is exactly instantiable at our carrier.** At grade 4, `T_0 = 0`,
  `Θ_{I,b} = 0` and `U^(2) = Q^H(a)`, so the law becomes a computation whose answer
  is already filed twice over. That is the build (SR-A22).
  (iv) **It carries the convention rule.** By Corollary SR-conv both `s` and `d`
  are differences at a common `I` and scale by `α` with no additive residue, so the
  law is convention-covariant and a count-convention `Δ^(2)` is exactly half its
  differential value.
- **SR-A9 (§6.1, the policy-level form: the identity CONFIRMED, the justification
  REPAIRED).** The identity is Proposition SR-sep's first equality and is correct.
  The note then writes *"Because first-frontier states are mutually exclusive and
  policy-independent, the policy-level minimum separates into the local formula of
  Theorem 6.2"* — and that is the wrong reason, for the reason given in Proposition
  SR-sep's closing note. **No logical gap results**, because Theorem 6.2 is proved
  independently of §6.1 and §6.1 is a re-reading of it, so the note is sound as
  written. It is filed as a repair because a wrong reason attached to a right
  identity is the kind of sentence that survives review and then gets cited as
  authority for a case the identity does not cover — for instance a variant in
  which two first-frontier records share an information state, where mutual
  exclusivity and fixed arrival still hold and the separation fails.
- **SR-A10 (§6.2–6.4: all CONFIRMED, every inequality direction checked).**
  (i) **§6.2 CONFIRMED.** `s_{I,b} ≥ 0` and `d_{I,b} ≥ 0` give
  `Δ_I^(2) = 0 ⟺ ∃b` with both zero, and the boxed restatement — some `b` in the
  rung-one optimal face **every** one of whose reached second frontiers has a
  common `C`-optimal action — is correct and is the rung-two exposed-face stop
  rule.
  (ii) **§6.3 CONFIRMED, and it is the note's most important safety statement.**
  `Δ_I^(2) = min_b(s_{I,b} + d_{I,b}) ≤ min_{b ∈ B*_I} d_{I,b}` because `s = 0` on
  `B*_I`, so **the right-hand side is an upper bound on the true tax and using it
  as a lower witness is unsound.** Direction verified. The consequence the note
  draws is correct and binding: *a valid lower [certificate] for `Δ_I^(2)` must cover
  every first action, not only one optimiser and not only the complete optimal
  face.* Note carefully that this is strictly stronger than the FT-chapter
  discipline it superficially resembles: FT-A8 bars **tie-broken** optimisers in
  favour of the complete optimal face; §6.3 says the complete optimal face is
  **also** not enough at rung two, because an escape action outside it may carry
  the minimum. Both bind, and they are different rules.
  (iii) **§6.4 CONFIRMED.** `b* ∈ B*_I` stays optimal after gluing iff
  `F^(2)_{I,b*} ≥ F^(2)_{I,b}` for all `b`; substituting
  `F^(2)_{I,b} = M_I − s_{I,b} − d_{I,b}` and `s_{I,b*} = 0` gives exactly
  `d_{I,b*} ≤ s_{I,b} + d_{I,b}`. Re-derived. Its closing remark — that policy
  adjustment is not a nuisance in the proof but part of the exact optimal response
  — is right and is what makes the escape-action census a real measurement
  (SR-A24(c)).
- **SR-A11 (§7, the recursive law: CONFIRMED, scope sharpened by Proposition
  SR-post).** The displayed identity `M^(r-1)_G − max_a F^(r)_{G,a} =
  min_a[s^(r)_{G,a} + d^(r)_{G,a}]` is Lemma 6.1 instantiated and needs no
  further hypothesis (SR-A7). The scope caveat — *"It must not be flattened with
  policy-independent weights unless an additional invariance theorem proves that
  flattening valid"* — is **CONFIRMED and is exactly right**; it is Lemma
  FT-arrive's "why it is load-bearing" note carried to every rung. **The sharpening
  this section adds** is Proposition SR-post: what may not be flattened is the
  *unnormalised occupancy*; the recursion itself consumes only *posteriors* and
  field kernels, both policy-independent, and touches an arrival mass exactly once,
  at the first frontier. Without that, §7 and §9 read as if in tension. With it,
  they are one statement.
- **SR-A12 (§8, the incremental second-rung penalty dual: both CONFIRMED).**
  (i) **Theorem 8.1 CONFIRMED.** For fixed `c`, centering gives
  `Σ_ω μ q(·,c) = Σ_ω μ [q(·,c) − λ(·,c)] ≤ Σ_ω μ max_{c'}[q(·,c') − λ(·,c')]`;
  taking `max_c` on the left, summing over conditionally exclusive `J`, adding
  `Θ_{I,b}`, maximising over `b` and summing over `I` gives
  `U^(2) ≤ Ū^(2)(λ)`. Valid for every centered family. **The note's own warning is
  the load-bearing part and is confirmed:** *"The centering law is indexed by `b`.
  A penalty centered under one first action's occupancy is not automatically valid
  under another first action."* An artifact that centers once per `(I,J)` and reuses
  it across `b` produces an invalid upper witness, silently.
  (ii) **Proposition 8.2 CONFIRMED.** `λ* = q − q̄` is centered by construction and
  `q − λ* = q̄(c)` is `ω`-free, so the pointwise max commutes out and Theorem 8.1
  collapses to Theorem 4.1's `U^(2)`. The zero-mass convention (set the penalty
  arbitrarily) is harmless since those terms carry weight zero. **And FT-A13(ii)'s
  verdict carries verbatim one rung up: it buys no compute**, because the perfect
  penalty contains the exact `q̄` table, which is the object the whole programme is
  trying to avoid. The note says so itself at §14.3 and is right to.

- **SR-A13 (§9.1, the lawful posterior: CONFIRMED-WITH-REPAIR — the argument is
  right and three of its steps are unstated).** This is the route the handoff said
  we care most about, so it is checked hardest. The claim is that
  `ν_t(· | G_t = g)` is well defined at every reachable public history and does not
  depend on which lawful policy reaches it. The note's argument: *"each action
  probability depends only on the public history and therefore cancels from Bayes'
  rule inside that information state."* **The argument is correct.** Write
  `Pr(ω, g) = β(ω)·1[ω ⊨ g]·Φ(ω,g)·Ψ(g)`, where `Φ` collects the field and chance
  factors along `g` in world `ω` and `Ψ` collects the focal action probabilities.
  Every `ω` consistent with `g` shares the same public record and the same focal
  hand, hence the same sequence of focal information states, hence the same `Ψ(g)`;
  so `Ψ` is an `ω`-free constant and cancels between numerator and denominator.
  Four clauses.
  (i) **Unstated step one: the field factor must itself be policy-free.** `Φ`
  cancels nothing — it stays in the posterior — but the argument needs `Φ` not to
  depend on the focal policy, which holds because `σ_{-m}` is fixed and reads only
  the moving seat's own information. Under a field that conditioned on the focal
  seat's behaviour, `Φ` would carry `π`-dependence that no cancellation removes and
  the whole of §9 would fail. Name it.
  (ii) **Unstated step two: relaxed histories are lawfully reachable.** Theorem
  9.2's supremum ranges over `C^(0)`, and the centering condition of Definition 9.1
  must be meaningful at every public history that supremum can visit. It is: given
  any history `g` reached with positive probability by any policy in world `ω`,
  the deterministic lawful policy that plays `g`'s focal actions at `g`'s focal
  information states reaches `g` in `ω` with positive probability. Hence
  lawful-reachability and reachability coincide and `ν_t(·|g)` is defined
  everywhere it is used. The note asserts "for every reachable public history"
  without this line.
  (iii) **Unstated step three: mixed policies.** The note restricts to behavioral
  policies without saying why that is without loss. It is: the model is finite with
  perfect recall (Lemma SR-coord), so Kuhn's theorem gives a realization-equivalent
  behavioral policy for every mixed one, and realization equivalence preserves the
  joint law of `(ω, path)` and hence the posterior. Independently, the note's own
  §2 remark — fixed field, linear expectation, deterministic focal policy optimal —
  makes randomisation unnecessary in the first place. Either route suffices; one
  should be written.
  (iv) **The complementary fact, which the note does not state and which is worth
  having:** under a **relaxed** policy the penalties do *not* have zero mean,
  because a relaxed policy's actions depend on `ω` and Bayes no longer cancels.
  That is not a defect — Theorem 9.2 uses centering only on the lawful side — but
  it is the exact reason the dual bound is not tight for an arbitrary centered
  family, and a builder who assumes otherwise will misread a loose bound as a bug.
- **SR-A14 (§9, the multistage martingale dual: CONFIRMED throughout, and
  FT-A13(iv) is discharged on its validity half).** Six clauses.
  (i) **Definition 9.1 CONFIRMED, with one step named.** The displayed condition is
  `E[λ_t(X_t,g,a) | G_t = g] = 0` for each fixed `a`. The note's "equivalently,
  under every policy lawful through stage `t`, `E[λ_t(X_t,G_t,A_t) | G_t, A_t] =
  0`" is the step that will be used, and it is not a restatement — it needs
  `A_t ⊥ X_t | G_t`, i.e. that the action is chosen from lawful information with
  randomisation independent of the latent state. That *is* what "lawful" means
  here, so the step is sound; it should be written as an argument rather than an
  equivalence.
  (ii) **Theorem 9.2 CONFIRMED.** For `ρ` lawful through stage `k`, the tower
  property with (i) gives `E_ρ λ_t = 0` at each `t`, so `E_ρ U = E_ρ[U − Σ_t λ_t]`;
  the relaxed class `C^(0)` contains `ρ` (it only adds information), so
  `Ū_k(Λ) ≥ E_ρ U`; maximising over lawful-through-`k` policies gives
  `U^(k) ≤ Ū_k(Λ)`. Every step licensed. The closing observation — that under a
  lawful policy the partial sums are martingale differences relative to the lawful
  public decision filtration — is correct and is the reason the family is closed
  under addition across stages.
  (iii) **Theorem 9.3 CONFIRMED.** Step-checked in both directions. *Centering:*
  `E[λ*_t(X_t,g,a) | G_t=g] = E[Q*_t(X_t,g,a)|G_t=g] − Q̄_t(g,a) = 0` by definition
  of `Q̄_t`. *Collapse:* `Q*_t − λ*_t = Q̄_t(g,a)` is `x`-free, so a controller that
  sees `x` gains nothing and obtains `max_a Q̄_t(g,a) = V_t(g)`. *Induction:*
  assuming the penalised relaxed continuation at every next public state `g'`
  equals `V_{t+1}(g')` independent of its latent copy, the stage-`t` penalised
  relaxed value is `max_a [Q*_t(x,g,a) − λ*_t(x,g,a)] = V_t(g)`, which is the
  hypothesis at `t`. *Base:* at `t = k` revelation is permitted after the action, so
  `Q*_k` already carries the revealed-world continuation and the same collapse
  applies. *Top:* the first frontier's arrival masses are policy-independent
  (Lemma FT-arrive), so `T_0 + Σ_g p_g V_1(g) = U^(k)`. The stated consequence
  `U^(k) = inf_Λ Ū_k(Λ)` with the infimum attained follows from 9.2 plus 9.3.
  **This is the one place the recursion could have failed and does not, and
  Proposition SR-post says exactly why: no occupancy below the first frontier ever
  enters.**
  (iv) **§9.2 CONFIRMED.** The explicit two-stage pair is Proposition 8.2 at stage
  2 and the same construction at stage 1 over the already-glued continuation
  `V_2(I,b,J) = max_c q̄_{I,b,J}(c)`. The order matters and the note states it: the
  second penalty removes world-contingent second actions, then the first removes
  world-contingent first actions *after* evaluating the glued second stage.
  (v) **§9.3 CONFIRMED as a valid family; BLOCKED as a probe.** Every
  `λ_t = Σ_j θ_{t,j}(φ_{t,j} − E[φ_{t,j}|G_t=g])` is centered for every `θ`,
  including negative and mixed signs, by linearity. **FT-A13(iii) binds unchanged
  and is if anything sharper at rung two:** the centering is an *exact equality*,
  so no float and no sampled expectation may appear in any conditional feature
  moment; and the note's own clause that at depth two the center must use the
  posterior indexed by the complete public parent history `(I,b,J)` is correct and
  is the rung-two form of §8's `b`-indexing warning.
  (vi) **FT-A13(iv) is DISCHARGED on validity and remains BLOCKED as a probe.**
  FT-A13(iv) BLOCKED the received §12.4 of inbox 016 with the reason *"the
  conditional induction is unwritten"*. Theorems 9.2 and 9.3 write it, correctly.
  **What that changes:** the multi-stage penalty family is now a proved family, not
  a proposal, and Proposition 12.2's exact-recovery result does extend to every
  rung — which answers ask (2) of handoff 017 in full. **What it does not change:**
  nothing about cost. The perfect penalty encodes the exact continuation values,
  the note says so at §14.3, and FT-A13(ii)'s "buys no compute" verdict carries.
  Freeze 38 v1(g) listed multi-stage penalties as **not** in freeze 38 v1; that
  remains true — the reveal-delay ladder is frozen, the penalty route is not, and it
  re-enters as freeze 38 v2 fixed by a later adjudication if and when a probe needs
  it. Nothing here fixes freeze 38 v2.
- **SR-A15 (§10, depth-two regret events: CONFIRMED).** Three clauses.
  (i) **Theorem 10.1 CONFIRMED.** `0 ≤ g ≤ R` pointwise gives
  `Σ_ω μ g(·,c) ≤ Σ_ω μ R(·,c)` for each fixed `c`; the minimum over `c` preserves
  it; conditional additivity over `J` gives the branch inequality. **One hypothesis
  must be printed wherever this is used:** the minimum is over the **complete**
  `A(J)`. A minorant supplied for only some `c` yields a minimum over a subset,
  which over-estimates `δ` and inverts the direction — the same failure the
  complete-optimal-face rule prevents one line down.
  (ii) **§10.1 CONFIRMED.** The event form is Theorem 10.1 with
  `g = η·1_E`, giving `min_c η_{I,b,J,c}·μ_{I,b,J}(E_{I,b,J,c})`; the caveat that
  several event terms may be summed only while their combined value stays pointwise
  below `R` is §11.2 rule 1 and is right.
  (iii) **§10.2 CONFIRMED, and it answers ask (3) of handoff 017 in the direction
  we suspected.** The primitive is indexed by `(I,b,J,c)`. A `b`-uniform event is
  admissible only if its regret inequality holds for every first action under
  consideration, and **even then its mass must be recomputed under each
  action-conditioned occupancy** — there is generally no policy-independent
  depth-two event probability. The note's summary — *"a `b`-uniform event is a
  theorem about a family of such primitives, not a replacement for the action
  conditioning"* — is the correct typing and is adopted.
- **SR-A16 (§11, covering policy adjustment: CONFIRMED, all four items and all
  five rules).**
  (i) **Theorem 11.1 CONFIRMED.** Termwise `ŝ + d̂ ≤ s + d`, so the minimum over
  `b` is dominated, so the sum over `I` is dominated, and Theorem 6.2 identifies the
  dominating sum as `Δ^(2)`. The interpretation the note boxes — *every first action
  must be covered by either rung-one slack or a downstream fusion tax* — is the
  exact proof obligation and is the right thing to have written down.
  (ii) **§11.1 CONFIRMED.** With `L_I ≤ M_I` and `F^(1)_{I,b} ≤ B_{I,b}`,
  `s_{I,b} = M_I − F^(1)_{I,b} ≥ L_I − B_{I,b}`, and `s_{I,b} ≥ 0` always, so
  `max{0, L_I − B_{I,b}} ≤ s_{I,b}`. Re-derived and correct. Sanity: for `b` on the
  optimal face, `B_{I,b} ≥ F^(1)_{I,b} = M_I ≥ L_I`, so the bound returns 0, which
  is the true value — the estimator is not accidentally vacuous elsewhere and not
  accidentally wrong there.
  (iii) **§11.2, all five rules CONFIRMED, each against the exclusivity fact it
  invokes.** (1) *within one `(I,b,J,c)`*: add only on disjoint supports or under a
  proved pointwise bound — correct, since pointwise domination by `R` is what
  Theorem 10.1 consumes. (2) *across second actions `c`*: **minimum**, never a sum
  — correct, `δ` is a `min_c`. (3) *across second states `J` after fixed `(I,b)`*:
  **add** — correct, Proposition 5.3, conditional mutual exclusivity. (4) *across
  alternative first actions `b`*: **slack-plus-tax minimum**, never a sum —
  correct, Theorem 6.2, and this is the rule the whole note exists to establish.
  (5) *across first states `I`*: **add** — correct, mutual exclusivity plus
  policy-independent arrival, i.e. Theorem 4.1's outer decomposition. The two
  minima (2) and (4) are the two places a plausible-looking addition would silently
  produce an invalid lower witness, and both are correctly typed.
  (iv) **Theorem 11.2 CONFIRMED.** Exact increments telescope and rungwise lower
  bounds sum. The accompanying warning is correct and is our overlap discipline
  verbatim: *each claimed amount must be attached to and proved against its own
  marginal relaxation difference*, and a structural loss describable at two stages
  may not be counted twice.
- **SR-A17 (§12, the receipt schema and its verifier: schema
  CONFIRMED-WITH-REPAIR, program REJECTED AS A RECEIPT).** Two clauses and three
  amendments.
  (i) **The program is REJECTED as a receipt.** Proposition SR-taut proves that its
  two structural assertions are identities in its own recomputed quantities and can
  never fire; 20,000 randomised admissible receipts produced zero failures at
  adjudication time, which is confirmation, not proof — the proof is SR-taut. Its
  contentful content is `assert delta >= 0`, an input-sanity check, and the
  comparison against `expected_delta2`, which the code makes **optional**. The note
  is honest about the *provenance* limitation it does have (*"This verifier does not
  establish that a reported `clairvoyant_sum` or `common_action_sum` came from the
  Texas 42 rules"*), and does not notice the *algebraic* one, which is larger: the
  program verifies the depth-two algebra against nothing.
  (ii) **The schema is CONFIRMED-WITH-REPAIR and its quantity list is adopted.**
  `(Θ_{I,b}, C_{I,b,J}, {A_{I,b,J,c}}_c)` is exactly the right minimal aggregate:
  it determines `δ`, `d`, `F^(1)`, `F^(2)`, `s` and `Δ^(2)` and nothing is
  redundant. **Three amendments make it a receipt.**
  **(A) Mass rows are mandatory.** The schema records no masses, so the
  exhaustiveness of the `J` list is unverifiable — a dropped `J` lowers `C`, `A`,
  `F^(1)` and `F^(2)` together and understates `Δ^(2)` with every assertion still
  green. Record `p_I`, `p_{I,b,J}` and the early-terminal mass, and assert
  `Σ_J p_{I,b,J} + p^term_{I,b} = p_I` for **every** `(I,b)` and `Σ_I p_I = 1`.
  This is (FT-R2) lifted one rung and it is the amendment that matters most.
  **(B) Action-set completeness is mandatory.** §6.3 makes the minimum over **all**
  `b` load-bearing, and the schema has no way to detect a missing branch. Record
  `|A(I)|` and `|A(J)|` and assert them against the engine's `legal_plays` at the
  reconstructed position — the (FT-R8) pattern.
  **(C) The reference value is mandatory and must name its carrier.** Make
  `expected_delta2` required, and require it to be a **frozen transcribed constant
  with a provenance line** or an **in-run recomputation** — never "the previous
  emission" and never re-parsed results text. That is FT-A28(i)'s standing
  discipline applied before the fact rather than after it.
  (iii) **How the Python interacts with our discipline: it does not, and is not
  adopted as a build artifact.** Our probe receipts are Rust, asserted **in-run**,
  under `ci/check.sh` with `-D float_arithmetic` and the no-float grep; that is
  strictly stronger than an out-of-band post-processing pass, and SEP-A14(ii) bars
  a *program* from treating results text as an interface in any case. **The Python
  is therefore NOT commissioned.** It is recorded here as a specification of the
  quantity list, which is where its value is. Should an independent replay artifact
  ever be wanted, the admissible form is: the probe **emits** a canonical JSON
  receipt as a first-class committed artifact with its own SHA-256 in the results
  header — a machine-readable interface by construction rather than by scraping —
  and a stdlib-only Python 3.12 verifier under ingest-verifier rules checks
  amendments (A), (B), (C) plus the two SR-taut identities *typed as arithmetic
  remarks in its own comments*. **DEFERRED, owed to nobody now.**
- **SR-A18 (§13, the grading: arithmetic CONFIRMED EXACT; the reading REPAIRED by
  Proposition SR-degen; one fence MISSING).** Five clauses. Every figure below was
  re-run in exact rationals at adjudication time and independently against the
  filed rows of `fusion_tax_2026-08-14.txt`.
  (i) **Transcription: CONFIRMED EXACT, all ten rationals.** The note's §13 table
  reproduces our filed `(Δ^(1), Δ^(2))` pairs without error: h0
  `(19863799/179625600, 387281/5132160)`; h6 `(611579/21772800,
  5399143/479001600)`; h2 `(145/22176, 1483/138600)`; h9 `(227251/3326400,
  4532503/26611200)`; h12 `(34519/1995840, 95917/4989600)`. Each was checked
  against the `SUMMARY` line of the corresponding FT unit. The coordinate
  identifications are also correct: its h0 is pip 3 `[00 21 32 53]`, h6 pip 4
  `[11 40 43 53]`, h2 pip 5 `[21 33 53 54]`, h9 pip 4 `[30 41 54 61]`, h12 pip 0
  `[20 30 40 65]`, matching freeze 50 v1.1(a). The note labels them "experimental
  receipt — reported" and declines to claim independent regeneration, which is the
  correct self-typing and is confirmed as such.
  (ii) **§13.1 CONFIRMED as arithmetic; typed by Proposition SR-degen.**
  `4930081/479001600 + 5399143/479001600 = 10329224/479001600 = 1291153/59875200`,
  exactly as claimed, and its decimal `0.021564070` is the correct 9-place
  round-half-up of `0.02156406993…`. **And it is not an independent result:**
  computed here from the filed `R3` rows, `L_{40} − Q^H(11) = 541161923/239500800 −
  535997311/239500800 = 1291153/59875200` — the same rational. The note's "full-`H`
  strict surplus" is the already-filed exact `H` gap recovered by addition. That
  makes it a genuine cross-check between the revealed-summary table and the S6k
  decomposition, and it holds; it makes it no kind of test of the depth-two theory.
  (iii) **§13.2 CONFIRMED as arithmetic; typed by Proposition SR-degen.**
  `387281/5132160 − 5390549/179625600 = 8164286/179625600 = 4082143/89812800`,
  exactly as claimed, decimal `0.045451684` correct. **And likewise:**
  `L_{53} − Q^H(00) = 33701/9900 − 301653329/89812800 = 4082143/89812800`. The h0
  "excess" is the exact `H` gap. The note's framing — *"rung two closes what rung
  one missed"* — invites reading a forced consequence as a discovery, and by
  Proposition SR-degen closure at rung two is unconditional at grade 4 for every
  binding pair and strict for every untied one. **Repair: the framing, not the
  number.** The filed h0 shortfall `U^(1) − L = 5390549/179625600` also reproduces
  exactly from the filed rows, so all three of the note's h0 inputs check.
  (iv) **§13.3 CONFIRMED, and it is FT-A26(ii) arriving independently.** At the
  three tied coordinates the shortfall equals `Δ^(2)` exactly and the second rung
  terminates at equality. This cannot fail: given the tie, `U^(1) − L_{a⋆} =
  U^(1) − Q^H(a) = Δ^(2)` by definition plus Corollary FT-grade4. Verified at all
  three: h2 `1483/138600`, h9 `4532503/26611200`, h12 `95917/4989600`, each equal
  to its filed `U^(1) − L` to the rational. **The note's added sentence is right and
  is worth keeping:** *"No valid method should produce strict separation between
  actions whose exact `H` values are tied."* That is a soundness fence on any future
  rung-two witness and it is adopted.
  (v) **§13.4 CONFIRMED AS DECIMALS; the fence around them is MISSING and is
  supplied here.** All five second-rung shares are correct to three places —
  h0 40.561%, h6 28.637%, h2 62.070%, h9 71.372%, h12 52.640% — as are all five
  9-place decimal expansions of our `Δ^(2)` rationals: `0.075461599`,
  `0.011271660`, `0.010699856`, `0.170323135`, `0.019223385`. **They are
  presentation only and enter no proof, and this is confirmed by inspection: no
  displayed derivation anywhere in the note consumes a decimal.** P-A19 is
  therefore not violated. **What is missing is the selection fence.** The note
  reads the spread as a property of hands — *"h9 is strongly depth-two dominated";
  "h12 is comparatively balanced"* — and the five coordinates are a **carrier
  selected by negative binding margin, not a sample**, with the selection criterion
  correlated with the quantity being described (FT-A26(iii)). P-A21 also binds: **no
  distribution measured at grade 4 is quoted for trick 1 or for the opening.** Both
  fences travel with any citation of this table and are printed in the depth-two
  results header. The note's own §13.4 closing paragraph — that the five-row summary
  cannot distinguish occupancy mass from per-world regret from conflict count from
  escape slack, and that the decomposition `(I,b) ↦ (s_{I,b}, {δ_{I,b,J}}_J)` is
  what would — is correct, is the right diagnosis, and is precisely what the build
  emits.
  (vi) **One conflation guarded in advance.** At h2 every printed frontier row
  carries `p_I = 1/330`, `|X_I| = 1680`, `|A(I)| = 3` — 216 of 216 printed rows,
  with the remaining 114 zero-tax rows held only by the uncommitted companion, so
  the uniformity is stated **of the printed rows** and of nothing else (FT-A29's
  discipline: only the artifact carries the status). **Constant `p_I` and constant
  `|X_I|` do not make `ν_I` uniform**, and no depth-two artifact may treat them as
  doing so: `ν_I(ω) = μ_I(ω)/p_I` and `μ_I` is the field's per-world legal-set-size
  product, which varies across `X_I` (Lemma FT-post). This is the sharpest trap in
  the FT chapter and it is one aggregate coincidence away from being fallen into.
- **SR-A19 (§14, the trick-1 program: NOTED; FT-A21 stands BLOCKED, with one
  obligation now sharper).** The synthesis is coherent and its §14.1 statement of
  the two-mechanism action cover is Theorem 11.1 in words. Three clauses.
  (i) **§14.2 names an obligation, not a method.** *"A structural penalty family may
  be counted by exact symbolic methods provided the following quantities can be
  obtained under every relevant public/action prefix: `E[φ_{t,j}(X_t,G_t,a) |
  G_t]`."* True — and Proposition SR-post is why it is even well posed, since those
  are posterior quantities and never occupancy quantities. But *provided* is
  carrying the entire claim: exactly counting a conditional feature moment over the
  trick-1 fiber is the thing nobody can do, and no instance is exhibited. **FT-A21
  stands BLOCKED in full**, with its three obligations unchanged, and Proposition
  FT-flat still forecloses the cheapest instinct.
  (ii) **§14.3 is honest and is adopted.** *"Exact recovery proves existence, not
  cheapness."* That is FT-A13(ii) independently rediscovered and it is the correct
  reading of Theorem 9.3.
  (iii) **§14.4 restates the lower side correctly**, including that residual plans
  must carry the true arrival posterior or be pointwise guaranteed — subject to
  SR-A2(iv)'s repair, which is that "carry the posterior" must mean *exhibit* it.
  The compact lawful plan remains open and nothing here advances it.
- **SR-A20 (§15, the claim ledger re-derived row by row; four labels amended).**
  My status on the left of the reason column; where it differs from the note's, the
  difference is stated.

| Note's row | Note's label | This section's status | Where they differ |
|---|---|---|---|
| First-frontier occupancy is policy-independent | Exact result | **CONFIRMED** | — (it is Lemma FT-arrive) |
| Second-frontier occupancy indexed by `(I,b,J)` | Exact result | **CONFIRMED** | Its obligation "engine must preserve the complete public/action history" is **discharged** here — Lemma SR-coord(b) |
| Exact nested formulas for `U^(1)`, `U^(2)` | Exact result | **CONFIRMED-WITH-REPAIR** | **Amended:** a hypothesis is unnamed (free product, Lemma SR-coord(a)). Not "exact result" as stated |
| Conditional additivity `d = Σ_J δ` | Exact result | **CONFIRMED** | — |
| Slack–tax interchange law | Exact result | **CONFIRMED** | Its "None beyond the finite-model hypotheses" understates: it inherits Theorem 4.1's hypotheses, including the unnamed one |
| Taxing only a rung-one optimiser is unsafe | Exact corollary | **CONFIRMED** | — |
| Recursive local law for deeper rungs | Exact algebraic result | **CONFIRMED** | — ; scope sharpened by Proposition SR-post |
| Incremental second-rung centered-penalty bound | Exact result | **CONFIRMED** | — |
| Perfect second-rung penalty recovers `U^(2)` | Exact result | **CONFIRMED** | — ; FT-A13(ii)'s "buys no compute" attaches |
| Multistage martingale weak duality | Exact result | **CONFIRMED** | Its obligation "public filtration and action chronology must match the engine" is **discharged** — Lemma SR-coord |
| Backward-centered penalties recover every rung | Exact result | **CONFIRMED** | — |
| Structural feature penalties valid after centering | [Certificate] schema | **CONFIRMED as a family; BLOCKED as a probe** | **Amended:** exactness of the centering is not optional (FT-A13(iii)) — no float, no sampled moment |
| Depth-two regret-event minorants | [Certificate] schema | **CONFIRMED** | Add: the `min_c` must run over the **complete** `A(J)` |
| Two-stage action-cover lower bound | [Certificate] schema | **CONFIRMED** | — |
| Safe telescoping of adjacent-rung [certificate]s | Exact result | **CONFIRMED** | — |
| Five effective opening-lead rungs | Exact combinatorial correction | **CONFIRMED** | It is ours (Lemma FT-trunc); its own obligation "forced-action convention must be explicit" is **discharged** at Lemma SR-forced and freeze 51(c) |
| Action-independent pointwise upper cannot shave `C` | Exact result | **CONFIRMED** | It is ours (Proposition FT-flat); "Remaining obligation: None" is right, but FT-flat's **scope clause** must travel |
| Residual-plan witness must use posterior or be pointwise | Exact validity boundary | **CONFIRMED-WITH-REPAIR** | **Amended:** its form (1) is not receiptable; FT-post's operational form (i) and its print-clause govern (SR-A2(iv)) |
| Experiment 15.1 exact fractions | Experimental receipt — reported | **TRANSCRIPTION CONFIRMED EXACT** | Correct self-typing; the values are ours and were re-checked, not accepted |
| h0 closes strictly at rung two | Exact arithmetic consequence | **CONFIRMED as arithmetic; RE-TYPED** | **Amended:** it is *unconditional* at grade 4 (Proposition SR-degen), not a consequence peculiar to the reported receipts |
| h2's `1483/138600` reconstructed from frontier rows | Open | **OPEN — and it is the build** | Agreed; scoped at SR-A22 |
| Small feature penalties close trick-1 competitors | Open | **OPEN, and BLOCKED as a probe** | FT-A21 |
| Second-rung fusion cores remain binary | Open | **NOT OPEN AT THIS CARRIER — vacuous at grade 4** | **Amended:** `|A(J)| ≤ 2` forces every positive-`δ` minimal core to size exactly 2. Unmeasurable here; open only at grade ≥ 5 (SR-A24(f)) |

- **SR-A21 (FREEZE 38 v1.1(d): the rung-two cut order, a CLARIFICATION with no new
  content; freeze 38 v1 is NOT amended and freeze 38 v2 is NOT opened).** Three
  clauses.
  (i) **The depth-two cut is already inside freeze 38 v1 and needs no new
  authority.** Freeze 38(b)(1) declares the canonical family as *"the reveal-delay
  ladder: `C^(k)` is the one-block partition at every focal frontier of depth `≤ k`
  and the singleton partition below"*, and 38(c) discharges validity for the whole
  family in one argument — a lawful policy chooses one action per information state
  and therefore satisfies every block identification within that state. `k = 2` is
  a member. **Nothing about the depth-two build requires a freeze 38 v2**, and
  38(g)'s exclusions (feature penalties, multi-stage penalties, adaptive block
  search beyond the first frontier, `κ_a(T)` cost models) are untouched by anything
  ruled here — SR-A14(vi) discharges an obligation about the *mathematics* of
  multi-stage penalties and commissions no probe of them.
  (ii) **FREEZE 38 v1.1(d) — the induced total order, exhibited rather than
  described.** Freeze 38(d) reads *"layers ascending, `k = 1, 2, …`; within a
  layer, frontier information states in ascending observation-record order (freeze
  36(b)'s lexicographic order over the canonical ascending domino index); within a
  state, actions in ascending domino index"*. That rule already generates the
  rung-two order, and FT-A23's lesson is that a rule which does not visibly
  generate its list is a defect, so the generated order is exhibited here: **first
  states `I` in ascending record order; within `I`, first actions `b` in ascending
  domino index; within `(I,b)`, second states `J` in ascending record order; within
  `J`, second actions `c` in ascending domino index.** Second-frontier records are
  frontier information states of layer 2 and are ordered by the same freeze-36(b)
  lexicographic rule, and `J`'s record strictly extends `I`'s (Lemma SR-coord(b)),
  so the order is a well-defined total order on `(I,b,J,c)` and is declared before
  the run. **No block merges are used** — freeze 38(b)(2) is scoped to the first
  frontier and is not exercised.
  (iii) **Freeze 38(e) and (f) apply unchanged at rung two.** The stop rule is
  Corollary 5.2's zero-tax test computed from **complete** argmax sets; freeze 26's
  least-domino-index tie rule is not used; and every reported column is in the
  **count** convention by Corollary SR-conv, a differential tax being exactly twice
  its count value. **§6.3 adds a second discipline on top of (e), and it is new:
  the complete optimal face is not sufficient at rung two either — every first
  action must be covered.**
- **SR-A22 (the depth-two probe: BUILDABLE AGAINST THIS ENGINE AS-IS — GRANTED as
  the SR family; FREEZE 51 fixes its carrier).** The build the note proposes at
  §13.5 is the right build and is admitted, re-scoped. Seven clauses.
  (i) **What it computes.** For each carrier unit — a (coordinate, binding
  competitor action `a`) pair already in freeze 50 — the second-frontier data
  `μ_{I,b,J}(ω)` and `q_{I,b,J}(ω,c)`; per `(I,b,J)` the aggregates `C_{I,b,J}`,
  every `A_{I,b,J,c}`, and `δ_{I,b,J}`; per `(I,b)` the branch values
  `F^(1)_{I,b}`, `F^(2)_{I,b}`, the slack `s_{I,b}` and the downstream tax
  `d_{I,b}`; per `I` the complete rung-one optimal face `B*_I`, the complete
  argmin set of `s + d`, the local tax `Δ_I^(2) = min_b(s_{I,b} + d_{I,b})` and the
  **escape flag** `[argmin_b(s+d)] ∩ B*_I = ∅`; and per unit
  `U^(2) = Σ_I max_b F^(2)_{I,b}` and `Δ^(2) = Σ_I Δ_I^(2)`.
  (ii) **Why the machinery already exists, and the four changes it needs.** The
  `walk` of `fusion_tax.rs` already descends into **every** action at the first
  frontier and records `child[j]` per action, so the traversal that produces the
  rung-two table is the traversal S6k already runs — the depth-two probe adds
  recording, not search. Four concrete changes, named so the builder does not
  rediscover them: **(1)** `Arrival::den` stops accumulating once `seen_focal` is
  set (`den: if arr.seen_focal { arr.den } else { arr.den * legal.len() }`) and
  must accumulate through the second frontier; **(2)** `Arrival::prefix` likewise
  stops accumulating and must carry the between-frontier increment; **(3)**
  `seen_focal` becomes a depth counter, not a bool; **(4)** `DEN_MU = 12^6` carries
  only pre-frontier-1 arrival denominators — at most six field plies precede the
  first frontier (records of length 4 to 7 are observed) and at most six more
  precede the second, so the depth-two common denominator is `12^12`, which is
  `SCALE`. **Every one of these is asserted, never assumed**, exactly as the
  existing `assert_eq!(w * arr.den, DEN_MU, …)` and the exact-field-average
  assertion already are. The rung-one frontier table is otherwise reused verbatim,
  and `FrontierState::acc_q[j]` **is** `F^(1)_{I,b_j}` — the slack column is
  already computed and merely unprinted.
  (iii) **FREEZE 51 — the depth-two probe carrier.**
  **(a) The carrier, enumerated, with no generating rule** (FT-A23's rule: a freeze
  is a constant, not a rule): **arm 1, mandatory** — coordinate h2, pip 5, hand
  `[21 33 53 54]`, both freeze-50 units, competitor `a = 53` and competitor
  `a = 54`, in that order. **Arm 2, attempted after arm 1 completes, with a declared
  stop** — coordinate h9, pip 4, hand `[30 41 54 61]`, units `a = 41` then
  `a = 54`. h0, h6 and h12 are **out of scope** for this build. Coordinate identity
  is asserted first in freeze 45's form — grade, declaration, hand and pool as
  canonical ascending domino-index tile lists, leader offset 0, `|X| = 34,650`
  against `kernel.count()`, freeze-7/23 enumeration order, kernel rebuilt in-run and
  asserted equal.
  **(b) Why h2 first and why h9 second.** h2 is the smallest first frontier in the
  carrier (330 states, 554,400 arrivals) and is the coordinate the note itself
  nominates; h9 is the second smallest (1,320 states, 2,217,600 arrivals), carries
  the branch's largest exact negative, and is the coordinate the exact primal route
  **cannot price** — so a rung-two `U^(2)` there is a second independent check on a
  `Q^H` that has been computed once. **h9's NOT PRICED label stands verbatim and is
  not weakened by this** (FT-A18(iv), RW-A3(iii)), and at h9 `L_{a⋆} = Q^H(a⋆)` is
  Corollary E4.1(2)'s ceiling, not a receipted primal witness; every h9 row says so
  in place.
  **(c) The frontier-2 convention, fixed by Lemma SR-forced.** The second frontier
  is the focal seat's **next decision after `b`, forced or not**. A forced `J` is a
  frontier state with `|A(J)| = 1` and `δ_{I,b,J} = 0`, and it is **counted, not
  skipped** — matching rung one, where S6k already emits forced frontier states
  (3,188 of them at h0). The early-terminal mass `p^term_{I,b}` and `Θ_{I,b}` are
  **asserted zero** at grade 4 and the assertion is contentful (SR-R2).
  **(d) The emission format, cut by CONTENT per FT-A24.** *Committed file:* one row
  per `(unit, I, b)` carrying `I`'s record as an ascending-domino-index play list,
  `p_I`, `|A(I)|`, `b`, `F^(1)_{I,b}`, `s_{I,b}`, `d_{I,b}`, `|I_2(I,b)|`,
  `#{J : δ_{I,b,J} > 0}`, and two flags (`b ∈ B*_I`, `b ∈ argmin(s+d)`); plus one
  row per `(unit, I)` carrying `M_I`, the **complete** `B*_I`, the **complete**
  argmin set, `Δ_I^(2)`, and the ESCAPE flag. At h2 that is at most `330 × 3 = 990`
  branch rows and 330 state rows per unit — printed entire, no cap needed; a cap
  fires only at h9 and is declared, never silent. *Companion, regenerable and NOT
  committed, with its SHA-256 and byte/line counts in the committed header:* one
  row per `(I,b,J)` with `J`'s record, `p_{I,b,J}`, `|X_J|`, `|A(J)|`,
  `C_{I,b,J}`, every `A_{I,b,J,c}`, `δ_{I,b,J}`, and the complete `argmax_c` set.
  **Accounting integers per unit make the omission auditable** (FT-A24(iv)):
  `|I_1|`, `Σ_I |A(I)|`, `Σ_{I,b} |I_2(I,b)|`, `#{(I,b,J) : δ > 0}`,
  `#{(I,b,J) : |A(J)| = 1}`, `#{I : ESCAPE}`, and `Σ_I Δ_I^(2)` asserted equal to
  the reported `Δ^(2)`.
  **(e) Every column in the COUNT convention** (freeze 38(f), Corollary SR-conv),
  with the convention stated in the header and the differential-is-twice rule
  printed. **(f) Belief and field are NOT re-declared** — freeze 26 and freeze
  37(d), cited unchanged, uniform over the full enumerated fiber, no decimation
  anywhere inside any `L`, `U`, `s`, `d` or `δ` ((C2)). **(g) No library entry is
  written at any coordinate** (freeze 45). **(h) The freeze-set digest travels on
  every record; a digest mismatch is corruption and the cache is discarded entire**
  (freeze 41, DS-A30).
  (iv) **Budgets: no new constant is fixed.** Freeze 44(b) v2's contract binds
  every `walk`-based evaluator unchanged — `B = 10,000,000,000` walk-steps per
  (coordinate, action) for **each** evaluator, charge-then-descend at `bag.len()`
  on entry, `Option` return, and **on exhaustion no partial fold of any kind is
  retained**, which here means no partial `s`, no partial `d`, no partial `Δ^(2)`
  and no partial `U^(2)`. The second-frontier partition count
  `Σ_{I,b} |I_2(I,b)|` is asserted against `P_max v2 = 192,000,000` **before** the
  aggregate pass, and the assertion is contentful.
  (v) **Filed values enter as a frozen table with a named carrier** (SEP-A14(ii),
  FT-A28(i)). Three sources, each transcribed into the probe source with its own
  provenance line and **never re-parsed from results text**: `Q^H(a)` and `U_a`
  quoted from `separation_n4_2026-08-14.txt`; `Δ^(1)`, `U^(1)`, `Δ^(2)`, the
  frontier census and the arrival count per unit quoted from
  `fusion_tax_2026-08-14.txt` and from FT-A24(ii)/the FT closing note — this is the
  `FT_FIRST` table of (FT-R7a), extended; and `L_{a⋆}` per binding pair from the
  same S6k rows. At h2 the reference constants are `Q^H = 85117/23100`,
  `U_a = 58639/15840`, `U^(1) = 102437/27720`, `Δ^(1) = 145/22176`,
  `Δ^(2) = 1483/138600`, `|I_1| = 330`, arrivals `554,400`, census `114/216`.
  (vi) **Vocabulary and typing, binding on every row.** No row names a per-world
  **bound**: the `q_{I,b,J}(ω,c)` are exact world-informed continuation values, not
  bounds, and none is ever carried out of a frontier and installed as a root primal
  witness (Non-theorem E4′, FT-A12(iii)). The probe pastes **no residual witness**;
  every continuation value is evaluated inside the same walk under the carried
  arrival weights — **form (i) of Lemma FT-post**, printed in place per SR-A2(iv).
  Every argmax and argmin printed is a **complete** set; a tie-broken optimiser
  appears nowhere (freeze 38(e), FT-A8, and §6.3's stronger rule).
  (vii) **What is NOT commissioned.** No penalty family, no feature moment, no
  regret-event minorant, no `b`-uniform event, no deeper rung, no trick-1 object,
  and no Python verifier (SR-A17(iii)). Freeze 38(g)'s exclusions stand.
- **SR-A23 (the receipts, since "by construction is not a receipt" — and after
  Proposition SR-taut, since "the algebra checks out" is not one either).**
  Mandatory, in the PG-A8 style, with the non-receipts named as such.
  (i) **(SR-R1) the branch reconstruction receipt — the one that earns the
  section.** For **every** `(I,b)`: assert
  `Θ_{I,b} + Σ_J C_{I,b,J} = F^(1)_{I,b}`, where the right-hand side is the
  rung-one branch value `Σ_ω μ_I(ω) q_I(ω,b)` computed by the rung-one path
  (`FrontierState::acc_q`). **Contentful and strong**: the two sides come from
  different passes over different intermediate quantities, and it fails on any
  error in second-frontier detection, in the between-frontier arrival weights, in
  the `J` keying, or in the parent attribution. A mismatch is stop-and-report; it
  is a bug in the probe or in Theorem 4.1's hypotheses, never a finding about the
  game (R-A18, NO-RESCUE).
  (ii) **(SR-R2) the mass receipt.** Assert `Σ_J p_{I,b,J} + p^term_{I,b} = p_I`
  for every `(I,b)`; `Σ_I p_I = 1`; `p^term_{I,b} = 0` and `Θ_{I,b} = 0`, the
  latter two by asserting the focal seat has a further decision in every
  positive-mass world after `b`. **Contentful**: it is the only check that the `J`
  list is exhaustive, and a dropped `J` is otherwise invisible — it lowers `C`,
  `A`, `F^(1)` and `F^(2)` together and understates `Δ^(2)` with every algebraic
  assertion still green. This is amendment (A) of SR-A17(ii).
  (iii) **(SR-R3) the parent receipt.** For every emitted `J`, assert that
  replaying `J`'s record recovers `(I,b)` exactly — Lemma SR-coord(b) instantiated
  — and that `I_2(I,b) ∩ I_2(I',b') = ∅` for distinct branches. **Contentful**: it
  is the direct test of the note's §2.2 hypothesis in this engine, and it fails if
  any coordinate coarsening has crept in.
  (iv) **(SR-R4) the ladder receipt.** Assert
  `U^(2) := T_0 + Σ_I max_b F^(2)_{I,b} = Q^H(a)` against the frozen filed
  `Q^H(a)`. **Contentful and the strongest single check in the build**: it tests
  Lemma FT-trunc, Corollary FT-grade4, Theorem 4.1's `U^(2)` formula and the entire
  depth-two construction at once, against an exact solve produced by a different
  evaluator on a different day.
  (v) **(SR-R5) the interchange receipt.** Assert
  `Σ_I min_b [s_{I,b} + d_{I,b}] = Δ^(2)` against the **frozen filed** `Δ^(2)`
  (`1483/138600` at h2). **Contentful**: this is Theorem 6.2 instantiated against a
  value produced by an entirely different route — `U^(1) − Q^H` from two filed
  columns. **Named as NON-receipts in place, per Proposition SR-taut:**
  `δ_{I,b,J} ≥ 0`, `d_{I,b} ≥ 0`, `s_{I,b} ≥ 0`, `F^(1) − F^(2) = Σ_J δ`, and
  `max_b F^(1) − max_b F^(2) = min_b(s + d)` are **arithmetic remarks — they cannot
  fail** and are printed as remarks, never counted among receipts HELD.
  (vi) **(SR-R6) the two-path receipt.** Compute `U^(2)` a second time by an
  independently written **glue-two-then-reveal** walker — a pooled bag lawful at
  the first frontier and lawful again at the second (`max_c` outside the world
  sum at both), world-informed below — and assert exact equality with the
  table-derived value. **Contentful**, because the two computations share only the
  rule algebra. This is (FT-R4) one rung up and it is the receipt that would catch
  a wrong `max`/`Σ` order.
  (vii) **(SR-R7) the rung-one invariance receipt, carrying FT-A28(iv)'s open
  obligation.** Assert the rung-one quantities recomputed by this probe —
  `p_I`, every `δ_I`, the complete argmax sets, `Δ^(1)`, `U^(1)`, `|I_1|`, the
  arrival count and the `zero/positive` census — against the extended `FT_FIRST`
  frozen table of SR-A22(v). **And discharge (FT-R7c):** emit per unit a SHA-256
  over the canonical serialisation of the `(record, δ_I)` pairs in freeze-38(d)
  order, transcribe it into `FT_FIRST` on the next run, and adopt **(FT-R7a)'s
  corrected scope line** verbatim: *"reaches `Σ_I δ_I` and `|supp δ_I|` per unit
  across executions; does not reach individual `δ_I`."* FT-A28(iv) made (FT-R7c)
  **binding on the next FT run that regenerates a frontier**; this is that run, and
  the obligation is live because no artifact discharges it (FT-A29(i)'s
  discipline — a ruling that creates an obligation is not evidence it is still
  open, and I checked: no results file carries a frontier digest).
  (viii) **(SR-R8) the complete-face receipt at rung two.** For a declared
  deterministic sample — the first ten `(I,b,J)` triples in freeze-38 v1.1(d) order
  at each unit — assert Corollary 5.2 both ways: where `δ_{I,b,J} = 0`, the
  complete `argmax_c` sets intersect; where `δ_{I,b,J} > 0`, the intersection is
  empty and a minimal fusion core of size exactly 2 is printed **with SR-A24(f)'s
  a-priori note beside it**. Additionally assert `|A(I)|` and `|A(J)|` against
  `legal_plays` at the reconstructed position — amendment (B) of SR-A17(ii), the
  (FT-R8) pattern — since §6.3 makes the completeness of the `b` range
  load-bearing.
  (ix) **(SR-R9) the reduced-grade cross-check — BLOCKING, run before any carrier
  number exists.** At a declared grade-3 coordinate the focal seat has two
  decisions after the root and the second is forced, so the second frontier is
  **entirely forced**: assert every `|A(J)| = 1`, every `δ_{I,b,J} = 0`, every
  `F^(2)_{I,b} = F^(1)_{I,b}`, `Δ^(2) = 0` and `U^(2) = U^(1) = Q^H` against the
  engine's own `H` operator. **Contentful and it tests the lemmas, not just the
  code** — a nonzero grade-3 `Δ^(2)` falsifies Lemma FT-trunc, Lemma SR-forced or
  the implementation, and either is stop-and-report. This is (FT-R6) one rung up
  and it is the only check in the build that exercises the frontier-2 detector
  against a case whose answer is known by proof rather than by a filed number.
  (x) **(SR-R10) determinism.** (FT-R7b)'s pattern: a full in-run second pass with
  fresh maps, accumulators and budgets, every printed row and every summary value
  asserted identical; plus (SR-R7)'s per-unit digest, which closes the across-process
  residual FT-A28(iii) named.
- **SR-A24 (all outcomes pre-declared, before any depth-two number exists;
  including two that are settled a priori and must not be reported as
  measurements).** F7 binds: both answers to every open gate are results.
  (a) **(SR-R4) and (SR-R5) HOLD** → Theorem 6.2 and Theorem 4.1 are instantiated
  exactly at this carrier against two independently produced filed columns, and the
  `(s, d)` decomposition of the fusion gap exists as an artifact for the first time.
  This is the result the build is for, and it is a result about the **proof
  machinery**, not a discovery about 42 — the exact value column already knows
  h2's answer.
  (b) **(SR-R4) or (SR-R5) FAILS** → the most informative outcome available and
  pre-declared as such: either Theorem 4.1's hypotheses fail in this engine, or
  Theorem 6.2 does, or the implementation is wrong. Nothing is claimed, the
  disagreeing exact rationals are printed, and no patch is attempted (F7,
  NO-RESCUE).
  (c) **ESCAPE ACTIONS PRESENT** — at some `I`, `argmin_b(s_{I,b} + d_{I,b})` is
  disjoint from `B*_I` → the first measured instance of policy adjustment in the
  branch. §6.3's warning is then not hypothetical at our scale, and **every future
  rung-two lower witness must cover every first action**, not the optimal face. A
  result, filed as one, scoped to this coordinate and nothing wider.
  (d) **ESCAPE ACTIONS ABSENT at every `I` of the carrier** → the minimum is
  attained on the rung-one optimal face throughout, so the naive
  `min_{b ∈ B*_I} d_{I,b}` would have coincided with the truth here. **This is a
  result too, and it is not a licence**: §6.3's inequality is one-directional and a
  coincidence at two coordinates licenses nothing about a third. Filed under F7 with
  P-A21 and the FT-A26(iii) selection fence attached.
  (e) **A budget stop, or `P_max` exceeded at the second frontier** → declared stop,
  no partial fold retained (freeze 44(b)), no partial `s`, `d` or `Δ^(2)` reported,
  printed as a stop and never as a finding (R-A18). Arm 2 (h9) stopping while arm 1
  (h2) completes is an ordinary outcome, not a failure.
  (f) **SETTLED A PRIORI, and reportable only as such — rung-two fusion cores.**
  At grade 4 the focal seat holds two tiles at the second frontier, so
  `|A(J)| ≤ 2`; argmax sets are then nonempty subsets of a two-element set, and an
  empty intersection forces one world with `{c_1}` and one with `{c_2}`. **Every
  positive-`δ` minimal core therefore has size exactly 2, by arithmetic, before any
  measurement.** The note's open ledger row *"second-rung fusion cores remain
  binary — measure the second-frontier conflict hypergraph separately"* is
  **unmeasurable at this carrier**, and the run may not be reported as answering
  it. It is open only at grade ≥ 5, where `|A(J)| ≥ 3` becomes possible. This is
  FT-A26(ii)'s lesson caught before the run instead of after it.
  (g) **SETTLED A PRIORI — pair closure.** By Proposition SR-degen, `L_{a⋆} ≥
  U_a^(2)` holds at every binding pair of this carrier unconditionally, with
  equality at h2 and h9 (both tied). **No closure verdict is reported for this
  build**, and the results header prints Proposition SR-degen in place. What the
  build reports is the identity, the decomposition and the escape census.
- **SR-A25 (fences, carried obligations, and what this section is not).**
  (i) **The R-A2 fence, mandatory in the results header**, unchanged: no object
  produced by this probe is an identity-bearing witness of anything; reachability
  is a proof-irrelevant proposition; the carrier is the void-free capacity fiber
  whose members are **FEASIBLE and never reachable** (P-A1). The N4-A8 real-deal
  fence travels with every carrier coordinate verbatim — the hands and pools come
  from rob's receipt corpus, **the belief does not**, the voids the play record had
  already revealed are deliberately discarded, and the void-filtered column licenses
  nothing.
  (ii) **Not claimed, printed in place:** nothing about points or marks (the
  valuation is the count-free trick differential; E-A2's boundary, and a count
  re-entry voids every form-keyed record wholesale); nothing about bidding; nothing
  about how real opponents play; **no distribution measured at grade 4 quoted for
  trick 1 or for the opening** (P-A21); and no cost, timing or tractability claim
  read off any traversal observable (SEP-A19(b), N4-A16) — walk-step and wall-clock
  columns are provenance.
  (iii) **The selection fence, restated because the received note omits it and
  because it binds every escape-action and sparsity number this build produces:**
  five coordinates chosen by negative binding margin are a **carrier, not a
  sample**, and the selection criterion is correlated with the quantity being
  described. Two of the five are in scope here, and neither the escape rate nor the
  rung-two tax density may be read as a distribution over coordinates or over
  hands.
  (iv) **DS-A28(ii) remains CARRIED**, and the errata §9 queue named at FT-A22(iii)
  and FT-A27(i) now also carries **Lemma SR-coord, Lemma SR-forced, Proposition
  SR-sep, Proposition SR-post, Corollary SR-conv, Proposition SR-degen and
  Proposition SR-taut**, together with the confirmed second-rung mathematics of the
  received note. Until that amendment, `walt/CENSUS-RULINGS.md` is their only
  authority.
  (v) **The wiki is not mine to touch and one item is owed to its owner.** The
  freeze register was de-staled through freeze 50 v1.1; **freeze 51 (SR-A22(iii))
  and freeze 38 v1.1(d) (SR-A21(ii)) are new and are not in it**, and the
  claim-ledger / FINDINGS / open-problems cross-references for this adjudication
  are likewise owed. Listed here so it is not lost; not dischargeable in this file.
  (vi) **On mechanization, the FT-A22(iv) tier fence carries unchanged and now
  covers more.** Theorem 4.1, Lemma 6.1, Theorem 6.2, Theorems 8.1/9.2/9.3 and the
  §11 rules are small finite-model statements and are legitimate Lean targets —
  Lemma 6.1 and Proposition SR-taut in particular are two-line kernel targets. **A
  kernel proof of the abstract model is a kernel-tier fact about that model and
  promotes nothing about walt's engine**, whose claims remain exploratory and remain
  hostage to T1-A12's implementation-versus-corpus risk. Tiers are never blurred by
  a proof of a neighbouring statement.
  (vii) **The load-bearing risk, named so it is watched, and it is sharper here
  than at rung one.** Lemma SR-coord — the hypothesis that makes Theorem 4.1 true
  at all — was discharged by reading `fusion_tax.rs` and freeze 26's contract at
  adjudication time. If the implementation and the rules corpus disagree, the
  mathematics above is still correct and its application here is wrong, and **no
  receipt inside this section can detect it**, because every receipt is computed by
  the same implementation. (SR-R9) is the partial guard. The corpus check of
  T1-A12 and LD-A10(ii) is still owed before any of this leaves walt.
- **SR-A26 (what none of this claims, in one paragraph).** The received note is
  correct mathematics and it answers all four asks. It does not price a single
  trick-1 competitor, it does not produce a compact lower plan, it does not make
  any feature moment countable, and it does not close anything at grade 4 that
  Proposition SR-degen does not close for free. Its contribution is that the
  nonanticipativity ladder now has an exact rung-two law with a named policy
  adjustment term, a proved multistage dual with exact recovery at every rung, and
  a safe-addition calculus with the two minima correctly typed. That is a
  well-specified next probe and a well-specified next wall — nothing more, and the
  h6 closure of S6k remains what FT-A27(iii) says it is.

**What the build owes this section.** The SR probe of SR-A22 over freeze 51's
arm 1 (h2, both units), with arm 2 (h9) attempted under a declared stop; the ten
receipts of SR-A23 with (SR-R9) blocking before any carrier number exists and the
non-receipts of (SR-R5) printed as arithmetic remarks; the freeze-38 v1.1(d) cut
order and the freeze-38(f)/Corollary SR-conv count convention on every column;
FT-A28(iv)'s deferred (FT-R7c) digest and (FT-R7a)'s corrected scope line,
discharged here; Proposition SR-degen and SR-A24(f)'s a-priori note printed in the
results header beside the outcomes; and all of SR-A24's outcomes pre-printed
before any number exists. Everything else above is proof and needs no code. If
(SR-R1), (SR-R4), (SR-R5) or (SR-R6) fails, nothing is claimed and the disagreeing
exact rationals are reported.

---

### Closing note: the SR probe returned (2026-08-14, after the run)

**Object:** `walt-factory/examples/second_rung.rs` and
`walt-factory/results/second_rung_2026-08-14.txt`, committed at `8e415aa`;
`ci/check.sh` PASS; the companion `second_rung_frontier_2026-08-14.txt`
regenerable and not committed, with its SHA-256, byte count and line count in the
committed header. Four units — h2 `a = 53`, h2 `a = 54`, h9 `a = 41`, h9 `a = 54`
— **all ten receipts HELD at every unit**, with (SR-R9) HELD as a blocking
pre-check at the declared grade-3 coordinate before any carrier number existed.
**Arm 2 completed; no declared stop occurred.** Three questions the build raises
are ruled at SR-A27..SR-A30 and three findings are typed at SR-A31.

**Re-derived at adjudication time, independently of the run**, by parsing the
committed file's 3,300 printed state rows and 9,900 printed branch rows and
recomputing every quantity from the branch rows alone — the run's own summary
values were used only as the thing being checked:

- `M_I = max_b F^(1)_{I,b}`, `s_{I,b} = M_I − F^(1)_{I,b}`, the complete `B*_I`,
  `Δ_I^(2) = min_b(s_{I,b} + d_{I,b})`, the complete argmin, and the ESCAPE flag
  **reproduce at every one of the 3,300 states — zero deviations.**
- `Σ_I p_I = 1` exactly at all four units.
- `Σ_I M_I` = `102437/27720` at h2 and `1122491/332640` at h9 = the filed
  `U^(1)` exactly.
- `Σ_I Δ_I^(2)` = `1483/138600` at h2 and `4532503/26611200` at h9 = the filed
  `Δ^(2)` exactly.
- `Σ_I (M_I − Δ_I^(2)) = Σ_I max_b F^(2)_{I,b}` = `85117/23100` at h2 and
  `28422259/8870400` at h9 = the filed `Q^H(a)` exactly.
- The escape census reproduces: **36 of 330 at each h2 unit, 498 of 1,320 at each
  h9 unit.**
- `s ≥ 0` and `d ≥ 0` everywhere; `d_{I,b} > 0` iff `#{J : δ_{I,b,J} > 0} > 0` at
  every one of the 9,900 branches; and **§6.3's inequality
  `min_b(s+d) ≤ min_{b∈B*_I} d` holds at every one of the 3,300 states**, strictly
  at exactly the escape states.

### Proposition SR-loc (escape is exactly where §6.3 is strict, and the naive error is exactly localised) — delivered here

At a first-frontier state `I`, write `N_I = min_{b ∈ B*_I} d_{I,b}` for the naive
local tax that taxes only the rung-one optimal face. Then

  **(a)** `Δ_I^(2) = N_I` **if and only if** `argmin_b(s_{I,b} + d_{I,b})` meets
  `B*_I`; equivalently, `Δ_I^(2) < N_I` **if and only if** `I` is an ESCAPE state;

  **(b)** consequently `Σ_I N_I − Δ^(2) = Σ_{I ∈ ESCAPE} (N_I − Δ_I^(2))`, every
  non-escape state contributing exactly zero.

*Proof.* On `B*_I` the slack vanishes, so `min_{b∈B*_I}(s+d) = N_I`, and
minimising over the larger set `A(I)` gives `Δ_I^(2) ≤ N_I` — which is §6.3.
If some `b_0 ∈ argmin ∩ B*_I`, then `Δ_I^(2) = s_{I,b_0} + d_{I,b_0} = d_{I,b_0}
≥ N_I`, so equality. Conversely if `Δ_I^(2) = N_I` then the `b ∈ B*_I` attaining
`N_I` attains the global minimum, so the argmin meets `B*_I`. That is (a), and
(b) is (a) summed. ∎

**Why it is worth stating.** It converts the escape flag from a *diagnostic* into
an *exact accounting identity*: the escape set is precisely the support of the
error a §6.3-violating witness would make, and the error is the sum over that
support and over nothing else. It also makes the census auditable from the branch
rows alone, which is how it was audited above.

**Measured at adjudication time from the committed rows.** The naive quantity
`Σ_I N_I` is `1543/138600` at h2 against the true `1483/138600` — an
overstatement of exactly `1/2310`, which is `60/1483` = **4.0459%** of the true
tax; and `12667/66528` at h9 against `4532503/26611200` — an overstatement of
exactly `178099/8870400`, or **11.7881%**. At h2 the arithmetic closes to the
unit: each of the 36 escape states overstates by exactly `1/83160`, and
`36 × 1/83160 = 1/2310` is the whole of it.

---

- **SR-A27 (the artifact against the SR-A22/SR-A23 contract: NO DEVIATION FOUND;
  four places where the build exceeded what I specified, and one presentational
  nit).** Five clauses.
  (i) **Every contract item is present and correct.** Freeze 51(a)'s carrier in
  the enumerated order with no generating rule; freeze 51(c)'s counted-forced-`J`
  convention with `Θ_{I,b}` and `p^term_{I,b}` asserted zero; freeze 38 v1.1(d)'s
  induced total order exhibited in the header; freeze 38(f)/Corollary SR-conv's
  count convention **with the two bridges kept separate and correctly stated** —
  differences halved, `p`-weighted values bridged as `(x_diff + grade·p)/2`, which
  is the exact distinction SR-conv was written to prevent collapsing; the four
  engine changes of SR-A22(ii) each asserted in-run rather than assumed; SR-A22(vi)'s
  slot typing and Lemma FT-post form (i); the accounting integers of SR-A22(iii)(d);
  and all seven of SR-A24's outcomes pre-printed before any number.
  (ii) **Exceeded, four times, each in the safe direction.** (1) **(SR-R8)** was
  specified over a declared ten-triple sample and was run over **every**
  `(I,b,J)` triple — 2,535,480 per h2 unit, 18,110,322 per h9 unit — with the
  sample printed. (2) **(SR-R3)** likewise runs at every second-frontier state
  rather than as a spot check, and does so by an **independent replay that shares
  no bookkeeping with the walk**, which is a stronger instantiation of Lemma
  SR-coord(b) than I asked for. (3) `|A(I)|` and `|A(J)|` are asserted against
  `legal_plays` **at the independently reconstructed position**, not against the
  walk's own cached legal set — amendment (B) of SR-A17(ii) implemented in its
  strong form. (4) **(SR-R10)** covers the companion digest and the frontier digest
  as well as the printed rows. **An implementation that claims more coverage than
  the ruling demanded is checked, not waved through** — I re-derived the
  aggregates from the printed rows precisely because the receipts are self-reported;
  they agree.
  (iii) **The one non-receipt correctly refused.** SR-A24(f) is printed and its
  closing sentence reads *"The 57240 positive-delta triples of this unit all carry
  a size-2 core and that is a restatement of the arithmetic, not a measurement."*
  That is the sentence FT-A26(ii) had to be written *after* a run; here it was
  written before one and honoured in the artifact. Likewise Proposition SR-taut's
  five are printed as ARITHMETIC REMARKS and are excluded from "all ten receipts
  HELD". The count of ten is therefore accurate and I have verified it names ten
  distinct contentful checks.
  (iv) **The presentational nit, recorded because it cost me a re-run.** The
  ESCAPE column prints `YES` at escaping states and `no` elsewhere — a case
  inconsistency that made my first adjudication-time parse mis-flag all 36+498
  escapes before I read the raw row. **It is cosmetic and nothing downstream may
  key off it** (SEP-A14(ii): results text is not an interface), so no re-emission
  is owed; but a column that is read by eye should not change case with its value,
  and the next emitter should print `yes`/`no`. **Binding on the next SR emission,
  forcing nothing now.**
  (v) **A trend flagged before rung three, not a defect now.** The companion is
  8,811,585,684 bytes over 41,291,617 lines, against S6k's 36 MB. FT-A24's design
  — accounting integers plus a digest — is exactly what makes that omission
  auditable and it works here. But "regenerable" is a weaker practical guarantee at
  8.8 GB than at 36 MB, because auditing the digest now costs a full re-run.
  **At rung three the committed/companion split must be re-designed rather than
  re-applied**, and that is a design obligation on the next adjudication, not a
  finding against this one.
- **SR-A28 (the escape census: SR-A24(c) FIRES — the first measured instance of
  policy adjustment in the branch; RESULT, with its scope cut three ways).** Five
  clauses.
  (i) **The finding.** At both carrier coordinates and at all four units,
  `argmin_b(s_{I,b} + d_{I,b})` is disjoint from the rung-one optimal face `B*_I`
  at a positive fraction of first-frontier states: **36 of 330 (10.91%) at h2, 498
  of 1,320 (37.73%) at h9**, re-derived here from the branch rows. §6.3's warning
  is therefore **not hypothetical at our scale**, and the pre-declared consequence
  binds: **every future rung-two lower witness must cover every first action, not
  the rung-one optimal face and not merely the complete optimal face.** FT-A8 bars
  a tie-broken optimiser in favour of the complete face; §6.3 as now measured says
  the complete face is *also* insufficient. Both rules bind and they are different
  rules.
  (ii) **What it would have cost, exactly** (Proposition SR-loc, computed at
  adjudication time): a witness taxing only the rung-one optimal face would have
  reported `1543/138600` at h2 against the true `1483/138600`, and `12667/66528`
  at h9 against the true `4532503/26611200` — overstatements of `1/2310`
  (**4.0459%**) and `178099/8870400` (**11.7881%**). Since the naive quantity is an
  **upper** bound on the true tax, a witness built on it claims to have shaved more
  than it did; at h9 it would have claimed nearly an eighth more. **That is the
  number that makes §6.3 a safety rule rather than a technicality.**
  (iii) **The counts are NOT independent observations, and must never be reported
  as though they were.** At h2 all 36 escapes carry **one signature**:
  `Δ_I^(2) = 1/49896`, `N_I = 1/31185` (a ratio of exactly `5/8`), `B*_I = {33}`,
  `argmin = {54}` — and `{53}` in the mirror unit. At h9 the escape action is the
  single tile **61 at every one of the 498**, against a singleton face `{54}`
  (resp. `{41}`), with the multiplicities arriving in blocks of 6 and 12. These are
  one structural phenomenon reached by many field continuations, not many
  phenomena. **The honest statement is "escape occurs, at these coordinates, with
  this structure"; the dishonest one is "escape occurs at 37.73% of states" read as
  a rate.** This is FT-A26(iii)'s selection fence in its sharpest form yet, and the
  results file's own *"scoped to this coordinate and nothing wider"* is correct.
  (iv) **Two structural facts, typed as observations and claimed as nothing
  more.** No escape state has `Δ_I^(2) = 0` — escaping *reduces* the local tax
  everywhere it happens and *eliminates* it nowhere, so the escape route is never a
  free rescue at this carrier. And at h9 the escape states carry
  `2134575/4532503` = **47.09%** of the whole `Δ^(2)` while being 37.73% of the
  states (at h2, `100/1483` = 6.74% of `Δ^(2)` from 10.91% of the states). **No
  causal claim, no growth law, and nothing quoted for trick 1 or the opening**
  (P-A21); the coordinates were selected by negative binding margin and the
  selection criterion is correlated with the quantity described.
  (v) **What SR-A24(d) would have meant, recorded because it did not happen.**
  Had escapes been absent, the reading was pre-declared as a result and not a
  licence. It is worth noting in place that the outcome went the *other* way, which
  is the direction that makes the received note's §6.3 load-bearing rather than
  cautionary — and that this is the second time in the FT/SR chapters that a
  pre-declared "both answers are results" gate came back on the side that costs
  more work.
- **SR-A29 (arm 2 completed: SR-A24(e)'s stop did NOT occur, and the h9 ladder
  agreement is typed precisely).** Four clauses.
  (i) **The stop did not occur and that is an ordinary outcome, not a bonus.**
  `Σ_{I,b} |I_2(I,b)| = 18,110,322 ≤ P_max v2 = 192,000,000`, asserted before the
  aggregate pass; PATH A2 and PATH B2 each charged 7,253,759,970 of their separate
  `10^10` freeze-44(b) budgets. SR-A24(e) pre-declared arm 2 stopping as ordinary;
  arm 2 completing is equally ordinary and **no cost, timing or tractability claim
  is read off any of those integers** (SEP-A19(b), N4-A16) — the walk-step columns
  are provenance.
  (ii) **What (SR-R4) at h9 IS.** `Σ_I max_b F^(2)_{I,b} = 28422259/8870400`
  exactly equals the filed `Q^H(41)` (resp. `Q^H(54)`). h9's `Q^H` had been
  produced once, by the revealed/H traversal. This reconstructs it from a
  **depth-two decomposition** — a different traversal, different intermediate
  quantities, a different theorem — and agrees to the rational. **It is a second
  independent confirmation of h9's exact lawful value, and it is a different
  decomposition from (FT-R1)'s**, which reconstructed h9's `U` (the *revealed*
  value) from the depth-one frontier. So h9's two filed columns have now each been
  independently reconstructed once, by two different routes. That is the quietest
  good news in this file, as (FT-R1) at h9 was in the last one.
  (iii) **What it is NOT, and this is the clause that must travel.** **h9's NOT
  PRICED label stands verbatim and is not weakened by any of it** (FT-A18(iv),
  RW-A3(iii): the labels never merge). NOT PRICED is a statement about the
  **primal pipeline** — h9's extraction map measured 517,562,322 states against
  `P_max v2 = 192,000,000`, so no primal witness is exhibited at that coordinate,
  and `L_{a⋆} = Q^H(a⋆)` there is Corollary E4.1(2)'s **ceiling**, not a receipted
  primal witness. Reconstructing a value twice on the **dual** side says nothing
  about the primal side. A cross-check is not a witness; agreement between two
  computations of the same quantity does not manufacture the object the pipeline
  could not build. The results file says this in place at every h9 row and is
  correct to.
  (iv) **And it is still hostage to SR-A25(vii).** Both reconstructions are
  computed by the same implementation whose reading of the rules discharged Lemma
  SR-coord. Two agreeing traversals inside one implementation cannot detect an
  implementation-versus-corpus divergence. (SR-R9) is the partial guard; T1-A12
  and LD-A10(ii)'s corpus check is still owed before any of this leaves walt.
- **SR-A30 (FT-A28(iv)'s deferred (FT-R7c) is DISCHARGED — cited to the artifact,
  per FT-A29's discipline).** Four clauses.
  (i) **The obligation and why it was live.** FT-A28(iv) specified (FT-R7c), the
  per-unit frontier digest, and made it *"binding on the next FT run that
  regenerates a frontier"*. SR-A23(vii) carried it into this build after checking —
  per FT-A29(i), *a ruling that creates an obligation is not evidence it is still
  open; only the artifact is* — that no results file carried a frontier digest.
  (ii) **It is discharged, and here are the carrying rows.** Each of the four
  units prints, on its (SR-R7) line, a SHA-256 over the canonical serialisation of
  its `(record, δ_I)` pairs in freeze-38(d) order — one line per state,
  `<record>|<δ_I as num/den, count>` — asserted equal to a frozen transcribed
  value: `bcd7e915…1106` (h2 `a = 53`, results line 1422), `de460262…244fa`
  (h2 `a = 54`, line 2815), `0d059121…514e` (h9 `a = 41`, line 8630),
  `b196c789…242c` (h9 `a = 54`, line 14445). Each is *"a comparison against a PRIOR
  PROCESS"*, which is exactly what FT-A28(iii)'s named residual required: a
  per-row discrepancy that arises across processes while preserving `Σ_I δ_I` and
  `|supp δ_I|` per unit and reproducing within each process is now caught, because
  one scalar per unit reaches **every individual `δ_I` across executions**.
  **FT-A28(iii)'s residual is CLOSED — by receipt, not evidentially**, and
  FT-A28(v)'s orchestrator byte-diff remains what it was ruled: an audit note,
  never a receipt.
  (iii) **(FT-R7a)'s corrected scope line is adopted verbatim** at every unit
  (*"reaches `Σ_I δ_I` and `|supp δ_I|` per unit across executions; does not reach
  individual `δ_I`"*), with the file adding, correctly, that the digest is what
  extends the reach and *"does so only once a later run carries it transcribed"* —
  which is the right reading of a digest's evidentiary direction and is a sharper
  statement than my own SR-A23(vii).
  (iv) **What remains owed on this line: nothing.** FT-A28(vii) listed exactly two
  items for the next FT run; both are discharged here. **FT-A28 is fully
  discharged.** The digest becomes part of the emitter, as FT-A28(iv) recommended,
  and any future frontier-regenerating run carries one without further ruling.
- **SR-A31 (three findings of the run that are results in their own right, each
  with its fence).**
  (i) **Theorem 6.2 and Theorem 4.1 are instantiated exactly, at two coordinates,
  against three independently filed columns.** (SR-R4) reproduces the filed `Q^H`,
  (SR-R5) reproduces the filed `Δ^(2)`, and (SR-R7) reproduces the filed `U^(0)`,
  `U^(1)` and `Δ^(1)` as by-products of a depth-two traversal — all exact, all four
  units, and all re-derived here from the printed rows. **This is SR-A24(a) and it
  is a result about the proof machinery, not a discovery about 42**: the exact
  value column already knew both answers, Proposition SR-degen fixed the closure
  verdict a priori, and no pair verdict is reported. What is new is that the
  received note's rung-two law now has an exact instantiation in this engine, and
  that the `(s, d)` decomposition of a fusion gap exists as an artifact for the
  first time.
  (ii) **The escape census, SR-A28, with its three fences.**
  (iii) **(SR-R9) did work that no filed number could have done.** At the grade-3
  coordinate all second-frontier states are forced — 50,328, 40,596 and 41,364 of
  them at the three roots, every one with `|A(J)| = 1` and `δ = 0` — and
  `U^(2) = U^(1) = Q^H` held against **the engine's own `H` operator**, an
  independent evaluator the grade-4 carrier cannot consult. It is the only check in
  the build whose answer was known **by proof** (Lemma FT-trunc and Lemma
  SR-forced) rather than by a filed rational, and it ran blocking, before any
  carrier number existed. It also exercises freeze 51(c)'s counted-not-skipped
  convention against the one case where the two ladder indexings visibly differ.
  **A build whose strongest checks are all against filed numbers can be
  self-consistently wrong; this is the check that is not.**
- **SR-A32 (carried obligations, and what none of this claims).**
  (i) **Nothing is promoted.** Every receipt, verdict, tax, census and escape row
  above is exploratory, cited by nothing above this tier, and quotable as a result
  only by brief amendment adding it to a verifier receipt. TRUST-01 unchanged: the
  received note is not imported as an axiom, and every number in the artifact is an
  exact rational of this engine.
  (ii) **Not claimed**, everything at SR-A25(ii) unchanged, plus three specific to
  this run: **the escape census is not a rate** (SR-A28(iii)); **the h9 double
  reconstruction is not a primal witness and does not touch NOT PRICED**
  (SR-A29(iii)); and **the rung-two core sizes are not a measurement**
  (SR-A24(f), which the artifact itself states).
  (iii) **The errata §9 queue** of FT-A22(iii)/FT-A27(i)/SR-A25(iv) now also
  carries **Proposition SR-loc**. DS-A28(ii) remains carried.
  (iv) **Owed to the wiki owner, not dischargeable here**, and now three items:
  freeze 51 and freeze 38 v1.1(d) are absent from the register; the claim-ledger,
  FINDINGS and open-problems cross-references for both the SR adjudication and this
  closing note; and the LOG entry for the SR probe.
  (v) **Two obligations created here, both small and both on the next SR run:**
  the `yes`/`no` case fix of SR-A27(iv), and the committed/companion split
  re-design of SR-A27(v) before any rung-three build. **On nobody, now: anything
  else. Nothing is routed back to the builder** — the artifact discharges its
  contract in full and exceeds it in four places.

**What the build owes this section.** Nothing. The SR probe stands as committed at
`8e415aa`: ten receipts HELD at four units, (SR-R9) blocking and HELD, arm 2
completed, SR-A24(c) fired and filed with its fences, and FT-A28 discharged
entire. The next question is not a rung-three build — at grade 4 there is no rung
three, and Proposition SR-degen says grade 4 can no longer test closure at all.
**The next question is a coordinate where the ladder is longer than the corpus of
filed answers**, and that is where the received note's §14 program and FT-A21's
three obligations become the binding constraint rather than the escape census.

---

- **SR-A33 (the builder's first self-found defect — the streaming SHA-256 — and
  why its known-answer check is load-bearing for SR-A30).** Four clauses.
  (i) **What happened.** The probe's own SHA-256 `update` clobbered the buffered
  length across calls. The FIPS 180-4 known-answer self-check caught it **before
  any carrier number existed** — it hung on the first vector rather than filing a
  wrong digest — and the repair replaced a padding search loop with a computed pad
  length plus an assertion, and added the one-million-`a` many-block vector and
  irregular-chunk streaming checks. Read at adjudication time,
  `second_rung.rs:554` now carries four published vectors (`"abc"`, the empty
  string, the two-block message, and the one-million-`a` message) plus
  streaming-versus-one-shot agreement fed byte-at-a-time and in irregular chunks,
  and `sha256_self_check()` is **the first statement of `main`** (`:2993`), ahead
  of the (SR-R9) blocking block and everything else.
  (ii) **It is load-bearing for SR-A30 and I am recording that dependency
  explicitly.** SR-A30 discharged FT-A28(iv) on the strength of four digests, and
  ratified (FT-R7c)'s scope claim that one scalar per unit *reaches every
  individual `δ_I` across executions*. **That scope claim is a statement about the
  digest function, not about the probe.** A hash that mis-buffers is still
  deterministic, so run A and run B would still have agreed — the receipt would
  have been **green and worthless**, because a broken compression function may be
  wildly non-injective and the reach would silently degrade from "every individual
  `δ_I`" to "some lossy functional of them". **A digest receipt is only as strong
  as the primitive's anchoring to a published vector**, and this file's comment
  says exactly that: *"A digest primitive that has never been checked against a
  published vector is not a receipt of anything."* Correct, and now cited: SR-A30's
  discharge stands **because** SR-A33's self-check ran first.
  (iii) **The standing discipline, stated so it generalises.** *Any receipt whose
  assertion is an equality of digests carries a second, silent obligation: that the
  digest function is anchored to published known-answer vectors covering the code
  path actually used — including the streaming path if the receipt streams.* The
  streaming path is the one that broke here, and a one-shot-only vector set would
  have passed. This joins Proposition SR-taut and FT-A28(i) as the third member of
  one family: **a check is only a check against something it does not itself
  produce.**
  (iv) **The behaviour is commended in place.** The defect was in the builder's own
  code, was found by a check the builder wrote against itself, was found before any
  number existed, and was reported in full rather than quietly fixed. That is the
  third time in the FT/SR chapters that the build's own honesty is the reason a
  ruling can be trusted, and the first time the defect was the build's rather than
  the adjudicator's.
- **SR-A34 (the builder's second self-found defect — the `a⋆` selection: the
  "no receipt affected" claim is VERIFIED and RATIFIED; the guard's typing is
  CORRECTED; one residual named).** Five clauses.
  (i) **The claim, and it is true.** The pair-typing line initially selected `a⋆`
  as the first filed H-argmax, which at a tied coordinate is the unit's own action
  — so at h2 `a = 53` and h9 `a = 41` the line would have compared an action with
  itself. **Verified against the committed artifact at adjudication time**, not
  accepted: the four printed pairs are `a⋆ = 54 / a = 53` (line 1438),
  `a⋆ = 53 / a = 54` (2831), `a⋆ = 54 / a = 41` (8646), `a⋆ = 41 / a = 54`
  (14461), and **all four match S6k's filed `PAIR` rows exactly** — freeze 50(b)'s
  binding pairs, unchanged. And **no receipt line references `a⋆` or `L_{a⋆}` at
  any unit** (grep over every `(SR-R…)` line: zero hits). The claim is RATIFIED.
  (ii) **Why no receipt could have been affected, stated structurally rather than
  by grep.** (SR-R1) and (SR-R2) are internal to a branch; (SR-R4), (SR-R5) and
  (SR-R7) compare against `Q^H(a)`, `Δ^(2)` and the `FT_FIRST` row **of the unit's
  own action**; (SR-R3), (SR-R6), (SR-R8), (SR-R9) and (SR-R10) name no root
  action but `a`. `a⋆` enters this build in exactly one place — the pair-typing
  line — and by SR-A24(g) **no closure verdict is reported at all**, so the
  quantity `L_{a⋆}` does no work here beyond being printed. The exposure was
  bounded to a misprinted typing line, and the misprint would have been *vacuous*
  (`L_a ≥ U_a^(2)` is true and empty) rather than false.
  (iii) **But it was more than cosmetic, and I decline the softer reading.** The
  `(a⋆, a)` pair is **carrier data** — freeze 50(b) enumerates the binding pairs
  and freeze 51(a) inherits them — so printing `a⋆ = a` would have misidentified
  the freeze-50 binding pair in an artifact whose header claims to run over those
  pairs. It is a **carrier-identity defect in a printed line**, not a typo, and it
  is right that the builder treated it as one.
  (iv) **The guard's typing is corrected, and this is the part that matters.**
  The builder's report reads *"corrected to first H-argmax distinct from `a`, with
  the binding-margin sign `L_{a⋆} − U_a < 0` asserted"*, which invites the reading
  that the sign assertion backstops the correction. **It does not, and cannot.**
  Computed exactly at adjudication time: had `a⋆ = a` been selected, the asserted
  quantity would have been `Q^H(a) − U_a`, which is `−9557/554400` at h2 and
  `−2116837/8870400` at h9 — **negative at both, so the assertion passes and the
  defect goes undetected.** (Those are, recognisably, Proposition FT-tie's filed
  "required shave" figures, which is what `Q^H(a) − U_a` is.) The sign assertion is
  a real guard for a **different** property — that the unit is a binding competitor
  at all — and it should be described as that. The actual fix is the `*i != ia`
  predicate in the selector at `second_rung.rs:2413` together with its
  `.expect("a binding pair has an H-argmax competitor distinct from a")`, which
  fires if the H-argmax set is the singleton `{a}`. **Distinctness is therefore
  enforced by construction, and "by construction is not a receipt" (PG-A8).**
  (v) **The residual, named rather than papered over, and its cheap closure.**
  If the `*i != ia` predicate were ever dropped, nothing in-run would notice: the
  sign assertion passes either way and no receipt reads `a⋆`. The property
  `a⋆ ≠ a` is true in the committed artifact — I checked all four rows by hand
  against S6k above — but it is **not receipted**. **No re-emission is owed**: the
  exposure is one printed line, the property is verified here, and requiring a
  re-run to convert a known-true fact into a receipt spends real cost on process
  hygiene alone (FT-A28(iv)'s proportionality, applied). **Binding on the next SR
  emission**, and it should be the strong form rather than the cheap one: transcribe
  the filed `(a⋆, a)` binding pairs from S6k into `SR_FIRST` alongside the per-unit
  data already there, and assert the printed pair against them. That converts a
  construction into a comparison **against a named carrier**, which is FT-A28(i)'s
  discipline, and it costs one tuple field.
- **SR-A35 (the companion's cross-process digest identity: an AUDIT NOTE of real
  weight, not a receipt; convertible at zero cost).** Four clauses.
  (i) **What was observed.** (FT-R7c) was discharged across two deliberate
  processes — run A emitted the four frontier digests, the builder transcribed them
  into `SR_FIRST`, run B asserted against that prior process. Unplanned, the
  8.8 GB companion's SHA-256 also came out **byte-identical across the two
  processes**, over 41,291,617 rows.
  (ii) **Its evidentiary surface is far broader than the receipt's, and that is
  worth saying plainly.** The four (FT-R7c) digests cover the 3,300 rung-one
  `(record, δ_I)` pairs. The companion digest covers every `(I,b,J)` row — 2,535,480
  per h2 unit and 18,110,322 per h9 unit — i.e. every `C_{I,b,J}`, every
  `A_{I,b,J,c}`, every `δ_{I,b,J}` and every second-frontier record, across two
  processes. As *evidence* about the depth-two layer's cross-process determinism it
  is the strongest thing in the file.
  (iii) **It is nevertheless an AUDIT NOTE and may never be called a receipt.**
  FT-A28(v)'s three independently sufficient reasons apply verbatim: it is not
  asserted in-run against a transcribed constant (the companion digest is computed
  and printed but is **not** carried in `SR_FIRST` — verified at adjudication time
  against the frozen tuple, which holds one digest, the frontier one); it is not
  reproduced by any verify path; and it does not survive into a future run, which is
  what a receipt is *for*. **What it may be:** adjudication-time evidence, recorded
  here as such. **What it may never be:** cited as a receipt status, printed as
  HELD, or counted among "all ten receipts HELD" — and the artifact does not do any
  of those, correctly.
  (iv) **Convertible at zero cost, and it should be converted.** Adding
  `companion_digest` to `SR_FIRST` beside the frontier digest makes the next run
  assert **one further scalar per unit** and thereby reach every depth-two row
  across executions — the same move FT-A28(iv) made for the frontier. **Binding on
  the next SR emission, forcing nothing now**, and it joins SR-A34(v)'s tuple field
  and SR-A27(iv)'s case fix as one small batch of emitter work.
- **SR-A36 (the remaining disclosures: no spec conflicts, provenance, and the
  scaling flag reconciled).** Five clauses.
  (i) **The ambiguity protocol was never invoked, and that is a fact about the
  ruling, not only about the build.** All four SR-A22(ii) engine changes were
  implementable as written and each is asserted in-run — `den_2` a multiple of
  `den_1`; the pre-frontier and between-frontier increments exactly one trick each;
  the depth counter reaching `grade − 1` at every terminal, which is simultaneously
  the `T_0 = 0` and `Θ_{I,b} = 0` assertion; and `DEN2 = 12^12 = SCALE = DEN_MU²`.
  The FT chapter recorded two specification defects of mine and one of walt-math-10's
  (FT-A23, FT-A28(i), FT-A29(i)); this section records none, which is the first
  clean pass in the chapter and is worth noting **only** because the previous three
  were found by the build rather than by the adjudicator, and a builder that reports
  conflicts when they exist is the reason the absence of a report is informative.
  (ii) **Provenance, recorded and typed.** Six clippy lints fixed before any run;
  CI green before every launch and on the final tree; `ROW_CAP = 20,000` declared
  and never fired, so **every branch row and every state row of every unit is
  printed** and the emission carries no truncation — which is what made the
  adjudication-time re-derivation of all four units possible at all. Peak RSS
  ≈ 21 GB and the ≈ 10.5 minute runtime are **provenance only**: no cost, timing,
  memory or tractability claim is read off them (SEP-A19(b), N4-A16), and this
  clause is not an exception to that rule but an instance of it.
  (iii) **The walk-step equality is a confirmed prediction and is typed as a
  traversal-shape observable.** PATH A2's charge equals the filed FT PATH A
  subtotal exactly at every `(coordinate, action)`, both evaluators. That is
  SR-A22(ii)'s claim — *the depth-two probe adds recording, not search, because the
  rung-one walk already descends into every action at the first frontier* —
  confirmed against a filed integer. **It constrains no value and is not a
  receipt**; the value checks are (SR-R1), (SR-R4), (SR-R5) and (SR-R6). It is
  recorded because a design prediction that could have been wrong was not.
  (iv) **The scaling flag: SR-A27(v) stands, and the builder's framing is adopted
  with one sharpening.** The companion is 8.8 GB because h9 carries 18.1 M
  second-frontier states and SR-A22(iii)(d) mandates one row per `(I,b,J)`. **No
  rule is broken and nothing is owed now.** The sharpening is that the growth is
  not incidental: `|I_2(I,b)|` grows with the number of field plies between
  frontiers, so a third rung or a longer ladder multiplies it again, and the
  committed/companion split must be **re-designed rather than re-applied** before
  any such build. The design question to answer then is which *functionals* of the
  depth-`k` table a later run must be able to re-check — SR-A35(iv)'s digest is
  most of the answer, and a per-`(I,b)` committed summary with a per-unit digest
  over the full table may make the companion unnecessary rather than merely large.
  **Ruled when the family is extended, per the builder's own suggestion, and not
  before.**
  (v) **Nothing is routed back to the builder and nothing is promoted.** The
  artifact stands as committed at `8e415aa`. Both self-found defects were caught by
  the build's own checks before any carrier number existed, both were disclosed in
  full, and neither touches a receipt, a number or a verdict — the first verified
  structurally and by grep at SR-A34(i)–(ii), the second by the known-answer check
  having fired at SR-A33(i). Everything in this section is exploratory, cited by
  nothing above this tier.

**What the next SR emission owes, consolidated (nothing is owed now).** Four
items, all small and all in the emitter: SR-A27(iv)'s `yes`/`no` case fix;
SR-A34(v)'s filed `(a⋆, a)` binding pairs transcribed into `SR_FIRST` and
asserted; SR-A35(iv)'s `companion_digest` carried in `SR_FIRST`; and, before any
rung-three or longer-ladder build, SR-A36(iv)'s re-design of the
committed/companion split. **On nobody, now: anything else.**

- **SR-A37 (three corrections to this section's carried-obligation list, all
  verified at adjudication time before filing; the corrected text stays visible
  and the errors are not erased, per LD-A11(ii)'s convention. Nothing downstream
  depends on any of them.)**
  (i) **SR-A25(v) and SR-A32(iv)'s "claim-ledger / FINDINGS / open-problems
  cross-references are owed" is WRONG, twice, and is corrected here. Those
  cross-references were never owed and must never be filed.** The wiki owner
  declined the job with cause and I have verified the cause rather than accepted
  it. `wiki/walt-math-open-questions.md` rules, in its own owned scope: walt has
  **"no entries in claim-ledger and should acquire none. The ledger records claim
  tiers; walt is below every tier, so its correct entry count is zero"**, with
  promotion — independent re-verification through Lean — being *"what creates a
  ledger entry, never the other way round"*; and a walt question in
  `open-problems` would *"blur the tier boundary in exactly the direction the
  project forbids: an exploratory question would acquire, by adjacency, the
  standing of a corpus-proved boundary."* **`FINDINGS` is not named in that ruling
  and I checked it separately**: its own `owns:` line scopes it to the overall
  assessment of the two immutable specification packages, cited per Home's
  v0.7/rec convention — a corpus/exchange-tier page, on which the same adjacency
  argument bites identically. **Precedent confirms all three**: `walt` appears
  **zero** times in `claim-ledger.md`, `FINDINGS.md` and `open-problems.md`, so
  neither the whole FT chapter nor inbox 016 nor anything earlier in this file ever
  acquired a row. The correct count is zero and it is zero.
  (ii) **The shape of my error, named because it is the one this record exists to
  catch.** CLAUDE.md instructs that when a result's tier changes the owning page
  *and* `claim-ledger`, `FINDINGS` and `open-problems` be updated **"as
  applicable"**. I carried the list and dropped the qualifier, never opening the
  page that determines applicability — and **no result's tier changed here in the
  first place**, since everything in this chapter is exploratory by SR-A1 and
  FT-A1. That is FT-A29(i)'s failure **exactly inverted**: FT-A29(i) asserted an
  obligation was still *open* without opening the artifact that had discharged it;
  I asserted an obligation *existed* without opening the ruling that forecloses it.
  Both are the same single fault — **asserting a status from a text that governs
  generally rather than from the object that carries the specific case** — and it is
  now the fifth instance in the FT/SR chapters and the second of mine found by
  someone else checking. The sharpest detail is that the owning page anticipates me
  by name: *"A successor who notices the absence should not 'fix' it."* I noticed
  the absence and filed it as owed, twice, without reading the sentence that says
  not to. **The rule this yields, for the next adjudicator of this file: an
  obligation to write somewhere is asserted only after reading that destination's
  own `owns:` line and its rulings — a cross-reference list in governing text
  names candidates, never obligations.**
  (iii) **The remaining two items of SR-A25(v)/SR-A32(iv) are DISCHARGED, cited to
  the artifacts and not to the report of them** (FT-A29(i)'s discipline, applied
  the right way round this time). **The freeze register**: `wiki/walt-math-freezes.md`
  now carries **freeze 51** as a full row, carries **freeze 38 v1, clause (d)
  clarified at v1.1(d)** with its detail paragraph, and its own `owns:` line reads
  *"the register of walt's determinism freezes 1–51"* — the staleness FT-A27(ii)
  first reported and SR-A25(v) re-reported is closed. **The LOG**: the S6l entry for
  the SR probe is filed on `wiki/walt-s6-era.md` and is cross-referenced from
  `Home.md`, `walt.md` and `walt-decision-sparse.md`. Both were verified by opening
  the files.
  (iv) **What is therefore owed to the wiki owner: nothing.** SR-A25(v) and
  SR-A32(iv) are superseded in full by this ruling — two items discharged, one
  item never owed. **Nothing changes in any verdict, receipt, freeze, number or
  results file**, no re-emission and no re-run is implied, and SR-A32(v)'s four
  small emitter items for the next SR run stand exactly as written.

---

## The feature-fee audition: Jason's control feature, specified (2026-08-14)

**Adjudicator:** walt-math-11. **Object:** a request, relayed 2026-08-14, for a
minimal rung-one *feature-fee audition* — a 2–5 minute test of whether
control-flavoured structural features bite against the grade-4 receipts, using a
candidate feature Jason derived at the table while reasoning through h0. This is
the experiment inbox 017's §14.3 and SR-A19 both anticipated: **measure which
structural features approximate the perfect penalties, on a carrier where the
perfect answer is already filed, before any counting problem is faced.**
**Tier:** exploratory throughout, below every tier, like everything in walt.
**Basis:** the FT and SR chapters entire, with Proposition FT-flat, Lemma
FT-arrive, Lemma FT-post, Corollary FT-conv, Corollary FT-grade4, Proposition
SR-degen, Proposition SR-taut; FT-A13(i)/(iii) and SR-A12(i)/SR-A14(v) on
centering; freeze 26, 37(d), 38 v1.1, 44 v2, 45, 50 v1.1; the filed rows of
`fusion_tax_2026-08-14.txt`. Rulings **FF-A1..FF-A9**; two propositions and one
lemma delivered with proofs; **freeze 52** fixed at FF-A6. The prefix `FF-` and
every name below were grep-checked unused at adjudication time.

**The audition is GRANTED and re-shaped. It is not premature — it is the right
experiment at the right moment, and it is the cheapest one the branch has left at
grade 4.** But two of its four requested elements do not survive contact with the
mathematics, and both failures are informative rather than fatal:

1. **One of the two requested features captures exactly zero, by theorem, at
   every state and for every fee.** F2 as requested — *"boss outstanding-trump
   ownership (opponent vs partner)"* — is a function of the world alone. An
   action-blind fee cancels identically against the clairvoyant term and leaves
   the tax untouched (**Proposition FF-blind**, the penalty-side twin of
   Proposition FT-flat). **Auditioning it as a live candidate would burn the run
   to rediscover a theorem.** It is retained — because it is exactly what an
   audition needs and has no other way to get: a **null control** with an exact
   pre-declared prediction of zero, which is the only contentful check the harness
   can have. Repurposed, not discarded.
2. **The requested centering is unsound as written.** The spec gives the fee as
   `λ = θ(φ − E_{ν_I}[φ])` with a single centre per state. F1 depends on `b`, and
   Theorem 12.1's hypothesis is `Σ_ω μ_I(ω) λ_I(ω,b) = 0` **for every `b`
   separately**. A per-state centre does not satisfy it, and a fee that is not
   centred per action **is not a valid upper witness at all** — the resulting
   number would be neither an upper nor a lower bound on anything. This is
   FT-A13(i)'s *"the centering law is indexed by `b`"* one rung down, and it is the
   single most likely way this build could have produced a plausible wrong number.
   Corrected at FF-A2 and frozen at freeze 52(c).

And one finding that governs how the result may be read, whichever way it comes
out:

3. **The audition's two outcomes are not logically symmetric, and the
   pre-declaration must say so.** Optimising `θ` per state spends one free
   parameter per information state — 1,332 of them at h0, 216 at h2 — so the
   measured capture is an **oracle** quantity that upper-bounds what any shared,
   small parameterisation could achieve (**Proposition FF-oracle**). Therefore a
   **low** capture **refutes the feature conclusively**, and a **high** capture
   **establishes nothing about a usable fee family** and merely licenses the next
   experiment. F7 says both outcomes are results; it does not say they are results
   of equal strength, and here they are not.

---

### Proposition FF-blind (an action-blind fee captures exactly zero) — delivered here

Fix a frontier information state `I` with `p_I > 0`. Let `ψ_I(ω)` be any
world-feature not depending on the action, let
`c_I = Σ_ω μ_I(ω) ψ_I(ω) / p_I` be its `ν_I`-mean, and let
`λ_I(ω,b) = θ(ψ_I(ω) − c_I)` for any `θ`. Then `λ_I` is centred for every `b`,
and the penalised local value is unchanged:

  `Σ_ω μ_I(ω) max_b [q_I(ω,b) − λ_I(ω,b)] = Σ_ω μ_I(ω) m_I(ω)`,

so the residual tax `δ_I^λ` equals `δ_I` exactly, for every `θ`. **The captured
fraction is identically zero.**

*Proof.* Centring: `Σ_ω μ_I λ_I(·,b) = θ(Σ_ω μ_I ψ_I − p_I c_I) = 0` for each `b`
by the definition of `c_I`. Because `λ_I(ω,b)` does not depend on `b` it passes
through the inner maximum: `max_b [q_I(ω,b) − λ_I(ω)] = m_I(ω) − λ_I(ω)`.
Summing against `μ_I` and using centring once more kills the `λ` term. Subtracting
the `b`-free glued value `max_b Σ_ω μ_I q_I(·,b)` gives `δ_I^λ = δ_I`. ∎

**What it says, and why it is the twin of FT-flat.** Proposition FT-flat proved
that an action-blind *upper feature* returns a bound no better than `U_a^C`.
Proposition FF-blind proves the dual half: an action-blind *fee* removes no fusion
value at all. Together with Proposition T1-blind on the primal side, the branch now
has the same lesson proved three times in three formalisms — **a witness, a bound
or a fee must be conditioned on the decision it is trying to price.** The scope is
equally precise: this constrains `φ` as a function of `b` only. It says nothing
against a `b`-dependent but crude feature, and nothing against a feature whose
`b`-dependence is weak.

### Lemma FF-min (the fee objective is convex piecewise-linear, bounded below, and exactly minimisable) — delivered here

For a feature `φ_I(ω,b)`, write `Φ_I(ω,b) = φ_I(ω,b) − c_I(b)` with
`c_I(b) = Σ_ω μ_I(ω) φ_I(ω,b) / p_I`, and

  `G_I(θ) = Σ_ω μ_I(ω) max_{b ∈ A(I)} [ q_I(ω,b) − θ Φ_I(ω,b) ]`.

Then: **(a)** `G_I` is convex and piecewise linear in `θ`, with breakpoints among
the finitely many rationals `θ_{ω,b,b'} = (q_I(ω,b) − q_I(ω,b')) /
(Φ_I(ω,b) − Φ_I(ω,b'))` taken over positive-mass `ω` and pairs `b ≠ b'` with
`Φ_I(ω,b) ≠ Φ_I(ω,b')`; **(b)** `G_I(θ) ≥ max_b Σ_ω μ_I(ω) q_I(ω,b)` for every
`θ`, so `G_I` is bounded below and the residual `δ_I^θ := G_I(θ) − max_b Σ_ω μ_I q_I(·,b)`
is nonnegative; **(c)** the infimum of `G_I` is attained, at a breakpoint if any
exists and everywhere otherwise; and **(d)** `min_θ δ_I^θ ≤ δ_I`, since `θ = 0`
is feasible and `δ_I^0 = δ_I`.

*Proof.* (a) For each `ω` the inner expression is a maximum of `|A(I)|` affine
functions of `θ`, hence convex piecewise linear with breakpoints exactly at the
displayed crossings; a nonnegative-weighted sum of convex piecewise-linear
functions is convex piecewise linear and its breakpoints are contained in the
union. (b) For any fixed `b₀`, `max_b [q − θΦ] ≥ q(ω,b₀) − θΦ(ω,b₀)` pointwise;
summing against `μ_I` and using `Σ_ω μ_I Φ_I(·,b₀) = 0` gives
`G_I(θ) ≥ Σ_ω μ_I q_I(·,b₀)`; take the maximum over `b₀`. (c) A convex
piecewise-linear function with finitely many pieces that is bounded below attains
its infimum; if it has at least one breakpoint the minimum is attained at one, and
if it has none it is affine and bounded below, hence constant. (d) At `θ = 0`,
`G_I(0) = Σ_ω μ_I m_I` and the residual is `δ_I`. ∎

**Why it is stated.** It is what licenses **exact** minimisation with no grid and
no float (P-A19): enumerate the breakpoints, evaluate, take the least. It also
supplies two of the audition's receipts — (b) gives `δ_I^{θ*} ≥ 0` and (d) gives
`δ_I^{θ*} ≤ δ_I`, and **both are contentful**, because the swept minimum and the
filed `δ_I` come from different computations (unlike Proposition SR-taut's five,
which compare a checker against itself).

### Proposition FF-oracle (per-state fees bound shared fees; the audition refutes conclusively and confirms only weakly) — delivered here

For a feature `φ` let `R_free = Σ_I min_θ δ_I^θ` be the residual under a fee
optimised **independently at every state**, and let
`R_shared = min_θ Σ_I δ_I^θ` be the residual under one **shared** `θ`. Then

  `R_free ≤ R_shared`,  hence  `capture_free ≥ capture_shared`,

where `capture = (Δ^(1) − R)/Δ^(1)`. The same holds against any fee family whose
parameter is constrained to depend on less than the full identity of `I`.

*Proof.* For every `θ`, `R_free = Σ_I min_{θ'} δ_I^{θ'} ≤ Σ_I δ_I^θ`; minimise the
right side over `θ`. Any constrained family is a subset of the per-state-free
family, so its optimum is no better. ∎

**The reading this forces, pre-declared.** `capture_free` is an **oracle-θ**
number: it spends one free rational per information state — 1,332 at h0's
positive support and 216 at h2's — which is a lookup table, not a feature basis.
Inbox 017's §14.3 asks for *"a small action-conditioned feature family"*, and this
audition measures no such thing. Consequently:

- **`capture_free` small ⟹ the feature is refuted, and strongly**, because no
  shared or coarser parameterisation can beat it. This is the outcome the audition
  is *for*, and it is the cheap half of the question.
- **`capture_free` large ⟹ nothing follows about a usable fee family.** It licenses
  exactly one thing: the next experiment, which is the shared-`θ` fit and then a
  multi-feature fit.

**Binding: the column is named `oracle-θ capture` in every artifact and every
sentence, never "capture" unqualified, and never "the feature's capture."**

---

- **FF-A1 (typing, tier, and what this section is).** The audition is **GRANTED
  as the FF family**, re-shaped at FF-A2..FF-A5 and frozen at FF-A6. Everything is
  exploratory, cited by nothing above this tier, quotable as a result only by brief
  amendment adding it to a verifier receipt. DS-A1 binds: **witness**, **receipt**,
  **necessary outer profile**, never the forbidden word. Both outcomes of every
  gate are results (F7), with FF-oracle's asymmetry attached. A receipt failure is
  stop-and-report, never a patch (NO-RESCUE). **The feature is Jason's and its
  provenance is table reasoning about a real hand; that is a perfectly good source
  of a hypothesis and no kind of evidence for it** — the audition exists precisely
  to price it, and a null result costs the hypothesis nothing but its candidacy.
- **FF-A2 (the requested centering is UNSOUND as written and is corrected; this is
  the defect most likely to have produced a plausible wrong number).** The request
  specifies `λ = θ(φ − E_{ν_I}[φ])`, one centre per state. **Theorem 12.1's
  hypothesis (FT-A13(i), CONFIRMED) is `Σ_ω μ_I(ω) λ_I(ω,b) = 0` for every `b`
  separately.** With `φ` depending on `b` — which F1 does, and must, by Proposition
  FF-blind — a single per-state centre leaves `Σ_ω μ_I λ_I(·,b) ≠ 0` at every `b`
  whose feature-mean differs from the pooled mean, the penalty theorem's hypothesis
  fails, and **the resulting `G_I(θ)` bounds nothing in either direction.** The
  correct object is the per-action centre
  `c_I(b) = Σ_ω μ_I(ω) φ_I(ω,b) / p_I` and the fee
  `λ_I(ω,b) = θ(φ_I(ω,b) − c_I(b))`, which is centred for every `b` and every `θ`
  by construction. **Frozen at 52(c) and receipted at (FF-R2)** — by construction is
  not a receipt (PG-A8), and this is exactly the hypothesis whose silent failure
  the FT chapter was built to catch.
- **FF-A3 (the requested F2 is provably vacuous and is REPURPOSED as the null
  control — the audition's only contentful theorem-backed check).** *"Boss
  outstanding-trump ownership (opponent vs partner)"* is a predicate on `ω` and the
  record; the record is fixed at `I`; so it is action-blind and Proposition
  FF-blind gives capture **exactly zero** at every state, for every `θ`. Three
  clauses. (i) **As a live candidate it is dead before the run** and auditioning it
  as one would spend the run rediscovering a theorem. (ii) **As a null control it
  is worth more than a live candidate**, because it is the one quantity in the
  build whose exact value is known **by proof** rather than by a filed number —
  the (SR-R9) role, which SR-A31(iii) identified as the only check in that build
  that could not be self-consistently wrong. A nonzero measured capture for F0
  falsifies the centring, the sweep, the accumulation or the arithmetic, and is
  stop-and-report. (iii) **It also pins the harness's sign and scale**: F0's
  measured `θ*` is unconstrained (every `θ` is optimal), so freeze 52(e)'s tie rule
  must return `θ* = 0` there, which is itself a check on the tie rule.
- **FF-A4 (the feature list, FROZEN EXACTLY — three members, one of them the null
  control; a feature is a function and an ambiguous one is unusable).** All are
  evaluated at a frontier state `I` in world `ω` for a legal action `b`, from data
  the walk already carries. "Outstanding trump" means: the highest-ranking trump
  under the declaration among tiles **not yet played in the record and not in the
  focal seat's hand at `I`** — i.e. held by a field seat in `ω`. Write `h(ω)` for
  the seat holding it. If no trump is outstanding at `I`, **every feature below is
  0 for every `b`** at that state, which by Proposition FF-blind makes its capture
  zero there; the count of such states is emitted.
  **(F0) NULL CONTROL — `boss_owner`.** `φ(ω,b) = 1` if `h(ω)` is an opponent of
  the focal seat, `0` if a partner. Action-blind. **Pre-declared exact prediction:
  oracle-θ capture `= 0` at every state and every unit.**
  **(F1) JASON'S FEATURE — `boss_can_follow_b`.** `φ(ω,b) = 1` if `h(ω)` holds at
  least one tile of `b`'s suit under the declaration, else `0`. `b`'s suit is the
  declaration-relative suit of the tile `b`; where the focal seat is **leading** at
  `I` this is the context `b` establishes, and where it is **following** it is the
  suit `b` belongs to, which still varies across legal `b` when the focal can trump
  or throw off. **The leading/following split is emitted per unit and the capture
  is reported separately on each part**, because the feature's motivating reading —
  *the boss-trump holder can follow the context I am about to establish* — is the
  leading one, and h2's 330 states are all leading while h0's are mixed.
  **(F2) CONTROL SIBLING — `b_is_beatable`.** `φ(ω,b) = 1` if in world `ω` some
  opponent holds a tile that would beat `b` in the trick as it stands at `I` after
  `b` is played, else `0`. Computed from the existing rule algebra
  (`legal_plays`, `Trick::winner`) and never by a re-implementation of it.
  Action-conditioned, hidden, and the most directly control-flavoured of the three.
  **Declared extension, NOT commissioned now:** the graded form of F1 (the *number*
  of `b`-suit tiles `h(ω)` holds) and any joint two-feature fee. The joint problem
  is convex in `θ⃗` but is no longer a one-dimensional sweep, and it re-enters with
  its own ruling if and only if F1 or F2 survives (FF-A8(c)).
- **FF-A5 (the carrier: one factual correction, and the arms).** **h0 has ONE
  freeze-50 unit, not two** — verified at adjudication time: `fusion_tax_2026-08-14.txt`
  carries exactly one h0 unit, competitor `a = 00`, against `a⋆ = 53`, because h0
  is the untied coordinate with a single binding pair. The request's "both
  freeze-50 units" for h0 is corrected here. h2 has two (`a = 53`, `a = 54`). **h6,
  h9 and h12 are OUT OF SCOPE** — h9 for cost as requested, h6 and h12 because 53,570
  and 69,512 frontier states buy nothing an audition needs that 330 and 16,136 do
  not, and because SR-A25(iii)'s selection fence binds equally hard at three
  coordinates as at five.
- **FF-A6 (FREEZE 52 — the feature-fee audition carrier).**
  **(a) The carrier, enumerated with no generating rule** (FT-A23: a freeze is a
  constant, not a rule): **arm 1** — h0, pip 3, hand `[00 21 32 53]`, unit
  `a = 00`. **Arm 2, attempted after arm 1 completes, with a declared stop** — h2,
  pip 5, hand `[21 33 53 54]`, units `a = 53` then `a = 54`. Arm 1 is both the
  cheapest unit in the carrier and the hand the feature came from. Coordinate
  identity is asserted in freeze 45's form at every unit, kernel rebuilt in-run and
  asserted equal.
  **(b) The measured object.** Per frontier state `I` **with `δ_I > 0`** and per
  feature: `c_I(b)` for every `b`, the breakpoint count, `θ*`, `δ_I^{θ*}`, and the
  captured amount `δ_I − δ_I^{θ*}`. States with `δ_I = 0` are counted and skipped —
  there is nothing to capture there and Lemma FF-min(d) gives `δ_I^{θ*} = 0`.
  **(c) The fee, per FF-A2:** `λ_I(ω,b) = θ(φ_I(ω,b) − c_I(b))` with
  `c_I(b) = Σ_ω μ_I(ω) φ_I(ω,b) / p_I`. **Per-action centring is mandatory and
  receipted.**
  **(d) Exact minimisation only** (Lemma FF-min): enumerate the breakpoints,
  evaluate `G_I` at each, take the least. **No grid, no search, no float anywhere**
  (P-A19; clippy `-D float_arithmetic` and the no-float grep bind). Every
  denominator that must divide is **asserted** to divide, and the arithmetic is
  **checked** — an overflow in the breakpoint or evaluation arithmetic is
  stop-and-report, not a wrap.
  **(e) The tie rule, declared before the run and never chosen by result:** `θ*` is
  the **smallest** breakpoint attaining the minimum, in ascending rational order;
  if a state has no breakpoints, `θ* = 0`.
  **(f) Reporting.** Per unit and per feature, in the **count convention**
  (Corollary SR-conv; taxes are differences and are exactly half their differential
  value): `Σ_I δ_I^{θ*}`, the **oracle-θ capture** `(Δ^(1) − Σ_I δ_I^{θ*})/Δ^(1)` as
  an exact rational, and the three-way census `{all, some, none}` of states by
  whether `δ_I^{θ*}` is `0`, in `(0, δ_I)`, or `= δ_I`. Split additionally by
  leading/following at `I`, and report the count of states with no outstanding
  trump.
  **(g) Belief and field are NOT re-declared** — freeze 26 and 37(d), uniform over
  the full enumerated fiber, no decimation inside anything ((C2)). **No library
  entry at any coordinate** (freeze 45). The freeze-set digest travels on every
  record.
  **(h) Budgets:** freeze 44(b) v2 unchanged; no new constant. On exhaustion, no
  partial fold — no partial capture, no partial residual.
- **FF-A7 (the receipts, at audition scale — six, with the non-receipts named).**
  (i) **(FF-R1) the null-control receipt — BLOCKING, before any F1 or F2 number
  exists.** Assert F0's `δ_I^{θ*} = δ_I` **exactly at every state** and its
  per-unit oracle-θ capture `= 0` exactly, and `θ* = 0` by freeze 52(e). **The only
  check here whose answer is known by proof** (Proposition FF-blind) rather than by
  a filed rational — the (SR-R9) role. It tests the per-action centring, the
  breakpoint enumeration, the sweep, the exact accumulation and the tie rule at
  once. Failure is stop-and-report.
  (ii) **(FF-R2) the centring receipt.** At every state and **every** `b` — not
  only the argmax `b` — assert `Σ_ω μ_I(ω) λ_I(ω,b) = 0` exactly at the reported
  `θ*`. **Contentful**: it is Theorem 12.1's hypothesis, and it is precisely what
  fails under the per-state centring of FF-A2. By construction is not a receipt.
  (iii) **(FF-R3) the bound receipt.** Assert `0 ≤ δ_I^{θ*} ≤ δ_I` at every state,
  with `δ_I` taken from the frontier pass. **Contentful** by Lemma FF-min(b),(d) —
  the swept minimum and `δ_I` are different computations, so a sweep bug shows up
  here; this is *not* a Proposition SR-taut identity.
  (iv) **(FF-R4) the direct-evaluation receipt.** At the reported `θ*`, recompute
  `G_I(θ*)` by direct summation over worlds — `Σ_ω μ_I(ω) max_b[q − θ*Φ]`, with no
  incremental state — and assert equality with the swept value. **Contentful**: two
  independently written paths sharing only the inputs. The (SR-R6) role at audition
  scale.
  (v) **(FF-R5) the rung-one invariance receipt.** Assert `|I_1|`, the arrival
  count, the zero/positive census, `Δ^(1)`, `U^(1)` and `U^(0)` against a frozen
  table transcribed from `fusion_tax_2026-08-14.txt` with its provenance line,
  never re-parsed (SEP-A14(ii), FT-A28(i)). **This build regenerates a rung-one
  frontier, so FT-A28(iv)'s (FT-R7c) applies**: emit the per-unit frontier digest;
  **assert** it against the transcribed value at h2, where `SR_FIRST` already carries
  one; **emit and file** it at h0, where none exists yet, with (FT-R7a)'s corrected
  scope line.
  (vi) **(FF-R6) determinism.** An in-run second pass with fresh maps, accumulators
  and budgets; every printed row and summary asserted identical.
  (vii) **NAMED AS NON-RECEIPTS, per Proposition SR-taut and printed as arithmetic
  remarks:** `δ_I^{θ*} ≥ 0` *as re-derived from the probe's own `G` and glued
  value*; `capture ≤ 1`; `Σ_I(δ_I − δ_I^{θ*}) = Δ^(1) − Σ_I δ_I^{θ*}`; and
  convexity of `G_I`. They cannot fail. **(FF-R3) is a receipt only because it
  compares against the frontier pass's `δ_I`**, and that distinction must be printed
  beside it.
- **FF-A8 (all outcomes pre-declared, before any number exists; F7 binds, with
  Proposition FF-oracle's asymmetry attached to every one).**
  (a) **F0's capture is anything other than exactly 0** → stop-and-report. The
  harness is wrong; no F1 or F2 number is reported, emitted or discussed. This gate
  is blocking and comes first.
  (b) **F1's oracle-θ capture is small at both arms** → **the feature is refuted,
  and conclusively**, because by Proposition FF-oracle no shared or coarser
  parameterisation can do better than the per-state oracle. **This is a RESULT and
  the most likely one**; it costs Jason's hypothesis its candidacy and nothing else,
  and it is filed as a result under F7, not as a null. The same for F2.
  (c) **F1's or F2's oracle-θ capture is large** → **nothing is established about a
  usable fee family** (Proposition FF-oracle), and no claim is made beyond the
  measurement. It licenses exactly one thing: a follow-on ruling commissioning the
  shared-`θ` fit and then the joint two-feature fit, which are different experiments
  with different objects. **No artifact of this build may say a feature "works."**
  (d) **The leading/following split differs sharply** → reported as a scope fact
  about F1's definition (FF-A4), not as a fact about 42, and read against h2 being
  wholly leading and h0 mixed.
  (e) **A budget stop, or arm 2 not reached** → declared stop, no partial fold, no
  partial capture, printed as a stop and never as a finding (R-A18, freeze 44(b)).
  (f) **SETTLED A PRIORI, reportable only as such.** By Proposition SR-degen no
  closure verdict exists at grade 4 for either coordinate, and **a shave measured
  here changes no verdict** — h0's binding pair is untied and already closes at rung
  two, h2's is tied and terminates at equality. **The entire value of this build is
  as a screening measurement for trick 1**, where FT-A21's three obligations are the
  binding constraint. Any sentence in the artifact implying a grade-4 verdict moved
  is void on its face.
- **FF-A9 (fences, runtime, and what none of this can ever claim).**
  (i) **Every standing fence travels verbatim**: the R-A2/P-A1 fence; the N4-A8
  real-deal fence (the hands come from rob's corpus, **the belief does not**);
  SR-A25(iii)'s selection fence, which binds *harder* here than at rung two —
  **three units at two coordinates chosen by negative binding margin are a carrier,
  not a sample**, and a capture fraction is exactly the kind of number that reads
  like a rate; P-A21 — **no quantity measured at grade 4 is quoted for trick 1 or
  for the opening**, which for a screening experiment aimed at trick 1 is the fence
  most at risk and must be printed in the header; and SR-A25(vii)'s
  implementation-versus-corpus risk, undiminished.
  (ii) **Not claimed, printed in place**: nothing about points or marks; nothing
  about bidding; nothing about how real opponents play; no cost or tractability
  claim off any traversal observable (SEP-A19(b)); and **nothing whatever about
  whether Jason's reading of h0 at the table is correct** — the feature is being
  priced as a fee, and a fee's capture is not a statement about the reasoning that
  suggested it.
  (iii) **Runtime.** The dominant cost is the rung-one frontier pass, which
  `fusion_tax.rs` already performs: PATH A charged **539,583,224** walk-steps at h0
  and **1,297,073,736** at h2 per unit, so the audition's three units total
  ≈ 3.13 × 10⁹ walk-steps against the SR run's four much heavier units at ≈ 10.5
  minutes. Per state the sweep is `O(|X_I|·|A(I)|²)` breakpoints — at most two per
  world since `|A(I)| ≤ 3` — sorted and swept, which is negligible beside the
  traversal. **The 2–5 minute target is plausible and is a target, never a receipt**
  (SEP-A19(b)): no outcome of this build turns on it, and if arm 2 does not fit,
  FF-A8(e) is the declared answer rather than a re-scope mid-run.
  (iv) **If this reuses the SR emitter, SR-A32(v)'s four items apply**; if it is a
  fresh example, only the `yes`/`no` case rule and the frontier digest of (FF-R5)
  carry, and the companion machinery is **not** wanted — this build's emission is
  small enough to commit entire, and it should be.

**What the build owes this section.** The FF probe of FF-A6 over freeze 52's
arm 1 (h0, one unit) with arm 2 (h2, two units) attempted under a declared stop;
the three frozen features of FF-A4 with F0 auditioned **first and blocking**; the
per-action centring of FF-A2; exact breakpoint minimisation per Lemma FF-min with
no float and checked arithmetic; the six receipts of FF-A7 with the non-receipts
printed as arithmetic remarks; the **oracle-θ capture** column named as such
everywhere; and all six outcomes of FF-A8 pre-printed with Proposition FF-oracle's
asymmetry beside them, before any number exists. Everything else here is proof and
needs no code.

---

### Closing note: the FF audition returned (2026-08-14, after the run)

**Object:** `walt-factory/examples/feature_fee.rs` and
`walt-factory/results/feature_fee_2026-08-14.txt`, committed; 47.5 s wall-clock,
all three units, no declared stop. **(FF-R1) HELD blocking at every unit** before
any F1 or F2 number existed, and (FF-R2)–(FF-R6) HELD at every unit. **The
headline is that one feature bit hard and one did not — and that a clause I wrote
in FF-A4 silently voided six of the twelve measurements, including the single
most informative one.** Three questions are ruled at FF-A11..FF-A14 and the
extensions at FF-A15.

**Re-derived at adjudication time, independently of the run**, from the artifact's
own emitted rows and censuses:

- h0: `Σδ_I` over the 574 leading states `3217979/29937600` plus over the 758
  following states `22237/7185024` equals `Δ^(1) = 19863799/179625600` **exactly**.
- F2 at h0: `(Δ^(1) − 1963673387/69155856000)/Δ^(1) = 5683889228/7647562615`
  **exactly as reported**, and its ppm floor is 743,228 as printed.
- F2's leading-part capture `2841944614/3716765745` **exactly as reported**, and
  the captured amount computed two ways — whole-unit and leading-part — agrees at
  `1420972307/17288964000`.
- F1 at h0: `88457474377/24775917854710`, i.e. **3,570.3 ppm**, with implied
  residual `8229153460111/74681738208000`.
- Censuses close: `32 + 486 + 814 = 1332`, `574 + 758 = 1332`.
- **The decisive structural fact, established by grep over every emitted row:** at
  h0, **all 2,274 `lead = no` rows carry `boss = none` and no `lead = yes` row
  does** — the no-outstanding-trump set and the following set coincide *exactly* —
  and **every h2 row at every feature carries `boss = none` with `breakpoints = 0`**.

### Proposition FF-degen (zero breakpoints is exactly vacuity, and it is emitted) — delivered here

At a frontier state `I` with `|A(I)| ≥ 2`, the breakpoint set of Lemma FF-min is
empty **if and only if** `Φ_I(ω,b)` does not depend on `b` at any positive-mass
`ω`; and in that case the fee is action-blind on `I`, so by Proposition FF-blind

  `δ_I^{θ} = δ_I` for every `θ`,  and the state's captured amount is exactly 0.

*Proof.* A breakpoint exists iff some positive-mass `ω` and some `b ≠ b'` have
`Φ_I(ω,b) ≠ Φ_I(ω,b')`; its absence is exactly the stated constancy in `b`. Then
`λ_I(ω,b) = θΦ_I(ω,b) = θρ_I(ω)` for a `b`-free `ρ_I`, which is centred because
`Σ_ω μ_I ρ_I = Σ_ω μ_I Φ_I(·,b) = 0` for any `b`. Proposition FF-blind applies. ∎

**Why it matters operationally, and it is the reason this run is auditable at
all.** Freeze 52(b) requires the breakpoint count per state, so **the artifact
separates "the feature was priced and failed" from "the feature had no content
here" mechanically, without re-running anything.** A capture of zero at a state
with thousands of breakpoints is a measurement; a capture of zero at a state with
none is a tautology. Every zero in this run can therefore be typed, and below they
are.

---

- **FF-A10 (the artifact against FF-A1..A9 and freeze 52: NO DEVIATION FOUND).**
  Every contract item is present: freeze 52's enumerated carrier and arms; the
  per-action centring of FF-A2 receipted at **every** `b`; exact breakpoint
  minimisation with no float; freeze 52(e)'s tie rule (visible in F0's `θ* = 0`
  everywhere); the count convention; the leading/following split; the
  no-outstanding-trump census; the **`oracle-θ capture`** column named as such
  throughout; the ppm figures marked PRESENTATION ONLY and entering no proof; the
  SR-taut set printed as arithmetic remarks and excluded from every HELD count;
  and all six FF-A8 outcomes pre-printed. **(FF-R1) ran first and blocking**, and
  its result is doubly satisfying: F0's zero capture **and** its zero breakpoint
  count are Proposition FF-degen and Proposition FF-blind agreeing on the same
  state, which is the harness confirming two independent predictions at once. The
  builder also emitted h0's (FT-R7c) frontier digest for the first time and filed
  it, as (FF-R5) required — that obligation is discharged at h0.
- **FF-A11 (MY DEFECT: FF-A4's blanket clause voided six of the twelve
  measurements, and one of them was the one that mattered. The corrected text
  stays visible and the error is not erased, per LD-A11(ii).)** Five clauses.
  (i) **What I wrote.** FF-A4 defines the outstanding trump and then says: *"If no
  trump is outstanding at `I`, **every feature below is 0 for every `b`** at that
  state."* I attached that fallback to **all three** features. Only F0 and F1
  reference the boss-trump holder `h(ω)`. **F2 (`b_is_beatable`) never mentions
  `h(ω)` and is perfectly well defined with no trump outstanding** — indeed that is
  when it is *most* interesting, because control then turns entirely on suit rank.
  (ii) **What it cost, exactly.** At h2, **all 330 frontier states have no
  outstanding trump** — the focal seat leads a top trump at the root, the lone
  outstanding trump `52` is forced out in trick 1 in every world, and nothing
  trump-bearing survives to the frontier. So my clause set `φ ≡ 0` for **F2 as well
  as F0 and F1** at every h2 state, giving `Φ ≡ 0`, zero breakpoints and, by
  Proposition FF-degen, capture zero **as a tautology**. At h0 the same clause fired
  at the 758 following states, which coincide exactly with the no-outstanding-trump
  set. **Six of the twelve (feature, unit) cells are therefore vacuous by
  construction: F1 and F2 at both h2 units, and F2's following part at h0.**
  (iii) **The one that mattered.** F2 at h2 is the measurement this build should
  have produced and did not. h2's frontier is **wholly leading**, so F2's
  definition there is unambiguous and its sweep would have been clean — and h2 is
  the coordinate where the focal seat holds the top trumps and control questions
  are purely about suit rank, which is precisely the regime F2 was written for.
  **That measurement is UNMEASURED, not zero**, and no sentence anywhere may report
  h2's F2 as evidence about the feature.
  (iv) **The shape of the error, named.** I wrote a scoping clause for a *family*
  after defining a term that only *part* of the family uses, and never checked the
  clause against each member's own definition. It is the same fault the chapter has
  now caught five times in different clothes — **a statement whose scope was
  asserted rather than checked against the object it governs** — and this is the
  fourth instance that is mine. It is also the exact fault Proposition FF-degen
  exists to expose, which is the only reason it was catchable from the committed
  artifact rather than requiring a re-run to discover.
  (v) **What survives.** The clause is *sound* for F0 and F1, which genuinely have
  no value when `h(ω)` does not exist; it is *unsound as applied to F2*. **Freeze
  52 is amended to v1.1 at FF-A15(i)**: the fallback binds only features that
  reference `h(ω)`, and every feature carries its own domain clause.
- **FF-A12 (a SECOND defect of mine, smaller, and it partly excuses the first).**
  F2's frozen definition — *"some opponent holds a tile that would beat `b` in the
  trick as it stands at `I` after `b` is played"* — is **ambiguous at following
  states**, where a tile already on the table may already beat `b`. It does not say
  whether the current winner counts. At h0's 758 following states F2 was therefore
  **doubly unusable**: suppressed by FF-A11's clause and undefined by its own text.
  The clause accidentally prevented an ill-defined number from being printed, which
  is luck and not design. **The amendment at FF-A15(i) fixes the definition
  explicitly**: `φ = 1` iff some opponent **who has not yet played at `I`** holds a
  tile that, if played, would win the trick over `b` and over every tile already on
  the table. At leading states this is identical to the frozen reading, so **h0's
  leading measurement is unaffected and stands.**
- **FF-A13 (F1, Jason's feature: REFUTED at its only in-scope carrier part — and
  the refutation is conclusive there and empty everywhere else).** Four clauses.
  (i) **The measurement.** At h0's 574 leading states — the only place in this
  carrier where a boss trump exists at the frontier — F1 was genuinely swept, 23,016
  breakpoints across the support, and its oracle-θ capture is
  `88457474377/24775917854710`, **3,570 parts per million: about one third of one
  percent** of the first-layer tax.
  (ii) **The verdict, and why it is strong.** By Proposition FF-oracle this is an
  **upper bound** on what any shared or coarser parameterisation of F1 could
  achieve, because it spends one free rational per information state. **A family
  that cannot break 0.36% with 574 free parameters cannot break it with one.**
  FF-A8(b) fires: **F1 is refuted as a fee at this carrier part, conclusively**,
  and it is filed as a RESULT under F7, not as a null.
  (iii) **Elsewhere F1 is not refuted — it is inapplicable, which is a different
  and more interesting finding.** At h0's following states and at all of h2 there
  **is no outstanding trump at the frontier**, so `h(ω)` does not exist and F1 has
  no content whatever. The h2 zeros are Proposition FF-degen tautologies. **The
  scope discovery is worth more than the refutation**: a feature keyed to the boss
  outstanding trump is empty exactly where the focal seat has already drawn the
  trumps, and at grade-4 endgames reached by leading a top trump that is *every*
  state. A boss-trump feature has a shrinking domain precisely as the hand
  simplifies, which is the opposite of where a cheap witness is wanted.
  (iv) **FF-A9(ii) is restated here because this is where it bites.** **None of
  this is a verdict on Jason's reading of h0 at the table.** The feature was priced
  *as a centred fee against the first-layer tax*, which is one specific job; a
  refutation in that job says the quantity does not linearise the Jensen gap, and
  says nothing about whether the boss-trump-can-follow relation is the right thing
  to be thinking about at that hand. The hypothesis loses its candidacy as a fee
  and nothing else. It is also worth saying plainly that the hypothesis was sharp
  enough to be killed in 47.5 seconds, which is the property one actually wants
  from a table intuition.
- **FF-A14 (F2, the control sibling: it BIT, and the honest number is the
  leading-part one; FF-A8(c) fires with its asymmetry attached).** Five clauses.
  (i) **The measurement, correctly scoped.** F2 was genuinely swept only at h0's
  574 leading states, 26,954 breakpoints. **Oracle-θ capture on that part:
  `2841944614/3716765745` ≈ 76.46%.** That is the number, and it is the first
  non-trivial capture the branch has ever measured.
  (ii) **The whole-unit figure `5683889228/7647562615` ≈ 74.32% is NOT void, but it
  is NOT F2's capture either — it is a lower bound.** It averages the genuine
  leading measurement against 758 states forced to zero by FF-A11's clause. Because
  removing the clause can only lower each state's residual (`θ = 0` remains
  feasible), a properly scoped F2 would capture **at least** 74.32% over the whole
  unit. **Binding: the 74.32% figure may be quoted only as a lower bound and never
  as "F2's capture"; the quotable measured number is the leading-part 76.46%.**
  (iii) **The three-way census must be re-read, and re-reading it strengthens the
  result.** The printed `32 ALL / 486 SOME / 814 NONE` counts the 758 vacuous
  states among the NONEs. Over the 574 genuinely swept states the census is
  **32 ALL / 486 SOME / 56 NONE** — computed here as `814 − 758 = 56`, with
  `32 + 486 + 56 = 574` — so **a single centred fee on `b_is_beatable` removes some
  of the local tax at 518 of 574 swept states, about 90%, and all of it at 32.**
  (iv) **What FF-A8(c) permits, and it is exactly one thing.** By Proposition
  FF-oracle a large oracle-θ capture **establishes nothing about a usable fee
  family** — 574 free rationals is a lookup table. It licenses the follow-on, which
  is commissioned at FF-A15(ii). **No artifact of this build may say F2 "works", and
  none does.** The claim that is now supported is narrow and real: *at h0's leading
  frontier states, the first-layer Jensen gap is substantially aligned with a single
  binary control predicate — enough that per-state optimal pricing of it removes
  about three quarters of the tax.* Whether one shared `θ` recovers any of that is
  unknown and is the next question.
  (v) **Every standing fence travels and one binds hardest.** Two coordinates
  selected by negative binding margin are a carrier, not a sample (SR-A25(iii)), and
  a capture fraction reads exactly like a rate. **P-A21: no quantity here is quoted
  for trick 1 or for the opening**, which is the fence most at risk given that
  screening for trick 1 is the entire motive. And by Proposition SR-degen **no
  grade-4 verdict moved and none could have** — h0's binding pair is untied and
  already closes at rung two.
- **FF-A15 (the amendments and what is now commissioned).** Four clauses.
  (i) **FREEZE 52 v1.1 — the domain clause, corrected.** Freeze 52(a)–(h) stand
  except that FF-A4's no-outstanding-trump fallback is **scoped to features that
  reference `h(ω)` — F0 and F1 only** — and **every feature carries its own domain
  clause**. F2's definition is amended per FF-A12: `φ(ω,b) = 1` iff some opponent
  **yet to play at `I`** holds a tile that would win the trick over `b` and over
  every tile already on the table. **At leading states this is identical to the
  frozen reading**, so nothing measured at h0's leading part changes and no
  re-derivation is owed there.
  (ii) **COMMISSIONED — the corrected re-run, and it is small.** Re-run freeze 52
  v1.1 over the same three units, reporting **F2 only** (F0 and F1 are settled:
  F0 by theorem, F1 by FF-A13). What it must produce: F2 at **all 330 h2 states per
  unit**, and at h0's 758 following states under the amended definition. Receipts as
  before, with **(FF-R1) still blocking** — F0 must be re-run purely as the null
  control even though its verdict is known, because the control is what makes the
  new numbers trustworthy. Pre-declared, per FF-A8: a large h2 capture is a second
  in-scope datum and still licenses nothing about a shared family; **a small or zero
  h2 capture with a large breakpoint count refutes F2 at h2 conclusively** and is
  the more interesting outcome, since h2 is the pure-suit-rank regime; **zero
  breakpoints again at h2 would mean F2 is constant across the fiber there**, which
  is a third possibility and must be reported as such rather than as either.
  (iii) **COMMISSIONED — the shared-`θ` fit for F2, on the h0 leading part.** This
  is what FF-A8(c) licenses and no more. The object is
  `min_θ Σ_I δ_I^θ` over the 574 leading states with **one** `θ`, exactly minimised:
  the sum of convex piecewise-linear functions is convex piecewise-linear, so
  Lemma FF-min applies verbatim to the pooled objective with the union of the
  per-state breakpoints. Report the shared capture, the ratio to the oracle-θ
  capture — **that ratio is the number the whole programme actually wants** — and
  the per-state distribution of `θ*` from this run, which is already emitted and
  costs nothing to summarise. **It may run in the same build as (ii).**
  (iv) **NOT commissioned, and each for its own reason.** The **joint two-feature
  fit** — one live feature is not a joint problem, and pairing a live feature with a
  refuted one buys at most F1's 0.36%. The **graded form of F1** — F1's refutation
  is of the binary predicate and does not formally transfer to the graded one, so
  this remains *eligible* rather than refuted, but a 0.36% binary is a weak prior and
  it earns a run only on request. **Any trick-1 object** — FT-A21 stands BLOCKED in
  full and nothing here touches its three obligations.
- **FF-A16 (carried obligations, and what none of this claims).** Four clauses.
  (i) **Nothing is promoted.** Everything above is exploratory, below every tier,
  cited by nothing above this one, quotable only by brief amendment adding it to a
  verifier receipt. Per SR-A37 this chapter acquires **no** claim-ledger, FINDINGS
  or open-problems row, and none is owed.
  (ii) **Not claimed:** that F2 is a usable fee (FF-oracle forbids it on this
  evidence); that F1 is refuted anywhere but h0's leading part; that h2 says
  anything about either feature; that any grade-4 verdict moved; anything about
  points, marks, bidding or real opponents; and **anything about Jason's reading of
  h0**, per FF-A9(ii) and FF-A13(iv).
  (iii) **Owed to the wiki owner:** freeze 52 and its v1.1 amendment are new and are
  not in the register, and the FF chapter needs its era-page and LOG entries. Per
  SR-A37(i) that is the **whole** list — no tier-page cross-references exist or are
  owed.
  (iv) **Owed on the next FF run:** freeze 52 v1.1's domain clauses; the corrected
  F2 definition; and, since it regenerates rung-one frontiers, the (FT-R7c) digests
  — **asserted** at h0 now that this run filed one, and asserted at h2 against
  `SR_FIRST`. **Owed to nobody, now: anything else.** The artifact discharges its
  contract; the two defects in it are mine, not the builder's, and both were
  detectable from the committed file only because freeze 52(b) required the
  breakpoint counts that Proposition FF-degen reads.

**What the build owes this section.** The freeze 52 v1.1 re-run of FF-A15(ii)
with the shared-`θ` fit of FF-A15(iii) in the same pass, F0 blocking, F2 only,
and the three pre-declared h2 readings printed before any number exists.
Nothing else.

- **FF-A17 (arm 2's typing: the builder's EMPTY TEST reading is RATIFIED, its
  mechanism is verified here independently, and FF-A8(b)'s literal gate did NOT
  fire).** Four clauses.
  (i) **The mechanism, checked at adjudication time rather than accepted.** h2's
  pool is `[10 11 20 22 30 31 32 41 43 52 63 66]` and the declaration is pip 5;
  **exactly one pool tile bears a 5 — the `52`** — while `53` and `54` sit in the
  focal hand. Both h2 units lead a trump at the root, the field must follow trump
  when able, so the sole field trump burns on trick 1 in every world and nothing is
  outstanding at the frontier. The census the artifact emits — **216 of 216 swept
  states, 330 of 330 states** — is the arithmetic consequence. Verified.
  (ii) **The typing is RATIFIED: arm 2 is an EMPTY TEST and contributes no
  evidence in either direction.** It is not a second refutation of F1, not
  coordinate-dependence of feature quality, and not a measurement of F2. This is
  FF-A11(ii)–(iii) confirmed by a mechanism rather than by a breakpoint count, and
  the two agree.
  (iii) **FF-A8(b)'s pre-declared gate was NOT met, and my FF-A13 verdict is
  narrower than the one I pre-declared.** FF-A8(b) reads *"F1's oracle-θ capture is
  small at **both arms** → the feature is refuted, and conclusively."* With arm 2
  empty, that condition is unsatisfied and **outcome (b) did not fire as written.**
  What FF-A13 rules instead is a *scoped* refutation — conclusive on the 574 states
  where F1 has a domain, empty elsewhere — which I introduced at adjudication after
  seeing the numbers. **That is legitimate only because it claims strictly less than
  the pre-declared outcome and rests on Proposition FF-oracle rather than on the
  gate**, and it must never be cited as though the pre-declared gate had fired.
  Pre-declaration exists so a reading cannot be shopped for after the fact; the
  honest record is that the declared gate was unsatisfiable at this carrier and a
  weaker verdict was substituted in the open.
  (iv) **The lesson for the next pre-declaration.** A gate quantified over "both
  arms" silently assumes both arms are non-empty. **A pre-declared outcome must
  either quantify over *non-empty* arms or carry an explicit empty-arm branch.**
  FF-A8 had no such branch; freeze 52 v1.2 at FF-A20 supplies the screen that makes
  the case visible before a unit is spent.
- **FF-A18 (MY THIRD DEFECT in this chapter: FF-A13(i) attaches a whole-unit
  rational to a leading-state sentence. Corrected in place; the verdict is
  unchanged.)** Three clauses.
  (i) **The error.** FF-A13(i) reads *"At h0's 574 leading states … its oracle-θ
  capture is `88457474377/24775917854710`, 3,570 parts per million."* That rational
  is the **whole-unit** capture, over all 1,332 swept states. The **leading-part**
  capture — the correctly scoped figure, and the one the verdict actually rests on
  — is `88457474377/24082518161460`, **3,673 ppm (0.3673%)**, which is what the
  builder reports and which I confirm by exact computation. Both rationals share
  the numerator because all of F1's capture lies on the leading part; only the
  denominator differs, `Δ^(1)` versus `Σ_I δ_I` over the leading states.
  (ii) **The verdict is unchanged and is if anything unchanged for the better.**
  0.3673% against 0.3570% is the same refutation; FF-A13(ii)'s argument from
  Proposition FF-oracle is untouched. **FF-A13(i)'s sentence is corrected to read
  3,673 ppm on the leading part, with 3,570 ppm named as the whole-unit figure.**
  (iii) **The shape, and I record it because it is embarrassing in a useful way.**
  I ruled at FF-A14(ii) that the artifact must not quote a whole-unit figure as if
  it were the leading-part measurement — and then did exactly that to F1 one clause
  earlier. **Scope mislabelling is not a mistake other people make; it is the
  standing hazard of a file that reports the same quantity over nested state sets**,
  and the only defence is the one that caught it here: every rational is printed
  with the state set it is a ratio over. **Binding on this file: no capture figure
  appears anywhere without its denominator's state set named in the same
  sentence.**
- **FF-A19 (the h0 leading/following split: the correspondence is RATIFIED as
  verified; the rules-level mechanism is the builder's reading and is typed as
  hostage to T1-A12).** Three clauses.
  (i) **What is verified and is mine.** Across all 2,274 `lead = no` rows of the h0
  block, **every one carries `boss = none`, and no `lead = yes` row does** —
  checked by grep at adjudication time. So at h0 the following set and the
  no-outstanding-trump set coincide **exactly**, 758 and 758, and the leading set
  and the boss-survives set coincide, 574 and 574. **The split is feature
  AVAILABILITY, not feature quality**, and the builder's framing is adopted.
  (ii) **Half the mechanism is immediate and I confirm it.** h0's pool is
  `[10 11 20 22 40 41 42 43 44 51 52 62]` under pip 3, and **exactly one pool tile
  bears a 3 — the `43`**. A played `43` is a trump and beats the non-trump root
  `00`, so **`43` played ⟹ focal loses trick 1 ⟹ following, and boss gone.** That
  direction is rules-light and holds.
  (iii) **The converse is the builder's reading and I do not ratify it as a rules
  fact.** That `43` unplayed ⟹ focal's `00` wins depends on the engine's suit
  convention — specifically on which dominoes can follow a led `00`. The observed
  equivalence is consistent with a convention in which suit 0 has `00` as its only
  member at this coordinate, and inconsistent with one in which every domino
  bearing a blank can follow. **I decline to adjudicate which, because the artifact
  cannot tell me and because that is precisely the T1-A12 / LD-A10(ii)
  implementation-versus-corpus risk that is still owed.** It is worth saying that
  this is the first place in three chapters where a *reported mechanism* — not a
  value — turns on the unchecked convention, which is a small argument for paying
  that debt sooner. **Nothing in FF-A13 or FF-A14 depends on the converse**: both
  verdicts rest on the emitted `boss` field, not on why it is set.
- **FF-A20 (the builder's arm-screening rule: ADOPTED, sharpened into a
  pre-traversal test, and frozen as 52 v1.2).** Three clauses.
  (i) **The rule, adopted.** *Screen a candidate unit for whether the feature has a
  non-empty domain at the frontier before spending the unit on it.* Of the original
  five freeze-50 coordinates, only h0 exercises a boss-trump feature at all, and
  discovering that by running was a waste that the emitted census made visible only
  afterwards.
  (ii) **Sharpened: for a boss-keyed feature the screen is computable from the
  coordinate and the root action alone, with no traversal.** Both carrier
  coordinates have **exactly one trump outside the focal hand**. The difference is
  the root action: at h2 it *is* a trump, so the field must follow and the lone
  boss burns on trick 1 in every world — **empty domain, provably, before any
  walk**; at h0 it is not, so the boss survives on some paths. The general
  sufficient condition: **if the number of trumps outside the focal hand is 1 and
  the root action is a trump, every boss-keyed feature has empty domain at every
  frontier state.** More trumps outstanding weakens it to a partial screen, which is
  what the emitted census then measures.
  (iii) **FREEZE 52 v1.2 — the domain screen.** Freeze 52 v1.1 (FF-A15(i)) stands,
  plus: **every FF unit emits its feature-domain census *before* the sweep, and a
  unit whose domain is empty at every swept state is declared an EMPTY TEST, is not
  swept, and contributes to no capture figure and no outcome gate.** An empty test
  is reported as a unit that did not run, never as a zero. This is FF-A8's missing
  empty-arm branch (FF-A17(iv)) made operational, and it costs one pass over the
  frontier records.
- **FF-A21 (the (FT-R7c) digest assertion at h2 is a CROSS-PROGRAM check and is
  stronger than what I specified).** (FF-R5) required the h2 digests asserted
  against `SR_FIRST`'s transcribed values and h0's emitted-and-filed. Both were
  done. **What I did not specify and the build delivered:** the h2 assertion
  compares a digest produced by `feature_fee.rs` against one produced by
  `second_rung.rs` and transcribed from a run of `fusion_tax.rs`'s frontier —
  **byte-identical canonical serialisation of the `(record, δ_I)` vector across
  three independently written programs.** FT-A28(iv) conceived (FT-R7c) as a
  cross-*process* determinism receipt; at h2 it is now a cross-*program* one, which
  reaches the record keying, the arrival weights and the tax vector simultaneously
  and would fail on any divergence in the frontier construction between the three
  probes. **It is a receipt, not an audit note** — asserted in-run against a
  transcribed constant with its carrier named — and it is the strongest evidence
  the rung-one frontier has yet received. h0's digest, having no filed value, was
  emitted and filed under the same division; **it becomes assertable on the next FF
  run** (FF-A16(iv)).
- **FF-A22 (the audition's cleanest statement, and the one sentence worth
  carrying out of this chapter).** On **one and the same set of 574 states**, with
  one and the same sweep, arithmetic, centring and tie rule, the two
  action-conditioned candidates return oracle-θ captures of **0.3673%** and
  **76.46%** — a ratio of **about 208×**. Because the comparison is within a
  single unit and a single pass, it is free of every between-coordinate confound
  the selection fence guards against, and it is the only comparison in this chapter
  that is. **What it supports, stated at its exact strength:** *at h0's leading
  frontier states, the first-layer Jensen gap is substantially aligned with whether
  the focal seat's action can be beaten, and essentially not at all with whether
  the boss-trump holder can follow it.* **What it does not support**, and the
  fences are unchanged: nothing about a usable fee family (Proposition FF-oracle —
  574 free rationals), nothing about any other coordinate (SR-A25(iii): a carrier,
  not a sample), nothing about trick 1 or the opening (P-A21), and no grade-4
  verdict (Proposition SR-degen). And FF-A9(ii) travels with it: the feature that
  lost was priced as a fee against one specific object, and losing that job is not
  a verdict on the reasoning that proposed it.

**What the build owes this section, restated after the supplement.** Unchanged
from FF-A16's closing note, plus freeze 52 v1.2's domain screen emitted before the
sweep at every unit, and the empty-arm branch made explicit in the next
pre-declaration. Arm 2 needs no re-run *as an F1 test* — it never was one — but
the corrected-F2 re-run of FF-A15(ii) still wants h2, where F2's domain is **not**
empty and only my FF-A4 clause made it look so.

- **FF-A23 (FF-A15(ii)'s "all 330 h2 states": the conflict is MINE, freeze 52(b)
  GOVERNS the sweep, and the two counts are separated by purpose. The builder
  reported it rather than picking, which is the ambiguity protocol executed
  correctly and is commended in place.)** Six clauses.
  (i) **The defect is mine and the builder's diagnosis of it is exactly right.**
  FF-A15(i) says *"Freeze 52(a)–(h) stand except that FF-A4's
  no-outstanding-trump fallback is scoped…"*, so freeze 52(b)'s skip rule stands
  by my own text; and FF-A15(ii) then says *"F2 at all 330 h2 states per unit, and
  at h0's 758 following states"*. **The sentence is not internally parallel**: 758
  is a count of *swept* states and 330 is a count of *all* frontier states. The
  "330" was inherited verbatim from FF-A11(ii)'s *"at h2, all 330 frontier states
  have no outstanding trump"*, where it describes **the extent of what my clause
  voided** — a scope-of-correction phrase, not a sweeping instruction. I carried a
  number across from one job to another without re-checking which set it counted.
  (ii) **RULING: freeze 52(b) governs. Sweep the 216 positive-`δ_I` states per h2
  unit.** FF-A15(ii)'s "all 330" is **corrected in place** to read *"F2 at every
  swept h2 state — 216 per unit — and at h0's 758 following states"*, per
  LD-A11(ii); the erroneous phrasing stays visible above.
  (iii) **Nothing numeric turns on this, and it is worth saying why so the ruling
  is not mistaken for a close call.** `Δ^(1) = Σ_I δ_I` over *all* states equals
  the sum over swept states, because the rest are zero; and
  `Σ_I δ_I^{θ*} = Σ_swept δ_I^{θ*}` for the same reason, since Lemma FF-min(b)+(d)
  force `0 ≤ δ_I^{θ*} ≤ δ_I = 0` at every skipped state. **The oracle-θ capture,
  the shared-θ capture and their ratio are identical under either reading.** What
  differs is only which rows are emitted and whether a census reads 216 or 330 —
  which is precisely why a wrong choice would have been a reportable deviation and
  not a private one, exactly as the builder said.
  (iv) **But the two counts each have a job, and separating them is better than
  either option offered.** The **sweep** is over swept states, because a state with
  no tax has nothing to price. The **domain census** of freeze 52 v1.2 is a
  *screen* — its purpose is to characterise whether the feature has content at the
  coordinate at all — and a screen that looked only at swept states would be
  answering a different question. **FREEZE 52 v1.3, clarifying v1.2:** the
  feature-domain census is emitted **over every frontier state**, and the swept
  census over the swept support, with **both printed and both labelled with the set
  they count**. The prior artifact already did this — *"States with NO OUTSTANDING
  TRUMP: 216 of the swept support, 330 of all 330 states"* — so this ratifies
  existing practice rather than imposing new work.
  (v) **FF-A18's binding rule applies to every figure this re-run emits**, and it
  is the rule this very conflict illustrates: **no count and no capture figure
  appears without the state set it ranges over named in the same sentence.** In
  particular the shared-θ result of FF-A15(iii) is a ratio over **h0's 574 leading
  states** and must say so wherever it appears, as must its ratio to the oracle-θ
  capture over the same 574.
  (vi) **This is the second specification defect of mine the FF chapter has
  produced and the fifth across FT/SR/FF** (FT-A23 and FT-A28(i) were
  walt-math-10's, FF-A11 and this are mine, with FF-A12 and FF-A18 alongside). All
  are one fault in different clothes — **a term, a clause or a count applied
  outside the scope in which it was defined** — and every one has been caught by
  someone else reading the text against the object. The protocol is doing the work
  it exists to do.
- **FF-A24 (the re-run emits to a NEW results file: RATIFIED, and it is required
  rather than merely preferred).** Three clauses.
  (i) **The builder's proposal is correct and is adopted.** `feature_fee_2026-08-14.txt`
  is the adjudicated artifact; FF-A10..FF-A22 cite it by content and by line, and
  FF-A11's typing of the six voided cells is a ruling *about that file as it
  stands*. **Overwriting it would silently invalidate every citation in this
  chapter and erase the evidence for the defect it records.** LD-A11(ii)'s
  convention — the corrected text stays visible and the error is not erased —
  applies to artifacts as much as to rulings, and CLAUDE.md's rule that results are
  regenerated and never hand-edited means the old file cannot be annotated in
  place either. **The new file is required.**
  (ii) **One thing the new file owes, because two results files for one experiment
  is a reader trap.** Its header names `feature_fee_2026-08-14.txt` and states
  exactly what supersedes and what stands: **superseded** — F2 at both h2 units and
  F2's h0 following part, the six cells FF-A11 typed as vacuous; **standing,
  unaltered and not re-derived** — F0 everywhere (settled by Proposition FF-blind),
  F1 everywhere (settled at FF-A13, refuted on h0's leading part and inapplicable
  elsewhere), and **F2's h0 leading measurement**, which FF-A15(i) proves is
  untouched by the amended definition.
  (iii) **The builder's verification of that last point is confirmed here.** At a
  leading state nothing is yet on the table and all three field seats are still to
  play, so *"some opponent yet to play at `I` holding a tile that would win over
  `b` and over every tile already on the table"* reduces to *"some opponent holds a
  tile that would beat `b`"* — the frozen reading exactly. **h0's leading F2 number
  `2841944614/3716765745` over 574 states therefore stands without re-derivation**,
  and the re-run should reproduce it, which makes it a free cross-check the new
  file should assert rather than merely print.
  (iv) **The rest of the builder's plan is confirmed as ruled** and needs no answer
  from me: F0 re-run first and blocking; F1 not reported; the pooled shared-θ
  objective with the union of per-state breakpoints and freeze 52(e)'s tie rule —
  **Lemma FF-min applies verbatim to the pooled function**, being a finite sum of
  convex piecewise-linear functions bounded below by the sum of the glued values,
  and minimising `Σ_I G_I(θ)` is equivalent to minimising `Σ_I δ_I^θ` because the
  subtracted term is `θ`-free; Proposition FF-degen's per-state breakpoint counts
  mandatory; the three pre-declared h2 readings printed before any number; and the
  (FT-R7c) digests asserted at all three units, which **discharges FF-A16(iv)'s h0
  item** now that this build has filed one.

---

### Closing note: the freeze 52 v1.1 re-run returned (2026-08-14, after the run)

**Object:** `walt-factory/examples/feature_fee_v11.rs` and
`walt-factory/results/feature_fee_v11_2026-08-14.txt`, 3,722 lines, uncommitted at
adjudication time; CI green. Three units, six receipts HELD at each, (FF-R1)
blocking and first. **The chapter's question is answered: a single
action-conditioned feature with ONE shared parameter captures 99.87% of what 574
free parameters capture.** Four questions are ruled at FF-A26..FF-A29.

**Re-derived at adjudication time from the reported aggregates**, all exact:

- F2 at h0, whole unit, amended: `16474225753499603/21924124275433380` =
  **75.1420%**, ppm floor 751,420 as printed, and **≥ the clause-suppressed
  74.3229%** — confirming FF-A14(ii)'s lower-bound reasoning on the numbers.
- Shared-θ capture `61431886/80449475` = **76.3608%** against oracle-θ
  `2841944614/3716765745` = **76.4628%**, both over h0's 574 leading states;
  **ratio `7095382833/7104861535` = 99.86659%**, ppm floor 998,665 as printed. The
  shared fee gives up `18957404/18583828725` — **0.102% of the leading-part tax** —
  relative to per-state tuning.
- **A figure the build did not report, available by arithmetic from ones it did:**
  since the leading-part capture is asserted unchanged, the amended F2's capture on
  h0's **758 following states** is `25654843999781/87655285360500` = **29.2679%**.
  This is new and it matters (FF-A27(ii)).

### Proposition FF-corr (what a fee bites on, exactly) — delivered here

At a frontier state `I`, write `Φ_I(ω,b) = φ_I(ω,b) − c_I(b)` and let
`argmax_b q_I(ω,b)` be the clairvoyant optimal set at `ω`. Then the oracle-θ
capture at `I` is **zero** if and only if `θ = 0` minimises `G_I`, which holds if
and only if

  `Σ_ω μ_I(ω) min_{b ∈ argmax} (−Φ_I(ω,b)) ≤ 0 ≤ Σ_ω μ_I(ω) max_{b ∈ argmax} (−Φ_I(ω,b))`.

In particular, when the clairvoyant argmax is unique at every positive-mass `ω`,
capture is zero **iff**

  `Σ_ω μ_I(ω) Φ_I(ω, b*(ω)) = 0`,

that is, iff **the centred feature, evaluated along the clairvoyant policy, has
zero mean under `μ_I`**.

*Proof.* `G_I` is convex (Lemma FF-min), so `θ = 0` is a minimiser iff
`G_I'(0^-) ≤ 0 ≤ G_I'(0^+)`. For a maximum of affine functions the one-sided
derivatives at a point are the extreme slopes over the active set, and at `θ = 0`
the active set at `ω` is `argmax_b q_I(ω,b)` with the `b`-th slope `−Φ_I(ω,b)`;
summing against `μ_I` gives the displayed condition. Under uniqueness both bounds
collapse to `−Σ_ω μ_I Φ_I(ω,b*(ω))`. ∎

**Why this is the sentence the whole feature-fee idea reduces to.** By
construction `Φ_I(·,b)` has zero `μ_I`-mean for each **fixed** `b`. So a fee can
only bite through the correlation that **selecting `b` by the world** induces. A
feature therefore prices a Jensen gap exactly to the extent that its centred value
*along the clairvoyant choice* is off-centre — and a feature can be rich,
action-conditioned, and full of breakpoints while being exactly orthogonal to the
choice, which is what h2 turns out to be. It also completes the trio: Proposition
FF-blind says an action-blind fee is worthless, Proposition FF-degen says a
fibre-constant fee is worthless, and **Proposition FF-corr says what is left must
correlate with the clairvoyant action or it is worthless too.**

---

- **FF-A25 (the artifact against FF-A23/FF-A24: NO DEVIATION, and FF-A23(iii)'s
  prediction is confirmed empirically).** The sweep is 216 per h2 unit per freeze
  52(b); the domain census is emitted over every frontier state alongside the swept
  census, both labelled, per freeze 52 v1.3; FF-A18's rule is applied to every
  figure, with the shared/oracle line even carrying *"it is a ratio of two captures
  over the SAME 574 states and is meaningless against any other state set"*, which
  is the rule stated better than I stated it; and FF-A24(ii)'s supersession header
  is present with both lists. **FF-A23(iii) predicted that nothing numeric turns on
  the sweep ruling; the re-run reports the oracle-θ capture, the shared-θ capture
  and their ratio byte-identical to the pre-ruling run.** A prediction that could
  have been wrong was not. The h0 leading-part figure was **asserted** rather than
  printed and held, so the amendment's collapse at leading states is now checked on
  the numbers by a second program and not only argued (FF-A24(iii)).
- **FF-A26 (F2 at h2: REFUTED CONCLUSIVELY — pre-declared reading (h2-2) fired,
  and the reason the exact zero is trustworthy is not the null control).** Five
  clauses.
  (i) **The result.** Oracle-θ capture **exactly `0/1`** over the 216 swept states
  at **both** h2 units, with **3,126 breakpoints**. FF-A15(ii) pre-declared three
  readings; this is the second — *"a small or zero h2 capture with a large
  breakpoint count refutes F2 at h2 conclusively"* — and I called it *"the more
  interesting outcome"* before any number existed. It fired. **F2 is refuted at
  h2**, and by Proposition FF-oracle the refutation extends to every shared or
  coarser parameterisation there.
  (ii) **Priced and failed, not vacuous — and the distinction is entirely earned by
  the breakpoint count.** This is Proposition FF-degen doing the exact job it was
  written for: 3,126 breakpoints means the fee genuinely varied and the minimum was
  found at `θ = 0` anyway. Contrast the first run's h2, where the same zero came
  with **zero** breakpoints and was a tautology. **The same number means opposite
  things in the two files**, and only the diagnostic separates them.
  (iii) **What the exact zero means, by Proposition FF-corr.** At every one of 216
  states, twice, `Σ_ω μ_I Φ_I(ω,b*(ω))` sits at zero (or the subgradient straddles
  it). **An exact rational identity holding at 432 independent states is not a
  coincidence and there is a structural cause I cannot identify from this
  artifact.** Two candidates: the feature is genuinely orthogonal to the
  clairvoyant choice at h2's frontier, or the clairvoyant argmax sets are widely
  non-unique there and the subgradient straddles zero for a tie-driven reason.
  **OPTIONAL and cheap, not mandated:** emitting `Σ_ω μ_I Φ_I(ω,b*(ω))` and the
  per-world argmax cardinality per state would separate the two in one pass. It is
  not owed — the refutation stands either way, and proportionality is part of the
  discipline.
  (iv) **THE RECEIPT-DESIGN POINT, and it is the one worth carrying forward.**
  (FF-R1), the null control, expects `θ* = 0` — **and so does the failure mode "the
  sweep is broken and always returns `θ* = 0`".** A null control whose expected
  answer coincides with a plausible bug's answer **cannot** validate an exact zero
  elsewhere. What licenses reading h2's zero as a measurement is that **F2 at h0
  returns `θ* ≠ 0` at all 574 leading states, across 27 distinct values, none zero,
  and a nonzero shared `θ* = −56/45`** — the sweep demonstrably moves. **The
  general rule, filed alongside Proposition SR-taut and Proposition FF-degen: a
  null control is complete only when paired with a case whose correct answer is
  known to be non-null.** Here the pairing exists by luck of the carrier rather
  than by design, and the next FF pre-declaration should require it.
  (v) **FF-A22's 208× comparison is unaffected**, being within h0's 574 states.
- **FF-A27 (F2 at h0 under the amendment: FF-A14(ii)'s lower bound CONFIRMED, and
  FF-A19's "availability not quality" is SUPERSEDED for the amended feature).**
  Three clauses.
  (i) **The whole-unit figure is 75.1420% over h0's 1,332 swept states**, against
  the clause-suppressed 74.3229%. FF-A14(ii) ruled that the earlier number was *"not
  void but a lower bound"*, on the reasoning that removing the clause can only lower
  each state's residual. **75.1420% ≥ 74.3229%, so the reasoning is confirmed on the
  numbers**, and the earlier figure may continue to be cited as what it was ruled to
  be. The census moves from `32/486/814` to **`70 ALL / 1182 SOME / 80 NONE`**: with
  the domain repaired, **1,252 of 1,332 swept states now capture something.**
  (ii) **The new finding, derived here: the following-part capture is 29.2679% over
  h0's 758 following states.** The build reports the whole-unit and the leading-part
  figures and asserts the latter unchanged, so the following part follows by exact
  arithmetic. **This supersedes FF-A19's summary sentence for the amended feature.**
  FF-A19 ruled *"the split is feature AVAILABILITY, not feature quality"* — true of
  **F2 as frozen**, whose domain was empty at following states, and it remains true
  of **F1**, which is boss-keyed and genuinely has no domain there. It is **no longer
  true of amended F2**: with availability repaired, a large quality difference
  remains — **76.4628% leading against 29.2679% following, the same feature, the same
  unit, the same sweep.** FF-A19 is corrected in place accordingly; the erroneous
  generalisation stays visible per LD-A11(ii).
  (iii) **Typed at its exact strength, with the fences.** *At h0, the amended
  control predicate prices roughly three quarters of the leading-part first-layer
  tax and under a third of the following-part tax.* Whether that gap is about
  leading versus following as such, or about what else differs between those two
  state sets at this one coordinate, is **not determined** — and the same P-A21 and
  selection fences bind (SR-A25(iii)); nothing here is quoted for trick 1.
- **FF-A28 (the shared-θ fit: FF-A8(c)'s licensed follow-on has RETURNED, and it
  is the strongest result in the FF chapter).** Five clauses.
  (i) **The result.** Over h0's 574 leading states, one pooled `θ* = −56/45` gives
  shared capture **76.3608%** against the per-state oracle's **76.4628%** —
  **99.8666% of the oracle survives collapsing 574 free rationals to one.** The
  shortfall is `18957404/18583828725`, **0.102% of the leading-part tax.**
  (ii) **Why this is the number the chapter existed to produce.** Proposition
  FF-oracle established that a large oracle-θ capture establishes *nothing* about a
  usable family, because per-state tuning is a lookup table; FF-A8(c) therefore
  licensed exactly one thing, the shared fit. **That fit has now returned and it says
  the feature does not need per-state tuning at all.** Inbox 017's §14.3 asked for
  *"a small action-conditioned feature family whose conditionally centered penalties
  remove enough of each competitor's fusion value"*. **At this carrier part, one
  feature and one rational do it.** That is the first time in the branch that a
  *small* fee family has been shown to carry a first-layer tax, and it is what makes
  the penalty route more than a theorem.
  (iii) **The corroborating structure, and it is consistent.** The per-state `θ*`
  over those 574 states takes only **27 distinct values**, none zero, over
  `[−21/4, −32/33]`, with 12 states matching the shared value exactly. A feature
  whose optimal price is nearly constant across states is precisely one whose shared
  fit should lose almost nothing, and it does.
  (iv) **What it still does NOT establish, and the fences are undiminished.** One
  coordinate; one part of it (574 of that unit's 1,332 swept states, and h0 is one of
  nine n = 4 coordinates); selected by negative binding margin, so **a carrier and
  not a sample** (SR-A25(iii)); **grade 4, so no verdict moved and none could**
  (Proposition SR-degen); and **P-A21 — nothing here is quoted for trick 1 or for the
  opening**, which is where the whole programme is aimed and where FT-A21's three
  obligations remain untouched. **A 99.87% shared/oracle ratio at one coordinate part
  is a licence to test the fee at a second coordinate, not a licence to believe in
  it.**
  (v) **And it is a rung-one result.** By Corollary FT-grade4 the h0 fusion gap is
  `Δ^(1) + Δ^(2)`, and every number in this chapter prices `Δ^(1)` only. A fee that
  captured 100% of `Δ^(1)` would still leave `Δ^(2) = 387281/5132160` untouched.
- **FF-A29 (the builder's discriminating-variable hypothesis: ACCEPTED AS A DESIGN
  INPUT, refused as a finding, and correctly self-fenced).** Three clauses.
  (i) **The observation.** F2 captures ~76% at h0's leading frontier and exactly 0%
  at h2's, **both with real breakpoint content**, and the two leading frontiers
  differ in that h0's still has an outstanding trump behind it while h2's does not.
  The builder raised it flagged as hypothesis, not finding, with P-A21 attached.
  **That self-typing is correct and is adopted.**
  (ii) **Why it cannot be a finding here.** `n = 2`, both chosen by negative binding
  margin, and the two coordinates differ in **many** ways besides trump survival —
  declaration, hand, whether the root action is itself a trump, the leading/following
  mix, the frontier size (16,136 against 330), and the fibre structure. **A single
  contrast cannot isolate one variable from a dozen co-varying ones**, and the
  selection fence bites hardest exactly where a mechanism is being proposed.
  (iii) **What it IS good for, and it is genuinely useful.** It supplies a
  **discriminating variable for arm selection** if the family is extended: a third
  coordinate should be chosen to **vary trump survival at the frontier while holding
  as much else fixed as possible** — ideally the same declaration and a root action of
  the same trump/non-trump type — so that the contrast is informative rather than
  merely another observation. That is the correct use of an untested hypothesis:
  **it designs the next experiment; it does not conclude the last one.** Combined
  with freeze 52 v1.2's domain screen, arm selection now has two inputs and no
  longer needs to discover either by running.
- **FF-A30 (status, what is commissioned, and what is owed).** Five clauses.
  (i) **Settled and closed at this carrier:** F0 by theorem (Proposition FF-blind,
  twice confirmed); **F1 refuted** on h0's leading part and inapplicable elsewhere
  (FF-A13, FF-A18's scope correction); **F2 refuted at h2** (FF-A26); **F2 live at
  h0** with 75.14% whole-unit, 76.46% leading, 29.27% following, and a shared-θ fit
  retaining 99.87% (FF-A27, FF-A28). **FF-A16(iv)'s h0 digest item is DISCHARGED**
  by the three asserted (FT-R7c) digests.
  (ii) **NOTHING further is commissioned now.** FF-A15(ii)–(iii)'s two commissions
  are both discharged by this run. The obvious next experiment — F2 at a third
  coordinate chosen per FF-A29(iii) — is **not commissioned here**, because it is a
  new carrier and wants its own freeze, its own pre-declared readings including the
  empty-arm branch of FF-A17(iv), and the non-null pairing of FF-A26(iv). It should
  be asked for deliberately, not inherited.
  (iii) **Optional and unmandated:** FF-A26(iii)'s two-column diagnostic, if anyone
  wants h2's exact zero explained rather than merely established.
  (iv) **Owed to the wiki owner:** freeze 52 with its v1.1, v1.2 and v1.3
  amendments, and the FF era-page and LOG entries. Per SR-A37(i) that is the whole
  list; no tier-page rows exist or are owed.
  (v) **Not claimed**, all of FF-A16(ii) unchanged, and one added: **no sentence
  anywhere may report the shared-θ result as showing that a feature fee "works" at
  trick 1, at the opening, or at any coordinate other than h0's leading part.**
  FF-A9(ii) also still travels: the feature that was refuted was priced as a fee
  against one specific object, and that is not a verdict on the reasoning that
  proposed it. Jason's hypothesis suggested one feature that died and a sibling that
  did not, and the sibling is now the best-supported object in the FF chapter.

**What the build owes this section.** Nothing. The v1.1 re-run discharges both
commissions, holds all six receipts at three units, and answers the chapter's
question. Any third coordinate is a new commission and needs a new freeze.

- **FF-A31 (the v1.1 header defects: REGENERATE — option (a) — and FF-A24's
  no-overwrite doctrine does NOT extend to this file. One binding condition and
  one thing the corrected header must not do.)** Six clauses.
  (i) **RULING: the header-corrected emission is the file FF-A25..FF-A30 refer
  to.** The coordinator's instinct is right and the doctrine is mine to apply, so
  here it is applied.
  (ii) **Why FF-A24 does not transfer, stated as a distinction rather than an
  exception.** FF-A24 forbids overwriting `feature_fee_2026-08-14.txt` because
  **that file's content is the subject of a ruling** — FF-A11 types six of its
  cells as vacuous, and the file is the evidence for a defect of mine that the
  record exists to keep visible. Overwriting it would erase the evidence for the
  error. **No ruling of mine is about a defect in the v1.1 file's content**; that
  file is *evidence for results*, not the subject of a finding, and it is
  **uncommitted**, so it is not yet an artifact of record. Regenerating it erases
  nothing. **The doctrine protects adjudicated defects, not adjudicated files.**
  (iii) **The regenerate-line defect must not be committed, and it is the more
  serious of the two.** A results file whose own regenerate line names a
  *different* probe asserts a false provenance: it claims to be a deterministic
  function of a command that produces another object. That is the FT-A28(i) family
  — naming a relation without naming its relata — and it is **sharper here, because
  it names the wrong relatum rather than none**, so a reader who follows the
  instruction gets a plausible wrong file rather than an error. CLAUDE.md's rule
  that results are regenerated and never hand-edited makes regeneration the
  sanctioned repair, not a workaround.
  (iv) **My citations survive, and I checked rather than assumed.** Grep over
  FF-A25..FF-A30 shows the v1.1 file is cited **by name and by total line count
  only** — no line-number citation reaches into its body — and the single content
  quotation is of the shared/oracle sentence, which sits below the header. Every
  number I relied on was **re-derived independently from the reported aggregates**
  before adjudication, so the ruling is not hostage to the file's byte layout at
  all. **The one figure at risk is the object line's "3,722 lines," which is
  provenance**: if the regenerated file's count differs, that figure alone is
  corrected in place and nothing else moves.
  (v) **BINDING CONDITION on the ratification: the confinement claim is checked,
  not asserted.** *"Numbers and all content below the header are unchanged"* is
  exactly the kind of by-construction claim this chapter does not accept on its
  word (PG-A8, Proposition SR-taut). Two things discharge it, and they are typed
  differently. **A receipt:** the three per-unit (FT-R7c) digests re-assert in-run
  against their transcribed values, which reaches every frontier row across the two
  processes. **An audit note, never a receipt:** an orchestrator byte-diff of
  everything below the header block. FT-A28(v)'s typing applies verbatim — it is
  not asserted in-run, not reproduced by any verify path, and does not survive into
  a future run, so it is adjudication-time evidence and **may never be printed as
  HELD or counted among "all six receipts."**
  (vi) **What the corrected citation line must NOT do, because the obvious
  over-correction is circular.** It should name **the rulings the build was
  executed under — FF-A1..FF-A24 with freeze 52 v1.1, v1.2 and v1.3** — and must
  **not** be extended to FF-A25..FF-A30, which are rulings *about* this file
  written after it ran. **A results file citing the rulings that adjudicate it
  asserts a provenance that runs backwards in time**, and it would also make the
  file's own header a moving target every time the chapter is extended. The
  governing set is fixed at the moment of the run; the adjudicating set lives here.

- **FF-A32 (RATIFICATION of the committed v1.1 emission, with two corrections in
  place and one boundary discipline. The FF chapter closes here.)** Six clauses.
  (i) **RATIFIED. FF-A25..FF-A30 refer to the emission committed at `fe10b50`**,
  and to no other object. The coordinator's resolution is correct on the facts: the
  pre-fix emission was never committed, so the header-corrected file is the only
  repository object those rulings can attach to. **Nothing in FF-A25..FF-A30 is
  disturbed** — every figure in them was re-derived at adjudication time from the
  reported aggregates, independently of the file's bytes (FF-A31(iv)).
  (ii) **The header conformance is verified here, not accepted on report.** The
  rulings line reads `FF-A1..FF-A24` with freeze 52 v1.1 and v1.3 named — **and it
  is correctly NOT extended to FF-A25..FF-A30**, which is FF-A31(vi) honoured
  exactly, so the file makes no provenance claim that runs backwards in time. The
  regenerate line names `--example feature_fee_v11` and carries an explicit warning
  that the unsuffixed name regenerates the *other* file — better than the repair I
  asked for, since it fixes the defect and inoculates against the confusion that
  produced it. The freeze-set digest propagates `52v1.3` to the three per-unit
  provenance lines per 52(g). The three (FT-R7c) frontier digests are asserted
  in-run, and the HELD count is 33.
  (iii) **FF-A31(iv)'s pre-declared correction fires, and is made.** The committed
  file is **3,723 lines**, not 3,722; the added supersession line accounts for the
  difference. **FF-A25's object line is corrected in place from "3,722 lines" to
  "3,723 lines"** and nothing else moves — exactly the single-figure, no-other-
  consequence repair FF-A31(iv) reserved. A pre-declaration that fired and cost one
  word is the cheap end of this discipline working.
  (iv) **The added ALSO SUPERSEDED line: ACCEPTED IN EFFECT, its reason CORRECTED,
  and one surviving use named.** The line strikes the earlier whole-unit figure
  `5683889228/7647562615` and forbids quoting it *even as a bound*, *"because a
  measurement of the same quantity over the same state set now exists."* **The state
  set is the same; the quantity is not.** 74.3229% is the exact capture of
  **F2-as-frozen**, whose domain clause zeroed 758 states; 75.1420% is the capture
  of **amended F2**. Those are different features and therefore different
  quantities — which is precisely *why* the first bounds the second rather than
  approximating it. The instruction the line yields is nonetheless **right in
  effect**: with the amended feature's capture measured directly, the bound has no
  remaining forward use. **No re-emission is owed** — the imprecision is in a reason,
  not in a number, and following the instruction loses nothing (FT-A28(iv)'s
  proportionality).
  **The one surviving use, and FF-A27(i) is corrected in place to name it.**
  FF-A27(i) wrote that the earlier figure *"may continue to be cited as what it was
  ruled to be"*, which is now too permissive. It retains exactly one legitimate
  use: **as the historical measurement of F2-as-frozen, and hence as the second
  term in FF-A14(ii)'s confirmation** — the comparison `75.1420% ≥ 74.3229%` needs
  both numbers, and striking one would make FF-A27(i)'s own confirmation
  unciteable. **For every other purpose the figure is superseded.**
  (v) **The boundary discipline this raises, filed because the FF chapter is the
  first place it has come up.** That line does not merely record a number: it
  **restricts how a number may be quoted**, which is a ruling-shaped statement. The
  results files in this chapter are full of such statements — fences,
  pre-declarations, NOT-CLAIMED blocks, the SR-taut arithmetic remarks — but every
  one of those was **mandated by a ruling and restated by the build**. This one was
  **originated by the build**. The rule, stated once so the line stays clean:
  **a results file may restate a reading-rule that a ruling has fixed; it may not
  originate one.** Numbers and provenance are the artifact's to assert; how a number
  may be read is this file's. The builder's instinct was sound and it routed the
  addition through the coordinator rather than burying it, which is why this is a
  discipline being written down and not a defect being reported.
  (vi) **The FF chapter is CLOSED.** Settled: F0 by theorem; F1 refuted on h0's
  leading part and inapplicable elsewhere; F2 refuted at h2, live at h0 with
  75.1420% whole-unit, 76.4628% leading, 29.2679% following, and a shared-θ fit
  retaining **99.8666%** of the per-state oracle. All commissions discharged;
  **nothing further is commissioned**, and a third coordinate is a new carrier
  wanting its own freeze, its own pre-declared readings with FF-A17(iv)'s empty-arm
  branch, and FF-A26(iv)'s non-null pairing. **Owed to the wiki owner and to nobody
  else:** freeze 52 with its v1.1/v1.2/v1.3 amendments in the register, and the FF
  era-page and LOG entries. Per SR-A37(i) that is the whole list. **Every fence of
  FF-A28(iv) travels with the headline number wherever it goes** — one coordinate,
  one part of it, a carrier and not a sample, grade 4 so no verdict moved, `Δ^(1)`
  only, and nothing quoted for trick 1.

- **FF-A33 (FF-A31 compliance: the line-count fix is already made, the v1.2
  citation omission is NOT material, the missing audit note is immaterial and a
  stronger check is performed here instead — and the enquiry surfaced a real
  imprecision in freeze 52 v1.2 that is sharpened to v1.4).** Five clauses.
  (i) **Clause (iv)'s correction was made at FF-A32(iii) and is sitting
  uncommitted; the convention is worth restating because it caused the
  duplication.** In this file, *"corrected in place"* means what FT-A29(i) and
  LD-A11(ii) established: **a later ruling states the correction and the erroneous
  text stays visible above it** — the earlier ruling is never edited. FF-A32(iii)
  therefore *is* the fix: FF-A25's object line reads 3,723 lines, not 3,722, by
  operation of that clause. Nothing further is owed and no append is pending on
  this account.
  (ii) **Clause (vi), the v1.2 omission: NOT MATERIAL. No regeneration.** Four
  reasons, each independently sufficient. **(1)** Freeze 52 v1.3 is *defined* as
  clarifying v1.2 (FF-A23(iv)), so a citation of v1.3 reaches v1.2 by construction.
  **(2)** The body's freeze block names v1.2 explicitly one line below. **(3)**
  Nothing in the run's conduct turns on the distinction: v1.2's screen was applied
  correctly — h2's F2 domain is **not** empty under the v1.1 amendment, so h2 was
  rightly swept rather than declared an EMPTY TEST, and 3,126 breakpoints is the
  proof that it had content. **(4)** Proportionality (FT-A28(iv)): a 49-second
  regeneration to add three characters to a citation line, when the omitted item is
  named on the next line and nothing depends on the difference, spends real cost on
  process hygiene alone. **Binding on the next FF emission; forcing nothing now.**
  (iii) **The enquiry surfaced a genuine imprecision in freeze 52 v1.2, and this
  is why the question was worth asking.** v1.2 (FF-A20(iii)) reads *"a **unit**
  whose domain is empty at every swept state is declared an EMPTY TEST, is not
  swept…"*. **Domain-emptiness is a property of a (unit, feature) pair, not of a
  unit**, and read literally v1.2 would have barred the **F0 null control from
  running at h2**, where F0's domain *is* empty — contradicting FF-A15(ii)'s
  mandate that F0 run first and blocking at every unit *"even though its verdict is
  known, because the control is what makes the new numbers trustworthy."* The build
  resolved this correctly and silently by running F0 everywhere. **FREEZE 52 v1.4,
  sharpening v1.2:** the screen applies **per (unit, feature) cell**, not per unit;
  and **the null control is exempt from the screen in all cases**, because its job
  is to test the harness rather than the feature, and a harness check that skips
  the states where the feature is empty is precisely a harness check that has not
  been run where it is cheapest to run. **Binding on the next FF run; nothing is
  owed now**, the v1.1 run having done the right thing already.
  (iv) **Clause (v)'s audit-note half: IMMATERIAL, and the coordinator was right to
  refuse to represent the builder's self-diff as an orchestrator diff.** Three
  clauses of reasoning and one substitute check.
  **Why its absence subtracts nothing from the evidentiary tier.** FT-A28(v) already
  ruled that an orchestrator byte-diff **is never a receipt** — not asserted in-run,
  not reproduced by any verify path, not surviving into a future run. A thing that
  was never going to be a receipt cannot, by being absent, lower a receipt count. It
  was specified as adjudication-time evidence and adjudication-time evidence is what
  it would have been.
  **What the digest receipt does and does not reach, stated honestly.** The three
  (FT-R7c) digests reach every rung-one `(record, δ_I)` pair across processes. They
  do **not** reach the FF-specific outputs — the per-state `θ*`, the residuals, the
  capture figures, the shared fit. So the digest alone does not discharge the
  confinement claim over the content that matters most here.
  **The substitute, performed at adjudication time and stronger for that content.**
  Every headline figure of FF-A25..FF-A30 was derived by me from the aggregates
  reported **before** the regeneration existed, and each is confirmed present in the
  committed file: `16474225753499603/21924124275433380` (h0 whole-unit),
  `61431886/80449475` (shared), `2841944614/3716765745` (oracle over the 574),
  `7095382833/7104861535` (the ratio), `0/1` over the 216 swept states at **both**
  h2 units, and `3126` at each. **That is a comparison against values recorded prior
  to the regeneration by an independent party, which is exactly what the byte-diff
  was for over the content anyone will cite.** It is adjudication-time evidence and
  is typed as such — **not a receipt**, not printed as HELD, not counted among the
  33.
  (v) **The standing lesson, cheap and general.** A byte-diff between two emissions
  **must be produced while both objects exist, or not at all** — it cannot be
  reconstructed after a sanctioned regeneration has overwritten its comparand, as
  this one could not. **Any future regeneration protocol that wants that audit note
  must snapshot the prior emission before regenerating.** This is not a defect in
  anyone's conduct here: FF-A31(v) specified the note without specifying when it had
  to be taken, which is my omission, and it is the same family as FT-A28(i) —
  **naming a check without naming the moment at which its comparand exists.** Filed
  so the next such ruling names both.

---

## The fee-correlation chapter: what a fee bites on, measured (2026-08-14)

**Adjudicator:** walt-math-11. **Object:** the specification of the next chapter
of the feature-fee line, requested deliberately by Jason — *"of course we have to
start the next seed immediately"* — and relayed with a three-item agenda by
walt-steward. FF-A30(ii) and FF-A32(vi) closed the FF chapter with the next
experiment named but **not** commissioned, on the ground that it *"should be asked
for deliberately, not inherited."* It has been. **Tier:** exploratory throughout,
below every tier. **Basis:** the FF chapter entire, with Proposition FF-blind,
Proposition FF-degen, Proposition FF-corr, Proposition FF-oracle, Lemma FF-min;
freeze 52 through v1.4; and beneath them the FT and SR chapters. Rulings
**FC-A1..FC-A9**; one proposition and one corollary delivered with proofs;
**freeze 53** fixed at FC-A5. The prefix `FC-` and every name below were
grep-checked unused at adjudication time.

**The scope ruling, stated first because it departs from the proposed agenda.**
The steward's item (1) — a third coordinate, chosen to vary trump survival — is
**DEFERRED, not refused**, and the chapter commissioned instead is **the
correlation diagnostic on the carrier we already have.** The reason is not cost;
it is that we are one cheap measurement away from knowing *why* a fee bites, and
a third coordinate taken before that measurement would add an observation to a set
we cannot yet interpret. **That is precisely the trap FF-A29(ii) named** — a
single contrast cannot isolate one variable from a dozen co-varying ones — and
taking a third observation under the same conditions does not escape it; it
enlarges it.

**What makes the diagnostic ripe, and it is a fact about our own mathematics.**
Proposition FF-corr already says exactly what determines capture: the centred
feature, evaluated **along the clairvoyant choice**, must have nonzero mean. That
quantity is one rational per state and is computable from data the v1.1 probe
already assembles, **with no sweep at all**. And the branch is holding an
unexplained exact zero — F2 at h2, capture `0/1` at 216 states twice over, with
3,126 breakpoints proving the fee genuinely varied. FF-A26(iii) named two
candidate causes and made the diagnostic optional. **That was the wrong call and
this chapter reverses it:** an exact rational identity holding at 432 independent
states is the most informative unexplained fact in the branch, and the instrument
that would explain it costs seconds.

**And the diagnostic is worth more than an explanation, which is why it outranks
the third coordinate.** Proposition FC-drop below turns FF-corr's zero-test into a
**quantitative lower bound on capture** computable without minimising anything.
FT-A21 is blocked at trick 1 precisely because nothing predicts where a witness
can bite short of enumerating the frontier. **A screening functional that
lower-bounds capture is the first object in this branch that could, in principle,
do that** — and its behaviour must be understood where the exact answer is already
filed before it is trusted anywhere it is not.

---

### Proposition FC-drop (the quantitative form of FF-corr: correlation times reach, as an exact lower bound on capture) — delivered here

Fix a frontier state `I` with `δ_I > 0`. With Lemma FF-min's `G_I` and
`Φ_I(ω,b) = φ_I(ω,b) − c_I(b)`, write

  `s^+ = G_I'(0^+) = Σ_ω μ_I(ω) max_{b ∈ argmax_b q_I(ω,b)} (−Φ_I(ω,b))`,
  `s^- = G_I'(0^-) = Σ_ω μ_I(ω) min_{b ∈ argmax_b q_I(ω,b)} (−Φ_I(ω,b))`,

so `s^- ≤ s^+` by convexity, and let the state's **captured amount** be
`κ_I = δ_I − min_θ δ_I^θ = G_I(0) − min_θ G_I(θ)`. Then:

**(a) Zero test.** `κ_I = 0` **iff** `s^- ≤ 0 ≤ s^+`. When the clairvoyant argmax
is unique at every positive-mass `ω`, both collapse to `−C_I` with
`C_I = Σ_ω μ_I(ω) Φ_I(ω, b^*(ω))`, and `κ_I = 0` iff `C_I = 0`.

**(b) Descending side is populated.** If `s^+ < 0` there is a breakpoint strictly
to the right of `0`; if `s^- > 0` there is one strictly to the left.

**(c) The drop bound.** Let `t_0` be the distance from `0` to the nearest
breakpoint on the descending side — to the right when `s^+ < 0`, to the left when
`s^- > 0`. Then

  `κ_I ≥ |s| · t_0`,  where `s = s^+` in the first case and `s = s^-` in the
  second,

and this is exact arithmetic in rationals requiring **no minimisation** — one
directional slope and one breakpoint distance.

*Proof.* (a) is Proposition FF-corr restated with the one-sided derivatives named;
for a maximum of affine functions the one-sided slopes at a point are the extreme
slopes over the active set, and at `θ = 0` the active set at `ω` is
`argmax_b q_I(ω,b)` with the `b`-th slope `−Φ_I(ω,b)`. (b): if `s^+ < 0` and no
breakpoint lay to the right, `G_I` would be affine with negative slope on `[0,∞)`
and hence unbounded below, contradicting Lemma FF-min(b), which bounds `G_I` below
by the glued value; symmetrically on the left. (c): on `[0,t_0]` the function is
affine with slope `s^+`, so `G_I(t_0) = G_I(0) − |s^+| t_0`, and
`min_θ G_I ≤ G_I(t_0)`; subtract from `G_I(0)`. The left case is symmetric. ∎

**What it is and what it is not.** It is a **lower** bound, so a large value
proves a fee bites and a small value proves nothing — the true drop may continue
past `t_0` across many further pieces. Its content is that **capture is at least
correlation times reach**: `|s|` measures how far the feature leans on the
clairvoyant choice, `t_0` measures how far the fee can be pushed before the
choice starts changing, and the fee collects their product at minimum. It is the
first quantity in this branch that predicts capture without computing it.

### Corollary FC-null (an action-blind feature has exactly zero correlation) — delivered here

If `φ_I(ω,b) = ψ_I(ω)` does not depend on the action, then `s^+ = s^- = 0` at
every state, hence `κ_I = 0` — recovering Proposition FF-blind, and giving the
diagnostic a null control whose exact value is fixed by theorem.

*Proof.* `Φ_I(ω,b) = ψ_I(ω) − c_I` is `b`-free, so the inner max and min over the
active set both return `−Φ_I(ω)`, and `s^± = −Σ_ω μ_I(ω)(ψ_I(ω) − c_I) = 0` by the
definition of `c_I`. Then (a) gives `κ_I = 0`. ∎

---

- **FC-A1 (typing, tier, and what this chapter is).** The chapter is **GRANTED as
  the FC family**, scoped at FC-A2..FC-A4 and frozen at FC-A5. Everything is
  exploratory, below every tier, cited by nothing above it, quotable as a result
  only by brief amendment adding it to a verifier receipt. DS-A1 binds: **witness**,
  **receipt**, **necessary outer profile**, never the forbidden word. Both outcomes
  of every gate are results (F7). NO-RESCUE binds: a receipt failure is
  stop-and-report. **The chapter measures an instrument, not the game** — every
  number it produces is about how a fee behaves at coordinates whose exact answers
  are already filed, and by Proposition SR-degen no grade-4 verdict can move.
- **FC-A2 (the third coordinate: DEFERRED with its selection criteria fixed, not
  refused; and the freeze is deliberately withheld).** Four clauses.
  (i) **Deferred, and the reason is mathematical rather than budgetary.** See the
  scope ruling above. A third observation taken before the mechanism is measured
  enlarges FF-A29(ii)'s confound rather than resolving it.
  (ii) **No carrier is frozen here, on purpose.** FT-A23 fixed that **a freeze is a
  constant, not a rule**, so I may not freeze a coordinate "to be chosen by a rule
  after arm 1". The third coordinate therefore gets its **own freeze in a later
  ruling**, fixed as an enumerated constant once arm 1 has reported.
  (iii) **Its selection criteria are recorded now so they are not lost, and they
  are now two rather than one.** *(1)* FF-A29(iii)'s design input: vary trump
  survival at the frontier while holding declaration and root-action trump/non-trump
  type as fixed as possible. *(2)* **New, and it takes precedence if the two
  conflict:** vary whatever arm 1 shows to drive `s^±`. Whether trump survival is
  the mechanism or a correlate of it is exactly what arm 1 measures, and selecting
  on a correlate when the driver is known would repeat the error a third time.
  (iv) **The steward's reading of FF-A17(ii) is CONFIRMED and is a selection
  instrument, not merely an observation.** *If the number of trumps outside the
  focal hand is 1 and the root action is a trump, every boss-keyed feature has empty
  domain at every frontier state* — computable from the coordinate and root action
  with **no traversal**. Combined with freeze 52 v1.4's per-cell screen, arm
  selection has a pre-run test and no longer needs to discover emptiness by running.
- **FC-A3 (the joint two-feature fit: NOT commissioned, and the steward's own
  argument against its own agenda item is adopted verbatim).** FF-A15(iv) declined
  it because *"one live feature is not a joint problem, and pairing a live feature
  with a refuted one buys at most F1's 0.36%."* The steward flagged, against its own
  proposal, that this reason may still stand entirely. **It does, and unchanged.**
  Nothing since has produced a second live feature: F1 is refuted where it has a
  domain, F0 is refuted by theorem, and F2 is refuted at h2 and live only at h0.
  **Re-entry requires a second live feature, not a further declaration**, and
  FF-A15's naming of the LP form licenses nothing on its own. Raising it to be ruled
  on rather than leaving it to lapse silently is the right instinct and is
  commended; the ruling is that it stays declared and uncommissioned.
- **FC-A4 (graded F1: settled inside arm 1, cheaply, with no sweep).** FF-A15(iv)
  left the graded form of F1 **eligible rather than refuted**, on the ground that
  F1's refutation is of the binary predicate and does not formally transfer. The
  diagnostic settles it almost free: `s^±` is computable for graded F1 from the same
  pass, and **Proposition FC-drop(a) makes a zero correlation at every state a
  conclusive refutation of the graded form too** — no minimisation, no breakpoint
  enumeration, no capture figure. **Commissioned as a diagnostic-only member of arm
  1**: `s^±` is emitted for graded F1 and **no capture number is computed or
  reported for it**. If its correlation is broadly nonzero, that is a reason to
  commission a sweep, in its own later ruling; if it is zero, graded F1 joins the
  refuted list and FF-A15(iv)'s open item is discharged.
- **FC-A5 (FREEZE 53 — the fee-correlation diagnostic).**
  **(a) The carrier, enumerated with no generating rule** (FT-A23): the **same
  three units as freeze 52 v1.1** — h0 pip 3 `[00 21 32 53]` unit `a = 00`; h2
  pip 5 `[21 33 53 54]` units `a = 53` then `a = 54`. **No new coordinate is
  introduced**, and the coordinate identity is asserted in freeze 45's form at every
  unit with the kernel rebuilt in-run and asserted equal.
  **(b) The features, four, with their roles fixed.** **F0** `boss_owner`, the
  **null control**, exempt from freeze 52 v1.4's screen and run first and blocking.
  **F1** `boss_can_follow_b` and **F1g**, its graded form (the number of `b`-suit
  tiles the boss holder holds), the latter **diagnostic-only per FC-A4**. **F2**
  `b_is_beatable`, in the **amended** definition of FF-A15(i) — the opponent yet to
  play at `I` beating `b` and every tile already on the table.
  **(c) The measured object, per swept state `I` and per feature:** `s^+`, `s^-`,
  the **clairvoyant-argmax cardinality profile** (how many `ω` have a non-singleton
  argmax, which is what separates FC-drop(a)'s two collapse cases), `t_0` and the
  side it lies on, and the **FC-drop bound** `|s|·t_0`. Freeze 52(b)'s skip rule
  governs as ruled at FF-A23: **swept states only** — 1,332 at h0, 216 per h2 unit —
  with the domain census emitted over **every** frontier state per freeze 52 v1.3.
  **(d) The frozen comparison table.** The v1.1 run's per-state captured amounts
  `κ_I` are transcribed into the probe source with the provenance line *"quoted from
  `feature_fee_v11_2026-08-14.txt`, exploratory tier"* and **never re-parsed from
  results text** (SEP-A14(ii), FT-A28(i)). They are the reference against which
  (FC-R3) and (FC-R4) compare.
  **(e) Exact rationals only, no float anywhere** (P-A19), checked arithmetic, every
  divisibility asserted rather than assumed. **Count convention on every tax column**
  (Corollary SR-conv), with the two bridges kept separate as the v1.1 header does.
  **(f) Emission.** One row per `(unit, feature, state)` carrying the state record,
  `s^+`, `s^-`, the argmax profile, `t_0`, the bound, the frozen `κ_I`, and a verdict
  cell (`zero`/`positive`). Committed entire — the emission is small and there is no
  companion. **FF-A18 binds every figure: no count and no capture figure without its
  state set named in the same sentence.**
  **(g) Belief and field are NOT re-declared** — freeze 26 and 37(d) unchanged, no
  decimation ((C2)). **No library entry at any coordinate** (freeze 45). The
  freeze-set digest travels on every record. **(h) Budgets:** freeze 44(b) v2
  unchanged, no new constant; on exhaustion no partial fold.
- **FC-A6 (the receipts, six, with the non-receipts named).**
  (i) **(FC-R1) the null-control receipt — BLOCKING, first, before any other
  number.** Assert F0's `s^+ = s^- = 0` **exactly** at every swept state and every
  unit. Its answer is fixed by **Corollary FC-null**, i.e. by theorem and not by a
  filed number — the (SR-R9)/(FF-R1) role.
  (ii) **(FC-R2) the non-null pairing receipt — BLOCKING, and it exists because
  FF-A26(iv) said the next pre-declaration must require by design what h2 got by
  luck.** Assert that **F2 at h0's leading part has `s^± ≠ 0` at at least one
  state**, and emit the count of such states. **Contentful**: a diagnostic stuck at
  zero would satisfy (FC-R1) and fail here, which is exactly the failure mode a null
  control alone cannot see.
  (iii) **(FC-R3) the zero-characterisation receipt.** At every swept state and
  every swept feature, assert `κ_I = 0 ⟺ s^- ≤ 0 ≤ s^+`, with `κ_I` taken from the
  **frozen v1.1 table**. **Contentful**: it tests Proposition FC-drop(a) against a
  value produced by a different program in a different run, at over three thousand
  states.
  (iv) **(FC-R4) the drop receipt.** At every swept state with `s^- ≤ 0 ≤ s^+`
  false, assert `κ_I ≥ |s|·t_0` against the frozen `κ_I`. **Contentful**: it tests
  Proposition FC-drop(c) on real data and fails on any error in the slope, the
  breakpoint side, or the reach.
  (v) **(FC-R5) rung-one invariance and the digests.** Assert `|I_1|`, arrivals, the
  zero/positive census, `Δ^(1)` and `U^(1)` against the frozen table, and **assert
  the (FT-R7c) frontier digest at all three units** — h0 against the value the first
  FF run filed, h2's two against `SR_FIRST`. This build regenerates rung-one
  frontiers, so FT-A28(iv) binds.
  (vi) **(FC-R6) determinism.** An in-run second pass with fresh maps, accumulators
  and budgets; every printed row and summary asserted identical.
  (vii) **NAMED AS NON-RECEIPTS and printed as arithmetic remarks** (Proposition
  SR-taut): `s^- ≤ s^+`; `t_0 > 0`; `|s|·t_0 ≥ 0`; and `κ_I ≥ 0` as re-derived from
  the probe's own quantities. They cannot fail. **(FC-R3) and (FC-R4) are receipts
  only because they compare against the frozen v1.1 `κ_I`**, and that sentence is
  printed beside them.
- **FC-A7 (all outcomes pre-declared, before any number exists).**
  (a) **F0's `s^±` is anything but exactly 0** → stop-and-report; the harness is
  wrong and no other number is reported. **(b) (FC-R2) fails** → same, and it is the
  failure a null control could not have caught. **(c) At h2, `s^+ = s^- = 0`
  exactly at every state** → **F2 is exactly orthogonal to the clairvoyant choice
  there**; the h2 refutation acquires its mechanism, and an exact rational identity
  at 432 states becomes a structural fact about this coordinate demanding an
  explanation the branch does not yet have. **This is the outcome that would most
  change what we do next.** (d) **At h2, `s^- < 0 < s^+` with `s^± ≠ 0`** → the zero
  is **tie-driven**: the clairvoyant argmax is widely non-unique and the subgradient
  straddles zero. A mundane explanation, and the h2 refutation becomes a statement
  about argmax multiplicity rather than about the feature. (e) **Mixed across
  states** → report the split; no single mechanism. (f) **F1g's correlation is zero
  at every state** → **graded F1 is refuted** by FC-drop(a) with no sweep, and
  FF-A15(iv)'s open item is discharged. (g) **F1g's correlation is broadly nonzero**
  → a sweep is warranted and is commissioned in its own later ruling, **never
  inherited from this one**. (h) **(FC-R4)'s bound is tight nowhere** → the drop
  bound is valid but weak, and is reported as an instrument that screens rather than
  predicts; **that is a result under F7 and not a null**, because a weak screening
  bound is exactly what the trick-1 programme must know before relying on one.
  (i) **A budget stop** → declared stop, no partial fold, printed as a stop.
- **FC-A8 (the steward's item (3): the binding emitter items, CONFIRMED and
  complete).** All four carry, and the steward's list is correct: **freeze 52
  v1.4**'s per-cell screen with the null-control exemption (FF-A33(iii)); **the v1.2
  citation item** — the rulings line names the freezes the run executed under, which
  for this build is **freeze 53 with freeze 52 v1.4** and **must not** cite FC-A
  rulings written after it (FF-A31(vi)'s backwards-provenance trap); **FF-A32(v)**,
  a results file may restate a reading-rule a ruling has fixed and may never
  originate one; and **FF-A33(v)**, a byte-diff comparand must exist at the moment
  the diff is taken. One addition: **FF-A18's state-set rule binds the diagnostic's
  own columns too** — `s^±` and the bound are per-state quantities and any aggregate
  over them names its set.
- **FC-A9 (fences, what needs Jason, and what is owed).** Five clauses.
  (i) **Every standing fence travels verbatim**: R-A2/P-A1; the N4-A8 real-deal
  fence (the hands come from rob's corpus, **the belief does not**); SR-A25(iii)'s
  selection fence — **three units at two coordinates chosen by negative binding
  margin are a carrier, not a sample**; **P-A21 — no quantity measured at grade 4 is
  quoted for trick 1 or for the opening**, which binds hardest here because a
  screening functional is *for* trick 1 and will be tempting to quote there; and
  SR-A25(vii)'s implementation-versus-corpus risk, undiminished, with T1-A12's
  corpus check still owed.
  (ii) **Not claimed:** that `s^±` predicts capture (FC-drop is a **lower** bound
  and a small value proves nothing); that any mechanism found at h2 generalises;
  that any grade-4 verdict moved (Proposition SR-degen forbids it); anything about
  points, marks, bidding or real opponents; and **nothing about Jason's reading of
  h0** — FF-A9(ii) travels unchanged.
  (iii) **Nothing here needs Jason.** The chapter is inside the feature-fee/convex-fit
  commission he authorised, it introduces no new coordinate and no new budget
  constant, it touches no other repo, and **it touches `exchange/` not at all** —
  no dispatch, no outbox, no count. The steward's fence on exchange is noted and is
  not approached.
  (iv) **Owed to the wiki owner:** freeze 53, and the FC era-page and LOG entries,
  at chapter close. Per SR-A37(i) that is the whole list — **no claim-ledger,
  FINDINGS or open-problems row exists or is owed**, and a successor noticing their
  absence should not fix it.
  (v) **What this chapter cannot do, said plainly so the next ask is well aimed.**
  It will not decide a root action, will not touch `Δ^(2)`, and will not advance
  FT-A21's three trick-1 obligations by itself. What it can do is tell us whether
  the branch now holds an instrument that says *where* a fee can bite before one is
  built — and that is the only thing standing between the feature-fee line and the
  399-million-world wall.

**What the build owes this section.** The FC probe of FC-A5 over freeze 53's three
units and four features, with F0 first and blocking, F1g diagnostic-only and no
capture figure computed for it, the six receipts of FC-A6 with the non-receipts
printed as arithmetic remarks, the frozen v1.1 `κ_I` table as the comparison
carrier, and all nine outcomes of FC-A7 pre-printed before any number exists.
Everything else here is proof and needs no code.

- **FC-A10 ((FC-R3)'s "every swept feature": reading (A) governs — F0 and F2 — and
  the phrase is struck as ambiguous. The builder reported it rather than picking the
  reading that would have made my text consistent, which is the ambiguity protocol
  executed at its best.)** Six clauses.
  (i) **RULING: (A). (FC-R3) and (FC-R4) range over F0 and F2, at every swept
  state — 1,332 at h0 and 216 at each h2 unit, 1,764 per feature.** The frozen
  table is then exactly complete for the receipts that consume it.
  (ii) **Verified at adjudication time, not accepted.** `feature_fee_v11_2026-08-14.txt`
  carries **3,528** rows with a `captured =` column and its SHA-256 is
  `8ad7ec1f…d4670`, matching the fingerprint the steward supplied. The arithmetic
  closes on reading (A) and on no other: `1332 + 1332` at h0 plus `216 + 216` at
  each h2 unit is `3,528` for **two** features. F1 was not swept in the v1.1 run,
  being settled at FF-A13; F1g had never been computed before this chapter. The
  builder's count is exact.
  (iii) **The phrase is my defect and is struck rather than repaired.**
  *"Swept feature"* means one thing in the v1.1 frame (F0 and F2, the features that
  run swept) and another in the FC frame (all four, the features this probe
  sweeps) — **the same words denoting different sets in two adjacent chapters**,
  which is the FF-A18/FF-A23 family exactly and is the third appearance of it.
  **FC-A6(iii) and FC-A6(iv) are corrected in place to read "every feature carrying
  a frozen `κ_I` — F0 and F2"**, naming the set rather than deriving it from a
  word. The erroneous phrasing stays visible per LD-A11(ii).
  (iv) **FF-A18 is GENERALISED, and this catch is why.** The rule read *"no count
  and no capture figure appears without its state set named in the same
  sentence."* A receipt's scope here has **two** dimensions, states and features,
  and naming only one is what made the mislabel possible. **The rule now reads: a
  receipt's or a figure's scope names EVERY dimension it ranges over — state set,
  feature set, unit set — in the same sentence, and a scope derived from an
  adjective rather than stated as a set is not a scope.** The builder's proposed
  artifact wording already satisfies this and is adopted as the pattern.
  (v) **What F1 and F1g emit, settling the adjacent point: the FC-drop bound IS
  emitted for F1g, and it is not a capture number.** FC-A4 bars *"a capture number
  computed or reported"* for F1g, and the builder is right that the bound is a
  different object. Three reasons it is emitted. **(1)** It is a **proved lower
  bound**: by Proposition FC-drop(c) a positive `|s|·t_0` *establishes* that F1g
  captures at least that much, with no minimisation and no `κ_I` to compare
  against — it is self-standing evidence rather than a measurement awaiting a
  check. **(2)** It is exactly the screening quantity this chapter exists to
  characterise, and withholding it at the one feature where no capture is known
  would test the instrument only where the answer is already filed. **(3)** It
  needs no sweep, which is what FC-A4's exemption was protecting.
  **Binding on how it is printed:** the column is labelled **"proved lower bound on
  capture (Proposition FC-drop(c))"**, **never** "capture", it carries **no `κ_I`
  column and no comparison** because none exists, and every F1 and F1g row says in
  place why there is no comparison. F1 emits the same set as F1g.
  (vi) **Two confirmations on the not-blocked list, one of them load-bearing.**
  F1g as the count whose positivity F1 tests, from the same incidence
  intersection, is correct and the by-construction agreement is worth asserting in
  place. And the builder's own note that `s^±` must be taken over the **complete**
  clairvoyant argmax set is **exactly right and is the sharpest hazard in this
  build**: a tie-broken argmax collapses `s^-` and `s^+` to one number, which turns
  FC-drop(a)'s **straddle** test into a **point** test — and the straddle is
  precisely what distinguishes FC-A7's outcome (c), genuine orthogonality, from
  outcome (d), a tie-driven zero. **A tie-break there would not perturb the
  chapter's headline; it would silently answer its central question with the wrong
  one of two pre-declared readings.** Freeze 38(e)'s complete-face rule binds, and
  it binds here harder than anywhere it has bound before.
  (vii) **FC-A7(g) is sharpened by (v).** With the bound emitted, a positive
  `|s|·t_0` for F1g does not merely *warrant* a sweep — it **proves F1g's capture is
  positive** at that state, by theorem. The pre-declared reading becomes: *bound
  positive somewhere* → F1g bites, magnitude unknown, and a sweep is commissioned
  in its own later ruling to measure how much; *`s^± = 0` everywhere* → F1g refuted
  by FC-drop(a) and FF-A15(iv)'s open item discharged; *`s^± ≠ 0` but the bound
  zero or negligible everywhere* → correlation without reach, which is a third
  reading and the most interesting of the three for the instrument's own
  characterisation, since it is the case where the screening functional would
  mislead if used alone.

- **FC-A11 (the collapsed-argmax hazard gets a RECEIPT rather than a code
  reading: (FC-R7), added to freeze 53's receipt set. Prompted by the steward's
  gate note and by reading where the per-world argmax actually lives.)** Five
  clauses.
  (i) **Why a code reading is the wrong instrument, stated so the gate is not
  mis-aimed.** The steward proposed to check the argmax face by reading the code
  path rather than the receipts, *"since a receipt computed from a collapsed face
  would hold."* That reasoning is correct about the receipts as they stood, and it
  is exactly why a receipt is owed instead: **a hazard that survives every existing
  check is a hazard the check set is missing, not one to be handled by reading.**
  Code reading catches it once, in one session, by one reader; a receipt catches it
  on every run forever.
  (ii) **Where the hazard actually lives, read at adjudication time.** The FF probe
  does **not** carry a per-world argmax mask: `feature_fee.rs` stores
  `WorldRow { w, q: qrow, phi }` — the full per-world `q` vector — and any argmax is
  derived downstream from `qrow`. So the collapse cannot happen in the walk; it
  happens in whatever derives `b^*(ω)`, and **the natural Rust idiom is exactly the
  defect**: `qrow.iter().enumerate().max_by_key(…)` returns **one** index. The
  correct object is the **set** `{ j : qrow[j] == max(qrow) }`, obtained by an
  equality test across all `j` — which is the pattern `fusion_tax.rs` already uses
  in its walk (`if child[j] == best { mask.insert(d) }`) and which (FT-R5) and
  (SR-R8) assert at millions of states. **Mirror that pattern; do not re-derive an
  argmax by index.**
  (iii) **(FC-R7) THE FILED-FACE RECEIPT — added to FC-A6, and it catches the
  collapse outright.** `fusion_tax_2026-08-14.txt` prints, for every state with
  `δ_I > 0`, a minimal fusion core as **per-world complete argmax sets** by fiber
  index — e.g. `[5685:{32} 5689:{21 53}]`, where `{21 53}` is a two-element face.
  Those masks are complete by freeze 38(e) and are already receipted by (FT-R5).
  **For every swept state of freeze 53's three units, assert that the FC probe's own
  per-world complete argmax set at each fiber index named in that state's filed core
  equals the filed mask exactly.** A collapsed face returns `{21}` or `{53}` where
  the file says `{21 53}` and the receipt fails on the spot.
  **Contentful, and in the strongest way available:** it compares against masks
  produced by a **different program**, committed on a different day, at every swept
  state of the carrier — h0's 1,332 and h2's 216 per unit, the FT file carrying
  12,693 such cores in total. It costs two world lookups per state.
  (iv) **What it does and does not reach.** It reaches every state that has a filed
  core, which is exactly the swept set, and it proves the face construction is
  complete **at the two worlds of each core**. It does **not** prove completeness at
  every world of every state — a collapse that happened to preserve the core worlds
  and no others would survive. That residual is named rather than hidden; I know of
  no mechanism that would produce it, since the derivation is uniform across worlds,
  and (FC-R7) plus (ii)'s pattern requirement together make it remote. **A named
  residual costs nothing; an unnamed one is how a chapter goes wrong.**
  (v) **The steward's gate stands alongside it, re-aimed.** With (FC-R7) in place
  the gate is no longer "read the code path for correctness" but the far cheaper
  **"confirm the derivation accumulates a set by equality across all `j`, and does
  not track an index"** — one glance, and the receipt covers the rest on every
  future run. This is the general shape: **when a defect class is invisible to the
  receipts, the answer is a new receipt, and the human check shrinks to confirming
  the one line the receipt cannot see.**

- **FC-A12 (`7646ca6` checked; the transcription guard already exists and is
  stronger than the one I would have added; and the boundary of FC-A11's lesson,
  fixed so it is not over-applied).** Three clauses.
  (i) **FC-A10(ii)'s cited fingerprint still identifies the committed object.**
  `7646ca6` touches `walt-factory/src/fc_kappa.rs` — the transcribed table in the
  probe **source** — and not `feature_fee_v11_2026-08-14.txt`. The results file's
  SHA-256 re-reads `8ad7ec1f…d4670` at adjudication time, unchanged, so the
  fingerprint I quoted in a ruling still resolves. Checked rather than assumed,
  because a ruling that cites a fingerprint has taken on the obligation to notice
  when it goes stale.
  (ii) **The guard I was going to require is already in place, and it is a
  bijection rather than an aggregate — nothing is added.** I had intended to
  require a cross-check of the 3,528 transcribed `κ_I` against an independently
  transcribed aggregate, on the reasoning that a transcription slip would present
  as a **theorem failure** — (FC-R4) reporting `κ_I < |s|·t_0` — and so would
  trigger stop-and-report against Proposition FC-drop when the proposition is fine.
  That reasoning stands; the requirement does not, because `fc_kappa.rs`'s header
  already records three checks made at transcription time: the six per-block counts;
  **every `captured =` row in the source claimed by exactly one block, 3,528 of
  3,528, none unclaimed and none double-claimed**; and all 3,528 `(unit, feature,
  record)` keys distinct. **A bijection between source rows and table entries is
  strictly stronger than any sum over them**, and it forecloses the failure mode
  that motivated my addition. **Proportionality (FT-A28(iv)): nothing is added, and
  the reason it is not added is recorded so the omission is a decision rather than
  an oversight.**
  (iii) **The boundary of FC-A11's lesson, fixed now because a successor will
  otherwise cross it.** FC-A11 ruled that when a defect class is invisible to the
  receipts, the answer is a **new receipt** rather than a human reading. Read
  without a boundary that licenses converting *every* one-time check into a
  run-time assertion — and a successor arriving at `fc_kappa.rs`'s
  transcription-time audit would naturally ask why it is not receipted. **It must
  not be, and the distinction is the object's mutability.** (FC-R7) guards a
  quantity the probe **recomputes on every run**, where a wrong construction
  reproduces silently forever; the transcription audit guards a **compiled
  constant** that cannot drift between runs and can only change under a commit,
  where version control is the guard. **A run-time assertion over a compiled
  constant compares the constant against itself and is an arithmetic remark
  (Proposition SR-taut), not a receipt.** So: **convert a human check into a receipt
  when the guarded object is recomputed each run; leave it a documented one-time
  audit when the object is fixed source.** That also delimits FF-A33(v) correctly —
  its "the comparand must exist at the moment the diff is taken" governs
  **emissions**, which differ run to run, and has nothing to say about constants,
  which do not.

- **FC-A13 (the v1.4 screen against FC-A7(f)/(g) at h2: reading (A) GOVERNS, with
  one sharpening the builder did not propose; and FC-A11(iii)'s core count is
  corrected with the right diagnosis of my own error).** Five clauses.
  (i) **RULING: (A). The four cells — `(h2 a=53, F1)`, `(h2 a=53, F1g)`,
  `(h2 a=54, F1)`, `(h2 a=54, F1g)` — are EMPTY TESTs**, emit no `s^±` rows, no
  bound and no census beyond the domain census, and say in place that `φ ≡ 0` there
  so **Corollary FC-null fixes `s^± = 0` by theorem rather than by measurement.**
  Three independent reasons.
  **(1) The screen's operative clause is written for exactly this.** Freeze 52
  v1.2's *"contributes to no capture figure and no outcome gate"* exists because
  FF-A11's empty-domain zeros entered six cells as measurements. FC-A7(f)/(g) is an
  outcome gate. The screen reaches it.
  **(2) The decisive reason, which is that F0's exemption already does this job.**
  F0 is exempt from the screen and therefore runs at h2, where **its domain is also
  empty**, and (FC-R1) asserts `s^± = 0` there blocking. F1 and F1g at h2 would be
  zero for the *same reason by the same theorem* — `φ ≡ 0` makes a feature
  action-blind, and Corollary FC-null then fixes `s^±`. **So the empty-domain
  theorem-zero is already exercised at h2 by the null control; F1 and F1g there
  would be two further copies of it, not additional evidence.** This is the argument
  that settles it, and it is why (A) loses nothing.
  **(3) The builder's own reason stands.** Under (B), *"F1g's correlation is zero at
  every state"* would range over 1,764 states of which 432 are zero tautologically —
  a zero-by-construction averaged into a refutation count, which is FF-A11 in new
  dress and the fault this chapter has now met in three dimensions.
  (ii) **The sharpening, and it applies the same principle one level down where the
  builder stopped at a note.** At h0 the boss-keyed domain is nonempty at **574** of
  the 1,332 swept states. Under (A)'s own logic the remaining 758 are theorem-fixed
  exactly as h2's 330 are, so **FC-A7(f)'s refutation reading for F1 and F1g ranges
  over the 574 domain-nonempty states of `h0 a=00`, and over no others.** A reading
  scoped to 1,332 would be 57% tautological — the identical fault, merely smaller.
  A note recording the 574 is not sufficient; **the scope sentence must name it as
  the set.**
  (iii) **The cell/state distinction this yields, stated so it generalises.**
  **A wholly empty cell is screened and emits no rows** (h2's four). **A partly
  empty cell runs, emits every swept row carrying a domain flag, and has each
  reading over it scoped to the domain-nonempty subset** (h0's two). Emit over the
  full set, read over the meaningful subset, **name both** — which is FC-A10's
  sweep-versus-census separation arriving in a third place, and by now it should be
  treated as the house pattern rather than rediscovered each time.
  (iv) **The builder's counter-worry is answerable, and the answer is that (A)
  preserves FC-A10(v)'s purpose entirely.** FC-A10(v) emitted F1g's bound because
  withholding it would *"test the instrument only where the answer is already
  filed"* — where "the answer" means **a filed `κ_I`**, which exists for F0 and F2
  and not for F1g. Under (A) the bound **is** emitted for F1g at h0's 574
  domain-nonempty states, which is precisely a place with no filed `κ_I`, so the
  instrument is exercised exactly where FC-A10(v) wanted it. What (A) withholds is
  h2, where the bound is **0 by Corollary FC-null** — and **a bound fixed by theorem
  tests the instrument not at all.** No purpose is lost.
  (v) **FC-A11(iii)'s "12,693" is WRONG, the count is 12,639, and the diagnosis
  matters more than the digit.** Verified at adjudication time: the FT file carries
  **12,639** occurrences of `"minimal fusion core (fiber indices"` — the filed
  row-level cores — plus **54** of `"minimal fusion core of size"`, the (FT-R5)
  complete-face sample lines, totalling the 12,693 I quoted. **It was not a
  transposition.** I counted with a pattern that matched two different objects and
  reported the union as one of them — **a set quoted without its defining predicate
  pinned**, which is the same fault as "swept feature" and as the sweep/census
  conflation, and it is the fourth instance and mine. That I committed it in the
  very ruling that generalised FF-A18 to *"a scope derived from an adjective is not
  a scope"* is the useful part of the record. The correct figure **12,639 also
  matches FT-A29's independently filed census** of states with `δ_I > 0`, which is
  the cross-check that confirms it. **Nothing in (FC-R7) turns on the total**: it
  ranges over freeze 53's three units, `1,332 + 216 + 216 = 1,764` cores, and the
  builder's verification that those cores' `(unit, record)` key set equals
  `fc_kappa.rs`'s at F0 and again at F2 is the check that matters. **FC-A11(iii) is
  corrected in place; the erroneous figure stays visible per LD-A11(ii).**

- **FC-A14 (two notes on the steward's independent verification: the core-count
  diagnosis is corrected a second time, and the builder's unprompted scope-equality
  check yields a discipline neither FC-A10 nor FC-A11 names).** Three clauses.
  (i) **The 12,693 → 12,639 correction stands as ruled at FC-A13(v), but the
  steward's diagnosis of it — "a digit transposition" — is WRONG, and the
  difference is not pedantry.** Counted a third time here: the FT file carries
  **12,639** lines matching `"minimal fusion core (fiber indices"` and **54**
  matching `"minimal fusion core of size"`, and `12,639 + 54 = 12,693` exactly. **A
  transposition of 12,639 would give 12,369 or 12,693 by coincidence of digits; what
  actually happened is that a grep matched two different objects and I reported the
  union as one of them.** The two diagnoses carry different lessons. *Transposition*
  says be careful typing. *The truth* says **a count was quoted without its defining
  predicate pinned** — which is the fault this chapter has now met four times, and
  which is exactly what the generalised FF-A18 exists to catch. The steward enforces
  FF-A18; a steward who believes my error was a typo will under-apply the rule at
  precisely the moment it is needed. **The mechanism of a defect is what the record
  is for; the digit is not.**
  (ii) **The builder's scope-equality check closes a real gap that neither FC-A10
  nor FC-A11 requires, and it is recorded here as a discipline.** Transcribing
  (FC-R7)'s material it verified that the filed-face table's `(unit, record)` key
  set is **equal** to the frozen `κ_I` table's key set at F0 and again at F2. Why
  that matters: **(FC-R7) ranges over the states carrying a filed core, and (FC-R4)
  over the states carrying a frozen `κ_I`, and those two sets come from different
  carriers** — `fc_cores.rs` transcribed from the FT file, `fc_kappa.rs` from the
  v1.1 file. Nothing in my rulings asserts they coincide. Had they differed, **both
  receipts could hold while silently covering different states**, and the artifact
  would report two green checks over sets it never claimed were the same one.
  **The discipline, which generalises FF-A18 one step further: naming a scope is not
  enough when two scopes must coincide — that they coincide is itself a thing to
  check.** FF-A18 said state your scope as a set; this adds: **when two receipts
  range over sets derived from different carriers and the reading depends on their
  being the same set, assert the equality.** Filed as binding on any future probe
  carrying more than one transcribed table.
  (iii) **It correctly stays a one-time audit and is not receipted, which is
  FC-A12's boundary landing where it was aimed.** Both key sets are compiled
  constants that cannot drift between runs; a run-time assertion would compare
  constants against constants (Proposition SR-taut). The steward placed it correctly
  without being asked, and the builder — having been given FC-A12's boundary in
  operational form — built a second frozen table and **did not propose receipting
  it either**. Whether the boundary arrived in time or was reached independently,
  the outcome is the same and it is the outcome the boundary was written for.

- **FC-A15 (what independent verification actually requires, discovered by the
  steward about its own check and ratified here as doctrine; it is the SR-taut
  principle raised one level, from the checked object to the checking process).**
  Four clauses.
  (i) **The steward's self-observation, stated as it stated it and ratified.** It
  had reported the builder's 12,639 as independently confirmed, and it was — *in
  the weak sense*: it ran **the same grep phrase the builder ran** and got the same
  number. Its own conclusion is the right one: **"my check confirmed the count and
  not the predicate"**, and had the error been in the predicate rather than the
  arithmetic it would have confirmed a wrong number with full confidence and
  reported it upward as verified. It volunteered this rather than let a
  successful-looking check stand, which is the behaviour this record exists to
  reward.
  (ii) **The mechanism of the actual catch, named because it is the instructive
  part: three parties, two predicates — and the independence that mattered was the
  predicate's, not the party's.** My original count used the phrase
  `"minimal fusion core"` and returned 12,693. The steward re-ran that same phrase
  and confirmed 12,693's successor figure by the same route. **What broke the tie
  was that the builder was counting a different object** — it transcribed the cores
  themselves while wiring `fc_cores.rs`, so it counted *cores* where we counted
  *lines matching a string* — and the diagnosis then came from decomposing the
  predicate into `"…(fiber indices"` (12,639) and `"…of size"` (54). **Two agents
  running one grep are one check, however many agents there are.**
  (iii) **The doctrine.** *Independent verification means an independent
  **predicate**, not an independent **party**.* A second reader re-running the first
  reader's query has confirmed reproducibility and nothing else — and
  reproducibility is exactly what a wrong predicate has in abundance. **Practical
  form, adopted for this record: when confirming a count or a figure drawn from an
  artifact, state the predicate alongside the number**, so that the next party can
  see what was matched and deliberately choose a different route. A figure quoted
  without its predicate cannot be independently checked; it can only be re-run.
  (iv) **Why this is Proposition SR-taut one level up, and why it belongs in this
  file.** SR-taut ruled that an assertion between two quantities the checker itself
  derived is an arithmetic remark, and that a receipt must compare against something
  it did not produce. **The same principle governs verification: a re-check that
  reuses the original's method has not produced independent evidence, it has
  produced the original's output a second time.** FT-A28(i) required a receipt to
  name the carrier of its reference value; FC-A14(ii) required two scopes that must
  coincide to be asserted equal; this requires a confirmation to name the route by
  which it confirmed. **Four instances of one idea: evidence is only as independent
  as the thing it did not share with what it checks.**

---

### Closing note: the FC probe returned (2026-08-14, after the run)

**Object:** `walt-factory/results/fc_correlation_2026-08-14.txt`, committed at
`08f1b61` after the FC-A13(ii) regeneration at `4486aa0`/`404983c`. Three units,
four features, seven receipts HELD at every non-screened cell, (FC-R1) blocking
and first. **The chapter's central question is answered, unanimously, and it is
the answer I called mundane before the run: outcome (d).** Rulings FC-A16..FC-A21;
two propositions delivered.

**Re-derived at adjudication time from the artifact's own rows**, all exact:

- **h2, both units, F2: the straddle holds at all 216 swept states and `s^±` are
  NOT both exactly 0 at all 216.** So `s^- < 0 < s^+` strictly, everywhere.
  **Outcome (c) is refuted at every state of the carrier**, not merely
  unsupported.
- Argmax multiplicity: **236,784 of 362,880 arrivals (65.25%)** at each h2 unit
  carry a non-singleton clairvoyant argmax, against **59,776 of 266,132 (22.46%)**
  at h0. The h2 arrival count checks: `216 × 1680 = 362,880`.
- h0 F2 tightness: the bound is exact at **258 of the 1,252** straddle-false states
  (**20.61%**), and the summed bound `9774908973343/790918236108000` is
  **14.873%** of the summed captured amount `16474225753499603/198256837851072000`
  — the builder's "about 14.9%", confirmed, and that captured amount reproduces
  75.1420% of `Δ^(1)` exactly as the v1.1 run filed it.
- Straddle-false counts over h0's **574 domain-nonempty** states: **F1 at 374
  (65.16%)**, **F1g at 322 (56.10%)**. Both reconstruct from the 1,332-state
  censuses by subtracting the 758 empty-domain states, which are straddle-true by
  Corollary FC-null.

### Proposition FC-width (the subgradient's width is a tie functional, and it is why h2's zero is robust rather than coincidental) — delivered here

At a swept state `I`, with `A^*(ω) = argmax_b q_I(ω,b)` the complete clairvoyant
face,

  `s^+ - s^- = Σ_ω μ_I(ω) · [ max_{b ∈ A^*(ω)} Φ_I(ω,b) − min_{b ∈ A^*(ω)} Φ_I(ω,b) ] ≥ 0`,

so the width of the subgradient at `θ = 0` is **the `μ_I`-weighted sum of the
feature's spread across the clairvoyant tie**. In particular the width is `0`
whenever every positive-mass world has a singleton face, and a world contributes
positively **only if** its face is non-singleton *and* `Φ_I(ω,·)` is non-constant
on it.

*Proof.* Subtract the two displayed forms of `s^±` from Proposition FC-drop(a);
the `μ_I`-weights are common and the bracket is the spread. Nonnegativity is
`max ≥ min`. A singleton face makes the bracket `0` termwise. ∎

**What it explains, and it is the whole of the h2 result.** By FC-drop(a) capture
vanishes iff `0 ∈ [s^-, s^+]`. **Without ties that interval is a point**, so a zero
capture demands the exact rational identity `s^± = 0` — a coincidence, and one
that could not plausibly hold at 216 states twice over. **With ties it is an
interval of positive width**, and a zero capture requires only that `0` fall
inside it — which is robust, not coincidental. h2 carries a non-singleton face at
**65.25%** of arrivals and h0 at **22.46%**; h2's straddle holds at 216 of 216
states and h0's fails at 1,252 of 1,332. **The mechanism and the measurement
agree.**

### Proposition FC-tight (the drop bound is exact exactly when the descent is one piece) — delivered here

At a state where the straddle is false, `κ_I = |s| · t_0` **if and only if** the
minimum of `G_I` is attained at the first breakpoint on the descending side —
equivalently, iff `G_I`'s slope becomes non-negative immediately after `t_0`.
Otherwise `κ_I > |s| · t_0` strictly.

*Proof.* `G_I` is affine with slope `s` on the segment from `0` to `t_0`
(Lemma FF-min(a)), so `G_I(t_0) = G_I(0) − |s| t_0` exactly; the bound equals the
drop iff `min_θ G_I = G_I(t_0)`, which for a convex piecewise-linear function is
iff its slope does not remain negative past `t_0`. ∎

**Measured:** that holds at **258 of 1,252** biting states at h0 F2 — the descent
is a single linear piece about one time in five — while the summed bound recovers
**14.873%** of the summed capture. **The instrument is loose in aggregate and
exact at a fifth of the states where it bites**, which is a different object from a
uniformly weak screen and is what FC-A18 rules on.

---

- **FC-A16 (the artifact against freeze 53 and FC-A13: NO DEVIATION, and the
  regeneration is verified).** The four h2 `(F1, F1g)` cells are EMPTY TESTs as
  FC-A13(i) ruled; every non-screened cell carries an explicit READING SCOPE line;
  the two partly empty cells read over h0's **574** domain-nonempty states as
  FC-A13(ii) requires; the null control is read over every swept state of every
  unit with that exception **stated as deliberate rather than left to inference**,
  which is the right handling of the one place the pattern is knowingly broken.
  The regeneration diff is 13 lines added and 1 removed, the removal being
  wall-clock, so **no computed figure moved** — consistent with FC-A13(iii)'s
  claim that this was a labelling change and not a computation change.
  **The builder's own demonstration of why the sharpening mattered is the best
  argument in its report and is adopted verbatim:** over the 1,332 the F1g straddle
  census reads 1,010/322, and over the 574 it reads 252/322 — *"the first pair
  invites 'zero at most states' and the second says what is true."* **A note is not
  a scope**, and here the difference between the two is the difference between a
  56% minority and a 76% majority describing the same fact.
- **FC-A17 (OUTCOME (d), UNANIMOUS: h2's exact zero is TIE-DRIVEN, outcome (c) is
  REFUTED at every state of the carrier, and the finding is worth more than the
  "mundane" label I gave it before the run).** Five clauses.
  (i) **The ruling.** At both h2 units, at all 216 swept states, the straddle holds
  while `s^±` are not both zero — strict straddle everywhere. FC-A7's outcome (d)
  fires and outcome (c) is **refuted rather than unsupported**, which is the
  stronger of the two ways a pre-declared alternative can fail.
  (ii) **The mechanism, and it is Proposition FC-width.** Zero capture at h2 does
  not mean F2 is orthogonal to the clairvoyant choice; it means **the clairvoyant
  choice is not pinned down** — 65.25% of arrivals carry a non-singleton face, the
  subgradient is an interval of positive width, and `0` lies inside it robustly.
  **The h2 refutation of F2 is therefore not a statement about F2 at all.**
  (iii) **What it buys, and this is why "mundane" undersold it.** *(1)* It
  **decouples the refutation from the feature**: no fee keyed on the clairvoyant
  choice can be expected to bite where the face is widely non-singleton, because
  FC-width makes the subgradient wide for *any* such feature. That is a statement
  about the **fee route**, not about a candidate. *(2)* It identifies a
  **pre-fee screening statistic** — the argmax cardinality profile — which is a
  property of the coordinate's world structure, is measurable before any fee is
  built, and gates whether building one is worth attempting. *(3)* It is therefore a
  **negative result about the fee route's reach that is structural rather than a
  failure of cleverness**, which is the most useful kind this branch produces.
  (iv) **What it does NOT establish, and the fences are undiminished.** Two
  coordinates chosen by negative binding margin are a **carrier, not a sample**
  (SR-A25(iii)); **P-A21 binds hardest here** — the multiplicity figures are
  grade-4 measurements and **nothing about trick-1 or opening multiplicity is
  quoted or implied**, which matters precisely because multiplicity is now the
  variable everyone will want to extrapolate; no grade-4 verdict moved
  (Proposition SR-degen); and the whole chapter prices `Δ^(1)` only.
  (v) **One honest note on my own pre-declaration.** I wrote outcome (d) as *"a
  mundane explanation"* and outcome (c) as *"the outcome that would most change
  what we do next."* **That ordering was wrong.** (c) would have been a puzzle —
  an unexplained exact identity demanding further work. (d) is an **answer**, and
  it hands the programme a screening variable it did not have. Pre-declaring both
  was right; ranking them was a guess, and the guess did not survive.
- **FC-A18 (FC-A7(h): the bound is LOOSE IN AGGREGATE AND EXACT AT A FIFTH, which
  is a third reading and better than either pre-declared one).** Three clauses.
  (i) **The measurement.** At h0 F2 over the 1,252 straddle-false states of that one
  unit and feature: the bound equals the frozen `κ_I` at **258** of them (20.61%),
  the largest ratio being `1/1` attained at `I=[00 10 11 43 42]`, while the summed
  bound is **14.873%** of the summed capture. Re-derived here from the artifact's
  own two totals.
  (ii) **Proposition FC-tight says what the 258 are:** exactly the states where the
  descent is a **single linear piece**. So the instrument is not uniformly weak —
  it is *exact* where the fee's optimum sits at the first breakpoint and
  progressively conservative as the descent runs through more pieces. **A screen
  that is exact one time in five and recovers a seventh of the total is a usable
  triage instrument and a poor estimator**, and those are different jobs.
  (iii) **Typed for what it licenses.** As a **lower** bound it can prove a fee
  bites and can never prove one does not; as a screen its false-negative rate is
  what matters and is now bounded below by these numbers at this carrier. **No
  extrapolation to trick 1** (P-A21). FC-A7(h)'s pre-declared reading — a weak bound
  is a result and not a null — stands, with the refinement that "weak" was the wrong
  single word.
- **FC-A19 (FC-A7(g): F1g's capture is PROVED POSITIVE at 322 states — and the
  calibration that stops this being read as promising. The sweep is NOT
  commissioned.)** Four clauses.
  (i) **The finding.** F1g's straddle fails at **322 of h0's 574** domain-nonempty
  swept states, so by Proposition FC-drop(c) **graded F1's capture is proved
  positive there, by theorem, with no sweep** — exactly what FC-A4 designed the
  diagnostic-only cell to be able to do. FF-A15(iv)'s open item is **answered, and
  answered away from refutation.**
  (ii) **The calibration, which the report does not draw and which governs how this
  reads.** **F1 — the binary form, already refuted — has straddle-false at 374 of
  the same 574 states, which is MORE than F1g's 322.** And F1's measured oracle-θ
  capture at those very states was **0.367%**. So this carrier supplies a direct
  price for the phrase "proved positive at N states": at F1 it was worth about a
  third of one percent. **F1g clears the bar at fewer states than the feature that
  cashed out at 0.367%.** Proved-positive and negligible are entirely compatible,
  and here the compatible reading is the likely one.
  (iii) **The sweep is NOT commissioned.** FC-A7(g) pre-declared that this reading
  *"warrants"* a sweep in its own later ruling; that ruling is this one and it
  declines. Two reasons: the calibration in (ii) makes the expected yield small,
  and **Proposition FF-oracle means even a large per-state-oracle result would
  establish nothing about a usable family** — it would license a shared-θ fit and
  no more, which is a third run to answer a question whose prior is already weak.
  **A run is better spent on FC-A20's variable.**
  (iv) **What would reopen it:** a measured shared-θ capture for F2 at a second
  coordinate that is materially different from h0's, which would show the fee route
  is coordinate-robust and make a second live feature worth having. Nothing less.
- **FC-A20 (FC-A2(iii)'s second criterion is now ANSWERABLE, and it supersedes the
  first: a third coordinate is selected on argmax multiplicity, not on trump
  survival).** Three clauses.
  (i) **The criterion fires as written.** FC-A2(iii) fixed two selection inputs and
  ruled that *"vary whatever arm 1 shows to drive `s^±`"* takes precedence if the
  two conflict. **Arm 1 shows the driver is clairvoyant-argmax multiplicity**, by
  Proposition FC-width and by 65.25% against 22.46%. The builder's reading is
  confirmed: multiplicity is a property of the coordinate's world structure,
  measurable **before any fee is built**, so whether trump survival is the mechanism
  or a correlate is now **answerable by measurement rather than settled by
  selection**.
  (ii) **Consequently FF-A29(iii)'s trump-survival input is DEMOTED to a
  hypothesis about a correlate**, and a third coordinate is chosen to **vary
  multiplicity while holding as much else fixed as possible**. If a candidate varies
  both, the multiplicity contrast governs the reading and the trump contrast is
  reported as a co-varying fact, not as an explanation.
  (iii) **Still not commissioned here, and still needing its own freeze**
  (FT-A23: a freeze is a constant, not a rule). What has changed is that the
  selection is now made on a **measured** variable rather than a guessed one, which
  is precisely what FC-A2 deferred the coordinate in order to achieve. **That
  deferral is now discharged as having been the right call** — had a third
  coordinate been run first, it would have been selected on trump survival and the
  multiplicity mechanism would still be unknown.
- **FC-A21 ((FC-R7) vindicated on the numbers; FC-A15(iii) adopted by the builder;
  and what is owed).** Four clauses.
  (i) **(FC-R7) HELD over 3,528 filed masks, 1,374 of them two-tile, all equal —
  and the builder's analysis of what it was worth is exactly right and is
  adopted.** At h2 a collapsed face would **most likely** have failed (FC-R3)
  loudly, because collapsing a strict straddle generically yields a nonzero point
  value while the frozen `κ_I` is `0`. **But only most likely**: a collapsed value
  landing exactly on `0` would have reported **outcome (c) with every receipt
  green** — the chapter's central question answered with the wrong one of two
  pre-declared readings, silently. **That is the residue the reasoning could not
  close and the receipt did**, which is FC-A11(i)'s argument confirmed by numbers
  rather than assent. The 1,374 two-tile masks are the ones that would have caught
  it, and they are 39% of the comparison.
  (ii) **FC-A15(iii) is adopted by the builder and its predicate is exemplary.**
  Its 1,764 count came from **parsing cores** — matching the row shape and then
  requiring every split body part to match `index:{tiles}`, so a line carrying the
  phrase but not the shape would have **failed loudly rather than been counted**.
  That is why its route could not absorb the 54 `"of size"` lines and why the
  disagreement was informative. **A predicate that fails loudly on a near-miss is
  strictly better than one that silently matches it**, and that is the practical
  form of FC-A15 for anyone quoting a figure from an artifact.
  (iii) **Nothing is routed back to the builder.** The artifact discharges its
  contract, the regeneration is verified, and the three points it offered were all
  correct and are all ruled on above.
  (iv) **Owed to the wiki owner:** freeze 53, the FC era-page and LOG entries, and
  the chapter-close digest. Per SR-A37(i) that is the whole list — **no
  claim-ledger, FINDINGS or open-problems row exists or is owed.** Everything in
  this chapter is exploratory, below every tier, cited by nothing above it.

**Where this leaves the line, said plainly.** The fee route works at h0's leading
frontier — 76.46% oracle, 99.87% of it retained by a single shared parameter — and
is **structurally blocked** wherever the clairvoyant face is widely non-singleton,
which is not a fixable defect of any candidate feature. The programme therefore
has a new first question at any coordinate it contemplates: **not "which feature",
but "is the clairvoyant choice pinned down enough for any fee to bite".** That
question is cheaper than building a fee and it now has an exact statistic. It does
not advance FT-A21's three trick-1 obligations, and nothing here is quoted for
trick 1 — but it is the first thing this branch has that says where **not** to
spend the attempt.

- **FC-A22 (supplement to the FC adjudication: four items the steward held, and a
  phrasing constraint that lands on MY OWN text before it could land on the era
  page).** Six clauses.
  (i) **The byte-diff exists, was taken by a third party while both objects
  existed, and upgrades FC-A16's basis.** `git diff --numstat 4486aa0 08f1b61`
  gives 13 added and 1 removed, the removal being wall-clock. FC-A16 accepted the
  confinement claim on the builder's report; it is now **measured, by someone who
  did not produce either emission, at the only moment it could be taken.** FF-A33(v)
  named that trap after it had already cost us one; here the builder committed the
  first emission before regenerating and the check was therefore possible.
  **Typed as adjudication-time evidence and not a receipt** (FT-A28(v)), and it is
  the first time in three chapters that this particular check has been available
  when wanted.
  (ii) **The `t_0` caveat is real, and I have checked that it breaks neither
  proposition.** If two worlds' kinks cancel at one `σ`, that `σ` is a **candidate**
  breakpoint enumerated by Lemma FF-min but not a kink of `G_I`, so `t_0` may be
  **smaller** than the first true kink. **FC-drop(c) is unaffected:** its proof needs
  only that `G_I` be affine on `[0, t_0]`, which holds a fortiori when `t_0` falls
  short of a kink, so the bound stays proved and merely weaker — **never larger.**
  **FC-tight is unaffected as stated**, and deliberately so: its criterion is *"the
  slope becomes non-negative immediately after `t_0`"*, and a non-kink leaves the
  slope **unchanged and therefore still negative**, so the criterion is false and
  the bound is strict there. Both formulations of the `iff` survive. **What the
  caveat does add is a second, independent conservatism in the bound** — one from
  the descent running past `t_0`, one from `t_0` possibly falling short of the
  kink — and a reader who assumed `t_0` was the first true kink would over-read the
  instrument. The artifact prints it; it belongs in any citation of the 14.873%.
  (iii) **The convention typing is RATIFIED and it is load-bearing, not
  bookkeeping.** `s^±` are **convention-free**: no value enters them, `q` being read
  only to decide which `b` lie in the argmax, and **argmax membership is invariant
  under `v ↦ αv + c` with `α > 0`** (freeze 37(c)). `t_0`, the bound and the frozen
  `κ_I` are **tax columns in the count convention**, each scaling by `α`. **Had
  `s^±` been treated as a tax column and halved, the bound would have been wrong by
  a factor of two and (FC-R4) would have compared a differential quantity against a
  count one** — Corollary SR-conv's void case, arriving in a new place. That the
  artifact states this per cell rather than once in the header is what makes
  (FC-R4) a comparison rather than a category error.
  (iv) **The declined cross-check was typed against the builder's own interest and
  that is the notable part.** The probe enumerates 3,126 breakpoints at each h2 unit
  over the 216 swept states at F2, **equal to the v1.1 filed value** — an agreement
  it would have been flattering to call verification. It declined twice: it did not
  wire it as a receipt (FF-A32(v), a build does not originate one), and it typed it
  under FC-A15(iii) as **same predicate, different program — reproducibility across
  implementations, not an independent predicate.** **FC-A15 was filed hours earlier
  and its first application was by the party it cost something.**
  (v) **The steward's own predicate failure is the sharpest instance of FC-A15 yet
  and is recorded because of how it failed.** Checking the h2 sign pattern with
  `grep -c "s^+ = -"` — where `^` is a regex anchor — matched nothing and returned
  a clean, confident **zero violations**. **A wrong predicate returned exactly the
  answer being hoped for.** That is the mechanism that makes FC-A15 more than
  hygiene: a shared-method re-check is dangerous in proportion to how much the
  checker wants the result, and a null-matching predicate is indistinguishable from
  a clean result **unless something independent makes the clean result implausible.**
  It was caught by implausibility across four simultaneous queries, not by suspicion
  of the predicate. **Adopted as the operational tell: a check that returns exactly
  the hoped-for answer with no exceptions deserves one more query by a different
  route before it is believed.**
  (vi) **THE PHRASING CONSTRAINT — "ATTAINED", NEVER "EXACT" — and it corrects my
  own text first.** The builder surfaced that *"the bound is exact at 258 states"*
  is wrong in a way that matters, declined to fix it in the artifact (FF-A32(v): a
  reading-rule originates in a ruling), and the steward routed it here because the
  librarian writes the era page from these rulings. **It is right, and it lands on
  me: FC-A18(ii) says the instrument "is *exact* where the fee's optimum sits at the
  first breakpoint", and Proposition FC-tight's own title says "the drop bound is
  exact exactly when…". Both are corrected in place, and the erroneous wording stays
  visible per LD-A11(ii).**
  **The rule.** Proposition FC-drop(c) is a **lower bound at every state**, the 258
  included. At those 258 the bound is **ATTAINED** — it equals the frozen `κ_I`
  there. **It is never *exact* as a property of the functional anywhere**, and
  "exact at 258 states" invites the reading that the screen predicts capture a fifth
  of the time, which is precisely what FC-A9(ii)'s *"`s^±` does not predict
  capture"* exists to prevent.
  **And the decisive clause, which neither the builder nor the steward stated:
  attainment is not identifiable in advance.** Nothing in the emitted data tells you
  *which* 258 without computing `κ_I` — **the very quantity the bound exists to
  avoid computing.** So the 258 is a fact about the distribution of the gap and
  **never a usable property of the instrument**: a screen that is tight at a fifth
  of states you cannot pick out is, in use, exactly as weak as its aggregate 14.873%
  and no weaker. **Binding on the era page, on the close digest, and on every future
  citation:** say *attained at 258 of 1,252*, say *a lower bound everywhere*, and
  say *which states attain it is not knowable without `κ_I`*. FC-A18's substantive
  ruling — triage instrument, poor estimator — is unchanged and is what those three
  sentences say precisely.

- **FC-A23 ("usable triage instrument" does NOT survive unqualified — replaced
  with something stronger; the librarian's modality catch is ratified and applies
  to MY OWN closing note; and the seventh read-to-the-end instance is mine to
  own).** Five clauses.
  (i) **RULING: "usable" goes, and what replaces it says more.** The steward is
  right that the phrase is the one most likely to be excerpted away from its
  escort, and it fails for the same reason "exact" did — **true under one reading,
  inviting a stronger one, and silent about which.** The precise fact is that the
  bound is **one-sided**:
  **a positive bound PROVES a fee bites at that state; a zero or small bound proves
  NOTHING.** No false positives, unbounded false negatives. **That is what may be
  said, and it is both shorter and stronger than "usable triage instrument"** — it
  survives excerpting because it states what follows in each direction instead of
  grading the instrument on a scale the reader supplies. **Binding on the era page,
  the LOG and the close digest**; FC-A18(ii)'s substantive point — that screening
  and estimating are different jobs — **stands unchanged**, and this is the sentence
  that says it without inviting the over-read.
  (ii) **The general tell, now the third instance in two rulings, stated so the
  next one is caught before it is written.** *"Exact"*, *"usable"*, and their
  siblings *"tight"* and *"strong"* are **adjectives that grade an instrument
  without naming a direction or a use.** Every one of them is excerptable into a
  claim nobody made. **The house form: do not grade an instrument — state what
  follows from a positive reading and what follows from a negative one.** A sentence
  built that way cannot be excerpted into something stronger than itself, which is
  the only durable protection, since escorts do not travel and sentences do.
  (iii) **The librarian's modality catch is RATIFIED, it was the most important of
  the five, and it lands on my own text too.** *"No fee **can** bite"* against
  *"can be **expected** to bite"* is a change of modality, not of emphasis:
  **Proposition FC-width gives a positive-width subgradient, which makes zero
  capture ROBUST — it does not make positive capture IMPOSSIBLE.** A feature whose
  mean slope exceeded the half-width would still bite despite widespread ties;
  nothing forbids it. The hedge is correct and the restoration is right.
  **And I must correct myself in the same breath:** FC-A21's closing paragraph says
  the fee route **"is structurally blocked"** wherever the clairvoyant face is widely
  non-singleton. **"Blocked" carries the same impossibility the LOG drifted into
  and is corrected in place to "is not to be expected to bite, robustly so"**, with
  the erroneous wording left visible per LD-A11(ii). The librarian caught in a
  downstream page the exact overstatement I had committed upstream, which is the
  best possible argument for a librarian that reads rather than transcribes.
  (iv) **The five self-caught defects are commended, and the taxonomy is worth
  keeping**: one modality drift (iii), one rendering break that would have detached
  freeze 53 from its table, one over-read of FC-A22(vi) — *"identifies those 258
  exactly"* rewritten to *"says what those 258 are"*, which is the same trap one
  layer down — and **three figures naming state and unit but not feature**, where
  the straddle census reads 0 / 374 / 322 / 1,252 across four features over the
  *same* 1,332 states. **That last is precisely the ambiguity the generalised FF-A18
  exists to close, found by the rule rather than by luck**, and it is the first time
  in this chapter a scope defect was caught by applying the rule prospectively
  rather than by someone noticing an inconsistency afterwards.
  (v) **The seventh read-to-the-end instance is mine, and the fix is a freeze I
  declared for one phase and not the other.** I froze the ruling range for the
  **build** phase deliberately (FC-A15's declaration) after the steward pointed out
  that a moving target made a one-time confirmation behave like a running
  obligation. **I then appended FC-A22 while the librarian was mid-read — the very
  ruling binding the phrasing that page had to get right.** The steward's brief was
  accurate when written and stale before it was finished, and that is my doing, not
  the brief's.
  **The rule I adopt for myself: the ruling range is frozen at chapter close,
  before the write-up phase is briefed, exactly as it is frozen before the build
  phase; and any post-freeze ruling requires an explicit re-brief rather than
  relying on the reader to notice.** The steward is right that seven instances at
  zero cost is the shape of a hazard that will eventually cost something — and
  **this instance came closest**, because the late ruling was the one that governed
  the words. **The severity is rising even though the cost has not**, since the
  rulings arriving late are increasingly the ones that bind phrasing rather than
  computation. **RANGE FROZEN: FC-A1..FC-A23, and the chapter is closed at it.**

---

## The seed survey: a hundred fresh coordinates, designed (2026-08-15)

**Adjudicator:** walt-math-11. **Object:** the design of a 100-seed, 400-unit
survey of grade-4 coordinates reached by construction, commissioned by Jason
tonight — *"so we need some more full solves. we have a night to do it in."*
**Tier:** exploratory throughout, below every tier. **Basis:** freeze 7/23
(fiber enumeration), 26 (observation contract and the least-domino-index dumb
policy), 37(d) (belief), 38 v1.1 (the gluing cut and its ordering), 44 v2 (walk
budgets), 45 (the n = 4 coordinate identity), 46/49 (the rule arms), 50 v1.1,
52 through v1.4, 53; Corollary E4.1, Theorem E6.4, Corollary FT-grade4,
Corollary 5.2, Proposition FT-tie, Proposition SR-degen, Propositions FF-blind
/ FF-degen / FF-oracle, Propositions FC-drop / FC-width / FC-tight, and the
scope, receipt-versus-audit and predicate disciplines of FF-A18 as generalised,
FC-A12, FC-A14 and FC-A15. Rulings **SS-A1..SS-A9**; **freeze 54** fixed at
SS-A4. The prefix `SS-` and every name below were grep-checked unused.

**One defect in the design as handed to me, found before any code was written,
and it would have voided the survey.** The generator was specified as *"seed `n`
→ deal `n` via the canonical enumeration."* Taken literally that is fatal. The
deal space has

  `D = C(28,7) · C(21,7) · C(14,7) = 472,518,347,558,400`

members under the standard mixed-radix unranking, and the **first seat's hand
does not change until index `C(21,7) · C(14,7) = 399,072,960`.** Seeds `0..99`
would therefore have produced **one hundred deals sharing an identical first
hand** — a survey of one deal, sampled a hundred times. (That the blocking
constant is exactly the trick-1 world count we have been quoting all chapter is
a coincidence of the same combinatorics, and a pleasing one.) The repair is at
SS-A2 and it is the only change of substance I have made to Jason's design.

---

- **SS-A1 (typing, tier, and what this survey is and is not).** **GRANTED as the
  SS family.** Everything is exploratory, cited by nothing above this tier,
  quotable as a result only by brief amendment adding it to a verifier receipt.
  DS-A1 binds: **witness**, **receipt**, **necessary outer profile**, never the
  forbidden word. Both outcomes of every gate are results (F7); a receipt failure
  is stop-and-report, never a patch (NO-RESCUE).
  **What it is.** The first carrier in this branch that is **not selected by
  outcome.** Every previous n = 4 carrier was chosen by negative binding margin,
  and SR-A25(iii)'s selection fence has had to travel with every number because
  of it. **These hundred coordinates are selected by a declared arithmetic map
  from the natural numbers and by nothing else**, and every root action is a unit,
  so neither the coordinate nor the action is chosen by result. **That is the
  survey's whole methodological point** and it is worth more than any single
  number it will produce.
  **What it is not.** It is **not a fee measurement.** Tonight measures whether
  tie multiplicity tracks separation structure across fresh coordinates; whether
  it tracks *fee capture* needs fees, which is a later run. **No sentence in the
  artifact may say the survey tested fee viability.** It is also not a
  distribution over 42: a hundred deals under one declared map is a **carrier**,
  and P-A21 binds — **nothing measured at grade 4 is quoted for trick 1 or for the
  opening.**
- **SS-A2 (the generator: the unranking is REPAIRED with a declared spreading
  map; the map is a constant and a stated rule, both asserted).** Five clauses.
  (i) **The repair.** The seed does not index the deal directly. It indexes it
  through a fixed multiplier:
  **`index(n) = (n · A) mod D`**, with **`A = 292,032,399,099,041`** and
  **`D = 472,518,347,558,400`**.
  (ii) **Why that `A`, declared before any result and never chosen by one.** `A`
  is the least prime at or above `D/φ`, the golden-ratio multiplier, which is the
  standard low-discrepancy choice and spreads consecutive seeds maximally rather
  than adjacently. **`D`'s prime factorisation contains only primes ≤ 23**, being
  a product of integers ≤ 28, so **every prime greater than 28 is automatically
  coprime to `D`** and `A` is a bijection on the index space. Verified at
  adjudication time: over seeds `0..99` the map yields **100 distinct deal
  indices** and **100 distinct first-hand ranks spanning 0 to 1,174,413 of
  1,184,040** — essentially the whole range, against the one value the unrepaired
  design would have given.
  (iii) **Any fixed spreading map is arbitrary and that is not a defect; choosing
  one after seeing results would be.** `A` is frozen here, before the build, and
  a successor changing it is running a different survey and files it as one.
  (iv) **The unranking itself is the standard mixed radix** — `index` split by
  division into `(r₀, r₁, r₂)` over the radices `C(21,7)·C(14,7)`, `C(14,7)`, `1`,
  then three combinadic unrankings giving seats 0, 1, 2 their hands and seat 3 the
  remainder. **It reuses the existing `unrank_comb` rather than a fresh
  implementation**, which is the FC-A11(ii) rule: mirror the receipted path, do not
  re-derive it.
  (v) **The rest of the generator, fixed as constants.** Declaration
  `PipTrump(n mod 7)`. Seat 0 leads trick 1. The frozen dumb policy of freeze 26 —
  least legal domino index — plays **three complete tricks**, twelve tiles, leaving
  four in every hand. **The focal seat is the winner of trick 3**, which is the
  seat on lead at the coordinate and makes leader offset 0 automatically, matching
  freeze 45 rather than restating it.
- **SS-A3 (units, and the collision question).** **Every legal root action of the
  focal seat is a unit.** The focal seat leads at the coordinate, so its legal set
  is its whole hand and **`|A| = 4` exactly at every coordinate — asserted in-run,
  not assumed**, which is contentful and fails if the coordinate is malformed. So
  **400 units, 4 per seed, no pair selection anywhere.**
  **Collisions are filed as-is, not deduplicated**, and the coordinator's instinct
  is right: collision frequency is data about the generator, and deduplication
  would silently make the unit count depend on the results. **Each seed emits a
  canonical coordinate key** — declaration, focal hand, pool, as ascending
  domino-index tile lists in freeze-45 form — so collisions are computable after
  the fact by anyone, from the committed summary alone.
- **SS-A4 (FREEZE 54 — the seed-survey carrier and its measured objects).**
  **(a) The carrier, as a generating rule with every constant asserted** (FT-A23's
  discipline: a freeze states a constant **or** a rule, never both unasserted —
  here the rule is the content and its constants are named): seeds `0..99`
  inclusive; `index(n) = (n·A) mod D` with `A` and `D` as at SS-A2(i);
  mixed-radix combinadic unranking; `PipTrump(n mod 7)`; seat 0 leads; freeze-26
  dumb policy, three tricks; focal seat = trick-3 winner; every legal root action a
  unit. **The 100 resulting coordinate identities are a function of that rule and
  are not separately enumerated** — the rule generates them and the artifact prints
  each in freeze-45 form.
  **(b) Measured per unit, all exact rationals in the count convention, no float
  anywhere** (P-A19): **(M1)** `Q^H(b)`, the full lawful solve. **(M2)** the
  complete `H`-argmax set at the coordinate and the exact margin of `b` against
  the best competitor, with the verdict cell UNIQUE-OPTIMAL / TIED-OPTIMAL /
  DOMINATED — **walt's separation sense, an exact separation of one action from
  every competitor, and never D3's sense**, printed with that sentence attached.
  **(M3)** `U^C(b)`, the revealed value, and the fusion gap `U^C(b) − Q^H(b)`.
  **(M4)** the depth-one frontier census — `|I₁|`, arrivals, the `|A(I)|`
  distribution including the forced count, and the **tie-multiplicity statistic**:
  the number of `(state, world)` arrivals whose **complete** clairvoyant argmax is
  non-singleton, over the total. **(M5)** `Δ^(1) = Σ_I δ_I`, hence
  `Δ^(2) = fusion gap − Δ^(1)` by Corollary FT-grade4, and `#{I : δ_I > 0}`, the
  tax support size. **(M6)** `policy_value_by_rule` for the four frozen rules of
  freezes 46/49 and each one's gap to `Q^H(b)`. **(M7)** the count-only partition
  pass — exact extraction-map state count and FNV-128 digest, `O(1)` memory —
  against `P_max v2 = 192,000,000`.
  **(c) What `P_max` gates, stated precisely because it is the h9 typing
  generalised.** An over-threshold count bars **the primal-witness pipeline only**
  and is recorded as **NOT PRICED** on that route, never attempted. **It does not
  bar (M1) through (M6)**, which need the depth-one frontier and the revealed
  continuations below it and no extraction map at all. A NOT PRICED unit still
  carries its solve, its tie census and its taxes, and every such row says so in
  place.
  **(d) Emission, cut by content** (FT-A24, freeze 50 v1.1(c)): a **thin committed
  summary, one row per unit — 400 rows, committed entire**; fat companions
  gitignored under pinned SHA-256 with byte and line counts in the committed
  header, carrying the per-seed deal, the twelve played tiles, and the per-frontier
  rows. **Accounting integers per unit make the omission auditable.**
  **(e) Checkpointed in blocks** (DS-A36): results assembled in canonical unit
  order and never completion order; a block is durable when complete; **the morning
  has whatever completed and nothing partial.**
  **(f) Belief and field are NOT re-declared** — freeze 26 and 37(d), no decimation
  ((C2)). **No library entry at any coordinate** (freeze 45). The freeze-set digest
  travels on every record.
- **SS-A5 (budgets and stops: what may gate a schedule, and what may never gate a
  value).** Three clauses.
  (i) **Freeze 44(b) v2 binds unchanged** — `B = 10^10` walk-steps per
  (coordinate, action) per evaluator, charge-then-descend, `Option` return, **and
  on exhaustion no partial fold of any kind**: no partial tax, no partial census,
  no partial solve. No new constant is fixed here.
  (ii) **Wall-clock is the run owner's to declare and is provenance, never a
  receipt** (N4-A13, SEP-A19(b)). The run owner may declare a per-unit `T_pass` and
  a per-block `M_budget` tonight. **A wall-clock stop terminates a unit and files
  it as a DECLARED STOP; it never truncates a value.** A stopped unit contributes
  no number to any census, ratio or aggregate, and every aggregate names the unit
  set it ranges over (FF-A18 as generalised) so a stop cannot silently shrink a
  denominator.
  (iii) **Heavy tails become verdicts.** A unit that stops is a filed outcome under
  F7, printed as a stop and never as a finding (R-A18) — and the count of stops,
  with their seeds, is itself a recorded measurement.
- **SS-A6 (the receipts — nine, with the non-receipts named).**
  (i) **(SS-R1) GENERATOR SOUNDNESS — BLOCKING, before any unit runs.** Per seed:
  the four hands partition all 28 dominoes — disjoint, seven each, union complete;
  every tile of the playout was **legal at the moment it was played** against
  `legal_plays`; exactly twelve tiles played; exactly four remain in every hand;
  and the trick-3 winner is the focal seat. **Contentful and it is the check that
  catches an unranking error**, which is otherwise invisible because a wrong deal
  is still a well-formed deal.
  (ii) **(SS-R2) THE SPREADING RECEIPT — BLOCKING.** Assert `gcd(A, D) = 1` and
  that the 100 indices are **pairwise distinct**. **Contentful**: it fails on a
  mistyped `A`, and a mistyped `A` is exactly the defect SS-A2 exists to prevent
  recurring silently.
  (iii) **(SS-R3) GENERATOR DETERMINISM.** Recompute each seed's coordinate a
  second time from the seed alone with fresh state and assert the freeze-45
  identity tuple byte-identical. **Contentful across the whole survey**, and cheap.
  (iv) **(SS-R4) COORDINATE IDENTITY.** Freeze 45's form asserted at every
  coordinate, `|X| = 34,650` against `kernel.count()`, kernel rebuilt in-run and
  asserted equal, `|A| = 4` asserted at the root.
  (v) **(SS-R5) THE LADDER RECEIPT.** Per unit assert
  `U^C(b) − Q^H(b) = Δ^(1)(b) + Δ^(2)(b)` with `Δ^(1)` summed from the frontier
  table and `U^C`, `Q^H` from their own solves. **Contentful**: three quantities
  from different passes, tied by Corollary FT-grade4, and it fails on any error in
  the frontier decomposition.
  (vi) **(SS-R6) THE COMPLETE-FACE RECEIPT — the one that guards the headline
  statistic.** Assert Corollary 5.2 **both ways at every frontier state**: where
  `δ_I = 0` the complete per-world argmax sets intersect, where `δ_I > 0` they do
  not. **A collapsed face is caught here loudly**, because collapsing to
  singletons makes the `δ_I = 0` states report empty intersections almost
  everywhere. The construction must accumulate a **set by equality across all
  candidates and track no index** — `max_by_key` returning one index **is** the
  defect (FC-A11(ii)). **Plus the run-level non-null pairing** (FC-A26(iv)'s
  discipline, required by design rather than by luck): assert that the
  non-singleton arrival count is **positive somewhere in the survey**, since a
  stuck-at-singleton implementation would otherwise report a tie multiplicity of
  zero everywhere and satisfy every other check.
  (vii) **(SS-R7) THE RULE BAR.** Assert `policy_value_by_rule ≤ Q^H(b)` for every
  rule and unit. **Contentful**: a lawful rule policy cannot beat the lawful
  optimum, so a failure means the rule was evaluated against the wrong field,
  belief or convention.
  (viii) **(SS-R8) DETERMINISM SAMPLE.** A full in-run second pass with fresh maps,
  accumulators and budgets on a **declared sample — the first unit of every block —**
  every printed figure asserted identical. Declared rather than universal because
  at 400 units a universal second pass doubles the night; **(SS-R3) covers the
  generator at every seed regardless.**
  (ix) **(SS-R9) SCOPE.** Every emitted figure names **every dimension it ranges
  over** — unit set, state set, seed set — in the same sentence, and a scope
  derived from an adjective is not a scope (FF-A18 as generalised at FC-A10(iv),
  FC-A14(ii)).
  (x) **NAMED AS NON-RECEIPTS and printed as arithmetic remarks** (Proposition
  SR-taut): `δ_I ≥ 0`; `Δ^(1) ≥ 0`; the fusion gap `≥ 0`; and the tie fraction
  lying in `[0,1]`. They cannot fail.
- **SS-A7 (all outcomes pre-declared, before any number exists; F7 binds).**
  (a) **(SS-R1) or (SS-R2) fails** → stop-and-report before any unit runs; the
  generator is wrong and no coordinate is trustworthy. (b) **Tie multiplicity
  tracks separation structure** — units with high multiplicity systematically show
  smaller margins, more ties in `Opt^H`, or larger `Δ^(2)` share → the screening
  statistic generalises off its home carrier, and it becomes the selection variable
  for fee work rather than a two-coordinate observation. (c) **It does not track
  them** → **Proposition FC-width's statistic is carrier-local**, which is a result
  and a sharp one: it would mean the h2/h0 contrast was driven by something
  co-varying with multiplicity rather than by multiplicity, and the fee programme
  loses its cheap screen. **This is the more informative outcome and it is not the
  one we expect**, which is exactly why it is written down now. (d) **The relation
  is present but weak or non-monotone** → reported as measured with no mechanism
  claimed. (e) **Tax sparsity off-carrier** — the fraction of frontier states with
  `δ_I > 0` at 400 unselected units against the 4.49% measured at five
  margin-selected ones → the first out-of-carrier reading of that number, filed
  either way, **and never quoted for trick 1** (P-A21). (f) **Stops** → declared,
  counted with their seeds, no partial anything. (g) **Collisions** → counted and
  filed; a high collision rate is a fact about the dumb policy's funnelling and is
  reported as one, not as a defect.
- **SS-A8 (what I would have regretted not measuring, since my own letter asked
  that question of someone else).** Four, all cheap, all in freeze 54(b) above.
  **The fusion gap and its two-rung split** — every tax this branch prices is a
  fraction of it, and without it tonight's numbers are incomparable to every number
  we already have. **The tax support size** — it makes the 4.49% sparsity figure
  testable off-carrier for the first time, at four hundred units instead of nine.
  **The `|A(I)|` distribution including forced states** — decision-deadness at the
  frontier is free from the same pass and nothing else will ever be cheaper.
  **The canonical coordinate key per seed** — without it collisions are
  uncomputable afterwards and the generator's funnelling is invisible forever.
  **And one deliberate omission:** no per-`(state, world)` rows are retained. At
  four hundred units that is hundreds of millions of rows for a fee pass we have
  not designed. The aggregates above are chosen so a **later** fee pass can *select
  its coordinates* without re-solving — which is the honest scope: **the survey is
  a selection instrument for fee work, not a fee measurement.**
- **SS-A9 (fences, and what is owed).** Every standing fence travels verbatim:
  the R-A2/P-A1 fence; **the N4-A8 real-deal fence in its amended form — these
  hands do not come from rob's receipt corpus at all but from a declared arithmetic
  map, so they are FEASIBLE constructions and not deals anyone played**, and no row
  is a statement about correct play in any hand; **P-A21**, no grade-4 quantity
  quoted for trick 1 or the opening; **Proposition SR-degen**, no verdict at grade
  4 turns on any relaxation here; and SR-A25(vii)'s implementation-versus-corpus
  risk undiminished, with T1-A12's check still owed. **Not claimed:** nothing about
  points or marks; nothing about bidding; nothing about how real opponents play; no
  cost or tractability claim read off any traversal observable. **Owed to the wiki
  owner at chapter close:** freeze 54 and the SS era-page and LOG entries; per
  SR-A37(i) that is the whole list.
  **RANGE FROZEN: SS-A1..SS-A9, and freeze 54.** Per FC-A23(v) the range is frozen
  **before** the build brief is issued; any later ruling requires an explicit
  re-brief rather than relying on the builder to notice.

- **SS-A10 (the tie statistic's aggregation convention: BOTH are reported, the
  per-unit mean is primary on a stated principle, and an association must survive
  BOTH or the disagreement is itself the finding. Ruled before any survey total
  exists.)** Six clauses.
  (i) **The gap is mine.** SS-A4(b)(M4) fixed the per-unit statistic and left the
  cross-unit convention unstated, and the two readings can point opposite ways.
  Verified at adjudication time on the seed-5 smoke figures: arrival-pooled
  `918336/3099816 = 38264/129159`, against an unweighted per-unit mean near two
  fifths. **The divergence is structural, not noise:** `|I₁|` spans **480 to
  37,584 across the four actions of one coordinate** — a factor of **78** — and
  arrivals span 268,800 to 976,416.
  (ii) **RULING: emit both; the per-unit mean is PRIMARY.** The reason is what the
  statistic is *for*, not which way seed 5 points. **SS-A7(b) asks whether
  multiplicity tracks separation structure, and separation structure is carried by
  the unit** — the verdict cell is per-unit, and a fee, if one is ever built, is
  built at a unit. **The unit is therefore the observational unit and units are
  weighted equally.** Arrival-pooling answers a different question — the chance
  that a randomly drawn arrival from the pooled survey is non-singleton — which
  weights units by frontier size, and frontier size is not what SS-A7(b) is about.
  (iii) **The usual argument for pooling does not apply here, and it is worth
  saying why.** One normally weights by `n` because a larger sample is a more
  reliable estimate. **There is no sampling error in this survey.** Every per-unit
  fraction is an exact rational over that unit's complete arrival set. Reliability
  is not in question at any unit, so the only thing a weighting choice does is
  choose the population being averaged over. That makes the choice purely a
  question of what is being asked, which is what (ii) answers.
  (iv) **THE BINDING CLAUSE, and it is why both are emitted.** **An SS-A7(b)
  association is reportable only if it holds under BOTH conventions.** If the two
  disagree, **the disagreement is the finding and is reported as such**, never
  resolved by preferring one. The reasoning: a mechanism-driven association should
  survive reweighting; **one that appears under a unit weighting and vanishes under
  an arrival weighting is telling you that frontier size, not tie multiplicity, is
  the thing associated.** That is a real and different result, and a valuable one.
  (v) **The confound seed 5 already exhibits must be reported alongside, and this
  is the clause I would least want added after results.** At seed 5 the
  unique-optimal unit has **both the smallest frontier and the highest tie
  fraction** — `4749/5600` against roughly a third at the three dominated units.
  Those two explanators are **collinear at that unit**, and at `n = 1` nothing
  separates them. **Binding: every SS-A7(b)/(c) reading reports the multiplicity
  association and the frontier-size association side by side, over the same unit
  set, and states explicitly whether the survey separates them.** If `|I₁|` and the
  tie fraction are broadly collinear across 400 units, **the survey cannot
  attribute and must say so** rather than crediting the variable we came in
  believing.
  (vi) **Why this could only be ruled now.** Seed 5's smoke numbers are visible and
  they show the direction — the per-unit reading favours the association we expect
  and pooling suppresses it. **Choosing per-unit on its merits while that is known
  is exactly the move pre-declaration exists to prevent**, so the merits are stated
  in (ii)–(iii) independently of seed 5, and (iv) removes the incentive entirely by
  requiring both. Nothing here is re-runnable-dependent: the committed file carries
  every numerator and denominator, so both conventions and the confound check are
  computable from it without touching the run.
- **SS-A11 ((SS-R5) is amended: my stated content was tautological and the
  builder's placement is RATIFIED).** Three clauses.
  (i) **The defect is mine and it is Proposition SR-taut again.** SS-A6(v) called
  the ladder receipt contentful because "three quantities from different passes".
  **With `Δ² := gap − Δ¹` the identity `U^C − Q^H = Δ¹ + Δ²` is an identity in the
  probe's own recomputed quantities and cannot fail.** I wrote a receipt whose
  stated content was an arithmetic remark, in a ruling that names the non-receipts
  two clauses later. That is the fifth instance of this family and the second of
  mine in as many chapters.
  (ii) **The builder's placement is correct and is where the content actually
  lives:** assert at every unit that the **frontier table's own `U^(0)` equals the
  revealed solve's `U^C`** — two passes, different intermediate quantities — and on
  the declared sample strengthen it by having PATH B compute `U^(1)` independently.
  **That is a comparison against something the checker did not produce**, which is
  the SR-taut test, and it fails on any error in the frontier decomposition.
  (iii) **SS-A6(v) is amended in place to say exactly that**, the erroneous
  justification staying visible per LD-A11(ii). **The tautological form is retained
  and printed, but as an arithmetic remark under SS-A6(x), never counted among
  receipts HELD.**
- **SS-A12 (the two smaller readings: both RATIFIED as read).** Two clauses.
  (i) **`T_pass` interrupts unit-own passes only; the shared coordinate solve is
  timed, recorded and never stopped. RATIFIED, and the builder's reason is better
  than my clause.** A timer on a shared solve could only fire *after* the solve
  completed, and would then discard a correct value — which is not a stop, it is
  destroying evidence already paid for. **And the safety it might seem to give up
  is already provided: freeze 44(b) v2's walk budget bounds the solve hard**, with
  the no-partial-fold path on exhaustion, so a pathological seed terminates as a
  declared stop regardless of any wall clock. `T_pass` is therefore a
  **schedule-level** instrument sitting above a hard bound, exactly as SS-A5(ii)
  intended when it said wall-clock never truncates a value.
  (ii) **(SS-R8)'s object is ONE UNIT — the coordinate solve plus that unit alone
  re-run — not the whole seed. CONFIRMED.** That is what "first unit of every
  block" meant and it is the cheaper reading, which is correct for a declared
  sample. Its scope: it reaches accumulator reuse and iteration-order dependence
  within a unit's own path; **(SS-R3) covers the generator at every seed
  regardless**, and per-unit content is a function of (kernel, budgets) alone.
  **Scope stated rather than implied**, so nobody reads the sample as covering more
  than it does.
- **SS-A13 (the seed-5 observation, typed; and the range re-frozen).** Two clauses.
  (i) **The builder's typing is correct and is adopted: observation, not finding,
  `n = 1`, no mechanism claimed.** It is SS-A7(b)-shaped and it is exactly one
  coordinate. What makes it worth recording is not the direction but that it
  **exposed the aggregation gap before any total existed** — an artifact of the
  smoke run doing its job. **It must not appear in any results header as evidence**,
  and if the survey's 400 units contradict it that is unremarkable rather than
  surprising.
  (ii) **RANGE RE-FROZEN: SS-A1..SS-A13, freeze 54 unchanged.** Per FC-A23(v) this
  is an explicit extension of a frozen range and requires a verbatim re-brief to
  the builder rather than reliance on it noticing; the coordinator has undertaken
  that relay. **No freeze is amended and no measured object is added** — SS-A10
  fixes a reporting convention over quantities freeze 54(b) already commissions,
  and every figure it requires is computable from the committed summary without
  re-running anything.

- **SS-A14 (the confound-table design: RATIFIED with three amendments, one of
  which prevents a defect that would corrupt the carrier. Ruled before any total
  exists.)** Six clauses.
  (i) **The design is right and its restraint is the best part.** Verdict-cell rows
  carrying both conventions side by side; `|I₁|` quartile strata from the survey's
  own order statistics; the exact 2×2; and **a separability sentence driven by a
  rule stated in the file and applied mechanically, with no invented inferential
  test.** Stratifying on `|I₁|` and comparing verdict cells *within* strata is the
  correct instrument for SS-A10(v) — it holds the rival explanator approximately
  fixed instead of modelling it — and declining to manufacture a significance
  statistic is exactly right for a survey whose every figure is an exact count over
  a complete set. **RATIFIED**, with three amendments, all minimal and all in the
  same spirit of tables plus a stated rule.
  (ii) **AMENDMENT 1 — the separability rule as stated can pass on a single
  unit.** *"If no `|I₁|` stratum contains units from two or more verdict cells →
  CANNOT ATTRIBUTE"* is satisfied by a stratum holding 99 dominated units and one
  unique-optimal unit, which would let an attribution claim rest on an `n` of one.
  **Binding: every within-stratum contrast prints its per-stratum per-cell `n`, and
  the separability sentence names the smallest cell count any part of it rests
  on.** No threshold and no test — a threshold would be the invented inference the
  design rightly refuses. **Just print the `n` and name the weakest link**, so a
  reader sees immediately whether a contrast is carried by a hundred units or by
  one.
  (iii) **AMENDMENT 2 — print all four cells of the 2×2, not only the concordance
  count.** The direction of the association is the informative part and a single
  concordance integer hides it. Seed 5 hints at *anti*-correlation — small frontier
  with high tie fraction — and if that is the survey's pattern the concordance
  count alone reads as weak association when the truth is a strong association of
  the opposite sign. **Four integers, each labelled with its half-plane.**
  (iv) **AMENDMENT 3, and this one prevents a defect rather than sharpening a
  reading. "SS-A13(i) seed-5 exclusion" is ambiguous and one reading of it would
  corrupt the carrier.** SS-A13(i) barred the pre-run seed-5 **observation** from
  appearing as evidence. It did **not** license removing seed 5's units from the
  survey. **Seed 5's four units are in the carrier, in every total, in every
  stratum and in every verdict cell, exactly like the units of every other seed** —
  the carrier is defined by the freeze-54 generating rule and removing a seed
  *because we looked at it early* would be precisely the selection-by-result that
  SS-A1 says this survey exists to avoid. **What is excluded is one sentence, not
  four units.** Binding, and the artifact says so in place so no successor
  re-litigates it.
  (v) **The BigRational decision is RATIFIED and was flagged the right way.** Four
  hundred exact fractions over denominators near `10^6` have a least common
  denominator no `i128` can hold; `BigRational` is the existing dependency with the
  `fc_correlation` precedent; **the primary figure stays exact and the ppm bracket
  is presentation-only, entering no proof and no comparison.** Raising it before a
  stack trace rather than after is the same instinct as the SHA-256 self-check two
  chapters ago, and it is worth noting that the builder has now twice reported an
  arithmetic hazard while the choice was still cheap.
  (vi) **One free companion, offered and not required.** The **median of the 400
  per-unit fractions** needs only comparisons — no common denominator, no
  `BigRational` arithmetic — and it is robust to a handful of extreme units driving
  a mean. It is **computable after the fact from the committed summary**, since
  every numerator and denominator is there, so **it need not be in tonight's run at
  all.** If the mean and the median disagree materially in the reading, that
  disagreement is worth a sentence, on the same principle as SS-A10(iv): a summary
  that changes with the summarising choice is telling you about the distribution
  rather than about the variable.
  **RANGE RE-FROZEN: SS-A1..SS-A14**, freeze 54 unchanged, no measured object
  added, nothing re-runnable-dependent. Verbatim re-brief owed per FC-A23(v).

- **SS-A15 (the survey's reading: SS-A10(iv) fired, the association is NOT
  reportable, and the disagreement diagnoses to a structural fact nobody
  pre-declared).** Seven clauses, in the relaxed register — this is filed because
  it will be quoted, not because the run needed ceremony.
  (i) **SS-A10(iv) fired on its first live test and the builder applied it
  correctly.** The two conventions agree on the survey total (per-unit 450,298 ppm,
  pooled 463,222 ppm) and **reverse the verdict-cell ordering**: per-unit puts
  TIED-OPTIMAL first (484,114 > 448,365 > 447,315), pooled puts it last
  (469,362 > 465,742 > 422,608). **The SS-A7(b) association is therefore not
  reportable**, and the disagreement is the finding.
  (ii) **What the finding is, which is more than "they disagreed."** The two
  conventions differ in exactly one thing — how much they weight frontier size —
  so a reversal means the verdict-cell contrast is carried by frontier size rather
  than by tie multiplicity. **The stratum composition shows it directly: `Opt^H`
  units are 52 of 101 in the smallest-`|I₁|` quartile — 51.5% — against 23.0%,
  19.8% and 20.4% in Q2, Q3, Q4.** That is a **step at Q1, not a gradient**: 115
  optimal units against a uniform expectation near 29 per quartile, a 2.6× skew
  between the extremes. **Any statistic that varies with frontier size will appear
  to sort verdict cells at this carrier**, and the marginal 2×2 (108/91/91/110,
  concordant 218 against 200 expected) does not see it because the entanglement is
  *between cells and strata*, not marginal between the two variables.
  (iii) **The builder's SS-A7(d) reading is ADOPTED WITH A SHARPENING.**
  "Present but non-monotone" is defensible and under-diagnoses. The relation is not
  merely non-monotone — **it is confounded by a structural regularity of the game,
  and the survey's own declared rule already reaches CANNOT ATTRIBUTE without
  needing this diagnosis.** Filed as (d), with (ii) as the mechanism and no claim
  that the mechanism is established.
  (iv) **THE FINDING NOBODY PRE-DECLARED, and it is the survey's most interesting
  output: optimal root actions systematically produce SMALLER depth-one
  frontiers.** Half the smallest-frontier quartile is `Opt^H` against a fifth of
  every other quartile. **It is an observation and not a test** — it was not
  pre-declared, so it gets no F7 protection and no mechanism is claimed. **But it
  is on an unselected carrier, which is the one thing this survey was built to make
  possible**, and it is the first structural regularity this branch has measured on
  coordinates chosen by arithmetic rather than by outcome. A plain candidate
  reading, offered as speculation: an action that constrains the opponents' legal
  sets produces fewer distinct continuations *and* tends to be strong — leading
  trump forces follows. Untested.
  (v) **A free design note for whoever cuts this next.** Global `|I₁|` quartiles
  mix **within-coordinate** variation (the four actions at one coordinate differ in
  frontier size — seed 5's 480 against 37,584 was exactly that) with
  **between-coordinate** variation (whole coordinates differ). The within-coordinate
  rank of `|I₁|` is the sharper cut and would separate the two, and **it is
  computable from the committed summary at no cost and with no re-run.** Not owed;
  worth an hour.
  (vi) **Two ratifications and three filings, one line each.** The degenerate-split
  guard: **RATIFIED**, moot at 400 as expected, correct to keep in the format. The
  parallel (SS-R8) schedule change: **RATIFIED**, DS-A36 exercised and held.
  Sparsity off-carrier `1302799/20833948` = **6.25%**, against 4.49% at the five
  margin-selected units — **the selected carrier was sparser than typical**, which
  is the direction that makes the earlier number a mild under-read rather than an
  over-read. Forced frontier states `7559571/20833948` = **36.3%** — over a third
  of frontier states carry no decision at all, the first such measurement on an
  unselected carrier, and typed per J-A1 as its own column and never as a deadness
  count. Median of the per-unit fractions 425,217 ppm against the mean 450,298 —
  **right-skewed, a tail of high-tie units pulling the mean**, which is exactly the
  disagreement SS-A14(vi) suggested watching for and it is material.
  (vii) **What this does and does not say about Proposition FC-width.** FC-width is
  a theorem about the subgradient and is untouched. What loses support is the
  **screening chain**: multiplicity → separation difficulty → fee viability. **This
  survey breaks the first arrow at unselected coordinates.** The second arrow was
  never tested here (SS-A1 said so before the run) and still rests on two
  coordinates. **The h2/h0 fee contrast is not refuted by this** — it is simply not
  corroborated, and the cheap screen it promised is not available on this evidence.

- **SS-A16 (the two scratch cuts: (A) promotable and it has a use nobody has
  named yet; (B) not yet, one arbitrary choice is a live confound — and it
  corrects an error in my own count-pruning note).** Six clauses, relaxed
  register; filed because (A)'s synthesis will be quoted.
  (i) **(A) survives the sharper cut and strengthens.** Within-coordinate rank of
  `|I₁|` for the unique-optimal unit, 85 clean seeds: **45 / 22 / 11 / 7** from
  smallest frontier to largest, against a uniform 21.25. **Monotone, and rank 1 is
  2.12× expected while rank 4 is 0.33×.** SS-A15(iv) recorded this as a
  possibly-composite between-coordinate effect; **it is not — it holds at a fixed
  coordinate, among the four actions of one hand.**
  (ii) **The honest causal reading is weaker than the effect and should travel with
  it.** `|I₁|` almost certainly does not *cause* quality. Both are plausibly
  effects of a third thing — an action that constrains the field's legal sets
  produces fewer distinct continuations **and** tends to be strong, leading trump
  being the obvious case. **`|I₁|` is a correlate of a play type, not a measure of
  merit.** That is not a demotion: a correlate computable **without solving** is
  exactly what an instrument is.
  (iii) **And here is the use, which I think is the real find and which nobody has
  stated: this is a move-ordering statistic for the count-pruning search.**
  Branch-and-bound prunes in proportion to how early it establishes a strong
  incumbent bound, so **ordering quality is the dominant lever on pruning yield** —
  search the best action first and everything after it dies cheaply. **A solve-free
  statistic that ranks the optimal action first 53% of the time against a 25%
  baseline is precisely the input Jason's pruning scheme needs**, and it was
  measured on a carrier chosen by arithmetic rather than by outcome. The count
  bounds say *what* can be excised; this says *what order to try things in* so that
  more of it can be. They are complements, and neither was designed with the other
  in mind.
  (iv) **(B) is real but not filed, because one arbitrary choice could be
  producing it.** Nearer-to-decided coordinates carry **fewer** count-free ties
  (337,128 ppm against 521,495, a 1.55× spread, with the near quartile below both
  the survey mean and median) and **more** unique optima (23/25 against 19/25).
  **The bid-team assignment to the focal seat is arbitrary and `dd` is not
  symmetric in it**, so the association may be an artifact of that choice. **The
  check is cheap and comes first: recompute with the assignment flipped.** If it
  survives, file it; if it does not, it was never a finding.
  (v) **(B) corrects a mistake of mine, and the correction is more interesting than
  the number.** My count-pruning note guessed that count-free ties might be "a
  blurry shadow of threshold-decidedness," with decidedness producing indifference.
  **That conflated two opposite regimes.** *Already decided* (`dd = 0`, outcome
  fixed) makes everything indifferent — maximum ties. *Near but not decided*
  (`dd` small and positive) makes a single point flip the contract — maximum
  sensitivity, **minimum** ties. **The survey sees `dd` from 1 to 13 with median
  12, so it measures the second regime only**, and its direction is the sensible
  one for that regime. My guess was not wrong about the mechanism; it was wrong
  about which arm we were standing on.
  (vi) **Which yields a prediction worth testing rather than a correction to
  absorb: ties should be non-monotone in `dd`** — high at `dd = 0`, minimum at
  small positive `dd`, rising again as slack grows. **The survey's 100 seeds
  contain no `dd = 0` coordinate**, so the left arm is unmeasured and the U is
  currently one-armed. If someone wants (B) to be worth filing, the flip check of
  (iv) plus a handful of already-settled coordinates would decide the shape — and a
  confirmed U would be a considerably better object than a monotone association,
  because it would identify *knife-edge* states rather than merely decided ones,
  and knife-edge is where a decision actually matters.

- **SS-A17 (SS-A16 amended: the histogram I quoted was tie-broken, the finding
  survives with a range instead of a number, and my `U` prediction is REFUTED by
  the flip check — with the reasoning error that produced it named, because I had
  already ruled against it myself).** Five clauses.
  (i) **The number in SS-A16(i) came from a silent tie-break and is corrected.**
  33 of the 85 seeds carry within-seed `|I₁|` ties, and the unique-optimal unit is
  itself party to a tie at 20 of them; a stable sort broke them by row order.
  **That is the `max_by_key` family exactly** — the hazard this chapter has ruled
  on three times — and there is no way to say this except plainly: **I filed a
  ruling forbidding tie-broken faces and then quoted a figure produced by a silent
  tie-break.** The coordinator caught it in its own scratch work and filed both
  lawful conventions rather than picking one, which is the right handling and the
  reason the correction costs nothing.
  (ii) **The finding survives both conventions and the promotion stands.**
  Ties→best `[50, 19, 11, 5]`, ties→worst `[37, 28, 11, 9]`; **both sum to 85, both
  strictly monotone decreasing, rank 1 at 2.35× and 1.74× uniform.** **The quotable
  form is a convention-stated range — the optimal action is the smallest-frontier
  action of its own four in 43.5% to 58.8% of seeds against a 25% baseline — and
  never a single percentage**, since no single number is a fact about the data.
  My "53%" was inside the range and was not a measurement.
  (iii) **One practical consequence for the move-ordering use, which the tie
  structure makes concrete.** Ties at 33 of 85 seeds mean **`|I₁|` supplies a
  partial order, not a total one**, so an implementation needs a declared secondary
  key — and **the two conventions bracket what the ordering can be worth**, 1.74×
  to 2.35× on rank-1 accuracy, depending on how that key happens to correlate.
  That is a more useful thing to hand a builder than a point estimate would have
  been. SS-A16(iii)'s synthesis is otherwise unchanged: this remains a solve-free
  ordering statistic and ordering is the dominant lever on branch-and-bound yield.
  (iv) **My `U` prediction at SS-A16(vi) is REFUTED, and the error is one I had
  already ruled against.** The flip check reaches 23 seeds with `dd ≤ 0`, and they
  carry **fewer** ties than the knife bucket — roughly 431k against 479k ppm —
  where I predicted `dd = 0` would carry the **most**. **The reasoning error:
  I assumed the tie statistic would see contract-decidedness. It cannot.** The tie
  statistic is the multiplicity of the **count-free** clairvoyant argmax, and `dd`
  is a **count** quantity; a settled contract has no reason whatever to produce
  indifference in the trick-differential objective. **That is precisely the
  transport error I named as a sharp negative in the count-pruning note — the count
  objective is not an affine image of the trick differential, so nothing keyed to
  one transports to the other — and I committed it two messages later.**
  (v) **What the data supports instead, offered deflationary because that is where
  the evidence sits.** The coarse gradient survives the flip — less slack, fewer
  count-free ties, monotone, no `U` — and a plainer reading fits it: **`dd` may be
  measuring how lopsided the hand already is rather than how decided the contract
  is.** A hand where one side banked heavily early is likely structurally one-sided,
  with clearer plays and fewer genuine ties; a hand with points still spread is
  more balanced and has more close decisions. If that is what `dd` is, the
  association is a hand-strength proxy and considerably less interesting than a
  decision-structure statistic would have been. **Cut B stays a hangout number, as
  ruled, and this is now the reading it would have to beat.**

- **SS-A18 (two ledger repairs: one cross-reference, then the administrative
  range close; freeze 54 unchanged).** Two clauses.
  (i) SS-A6(vi)'s `FC-A26(iv)` is a mistyped cross-reference. The governing
  non-null-pairing clause is `FF-A26(iv)`. This corrects provenance only; no
  receipt, result, measured object, or reading changes.
  (ii) FC-A23(v) requires the ruling range to be frozen at chapter close. SS-A13
  and SS-A14 re-froze the range through SS-A14; SS-A15..SS-A17 were later
  substantive adjudications, but no closing range marker followed them.
  **RANGE RE-FROZEN: SS-A1..SS-A18, and the chapter is closed at it; freeze 54
  unchanged.** No measured object, carrier, result, verdict, receipt, source, or
  reading is added or amended; SS-A17 remains the latest substantive ruling and
  governs its correction of SS-A16. Any later SS ruling requires an explicit
  re-brief and another range re-freeze.

---

## GPU-native trick-1: the bounded portable foundation (2026-08-16)

**Adjudicator:** walt-math. **Object:** the received
`math/gpu_native_trick1_implementers_guide_v0.2.md`, the repaired first-build
contract `GPU-NATIVE-TRICK1.md` v0.3, the portable `walt-gpu-spec` and
`walt-gpu-ref` implementation, its bounded reduced carrier, the Lean foundation,
and the current host's Metal Gate-0 record. **Tier:** exploratory throughout,
below every project tier; no result here is an opening-play claim. **Basis:** F7,
DS-A1, DS-A28, PG-A8, PG-A13, N4-A16(iv), T1-A12, freezes 7, 23, 26 and 47,
and first-hand reading of the named source and implementation. Rulings are
**GT1-A1..GT1-A9**. **Freeze 55** is fixed at GT1-A9. Numbers 39 and 40 remain
reserved.

The received architecture is accepted only in its repaired v0.3 form. The
portable implementation is a correctness foundation for a later Metal path; it
is not itself GPU evidence, a perfect-recall controller, or a first-lead verdict.

- **GT1-A1 (authority, provenance, and tier).** The received v0.2 guide is
  preserved byte-for-byte. Its source identity is: original source commit
  `ca18bc6807b974b31d4640786d7a2d63ae0b79fe`, intake commit
  `c230949c77ff7e8e22f912ed70f8206488ac9022`, SHA-256
  `ee2e78da20eb7d087fb121f467a56bafc0179a45fb692ca0b938f4c4210b6a44`.
  `GPU-NATIVE-TRICK1.md` v0.3 is the binding first-build contract wherever it
  narrows, repairs or rejects that received source. `CENSUS-RULINGS.md` remains
  the append-only adjudication authority. The Rust, Lean and receipt surfaces
  are evidence at this exploratory tier only; one never silently stands in for
  another.
- **GT1-A2 (`OpeningRootV1`: ACCEPTED as the only first-slice model).** The root
  is exactly one focal seat with `focal = bidder = leader = actor`, one legal
  seven-tile focal hand, empty public play record and empty current trick, one
  `walt-core` declaration, and contract normal form `PointBid(30..41)` or `Mark`.
  The loss budget is derived only (`42 - bid` or `0`) and asserted at most 12.
  The closed profiles are `IgnoreAuctionEvidenceV1`,
  `UniformCompatibleOpeningDealsV1`, `UniformRandomLegalV1`,
  `DeclaringTeamMakesV1`, and `OpeningStraightHand21FieldActionsV1`. The hidden
  support is the complete ordered 7/7/7 allocation of the other 21 tiles,
  `N0 = 399,072,960`. No generic public-state adapter, auction-conditioned prior,
  arbitrary field, utility, or horizon is admitted by this ruling; any one is a
  new profile and re-enters adjudication.
- **GT1-A3 (exact arithmetic and generated semantics).** The field scale is
  `L = 420`; the response exponent is 3 and the full opening horizon exponent is
  21. Exact mass uses `U256MassV1`, eight little-endian `u32` limbs, with checked
  addition, ordered subtraction and small multiplication; no floating-point type
  enters the path. Support counts, likelihood coefficients, cell masses,
  conditional values and weighted contributions remain distinct roles in checked
  frames. `SemanticTablesCanonicalV2` is generated from `walt-core`, serialized
  canonically and hashed by the implementation; it is not a hand-maintained
  second rules table. SHA-256 is anchored by published known-answer vectors.
  T1-A12 is addressed for this slice by a genuinely independent prose-rules
  bridge to Rob: led context and compelled-follow behavior are compared over their
  complete declared finite domains, and winner plus points over every declaration,
  leader and distinct four-tile trick. A bridge failure is stop-and-report.
- **GT1-A4 (the opening-response projector: ACCEPTED with its exact boundary).**
  For a selected effective led context, `m` is the number of hidden matching
  tiles and is checked in `0..6`. The scalar projector emits the unique feasible
  `(response triple, remaining matching counts)` cells in the generating order:
  response indices lexicographic by seats 1, 2, 3, then matching-count vectors in
  that same seat order. Each cell separately computes the support count `A`, the
  scaled response coefficient `C`, and `W = A*C`. The mandatory whole-projection
  mass is `N0 * 420^3 = 29,566,517,460,480,000`. The exact cell counts for
  `m = 0..6` are `7,980, 1,140, 2,166, 3,408, 5,172, 7,800, 11,730`; 11,730 is a
  hard failure cap, never a truncation. Response payloads may be reused only for
  identical contexts. Distinct physical root leads remain distinct semantic
  actions and persisted envelope identities even where their reusable projector
  payload is byte-identical.
- **GT1-A5 (the independent reduced carrier and stop).**
  `ReducedOpeningCarrierV1` is generated deterministically from each declared
  root in grade, led-context and matching-count order; its pool is the
  lexicographically least feasible pool and is checked against an independent
  brute-force oracle. The direct work unit is one complete physical world in
  freeze-7/23 order, and `M1_DIRECT_WORLD_CAP_V1 = 100,000`. Grades 2, 3 and 4
  have respectively 90, 1,680 and 34,650 worlds and are mandatory closed-form
  versus independent-direct parity rungs. The declared two-root carrier covers
  all feasible `m = 0..6` coordinates at those grades, 48 root-bound parity
  coordinates in total. Grade 5 has 756,756 worlds: its 16 root-bound coordinates
  file a typed `DECLARED STOP` before enumeration, with emitted worlds, cells and
  payload all zero. No partial grade-5 comparison is retained and no grade-4
  result is evidence about the opening.
- **GT1-A6 (persistence, build identity, and replay validation).** A raw projector
  payload is explicitly non-persistable. The canonical persisted run envelope
  binds the freeze-55 descriptor, complete root/profile key, physical root action,
  canonical semantic-table bytes and digest, arithmetic/frame identity, a nonzero
  32-byte build identity derived from the checked source manifest, and the complete
  projector payload. The grade-5 stop record separately binds the same authority,
  root, table and build identity to its exact reduced coordinate, 756,756-world
  count, 100,000 cap and zero-output fields. Validators parse, reconstruct and
  compare the canonical objects and reject unknown versions, build mismatch,
  malformed sections, changed semantics, corruption, truncation or partial stop
  output. The reproducibility gate regenerates these objects from fresh state and
  requires byte identity with the committed comparands; a self-produced hash
  equality without this binding is not a receipt under PG-A8.
- **GT1-A7 (the semantic and verdict fences).** Freeze 26 is cited unchanged; it
  is not redefined. The portable slice builds neither an information net nor a
  candidate/program policy dimension, computes no `Q^H`, lower witness, upper
  witness, root interval, optimal set or selected play, and creates no player.
  The non-strict member, strict uniqueness and least-index canonical distinctions
  remain obligations for the later controller. Freeze 44's CPU walk-step unit and
  budget do not type complete direct worlds or projector cells and are deliberately
  excluded from this carrier. Freezes 39 and 40 remain reserved. Freeze 47's
  trick-1 carrier remains a separate prior object; nothing in this section mutates
  it or transports a verdict from its reduced-grade checks.
- **GT1-A8 (Lean boundary and Metal Gate 0).** The Lean foundation discharges:
  the legal nonpass loss-budget bound; initial and transition-preserved seven-tile
  hand capacity; actual `PlayState.legalSet` nonemptiness, at-most-seven and
  divisibility by 420; the 212-bit denominator and 217-magnitude-bit utility
  windows; the state-tied unbanked-point invariant including the unresolved
  current trick and its preservation across legal play; all seven opening-cell
  counts and the 11,730 maximum; positive component upper summation; one-shared-
  policy lower summation; dominance; and member versus uniqueness. It does not
  yet prove the semantic `(response,e)` partition, the `A/C/W` formulas or global
  conservation, posterior stratification/factorization, the exact information-key
  equivalence, canonical least-index verdict, sparse-DP or meet-in-the-middle
  refinement, Rust/Lean correspondence, or Metal/Rust correspondence. Those are
  explicit proof debt. On the current Apple M5 Max host, Metal support exists but
  only Command Line Tools are selected: `xcodebuild`, `metal`, `metallib`,
  `metal-ar` and `xctrace` are unavailable. Metal Gate 0 is therefore **NO-GO**;
  M2 and every Metal-dependent gate remain unrun until a compatible full Xcode is
  installed and selected. A portable green result is never reported as Metal
  green.
- **GT1-A9 (FREEZE 55 — the portable M0/M1 authority and deterministic
  encodings; conditional gate close).** Freeze 55 fixes the byte string
  `GT1_FREEZE_SET_DESCRIPTOR_V1` and the v1 canonical encoding rules it names:
  v0.3 plus GT1-A1..GT1-A9 authority; freezes 7, 23, 26, 47 and 55 with 39/40
  reserved; the exact `OpeningRootV1` profiles of GT1-A2; `U256MassV1` and scale
  of GT1-A3; `SemanticTablesCanonicalV2`; the GT1-A4 cell generator/order and
  11,730 cap; `ReducedOpeningCarrierV1` grades 2..5, the 100,000 direct-world cap
  and 756,756-world grade-5 stop; tasks `M1OpeningResponseProjectorV1` and
  `M1OpeningDirectParityDeclaredStopV1`; and the GT1-A6 run-envelope, declared-stop
  and build-identity schemas. Exact hashes, lengths, counts generated by these
  rules are derived checks asserted against the canonical objects, never parallel
  authorities. Freeze 44 and M2+ are explicitly out of scope.

  The portable M0/M1 implementation is source-complete for this bounded slice,
  but the **whole gate is CLOSED only when** the checked source manifest, committed
  canonical envelope and grade-5 stop, fresh byte-for-byte regeneration, guide
  checksum, formatting, warning/float denials, release workspace tests and Lean
  target all pass together. Until that integrated reproducibility artifact exists
  and the gate is green, report **IMPLEMENTED, GATE PENDING**, not complete. A green
  integrated gate may report **PORTABLE M0/M1 COMPLETE under freeze 55** without a
  new ruling; any changed authority, profile, carrier or encoding is a new freeze.
  It still reports no Metal result and no opening-root verdict.

  **RANGE FROZEN: GT1-A1..GT1-A9; freeze 55 fixed; chapter closed.** All issued
  freeze numbers 1..55 are accounted for: 53 spent and 39/40 reserved. Any later
  GT1 ruling requires an explicit re-brief and range re-freeze.

---

## GPU-native trick-1: the binding M2 Metal parity gate (2026-08-16)

**Adjudicator:** walt-math. **Object:** the complete 44,079-byte M2 rebrief,
the 46,133-byte binding M2 contract, the newly available full Xcode/Metal
toolchain on the Apple M5 Max host, and only the arithmetic/projector parity
slice those documents name. **Tier:** exploratory throughout, below every
project tier; no result here is an action-value or opening-play claim. **Parent:**
GT1-A1..GT1-A9 and freeze 55 remain immutable. Rulings are
**GT1-A10..GT1-A17**. **Freeze 56** is fixed at GT1-A17. Reserved freeze
numbers 39 and 40 remain reserved.

- **GT1-A10 (explicit rebrief and historical Gate-0 supersession).** The exact
  M2 rebrief is `math/gpu_native_trick1_m2_rebrief_v0.1.md`, SHA-256
  `9183132529a42289a104a73d8f7e196eb95058ac2edda60bb42c715f1f8a139a`.
  It is accepted as the mandatory bridge from freeze 55 to M2. The historical
  Gate-0 NO-GO receipt remains a true immutable observation of its old
  environment; it is not rewritten. Full Xcode 26.6 and the downloaded Metal
  32023.883 component are now present, and an elevated first-hand diagnostic
  created the M5 Max device, queue, pipelines and command buffer and passed
  strict integer cases. That opens the host precondition only. The checked-in
  Rust Gate 0, full corpus and official receipt remain mandatory, and a sandbox
  no-device result is an environment failure rather than a skip. No Codex-app
  restart or privacy-setting change is part of this gate.
- **GT1-A11 (scope, claim fence and carrier: ACCEPTED exactly).** M2 contains
  only `U256MetalParityV1` and `OpeningProjectorMetalParityV1`. Its canonical
  carrier is `M2OpeningParityCarrierV1` in Reduced, GradeMatching,
  SameContextPair order with the exact generators and binding split in the
  contract. Context payload reuse never collapses distinct physical actions;
  genuinely equal physical tuples across different evidence instances remain
  equal rather than receiving an ordinal salt. M2 computes no action value,
  selected lead, optimal set, information net, continuation, performance
  crossover or player. Its sole admitted green sentence is
  **M2 METAL PROJECTOR PARITY COMPLETE under freeze 56**.
- **GT1-A12 (integer corpus and extracted choose table).** The arithmetic ABI is
  eight little-endian `u32` limbs with the five closed checked operations and
  exact SUCCESS, CHECKED_UNDEFINED and HARD encodings. `U256MetalCorpusV1` is the
  fixed 16,384-case edge/SplitMix corpus with an independent BigUint oracle;
  its thirteen malformed cases are controls and never accepted results.
  `OpeningChooseTableV1` is the separately identified 22-by-22 extraction from
  unchanged `SemanticTablesCanonicalV2`, checked entrywise against BigUint. It
  neither mutates nor masquerades as that parent table. Float types, literals
  and operations remain excluded from the Walt proof path.
- **GT1-A13 (work unit, ABI, dispatch and canonical order).** One official
  projector work unit is one complete validated `OpeningContext`, one retained
  command buffer and one exact response-count grid. One thread owns one ordered
  response and ten fixed slots. `OpeningTaskV1`, `OpeningSlotV1`,
  `ArithmeticInputV1` and `ArithmeticOutputV1` have the exact scalar-word
  layouts and sizes in the binding contract; no native vector, enum, bool,
  pointer, padding or host `usize` enters persisted bytes. Stable host scanning
  fixes order independently of execution order. Complete raw-slot parity,
  GPU-field-derived compact-payload parity, cap, tail, poison and two-guard
  checks are separate mandatory conjuncts. The thirteen malformed opening
  controls each use one private memory-safe one-thread command and are
  unreachable from the production API.
- **GT1-A14 (scheduler, completion, timeout and no partial result).** M2 is
  deliberately sequential: no atomics, reductions, scans, indirect dispatch,
  slabs, adaptive batching or concurrent commands. Only native COMPLETED with
  no command error permits a read. The child polls for at most 120,000 ms; the
  parent's unextendable committed-command watchdog is 125,000 ms, while framed
  CPU phases use the separate 600,000 ms liveness rule. A timed-out child flushes
  its typed terminal frame and exits 124 without unwinding through live Metal
  resources. Crash, malformed progress, timeout, guard write, mutation, hard
  status or any mismatch yields the distinct zero-accepted failure receipt.
  No earlier task, partial arena or CPU fallback becomes M2 evidence.
- **GT1-A15 (Rust binding, unsafe boundary and compiler identity).** The direct
  `objc2`/`objc2-metal` dependency and feature closure, safe portable/Metal token
  boundary and exactly three private unsafe operation classes in the contract
  are binding. A pure byte-parity token carries no GPU provenance; only
  `walt-metal` may join it to retained completed-command evidence. The MSL entry
  points, buffer indices, shared storage, exact grids and checked-in library are
  fixed. The Metal 3.2 compiler profile and normalized argv are fixed. Two
  pre-freeze fresh-directory builds established the intended reproducibility
  boundary: AIR differed because it embedded source paths, while final metallib
  bytes matched. The final sources and library must repeat that result; the
  smoke digest is not substituted for the final library identity.
- **GT1-A16 (historical/current persistence and the blocking conjunction).** The
  old manifest is verified against blobs at parent commit
  `3b4c6d60fef371e3050de151ccf9eaefbc2d2da7`, replacement objects disabled;
  old receipts remain immutable comparands; and current `CENSUS-RULINGS.md`
  must have the exact 921,481-byte parent blob as its prefix. M2 has a new
  repository-root-relative source manifest whose exact bytes, excluding itself
  and M2 receipt comparands, define `M2BuildIdentityV1`. The closed binary
  success/failure receipt, digest domains, task and binding records, child
  protocol, identity direction and cycle exclusions in the contract are
  mandatory. Portable CI, historical verification, source verification,
  warning/no-float gates, release tests, final-metallib double build, elevated
  controls, discarded maximum smoke, two fresh full official runs, receipt byte
  equality, Lean build/axiom audit and final source re-verification form one
  conjunction. Nothing less issues the success sentence.
- **GT1-A17 (FREEZE 56 — exact binding contract, descriptor and range close).**
  The binding authority is `GPU-NATIVE-TRICK1-M2.md` v1, exact SHA-256
  `aacb6df5e9106b3b6bf00ccfb496c71f762c0fb4644c13a17f76d2ac2f0326e3`.
  It survived independent kernel/API, binary-persistence and adversarial
  consistency audits after all blockers were closed. Freeze 56 fixes the exact
  ASCII bytes below, with no trailing NUL or newline; their SHA-256 is
  `7bdc5e05513fd1d7e7b6c26870cf9bd4a16966c5daf48963729d999c4b6b28cf`:

  ```text
  GT1-M2-FREEZE-SET-V1|authority=GPU-NATIVE-TRICK1-M2-v1@aacb6df5e9106b3b6bf00ccfb496c71f762c0fb4644c13a17f76d2ac2f0326e3+GT1-A10..GT1-A17+freeze56|parent=freeze55@9b181092045b003893cae7c09cc7b7c8b57f75c3c5c4cf7043b8d428df738efa;commit=3b4c6d60fef371e3050de151ccf9eaefbc2d2da7|guide=ee2e78da20eb7d087fb121f467a56bafc0179a45fb692ca0b938f4c4210b6a44|rebrief=9183132529a42289a104a73d8f7e196eb95058ac2edda60bb42c715f1f8a139a|tasks=U256MetalParityV1,OpeningProjectorMetalParityV1|arithmetic=U256MassV1,U256MetalCorpusV1|projector=M2MetalAbiV1,OpeningChooseTableV1|carrier=M2OpeningParityCarrierV1|bindings=ReducedEvidenceBindingV1,PhysicalActionBindingV1|runner=M2SequentialRunnerV1|compiler=M2MetalCompilerProfileV1|receipt=M2MetalParityReceiptV1|manifest=M2SourceManifestV1|proof=Texas42.Trick1MetalFoundation|reserved=39,40|excluded=action-value,selected-lead,information-net,K-OPEN4+,performance,player
  ```

  Exact hashes, byte lengths, carrier counts, observed device limits and derived
  arithmetic bounds remain checked consequences of the frozen generators and
  objects, not parallel mathematical authorities. Any implementation discrepancy
  fails this gate or requires a new append-only adjudication; observing a Metal
  result never permits silent target repair.

  **RANGE RE-FROZEN: GT1-A1..GT1-A17; freeze 56 fixed; chapter closed.** All
  issued freeze numbers 1..56 are accounted for: 54 spent and 39/40 reserved.
  Any later GT1 ruling requires an explicit re-brief and another range re-freeze.

---

## GPU-native trick-1: the binding M3 perfect-recall-net parity gate (2026-08-17)

**Adjudicator:** walt-math. **Object:** the exact 44,738-byte M3 rebrief and
152,251-byte binding M3 contract, on the immutable freeze-56 carrier and
toolchain ancestry. **Tier:** exploratory throughout, below every project tier;
no result here is an opening-play claim. **Parent:** GT1-A1..GT1-A17 and freeze
56 remain immutable. Rulings are **GT1-A18..GT1-A24**. **Freeze 57** is fixed at
GT1-A24. Reserved freeze numbers 39 and 40 remain reserved.

- **GT1-A18 (explicit M3 rebrief and parent continuity).** The mandatory bridge
  is `math/gpu_native_trick1_m3_rebrief_v0.1.md`, exact SHA-256
  `07b3c993260ca25524ac1df2c3e3bd864ce66401ba6666d5ac918f633be3bf31`.
  Freeze-55/freeze-56 contracts, descriptors, manifests, receipts, metallib and
  the CENSUS prefix remain immutable historical authorities. The binding M3
  contract closes every enumerated pre-freeze decision and supersedes the
  rebrief only where it is more specific; orientation-only fractions and counts
  are not frozen results.
- **GT1-A19 (scope, carrier and claim fence: ACCEPTED exactly).** M3 is one
  grade-4 h8 carrier immediately before trick 4, uniform over the exact 1,200
  compatible worlds, with `HistoricalVoidFeasibilityOnlyV1` and
  `UniformRandomLegalV1`; S1 alone is focal. It evaluates roots 21, 31, 33 and
  55 for objectives M3A future-trick differential and M3B P30 make, each under
  separately typed H lawful-perfect-recall and C world-revealed treatments.
  M3A requires pairwise CPU/Metal parity and strict `C > H`; M3B requires
  pairwise parity without a predeclared gap; objective topology must agree root
  by root. The sole green sentence is **M3 PERFECT-RECALL NET PARITY COMPLETE
  under freeze 57**. It is not a trick-1 value, lead choice, compression,
  growth, performance, controller, strategy-strength or player claim.
- **GT1-A20 (perfect-recall net and independent authorities).**
  `M3PerfectRecallKeyV1` is exactly the scoped S1 observation with complete own
  action-observation memory and no hidden-world identity;
  `M3WorldRevealedKeyV1` is a disjoint C type. Every focal key retains its
  complete legal face, all actions and the forced singleton frontier. Unique
  parent/action, complete all-run sealing and sum-before-max license the exact
  unnormalized recurrence; focal branches copy mass and field choices alone
  divide it. Independent H/C CPU authorities share only frozen carrier/rules,
  and the host-only no-max repricer consumes raw sealed evidence rather than
  production values. No strategy fusion, slab-local maximum, key
  renormalization, objective pruning or cross-world C pooling is admitted.
- **GT1-A21 (Metal ABI, two-family reductions and closed caps).** The exact
  scalar-word ABIs, two MSL entry points, bindings, status/error precedence,
  poison/tail/guards, serial completion schedule and compiler/device profile in
  the contract are binding. Exactly two noncoexisting REDUCE families exist:
  `MASS_BUCKET` partitions EDGE_A once; `BACKWARD_VALUE` uses one least-witness
  child value or each terminal edge once. Conservation and selected-policy
  terminal-bucket checks are disjoint host folds, never a third family.
  Count-one retirement, immutable dense replacement, real epoch order (MASS
  0..3; BACKWARD 3..0), the 21-level/range proof and every command/frame and
  live-byte cap are fixed. The headline caps are H 2,048, C 16,384, task 32,768
  and run 524,288 commands; task/run frames 131,072/2,097,152; Metal 512 MiB,
  host 2 GiB and spill 16 GiB. Float and timing-derived proof-path arithmetic
  remain forbidden.
- **GT1-A22 (semantic evidence, controls, persistence and reproducibility).**
  Visits, lawful successor emissions, EDGE_A, family-level-zero rows, keys,
  faces, actions, singletons and terminals have the sole owners and exact
  epoch/treatment/GLOBAL aggregation rules in the contract. CPU and post-Metal
  semantic streams are independently rendered. The 36-control registry,
  constant-hash duplicate, exact child protocol, 4,096-byte complete-task
  checkpoint, 52,880-byte success receipt and 512-byte failure receipt are
  closed grammars with no partial salvage. Historical blobs and the exact
  929,957-byte freeze-56 CENSUS prefix are verified with replacements disabled;
  current exact-HEAD bytes define `M3SourceManifestV1` and `M3BuildIdentityV1`.
  Two fresh source-verified metallib builds and two fresh checkpoint-disabled
  official runs must produce byte-identical library and receipt bytes.
- **GT1-A23 (proof boundary and blocking conjunction).**
  `Texas42.Trick1PerfectRecallNet` must build and pass the axiom audit for
  codec/scoping, replay, unique parent, complete face, sum-before-max, mass,
  objective bridges, the two-family census/range/compaction/cap proofs, epoch
  counters and all-or-nothing composition. Portable tests, exhaustive finite
  bridges, independent-oracle review, exact toolchain/metallib checks, elevated
  native controls, both complete runs, receipt validation and final source
  re-verification are one conjunction. Rust-to-Lean, Metal-to-Rust, general
  independent-oracle correctness and grade-4-to-trick-1 transport remain named
  correspondence debt; parity does not discharge them.
- **GT1-A24 (FREEZE 57 — exact binding contract, descriptor and range close).**
  The binding authority is `GPU-NATIVE-TRICK1-M3.md` v1, exact SHA-256
  `79de73e9ee9b0e1fd3b0467ddf27a66dcc9e135419cba531cb73218d71eee147`.
  It survived independent exact-hash mathematics and systems audits after the
  reduction-pass and epoch-visit blockers were closed. Freeze 57 fixes the exact
  962 ASCII bytes below, with no trailing NUL or newline; their SHA-256 is
  `e5efe6ce5c293b29fc05902e7bf913fd13f04a031c2951f7a1bf5cf92137f852`:

  ```text
  GT1-M3-FREEZE-SET-V1|authority=GPU-NATIVE-TRICK1-M3-v1@79de73e9ee9b0e1fd3b0467ddf27a66dcc9e135419cba531cb73218d71eee147+GT1-A18..GT1-A24+freeze57|parent=freeze56@7bdc5e05513fd1d7e7b6c26870cf9bd4a16966c5daf48963729d999c4b6b28cf;commit=20a9feccb71660d10dcca3e334867e7b5400a837|rebrief=07b3c993260ca25524ac1df2c3e3bd864ce66401ba6666d5ac918f633be3bf31|profile=UniformCompatibleSupportV1,HistoricalVoidFeasibilityOnlyV1,UniformRandomLegalV1|objectives=M3A_FUTURE_TRICK_DIFFERENTIAL,M3B_P30_MAKE|treatments=H_LAWFUL_PERFECT_RECALL,C_WORLD_REVEALED|carrier=M3CarrierProfileV1,h8,roots21-31-33-55|keys=M3PerfectRecallKeyV1,M3WorldRevealedKeyV1|arithmetic=U256MassV1|kernels=m3_field_expand_v1,m3_u256_reduce_pass_v1|reductions=MASS_BUCKET,BACKWARD_VALUE|receipt=W42M3R01,M3FailureReceiptV1|manifest=M3SourceManifestV1|proof=Texas42.Trick1PerfectRecallNet|reserved=39,40|excluded=trick1-value,lead-choice,compression,growth,performance,controller,strategy-strength,player
  ```

  Exact hashes, byte lengths, carrier counts and observed values remain checked
  consequences of the frozen generators and objects, never parallel
  mathematical authorities. Freeze 57 authorizes only the gate; it records no
  M3 result. Any implementation discrepancy fails M3 or requires a new
  append-only adjudication after observation; code may not silently repair the
  target.

  **RANGE RE-FROZEN: GT1-A1..GT1-A24; freeze 57 fixed; chapter closed.** All
  issued freeze numbers 1..57 are accounted for: 55 spent and 39/40 reserved.
  Any later GT1 ruling requires an explicit rebrief and another range re-freeze.

## Signed-pivotal intake adjudication (2026-08-18)

**Adjudicator:** walt-math. **Object:** `walt/math/signed_pivotal_geometry_v0.1.md`
(filed 2026-08-18, commit eaf9b23; intake companion
`signed_pivotal_geometry_v0.1_intake.md`, whose exact-rational verification of
every boxed identity this audit takes as read). **Tier:** exploratory; an
intake audit in the DS-A1..DS-A16 shape. Nothing in the parent is promoted by
being audited, and nothing below may be cited above exploratory tier.
**Basis:** the parent; the intake companion; SCENARIO-PLAYER.md v0.1 and its
obligations ledger; POLICY-GEOMETRY.md (Gate E); the decision-sparse audit
DS-A1..DS-A16 and, through it, PG-A1..PG-A18, Lemma R, Lemma G. Amendments are
numbered SP-A1.. and bind any design that consumes the parent — in particular
the E0 experiment, adopted below as **the tilt audit**.

**Headline, stated first — four findings decide how the parent may be used.**

1. **The central mathematics is SOUND.** Every boxed identity — g = qτ,
   E[Y²] = q, Var(Y) = q − g², H = 1/(qτ²) − 1, the world/tape projection,
   strata linearity, the cover identity w²·Var(Y|P) = wq − g² — was verified
   by hand and exactly on 2,000 random exact-rational instances at intake.
   Exactly one general claim in the parent is FALSE as written: §2.1's
   sentence that paired evaluation is "strictly sharper" than unpaired.
   Var(Y) = Var(u_a) + Var(u_b) − 2·Cov(u_a, u_b), so pairing helps exactly
   when the two policies' outcomes positively correlate; the parent's own
   Case C at maximal anticorrelation (u_a = 1−u_b on every scenario, V_a =
   V_b = 1/2) gives paired variance 1 against unpaired 1/2. The repair and
   its consequence are SP-A5. Everything else in §§2, 4, 5, 6 stands.
2. **Two vocabulary collisions with standing rulings are resolved by rename**
   (SP-A1, SP-A2): the parent's θ = (1+τ)/2 collides with walt's auction
   threshold θ, and the parent's "pivotal envelope" collides with the value
   upper envelopes of the decision-sparse/PG chapters (DS T1: the minimal
   envelope IS the Exp set). In walt artifacts: **pivotal win share** (never
   bare θ) and **pivotal cover**. The formal object remains the *policy*
   (deterministic, information-consistent — PG's word); "frozen plan" is read
   as **frozen policy** (SP-A3).
3. **E0 is ADOPTED as the next experiment — the tilt audit — with three
   design corrections** (SP-A8..SP-A10): frozen policies need no DAG
   serialization for the smoke (the freeze tuple is the policy, SP-A8); with
   that implicit representation replay costs the same as re-solving, so
   §9.4's cheap-replay premise and its 10,000-scenario panels are deferred
   until explicit extraction exists (SP-A9); and the parent's "existing
   n=800 panel" matches no filed artifact — the corpus anchors are named in
   SP-A10.
4. **The three locks (§8) are the standing sandwich discipline in new
   clothing** — measure ↔ exact counts, response ↔ exact evaluation,
   optimization ↔ Lemma G / DS T7–T8 — and inherit the decision-sparse
   obligations verbatim (SP-A7): no sampled quantity inside an L or a U, and
   a lower witness is the exact value of a *fixed* lawful policy under the
   declared belief (DS-A14's fatal failure mode is the optimization lock's
   failure mode). Separately: **O10–O11 are retired permanently** — the gap
   is a numbering artifact of the side-channel hops (Jason, 2026-08-18), the
   unfiled import is not being retrieved, and retired numbers are never
   reused (SP-A11).

### Section-by-section verdicts

- **§1 setting — SOUND, well-typed.** B = (K, e, β) is the key + seat context
  + fiber belief of SCENARIO-PLAYER §§2–4; u_ρ ∈ {0,1} is the pmake
  objective; V_ρ linear in β is PG-A2's linearity; §1.1's frozen-plan /
  optimized-root-action distinction (one hyperplane vs piecewise-linear
  envelope) is exactly Lemma R(b)'s shape and is already adjudicated ground.
- **§2 signed pivotal theorem — SOUND** (verified exactly). §2.1 sound except
  the comparative claim (SP-A5). §2.2 multinomial decomposition — SOUND,
  standard. §2.3 — SOUND as a cost *scale*; the parent's own sequential-
  validity caveat is binding (O14). §2.4 four ties — SOUND; note Case A is
  exact: q = 0 forces g = 0, so equal value is a theorem there, not an
  estimate.
- **§3 discovery/evaluation separation — SOUND**, standard selection-bias
  discipline; filed as O13/O14. §3.3 local flip surfaces — SOUND; the
  "multiple crossings" caution is real and matches the envelope picture.
- **§4 world/tape projection — SOUND under SP-A6's typing** (r is the tape
  seed, assigned world-independently; walt's current single-stream
  derivation does not yet satisfy the split).
- **§5 counted strata — SOUND** (verified exactly); Neyman and cost-aware
  allocations standard; §5.1 interval composition exact by linearity and
  w_j ≥ 0. The marginal-Δ_j spend rule is approximate and so labeled.
- **§6 covers — SOUND** with the SP-A2 rename; §6.2's complement floor is
  O16 and is the load-bearing safety clause; §6.3's count/generate
  capability split is real and filed as O17.
- **§7 refinement signature — ADOPTED** as the per-decision diagnostic
  contract for the tilt audit; the spend-routing table is design guidance,
  not mathematics, and binds nothing.
- **§8 three locks — SOUND as discipline; inherits DS obligations (SP-A7).**
  The escalation ladder's rungs 1–6 are lawful individually; rung 7 is
  DS T8's certificate-shape with its "member of the optimal set" obligation.
- **§9 E0 — ADOPTED with SP-A8..SP-A10** as the tilt audit. Phase E requires
  the SP-A6 seed split before it can run. Falsifiers and decision gates
  (§9.9–9.10) adopted as written.
- **§10 censuses — SOUND; concordance with Gate E filed (SP-A12).** §10.2's
  rank claim is elementary and correct: all pairwise comparisons factor
  through the r coordinates ⟨β, b_i⟩. On samples it is an estimate; on atoms
  it is exact after the quotient — the parent says so.
- **§11 amortization — SOUND.** §11.1's panel reweighting is valid only on
  declared support containment (the parent states the condition; the
  atom-table form is the exact one). §11.3's paired-dynamics telescoping is
  exact by linearity of expectation.
- **§12–§13 — ADOPTED conditionally** on tilt-audit outcomes; §13 is the
  driver's persistence contract. **§14 — filed** as O12–O19 (2026-08-18).
  **§15 — honest**, and its last bullet keeps the unfiled import's
  literature mappings unciteable.

### SP-A1..SP-A12 — amendments (binding on consumers of the parent)

- **SP-A1 (θ is the auction's).** In walt artifacts the pivotal quantity is
  always written τ; (1+τ)/2, where needed, is the **pivotal win share**,
  never bare θ. The auction threshold keeps θ unqualified.
- **SP-A2 (cover, not envelope).** A structural predicate containing all
  pivotal scenarios after projection to worlds is a **pivotal cover**.
  "Envelope" remains reserved for value upper envelopes (DS T1, PG-A4's Exp
  set). The parent's §6 is read with this substitution throughout.
- **SP-A3 (frozen policy).** The formal object is the deterministic
  information-consistent **policy**; "plan" in the parent is read as policy.
  A **frozen policy** is a policy together with its freeze tuple —
  (solver version, seed schedule, n, n0, field-model version, tie-refinement
  config) — under which walt's seat is a deterministic function of the
  observation record (O1 gives information-consistency by construction).
- **SP-A4 (adopted names).** **Pivotal mass** q, **tilt** τ, **gap** g,
  **fixed-pair hardness** H, **scenario** ξ = (ω, r), **panel** (common
  scenario panel). The E0 experiment is named **the tilt audit**; "E0"
  survives as its ID.
- **SP-A5 (paired-variance repair).** Replace §2.1's comparative sentence
  with: Var(Y) = Var(u_a) + Var(u_b) − 2·Cov(u_a, u_b); paired evaluation
  is sharper than independent evaluation **iff Cov(u_a, u_b) > 0**. The
  practical reading — near-identical policies agree on most scenarios, so
  Cov is large and pairing wins — is a *hypothesis the tilt audit itself
  measures*, not a theorem. Case C is the honest counterexample and stays
  in the parent.
- **SP-A6 (tape typing).** The tape r is a u64 seed drawn/assigned
  independently of the world ω; the scenario law is the product law. Walt's
  current code derives modeled-mind randomness from one stream keyed by
  (seed, hand, record) — world and tape are not separable today. The tilt
  audit's Phase E, and any use of the §4 projection, requires the split:
  world ID and tape seed stored and varied independently. Until then,
  d(ω)/s(ω) estimates are undefined artifacts.
- **SP-A7 (locks inherit the sandwich discipline).** Any L or U appearing in
  lock-closure arguments obeys DS T7's obligations: no decimation or sampled
  mean inside a bound; candidates information-consistent at the node; L and
  U at the same β with the same α-map. A lower witness is the exact value of
  a fixed lawful policy (DS-A14). An exact frozen-pair gap is never labeled
  an exact root-action result (O18 = DS T8's obligation restated).
- **SP-A8 (implicit frozen policies suffice for the smoke).** Freezing needs
  no policy-DAG serialization: the freeze tuple of SP-A3 *is* the policy,
  its content hash is the policy ID, and behavioral identity is decided by
  outcome bitsets on the panel (§10's behavioral census). Phase A is
  therefore runnable with the existing solver plus a driver.
- **SP-A9 (replay-cost correction).** Under SP-A8's representation,
  replaying a frozen policy on a scenario re-runs the per-decision solves —
  replay cost ≈ solve cost, and §9.4's premise ("replay much cheaper than
  re-solving") FAILS until explicit extraction exists. Budgets follow:
  smoke panels at the hundreds scale; mid/late-grade anchor positions get
  the full Phase A–D treatment first (per-decision solves are cheap there);
  early-grade anchors get reduced panels. The 10,000-scenario panels of
  §9.4 and the bitset replay kernel of §12.2 are deferred, not refused.
- **SP-A10 (corpus anchors).** The parent's "existing n=800 panel" matches
  no filed artifact and is presumed side-channel residue; it binds nothing.
  The tilt audit's anchors are: the level-2 trick-1 saturation/tie episode;
  the dropped-30 arena divergence positions (2026-08-17/18); the divergence
  miner's 900 self-played hands / 4,156 level-2-shadowed decisions
  (2026-08-18). A stratified early-trick sample is drawn fresh under the
  audit's own seeds.
- **SP-A11 (O10–O11 retired).** The obligations ledger runs O1–O9, then
  O12–O19. O10–O11 are permanently retired as a numbering artifact of the
  side-channel hops; retired numbers are never reused. The unfiled import
  `HANDOFF-plan-geometry-and-names.md` stays unfiled and unciteable.
- **SP-A12 (Gate E concordance).** The §10 censuses refine Gate E's chain:
  syntactic count ↔ N_pol-side diversity; behavioral count = a panel
  estimate of N_vec; the signed boundary count and decision rank are new
  objects between N_vec and N_exp. E-A8's rule applies unchanged: where two
  names measure one number (e.g. behavioral census on the full fiber =
  N_vec) it is reported once. Neither document supersedes the other; the
  tilt audit measures the sampled analogues, Gate E's probe counts the
  exact ones.

## The freeze-56 v2 amendment (2026-08-24) — the one-crate unification meets the source closure

Ruling authority: Jason, 2026-08-24 ("go for it… we can't let it hold us
back from one clean Walt"), adjudicating the four questions filed in
`UNIFICATION-CENSUS.md` § Execution. Append-only; nothing in the v1
freeze artifacts is edited. Exploratory tier as always; no result's
status changes by this text.

- **FZ-A1 (re-issue).** Freeze-56's cumulative source closure is
  re-issued at the post-fold layout as
  `math/gpu_native_trick1_m0_m2_sources_v2.sha256`. The v1 manifest is
  byte-immutable and remains on disk beside it; its digest — the
  M2BuildIdentityV1 the standing receipt names — is unchanged and
  unchanged forever. v2's own digest is a NEW build identity for the
  post-fold layout, attested by no hardware receipt yet ([[m2-receipt-reearn]]).
- **FZ-A2 (translation is amendment).** `ci/verify_m2_sources.sh` gains
  an explicit 32-entry fold-translation table carrying the immutable
  M0/M1 paths (14 walt-core, 11 walt-kernel, 7 walt-gpu-spec) to their
  post-fold locations (`walt/src/{rules,kernel,spec}`, the unified
  `walt/Cargo.toml`, prefix-renamed tests). This table is an amendment
  to the freeze verifier, never a "correct reading" of the v1 freeze:
  v1 pinned paths, the paths moved, and the table is the auditable
  record of where.
- **FZ-A3 (the standing receipt is old-layout evidence).**
  `receipts/gpu_native_trick1_m2_v1/m2_metal_parity_v1.bin` keeps its
  bytes and keeps its meaning: hardware parity evidence for the v1
  build identity — the OLD layout. It is never presented as attesting
  the v2 identity. Re-earning it under v2 (614-task carrier, run
  twice, on hardware) is deferred to [[m2-receipt-reearn]], expected at
  the GPU program's unparking.
- **FZ-A4 (drift disposition).** Of the 97ce321 drift: walt-strat's two
  files are REVERTED to frozen digests (their additions served only the
  deleted oracle-a orphan; verified byte-exact against v1 entries).
  `lean/Texas42.lean` is ABSORBED into v2: its drift imports the
  Trick1PerfectRecallNet module tree — kernel-audited freeze-57
  mathematics, protected work, not scaffolding. CENSUS-RULINGS.md,
  Cargo.toml and Cargo.lock ordinary drift is absorbed into v2 by
  re-pinning.
- **FZ-A5 (freeze-event demotion).** Since the unified crate contains
  the actively developed solver, a per-commit full-digest closure would
  be red on every ordinary commit — the closure check is therefore a
  FREEZE-EVENT verification from v2 onward: run
  `/bin/bash -p ci/verify_m2_sources.sh` when a freeze event re-issues
  the manifest; `ci/check.sh` retains the per-run immutable checks
  (M0/M1 history at its producing commit, guide identity, M0/M1
  receipt replay, the Lean axiom audit). This resolves the recorded
  design tension (append-only rulings log vs full-digest closure): the
  closure is a snapshot certified at freeze events, so living
  append-only documents may be pinned by full digest in it without
  making routine appends a CI failure.
- **FZ-A6 (closure scope).** v2 package roots: the unified `walt/walt`,
  the GPU trio (`walt-gpu-ref`, `walt-metal`, `walt-m2-runner`), and
  the rob oracle set. `walt-wasm` stays outside: a packaging shell, not
  a build input to any M-gate receipt. The factory/skeleton package
  roots are gone with their crates (artifacts archived per
  `ARCHIVE.md`; producer code addressable at 648f93a).

## The calculated-evidence adjudication (2026-08-24)

Adjudicates the received `math/calculated_evidence_v0.1.md` (verbatim,
SHA-256 `9b32b14f…`; intake companion
`math/calculated_evidence_v0.1_intake.md`, 18/18 exact-rational
verification PASS). Provenance: produced in Jason's ChatGPT 5.6 Pro
session iterating on the post-reorganization repository state
(`4231cb2`), hand-ferried 2026-08-24 — the in-conversation refinement
pass of the exchange iteration policy is therefore already embodied in
the parent; Jason ruled "go unless something needs refining" the same
day. Everything below is exploratory tier; nothing is promoted.

- **CE-A1 (identities SOUND at intake).** CE-T1 through CE-T5 and every
  boxed identity with a mechanical route verified exactly (the pivotal
  closed form three independent ways over the V1 grid; anchors;
  supermartingale one-step algebra; ledger telescoping; the §10.1
  sign-vs-mean counterexample; the §7.1 expansion). Hand-checked
  arguments (Ville, mixture closure, union-bound accounting,
  safe elimination) found no defect. The companion's receipt is the
  record; the verifier stays beside the parent.
- **CE-A2 (θ/ϑ split ADOPTED).** Walt-wide vocabulary: **θ is the
  pivotal win share** `(1+τ)/2`; **ϑ is an auction/policy threshold**
  (the empirical 11/16 is a ϑ). This supersedes the signed-pivotal
  companion's earlier never-bare-θ proposal (a dated pointer is filed
  there). Existing code fields named `theta` that mean ϑ keep their
  serialized names; docs and new code use the split.
- **CE-A3 (result-type ladder BINDING for the new path).**
  `ExactFiberRoot` / `ExactFrozenSet` / `DeltaSettled` /
  `EpsilonEquivalent` / `Unresolved` / `HeuristicFallback` are
  mechanically distinct in every new API, log, and report. A sample cap
  is a resource limit, never a proof rule. `Unresolved` is a successful
  output. Nothing existing is deleted by this ruling; existing fixed-n
  play paths are HeuristicFallback-status until retyped.
- **CE-A4 (O20–O28 ACCEPTED).** The parent's proposed obligations enter
  the SCENARIO-PLAYER obligations ledger as O20–O28 with their stated
  routes. They are obligations (proof/audit debts), not results.
- **CE-A5 (correctness-path rule).** Per parent §0: fixed sample counts
  leave the correctness path — they may persist only as replay
  fixtures, heuristic-fallback defaults, historical coordinates, or
  throughput batch sizes. Per §13: the faithful Boolean replay race is
  the signed-pivotal object; the block sign racer is NOT (wrong target,
  sign-only, no anytime guarantee) and stays a heuristic/experiment;
  `race-refined` stays opt-in outside the proof path. The §10.1
  counterexample becomes a mandatory gate fixture (V7).
- **CE-A6 (level-2 probe correction ADOPTED).** LEVEL2-PROBE.md is
  amended (dated append) to split the detector into response wake-up
  (q), value wake-up (g), and decision wake-up, with sampling cost
  compared by the information rate `𝓘_f = q_f·D_{1/2}(τ_f)` — never by
  q̂ alone; exact-zero vs practical-zero (`q ≤ ε_q` at declared risk)
  preserved in the output contract. The probe's gate now reads:
  unification (done) + calculated-evidence outer controller (in build).
- **CE-A7 (implementation sequence ADOPTED).** Parent §22 is the build
  program: kernel adapter → `solver::evidence` → lazy frozen policies →
  fixed-candidate adaptive evaluator → exact endpoints → shadow → V5
  flip repair → level-2 probe → inner recursion last. Phase 1 is outer
  only (§18); inner sample schedules remain declared approximations,
  visible in every result identity. The A.6 minimum vertical slice
  lands first and proves the architecture before any live default
  changes (§20.16: the old player remains the default until arena and
  conformance gates justify a change, on Jason's word).
- **CE-A8 (refinement-agenda disposition).** The intake companion's six
  adjudication targets, reviewed: (1) the predictable-sequence
  hypothesis — the common-stream design evaluates every live candidate
  on world i before evidence updates, and elimination depends only on
  past evidence, so the conditional null is inherited; stated as an
  invariant to assert, O26's route. (2) §5 adaptive-order accounting —
  the union bound is over the fixed finite set of directed edges
  allocated up front; examination order and elimination spend nothing
  further. NON-BLOCKING. (3) ε-equivalence composition — two one-sided
  tests at summed risk is a plain union bound, same-stream validity per
  edge process. NON-BLOCKING. (4) O24 escalation bookkeeping — an
  implementation proof debt, gated by V9. (5) §14.6 paired evidence —
  needs world-level iid only; the pair (Y⁽⁰⁾,Y⁽¹⁾) is one observation.
  NON-BLOCKING. (6) θ/ϑ — ruled at CE-A2. No adversary panel is
  convened now; one may be convened later on Jason's word under the
  batch protocol, with CE-T1..T5 and O21/O24/O26 as the natural briefs.

## The targeted level-2 field-stability adjudication (2026-08-24)

Convened same-day on the second hand-ferried drop,
`math/targeted_level2_field_stability_v0.1.md` (SHA-256 `597d33c3…`,
intake companion beside it, verifier 19/19 exact). **Authorization
note, stated plainly:** these rulings are filed under the standing
same-lineage go from the calculated-evidence drop (Jason, 2026-08-24:
"a go unless you see something that needs refining"); the intake found
nothing needing refinement (companion §5). If Jason wants a Pro
refinement pass or an adversary panel first, later rulings append and
supersede — nothing here is edited in place. Everything below stays at
exploratory tier.

- **L2-A1 (L2-T1..T5 SOUND at intake).** First-disagreement
  localization, the root-action field Lipschitz bound
  `|Q_a^(1) − Q_a^(0)| ≤ R_a`, winner stability under
  `margin > R_a + R_b`, admissible-set screening, and eventual
  periodicity of deterministic finite best-response towers are sound:
  proofs step-checked, and every theorem model-checked exactly over
  1,584 enumerated finite games with full policy/world enumeration
  (19/19; the screen stayed sound under deliberately loosened bounds).
  The central targeting consequence is adopted as the level-2 frame:
  **only worlds and branches that can reach a field-disagreement state
  can carry any level-2 correction** — level 2 is a calculated
  refinement, never a universal re-solve.
- **L2-A2 (O29–O38 ACCEPTED).** The parent's proposed obligations
  enter the SCENARIO-PLAYER obligations ledger as O29–O38 with their
  stated routes, continuing the O20–O28 block. Obligations, not
  results.
- **L2-A3 (field-swap result kinds BINDING).** The seven §8 Stage-5
  semantic distinctions (`FieldStableExactRoot` /
  `FieldStableExactFrozenSet` / `FieldStableDeltaFrozenSet` /
  `FieldSensitive` / `FieldDecisionChanged` / `FieldUnresolved` /
  `HeuristicFallback`) are binding semantics for any field-swap code;
  Rust naming is free. Extends the CE-A3 ladder discipline: no UI or
  bridge flattens them into one unlabeled percentage (§19 item 24).
- **L2-A4 (exposure-tier typing BINDING).** `FrozenPolicyExposure`,
  `LibraryExposure`, and `RootActionExposureUpper` are mechanically
  distinct types; **only** `RootActionExposureUpper` may feed the
  L2-T2..T4 screen; every such bound names its derivation rung
  (E0–E4); a sampled lower witness to `R_a` is never an upper bound.
  This is the optimization lock of the exposure program (§7.4) and has
  the same standing CE-A5 gave the fixed-count rule.
- **L2-A5 (LEVEL2-PROBE reconciled).** The probe spec is amended
  (dated append) to become the *detection layer* inside the parent's
  targeted controller: its paired q̂/τ̂/ĝ/𝓘 output feeds Stage-1/2
  evidence; the targeting layer (exposure bounds, the stability
  screen, first-split traces, survivor-only field-1 optimization) is
  owned by the parent. One field-swap program, not two.
- **L2-A6 (build-order slot + anchors).** The parent's §21 steps 1–2
  are complete (this intake; the calculated-evidence correctness path
  is merged — mains `5baad99`/`bf432be`/`636d306`). The field-swap
  build (§21 steps 3+: `FieldId`, coupled first-split replay, exposure
  rungs, admissible set, targeted field-1 work) enters **after CE §22
  step 7 (shadow) merges**, and may proceed in parallel with CE step 8
  (V5 flip repair / E0 calibration), since it consumes the evidence
  authority, not the live player. The Gran anchor experiments G1–G4
  are gated on reconstructing the two Plunge game seeds/records
  (plunge-side; carded as [[gran-anchor-reconstruction]]); until then
  the screenshots remain discovery artifacts per parent §1.4.
- **L2-A7 (cycle discipline ADOPTED).** §13's typing is binding:
  recurrence claims are classified root / behavioral / local exact /
  global exact and never promoted across those lines; the §13.5
  tripwire (compare σ₁ vs σ₂ on the field-sensitive anchor corpus) is
  a standing precondition on any broad level-3 work; **no damping,
  mixtures, or robust-cycle policies without a separate mathematical
  intake** (§22 item 12). Level-model typing per O36: a level-2 result
  is a best response to a named σ₁ — never "equilibrium,"
  "convergence," or monotone improvement.
