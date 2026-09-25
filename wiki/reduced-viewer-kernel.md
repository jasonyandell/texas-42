# The Reduced Viewer Kernel and Future-Equivalence Minimality (rec only)

[Home](Home.md) · owns: the four reductions, the kernel, future equivalence, the
OPEN-01 collapse · Sources: **rec Math §7.16, §12.10, §15** (no v0.7 counterpart).
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
- The reduced kernel `K` is proved *exact*. Whether `K` **equals** the
  future-equivalence quotient was rec's "Not established" #1 (merged
  [OPEN-01](open-problems.md)); exchange dispatch 003 now settles it in the **COLLAPSE**
  direction for the support-aware `P30_DECLARING_POINTS` contract — **`K` is strictly
  finer than the quotient** [exchange-adjudicated CONFIRMED (ALL_PASS 0.43s; 3/3
  SOUND) — external tier ([claim-ledger](claim-ledger.md)), not a corpus theorem,
  not a kernel proof]. Two reachable kernels differing only in the raw fold ordinal
  (`r=7` vs `r=6`; NT, `P(30)`, viewer 0 / bidder 3, identical `ε`, 18-tile full-Ternary
  support normal form) are machine-verified output-equivalent, so the fold ordinal `r`
  is **not an injective memoization coordinate**. Mechanism — the **dead-cut lemma**:
  fold ordinals above the live cut of still-unplayed competitive tiles are unobservable
  through the support-aware contract until trick-boundary coalescence (here the max live
  ordinal 3 `<` 6 `<` 7, so the winner-update predicate is identical), so `r` can be
  compressed by re-ranking through the live competitive tiles. Source
  `exchange/inbox/003-kernel-vs-future-quotient.md`, verified program
  `exchange/adjudication/programs/003.py` + `witnesses/003.json`; product statistics
  204 / 22,848 / 1,604 / 1,280 reproduced and independently re-derived. (A provenance
  blemish in the response's claimed verifier SHA is recorded, non-load-bearing, in
  [discrepancies](discrepancies.md).)

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

## Mechanization status (proof-assistant kernel tier)

Rows are **v0.7** `65_MECHANIZATION_LEDGER.md` `PA-` rows (priority in parentheses);
"proved" = a declaration under `lean/Texas42/` checked by the Lean kernel over at most
`propext`/`Classical.choice`/`Quot.sound`, with no `sorry`, `native_decide` or local
axiom (grep re-verified 2026-09-12), as of commit d190b26 (2026-08-02; all 42
priority-0 rows closed). Map: [lean-row-index](lean-row-index.md). A kernel theorem
never promotes a corpus status; rob receipts are conformance evidence, never a status
change; the OPEN-01 COLLAPSE (x:003) is **exchange-adjudicated tier — external, not a
kernel proof** and stays labelled so.

**PLAY-12 through PLAY-17, QUO-09 through QUO-11, FAC-02 and the rec §15
factorization are not mechanized.** All are rec-only rows with **no `PA-` row** in
the v0.7 ledger (which predates rec's mathematics); rec's `60_PROOF_ASSISTANT_KERNEL.md`
lists them as spine sections **K12 (Folded play/support kernel)** and **K14
(Future-equivalence minimum)**, for which no Lean declaration exists. [FINDINGS](FINDINGS.md)
§6 flags PLAY-17's utility-accumulator interface ("supplies exactly the utility
residue not represented as transition reward") as the boundary where double-counting
bugs live — a prose proof composing four congruences, unmechanized.

| Object on this page | Ledger row (priority) | Kernel status (d190b26) | Declaration (`lean/Texas42/`) |
|---|---|---|---|
| The objective reduced play state the fold reduces — hands, leader, trick prefix, banked scores, contract; `tricksDone` and `scoredTiles` **derived**, never stored (TYPE-02) | PA-B07, PA-B08, PA-B09, PA-B10 (0) | **proved** (legal set; invariant-preserving step; seven tricks, 28 plays, 42 points) | `Play.lean` `PlayState`, `:359` `inv_step`, `:559` `terminal_scores` |
| `N`, the minimal exact support normal form (the kernel's third component) | PA-D01..D05 (0) | **proved** | `NormalForm.lean:1423` `fiber_eq_iff_totalNF_eq` |
| Hidden-action acceptance and exact support transition (the typed transitions) | PA-C09, PA-C10 (0) | **proved** | `Cells.lean:1075`, `:717` |
| PLAY-12/13 current-trick fold `χ = (q, r, w, z)`, chain ≤ 13, six pending values | none (rec K12) | **not mechanized**; rob `r_alg_competitive_ordinal` "max 13" conformance; `verify_reduced_kernel.py` "folded trick: 737,100 trick cases; 2,211,300 sequential updates" re-run 2026-09-12, identical | — |
| PLAY-14 actor from capacities (84 shapes); PLAY-15/16 score recovery (3,132 prefixes), utility-relative residue | none (rec K12) | **not mechanized** | — |
| PLAY-17 / FAC-02 reduced viewer kernel `K = (δ, H_m, N, τ, α_U)` | none (rec K12) | **not mechanized** | — |
| QUO-01 / PLAY-07 physical Markov congruence (the reduced play state is Markov for the hand) | PA-B12 (1) | **open** | — |
| QUO-09/10/11 mechanical Myhill–Nerode future equivalence (5,898 machines / 17,560 pairs) | none (rec K14) | **not mechanized** | — |
| OPEN-01 COLLAPSE: `K` strictly finer than the future-equivalence quotient (dead-cut lemma) | exchange tier x:003; no row | **not a kernel proof**; not reproduced by rob either (a slice-03 regression target, [rob-slices](rob-slices.md)) | — |
| "Storing both authorities is forbidden" (D2) | design discipline | realized by construction: `Cells.lean` computes cells as derived views of the public record, never as fields | `Cells.lean` module docstring |
