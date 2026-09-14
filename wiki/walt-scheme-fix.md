# Scheme/Fix — the relational expression language

[Home](Home.md) · owns: the Scheme/Fix relational expression language (implemented 2026-09-06 as `walt::scheme` — grammar, semantics, the predicate registry, beliefs and counterexamples, finite dynamics, executable policies, the sampled-table and shared-relational constructors built on it, the information-price examiner, and what the language has been used for) and the archived descriptor/compression research that preceded it (2026-08) · Sources: walt v0.4 §§3–6 (the basis: Scheme and Fix as typed relational queries, answer relations, finite transforms); [`walt/scheme/README.md`](../walt/scheme/README.md), [`DYNAMICS.md`](../walt/scheme/DYNAMICS.md), [`POLICIES.md`](../walt/scheme/POLICIES.md), [`COMPOSITION.md`](../walt/scheme/COMPOSITION.md), [`RELATIONAL.md`](../walt/scheme/RELATIONAL.md), [`INFORMATION-PRICES.md`](../walt/scheme/INFORMATION-PRICES.md), [`VALIDATION.md`](../walt/scheme/VALIDATION.md); the module `walt/walt/src/scheme/{mod,syntax,registry,eval,belief,dynamics,policy}.rs` and `walt/walt/src/policy_search.rs` + `policy_search/{request,program,relational,prices,learning_io,learning_eval}.rs`; the binaries `walt/walt/src/bin/{scheme,policy_lab,relational_lab}.rs`; the gates `walt/walt/tests/{scheme,scheme_dynamics,scheme_policy,policy_search,relational_runtime,relational_learning,information_prices,learning_io}.rs`; the four examples under `walt/scheme/examples/` and the query files under `walt/gym/queries/`; [`experiments/partnership/POLICY-SYNTHESIS.md`](../experiments/partnership/POLICY-SYNTHESIS.md) and [`RELATIONAL-LEARNING.md`](../experiments/partnership/RELATIONAL-LEARNING.md); the campaign records [`campaigns/policy-synthesis-v1/RESULTS.md`](../experiments/partnership/campaigns/policy-synthesis-v1/RESULTS.md) and [`campaigns/relational-learning-v1/RESULTS.md`](../experiments/partnership/campaigns/relational-learning-v1/RESULTS.md) with their `measurements.json` and `verification.json`; the gym guides `walt/gym/{README,DISCOVERY,BID-MAKING,PARTNERSHIP-COMPOSITION}.md`; `walt/LOG.md` entries 2026-09-06 and 2026-09-07; for the archived research, walt v0.4 §12.1, §12.5, §12.6, §12.7, §12.9, §16.11, §17.4, `walt/math/equivariant_lumpability_v0.5.md` (§12.6A), `walt/math/implementers_guide.md` §1.20–1.21, the archived `walt-skeleton`/`walt-factory` sources at producer commit `648f93a`, and `walt/probes/factory-results/`.

> **EXPLORATORY TIER — the whole page.** walt sits on its own frozen exploratory-tier
> basis (`walt/math/`). Nothing here is a corpus status, a Lean kernel proof, an
> exchange-adjudicated CONFIRMED, or a rob conformance receipt. Every number below is
> computed evidence about one declared finite domain, from a single Rust implementation,
> and is never an axiom (TRUST-01). A number is quotable as a result only through the
> gate file or record that pins it; each one below names its pin, or is labelled
> "probe record, not gate-pinned". The fence is on the [walt hub](walt.md).
>
> **Validation caveat, load-bearing for every number on this page:** every check in the
> Scheme slice (66 focused Rust tests, the Python runner tests, the replay audits) was run
> as a *focused gate* under the 2026-09-06/07 session waiver of full walt CI
> (`walt/ci/check.sh` was deliberately not run — `walt/scheme/VALIDATION.md`, both
> campaign `RESULTS.md`). These are test-pinned finite-domain checks, never rob-style
> byte-diffed receipts.

Siblings: [walt hub](walt.md) · [the partnership gym](walt-gym.md) · [the partnership program](walt-partnership-program.md) · [instruments](walt-instruments.md) · [architecture](walt-architecture.md) · [seat play](walt-seat-play.md) · [negative results](walt-negative-results.md) · [factory era](walt-factory-era.md) · [census era](walt-census-era.md) · [math reference](walt-math-reference.md) · [vocabulary](vocabulary.md).

**Repository state as of 2026-09-07 (`c00717d1`).** The language landed in three commits on branch `codex/partnership-launch`: `b764665f` (2026-09-06, the expression runtime), `08fad726` (2026-09-07, dynamics, policies, sampled-table synthesis) and `c00717d1` (2026-09-07, the shared relational learner and the information-price examiner). Live measurements on this page are dated 2026-09-12 and were made with the release binaries built 2026-09-07 11:54 under `walt/target/release/`; an independent re-run on 2026-09-13 with the same binaries reproduced every quoted §2, §5, §6, §7 and §13.1 output line (6 worlds, 1/3, 2 positive worlds, the counterexample world, `{2-2: 1/3}`, 28 registry lines with 4 World-access, the cap refusal at 17,153,136 worlds, the `offer-count-to-partner` refusal, and `seed 910003, attempts 4, worlds 210`).

**How to read this page.** A newcomer to 42 needs §1–§2 (what the language says and one worked example). A mathematician needs §3–§8 (the semantics, the belief operations, the dynamics) and §11 (the one clean mathematical statement in the slice, which is a negative result). An engineer who wants to run it needs §13 and the invocations in §9–§10. The archived descriptor research — the reason the language exists at all — is §16, kept intact because its counterexample discipline is what the new layer inherited.

---

## 1. The redirection and the fence (2026-09-06)

Scheme/Fix was invented in walt v0.4 (§§3–5, §12.7) as a *descriptor* language: a way to compress the hidden state of a hand into something a seat could carry and update. That program is §16 of this page, and it ended in refutations (the public chassis is never lumpable; vocabulary ceilings; a static passenger fails condition 1 structurally).

On 2026-09-06 Jason redirected it. `walt/LOG.md` (entry "SCHEME FOR EXPRESSION", 2026-09-06) records the direction in one line:

> Invented to compress; commissioned here to express.

The commissioned object is a language in which a *situation* becomes sayable — "partner holds a count tile", "a play that leaves my currently winning partner ahead", "a lead into a context where partner holds the live boss" — over the exact finite worlds of a decision state, with exact rational probabilities under a declared belief, and with concrete counterexample worlds when two descriptions disagree. `walt/scheme/README.md` opens with the same fence: "Its purpose is expression: queries, exercise families, belief events, and counterexamples. It makes no general compression claim and is not a new player or a set of move-scoring bonuses."

What is *not* claimed anywhere in the slice, stated once here and not repeated: a general compression theorem, a compact descriptor step compiler, a §12.6/§12.6A lumpability result for the expression layer, player strength, optimal partnership play, opening-scale performance, or factor-belief contraction (`README.md` "Validation and boundaries"; `DYNAMICS.md` last paragraph; `POLICIES.md` status line). §14 tabulates the delivered/undelivered boundary against v0.4 §12.7.

---

## 2. The language in one example

Plain statement first. At a late-game coordinate the seat can see its own two tiles and the public record; three tiles are still hidden, one in each other hand. The seat asks: *which count tiles does my partner hold?* The language answers with a **set** of tiles per hidden world and, under the uniform belief over the six compatible worlds, an exact probability for the event and for each candidate tile.

The query, verbatim from `walt/scheme/examples/partner-count.scheme`:

```scheme
; Return count tiles held by partner. Partner is an existential chair role;
; the output is a SET of tiles, not one chosen tile and not an answer lottery.
(fix
  (roles (chair me) (chair partner) (domino count-tile))
  (out count-tile)
  (case
    (viewer me)
    (partner me partner)
    (holds partner count-tile)
    (count count-tile 5))
  (case
    (viewer me)
    (partner me partner)
    (holds partner count-tile)
    (count count-tile 10)))
```

Run it against the real receipt coordinate (from the repository root; the default coordinate is hand 0, trick 6, seat 0 of `rob/receipts/verify_player.txt`):

```sh
walt/target/release/scheme --query walt/scheme/examples/partner-count.scheme --hand 0 --trick 6 --seat 0
```

The report (abridged to the load-bearing lines; **measured 2026-09-12 on this machine**, median wall 2.4 ms over ten runs, min 2.1 ms, max 3.0 ms):

```
coordinate: decl=P3 viewer=S0 hand={2-0 4-2} pool={2-1 2-2 4-1}
            hidden=[S1 cap 1 voids {}, S2 cap 1 voids {q0 q*}, S3 cap 1 voids {q0 q*}]
            leader=S1 prefix=[0-0, 5-2, 4-4]
support-worlds: 6
belief: uniform-full-support-v1
event-probability: 1/3
answer-presence: [count-tile=4-1] probability=1/3
work: 1084
```

Reading it: three pool tiles into three one-tile hands gives 3! = 6 compatible worlds (the recorded voids exclude none of them). The only live count tile is 4-1; the partner (S2, two seats clockwise from the viewer) holds it in 2 of the 6 worlds, so the event "partner holds a count tile" has probability exactly 1/3, and the only possible answer tuple is `4-1` with presence 1/3. The program does *not* use the receipt's actual hidden deal as its belief — the receipt hand/trick identifiers are provenance only.

Now condition on the Boolean event (`partner-has-count.scheme` is the same Fix with `(out)`):

```sh
walt/target/release/scheme --query walt/scheme/examples/partner-count.scheme \
  --condition walt/scheme/examples/partner-has-count.scheme --hand 0 --trick 6 --seat 0
```

```
positive-worlds: 2
conditioning-event-probability: 1/3
event-probability: 1
certainty: event_certain: true, constant_multiplicity: Some(1), world_functional: true, identity: Some(4-1)
work: 1396
```

Two worlds carry positive mass; the answer is now certain and its identity is known. This is an **analyst's intervention** — the seat did not observe it, and the program never claims it did (§7).

Pin: these six-world / 1/3 / two-world numbers are pinned by the integration test `receipt_cli_queries_conditions_and_refuses_without_partial_reports` (`walt/walt/tests/scheme.rs` line 531) and recorded in `walt/scheme/VALIDATION.md`. The live re-run above reproduces them.

---

## 3. Grammar

From `walt/scheme/README.md` (the source of truth for syntax):

```text
fix      := (fix (roles (sort name) ...) (out name ...) case ...)
sort     := chair | domino | context
case     := (case clause ...)
clause   := (same name name ...) | atom | (not atom)
atom     := (registered-predicate argument ...)
argument := role-name | literal
```

- Names begin with an ASCII letter and continue with letters, digits, `_` or `-`; literal spellings cannot be role names.
- Literals: dominoes such as `6-4` (endpoints normalize, so `4-6` is the same tile), chairs `S0`–`S3`, contexts `q0`–`q6` and `q*` (the called/trump context), teams `T0`/`T1`, nonnegative integers. Semicolon comments run to end of line.
- Limits (source facts, `walt/walt/src/scheme/syntax.rs` lines 162 and 211, `eval.rs` line 147): source at most 1 MiB (1,048,576 bytes), nesting depth at most 32, at most 64 roles at compilation.
- Rejected before evaluation: invalid names, unknown predicates, wrong argument sorts, duplicate declarations or outputs, mixed-sort equality classes (test `syntax_and_type_errors_are_refused_before_evaluation`).
- All roles range over the full finite sorts: 28 dominoes, four chairs, eight contexts. Add `(live tile)` when a domino role must be live; this is what lets `played` and constant tile references keep their meaning.

