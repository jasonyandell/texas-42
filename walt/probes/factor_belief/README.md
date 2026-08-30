# factor_belief — the Slice C (stages C0–C2) and Slice D probes

**EXPLORATORY — sits below every evidentiary tier and is cited by nothing
above it.** A probe number becomes quotable only by brief amendment that
adds it to a verifier receipt.

Instruments: `walt/walt/src/bin/factorbelief.rs` (Slice C) and
`walt/walt/src/bin/factorrecursion.rs` (Slice D). Gates (the CI-checked
part): `walt/walt/tests/solver_factor_belief.rs` and
`walt/walt/tests/solver_factor_recursion.rs`. Mathematical source:
`walt/math/counted_belief_sandwich_v0.1.md` §21–23, §25–26, §46 stages
C0–C2, §47 Slice D, rulings CBS-A6/CBS-A9.

## What it measures

Two routes to the same exact one-ply branch masses `{t ↦ Z_ht}` for the
first hidden seat after the viewer's lowest legal focal play:

- **contraction** — enumerate the acting seat's possible ROOT hands,
  weight each by its exact compatible-completion count (`pair_count`, one
  binomial per hand), classify each hand ONCE by the field (§21's boxed
  equation);
- **enumeration** — classify every complete world of the fiber one by
  one.

Route parity and `Z_h = Σ_t Z_ht` are asserted on every row, and
independently gated in CI on the same roots.

## run1.txt readings (2026-08-30)

- **The §22 representation change is real.** Section C: the opening root
  (h0-t1, fiber 399,072,960) yields its exact branch table in **8.7 ms**
  — 116,280 acting-seat hands classified once each, completion weights
  8.5 ms of that, no complete world ever materialized. The fiber mass
  itself is counted by the shipped DP in 13 µs. Conservation is exact:
  the fifteen branch masses sum to 399,072,960 on the nose.
- **Distinct information states = distinct hands, on both routes.**
  Section B: the level-0 field's insert-only cache materializes exactly
  as many states as the contraction route classifies hands (6/6, 15/15,
  28/28, 74/74) — the enumeration route's extra work is all cache hits.
  At these fiber sizes (worlds/hands ≈ 3) the two routes therefore cost
  about the same; the contraction advantage is the RATIO, which at the
  opening root is 3,432 and at richer roots is unbounded by any sample.
- **The trivial-field contraction is µs-scale everywhere** (Section A:
  4–14 µs against 4–94 µs enumeration; the gap already grows with fiber
  size at 200 worlds).
- **The branch tables differ between fields** (lowest-first vs the σ0
  modeled mind pick different tiles from the same hands — e.g. h4-t6's
  2-2/5-1 masses move). Nothing here ranks fields; the probe only
  demonstrates that the posterior machinery tracks whichever field is
  declared.

## opening_level0_run1.txt readings (2026-08-30)

`factorbelief opening-level0` — the deliberate expensive coordinate:
level-0 (`n0 = 2`) classification of all 116,280 opening hands — §46
stage C2's shape realized with the σ0 field.

- **The exact opening action distribution of the modeled mind exists and
  costs 5.6 seconds.** All 116,280 hands classified (~48 µs per
  modeled-mind read), 20 distinct branch tiles, conservation exact over
  399,072,960. The §22 claim is now a measurement: the first hidden
  seat's exact posterior action distribution under σ0 never touches a
  complete world.
