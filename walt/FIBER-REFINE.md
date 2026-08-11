# The fiber-refinement probe — design for adjudication

Status: DESIGN, awaiting walt-math rulings (X-Q1..X-Q7) in
`walt/CENSUS-RULINGS.md`. One author rule unchanged: rulings are walt-math's,
this file is the orchestrator's. Everything from the fiber-probe rulings
(P-A1..P-A21) inherits unchanged: void-free capacity fiber vocabulary (P-A1),
gap report (P-A2), one-sided monotonicity (P-A3), operator and valuation
freezes (P-A5/P-A6), decimation (P-A15), timing discipline (P-A19),
boilerplate (P-A20). Tier: exploratory; every X predicate is a DECLARED
object; nothing here changes any class or value claim.

## The hope (Jason, 2026-08-11)

"The hope was everything that could happen, except X" — a fiber is everything
that could happen; refine it by declared exclusions X ("lose every time,"
"involves these classes"), especially once count re-enters. Smaller fibers,
not just fewer fibers: drag the constraints into the fiber evaluation.
Explicitly allowed to go nowhere; either outcome is a result.

S5h's finding this builds on: the class store is a storage/transport object —
a class is a future cone, so predicates about "what the future holds" are
decidable on classes (and not on bare state keys), computed once, inherited by
every world and coordinate that touches the class. Also the multi-pass
economics (Jason): a first search may cost; a second over the same store
should be nearly free; things unchanged between passes should be entirely
free. This probe measures the bite (how much do natural X's shrink real
fibers?) and the economics (what does pass 2 cost?).

## Declared predicate family (proposal)

Over one coordinate's built class store (the S5h arm-B object: carrier + r3 +
Lemma-V per-class values under the frozen operator/valuation):

- X_reach(F) — "the future passes through a forbidden class": worlds whose
  root class reaches any class in a declared set F on the class DAG.
  Support-side in flavor (pure reachability, no valuation read). Proposed F
  for the run: the terminal-loss classes — but any declared F works; the
  probe freezes its F's.
- X_val0 — "loses every remaining trick": worlds whose root class has
  Lemma-V value 0 under the frozen world-informed operator. VALUATION
  predicate: policy-relative, exact only over the transported abstract-policy
  class (v0.5 BOUNDARY), never an H claim, never a support fact.
- X_val_max — the dual ("wins every remaining trick", value = n) as a
  symmetry check on the machinery.

Count-bearing X's ("loses the bid") are OUT OF SCOPE until role re-entry;
named as the follow-on.

## Measurements

For declared coordinates (reuse of S5h's rungs and decimation where the arms
are re-run; the store side may use the full evaluated set):

1. BITE: |evaluated set| -> |remnant| under each declared X, per coordinate;
   the predicate-pass cost (one pass over the class store: reachability
   marking / value reads) in integer ns.
2. MULTI-PASS ECONOMICS: pass 1 = build store + evaluate (S5h arm B,
   unchanged); pass 2 = a DIFFERENT declared X + re-aggregation over the
   fixed store; report pass2 : pass1 wall ratio (the "second search is
   basically free" claim, measured; the S5h refold rows suggest ~1e-5 but
   only for the trivial re-fold — a predicate pass is more work).
3. REMNANT EVALUATION: the cost of evaluating only the remnant vs the full
   set with the same machinery (does exclusion actually save proportional
   work, or is the store already paid so exclusion saves nothing at
   evaluation time? — an honest possible outcome).

## Design questions for walt-math

X-Q1 (naming and tier of the refined object). Fiber-minus-X is NOT a support
restriction (X is declared, not rule-derived) and not a belief. Proposed
name: the DECLARED EXCLUSION REMNANT, a declared search/cost domain like
Phi(C0) itself; members stay "feasible worlds." Confirm or coin, and state
what may never be read from a remnant (support facts, H values, seat claims).

X-Q2 (lawfulness of each predicate). X_reach reads only DAG structure;
X_val0/X_val_max read Lemma-V values under the frozen operator. Confirm the
tier language each must carry (esp. the v0.5 boundary sentence for value
predicates), and that stamping predicate outcomes onto the persistent store
(class -> flag) is lawful cached derivation, not a status change.

X-Q3 (the exclusion semantics). Excluding a WORLD by a property of its root
class removes the whole world; excluding BRANCHES mid-cone (sub-DAG
restriction) is a different object (Y3's support restriction was
kernel-driven; this is predicate-driven). Rule which is in scope — proposal:
world-level exclusion only for this probe; branch-level named as follow-on.

X-Q4 (what "bite" may claim). Shrink factors are measured on the void-free
capacity fiber's evaluated sets (P-A1/P-A3 inherited); a large bite is a
statement about the cost domain, never about the seat's real support.
Confirm the reporting sentence.

X-Q5 (coordinates and freezes). Reuse S5h coordinates and decimation
(declared subsets, same g/W); freeze the predicate definitions (F sets, the
value thresholds) as new numbered freezes continuing the fiber-probe list.

X-Q6 (results discipline). One file `results/fiber_refine_2026-08-11.txt`,
P-A20 boilerplate inherited, per-predicate per-coordinate rows, both-outcomes
framing per F7/NO-RESCUE.

X-Q7 (persistence). The store currently lives only for one process run.
Proposal: an on-disk append-only content-addressed store (class id -> record),
stamped with its freeze set; a warm store is lawful exactly when the freeze
set matches (P-A17's discipline), and any freeze change invalidates the file
wholesale. Rule on the discipline; implementation may follow in a later
iteration.
