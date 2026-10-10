# L2 versus phone Walt: paired dropped-30 results

2026-09-30. **L2 with 160 outer / 24 inner deals won the independent
confirmation panel**, completing every game well below the 30-second ceiling.
This is evidence of an advantage on the declared random-deal, pip-trump,
dropped-30 population against the pinned phone-player profile. It is not an
auction comparison, a phone-device timing, or a result for other contracts.

The candidate is the unchanged addressed-v2 **compiled CPU** implementation
from [AMORTIZE.md](AMORTIZE.md). Despite the conversation's “GPU Walt” name,
these matches do not execute GPU kernels.

## Independent confirmation

After exploring three settings, freeze L2 at 160/24 and play **512 new pairs**
(1,024 games), seeds 9306001–9306512. Configuration, sample size and a positive
net with two-sided paired p<0.05 were fixed before this panel began. No earlier
screening or holdout data is pooled into its test.

| Measure | Result |
|---|---:|
| Paired L2 wins / losses / ties | **84 / 46 / 382** |
| Individual game wins: L2 / Walt | **550 / 474** |
| L2 game win fraction | **53.71%** |
| Paired-deal bootstrap 95% interval | **51.56%–55.86%** |
| Exact two-sided McNemar/binomial p | **0.001093** |
| L2 declaring make rate | 58.79% |
| Walt declaring make rate | 51.37% |
| Median whole-game time | **1.111 s** |
| p95 whole-game time | **2.241 s** |
| Slowest whole game | **4.769 s** |
| Median L2 partnership decision time | 0.944 s |
| Median Walt partnership decision time | 0.164 s |
| Timeouts / errors | **0 / 0** |

The two engine medians are each measured across games; medians need not add
to the median total. Whole-game times include both partnerships, native process
and candidate context initialization, transport and referee work. Python
imports, initial dealing, final result serialization and teardown are excluded.
Four game workers run concurrently; each native search is serial. Hardware:
Apple M5 Max, 18 CPU cores, 48 GiB RAM, macOS 26.5.1.

The fixed-setting confirmation met its criterion, so the budget ladder stopped
at **160/24**, well before the 30-second limit. No claim is made that it is
the strongest setting achievable within 30 seconds.

## Selection history, retained separately

Sample budgets are outermost first. The three screening rows use the **same
64 deals**, seeds 9303001–9303064. Each holdout has a separate fresh seed range.

| Phase | L2 outer / inner | Pairs | Paired W / L / T | Median game | Maximum game | Timeouts |
|---|---:|---:|---|---:|---:|---:|
| Screen | 30 / 8 | 64 | 5 / 7 / 52 | 0.223 s | 0.541 s | 0 |
| Screen | 160 / 8 | 64 | 9 / 5 / 50 | 0.489 s | 1.338 s | 0 |
| Fresh holdout | 160 / 8 | 256 | 37 / 23 / 196 | 0.478 s | 1.570 s | 0 |
| Screen | 160 / 24 | 64 | 11 / 1 / 52 | 1.079 s | 3.708 s | 0 |
| Fresh holdout | 160 / 24 | 256 | 41 / 22 / 193 | 1.131 s | 4.692 s | 0 |
| Fixed confirmation | **160 / 24** | **512** | **84 / 46 / 382** | **1.111 s** | **4.769 s** | **0** |

The 160/8 holdout had a positive net but its paired 95% interval included 50%
(49.80%–55.66%; exact p=0.0925). The 160/24 holdout favored L2, but that
setting was selected during exploration. Therefore an additional fresh,
fixed-setting confirmation was run and analyzed only after completion. The
confirmation supplies the headline evidence. Earlier exploratory p-values
and a conservative seven-setting adjustment are retained in the analysis,
with their scope kept separate.

## What a pair means

Uniformly shuffle 28 dominoes and deal seven to each seat. **Seat 1 is forced
to bid 30** regardless of its hand; there is no search for a stronger bidder
among the four seats. Choose pip trump from the bidder's hand alone: largest
trump count, then possession of the trump double, then largest pip.

Play the same physical deal, bidder and trump twice:

1. L2 occupies the declaring partnership, seats 1+3; Walt defends.
2. Walt occupies seats 1+3; L2 defends.

L2 wins the pair if it makes when declaring and sets Walt when defending.
It loses the pair if both games go the other way. Both-make and both-set are
ties: each engine wins one game. Thus 84 paired wins and 382 ties give L2
`2 × 84 + 382 = 550` individual game wins. Games end as soon as make/set is
settled. A timeout has no win/loss outcome; its entire pair remains unscored.

Field seed 42 is fixed independently of the private physical-deal seeds. Both
legs use identical fields and baseline profiles. All settings use delta=1.
The candidate requests only an action, allowing its existing exact pruning to
avoid fully pricing losing alternatives.

## Baseline identity and information boundaries

“Walt” is the native build of the checked-out Plunge phone player, including
its actual deployed decision wrapper:

- Opening: 160 outer / 8 inner worlds, partner review off, 20-second move budget.
- Ordinary play: 40 outer / 8 inner worlds, existing partner review enabled,
  14-second move budget.
- Source hash exactly matches the phone manifest at commit
  `1dfd0e22f5fe2a2ee266f166493442547ad33834`; the checked-out WASM also matches
  that manifest. Rust 1.95.0, release build, cpu-speedups enabled, parallel
  search disabled. The player code was not modified.

Every nonforced completed baseline decision in the retained games reached its
requested 40- or 160-world evaluation. No baseline evaluation interruption or
weaker fallback explains the observed edge. The existing partner review did
change 26 choices across the complete retained games, including smoke runs.

Full physical deals live in the host. Native Walt receives only the actor's
original own hand and chronological public history. C++ L2 receives only the
remaining own hand and public state; its nested policies sample their own
compatible worlds. No real or enclosing complete deal crosses a policy entry
point. The prior shared-information-state search and no-fusion checks remain
unchanged. Moving to larger samples changed budgets, not that information rule.

## Audit, timing limit and artifacts

All **2,432 study games** completed. Four separate smoke games also completed.
The independent audit verified **55,432 plays** across these 2,436 complete
games, plus **19,391 nonforced Walt decisions** at their requested fidelity.
Every move agrees with the scalar referee, NumPy public transitions, and, on
Walt turns, the Rust rules' legal list, leader and score.

An external process-group watchdog enforces the 30-second limit in addition
to the candidate's internal game deadline. A result arriving after the limit
is a timeout even if it contains an outcome. Two separate, artificially tiny
deadline tests verified forced termination and exclusion from pair scores.
Those synthetic timeouts appear only in the harness audit, not in the study
rows or their win/loss totals. No actual study game timed out.

Every game's fixture, public trace, baseline requests/responses, timings and
terminal status are retained. The candidate and baseline binary hashes appear
in each game's job identity. Baseline source inputs and manifest are archived.

- [Protocol and chronological amendments](h2h/PROTOCOL.md)
- [Build and reproduction instructions](h2h/RUN.md)
- [Confirmation summary and every pair](h2h/confirm-160-24/summary.json)
- [Recomputed analysis and uncertainty](h2h/analysis.json)
- [Independent replay and baseline-fidelity audit](h2h/verification.json)
- [Baseline source/build identity](h2h/baseline-identity.json)
- [Current source, binaries and evidence hashes](H2H-MANIFEST.json)

Confidence intervals resample whole paired deals, preserving the pairing:
100,000 multinomial bootstrap draws, seed 83047. The exact paired test is a
two-sided binomial test on the 130 discordant confirmation pairs. All evidence
is about completed actual play in this experiment; modeled make estimates
are not substituted for game outcomes.