- **Cost attribution is total:** counting is 8.5 ms of the 5.6 s —
  field classification is >99.8% of the bill, exactly the compression
  problem §22 predicted ("a separate compression problem, not a reason
  to retain complete worlds as the belief representation").
- **No reuse at the opening root:** every hand is a distinct
  information state (116,280 materialized), so caching buys nothing
  within one root — cross-root and cross-action reuse is where C1's
  cache study must look, and hand-class instrumentation (Slice F) is
  the declared route to compressing the classifier itself.

## cache_run1.txt readings (2026-08-30) — the stage-C1 cache study

`factorbelief cache` — the cached σ0 field under the contraction route,
gated in CI by `solver_factor_belief.rs` gates 7–10 (bundled one-ply
parity with full extensional cache identity, once-per-state, the zero
cross-history law, the opening root once-per-hand).

- **The two routes materialize the SAME states with the SAME actions.**
  On all six receipt fibers, the contraction route's cache and the
  bundled one-ply oracle's cache (fresh instance each) are equal as
  maps — the feasible root hands exactly, one action each. Parity of
  buckets AND parity of materialized information states, in one map
  equality.
- **Identity cost at scale is 200 ns/query.** The opening root: first
  σ0 contraction 5.36 s (46 µs/hand over 116,280 hands — the field
  classifier, as §22 predicted); the REPEAT contraction 23.3 ms with
  zero classifications — counting plus full-key cache identity only,
  a ×230 saving. Conditioning on the heaviest branch (1-0, mass
  125,370,960 of 399,072,960) costs 16.8 ms and materializes 0 new
  states: at the voidless opening the whole support is already
  classified, so the posterior update is pure table filtering.
- **Cross-history reuse is exactly zero, and that is the finding.**
  0 of 36 queries hit across focal candidates (h4-t6, two candidates)
  or across roots (h12-t6, same instance): the full §43 identity key
  carries the public history, so no two histories share a state.
  Sharing across histories requires a proven state reduction — the
  Slice F vocabulary — never a looser key (a hit under an omitted
  coordinate is the PiKey defect reborn, CBS-A6).
- **Within-history reuse is total.** A repeated contraction and every
  conditioning answer entirely from cache (gates 8/10); the σ0
  first-call cost at trick 5–6 roots is 2–28 µs/hand, repeats 5–17 µs
  total.
- **At small worlds/hands ratios the bundled and contraction routes
  cost the same** (both are bounded by distinct-hand classification);
  the contraction advantage remains the ratio, 3,432 at the opening.

## c2_run1.txt readings (2026-08-30) — the stage-C2 opening-root report

`factorbelief c2` — §46 stage C2's seven required coordinates from ONE
run at the frozen `verify_player` root h0-t1 under the σ0
`Level0 { n0 = 2 }` field. Earlier records had most of these numbers
piecemeal; C2's deliverable is that they are one measurement of one
run, plus the memory coordinate that was deferred until now.

- **The seven, as reported.** hands 116,280 (asserted); contraction
  5,933 µs for the completion weights alone and 21,818 µs warm
  (weights plus full §43-key identity, zero classifications); field
  classification 5,339,731 µs derived by subtraction (cold 5,361,549 µs
  minus warm), 45 µs/hand, 99% of the cold pass; 20 distinct branch
  tiles; cache reuse ×245 with 187 ns/query identity cost; memory
  23,563,392 bytes accounted for the action cache against a 63,340,544-
  byte measured resident size; conservation exact at 399,072,960.
- **The memory coordinate is two figures and they are not
  interchangeable.** The DECLARED ACCOUNTING is arithmetic over
  `size_of::<(FieldStateKey, Domino)>() = 88` bytes, the documented std
  map growth policy (262,144 buckets for 116,280 entries), one control
  byte per bucket, and the key's one-tile history Vec — 23,563,392
  bytes for the cache. The MEASUREMENT is resident set size: 63,340,544
  bytes maximum under `/usr/bin/time -l`, agreeing to the byte with the
  in-run `/bin/ps` reading at exit; peak memory footprint 62,390,680
  bytes. The gap is the completion-weight vector, the receipt, the
  kernel, and allocator slack. Neither figure is offered as the other,
  and no estimate is dressed as a measurement.
- **The bill is still the classifier, now stated as one ratio.** 99% of
  the cold pass is σ0 classification; the counting side of the same run
  is 5.9 ms. At 3,432 worlds per hand the representation change is what
  buys the run at all — no complete world is materialized at any point,
  and the enumeration and bundled routes are deliberately absent.
- **Support shrinkage, beyond the seven (§26 item 5).** Conditioning on
  the heaviest branch (1-0, mass 125,370,960) costs 15,864 µs,
  materializes 0 new states, and leaves 36,530 of 116,280 hands in
  support: at the voidless opening the posterior update is table
  filtering over an already-classified support.
- **No new CI gate, deliberately.** C2 is a REPORT stage and every
  invariant its run asserts is already gated: the opening root's hand
  count, once-per-hand σ0 classification, repeat-is-pure-identity, and
  `Z_h = Σ_t Z_ht` over 399,072,960 by gate 10 (and by gate 6 under the
  trivial field); branch-table parity with enumeration and the
  bundled route by gates 1–9. Nothing new is cheaply assertable — at
  small fibers the distinct-action count is implied exactly by the
  existing table-equality gates, and the memory figures are properties
  of a run, not laws.

## recursion_run1.txt readings (2026-08-30) — the Slice D probe

`factorrecursion report` — the §23 factorized fixed-policy recursion
over the support backend (`SupportOracle`), gated in CI by
`solver_factor_recursion.rs` (5 gates: C0-domain parity with backend
zero, surviving-world mass parity beyond it with backend zero's refusal
at the two-table boundary, value parity with the bundled walk under the
trivial field and under σ0, and the every-node checker).

- **The §47 value gate holds on every row.** Six trick-5/6 roots × two
  frozen focal policies × two fields, plus four trick-4 roots (16
  post-root plies): the factorized success mass `M` equals the bundled
  walk's wins exactly, with `V = M/Z` the exact value pair. The deepest
  row (h4-t4, fiber 34,650, σ0) maintains the exact posterior through
  121,868 conditionings and lands the same 25,039/34,650 as the bundled
  route.
- **Conditioning beyond one ply required a record-consistency law, and
  σ0's own purity fence found it.** A deep uniform support still
  contains hands holding tiles another seat has already played; such
  hands carry zero completion weight in every contraction, and their
  action likelihood is undefined (their information state contradicts
  the record). The trivial field classified them silently; the σ0
  field's type-enforced information-state constructor refused, and the
  fix is the lawful one: `condition` drops record-inconsistent hands
  (provably zero-mass) instead of classifying them. At one ply the
  filter is a no-op — the C1 conditioning-support law is unchanged.
- **The decided cutoff prunes at every depth, including the root.**
  Three rows (h12-t6, h10-t6, h12-t4) settle with ZERO recursion nodes —
  the bid is already made or missed at the root, so `M ∈ {0, Z}` by the
  monotone cutoff alone. Elsewhere the early/terminal decided split
  shows most leaves settling before terminal depth under the trivial
  field.
- **At small worlds/hands ratios the bundled walk is FASTER** (σ0
  h4-t4: 6.8 s recursion vs 2.7 s bundled; trivial-field rows ~3–4×) —
  the recursion pays per-node contraction and support classification
  where the bundled walk shares one tree. The contraction advantage
  remains the RATIO of worlds to hands (≈3 at these fibers, 3,432 at
  the opening root); nothing here measures the opening-scale recursion,
  which is deliberately not attempted (its conditioned completions walk
  116,280-hand tables per node — a later slice's coordinate).
- **The recursion route materializes MORE σ0 states than the bundled
  route** (h3-t4: 146,342 vs 52,322) — conditioning classifies the
  record-consistent support unfiltered by completions (the C1 law at
  depth), so zero-completion hands the bundled walk never meets are
  classified once each. Reuse across the recursion's own nodes is
  already total (the C1 within-history law); compressing the classifier
  itself stays Slice F's problem.

## Boundaries

- Deterministic fields only; a stochastic field needs an explicit tape
  factor (CBS-A6 boundary obligation) and has no entry point here.
- Backend zero contracts at most one conditioned factor (the declared C0
  domain, still refused by panic beyond it); the general contraction is
  `SupportOracle` (Slice D), gated by parity on the C0 domain and by
  surviving-world enumeration beyond it.
- The recursion evaluates ONE frozen focal policy under the declared
  field (§47): no maximization, no best-response claims, no
  play-strength readings. Focal optimization inside a grammar is
  Slice E.
- The opening-root recursion is not attempted; only the opening-root
  ONE-PLY contraction is measured (the Slice C probes above).
