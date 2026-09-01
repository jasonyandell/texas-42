# factor_belief — the Slice C (stages C0–C2), D, E, F, G, Phase 1/2/3/4/5/6/7/8 and doom-census probes

**EXPLORATORY — sits below every evidentiary tier and is cited by nothing
above it.** A probe number becomes quotable only by brief amendment that
adds it to a verifier receipt.

Instruments: `walt/walt/src/bin/factorbelief.rs` (Slice C),
`walt/walt/src/bin/factorrecursion.rs` (Slice D),
`walt/walt/src/bin/factorresponse.rs` (Slice E),
`walt/walt/src/bin/factorcegar.rs` (Slice F),
`walt/walt/src/bin/factorrefine.rs` (Slice G),
`walt/walt/src/bin/factorprofile.rs` (anytime proof-state Phase 2),
`walt/walt/src/bin/proofreport.rs` (anytime proof-state Phase 3),
`walt/walt/src/bin/extractreport.rs` (anytime proof-state Phase 6),
`walt/walt/src/bin/frontierreport.rs` (anytime proof-state Phase 1) and
`walt/walt/src/bin/bellmanreport.rs` (anytime proof-state Phases 4/5),
`walt/walt/src/bin/laydownreport.rs` (anytime proof-state Phase 7) and
`walt/walt/src/bin/openingreport.rs` (anytime proof-state Phase 8) and
`walt/walt/src/bin/doomreport.rs` (the doom census — the §70
structural producer, ∀-fail dual of §16).
Gates (the CI-checked part): `walt/walt/tests/solver_factor_belief.rs`,
`walt/walt/tests/solver_factor_recursion.rs`,
`walt/walt/tests/solver_factor_response.rs`,
`walt/walt/tests/solver_factor_consequence.rs`,
`walt/walt/tests/solver_factor_refine.rs`,
`walt/walt/tests/solver_factor_profile.rs`,
`walt/walt/tests/solver_proof_state.rs`,
`walt/walt/tests/solver_proof_regret.rs`,
`walt/walt/tests/solver_extraction.rs`,
`walt/walt/tests/solver_frontier.rs`,
`walt/walt/tests/solver_residual.rs` and
`walt/walt/tests/solver_covers.rs` and
`walt/walt/tests/solver_laydown.rs` and
`walt/walt/tests/solver_opening.rs` and
`walt/walt/tests/solver_doom.rs`. Mathematical source:
`walt/math/counted_belief_sandwich_v0.1.md` §11–12, §21–23, §25–31,
Part VIII §32–37, §46 stages C0–C2, §47 Slice D, §48 Slice E, §49
Slice F, §50 Slice G, rulings CBS-A4/CBS-A6/CBS-A9; and
`walt/math/anytime_proof_state_score_v0.1.md` §2–§13, §18, §22–§23,
§24–§33, §35, §15–§17, §39–§44, §49–§56, §60–§65, rulings
APS-A2/APS-A4/APS-A6/APS-A7/APS-A8/APS-A9.

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

## response_run1.txt readings (2026-08-30) — the Slice E probe

