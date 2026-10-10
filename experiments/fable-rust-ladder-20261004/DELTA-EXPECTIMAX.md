# Delta expectimax (walt42x `turbo.cpp`) evaluated for the inner-rung problem

**Exploratory, 2026-10-04 evening.** Claude Fable 5.1, same session. Jason asked whether his
"delta expectimax" frontier ladder (untracked `experiments/walt42x-intake`, snapshot in
`experiments/walt42x-intake-snapshot/`) is better and/or faster for our purposes: producing the
array of declarer-make probabilities per legal play (the inner-rung target), serving as the inner
mind in play, and training. All runs under the 295 s process-group cap; nothing outside this
directory changed; the engine copy in `ladder/csrc/turbo.cpp` is the snapshot plus one
FORK-ONLY addition (`enumerate_cap`: use every void-consistent world once when the support is
small; `walt_set_enumerate`).

## What the engine is, in our vocabulary

Level 1 of walt42x = best response to uniform random legal play over `n` sampled worlds drawn
exactly uniformly from the void-consistent support (integer DP ranking, no rejection). That is the
same model quantity Walt's level-0 mind estimates (declarer-make probability per legal play with
own play optimized per public information state, "no strategy fusion"), with void-awareness.
`delta` decides, per live world at an opponent node, whether to branch over all legal opponent
moves exactly (when each child keeps weight >= delta) or sample one continuation (delta = 1,
Walt's dice). Level 2 = best response to level 1, i.e. Walt's outer search. The compiled engine
uses addressed randomness so exact 0/N incumbent bounds can prune at delta = 1.

Plumbing added to the Rust ladder: static link of the engine (`build.rs`, system clang++, no new
crates); `turbo.rs` with the FFI, an inner-rung policy (`--net turbo:N[:DELTA[:MINDEPTH[:ENUMCAP]]]`)
that answers Walt's level-0 calls through the hook, a whole-player arm (`--net l2:OUTER:INNER[:DELTA]`)
where turbo level 2 replaces Walt's outer search for the hybrid partnership, `label --engine turbo`,
and `turbo-eval` (label quality per microsecond). Conventions verified: key frame with odd seats
declaring, triangular tile index, forbidden-tile void masks, trick scoring; 8-world turbo values match
Walt's 8-world dice labels in distribution (RMSE .134 vs .131 against the same reference).

## 1. Labels: delta is not a good buy; the compiled sampler is

1,620 held-out void-aware level-0 calls (240 to 300 per trick, tricks 1 to 6; trick 7 has no
multi-legal calls). Reference = 4,096 sampled worlds. RMSE on the pmake vector over legal tiles;
regret = reference pmake lost by the frozen selector's choice (max for declarers, min for
defenders, lowest tile on ties). Timings are per call from a cold engine on a loaded machine.

| Estimator | µs/call | RMSE | regret | selector agreement |
|---|---:|---:|---:|---:|
| Walt labeler, 8 worlds, void-aware | 45 | .131 | .016 | .58 |
| turbo 8, δ=1 | 19 | .134 | .017 | .58 |
| turbo 8, δ=1/8 | 587 | .100 | .008 | .66 |
| turbo 8, δ=1/24 | 1,739 | .088 | .005 | .73 |
| turbo 16, δ=1 | 35 | .097 | .012 | .61 |
| turbo 24, δ=1 | 50 | .084 | .009 | .64 |
| turbo 24, δ=1/8 | 1,727 | .059 | .004 | .73 |
| turbo 24, δ=1/24 | 5,310 | .053 | .002 | .79 |
| turbo 32, δ=1 | 66 | .074 | .009 | .66 |
| turbo 64, δ=1 | 132 | .056 | .006 | .69 |
| turbo 160, δ=1 | 342 | .040 | .003 | .74 |
| Walt labeler, 160 worlds, void-aware | 4,322 | .041 | .003 | .76 |

Per microsecond, more sampled worlds beat exact opponent branching at every trick: 160 worlds at
δ=1 costs less than 8 worlds at δ=1/8 and is 2.5x more accurate. Exact branching does buy
selector agreement at equal RMSE (δ=1/24 at 24 worlds: .79), but at 15x the cost of 160 worlds,
which would buy 2,400 worlds instead. The compiled engine is 12.6x faster than our Walt-based
labeler at identical 160-world quality (its exact support sampler and lean search, not delta).

