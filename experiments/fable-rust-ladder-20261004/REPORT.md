# Replacing Walt's inner level-0 mind with a net, judged by paired win rate

**Exploratory, 2026-10-04.** Claude Fable 5.1 (`claude-fable-5-1`), same session.
Jason's brief: compress the pinned lower-level Walt computation into a net in
Rust, use the whole machine, judge by head-to-head outcomes rather than
agreement, log what Walt actually computes and train on it. Everything here is
new under this directory; the three prior experiments are untouched. All runs
under the 295 s process-group cap; no cloud, pushes, installs or exports.

## Answer

Walt's inner level-0 mind was lifted out of the pinned solver as a
vector-returning seam (fork, +72 lines, production source untouched), its call
stream logged from real self-play, labeled at 160 inner worlds, and distilled
into dense nets that then play *inside* the unchanged outer search in Rust.
Measured by paired mirrored games against the pinned 40/8 baseline on 4,096
fresh deals per arm (+ means the hybrid won a deal the native lost in the
mirrored seating):

| Inner policy inside the outer search | Val RMSE vs k/160 | Paired advantage [95%] | Outer decision µs (hybrid / native, same run) |
|---|---:|---:|---:|
| no net (harness control: native vs native) | | 0 / 0 flips in 4,096 deals | |
| lowest legal tile, no evaluation | | −.086 [−.102, −.070] | 0.5 ms / 4.1 ms |
| one layer 32 (28k params) | .172 | −.076 [−.092, −.061] | 1.2 ms / 4.2 ms |
| one layer 64 | .160 | −.064 [−.080, −.049] | 1.2 ms / 4.1 ms |
| one layer 128 | .144 | −.073 [−.089, −.057] | 1.7 ms / 4.2 ms |
| one layer 256 | .135 | −.061 [−.076, −.045] | 3.1 ms / 4.5 ms |
| one layer 256, decl-aware encoding (v2) | .124 | −.042 [−.057, −.027] | 3.1 ms / 4.6 ms |
| two layers 256→256 | .092 | **−.015 [−.030, +.000]** | 6.4 ms / 4.1 ms |
| two layers 512→512 (2,048 deals) | .087 | −.012 [−.033, +.010] | 24 ms / 6.2 ms |
| v2 two layers 128→128 | .101 | **−.014 [−.029, +.001]** | 4.0 ms / 4.5 ms |
| v2 two layers 256→256 | .090 | −.021 [−.036, −.006] | 7.3 ms / 4.4 ms |
| exact 64-world inner mind (96 deals, slow) | ceiling | +.031 [−.052, +.115] | 324 ms / 4.5 ms |

Three findings:

1. **Outcome fidelity tracks vector fidelity, and only the deeper nets get
   close.** Every one-layer net, from 28k to 228k parameters, loses 6 to 8
   points of paired deals, barely better than the "no evaluation at all"
   control at −8.6. Two hidden layers at RMSE .09 bring the hybrid within 1.5
   points of parity. A 1,024-deal run had earlier shown the one-layer 256 net
   at parity; 4,096 deals on a fresh seed say −.061. Decisions in 42 are
   sparse and sharp; the noise floor on 1,024 paired deals is about ±.03.
2. **The cheap nets are cheap, and one is nearly good enough.** The one-layer
   32 net cuts the whole outer decision from 4.2 ms to 1.2 ms (the inner call
   from about 11 µs to about 1 µs) but loses 7.6 points. The v2 two-layer
   128→128 net (about 300k parameters because of the declaration-indexed
   embedding, but only 128×128 + 128×7 MACs per call after the gather) is
   within 1.4 points of parity and, on a quiet machine, halves the total
   decision time of the pinned player (494 ms vs 1,010 ms over 122 decisions)
   with a naive f32 forward. That is the current best point on the
   speed/strength curve.
3. **The inner level is not where Walt's strength mostly lives.** Replacing
   it with a tile-ordering heuristic costs 8.6 points, and replacing it with a
   sharper 64-world version of itself shows no gain on 96 deals. The ladder
   can collapse this level, but the payoff is cost, not strength.

## What was built

- `walt-fork/`: pinned `walt/walt` (`cb1ef3b2`, byte-identical at HEAD) plus
  `solver/net_hook.rs` and two lines at the top of `Solver::pi` for level 0:
  log the call, and if a net is installed answer with it. No other change.
  Built single-threaded (no `parallel` feature), like the phone WASM.
