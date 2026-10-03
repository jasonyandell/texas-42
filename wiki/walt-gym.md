[Home](Home.md) · owns: the exact partnership gym — what a coordinate is, how an answer key is defined and audited, Scheme-driven discovery, the 6 → 170 → 433 → 30 results ladder, and how to reproduce every number · Sources: [`walt/gym/README.md`](../walt/gym/README.md), [`RESULTS.md`](../walt/gym/RESULTS.md), [`DISCOVERY.md`](../walt/gym/DISCOVERY.md), [`BID-MAKING.md`](../walt/gym/BID-MAKING.md), [`SPECIFICATIONS.md`](../walt/gym/SPECIFICATIONS.md), [`PARTNERSHIP-COMPOSITION.md`](../walt/gym/PARTNERSHIP-COMPOSITION.md); the query files `walt/gym/queries/*.scheme` and `walt/gym/offer-count.scheme`; the JSON artifacts (`walt/gym/mining.json`, `scenarios/`, `collections/scheme-v1/`, `collections/bid-making-v1/`, `specs/`, `benchmarks/`); `walt/walt/src/gym.rs`, `walt/walt/src/bin/partnership_gym.rs`, `walt/walt/tests/gym.rs`; `experiments/partnership/gym.py`, `gym_spec.py`, `rules.py`, `test_gym.py`, `test_gym_spec.py`; `experiments/partnership/runs/*/run.json`; `walt/LOG.md` entries of 2026-09-06 and 2026-09-07; Sunshine (2026-09-13): `git diff c00717d1..afd46420 -- walt/gym` (`README.md`, `SPECIFICATIONS.md`, [`queries/offer-five.scheme`](../walt/gym/queries/offer-five.scheme), [`specs/partnership-count-l1.json`](../walt/gym/specs/partnership-count-l1.json)), [`experiments/partnership/GYM-REPLAY.md`](../experiments/partnership/GYM-REPLAY.md), [`PLUNGE.md`](../experiments/partnership/PLUNGE.md), `gym_deployed.py`, `gym_generation.py`, `gym_compare.py`, `gym_replay.py`, `plunge_bridge.py`, `plunge_io.py`, `plunge_analysis.py`, `play_plunge.py`, the campaigns `campaigns/sunshine-{recipes,gym-replay,playable,review}-v1/` (`PROTOCOL.md`, `RESULTS.md`, `summary.json`, `validation.json`, `WITNESSES.md`), `walt/LOG.md` entries of 2026-09-13, commits `7abf2aee`, `b11aa189`, `0727103e`, `5bdd8b48`, `2c2d7ae2`. Results files outrank prose throughout.

# walt — the exact partnership gym

