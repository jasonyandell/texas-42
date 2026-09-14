# One deployed Walt, two hosts

Exploratory player engineering, not a new strength result. Initial phone target:
Pixel 9 / Chrome installed web app; newer iPhones are a secondary target.

`walt-player` owns the complete live decision sequence: independently reconstruct
the actor's own/public position, retain a legal fallback, complete an 8/2 L1
comparison, try default fixed L1 40/8, then optionally spend up to 500 ms on the
existing count-offer partner rollout. Total compute budget remains 14 seconds.
The inner model remains voidless. Neither the policy nor its strength claim was
changed to make the port. A 160-world inspection uses L1 without a partner check.

Both hosts call `handle` with a JSON object containing `request` (exactly the
seven existing fields), `worlds`, `partner`, and optionally `budget_ms`.
Unknown fields, hidden hands and unsupported contracts are rejected. The Rust
wire adapter is shared with the existing `partnership` research binary.
Each completed stage emits a full checkpoint. A host that exhausts its outer
deadline keeps the latest complete checkpoint; it never ranks partially
evaluated actions. Cancelling because the user left a position discards the call.

- Native: `walt-table` emits newline-delimited checkpoint/result envelopes.
  `experiments/partnership/table_player.py` supplies process lifetime and checks
  the answer against the independent Python rules. Plunge's bridge calls this
  adapter. Historical `player.py` remains a reproducer for earlier experiments
  and selectable research profiles, including the archived phone comparand.
- Browser: a dedicated Web Worker loads the same crate as WebAssembly. Only two
  host imports are allowed: monotonic microseconds and copying a checkpoint.
  The clock is installed by the adapter before solving. Core code remains safe
  Rust; the two foreign calls are isolated in the ABI module. A missing browser
  clock now fails explicitly instead of silently disabling deadlines.

There is no JS translation of L1, the partner gate, selection, or fallbacks.
The same complete sample produces the same values. A clock-limited partner
prefix can differ between devices; it remains an explicitly fallible heuristic,
not a confidence-certified decision. The existing own-hand declaration chooser
is still a separate, inexpensive table choice. All contracts are assigned 30.

## Build and check

Run builds/tests through the packet watchdog with a fresh output directory:

```sh
cargo build --locked --release --manifest-path walt/Cargo.toml -p walt-player --bin walt-table
cargo build --locked --release --manifest-path walt/Cargo.toml -p walt-player --lib --no-default-features --target wasm32-unknown-unknown
node walt/walt-player/check.mjs /path/to/parity.json
```

The check compares all nine declarations of a fixed opening hand plus a late
position, native versus WebAssembly, including exact rational option vectors
and the completed fallback. It compares a full 64-world partner prefix against
the native rollout instrument, tests malformed inputs, and advances a fake
browser clock to force a deadline without relying on machine speed.

The first audit passed all ten L1 comparisons and the 64-world paired partner
comparison. Native and Node-hosted wasm opening decisions on this Mac were
roughly 0.8–2.2 seconds; these are host measurements, not Pixel measurements.
The 500 ms native review completed only a prefix in that run, as expected.
Raw receipts: `/Users/jason/data/texas-42/phone-v2/`.

## Ship an iteration

In the Plunge checkout, `python3 scripts/update-walt.py /path/to/texas-42`
builds this crate under the watchdog, copies the wasm, and records source and
asset hashes. Commit the source first, then import. Plunge's build verifies the
asset hash and allowed host imports; CI publishes the site and hashed wasm
together. The phone offers a reload on a new deployment.

Plunge stores original decision receipts in IndexedDB on the device. A finished
hand's observation link includes the replay, selected move, public seed, note,
alternative and original receipt when available. The Mac can import that link
into the gym. Imported links are examiner data and never become live chooser
inputs. Live server logging and full gym comparisons remain Mac facilities.