Programmatic construction uses the public types `Fix`, `Scheme`, `Role`, `Atom`, `Term`; `source.parse::<Fix>()` parses text; `Fix::compile(&Registry)` validates and captures immutable predicate handles. `CompiledFix::identity()` includes the versioned syntax and every used predicate specification — a complete *textual* identity, not a canonical form under logical equivalence.

---

## 4. The five semantic rules

These are the rules a reader must internalize before writing a query; each is pinned by a named test in `walt/walt/tests/scheme.rs`.

1. **Distinct unless `same`.** Two role names of the same sort denote *different* objects within a case unless a `(same …)` group identifies them; groups are transitively closed; different sorts never compete for an identity. Binding is injective *after* quotienting by the equality pattern. To cover "a and b may or may not be the same tile" write two cases, one with `(same a b)` and one without. (Pin: `equality_patterns_are_complete_and_quotient_before_injective_binding`.)

2. **Roles omitted from `out` are existential.** Multiple witnesses for the hidden roles yield one output tuple. `(out)` asks a Boolean existence question; a Fix with no cases is false; an empty case is true whenever its equality pattern has an interpretation. (Pin: `empty_fix_false_empty_case_true_and_hidden_witnesses_are_existential`.)

3. **A Fix returns a SET.** Cases combine by set union; overlapping cases and internal witnesses never duplicate answers or probability mass. "Partner holds a count tile" contributes a world's mass once even if partner holds two. (Pin: `overlapping_branches_and_internal_witnesses_do_not_multiply_mass`.)

4. **Undefined is neither true nor false.** `not` is finite-world negation of a registered atom, and a predicate may return *undefined* when its precondition fails; then neither the atom nor its negation holds. With no lead yet, neither `(led-context q*)` nor `(not (led-context q*))` is satisfied — negation cannot manufacture a fact out of missing context. (Pin: `undefined_is_not_false_and_never_satisfies_negation`, `scheme.rs` line 425.)

5. **Presence masses are not a distribution, and every selector is an explicit convention.** If a world has two answers, both presence events include that world, so per-answer presence probabilities need not sum to one. Picking *one* answer per world requires `summary.select(...)` with an explicit `LexicographicFirst` or `UniformWithinWorld` law, which returns a separate probability law including the no-answer probability and preserves each world's mass. Neither convention is implied by Scheme, and an offline selector is not automatically an information-local policy. (Pins: `answer_multiplicity_is_not_a_probability_distribution`; the live `--selector uniform` run of `partner-master.scheme` on 2026-09-12 gave `{2-2: 1/3}`, no-answer probability 2/3, work 520.)

A sixth rule sits underneath all five: evaluation is **exact on a declared finite measure**, and a computation either returns a complete value or refuses. There is no sampled fallback and no partial report (§6).

---

## 5. What a query can see: the Viewer/World access split

Every predicate is registered with a `PredicateSpec`: name, semantic version, typed parameters, horizon in plies, and `Access::Viewer` or `Access::World`. Viewer predicates receive only the `Frame` (the focal private hand plus the public kernel and residue — never another hand); World predicates additionally receive the concrete world assignment. The compiler checks argument sorts; the runtime enforces the access split by what it passes to the callback (test `extension_registry_freezes_semantics_and_enforces_information_access`).

The standard registry (`Registry::standard()`, `walt/walt/src/scheme/registry.rs`) has **28 predicates, all version `scheme-v1/straight-v0.4`, all horizon 0**; exactly four need World access (`holds`, `void`, `legal`, `forced`). Verified live with `scheme --registry` on 2026-09-12 (28 lines). The table adds the undefined/false cases that the README's table leaves to prose (source: `registry.rs` lines 185–221; `Frame::next_actor` in `eval.rs` line 100).

| Predicate | Arguments | Access | Meaning | Undefined / false cases |
|---|---|---|---|---|
| `live(d)` | Domino | Viewer | `d` in the kernel's live set | — |
| `played(d)` | Domino | Viewer | `d` in the explicitly supplied played history | non-live does not imply played |
| `holds(c,d)` | Chair, Domino | **World** | `c`'s remaining hand contains `d` in the concrete world | — |
| `void(c,q)` | Chair, Context | **World** | `c` has no effective follower in `q` in the concrete world | — |
| `in(d,q)` | Domino, Context | Viewer | declaration-relative incidence (`d` follows `q`) | — |
| `double(d)` | Domino | Viewer | `d` is a double | — |
| `beats(a,b,q)` | Domino, Domino, Context | Viewer | strict trick-key comparison in `q` (all nine declarations via the rules module) | — |
| `boss(d,q)` | Domino, Context | Viewer | live effective follower in `q` with no live tile of higher trick key in `q` | — |
| `master(d)` | Domino | Viewer | live tile with no live beater when led (`Kernel::masters`) | — |
| `quota(c,n)` | Chair, Number | Viewer | public remaining hand size | — |
| `count(d,n)` | Domino, Number | Viewer | exact count decoration 0, 5 or 10 | — |
| `tile(a,b)`, `chair(a,b)`, `context(a,b)` | same sort ×2 | Viewer | same-sort equality, usually anchoring a role to a literal | does not alter a case's equality pattern |
| `team(c,t)` | Chair, Team | Viewer | partnership membership | — |
| `partner(a,b)`, `opponent(a,b)`, `successor(a,b)` | Chair, Chair | Viewer | partner is two seats clockwise; successor is the next clockwise | — |
| `viewer(c)`, `leader(c)` | Chair | Viewer | the focal seat; the current trick leader | — |
| `next-actor(c)` | Chair | Viewer | the seat to act next | **false for every chair** (not undefined) when the seat to act has no remaining tile (`Frame::next_actor` is `None`) |
| `led-context(q)` | Context | Viewer | the context led in the current partial trick | **undefined** at an empty trick |
| `current-winner(c)` | Chair | Viewer | the seat currently winning the partial trick | **undefined** at an empty trick |
| `legal(c,d)`, `forced(c,d)` | Chair, Domino | **World** | legal play / unique legal play of `c` in the concrete world | **undefined** unless `c` is next to act |
| `own-legal(d)` | Domino | Viewer | legal action from the viewer's own hand | **undefined** unless the viewer is next to act |
| `trick-play(c,d)` | Chair, Domino | Viewer | `c` played `d` in the current partial trick | false when absent |
| `leads-context(d,q)` | Domino, Context | Viewer | the context `d` *leads* under the declaration (distinct from merely following `q`) | — |

The last three rows (`own-legal`, `trick-play`, `leads-context`) are the "three general public atoms" added on 2026-09-07 (`b0c7c0aa`) so that ordinary query files could drive gym discovery without a bespoke predicate.

**Adding a relation.** Implement `Predicate`, register once under a unique name, declare the spec. Evaluation returns `Some(true)`, `Some(false)` or `None` (undefined). An extension must be deterministic, obey its declared horizon, charge its inner work to the supplied `Budget`, and must not call the target solver, inspect response labels, or capture undeclared information. Rust cannot audit an arbitrary callback's captures or prove a horizon declaration — those obligations are the **trusted extension contract**, and the typed `horizon_plies`/`access` fields make the declaration machine-readable without making it machine-verified.

**The one extension in the repository.** The gym registers `offer-count-to-partner` (version `gym-v1`, one Domino argument, horizon 1 ply, `Access::Viewer`) in `walt/walt/src/gym.rs` `registry()`. It is not in the standard registry: on 2026-09-12 the plain `scheme` binary refused `walt/gym/offer-count.scheme` with `scheme: unregistered predicate offer-count-to-partner` (exit 1). The later gym query files (`walt/gym/queries/*.scheme`) use only standard predicates.

---

## 6. Frame, Belief, Budget — and the refusal discipline

Three objects, kept distinct by type:

- **`Frame::new(kernel, leader, prefix, played)`** is the public situation: the kernel (viewer hand, hidden pool, per-seat capacities and observed voids), the current leader, the current trick prefix, and the explicitly supplied played set. Construction validates public-residue coherence (test `frames_reject_incoherent_history_and_do_not_invent_played_tiles`); it does *not* assert reachability from a full deal. A Frame is *support*, not belief.
- **`Belief`** is an explicit finite rational measure over compatible worlds. `Belief::uniform(frame, cap)` materializes uniform **full legal support** — the exact world count is checked against the cap *before* enumeration (on 2026-09-12 the all-legal query at hand 0, trick 1, seat 0 with `--max-worlds 100` was refused before materializing: `support has 17153136 worlds, above cap 100`). `Belief::from_weights(frame, id, entries, cap)` accepts exact nonnegative rational weights; repeated physical worlds add, zero weights disappear; empty or zero-mass measures, negative weights and foreign worlds are errors. The supplied id names *provenance*; the frame and weights are the actual measure, and an empirical measure stays empirical. Stored weights may be unnormalized; probability accessors divide by total mass.
- **`Budget`** is an integer work counter charged for world checks, bindings and predicate calls (custom predicates must charge their inner work). It is not a wall-time guarantee — use the process watchdog (§13) when one is needed.

**Refusal.** Exceeding the world cap or the work budget is an error with no sampled fallback and no partial report; the CLI buffers the whole report so a refusal prints nothing (test `work_exhaustion_refuses_a_partial_query_or_summary`; the CLI pin). The same rule governs every later layer: a refused dynamics step returns no partial belief, a refused donor traversal publishes no partial pool, a refused price computation returns no bound.

`query.summarize(&belief, &mut budget)` returns the exact event probability, per-answer presence masses, the distribution of entire answer sets, and separately named **certainty** fields — possibility, event certainty, constant answer multiplicity, constant answer set, one answer per world (`world_functional`), and one known identity. They concern positive-mass worlds in the *named* measure: "partner has exactly one count tile in every world" does not say which tile it is (test `certainty_does_not_confuse_existence_with_identity`).

---

## 7. Belief conditioning and counterexamples

**Support is not belief.** The uniform full-support belief and a supplied weighted belief are both explicit measures; the certainty of an event under one is not its certainty under the other. `belief.condition(&query, posterior_id, &mut budget)` conditions on answer existence; `condition_likelihood` accepts explicit rational likelihoods in [0,1]. Both return the posterior *and* the evidence probability, leave the original unchanged, and refuse an impossible event (test `conditioning_uses_events_and_exact_likelihoods_transactionally`; zero-weight worlds never affect certainty, test `zero_weight_worlds_do_not_affect_belief_certainty`). Every conditioning is an **analyst intervention**: it never claims the seat observed the event. The seat's own information is the Frame; what the seat may *do* with a query is the policy layer's question (§9), and the policy layer forbids World predicates outright.