Consequence used immediately: the full void-aware call set (2.27M train, 360k val) was relabeled
at **1,024 worlds** (0.7 to 0.8 ms per call under load; 2 capped rounds of 5 workers for train, 2
of 1 worker for val) as `data/labelsT1024-{train,val}.bin` (records store round(10,000·k/N) of
10,000). RMSE between the 1,024-world and 160-world labels on the same 360k calls: .033.

## 2. Enumerating the tail instead of sampling (Jason's question)

Support sizes (void-consistent hidden deals) on the held-out calls: trick 4 start median 10,920
(max 34,650); trick 5 start max 1,680; trick 6 start max 90; last two plays at most 12. Trick 7 is
forced: no level-0 call exists there. `eCAP:δ` = enumerate every world when support <= CAP, else
8 sampled worlds; δ=0 integrates the random opponents exactly, so `e40000:0` is the exact model
value and serves as truth below.

| Estimator | trick 4 µs | trick 5 µs | trick 6 µs | RMSE vs exact (t5 / t6) | regret (t5 / t6) |
|---|---:|---:|---:|---|---|
| 8 sampled, δ=1 | 12 | 6 | 3 | .130 / .112 | .013 / .006 |
| 24 sampled, δ=1 | 30 | 14 | 5 | .084 / .063 | .004 / .002 |
| 160 sampled, δ=1 | 188 | 69 | 18 | .031 / .025 | .001 / .000 |
| enumerate <= 2,000, δ=1 (sampled continuations) | 526 | 133 | 5 | .035 / .075 | .002 / .005 |
| enumerate <= 2,000, δ=0 (exact) | 38,730 | 1,763 | 11 | 0 / 0 | 0 / 0 |
| enumerate <= 40,000, δ=0 (exact) | 1,566,615 | 1,751 | 10 | 0 / 0 | 0 / 0 |

So: **trick 6 should be solved exactly** (<= 90 worlds, exact opponents, about 10 µs, the price of
50 sampled worlds, zero error). Trick 5 exact costs 1.8 ms per call, 25x a 160-world sample that is
already within .03; trick 4 exact costs 1.6 s. The cost is the branching of random opponents over
three or four tricks, not the world count: enumerating all worlds with sampled continuations
(`e2000:1`) is no more accurate than 160 sampled worlds. Enumeration with sampled continuations
is also a waste at trick 6 (δ=1 with 90 enumerated worlds is *worse* than 24 sampled, because one
continuation per world over two tricks is noisy). The efficient split is: sampled worlds (or the
net) through trick 5, exact enumeration from trick 6. That also removes 37% of the multi-legal
calls (trick 6 is 132k of 360k) from what a net must learn.

## 3. Head-to-head (paired mirrored deals vs pinned production, seed 5000, 4,096 deals unless noted)

| Hybrid partnership | paired advantage [95%] | hybrid / native decision µs (same run, loaded) |
|---|---:|---:|
| void-aware native control (`inner_belief 1`, earlier) | +.018 [+.003, +.033] | |
| 32-world exact inner, voidless (603 deals) | +.035 [−.002, +.073] | 204,000 / 4,700 |
| turbo inner 8 worlds δ=1 | +.020 [+.004, +.034] | 5,900 / 3,700 |
| turbo inner 8 worlds δ=1/8 (1,526 deals, 3 rounds) | +.047 [+.022, +.071] | 55,500 / 4,300 |
| turbo inner 8 worlds sampled through trick 5, exact enumeration from trick 6 (2,765 deals) | +.032 [+.013, +.049] | 11,200 / 4,700 |

Jason called the ablation off here (remaining arms: 24-world inner, whole-player level 2 at 40/8,
24-world split tail). The pattern is consistent: a sharper inner mind (void-aware, more worlds,
exact branching, or an exact tail) raises the outer player's paired win rate by 2 to 5 points, and
each of those costs 2x to 13x the decision time. The net's job is to buy that sharpness for free.

