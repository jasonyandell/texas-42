# Nel-O preview headroom — exploratory timing

This is an engineering latency probe, not a playing-strength experiment.
`timing.mjs` runs the imported browser WASM in Node with its real clock and a
fresh instance per decision. Inputs contain only the acting player's own hand
and public history. The two early free-choice cases use the same fixture with
two seeds; the three Ruby positions share one recorded hand. This small panel
is not representative of all games or phone hardware.

The prior build uses 256 candidate deals per round and up to two remaining
seconds for counterexamples. The candidate uses 768 and reserves up to six
seconds before ordinary comparisons (at most half the caller's budget).
Both retain at most four failures in each of three rounds, at equal weight.
Normal play remains 40 worlds; explicit Think deeper rises from 160 to 500.
The 14/20-second total limits and inner sample size eight are unchanged.

`before.json` records the prior build. Candidate timings and conformance results
will be added after importing and checking the matched WASM.
