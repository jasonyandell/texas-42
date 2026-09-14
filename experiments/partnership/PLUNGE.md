# The Mac sunshine table

Play a bid-30 hand with the measured native player, save a particular decision,
then compare its alternatives in the continuation-selectable gym. This is the
first human-play loop for the [sunshine goals](SUNSHINE-NOTES.md).

## Start and play

The companion checkout is `/Users/jason/code/plunge-sunshine` on
`codex/sunshine-table`, based on `jasonyandell/plunge` main `122ea7a5`.
The implemented companion is commit `ac7e65a20f213823bfb926ad62e5b3c5443bbb4c`.
The research checkout is `/Users/jason/code/texas-42-partnership-launch`.
From anywhere:

```sh
python3 /Users/jason/code/texas-42-partnership-launch/experiments/partnership/play_plunge.py
```

Open <http://127.0.0.1:4244>. Choose **L1 + partner check** or **L1**, then
**Deal me in**. That choice controls all three computer seats. You play seat
zero with Gran as partner. Each hand has a fixed 30 bid; the first bidder
rotates with the shaker. The bidder chooses trump: you when it is your turn,
Plunge's existing own-information hard player for a computer bidder.
This is practice under assigned contracts, not a voluntary bidding evaluation.
The table ends a hand when the bid is made or set, as ordinary Plunge does.

The launcher keeps the table and native service together. Ctrl-C stops both
and any comparison; completed records remain. Restart the same command to
continue. **Resume your game** restores the browser save. Native failures
pause play with a Retry button; they do not substitute a different browser AI.
An identical request under the same recorded implementation reuses its saved
answer. Restart the launcher and reload the table after changing source code.

Optional launcher arguments: `--plunge PATH`, `--data PATH`, `--port 4244`,
`--bridge-port 4245`. Both services bind only to 127.0.0.1. An occupied port
produces an error rather than stopping someone else's service. The long-lived
table is a development service; its background gym jobs remain separately
bounded experiments. Local pages do not install Plunge's offline service
worker, and `/api/` responses bypass offline caching everywhere.

If setting up a new checkout, run `npm ci` in the Plunge checkout and build
`partnership`, `partner_rollout`, and `partnership_gym` in the research checkout:

```sh
python3 experiments/partnership/packet/texas42-partnership-launch-v0.1/tools/run_capped.py --seconds 295 --output-dir /tmp/sunshine-native-build-01 -- cargo build --release --manifest-path walt/Cargo.toml -p walt --bin partnership --bin partner_rollout --bin partnership_gym
```

Choose a new watchdog output directory for each workload. No wasm rebuild is
needed for this table. The measured native presets remain unchanged.

## Inspect any play

After a hand, choose **See how it went**, then tap a domino. On the Mac,
history sits beside the selected move's stats; narrow screens scroll to the
selected move. The view shows trump, the led suit, the actor's remaining hand
and legal choices. Original saved L1 option scores show **make** for a member
of the declaring team and **set** for a defender. These are sampled model
estimates. A forced move has an explanation, not an invented probability.
If the partner check changed L1's choice, the L1 scores are explicitly labeled
as preceding that check.

**Look closer · 160 worlds** runs the current native L1 default on that same
own/public position and seed. Human plays and shared hands without receipts
offer **Ask Walt · 40 worlds** too. The new result appears below the original;
it does not rewrite the decision or its evidence. Completed estimates survive
restart in `estimates/`, keyed by request, player settings and implementation.
Frontend presentation changes do not invalidate them. Historical receipts
remain available even when the current player implementation changes.

Inspection has the existing 14-second decision budget and a separate worker
from live play. Only one inspection runs at once; a concurrent request receives
a retry message. A failed large comparison reports the actual smaller fallback
sample if available, and remains retryable. There is no gym support-size cap
on this sampled comparison, so opening leads are inspectable too.

The [stats validation record](campaigns/sunshine-review-v1/RESULTS.md) includes
the saved double-six discard, forced-trump example, cache and evidence checks.

## Bring a move back to the gym

After a hand, choose **See how it went**, then tap a domino in the trick
history. The panel shows the computer's original decision receipt when one
exists. Add a note and optionally select another legal play. **Save this move
for the gym** stores the hand, the exact position before that play, and the
original receipt together. Human moves can be flagged too; they are explicitly
shown without a native decision receipt.

**Copy flagged-move link** reopens that selected move on this Mac, with its
note and receipt. **Share this hand** produces the existing portable Plunge
hand link; that carries the deal and actions, without the local note or native
receipt. Flagged links require the local server and its saved data. Opening a
link does not overwrite your ongoing game.

**Compare every legal play** offers three named continuation models:

| Selection | Focal player's later turns | Partner | Opponents |
|---|---|---|---|
| L1 at every seat | L1 default | L1 default | L1 default |
| Partner uses L2 | L1 default | L2 Partner default | L1 default |
| L1 + partner check at every seat | L1 + partner check | L1 + partner check | L1 + partner check |

The focal player is the actor at the flagged decision, not necessarily you.
Every root action is tried over **all mechanically compatible hands**, with
the same named players making later decisions from their own hands and public
history. This is a census under a uniform mechanical belief and frozen
continuations. It is not a prediction calibrated to a human's behavior, a
perfect-information solve, or a claim about optimal 42.

The display scores your focal team's make/set frequency. All equal frequencies
tie; excess count does not break ties. The present full-comparison limit is
400 compatible hands. Larger positions are still saved as gym inputs, with an
explicit outside-scope result. There is no hidden early-game enumeration.

One comparison runs at a time, using four trajectory workers with two native
threads each. Each slice has a 240-second working allowance inside an external
295-second watchdog. Pause at any time; complete trajectories and exact-input
decisions persist. Resume reuses them. An incomplete slice reports partial,
never a full-census score. Changing the continuation, actual player settings,
or evaluation implementation selects a different cache; UI presentation does
not invalidate analysis values.

## Records and reproducibility

Default data directory: `/Users/jason/data/texas-42/plunge-sunshine/`.

- `decisions/`: requests, preset parameters, implementation identity, original
  responses and timing, with a content checksum. Saved before replying to play.
- `flags/`: independently replayed finished hand, selected move, note,
  alternative and original receipt. Full hands here are examiner data.
- `gym-inputs/`: actor-only request plus a source annotation for each flag.
- `analyses/`: pinned evaluation manifest, resumable decisions/trajectories,
  complete audited gym key, and display result. The runner identity excludes
  presentation but includes actual continuation configurations and code.
- `server/`: table and bridge logs. Each comparison has its own `runs/` with
  watchdog status, timestamps and stdout/stderr.

`/api/decide` accepts exactly the seven own/public request fields plus the
selected preset and game/hand identifiers. Full-deal fields are rejected.
The independent Python importer validates ownership, turn order, following,
trick winners and the stopping point against the exported Plunge record.
Original decisions are retrieved, never reconstructed from a new solve.

The bridge lives in `plunge_bridge.py`; `plunge_io.py` imports finished hands;
`plunge_analysis.py` runs the existing deployed gym; `play_plunge.py` supervises
the two local services. `plunge_check.py` audits a live receipt and real native
pause/resume. [The study and integration results](campaigns/sunshine-playable-v1/RESULTS.md)
include the first played examples and the final checks.
