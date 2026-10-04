# Walt sense work, Oct 3–4 2026 — handoff

## Definitions (fixed)
A *sense* reports P(bid made | my action) over deals consistent with my information set, with other seats playing an assumed policy. L1 = best response to uniform-random others, own future fully optimized. Walt (Plunge, `walt-player.wasm`) = best response to L1: 40 outer worlds (160 at the opening lead), 8-world inner L1 at every other-seat node, plus an optional partner check (`native-partner`, the app default). Own nodes always commit to one tile per information set — never one per deal (no strategy fusion; "strong but wrong" is disallowed). In `walt42.py` the naming is offset by one: `L0 = walt(random)`, `L1 = walt(L0, 30)`.

## Advancements (implemented, verified)
1. **Exact-tape L1.** Pre-roll every other seat's randomness as a tape u∈[0,1) per (world, ply), resolved at play time as the ⌈u·n⌉-th legal tile. My own future then needs no ordering enumeration: branch only at my own plies and visit *realized* sequences. Verified identical values to full k!-ordering L1 on the same worlds/tape. Realized sequences per world at trick 1: ~22 (defender) vs 5,040 orderings; ~860 (bidder). Cost 12–160× lower; common random numbers across candidates come free.
2. **Information-set-exact tail.** Last three tricks: enumerate every consistent deal (≤1,680) and every play; others branch with weight 1/n (expectimax); my nodes group rows by public history and choose once per group. Reproduces the hand-enumerated truth to 5 decimals (515/2016, 449/2016 on the reference position). ~0.1 s in Python, faster than the fused version. Fusion changed 1 decision in 48 tail positions, cost 0.0003.
3. **No-fusion fold in a flat kernel.** The same grouping works for the sampled kernel; sampled worlds almost never share a history past the root, so fused vs unfused agree there in practice.

## Measured
Regret vs a 512-world reference across 4 hands (bidding and defending): Walt's 8-world inner rung 0.010–0.023; exact-tape at 128 worlds 0.0008–0.0018, same wall time. Head-to-head, deals played twice with teams swapped: new player beats the 8-world inner rung 64/36; beats `walt42.py` (30 outer) ~55/45 (n=60, noisy); **vs real Plunge Walt (wasm, 40 worlds): 52.2% ± 1.9 at `native-l1` (n=720), 48.3% ± 4.6 at `native-partner` (n=120).** So: equal strength to shipped Walt at ~2–3× lower cost (11 ms vs 28 ms per decision, C vs Rust), not a new rung. Rules engines agree: ~6,900 banked-points cross-checks passed.

## Negative results
Per-hand tiny nets (value from single rollouts; policy distilled from paired-rollout labels) do not learn the decision at any affordable budget — the signal is 1–5% sibling differences under the label noise floor; the exact-target control (2,048 paired rollouts per candidate) shows the target itself is fine (regret 0.006). Racing saves ~40% of worlds, not 5–10×. Sampled own-lines (24 random orderings) work for defenders, bias bidders (one-sided: max over a subset underestimates). V₀-greedy (one-step best response with a random future self) ranks tiles well but its values are biased and its floor does not improve with samples.

## Next experiment
Put the exact-tape kernel *inside* Walt's outer loop as the inner rung (10–20× more accurate than the 8-world inner at equal cost), keep the partner check, measure in the same harness (`harness.mjs`).

## For Lean
(a) exact-tape L1 ≡ ordering-enumeration L1 (same worlds, same tape); (b) first-legal-in-ordering policies realize exactly the leaves of the own-choice tree; (c) the grouped fold is the information-set expectimax (one action per history), and the per-deal fold is its fusion-biased relaxation with value ≤/≥ by side; (d) made ⇔ bidders ≥ bid ⇔ ¬(defenders ≥ 43−bid) given 42 total.

## Artifacts
This folder. Walt source of truth: `jasonyandell/plunge`, `src/ai/phone/walt-player.wasm`; call shape `{request:{decl,bid,bidder,seat,hand,plays,seed}, worlds, partner, budget_ms}`; host imports `walt_host.now_us` (BigInt µs) and `walt_host.checkpoint(ptr,len)`; exports `walt_in_prepare`, `walt_call`, `walt_out_ptr`, `memory`.
