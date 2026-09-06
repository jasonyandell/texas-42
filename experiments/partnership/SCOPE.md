# Partnership launch, 2026-09-06

Exploratory experiment authorized by Jason's launch request and attached prompt.
Base: `9d6a5a2` on the selected local `walt-gran` checkout; original checkout
was clean. Branch: `codex/partnership-launch`. Origin:
`https://github.com/jasonyandell/texas-42.git`.

The unzipped packet is preserved byte-for-byte in `packet/`; all six manifest
entries passed SHA-256 verification before its first commit (`d840071`). The
packet's historical results and mathematical companion-verifier claims are
source material, not independently reproduced results.

Local implementation and small experiments are authorized. Every experiment,
including its children, runs beneath the packet's process-group watchdog with
at most 295 seconds, reserving five seconds below the hard 300-second ceiling.
No detached jobs. Full Rust CI is deliberately waived for this experiment;
focused checks replace it. No merge, publication, or deployment is requested.

Player target: at most 60 seconds combined computation across the four plays
of each trick, from trick 1. The native wrapper allocates at most 14 seconds
per decision including validation, fallback, worker startup, and cleanup.
Human input time is excluded. Build costs are reported separately.

Hardware inspected: Apple M5 Max, 48 GiB RAM, 18 CPU cores. Rust/cargo 1.95.0.
The player remains below all foundational evidentiary tiers. No global
optimality or strength claim follows from a legal, fast decision.

First route: fixed-sample, information-grouped focal best response with only
the modeled partner upgraded to a level-1 mind. The weaker-partner ablation
uses the same samples, seed, opponents, tie rule, and focal search. Literal
phone-reference play uses the archived local Plunge WASM and its race setting;
it is a separate strength comparison. Contracts and declarations are fixed and
matched, so this experiment assesses play, not an auction improvement.