## 4. Training on the 1,024-world teacher

Two 128x128 trainings (v2 and additive-void v4 encodings, 50k updates) ran concurrently on the GPU
and were both cut by the 295 s cap before finishing; their best-so-far weights exist
(`models/T1024-{v2,v4}-h128x2.w`) but no selected metrics. Not pursued further: Jason redirected
the effort to the next rung (below).

## 5. Redirect: distill the whole decision (rung 2), then climb

Per Jason (2026-10-04 evening): stop ablating the inner rung; find out whether a net can be faster
than rollouts for a layer, then climb and watch for strength. Rung 2 is where the payoff is: Walt's
whole decision costs about 4.5 ms (40 outer worlds x inner minds); a 128x128 net decides in 3 to
6 µs (measured in the harness, 1000x). Built for this, same binary:

- `log2`: self-play by the void-aware turbo level-2 teacher (160 outer / 8 inner, full root vector
  per multi-legal state = count of outer worlds where the declarer makes, per legal play; moves
  follow the teacher except 10% random for coverage). 65 ms per labeled state (136 ms at 160/24).
  16 capped workers, fresh seed range per round (`log2_launch.py`); worker 15 is validation.
- `--net np:PATH`: the net alone as the whole player for the hybrid partnership.
- `--model PATH` on `log2` and `--net l2n:OUTER:PATH`: the engine's modeled rung-1 others play the
  net (C callback `walt_set_policy_hook`, FORK-ONLY), i.e. "best response to the net" = the rung-3
  teacher and the search-with-net-inside player (3.5 ms per decision at 40 worlds).
- Teacher ceiling: `l2:160:8` whole player vs production, 1,024 deals (running).

Cycle k: label vectors of best-response-to-Net_{k-1} (Net_0 = sampled level 1) -> train Net_k ->
h2h Net_k alone and search+Net_k vs production.

## 6. Rung-2 distillation: first cycle numbers (2026-10-04, late)

Teacher ceiling: turbo level 2 (160 outer / 8 inner, void-aware) as the whole player vs pinned
production, 1,024 paired deals, seed 5000: **+.077 [+.048, +.105]**, 21 ms vs 4.9 ms per decision.

Student = 128x128 additive-void net on the teacher's root vectors, whole player, 4,096 deals.
(A Rust forward bug for the additive-void encoding was found by the `agree` check and fixed; the
Rust forward is now bit-equal to a NumPy forward from the exported weights on 2,000 records.)

| teacher states | val RMSE | selector agreement (random .31) | paired advantage vs production | win rate | decision µs |
|---:|---:|---:|---:|---:|---:|
| 56k (160/8, 1 round) | .174 | .36 | −.469 [−.484, −.452] | .266 | 6 |
| 165k (160/8, 3 rounds) | .139 | .42 | −.428 [−.445, −.412] | .286 | 7 |
| 165k, 256x256 | .142 | .41 | | | |

Data-limited, not capacity-limited at this size (256x256 is no better than 128x128). The 40/8
teacher (same model, 4x cheaper, k/40 labels) is producing about 240k states per capped round;
next read at about 650k states, then student-driven rounds (`log2 --play np:...`).

### Bug found and fixed (affects every exported net before it)

The trainer's embedding table carries a dummy row (index `INPUTS`) gathered for absent features
(no led suit, defender seat, and, in the additive-void encoding, every non-void (seat, tile) pair,
about 80 gathers per sample). Gradients flowed into that row, so the trained function used it as a
count-weighted bias; the export drops the row and the Rust forward never adds it. Trainer metrics
and the deployed net therefore disagreed: for the void encoding the trainer reported 58% selector
agreement while the exported weights scored 40% in Rust and in a NumPy forward from the same file.
Fix: mask the row's gradient (`W1MASK`) and assert it is zero at export. After the fix the Rust
agreement equals the trainer's (59.6% vs 59.8%). Inner-rung v1/v2 nets from earlier today were
affected only mildly (1 to 2 dummy gathers per sample; their h2h numbers are what the Rust forward
played and stand as measured), but retraining them with the fix is a cheap likely gain.

### Rung-2 whole-player curve after the fix (vs production, seed 5000, 4,096 deals)

