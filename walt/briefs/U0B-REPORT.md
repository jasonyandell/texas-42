# U0B-REPORT — the in-solve horizon census, as built

**Slice:** U0b, the §38/§40 God-gap census of
`walt/math/salvation_complex_v0.1.md` run INSIDE the solve — at every
belief node the exact recursion reaches at a declared depth below a root
— together with the exact root consequence of a §39 fusion cut at that
depth. Authorized 2026-09-03 by Jason ("do the carry fix and the horizon
census, your way buddy"). **Status: COMPLETE.** New module
`solver/horizon.rs`, new probe `bin/horizonreport.rs`, 5 gates in
`walt/walt/tests/solver_horizon.rs` (142 s), record
`walt/probes/factor_belief/horizon_run1.txt` (53 censuses, 19 min 16 s).
Two visibility-only edits outside new files: `doom.rs`'s line-walk
internals are `pub(crate)` so the census runs the doom census's OWN
per-world make check rather than a copy (§47/SC-A3 — no behaviour
change; gate H2 checks the two agree), and `FactorBelief` gains a
`public_state()` snapshot accessor.

**EXPLORATORY tier throughout.** The horizon is a measurement on the
declared corpus, never a theorem (SC-A4). Nothing here is a
play-strength claim or a substitution; it measures whether the
substitution would be exact, and by how much it would not.

---

## THE QUESTION, AND WHY IT NEEDED A NEW INSTRUMENT

U0 measured the fusion horizon at receipt-root coordinates: fourteen
trick-5/trick-6 coordinates God-tight, twelve trick-4 coordinates
carrying a 6–22‰ information price. That is fourteen points, all
uniform roots with the viewer to lead. The §39 fusion-cut substitution
would be applied at the trick-5 belief nodes a trick-4 solve actually
REACHES: conditioned posteriors, any seat to move, hundreds to thousands
of them per root. The question U0 could not ask is whether God-tightness
holds THERE.

## WHAT WAS BUILT

`horizon_census(oracle, root, position, field, spec)` descends from the
root exactly as `response_success_mass` does — focal nodes over every
legal action, hidden nodes over every branch tile with the posterior
conditioned — and stops at the declared cut depth. Each frontier node is
priced three ways: exact `Q(B)` by the §48 recursion; `U^God(B)` by a
per-world make check over the worlds the node represents (the posterior
under a deterministic field is uniform on its surviving worlds, asserted
per world); and `Φ = U^God − Q`. Nodes decided by the §5 arithmetic
before the cut are recorded with their mass, not descended. Nodes above
a declared fiber cap are typed `Refused` and counted; nothing is dropped.

The same descent re-prices the root twice: with exact frontier values
(which must equal `response_success_mass` at the root — asserted in the
function and in gate H1) and with the God uppers substituted at the
frontier — what a fusion cut at that depth would compute. The two root
values and the two root argmaxes under the declared lowest-tile tie rule
are the practical reading.

## THE GATES (`solver_horizon.rs`, 5, 142 s)

- **H1** the frontier re-descent reproduces the root on ten roots × two
  contracts × two cuts (40 censuses, >200 priced nodes); every node's
  `Q ≤ U^God`; every priced node sits exactly at the cut.
- **H2** at cut depth 1 the frontier IS U0's root-action coordinates, and
  the census's per-node doomed count equals `doom_enumeration`'s, its
  upper equals the enumeration's, its `Q` equals the exact response — on
  all eighteen coordinates (U0's fourteen plus h8-t4's four). The world
  enumeration and line-state construction are the doom module's,
  checked rather than trusted.
- **H3** the cut never under-prices; equal to the exact root whenever
  every priced frontier node is God-tight; strictly above on a specimen.
- **H4** refusals typed and nothing dropped under a tiny cap: the same
  nodes, the exact side complete, the cut side absent (never a number),
  the argmax and over-pricing absent with it.
- **H5** determinism.

## THE FINDINGS (`horizon_run1.txt`)

Corpus: the four trick-4 gated roots × cuts 4 and 8 (the trick-5 and
trick-6 frontiers) × contracts {receipt 30, 33, 36, 39, 42}; the two
trick-5 roots at cut 4; the smallest trick-3 receipt root at cut 4 (its
trick-4 frontier). Node fiber cap 40,000 (12,000 at the trick-3 root);
nothing was refused anywhere.

### 1. The trick-5 frontier inside a trick-4 solve is NOT fusion-free

At the receipt contract, the share of trick-5 frontier nodes carrying a
positive price is 9% at h8-t4 (40 of 466), 22% at h3-t4 (171 of 779) and
31% at h4-t4 (384 of 1,228). Mass-weighted, the frontier price is 13–14‰
at every one. U0's "trick 5 is fusion-free" was true of fourteen uniform
receipt roots with the viewer on lead and is false of the conditioned
trick-5 nodes a trick-4 solve reaches.

A consistency check that is NOT a finding: at these roots the viewer
leads trick 4, so no focal decision lies between the root action and the
trick-5 frontier, and the cut-4 root over-pricing is exactly U0's Φ at
the root action — 1/60 at h8-t4 3-3, 1/66 at h3-t4 3-1, 157/11550 at
h4-t4 6-5. The census reproduces U0's twelve prices through partition
additivity, which is what it must do; the new content is WHERE on the
frontier the price sits and what a cut there does to the play.

### 2. The trick-6 frontier is nearly exact in value — and still flips a play

At cut 8 the root over-pricing is 0‰ on every h3-t4 contract (exactly 0
at 30, 33, 36), 0–5‰ at h4-t4, and 1–7‰ at h8-t4. The mass-weighted
frontier price is 0‰ at h3-t4 and 1–11‰ elsewhere. But at h8-t4 under
contracts 36 and 39 the 7‰ over-pricing FLIPS the root argmax. Two rows
of thirty: a trick-6 cut is exact to a few per mille in value on this
corpus and is not a safe substitution for the decision.

### 3. The price rises with the contract, then falls when the root decides

| root  | cut | bid 30 | 33 | 36 | 39 | 42 |
|-------|----:|-------:|---:|---:|---:|---:|
| h3-t4 | 4 (t5 frontier) over-pricing ‰ | 15 | 6 | 5 | 9 | 3 |
| h4-t4 | 4 | 13 | 65 | 85 | 105 | 33 |
| h8-t4 | 4 | 16 | 19 (flips) | 102 (flips) | 102 (flips) | decided |
| h4-t4 | 8 (t6 frontier) | 0 | 3 | 5 | 4 | 0 |
| h8-t4 | 8 | 2 | 1 | 7 (flips) | 7 (flips) | decided |

At h4-t4 a trick-5 cut over-prices the root by 13‰ at bid 30 and 105‰ at
bid 39; at h8-t4 by 16‰ at 30 and 102‰ at 36, where it flips the play.
The receipt contract is the friendliest one on this corpus, and U0's
single-contract horizon reading was taken at it. h12-t4 is decided at
the root under every contract (frontier 0, the §17 zero-cost path).

### 4. The trick-3 root: the first exact trick-3 value, and a cut that changes the play

h8-t3 (fiber 59,976) solved EXACTLY under σ0 in 14 min 13 s (289,407,472
field consultations): `Q* = 28859/29988` (962‰), argmax 1-1. MB1's
31-minute refusal at this root was the eight-profile mixture; the
single-field world recursion completes, so a trick-3 exact value is
affordable offline and the affordability wall MB1 located is the
mixture's, not the recursion's.

Its trick-4 frontier: 2,098 nodes, 624 with a positive price (30%),
mass-weighted 37‰, max 500‰. A fusion cut there over-prices the root by
31‰ and FLIPS the play, 1-1 → 3-3 (cut values 992‰ vs 993‰ against exact
962‰ vs 955‰). The report's second run of the same root under its own
cap reproduces every number.

### 5. The three-layer picture, on this corpus

- a cut at the trick-6 frontier: 0–7‰ in value, flips 2 of 30 rows;
- a cut at the trick-5 frontier: 3–105‰, contract-sensitive, flips 3 of 15
  substantive rows;
- a cut at the trick-4 frontier: 31‰ and flips, on the one root measured.

The §39 substitution as a live-player backend therefore needs either a
trick-6 frontier (cheap, near-exact, occasionally wrong on the play) or
gluing work at the trick-5 nodes that carry the price — and the census
now lists every such node with its history, mass, doomed count and gap,
which is the U1 salvation-mask work's input.

## WALL

19 min 16 s for the 53-census report (h4-t4 ≈ 30 s per census, h3-t4 ≈
4–10 s, h8-t4 ≈ 2–3 s, decided roots microseconds, h8-t3 13 min 17 s);
the trick-3 scout 14 min 13 s standalone. Wall is the only approximate
number here; reads are exact and printed per census.

## DEVIATIONS AND BOUNDARIES

1. **Deterministic fields only.** The per-world enumeration asserts unit
   weight on every surviving world; a stochastic field needs a tape
   coordinate this slice does not build.
2. **`doom.rs` touched for visibility only.** `WalkFrame`, `LineState`,
   `LineCtx`, `line_apply`, `line_can_make` became `pub(crate)`; no line
   of logic changed, and H2 is the check.
3. **The record is large** (1.4 MB): every positive-gap frontier node is
   listed with its history, so the U1 input is in the file rather than
   only in the type. God-tight nodes are counted, not listed.
4. **Contract variation is a different field per contract** (σ0 reads the
   bid; the field cache key includes it — checked). The sweep is per
   (root, contract) and the table says so.
5. **No producer, no substitution.** Installing a cut as an exact suffix
   oracle is U4's territory and needs the frontier records' compatibility
   proof (§39), which this census does not construct.

---

EXPLORATORY — below every evidentiary tier; quotable only via gate
receipts. The fusion horizon is a measurement on the declared corpus,
never a theorem (SC-A4).
