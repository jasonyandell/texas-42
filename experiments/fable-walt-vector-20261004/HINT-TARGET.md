# The hint target: exact source path for "Estimated chance to make the bid"

**Source mapping only, 2026-10-04.** Claude Fable 5.1 (`claude-fable-5-1`), same
session. Supersedes the generic live-player mapping in `DESIGN.md` section 4
where they differ. No run was started; the completed diagnosis is untouched.
Exploratory tier. Jason's screenshot (`1000004657.png`, forwarded text only, see
`JEB-HINT-TARGET.md`) shows the Plunge **move hint** dialog: title "Estimated
chance to make the bid", rows `2:0 100.0% 40/40 Suggested · Highest estimate`,
`2:1 100.0% 40/40`, `3:2 100.0% 40/40`, `5:0 100.0% 40/40`, `3:0 95.0% 38/40`,
footer "Hints use your hand and public plays. They compare the baseline player's
outcomes without the separate partner check."

## 1. Verified path, UI → API → Walt

| Layer | Source | Verification status |
|---|---|---|
| Hint dialog | Plunge `src/ui/MoveHint.tsx` → `getHint(g, sessionId, 40 or 160)`; "Think deeper" re-requests 160; the "Compare your choices" disclosure renders `MoveScores` and the footer sentence (line 80 at origin/main) | origin/main `3efd218` (2026-09-26) blob `26fdfc32e0d8`; local HEAD `aff966d` blob `2ac3d4520392` (same footer). **Pinned a0d9fa80 blob not verifiable locally** (section 5) |
| Rows | `src/ui/MoveScores.tsx`: label "Estimated chance to **{objective} the bid**", per row `(chance·100).toFixed(1)%` and `successes / worlds`, tags `Suggested` (the player's `choice`) and `Highest estimate` (every row with `best`) | blob `1bd2ef2b0ca1`, identical at origin/main and local HEAD |
| Hint logic | `src/ai/hint.ts`: `request = requestOf(g, seat 0, sessionId)`; `api('estimates', {request, worlds})`; validates `plunge-estimate-v1`, same request key, `identity.player.n === worlds`, legal choice, leader and points; `stats = decisionStats(response, request, legal)`; requires the choice to be among `best` | origin/main blob `ed00debdfcd6`; local HEAD `ac6c4b3b6c89` (differences: contract/`AnalysisWorlds` typing only) |
| Interpretation | `src/ai/decision-stats.ts` `decisionStats` (section 2) | origin/main blob `f255ccde51b2`; local HEAD `624bc47c5ab0` (origin/main adds Nel-O checks and `baseline-counterexamples`; the straight-42 arithmetic is identical) |
| API | `src/ai/native.ts` `api('estimates')` in phone mode: `runPlayer({request, worlds, partner: false, nello_counterexamples? (Nel-O defender only), budget_ms: 20000 if worlds > 40})`; wraps the response as `plunge-estimate-v1` with `identity.player.n = worlds` and `implementation = manifest + app build` | **Pinned-exact**: reference copy `experiments/astra-sol-20261004/reference/production-phone/native.ts`, blob `3a91f0d59cef` = `src/ai/native.ts` in the saved a0d9fa80 tree listing (`production-tree.json`). origin/main differs only by estimate persistence (`putEstimate`) |
| Worker | `src/ai/phone/client.ts` `runPlayer` → `worker.ts` instantiates the pinned WASM, `walt_in_prepare`/`walt_call`/`walt_out_ptr`, host clock, checkpoints; host timeout `budget_ms + 4000` | reference copies pinned by the identity receipt; `worker.ts` blob `47c29a0055f3` identical at origin/main and local HEAD |
| Player | `walt/walt-player/src/lib.rs` `decide` with `partner:false`: stages `fallback-l1` (n 8, n0 2) then `baseline` (n 40, n0 8, n1 2; for worlds 160 also a 40 stage first), each `partnership_wire::run_with_cache("baseline … inner_belief 0 selection 0 modeled_selection 0")`; **no partner-rollout review because `call.partner` is false** (`lib.rs:241`) | pinned source `cb1ef3b2`, identical at HEAD; WASM `40d7a1ee…` |
| Values | `partnership_wire.rs:238-250`: `options = [[tile, "numerator", "denominator"], …]` for every legal tile, `outer_worlds`; `partnership.rs` `evaluate_contract_with_cache`: common sampled worlds for all candidates, `Field::SeatLevels([0;4])`, focal maximize iff on `Team::T1`; `Solver::solve` = wins / alive worlds, where a win is `banked_t1 ≥ bid` (`contract.rs:48`) | pinned source, identical at HEAD |

The footer sentence is therefore literally true at the source level: the hint
is the pinned player's **baseline L1 comparison** (40 outer worlds, 8 inner
worlds, voidless inner belief, fixed selection, no refinement), and the
**separate partner check** that live play with `native-partner` difficulty
runs afterwards (`partner_rollout` review, which can change the live move) is
skipped because the estimates call passes `partner:false`.