| net (128x128 unless noted) | states | loss | selector agreement (Rust) | paired advantage [95%] | win rate | µs/decision |
|---|---:|---|---:|---:|---:|---:|
| lowest legal tile | | | | −.541 [−.556, −.525] | .230 | |
| L2b, pre-fix | 591k | BCE | .32 | −.436 | .282 | 7 |
| L2c, pre-fix | 591k | CE on teacher choice | .40 | −.457 | .271 | |
| L2e | 1.00M | BCE + CE | .596 | **−.154 [−.169, −.138]** | .423 | 5 |
| L2f | 1.42M (+419k student-driven) | BCE + CE | .611 | see below | | 5 |
| L2f 256x256 | 1.42M | BCE + CE | .617 | −.140 [−.156, −.124] (3,643 deals) | .430 | 12 |
| teacher (turbo level 2, 160/8) | | | 1 | +.077 [+.048, +.105] | .539 | 21,000 |

The cross-entropy term on the teacher's selector is worth as much as 10x data: BCE alone on 1.0M
reaches .455 agreement, BCE+CE .596.

### Cycle run by operator agent (Sonnet)

One runbook cycle, all runs under the 295 s cap, seed 5000, paired mirrored deals vs pinned production.
Exploratory tier: rob conformance receipts do not cover these numbers.

| net | player arm | states (train+val) | loss | selected update | val RMSE | agreement trainer / Rust | paired advantage [95% CI] | win rate | decision us (median) | deals paired |
|---|---|---:|---|---:|---:|---|---:|---:|---:|---:|
| L2g-mix-h128x2 | np (whole player) | 1,838,365 + 122,470 (mixD) | BCE + CE (mix) | 29000 | .1525 | .6144 / .6134 | -.134 [-.149, -.119] | .433 | 3 | 4096 |
| L3a-mix-h128x2 | np (whole player) | 202,588 + 13,117 (r3-L2g) | BCE + CE (mix) | 1500 | .1910 | .5287 / .5288 | -.291 [-.307, -.274] | .355 | 4 | 4096 |
| L3a-mix-h128x2 | l2n:40 (search, net as modeled others) | same | same | 1500 | .1910 | same | -.060 [-.091, -.027] | .470 | 1961 | 1024 |

Notes:
- mixD = mixC + 40/8s2 labels (cat). L2g agreement was measured by Rust on the first 20,000 val records.
- Rung-3 teacher data (`log2_launch.py 8 160 16 2 0.1 r3-L2g --model models/L2g-mix-h128x2.w`, seed 9000000): round 0 108,685 states, round 1 cumulative 215,705 (about 107,020 new); 202,588 train + 13,117 val.
- L3a selected update 1500 of 30000: the 215k-state set overfits early, so L3a is data-limited. The np arm is far below L2g (-.291 vs -.134), but search with L3a as the modeled others reaches -.060.
- Teacher ceiling for comparison (section 6): +.077 [+.048, +.105], 1,024 deals.

#### Cycle 2 (operator agent, Sonnet): rung-3 teacher at volume

Teacher data: `log2_launch.py 8 40 16 4 0.1 r3b-L2g --model models/L2g-mix-h128x2.w`, seed 11000000, four capped rounds
(280 s each). Cumulative states after each round: 440,039; 872,384; 1,313,242; 1,762,103 (about 440k, 432k, 441k, 449k new).
Final 1,651,309 train + 110,794 val. Merges: r3all = r3 + r3b = 1,853,897 train + 123,911 val; all = mixD + r3all = 3,692,262 train + 246,381 val.
Same trainer flags as before (enc 4, loss mix, 128x128, 30000 updates, batch 1024). Rust agreement read the first 20,000 val records.
Exploratory tier; seed 5000 paired mirrored deals vs pinned production; every run completed all its deals.

