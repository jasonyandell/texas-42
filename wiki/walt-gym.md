[Home](Home.md) · owns: the exact partnership gym — what a coordinate is, how an answer key is defined and audited, Scheme-driven discovery, the 6 → 170 → 433 → 30 results ladder, and how to reproduce every number · Sources: [`walt/gym/README.md`](../walt/gym/README.md), [`RESULTS.md`](../walt/gym/RESULTS.md), [`DISCOVERY.md`](../walt/gym/DISCOVERY.md), [`BID-MAKING.md`](../walt/gym/BID-MAKING.md), [`SPECIFICATIONS.md`](../walt/gym/SPECIFICATIONS.md), [`PARTNERSHIP-COMPOSITION.md`](../walt/gym/PARTNERSHIP-COMPOSITION.md); the query files `walt/gym/queries/*.scheme` and `walt/gym/offer-count.scheme`; the JSON artifacts (`walt/gym/mining.json`, `scenarios/`, `collections/scheme-v1/`, `collections/bid-making-v1/`, `specs/`, `benchmarks/`); `walt/walt/src/gym.rs`, `walt/walt/src/bin/partnership_gym.rs`, `walt/walt/tests/gym.rs`; `experiments/partnership/gym.py`, `gym_spec.py`, `rules.py`, `test_gym.py`, `test_gym_spec.py`; `experiments/partnership/runs/*/run.json`; `walt/LOG.md` entries of 2026-09-06 and 2026-09-07. Results files outrank prose throughout.

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

Repository state as of 2026-09-07 (`c00717d1`); the gym itself is unchanged since `1df741db` (2026-09-07 01:14 −0500). Fresh measurements on this page are labeled *measured 2026-09-12 on this machine*.

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
| Specification | `specs/*.json`, schema `gym-spec-v1`: `source.paths`, `domain` (tricks, limit, seed), `query` (name, embedded source, `min_presence`), `evaluation` (`contract` must be `partnership-gym-v1`, `max_worlds`, `partner_worlds`), `selection`; a `reference` block of fingerprints | strict schema (unknown fields refused, exact rational strings, `max_optimal` 1..7, unknown contract refused, never approximated); `evaluation_id` = digest(source, engine, gym, generator, rules, domain, query, evaluation, coordinates); `view_id` = digest(evaluation_id, selection); output roots `evaluations/<evaluation_id>/` and `collections/<view_id>/` with `latest.json` as the receipt; the reference (`count`, `coordinates_sha256`, `answers_sha256` over ids and complete keys, `arguments_sha256`, `source_sha256`) is checked **after** generation and never supplied to the matcher or solver; any `--set` override drops the reference expectation; editing defaults under a frozen reference, or a source that changed under a frozen reference, refuses the run |
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

## 10. Connections

- **Scheme/Fix** ([walt-scheme-fix](walt-scheme-fix.md)): the gym is the first consumer of `walt::scheme`; the three general predicates `own-legal`, `trick-play`, `leads-context` were added for it; the query-required composition happens at the gym-specification layer and gave Scheme no solver dependency.
- **Source corpus** ([walt-partnership-program](walt-partnership-program.md)): `experiments/partnership/campaigns/default-partner-battery` — 400 games / 11,200 moves on 100 shared deals (seeds 750600–750699), two campaigns (01-level: l2-partner-default vs l1-default 14/14/72; 02-voids: l2-partner-voids vs l2-partner-default 12/17/71), each seat's `result.json`/`checkpoint.json` supplying the original hands, decisions and choices; source fingerprint `d77f6b36…`. The starter `mine` default reads 01-level only; `discover`/`generate` default to both campaigns.
- **The pupils** ([walt-seat-play](walt-seat-play.md)): `l1-default` (the fixed L1 procedure), `l2-partner-default` (L2 Partner — a best response to a modeled L1 partner, never an equilibrium claim) and `l2-partner-voids`, served by the `partnership` binary through `experiments/partnership/player.py` with the 14 s ceiling and reserve/fallback routes; no fallback occurred in any gym run.
- **Consumers of the 30-exercise exam** (same fixed field; results pinned by those campaigns' own `measurements.json`, not by gym files): policy synthesis (`08fad726`, `policy_gym.py`) — sampled information-state tables at 1/4/16/64 training worlds chose an optimal first action on 16/22/25/**26** of 30, mean first-action regret 9.763/2.015/0.616/0.436 pp, mean complete-policy gap 12.322/3.551/1.299/1.008 pp; relational learning (`c00717d1`, `relational_exam.py`) — sampled 16-world table 25/30 (1.299 pp gap), shared exact-regret actor **21/30** (10.929), table + exact-regret fallback 25/30 (1.457), shared interval-cost actor **0/30** (31.129), table + interval fallback 25/30 (1.751); all native teacher optima matched the maintained keys. Both reports call the exam an outcome-selected diagnostic, not an unbiased sample ([walt-partnership-program](walt-partnership-program.md)).
- **Instruments and architecture**: `walt::gym` (548 lines), `partnership_gym`, `partnership`, `gym.py` (840 lines), `gym_spec.py` (240 lines), `rules.py`, `run_capped.py` — catalogued in [walt-instruments](walt-instruments.md); module placement (`gym` above `solver`, importing `scheme`) in [walt-architecture](walt-architecture.md).

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
