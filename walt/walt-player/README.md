# One deployed Walt, two hosts

Exploratory player engineering, not a new strength result. Initial phone target:
Pixel 9 / Chrome installed web app; newer iPhones are a secondary target.

`walt-player` owns the complete live decision sequence: independently reconstruct
the actor's own/public position, retain a legal fallback, complete an 8/2 L1
comparison, try default fixed L1 40/8, then optionally spend up to 500 ms on the
existing count-offer partner rollout. Total compute budget remains 14 seconds.
The bidder's opening lead instead requests the existing 160-world deeper L1
comparison with a 20-second budget. The inner model remains voidless. A
160-world opening or later inspection first retains a complete 40/8 comparison,
then tries 160-world L1 without a partner check. The partner review is defined
for the default 40/8 profile and is not reinterpreted at 160 worlds. All later
table play keeps the 40-world, 14-second profile and its selected difficulty's
normal partner-review behavior.

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
not a confidence-certified decision. Straight play now accepts targets 30–42;
any marks contract uses target 42. The partner continuation and its stopping
rule use that same target. Historical research specifications still default to 30.

## Regular auction

An `auction` call contains only `hand`, `seat`, `bid` (30–42) and `seed`, plus
the outer `worlds` (default 160) and `budget_ms` (default 20000). It compares all nine straight declarations
at that target through the same fixed L1/voidless solver. It needs only the best
opening value, so the solver can stop pricing a declaration at value 1 instead
of producing every opening move's score. Tested against full action vectors at
three contracts and seats; native and wasm prices agree.

The successive complete surveys use 4, 12, 40 and up to 160 worlds, with eight inner
worlds. Only a whole nine-declaration sweep replaces the previous survey. Each
completed survey emits a checkpoint. A partial sweep contributes no prices;
an entirely unpriced auction passes. Equal model prices use a public-seeded tie
choice. This is a small-sample, optimistic model, not calibrated table odds.

The table bids the cheapest legal raise when the best modeled make estimate is
at least 3/4, otherwise passes. It passes over its partner's standing bid. It
does not walk the bid amount upward: that old scheme overbid against a stronger
field. This threshold is an initial playing policy, not an empirical strength
result. Humans retain the full straight bid ladder. The winning AI remembers
its surveyed trump without another search, including across reloads. Each
auction decision has one 20-second wall budget; CPU throttling or host recovery
can add only the host's bounded shutdown margin.

### Independent declaration workers

Browser hosts may distribute declarations across ordinary Web Workers, each
with its own instance of this same WASM. No shared memory or threaded Rust build
is needed. Two workers are the initial phone default; one on a browser reporting
one logical processor. A worker reuses its instance within an auction, and all
workers are terminated when that auction finishes or is cancelled.

The shared JSON API adds two calls:

- `{"auction_price":{hand,seat,bid,seed},"decl":0,"worlds":4,"budget_ms":1000}`
  returns a `walt-auction-price-v1` receipt. This is exactly the serial auction's
  declaration evaluator, including sample generation and inner policy.
- `{"auction_merge":{hand,seat,bid,seed},"worlds":4,"receipts":[...]}` validates
  nine receipts and returns a normal auction survey. Receipt identity, sample
  size, inner budget, exact fractions and declaration coverage must match. Rows
  are sorted before the same seeded tiebreak. `worlds:0,receipts:[]` obtains the
  validated unpriced fallback.

The browser owns only scheduling and lifetime. It completes a whole 4-world
survey before beginning 12, then 40 and 160; it never combines fragments from different
rounds. All jobs share the original 20-second wall budget. Infrastructure
failures get one retry per job within that deadline; partial rounds are discarded.
Only a completed Rust merge replaces the checkpoint. Explicit cancellation
discards even that checkpoint. Job receipts identify inputs; they are not
cryptographic attestations of a remote worker's computation.

This splits independent declarations, not one policy solve into smaller world
sets. At the same completed sample size, serial and pooled results must agree
exactly. A faster host can reach a larger complete survey within the budget and
therefore legitimately choose a different declaration. `auction-check.mjs`
checks native/WASM/job/merge parity, order-independent ties, malformed or mixed
receipts and deadlines. Pool scheduling/lifetime tests live in Plunge. Receipts:
`/Users/jason/data/texas-42/auction-pool/`.

Regular bidding, points/marks scoring, original play scores and portable links
are supported. The existing Mac counterfactual gym comparison remains explicitly
scoped to bid 30; higher-bid flags can be saved and their moves re-inspected.
Validation: `auction-check.mjs`, `tests/contracts.rs`, Plunge's auction tests,
and the independent higher-contract replay fixtures. Raw receipts:
`/Users/jason/data/texas-42/regular-bidding/`.

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

## Validation of the first port

The focused Rust suites passed 31 tests; the Python suite passed 103 and the
Plunge suite 157. The browser smoke test completed a five-trick hand, retrieved
its original 40-world scores, completed a 160-world recheck (4.85 seconds on this
Mac's embedded browser), and recovered the original scores after reloading the
page. The ordinary 30-point make/set stopping rule ended that hand. The final
clock test expired inside both evaluator stages and retained a legal checkpoint
with no partial action values. These receipts establish tested behavior and
port conformance, not calibration or increased playing strength.