| net | arm | train data | selected update | val RMSE | agreement trainer / Rust | paired advantage [95% CI] | win rate | decision us (median) | deals paired |
|---|---|---|---:|---:|---|---:|---:|---:|---:|
| L3b-mix-h128x2 | np | r3all (1.85M) | 27250 | .1604 | .6236 / .6212 | -.110 [-.126, -.094] | .445 | 3 | 4096 |
| L3c-mix-h128x2 | np | all (3.69M) | 29000 | .1543 | .6183 / .6146 | -.100 [-.116, -.084] | .450 | 3 | 4096 |
| L3c-mix-h128x2 | l2n:40 (search) | all | 29000 | .1543 | same | +.008 [-.022, +.039] | .504 | 2038 | 1024 |

Reading (exploratory): volume fixed the L3a starvation (L3a np -.291, L3b np -.110). L3c is slightly better than L3b as a
whole player, but the two CIs overlap. Search with L3c as the modeled others is statistically level with production
(CI includes zero), against -.060 for L3a in the same arm. Teacher ceiling for comparison: +.077 [+.048, +.105].

#### Cycle 3 (operator agent, Sonnet): rung-4 teacher (L3c inside the search)

Full-size parity read: `l2n:40` with L3c, 4,096 deals, 8 workers: **+.0073 [-.0076, +.0227]**, win rate .504, 2,374 us median per decision, all 4,096 deals paired.
Parity with production holds at full size (CI includes zero); the 1,024-deal read was +.008.

Teacher data: `log2_launch.py 8 40 16 4 0.1 r4-L3c --model models/L3c-mix-h128x2.w`, seed 13000000, four capped rounds (280 s each).
Cumulative states: 449,372; 898,408; 1,298,621; 1,733,144 (about 449k, 449k, 400k, 435k new). Final 1,624,340 train + 108,804 val.
Merges: r34 = r3all + r4 = 3,478,237 train + 232,715 val; all2 = all + r4 = 5,316,602 train + 355,185 val.
Same trainer flags (enc 4, loss mix, 128x128, 30000 updates, batch 1024). No training was cut by the cap.
Agreement note: the default Rust `agree` reads only the first 20,000 val records, and its prefix gaps to the trainer were .0095 (L4a) and
.0106 (L4b). Rerunning with `--limit` = full val size gives Rust .62900 / .62467 against trainer .62900 / .62465, so the prefix gap is
subset sampling, not a forward mismatch. Table lists the full-val Rust number. Exploratory tier; seed 5000; all runs completed all deals.

| net | arm | train data | selected update | val RMSE | agreement trainer / Rust (full val) | paired advantage [95% CI] | win rate | decision us (median) | deals paired |
|---|---|---|---:|---:|---|---:|---:|---:|---:|
| L3c-mix-h128x2 | l2n:40 (search) | all (3.69M) | 29000 | .1543 | .6183 / .6146 (20k prefix) | +.007 [-.008, +.023] | .504 | 2374 | 4096 |
| L4a-mix-h128x2 | np | r34 (3.48M) | 28250 | .1573 | .6290 / .6290 | -.106 [-.122, -.090] | .447 | 3 | 4096 |
| L4b-mix-h128x2 | np | all2 (5.32M) | 30000 | .1540 | .6247 / .6247 | -.108 [-.124, -.092] | .446 | 3 | 4096 |
| L4a-mix-h128x2 | l2n:40 (search) | r34 | 28250 | .1573 | same | +.010 [-.005, +.025] | .505 | 2335 | 4096 |

Reading (exploratory): the rung-4 teacher gives no measurable gain over rung 3. L4a and L4b as whole players are
indistinguishable from L3b/L3c (-.106 and -.108 against -.110 and -.100), and the L4a search arm is level with L3c's (+.010 against +.007,
CIs overlap, both include zero). Selector agreement rose to .629 on its own validation data, which does not carry to play strength.
Teacher ceiling for comparison: +.077 [+.048, +.105], 1,024 deals.

### Teacher strength check (lead, 2026-10-05 ~20:40)

`l2n:160` (160-world search, modeled others = rung-4 net L4a) vs production, 1,024 paired deals, seed 5000,
production memo: **+.070 [+.041, +.100]**, win rate .535, 11.5 ms per decision (production 5.3 ms). The
40-world version of the same player is at parity (+.010). So the search width, not the net, is what separates
"level with production" from "beats production by the original teacher's margin", at half the original
teacher's cost. Decision: pipeline LAD2 uses `--outer 160` as the teacher and a 256x256 student (two changes
at once, deliberately, to look for strength rather than attribute it); LAD1 (40 worlds, 128x128) plateaued at
−.10 alone / parity in search for k0..k2.