## 2. Exact field semantics behind each displayed number

From `decisionStats` (origin/main `f255ccde51b2`, straight-42 branch):

- **Which evaluation.** `route` must be `baseline` (or `baseline-reviewed`,
  which cannot occur for hints) with a completed `baseline` phase, reading
  `response.evaluation`; or `route == 'l1-fallback'` with a completed
  `fallback-l1` phase, reading `response.fallback_evaluation` and setting
  `fallback = true` (UI then says "A smaller fallback comparison was used" and
  "N sampled deals · 40 requested"). Any other route yields no stats and the
  dialog reports that no complete comparison finished. **A hint can display an
  8-world fallback vector**; a training target must record which it is.
- **Raw fields.** Each row is `[tile, numerator, denominator]` as decimal
  strings (BigInt-parsed, `n ≤ d`). `worlds = evaluation.outer_worlds`.
- **Perspective.** `declaring = request.seat % 2 === request.bidder % 2`. For a
  declaring seat the displayed numerator is `n` and the label is "make the bid";
  for a defending seat it is `d − n` and the label is "**set** the bid". The
  underlying rational is always P(declaring partnership makes) over the
  sampled worlds; the UI shows the acting partnership's success count. The
  screenshot's "make" title means the user's seat was on the declaring side.
- **Percent and counts, rigorously.** With `N = outer_worlds` and the row's
  rational `n/d` (complemented to `(d − n)/d` for a defending seat):
  `chance = n/d`, printed to one decimal; `successes = n·N/d`, shown only when
  that product is an integer, otherwise omitted. The wire serializes a
  `BigRational`, which is reduced, so the stored numerator and denominator are
  **not** themselves the displayed counts: 38 of 40 arrives as `19/20` and the
  UI recomputes `19·40/20 = 38`; 40 of 40 arrives as `1/1`. Labels must store
  the raw `[tile, numerator, denominator]` and `N` and derive counts the same
  way; they must not assume the reduced numerator equals the success count.
  `40/40` means 40 sampled worlds of 40, not a guarantee; `38/40` is 95.0%.
- **Order and ties.** Rows sorted by fraction descending, then tile id
  ascending. `best` is true for **every** row tied with the top fraction, and
  each such row shows "Highest estimate"; `Suggested` marks `response.choice`,
  which the player picked with `best_of` (lowest tile id among exact ties) and
  which `getHint` asserts is one of the `best` rows. `hintExplanation` says
  "`k` plays tied for the highest estimate" when `k > 1`. Display ties and
  selection ties are the same exact-rational ties; there is no near-tie band
  anywhere in the UI or the player.
- **Legality.** `options` must contain exactly the legal set, else no stats;
  illegal tiles never appear.
- **Seed.** `requestOf` uses `nativeSeed(gameId, handNumber) = FNV-1a("sunshine/{gameId}/{handNumber}")`
  with `sessionId` as gameId; public, never derived from the hidden deal. Live
  play uses the same scheme for the same hand, so the 40-world hint for the
  human seat and the player's own live baseline draw from the same base seed
  (the solver then mixes in hand and record hash).
- **Levels.** Two named hint levels exist: `worlds 40` (budget 14 000 ms
  default) and "Think deeper" `worlds 160` (budget 20 000 ms). The pinned
  `native.ts` also allows 350/500 in the Mac native-table transport
  (`DEEP_WORLDS`), which is not the phone.

## 3. What the hint numbers are, stated precisely

They are **model estimates of the pinned baseline L1 player**: for each legal
first play, the fraction of 40 (or 160) sampled hidden deals, drawn uniformly
over completions consistent with the user's hand and public legality (voids
respected, history likelihood and bidding ignored), in which the declaring side
reaches the bid when the focal seat's later plays follow the solver's bundle
recursion and the three other seats follow level-0 modeled minds with 8 inner
worlds. They are not a lawful true posterior chance, not a game-theoretic value,
and not exact beyond the 40-world denominator; identical rows (40/40) are exact
ties of this estimator, not equal latent chances. Jason's instruction is to
imitate this named estimator, so the teacher is defined as exactly this output,
labeled as such, never relabeled as truth.

