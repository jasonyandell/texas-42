# Texas 42 partnership experiment — start here

Version 0.1 · Prepared 2026-09-06 for Jason's local Codex session

## Launch prompt

Paste the following into the Codex session running on the Mac, with this packet available locally. Replace the packet path if necessary.

> Begin the Texas 42 partnership experiment described in this packet. Read `EXPERIMENT-BRIEF.md`, then consult the two supplied math notes and the relevant repository entry points. This is my go to begin local implementation and experiments; do not return only a plan or ask me to reconfirm the agreed scope.
>
> Work in a new git worktree of my local `jasonyandell/texas-42` repository. The hardware is an M5 Max. The target is a lawful, partnership-aware player usable from trick 1 onward, with at most one minute of computation per trick. Every experiment or benchmark has a HARD five-minute wall-time cap, including any child work it launches. Skip the full Rust CI for this experiment; use focused checks. Backward compatibility is not a requirement.
>
> Walt 1's actual phone configuration is the reference. Keep sampling reasonably comparable so that merely increasing L1's samples does not count as closing the partnership gap. L2 is promising but optional; CPU, GPU, and other lawful approaches are all eligible. A fast GPU Walt 2 is an acceptable route. An L2 that loses is useful knowledge, not a reason to hide or reinterpret the result. Lawfulness and information consistency are mandatory; proving global optimality is not required.
>
> Make reasonable implementation choices and proceed autonomously. Start with small, informative runs; enforce timeouts externally, record configuration and results, and leave a runnable player or a clear account of the remaining obstacle. Keep me informed as you learn. The brief contains the agreed goal, source map, suggested first experiments, and reporting requirements.

## Packet contents

- `EXPERIMENT-BRIEF.md`: the complete task contract, starting directions, code map, and success criteria.
- `math/TEXAS42-UNIFIED-REVIEW-v0.1.md`: the supplied mathematical core and audit, unchanged.
- `math/TEXAS42-IMPROVISATION-v0.1.md`: the supplied exact-integration and information-price proposal, unchanged.
- `tools/run_capped.py`: a standard-library POSIX watchdog for macOS/Linux, with process-group termination and a JSON run record.
- `RESULTS-TEMPLATE.md`: a compact experiment record and end-of-session report scaffold.
- `MANIFEST.sha256`: checksums of the packet contents.

The packet carries both mathematical texts. It does not carry their separately referenced verification programs, ZIP packages, research datasets, or the repository itself. Those missing companions are explicitly not assumed to have been reproduced.

## The goal in one sentence

Build and test a tractable Texas 42 player that thinks usefully about its partner, acts lawfully from trick 1, and fits the M5 Max time budget, using Walt 1 as a practical strength reference.

The initial request mentioned one minute for a whole game. Jason subsequently replaced that with **one minute per trick**. The corrected target governs everywhere in this packet. The five-minute experiment cap remains independent of that player budget.

## Using the timeout runner

From the experimental worktree, set `PACKET_DIR` to this extracted packet's absolute path. Use a unique output directory for every run:

```sh
python3 "$PACKET_DIR/tools/run_capped.py" \
  --seconds 295 \
  --output-dir experiments/partnership/runs/001-baseline \
  -- ./path/to/a/verified-local-benchmark --its-supported-options
```

The command above is a shape, not the name of an existing repository benchmark. Locate an actual executable and its supported options first.

The runner defaults to 295 seconds and refuses larger allowances, reserving five seconds below the user's absolute 300-second experiment ceiling. It sends SIGKILL to the child process group when the allowance expires and also cleans up remaining group members after the command exits. Logs and `run.json` are retained. It does not invoke a shell, detach jobs, install dependencies, or escalate permissions.

The player must meet its own trick budget and return legal moves in time. Killing a benchmark at its deadline is an experiment outcome, not a playable fallback.

Keep experiment children in the foreground process group; do not daemonize or submit detached jobs. The runner controls local process groups, not independently submitted services or external jobs. OS scheduling prevents a mathematical real-time guarantee; the five-second reserve is operational headroom, and an overrun must be reported rather than called compliant.

Preparation checks: both math files were compared byte-for-byte to Jason's attachments. The watchdog passed short Linux checks for normal execution, nonzero exit, timeout of a parent and child, cleanup after an early parent exit, interruption, and refusal of oversized allowances. It has not been run on the M5 Max; do a short local smoke before relying on it. No Texas 42 benchmark was run while preparing this packet.
