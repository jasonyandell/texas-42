# Human-called Nel-O player

Exploratory player engineering for [issue #89](https://github.com/jasonyandell/texas-42/issues/89).
Implemented in isolated `codex/nello-player` worktrees for Texas 42 and Plunge.
The [initial audit](NELLO-INVESTIGATION.md) records the previous implementation
boundary. This document records the candidate implementation, not a deployment.

## Algebra and contract

The source is the original [Suit Algebra](math/SUIT_ALGEBRA_PURE.md), especially
§§3–6. This file is preserved byte for byte from Plunge commit
`27d7bc45eba7318559f744da762952d6533baa35`, `docs/SUIT_ALGEBRA_PURE.md`.
It already defines ten declarations. Doubles-suit calls the seven doubles but
gives them no power. Doubles follow doubles and rank by pip; mixed leads use
their high pip, and the highest follower wins. A double cannot trump a mixed
lead. `Decl::DoublesSuit` now represents this declaration directly.

The contract supplies the rest: four physical hands, three active actors,
clockwise play skipping the declarer's partner, and the zero-trick objective.
The inactive hand remains seven hidden dominoes in every outer and inner
completion. The declarer maximizes the probability of avoiding all seven
tricks; each defender minimizes that same event. Taking any trick immediately
sets Nel-O. Real count is retained for display and replay, never used as a
surrogate objective. One retained make has only 37 played points because count
remains in the inactive hand.

Every valid nonterminal history has zero declarer wins. Thus the previous
winner (the next leader) witnesses a newly terminal failure; after seven
completed tricks without that witness the contract succeeds. The wire adapter
rejects histories that continue past either settlement. Solver cache identity
includes contract and inactive-seat geometry. Nel-O uses the general recurrence;
the existing four-player compact paths, nine-declaration GPU formats, and
count-offer partner review retain their straight-contract scope.

## Interface and table

Straight requests retain their seven fields and identity. Nel-O adds
`"contract":"nello"`, uses `decl:8` for doubles-suit, and carries the actual
1–9 mark stake in `bid`. Its responses identify the contract and inactive seat;
the optional straight partner review reports `inapplicable-nello`.

The same L1 stages, deadlines, legal checkpoint and completed-comparison
fallbacks apply in native and WASM. Hidden hands are never request fields.
The Python table adapter independently checks legal play and state, and the
Mac bridge accepts the optional contract for decisions, receipts and rechecks.
The historical counterfactual gym stays explicitly scoped to straight bid 30.

Plunge enables its existing open Nel-O declaration after an ordinary mark bid.
The empirical book and computer auctions still contain only the nine straight
declarations. Replay preset `v1l` means forced-30 plus open own-suit Nel-O;
`v1f` keeps its original forced-30/straight-only meaning. Questions, examiner
positions and portable receipts use actual recorded trick lengths. Saved hands
keep their rules and contract; the next new table hand gets the new preset.

## Validation and first defense screen

Receipts: [nello-player-2026-09-20](probes/nello-player-2026-09-20/).

- Exhaustive independent three-player winner check: 235,872 cases, all physical
  declarer/leader geometries. Eight engine fixtures cover immediate sets and
  seven-trick makes at all four declarer seats.
- All 96 fixture positions match native/WASM state, chosen action and complete
  rational option vectors. Fake-clock interruption retains a legal completed
  checkpoint. Malformed contract, hidden-hand, inactive-seat and post-set inputs
  are rejected. Cache tests reject reuse across contracts or inactive seats.
- The original straight native/WASM check and the previous shipped versus
  candidate comparison pass. Full Plunge suite after integrating saved-hint support: 229 tests. Focused Rust suites:
  21 tests. Python Nel-O, bridge, replay and table suites: 23 tests. Workspace
  compilation and the player without default features are also checked.
- An unrelated existing `test_foundation` assertion still rejects a 15-second
  budget although its unchanged `Player` class now allows 20 seconds. This
  pre-existing mismatch is outside this change.

On this Mac, the 96-position Node-hosted WASM panel had median 6.1 ms, 95th
percentile 122.1 ms and maximum 141.3 ms. This is **not phone latency**.

The first paired screen used two very low fixed declarer hands with eight
hidden deals each. Both random defense and Walt set 0/16: useful make/continuation
coverage but uninformative for comparative defense. This pilot is retained.
The separate mixed screen used the two audit hands plus four seeded uniformly
drawn hands, eight hidden deals each, rotating the declarer through all seats.
Both arms used the same L1 declarer; the defenders were random legal play or
Walt L1 40/8. Walt set 38/48, random set 37/48: 31 mutual sets, four mutual
makes, seven improvements, six reversals. This does **not** establish stronger
defense. The reversals are seeds `4209104`, `4209105`, `4209107`, `4209203`,
`4209401`, `4209403`. All full histories, own/public requests and responses are
retained in the compressed JSON receipts.

All 726 Walt decisions in the mixed screen completed: 506 full comparisons and
220 forced moves. The 362 defender decisions had 95th percentile 64.8 ms and
maximum 506.9 ms native on this Mac. No partial comparison or legal fallback
was selected. Hands and continuations are sampled, not a census or a calibrated
strength result. A local browser smoke check also passed ordinary bidding, three-player play, reload/resume, saved hints, immediate set, original scores and a 160-world recheck (0.39 s on this Mac). A Pixel play/latency check remains a release follow-up.

## Counterexample research

The [wiki synthesis](../wiki/walt-nello.md) connects the sample ladder, tactical
failures, doom semantics and held-out limitations. Current integration checks
and merge order are in [merge readiness](NELLO-MERGE-READINESS.md).

The [2026-09-22 counterexample probe](probes/nello-counterexample-2026-09-22/REPORT.md)
retains sampled failure witnesses, jointly replans candidates, and adapts the
singleton fixed-field doom question to Nel-O. It demonstrates a 100% tie being
broken and records a concrete 6-6 discard vulnerability. The two-root fresh-deal
panel is mixed, with substantial off-tree L0 completion; it establishes neither
calibrated probabilities nor stronger live play.

The opt-in preview adapter accepts `nello_counterexamples:true` outside the
actor-only request. After a complete ordinary comparison, Nel-O defenders may
spend up to 4.2 reserved seconds on three rounds of 540 candidate worlds, retaining at
most four failures per round and jointly replanning all actions. Every complete
round emits a checkpoint; interruption retains that round or the ordinary move.
The reservation is capped at half the total budget for short calls; the total
14/20-second limits are unchanged. A deep comparison that runs out of its share
leaves the completed 40-world checkpoint for counterexample refinement.
The `counterexample_result` stores the deliberately biased stress scores
separately from `evaluation`. Each subsequent live decision replans normally.
This is an experiment for playtesting, not an established strength improvement.
Straight play, declarer play, and calls omitting the flag retain their behavior.

## Reproduce

Use the packet `run_capped.py` watchdog with fresh output directories for each
build or test. The standard build commands are in [walt-player](walt-player/README.md).

```sh
cargo test --locked --release --manifest-path walt/Cargo.toml -p walt-player -p walt --test contracts --test nello --test rules_exhaustive --test solver_hand_cache
cargo check --locked --manifest-path walt/Cargo.toml --workspace --all-targets
node walt/walt-player/nello-check.mjs /fresh/parity.json
node walt/walt-player/check.mjs /fresh/straight-parity.json
python3 -m unittest discover -s experiments/partnership -p test_nello.py
python3 experiments/partnership/nello_bench.py /fresh/mixed.json --panel mixed
python3 experiments/partnership/nello_bench.py /fresh/low.json --panel low
```

Commit the Rust source before running Plunge's `scripts/update-walt.py`; it
builds and imports the matched artifact and source manifest together.
