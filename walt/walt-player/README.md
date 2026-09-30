# One Rust Walt, native and browser

`walt-table-v3` implements an observation-only response ladder in safe Rust. The
same `walt-player` crate serves the gym's JSON-lines transport and Plunge's WASM
worker. There is no C++ or Emscripten dependency in this release.

This is exploratory player engineering, not an equilibrium result or a new
proof tier. The backend is CPU/WASM; WebGPU execution remains future work.

## Profiles and contracts

The default Plunge profile is **L2, delta 1, 160 outer / 24 inner sampled deals**.
The existing difficulty controls select `[160]` for L1 or `[24,160]` for L2;
Think deeper raises the outer count to 350. Profiles list counts from inner to
outer. L1 responds to uniform random legal moves; L2 responds to that named L1
policy. Counts at delta 1 are exact integers. Randomness is addressed by the
information state, deal index and public continuation, independent of traversal.

All nine straight declarations, targets 30–42, every bidder seat, and own-suit
Nel-O use the same ladder. Nel-O has three active seats, a zero-trick objective,
and **four physical hidden hands**: the bidder's inactive partner retains seven
tiles in the sampler. Auctions and the empirical bid book remain unchanged.

The existing Nel-O defender counterexample review is retained with up to 4.2 s
reserved inside the overall 20 s decision budget. Its deliberately biased
witness mixture stays separate from ordinary sampled estimates. Its modeled
continuation is the existing counterexample procedure, not a claim that its
scores are the new ladder's calibrated probabilities.

## One observation boundary

```json
{"request":{"decl":3,"bid":30,"bidder":1,"seat":1,"seed":57,"hand":[0,1,2,3,4,5,6],"plays":[]},"worlds":160,"profile":[24,160],"partner":false,"budget_ms":20000}
```

`hand` is the acting seat's original seven tiles; `plays` is the complete flat
actor/tile history. Add `contract:"nello"`, declaration 8 and the actual mark
stake for Nel-O. Unknown fields, hidden hands, duplicate tiles, impossible own
plays, inconsistent public voids, unsupported contracts and out-of-turn requests
are rejected. The seed accepts a u64 or its decimal string representation.

Every lower policy receives only its own remaining hand and the public record,
then independently samples its void-consistent support. This is uniform support
sampling, **not a Bayesian posterior conditioned on policy likelihoods**. No
physical deal or enclosing search fiber enters that lower-policy query.
All fibers at one public continuation share a focal action; observations sum,
and a focal decision maximizes/minimizes their combined value. Full actor history
is in the cache key. Cache context fixes the contract, seed and sample prefix.

The sampler counts and unranks assignments. Small assignment tables contain only
abstract allowed-seat patterns and capacities, capped at 8192 entries per thread.
Other speedups are exact incumbent bounds, reusable frame buffers, same-hand
query deduplication and a 50,000-entry per-job policy cache. The singleton L1
shortcut applies only after a public branch contains one sampled world.

## Jobs, checkpoints and future batching

`ladder::Job::new` validates an observation and profile. `step` finishes one root
alternative, retaining lower-policy caches across calls. Independent jobs can be
interleaved. It returns a decision only after **all** alternatives finish; a failed
job cannot resume or expose partial scores. Tie-break: lowest tile ID.

The deployment wrapper first keeps a legal fallback, then tries L1(8), L1 at the
requested outer sample count, and the requested higher profile. Only complete
comparisons replace the retained decision. Receipts name both the requested and
completed profile. A timeout is an incomplete computation, never a strength loss.
Cancellation discards the worker, including checkpoints for the abandoned turn.

This root-job boundary is an initial amortization interface. Lower search is
still recursive. A WebGPU executor still needs explicit lower-query continuations,
deduplicated queues across jobs and measured CPU/GPU dispatch thresholds. No GPU
or physical-phone performance claim is made here.

## Hosts and reproducibility

- `walt-table` reads JSON lines and emits checkpoint/result envelopes.
- `experiments/partnership/table_player.py::decide_ladder` supplies native process
  lifetime and independent rule checks. The Plunge bridge exposes `walt-l1` and
  `walt-l2`; receipt identity includes the profile and binary hash.
- Plunge's existing worker imports only `walt_host.now_us` and
  `walt_host.checkpoint`. The existing importer builds a fresh, locked,
  `wasm32-unknown-unknown` artifact from committed source, without native flags,
  and pins source/compiler/build/WASM identities in its manifest.

The old `decide_legacy` procedure and explicit `legacy:true` wire flag are retained
for reproducible historical research. Existing v2 gym configurations and the
full-hand bid-book host call that procedure explicitly; historical datasets must
not be silently relabeled as v3. This is not a new Plunge setting.

```sh
cd walt
cargo test --release -p walt-player --lib --no-default-features --features cpu-speedups
cargo clippy -p walt-player --lib --no-default-features --features cpu-speedups -- -D warnings
cargo build --release -p walt-player --bin walt-table --no-default-features --features cpu-speedups
cargo build --release -p walt-player --lib --no-default-features --features cpu-speedups --target wasm32-unknown-unknown
node walt-player/tests/ladder-wasm.mjs
```

Tests retain 18 C++ reference cases (L1–L4, including 160/24), their source hash,
all bidder rotations, and 40 own-view Nel-O fixtures. Native/WASM parity checks
exact actions, scores and public state. An independent unpruned traversal pins
a strategy-fusion witness: a shared decision makes **6/8**, while unlawful
per-world decisions make **8/8**. Additional tests cover Nel-O capacity/turn order,
authoritative suit algebra, field-prefix invariance, interleaving and deadlines.
Plunge checks the actual WASM against its independent game engine at 957 public
positions across 80 hands (all declarations, bidders and target extremes).

The source tree's broad `walt/ci/check.sh` was attempted during this release; it
stops at pre-existing workspace formatting differences, independently reproduced
from base commit `ccf14870`. Scoped player tests and clippy pass. This is not a
claim that the full workspace gate is green.
