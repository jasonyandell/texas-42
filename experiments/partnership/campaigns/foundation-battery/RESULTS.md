# Unified player foundation: results

2026-09-06. **EXPLORATORY.** The shared selection engine passes the foundation
checks. Native racing L1 reproduces the archived phone on completed matched
decisions. No selection rule or void option has established a general strength
gain; the tested partner-refinement configurations exceed the fallback gate.

## Completed matched panels

Bid 30 throughout. Each deal is played twice, with player partnerships swapped.
Pair score is make(A) minus make(B); both-make and both-set pairs tie regardless
of points. Win fraction is `(1 + mean pair score) / 2`, a comparative contract
score, not an estimate of a hand's absolute pmake. The random comparisons share
seeds 720600–720649; the conditional panel uses 820600–820649.

| A versus B | Paired deals | A wins / losses / ties | A contract win fraction |
|---|---:|---:|---:|
| [l1-race vs phone](01-phone/MATCH.md) | 50 | 3 / 1 / 46 | 52.0% |
| [l1-race vs l1-fixed](02-fixed/MATCH.md) | 50 | 4 / 5 / 41 | 49.0% |
| [l1-refine vs l1-race](03-refine/MATCH.md) | 50 | 4 / 2 / 44 | 52.0% |
| [l1-race-voids vs l1-race](05-voids/MATCH.md) | 50 | 9 / 5 / 36 | 54.0% |
| [l1-race-voids vs l1-race](07-worlds/MATCH.md) | 50 | 5 / 8 / 37 | 47.0% |

The four random-panel rough intervals all include 50%; this is unresolved
strength evidence. The conditional comparison is only **five focal hands**,
each with ten hidden-hand completions. Three hands favor voidless by one pair;
two tie. It supplies no broad voids advantage. The per-hand table is retained
in its match report. Repeated completions do not count as independent hands.
The descriptive report suppresses its normal approximation below ten units
or when observed variance is zero; this presentation guard does not alter
recorded scores or the predeclared stopping rule.

## Phone fidelity and latency

All **35/35 fallback-free mirrored pairs** played identically for all 28 moves. There were 12 first divergences; every one began at a phone L1 fallback. The 3/1 pair edge is therefore evidence about the bounded
implementations, not a discovered difference in their completed decision rule.
This complements the 64 pre-battery native/WASM decision comparisons. It is
observed parity, not a whole-program equivalence proof.

Representative means from the completed 50-deal matches (seconds per move,
including forced moves, wrapper overhead, and fallback preparation):

| Player | Seconds / move | Fallbacks / nonforced decisions | Matched report |
|---|---:|---:|---|
| l1-fixed | 0.137 | 0 / 929 | [02-fixed](02-fixed/MATCH.md) |
| l1-refine | 0.313 | 0 / 926 | [03-refine](03-refine/MATCH.md) |
| l1-race | 0.502 | 0 / 928 | [01-phone](01-phone/MATCH.md) |
| l1-race-voids | 0.535 | 0 / 932 | [05-voids](05-voids/MATCH.md) |
| phone | 0.778 | 17 / 923 | [01-phone](01-phone/MATCH.md) |

These are wall latencies under the shared ten-game pool, not isolated CPU
benchmarks. All completed L1-only comparisons had zero native fallbacks.
The archived phone had 17 fallbacks in its 923 nonforced decisions.

## Partner modeling: cost stop, not a strength verdict

The original profiles and two declared cost-driven extensions all crossed
the >5% fallback gate after at least 20 nonforced decisions for that player.
No completed outcome was rerolled and no gate was relaxed. These short,
technically stopped prefixes do not establish the deeper policy's strength.

| Tested A configuration | Completed pairs | A fallback / nonforced | Mean seconds / move |
|---|---:|---:|---:|
| [partner-race](04-partner/MATCH.md) | 2 | 10 / 33 | 3.582 |
| [partner-race-voids](06-partner-voids/MATCH.md) | 2 | 11 / 33 | 3.786 |
| [partner-race-small](08-partner-small-matched/MATCH.md) | 2 | 4 / 32 | 1.680 |
| [partner-race-small](09-partner-small-anchor/MATCH.md) | 2 | 3 / 31 | 1.562 |
| [partner-race-small-voids](10-partner-small-voids/MATCH.md) | 2 | 4 / 35 | 1.834 |
| [partner-race-fixedmind](11-partner-fixedmind/MATCH.md) | 1 | 2 / 20 | 2.200 |
| [partner-race-fixedmind-voids](12-partner-fixedmind-voids/MATCH.md) | 1 | 2 / 20 | 2.285 |

The original profile uses n=40/n0=8/n1=2 with race/refinement at both levels.
The small profiles use 8/8/1. The final diagnostic keeps the real root at
40/8 and race/refine while modeled L1 uses a single fixed two-world bundle.
Even that final variant exceeded the declared threshold on its first pair.
Root refinement over a partner field is itself costly; this experiment does
not isolate an implementation bottleneck. The earlier fixed-root partner
player remains available and is not refuted by these different configurations.
Under a clock cap, strengthening a modeled procedure need not strengthen
the executed player: it can increase fallback frequency.

## Durability and evidence

The four foreground pool slices consumed **824.62 seconds (13.74 minutes)** in total. They published **524 games / 14,672 moves**, all independently replay-verified. This includes 500 games in five completed
panels and 24 games in technically stopped prefixes. Additional speculative
work and stopped checkpoints remain on disk but do not enter those totals or
the primary scores. Wall time includes that work. The battery's four completed
random matchups use 50 shared deal units, not 200 independent deals.

All 4,639 moves saved at the first slice boundary survived subsequent resume
unchanged ([resume proof](resume-proof.json)). The separate pilot also tested
ten simultaneously interrupted games and worker restart. All workers are
stopped. Source/binary identities remain pinned; immutable manifests preserve
every experimental configuration independently of later preset edits.

The [protocol](PROTOCOL.md), [first extension](EXTENSION-1.md), and
[final diagnostic](EXTENSION-2.md) explain exactly what ran and why it stopped.
Each match contains its manifest, every move, per-seed paired scores,
per-player timing/fallback counts, and an independent verification record.
`analyze.py` regenerates this summary from saved data; it launches no players.

## Foundation verdict

Use `l1-race` as the native counterpart of the recovered phone procedure in
controlled comparisons. Keep `l1-fixed` and `l1-refine` as named cheaper
alternatives; this panel does not settle their strength order. Belief strategy
and partner modeling remain independent, explicit choices. Existing defaults
are unchanged. The next substantial engineering question is why partner-field
work reaches its budget, followed by a fresh cost-matched strength comparison.
No additional experiment is running or scheduled.

See [the design and validation record](../../FOUNDATION.md) for shared-engine
contracts and the bounded scope of the checks. This establishes a stronger
engineering foundation, not the original requested partner-strength claim.
