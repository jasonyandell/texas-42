# Walt-vector distillation: source map, teacher assessment and bounded plan

**Design response only, 2026-10-04.** Claude Fable 5.1 (`claude-fable-5-1`), same
session as the completed diagnosis in `../fable-tiny-net-20261004/`, which is
preserved unchanged (frozen weights, fresh test, metrics, chronology, commits
`53d21479`, `5b8217c7`, `cbca64eb`). No label, training, build, benchmark or test
run was started for this phase. Everything below is inspection of pinned sources
plus a plan; hypotheses are marked as such. Exploratory tier throughout.

## 00. Reconciliation after the Excel-chat boundary recovery (2026-10-04, latest)

Jason's clarified intent is that the tiny model replaces **one inner computation
level of Walt's evaluator**, with selection unchanged (legal argmax/argmin,
lowest tile on ties). [BOUNDARY-RECONCILIATION.md](BOUNDARY-RECONCILIATION.md)
recovers the actual contracts from `nofusion_sc.py`, `exact_tape.py`, `walt.c`
and the pinned Rust solver, and locates the replacement point at the level-0
modeled mind (`walt.c inner_L0`, Rust `Solver::pi(0, …)` with 8 inner worlds),
whose upper consumer uses **only its chosen tile**. The outer hint vector of
section 0 and HINT-TARGET.md is the top level's display, a later rung's target,
not the inner teacher. Consequences for this document: section 4's teacher is
the *outer* teacher; the inner teacher is `L0MIND-n8` with a K-seed reference;
section 6's rung option (b) becomes the primary structure, implemented first
through `walt.c`'s `Policy` seam rather than the closed Rust `Field` enum;
section 7's transfer hypotheses are narrowed (8-world labels make teacher noise
a live factor; ordinary larger capacity is a required control, not ruled out);
and because the inner consumer uses choices only, centered advantages remain a
legitimate primary target option alongside direct Q and baseline-plus-advantage.
Raw public/own-hand inputs stay the starting candidate; engineered features are
a diagnostic arm. The exact later evaluator API from the chat is a provenance
gap, not inferred.

## 0. Correction after Jason's screenshot (2026-10-04, later the same day)

Jason's final target is the Plunge **move hint** vector ("Estimated chance to
make the bid", `successes / worlds` per legal play, "baseline player's outcomes
without the separate partner check"). [HINT-TARGET.md](HINT-TARGET.md) traces
that display to its source and supersedes section 4's teacher name: the teacher
is the pinned player's `estimates` route (`partner:false`, worlds 40 or 160),
i.e. the same `baseline` L1 wire described in section 3 with the partner review
excluded by the route itself, and the label record must keep the raw
`[tile, numerator, denominator]` rows, `outer_worlds`, `route`, declaring flag,
the full exact tie set and the player's `choice`. Sections 1 to 8 below are
retained as written; where they say "partner review not applied" read "excluded
by the hint route", and where they name the teacher read the hint name in
HINT-TARGET.md section 4.

## 1. What Jason asked for, stated against what exists

Jason's target: input = one seat's information position; output = the chance
vector "like Walt sees" for every legal play, not a single action, because ties
are prevalent; the loss compares the whole vector Walt would compute with the
whole vector the net predicts.

The completed studies already trained on full legal-action vectors (all paired
per-action labels, masked vector MSE), never on one-hot actions. The core
intent, full per-legal-action supervision with ties preserved, is therefore
already met in shape. Three further axes are independent of that intent and of
each other, and none of them is a user-mandated architecture (Jason is
exploring how to articulate the goal; see `JEB-TARGET-NUANCE.md`):

1. **Target form (an option, not a repair).** The pilot regressed
   `4·(Q(a) − mean_legal Q)` with legally centered predictions. Centering
   preserves every pairwise gap and every exact tie and is not inherently
   wrong; it drops the common absolute baseline, so the model cannot display
   "this position is a 70% make." Direct absolute Q, centered advantages, and a
   state baseline plus advantages are three designs chosen by downstream need:
   action comparison alone is served by any of them; displaying absolute
   chances like the hint requires one of the latter two.
2. **Teacher identity (a separate decision).** T0 is the all-action make
   probability under uniform random legal play by every future actor, averaged
   over a support-uniform belief; the pinned Walt hint computes a different,
   named-level estimate (section 3, `HINT-TARGET.md`). Reports named T0
   correctly; nothing was claimed to be Walt. The tradeoff is cost and
   simplicity (T0 labels thousands of roots per second, no solver build)
   against imitating the actual named level Jason sees.