**Counterexamples are concrete worlds.** `query.compare(&other, &belief, Comparison, &mut budget)` checks Boolean existence (`--comparison existence`) or the whole answer relation (`--comparison answers`, which requires identical ordered output interfaces). A failure returns the *first concrete world* and both answer sets for replay; a pass is confined to the measure's support. Answer equivalence is strictly stronger than Boolean equivalence (test `answer_equivalence_is_stronger_than_boolean_equivalence`).

Live counterexample, 2026-09-12, at the §2 coordinate:

```sh
walt/target/release/scheme --query walt/scheme/examples/partner-count.scheme \
  --compare walt/scheme/examples/partner-master.scheme --comparison existence --hand 0 --trick 6 --seat 0
```

```
comparison: Existence; counterexample = world hands [{2-0 4-2}, {2-1}, {2-2}, {4-1}]
            left: {}   right: {2-2}
work: 1351
```

In the world S1 = {2-1}, S2 = {2-2}, S3 = {4-1}, partner (S2) holds no count tile but does hold a master (2-2), so "partner has a count tile" and "partner has an entry candidate" are different events, and the runtime says exactly where. Both queries have event probability 1/3 at this coordinate — equal probabilities, different events — which is why the comparison exists.

---

## 8. One play at a time — finite dynamics

Source: `walt/scheme/DYNAMICS.md`; module `dynamics.rs`; gate `walt/walt/tests/scheme_dynamics.rs` (10 tests). Everything here **executes v0.4 §6 extensionally on full finite states**; nothing compiles a Scheme `step`, compresses support, or claims a compact dynamic representation.

An `ObservedPlay` names the actor and the physical domino; a `PlayClass` says whether it is a lead, a follow in a named context, or a slough from a named context. A mismatch of actor, context, classification, holder or follow-legality is rejected (tests `typed_play_rejects_wrong_actor_class_and_illegal_world`, `frame_step_rejects_hidden_tile_forbidden_by_an_existing_void`).

- **`step_frame`** rebuilds the inherited kernel and public residue: a hidden play forces the holder, removes the tile, decrements that chair's capacity and, for a slough, records the observed void before the exact matching fiber is recomputed; a viewer play removes the tile from the known hand; the fourth play closes the trick through the rules engine and installs the winner as next leader.
- **`step_world`** is the corresponding partial deterministic map on a concrete world; `play_is_legal` recognizes its domain.
- **`step_belief`** performs the finite update **in this order**: (1) discard worlds outside the typed legal domain; (2) multiply each remaining weight by the caller's exact likelihood in [0,1]; (3) push each physical world through `step_world`; (4) merge equal successors and report the observation's prior probability. `unit_likelihood` is right for a forced action and for a viewer intervention randomized independently of the hidden deal (test `viewer_intervention_preserves_hidden_deal_odds`); an ordinary hidden choice needs a declared policy likelihood (test `explicit_likelihood_precedes_pushforward_and_normalization`).

**Marginal-only caveat.** A `Belief` is only a marginal over physical remaining hands. Two states with the same hands but different policy memory, private observations or other field state must stay distinct in a caller-owned augmented state until after likelihood and latent transition; a likelihood applied to the already-merged physical marginal cannot recover that information.

**Rigid transport.** Output bindings are rigid identities. `transport_answers` carries domino, chair and context values unchanged through a play without asserting that they still satisfy any predicate; `compare_answers` compares transported prior answers with a fresh successor evaluation and returns the **persistent**, **extinct** and **born** sets separately (test `rigid_identity_separates_persistence_extinction_and_birth`). Fresh roles are rebound on every evaluation; rigid roles are the explicit alternative, never an implicit selection of a hidden referent.

**`anchor_back`** computes a finite predecessor preimage against explicit later `(world, answer)` pairs. It is analyst hindsight filtering and adds no later information to the player's earlier policy (test `hindsight_anchor_is_a_preimage_not_revelation`).

All transforms return a complete value or an error; a refusal returns no partial frame, belief, comparison or anchor (test `refusal_returns_no_partial_belief`).

---

## 9. Executable policies

Source: `walt/scheme/POLICIES.md`; module `policy.rs`; gates `scheme_policy.rs` (6) and `relational_runtime.rs` (6). Status line from the source: "A policy is an information-measurable controller, not a claim that Scheme/Fix compresses optimal play."

**`PolicyProgram`** is a plain-text, roundtrip-stable artifact (parse then display gives canonical text; test `text_roundtrip_preserves_exact_information_key`) with three action layers, tried in order:

1. **Exact information-state rules.** An exact key contains the declaration, viewer, remaining viewer hand, the public kernel support (hidden pool, seat capacities, public void constraints), leader, current prefix, played set, the full actor-attributed public history, banked scores, bid and declaring team — and no hidden-world identifier or realized opponent hand. Keyed on the *full* public history so two public predecessors sharing a residue are never confused. A stored illegal action is rejected at compile time (test `exact_rule_precedes_relation_and_is_checked_for_legality`).
2. **Ordered relational rules.** Guards are ordinary Fix expressions; compilation **rejects any guard or binding query that mentions an `Access::World` predicate**, and evaluation goes through `CompiledFix::evaluate_viewer` (`eval.rs` line 336), whose call surface accepts no `World` (test `world_predicates_are_refused_and_fallback_is_total`). A guard with `(select answer)` must return one domino column; the policy plays the least legal returned domino. Literal and rigid selectors need a Boolean (zero-column) guard. A rule whose proposed action is currently illegal is "no match", and the next rule or the fallback acts.
3. **Mandatory `lowest-legal` fallback.** Every returned action is checked against the straight-42 legality function.

**Rigid bindings and modes.** A `(bind name (fix …))` runs once when a controller is initialized and must yield exactly one value from the initial viewer frame; rules may later `(select rigid name)`. A controller also stores a named **mode**; a successful relational rule moves it from its `(in …)` to its `(next …)`, exact-table actions leave it unchanged. This is the explicit distinction between solving each observation fresh and carrying a fixed choice or mode across observations. Mode changes happen only after guard evaluation and legal selection both succeed, so exhaustion or an illegal proposal cannot partially advance a controller (test `controller_provenance_and_mode_updates_are_transactional`). The example from `POLICIES.md`:

```scheme
(policy keep-opening-master
  (initial fresh)
  (fallback lowest-legal)
  (bind opening
    (fix (roles (domino d)) (out d) (case (tile d 6-6))))
  (rule reuse
    (in fresh) (next persistent)
    (select rigid opening)
    (guard (fix (roles) (out) (case))))
  (rule continue
    (in persistent) (next persistent)
    (select answer)
    (guard (fix (roles (domino action)) (out action)
                (case (own-legal action) (master action))))))
```

**Inputs are validated, not assembled.** `PolicyInput::new` checks that the actor-attributed history is a coherent full public predecessor (turn and trick order, viewer legality, observed voids, remaining capacities, score, leader and prefix, live/played partition) and that the residual kernel has nonempty support; `PolicyKey::from_input` builds the exact key. These are reference-path audits that scan the history and count support — experimental timing must separate search from input validation and artifact replay.

**Traced provenance.** `CompiledPolicy::choose_traced` returns the action plus its provenance (exact key / named rule / final fallback), controller modes before and after, and public contract resolution; `choose` is the projection. `replay_program_traced` retains the complete physical play sequence and each focal decision's depth, provenance, work and controller snapshot; post-resolution moves are kept and can be excluded from unresolved-policy diagnostics (tests `choose_trace_distinguishes_exact_relation_and_final_fallback`, `independent_replay_reports_depth_provenance_work_and_controller_state`).

**Sampled tables export into this language.** `policy_search::program::export` converts a sampled `TablePolicy` into exact rules by replaying each table history through the typed public-frame transform, then verifies a display/parse roundtrip (test `sampled_table_exports_and_replays_through_serialized_program`). A 200-sample opening program is roughly 880 stored decisions in 0.34 MB of text (`policy-synthesis-v1/RESULTS.md`).

**The hybrid combinator.** `combine_exact_table_with_relational` keeps a frozen exact table's keys and actions and places a *stateless* shared relational program above the final fallback; it refuses bindings, mode transitions or exact patches in the shared source (test `hybrid_helper_refuses_stateful_or_contaminated_sources`). Full-policy replay then measures the actual substitution, not outcomes conditioned on lookup misses.

**A learned actor, verbatim.** The frozen gym-field exact-regret actor from `experiments/partnership/campaigns/relational-learning-v1/policies/gym-exact.policy` (829 bytes; sha256 `149ee3c8…` in `measurements.json`): one mode, no bindings, no exact keys, three clauses, then the fallback.

```scheme
(policy shared-relational
  (initial shared)
  (fallback lowest-legal)
  (rule r0-partner-winning-count-10 (in shared) (next shared) (select answer) (guard (fix
  (roles (domino action) (chair viewer) (chair winner))
  (out action)
  (case
    (own-legal action)
    (count action 10)
    (viewer viewer)
    (current-winner winner)
    (partner viewer winner))
)))
  (rule r1-partner-winning-count-0 (in shared) (next shared) (select answer) (guard (fix
  (roles (domino action) (chair viewer) (chair winner))
  (out action)
  (case
    (own-legal action)
    (count action 0)
    (viewer viewer)
    (current-winner winner)
    (partner viewer winner))
)))
  (rule r2-master (in shared) (next shared) (select answer) (guard (fix
  (roles (domino action))
  (out action)
  (case
    (own-legal action)
    (master action))
)))
)
```

Read as prose: if partner is currently winning the trick, play a ten-count tile; else if partner is currently winning, play a zero-count tile; else play a master; else the lowest legal tile. The priced and unpriced interval-cost actors (`gym-priced.policy`, `gym-unpriced.policy`, byte-identical at 707 bytes) order the clauses partner-winning count-0, master, count-10. The three L0-8 actors are each the 72-byte empty program (fallback only). How well these play is §10.3: deployable, and weak.

---

## 10. What the language has been used for

> **Banner for every number in this section: exploratory, field-relative, endgame-only.**
> Every value is relative to a declared belief and a frozen field (the teammate and opponent
> models named in each record); the coordinates are tricks 5–6 or three-domino continuation
> roots; bid is 30 throughout; the objective is P(make) with make/set only (the pmake ruling of
> 2026-08-17). Nothing here is a strength result for the live player.

### 10.1 The partnership gym (2026-09-06 → 09-07) — owned by [walt-gym](walt-gym.md)

Scheme was the *matching authority* at each rung of the gym ladder; the exact make-30 answer keys came from the independent lawful evaluator, never from a Scheme predicate. Numbers are the gym's, cited here only as the language's first uses:

