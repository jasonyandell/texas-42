# Ruby double-lead research archive

Exploratory observations from 2026-09-21/22, preserved in git on 2026-09-25.
Start with [LADDER.md](LADDER.md); [the wiki](../../../wiki/walt-nello.md)
provides the synthesis and the later counterexample experiment.

The raw panels, individual 100k cases, original reports, bounded run logs,
exact rational summaries, figures and producers are preserved without rewriting
historical outcomes. Generated research WASM files are intentionally excluded
from git. Their SHA-256 digests and the two modified source-file digests are in
each budget's `summary*.json`; the original local binaries remain in the
`nello-player` research worktree. No current browser artifact uses these raised
sample/time limits.

To verify the recorded ladder without new search:

```sh
python3 walt/probes/nello-ruby-doubles-2026-09-21/compare-panels.py
```

It checks all sixteen completed cases at each budget and regenerates
`ladder-summary.json` using exact rational arithmetic. On 2026-09-25 the output
was byte-identical to the original summary.

To rerun a historical large-budget panel, start an isolated checkout at
`e4c549e2fd533c942f39125bf3134ac707c4e638`, whose engine matches the deployed
`dba963fe` source. The `2000/adapter-cap.patch`, `10000/adapter-cap.patch`, and
`100000/adapter-cap-time.patch` are exact replacement records (their `@@`
markers omit line numbers, so they are not directly `git apply` patches).
Apply the named replacements to the two files under `walt/`, then compare their
SHA-256 values to that budget's summary (all six reproduced from the pinned
source on 2026-09-25) before building the release WASM with
`--no-default-features --features cpu-speedups --target wasm32-unknown-unknown`.
Use the budget's `panel.mjs`, or `100000/case.mjs` with explicit output paths,
and the packet `run_capped.py` watchdog. Follow the original reports for time
limits and parity comparisons; never overwrite these retained evidence files.
Copy the producers and fixtures to a fresh output tree before running scripts
that write beside themselves. A rebuilt artifact may have a different binary
hash because the compiler environment is separate evidence.

The 100k runs used fresh Node processes to release high-water memory between
cases. Native and browser timing, optimized in-sample scores, fixed-policy
holdout rates and revealed-deal diagnostics are distinct measurements.
