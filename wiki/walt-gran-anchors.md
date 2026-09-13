[Home](Home.md) · owns: the 6-4 problem — the Gran anchors G1/G2/G3 and the synthetic lock, the waking seat's first real hand, exact indifference and the live seat's tie-break, obligation O5 and both void-aware inner-belief implementations, and the branch status of the whole program · Sources: walt/briefs/MORNING-2026-09-05.md (9d6a5a2e); walt/probes/gran/{README.md,g1.receipt.txt,g2g3.receipt.txt,summary-replay.txt,summary-driven.txt} (32aa14f1, 8174fa83); walt/probes/waking/{README.md,summary.txt} (PR #54 93d99563); kanban/backlog/gran-anchor-reconstruction.md; kanban/done/waking-seat-census.md; walt/SCENARIO-PLAYER.md Def 3.2 / Def 4.3 / O5; walt/walt/src/solver/{act,policy,mod,waking,wakeup,inner_belief,selection}.rs; walt/walt/src/rules/domino.rs; experiments/partnership/{INNER-BELIEF.md,REPORT.md,SESSION-STATUS.md,fixtures.json}; experiments/partnership/campaigns/{foundation-battery,default-partner-battery}/RESULTS.md (dbcc698f, e310e6ff, 0b65e5b1); branch walt-g1-l2 (walt/probes/gran/level2_g1.txt, level2_g2.txt, level2_lock.txt, synthetic_lock.receipt.txt; tip 6abdd78f); branch walt-o5 (walt/probes/o5/README.md, walt/kanban/backlog/inner-voids.md, walt/walt/tests/solver_inner_voids.rs; tip 2981e090)

# The 6-4 problem — the Gran anchors

**EXPLORATORY tier throughout.** Everything on this page sits below the corpus, kernel, exchange-adjudicated and rob-receipt tiers and is cited by nothing above it. Sampled numbers are estimates, never receipts. "Exact" below always means *exact over a stated support given a stated field policy* — an exact value of one modeled player over its own belief, not an exact root value of the game. A number becomes quotable as a result only through the gate file or receipt that pins it; where none exists this page says "probe record, not gate-pinned". Corrections and withdrawals are kept in place, as the records keep them. Repository state is as of 2026-09-07 (c00717d1); fresh measurements are dated 2026-09-12.

## In one paragraph

In a live game on the phone, Jason bid 30 on sixes and his robot partner — the live walt seat, which the game's screenshots name **Gran** — held the ten-count 6-4 and played the 6-2 instead at trick 1. The hand was set 25–17. On another hand the same seat, having already been forced to give up the 6-4, showed a review panel where every option read 100%. The question Jason asked was whether a partner who *models its partner as a thinking mind* would have released the 6-4. The two hands were reconstructed tile-by-tile from the screenshots and validated by the rules engine (no game seed needed), and three different walt players were put in Gran's chair on the real hand. The answer is not what the question expected: at trick 1 the choice is a near-tie that different samplers decide differently, and no number "answers" it; the level-2 player hoards *more* than the live seat, not less; and when the seat is exactly indifferent — every play makes the bid in every world it considers — the objective P(make) is pinned at 1, no model of the partner can reach the decision through it, and the play falls to a deterministic tie-break that discards tiles in ascending index order, which keeps the 6-4 (index 25 of 0..27) almost to the end. The "random chance" Jason saw is a coin that always lands the same way. The levers are the objective and the tie-break, not another rung of partner modelling. Alongside, obligation O5 (the modeled minds' no-void belief) was measured on a branch and found to be a large exact bias — and whether fixing it plays better is unresolved, epoch-dependent, and now carried by two different implementations on two different branches.

For the newcomer: a *fiber* is the set of deals consistent with what a seat can see (see [support-fiber.md](support-fiber.md)); a seat's "belief" here is uniform over that set; *pmake* is the probability the bidding side makes its bid, the objective ruled 2026-08-17 ([walt-seat-play.md](walt-seat-play.md)); a *level-k* player models the other seats as level-(k−1) players; an *epoch* is the declared tuple of sample sizes (n_outer worlds for the seat, n0/n1 for the minds it models) — numbers from different epochs never compose. The vocabulary is fixed on [vocabulary.md](vocabulary.md).

## 1. The human story

Dates 2026-08-23/24. Jason played walt live in Plunge (the phone app; the live seat is the level-1 walt bridge, n = 40 worlds with racing, per [walt-partnership-program.md](walt-partnership-program.md)). Two hands became the specimens:

- **The failed hand.** Jason (declarer) bid 30 on sixes. His partner Gran, holding both the 6-2 and the ten-count 6-4, followed the 6-6 lead with the **6-2** at trick 1. The review panel over that decision read: 40 worlds, 6-2 at 90% (played) against 6-4 at 80%. At trick 3 the panel read 160 worlds, 77% for its pick against 72% for the tile it played (the 6-4, released then). The hand ended **25–17, set** ("Us 25 – Them 17").
- **The made hand.** Jason bid 31 on sixes; the hand ran **36–0, made**. Its trick-4 review panel showed four options **all at 100% on 160 worlds** — the seat was, from its own chair, indifferent among everything it held.

Jason's reading of the two together, in his words (recorded in `level2_lock.txt` on branch walt-g1-l2): "Gran was dumping dominoes and held the 6-4 for no reason other than everything was 100% pmake from that chair. Not because the score was higher — random chance." And his prediction: "100s at L1. not 100s at L2. maybe." His experience of the failed hand: "I felt like I almost lost the game trying to dodge that 6-4."

The three screenshots were archived on 2026-08-24 at `~/data/texas-42/gran-anchors-2026-08-24/` with a `MANIFEST.sha256` (`gran-failed-hand-trick1-40w.png` d21b0d0c…, `gran-failed-hand-trick3-160w.png` f528266a…, `gran-made-hand-trick4-saturation.png` 3262da08…). The level-2 field-stability intake adjudicated the same day carded them as the **Gran anchors G1–G4** (ruling L2-A6; card [[gran-anchor-reconstruction]] opened 2026-08-24). G1 = the failed hand's trick-1 decision; G2 = the made hand's trick-1 decision (the early 6-4 reveal); G3 = the made hand's trick-4 saturation panel; G4 = "mechanism adjudication" on a future real game, reserved for when Jason's tooling captures the seed.

## 2. The anchor discipline

Four rules governed turning a screenshot into a research object, and they are the reason the rest of the page can be trusted at its tier.

1. **Screenshots are discovery artifacts.** They carry a manifest and are never modified; until a validated record was committed they backed nothing (L2-A6, `walt/math/targeted_level2_field_stability_v0.1.md` §1.4).
2. **The "How it went" grid IS the deal.** Plunge's post-hand grid shows every seat's tile at every trick; 28 tiles in play order is the full deal. No game seed was captured when the screenshots were taken and none was needed — the kanban card's own "no seed needed" path is what was used. (Jason's tooling captures seeds now; future anchors are exact by construction.)
3. **Mechanical validation by the rules layer is the bar.** A transcription is accepted only when `rules::replay::replay_hand`, which trusts nothing but the tiles and who played them, re-derives the whole hand: 28 distinct tiles, 7 per seat, every follow legal under the declaration, every trick winner, every trick's count (+1/+6/+11), the team totals and the made/set verdict. A single misread pip breaks the partition and localizes itself.
4. **Honest partiality.** Where the record does not determine something, the runner enumerates the alternatives rather than picking one. G2/G3's residual is stated as 6-way ambiguous, not guessed.

The runner is `walt/walt/src/bin/granrun.rs` (on main since 32aa14f1, 2026-09-04), a VARIANT surface that modifies nothing on the live path. Its four modes are `validate`, `validate-partial`, `replay <fixture> <seat> <out.jsonl>` and `driven <fixture> <out.jsonl>` (usage confirmed 2026-09-12 by running the binary with no arguments). The fixtures are written in rob's receipt grammar only because `rules::receipt::parse` already reads it; they are **not rob receipts** — no engine produced them and they back no claim above the exploratory tier.

Seat map for both anchors: **Y = S0** ("You", Jason, the declarer), **E = S1**, **G = S2** (Gran, the walt seat, Jason's partner), **R = S3**. Teams are S0/S2 = T0 (the screenshots' "Us") against S1/S3 = T1.

## 3. G1 — the failed hand, complete and validated

`walt/probes/gran/g1.receipt.txt`, committed 32aa14f1 (2026-09-04 21:24). Bid 30, sixes, declarer S0. Two screenshots of the same hand gave two independent reads of the grid; nothing was ambiguous and the 28-tile partition closed on the first read.

**The deal** (derived by `granrun validate`; re-derived 2026-09-12 on this machine, identical):

| seat | hand |
|---|---|
| S0 — Y, Jason, declarer | 0-0 2-1 3-3 4-4 6-1 6-3 6-6 |
| S1 — E | 1-1 3-0 4-3 5-1 5-4 6-0 6-5 |
| S2 — G, Gran, the walt seat | 1-0 2-2 4-1 4-2 5-2 6-2 6-4 |
| S3 — R | 2-0 3-1 3-2 4-0 5-0 5-3 5-5 |

**The record** (leader first, then in play order; winner and trick count):

| trick | plays | winner | count |
|---|---|---|---|
| 1 | S0:6-6 S1:6-0 **S2:6-2** S3:3-1 | S0 | +1 |
| 2 | S0:4-4 S1:5-4 S2:4-1 S3:4-0 | S0 | +6 |
| 3 | S0:3-3 S1:3-0 **S2:6-4** S3:5-3 | S2 | +11 |
| 4 | S2:2-2 S3:3-2 S0:2-1 S1:5-1 | S2 | +6 |
| 5 | S2:4-2 S3:2-0 S0:6-1 S1:4-3 | S0 | +1 |
| 6 | S0:6-3 S1:6-5 S2:1-0 S3:5-0 | S1 | +6 |
| 7 | S1:1-1 S2:5-2 S3:5-5 S0:0-0 | S1 | +11 |

Totals T0 25 – T1 17 (tricks 5–2); declaring T0 took 25 against 30 → **set**. An independent cross-check of S2's hand: the trick-3 review panel lists S2's five options as 5-2 / 6-4 / 1-0 / 4-2 / 2-2, exactly the derived hand minus the 6-2 and 4-1 already played.

**Not recovered:** the shaker, the auction, and the match's prior mark ledger. `shaker S3` in the fixture is a placeholder (`replay_hand` never reads it) and the mark lines are fixture-local.

**Validation, measured 2026-09-12 on this machine:** `granrun validate walt/probes/gran/g1.receipt.txt` exits 0 in 0.005 s (subprocess wall including spawn) and prints `VALIDATED: 28 distinct tiles, 7 per seat, every follow legal, every winner and every trick's points re-derived, totals and verdict agree.`

**S2's information sets shrink by trick.** The fiber at each of Gran's seven turns — the number of deals consistent with her hand, the public record and the voids the record has proved — is the reader's first feel for how hidden information collapses inside a hand (from the replay table in `walt/probes/gran/README.md`; the same seven numbers are reproduced independently by the level-2 record on walt-g1-l2 and by the O5 census on walt-o5):

| trick | 1 | 2 | 3 | 4 | 5 | 6 | 7 |
|---|---|---|---|---|---|---|---|
| S2's fiber | 46,558,512 | 432,432 | 17,640 | 8,820 | 300 | 12 | 2 (forced) |
| legal set | 6-2 6-4 | 4-1 4-2 | 1-0 2-2 4-2 5-2 6-4 | 1-0 2-2 4-2 5-2 | 1-0 4-2 5-2 | 1-0 5-2 | 5-2 |

The 6-4 is legal at exactly two of Gran's decisions: trick 1 (she must follow sixes; 6-2 or 6-4) and trick 3 (she holds no threes, so anything goes — and S3 showed void in sixes at trick 1, so the 6-4 trumps the 3-3 lead for a *guaranteed* +11, which is what she did).

## 4. G2 / G3 — the made hand, committed as a validated partial

`walt/probes/gran/g2g3.receipt.txt`, committed 8174fa83 (2026-09-04 21:37). Bid 31, sixes, declarer S0. **The grid shows six tricks, not seven:** the declaring side had 36 against 31 after trick 6 and the app ended the hand. So 24 of 28 tiles are recorded and **the deal is not fully recovered**.

| trick | plays | winner | count |
|---|---|---|---|
| 1 | S0:6-6 S1:6-2 **S2:6-4** S3:2-0 | S0 | +11 |
| 2 | S0:6-1 S1:2-2 S2:5-0 S3:5-1 | S0 | +6 |
| 3 | S0:6-5 S1:2-1 S2:3-2 S3:3-1 | S0 | +6 |
| 4 | S0:0-0 S1:1-0 **S2:3-3** S3:3-0 | S0 | +1 |
| 5 | S0:6-3 S1:4-2 S2:4-3 S3:5-4 | S0 | +1 |
| 6 | S0:6-0 S1:4-0 S2:5-5 S3:1-1 | S0 | +11 |

Prefix 36–0, made. S0 won every trick, so S0 led every trick and S2 always acted third.

**The residual.** The four unplayed tiles are 4-1 4-4 5-2 5-3, one per seat. The trick-4 review panel lists S2's options as 3-3 / 4-3 / 5-2 / 5-5, which pins S2's residual as the **5-2**, so S2's complete hand is known: **6-4 5-0 3-2 3-3 4-3 5-5 5-2** (the fixture carries this as `known S2: 5-2`, a panel-read constraint, not part of the record). The assignment of {4-1, 4-4, 5-3} to S0/S1/S3 is **6-way ambiguous and mechanically undecidable**: none of the three is a six or a blank, and every failure to follow in the prefix was on sixes or blanks, so all six assignments are equally legal. `granrun validate-partial` prints all six (measured 2026-09-12: exit 0, 0.003 s, `VALIDATED over the prefix … The DEAL is NOT recovered: 6 assignments of the residual are equally consistent.`).

**Consequence.** The G2 root (S2's trick-1 decision) and the G3 root (S2's trick-4 decision) are fully determined **as information sets** — a decision at S2 needs S2's own hand (known) plus the public record (known); the three unassigned tiles are exactly what S2 cannot see. What is missing is the *deal*, so no driven whole-hand run and no `replay_hand` validation exist for this anchor. The kanban card stays open on this item and on repointing the intake companion's "Gran-anchor gap" note (`walt/math/targeted_level2_field_stability_v0.1_intake.md`), which has not been done.

One fact about G2 settles a question before any solver runs (recorded d29f043a on walt-g1-l2, but checkable from the main-side fixture): **G2's trick-1 6-4 was forced.** S0 led the 6-6 with sixes trump and the 6-4 is S2's only six, so the legal set at that node is the singleton {6-4}. The counterfactual "she holds the 6-4 at trick 1" is an illegal history, which the fixture loader rejects. G2 therefore does not contain Jason's scenario (holding the 6-4 *and* indifferent); what it contains instead is §6.

## 5. Three players on one hand

Three walt players were put in Gran's chair on G1 with the other three seats playing exactly what the record says. They run at three different, non-composing epochs, which is the point of reading them together.

### 5.1 The waking seat — replay and driven (main, 8174fa83)

`granrun replay probes/gran/g1.receipt.txt S2 …` seats `solver::waking::WakingSeat` (§7) at S2 at the `probes/waking` live epoch (σ0 = Level0{n0=2} under `ActConfig::interactive`, σ1 = Level1{n_outer=4, n0=2}). Records: `walt/probes/gran/g1-replay.jsonl`, `summary-replay.txt`. Probe record, not gate-pinned.

| trick | d | fiber | record | σ0 | played | path | wake evidence | wall |
|---|---|---|---|---|---|---|---|---|
| 1 | 3 | 46,558,512 | **6-2** | 6-4 | **6-4** | no-wake-budget-exhausted | sampled-open, 24 worlds, rival 6-2 | 12.550 s |
| 2 | 7 | 432,432 | 4-1 | 4-1 | 4-1 | no-wake-budget-exhausted | sampled-open, rival 4-2 | 5.298 s |
| 3 | 11 | 17,640 | 6-4 | 6-4 | 6-4 | no-wake-budget-exhausted | sampled-open, rival 1-0 | 2.756 s |
| 4 | 13 | 8,820 | 2-2 | 2-2 | 2-2 | no-wake-budget-exhausted | sampled-open, rival 1-0 | 1.581 s |
| 5 | 17 | 300 | 4-2 | 1-0 | **4-2** | **wake** | exact-sigma1-selects-rival, 300 worlds | 2.849 s |
| 6 | 23 | 12 | 1-0 | 1-0 | 1-0 | no-wake-settled | exact-sigma1-tie | 9.550 ms |
| 7 | 26 | 2 | 5-2 | 5-2 | 5-2 | forced | — | 2 µs |

Agreement with the record **6 of 7** (only trick 1 differs). Wake rate 1 of 6 checked decisions. Total **25,044,233 µs = 25.04 s**: baseline (σ0 act) 11,021,108 µs (440‰), wake check 13,433,307 (536‰), escalation 589,818 (23‰); trick 1 alone 12.55 s. σ1 action cache 9,489 entries.

Two things the record says loudly and this page repeats:

- **At trick 1 the waking seat plays the 6-4 — but the wake did not fire, so the waking machinery did not cause it.** σ0 (`solver::act`, the same call the live bridge makes) already picks 6-4, by route `unresolved-level1` — act's own evidence process did not settle at δ, and the 6-4 comes from the level-1 fallback ranking, not a settled selection. The 46.5-million-world fiber sent the wake check down the sampled route, which spent its 24 paired worlds and returned honestly open. So the trick-1 flip is an **epoch/baseline difference** between this σ0 and whatever the live Plunge seat ran when the screenshot was taken (40 sampled worlds, 6-2 at 90% vs 6-4 at 80%); the two samplers do not compose. This run does not demonstrate that partner-modelling fixes the 6-4 problem.
- **The one real wake landed at trick 5 and agreed with the human.** Fiber 300 is under the exact cap, so the check ran the exact route and positively selected the rival (`exact-sigma1-selects-rival`); the escalation returned `exact-survivors / provably-useless / exact-argmax` and moved the play from σ0's 1-0 to 4-2 — what the real Gran played. Its 589,818 µs went to the screen machinery, not the σ1 values: steering 237,596 µs, rung-e2 198,873, baseline-σ0 152,301, stage4-σ1 968, stage4-ladders 17, rung-e1 23.

**Driven mode** (`granrun driven`: the waking seat at all four chairs from the G1 deal; `g1-driven.jsonl`, `summary-driven.txt`): **T0 26 – T1 16, still set** against 30, one point better than the real 25. 28 decisions, 9 forced, **zero wakes** (14 `no-wake-budget-exhausted`, 5 `no-wake-settled`: 4 `exact-sigma1-selects-baseline`, 1 `exact-sigma1-tie`); agreement with σ0 28/28; total 166,112,970 µs = 166.11 s (baseline 677‰, wake check 322‰, escalation 0), trick 1 alone 102,824,446 µs = 619‰ of the run; σ1 cache 14,414. The line diverges from the record at the opening lead (S0 opens 4-4, not 6-6), so the 26 is not a partner-only counterfactual; the replay run is the one that isolates the partner's seat.

### 5.2 The level-2 player at Gran's seat (branch walt-g1-l2 only)

The 2026-08-17 level-2 ladder binary (`walt/walt/src/bin/level2.rs`; epoch of record n_outer 200, n1 8, n0 4, 18 rayon threads, outer seed `OUTER_SEED ^ t` with `OUTER_SEED = 0x8CB92BA72F3D8DD7`, inner seed `0x243F6A8885A308D3`, no O5 flag) gained a `fixture` mode on the branch (`level2 fixture <anchor> <SEAT> <trick> <ply> --worlds N --n1 8 --n0 4 [--field-level 0] [--viewer-hand …] [--sub …]`; `--worlds` at or above the support runs the full support and says so). Record: `walt-g1-l2:walt/probes/gran/level2_g1.txt` (commits 662c9566, 92e89361, e96b602f, 2026-09-04). Its fibers reproduce the replay table's seven numbers exactly, an independent check that the same seven decisions were faced. Probe record, not gate-pinned; **not on main**.

The question put to it (Jason, 2026-09-04): the real game exactly as it played out, Gran's seat, level 2 — does it give up the 6-4 or not. **It does not. At both nodes where the 6-4 was legal, the level-2 player kept it.**

| trick | S2 is | fiber | rows | record | level 2 | 6-4 legal? |
|---|---|---|---|---|---|---|
| 1 | 3rd | 46,558,512 | sampled 200 | 6-2 | 6-2 (815 vs 730‰) | yes — **held** |
| 2 | 3rd | 432,432 | sampled 200 | 4-1 | 4-1 (765 vs 745‰) | no (must follow fours) |
| 3 | 3rd | 17,640 | **exact** | 6-4 | **5-2** | yes — **held** |
| 4 | leads | 8,820 | exact | 2-2 | 2-2 (589‰; next 485) | already gone |
| 5 | leads | 300 | exact | 4-2 | 5-2 (516 vs 513 vs 486‰ — one world of 300) | already gone |
| 6 | 3rd | 12 | exact | 1-0 | 1-0 / 5-2 tie (1/12 each) | already gone |
| 7 | 2nd | 2 | forced | 5-2 | 5-2 | already gone |

**Trick 3, exact over all 17,640 void-consistent deals** (wall 160.970 s), given the level-1 field policy:

| play | exact | ‰ |
|---|---|---|
| 1-0 | 10753/17640 | 609 |
| 2-2 | 1273/2520 | 505 |
| 4-2 | 157/280 | 560 |
| **5-2** | 1283/1960 | **654** |
| 6-4 | 251/392 | 640 |

Level 2 declines a guaranteed +11 (S3 is void in sixes; the 6-4 cannot be overtrumped) by 14‰ of its own policy's value. That is a statement about the level-2 player over this support, not about 42.

**Correction history, carried verbatim because it is the methodological lesson of the round.** The 200-world *sampled* run at the same node (wall 7.419 s) put the 6-4 first: 6-4 695, 1-0 690, 5-2 660, 4-2 655, 2-2 500. A 200-world sample here has a standard error near 35‰; the 55‰ error on the 6-4 line is ordinary noise, but it inverted the argmax and would have supported exactly the wrong headline. The exact row supersedes it. **Whenever the fiber allows it, run the full support.**

**The trick-1 refinement ladder.** Nested samples (the draw stream is a pure function of seed and boundary, so 50 ⊂ 100 ⊂ 200 ⊂ 800 — genuine refinement, not four independent draws), both levels, the level-1 control drawing the *same* outer worlds with only the field model dropped a rung:

| worlds | L2 6-2 | L2 6-4 | L2 margin | L1 6-2 | L1 6-4 | L1 margin |
|---|---|---|---|---|---|---|
| 50 | 39/50 = 780 | 17/25 = 680 | 100 | 23/25 = 920 | 41/50 = 820 | 100 |
| 100 | 41/50 = 820 | 73/100 = 730 | 90 | 91/100 = 910 | 83/100 = 830 | 80 |
| 200 | 163/200 = 815 | 73/100 = 730 | 85 | 9/10 = 900 | 83/100 = 830 | 70 |
| 800 | 639/800 = 798 | 621/800 = 776 | **22** | 347/400 = 867 | 343/400 = 857 | **10** |

Eight rows, every one plays the 6-2; the argmax is stable, the **margin decays monotonically** at both levels and is nearly gone at 800. Trick 1 is **unsettled and trending toward indifference**, and the level upgrade is not what decides it — level 2's values sit about 100‰ below level 1's throughout (a level-1 field defends better) but the order never changes. Trick 1 remains an estimate; no exact row exists there (46.5 M is past the 30,000,000 materialization cap).

A secondary counterfactual in the same record (Part 3): from S0's chair at his trick-2 lead, with the 6-4 landed at trick 1 instead of the 6-2, the level of every lead rises by 163–343‰ at 800 worlds and the spread between best and worst lead falls from 210‰ to 84‰ — the shape of "trying to dodge that 6-4", seen from the chair that can see what landing it buys.

### 5.3 The partnership program's L2 at the G1 root (main, experiments/partnership, 2026-09-06)

The partnership program ([walt-partnership-program.md](walt-partnership-program.md)) took the G1 trick-1 root as a fixture (`experiments/partnership/fixtures.json`, sourced to `walt/probes/gran/g1.receipt.txt:24-32`) and ran its three native profiles over the *same* 40/8 outer evidence with a new experiment seed (REPORT.md "Gran's actual information root"):

| profile | make after 6-2 | after 6-4 | choice |
|---|---|---|---|
| Baseline (L1) | 33/40 | 33/40 | 6-2, **index tie** |
| Partner-only L2, n1 = 2 | 34/40 | 37/40 | **6-4** |
| Full L2, n1 = 2 | 38/40 | 36/40 | 6-2 |

Changing only the partner model changes the ranking (0.790 s for the partner-only decision); modelling the opponents too reverses it again. The report is careful that this isolates neither revelation from score effects nor the correctness of the partner model. The baseline row is the tie-break of §6.3 observed on a *sampled* tie. G3's saturation root stayed a four-way sampled tie under all three profiles. The report also states that "the independent phone run selected 6-4 as well" — under the archived WASM's own settings — while the live screenshot shows the deployed seat playing 6-2 at 90% vs 80%; SESSION-STATUS notes the deployed phone version was not independently verified, so which binary Gran actually ran is not known.

### 5.4 Trick 1 is decided by epoch and sampler, not by model level

No single record states this; it is the cross-record reading of everything above plus the O5 rows (§8), all at different, non-composing epochs:

| player | epoch | trick-1 choice |
|---|---|---|
| live Plunge seat (screenshot) | level 1, 40 worlds | 6-2 (90% vs 80%) |
| waking seat σ0 = `solver::act` | Level0{n0=2}, `ActConfig::interactive` | 6-4 (route unresolved-level1) |
| void-blind level 1 (o5level1, walt-o5) | n_outer 50, n0 8 | 6-4 at 880‰ |
| void-blind level 1 (L2 record's control; o5 cross-check) | n_outer 200, n0 4 | 6-2 at 900‰ |
| level 2 (walt-g1-l2) | 200 / 8 / 4 | 6-2, 815 vs 730‰ (22‰ margin at 800 worlds) |
| partnership baseline | 40 / 8 | 33/40 vs 33/40, 6-2 by index |
| partnership partner-only L2 | 40 / 8 / n1 2 | 6-4 |
| partnership full L2 | 40 / 8 / n1 2 | 6-2 |

The choice was close enough that the epoch decides it. No number on this page "answers" trick 1, and the page does not claim that holding the 6-4 there is wrong play. Why level 1 alone flips between 50/8 and 200/4 is not isolated by any record (open).

## 6. Exact indifference and the deterministic coin

### 6.1 G2 goes exactly locked from trick 3

Record: `walt-g1-l2:walt/probes/gran/level2_g2.txt` (d29f043a, 2026-09-05 00:15). The partial anchor was loaded with `--viewer-hand 6-4,5-0,3-2,3-3,4-3,5-5,5-2` — S2's panel-pinned hand — and **no completion of the residual was used anywhere**: the three ambiguous tiles are part of what S2 cannot see and are sampled like any other unseen tile. Same epoch as §5.2. Not on main.

| trick | legal | fiber | rows | record | level 2 | level 1 control |
|---|---|---|---|---|---|---|
| 1 | 1 | 46,558,512 | sampled | 6-4 | forced (value ≈ 685–697‰ at 200–800 worlds; not a lock) | forced (710–737‰) |
| 2 | 6 | 36,036 | L1 exact; L2 800w | 5-0 | 5-0 / 5-5 tie at 1000 (sampled saturation) | **5-0 alone at 1/1**; others 984–994 |
| 3 | 5 | 6,930 | **exact, both** | 3-2 | 3-2 / 4-3 / 5-2 / 5-5 all **1/1**; 3-3 at 996 | identical tie set |
| 4 | 4 | 280 | **exact, both** | 3-3 | **all four at 1/1** | identical |
| 5 | 3 | 60 | **exact, both** | 4-3 | all three at 1/1 | identical |
| 6 | 2 | 12 | exact, both | 5-5 | 5-5 at 1/1 alone; 5-2 at 3/4 = 750 | identical |
| 7 | 1 | — | forced | 5-2 | forced | forced |

Walk agreement with the record 7/7 (two forced, three inside exact ties, two unique argmaxes). **Trick 4 is the G3 panel** — the four legal tiles are exactly the four the screenshot lists, and "all 100% on 160 worlds" is reproduced and sharpened: every option certain over all **280** deals of the complete support, at both levels. At trick 2 the seat is not yet indifferent: the level-1 exact row has a unique certain play, the 5-0 — the tile the real Gran led. So the lock closes between trick 2 and trick 3, and it is not monotone (trick 6 has a strict best play again).

Not available: the level-2 exact row at trick 2 — the run **died on its wall-clock budget after 7,595,251,011 nodes and 98,391,159 π evaluations in 560.558 s**; the level-2 answer there is an 800-world estimate whose 5-0/5-5 tie at 1000 is sampled saturation against a 36,036-deal support, exactly the artifact class the 2026-08-17 ladder flags.

This is indifference *under a field model*: every deal the seat considers possible makes the bid whatever she plays. It is not a laydown claim (which would need ∀σ; see [walt-counted-belief-era.md](walt-counted-belief-era.md)).

### 6.2 Why a modeled partner cannot reach the decision

The record's own statement: *when the seat is exactly indifferent, neither level can release the 6-4 for the partner's sake, and level 2 is worth nothing over level 1.* Both maximize P(make); at an exact tie at 1 every option is optimal and the objective has no gradient; a modeled partner's reading of the play can only reach the decision *through* P(make), and P(make) cannot exceed 1. Two consequences:

- **Refinement cannot rescue it.** The 2026-08-17 discipline — saturation ties are artifacts to refine, never to break by index — is about *sampled* ties, where adding worlds separates candidates. These ties are exact over the whole support; there are no worlds to add.
- **So at an exact tie the play falls out of tile order** — whatever the surface's tie rule says. `level2.rs` deliberately carries no tie refinement, so which tied tile it names is an artifact of ordering, not a decision. The live seat's rule is §6.3.

### 6.3 The tie-break: `TieRule::LowestTileIndex` (source reading, verified against main, never executed at a tied node)

Read from main's source on 2026-09-12 (c00717d1); the branch record's reading with one attribution correction.

1. **The declaration.** `walt/walt/src/solver/act.rs` declares `tie_rule: TieRule::LowestTileIndex` in the two constructors that fix act's σ0 field identity — `continuation_tuple` (line 288) and `act_field_spec` (line 315). (The branch record attributes the declaration to `ActConfig::interactive`; that struct has only `n_outer_frozen 8, n0_frozen 2, world_cap 128, exact_cap 2000, fallback_n_outer 200, fallback_n0 8` and no `tie_rule` field — the declaration lives in the field specs.) `TieRule` is defined in `solver/policy.rs` (lines 322–327): `FirstInPreference` / `LowestTileIndex` ("ties break toward the lowest stable tile index"), and it is part of a policy's frozen identity.
2. **The route.** On an honest exact tie among the frozen candidates, `route_result` returns `RouteChoice::Fallback { among, route: ActRoute::ExactTieLevel1 }` (act.rs ≈205–215; the enum's doc: "Honest exact tie (`winner:null`): level-1 rank among the TIED maxima. An ordering choice, not a settlement").
3. **The fallback ranking.** The `Fallback` arm (act.rs ≈413–451) calls `level1_evaluate` over the tied set at `cfg.fallback_n_outer = 200`, `cfg.fallback_n0 = 8`, then chooses with `best_of(&opts, viewer.team() == Team::T1)`.
4. **The reduce.** `solver::best_of` (`solver/mod.rs` 1500–1517; doc comment: "Argmax (or argmin for T0 seats) over evaluated options, **first-listed on exact ties — the ascending-tile-order convention of the whole stack**") replaces the incumbent only when a candidate is *strictly* better. A residual tie keeps the first candidate, and candidate order is legal-tile order, ascending by domino index.
5. **The index.** `walt/walt/src/rules/domino.rs`: `index = hi·(hi+1)/2 + lo`. The 6-4 is **25** of 0..27 — only the 6-5 (26) and 6-6 (27) sit above it. The other count tiles: 5-5 = 20, 5-0 = 15, 4-1 = 11, 3-2 = 8. (The census JSONL confirms the encoding: S2's hand is stored as `[1,5,11,12,17,23,25]` = 1-0 2-2 4-1 4-2 5-2 6-2 6-4.)

**A tied seat therefore discards almost anything before it discards the 6-4.** Lowest-index-first is the worst possible rule for this failure, since count tiles are high-index by construction. The same convention is stack-wide: SCENARIO-PLAYER Def 3.2 gives modeled minds "fixed low-index-first evaluation order, strict improvement to replace", and `solver/selection.rs`'s `Rule::Fixed` (2026-09-06) resolves exact ties to the lowest tile index.

**What this is not.** No run of `solver::act` at a tied node exists; there is no harness to drive act at an arbitrary constructed information state, and G2's exact ties are the natural specimen. The claim is about the source and should be checked against an execution before it is leaned on. The partnership baseline row of §5.3 (33/40 vs 33/40 → 6-2) is the same mechanism observed on a *sampled* tie.

### 6.4 The design conclusion

From the MORNING readout (item 4) and `level2_g2.txt`: **another rung of partner modelling is not the remedy.** The levers are (a) the **objective** — P(make) is Boolean and pins at 1, so it goes flat exactly where the partner's information problem is sharpest; a margin term or a partner-information term would not — and (b) an explicit **tie-break at exact indifference** (for instance preferring to release count when tied). Neither is a feature; neither has a design, an intake or a ruling. Jason's call (C), §10.

### 6.5 The synthetic lock — the one place the level upgrade moves the tile

Record: `walt-g1-l2:walt/probes/gran/level2_lock.txt` and `synthetic_lock.receipt.txt` (37d3392d and 6abdd78f, 2026-09-05). **A step below the anchors:** constructed by hand from Jason's spoken description, no screenshot, never G4. Rules-layer validated by `granrun validate`. Not on main.

The construction: threes trump, bid P(36), S0 leads every trick. S0 = 3-3 6-3 5-3 4-3 3-2 (the top five trumps), 2-2, and the **6-1** "atrocious off"; S2/Gran = **6-4** plus zero-count junk 0-0 1-0 1-1 2-0 2-1 4-0, void in trump, the 6-4 her only six; S1 = 3-0 4-1 4-4 5-0 5-2 6-0 6-6; S3 = 3-1 4-2 5-1 5-4 5-5 6-2 6-5. Gran is never forced to release the 6-4; if she hoards it to trick 7 she must follow S0's 6-1 with it and S1's 6-6 takes it. Hoard line 31–11 (set); release line 41–1 (made); 31 < 36 ≤ 41. The recorded line is the hoard line, with Gran's discards in ascending index order (0-0, 1-0, 1-1, 2-0, 2-1, 4-0) — the artifact under test written into the record on purpose. Opponents play the lowest legal tile by index (fixing the prefix only). Bid invariance checked: the trick-4 exact level-1 table is identical at bids 32–36.

**What did not happen: saturation.** No node at either level reads 1000 across the board except trick 6, the last-chance node (6-4 at 1000 vs 4-0 at 222, both levels). So Jason's literal "100s at L1, not at L2" was untestable here — the precondition failed, not the mechanism. Two further constructions (loose count moved to the declaring side; all loose count in Gran's hand at bid 32) refused to saturate as well.

**The right statistic is the release margin** — the 6-4's value minus the best alternative — and how it moves from level 1 to level 2:

| trick | rows | L1 margin | L2 margin | shift | argmax L1 → L2 |
|---|---|---|---|---|---|
| 1 | 200 | −55 (vs 1-1) | −25 (vs 4-0) | +30 | 1-1 → 4-0 |
| 2 | 200 | +35 | +20 | −15 | 6-4 → 6-4 |
| 3 | exact 6,930 | +0 | +40 | +40 | 6-4 → 6-4 |
| 4 | exact 1,260 | +39 | +173 | +134 | 6-4 → 6-4 |
| 5 | exact 210 | −39 | +4 | +43 | **2-1 → 6-4** |
| 6 | exact 9 | +777 | +777 | 0 | 6-4 → 6-4 |

Trick 5, exact over all 210 deals at both levels: L1 holds (2-1 59/105 = 561, 6-4 11/21 = 523, 4-0 89/210 = 423); L2 releases (6-4 107/210 = 509, 2-1 53/105 = 504, 4-0 7/15 = 466). The margin is 5‰ — one world of 210 — a direction, not a verdict, but the only place in three constructions where the upgrade moves the tile across the argmax. Trick 4, exact over all 1,260: 6-4 556 → 596 while 2-0 490 → 365, 2-1 479 → 386, 4-0 516 → 423 — the release line is the only one that rises; under a level-1 field the modeled declarer handles his 6-1 worse while the ten is loose. Cost to the partner (S0's trick-2 lead, level 2, 200 worlds): best lead 975 → 1000, spread across six leads 290 → 10‰, the 6-1 **685 → 990 (+305)**.

**The withdrawal, reported as a withdrawal.** The first commit of this record (37d3392d) headlined "saturation and the 6-4 mattering are mutually exclusive". Within one fixed field model, saturation does imply indifference. But that is a single-model statement; it does not reach across field models and so does not rule out Jason's case (saturated under one field model, not under another). The over-claim was withdrawn in-record in 6abdd78f five minutes later; the trick-5 flip and the trick-4 asymmetry are evidence *for* his mechanism, not against. Testing the literal prediction would need a position where the declarer's trick-7 outcome depends only on the modeled declarer's *skill* with the 6-1, not on the deal — which first needs a measurement of how level-0 and level-1 modeled declarers handle that decision (not run; Jason's call (B)).

## 7. The waking seat

`solver::waking` + `bin/waking_bridge`, PR #54 (93d99563, 2026-08-25); gated by `walt/walt/tests/solver_waking.rs` (9 `#[test]` functions, counted 2026-09-12) plus two `compile_fail` doc locks in the module. On main; **never a default** (CE-A7 / §20.16 fence). The era context is on [walt-calculated-evidence.md](walt-calculated-evidence.md); the instrument row on [walt-instruments.md](walt-instruments.md).

**What a thinking-teammate model is here.** Per non-forced decision:

1. **Baseline (CE thread).** `solver::act` runs exactly as the live bridge runs it; the σ0 choice is *always* computed and is the fallback for every path. A forced play plays immediately.
2. **Wake check (hard-budgeted).** A paired detection between act's tile and its strongest rival under the declared epoch pair σ0 = Level0{n0=2} — asserted to be the *same* field act evaluates against — and σ1 = Level1{n_outer=4, n0=2}, frozen candidates at schedule [8, 2] (`ActionRule::PinnedThenLevel1`). Fibers at or under the exact cap take the exact route (complete-fiber exposure + `exact_paired_detection`, from `solver::wakeup`); larger fibers take `sampled_paired_detection` over 24 paired worlds — a resource limit, never a settlement rule.
3. **The wake rule: positive evidence only.** The seat wakes exactly when the σ1 leg *positively selects the rival*. σ1 selecting the baseline, an exact σ1 tie, or a within-budget-unsettled probe are all no-wake: **unsettled means play σ0**, recorded as such. Degradation is the current player, never fake certainty.
4. **Escalation (L2 thread).** `solver::targeted::targeted_root` over one frozen focal candidate per legal action; a δ-settled selection or a strict σ1 argmax among exact survivors is played; every open state and every typed refusal falls back to σ0, recorded as a fallback.

**Type locks.** A `WakeEvidence` has no public constructor — a wake is a crossing witness, never a construction; a `RecordedFallback` has no numeric accessor — a refusal-backed fallback can never be read as an evaluation. Both are `compile_fail` doc tests in `waking.rs`.

**Declared configuration** (`WakingConfig::live`, `waking.rs` ≈248–262; identical in `waking_bridge` and `granrun`, so their censuses compose): act at `ActConfig::interactive` (world cap 128, exact cap 2000, fallback 200×8; δ_run 1/100 per hand); σ1 4/2; wake budget 24 paired worlds; exact wake route at fiber ≤ **1024**; escalation exact fiber cap 4096, baseline prefix 128, E3 prefix 24; waking risk δ = 1/20 per hand under `wake:`-prefixed scopes asserted disjoint from act's; ε = 1/20. Seeds `WAKING_DRIVEN_SEED = 0x51EE_D42A_11FE_600D`, `WAKING_DECLARE_SEED = 0x7A3E_9B21_5C48_D6F1`; no wall clock in the value path — choices are a pure function of the hand, walls vary. **The `l2_controller` probe's epoch (σ0 n0 = 8) is different and does not compose.**

**The 64 → 1024 retune.** After the first smoke hand priced the routes: under the telescoping risk convention the 24-world sampled probe needs a net pivotal margin of roughly 10–17 worlds out of 24 to settle — effectively uncrossable, so its honest outcome is almost always `no-wake-budget-exhausted` — while the exact route settles always and costs fractions of a second up to fiber ~1000. **The wake gate's real coverage is the exact route**; the sampled budget stays as the declared cost bound above the cap.

**The census** (`walt/probes/waking/`, 2 driven hands, 56 decisions, live epoch; kanban card [[waking-seat-census]] opened and closed 2026-08-25). Probe record, not gate-pinned:

| phase | µs | ‰ |
|---|---|---|
| σ0 baseline (act) | 206,971,014 | **729** |
| wake check | 76,474,588 | **269** |
| escalation | 454,039 | **1** |
| total | 283,899,641 | |

Tricks 1–2 alone: 202,534,607 + 60,372,875 = 262.9 s of 283.9 s = **926‰**. Wake rate 1/34 checked decisions (the one wake at trick 5, exact route, moved the played tile; agreement with σ0 55/56). The exact route settled 13 of 13 checks it reached (10 ties, 2 baseline-confirms, 1 rival); all 21 sampled checks above fiber 1024 stayed open. Per-decision total p50 14,337 µs, p90 21,665,288, max 68,829,911; fibers p50 780, p90 8,588,580, max 399,072,960. Inside the one escalation: rung-e2 240,300 µs (529‰), steering 149,329 (328‰), baseline-σ0 63,223 (139‰), stage4-σ1 1,130 (2‰). **Affordability verdict: not affordable as-is** — minutes of decision compute per natural hand. The scaled census was deliberately skipped on Jason's ruling; the profile's job was attribution, and it targeted the 2026-08-25 speed campaign (#53/#55/#56), whose conclusion was that the modeled minds are the bill.

The MORNING brief's item 6 says "wake retired" with no accompanying commit or record; `waking.rs` remains on main, gated and un-defaulted. What exactly was retired — the waking seat as the 6-4 remedy, or the program — is not written down (open).

## 8. O5 — the belief the modeled minds are not allowed to have

### 8.1 The obligation

`walt/SCENARIO-PLAYER.md` **Def 4.3** (no-void simplification, inner minds): *modeled minds sample the sizes-fiber without void conditioning* — a shuffle of the unseen tiles into the other seats' hand sizes, ignoring the voids the public record has proved. Declared as an approximation, not an error, cost unmeasured: obligation **O5**, filed 2026-08-18, "ablation probe (void-conditioned inner minds vs current)". Walt's *outer* sampler has always been void-correct. So the seat believes the truth about its opponents while every mind it models believes a fiction — and the fiction errs one way: every impossible world hands a void seat a tile it cannot hold and drains that tile from the seats that must hold it. **A bias floor, not variance.**

### 8.2 The measurement (branch walt-o5 only, 2026-09-04/05)

Record: `walt-o5:walt/probes/o5/README.md` (9 commits 0b65efb9 … 2981e090, tip 2981e090; `walt/ci/check.sh` PASS on the branch including Lean, per the record). Corpus: `rob/receipts/verify_player.txt` (13 hands, rob's determinism freeze, read as a corpus of lawful play and never as an axiom — TRUST-01) plus G1 passed by `--extra`, never vendored: 14 hands, 392 decision points. Exact integers and rationals throughout; walls contended and reported only as ratios. Probe record; gate-pinned on the branch only where §8.6 says so. **Not on main.**

**Stage 1 — the dead-sample census (exact; `voidcensus`).** DEAD = sizes-fiber (`Kernel::unconstrained_count`, σ0's actual sampling space) − lawful fiber (`Kernel::count`, |Φ(C)| by the §2.1 grouped-multinomial DP). Dead fraction in ‰ over all 392 points, by trick:

| trick | 1 | 2 | 3 | 4 | 5 | 6 | 7 |
|---|---|---|---|---|---|---|---|
| median | 0 | 0 | 333 | 363 | 333 | 250 | 0 |
| max | 838 | 888 | 979 | 988 | 982 | 933 | 666 |

Zero-void points are exactly zero-dead (the sanity check the census owes). G1's S2 line, exact: lawful/sizes 46,558,512/46,558,512 (0‰) · 432,432/2,018,016 (785‰) · 17,640/90,090 (804‰) · 8,820/34,650 (745‰) · 300/1,680 (821‰) · 12/12 (0‰). From trick 2 on, four fifths of what Gran's modeled minds believed had already been refuted by the record.

**Stage 3 — σ0 flips (`o5flip`).** Void-aware vs void-blind level-0 choice at 243 unforced points: flip rate **102‰ at n0 = 2, 189‰ at n0 = 8, 164‰ at n0 = 32** — not a shrinking sequence; a variance defect would shrink, a bias floor need not, and the natural reading (stated as a reading, not a result) is that more draws from the wrong distribution converge harder onto the wrong argmax. Exactly 0 flips at dead-fraction-0 points at every n0. σ0 price: 1012‰ of nodes and 1066‰ of wall at n0 = 2.

**Stage 3 — the live level-1 player (`o5level1`, n_outer 50, n0 8).** Recommendation changes by trick: **371 / 454 / 342 / 150 / 90‰** at tricks 1 / 2 / 3 / 4 / 5–7. G1, all seats: 7 of 20 moved (350‰). G1's S2 line: trick 3 moves 6-4 → 5-2 (blind: 1-0 680, 2-2 600, 4-2 620, 5-2 620, **6-4 760**; aware: 1-0 660, 2-2 500, 4-2 520, **5-2 740**, 6-4 680 — the 80% of the modeled belief the record had refuted was holding the 6-4 up) and trick 5 moves 4-2 → 5-2; trick 1 stays 6-4 (880‰ blind, 820‰ aware). Defect (a), fixed 43790099: the probe had printed `P(banked_T1 ≥ bid)`, the internal T1-frame value, which on T0-declared hands like G1 is P(declaring side *set*); now rotated to P(declaring side makes). Label only — no decision changed.

**The second channel, not anticipated.** At trick 1 the record has proved nothing (31 of 35 trick-1 decisions have dead fraction exactly 0), yet 13 recommendations moved and **12 of the 13 are at dead-fraction-0 points**. The σ0 probe flips nothing there. The difference is depth: a level-1 solve walks the modeled continuation, modeled seats fail to follow inside it, and those proved voids are carried by the void-aware key and discarded by the void-blind one. The no-void simplification costs decisions even where the public record is empty; the stage-1 census is a lower bound on the mechanism.

**The memo price, with its caveat.** The fear was that voids in the key would split positions the memo used to merge. Measured at the live epoch: distinct π keys −7% to +9%, hit rate −1 to +3 points, wall −6% to +16%; no blow-up (the lawful fiber is smaller and induces smaller subtrees). **Caveat specimen:** at the G1 S2 trick-1 root at n_outer 200 / n0 4 the aware arm used **8.0× the distinct π keys and 6.0× the wall** — mostly a second tie-refinement round at 4× worlds rather than key splitting, but real, invisible at the live epoch, and a reminder that this is one 14-hand corpus at one epoch. The brief's item 5: "'No memo price' was live-epoch only."

### 8.3 The mirrored match, told as it happened

`o5match` decides every play with `solver::level1_evaluate` (the library authority; it could not be built on `playout.rs` or `walt_bridge.rs`, each of which carries a private `Key`/`PiKey`/`Solver`). Each deal is played twice with the epochs swapped between the teams, so deal luck cancels and pair totals sum to 84 (asserted every deal). The frame is declared and artificial: contract P30 held by T1, S1 leads, declaration cycling over all nine, no auction. Both arms share every belief seed so only the modeled minds differ.

- **2026-09-05 00:05 (331fcfbc): "a DEAD HEAT, and the record says so."** Reduced epoch +158 for aware (82/81/29 pairs), live epoch +52 (26/26/20).
- **2026-09-05 00:42 (2981e090): the seed defect.** The belief seed was (seat, played mask) only, so every deal's trick-1 draws shared one stream per seat. Defect (b), repaired by adding the deal index (the epoch is deliberately not a seed coordinate). Rerun:

| epoch | deals | games | aware pts | blind pts | margin | pairs aware / blind / level | contracts aware / blind |
|---|---|---|---|---|---|---|---|
| n_outer 16, n0 4 (reduced) | 192 | 384 | 7951 | 8177 | **−226 of 16128** | 76 / 83 / 33 | 61 / 61 |
| n_outer 50, n0 8 (live) | 72 | 144 | 3155 | 2893 | **+262 of 6048** | **33 / 19 / 20** | 27 / 25 |

Per-pair margins: median exactly 0 in both runs, tails −64..+46 (reduced) and −62..+54 (live); both halves of the live run agree in sign (+112, +150). **"Dead heat" is withdrawn.** The two epochs point opposite ways on a 2.7× sample-size difference; 33 to 19 over 52 decided pairs is suggestive and under-powered; no interval is computed because O6 is unbuilt. The result is not stable against the seeding of the draws, let alone against epoch. Readings left open in the record, none chosen: partial self-cancellation in a mirrored pairing where both sides carry the bias; a pmake surface flat enough near the argmax that a changed decision is usually a near-tie; a real but small effect; or genuine epoch-dependence (the aware belief paying only once the outer sample is wide enough to exploit it). **The correctness case for the fix is strong and the play case is unmade. Flag OFF. A default swap is Jason's word** (call (D)).

Not measured: the void epoch at level 2 or above; no gate pins a doubles-trump or no-trump void specimen (stages 1 and 3 see only pip trumps; `stepped_voids` delegates follow to the rules layer's `Decl::follows`, "correct by delegation" — an argument, not a gate).

### 8.4 The branch implementation (walt-o5)

`Key` and `PiKey` carry `voids: Option<[u32; 4]>` — `None` = void-blind (the default, and the epoch of every existing receipt, gate and frozen transcript), `Some(v)` = void-aware; the root seeds it from the record's derived voids (`ContinuationFrame::void_aware_key`), `key_step` and `Solver::child_after_play` recompute it through `Decl::follows`, `Solver::pi` consumes it. Field `Level0Field::void_aware(n0)`, policy id `field:level0-voidaware-n{n0}-v1` against `field:level0-n{n0}-v1` — a different field model, never compared as the same object. Sampler: **shuffle-and-reject** (`solver::sample_belief` with the carried mask; with mask 0 the draw is bit-identical to the old inline loop). Gates: `walt-o5:walt/walt/tests/solver_inner_voids.rs`, **7** `#[test]` functions (counted 2026-09-12 from the branch), one coordinate per law plus pinned strictness witnesses — including h8-t5 S3 (blind plays 5-3, aware plays 0-0) and a 117-decision pre-slice σ0 oracle built at a0d594b (walt-o5's merge base with main) reproduced byte-for-byte. Whole workspace with the flag off: 128 test binaries, 0 failed.

**Jason's ruling of 2026-09-05** (`walt-o5:walt/kanban/backlog/inner-voids.md`): the shipping design must be **rank/unrank over a counting DP** — no rejection loop, N = |lawful fiber| free at every node, stratification free, uniformity structural — and the builder must check whether `kernel/fiber.rs`'s `FiberDp` or `solver/model_belief.rs` already *is* the counter before writing one. The O5 measurements stand under either sampler (same distribution, same support). The card's blocking item is the unresolved play evidence; its next steps — 1000+ deals at both epochs, sweep n_outer at fixed n0, pair against a third reference seat instead of the mirror, bid instead of fixing P30 — have not run.

### 8.5 The second implementation, on main (dbcc698f, 2026-09-06 17:02)

Independently of walt-o5, the partnership program added `solver::inner_belief::InnerBelief { Voidless (default), VoidsCounted }` to the shared solver (`experiments/partnership/INNER-BELIEF.md`; SCENARIO-PLAYER's 2026-09-06 note: "implements an alternative for obligation O5; it does not discharge the cost/strength measurement obligation").

- Selected once on `Shared` (`Shared::with_inner_belief`; `Shared::new` defaults to `Voidless`) and inherited by every nested modeled level; seat levels (baseline / partner-only / all-L1) are an independent choice.
- **Key contract.** `Key` and `PiKey` carry `voids: Option<[u32; 4]>`: `None` = legacy approximation (later simulated voids also ignored); `Some([0; 4])` = tracked opening state with no deductions yet; `Some(masks)` = tracked public forbidden-tile masks by seat. The last two differ from `None` — treating an empty tracked state as legacy would lose every later failure-to-follow deduction. `inner_belief::after_play` is the single void-update authority (declaration-relative following, deductions preserved across trick clears); a key/strategy mismatch is an invariant assertion.
- **Sampler.** `VoidsCounted` builds a `Kernel` from the modeled seat's own hand, the public played tiles, capacities and voids — no host-world access — and draws through the kernel's existing exact counted sampler (`FiberDp` + `sample_with`, a separate `KernelRng` seeded from `rng.next_u64()`); the adapter asserts the mask reconstructs exactly from contexts and that the fiber is non-empty (an inconsistent frame is an invariant failure, never a weakened belief). This is uniform **physical support**, not a likelihood posterior.
- **Validation recorded:** nine pre-change decisions reproduced exactly (`runs/voids-before/golden.json`); the same nine complete under counted beliefs at 4/2/2 (`runs/voids-compatibility/`); partnership Rust gates (INNER-BELIEF.md cites eleven; `walt/walt/tests/solver_partnership.rs` carries 12 `#[test]` functions as of c00717d1, four of them void-specific: fiber membership under independent replay, deductions across trick resolution under all nine declarations, cache separation and host-world independence, all three seat profiles and expired sampling); 36 focused Rust tests; 252 Python/native comparisons; wasm32 compile check. Existing entry points, the archived phone WASM (which rejects the option) and every default are unchanged.
- **Caveat stated in the record:** the outer sampler stays shuffle-and-reject and the inner counted sampler uses a *different deterministic stream*, so action changes cannot be attributed to void conditioning alone.

**The batteries** (paired mirrored bid-30 deals, partnerships swapped; win/loss/tie on making the contract only; the comparative fraction `(1 + mean pair score)/2` is not absolute pmake):

| A vs B | deals | A wins / losses / ties | fraction | rough ±2 SE | record |
|---|---|---|---|---|---|
| l1-race-voids vs l1-race (random, seeds 720600–720649) | 50 | **9 / 5 / 36** | 54.0% | 46.5–61.5% | foundation-battery/05-voids (e310e6ff) |
| l1-race-voids vs l1-race (five focal hands × ten completions, 820600–820649) | 50 | **5 / 8 / 37** | 47.0% | none (fewer than ten independent units) | foundation-battery/07-worlds |
| l2-partner-voids vs l2-partner-default (Fixed 40/8/2, deals 750600–750699) | 100 | **12 / 17 / 71** | 47.5% | 42.1–52.9% | default-partner-battery/02-voids (0b65e5b1) |
| l2-partner-default vs l1-default (same deals) | 100 | 14 / 14 / 72 | 50.0% | 44.7–55.3% | default-partner-battery/01-level |

Per focal hand in 07-worlds: 820600 1/2/7, 820610 1/2/7, 820620 0/0/10, 820630 2/2/6, 820640 1/2/7. Latency in the 05-voids match itself: 0.535 s/move (l1-race-voids, 0 fallbacks / 932 nonforced) vs 0.497 (l1-race, 0 / 933) — RESULTS.md's representative table lists l1-race at 0.502 from the 01-phone match; 0.228 / 1.127 / 1.318 s per move for l1-default / l2-partner-default / l2-partner-voids with 0/1821, 43/3690, 60/1809 fallbacks. **No strength gain established in any of them**; the refined partner configurations crossed the >5% fallback gate and were stopped as a cost problem, not a strength verdict. Details on [walt-partnership-program.md](walt-partnership-program.md).

### 8.6 The fork, named

Two void-aware inner-belief implementations now exist, and they are not the same object:

| | walt-o5 (branch, 2026-09-04/05) | main (dbcc698f, 2026-09-06) |
|---|---|---|
| switch | `Level0Field::void_aware(n0)`; `Key::voids` `None`/`Some(v)` | `InnerBelief::VoidsCounted` on `Shared`; `Key::voids` `None`/`Some([0;4])`/`Some(masks)` |
| sampler | shuffle-and-reject `sample_belief` with mask | kernel `FiberDp` + `sample_with` (counted) |
| draw stream vs legacy | bit-identical with mask 0 | different stream (not attributable to voids alone) |
| gates | `solver_inner_voids.rs`, 7 | `solver_partnership.rs`, void subset |
| measurement | O5 stages 1–4 (census, flips, level-1 moves, mirrored match) | foundation and default batteries |
| SCENARIO-PLAYER O5 row | rewritten on the branch | line 254 still reads "ablation probe"; 2026-09-06 note added |

Both add a `voids: Option<[u32; 4]>` field to `Key`/`PiKey` with different semantics and samplers: merging walt-o5 into main will conflict on that surface, and whichever lands first rewrites the O5 row. Jason's rank/unrank ruling may already be satisfied by main's `FiberDp` path, but no record says so. Which implementation goes forward is Jason's call (E).

## 9. Methodology box

- **EXPLORATORY, estimate never receipt.** Every number here is a probe record unless it names a gate (`solver_waking.rs`, `solver_partnership.rs`, the branch's `solver_inner_voids.rs`). A green gate is evidence, never a status change.
- **"Exact" is exact over a support given a field policy.** The level-2 rows marked exact solved every deal of the void-consistent support, but the approximation lives inside the modeled minds (n1 = 8, n0 = 4, no-void inner beliefs). They are statements about *that player*, not about 42, and not receipts.
- **Corrections stay in place.** The trick-3 sampled inversion (§5.2), "dead heat" (§8.3), "mutually exclusive" (§6.5) and the P(set) label defect (§8.2) are all carried verbatim in their records and here.
- **Records declare seeds, thread counts and nesting.** Nested samples make a refinement ladder real refinement; RESULT lines are thread-count invariant (exact rationals, key-deterministic caches) while node counts and walls drift.
- **Epochs do not compose.** The live Plunge seat (40 worlds, racing), the waking seat (σ0 n0 = 2), the level-2 ladder (200/8/4), the O5 probes (50/8 and 16/4) and the partnership profiles (40/8/2) are five different instruments; every number above carries its epoch, and §5.4 is the reason.
- **Typed distinctions kept:** support ≠ belief (the modeled minds' *support* is what O5 corrects; nothing here is a likelihood posterior); exact-for-the-frozen-set ≠ exact root; indifference under a model ≠ laydown; a level-2 player is a best response to a named σ1 field, never an equilibrium.

## 10. Status appendix (repository state as of 2026-09-07, c00717d1; verified read-only 2026-09-12)

**On main:**

| item | path | landed |
|---|---|---|
| G1 fixture, G2/G3 partial fixture, census JSONL and summaries, `summarize.py`, README | `walt/probes/gran/` | 32aa14f1, 8174fa83 (2026-09-04) |
| granrun (`validate` / `validate-partial` / `replay` / `driven`) | `walt/walt/src/bin/granrun.rs`; release binary `walt/target/release/granrun` | 32aa14f1 |
| waking seat, wakeup layer, waking_bridge, waking profile | `walt/walt/src/solver/{waking,wakeup}.rs`, `bin/{waking_bridge,wakeup}.rs`, `walt/probes/waking/` | PR #54 93d99563 (2026-08-25) |
| InnerBelief::{Voidless, VoidsCounted}; selection rules | `walt/walt/src/solver/{inner_belief,selection}.rs` | dbcc698f, 9236ca7f (2026-09-06) |
| partnership batteries | `experiments/partnership/campaigns/{foundation-battery,default-partner-battery}/` | e310e6ff, 0b65e5b1 (2026-09-06) |
| MORNING readout | `walt/briefs/MORNING-2026-09-05.md` | 9d6a5a2e (2026-09-05 02:09; walt-gran, since merged) |
| kanban cards | `kanban/backlog/gran-anchor-reconstruction.md` (open on two items), `kanban/done/waking-seat-census.md` | 2026-08-24 / 2026-09-04; 2026-08-25 |

**Only on branch walt-o5** (9 commits not in main, 0b65efb9 … tip 2981e090, 2026-09-04 22:10 → 2026-09-05 00:42; merge base a0d594b2; 33 files, +6460/−32; local branch, not on origin per `git branch -a`): `walt/probes/o5/*`, bins `voidcensus` / `o5flip` / `o5level1` / `o5match` / `o5anchor`, `tests/solver_inner_voids.rs` (7 gates), `Level0Field::void_aware`, `walt/kanban/backlog/inner-voids.md`, the SCENARIO-PLAYER O5 row rewrite. The MORNING brief counts this branch as "walt-o5 (10, check.sh PASS incl. Lean)"; `git log main..walt-o5` counts 9 at c00717d1.

**Only on branch walt-g1-l2** (8 commits not in main, 2d3907bd … tip 6abdd78f, 2026-09-04 22:38 → 2026-09-05 00:39; based on walt-gran at 8174fa83; 9 files, +3784/−63; local branch): `level2.rs`'s `fixture` mode (`--viewer-hand`, `--sub`, `--field-level`, `--worlds`), `level2_g1.txt`, `level2_g2.txt`, `level2_lock.txt` with their `*_runs.txt`, `synthetic_lock.receipt.txt`, and a 224-line README extension. Main's `level2.rs` has no `fixture` mode; the release binary at `walt/target/release/level2` is main's build.

**Merged:** walt-gran and walt-fh (PR #88) are fully contained in main (`git log main..<branch>` empty for both).

**Still open on main:** `v5_literal_count_timing_position_reconstructs` (`walt/walt/tests/solver_calibrate.rs:419-420`) remains `#[ignore = "blocked: plunge-side game seeds (L2-A6 [[gran-anchor-reconstruction]])"]` although its stated blocker is discharged for G1; whether the G1 record is the literal position that test names is unverified.

**Jason's pending calls, verbatim from `walt/briefs/MORNING-2026-09-05.md`:**

> **Your calls:** (A) more live-epoch match deals; (B) declarer-side measurement for the literal 100s-at-L1 test; (C) objective + tie-break design; (D) void flag default; (E) merge #88 and shape of the three branches; (F) G4 when the real game recurs.

Of these, only the merge half of (E) is recorded as done (walt-fh is in main); none of (A)–(D), the branch-shape half of (E), or (F) is recorded as decided in the repository after 2026-09-05.

**Open items carried by this page:** which of the six residual assignments is G2's real deal (undecidable from the screenshot); the level-2 exact row at G2 trick 2 (died at 7.6 G nodes / 560 s); the tie-break executed at a tied node (no harness); why level 1 alone flips trick 1 between 50/8 and 200/4; which binary the deployed phone ran when the screenshot was taken; what "wake retired" retired; the two-implementation fork.

## 11. How to run

From the repository root; the release binaries already exist under `walt/target/release/`.

```
walt/target/release/granrun validate         walt/probes/gran/g1.receipt.txt      # 0.005 s, measured 2026-09-12
walt/target/release/granrun validate-partial walt/probes/gran/g2g3.receipt.txt    # 0.003 s, measured 2026-09-12
walt/target/release/granrun replay  walt/probes/gran/g1.receipt.txt S2 out.jsonl  # ~25 s at the live epoch (record)
walt/target/release/granrun driven  walt/probes/gran/g1.receipt.txt out.jsonl     # ~166 s (record)
python3 walt/probes/gran/summarize.py out.jsonl
walt/target/release/waking_bridge driven driven.jsonl 2                           # minutes per hand (record)
python3 walt/probes/waking/summarize.py driven.jsonl
python3 experiments/partnership/player.py --mode partner --inner-belief voids-counted
```

The level-2 fixture runs and the O5 probes require checking out walt-g1-l2 or walt-o5 respectively and building (`cargo build --release -p walt --bin level2`; `cargo build --release --bin voidcensus --bin o5flip --bin o5level1 --bin o5match`); their reproduction blocks are in the records named above. The screenshots themselves live outside the repository at `~/data/texas-42/gran-anchors-2026-08-24/` and are verified by `MANIFEST.sha256` before reading.

Related chapters: [walt-seat-play.md](walt-seat-play.md) (the live seat, the pmake objective, the 2026-08-17 ladder), [walt-calculated-evidence.md](walt-calculated-evidence.md) (the waking seat's era), [walt-focal-horizon-era.md](walt-focal-horizon-era.md) (the same days' focal-horizon work), [walt-partnership-program.md](walt-partnership-program.md) (the phone artifact, player families, batteries), [walt-instruments.md](walt-instruments.md), [walt-architecture.md](walt-architecture.md), [walt-negative-results.md](walt-negative-results.md), [walt-math-open-questions.md](walt-math-open-questions.md), [timeline.md](timeline.md).