#### Cycle 4+ (operator agent, Sonnet): pipeline LAD1, stopped early (partial, 3 nets of 12)

`pipeline.py --name LAD1 --ladder --h2h-search`, start net L3c, base data all2 (5,316,602 train + 355,185 val), 12 label workers,
4 h2h workers, 4,096 deals each, seed 5000, binary ladder/target-s. Exploratory tier. Rust agreement was run over the whole val file.

| net | train rows | selected update | val RMSE | agreement trainer / Rust | np adv [95% CI] | np win | l2n:40 adv [95% CI] | l2n win | deals paired |
|---|---:|---:|---:|---|---:|---:|---:|---:|---:|
| LAD1-k0 | 5,821,800 | 28750 | .1554 | .6267 / .6267 | -.110 [-.126, -.094] | .445 | +.002 [-.013, +.017] | .501 | 4096 / 4096 |
| LAD1-k1 | 6,236,199 | 30000 | .1569 | .6258 / .6259 | -.097 [-.113, -.082] | .451 | +.011 [-.004, +.027] | .506 | 4096 / 4096 |
| LAD1-k2 | 6,656,842 | 30000 | .1544 | .6259 / .6258 | -.101 [-.116, -.086] | .449 | +.007 [-.008, +.023] | .504 | 4096 / 4096 |

Failure: nets k3 and k4 (and by construction every later net) did not train. `train.py` aborts in its validation pass with
`[metal::malloc] Attempting to allocate 30629873664 bytes which is greater than the maximum allowed buffer size of 30150672384 bytes`
once the growing val file passes about 500k rows (k2 trained at 476,558 val rows; k3 failed at 511,316; k4 at 552,871 needed 33.1 GB).
`train.py` scores the whole val set in one batch, so the pipeline's append-only val file hits the Metal single-buffer limit.
Fix needs a change to train.py (chunked or capped val evaluation) or a val cap in pipeline.py; both are outside the operator's remit.

Reading (exploratory): across three successive teacher/student rounds nothing moved: np stays near -.10 and the search arm near +.007,
all level with production within CI. The pipeline did not improve on L3c.

### Strong-teacher isolation test (lead, 2026-10-05 ~21:45)

LAD2 (160-world teacher, 256x256 student, base = 7.95M rows of 40-world labels + new rounds): k0 −.100 alone /
+.004 in 40-world search; k1 −.118 alone. The strong-teacher states were diluted about 1:60. Fine-tuning
LAD2-k1 on ONLY the 160-world-teacher states (228k rows, 800 updates, lr 5e-4; `train.py --init`): agreement with
that teacher .624 -> .645, and as the whole player **−.075 [−.090, −.059]** on 4,096 deals (previous best −.097).
First movement of the net alone since rung 3. Decision: LAD3 = strong-teacher data only, each net fine-tuned
from the previous (`pipeline.py --init-prev --lr 0.0005 --updates 3000`), 160-world search teacher, 256x256.

# Overnight 2026-10-05: student plateau broken, regret-gated ladder LAD5

