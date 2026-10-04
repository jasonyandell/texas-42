# walt-sense — handoff bundle (Oct 2026)

Files
- engine42.py      vectorized 42 engine: bitmask hands, flat tape kernel, consistent-world sampling, orderings-based L1
- exact_tape.py    exact-tape L1 (branch only at own plies; realized sequences, not orderings) + comparison rungs
- nofusion_sc.py   information-set-exact expectimax (tail) and the no-fusion fold; self-contained
- walt.c           C port of walt42.py (Walt to the line) + new player (exact-tape 128 + exact tail) + two-sided match driver
                   build: gcc -O2 -o walt walt.c -lm ; match: ./walt <deals> <seed> new walt [outer] [worlds]
                   single decision: ./walt decide <seat> <bid> <bidder> <trump> <dealt-mask> [actor tile]...  (bidding team = odd seats)
- harness.mjs      node harness: real Plunge Walt (src/ai/phone/walt-player.wasm from jasonyandell/plunge) vs ./walt decide,
                   deals played twice with teams swapped, rules cross-checked against Walt's banked points each call
                   run: node harness.mjs <deals> <seed> [worlds=40] [partner=0|1]
- endgame.py       hand-rolled exact enumeration of the walkthrough position (560 deals; 515/2016 vs 449/2016)
- build.py/flat.py/data.json/tape.json   the Excel workbook generator ("Walt in Excel - All the Way Down.xlsx")
- plunge-match.log results vs real Walt

Tile ids: triangular, id(h,l) = h(h+1)/2 + l (same in walt.c, Python and Plunge). Hands are 28-bit masks.
