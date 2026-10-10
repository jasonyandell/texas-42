# Snapshot of /Users/jason/code/texas-42/experiments/walt42x-intake (untracked there)

Copied 2026-10-04 by Claude Fable 5.1 from the main checkout at branch `walt-gran`
(HEAD 9d6a5a2e), where the directory is **untracked** (`git status` shows `??`).
It was neither lost nor landed. This copy holds code, docs, manifests and
checkpoints only; `results/` (53 MB) and `h2h/` (163 MB game receipts) were not
copied and remain in the original location. Nothing in the original was modified.

What it is: Jason's "delta expectimax" frontier ladder, 2026-09-29/30. A pure,
void-aware NumPy/Numba (plus compiled C++ "turbo") Walt ladder, L0 = uniform
random, L1 = best response to L0, L2 = best response to L1, uniform samples from
the public-void support, full public-history keys. `delta` in [0,1] controls how
opponent randomness is handled per sampled world: a live fiber of weight w with
n legal opponent moves is split exactly into n children of weight w/n when that
keeps each child >= delta, otherwise one move is sampled on the tape. delta = 0
is exact expectimax over the finite sample; delta = 1 is pure sampled
continuation (Walt-like); delta = 1/8 bounds simultaneous opponent branches per
world by 8. Headlines recorded there: L2 games at 0.130 s median and 5,159
games/minute on 18 threads (AMORTIZE.md); L2 at 160 outer / 24 inner beat the
pinned phone Walt 84 / 46 / 382 on 512 independent pairs, p = .0011 (H2H.md).
