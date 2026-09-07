# Scheme expressive runtime v1 — validation record

2026-09-06, `codex/partnership-launch`. Exploratory implementation evidence.
Source scope: the new `walt::scheme` module, `scheme` binary, examples, and
documentation. Rules, support mechanics, solver behavior, and mathematical
source packages were not edited.

Completed checks (from `walt/`):

```sh
cargo test -p walt --test scheme
cargo clippy -p walt --lib --bin scheme --test scheme -- -D warnings
cargo check -p walt --all-targets
cargo fmt --all -- --check
cargo check -p walt --lib --no-default-features --target wasm32-unknown-unknown
```

All passed. The focused test suite contains **19 tests**, including direct
relation comparisons on all 90 worlds under each of the nine declarations,
equality patterns, duplicate branches, weighted measures, certainty, selectors,
conditioning, counterexamples, public residue, and command-line refusal paths.
The WASM check is compilation of the library, not WASM execution/parity evidence.

The receipt CLI also completed a manual query at hand 0, trick 6, seat 0:
six legal worlds, partner holds the remaining count tile (4-1) in two, event
probability 1/3. Conditioning on that event leaves two worlds. These results
are independently pinned by the CLI integration test.

The full Rust CI was deliberately not run under this session's focused-check
waiver. No large census, player-strength match, general compression experiment,
or compact-transducer proof was performed. Exact evaluation is of a declared
finite physical-world measure. Query expression does not certify that a
hidden event is observable or that a described partnership move is optimal.

## 2026-09-07 focused extension gates

The finite dynamics, executable policy, policy export/replay, and sampled
policy-search extensions add these focused gates:

```sh
cargo test -p walt --test scheme_dynamics
cargo test -p walt --test scheme_policy
cargo test -p walt --test policy_search
```

They cover typed predecessor legality, exact support and physical-belief
pushforward, viewer interventions, rigid transport versus fresh evaluation,
hindsight preimages, controller persistence and refusal atomicity, policy text
roundtrips and independent replay, accumulated-sample persistence versus fresh
solution, complete donor-alternative composition, duplicate sample weight,
cache scoping, and held-out replay. Final test counts and broader validation are
recorded in the [experiment report](../../experiments/partnership/campaigns/policy-synthesis-v1/RESULTS.md)
and its verification receipt. The completed gates passed 46 focused Rust tests
(including the existing 19 Scheme tests and two request-parser tests), 12 Python
runner tests, strict native lint checks, and 222 independent full-game rules
replays. A real interruption/resume check preserved committed seed bytes and
finished all six seeds without leaving native workers running.
