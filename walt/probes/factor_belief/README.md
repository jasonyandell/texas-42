# factor_belief — the Slice C (stages C0–C2), D, E, F, G and Phase 2 probes

**EXPLORATORY — sits below every evidentiary tier and is cited by nothing
above it.** A probe number becomes quotable only by brief amendment that
adds it to a verifier receipt.

Instruments: `walt/walt/src/bin/factorbelief.rs` (Slice C),
`walt/walt/src/bin/factorrecursion.rs` (Slice D),
`walt/walt/src/bin/factorresponse.rs` (Slice E),
`walt/walt/src/bin/factorcegar.rs` (Slice F),
`walt/walt/src/bin/factorrefine.rs` (Slice G) and
`walt/walt/src/bin/factorprofile.rs` (anytime proof-state Phase 2).
Gates (the CI-checked part): `walt/walt/tests/solver_factor_belief.rs`,
`walt/walt/tests/solver_factor_recursion.rs`,
`walt/walt/tests/solver_factor_response.rs`,
`walt/walt/tests/solver_factor_consequence.rs`,
`walt/walt/tests/solver_factor_refine.rs` and
`walt/walt/tests/solver_factor_profile.rs`. Mathematical source:
`walt/math/counted_belief_sandwich_v0.1.md` §11–12, §21–23, §25–31,
Part VIII §32–37, §46 stages C0–C2, §47 Slice D, §48 Slice E, §49
Slice F, §50 Slice G, rulings CBS-A4/CBS-A6/CBS-A9; and
`walt/math/anytime_proof_state_score_v0.1.md` §2–§4, §10–§11, §18, §44,
ruling APS-A2.

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

## Boundaries

- Deterministic fields only; a stochastic field needs an explicit tape
  factor (CBS-A6 boundary obligation) and has no entry point here.
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
  enumeration split, and no argmax or policy is extracted (that needs a
  declared tie order — not a Slice E claim). Neither makes
  play-strength claims: a grammar optimum under a modeled field is an
  evaluation subject, not a recommendation.
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
- The opening-root recursion is not attempted; only the opening-root
  ONE-PLY contraction is measured (the Slice C probes above, Slice F's
  refinement of the same contraction, and Slice G's Section D, where
  every opening-root exact recursion is refused by its own declared
  forecast).
