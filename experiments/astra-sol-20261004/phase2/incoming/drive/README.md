# walt-sense — handoff (Oct 3–4, 2026)

## Files
- `HANDOFF.md`       — the brief: definitions, advancements, measurements, negative results, Lean targets.
- `TINY-MODEL.md`    — the per-hand tiny-net idea: promise, what was tried, why it failed, what remains open.
- `engine42.py`      — vectorized 42 engine (bitmask hands, tape kernel, consistent-world sampling, orderings-based L1 reference).
- `exact_tape.py`    — exact-tape L1: branch only at own plies (realized sequences, not orderings). Verified ≡ orderings L1.
- `nofusion_sc.py`   — information-set-exact expectimax (the tail) and the no-fusion fold. Reproduces 515/2016, 449/2016.
- `walt.c`           — C: `walt42.py` ported line for line (Walt = walt(walt(random,8),30)), the new player (exact-tape 128 + exact tail), two-sided match driver, and a `decide` mode.
- `harness.mjs`      — Node: real Plunge Walt (`walt-player.wasm` from jasonyandell/plunge) vs `./walt decide`; each deal twice, teams swapped; rules cross-checked against Walt's banked points every call.
- `plunge-match.log` — results vs the real wasm Walt.

## Build / run
```
gcc -O2 -o walt walt.c -lm
./walt 100 7 new walt            # new player vs walt42-Walt (30 outer, 8 inner), 200 hands
./walt decide 2 30 1 5 <mask> 1 25   # one decision (bidding team = odd seats; relabel so bidder is seat 1)
WASM=path/to/walt-player.wasm node harness.mjs 120 11 40 0   # vs real Walt, native-l1
WASM=path/to/walt-player.wasm node harness.mjs 120 11 40 1   # vs real Walt, native-partner (app default)
```

## Conventions
Tile id = h(h+1)/2 + l (same in C, Python, Plunge). Hands are 28-bit masks. Bid 30, pip trump.
Contract (walt42): the seat/pip with the most trumps, then the trump double, then the most doubles; bidder leads.
Made ⇔ bidders ≥ 30 ⇔ ¬(defenders ≥ 13).

## Headline numbers
- Regret vs 512-world reference (4 hands): Walt's 8-world inner rung 0.010–0.023; exact-tape @128 0.0008–0.0018 at the same wall time.
- vs walt42 inner rung: 64/36. vs real Plunge Walt (wasm, 40 worlds): 52.2% ± 1.9 at native-l1 (n=720); 48.3% ± 4.6 at native-partner (n=120).
- Read: equal strength to shipped Walt at ~2–3× lower cost; not a new rung. Next: exact-tape kernel as Walt's inner rung, keep the partner check, re-measure.