| Rung | Scheme's role | Yield | Record |
|---|---|---|---|
| Six count-offer starters (`c59f1115`) | the `offer-count-to-partner` extension predicate, presence checked to be 0 or 1 across the full belief | L1 5/6 optimal (mean regret 1/216), L2 Partner 6/6, L2 Partner with voids 5/6 | `walt/gym/RESULTS.md`; `walt/LOG.md` 2026-09-06 |
| Query-driven discovery (`b0c7c0aa`) | three ordinary query files (`offer-count`, `overtake-partner`, `lead-to-partner-boss`) over a generic stream of 1,929 saved late-game coordinates, 245 above-cap skipped per query | 176 strict memberships = **170 distinct coordinates**; about 13 s on ten workers | `walt/gym/DISCOVERY.md` lines 80–95; `walt/gym/collections/scheme-v1/` |
| Outcome-only bid-making (`67f1e4ab`) | the four-line `all-legal.scheme` — `(fix (roles (domino action)) (out action) (case (own-legal action)))` — with selection by strict make-probability difference | **433** declaring coordinates from 837 graded (367 unique best play, 26 certain make vs certain set); sweep 50.616 s, re-audit 16.414 s | `walt/gym/BID-MAKING.md`; `walt/gym/collections/bid-making-v1/` |
| Composed exam (`1df741db`) | the count-offer Fix under query-required selection: the best matching action must strictly beat the best non-matching action | **30** positions from 21 deal seeds; L1 24/30 (mean regret 1643/205200 = 0.8007 pp), L2 Partner 26/30 (959/205200 = 0.4673 pp); mean regret reduction 1/300 | `walt/gym/PARTNERSHIP-COMPOSITION.md`; `walt/gym/benchmarks/partnership-bid-making-v1/` |

"L2 Partner" is a best response to a *named* field (partner modeled at L1, opponents at L0 — [walt-seat-play](walt-seat-play.md)), never an equilibrium. Hidden-holding queries such as `lead-to-partner-boss` report exact full-belief presence without narrowing the grading belief. Composition and grading happen at the gym-specification layer (`walt/gym/SPECIFICATIONS.md`: a specification packages an embedded Scheme query with its source, coordinate, evaluator and selection arguments as a repeatable exercise family, with position-set and answer-key fingerprints that validate a reproduction after solving without becoming inputs to discovery or play); the language gained no solver dependency and no value predicate.

### 10.2 Persistent policy synthesis (`08fad726`, 2026-09-07)

Question (`experiments/partnership/POLICY-SYNTHESIS.md`): can a constructor retain or compose work as its world sample grows while matching the policy obtained by solving the accumulated sample afresh? Three arms on identical nested sample streams under a frozen field: `fresh` (empty cache), `persistent` (retains completed exact subproblems; must equal fresh), `compose` (union of every successful singleton-donor action at each information state, then a restricted joint search — `walt/scheme/COMPOSITION.md` states, as a walt-internal argument pinned by the donor-completeness gate in `policy_search.rs`, that a *complete* donor pool preserves the training optimum and that the guarantee does not extend to a partial pool). Record: `campaigns/policy-synthesis-v1/RESULTS.md` + `measurements.json` (schema `policy-synthesis-evidence-v1`).

| Panel | Roots | Schedule | Test worlds/root | Campaign wall |
|---|---:|---|---:|---:|
| cheap `hash-legal-v1` field (opening, seven tiles, sixes, bid 30) | 32 | 1…200 samples (nine stages) | 256 | 17.189 s |
| native L0-8 field | 12 | 1…32 (six stages) | 64 | 64.590 s |

- **Parity.** Across all 287 hash-field and 72 L0 three-arm comparisons, serialized policy identities and training/test outcome vectors agree exactly. One opening root hit the 2,000,000-node ceiling at 200 samples in `fresh` and `compose`; persistence completed it; that root is excluded from the timing pair (`measurements.json` `panels/opening`: `root_units` 32, `fully_completed_root_units` 31; `panels/l0`: 12/12).
- **Persistence saves.** Complete paired schedules: hash field, 31 roots, 20.531 s fresh vs 9.664 s persistent = **52.9 %**; L0-8, 12 roots, 114.369 s vs 61.298 s = **46.4 %**. Node savings 56.0 % and 49.4 %. Final-stage means: 302 ms fresh vs 119 ms persistent at 200 samples (hash); 4.744 s vs 2.557 s at 32 samples (L0-8). These are construction times at these roots, not L1/L2 wrapper timings.
- **Composition costs more.** 21.831 s (6.3 % more than fresh) and 129.513 s (13.2 % more), for byte-identical policies: it "produces the same program at greater overall cost".
- **Overfitting is the finding.** 200-sample opening programs made 4,979/6,400 (77.8 %) of *training* worlds and 2,100/8,192 (25.6 %) of independent test draws; L0-8 335/384 (87.2 %) vs 182/768 (23.7 %). Explicitly not calibrated estimates of the optimal lawful make probability. The constructor emits per-root exact tables and infers **no relational rules**.
- **On the 30 composed gym exercises** (360 rows, 2.118 s; arm timing is not a speed claim because the gym field cache is shared), exact fractions from `measurements.json` `panels/gym/stages`:

| Training worlds | Optimal first actions | Mean first-action regret | Mean complete-policy gap |
|---:|---:|---:|---:|
| 1 | 16/30 | 15566189/159440400 = 9.763 pp | 12278369/99650250 = 12.322 pp |
| 4 | 22/30 | 434107/21546000 = 2.015 pp | 382519/10773000 = 3.551 pp |
| 16 | 25/30 | 421/68400 = 0.616 pp | 691/53200 = 1.299 pp |
| 64 | **26/30** | 1649/378000 = **0.436 pp** | 3809/378000 = **1.008 pp** |

First-action regret credits the chosen move with its best lawful continuation; the complete-policy gap grades the actual saved continuation including its fallback — two different scores.

*Correction in place (2026-09-12).* An earlier version of this page said "across 44 random opening hands … saved 46–53 % of paired schedule search time". The timing comparison is over 31 complete hash-field roots and 12 L0-8 roots (43), the two savings are 52.9 % and 46.4 % on *different fields*, not a range over one population, and one of the 32 hash roots was refused at 200 samples and excluded from the pair.

### 10.3 Shared relational learning (`c00717d1`, 2026-09-07)

Design input: the Astra proposal packet, preserved unchanged in `14e01322` (`experiments/partnership/packet/texas42_relational_learning/`), whose own verifier passed 8,192 generic exact-rational policy checks — generic finite mathematics, explicitly not a Texas 42 result. Its `partner-count-candidate.scheme` is pinned as lawful current syntax by `relational_runtime.rs` line 288.

The constructor (`walt/scheme/RELATIONAL.md`, `policy_search/relational.rs`, grammar id `scheme-relational-actor-v1/grammar-v1/straight-v0.4`): a fixed library of **14 clauses** — seven selector relations (`legal`; `count-0`, `count-5`, `count-10`; `master`; `follow-led`; `boss-led`) crossed with two qualifications (any; partner currently winning via `viewer`/`current-winner`/`partner`) — every clause including `own-legal(action)`; a deterministic cost-sensitive beam over insertions, deletions and reorderings, ranked by exact weighted cost then clause count, AST nodes and canonical order (an empirical tie always favors the simpler program; test `tied_action_costs_do_not_force_a_relational_split`); the empty lowest-legal baseline always retained. Guards are Viewer-only with no tile literal (test `frozen_grammar_is_current_viewer_only_and_contains_no_physical_tile_literal`). Default `RelationalLimits`: 4 clauses, 96 AST nodes, beam 32, 20,000 search work, 100,000 inference work per example; the campaign ran 3 clauses, 256 AST nodes, beam 6.

Frozen design (`campaigns/relational-learning-v1/RESULTS.md`; `gym-run.json`): two panels (maintained gym field: L1 partner at 40 worlds / L0 opponents; native L0-8), each **144 source-deal groups** = 32 train-a + 32 train-b (on-policy discovery) + 16 development + 64 untouched final test; the focal seat S0 holds **three dominoes**, is next to act with at least two legal plays, bid 30 unresolved, support **cap 512 worlds**, sixes declared; no starting hand crosses a split; digests pinned before test values were computed. Every evaluated root used its whole compatible fiber — 11,836 worlds across the 64 gym-field test roots, 8,892 in L0-8. Whole-pipeline wall 39.411 s (gym field, 10 workers) and 11.352 s (L0-8, 8 workers).

Three cost arms: exact optimal-action regret `max_b Q*(I,b) − Q*(I,a)`; an interval cost from an unpriced perfect-information upper and the actor's own lawful continuation lower; the same with a priced upper (§11). Priced and unpriced produced identical programs.

**Held-out full-policy results** — equal-root mean make probabilities, exact fractions from `measurements.json` `panels/*/summary/arms` (verified 2026-09-12 to reproduce every percentage in `RESULTS.md`); the optimum row is from `RESULTS.md`:

| Frozen actor / composition | Gym field | L0-8 |
|---|---:|---:|
| Sampled exact table (16 worlds/root), lowest-legal fallback | 90245471/195350400 = **46.197 %** | 19645261859/38887296000 = **50.518 %** |
| Shared actor, exact-regret cost | 6364625549/15628032000 = 40.726 % | 13261856039/34998566400 = 37.893 % |
| Same table + exact-regret shared fallback | 7340057381/15628032000 = **46.967 %** | 50.518 % (identical) |
| Shared actor, interval cost (priced = unpriced) | 756669241/1736448000 = 43.576 % | 37.893 % (empty) |
| Same table + interval-trained fallback | 267575543/578816000 = 46.228 % | 50.518 % (identical) |
| Exact lawful root optimum under that frozen field | 49.503 % | 52.427 % |

- **Actors trail tables.** Gym field: the exact-regret actor trails the table by 5.47 pp, paired root bootstrap 95 % interval [−9.58, −1.76] (`measurements.json` `[-0.0957686, -0.0176218]`; 8 roots improved, 23 harmed); the interval actor trails by 2.62 pp, [−4.52, −0.91]. L0-8: development rejected every learned clause and selected the **empty program** for all three arms, trailing the table by 12.63 pp.
- **The one positive signal, and its interval.** Table + exact-regret fallback gained **+0.771 pp**, 95 % interval **[−0.122, +1.909]** (`[-0.0012163, 0.0190909]`; 11 roots improved, 9 harmed, the rest tied). The report's own words: "an uncertain positive signal, not an established strength improvement." The gain comes from changed continuations after a table miss, not better root actions: first-action regret is preserved (0.693 pp) while the whole-policy gap falls 3.306 → 2.536 pp and the probability of reaching the final fallback before resolution falls 45.50 % → 24.83 % — a coverage diagnostic, not the effect measurement. The L0-8 hybrids equal the table exactly (bootstrap [0, 0]) because the empty fallback *is* the table's fallback.
- **Post-freeze exam on the 30 composed gym exercises** (outcome-selected, 21 source deals, 2.181 s; `exam-run.json`): table 25/30 optimal first actions with 1.299 pp mean whole-policy gap; shared exact-regret actor 21/30, 10.929 pp; table + exact fallback 25/30, 1.457 pp; shared interval-cost actor **0/30**, 31.129 pp; table + interval fallback 25/30, 1.751 pp. "The learned rules are therefore not a partnership solution. Their weak transfer is useful counterexample material."

The bootstrap intervals are Python-side descriptive floats in `measurements.json`; every policy value, regret and gap is an exact rational (`BigRational`) computed natively.

---

## 11. Information prices — what they can and cannot do

