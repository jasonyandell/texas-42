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