`factorresponse report` — the §48 factorized grammar best response
(`grammar_success_mass`: the §23 recursion with the focal case's frozen
action replaced by a max over the grammar's actions), gated in CI by
`solver_factor_response.rs` (4 gates: per-root-action parity with the
Slice B enumeration split under σ0, singleton-grammar collapse to the
Slice D recursion, source dominance with the binding check, and the
every-node checker with the grammar max structure enumerated).

- **The §48 witness holds on every checked row.** Under σ0, every
  grammar root action's factorized `Q^G_a` equals the Slice B split's
  grammar optimum exactly — twenty-five action rows across two grammars
  (two-source lowest/highest; three-source adding the
  count-preservation safety policy), six roots, no complete world ever
  materialized on the recursion route.
- **At depth the mix pays — the §11 freedom is real value.** At trick-4
  roots the grammar optimum strictly beats every source policy: h4-t4
  (trivial field) Q^G = 34,650 = Z — the grammar mix makes the bid on
  EVERY represented world — against 34,170 (permille 986) for the best
  single source; h3-t4 Q^G 3,815 against 3,062; h8-t4 1,163 against
  1,073. The information-state-wise combination of sources, §11's "may
  legally combine," is worth 90–753 worlds of make-mass at these
  roots. At trick-5/6 roots the mix NEVER exceeds the best source (all
  gaps zero against the best of three sources) — the room the grammar
  needs grows with depth.
- **At trick 5–6 the two-source grammar saturates everything reached.**
  Every §12 verdict in Section A is "closes" with NO deviating
  continuation at any reached, still-undecided state (`dev = -`) — with
  2–3 legal tiles per endgame state, two sources usually cover the
  whole legal set, so `Q^G = free` on all six roots and the §12
  exclusion is trivially achieved. The gates therefore draw their
  non-vacuity witness from SINGLETON grammars, which bind exactly where
  the two sources' values differ. Grammar-vs-free daylight at richer
  roots is a Slice F/G-era measurement, not assumed here.
- **The honest negative, amplified.** At these fibers (worlds/hands
  ≈ 3) the Slice B enumeration split answers in 30–40× less time than
  the factorized recursion (e.g. h3-t5: 0.6 ms split vs 20–24 ms
  recursion) — same cause as the Slice D finding: per-node contraction
  and support classification against one shared enumeration tree. The
  contraction advantage remains the worlds/hands ratio (3,432 at the
  opening root, unmeasured here — the opening-root recursion stays not
  attempted).
- **The max multiplies the walk, not the law.** The grammar recursion's
  node counts run ~1.3–1.6× the fixed-policy walk per explored action
  (h4-t4 σ0: 146,346 focal nodes, 188,594 focal actions explored,
  561,753 conditionings, 9.8 s vs Slice D's 6.8 s fixed row; 1,652,377
  σ0 states materialized across the maximizing subtrees). Reuse within
  the walk is the C1 within-history law, unchanged; compressing the
  classifier stays Slice F's problem.

## cegar_run1.txt readings (2026-08-30) — the Slice F probe

`factorcegar report` — the §49 consequence CEGAR at the
field-classification bottleneck (`refine_to_action_exact`: §28 hand
classes under the starting vocabulary, §30 witness-pair refinement to
the action-exact endpoint), gated in CI by
`solver_factor_consequence.rs` (4 gates: Theorem 30.1's monotone
narrowing with nested per-branch intervals, endpoint parity with
`branch_masses`, the witness requirement re-derived independently, and
non-vacuity of vocabulary/aggregation/refinement).

- **Mass concentrates fast — the §51 success signal.** At the opening
  root under σ0 (116,280 hands, 399,072,960 worlds), 513‰ of the
  posterior mass sits in action-exact classes at just 5,387 classes
  (21 hands/class), 805‰ at 36,923 classes (3 hands/class): a few
  exact features DO determine the modeled mind's action for most of
  the mass. The refinement machinery itself is cheap — the 16-stage
  loop costs 183 ms of pure partition arithmetic against the 5.4 s
  classification bill it instruments.
- **The tail fragments — the §51 falsifier for the last slice of
  mass.** Driving residual class mass to ZERO under σ0 costs full
  fragmentation: 116,280 singleton classes after 15 refinements (the
  critical set reaches 15 of 21 pool tiles). Under a SAMPLED modeled
  mind the action's tail structure is the sampler's, not the hand's —
  the trivial-field endpoints on the same roots DO aggregate (255/495,
  56/126, 246/330, 147/495 at trick 4), so the fragmentation is a
  property of σ0's sampling, not of the vocabulary. §49's measurement
  discipline ("residual class mass and root interval impact, not
  classifier accuracy") is thereby vindicated as DESIGN guidance:
  carry small residual as per-branch intervals; don't chase the
  action-exact endpoint.
- **Root-interval impact is the usable dial.** The per-branch interval
  `[L_t, U_t]` narrows from a max width of 828‰ of Z (bare vocabulary)
  to 81‰ at 36,923 classes and 32‰ at 70,829 — every stage's widths
  are gated to nest, so a budgeted stop at ANY stage yields sound
  branch-mass intervals.
- **Every refinement carries its witness.** All recorded refinements
  (0–15 per root) name two same-class hands, their differing field
  actions (re-derived through the field itself in the gates), and the
  discriminator tile that entered the §31 critical set — at trick-4
  roots the witnessed critical tiles are recognizably consequential
  (e.g. h12-t4: 1-1 3-1 3-2 3-3 under both fields).
- **What the instrument does not claim.** It pays one field
  classification per support hand — the same bill as `branch_masses` —
  so it measures REPRESENTATIONAL structure (how few classes an §29
  action-exact class verifier would face), never a faster classifier;
  no such verifier is built here.

## refine_run1.txt readings (2026-08-30) — the Slice G probe

`factorrefine report` — the §50 integrated refinement controller
(`refine_root` in `solver/refine.rs`: typed per-action intervals, the
§33 work-item subset, §34 refusals, §35 width-per-cost scheduling, §36
loop, §37 invariant), gated in CI by `solver_factor_refine.rs` (4
gates: escalation parity with the bundled exact authority plus the
containment chain, the soundness invariant walked over full runs,
steering refusals with bytewise determinism, starvation honesty with
the δ ledger).

- **The exact ladder settles every gated root** (Section A: six SETTLED,
  four honest EQUIVALENT exact ties, all with `risk 0`), and twice it
  settles WITHOUT escalating the winner: at h4-t6 and h4-t4 the
  winner's cheap exact-fixed lower (866‰, 892‰) cleared every rival's
  escalated exact point — §36's promise that one lower witness can
  clear all rival uppers, realized on trace.
- **The sampled tier pays for itself exactly where theory says**
  (Section B): on the small t5/t6 fibers, δ bounds settle the root
  before ANY exact recursion runs (h4-t6 at 64 work units against 420
  exact-only; h8-t5 at 3,776 against 13,860) and the result is
  correctly demoted to `delta-qualified`; at trick 4 the prefix-16
  uppers are too loose to prune, the exact ladder does the work, and
  the wasted sampled spend is 128 units of ~237,000 — both regimes on
  one deterministic trace, with the winner agreeing across tiers
  everywhere both settle.
- **Decision width against cumulative cost is the §53 staircase**
  (Section C): at h3-t4 the bar climbs 0‰ → 238‰ → 267‰ → 350‰ as the
  budget grows 1k → 12k → 48k → ample, with the honest UNRESOLVED rows
  carrying the full surviving set and the named fallback until the
  escalations land and the root settles.
- **The affordability cliff is walked honestly** (Section D): at the
  opening root every exact item is refused by its own declared
  forecast (the §40 contraction/field-classification walls, labeled),
  fourteen sampled endpoints produce real δ intervals over the
  399,072,960-world fiber (one upper reaches 974‰), nothing prunes at
  bar 490‰, and the controller returns UNRESOLVED with all seven
  actions, the fallback named, risk 7/10 inside the declared 4/5
  scope. No complete-world enumeration ran.
- **What the controller does not claim.** It schedules and accounts; it
  manufactures no bound (§37.8) — every number is produced by a
  pre-existing authority or by the escalation recursion the gates hold
  to bundled parity. Forecasts are declared heuristics (wrong forecasts
  cost scheduling quality, never soundness), and the fallback choice in
  an UNRESOLVED result is never promoted to a settled claim.

## profile_run1.txt readings (2026-08-31) — the Phase 2 score-profile probe

- **The whole curve costs almost nothing extra where it matters.** The
  profile walk forgoes the decided cutoff (§18's caveat: a decided
  state knows the indicator, not the score), roughly doubling the
  walked nodes at trick 4 — and the wall grows only ~7–12% (h3-t4 σ0:
  1.149 s vs 1.023 s; h4-t4 σ0: 3.316 s vs 2.955 s). The forgone
  subtrees are the cheap late-hand ones; the modeled-mind bill
  concentrates early. One run buys all 42 thresholds where the Boolean
  bought one.
- **Certain outcomes now carry their explanation.** h12-t6's certain
  miss (refine: EQUIVALENT value 0) is the single bin 20:6 — every
  world banks exactly 20 of 30, all of it inside the §10 rescue band at
  d = 10 (1000‰) and none at d = 5: only a ten-point swing could ever
  have rescued it. h10-t6's certain make is 35:1 41:6 42:12 — expected
  score 41.315, fragile only at d = 10 (52‰).
- **The contract sits ON the score spike under σ0.** h8-t5 σ0-as-focal:
  445‰ of the fiber lands EXACTLY at s = 30 = bid (fragile-make at
  d = 1 is 445‰); h4-t6: 600‰ at exactly 30. The modeled mind's
  settled branch reads the bid and stops accumulating once made — the
  profile detects the field's bid-reading behavior as a spike at the
  contract, which is precisely the §4 explanatory promise (which
  points moved, not just whether).
- **The reuse boundary is real and now mechanical.** σ0 reads the bid
  by construction (its settled/desperation branches; the bid is in its
  cache key), so a σ0 profile priced at one contract does NOT answer
  another — gate 3b freezes the specimen (h10-t6: projection at 42
  gives 12, evaluation at bid 42 gives 9). Under bid-blind semantics
  (trivial field, fixed focal) reuse is exact at every threshold tried
  (gate 3). §44's reuse claim is exactly as strong as the semantics'
  bid-blindness.
- **Focal quality is visible in the curve, not just the scalar.**
  h4-t4: σ0-as-focal beats lowest-first at the bid (769‰ vs 722‰) and
  everywhere in the upper tail, with expected score 35.544 vs 35.198 —
  while h3-t4 σ0-as-focal (280‰, 22 distinct scores, expected 19.344)
  shows a genuinely losing position whose 207‰ rescue band at d = 10
  says most of the miss-mass is not even close.
- No root was dropped: all ten gated roots ran under both focals;
  whole probe 18.8 s.

## proofreport_run1.txt readings (2026-08-31) — the Phase 3 §33 blocks

- **Certified regret zero far from certain make.** h5-t6: floor 444‰,
  upper 444‰, Γ = 0 — the recommended policy is CERTIFIED optimal
  under the declared semantics at a thoroughly uncertain value (the
  escalated exact-response uppers pinned U*). Certainty about
  optimality and certainty about making are different things, on
  trace.
- **§30's gap made flesh at h3-t4.** The controller settled the best
  ACTION as 3-1 (Q = 350‰), but the best MATERIALIZED policy in the
  state starts 4-4 at floor 267‰ — 3-1's naive lowest-first
  continuation prices below 267‰, so the recommendation honestly
  plays 4-4 with Γ = 83‰ covering the gap. pmake belongs to the
  policy, not the first tile; the obvious next work item is a
  stronger 3-1 continuation (Phase 6 argmax extraction).
- **Certain outcomes certify in both directions.** h12-t6: floor =
  upper = 0‰, Γ = 0 (a certified unmakeable contract, with the score
  explanation on the same block: floor = ceiling = 20). h10-t6:
  floor = upper = 1000‰, Γ = 0.
- **The bands ride along.** h4-t6 (setting viewer): rescue(d = 1)
  866‰ — the declaring side's mass sits one point under the contract
  on 866‰ of worlds, the setting side's whole case in one number.
- All seven roots completed; whole probe 14.1 s (13.4 s of it the
  h3-t4 refine + three t4 profiles).

## extractreport_run1.txt readings (2026-08-31) — the Phase 6 §30 bridge

- **h3-t4: Γ 83‰ → 0‰ EXACTLY.** The extraction producer materializes
  3-1's optimal continuation — a 12,420-state argmax DAG, ~1.1 s of
  extraction — and the executable bar rises 267‰ → 350‰ to meet the
  proof bar. The recommendation switches from 4-4 (Phase 3's honest
  fallback) to 3-1 under the extracted content id
  (`profile:argmax-full-legal-5357…`), at certified regret zero. The
  §30 gap Phase 3 priced is closed by exactly the §63 machinery the
  parent said would close it.
- **h8-t5: Γ 282‰ → 10‰.** Extraction lifts B_exec 717‰ → 989‰; the
  10‰ residue is the winner's δ-tier upper (1000‰), not an executable
  shortfall — upper-side work, not policy work.
- **The vacuous winner-upper, on trace.** h4-t6 keeps Γ = 133‰ after
  extraction with the action certain: RefineV1 settles on cross-action
  dominance, so the winner's own upper stays 1000‰ and Γ honestly
  refuses to reach zero until an upper fact prices it (gate 6 stage
  two proves the §36 exact upper collapses it to 0).
- **The residual proves where the grammar leaks.** h3-t4, two-source
  grammar: ESCAPE on all four root actions (`m* = dev > gram`, e.g.
  4047 > 3498 on 3-1) — trick-4 room is real and the residual finds
  it, Slice E's trick-4 finding from the other side. At t5/t6 every
  multi-source verdict is `empty-class`: post-root focal states hold
  ≤ 2 tiles, so saturation makes the deviating class literally empty —
  Slice E's "ties free everywhere at t5/t6" explained structurally.
  The singleton grammar escapes at h8-t5 (71 > 64) and closes by tie
  at h3-t5 (dev = gram = 200 = z: everything makes).
- **Extraction is cheap where it matters.** Sub-millisecond DAGs at
  t5/t6; ~0.5–1.1 s per action at h3-t4 (the response walk's own
  price). Whole probe 29.2 s, dominated by the h3-t4 refine (9.7 s)
  plus per-action extractions and the four t4 residual walks; no root
  was dropped.

## frontierreport_run1.txt readings (2026-08-31) — the Phase 1 anytime schedules

- **Goal separation is real money.** h10-t6 and h3-t5 certify Γ = 0
  for ONE baseline profile (1Z spent): the executable floor reaches
  the vacuous ceiling and the ε-goal owes nothing more — while
  `SelectAction` on the same roots costs 7–10Z of upper work. Unlike
  goals, unlike bills (§39).
- **h3-t4 SelectAction never buys a DAG.** Settled at 16Z (baselines
  + four exact values), recommendation honestly 4-4 at Γ = 83‰ —
  action selection did not need extraction, and the schedule knew it.
  Only `RecommendEpsilonPolicy(0)` pays for extraction, and its block
  then reads 3-1 at Γ = 0.
- **h4-t6 SelectAction is 5Z.** One exact upper on 0-0 (333‰ falls
  below the baseline bar 866‰) excludes it; exact(1-1) is never
  bought. Width-per-cost steering is visible at h8-t5 Strengthen too:
  exacts bought in declared-bound order 315‰, 304‰, 282‰.
- **The §41 macro is visibly load-bearing.** Every ε-schedule's upper
  side moves ONLY via `exact-survivors[§41 macro]` — standalone exact
  values are §42-refused from the top state, exactly as the module
  doc proves.
- **One honest waste, recorded, not patched.** Under vacuous uppers
  the §42 bounds cannot rank extractions (every `U_a − B_exec` ties),
  so h3-t4's ε-goal bought all four before the macro priced the
  uppers: 28Z where uppers-first pays ~15Z. §43 verbatim — a poor
  forecast wastes time, it cannot weaken the proof state. Bound
  refinement that prices upper-information value is future frontier
  work.
- **Certain roots strengthen for free.** h3-t5 StrengthenToExact met
  by three baselines alone (3Z): the baseline value 1000‰ meets the
  vacuous upper, so every interval is already a point.
- Whole probe 13.3 s, dominated by h3-t4's three goals; no root was
  dropped.

## bellmanreport_run1.txt readings (2026-08-31) — the Phase 4/5 staircases and covers

- **The staircase is a real anytime object.** h3-t4 walks every action
  from a wide stage-0 interval (3-1: [145,606]‰ over 50 classes) to
  the exact response in 5–6 refinements, narrowing monotonically on
  both sides; mid-staircase stages are honest intermediate answers
  (stage 4 on 4-1: [223,387]‰ at 836‰ exact mass). h8-t5 0-0 climbs
  86‰ → 771‰ exact-mass across five stages. The interval width IS the
  unresolved mass — §22's identity, visible line by line.
- **Decided cells stage for free.** h12-t6 and h10-t6 sit at exact
  intervals from stage 0 (classes 0(0): the post-action node is
  decided) — the staircase never spends where the game already knows.
- **Covers certify BOTH collapses.** h12-t6: verified gain 0 against
  an arithmetic envelope of 7 — the range walk proves no deviation
  moves the declaring score, and the derived upper collapses both
  actions to V* = 0 at range-walk cost (a failure certificate cheaper
  than the response walk). h10-t6: the incumbent is a certain make and
  the rescue band closes at 1000‰. h4-t6: the range walk beats the
  arithmetic envelope by exactly one point (gain 11 vs 12) and leaves
  the 5-5 ten-count hazard VISIBLE at 134‰ on 1-1 — small, nonzero,
  never averaged away.
- **The §70 caveat is live, honestly.** At rich early roots (h8-t5,
  h5-t6, h3-t4) every trick and count tile is still contested: the
  verified gain equals the whole §5 envelope and the first-generation
  covers are vacuous (upper 1000‰). Richer structural producers
  (protection conditions, per-cell partitions) are the declared next
  answer — not a lowered verifier bar.
- Whole probe 15.7 s, dominated by h3-t4's staircases; no root
  dropped.

## laydownreport_run1.txt readings (2026-08-31) — the Phase 7 typed census

- **Proved from the rules, not the phrase.** The boss-chain control
  (6-6/6-5/6-4 at three remaining tricks, all-or-nothing contract)
  certifies as a TRUE Laydown in 1.49M walk nodes / 0.37 s; the
  already-made root classifies in 3 decided reads (10 μs — §17's
  zero-cost closure, live); the loose-boss counterexample refutes
  FAIL-FAST in 280 nodes.
- **h10-t6 is a real receipt-root Laydown** — all four tiers true with
  forcing witness 2-2: the first certified "expose the hand" on corpus
  data.
- Boundary: the universal walk quantifies over a per-seat RELAXATION
  of the world set (jointly-impossible branches are walked as vacuous
  or phantom worlds) — sound for certification, possibly conservative;
  and it is an endgame instrument, exponential in remaining plays.

## openingreport_run1.txt readings (2026-08-31/09-01) — the Phase 8 §65 ladder

- **The ladder ladders.** At h0-t1 (Z = 399,072,960, seven legal leads,
  contract 30 on threes) across declared stops p=16/64/256/512: the
  proof bar climbs 0 → 407 → 594 → 732‰, certified regret falls
  1000 → 592 → 405 → 267‰, and the recommendation MIGRATES
  0-0 → 2-1 → 6-5 as evidence deepens — every number δ-qualified with
  the ledger exact (14/25 over 56 distinct scopes against the declared
  3/5 budget).
- **The sampled tier plateaus, and says so.** p=256 → p=512 leaves the
  bar and Γ unchanged (732‰ / 267‰) while the last vacuous uppers
  break (U* 1000 → 999‰ — every sample-fitted optimum finally lost at
  least one of 512 worlds, the blank double included). The remaining Γ
  is policy gap plus lock looseness — purchasable by structural work
  (extraction, counted failing sub-fibers as deterministic uppers),
  not by more prefix.
- **The cliff, sharpened.** All 29 frontier refusals at the opening
  root are pure AFFORDABILITY — with sampled facts installed, every
  exact item has positive declared potential; the refusals are the
  cliff, not the §41 stall. Zero Z spent, zero facts installed by the
  pass (§66.14 live).
- **The census at stage 4**: 159–444‰ exact mass across the seven
  leads — the opening field far from action-exact at the declared
  vocabulary depth (Slice F's fragmentation finding from a new angle);
  richest after mid-tile leads (5-3 at 444‰), thinnest after 6-5
  (159‰).
- **The whole opening-root proof state is 56 facts, 10,439 bytes**,
  serialization-versioned and resumed bytewise (the §67.5 gate).
  Verdict at every stop: honest UNRESOLVED at ε = 1/4 (Γ = 267‰ vs
  ε = 250‰) — §65's first target met in its certified-regret form.
- Wall honesty: stops cost ≈0.1 ms / 5.7 s / 30 s / 11 min / 76 min —
  the optimization-lock upper's cost grows quadratically in the prefix
  while its declared forecast unit is linear; a staged cost model for
  the sampled tier is follow-on work, priced but unbuilt.

## doomreport_run1.txt readings (2026-09-01) — the doom census

- **The instrument.** `solver/doom.rs`: `universal_viewer_failure` — the
  ∀-fail dual of the §16 walk (focal nodes AND over every viewer
  escape; hidden nodes partition the record-consistent support by the
  DECLARED σ0's deterministic choice — `pmake`'s own field semantics,
  where a ∀σ-fail dual would certify nearly nothing); `doom_census` —
  the §28/§49 signature-vocabulary descent over one to three hidden
  seats with exact oracle masses, a full mode under the §46 partition
  law and a priority mode (declared punish order: opponents
  nastiest-first, partner weakest-first, feasible-class cut) for rich
  roots; `doom_enumeration` — the per-world ground truth (a singleton
  class is a belief, so the exact recursion is a world-aware make
  check); `DoomCensusProducer` — deterministic uppers
  `(Z − M_doom)/Z` through the open §49 registry. Eight gates in
  `solver_doom.rs`; nothing sampled exists on any path.
- **Where doom lives, the census harvests it.** On the enumerable
  receipt roots the full census recovers 809–1000‰ of the per-world
  doom truth wholesale: h12-t6 is WHOLE-FIBER doomed in one decided
  read per action (the §17-dual zero-cost path on a real root);
  h5-t6's two actions certify 15/27 exactly; h4-t6's 0-0 certifies
  56 of the true 60; h8-t5's 5-0 certifies 28/28 while its 0-0 takes
  17 of 21 and its 5-3 misses the single doomed world (the per-seat
  relaxation's price, stated by the instrument itself).
- **At the opening root the census certifies ZERO — and the God grid
  shows the zero is real, not instrumental.** All seven h0-t1 leads:
  no class at any level of the declared top-8 priority descent
  certifies; the trump leads (3-2/3-3/5-3) EXHAUST their priority
  region (no refusals — every walked leaf found a genuine escape),
  the six leads (6-0/6-5) refuse at the 500k-node budget inside their
  first huge classes. Diagnosis by singleton checks: two adversarially
  hand-built crusher worlds after the 0-0 lead (opponents holding the
  top pool trump, both tens, both loose five-counts, junk partner; a
  trump-wall over the viewer's 3-5) both let the world-aware viewer
  MAKE 30 against σ0, and a declared stride-512 grid over the S2
  support (lex-first completions — a structured grid, NOT a
  probability estimate) finds 0 doomed worlds in 228.
- **The finding that reframes the plateau.** Any doom-family upper is
  floored at the world-aware (God) make rate, and at h0-t1 that rate
  is ≈ 1 everywhere probed: with 3-3, 3-5 and 3-2 in the viewer's
  hand, COUNTED counterexamples cannot move the 999‰ ceilings
  appreciably — the openingreport plateau's remaining Γ is
  overwhelmingly POLICY GAP (the info-consistency price), purchasable
  on the floor side (extraction across the cliff) or by
  info-consistency-aware uppers (optimization locks, §42 exact items),
  never by doom counting. The composed panel says it plainly: p16+p64
  sampled stops, then 0 doom uppers installed, panel unchanged, Γ
  stands at 405‰.
- **The census's home is therefore the endgame and the in-play
  middlegame** — exactly where Phase 7's laydown walk also lives:
  deterministic set-certificates (viewer-objective duals) at t4–t6
  depths where fibers are enumerable-adjacent and σ0 reads are cheap.
  A played hand REACHES that domain every deal; the opening root was
  the stress test, and it priced the wall honestly: each non-forced
  σ0 read is a modeled-mind mini-solve (the field-classification
  bottleneck measured from the doom side), ~2.5–9k walk nodes/s at
  trick-1 depth.
- Wall honesty: Part 1 (twelve receipt-root censuses + enumerations)
  runs in seconds; each opening census costs 41–55 s at the declared
  budgets; the God grid costs 3.2 s for 228 worlds (12–25 ms per
  singleton check, FieldModel-cached); the composed panel re-imports
  p16+p64 in ~41 s.

## Boundaries

- Deterministic fields only; a stochastic field needs an explicit tape
  factor (CBS-A6 boundary obligation) and has no entry point here. The
  doom walk leans on this twice: σ0-determinism is what makes hidden
  branches a PARTITION, and the certified classes bound only the
  declared field's `pmake` — the authority string carries the field
  identity for exactly this reason.
- The Phase 2 profile is the record of ONE policy — no envelope across
  policies exists anywhere (the §20 fence, APS-A4), and no profile was
  attempted at the opening root (the full walk without decided cutoffs
  over 399M worlds is unaffordable by inspection; the ten gated roots
  are the declared domain). Cross-contract reuse of a σ0 profile is
  VOID (gate 3b's frozen specimen): under a bid-reading field a new
  contract is a new evaluation.
- Backend zero contracts at most one conditioned factor (the declared C0
  domain, still refused by panic beyond it); the general contraction is
  `SupportOracle` (Slice D), gated by parity on the C0 domain and by
  surviving-world enumeration beyond it.
- The Slice D recursion evaluates ONE frozen focal policy under the
  declared field (§47): no maximization. The Slice E recursion
  maximizes over GRAMMAR actions only (§48's fence): the full action
  set has no entry point, `free` figures come only from the Slice B
  enumeration split, and no argmax or policy is extracted THERE (the
  declared-tie-order extraction is Phase 6's, on its own gates). Neither
  makes
  play-strength claims: a grammar optimum under a modeled field is an
  evaluation subject, not a recommendation.
- The Phase 4 staircase stages at ONE hidden node per action — each
  path's first field decision, the Slice F bottleneck — with the exact
  recursion below every exact branch: per-node staging deeper is
  Phase 8 scheduling territory, and the fused per-class sum exists
  ONLY as the §23 rejection gate's instrument, never as a bound. The
  Phase 5 covers are first-generation (§62 "deliberately incomplete"):
  whole-cell resources, one cover per action against the strongest
  incumbent, both viewer parities, nothing sampled; vacuity at rich
  roots is recorded, not patched (§70).
- The Slice F loop refines at ONE hidden node (the one-ply contraction
  after the fixed focal play): no cross-node or cross-history class
  transfer is measured or claimed — a signature that is action-exact at
  one public state must be re-verified at any other (the C1 identity
  law, unweakened). No class verifier exists: exactness is established
  by classifying every member hand, and the §29 verifier interface is
  named, not built.
- The Slice G controller works at ONE root per run: no cross-root
  reuse, no cost-model learning, and the §48 fence is lifted only
  through `response_success_mass` (whose sole role is the §36
  escalation, gated to bundled parity). Grammar/residual UPPER bounds
  (Slice B's `residual_empirical_max_upper`) are not wired in as a work
  item — before escalation, the only nontrivial uppers on this ladder
  are the sampled optimization-lock bounds. The existing controller
  player remains the fallback surface; nothing here touches the
  default player.
- Phase 6 extraction is AMPLE, not scheduled: the producer extracts
  every root action unconditionally — §33 work-item selection (extract
  only where the gap pays) is Phase 1's frontier. The extracted DAG is
  rooted at its extraction belief: off-DAG queries complete by the
  declared lowest-legal rule and the re-pricing equality is the
  receipt that the completion never carries objective weight FROM THAT
  ROOT — reuse from any other belief state is unmeasured and
  unclaimed. Grammar-source extraction exists in the library and is
  gated, but the shipped producer is full-legal only. Phase 4 envelope
  cells and Phase 5 count-threat covers (bounding `D` without walking
  it) are unbuilt; `residual_split` pays the full response-walk price.
- The Phase 1 frontier is deterministic-only: RefineV1's sampled tier
  is not a work item (importing its facts is the caller's move), costs
  are a DECLARED crude forecast model (Z/3Z units — no clock, no
  learning), goals beyond the four built (laydown proving, count-risk
  explanation, policy pricing) are Phases 5/7 vocabulary, and the §42
  bounds are conservative by design — the recorded
  extraction-under-vacuous-uppers waste is the price of safe bounds,
  not a defect in them.
- The opening-root recursion is not attempted; only the opening-root
  ONE-PLY contraction is measured (the Slice C probes above, Slice F's
  refinement of the same contraction, and Slice G's Section D, where
  every opening-root exact recursion is refused by its own declared
  forecast). The Phase 8 ladder keeps this boundary: its opening-root
  content is the sampled tier, the census coordinate, and the
  frontier's RECORDED refusals — no exact walk runs at h0-t1, and the
  §65 report says so at every stop.
