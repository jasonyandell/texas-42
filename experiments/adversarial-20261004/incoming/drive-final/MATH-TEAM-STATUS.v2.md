# MATH-TEAM-STATUS (v2, Oct 4 2026) — supersedes MATH-TEAM-STATUS.md; caveats preserved in substance

## 1. Vectorized flat shared-plan prototype (math team)
105 pip-trump roots, 100,800 (world, shared-order) rows.
- Exact terminal cells and action values matched the scalar oracle.
- Pilot cold array prep + simulation + reduction: 0.182–0.186 s, versus 1.300 s for scalar plans and 0.385 s for the grouped tree,
  in the same Python/NumPy runtime. Counted arrays 3.82 MB; process high-water 66.2 MB.
- Broader 134-root audit: 65 strict best-value gaps, 11 worse root choices, mean root regret 0.003731 — CONDITIONAL ON THESE FIXTURES.
Caveats: not population or phone-strength evidence; fixtures independently legality-replayed (avoiding the original sampler bug);
exact uint64 high-multiply indexing; unsupported non-pip declarations excluded; no native, GPU, or phone speedup claim.

## 2. frontier.py — ply-synchronous FULL own-choice tree (math team, validated)
- Exact uint64 tapes; collision-free history IDs (parent-history ID, tile, root prefix).
- On 105 pip roots and 69 partial tricks, EVERY action vector matched the lawful grouped recursion.
- Four alternating runs: 0.209–0.216 s versus 0.389–0.403 s (~1.85x), including preparation, branching, group sort, and reverse fold.
- Peak 2,997 scenario rows; 1.29 MB counted arrays.
- This is full-tree semantics with NO restricted-plan gap.
Caveats: makes no native, phone, or higher-k claim.

## 3. Sol's consistent_worlds repair (math team, validated)
- Disables settlement after every historical play (i.e., no early stop during the legality replay).
- Reproduces 700 exact legal worlds and 384/384 legal samples; originals preserved.
Caveat: these results do NOT replace testing the exact engine42 witness (bidder=1, trump=2, 16-ply, even=7/odd=17), which has not been
run in this workspace. The exact fixture is available from the math team on request; see "Open" below.

## How these sit with the folder's existing results
- Item 2 is the strongest result in the folder on the flattening question: a lawful full-tree frontier at ~1.85x the grouped recursion's
  speed in NumPy, with no class gap. It supersedes the shared-plan rung as the recommended direction for the inner rung (shared plans
  remain a defender-seat/first-pass candidate given flatplan.v3's class gap: 0.054 defending, 0.103 bidding, 0.336 on the opening lead).
- Item 3 is the same fix as engine42.v2.py's `no_stop=True` replay (test_support.py: 0 violations / 37,872 worlds; old path 28% illegal
  on odd-bidder positions), reached independently. Two implementations, same semantics; neither has yet been run on the exact witness.
- Nothing here is population or phone-strength evidence. The only phone-strength numbers in the folder remain harness.mjs vs Plunge
  a0d9fa80 / Rust cb1ef3b2: 52.2% ± 1.9 (native-l1, n=720), 48.3% ± 4.6 (native-partner, n=120).

## Open
- Run the exact engine42 witness fixture against engine42.v2.consistent_worlds and Sol's repair (requested from the math team).
- Population-level test of frontier.py-style inner rung inside Walt's outer loop vs native-partner via harness.mjs.
