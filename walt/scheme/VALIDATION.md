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
