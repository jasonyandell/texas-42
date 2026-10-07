# How Walt picks a domino — explainer page

EXPLORATORY tier, like everything under `walt/`. An interactive page in the style
of Red Blob Games that walks one Walt decision down the stack and back up:
the answer, the 40 imagined deals, one deal played out, a modeled player's
8 guesses, the dice at the bottom, then back to the top.

- The top-level numbers are the real Walt's, copied from
  `experiments/partnership/campaigns/foundation-battery/02-fixed/seeds/720641`
  (default L1, 40/8, bidder's opening lead, fives trump, bid 30).
- Everything below the top is a JavaScript re-creation of the L1 baseline method
  (`SCENARIO-PLAYER.md` Defs 3.2, 3.5, 4.1–4.3, 5, 6.1; fixed low-index tie rule)
  in `engine.js`. It uses its own random streams, so its deals and counts differ
  from Walt's; it is an illustration, not a conformance receipt.

Rebuild: `node pre.js` (writes `data.json`, about 6 s) then `python3 build.py`.
`walt-explained.html` is a body fragment; the artifact host wraps it in a document.
