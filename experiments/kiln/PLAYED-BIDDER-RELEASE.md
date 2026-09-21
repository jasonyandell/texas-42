# Actual-play bidder release

The empirical bidder is now playable in [Plunge](https://plunge.jasonyandell.workers.dev/).
New matches and subsequent hands deal from a shuffled 125-deal catalogue with
500 recorded bidder hands. Every covered auction is a table lookup; ordinary
short presentation pauses remain. No live declaration-pricing workers run.

## Frozen evidence

- 305,440 complete games, 8,552,320 independently replayed moves, 4,500 panels.
- All 42,008 original receipt hashes preserved; database integrity check passed.
- Depths: 1,575 panels at 8 games; 2,189 at 40; 541 at 160; 19 at 320; 176 at 640.
- Allocation: 3,764 screened, 582 resolved, 79 audit-complete, 75 capped-unsettled.
- 80% observed-score cutoff: 80 qualifying panels, 61 bidder hands, 52 deals.
- Source book: `77cb49c7a8d7f8b4548cf982d97f424d062ed1a88f9091bbb0ba0484528f2bc2`.
- Full export: `/Users/jason/data/texas-42/kiln-played-v1/played-book-500-final.json`.
  The older `played-book-final.json` remains unchanged.
- Durable release evidence: `kiln-played-v1/releases/plunge-c7a1215/`.

The book contains score tails of the unchanged actual deployed policy targeting30
through all seven tricks. The higher bids are this agreed empirical heuristic,
not a measurement of retargeted policies or an 80% guarantee against humans.
Capped uncertainty is retained, not relabeled as confidence. The previous scalar
model-price survey stays separate and is not consumed by Plunge.

## Table behavior

Walt selects the highest legal 30–42 target clearing the 4/5 frequency cutoff,
then the declaration with the best recorded tail at that target. It passes over
partner's standing bid. A forced 30 uses the best available panel even when below
cutoff. At a qualifying42, select the cheapest legal plain-marks bid. Human bids,
marks, shaker rotation, shared replays and ordinary Walt play remain unchanged.

The lookup receives only own seven tiles and seat, plus the public auction target;
it does not inspect the catalogue's actual opposing hands. Auction records carry
an explicit empirical schema, source book/profile, full nine-panel counts and
allocation states. A saved non-catalogue hand finishes with the existing fallback;
its next hand enters the catalogue. Each match visits all125 deals before repeating.
Weak deals are retained; all-pass reshakes remain normal. The80% cutoff is selective.

Plunge main commit: `c7a1215d4d1ece0e24e13fef693ed17f55475112`.
The playing WASM and manifest are unchanged. Typecheck,201 tests in25 files,
production build and Cloudflare dry run passed. Pixel-size Chrome completed a
real hand, reshuffled, reloaded and resumed; no auction workers, JavaScript errors
or horizontal overflow. Original local timing included3.396s for a full auction
and declaration, including presentation pauses and automated human response.
This is a Mac browser at Pixel viewport size, not a new Pixel hardware benchmark.

The published build was verified directly. Offline reload and a fresh empirical
auction also passed with the network disabled.

[Compact release and deployment evidence](played-bidder-release.json) contains
exact hashes and hosted checks. The stopped monitor was not restarted, no new
games were generated, and the old survey remains frozen.