3. **Loss and calibration (open).** Options are listed in section 5; none is
   asserted as statistically exact for bundle-coupled solver outputs.

A softmax over per-action outcomes is not among the options: Q(a) are chances
of separate hypothetical continuations, not a categorical distribution over
plays. The vector itself, legal-masked, with its ties, is the object.

**Pilot rationale, from contemporaneous evidence only.** The pilot's
`plan.json` (`tiny-net-ladder-20261004`, `architecture` and `rung_indexing`
fields) and `REPORT.md` state that P−1 uniform was chosen because it is
lawful (the archived `walt_decide` comparator "optimized own continuations
separately inside hidden worlds and was fused", and "there is no hidden-world
maximizing continuation") and cheap (the validation gate required "a useful
low-budget teacher"). For centering, the report records only that "outputs are
scores for legal argmax, not calibrated P(make) probabilities" and that MSE was
not used to bound game loss; an explicit reason for discarding the baseline is
not documented in the plan, report or code.

## 2. Source map: what was inspected, where, pinned by what

Repository HEAD `cbca64ebc97cd3f0cd3b59ea07eab98a5d817301` on
`experiment/fable-tiny-net-ladder`. The production player source commit
`cb1ef3b23072e4c268f31f625f2b61d5facc1929` (2026-09-24, "Trim Nel-O preview
sampling budgets") is an ancestor of HEAD, and `git diff --stat cb1ef3b2 HEAD --
walt/walt-player walt/walt/src walt/walt-wasm` is empty: the player, solver and
WASM crate sources at HEAD are byte-identical to the pinned production source.
Pinned phone blob SHA-256 `40d7a1eea658627f7bab37d3c1888ad3bf00216fb23a5d2ceeb7d2d1f4219119`
(`walt-table-v2`), Plunge `a0d9fa806166b0e63fe016bb49d93f91f47b1af8`, source
SHA-256 `5bedf14af0e5a5ef354acca4765f67843469d074d163e8e4786631aa23436e32`
(manifest below). Column `blob` is the git blob id at HEAD.

| File (role) | SHA-256 prefix | blob |
|---|---|---|
| `walt/walt-player/src/lib.rs` (pinned `decide`, output schema, review hook) | `079caf5a3103b464` | `3d6528367cb9` |
| `walt/walt-player/src/played.rs` (native full-game host, seat rotation of points) | `43372a7109c8983e` | `9e65cc7c7d12` |
| `walt/walt-player/src/bin/walt-table.rs` (line-delimited native transport) | `1913cc33159ae66c` | `afb91d6ef957` |
| `walt/walt/src/solver/mod.rs` (`sample_belief`, `best_of`, `solve`, `Field`, `replay`) | `3e93d9878309bc0c` | `0cfae78e9208` |
| `walt/walt/src/solver/partnership.rs` (`FieldProfile`, `evaluate_contract_with_cache`) | `66f90da885bf8790` | `e28aa278969c` |
| `walt/walt/src/solver/partnership_wire.rs` (`baseline` wire, `options` serialization) | `84f7ce16a0071515` | `ad0e18fd51e0` |
| `walt/walt/src/solver/selection.rs` (`Rule::Fixed`, `tied`) | `555e88af421b2465` | `c3aa33260a42` |
| `walt/walt/src/solver/contract.rs` (`terminal`, `sizes`: make rule) | `c5c5636d8725454c` | `f5f60991f0a9` |
| `walt/walt/src/policy_search/partner_rollout.rs` (partner review that can change the choice) | `514ef4847bb54b95` | `c5752f5c03bb` |
| `experiments/astra-sol-20261004/tools/arena.py` (`profile()`, `Worker`, request boundary) | `653928d3e532cb01` | `c4a5586a2224` |
| `experiments/astra-sol-20261004/tools/phone_worker.mjs` (exact WASM ABI, host clock) | `1e965dd5f9274186` | `8d92fec30d4f` |
| `experiments/astra-sol-20261004/reference/production-phone/manifest.json` | `e15bc53ef6fa923e` | `a9e70db8671a` |
| `experiments/astra-sol-20261004/reference/production-phone/walt-player.wasm` | `40d7a1eea658627f` | `b6a1aec0a8fd` |
| `experiments/astra-sol-20261004/reference/production-phone/native.ts` (`livePlayerCall`, `NativeDecision` type) | `681e5f983688919f` | `3a91f0d59cef` |
| `experiments/native-policy-check-20261004/adapter/src/lib.rs` (late L3 adapter: a *different* policy, not the phone default) | `11cf8c59e8f4a1a1` | `4cb48ad60d80` |
| `experiments/tiny-net-coverage-20261004/match.py`, `../tiny-net-ladder-20261004/match.py` (how the smokes called the phone) | `60c70289c241133d`, `29a86fca7d7cee2e` | `b92757caa15a`, `4414a3c01d5a` |
| `experiments/tiny-net-ladder-20261004/kernel.c` (the T0 kernel, for contrast) | `f905d0a349273d01` | `da9977dfa2ab` |
| `experiments/partnership/rules.py` (independent referee) | `4393683bc431f868` | `8114b8a52721` |

Also read: `walt/MAP.md`, `experiments/partnership/PLAYERS.md`, `HEAD-TO-HEAD.md`,
`BASELINE.md`, `reference/phone/PROVENANCE.md` (the *older* August phone
`af0200af…` at commit `9a056f20`, race/refine procedure; **not** the pinned
production player and not used here), `experiments/native-policy-check-20261004/README.md`
and `REPORT.md`, `experiments/higher-k-budget-20261004/README.md`,
`experiments/native-frontier-20261004/README.md`, and the walt build script
`walt/tools/build_cpu.sh`. The Plunge checkout `/Users/jason/code/plunge` exists
but was not opened; the pinned copies of its `native.ts`, `client.ts`,
`worker.ts`, `store.ts` under the production-phone reference are what the prior
identity receipt checked against saved Plunge blobs.

## 3. What the pinned production Walt actually computes and exposes

Trace: `arena.py:profile()` builds `{request, worlds, partner, budget_ms}` exactly
as Plunge `native.ts:livePlayerCall` with `thinkDeeper=false` (opening = bidder's
first play: 160 worlds, partner off, 20 s; otherwise 40 worlds, partner on, 14 s).
`phone_worker.mjs` instantiates the pinned WASM per decision and calls
`walt_in_prepare` / `walt_call` / `walt_out_ptr`. Inside, `walt_player::handle`
parses a `Call` and runs `decide` (`lib.rs:118-268`).

**Request boundary.** `Request` is exactly `{decl, bid, bidder, seat, hand (own
original seven tiles), plays (public actor/tile record), seed}` with
`deny_unknown_fields`. No hidden hands can enter. `seat` is the acting seat; the
player refuses if it is not that seat's turn (status check).

**Decision procedure (`decide`).** Stages, each a complete comparison or nothing:
`fallback-l1` at n=8, n0=2 (≤1.5 s), then `baseline` at n=40, n0=8 (and at 160 first
for the opening profile). Each stage calls `partnership_wire::run_with_cache` with
the text wire `baseline … n N / n0 8 / n1 2 / budget_ms / inner_belief 0
(Voidless) / selection 0 (Fixed) / modeled_selection 0 (Fixed)`. A completed
baseline sets `route:"baseline"`, `choice`, and `evaluation`; a timed-out
baseline leaves `route:"l1-fallback"` with the 8/2 vector in
`fallback_evaluation`. Then, only if `partner` is true and the route is
`baseline`, a **partner-rollout review** (`partner_rollout.rs`, ≤500 ms, ≤64
samples over a support ≤400) may replace `choice` with another legal tile
(`route:"baseline-reviewed"`) when its own paired rollout values prefer it
(`partner_rollout.rs:356-366`). The review's values are a different estimator
(completed deployed-L1 continuations on different samples), not the baseline
vector.

**Exposed outputs** (`NativeDecision` in `native.ts`, assembled in `lib.rs`):
`choice, legal, leader, points, trick, route, mode, n, n0, n1, budget_ms,
inner_belief, selection, modeled_selection, phases[], elapsed_us, over_budget,
evaluation, fallback_evaluation, review_result`. `evaluation` is the wire's
JSON: `choice`, **`options: [[tile, "numerator", "denominator"], …]` for every
legal tile**, `outer_worlds`, `outer_draw_attempts`, `pi_calls_by_level`,
`inner_worlds_by_level`, `nodes`, `solver_us` (`partnership_wire.rs:238-250`).
So the production player **does expose a complete per-legal-action value vector
as exact rationals**; `choice` is derived from it by `best_of`, except when the
partner review overrides it.

**Objective and perspective.** `replay_contract` (`mod.rs:1824-1838`) rotates
arena seats by `r = 1 if bidder even else 0`, which puts the bidder's partnership
on `Team::T1`; the contract is `Straight{bid}` and `terminal` (`contract.rs:48-58`)
is make ⇔ `banked_t1 ≥ bid`, set ⇔ `banked_t0 > 42 − bid`. `Solver::solve`
(`mod.rs:773`) returns `wins / alive`, the fraction of the sampled worlds in which
**T1 (the declaring partnership) makes the bid**. The focal seat maximizes if it
is on T1 and minimizes otherwise (`seat.team() == Team::T1` passed as `maximize`
in `partnership.rs:349-382`; `best_of` takes argmin for T0 seats). The exposed
`options` are therefore **declarer-make probabilities from the acting seat's
belief, for both declarers and defenders**; the acting partnership's win chance
is `v` for a declaring seat and `1 − v` for a defending seat. This is the same
objective (pmake, fixed bid 30 in play) that T0 used, expressed from the
declarer side.

**Belief.** `sample_belief` (`mod.rs:1735-1775`): uniform shuffles of the unseen
tiles into the three hidden hands by remaining sizes, rejecting any assignment
that puts a tile into a seat's publicly revealed void. That is **uniform over
capacity-compatible completions given own hand and public legality**, the same
support-conditioned surrogate the T0 kernel samples by exact DP. Walt does not
weight worlds by the likelihood of the observed history under any policy, and
does not use bidding. So "chances like Walt sees" are chances under a
support-uniform belief, not a Bayesian posterior; this must stay in the
teacher's name.

**Continuation.** Each candidate action is evaluated on the **same 40 sampled
worlds** (common worlds across candidates, `partnership.rs` header). After the
root action, the focal seat's future plays are the solver's own recursion: at
each future focal node the max (or min) is taken over the worlds still alive
under the public history, i.e. the viewer's choice is a function of its
information *within the sampled bundle*. The other three seats are modeled
**level-0 minds** (`FieldProfile::Baseline → SeatLevels([0;4])`), each a cached,
deterministic policy from `Solver::pi` with `n0 = 8` inner worlds and the
Voidless inner belief. Values are exact rationals with denominator 40 (counts of
worlds), multiples of 1/40.

**Fusion assessment.** There is no perfect-information continuation anywhere in
this path: the focal seat never picks a different action per hidden world at the
root, and `partner_rollout` explicitly refuses perfect-information substitution.
However, the bundle recursion merges only the *sampled* worlds: as public plays
filter the 40 worlds, a future focal node can be alive in a single world and then
plays with knowledge of it. This is the known "bundle-coupled solver value"
(`mod.rs` comment above `level1_raced`: it differs from the value of a frozen
continuation policy). Its optimism shrinks with larger n, so **n is part of the
objective, not a noise knob**: a 160-world bundle and four 40-world bundles are
different estimators. Any Walt teacher must name n and n0.

**Ties.** `Rule::Fixed` evaluates once and `best_of` returns the first-listed
(lowest tile id) among exact rational ties (`selection.rs:66-100`, `mod.rs:2067`).
With 40-world denominators exact ties are common by construction; the pinned
player does not refine them in play. The older archived phone raced and refined;
the pinned one does not.

**Legality.** `status` computes `legal` from own remaining hand and the led
context; illegal tiles are never in `options`. Callers (arena, Plunge
`checkedAction`) re-check legality, leader and points independently.

**What the completed smokes used.** Both `match.py` files called this player
only as the *opponent/other-seat* policy and never read `evaluation.options`;
the candidate nets' moves came from the T0 kernel or the raw net. No Walt vector
was ever a training target.

**Cost of the production path.** Phone budgets are 14 s/20 s with a 4 s host
grace; the native `partnership` battery measured a mean of about 0.23 s per
L1-default move under load (`walt/MAP.md`, exploratory, includes the fallback
stage). A direct native call to `partnership_wire::run("baseline …")` per root
is the label primitive; its per-root wall on this host is unmeasured and must be
probed before any campaign.

## 4. The lawful, versioned teacher I propose

Name every coordinate so nothing is inherited quietly:

`WALT-L1-FIXED-n40-n0_8-n1_2-VOIDLESS-BID30-STRAIGHT-DECLMAKE-v1`

- Source: `partnership_wire::run` with mode `baseline`, `n 40 / n0 8 / n1 2 /
  inner_belief 0 / selection 0 / modeled_selection 0`, exactly the stage the
  pinned `decide` runs for an ordinary play; built from the pinned source
  (identical at HEAD) via a tiny native bin (the `walt-table` binary or a
  one-file adapter crate like `native-policy-check-20261004/adapter`, which
  built in 28 s under the cap). The WASM path is the conformance oracle, not the
  label engine.
- Objective: P(declaring partnership makes bid 30) per legal action, exact
  `k/40`; also stored as acting-partnership win chance for the net's API.
- Belief: support-uniform rejection sampler, seed = request `seed ^ mix(hand) ^
  record_hash`. Label seed domain fresh and recorded; the request `seed` field is
  public and independent of the hidden deal (as in all prior experiments).
- Continuation: bundle recursion for the focal seat; level-0 minds (8 inner
  worlds, Voidless) for the field. The partner review is **not** part of the
  vector and is **not** applied (it is a choice modifier with a different
  estimator). Record `route`; accept only completed `baseline` vectors; a
  deadline refusal is retried once with a larger budget and otherwise recorded
  as a refusal, never replaced by the 8/2 fallback vector.
- Independent reference for evaluation: the same teacher at K=8 independent
  seeds (eight 40-world bundles), reporting the mean vector and the per-seed
  spread. This estimates the expectation over belief samples of the n=40
  estimator; it is deliberately **not** `n=320`, which is a different
  (less fused) objective.
- Noise model: each option is `k/40` over paired worlds; the per-action standard
  error is at most 0.079 and the paired difference between two actions has a
  smaller, world-paired variance. Exact ties mean `k_a = k_b` on this bundle;
  "near-tie" is defined on the K-seed reference as a mean difference below
  twice its paired standard error. Both are reported, neither is a claim of a
  unique best move.

Why not the L3 late adapter or model-level continuations: they are different
policies (`late-l3-native-40-4-2-2`, `Field::Level(2)`) that the reports
themselves keep separate from the default phone; using them would be
substituting a stronger experimental policy and calling it Walt. Why not
`walt_decide` history: fused comparator, already ruled out.

## 5. Desired model API and loss

**Interface.** `chances(position) → {teacher: name, legal: [tiles], p_make:
[28] (NaN/masked for illegal), p_acting: [28], recommendation: tile or null,
ties: [tiles within ε of the best], epsilon}`. The recommendation is a separate,
optional derivation (argmax with ascending-tile exact ties), never the only
output. Illegal slots are masked at the output layer and excluded from loss.

**Target and loss (options, to be assessed, not mandated).** Target forms:
(a) direct absolute per-action chance; (b) state baseline `v` plus legal
advantages `Δ_a` with `Σ_legal Δ_a = 0`, `p̂_a = σ(v + Δ_a)` or `v + Δ_a`; (c)
centered advantages as in the pilot, which cannot display absolute chances but
is the cheapest continuity control. Losses on the per-action targets `k_a / N`:
masked vector MSE / Brier on probabilities; soft-target binary cross-entropy
with `k_a / N` as the soft label; or count-weighted cross-entropy
`−Σ_a [k_a log p̂_a + (N − k_a) log(1 − p̂_a)] / N`. The last has the form of a
binomial likelihood, but the `k_a` are counts over one common bundle whose
per-world outcomes are coupled across actions and through the solver's
recursion, so it is a convenient proper scoring rule here, not a verified
exact likelihood; no independence or generative assumption is claimed. All
options preserve exact ties (equal `k` → equal targets) and the legal mask;
none uses a softmax. Keep the completed studies' per-action feature scorer as
the representation arm, the raw encoding as the control. Report absolute vector errors (RMSE in
probability, NLL, expected calibration error on the K-seed reference), gap
errors (paired `p̂_a − p̂_b` vs reference), tie recovery, chosen-move regret
against the reference (separate, as before), parameter count, bytes, latency,
training and label cost.

**Selection.** Checkpoint by validation NLL on the K-seed reference (not by
regret); report the regret curve. The completed diagnosis found MSE-on-advantage
selection badly misaligned with regret for raw nets; a calibrated absolute loss
may or may not realign them. That is a hypothesis to test, not an assumption.

## 6. Ladder implications

- **Rung 0.** Distill the named Walt vector into the net (feature scorer and raw
  control, matched budgets). Freeze by hash. The continuation policy `P_0` of
  the frozen net for any actor is: argmax of predicted acting-partnership
  chance, ascending tile id on exact ties; ε-ties are reported but not
  randomized, so every tape stays deterministic. Nobody claims the chosen tile
  is uniquely best; the vector is the deliverable, the choice is a derived rule.
- **Rung 1 teacher options**, both versioned, neither inherited silently:
  (a) *frozen-policy rollouts*: the existing `kernel.c` with `P_0` for every
  future actor (focal seat included), common worlds and tapes, exact paired
  per-action outcomes. Semantics: value of the frozen continuation, lower-witness
  shaped, no bundle fusion; cheap (thousands of roots per second). Requires
  porting the per-action featurizer into C if the feature scorer is the frozen
  model (the kernel supports only dense raw nets today).
  (b) *Walt-style search against the net*: the bundle solver with the field
  replaced by `P_0` for the other seats. `Field` is a closed enum in
  `walt/walt/src/solver/mod.rs`; this needs a new variant or a local solver
  fork, i.e. Rust work inside the exploratory `walt` crate, not a wrapper.
  Recommend (a) first: it exists, keeps the tape discipline, and tests the
  distill-search-distill loop; (b) is the faithful "search like Walt" rung and
  is the right second step if (a)'s rung shows a resolved gain.
- **Independence.** New source deals excluded from all prior sources (now
  including the 25,088 deals of the completed diagnosis and every phone-game
  deal); whole-deal partitions including rotations and seat permutations; fresh
  validation deals labeled with the K-seed reference; a final test block drawn
  up front, mined and labeled only after freeze, exactly as in
  `../fable-tiny-net-20261004/plan.json`. The completed study's fresh test
  (512 deals) must not be reused as a test for Walt-target models: it has been
  opened once and would become a validation set at best.
- **Phone comparison.** Only after a Walt-vector candidate survives on fresh
  deals, and only with balanced independent-deal blocks through the existing
  `arena.py`/`match.py` route; never from the two-deal smokes.

## 7. Do the completed diagnostics transfer? Hypotheses, labeled

Established on T0 targets (fresh 512-deal test, preserved): representation was
the strongest tested factor (feature scorer 77→88% retained vs raw 26→42%);
source-deal coverage helped; capacity alone did not at the prior scale and
helped only with data plus regret selection; the 128-world noise floor was
small relative to the fit gap; the subset512 control was null. Not established:
that raw capacity can never help (it did at 16x with regret selection), that
label noise is irrelevant in general, or that these orderings hold for a
different target. The self-audit (`checks/audit.py`) used independent code paths
but was written and run by the same agent, not an independent reviewer.

Hypotheses for Walt targets: (H1) the representation gap transfers in direction,
because it is about sample efficiency of the input encoding, not the target;
(H2) coverage transfers in direction; (H3) the absolute level `v` is a smoother
function of public score and tricks remaining than the advantages and is
learnable by either encoding, so decomposition will show the baseline is the
easy part; (H4) 40-world labels are coarser than 128-world T0 labels (SE up to
.079 vs .044), so label noise may matter more here, which is why the K-seed
reference and a `n40×K` precision control belong in the first plan. Each is
testable at d1 scale before anything larger.

## 8. Bounded next-phase plan (not started)

1. **Build and probe (one session, all capped ≤295 s):** build a native
   `walt-table`-equivalent bin from pinned source (`cargo build --offline
   --locked --release -p walt-player --bins`, split per crate if a single build
   approaches the cap; the smaller adapter crate built in 28 s); verify one
   request's `evaluation.options` byte-equal between native bin and the pinned
   WASM through `phone_worker.mjs` (the conformance oracle); time 100 roots at
   `n 40 / n0 8` native to set batch sizes.
2. **Preregister** `plan.json` with the teacher name above, data arms, archs,
   loss, selection, K, ε, budgets, caps, and the no-rung rule for this phase.
3. **Labels:** d1-scale first (about 4,500 roots) with bounded parallel workers
   in resumable ≤295 s batches, each batch writing a receipt; K-seed reference
   on validation; then decide d4/d16 from measured cost.
4. **Train** feature scorer and raw control under matched budgets with the
   binomial loss and the decomposition control; freeze.
5. **Fresh test** labeled with the K-seed reference after freeze; report vector
   errors, calibration, gaps, tie recovery, regret, size, latency, cost.
6. **Report back** before any rung; rung 1 under option (a) only on Jason's go.

Constraints carried: every run under `run_capped.py` ≤295 s with resumable
batches and at most a declared small worker count; existing credentials only;
no cloud, installs, pushes, PRs, Slack, email, production changes or fixture
exports; the separately blocked fixture export stays blocked.