> **EXPLORATORY TIER — the whole page.** Everything under `walt/` and
> `experiments/` sits below every tier on
> [Home](Home.md#evidentiary-tiers--never-promoted-never-blurred): no corpus
> status, no Lean kernel proof, no exchange-adjudicated CONFIRMED, no rob
> conformance receipt. Every number below is computed evidence about one
> declared finite domain from a single Rust implementation plus an independent
> Python replay, pinned by checked-in fixtures with sha256 catalogs, eight Rust
> gates in `walt/walt/tests/gym.rs` and twenty-five Python tests. A green gate
> is evidence, never a status change. Full walt CI (`walt/ci/check.sh`) was
> recorded as **waived** in every gym session (`walt/LOG.md`); whether it has
> since passed on main with `tests/gym.rs` included is not verified in this
> pass. The fence is on the [walt hub](walt.md).

Siblings: [walt hub](walt.md) · [partnership program](walt-partnership-program.md) (the players and batteries the gym grades) · [Scheme/Fix](walt-scheme-fix.md) (the query language) · [seat play](walt-seat-play.md) (L1, L2 Partner) · [instruments](walt-instruments.md) · [architecture](walt-architecture.md) · [timeline](timeline.md).

Repository state as of 2026-09-07 (`c00717d1`); the gym itself is unchanged since `1df741db` (2026-09-07 01:14 −0500). (Corrected 2026-09-20: the gym changed again on 2026-09-13 — `0727103e`, `7abf2aee`, `b11aa189`, `5bdd8b48`, `2c2d7ae2`; curated in §11. §1–§10 describe the state at `1df741db`, which every original collection and contract still has.) Fresh measurements on this page are labeled *measured 2026-09-12 on this machine*.

## The gym in one paragraph

The gym turns saved late-game positions into exercises with exact, audited
answer keys, and grades the playable native players on their first choice.
An exercise (a *coordinate*) is one seat's own seven original tiles plus the
public record — never the hidden hands. For every legal play the key computes
the exact probability that the seat's team makes (or sets) the bid 30 when
that play is followed by the best lawful continuation against a **fixed,
declared** teammate and opponents, averaged uniformly over every deal the
public record still allows. The pupil's score is how much make-probability its
choice gives away. Scheme/Fix expressions say which situations to look for;
exact evaluation says whether the move works; an explicit selection stage says
which coordinates become exercises. It is a diagnostic instrument, not a
strength measurement. It was built in five commits between 2026-09-06 22:49
and 2026-09-07 01:14 (−0500), `c59f1115` → `1df741db`.

## 1. A coordinate, and what the pupil sees

**Plain statement.** A coordinate is a decision at trick 5 or 6, at bid 30,
from one chair. The examiner knows the whole deal; the pupil sees only what
that chair legally sees.

**The precise object.** A coordinate is one original seven-tile hand plus the
actor-attributed public history, the declaration, the bidder and the seat to
move. The pupil request is exactly seven fields and nothing else
(`INPUT_KEYS` in `experiments/partnership/gym.py`; `pupil_request()` refuses
any other field, any bid other than 30, and any seed outside 64 bits):

| Field | Meaning | Encoding |
|---|---|---|
| `decl` | declaration | 0–6 = that pip is trump, 7 = doubles trump, 9 = no trump (8 is not a straight-42 declaration; `walt::solver::decl_of` panics on it) |
| `bid` | always 30 | integer |
| `bidder` | the seat that holds the contract | 0–3 |
| `seat` | the seat to move (the viewer) | 0–3; seats 0/2 and 1/3 are partners |
| `hand` | the viewer's seven **original** tiles | tile ids in the repository's triangular 0–27 ordering (0-0 = 0, 1-0 = 1, 1-1 = 2, 2-0 = 3, …, 5-0 = 15, 5-3 = 18, 5-5 = 20, 6-0 = 21, 6-3 = 24, 6-4 = 25, 6-6 = 27) |
| `plays` | the public history as actor/tile pairs | flat list `seat, tile, seat, tile, …` |
| `seed` | the pupil's request seed | 420600 in every published collection |

The native entry point (`partnership_gym`, one atomic exact assessment per
process) reads these seven lines on stdin, refuses requests over 64 KiB,
unknown or duplicate fields, non-straight declarations, hands that are not
seven distinct tiles, and odd-length histories
(`walt/walt/src/bin/partnership_gym.rs`). `walt::gym::from_request` then
rejects a history that violates turn order, capacities or tile uniqueness,
contradicts an observed void, contains an illegal own play, shows another seat
playing an own-hand tile, is not the viewer's turn, or has **no legal hidden
completion**. What survives is an `ExerciseRoot`: a canonical root over the
support fiber (every remaining deal compatible with the record), the root
position (leader, trick prefix, banked points) and a Scheme `Frame`.

**What the pupil never receives.** The actual other hands, the query that
found the position, its label, the answer key, the same-world witnesses. A
Python test pins the refusal
(`test_gym.py::test_private_teacher_fields_are_refused`) [EXPLORATORY; gate].

**Domain.** Tricks 5–6 only (the Python audit requires at least sixteen public
plays), bid 30, straight declarations, contract unsettled (declaring side
below 30 and defenders at or below 12 when the generic stream is used), and
more than one legal play. Opening-depth exercises would need a structural
producer that does not exist (`BID-MAKING.md`, `DISCOVERY.md`).

## 2. The answer key as mathematics

### 2.1 The grading measure: uniform on the support, not a learned belief

The key's measure is **uniform over every mechanically compatible remaining
deal** — the support fiber of the viewer's record, with public voids and exact
capacities respected ([support-fiber](support-fiber.md) for the exact
object). It is a declared experimental prior. Earlier observed actions are
*not* reweighted under an assumed historical player: there is no pre-root
policy likelihood (`SEMANTICS["belief"]`, asserted equal in every fixture by
`verify()`). Keep the typed distinction: the **support** is the set of deals
the rules allow; the gym's **belief** is the uniform weighting it declares on
that set. Nothing here is a learned or maintained belief in the sense of
[belief-vs-support](belief-vs-support.md); changing the weighting would
change the exam.

### 2.2 The fixed field

Every hidden seat is played by a specified, deterministic policy. Its
identity string is written into every key (`field_id`), verified in
`walt/walt/src/gym.rs` (`GymField::new`):

```
gym-field-v1/partner=S*/l1-fixed-40-8/inner=voidless/opponents=field:level0-n8-v1/seed=420600-state-v1/tie=lowest/fallback=none
```

where `S*` is the viewer's partner (`viewer + 2`) and 40 is `partner_worlds`
(asserted in 1..=640; 40 in every published collection).

| Component | Value (code fact, `gym.rs` lines ~244–316) |
|---|---|
| Teammate | the shared playable L1 evaluator `solver::partnership::evaluate` with `FieldProfile::Baseline`, `InnerBelief::Voidless`, `Rule::Fixed` for selection and modeled selection, `n_outer = partner_worlds` (40), `n0 = 8`, `n1 = 2`, no deadline |
| Teammate's seed | `mix(420600 ^ FieldStateKey.digest64())` — one deterministic seed per field information state, derived from 420600; the live pupil wrapper derives its seed differently (from the original hand and record), so the teacher is "a specified policy, not a promise of identical live random tapes" (`README.md`) |
| Opponents | `Level0Field::new(8)`, id `field:level0-n8-v1` (the L0-8 seat-local field) |
| Ties | lowest tile index |
| Fallback | none; forced moves are returned directly; every field choice is cached per `FieldStateKey` and asserted equal on re-insertion |

The teammate's outer sampled beliefs obey public voids; "voidless" names the
L0 minds it models inside. Changing any component changes the exam and
requires a new key.

### 2.3 Q(a): an exact focal best response

For each legal root action `a`:

`Q(a) = max over lawful focal continuations of P(viewer's team succeeds | a, fixed field)`

with success = make 30 when the viewer's team is declaring, set 30 when it
is defending. The maximum chooses **one action per focal information state**
(the viewer's own hand and the public record), never a per-world choice.
Future hidden-seat moves are public actions weighted by their likelihood
under the declared field. Neither the examiner's actual deal nor any
perfect-information choice enters the maximization. Optimality comes from
the existing exact recurrence — `solver::factor_belief::extract_success_policy`
over the `SupportOracle` with `ExtractionSource::FullLegal` — not from a second
optimizer ([walt-counted-belief-era](walt-counted-belief-era.md) for the
recurrence's provenance). This is an exact focal best response to a declared
field; it is **not** a joint-team optimality claim.

### 2.4 Root-action regret, ties, diagnostics

The score is `max_a Q(a) − Q(chosen)`: **root-action regret** in make/set
probability, an exact rational with the world count as denominator. Zero
means the choice belongs to the entire optimal set. **All makes tie and all
sets tie** — a set that banks 7 and a set that banks 29 are the same outcome,
and no point tie-break exists (`test_gym.py::test_full_optimal_set_and_set_outcomes_tie_without_point_tiebreak`).
The 43-bin score law (`score_bins`, indexed by the declaring side's final
points) and partner count are recorded as **diagnostics only**. After
make/set is settled the extracted focal tail completes with lowest legal
plays; those tails are not count-optimal policies. Root regret prices the
first choice followed by an optimal lawful focal continuation, not the pupil's
complete future policy — a complete-policy benchmark is a distinct exam
(policy synthesis runs one; §10).

### 2.5 What a key is not

- Not perfect-information optimality and not field-independent optimality:
  every value is relative to the declared belief and future field.
- Not a proof of the game's mechanics: the rules are replayed by two
  implementations, not proved.
- Not a joint-team optimum: the partner is a fixed policy, not a co-optimizer.
- Not a statement about the pupil's whole policy (§2.4).
- Not evidence about causes: an "advantage" label names an action pair under
  this field, and the replays show what the continuation actually did; a
  pattern label alone is never a causal proof (`README.md`).

## 3. The three audits, and the gym's first discovery

Every accepted key passes three cross-checks; the first two are assertions
inside the Rust `assess()` and the third is an independent Python program.

| Audit | What it checks | Where |
|---|---|---|
| Fixed-policy repricing equals extraction | The extracted policy for `a` is repriced by the separate `viewer_score_profile` walk; the success mass read off its bins must equal the extraction's mass, and `profile.total()` must equal the world count | `gym.rs::assess`, two `assert_eq!` |
| Complete same-world replays | Every hidden world is replayed to the last tile with the focal policy in the viewer's chair and the field elsewhere (`gym.rs::replay`); the resulting 43-bin score law and success count must rederive the recursion's bins and mass; every replay banks exactly 42 points | `gym.rs::assess`, per-action `assert_eq!` |
| Independent Python enumeration with information consistency | `compatible_worlds()` reconstructs every full original deal by combinatorial enumeration and legally replays the record with its own rules (`rules.py`), trusting no native void, count or trace; `verify()` then checks each trace's turn order, legality, terminal 42 points, banked totals, partner count, success flag, bins and mass, the `offers` field against a literal Python restatement, and that **identical seat information yields identical field actions** across all worlds and root alternatives (the `decisions` dictionary; the focal continuation may differ only by root action) | `experiments/partnership/gym.py` |

The Python audit verifies attained values and lawful witnesses. It does not
re-derive optimality: "this is not a second independent optimizing
implementation" (`README.md`). Pins: `test_gym.py::test_every_published_key_has_exact_support_and_lawful_replays`,
`test_tampered_mass_missing_world_or_illegal_trace_is_rejected`,
`test_native_key_agrees_with_published_key` [EXPLORATORY; gates].

*Measured 2026-09-12 on this machine:* `python3 experiments/partnership/gym.py verify`
re-audited the six starter fixtures in 0.08 s and printed audit counts equal
to the fixtures' own `audit` fields — advantage-01 {36 worlds, 2 actions, 517
information sets}, advantage-02 {12, 2, 102}, advantage-03 {150, 3, 2849},
disadvantage-01 {36, 3, 660}, disadvantage-02 {50, 3, 1112}, disadvantage-03
{24, 3, 556}.

### 3.1 The zero-completion boundary repair

The first gym pilot exposed a boundary in single-field conditioning already
guarded on the model-belief path: while building an action-likelihood factor,
the evaluator could ask a field policy to classify a hand with no legal
completion. The common evaluator now removes zero-marginal hands **before**
calling a field policy, and retains the original factor weights rather than
marginal weights, so the joint measure is preserved (double counting of
completions is what marginal weights would introduce). Two older tests that
expected the raw Level1 evaluator to refuse four of six receipt roots were
strengthened to require exact single-field/mixture parity on all six; the
independent impossible-frame witness remains in the suite, so the change
neither hides invalid states nor supplies a fallback answer. Playable
selection rules and defaults did not change. Pins:
`tests/gym.rs::positive_support_pruning_preserves_factor_weights_not_marginal_weights`
(factor weights 2 and 3, mass 6·2·3, weights preserved after every branch)
and `third_hand_offer_regression_never_queries_an_uncompletable_partner_hand`
(the 36-world advantage-01 root) [EXPLORATORY; gates; `RESULTS.md` §Verification
and repair record; `walt/LOG.md` 2026-09-06]. This is a change to
`walt/walt/src/solver`, recorded here because the gym found it.

## 4. Scheme as the situation language

The gym is the first consumer of [`walt::scheme`](walt-scheme-fix.md). A
`.scheme` file is a Fix — typed roles, an `(out …)` interface, one or more
`(case …)` conjunctions of registered predicates — and the gym's contract is
that it returns **exactly one domino action**; every other role is an
existential witness, and repeated witnesses or overlapping cases never
multiply answers or mass. Dependency direction: `walt::gym` imports
`scheme` and the solver; Scheme has no solver dependency and no value
predicate.

### 4.1 The predicate registry

The standard registry (`walt/walt/src/scheme/registry.rs`, version
`scheme-v1/straight-v0.4`, all horizon 0) holds 28 predicates. Four have
`Access::World` — `holds`, `void`, `legal`, `forced` — and may inspect a
hidden world; the other 24 have `Access::Viewer` and read only the viewer's
hand and the public kernel. Three Viewer predicates were added for the gym on
2026-09-07 (`b0c7c0aa`):

| Predicate | Access | Meaning |
|---|---|---|
| `own-legal(action)` | Viewer | `action` is legal for the viewer, who must be next to act |
| `trick-play(seat, tile)` | Viewer | `seat` played `tile` in the current partial trick |
| `leads-context(tile, context)` | Viewer | the context `tile` would lead under the declaration (containing a pip is not the same as leading it) |

The gym registers one extension predicate of its own, `offer-count-to-partner`
(version `gym-v1`, horizon 1 ply, Viewer; `gym.rs::registry()`): true for a
legal count tile when the viewer plays third or fourth, partner is currently
winning, and the candidate leaves partner winning immediately after the play.
It is used by the legacy starter miner and is refused by the plain `scheme`
binary, which knows only the standard registry.

### 4.2 The four query files

`walt/gym/queries/all-legal.scheme` — the whole structural query of the
bid-making family:

```scheme
(fix
  (roles (domino action))
  (out action)
  (case (own-legal action)))
```

`walt/gym/queries/offer-count.scheme` — a legal five- or ten-count play that
leaves the currently winning partner ahead; public and own-hand only:

```scheme
; Return count plays that leave our currently winning partner ahead.
; Every relation is public or own-hand. No bespoke gym predicate.
(fix
  (roles (chair me) (chair mate) (domino action) (domino incumbent) (context led))
  (out action)
  (case (viewer me) (partner me mate) (current-winner mate)
        (trick-play mate incumbent) (led-context led)
        (own-legal action) (count action 5) (beats incumbent action led))
  (case (viewer me) (partner me mate) (current-winner mate)
        (trick-play mate incumbent) (led-context led)
        (own-legal action) (count action 10) (beats incumbent action led)))
```

`walt/gym/queries/overtake-partner.scheme` — a legal play that takes the
current lead away from partner:

```scheme
; Return legal plays that take the current lead away from partner.
; Whether this is useful or wasteful is the independent grader's question.
(fix
  (roles (chair me) (chair mate) (domino action) (domino incumbent) (context led))
  (out action)
  (case (viewer me) (partner me mate) (current-winner mate)
        (trick-play mate incumbent) (led-context led)
        (own-legal action) (beats action incumbent led)))
```

`walt/gym/queries/lead-to-partner-boss.scheme` — a belief query: at the
viewer's lead, partner holds the live boss of the context the candidate would
lead. It uses `holds` (World access):

```scheme
; A belief query: lead a context in which partner holds the live boss.
; This is an entry opportunity, not a guarantee against off-suit trumping.
; "leads-context" is intentional: containing a pip is not the same as leading it.
(fix
  (roles (chair me) (chair mate) (domino action) (domino entry) (context led))
  (out action)
  (case (viewer me) (leader me) (next-actor me) (partner me mate)
        (own-legal action) (leads-context action led)
        (holds mate entry) (boss entry led) (beats entry action led)))
```

The legacy single-predicate form `walt/gym/offer-count.scheme`
(`(fix (roles (domino offer)) (out offer) (case (offer-count-to-partner offer)))`)
is what `assess()` still evaluates to fill every key's `offers` field, and
what the starter `mine` path uses; the ordinary expression above matches it
exactly in every seat rotation, with a different expression identity
(`tests/gym.rs::ordinary_scheme_offer_matches_the_original_predicate_in_every_rotation`).

### 4.3 Presence semantics, and why matching never narrows the grade

`gym::match_query` evaluates the Fix per world and reports **presence**: for
each returned action, the number of compatible worlds in which the relation
holds. If every predicate in the expression has Viewer access the answer is
constant across the fiber, so one world evaluation times the exact world
count suffices and the match is flagged `public: true`. If any predicate has
World access the expression is evaluated over **every** world up to the
explicit cap (`--max-worlds`, refusal when exceeded) and the match is
`public: false`. Discovery selects an action when its presence over the whole
belief is at least `--min-presence` (an exact rational; 1 by default,
`1/4` for the lead family). That threshold is a discovery filter, not a
statistical claim and not a conditioning event: **exact grading still uses
the entire original belief**.

The stored example (`DISCOVERY.md` §Belief queries; the file is
`collections/scheme-v1/lead-to-partner-boss/disadvantage-02.json`, coordinate
`1bdec02c98fb0deb9df4`): sixes trump, seat 3 leads trick 5 holding 4-3, 6-0
and 6-4 with 140 compatible worlds. Leads 6-0 (tile 21) and 6-4 (tile 25)
each connect to a partner boss in exactly 40 worlds — presence 2/7 each, both
above 1/4 — so both are query targets. The key grades all 140 worlds: 4-3
makes in 138/140 and is the unique best; 6-0 in 135/140; 6-4 in 100/140. Both
partner-boss leads are non-optimal here, and the pupil is never handed a
"certain" partner boss. Pins: `test_gym.py::test_fractional_partner_boss_query_keeps_the_entire_grading_belief`
(recomputes the presence and reproduces the unchanged full-belief key with
the native binary) and `tests/gym.rs::hidden_relation_reports_full_fiber_presence_without_conditioning_it`
[EXPLORATORY; gates].

Refusals are total, never partial: wrong output sort, an illegal returned
action, an unregistered predicate, an exhausted work budget
(`--query-work`, default 20,000,000) or a fiber over the cap all refuse the
whole match (`tests/gym.rs::query_contract_budget_and_source_errors_refuse_partial_matches`).
Compile checking (`partnership_gym --check-query FILE`) runs before any worker
starts.

## 5. Matching is not selecting

The pipeline has separate stages, and the separation is the design:

```
coordinate stream (gym.py positions(): tricks 5–6, >1 legal play, contract unsettled, bid 30)
   → Scheme match (targets + presence; public or full-fiber)
   → exact key (assess: every legal action, replays, audits)
   → selection (query | query-required | outcome, plus exact filters, plus side)
   → publication (catalog with sha256 per fixture; each fixture re-verified)
   → pupils (run; grade against the whole optimal set)
```

Nothing in the stream encodes a partnership pattern; nothing in the matcher
reads a value; nothing in the selector re-solves.

### 5.1 The three criteria

With `N` worlds and success mass `m(a)`: `Q(a) = m(a)/N`;
`A* = {a : Q(a) = max Q}` (every tied best play); `regret(a) = max Q − Q(a)`;
`spread = max Q − min Q`; `nearest_mistake = min positive regret` (absent
when all plays tie). Let `A` be the legal plays and `T ⊆ A` the query targets.

| Criterion | Keeps a coordinate when | Category labels | Used by |
|---|---|---|---|
| `query` | some target–non-target pair has a strict difference: **advantage** = a target is in `A*` and strictly beats some non-target; **disadvantage** = a non-target is in `A*` and strictly beats some target (`classify()`) | advantage / disadvantage | starter gallery, scheme-v1 |
| `query-required` | `T` and `A \ T` both nonempty and `max_{a∈T} Q(a) > max_{b∈A\T} Q(b)` — the best matched action strictly beats the **best** unmatched one, so every optimal choice matches (`query_contrast()`, `target_required`) | advantage only | the composed exam |
| `outcome` | `spread > 0` — any strict success-probability difference; no pattern at all | bid-making (declaring) / bid-setting (defending) | bid-making-v1 |

Why "best matched beats best unmatched" matters: beating one bad non-match
while tying another would not show that matching is *needed* for an optimal
choice. `test_gym_spec.py::test_query_required_compares_against_best_alternative_not_a_weak_one`
pins the counterexample; the criterion also refuses empty and full target
sets, which have no matched/unmatched comparison.

### 5.2 The exact filters (specification layer)

| Filter | Meaning | Note |
|---|---|---|
| `min_spread` | inclusive lower bound on `spread` | exact rational string; `0` still requires strict difference |
| `min_mistake` | inclusive lower bound on `nearest_mistake` | the worst mistake may be large while the least harmful one costs almost nothing — a different question from `min_spread` |
| `max_optimal` | upper bound on the size of `A*` (1..7) | counts all best plays; never chooses a tiebreak winner |
| `certain` | keep only certain-success-versus-certain-failure swings (`max Q = 1`, `min Q = 0`) | 26 of the 433 |
| `side` | `declaring` / `defending` / `both` | applied after the criterion |

### 5.3 The label is never a single tile

Selection stores a display pair — the strongest preferred/comparison pair by
gap — but the key and the grading keep **all** actions and the **whole**
optimal set. `bid-making-139` (`4db0e8c642aa5329a4e5`; blanks trump, trick 5,
seat 1 is the bidder, remaining 5-0, 5-5, 6-0, partner's 4-4 led, 200
worlds): 5-0 and 6-0 each make in 200/200, 5-5 in 0/200. The optimal set is
{5-0, 6-0}; either earns full credit; the displayed pair is 5-0 versus 5-5
with gap 1 [EXPLORATORY; fixture-pinned, `BID-MAKING.md`]. All-tied
coordinates are retained as controls, never labeled.

## 6. Artifacts and identities

| Level | Object | Identity and refusal rules |
|---|---|---|
| Query | a `.scheme` file | expression identity = canonical source plus the registry specs it uses (`scheme_identity`, `query_match.identity`); discovery freezes the source as `query.scheme` in its output directory |
| Scenario / fixture | one JSON file: `request` (the seven fields), `key` (schema `partnership-gym-v1`: `field_id`, `worlds`, `legal`, `best`, `offers`, `leader`, `prefix`, `banked`, `trick`, `remaining`, `root_id`, `partner_worlds`, `scheme_identity`, and per action `success_mass`, `score_bins`, `policy_id`, `policy_states`, `traces` with hands/plays/banked/partner_count/success), `semantics`, `audit`, `source` (corpus, path, decision, result and checkpoint sha256), `provenance` (engine/runner/rules/manifest sha256); discovery fixtures add `query_match`, `target_actions`, `min_presence`, `criterion`, `family`, `categories`, `pair`; outcome fixtures add `outcome` (schema `gym-outcome-v1`), `paired_outcomes`, `query_contrast` | self-contained: readable, verifiable and gradable with no raw directory |
| Collection / gallery | `catalog.json` (schema `gym-catalog-v1`: id, file, sha256 per fixture, selection sentence, criterion, side) plus fixtures; `discovery.json` with the full candidate manifest and every row's outcome profile, skips and controls; `coordinates.json` (scheme-v1) | `gallery()` refuses a digest mismatch; every selected fixture is re-verified at publication |
| Specification | `specs/*.json`, schema `gym-spec-v1`: `source.paths`, `domain` (tricks, limit, seed), `query` (name, embedded source, `min_presence`), `evaluation` (`contract` must be `partnership-gym-v1` — corrected 2026-09-20: since `7abf2aee` also `deployed-gym-v1`, §11.1 — `max_worlds`, `partner_worlds`), `selection`; a `reference` block of fingerprints | strict schema (unknown fields refused, exact rational strings, `max_optimal` 1..7, unknown contract refused, never approximated); `evaluation_id` = digest(source, engine, gym, generator, rules, domain, query, evaluation, coordinates); `view_id` = digest(evaluation_id, selection); output roots `evaluations/<evaluation_id>/` and `collections/<view_id>/` with `latest.json` as the receipt; the reference (`count`, `coordinates_sha256`, `answers_sha256` over ids and complete keys, `arguments_sha256`, `source_sha256`) is checked **after** generation and never supplied to the matcher or solver; any `--set` override drops the reference expectation; editing defaults under a frozen reference, or a source that changed under a frozen reference, refuses the run |
| Benchmark | `benchmarks/<name>/manifest.json` (schema `gym-pupils-v1` or `-v2`), `items/<exercise>-<player>.json` (`scenario`, `player`, `response` with the native wire fields, `grade` = choice/optimal/regret/success), `status.json`, and for the composed exam `summary.json` (schema `gym-composed-exam-v1`) and `report.md` | the manifest pins catalog identity, engine, player binary, runner, rules, runtime and matchup hashes and every player configuration; `pin()` refuses resume when the manifest differs ("resume identity changed; use a new output directory"); `report --results` refuses a foreign catalog or a saved grade inconsistent with the saved choice; `gym-pupils-v2` identifies the exam by **questions and full keys** (`catalog_identity: questions-and-keys-v1`, `exam_identity`), so faithful regeneration with new timing metadata still matches (`portable_across_both_generations: true`) |

**Raw runs live outside git.** Non-selected traces, the defending-side and
all-tied bid-making keys, the evaluation caches and the 30 composed fixtures
with their traces are under `/Users/jason/data/texas-42/…` (paths recorded in
`mining.json`, each `discovery.json` and `summary.json`). Checked-in fixtures
are self-contained; a regenerated gallery can be produced from the
specification.

**Resume rules.** Each item is flushed and atomically renamed (`atomic()`:
fsync then rename then directory fsync); `run_lock()` (`flock`) prevents two
coordinators writing one run; failed items retry once on the next invocation
and completed items are never recomputed; Ctrl-C stops admissions and drains
the active coordinate per worker; the outer watchdog kills the process group
on overrun. Pins: `test_gym.py::test_failed_item_retries_but_saved_item_is_not_recomputed`,
`test_interrupt_drains_current_coordinate_then_resume_completes`,
`test_manifest_drift_refuses_resume`, `test_outcome_discovery_publication_and_resume_use_same_full_key`.

**Two warts to know before citing portability.** (1) The shared release
binaries on this machine (`partnership_gym` sha256 `fdf5ae47…`, `partnership`
sha256 `c1051547…`, built 2026-09-07 11:54; re-hashed 2026-09-12) differ from
every engine hash in the checked-in manifests (starter mining `9d8b6471…`,
starter benchmark player `bc27a950…`, composed benchmark player
`cd7014b0…`), so resuming *into* a checked-in benchmark or collection
directory is refused by design; a fresh output directory reproduces the keys
(§8). (2) `collections/scheme-v1/coordinates.json` embeds an absolute corpus
path to the `texas-42-partnership-launch` checkout that produced all five gym
commits; `bid-making-v1` fixtures and the spec layer use repository-relative
paths, and the spec's source fingerprint is over paths relative to each source
(`test_gym_spec.py::test_source_identity_is_portable_and_detects_record_edits`).
Whether anything in `verify`/`report` on scheme-v1 depends on the absolute
path was not verified in this pass (source hashes there are relative, so it
appears not).

## 7. The results ladder

All rows are EXPLORATORY. "Pin" names the checked-in artifact or gate that
makes a number quotable; wall times are machine facts from
`experiments/partnership/runs/<name>/run.json` (all runs on
2026-09-07 UTC, cwd `/Users/jason/code/texas-42-partnership-launch`, status
completed, rc 0).

| Rung | Commit | Result | Pin |
|---|---|---|---|
| 60 mined, 6 starters | `c59f1115` 2026-09-06 | 60 complete keys, 3,273 worlds, 9,554 action/world replays, 21 advantage / 13 disadvantage / 26 no strict pair; 6 published; pupils L1 5/6, L2 Partner 6/6, L2 voids 5/6 | `mining.json`; `scenarios/catalog.json`; `benchmarks/starter-v1`; `tests/gym.rs` third-hand regression |
| 1,929 eligible, 170 strict | `b0c7c0aa` 2026-09-07 | three queries × 1,929 coordinates; 176 memberships = 170 unique; 245 above-cap skips per query | `collections/scheme-v1/{catalog,coordinates}.json` and per-family `discovery.json` |
| 433 bid-making | `67f1e4ab` 2026-09-07 | 1,024 declaring eligible → 837 graded → 433 strict (367 unique best, 26 certain swings); 460 defending strict computed, unpublished | `collections/bid-making-v1/{summary,discovery,catalog}.json` + 433 fixtures; `specs/bid-making.json` reference |
| Specification validation | `75b6a3f1` 2026-09-07 | fresh generation reproduced the 433 identically; `max_optimal=1` → 367; `certain=true` → 26 | `specs/bid-making.validation.json` |
| Composed exam of 30 | `1df741db` 2026-09-07 | 30 positions where offering count is required; L1 24/30, L2 Partner 26/30 | `specs/partnership-bid-making.json` reference; `benchmarks/partnership-bid-making-v1` |

### 7.1 Sixty mined coordinates and the six starters

The legacy `mine` path took the first 60 qualifying trick-5/6 count-offer
positions from the 01-level campaign of the default-partner battery,
deduplicated by request: 60 complete audited keys, zero failures, zero
world-cap skips, 3,273 worlds and 9,554 replays in total (work units, not
samples), final capped run 1.305 s on ten workers (`gym-final-mining`). The
gallery took the first three stable ids per category **before any pupil
ran**; the six keys cover 308 worlds. All numbers recounted from
`mining.json` and the fixtures on 2026-09-12.

| Exercise | Goal | Worlds | Offered count | Preferred | Compared | Preferred success | Compared success | Audit (worlds / actions / information sets) |
|---|---|---:|---|---|---|---:|---:|---|
| advantage-01 | set 30 | 36 | 5-0 | 5-0 | 5-3 | 30/36 | 8/36 | 36 / 2 / 517 |
| advantage-02 | make 30 | 12 | 5-5 | 5-5 | 6-6 | 12/12 | 10/12 | 12 / 2 / 102 |
| advantage-03 | make 30 | 150 | 5-5 | 5-5 | 5-4 | 92/150 | 83/150 | 150 / 3 / 2849 |
| disadvantage-01 | set 30 | 36 | 5-0 | 3-3 | 5-0 | 3/36 | 1/36 | 36 / 3 / 660 |
| disadvantage-02 | make 30 | 50 | 3-2 | 1-1 | 3-2 | 35/50 | 28/50 | 50 / 3 / 1112 |
| disadvantage-03 | make 30 | 24 | 5-5 | 3-0 | 5-5 | 24/24 | 17/24 | 24 / 3 / 556 |

First pupil results (`benchmarks/starter-v1`, manifest `gym-pupils-v1`; seed
420600; ten workers, one native thread each; 14 s move ceiling; 18 decisions
in 0.201 s of capped batch time, no fallbacks; a second invocation resumed
without changing any of the 18 item hashes, `gym-resume`):

| Player (`players.json`: n 40, n0 8, n1 2, fixed selection, 14,000 ms) | Optimal | Mean root regret | Mean decision time |
|---|---:|---:|---:|
| `l1-default` | 5/6 | 1/216 ≈ 0.463 pp | 0.028 s |
| `l2-partner-default` | 6/6 | 0 | 0.031 s |
| `l2-partner-voids` | 5/6 | 1/216 ≈ 0.463 pp | 0.029 s |

The single miss is `disadvantage-01`: L1 and L2-voids choose 2-0 (2/36), L2
Partner chooses 3-3 (3/36); both beat the offer 5-0 (1/36), so the missed
value is **1/36**, not the displayed pair gap of 2/36. `RESULTS.md`: "a
functioning diagnostic score, not evidence that L2 generally beats L1 or that
voids hurt."

**Worked example, advantage-01** (`show advantage-01`, reproduced 2026-09-12):
twos are trump; seat 3 defends with 3-1, 5-0, 5-3 remaining, banked
[8 declaring, 1 defending]; partner seat 1 has led 5-5 and seat 2 played 1-1.
Offering 5-0 raises set probability from 8/36 to 30/36. In the displayed
compatible world (seat 0: 2-1 4-1 5-1; seat 1: 4-3 5-4; seat 2: 4-4 6-4) the
offer lets partner capture 15 count and the declaring team finishes at 25;
playing 5-3 hands partner 10 count and the declaring team reaches 30.

**Worked example, disadvantage-01**: sixes are trump; seat 2 defends and plays
last with 2-0, 3-3, 5-0; partner seat 0 is already winning with 5-4 over the
led 4-3. Withholding count with 3-3 leads partner to lead 5-5 next and the
saved 5-0 follows into that winning trick — declaring team ends at 19 in the
displayed world; offering 5-0 immediately makes partner lead 4-4 next, which
is trumped, and the declaring team reaches 30. "Partner is winning, give
count" has a concrete counterexample; the exact root values average all 36
worlds.

### 7.2 1,929 eligible coordinates; 170 strict across three families

The generic stream over **both** campaigns of the default-partner battery
(400 games on 100 deals) yields 1,929 distinct eligible coordinates. Each
query scanned all of them under the 400-world cap (recounted from the three
`discovery.json` files on 2026-09-12):

| Query | Matched and graded | Target helps | Target hurts | No strict pair | Above cap (skipped) | No query match | Capped wall (workers) |
|---|---:|---:|---:|---:|---:|---:|---:|
| offer-count | 206 | 66 | 51 | 89 | 245 | 1,478 | 13.048 s (4) |
| overtake-partner | 94 | 17 | 18 | 59 | 245 | 1,590 | 10.803 s (3) |
| lead-to-partner-boss (`--min-presence 1/4`) | 53 | 4 | 20 | 29 | 245 | 1,631 | 12.634 s (3) |

Published: 117 + 35 + 24 = 176 fixtures = 176 strict memberships = **170
unique coordinates** (six appear in more than one family; duplicates had
identical requests and keys across families). Additional tallies not in the
docs (2026-09-12): 118 trick-5 and 52 trick-6 coordinates; 10,835 worlds
across the 170 keys. No measurement failed; nonmatches and matched controls
are retained in the discovery summaries. Zero pupil runs were made on this
collection. Above-cap coordinates are unmeasured, not ties.

### 7.3 433 bid-making exercises (outcome only)

The four-line `all-legal` query plus `select --criterion outcome --side declaring --all`
over the same 1,929 coordinates (`bid-making-discovery-1`: 50.616 s on ten
workers; `bid-making-publish-1`: 16.414 s including an independent re-audit
of all 433). All numbers below recounted from `summary.json`, `discovery.json`
and the 433 fixtures on 2026-09-12:

| Measured property | Declaring: make 30 | Defending: set 30 |
|---|---:|---:|
| Eligible | 1,024 | 905 |
| Above the 400-world cap | 187 | 58 |
| Exactly graded | 837 | 847 |
| All legal plays tie | 404 | 387 |
| Strict outcome difference | **433** | 460 |
| Exactly one best play | **367** | 420 |
| Certain success vs certain failure | **26** | 8 |

The 433 published: 191 with 2 legal plays and 1 best, 176 with 3 legal and 1
best, 66 with 3 legal and 2 best; 265 trick-5 and 168 trick-6; in 142 the
least harmful mistake loses at least 1/4 make probability; 311 lie outside the
170-coordinate scheme-v1 gallery. Further tallies not in the docs: the keys
cover 38,886 worlds from 94 distinct source deals; declarations 0–6 all
occur (68/79/45/66/38/53/84), doubles and no-trump never. The 460 defending
keys and every tied key exist only in `/Users/jason/data/texas-42/bid-making-v1`;
`select --side defending` on that run would publish them without new solving.
**No pupil has taken this collection** (`BID-MAKING.md`).

**Worked example, bid-making-17** (`09c783c6730dc633ff10`; source
`02-voids/seeds/750634/declaring`, decision 19; `show bid-making-17`
reproduced 2026-09-12): twos are trump; seat 1 is the bidder, remaining 2-0,
5-0, 5-3, acting last on trick 5 with the trick 4-4, 6-1, 4-1 and banked
[11 defending, 8 declaring]. 60 compatible worlds: 2-0 makes in 60/60, 5-0 in
0/60, 5-3 in 0/60; spread 1; same-world comparison gained 60, lost 0.
Trumping with 2-0 takes the trick's six points and keeps the lead for the
remaining fives; every other five is already gone; giving away the trick
lets the defenders cross the set threshold at once. The key's `offers` field
is empty (the legacy count-offer predicate finds nothing) while
`target_actions` is all three legal plays — the two fields coexist in every
key. Pin: `test_gym.py::test_published_bid_making_witness_is_certain_across_all_sixty_worlds`
(independently replays a native reproduction).

### 7.4 The specification reproduces the collection

`specs/bid-making.json` (arguments sha256 `4bdb6144…`, source sha256
`d77f6b36…`, reference count 433, `coordinates_sha256 1500c25e…`,
`answers_sha256 1f766e67…`). The validation receipt
`specs/bid-making.validation.json`:

| Invocation | Exercises | Seconds |
|---|---:|---:|
| default specification, fresh evaluations | 433 | 67.707 |
| `--set selection.max_optimal=1`, cached evaluations | 367 | 13.991 |
| `--set selection.certain=true`, cached evaluations | 26 | 1.921 |

Position set and complete answer keys matched the reference exactly; both
variants used the same evaluation identity (`19349dac…`) and the cached item
bytes were unchanged afterward (`cache_unchanged_after_filters: true`).
Filtering time includes re-audit and publication.

### 7.5 The composed exam: offering count is required

`specs/partnership-bid-making.json` embeds the unchanged `offer-count` Fix
with `selection.criterion = query-required`, side declaring — every optimal
root choice must offer count. Generation reproduced **30 positions** with
complete native keys in 11.075 s (reference `coordinates_sha256 01d6c635…`,
`answers_sha256 05fa4082…`, arguments `2c14a38d…`); 21 source deal seeds; 23
trick-5 and 7 trick-6; all 30 happen to have a unique best play (not a
selection condition); exactly one certain swing (advantage-09: 3 worlds, 5-0
3/3 versus 3-1 0/3 with 25 banked). The scan graded 206 offer matches over
both sides, skipped 245 above cap, saved 1,478 nonmatches, no failures. Two
independent generations were checked against the saved benchmark identity.
The 30 fixtures with traces are **not** checked in — only `summary.json` rows
(per-action masses, best, `query_contrast`), the spec and the pupil items;
regenerate to inspect replays.

L1 versus L2 Partner on identical requests (`benchmarks/partnership-bid-making-v1`,
manifest `gym-pupils-v2`; seed 420600, fixed search, 14 s ceiling, ten workers;
60 decisions in 1.458 s including pre-run key audits, zero fallbacks, zero
over-budget; recounted from the 60 items on 2026-09-12):

| Player | Optimal | Mean root regret | Mean decision seconds |
|---|---:|---:|---:|
| `l1-default` | 24/30 (80.0%) | 1643/205200 = 0.8007 pp | 0.0860 |
| `l2-partner-default` | 26/30 (86.7%) | 959/205200 = 0.4673 pp | 0.0883 |

L2 improved three, worsened one, agreed on 26; mean regret reduction exactly
**1/300** make probability:

| Exercise | Worlds | L1 choice / regret | L2 Partner choice / regret |
|---|---:|---|---|
| advantage-06 | 210 | 1-1 / 11/105 | 5-0 / 0 |
| advantage-11 | 24 | 5-0 / 0 | 5-3 / 1/24 |
| advantage-29 | 210 | 6-1 / 1/35 | 6-4 / 0 |
| advantage-30 | 120 | 6-0 / 1/120 | 6-4 / 0 |

Both miss advantage-20 (1/20), advantage-22 (1/38) and advantage-27 (1/45);
both solve advantage-09. `PARTNERSHIP-COMPOSITION.md`: "a selected
30-position diagnostic with a specified teacher field, not a general
playing-strength estimate"; no uncertainty interval is inferred from four
disagreements; hidden completions are not independent games. The docs' own
phrase for the result is that offering count is required for an optimal root
choice **under this field** — not that a different partner policy or every
opponent makes cooperation necessary. The full-game battery the coordinates
come from scores L2 Partner against L1 at 14 wins / 14 losses / 72 ties
([walt-partnership-program](walt-partnership-program.md)).

## 8. Cost and reproduction

**The watchdog discipline.** Every long command runs under
`experiments/partnership/packet/texas42-partnership-launch-v0.1/tools/run_capped.py --seconds 295 --output-dir experiments/partnership/runs/<name> -- …`,
which writes `run.json` (command, cwd, start, elapsed, status, return codes)
and kills the process group on overrun; every wall time in the gym docs comes
from those files. `gym.py` itself enforces workers 1..10 and seconds 20..270
("run inside the session watchdog"), `case-seconds` ≥ 14 for discovery,
`max_worlds` 1..10,000, `partner_worlds` 1..640, tricks within 5..6, presence
in (0, 1], and refuses to run with `-O` (its audits are assertions).

| Task | Measured wall | Record |
|---|---:|---|
| 60-coordinate starter mining, 10 workers | 1.305 s | `runs/gym-final-mining` |
| 18 starter pupil decisions | 0.201 s | `runs/gym-final-pupils` |
| offer-count / overtake / lead sweeps over 1,929 (4+3+3 workers) | 13.048 / 10.803 / 12.634 s | `runs/scheme-discovery-*` |
| all-legal sweep over 1,929, 10 workers | 50.616 s | `runs/bid-making-discovery-1` |
| publish 433 with re-audit | 16.414 s | `runs/bid-making-publish-1` |
| fresh generation from `specs/bid-making.json` | 67.707 s (receipt; 67.751 s wall) | `runs/gym-spec-reproduce-1` |
| composed generation (30) | 11.075 s | `runs/partnership-composition-generate-1/-2` (11.053 / 11.153 s wall) |
| 60 composed pupil decisions | 1.458 s | `runs/partnership-composition-pupils-1` |
| focused Rust gates (gym + factor/extraction/model-belief suites) | 42.031 s | `runs/gym-focused-rust` |
| 25 Python tests | 2.759 s | `runs/partnership-composition-tests-2` |

**The 400-world cap.** `max_worlds = 400` is the exact root-support cap
(distinct from `partner_worlds`, the teammate's outer sample count). It
excludes the same 245 coordinates (187 declaring, 58 defending) from every
sweep; they are unmeasured. The spec allows up to 10,000; no larger run has
been made.

*Measured 2026-09-12 on this machine* (worktree at `c00717d1`, shared release
binary `/Users/jason/code/texas-42/walt/target/release/partnership_gym`,
sha256 `fdf5ae47…`):

- `partnership_gym --help` → rc 0: `Usage: partnership_gym [--inspect] [--query FILE] [--max-worlds N] [--query-work N] [--partner-worlds N]` / `Input lines: decl, bid (30), bidder, seat, hand (7 original ids), plays (actor/tile pairs), seed.` (`partnership --help` prints "missing mode": that binary has no help flag; its first stdin line is the mode.)
- `python3 experiments/partnership/gym.py show advantage-01` → the own hand, history, action values (5-0 30/36 optimal, 5-3 8/36) and the examiner-only witness, as quoted in §7.1.
- Reproducing the advantage-01 key: the fixture's seven request fields piped to `partnership_gym` (defaults `--max-worlds 400 --partner-worlds 40`) returned in **0.032 s** with `field_id` equal to the fixture's, 36 worlds, best [15], masses (15, 30) and (18, 8), and `score_bins` and all 72 traces JSON-identical to the checked-in key — despite the binary hash differing from the recorded engine hash. Two-fixture spot check only (the 2026-09-07 survey did the same for bid-making-17 in 0.032 s); byte-for-byte reproducibility across builds is otherwise unverified.
- `gym.py verify` (six starters) 0.08 s; `gym.py report --gallery walt/gym/collections/bid-making-v1` 0.253 s (433 rows); `report --gallery walt/gym/collections/scheme-v1` 0.081 s (170 rows); `report --results walt/gym/benchmarks/starter-v1` 0.033 s and reprinted the §7.1 pupil table.

**Exact command lines** (from the repository root; read-only commands need
no watchdog):

```sh
# read, inspect, audit the checked-in galleries (pure Python, seconds)
python3 experiments/partnership/gym.py report
python3 experiments/partnership/gym.py show advantage-01
python3 experiments/partnership/gym.py verify
python3 experiments/partnership/gym.py report --gallery walt/gym/collections/scheme-v1
python3 experiments/partnership/gym.py show offer-count--advantage-01 --gallery walt/gym/collections/scheme-v1
python3 experiments/partnership/gym.py show bid-making-17 --gallery walt/gym/collections/bid-making-v1
python3 experiments/partnership/gym.py report --results walt/gym/benchmarks/starter-v1

# one exact key by hand (stdin = the seven request lines)
printf 'decl 2\nbid 30\nbidder 0\nseat 3\nhand 0 1 7 9 15 18 22\nplays 0 5 1 3 2 17 3 22 0 23 1 6 2 8 3 9 0 12 1 27 2 10 3 1 0 21 1 26 2 24 3 0 1 20 2 2\nseed 420600\n' \
  | walt/target/release/partnership_gym --inspect --query walt/gym/queries/all-legal.scheme   # header + presence
walt/target/release/partnership_gym --check-query walt/gym/queries/offer-count.scheme          # compile only

# build the two native entry points (under the watchdog)
python3 experiments/partnership/packet/texas42-partnership-launch-v0.1/tools/run_capped.py --seconds 295 --output-dir experiments/partnership/runs/my-gym-build -- cargo build --release --manifest-path walt/Cargo.toml -p walt --bin partnership --bin partnership_gym

# discover with a query, inventory, publish
python3 experiments/partnership/packet/texas42-partnership-launch-v0.1/tools/run_capped.py --seconds 295 --output-dir experiments/partnership/runs/my-discovery-1 -- python3 experiments/partnership/gym.py discover --query walt/gym/queries/offer-count.scheme --source experiments/partnership/campaigns/default-partner-battery --output /tmp/my-discovery --workers 10 --seconds 240 --limit 5000
python3 experiments/partnership/gym.py inventory --source /tmp/my-discovery
python3 experiments/partnership/packet/texas42-partnership-launch-v0.1/tools/run_capped.py --seconds 120 --output-dir experiments/partnership/runs/my-discovery-select -- python3 experiments/partnership/gym.py select --source /tmp/my-discovery --output /tmp/my-gallery --all
#   (for lead-to-partner-boss add --min-presence 1/4; for bid-making use --query walt/gym/queries/all-legal.scheme and select --criterion outcome --side declaring --all)

# generate from a specification (discover + select + reference check in one command; repeat to resume)
python3 experiments/partnership/packet/texas42-partnership-launch-v0.1/tools/run_capped.py --seconds 295 --output-dir experiments/partnership/runs/my-spec-1 -- python3 experiments/partnership/gym.py generate --spec walt/gym/specs/bid-making.json --output /tmp/my-gym --workers 10 --seconds 240
#   variants: --set selection.max_optimal=1 | --set selection.min_spread=1/4 | --set selection.min_mistake=1/4 | --set selection.certain=true | --set selection.side=defending | --set 'domain.tricks=[6,6]' | --set evaluation.max_worlds=1000 | --set 'source.paths=["path/to/campaign"]'
python3 experiments/partnership/packet/texas42-partnership-launch-v0.1/tools/run_capped.py --seconds 295 --output-dir experiments/partnership/runs/my-partnership-generation -- python3 experiments/partnership/gym.py generate --spec walt/gym/specs/partnership-bid-making.json --output /tmp/my-partnership-gym --workers 10 --seconds 240

# run pupils and grade (same output directory + fresh watchdog directory resumes)
python3 experiments/partnership/packet/texas42-partnership-launch-v0.1/tools/run_capped.py --seconds 295 --output-dir experiments/partnership/runs/my-gym-slice-1 -- python3 experiments/partnership/gym.py run --output /tmp/my-gym-pupils --workers 10 --seconds 240 --players l1-default l2-partner-default l2-partner-voids
python3 experiments/partnership/gym.py report --results /tmp/my-gym-pupils
python3 experiments/partnership/packet/texas42-partnership-launch-v0.1/tools/run_capped.py --seconds 295 --output-dir experiments/partnership/runs/my-partnership-pupils -- python3 experiments/partnership/gym.py run --gallery /tmp/my-partnership-gym --output /tmp/my-partnership-pupils --workers 10 --seconds 240 --players l1-default l2-partner-default
python3 experiments/partnership/gym.py report --gallery /tmp/my-partnership-gym --results walt/gym/benchmarks/partnership-bid-making-v1

# gates
cargo test --release --manifest-path walt/Cargo.toml -p walt --test gym
cd experiments/partnership && python3 -m unittest discover -p 'test_gym*.py' -v
```

The Python tests that need the native binary look for it at
`<repo>/walt/target/release/partnership_gym`; a worktree without that target
directory can run only the pure-Python subset (nine of the 25 ran green in the
2026-09-07 survey, 0.112 s).

## 9. Boundaries and open questions

Boundaries, in the docs' own words and honored by the code:

- "Exactness here is relative to the declared uniform mechanical belief and fixed seat-local field; it is not perfect-information optimality or a general player-strength result" (`RESULTS.md`).
- "Selected exercises and hidden completions are not independent random games" (`SPECIFICATIONS.md`); "multiple coordinates can come from one deal" (`BID-MAKING.md`).
- "It is a diagnostic collection, not a representative strength sample or a held-out generalization test" (`README.md`).
- "This prices its root choice followed by an optimal lawful focal continuation. A complete-policy benchmark is a distinct exam" (`README.md`).
- "Teachers and pupils may model different teammates; changing the field changes the exam and requires a new answer key" (`README.md`).
- "It adds no count bonus or hand-crafted decision feature to the player" (`README.md`); the native solver and players were unchanged in every gym commit after the boundary repair of `c59f1115`.
- "Specifications organize experiments; they do not strengthen the evaluator's claims" (`SPECIFICATIONS.md`).
- "Scheme does not invent reachable histories, synthesize new predicates, or turn a structural match into a proven beneficial move on its own" (`DISCOVERY.md`); no compact descriptor transducer is claimed (`README.md` §Transition machinery).
- "One decisive root action does not establish that all later choices are irrelevant. Finding the shortest sufficient sequence of future decisions is a further question" (`BID-MAKING.md`).

Open questions (none has an instrument or a recorded plan unless stated):

1. The 433-exercise collection has never been taken by a pupil; the natural scores are optimal-choice rate and mean root regret with paired comparisons on identical coordinates.
2. The 460 defending-side strict keys and 8 certain set/make swings are unpublished (raw run only).
3. The 245 above-cap coordinates are unmeasured; no `max_worlds > 400` run exists.
4. Field sensitivity: every key is relative to `gym-field-v1`; no run varied `partner_worlds` or the opponent field, so how much of a key is field-specific is unknown.
5. Why L1 and L2 Partner both miss advantage-20, -22 and -27, and what distinguishes the four disagreements — named as "concrete candidates for the next analysis" (`PARTNERSHIP-COMPOSITION.md`), not analysed.
6. Whether L2 Partner's 3-vs-1 edge on 30 selected positions (net 1/300) generalizes: 30 positions from 21 deals, outcome-selected, no interval; the full-game battery is 14/14/72.
7. The shortest sufficient decision sequence after a decisive root action.
8. Materialization policy is inconsistent: the 433 fixtures are checked in as the materialized reference, the 30 composed fixtures are not.
9. Binary identity drift versus reproducibility: manifests pin engine hashes no current build matches; reproducibility across builds rests on two spot checks.
10. Whether `offers` (the legacy predicate) should be retired or kept as a control in all-legal collections; whether the `mine` path and `offer-count-to-partner` are deprecated — no decision recorded.
11. Opening-depth exercises need a structural producer; none exists.
12. Full walt CI with `tests/gym.rs` included has not been recorded green.

**Added 2026-09-20 (Sunshine, §11).** Item 4 (field sensitivity) now has one measurement: switching the continuation from the teacher to deployed L1 changed action values on 87 of 206 common coordinates and best-action sets on 35 (§11.1) — with the caveat that focal continuation, opponents and seed schedules "changed together; this run does not isolate which change causes each reversal." Item 5 (why L1 and L2 Partner miss advantage-20, -22, -27) is analysed under deployed L1: two of the six misses hold and four reverse; of item 5's three, advantage-20 and -22 reverse and advantage-27 holds (§11.2). Item 3 (the 245 above-cap coordinates) is unchanged; the rollout's fresh gym excluded 40 of 336 ([walt-partnership-program §11.3](walt-partnership-program.md#113-the-partner-rollout-l1-partner-rollout-a4c2c20d-2026-09-13)). Items 1, 2, 6–12 unchanged; item 12 still not recorded green. New: (13) the teacher-versus-deployed reversals are not attributed to focal continuation, opponents or seed schedule individually (§11.2); (14) the human at the Mac table is not a modeled continuation — "not a prediction calibrated to a human's behavior" (`PLUNGE.md`, §11.3); (15) gym-intake positions above 400 compatible worlds are saved with an outside-scope result and never valued (§11.3); (16) the live `l1-partner-count-review` still values with the teacher field while `partnership-count-l1.json` values with deployed L1 (partnership §11.4).

## 10. Connections

- **Scheme/Fix** ([walt-scheme-fix](walt-scheme-fix.md)): the gym is the first consumer of `walt::scheme`; the three general predicates `own-legal`, `trick-play`, `leads-context` were added for it; the query-required composition happens at the gym-specification layer and gave Scheme no solver dependency.
- **Source corpus** ([walt-partnership-program](walt-partnership-program.md)): `experiments/partnership/campaigns/default-partner-battery` — 400 games / 11,200 moves on 100 shared deals (seeds 750600–750699), two campaigns (01-level: l2-partner-default vs l1-default 14/14/72; 02-voids: l2-partner-voids vs l2-partner-default 12/17/71), each seat's `result.json`/`checkpoint.json` supplying the original hands, decisions and choices; source fingerprint `d77f6b36…`. The starter `mine` default reads 01-level only; `discover`/`generate` default to both campaigns.
- **The pupils** ([walt-seat-play](walt-seat-play.md)): `l1-default` (the fixed L1 procedure), `l2-partner-default` (L2 Partner — a best response to a modeled L1 partner, never an equilibrium claim) and `l2-partner-voids`, served by the `partnership` binary through `experiments/partnership/player.py` with the 14 s ceiling and reserve/fallback routes; no fallback occurred in any gym run.
- **Consumers of the 30-exercise exam** (same fixed field; results pinned by those campaigns' own `measurements.json`, not by gym files): policy synthesis (`08fad726`, `policy_gym.py`) — sampled information-state tables at 1/4/16/64 training worlds chose an optimal first action on 16/22/25/**26** of 30, mean first-action regret 9.763/2.015/0.616/0.436 pp, mean complete-policy gap 12.322/3.551/1.299/1.008 pp; relational learning (`c00717d1`, `relational_exam.py`) — sampled 16-world table 25/30 (1.299 pp gap), shared exact-regret actor **21/30** (10.929), table + exact-regret fallback 25/30 (1.457), shared interval-cost actor **0/30** (31.129), table + interval fallback 25/30 (1.751); all native teacher optima matched the maintained keys. Both reports call the exam an outcome-selected diagnostic, not an unbiased sample ([walt-partnership-program](walt-partnership-program.md)).
- **Instruments and architecture**: `walt::gym` (548 lines), `partnership_gym`, `partnership`, `gym.py` (840 lines), `gym_spec.py` (240 lines), `rules.py`, `run_capped.py` — catalogued in [walt-instruments](walt-instruments.md); module placement (`gym` above `solver`, importing `scheme`) in [walt-architecture](walt-architecture.md).

## 11. Sunshine (2026-09-13): the gym under deployed continuations, and the live-move intake

> **Tier: EXPLORATORY, as the page banner.** Every number below is a census or a sample under a declared uniform mechanical belief and *named frozen* continuation players, pinned by the campaign files named inline (`RESULTS.md`, `summary.json`, `validation.json`, `WITNESSES.md`); Python test counts are focused runs recorded in those files, never `walt/ci/check.sh` (still waived: "Full repository CI was not run for this Python gym change", `sunshine-recipes-v1/RESULTS.md`). Results files outrank the guides.

**Sorting (curator, 2026-09-20): adds to an area.** Five landings changed the gym on 2026-09-13: `7abf2aee` (12:22 −0500, continuation-selectable recipes), `b11aa189` (12:42, their validation and `offer-five.scheme`), `0727103e` (11:44, the targeted replay), `5bdd8b48` (14:48) and `2c2d7ae2` (23:42, the Mac table and its gym intake). The teacher contract `partnership-gym-v1` (§2) and every collection of §7 are unchanged and remain addressable: "The original teacher contract and frozen references remain available; they answer a different continuation question" (`SPECIFICATIONS.md`). What the players scored under the new recipe is on [walt-partnership-program §11.4](walt-partnership-program.md#114-the-recipes-campaign-default-l1-under-its-own-continuation-7abf2aee-b11aa189-2026-09-13); the two live presets that consume the gym's question are its §11.2 and §11.3.

### 11.1 Continuation-selectable recipes (7abf2aee, validated b11aa189)

**What changed in the specification contract** (`git diff c00717d1..afd46420 -- walt/gym`: four files, 156 insertions, 13 deletions — `SPECIFICATIONS.md` 91 changed lines, `README.md` 26, plus the two new files below):

- `evaluation.contract = deployed-gym-v1` names deployed continuation players separately for `focal`, `partner` and `opponents` (preset names from `players.json`, expanded to their full frozen configurations; binaries and wrapper sources pinned). "Force each legal root action, then let those ordinary players finish. Each decision receives only its own original hand and public history, with the recipe's seed. No optimal focal continuation is substituted. The full-support value is therefore `P(team succeeds | forced action, named frozen continuation)`." The teacher contract's Q(a) (§2.3) prices an *optimal* lawful focal continuation; the deployed value prices the *deployed* one. `README.md` now says "Under the original `partnership-gym-v1` contract, the fixed future field is …" where it said "The fixed future field is".
- `sample_worlds: null` evaluates every mechanically compatible root world under the cap; a positive integer draws that many worlds without replacement with the pinned `sample_seed` and coordinate, all root actions sharing exactly that subset; "Support must still be enumerated under `max_worlds` first: this is a bounded late-game instrument, not an opening-position support sampler." A key says `census` or `sample-without-replacement`; "Sampled values cannot establish a guaranteed make or set, and sampled recipes refuse `selection.certain=true`."
- **Query matching and action values have separate caches.** "Editing Scheme, its presence threshold, the source corpus or domain reuses applicable values for identical own/public requests. Only newly matched coordinates need new values. A changed continuation, sampling recipe, cap or implementation gets its own evaluation identity." "The first realized decision at an exact input is frozen, including a fallback; this is a replayable procedure, not a guarantee of identical timing on rerun." The receipt reports new and reused value keys; "Discovery retains ties, nonmatches and world cap exclusions even when they are absent from the selected exercises."
- `domain` now takes either `tricks: [5,6]` or `own_remaining: [2,3]` (inclusive).
- `selection.criterion` gains `query-avoided` (the best matched action is strictly worse than the best unmatched one; required/avoided refuse empty and full target sets); `selection.min_contrast` measures the absolute best-matched versus best-unmatched gap ("it differs from a comparison against a weaker alternative"); "The broad `query` criterion can include a tied best offer/non-offer when either beats another action. Read its explicit query contrast before calling an offer required. `query-required` is the unambiguous positive-skill collection."
- Execution: deployed evaluation writes `trajectories/status.json`, complete trajectory files and an exact-input decision cache; "SIGINT to the generator stops scheduling and drains active jobs. A hard stop can lose active trajectories, while completed trajectories and decisions survive. There can be up to ten active trajectories at once."
- Keys "retain full terminal traces and a table of frozen own/public decisions. Independent replay checks support, sample membership, legal play, information locality, role configurations, terminal points, paired outcomes and all action masses. It audits recorded decisions; it does not independently prove that a native algorithm would always produce them under every timing realization."
- New command — compare two completed collections (save the first `latest.json`'s gallery path before making a variant):

  ```sh
  python3 experiments/partnership/gym.py compare /tmp/my-gym/collections/FIRST /tmp/my-gym/collections/SECOND --output /tmp/query-change.json
  ```

  It writes JSON and Markdown: added/removed coordinates, changed action values and best-action sets, helpful/harmful/tied query relations, paired terminal witnesses where available. "Stable coordinate IDs identify positions; display names such as `advantage-01` may change when a collection is filtered."
- New `--set` variants listed in `SPECIFICATIONS.md`: `selection.criterion=query-required`, `selection.criterion=query-avoided`, `selection.min_contrast=1/20`, `'domain.own_remaining=[2,2]'`, `evaluation.players.partner=l2-partner-default`, `evaluation.sample_worlds=32`.
- Evidence boundary, reworded: "Census values are exact under a declared belief and frozen future players; sampled values are estimates. Both are audited by independent replay, rather than field-independent optimality or a formal proof of all game mechanics."

Code (`7abf2aee`): `experiments/partnership/gym_deployed.py` (250 lines, new), `gym_generation.py` (172, new), `gym_compare.py` (127, new), `gym_spec.py` (117 lines changed), `test_gym_deployed.py` (131, new), `gym.py` (35 lines changed). The native players were unchanged.

**The two new files.** [`walt/gym/specs/partnership-count-l1.json`](../walt/gym/specs/partnership-count-l1.json) (schema `gym-spec-v1`, name `partnership-count-l1`): `source.paths` = `experiments/partnership/campaigns/default-partner-battery`; `domain` `own_remaining [2, 3]`, `limit 5000`, `seed 420600`; `query` `offer-count` with its Fix embedded verbatim, `min_presence "1"`; `evaluation` `deployed-gym-v1`, `max_worlds 400`, `players` focal / partner / opponents all `l1-default`, `sample_worlds null`, `sample_seed 420600`; `selection` `criterion "query"`, `side "declaring"`, `min_spread "0"`, `min_mistake "0"`, `max_optimal null`, `certain false`, `min_contrast "0"`; `reference null`. [`walt/gym/queries/offer-five.scheme`](../walt/gym/queries/offer-five.scheme) (`b11aa189`) — the count-offer Fix of §4.2 with its ten-count `case` dropped:

```scheme
; Narrow the count-offer family to five-count dominoes only.
(fix
  (roles (chair me) (chair mate) (domino action) (domino incumbent) (context led))
  (out action)
  (case (viewer me) (partner me mate) (current-winner mate)
        (trick-play mate incumbent) (led-context led)
        (own-legal action) (count action 5) (beats incumbent action led)))
```

**Validation** ([`campaigns/sunshine-recipes-v1/`](../experiments/partnership/campaigns/sunshine-recipes-v1/RESULTS.md): protocol written first; `RESULTS.md`, `validation.json`, `run-receipts.json`, `sample-validation.json`, `value-keys.json`, `validate.py`, `publish.py`; "No player or live checker was changed"). The L1 recipe over the development corpus: 1,929 eligible positions with two or three own tiles; 245 above the 400-world cap, 1,478 no match, **206** matched; 9,188 worlds, 26,340 complete trajectories, 166,147 unique frozen own/public decisions; 143 declaring and 63 defending matches; 76 declaring exercises — 27 requiring an offer, 46 favoring withholding, 3 with a tied best offer/non-offer and a weaker third action (the players' scores: partnership page §11.4). The collection retains helpful and harmful cases; the model disagreements are the compare files there. Full generation 283.4 s across three capped invocations including a deliberate SIGINT (all 9,774 already durable decision/trajectory/value records survived byte-for-byte; time-allowance exhaustion also resumed cleanly) and publication; no trajectory failed. The teacher specification reproduced all 30 original coordinates and complete keys exactly. Query/filter variants on the saved valuations:

| Variant on the saved valuations | Exercises | Seconds | New deployed value keys |
|---|---:|---:|---:|
| Required count offers | 27 | 12.02 | 0 |
| Avoided count offers | 46 | 10.79 | 0 |
| Required offers, best-class gap at least 1/20 | 23 | 11.10 | 0 |
| Scheme changed to five-count offers only | 25 | 11.45 | 0 |

All 192,693 value/trajectory/decision records stayed byte-for-byte unchanged through the variants (timings include independent audit and publication). The five-count query matched 96 already-valued positions; its 25 exercises are a subset of the 76; on the 96 it changed nine target-class relationships but "**no action values or best-action sets**: exactly the intended separation between question and value." The independent verifier replayed all 206 complete value keys (support, legal play, own/public locality, roles, terminal points, make counts); a separate three-world native sample reproduced the corresponding census traces exactly and selected zero strict exercises because every root action made in two of the three sampled worlds — "a sampling/labeling check, not a strength result"; unit tests check that all-success or all-failure samples make no guaranteed-outcome claim. 83 partnership Python tests passed; after later publication fixes (sample-coverage labels, distinct collection identities for renamed recipes, atomic query snapshots) all 39 focused gym tests passed; "The native continuation and action values are unchanged." All 45 roots of §11.2 "reproduce their deployed action values and complete best-action sets exactly in this generic recipe." Raw frozen decisions and trajectories: `/Users/jason/data/texas-42/sunshine-recipes-v1` (compressed hash index identified by path and SHA-256 in `validation.json`). "The main measured implementation is commit `7abf2aee`, whose source hashes were checked against the receipts." As with earlier implementation changes, the conservative source-hash cache identity gives every regeneration a new namespace; previously published collections remain readable and usable for player exams.

### 11.2 The targeted replay: teacher labels under deployed L1 (0727103e)

Guide: [`experiments/partnership/GYM-REPLAY.md`](../experiments/partnership/GYM-REPLAY.md) — "The gym's original answer asks how well a first move can do with an optimal lawful focal continuation and a specified partner/opponent model. This replay instrument asks what happens when the deployed players actually continue. It targets competence on selected partnership situations, including rare mistakes that need not noticeably move ordinary-game win rates." Protocol, written before the outcomes: [`campaigns/sunshine-gym-replay-v1/PROTOCOL.md`](../experiments/partnership/campaigns/sunshine-gym-replay-v1/PROTOCOL.md); results: [`RESULTS.md`](../experiments/partnership/campaigns/sunshine-gym-replay-v1/RESULTS.md); `panel.json` (portable: the exact own/public requests, source coordinates, teacher grades and outcomes, and every mechanically compatible remaining deal), `summary.json` (schema `sunshine-gym-replay-report-v1`), `witnesses.json` and [`WITNESSES.md`](../experiments/partnership/campaigns/sunshine-gym-replay-v1/WITNESSES.md), `run-receipts.json`, `resume-verification.json`, `artifact-hashes.json.gz` (48,940 entries), `publish.py`, `verify-interruption.py`; runner `experiments/partnership/gym_replay.py` (329 lines), tests `test_gym_replay.py`. Purpose, as the protocol quotes Jason: "competence on an uncommon but frustrating partnership mistake. Ordinary-game prevalence is not the primary endpoint." "This experiment requires no new native player implementation."

**Panel.** The six baseline misses of the 30-position composed exam (§7.5, under `sunshine-partner-count-v1`'s `gym-30.json`), the single certain case advantage-09, and all 38 strict withholding successes of the broader gallery (declaring-side positions where review retained the baseline and the teacher value strictly exceeds every offer): **45 distinct roots from 24 source seeds, 1,919 root/world pairs** (746 + 1,170 + 3); bid 30, make/set only, equal world weights; support equality against the old teacher traces is checked by the independent rules when the panel is prepared. "These are selected diagnostic cases, not an estimate of false-positive frequency on ordinary play."

**Three comparisons on the same hidden hands** (deployed `l1-default` 40/8 at all seats after the root; the reviewed arm `l1-partner-count-review`; seed 420600 with the live wrapper's derivation): fixed offer versus baseline (force the teacher-preferred offer or the baseline first move, then ordinary L1 — "even if the reviewer fails to select it within its budget"); reviewed first move versus baseline (the reviewer's actual root choice, then L1; "Every legal action is replayed, so its rank can also be inspected"); complete reviewed partnership versus baseline (reviewed players on both declaring seats afterward, L1 opponents). "Only the examiner forces test actions"; decisions are cached by the exact request and complete player configuration, "Thus all worlds sharing information get the same frozen realized decision, including any timeout or fallback."

**The six misses** (`RESULTS.md`; per-root values in `summary.json` `roots`):

| Original exercise | Baseline play | Offer | Baseline makes | Offer makes | Offer saves / loses | Actual root review |
|---|---|---|---:|---:|---:|---|
| advantage-06 | 1–1 | 5–0 | 129/210 | **151/210** | 28 / 6 | times out; keeps baseline |
| advantage-20 | 2–0 | 3–2 | **28/40** | 27/40 | 0 / 1 | chooses offer |
| advantage-22 | 4–0 | 6–4 | **55/76** | 51/76 | 4 / 8 | chooses offer |
| advantage-27 | 6–5 | 6–4 | 57/90 | **58/90** | 1 / 0 | chooses offer |
| advantage-29 | 6–1 | 6–4 | **167/210** | 160/210 | 13 / 20 | chooses offer |
| advantage-30 | 6–0 | 6–4 | **108/120** | 107/120 | 2 / 3 | chooses offer |

"**Two of the six originally missed offers still improve making the bid when deployed L1 players continue; four reverse and become worse.**" The offers are uniquely best among all tested legal root moves in advantage-06 and advantage-27; the original baseline is uniquely best under L1 continuation in the other four — "action values for that fixed continuation policy, not best-response values against arbitrary players." Advantage-06 holds both a world where 5–0 moves declaring points 18 → 40 and one where it moves 40 → 23 (`WITNESSES.md`, worlds 0 and 13); "The offer remains the better guess under the declared uniform root prior: its 28 saves exceed its six losses."

| Comparison on the six misses (`summary.json` `groups`, group `miss`) | Saves | Losses | Ties | Mean change across the six roots |
|---|---:|---:|---:|---:|
| Always take the teacher's preferred offer, then L1 | 48 | 38 | 660 | −0.0571 pp (−41/71820) |
| Take the reviewer's actual root choice, then L1 | 20 | 32 | 694 | −1.8031 pp (−37/2052) |
| Use the reviewed declaring team throughout | 21 | 32 | 693 | −1.7238 pp (−619/35910) |

"Means give each root equal weight. Pooled saves/losses give each enumerated world equal weight, so they can have a different sign when roots have different support sizes. In particular, 48 saves versus 38 losses is a pooled net gain of 10/746, but not an equal-root gain. Neither weighting estimates ordinary-game prevalence or supplies independent-trial confidence bounds." The reviewer keeps the baseline at advantage-06 because its 250 ms allowance expires (`summary.json`: `reviewed_us` 264,788, status `unresolved`, reason `timeout`); later review adds one saved world there (whole-review 130/210 vs 129/210); the other five roots have identical outcomes under the root-only repair and the whole reviewed partnership — "later deployed reviews do not rescue the four adverse root changes in this panel."

**Controls.** All **38 withholding controls** retain their baseline root move; both the root-only and reviewed-team comparisons tie baseline on all **1,170 worlds** — "the intervention causes no additional harm on these selected controls." Their teacher labels also need qualification: under deployed play the teacher-preferred offer remains worse in 34 roots, ties in two, becomes better in two; forcing those offers saves nine worlds and loses 159. One reversal has an even better non-offer available; the other, disadvantage-23, favors offering 6–4 over keeping 6–0 by 109/120 versus 107/120 — "an additional missed opportunity under the deployed continuation." The positive control **advantage-09**: give partner 5–0 and make in **3/3** worlds, or overtake with 3–1 and make in **0/3**; both baseline and reviewed L1 already give the count.

**What it resolves** (`RESULTS.md`). "The Scheme expression still identifies the intended public relationship: giving count while leaving partner currently ahead. The old evaluator also answers its declared question correctly on the retained finite traces. What changed here is the continuation: deployed L1 opponents replace the teacher's L0 opponents; deployed focal L1 replaces an optimal lawful focal continuation; and live-wrapper field seed schedules differ from the teacher's schedules. This experiment changes those together and cannot assign the reversals to one of them individually." "Four labels fail to transfer even on their exact original public positions." "The query detects a useful kind of opportunity, but the current checker's value judgment is not reliably calibrated to the players that will continue." "This is a conditional competence test for a rare partnership annoyance, not a prevalence requirement" (the 2026-09-13 landing note on [walt-scheme-fix](walt-scheme-fix.md), now folded here); the deployed-continuation labels and paired witnesses are retained for reuse. No player, sample budget, belief, tie-break or reviewer cap changed.

**Execution.** Every legal action plus the reviewed-team arm completed: **7,338** full trajectories (each independently audited), **41,602** unique frozen decisions, **64,980** continuation decision uses (`summary.json`); decision routes baseline 12,195 / baseline-reviewed 10 / forced 29,397; root review statuses 38 retained, 5 changed, 1 timeout, 1 inactive. A deliberate SIGINT stopped after 13 durable trajectories; resume preserved all **121** saved trajectory/decision files byte-for-byte; 0.485 s interrupted + 57.712 s resumed = **58.20 s** including the final complete replay audit; panel preparation and original-trace checks 0.750 s. Mean root invocation 7.52 ms for L1 and 42.47 ms for reviewed L1 on this endgame panel — "Cached continuation speed is not live arena latency." **76 Python tests** passed (seven new). Player source and binary identities match `sunshine-partner-count-v1`. Raw report: `/Users/jason/data/texas-42/sunshine-gym-replay-v1/report.json` (sha256 `9bbaa7c4…`, pinned in `summary.json` `raw_report`). "No Lean or population-strength claim is made."

**Run and resume** (`GYM-REPLAY.md`; the release `partnership` and `partner_review` workers unchanged):

```sh
python3 experiments/partnership/packet/texas42-partnership-launch-v0.1/tools/run_capped.py --seconds 295 --output-dir /tmp/gym-replay-slice-1 -- python3 experiments/partnership/gym_replay.py run experiments/partnership/campaigns/sunshine-gym-replay-v1/panel.json /tmp/gym-replay-results
python3 experiments/partnership/gym_replay.py report experiments/partnership/campaigns/sunshine-gym-replay-v1/panel.json /tmp/gym-replay-results
```

Repeat `run` with the same panel and result directory and a new watchdog directory to resume; exit 75 means incomplete but resumable; changed code, binaries, panel or player configuration refuses silent resume; defaults ten continuations and two native threads per worker (`--workers` reduces). `report` "refuses a partial report and verifies every trajectory, exact world, request, player configuration, cached response hash, root intervention, score and make flag"; `status.json` reports saved jobs, active job IDs and failures.

**The four witnesses** (`WITNESSES.md`: "examiner-visible hidden-hand witnesses, not information available to the actor. Each pair changes only the root move; all later seats use deployed L1."): advantage-06 gain, world 0 — baseline 1–1 → declaring 18, set; offer 5–0 → 40, make. advantage-06 loss, world 13 — 1–1 → 40, make; 5–0 → 23, set. advantage-20 loss, world 32 — 2–0 → 30, make; 3–2 → 25, set. advantage-27 gain, world 80 — actor S3, partner S1, called suit 1, bid 30, banked [6, 8], current plays S0 3–3, S1 4–1, S2 4–0, S3 holding 2–0, 6–4, 6–5: 6–5 → 25, set; 6–4 → 36, make. Across advantage-27's 90 worlds the offer saves one contract and loses none, and the reviewed player selects it — the retained "concrete instance of the intended annoyance."

### 11.3 The Mac sunshine table and the live-move gym intake (5bdd8b48, 2c2d7ae2)

Guide: [`experiments/partnership/PLUNGE.md`](../experiments/partnership/PLUNGE.md) — "Play straight 42 with the shared Walt player, save a particular decision, then compare its alternatives in the continuation-selectable gym. This is the human-play loop for the sunshine goals." Study and integration evidence: [`campaigns/sunshine-playable-v1/RESULTS.md`](../experiments/partnership/campaigns/sunshine-playable-v1/RESULTS.md) (the override study and the played examples are on [walt-partnership-program §11.5](walt-partnership-program.md#115-the-playable-campaign-the-override-study-and-the-first-played-examples-5bdd8b48-2c2d7ae2-2026-09-13)); the stats view: [`campaigns/sunshine-review-v1/RESULTS.md`](../experiments/partnership/campaigns/sunshine-review-v1/RESULTS.md). Code: `plunge_bridge.py` (the localhost bridge, 217 lines at `5bdd8b48`), `plunge_io.py` (imports finished hands), `plunge_analysis.py` (runs the deployed gym), `play_plunge.py` (supervises the two local services), `plunge_check.py` (audits a live receipt and real native pause/resume), `override_study.py`, `test_plunge.py`. Plunge companion: checkout `/Users/jason/code/plunge-sunshine`, branch `codex/sunshine-table` from plunge main `122ea7a5`; commits `adfd7d4b3707f2324c395c88e5f7dbae1d3b315a` (`5bdd8b48`) and `ac7e65a20f213823bfb926ad62e5b3c5443bbb4c` (`2c2d7ae2`); "both worktrees remain local" (`walt/LOG.md`). Later changes to the same table — the 2026-09-14 `walt-player` port, regular auction and deeper profiles, and the 2026-09-19/20 played-game bid book, forced last bid and CPU speedups recorded at the top of `PLUNGE.md` — belong to [walt-seat-play](walt-seat-play.md) and [walt-kiln](walt-kiln.md), not to this section.

**As landed 2026-09-13.** `python3 /Users/jason/code/texas-42-partnership-launch/experiments/partnership/play_plunge.py` (the research checkout), then <http://127.0.0.1:4244>; choose **L1 + partner check** or **L1** — the native `l1-default` or `l1-partner-rollout`; the choice controls all three computer seats; you play seat zero with Gran as partner. At this landing contracts were assigned 30 bids with rotating bidders and computer declarations by Plunge's existing hard player ("practice under assigned contracts, not a voluntary bidding evaluation"; replaced by the regular auction on 2026-09-14). Optional arguments `--plunge PATH`, `--data PATH`, `--port 4244`, `--bridge-port 4245`; both services bind 127.0.0.1 only; an occupied port errors rather than stopping another service; Plunge's offline service worker is not installed locally and `/api/` responses bypass offline caching. Build `partnership`, `partner_rollout` and `partnership_gym` under the watchdog (and, since the port, `walt-table` from `walt-player`). Ctrl-C stops table, service and any comparison; completed records remain; **Resume your game** restores the browser save; native failures pause play with a Retry button — "they do not substitute a different browser AI."

**Information boundary.** `/api/decide` "accepts exactly the seven own/public request fields plus the selected preset and game/hand identifiers. Full-deal fields are rejected." Every native response is saved before replying; an identical request under the same recorded implementation reuses its saved answer. "Original decisions are retrieved, never reconstructed from a new solve."

**Flag a move and bring it back to the gym.** After a hand, **See how it went** → tap a domino in the trick history → add a note and optionally another legal play → **Save this move for the gym** (stores the hand, the exact position before that play, and the original receipt; human moves can be flagged too, "explicitly shown without a native decision receipt"). **Copy flagged-move link** reopens the move on this Mac (needs the local server and its data); **Share this hand** is the portable Plunge link (deal and actions, no note or receipt); neither overwrites an ongoing game. The importer (`plunge_io.py`) "validates ownership, turn order, following, trick winners and the stopping point against the exported Plunge record" — "Plunge stops when a points contract settles; the importer checks that stopping point instead of demanding artificial plays after the result." Full dealt hands are examiner data (`flags/`); actor-only requests plus a source annotation are the gym inputs (`gym-inputs/`).

**Compare every legal play** — three named continuation models (`PLUNGE.md`):

| Selection | Focal player's later turns | Partner | Opponents |
|---|---|---|---|
| L1 at every seat | L1 default | L1 default | L1 default |
| Partner uses L2 | L1 default | L2 Partner default | L1 default |
| L1 + partner check at every seat | L1 + partner check | L1 + partner check | L1 + partner check |

"The focal player is the actor at the flagged decision, not necessarily you." Every root action is tried over all mechanically compatible hands with the named players deciding from their own hands and public history: "This is a census under a uniform mechanical belief and frozen continuations. It is not a prediction calibrated to a human's behavior, a perfect-information solve, or a claim about optimal 42." The display scores the focal team's make/set frequency; equal frequencies tie; excess count does not break ties. Full-comparison limit **400** compatible hands — larger positions are saved as gym inputs with an explicit outside-scope result; "There is no hidden early-game enumeration." One comparison at a time, four trajectory workers with two native threads each, a 240-second working allowance per slice inside an external 295-second watchdog; pause at any time; complete trajectories and exact-input decisions persist and resume reuses them; "An incomplete slice reports partial, never a full-census score." Changing the continuation, player settings or evaluation implementation selects a different cache; UI presentation does not.

**Records** (default `/Users/jason/data/texas-42/plunge-sunshine/`): `decisions/` (requests, preset parameters, implementation identity, responses, timing, content checksum — saved before replying), `flags/`, `gym-inputs/`, `analyses/` (pinned evaluation manifest, resumable decisions/trajectories, complete audited key, display result), `server/` (logs), and a `runs/` directory per comparison with watchdog status.

**The stats view** (`2c2d7ae2`; `sunshine-review-v1/RESULTS.md`: "This is a review instrument change, not a player-strength experiment. Native live presets and their choices are unchanged."). The finished-hand review shows trump, the led suit, the actor's remaining hand and legal choices, and the original saved L1 option scores — **make** for a member of the declaring team, **set** for a defender; "These are sampled model estimates. A forced move has an explanation, not an invented probability." If the partner check changed L1's choice, the L1 scores are labelled as preceding that check. "Rejected primary results never replace accepted fallback evidence, which retains its actual sample size." **Look closer · 160 worlds** (and, for human plays and shared hands without receipts, **Ask Walt · 40 worlds**) runs the current native L1 on the same own/public position and seed in a separate worker — one inspection at a time, a concurrent request gets a retry message; 40 worlds at 14 s, 160 worlds at 20 s since `657576e8`; a failed large comparison reports its actual smaller fallback sample and stays retryable; no gym support-size cap, "so opening leads are inspectable too." Estimates persist under `estimates/`, keyed by request, player settings and implementation; frontend changes do not invalidate them; the original decision and its evidence are never rewritten. Examples: flag `5ed686b48b5e493e84866704d7b9b862`, Ruby's 6–6 discard instead of 5–1 — the recorded 40-world sample 24/40 sets (60 %) for 6–6 versus 22/40 (55 %) for 5–1; a later 160-world recheck 87/160 (54.375 %) versus 75/160 (46.875 %), again 6–6; "These are model-relative estimates, not proof that 6–6 is the optimal move." Flag `dbd53c9c432e444cb2db9010bcdc1f27`, Earl's 2–2 overtaking partner Ruby's 6–2 lead with deuces trump — 2–2 was Earl's only remaining trump; the view shows the legal constraint and no probability table. Timings: the human double-three lead's five-option 40-world comparison 0.31 s; the double-six 160-world inspection 0.55 s — "example timings, not a general performance estimate." Validation: 149 Plunge tests (18 focused native stats, transport, cache and component tests re-passed after layout changes); 100 partnership Python tests (40/160 cache separation, immutable original evidence, presentation-independent reuse, strict own/public input, concurrent-inspection rejection, separate live worker, retry after incomplete evaluation, worker cleanup); native-mode typecheck/build; a live native HTTP audit (40/160 completed estimates, exact cache reuse, forced moves without scores, hidden-input rejection, unchanged hashes for all original decisions, flags and gym inputs); browser checks of both saved examples. Receipts and the audit script: `/Users/jason/data/texas-42/sunshine-review-v1/`.

**First live examples** (`sunshine-playable-v1`; details on the partnership page): one browser-played hand, 18 native receipts, two flags — Earl's defending 6–4 sets 12/18 versus 6/18 for 2–2 under full L1 continuations ("competence already present in L1, not a new-check gain", `walt/LOG.md`); Gran's 6–2 under an L2 partner 41/60 versus 38/60 (4–4) and 25/60 (6–6) over 60 compatible hands. "Human play can now supply the next hypotheses and controls" (`SUNSHINE-NOTES.md`).

## Gates: the pin inventory

**Eight Rust gates, `walt/walt/tests/gym.rs** (run by `cargo test --workspace --release` in `walt/ci/check.sh`; last recorded focused run `runs/gym-focused-rust`, 2026-09-07 03:36 UTC, rc 0; not run in this pass):

| Gate | Pins |
|---|---|
| `public_bridge_and_offer_query_are_seat_covariant_and_world_invariant` | the 6-world hold fixture (hand [4,14,17,20,23,24,27], blanks trump, viewer S2, banked 19, prefix 4-0 4-2) builds identically in all four rotations and the offer query's answer is identical in every world |
| `hold_count_has_strict_make_advantage_and_complete_replay_keys` | offers [5-5], best [6-3], masses (5-5, 2) and (6-3, 4); 6 traces of 12 plays each summing to 42; cap 5 refused |
| `third_hand_offer_regression_never_queries_an_uncompletable_partner_hand` | the advantage-01 root: 36 worlds, best [5-0], masses (5-0, 30), (5-3, 8) — the zero-completion boundary |
| `malformed_or_private_inconsistent_history_is_rejected` | wrong actor, wrong seat, six-tile hand, another seat playing an own tile |
| `positive_support_pruning_preserves_factor_weights_not_marginal_weights` | §3.1 |
| `ordinary_scheme_offer_matches_the_original_predicate_in_every_rotation` | presence [(5-5, 6)] from the ordinary expression equals the legacy predicate's in every rotation, with different identities |
| `hidden_relation_reports_full_fiber_presence_without_conditioning_it` | a `holds` query reports fractional presence over all 6 worlds without conditioning; cap refusal when worlds exceed it |
| `query_contract_budget_and_source_errors_refuse_partial_matches` | wrong output sort, unregistered predicate, budget 1, plus compile of the other two query files |

Five of these landed at `c59f1115`, three at `b0c7c0aa`. `RESULTS.md` counts
61 distinct focused Rust tests in the first session (5 gym + 19 Scheme + 6
extraction + 11 factor-belief + 5 factor-recursion + 8 model-belief + 7
model-belief-recursion).

**Twenty-five Python tests** (`experiments/partnership/test_gym.py`, 17;
`test_gym_spec.py`, 8; not run by `walt/ci/check.sh`; recorded 25/25 OK in
`runs/partnership-composition-tests-2`):

`test_gym.py`: `test_every_published_key_has_exact_support_and_lawful_replays`,
`test_full_optimal_set_and_set_outcomes_tie_without_point_tiebreak`,
`test_private_teacher_fields_are_refused`,
`test_tampered_mass_missing_world_or_illegal_trace_is_rejected`,
`test_native_key_agrees_with_published_key`,
`test_manifest_drift_refuses_resume`,
`test_report_refuses_foreign_catalog_and_tampered_grade`,
`test_declarative_offer_matches_legacy_detector_on_all_original_mining_roots`,
`test_generic_stream_includes_positions_outside_original_pattern`,
`test_generic_targets_control_pair_categories`,
`test_outcome_geometry_retains_ties_and_distinguishes_certainty`,
`test_outcome_discovery_publication_and_resume_use_same_full_key`,
`test_published_bid_making_witness_is_certain_across_all_sixty_worlds`,
`test_published_discovery_is_distinct_by_coordinate_and_queries_broaden_it`,
`test_fractional_partner_boss_query_keeps_the_entire_grading_belief`,
`test_failed_item_retries_but_saved_item_is_not_recomputed`,
`test_interrupt_drains_current_coordinate_then_resume_completes`.

`test_gym_spec.py`: `test_reference_is_an_expectation_and_variants_are_explicit`,
`test_bad_arguments_refuse_before_running`,
`test_filters_use_exact_outcomes_and_accept_all_tied_best_actions`,
`test_query_required_compares_against_best_alternative_not_a_weak_one`,
`test_regenerated_exam_identity_ignores_artifact_metadata_but_pins_question_and_key`,
`test_partnership_specification_embeds_the_existing_expression_and_required_contrast`,
`test_source_identity_is_portable_and_detects_record_edits`,
`test_generation_retries_failed_coordinate_and_filters_share_evaluations`.

**Added 2026-09-20 (Sunshine, §11; none run in this pass).** New Python suites: `test_gym_deployed.py` (131 lines), `test_gym_replay.py`, `test_plunge.py`, and in the partnership program `test_partner_review.py`, `test_sunshine_worlds.py`, `test_partner_rollout.py`. Focused counts as the results files record them: 69 partnership Python tests (`sunshine-partner-count-v1`), 76 (`sunshine-gym-replay-v1`), 83 (`sunshine-recipes-v1`; 39 focused gym tests after its publication fixes), 89 (`sunshine-rollout-v1`), 97 (`sunshine-playable-v1`), 100 (`sunshine-review-v1`). New native gate file `walt/walt/tests/partner_rollout.rs` (106 lines; 38 targeted native tests in `sunshine-rollout-v1`; 35 focused Rust tests with strict clippy in `sunshine-partner-count-v1`). `walt/ci/check.sh` remained waived.

## Timeline

| When (−0500) | Commit | Landing |
|---|---|---|
| 2026-09-06 | — | `experiments/partnership/SCHEME-GYM-ASSESSMENT.md` proposes a Scheme-described exact-key gym; `walt::scheme` commissioned and implemented the same day (`b764665f`) |
| 2026-09-06 22:49 | `c59f1115` | first exact gym: `walt::gym`, `partnership_gym`, `gym.py` mine/select/run/verify/report/show, 60 mined, six starters, starter-v1 benchmark, zero-completion repair |
| 2026-09-07 00:09 | `b0c7c0aa` | Scheme query files drive discovery; three predicates added; 170 strict coordinates published as scheme-v1 |
| 2026-09-07 00:44 | `67f1e4ab` | outcome-only discovery; 433 bid-making exercises |
| 2026-09-07 01:01 | `75b6a3f1` | specification layer `gym_spec.py`, `specs/bid-making.json`, validation receipt |
| 2026-09-07 01:14 | `1df741db` | composed specification; 30-exercise exam; L1 24/30 vs L2 Partner 26/30 |
| 2026-09-07 03:29 | `08fad726` | policy synthesis consumes the exam (no gym file changed) |
| 2026-09-07 09:45 | `c00717d1` | relational learning consumes the exam; HEAD of the surveyed state |
| 2026-09-13 11:15 | `58e15cd1` | `sunshine-partner-count-v1` grades the composed exam and the 117-position gallery with the optional review (24/30 → 29/30, 100/117 → 107/117; partnership §11.2); no gym file changed |
| 2026-09-13 11:44 | `0727103e` | targeted replay: the six misses, 38 withholding controls and the certain case under deployed L1 — two hold, four reverse (§11.2); `GYM-REPLAY.md`, `gym_replay.py` |
| 2026-09-13 12:22 | `7abf2aee` | continuation-selectable recipes: `deployed-gym-v1`, separate matching/valuation caches, `query-avoided`, `min_contrast`, `own_remaining`, sample mode, `gym.py compare`; `specs/partnership-count-l1.json` (§11.1) |
| 2026-09-13 12:42 | `b11aa189` | `sunshine-recipes-v1`: 206 matched, 76 declaring exercises, four comparisons, interruption proof; `queries/offer-five.scheme` (§11.1) |
| 2026-09-13 14:48 | `5bdd8b48` | Mac Plunge bridge, flag-to-gym intake with three named continuation models, 400-world full-comparison cap; `PLUNGE.md` (§11.3) |
| 2026-09-13 23:42 | `2c2d7ae2` | isolated native 40/160-world move inspection in the Plunge stats view; `sunshine-review-v1` (§11.3) |