This is the slice's cleanest negative result, and it is presented as such. Source: `walt/scheme/INFORMATION-PRICES.md`; module `policy_search/prices.rs` (`EVENT_BASIS_ID = "scheme-information-events-v1"`); gate `walt/walt/tests/information_prices.rs` (8 tests) and `relational_learning.rs`.

**The idea.** A shared relational actor is trained on per-action costs `max_b U(I,b) − L(I,a)` where `U` is an upper on the best lawful continuation and `L` a lower from an actually executed lawful policy. A hidden-information *price* — a charge that is centered so its expectation is zero under every lawful (information-consistent) policy but not under a clairvoyant one — can tighten `U`: relax the future viewer decisions to per-world clairvoyance, charge the price, and weak duality gives an upper on the lawful value.

**The four events**, fixed by the basis version (`event_scheme_source` returns the actual Fix for each; the inner loop uses a native specialization checked extensionally equal on concrete worlds by test `native_event_basis_is_extensionally_equal_to_its_scheme_fixes`):

1. the viewer's partner holds a five-count tile;
2. the partner holds a ten-count tile;
3. the partner coholds the declaration-relative context the candidate action would lead;
4. the partner holds a count tile in that led context.

Event 3, for a candidate `action`, is literally
`(fix (roles (chair me) (chair ally) (domino held) (context q)) (out) (case (viewer me) (partner me ally) (holds ally held) (leads-context action q) (in held q)))`.
These use `holds` — World access — and so live only in the **examiner**; no executable policy sees them.

**Exact conditional centering.** For focal information node `I`, action `a`, active world multiset `W_I`, event `φ_j` and shared coefficient `θ_j`, the charged increment is `Σ_j θ_j · (φ_j(I,a,w) − mean_{w'∈W_I} φ_j(I,a,w'))`, every mean an exact `BigRational` over the full active multiset at the *full public history* (never a pooled or rebased center — pooling observably different states is an invalid centering, test `pooling_observably_different_states_is_an_invalid_centering_counterexample`). Centers are scoped by root, field, ordered prior digest, basis version and coefficient vector; any mismatch refuses without a bound (test `mismatched_centers_and_exhausted_budgets_refuse_without_a_bound`). Duplicate worlds keep multiplicity (test `duplicate_worlds_keep_multiplicity_in_information_states_and_centers`).

**The omitted root-action charge.** For a queried action the relaxed recursion fixes that action first and only then starts charging at future viewer decisions. A centered charge at the already-fixed action would integrate to zero and cannot tighten its own upper; omitting it makes that boundary explicit and stops a root-only calculation from looking like a gain (test `fixed_root_action_has_no_root_only_price`). The returned upper is the minimum of the priced relaxation, the independently computed unpriced perfect-information relaxation, and 1; the exact lawful Q must lie below it or the instrument refuses (test `every_full_history_center_is_exact_and_priced_upper_dominates_lawful_q`).

**What happened.** One coefficient vector `[0, 1, −1, 1]` was frozen before data selection (`learning_eval.rs` line 227; `relational_campaign.py`). Across the 128 discovery roots it tightened **zero of 64,806 action bounds**. There was little room: mean unpriced upper-minus-Q width, averaged over root actions then roots, was 0.186 pp (gym field) and 0.378 pp (L0-8), and pricing left both unchanged. The priced passes charged 2,856,972 and 1,932,754 units against 692,994 and 469,593 unpriced — about **4.1× the bound-pass work**, excluding the shared tree build; both routes had the same total allowance. No teacher acceleration was demonstrated. (`RESULTS.md` "What prices told us"; per-root rows in `measurements.json` `panels/*/price_diagnostics`.)

**The invariance statement** (a walt-internal, test-pinned finite-domain result at exploratory tier — not a kernel proof). With fixed continuation lowers, changing the upper changes the cost `max_b U(I,b) − L(I,a)` by a constant common to every action at that state; clipping to [0,1] does not alter this. Hence a tighter upper *alone* cannot rerank programs under this cost-sensitive constructor — whatever the price does to the bound, it cannot change which program is selected. Pinned natively by `changing_only_a_state_common_upper_cannot_rerank_programs` (`relational_learning.rs` line 53). The remaining legitimate uses the sources name — narrowing an action's interval, guiding adaptive teacher work, pruning a search — are stated and untested.

---

## 12. Where Scheme sits

**Relative to the solver.** The module doc of `walt/walt/src/scheme/mod.rs`: "Imports only rules/kernel and exact arithmetic." Verified from the `use crate::` lines: `scheme/*.rs` import `crate::kernel` and `crate::rules` only. The layering above it: `policy_search` imports `scheme`, `gym`, `solver::adaptive` (`SlicePolicy`, `PublicRecord`, `RootPosition`) and `solver::mix`; `gym` registers the one extension predicate. The expression engine has no solver dependency; the learners and examiner do. Crate module list at `c00717d1` (`lib.rs`): `carrier, geom, gym, kernel, policy_search, rules, scheme, solver, spec, strat` — see [walt-architecture](walt-architecture.md).

**Relative to the player.** The live L1/L2 player is unchanged by every commit in the slice (`RELATIONAL-LEARNING.md`: "The existing L1/L2 player is unchanged"). A learned `PolicyProgram` is *deployable* — `PolicyProgram::compile`, `initialize`, `choose`/`choose_traced` over an ordinary own/public `PolicyInput` — and *weak* (§10.3). The campaign used a stateless continuation initialization at each supplied root; it does not cold-start a learned stateful plan and pretend that memory was reached from the opening. [walt-seat-play](walt-seat-play.md) owns how walt actually plays.

**Relative to the objective.** Everything is P(make) with make/set only (the pmake ruling, 2026-08-17): `policy_lab` and `relational_lab` fix bid 30 (their usage lines say so), `Search`'s objective is the count of sampled worlds in which the declaring team makes, and every value is an exact rational. `Search` refuses a defending viewer rather than maximizing the wrong payoff (test `search_refuses_a_defending_viewer_instead_of_maximizing_the_wrong_payoff`).

---

## 13. Tooling and reproduction

### 13.1 The three binaries

All under `walt/target/release/` (built 2026-09-07 11:54; sources `walt/walt/src/bin/`). If a rebuild is unavoidable, `cargo run --manifest-path walt/Cargo.toml -p walt --bin <name> -- …` with `CARGO_TARGET_DIR=walt/target`.

| Binary | Purpose | Invocation |
|---|---|---|
| `scheme` | offline exact query over a real receipt coordinate; the §2 tool | `scheme --query FILE [--receipt FILE] [--hand N] [--trick 1..7] [--seat 0..3] [--max-worlds N] [--work N] [--condition FILE] [--compare FILE] [--comparison answers\|existence] [--selector first\|uniform]`; `scheme --registry`. Defaults: hand 0, trick 6, seat 0, world cap 40000, work 10000000. |
| `policy_lab` | one seed's paired fresh/persistent/compose construction under a frozen field, held-out evaluation, `TablePolicy` export to a `PolicyProgram`, one JSON row | `policy_lab --seed N --tiles 2..7 --samples 1,2,4,8 --test-worlds N --node-budget N --decl 0..7\|9 --mode random-own-hand\|fixed-root-hand [--hand seven,ids] [--field hash-legal\|l0-8\|gym] [--artifact-dir PATH]`; for an existing gym root `--request FILE --seed N [--exact-test 1]`. Bid is always 30. |
| `relational_lab` | `generate` (first qualifying own/public three-tile root in a seed range: unresolved bid 30, ≥2 legal plays, support ≤ cap; emits the seven-field public request), `fit` (reads a strict `.lessons` file of public request + rational weight + per-action costs; writes `candidate-*.policy` and metrics), `evaluate` (`learning_eval::run`: exact teacher, priced/unpriced bounds, full-fiber replay of saved actors) | `relational_lab generate --seed 910000 --tiles 3 --max-worlds 512 --attempts 200 --decl 6`; `relational_lab fit --lessons FILE --output DIR [--clauses 3 --beam 6 --ast 256 --work 2000000]`; `relational_lab evaluate --request FILE --policies FILE --output DIR [--field gym\|l0-8 --seed N --samples 16 --work N --prices on\|off]` |

Measured 2026-09-12 on this machine: `relational_lab generate` with the arguments above returned `seed 910003, attempts 4, worlds 210` in under 10 ms — the same root the 2026-09-07 survey recorded.

### 13.2 The Python runners and the watchdog protocol

`experiments/partnership/{policy_campaign,policy_gym,process_groups,relational_campaign,relational_exam,verify_relational}.py`, always run beneath the repository watchdog `experiments/partnership/packet/texas42-partnership-launch-v0.1/tools/run_capped.py --seconds 295` (the runners take `--seconds 285` inside it). Protocol common to both campaigns: the manifest pins configuration, runner source and binary bytes; each completed seed/job is committed atomically and hash-validated on resume; resume refuses changed inputs or damaged results, and must run under a *new* cap-receipt directory; SIGINT/SIGTERM kills the active native process groups and returns **status 75** (incomplete); in-flight jobs may retry, completed jobs are untouched; no native child may detach from its job. These semantics cover process interruption and restart, not power loss. Full invocations: `RELATIONAL-LEARNING.md` "Run, stop, inspect and resume"; `POLICY-SYNTHESIS.md` "Reproducible campaign". Raw job directories live off-repository under `/Users/jason/data/texas-42/{policy-synthesis-*,relational-*}-v1`; the checked-in `measurements.json` files are the durable record.

### 13.3 The focused Rust gate

66 tests, counted 2026-09-12 by `grep -c '#[test]'` over the eight files (run with `cargo test -p walt --test <name>` from `walt/`; **not run in this pass**):

| File | Tests | What it pins |
|---|---:|---|
| `scheme.rs` | 19 | grammar/type refusals; the five semantic rules; join parity against direct enumeration on all 90 worlds of a 6-hidden-tile, capacity-2×3 fixture under each of the nine declarations (`joins_match_direct_relations_on_every_world_under_all_nine_declarations`); weighted conditioning; selectors; certainty; registry access; undefined-not-false; budget refusal without partial report; the receipt CLI pin |
| `scheme_dynamics.rs` | 10 | typed play legality; belief pushforward order; viewer intervention preserves hidden odds; persistence/extinction/birth; hindsight preimage; refusal returns no partial belief |
| `scheme_policy.rs` | 6 | text roundtrip with exact keys; exact-before-relational with legality; World predicates refused, fallback total; table export and replay; transactional controller; malformed inputs refused |
| `policy_search.rs` | 9 | persistent-vs-fresh parity; duplicate samples keep multiplicity; donor completeness and union composition; restricted-cache scoping; refusal publishes no partial pool; defending viewer refused; held-out replay |
| `relational_runtime.rs` | 6 | traced provenance; atomic traced refusal; hybrid refusals; traced independent replay; the packet candidate is lawful syntax; contract resolution from banked public state |
| `relational_learning.rs` | 6 | tied costs do not split; the state-common-upper invariance; Viewer-only grammar with no tile literal; learned program roundtrip and cost parity; deterministic caps; complete bounded costs required |
| `information_prices.rs` | 8 | exact tree parity with `Search`; native events = compiled Scheme; exact centering and priced upper dominates lawful Q; no root-only price; duplicate multiplicity; refusals; malformed history refused (not rebased); pooled centering is invalid |
| `learning_io.rs` | 2 | lesson wire-format roundtrip; undefined/ambiguous rationals refused |