## 4. Corrections to the DESIGN.md plan

1. **Teacher = hint route, named by level.**
   `PLUNGE-HINT-ESTIMATES-walt-table-v2-partner_false-n40-n0_8-n1_2-VOIDLESS-FIXED-BID30-STRAIGHT-v1`
   (and `…-n160-…` as the second level). Operationally this is the same
   `baseline` wire the earlier proposal named, now pinned to the hint route:
   `partner:false`, `worlds` 40 or 160, request seed scheme recorded, and the
   WASM `estimates` response as the conformance oracle for the native labeler
   (field-for-field equality of `evaluation.options`, `outer_worlds`, `route`,
   `choice`, `phases[].status`; `elapsed_us`/`solver_us`/`over_budget` excluded).
2. **Label record per root**, mirroring `plunge-estimate-v1` so a hint can be
   replayed: `request` (7 fields), `worlds` requested, `route`, which
   evaluation completed, `outer_worlds`, every `[tile, numerator, denominator]`
   verbatim, `choice`, declaring flag, the derived acting-side counts, and the
   `best` set. Keep `fallback` vectors out of the training target by default
   (recorded, separately analyzable); retry a refused baseline once with the
   full 14 s budget before recording a refusal.
3. **Model output** = the full legal-action vector with illegal tiles masked
   and no softmax; the hint's own convention (`make` for declarers, `set` for
   defenders) plus the raw declarer-make form; recommendation and the full tie
   set derived separately, exactly as `decisionStats` does. Whether the vector
   is emitted as absolute chances, baseline plus advantages, or centered
   advantages is a design option (DESIGN.md sections 1 and 5): imitating the
   displayed hint needs an absolute form; action comparison alone does not.
4. **Loss options**, none mandated: masked MSE/Brier on `k/N`, soft-target
   BCE with `k/N`, or count-weighted cross-entropy on `(k, N − k)`. The
   count-weighted form is not claimed to be an exact binomial likelihood,
   because the `k` come from one common bundle with action- and
   recursion-coupled outcomes. Tie recovery is measured against the hint's
   exact ties; near-ties are an evaluation concept only, defined on the K-seed
   reference, never displayed as if the hint had them. Regret of the derived
   recommendation stays a separate evaluation beside vector error, calibration,
   counts and provenance.
5. **Paired common worlds.** The engine already evaluates all candidates on the
   same sampled bundle, so per-action labels are paired by construction; the
   engine does not expose per-world outcomes through the wire (only the
   aggregate rational), so per-world Bernoulli packing as in the T0 kernel is
   not available without a solver change. Report paired differences at the
   vector level only.
6. **Fusion review carried.** The bundle recursion merges only sampled worlds
   (section 3 of `DESIGN.md`); the hint inherits that optimism. The partner
   review is excluded by the route, which removes the one choice-modifier that
   is not derived from the displayed vector.

## 5. Unresolved mapping details, stated rather than invented

- **Pinned-exact hint UI blobs.** No local Plunge checkout contains
  `a0d9fa806166b0e63fe016bb49d93f91f47b1af8` (local `plunge` HEAD `aff966d`
  2026-09-20 predates it; fetched `origin/main` `3efd218` 2026-09-26 carries the
  pinned manifest and WASM hash and the pinned `store.ts` blob, but its
  `native.ts` predates the pinned one by two persistence lines). The saved
  pinned tree listing (`production-tree.json`) has only 15 entries and omits
  the hint files. So `hint.ts`, `decision-stats.ts`, `MoveScores.tsx`,
  `MoveHint.tsx` are verified at origin/main and local HEAD, not at a0d9fa80.
  Resolution: a read-only `git fetch` of that commit in a Plunge checkout, not
  performed here (network action, not authorized this turn).
- **Screenshot position.** Deal, seat, trump, bid, public plays, game id,
  hand number and the request seed are not in the forwarded text, so the
  displayed vector cannot be reproduced or checked against the player here.
- **"Highest estimate" tags.** The code tags every top-tied row; the forwarded
  text shows the tag only on 2:0. Either the forwarding abbreviated it or the
  layout differs; unknown.
- **Level of the screenshot.** `40/40` implies `outer_worlds = 40`, consistent
  with the 40-world hint and not a fallback (fallback would show 8); whether
  it was a first hint or a saved hint replay is unknown and immaterial to the
  numbers.
- **Build budget.** Native `walt-player` build time under the 295 s cap is
  unmeasured (the smaller adapter crate took 28 s).