- `ladder/` (Rust): `log` (native self-play, every unique level-0 call per
  decision), `label` (exact production contract at N inner worlds: Voidless
  shuffle, seeded dice others, committed own future, `k/N` declarer-make,
  `best_of`), `bench`, `h2h` (paired mirrored games, resumable, worker slices).
  `net.rs`: embedding-sum forward over the exact level-0 Key (28 tile states
  × 7 + context; v2 indexes tile rows by declaration and adds led suit), one
  or two hidden layers, legal-only outputs, same `best_of` rule.
- `train.py` (MLX): masked soft-target BCE on `k/160`, selection on
  validation BCE, flat f32 export. `h2h.py`: capped parallel workers and the
  paired summary.

Receipts: every build, log, label, train, bench and h2h worker ran under
`run_capped.py` (≤ 295 s); failed receipts are kept under `results/failed-*`
and the first hi-res attempt (`hires160-96`, died on the wire's 20 s budget
ceiling). The 8-world label reproduces the production choice on 300/300 and
all 21k seam checks of the earlier pilot carried over.

## Numbers behind the table

- Native decision (40 outer / 8 inner, one thread): median 2.8 ms alone,
  3.5 to 6 ms under 12 to 19 concurrent processes; opening up to 66 ms. A hand
  takes 0.10 to 0.17 s. One decision makes 1,188 level-0 calls at the median,
  10,264 at the opening.
- Call stream: 48 self-play games → 805 decisions → 2.17M unique level-0 calls;
  8 held-out games → 382k. Labeled at 160 worlds: 0.9 to 1.8 ms per call,
  2.16M calls in 215 s wall on 12 workers. The production 8-world choice
  equals the 160-world choice on 60% of calls.
- Training: 2.16M rows; 60k to 80k updates of 2,048; 100 to 250 s per arm on
  Metal; train and validation BCE within .02 of each other for every arm
  (underfit, not overfit). Validation RMSE vs `k/160` (SE of the label itself
  about .04): one layer 32/64/128/256 = .172/.160/.144/.135 (v2 256: .124);
  two layers 128/256/512 = .101/.092/.087.
- H2H harness: 4,096 paired deals (8,192 games) in 180 to 230 s wall on 12 to
  19 workers. Native vs native is deterministic: 0 flips.
- Quiet-machine bench (8 games, 122 decisions, one process at a time), total
  decision time native vs hybrid: v2 two-layer 128→128 1,010 ms vs 494 ms
  (mean 8.3 vs 4.1 ms; medians 2.8 vs 2.2 ms), two-layer 256→256 1,021 vs
  807 ms, one-layer 32 1,011 vs 152 ms. Openings dominate the totals, so the
  near-parity 128→128 net halves Walt's decision time end to end with the
  naive f32 forward. Outer choice agreement with native on those decisions:
  78/122, 70/122, 68/122.

## Limitations

Exploratory tier; teacher-relative fidelity to a named model estimator under
a Voidless belief; deal contract chosen by the recovered `walt.c` heuristic
(most trumps), bid fixed at 30, pip trumps only; h2h opponent is the pinned
baseline without the partner review on either side; one initialization per
arm; hi-res control underpowered; the native decision timings in the table
are from loaded runs and only the within-row ratio is meaningful; no
multiplicity correction across arms; the earlier 1,024-deal reads (seed 4000)
are superseded by the 4,096-deal reads (seed 5000) and kept in `results/`.

## What I would do next

- **Speed for the two-layer net:** int8/NEON forward with legal-only outputs
  and an incremental first-layer accumulator across the recursion. The
  256×256 middle layer is 65k MACs; at int8 on this core that is a few
  microseconds, below the 11 µs native inner call. That turns "within 1.5
  points, 1.5x slower" into "within 1.5 points, faster".
- **Representation for the small nets:** per-tile features computable from
  the Key (trump, count, rank in context, wins the current trick, master in
  context, unseen followers) as extra embedding channels. The earlier study's
  32-feature scorer suggests this is the lever for one-layer nets.
- **Then the next rung:** log the outer baseline's root vectors from self-play
  with the net inside, and distill the outer level the same way.