The 2026-09-06 round reported 19 tests; the 2026-09-07 dynamics round reported 46 (= 19 + 10 + 6 + 9 + the 2 unit tests in `policy_search/request.rs`); the final round reports 66. Python: 12 policy-runner tests and 7 relational-runner tests reported passing (`VALIDATION.md`). Full walt CI: waived in every session of the slice (banner at the top of this page).

### 13.4 Verification receipts

- **`campaigns/policy-synthesis-v1/verification.json`** (schema `texas42-policy-synthesis-resume-verification-v2`, "development verification only; excluded from training evidence"): a real SIGINT after 1 of 6 seeds completed — runner exit 75, no live native workers, resume completed 6 seeds with the pre-interrupt result hash unchanged; an independent Python rules replay passed all **222** retained full-game traces (gym 90, L0 36, opening 96). The receipt also records the pre-fix EPERM exit race (runner exited 2) that was fixed before the shipped runner; measured runner and binary snapshots are preserved under `reference/` with SHA-256s.
- **`campaigns/relational-learning-v1/verification.json`** (schema `texas42-relational-operational-verification-v1`; scope "operational resumability and static trace integrity; no performance claim"): static audits of **8,694** retained full games (gym panel 4,032; L0 panel 4,032; exam 630) and **25,935** focal decision traces (12,096 + 12,096 + 1,743) — turn order, legality, all 28 dominoes, 42 points, empty final hands, made/set, provenance and controller fields; a SIGINT/resume check against the final runner returned status 75, left no native children, and completed all 36 jobs on resume with the six durable results byte-unchanged. The shipped runner's SHA-256 differs from the measured snapshot because review later closed a reentrant-signal/spawn-registration edge; the final runner was verified independently and native search, programs and values were unchanged (`RESULTS.md` "Validation and provenance").

---

## 14. What is delivered and what is not

The delivered language covers v0.4 §§3–5 (typed role schemas, equality patterns, output interfaces, answer relations, probability distinctions) and the finite extensional operations of §6. Against the historical §12.7 compact-control-skeleton deliverable:

| Section 12.7 condition | Current status |
| --- | --- |
| Output roles explicit | Implemented and type-checked; internal witnesses are projected away. |
| Rigid and fresh roles distinguished | Implemented finite transport preserves explicit output identities; fresh successor queries rebind independently; persistence, extinction, and birth compare the two relations. |
| Exact support authoritative | Enforced by Frame/World compatibility and measure construction. |
| Continuation horizon and information access declared | Typed `PredicateSpec` fields (`horizon_plies`, `access`) required by the registry; Viewer callbacks receive no World. Callback semantic fidelity is a trusted extension obligation. |
| Step compiler preserves the answer relation | Typed finite frame/world/belief transforms are implemented. No compact descriptor step compiler or compressed transition is claimed. |
| Induced descriptor transition meets a selected theorem | Not claimed for the new expression layer. Archived finite-domain descriptor checkers (§16) retain their original limited status. |

The old concrete-tile implicant language (§16.4) still lacks the role-binding generality of the new expressions; recovering its widening/generalization workflow for a gym remains useful, but its old perfect-information pair verdicts cannot serve as lawful make-30 answer keys unchanged. The PARTS-catalog and compact-descriptor questions remain open; they are not prerequisites for asking relational questions, integrating query events under a declared belief, or producing counterexamples. Open extensions named by `relational-learning-v1/RESULTS.md` and not built: a learned opening policy, score/history predicates, mode induction, coefficient learning, belief-summary actor inputs, adaptive teacher allocation, price-based pruning. Which, if any, is next is Jason's call. No ledger (`claim-ledger.md`, `FINDINGS.md`, `open-problems.md`) carries a Scheme entry; that is consistent with the exploratory tier, and the unbuilt step compiler and the absent lumpability theorem for the expression layer are tracked only here.

---

## 15. Naming note

Two walt-internal uses of the word "certificate" appear in this slice's sources and are **not** adopted by this wiki, which follows D3: the object is a **necessary outer profile**, never a "certificate" (rob's greps enforce it; one ingest package's "outer certificate" naming is recorded as deprecated in [discrepancies](discrepancies.md) and [reachability](reachability.md)).

1. walt's §16.11 experimental record type is called a "certificate" in its own namespace (the archived files under `walt/walt-factory/results/certificates_2026-08-10/`, now `walt/probes/factory-results/`) — a replayable experiment record with per-record coverage labels, unrelated to the D3 concept.
2. The campaign reports and `INFORMATION-PRICES.md` write "certificate width" and "interval regret certificate" for, respectively, the width `U − Q` of an action's interval and the identity that on-policy interval costs bound the whole-policy gap. On this page those are the **action interval width** and the **interval regret bound**; the source phrases are quoted here only so a reader can find them.

The one permitted walt term of art is **certified regret** Γ = U* − B_exec ([walt-counted-belief-era](walt-counted-belief-era.md)), which this slice does not use.

---

## 16. Descriptors, soundness and lumpability (archived research, 2026-08)

> **Archive banner.** Everything in §16 is the *pre-implementation* descriptor/compression
> research: the `walt-skeleton`/`walt-factory` crates it quotes are archive-only at producer
> commit `648f93a` (deleted by the 2026-08-24 unification — [`walt/ARCHIVE.md`](../walt/ARCHIVE.md));
> the `results/...` files it names now live at `walt/probes/factory-results/`. Source paths and
> crate names are historical truth about where the instruments were built. The 2026-09-06
> expression layer (§1–§15) **inherits this section's counterexample discipline — a verdict is
> never quoted without its grade and operator pair, a failure names its witnessing world, an
> undefined atom is never defaulted — and none of its compression claims.** No lumpability
> result is claimed for the new layer. The text below is kept essentially verbatim from the
> page as it stood at `c00717d1`: tenses were moved to the past and the crate/test paths
> marked "archived", and three insertions are marked *[2026-09-12]* (§16.1, §16.4, §16.5 rule 4).
> No number, quotation or verdict was changed (diffed against `git show c00717d1:wiki/walt-scheme-fix.md` §§1–6 on 2026-09-13).

### 16.1 What is a descriptor?

A **descriptor** is a compressed stand-in for the hidden state of a hand. The seat cannot
see the other three hands and wants to carry something far smaller than "which of these 90
worlds am I in" while still deciding correctly.