Exploratory tier throughout (paired mirrored h2h vs pinned production Walt, dropped-30 protocol, seed 5000 unless
stated, 4,096 paired deals, 95% CIs, no multiplicity correction). Full chronology, commands and per-agent reports:
`STATUS.md` (sections "Overnight plan" onward, "Student-side experiments" rounds 1–3, "LAD5", "Inner-ring grid",
"How to refresh the teacher ensemble"), `reports/student-{tokens,features,scale}.md`, `PERF.md` ("Batched/memoized
net inference", "Inner worlds as the batch dimension"). Raw h2h summaries under `results/h2h-*/summary.json`;
pipeline events in `pipeline/LAD5/log.jsonl`.

## 1. The student plateau and what broke it

Until 01:00 every student (2-layer embedding-sum MLP, 128 or 256 wide, frozen max/min selector) sat at −.06 alone vs
production, 64% selector agreement with its labels, regardless of width, teacher worlds (40/160/350) or rows
(24k–8M). Three independent agents found the same cause by different routes: **generalization, not capacity or label
noise** (train regret .0067 vs val .0150 on 1.44M strong rows; a fixed student agrees .639 with 160-world labels and
.636 with 350-world labels, so cleaner labels cannot help), and **agreement is the wrong metric** (24% of val rows
are exact ties scored by the lowest-tile rule; an oracle with the exact label probabilities agrees only ~.78 with a
fresh 350-world label). What worked: the regret loss (mean lost value per pick, selection by regret), LayerNorm
residual bodies, token attention over the 28 tiles, strong decoupled weight decay with a 7.5M-row mix, and
distilling an ensemble of the big nets back into the cheap 256x256 MLP.

| net alone | regret* | h2h vs production, seed 5000 | seed 6000 (2,048 deals) | µs/decision |
|---|---|---|---|---|
| LAD4-k17 (old recipe, 256 MLP) | .0168 | −.060 [−.075, −.044] | −.061 [−.083, −.038] | 4 |
| STU-ft-regret-all (k17 + regret loss, 1.44M rows) | .0140 | −.023 [−.039, −.007] | | 3 |
| STU-s-res512-r1 (residual-LN 512, 3 blocks) | .0126 | −.004 [−.019, +.012] | | 76–94 |
| STU-e-mlp256 (256 MLP distilled from 5-net ensemble) | .0123 | −.004 [−.019, +.011] | +.009 [−.012, +.031] | 4 |
| STU-t-mlp256 (256 MLP, 4-net ensemble incl. tokens) | .0121 | +.007 [−.009, +.022] | | 3 |
| STU-w-mlp256 (256 MLP, res512+tokens+W4 ensemble; LAD5 start) | .0119 | +.010 [−.006, +.025] | | 4 |
| STU-t-res512 (residual, 4-net labels) | .0116 | +.012 [−.003, +.027] | | 62 |
| STU-tokens-b2-d48L2 (tile-token transformer, 49.6k params) | .0123 | +.025 [+.010, +.041] | | 64 |
| STU-tokens-b3-d48L2 (one more chained round) | .0117 | +.021 [+.006, +.036] | +.019 [−.003, +.041] | 40–65 |
| STU-scale-W2 (512x512 MLP, 7.5M rows, wd .1 per saved args) | .0117 | +.001 [−.013, +.016] | | 8 |
| STU-scale-W4-res512ln-40k (residual-LN 512, 7.5M rows, wd .1 per saved args; last-step weights, not best-regret step) | .0107 | +.029 [+.014, +.044] | **+.031 [+.010, +.052]** | 44–55 |
| STU-r3-tok-b8 (tokens-b3 + 5 rounds on LAD5 rows) | .0106 | +.030 [+.014, +.045] | | 64 |
| STU-r3-W4c (W4 continued on LAD5 rows) | .0105 | +.020 [+.005, +.035] | | 76 |

*Regret is Rust-side on each designer's validation snapshot (LAD4-val, STU-snap, r3-snap); comparable within a
block, not across the whole column. The roles designer's error analysis (`STATUS.md`, Sonnet report): 2.4% of rows
with a >50-world gap carry 46% of regret, trump/off-suit confusions 27%, roles and seat flat, the tie rule costs
nothing. The knowledge-feature net (.666 agreement trainer-side, `reports/student-features.md`) was never verified
in Rust: its build was denied by the permission classifier.

## 2. Inner-ring grid (pure turbo two-ring, dice leaves; 1,024 deals, seed 5000)

| outer:inner | h2h vs production | ms/decision |
|---|---|---|
| 40:8 (production's budget) | +.008 [−.024, +.038] | 3.7 |
| 40:32 | +.046 [+.017, +.073] | 14 |
| 40:160 | +.039 [+.009, +.069] | 73 |
| 160:8 (reference, earlier) | +.077 [+.048, +.105] | 21 |
| 160:32 | **+.096 [+.067, +.123]** | 94 |

8 inner worlds is too noisy, 32 is enough, 160 adds nothing; outer worlds are the better spend. Walt's occasional
partner-aware play is a sampling artifact of 8 inner worlds, as Jason predicted.

## 3. The net as the modeled others inside a 160-world one-ring search (`l2n:160:<net>`)

| net inside | h2h vs production | ms/decision |
|---|---|---|
| LAD4-k17 | +.033 [+.002, +.062] (1,024 deals) | 20 |
| STU-e-mlp256 | +.088 [+.059, +.118] (1,024) | 20 |
| STU-tokens-b3 | +.088 [+.046, +.130] (512) | ~300 |
| STU-s-res512-r1 | +.116 [+.061, +.171] (301) | ~690 |
| LAD5-k0 … k23 (pipeline arm) | +.090 to +.110 (4,096 each) | 20 |
| LAD5-k52 | +.104 [+.089, +.118] | 20 |
| LAD5-k62 | +.094 [+.079, +.108] | 20 |

The cheap 256 MLP inside the search is as strong as the expensive nets inside it, and the search player is capped
near +.10 at 160 worlds whatever rung sits inside: the remaining headroom is in the search, not the partner model.
Two-ring with the net in the dice slot stays infeasible as a teacher (403k forwards per decision, 1.44x from an
output-identical inference pass, `PERF.md`); the wide-and-shallow `l2w` variant is a flagged option at ~65 ms per
inner world.

## 4. LAD5: the regret-gated ladder (02:46–10:00 CDT, 80 rounds, 78 nets, 57 promoted / 21 rejected)

Teacher = `l2n:160` with the newest promoted 256 MLP inside, which also drives the states (`pipeline.py --ladder
--trainer stu --gate --gate-metric regret`); student = 256x256 MLP, regret loss, initialized from the newest
promoted net each round; promotion iff Rust-side regret on the current val file <= the teacher net's. One capped
round labels ~65k states; a rung every ~5 minutes.

| rung | regret (current val) | alone vs production | note |
|---|---|---|---|
| start (STU-w-mlp256) | .01207 | +.010 [−.006, +.025] | |
| k0 | .01180 | +.017 [+.002, +.032] | first rung clears zero |
| k4 | .01174 | +.024 | |
| k12 | .01134 | +.026 | |
| k25 | .01125 | +.033 | |
| k39 | .01091 | +.032 | end of the 40-net budget; alone flat ≈+.03 since k25 |
| k40 | .01032 | +.044 | **distillation pile dropped from the train mix** (round-3 finding): −.0006 regret in one rung |
| k52 | .01029 | +.054 [+.039, +.068] | |
| k62 | .01016 | +.045 | |
| k70 | .01008 | +.050 | |
| k73 | .01000 | **+.056 [+.041, +.071]** | best alone of the night |
| k77 (last) | .01000 | +.046 | alone flat ≈+.05 since k51 |

Two plateaus, each broken by a data change rather than by more rungs: the ensemble-distillation pile broke the first
(−.06 → parity), and removing that same pile once the teacher's own labels were better broke the second (+.03 →
+.05). The gate rejected 21 of 78 candidates and never let a worse net become the teacher. The round-3 ensemble
refresh (`STATUS.md`) could not produce a 256 MLP that beat the ladder's own: the ladder had overtaken the big nets
alone (k27 +.033 vs tokens-b8 +.030 at 16x the cost).

## 5. Lessons

Select and gate on regret, never on selector agreement (every improved net lost agreement). The old cap was
generalization: the fix was loss, regularization, more and better rows, and distillation, not width. A distilled
pile is a bootstrap, not a permanent ingredient: it lifts a weak student and then holds a strong one back, so drop
it when the teacher's own labels pass it. For a teacher, the cheapest net that is as good inside the search as the
expensive ones wins (4 µs vs 60–90 µs; the search is capped near +.10 at 160 worlds either way); the two-ring with a
net in the dice slot is two orders of magnitude too slow, and 8 inner worlds is the one clearly bad budget in Walt's
own design (32 is enough; outer worlds pay more). The cycle Jason asked to see: a self-improving loop that produced
40 rungs in 3.5 hours and took a 4 µs net from parity to +.05 against production, replicated on a second seed for
the pre-ladder nets.
