# The endgame store — precomputed floor, lazy fill, forward pathfinding
# (design for adjudication)

Status: DESIGN, awaiting walt-math rulings (E-Q1..E-Q7) in
`walt/CENSUS-RULINGS.md`. One-author rule unchanged. Everything standing
inherits: F1–F7, r3 Q1–Q5, Y1–Y3, P-A1..P-A21, X-A1..X-A19 (especially the
persistence discipline X-A16..A19, which this probe would be the first to
implement). Tier: exploratory throughout.

## Jason's direction (2026-08-11)

Build from the end back and memorize it; then pathfind forward to known
solutions as an alternative fiber-evaluation order. Same fiber, different
enumeration order, leveraging precomputed outcomes. Expectation: "lots and
lots of convergence late game"; precompute only the cheap end (the floor),
let the rest build lazily; walls hit = things learned.

## The mechanism (and why S5h's negative does not apply)

S5h: cone identity cannot short-circuit descent — a state must be fully
expanded before its r3 class is known, so the class store never beats the
identity cache at first build. The proposed escape: at the boundary of a
COMPLETE precomputed level, the lookup key is not the cone hash but the
TRANSPORT key — the r1 canonical form (structural, reads tiles and
relations, never the future; cheap on 4–8 tile endgame states). Same
canonical form ⇒ r1-equal ⇒ (r1 refines r3, receipt Q5.1) r3-equal ⇒ same
Lemma-V value. Forward evaluation then recurses only to grade 4k and looks
up, never expanding below the boundary — where the bulk of every tree's
nodes live.

Two layers:

- **The floor (k = 1, COMPLETE):** the a1 run's domain — every pip-trump
  last trick, 55,036,800 situations, 64 classes. Build once as
  canonical-form → (class, value); a lookup miss at the floor is a bug
  (stop and report), never a fill.
- **The lazy store (k = 2, maybe 3):** cannot be brute-forced (~10^10 raw
  at level 2) and does not need to be — first reach of a level-2 boundary
  state pays its cone once (computed against the floor below it), stores
  under the canonical form; every later reach, any world, any coordinate,
  any hand, is a lookup. Persistent across runs per X-A16..A19.

## Measurement arms (anti-strawman discipline inherited)

Same fibers, same decimation as S5h (rungs n=4: 13 hands, g=7919, W=240;
n=5: 4 hands, g=104729, W=24). Values must be bit-identical across arms
(P-A9-style receipt).

- **T0** = A1 alone (S5h control, unchanged) — the competitor to beat.
- **T1** = A1 + the complete level-1 floor (descent stops at grade 4).
- **T2** = A1 + lazy level-2 store, COLD (created empty, filled during the
  run, per-coordinate stats).
- **T3** = A1 + lazy level-2 store, WARM across coordinates (the store
  carried hand to hand in declared order; counts store-relative, declared
  per X-A19).

Honest framing: within ONE coordinate, A1's state key already catches
repeats, so the tablebase's edge is exactly cross-coordinate/cross-hand
novelty — states new to this fiber but old to the store. T3 vs T0 is the
headline; T1 vs T0 isolates the floor's contribution.

Saturation curves: store size and hit rate vs coordinates processed in
declared order — the direct measurement of "lots of convergence late game."

## Design questions

E-Q1 (the transport-lookup chain). Canonical form (r1, FINEST spec) as the
lookup key: form-equal ⇒ value-equal rests on r1's lawfulness, whose ECL
receipt was checked exhaustively on specific carriers — lookups will hit
states outside any checked carrier. What receipt keeps a hit honest? Proposal
(X-A17 style): for a declared sample of hits (stride over hit sequence),
expand the cone anyway and assert value agreement; mismatch = stop and
report per NO-RESCUE, and it would be a genuine mathematical event (an r1
transport unlawful off-carrier). Rule the receipt and its stride.

E-Q2 (the floor's completeness claim). May the run treat level 1 as
COMPLETE (miss = bug, stop and report)? What must the floor build assert
in-run (the closed-form domain count; the 64; agreement with the committed
a1 receipt)? Note the a1 result file is the standing record; the floor
build re-derives it and must byte-agree on the numbers it restates.

E-Q3 (record contents). Cone-intrinsic only (X-A18): canonical form key →
(value under the frozen operator; r3 class id optional). Rule what is
mandatory, what is forbidden, and whether the stored representative's
signature bytes (X-A16 collision discipline) are required at both layers.

E-Q4 (what the lazy insert pays). Proposal: on a level-2 miss, compute the
value by the same A1 recursion WITH floor lookups below (so the insert cost
is one-trick-deep, not two), and store (form → value). The r3 class id is
then unknown for lazy records — acceptable, or must the insert run the
retrograde signature too (costlier, buys class identity)? Rule which record
the store holds.

E-Q5 (the pathfinding framing). "Same fiber, different enumeration order
leveraging precomputed outcomes" — the arms change evaluation ORDER, never
the object; bit-identical values are the receipt. Confirm the framing and
the receipt.

E-Q6 (persistence mechanics, first implementation of freeze 14). On-disk
append-only file under `walt-factory/store/` (a CACHE, not a receipt: NOT
committed to git, regenerable, gitignored — rule on this), header carrying
the freeze-set digest; loader verifies stored signature bytes on every
insert-collision (X-A16); a warm run re-derives a declared sample and
asserts agreement before quoting anything (X-A17). Rule the file discipline
and what the results file must print about store provenance (which carriers
contributed, X-A19).

E-Q7 (results discipline). One file `results/endgame_store_2026-08-11.txt`;
inherited boilerplate; per-arm per-coordinate rows; saturation curves;
declared stops; both-outcomes framing (a weak T3 dividend = the convergence
hypothesis measured small on this corpus — a result).
