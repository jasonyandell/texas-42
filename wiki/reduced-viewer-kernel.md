# The Reduced Viewer Kernel and Future-Equivalence Minimality (rec only)

[Home](Home.md) · Sources: **rec Math §7.16, §12.10, §15** (no v0.7 counterpart).
Related: [support-dynamics](support-dynamics.md), [strategic-state](strategic-state.md),
[open-problems](open-problems.md).

rec's second big move: strip the *physical/support transition state* down to five
components, having proved each discarded field recoverable or utility-irrelevant.

## The four reductions

1. **Current-trick fold** [Theorem — proved, rec Math §7.16.1, PLAY-12]: an unresolved
   trick folds to `χ = (q, r, w, z)` — led context, ordinal of the current best trick
   key among the ≤13 competitive tiles of that context, current winner, pending count.
   Congruence: equal folds (given hands/support, declaration, actor order) have
   identical future legality, winner updates, and reward. Bounds [PLAY-13]:
   competitive chain ≤13; pending count ∈ {0,5,10,15,20,25}; transparent 12-bit field
   encoding (3+4+2+3), explicitly *not* claimed globally minimal packing.
2. **Actor from capacities** [Theorem — proved, §7.16.2, PLAY-14]: in an open trick the
   four remaining-hand sizes take two adjacent values; the low seats form one clockwise
   interval whose start is the leader, length the plays made, successor the next actor.
   Only at a trick boundary must the leader be stored explicitly.
3. **Score recovery** [Theorem — proved, §7.16.3, PLAY-15]: total banked points
   `= j + 35 − c(H_m) − c(U) − z` — storing both partnership totals is redundant.
   Moreover the needed score residue is **utility-relative** [PLAY-16]: differential
   utilities need only transition rewards; a point contract needs a capped remaining
   threshold; a mark contract needs one sweep-alive bit; match utility needs
   marks/shaker. No universal minimal accumulator is claimed.
4. **Reduced kernel** [Theorem — proved, §7.16.4, PLAY-17]:

```
K = (δ, H_m, N, τ, α_U)
    declaration · viewer hand · minimal exact support normal form
    · boundary-leader-or-folded-trick · named utility accumulator
```

   determines viewer legality, hidden-action acceptance + exact support transition,
   actor order, trick winner/reward, terminal detection, and the utility accumulator
   transition. Played attribution and raw void masks live only in the *evidence*
   record `e` when a policy/belief/utility actually reads them — history is retained
   "only in the factor where history acts" (rec Math §15).

## Future equivalence: what "minimal state" even means

**[Theorem — proved, rec Math §12.10, QUO-10]** For a finite reachable deterministic
machine and a *named output contract* (legality, physical outputs, support outputs,
rewards, terminal labels…), define `x ≡ y` iff every action word yields the same
complete legality/output trace. This is a right congruence; the quotient is, up to
isomorphism, **the unique smallest deterministic exact realization**, and every exact
realization factors onto it — a mechanical Myhill–Nerode theorem. Verified by
partition refinement vs future-trace search on all 5,898 ≤3-state two-action
binary-output machines [QUO-11].

Consequences:

- There is **no output-independent "minimal game state"**: a support-aware contract
  cannot merge states that a purely physical contract can.
- The reduced kernel `K` is proved *exact*; whether `K` **equals** the
  future-equivalence quotient for a given rich contract is deliberately left open
  (rec "Not established" #1; merged [OPEN-01](open-problems.md)). Distinct kernels
  could conceivably be future-equivalent (e.g. via unscored transports or deeper
  coincidences).

## The final factorization (rec Math §15)

```
declaration algebra + owned marked hand + minimal exact hidden support
+ folded physical play residue + utility accumulator
+ retained evidence + augmented belief
```

with continuation field, utility, and strategy class as typed parameters to value; a
complete objective world remains the latent physical witness. Either presentation —
evidence-rich mechanical state deriving `N`, or reduced kernel retaining `N` and
shedding provenance — is exact; **storing both as independent authorities is
forbidden** (this sentence is rec agreeing with v0.7's derived-view repair;
[discrepancies D2](discrepancies.md)).
