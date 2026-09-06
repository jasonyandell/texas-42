# Bid-30 random campaign results

Exploratory executed-policy comparison; generated contracts, no auction evaluation.

Completed **35/100 seeds**, **105 games**. Fresh comparisons: **4 wins, 7 losses, 53 ties** across 32 seeds. Make/set is the only ranking criterion.

| Fresh role | Wins | Losses | Ties |
|---|---:|---:|---:|
| Candidate declaring | 2 | 4 | 26 |
| Candidate defending | 2 | 3 | 27 |

Phone references made 23/32 fresh contracts. Recorded fallbacks: 18/1935 nonforced decisions. Longest decision 13.906s; longest four-play trick 22.155s.

Completed runner slices used 13.22 minutes; three games run concurrently within one seed. An active slice's time is added when it closes.

Downside monitor: 81/32 (pause threshold 20, minimum ten fresh seeds; see CAMPAIGN.md for assumptions).

Points below are diagnostic. Every number below 30 is an equally complete set; every number at least 30 is a make.

| Seed | Fresh | Bidder / trump | Phone declaring points | Candidate declaring points | Phone points vs candidate defense | Candidate W/L/T | Fallbacks |
|---|---|---|---:|---:|---:|---|---:|
| 420600 | yes | S3 / 3 | 42 | 42 | 42 | 0/0/2 | 2 |
| 420601 | development | S0 / 2 | 42 | 31 | 42 | 0/0/2 | 0 |
| 420602 | development | S3 / 1 | 4 | 23 | 36 | 0/1/1 | 0 |
| 420603 | development | S1 / 3 | 25 | 24 | 15 | 0/0/2 | 0 |
| 420604 | yes | S1 / 6 | 42 | 42 | 36 | 0/0/2 | 0 |
| 420605 | yes | S0 / 4 | 34 | 35 | 35 | 0/0/2 | 2 |
| 420606 | yes | S1 / 6 | 39 | 40 | 41 | 0/0/2 | 0 |
| 420607 | yes | S2 / 3 | 41 | 26 | 41 | 0/1/1 | 2 |
| 420608 | yes | S2 / 0 | 19 | 19 | 25 | 0/0/2 | 0 |
| 420609 | yes | S0 / 3 | 41 | 41 | 41 | 0/0/2 | 0 |
| 420610 | yes | S0 / 0 | 41 | 41 | 40 | 0/0/2 | 0 |
| 420611 | yes | S2 / 1 | 20 | 20 | 20 | 0/0/2 | 0 |
| 420612 | yes | S0 / 0 | 18 | 18 | 33 | 0/1/1 | 0 |
| 420613 | yes | S2 / 5 | 30 | 42 | 30 | 0/0/2 | 0 |
| 420614 | yes | S1 / 5 | 36 | 42 | 42 | 0/0/2 | 2 |
| 420615 | yes | S1 / 6 | 41 | 26 | 30 | 0/1/1 | 0 |
| 420616 | yes | S2 / 1 | 41 | 42 | 41 | 0/0/2 | 0 |
| 420617 | yes | S3 / 2 | 41 | 42 | 41 | 0/0/2 | 0 |
| 420618 | yes | S2 / 0 | 41 | 41 | 42 | 0/0/2 | 4 |
| 420619 | yes | S0 / 6 | 25 | 35 | 42 | 1/1/0 | 0 |
| 420620 | yes | S3 / 1 | 42 | 42 | 42 | 0/0/2 | 0 |
| 420621 | yes | S2 / 3 | 19 | 24 | 25 | 0/0/2 | 0 |
| 420622 | yes | S3 / 5 | 35 | 35 | 24 | 1/0/1 | 2 |
| 420623 | yes | S0 / 2 | 41 | 40 | 41 | 0/0/2 | 0 |
| 420624 | yes | S1 / 1 | 24 | 25 | 19 | 0/0/2 | 0 |
| 420625 | yes | S0 / 1 | 30 | 20 | 20 | 1/1/0 | 0 |
| 420626 | yes | S3 / 3 | 25 | 19 | 25 | 0/0/2 | 0 |
| 420627 | yes | S2 / 5 | 26 | 26 | 26 | 0/0/2 | 0 |
| 420628 | yes | S0 / 2 | 31 | 42 | 41 | 0/0/2 | 0 |
| 420629 | yes | S0 / 6 | 34 | 41 | 41 | 0/0/2 | 0 |
| 420630 | yes | S2 / 1 | 26 | 31 | 31 | 1/1/0 | 0 |
| 420631 | yes | S3 / 6 | 35 | 29 | 35 | 0/1/1 | 0 |
| 420632 | yes | S0 / 2 | 34 | 34 | 34 | 0/0/2 | 2 |
| 420633 | yes | S0 / 2 | 35 | 40 | 41 | 0/0/2 | 2 |
| 420634 | yes | S3 / 4 | 41 | 41 | 41 | 0/0/2 | 0 |

Checkpoints preserve all 28 decisions per game. Source/binary identities and the complete protocol are pinned in manifest.json. Seeds 420601–420603 are excluded from fresh random-panel evidence because they were previously examined; 420601 now uses the campaign's declaration heuristic rather than its earlier fixed sixes contract.