The tempting shape is a *labeling*: a function that looks at the true world and returns a
summary. walt refuses that. Here a descriptor is a **transducer** — typed state plus a
closed update law, `d' = step(d, obs)` (v0.4 §12.5's `D_{t+1} = δ_D(D_t, a_t, o_{t+1})`).

The distinction is the whole point. A labeling is evaluated *against the world*; evaluate it
again after a trick and you consult the world again. That is fine for an analyst with
God's-eye access and useless for a seat, which never had the world to consult. A transducer
reads the world **once**, at the root, and thereafter advances on public observations alone.
Closed updating prevents another hidden-world read, but does not make the initial
hidden descriptor known to the seat. Unless disclosed, the seat must carry belief
over its possible descriptor values. Public-only updates alone do not establish
information-local executability.

walt made this a type property rather than a promise (archived `walt-skeleton/src/skeleton.rs`; *[2026-09-12]* not to be confused with the live `walt::scheme::PolicyProgram`/`CompiledPolicy`, which are §9's objects):

```rust
pub trait ControlSkeleton {
    type State: Clone + Ord + core::fmt::Debug;
    fn name(&self) -> String;
    fn kind(&self) -> UpdateKind;
    fn init(&self, kernel: &Kernel, world: &World) -> Self::State;
    fn step(&self, d: &Self::State, obs: ObservedPlay) -> Self::State;
}
```

`init` is the only place a latent `World` appears. `step` sees the state and one observed
play — so an implementation that wanted to recompute from the hidden world at trick four
cannot: it has no world in scope. Non-closed updates are *unconstructible*, not merely
discouraged.

One escape hatch exists and is labelled. `UpdateKind::StaticPassenger` marks a descriptor
whose `step` is the identity — a frozen root evaluation riding the dynamic harness (that is
what `StaticWrap` builds, naming it `static[...]`). Passengers are allowed so the older
static-compression experiments can re-run through the new machinery, but the search
objective prefers closed updates and every report prints the kind.

Two inherited disciplines: descriptor state is a **derived view**, never a stored authority
(equality and ordering through projected content only), and it is never an identity-bearing
record of reachability. And support is not belief — a descriptor cell is a set of
rule-compatible worlds, not a weighting over them.

### 16.2 What was Scheme/Fix for, in this program?

Scheme/Fix is the **language in which a class or a situation becomes sayable**. walt's
census machinery can hand you a class of situations and prove things about it, but cannot
yet *say what the class is* in any form shorter than the class itself. §12.7 names that gap
and proposes the language to close it.

The syntax comes from v0.4 §3:

- A **role schema** `Σ = (N_Q, N_C, N_D)` names effective-context roles, chair (seat) roles,
  and domino roles; an interpretation binds those names to concrete contexts, seats, tiles.
- An **output interface** `O ⊆ Σ` says which names are *returned* and which are merely
  internal existential witnesses. Load-bearing: internal proof choices must not leak out as
  extra referents, extra probability mass, tracked identities, valued objects, or public
  observations. `O = ∅` is a Boolean event query; `O = Σ` a full witnessed realization query.
- A **Scheme case** `S = (π, φ)` pairs an *equality pattern* π — which distinct role names
  may denote the same object — with a finite conjunction φ of registered atoms (`Live(e)`,
  `Holds(c,e)`, `In(e,q)`, `Beats(e,f,q)`, `Void(c,q)`, `Team(c,t)`, and registered derived
  predicates).
- A **Fix** `F = S_1 ∨ … ∨ S_r` is a finite disjunction of Scheme cases over one common
  schema and output interface. The empty Fix is false.

Two rules matter more than the grammar. Every derived continuation atom (companion,
forced-follower, beater chain, mobility) must declare its **horizon** and its **information
access**. And a predicate that calls the target solver or reads the response class is
**forbidden target leakage**: you may not define an atom as "the one that gets the answer
right."

The language sits *above* the physics. `walt/PLAN.md` (retired; `git show 56e2173:walt/PLAN.md`): "Scheme/Fix as a query language
enters later inside walt-skeleton's descriptor vocabulary; it imports physics, never the
reverse." A Fix asks a typed relational question inside the worlds of a decision state; it
never replaces the decision state `B = (K, e, β)`, and its result is an *answer bundle over
worlds* — not automatically a Boolean, not a unique referent, not a distribution. A Scheme
cell is also not a symmetry orbit and not a purpose class (§11.1): it can contain part of one
orbit, several orbits, or several strategic classes, and there is no purpose-free canonical
class.

### 16.3 The two axes every descriptor is graded on

#### Axis 1 — soundness (§12.1): does the answer survive the compression?

With a finite domain `X`, an exact target response `R*: X → Y`, and a descriptor `D: X → Z`,
`D` is **purpose-sound** when `D(x) = D(y)` implies `R*(x) = R*(y)`; equivalently, `R*`
factors uniquely as `R* = R̄ ∘ D`. Intuition: you may merge two worlds only if the answer you
care about is the same in both.

Soundness is *purpose-relative*. There is no sound descriptor, only a descriptor sound for a
stated target — walt's own runs show one descriptor sound for one target and unsound for
another on the same fiber.

`check_soundness` (`walt-skeleton/src/soundness.rs`) enumerates the entire fiber, asserts the
world count against the exact counting DP, and reports `(worlds, cells, responses)` — the
"90 to 33 to 8" shape. On failure it emits a §12.9 step-4 witness: two concrete worlds in one
cell with different responses.

#### Axis 2 — lumpability (§12.6): does the compression survive the dynamics?

Soundness is about one moment; a seat plays a whole hand. §12.6's target is **strong
controlled lumpability**: whenever `d(x) = d(y)`, (1) the legal focal action sets agree,
`A(x) = A(y)`; and (2) for every legal action, feature increment, observation, and successor
class, the class-aggregated probabilities agree.

Condition 1 says two states in one class must offer the seat the same menu, or the class
cannot name an action. Condition 2 says the class-level transition law is well defined: from
anywhere inside the class, the chance of "score this much, see that, land in class `y'`" is
the same. When both hold, the abstract kernel exists, abstract belief updates using it alone,
the joint law of observations and accumulated features is identical concretely and
abstractly, and every utility and every optimization over the same abstract-policy class gets
the same value. Exact dynamic compression, not approximation — and the spec is explicit that
the bar is deliberately stringent: weaker belief-dependent or policy-relative quotients may
exist and would need their own theorems.

`check_lumpability` (`walt-skeleton/src/lumpability.rs`) builds the whole reachable carrier —
every fiber world, every reachable viewer-decision node, every legal action, every
positive-mass field segment, exact rationals, nothing sampled — and returns
`LumpabilityFailure::LegalSets` (condition 1), `LumpabilityFailure::Kernel` (condition 2,
with the disagreeing event and both masses), or a pass. The field is the fixed
uniform-over-legal chance law of §7.4; each kernel row is asserted to sum to one exactly.

**§12.6A, the equivariant version.** v0.4 §12.6 compares interfaces *literally*, so two
situations differing only by which tile plays a role can never merge — under that reading
only world-reconstructing skeletons pass. The v0.5 amendment replaces literal equality with
equality **up to declared typed transports** `Θ` on roles, actions, and observation labels,
with outcomes compared count-free; coherence is required, and the transports are explicitly
*not* claimed to be global symmetries of Straight 42. That is the axis the census work runs
on — see [census era](walt-census-era.md).

### 16.4 How to read an archived lesson implicant

*[2026-09-12]* This section reads the **archived** lesson-implicant language, not the current
Scheme/Fix syntax; a reader who wants to write a query today starts at §2–§3 and
`walt/scheme/README.md`. At the archived producer commit no Scheme/Fix parser existed; the
lesson implicant — a conjunction of typed cells with a graded, labelled verdict — was the closest
available thing to a Fix and is what you meet in the results files. Verbatim from
`walt/walt-factory/results/lesson_basins_2026-08-10_r4.txt`:

```
walt lesson (S5b) — exploratory tier
origin: regret conflict h0 S1 t5 p0 fiber 1680 grade worldwise-dominance at (C, minimax-omniscient)
  chosen 3-2 better [0-0 2-1] regret 61/21
verdict: refutation: value(decisive) >= value(max-count) at every matching (decision, world)
grade: worldwise at (C, minimax-omniscient); weighting-free
domain: receipt corpus, hands 0-12, all seats, tricks 5-6, fiber <= 40000 — 104 decisions, 23790 worlds, all fibers exhaustively enumerated (0 in-range decisions excluded by the fiber cap)
initial implicant (25 cells): hand=0 & seat=S1 & decl=P3 & role=declaring & horizon>=3 & horizon<=3 & ply=0 & beaters-total(1-1)=0 & beaters-total(2-0)=4 & beaters-total(2-2)=0 & beaters-total(4-1)=2 & beaters-total(4-2)=1 & beaters-total(4-4)=0 & beaters-total(5-1)=1 & beaters-total(5-2)=0 & beaters-total(6-2)=0
```

- **`origin`** — the conflict being generalized. A *conflict* is a refuted line: at hand 0,
  seat S1, trick 5, ply 0 (the seat led), the walker chose 3-2 while 0-0 and 2-1 were better,
  exact rational regret 61/21 over a 1,680-world fiber.
- **`verdict`** — three shapes exist. `refutation` (one action's value is at least another's
  at every matching decision and world), `win` (an action attains the world optimum —
  per-world sufficiency, explicitly *never* a seat-facing guarantee, since §7.6's
  strategy-fusion gap means no single information-consistent strategy need achieve per-world
  values), and `not-lumpable` (a named descriptor family fails §12.6 at every matching
  decision).
- **`grade` and the operator pair** — the two labels no verdict may be quoted without. Grade
  is the rung it was verified at: `worldwise` (every world of every matching fiber,
  weighting-free), `exact-expectation` (under a declared weighting), `sampled` (marked as
  such), or `checker` (the exhaustive §12.6 checker). The operator pair
  `(C, minimax-omniscient)` is a *pair*: a focal-information coordinate (F, C, or H — what
  the focal side may condition on, §10.3) and a field coordinate (omniscient adversarial
  minimax, or the fixed uniform-legal chance law, §10.8). Never one rung alone — §12.4 makes
  results label-relative, and the same measurement at another pair can shatter or merge. The
  origin keeps its own grade unchanged: a lesson verified worldwise does not upgrade a
  sampled origin.
- **`domain`** — the DomainSpec, and the application gate.
  `DomainSpec::covers(trick_no, fiber)` is consulted *before anything else*: a decision
  outside the trick range, or with a fiber above the cap, was never verified, so the lesson
  simply does not apply there. Capped decisions are excluded, never sampled — and when a cap
  bites, runs print the control-bias annotation, because fiber size anti-correlates with
  focal control, so an excluded set skews low-control.
- **`initial implicant`** — the conjunction describing the origin. **Decision cells**
  (`hand`, `seat`, `decl`, `role`, `ply`, `horizon`) are public facts of the decision point.
  **Atom cells** (`beaters-total(2-0)=4`, `team(1-1)=false`, `bestkeep=true`) are latent
  facts from the atom vocabularies. An atom cell is **partial**: where its precondition fails
  — tile not in the hidden pool, companion undefined — it is *unsatisfied*, never defaulted.

The trace is where the generalizer's reasoning shows:

```
trace:
  drop hand=0 -> dropped
  drop seat=S1 -> dropped
  drop beaters-total(1-1)>=0 -> dropped
  drop beaters-total(1-1)<=0 -> dropped
  ...
  introduce beaters-total(2-0)>=4 (cut refinement) excluding witness h4 S1 t5 p0 world S0={4-3 4-4 5-4} S1={2-1 4-0 5-1} S2={0-0 1-1 2-0} S3={2-2 4-2 5-5} values 2-1=9 4-0=-11 5-1=9
  drop decl=P3 -> dropped
  drop ply=0 -> dropped
  introduce beaters-total(1-1)<=0 (cut refinement) excluding witness h10 S0 t5 p1 world S0={3-0 4-4 6-5} S1={2-0 3-2 6-1} S2={2-2 3-3 4-0} S3={5-3 6-3} values 3-0=-6 4-4=-6 6-5=4
  drop role=declaring -> dropped
  drop horizon>=3 -> dropped
  drop horizon<=3 -> dropped
final implicant (2 cells): beaters-total(2-0)>=4 & beaters-total(1-1)<=0
```

Start from a 25-cell description of one concrete decision and try to throw each cell away,
re-verifying exhaustively over the whole domain after every attempt.

- **`drop X -> dropped`**: the cell came out and the verdict still held everywhere.
- **`drop X -> SURVIVES; witness …`**: the drop was refuted by a concrete counterexample and
  the cell was restored — load-bearing, with the witness naming why.
- **`introduce X (cut refinement) excluding witness …`**: a widening failed, so rather than
  give up the generalizer *added* a world-selecting cell that is constant across the
  already-verified set and false-or-undefined at the witness, then re-verified. This is the
  lesson-level twin of §12.9 steps 5–6. There is a hard budget (`intro budget: 2/4 spent`)
  and the exclusion property is enforced by construction.

**Why bounds relax while equalities can only be kept or deleted.** Numeric facts enter as
*pairs* of one-sided bounds: `beaters-total(2-0)=4` is stored as `>=4` together with `<=4`
and printed as the equality only when both halves survive. An equality cell is atomic — keep
it or delete it. A bound cell has a **relaxation ladder** (`>=4` weakens to `>=3`, to `>=2`,
to vacuity), so the generalizer can widen one step at a time. That was adopted for a concrete
observed reason: under the earlier equality-only scheme, four lessons died with zero basin
because a horizon pin could not bend. A real relaxation, same file:

```
  relax horizon>=4 -> BOUND HELD at horizon>=4; witness h2 S0 t5 p1 world S0={3-0 3-1 4-3} S1={1-0 1-1 2-1} S2={2-2 3-2 6-6} S3={4-1 5-4} values 3-0=-1 3-1=-1 4-3=-13
  drop horizon<=4 -> dropped
final implicant (1 cells): horizon>=4
```

The upper half went entirely; the lower half relaxed as far as verification allowed, then
*held* at its original value, named with that value and the witness refuting the next step.

Finally the measurement:

```
carrier: selector-resolvable decisions x their fiber worlds — eligible 32 decisions / 14387 worlds (domain context: 104 decisions / 23790 worlds, not a rate base)
basin [at grade: worldwise at (C, minimax-omniscient); weighting-free]: decisions 1/32 worlds 1680/14387 triple (488,1192,0)
basin dominance class: W (weak, strict somewhere)
```

The **basin** is everything the final implicant matched and verified. Its rate base is the
verdict's own **carrier** (§11.1: measures are carrier-relative), never the whole domain,
which appears as labelled context and is explicitly "not a rate base". The **triple**
`(gt, eq, lt)` counts worlds where the better action beat, tied, and lost to the worse one;
`lt` is structurally 0 in any verified basin, since verification aborts with the witnessing
world instead. Class W is weak dominance with strictness somewhere; class T (tied everywhere)
is an interchangeability statement, not a refutation, and is never collapsed into one.

### 16.5 How to write one (archived vocabularies)

Three vocabularies existed, all **registries** — closed lists of atoms with declared semantics,
not open-ended grammars.

**The walt-native registry** (`walt-skeleton/src/atoms.rs`, `enum Atom`) is genuine
transducer vocabulary: `HolderOf(tile)` (which hidden slot holds it, or shown),
`TeamOf(tile)`, `BeaterCounts(tile)` (per-slot counts of still-unshown tiles that beat it
when led). These update closed because possession is static until the tile appears: `init`
reads the world once, and each observed play only ever *removes* latent content. Alongside
rides the **public chassis** — viewer hand, current leader, current trick prefix — a pure
fold of the observation record carrying exactly what is needed to know the seat's own legal
set. Atom tiles must be in the hidden pool; facts about the viewer's own tiles are public and
belong to the chassis.

**The exp3A registry** (`enum Exp3aAtom`) is v0.4 §14.4's 22-observable vocabulary,
reimplemented from the preserved probe: a holder fact and a team fact per pool tile, plus ten
control shapes — `comp` (the tile sharing the valued tile's holder's hand), `comp-in-context`,
`comp-is-floor`, `comp-rank`, `focal-max`, `opp-max`, `focal-top`, `opp-beaters`, `bestkeep`,
`with-boss`. These are static root labelings, so descriptors over them are marked
`StaticPassenger`.

**The lesson vocabulary** (`walt-factory/src/lesson.rs`) unions the two, adds the decision
cells, and adds a small registry of *numeric* atoms (`beaters-total(tile)`, `opp-beaters`)
that order cells may bound.

Rules a new atom had to respect:

1. **Kernel-generic definition.** The exp3A vocabulary was designed against one hand; walt
   re-derives its parameters at any kernel (the decisive tile is the viewer tile whose led
   context touches the most hidden-pool tiles, ties to the higher tile). That rule is
   *recorded* in the code, because it is a choice, not a theorem.
2. **Partial evaluation, never defaulting.** `Exp3aContext::try_eval` returns `Option`, and
   `None` exactly where the precondition fails — a holder or team fact about a tile no hidden
   slot holds, or any companion-family atom when the valued tile's holder does not hold
   exactly two tiles. The §14.4 vocabulary is native to capacity-2 kernels; elsewhere the
   companion is *undefined*, not zero, not false. An undefined atom satisfies no equality
   cell and no bound.
3. **No target leakage** — no atom may call the solver or read the response class (§3.3).
4. **Declared horizon and information access** for derived continuation atoms (§3.3, §12.7).
   *[2026-09-12]* In the archived lesson vocabulary this was prose in the doc comment, not a
   machine-readable field. In the current registry (§5) it is a typed `PredicateSpec` field
   (`horizon_plies`, `access`) checked at compilation — though the semantic fidelity of a
   custom predicate to its declaration remains a trusted-extension obligation.
5. **One name per atom.** The lesson vocabulary wraps only the ten control shapes; holder and
   team coordinates come through the native variants, so no atom has two names.
6. **Registration gates numerics.** Only atoms listed in `NumericAtom` can carry order cells;
   adding one is a deliberate edit to that enum.
7. **Selectors before tiles.** A lesson names actions by `decisive`, `max-count`, or
   `min-count` before falling back to concrete `tile(x)`; an unresolved selector makes the
   decision inapplicable, never defaulted.

### 16.6 Worked examples

#### 16.6.1 The exp3A four-atom descriptor: 90 worlds, 33 cells, 8 responses

From the preserved probe's own Part 1 output (`walt/probes/exp3a/v3_output_postfix.txt`):

```
  registered atom vocabulary (22):
    41_with_22, comp41, comp41_in_suit2, comp41_is_2-0, comp41_rank2, h(2-0), h(2-2), h(4-1), h(4-2), h(4-4), h(5-2), nbeat_opp, oppmax2, s3_top2, s3max2, t7w_bestkeep, team(2-0), team(2-2), team(4-1), team(4-2), team(4-4), team(5-2)
  targets: R8 = 8-class parametric root-Q signature; R3 = 3-class action correspondence

  -- exhaustive minimum-size search, target R8 --
    smallest sound descriptor size: 4 atom(s); 8 minimal solution(s)
      ['comp41', 'h(2-0)', 'h(4-2)', 's3max2']  (69 cells)
      ['comp41', 'h(2-0)', 's3max2', 'team(4-2)']  (53 cells)
      ['comp41', 'h(4-2)', 's3max2', 'team(2-0)']  (53 cells)
      ['comp41', 's3max2', 'team(2-0)', 'team(4-2)']  (33 cells)
```

The domain is one trick-six kernel with a 90-world fiber. Target R8 is the eight-class
parametric root-Q signature — each world's exact response class. An exhaustive size-ordered
search over all subsets of the 22-atom registry finds no sound descriptor at size one, two,
or three, and exactly eight at size four. The last, `{comp41, s3max2, team(2-0), team(4-2)}`,
cuts 90 worlds into 33 cells through which the eight-class answer factors exactly — the
`R* = R̄ ∘ D` of §12.1, the "90 to 33 to 8" of §12.3. The winning atoms are control-shaped:
companion, decisive-context partner strength, two team facts. Swap `team` for `holder` and
the count rises to 53 or 69 — still sound, less compressed, because holder is finer.

walt's own checker reproduces this independently: `check_soundness` on the ported descriptor
returns `(90, 33, 8)`, and `(90, 33, 3)` for the three-class action-correspondence target —
the *same* 33 cells serve both. The full size-≤4 search reproduces the probe's whole record
(minimal size 4, eight solutions, cells 69/53/53/33, both targets). Those equalities are
CI-asserted pins in the archived `walt/walt-skeleton/tests/harness.rs`.

The caveat travels with the result: this descriptor is a **static passenger**, sound for a
root target and silent about dynamics. §12.4 records what the follow-up Experiment 4B did and
did not prove — genericized versions were sound on 30 of 72 fresh tasks against 21 of 72 for
a holder-only baseline (real partial signal), but removing the vocabulary ceiling collapsed
median held-out world compression to 1. That is one flat descriptor family failing for one
target, *not* a proof that Straight 42 has no compact exact representation.

#### 16.6.2 A lesson with an introduced cell, and one with a relaxed bound

The §16.4 walkthrough is itself the first: 25 cells down to 2, and both survivors were
*introduced by cut refinement* rather than retained — the entire original description dropped
away, and the generalizer then spent 2 of its 4-introduction budget re-carving the class
around two witnesses.

Read honestly: the lesson generalized *in vocabulary* (from "hand 0, seat S1, declaration P3,
these ten exact beater counts" down to two one-sided bounds) but not *in reach* — it still
matches one decision of the 32 eligible on its carrier, and the 1,680 worlds are that single
decision's whole fiber. Vocabulary generality and basin size are different measurements, and
this run shows them coming apart.

The relaxation half is the `horizon>=4` block quoted in §16.4. **No single printed
lesson in this run carries both a `BOUND HELD` relaxation and an `introduce` step in one
trace**; the two mechanisms are shown here from two different lessons in the same file. That
is a reporting fact about this run, not a claim about the machinery.

A cleaner survivor case from the same file, where the surviving cell reached five decisions:

```
  drop ply=2 -> SURVIVES; witness h5 S0 t5 p3 world S0={4-1 5-0 5-2} S1={1-1 4-2} S2={4-3 4-4} S3={5-1 5-5} values 4-1=-21 5-0=-11 5-2=-21
final implicant (1 cells): ply=2
surviving: [ply=2]
basin [at grade: worldwise at (C, minimax-omniscient); weighting-free]: decisions 5/11 worlds 651/2135 triple (0,651,0)
```

Everything except "the seat is third to play" dropped away; the triple `(0, 651, 0)` says the
named action tied the world optimum in all 651 matched worlds, the expected shape for a `win`.

#### 16.6.3 Descriptors that failed

**The static passenger fails condition 1, structurally.** A frozen root evaluation gives the
same state to a world's root node and to that world's later nodes — but the legal set has
changed by then, so `A(x) ≠ A(y)` inside one class. `check_lumpability` returns
`LumpabilityFailure::LegalSets`, on *any* kernel with a future focal decision. This is
asserted in the archived `walt/walt-skeleton/tests/harness.rs`
(`static_passenger_is_marked_and_fails_lumpability_on_legal_sets`), which also records the
static run for the same descriptor: `(90, 15, 8)`, not sound. Note this is a CI-asserted test
expectation, not a line in a results file — no dated file in `walt-factory/results/` prints a
`fail:legal-sets` verdict.

**The public chassis fails condition 2 everywhere it was checked**, from
`walt/walt-factory/results/falsification_2026-08-10_r2.txt`:

```
origin: §12.6 lumpability failure h0 t6 descriptor chassis — kernel witness at carrier nodes 325,327: action 0-0 increment -1 mass 1 != 0 (carrier rebuilds from the kernel)
verdict: not-lumpable: chassis fails §12.6 at every matching decision
grade: checker (§12.6 exhaustive lumpability, uniform-legal field, q_points valuation)
final implicant (0 cells): (empty)
carrier: lead-kernel trees (ply 0, horizon <= 2) — eligible 13 decisions / 647 worlds (domain context: 179 decisions / 924813 worlds, not a rate base)
basin [at grade: checker (§12.6 exhaustive lumpability, uniform-legal field, q_points valuation)]: decisions 13/13 worlds 647/647
```

(The `domain`, `initial implicant`, and 19-step `trace` lines, and the bookkeeping lines
after `final implicant`, are elided; nothing else is changed.) The witness is a condition-2
disagreement: two
carrier nodes in one chassis class where playing 0-0 for increment −1 has mass 1 at one node
and 0 at the other. Every cell of the 19-cell origin dropped, leaving the empty implicant —
the strongest form this verdict takes: *the public chassis alone is never lumpable*, on all
13 eligible lead-kernel trees of the domain. Quantifier discipline matters: the checker
verdict quantifies per matching *decision*, so atom cells would have to hold at every fiber
world to count, and the basin counts under exactly that quantifier.

**Vocabulary ceilings.** The pinned soundness table in the archived
`walt/walt-skeleton/tests/synthesis_run.rs` records, per trick-six kernel and target, the
exhaustive minimum-size search over the walt-native registry:

```
"h0 fiber=90 q_points: UNSOUND at every size <= 4 (vocabulary ceiling)",
"h1 fiber=90 q_points: min-size=4 solutions=2 first=team(5-3)+holder(1-0)+holder(4-0)+holder(5-4) cells=42",
"h2 fiber=36 action: min-size=0 solutions=1 first={} cells=1",
"h11 fiber=36 q_points: UNSOUND at every size <= 4 (vocabulary ceiling)",
"h11 fiber=36 action: min-size=4 solutions=4 first=team(3-1)+team(6-6)+holder(5-5)+holder(6-3) cells=24",
```

Three readings. On hand 0 no subset of four or fewer native atoms is sound for the q_points
target — an honest ceiling report, §12.9's failure branch, and *not* a claim that no
descriptor exists. On hand 2 the action target is sound at size *zero*: every world agrees on
the action, so the empty descriptor factors it. Hand 11 shows purpose-relativity in one place
— unsound for q_points, sound at size four for the action target, same fiber.

The companion lumpability table shows both outcomes:

```
"h0 nodes=738 chassis: fail:kernel classes=110 merged=628 largest=360",
"h0 nodes=738 chassis+holder-all: LUMPABLE nontrivial classes=366 merged=372 largest=6",
```

Adding every holder fact to the chassis does produce a genuinely lumpable, genuinely
compressing descriptor on hand 0 — 738 carrier nodes into 366 classes. It also very nearly
reconstructs the world, which is exactly the tension §12.4 named — and the tension the
2026-09-06 redirection (§1) stepped around rather than resolved.
