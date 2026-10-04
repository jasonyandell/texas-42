# AUDIT-RESPONSE-2 (Oct 4 2026) — supersedes AUDIT-RESPONSE.md item 2 and corrects the gap objectives

## Correction 1: engine42.consistent_worlds WAS affected (AUDIT-RESPONSE.md item 2 was wrong)
The legality replay in v1 was `Game(rules, full, 30, 0)`: bidder hard-coded to 0, early-stop live. For an odd bidder the true
bidders' points are attributed to the defenders, the replay marks the game `done` once they reach 13, `play()` becomes a no-op, and
every later legality check runs against stale table/hands and accepts follow-suit violations. This matches the independent witness
(bidder=1, trump=2, 16-ply, even=7/odd=17: fake replay stops after trick 1, accepts seat 2's tile 0 after a 6 lead while holding 2 and 4).
It is separate from the batched.v1 unchecked fallback. It reaches exact_tape.worlds (and therefore every Python rung and reference
that sampled from an odd-bidder position).

Fix: `Game(..., no_stop=True)` — replay every ply, never mark done. Legality depends only on winners/leaders (bidder-independent), so a
no-early-stop replay with any bidder label is exact. Applied to engine42.consistent_worlds (engine42.v2.py), nofusion_sc.all_deals
(line: `g = Game(rules, full, BID, history[0][0], no_stop=True)`), and batched.batched_worlds (line: `g = Game(rules, full, BID, bidder, no_stop=True)`).

Regression test (test_support.py, independent scalar bidder-aware no-early-stop checker):
- FIXED samplers (consistent_worlds, all_deals, batched_worlds): 37,872 worlds checked across even and odd bidders, violations = 0.
- OLD path reproduced: on 144 odd-bidder positions it accepted 13,288 candidate worlds, 3,731 (28%) illegal. Exit 0 iff both hold.

### What is verified / not verified
- Verified: the fixed samplers emit only legal worlds under the independent checker on 6 hands x ~36 positions each (includes the enumerated
  tail and the batched sampler). The old path's failure class is reproduced.
- NOT verified: the exact math-team fixture was not run here (available on request); results below are on my fixtures.
- Prior results that sampled from odd-bidder positions in Python are CONTAMINATED and should be re-measured: regret tables for hands C
  (bidder 3) and D (bidder 1); Python-engine tournaments (half the hands had an odd bidder; both sides used the same sampler, so
  symmetric, but not trustworthy); flatplan hand C (re-measured below).
- Prior results NOT affected: everything against the real Plunge Walt via harness.mjs (52.2% ± 1.9 native-l1 n=720; 48.3% ± 4.6
  native-partner n=120) and the C-vs-C matches — walt.c samples by voids tracked from true play; the exact-tail validation on the
  walkthrough (enumerated directly, 515/2016 & 449/2016); hands A and B tables and flatplan hand A (bidder = seat 2, even).

## Correction 2: two gap objectives, not conflicting measurements
flatplan.v2's "tree" comparator is per-world extrema (fused). Its gap therefore = plan-class restriction + fusion slack (an upper bound
on the class gap). The math team's gap is against the LAWFUL shared-history tree = the class gap proper. The v2 shared-plan estimator
itself is lawful conditional on valid input. flatplan.v3 reports both on the same worlds and tape (fixed sampler, 128 worlds, 50 positions/hand;
reference exact-tape@384 is itself per-world extrema, so "lawful-tree regret" below includes the reference's own slack):

| | defending C (sampler fixed) | bidding A |
|---|---|---|
| flat vs LAWFUL tree, same decision | 92% (was 86% contaminated) | 78% (trick 1: 25%) |
| flat regret / lawful-tree regret vs ref | 0.0051 / 0.0000 | 0.0131 / 0.0028 |
| class gap proper (vs lawful tree) | 0.054 | 0.103 (trick 1: 0.336) |
| gap vs fused tree (class + fusion slack) | 0.065 | 0.111 (trick 1: 0.338) |

Fusion slack is 0.01–0.02 on average and ~0.002 on the bidder's opening lead; the 0.34 there is class restriction. Conclusion unchanged:
shared plans are lawful and fusion-free; beat the 8-world inner for defenders; worse for a bidder on the opening lead. The math team's
0.003731 (134 roots) remains a different fixture set and reference and is recorded alongside, not combined.
