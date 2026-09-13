# Optional partnership investigation around L1

2026-09-13. Exploratory candidate. The
[protocol](campaigns/sunshine-partner-count-v1/PROTOCOL.md) and
[results](campaigns/sunshine-partner-count-v1/RESULTS.md)
define its evidence and scope. This implements the first bounded repair proposed
in [Sunshine Notes](SUNSHINE-NOTES.md).

The [subsequent replay of the exact gym misses](campaigns/sunshine-gym-replay-v1/RESULTS.md)
found that two offers still help under deployed L1 continuations, while four
reverse and hurt. The reviewer changes five choices but times out on the
strongest useful one. Its teacher-relative improvement therefore does not
establish a successful repair under the deployed continuation. This remains an
experimental preset; the [replay instrument](GYM-REPLAY.md) now measures that gap.

## Behavior

`l1-partner-count-review` is default fixed L1 with an optional review after its
completed decision. Its existing 40/8 samples, inner belief, and original
fallback are unchanged. Review is a separate configuration coordinate, not a
new modeled-mind level. Existing presets default to `review="off"`.

When the declaring-side player has at most three own dominoes, the contract is
unresolved, and it could give count to its currently winning partner, the public
detector checks whether L1 overlooked every such offer. If so, the maintained
Scheme query supplies alternatives for an investigation. The Python detector
is a specialization for cheap routing; the native query must agree before any
changed decision is accepted.

Only the baseline and offer actions are compared. Future focal choices remain
unrestricted and lawful. All alternatives share the same complete uniform
mechanical root fiber and deterministic GymField: an L1-40/8 partner and L0-8
opponents. Original world identities persist through observation-conditioned
branches. This is a declared model of possible continuations, not the actual
hidden deal or a guarantee about live L1 opponents and future deployed choices.

A strictly better modeled make count can replace the baseline; a tie preserves
it. The additional wall allowance is at most 250 ms and fits inside the existing
14-second overall deadline. The native work cap is 100,000 search nodes and the
root cap is 400 worlds. Refusal, timeout, or an invalid/incomplete comparison
retains the completed L1 move. If L1 itself fails, its original fallback path
runs without review.

There is no positive count bonus and no instruction to always feed partner.
The native review reports its compared values, field identity, support size,
query offers, search nodes, field calls, and elapsed time. The wrapper retains
the original L1 evaluation and a separate review phase, including failed cost.

## Run

Build both workers from the workspace's `walt/` directory:

```sh
cargo build --release -p walt --bin partnership --bin partner_review
```

The same seven-field public/own-hand JSON request works through the player:

```sh
python3 experiments/partnership/player.py --mode baseline --review partner-count
```

The native review worker is also callable with those seven fields in the
existing text wire format on standard input:

```sh
walt/target/release/partner_review --baseline TILE_ID
```

Its parent owns the process deadline; direct native invocation alone does not
provide the player wrapper's hard wall cap. In a match, use the named preset
through `match.py` and the existing watchdog/pool, as recorded in the protocol.

## Engineering boundaries

- `policy_search::Search::compare_root` restricts only the queried first actions.
  It shares completed unrestricted continuation memo entries safely with normal
  search. It returns no partial action comparison on budget refusal.
- `policy_search::partner_review` owns the query, scope, field, caps, and strict
  improvement rule. It accepts a reconstructed own/public root and baseline.
- `partner_review.py` supplies the cheap detector and validates the completed
  native response. `player.py` owns the overall deadline and backstop.
- Campaign and gym manifests pin the review worker and wrapper. Reports keep
  selected diagnostic cases, fresh full games, and conditional hidden-hand
  experiments separate.

The conditional experiment in `sunshine_worlds.py` persists the first decision
for each exact own/public request and player configuration. It studies a frozen
realization of the budgeted players across hidden completions. Its cached replay
speed is not a live-player latency result; the full-game arena measures latency.
