# Pattern Workshop: exploratory biclustering of actual play

[Open the Pattern Workshop](https://sunshine-atlas.psychometrix.chatgpt.site/biclusters/)
— published 2026-09-19 inside Sunshine Atlas, owner-private. The original Atlas
remains available. This is a static, phone-friendly view of saved games.

## Question and representation

Which families of whole bidder hands under a declaration share families of
hidden holdings associated with making 30? Biclustering chooses the row and
column groups together. Inputs are all 84 physical-tile/hidden-seat conditions
and all 3,402 two-tile holding conjunctions. There are no named trap or protection
features. A holding group is a collection of alternative predicates, not one
huge conjunction. Pair associations need not represent synergy.

Within each panel, compare make rates when a holding is present and absent,
shrink thin comparisons, and subtract that condition's discovery baseline.
A fixed 6×6 spectral biclustering groups this matrix. Fit and check use separate
trial windows; memberships, ranks and the featured block use discovery only.
These are regularized association scores, not changes in a player's win rate.

## Retained broad pilot and deeper study

The broad pilot uses the Atlas's 36,000 games, 4,500 panels, trials 0–3 to fit and
4–7 to check. Its strongest block falls from +5.719 to -0.006 association points;
16/24 independently shuffled, fully refitted controls exceed its checking score.
All blocks and failures remain visible.

The deeper study freezes 736 panels already at 160 games, including all 79
preselected audits. Most other panels survived adaptive screening. Exclude the
screening prefix 0–39, fit 40–99, check 100–159: 88,320 games, 60+60 per panel.
733 panels vary in discovery and enter the fit. This selected cohort is not the
full hand population, and the two studies do not isolate a sample-size effect.

The strongest deeper block, B × 3, contains 143 panels from 85 source deals and
86 holdings. Its discovery/check scores are +5.962/+5.587; the exploratory
source-deal bootstrap checking interval is [+3.738,+7.713]. None of 24 shuffled
controls reaches its checking score. 32/36 blocks retain their direction.
Partner's 6–4 recurs in its strongest conditions; the leading pair is partner
holding both 5–5 and 6–4. The row family contains 56 fours declarations, and
bidder double-four occurs in 48.95% versus 34.11% across fitted panels. This is
a recognizable count/trump association to inspect, not a proved tactic.

## What the viewer supports

- Compare all 36 discovery and checking blocks with a shared color scale.
- Unfold the reordered individual-entry matrix and select a hand/holding entry.
- Inspect complete hands and all holding conditions, including singletons.
- Filter all 120 deeper or eight pilot worlds by condition, outcome and window.
- Replay all seven tricks, optionally reveal hidden hands, reopen a saved view.
- See shuffled controls, formulas, source identities and original receipt hashes.

This is same-hand, different-world internal replication in an already explored
corpus. The study does not yet assign new hands to families or establish a
lawful threat detector, causal swap effect, sample saving, or player improvement.
The next question is which compact descriptions of these families survive new
hands and help an actual decision. No player, bidder, allocation or monitor changed.

## Reproduction and evidence

Source and full methods: `/Users/jason/code/texas-42-sunshine-atlas/BICLUSTERS.md`.
Frozen input, numerical matrices, logs, verification, screenshots and the validated
deployment archive: `/Users/jason/data/texas-42/sunshine-biclusters-v1/`.
Compact publication and study identities: [BICLUSTERS-v1.json](BICLUSTERS-v1.json).

The deeper extraction verifies all 88,320 original receipt hashes and complete
legal replays against independent rules. The pilot retains the Atlas's verified
36,000-game source. An independent audit reconstructs 400 comparisons per study
and checks all block means and memberships. Desktop and Pixel-sized Chrome checks
cover filters, matrix selection, seven-trick traces, hidden hands and deep links.
Source is committed and pushed to the Site's repository; no campaign data changed.
