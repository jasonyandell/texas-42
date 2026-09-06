# Early outcome witnesses

Exploratory trajectories from the pinned random-deal campaign, read after its
first 24 completed seeds. These are examples, not an exhaustive final analysis.
Bid is 30 throughout. All point columns give the declaring partnership's points.

| Seed | Contract | Phone | Candidate declaring | Against candidate defense | Make/set change |
|---|---|---:|---:|---:|---|
| 420607 | S2, threes | 41 | 26 | 41 | Candidate declaration loses |
| 420612 | S0, blanks | 18 | 18 | 33 | Candidate defense loses |
| 420615 | S1, sixes | 41 | 26 | 30 | Candidate declaration loses |
| 420619 | S0, sixes | 25 | 35 | 42 | Declaration wins, defense loses |
| 420622 | S3, fives | 35 | 35 | 24 | Candidate defense wins |

420612, 420615, and 420619 have no fallbacks in any lineup. On 420607, the phone
opening timed out and used the recorded L1 fallback; candidate's opening
completed. On 420622, both phone and candidate-defending lineups contain one
phone fallback. These remain valid executed-policy outcomes, with their timing
dependence visible; the zero-fallback losses establish that fallback behavior
is not required for every regression.

## Count timing in 420612

S0 bids 30 in blanks; S1/S3 defend. All lineups play the same first trick:
S0 4-0, S1 2-0, S2 6-0, S3 0-0. S3 wins one point and leads trick two.
The first divergent move is phone S3's 1-1 versus candidate S3's 2-2.
Neither decision falls back. This is a reproducible fork in the trajectory;
the eventual outcome also depends on subsequent choices.

| Trick | All-phone winner | Points | Candidate-defense winner | Points |
|---|---|---:|---|---:|
| 1 | S3 defender | 1 | S3 defender | 1 |
| 2 | S0 bidder | 6 | S3 defender | 1 |
| 3 | S0 bidder | 11 | S1 defender | 1 |
| 4 | S0 bidder | 1 | S3 defender | 6 |
| 5 | S1 defender | 11 | S0 bidder | 21 |
| 6 | S1 defender | 6 | S0 bidder | 1 |
| 7 | S1 defender | 6 | S0 bidder | 11 |

Candidate defenders win the first four tricks for nine points. On trick five,
S3 leads 5-3; S0 plays 5-5; S1 follows with 6-5; S2 discards 6-4. S0 takes 21
points. The final 5-0 lead collects another 11, and the declaring side makes
33. With all-phone play, defenders capture the later count and hold S0/S2 to
18. This suggests examining timing of count exposure and partner control,
without attributing the whole loss to a single initial tile.

## Opposite role outcomes on one deal

420619 improves from 25 to 35 when the candidate partnership declares, but
the candidate defenders allow 42 where phone defenders held 25. This seed has
one paired win and one paired loss, hence zero net seed outcome. The two role
comparisons share the same reference and must not be treated as independent
statistical trials. Keep their separate columns visible.

Full lawful decision requests can be reconstructed from original hands and
the saved public prefixes in `random-420600-699/seeds/`. The referee's complete
deal is never passed to a player.
