# Opt-in Nel-O counterexample preview

Exploratory browser integration; no established playing-strength claim.

The shared player accepts `nello_counterexamples:true` alongside the own/public
`request`. After an ordinary Nel-O defender comparison, it can complete up to
three joint replans using at most 12 retained witnesses. The extra phase has a
two-second cap within the existing total budget. It uses the offline probe's
candidate pools and equal weights, and publishes only complete rounds.

The ordinary `evaluation` remains intact. `counterexample_result` contains
separate witness-mixture fractions, the chosen action, rounds and stop reason.
Subsequent live turns replan normally. Declarer and straight calls are unchanged.

Validation receipts here: seven focused Rust integration tests; native/WASM
agreement on the changed Ruby lead and exact stress vector; a forced deadline
after round one retaining precisely that completed round; disabled and declarer
isolation. The 12-case old/new WASM comparison preserves complete ordinary
choices, exact values, search counters and full partner reviews.

The historical offline panel and its limitations are in
[the research report](../nello-counterexample-2026-09-22/REPORT.md).

Reproduce the ABI check under the packet watchdog:

```
node walt/walt-player/counterexample-check.mjs /path/to/walt_player.wasm /fresh/result.json
```
