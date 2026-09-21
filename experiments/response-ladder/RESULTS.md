# Native GPU player checkpoint — 2026-09-20

The GPU player is now wired through real modeled-field play from trick 1. GPU
lanes execute full-history policies and physical continuations, pausing for
batched exact CPU field queries. The CPU controller retains completed lower
witnesses and refines lawful SUM/MAX bounds. Seven new modeled-epoch tests pass,
including 1,480 full traces, adaptive policy overrides, canonical field state,
cancelled device submissions and preservation of earlier completed batches.
The previous core, adapter, rollout and portable CPU WASM checks also pass.

The [actual-current-CPU H2H](results/gpu-current-h2h-v1/REPORT.md) is complete:
72 paired deals, 144 games, 4,032 independently replayed legal plays, no reserve
fallbacks on either side. The GPU player won **4**, lost **5** and tied **63**.
Mean partnership thinking time was **358.83ms GPU versus 69.16ms CPU**—5.19×
slower. The GPU received 100ms per move while the current CPU retained its
normal 14,000ms allowance. This is not an equal-time strength test; there is
no observed quality gain and no basis for production replacement.

GPU computation was active throughout play: 5,663 completed modeled-field
batches, 945,480 completed policy/scenario lanes, and 1,007 completely priced
response reports. It is no longer a standalone Dice-only primitive. The current
CPU still outperforms this implementation end to end. Its landed acceleration
remains enabled in `/Users/jason/code/texas-42-partnership-launch` at `f1a0fb04`.

The [implementation note](docs/GPU-PLAYER-IMPLEMENTATION.md) records the exact
mathematical boundary, review, validation and remaining performance hypotheses.
Native M5 execution and portable CPU compilation are verified; phone GPU
plumbing and performance are not. `GPU-CHECKPOINT.json` pins this implementation;
the older `CHECKPOINT.json` and the following historical section describe the
first prototype only.

---

## Historical first implementation checkpoint

The remainder of this file is the earlier prototype record, before modeled GPU
epochs were connected. Its "current" and "next" statements are historical.

The production CPU work is complete in `/Users/jason/code/texas-42-partnership-launch` at `f1a0fb04`, with existing experiment binaries rebuilt. Against that branch's preceding `5ab08bbc` implementation under matched compiler settings, the 12-game panel preserves every exact root vector and choice and reports a 13.13-fold median paired speedup (median complete game 1.830s to 0.143s). This is the accumulated implementation gain, not the earlier roughly 7% final increment. See the inherited [CPU guide](../../walt/CPU-SPEEDUPS.md).

This branch adds the first runnable response-ladder stage and keeps it outside production defaults. It starts from the landed CPU branch. The new outer controller uses full public-history information buckets, total extracted policies, completed lower witnesses, valid upper bounds, and explicit interruption. Its lower-rung field remains frozen v34. GPU integration currently covers a separately tested historical-Dice rollout primitive.

## What passed

- One controller selection unit test, 11 core contract tests, two adapter tests, and four CPU rollout tests. Completed exact vectors and extracted-policy repricing agree with frozen Walt across all nine declarations and partial-trick offsets.
- Five GPU rollout tests. 16,512 complete continuations, 85,680 full-family payoff lanes and 672 directed arithmetic vectors match their CPU authorities. Every matrix in the 18-case timing panel also matches both scalar implementations.
- Portable CPU library check for `wasm32-unknown-unknown`; no phone runtime claim.
- Twenty-four complete mirrored games, 672 legal plays, independently replayed by the Python referee. Both sides received equal time banks with all fallbacks retained.

## What was measured

The [GPU report](results/gpu-rollout-v1/REPORT.md) records a useful crossover. At seven tiles and 40 scenarios, the complete 5,040-priority family took median **2.895ms GPU versus 25.765ms compact CPU**, about 8.9-fold. The 14-policy pool took **0.698ms GPU versus 0.093ms CPU**. Device/pipeline initialization was **103.786ms**, separate from those per-call timings. These results support a selective CPU/GPU split; they do not measure full L2 latency or establish that full enumeration is worth doing at every nested query.

The [player pilot](results/player-smoke-v1/summary.json) uses 12 pinned new deals spanning pip, doubles and no trump, all four bidder seats, and swapped partnerships. Median mixed-player game time was **164.60ms** under a 500ms combined allowance. The anytime candidate returned 156 completely priced decisions, with 82 canonical root certificates and 18 reserve fallbacks; the exact serial v34 comparator had 68 deadline fallbacks. These fallback counts include all played decisions, and the sides need not encounter identical numbers of nonforced states.

The candidate won three paired comparisons, lost two and tied seven. This is a small descriptive pilot. Its comparator is the frozen **serial exact solver plus canonical reserve**, not the production staged player. It neither establishes stronger play nor satisfies the exact full-L2 100ms goal. No formal or empirical quality claim is being promoted from this checkpoint.

## Next implementation boundary

The working controller currently reprices policies on CPU and recomputes completed horizon passes. The next structural work is retaining useful frontiers and batching ready lower-rung field queries, so broad GPU jobs emerge from a real information-consistent response. The new GPU kernel does not yet evaluate modeled L0/L1 fields or drive the full player. Small jobs should stay on CPU until a measured crossover justifies dispatch.

Source and binary identities, the pinned game protocol, raw timing samples and complete receipts are retained with this checkpoint. `tools/prepare_reference.py` verifies and unpacks the included immutable source archive before building on a fresh checkout. The Rust source here is byte-identical to the code used for the final pilot; only the independent referee import was made local for portability.
