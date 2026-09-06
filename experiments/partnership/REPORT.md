# Partnership launch — results, 2026-09-06

## Completed 100-deal follow-up

**EXPLORATORY.** The [bid-30 follow-up](campaigns/random-420600-699/RESULTS.md)
completed seeds 420600–420699: 300 games and 8,400 independently verified moves.
The 97 fresh seeds produced **15 favorable contract flips, 23 unfavorable
flips, and 156 ties**. Candidate declaring was 8 wins / 16 losses / 73 ties;
candidate defending was 7 / 7 / 83. No early-stop rule triggered. These
settings produced more losses than wins in this panel and did not establish
a strength improvement.

The user-authorized [shared pool](POOL.md) increased concurrency after 48
completed seeds. It finished the remaining 52 seeds in 6.2 minutes. The report
preserves the execution change and deadline fallbacks; points never break
make/set ties. Full records, resume verification, and runtime calibration are
checked in beside the source packet.

## Same opening hand, different hidden deals

**EXPLORATORY.** The [fixed-hand panel](campaigns/worlds-520600-699/WORLD-RESULTS.md)
completed ten opening hands, ten hidden completions per hand, and three lineups
per completion: 300 games / 8,400 verified moves. Each group had ten distinct
partner hands and ten distinct complete hidden deals. Every lineup kept the
same opening choice across its ten worlds, with zero opening fallbacks.

Outcomes still varied: phone had both makes and sets in all ten hand groups;
candidate declaring did in nine (the remaining group made 10/10). H1's fixed
candidate 2-1 lead yielded 14–42 points and 4/10 makes. H5 improved from phone
3/10 to candidate 7/10 makes, while H3 declined from 9/10 to 6/10. Full paired
outcomes were 21 favorable flips / 27 unfavorable flips / 152 ties. These are
ten hand clusters, not 100 independent opening positions. The report preserves
opening model scores as diagnostics rather than calibrated probabilities.

## Original launch batch

**EXPLORATORY. A lawful, playable native prototype was built and tested within
the latency target. The strength goal was not reached: the partner upgrade
produced no favorable contract flip in this small batch and caused one
defensive regression. Full L2 reproduced that regression.**

The useful result is a working player, a frozen literal local-phone comparison,
and a reproducible failure where deeper modeling changes a contract outcome.
The phone player should remain the strength reference. This is a legitimate
negative result for the tested settings, not evidence that partnership reasoning
cannot work.

## What was built

[`player.py`](player.py) is a JSON-lines decision oracle from trick 1 onward.
[`table.py`](table.py) seats a human with the prototype beside two phone Walts;
see [the launch commands](README.md). Scope is straight play, bids 30–42, all
nine declarations. The table has a chosen contract, not a new auction.

The native candidate uses the existing information-grouped focal solver.
At the partner's turn it evaluates a **level-1 modeled mind**, which samples
unknown hands and best responds to its level-0 field. At the two opponents'
turns it uses the original level-0 minds. The partner can change its response
when tiles and scores become public. Its actual hidden hand is never an input
to the focal player. `all-l1` upgrades all three modeled seats, giving a full
L2 comparison. `baseline` holds the same fixed-sample search and changes only
the modeled partner back to level 0.

Native comparison schedule: **40 outer worlds, n0=8, n1=2**, six Rayon threads,
ascending-tile tie rule, no tie refinement. Every root action shares the same
outer worlds. A completed result reports exact rational averages on those
samples, **not exact game probabilities**. Nested modeled minds keep their
existing coarse information approximation: they do not condition their inner
samples on old void deductions. No human behavior model was fitted.

The exact local Plunge WASM and wrapper were copied byte-for-byte. Its play
configuration is **40/8, racing on**, with its original tie refinement. It
differs from today's native source. [BASELINE.md](BASELINE.md) records the
digests, source commits, public seed convention, and actual heuristic bidding
versus Walt declaration behavior. The local checkout was verified; the currently
deployed phone version was not independently verified.

The new actor uses one public decision seed, 420600, independent of the hidden
deal generator. The internal stream matches the phone formula using the
**original** hand. Root support is conditioned on public mechanical evidence;
the player does not claim a calibrated behavioral posterior. Each modeled
policy is fixed by its own lawful inputs and approximation settings. Caches
are evaluation-local, and the model never receives the outer sample index,
other actual hands, or a hidden-dependent random seed.

## Four separate findings

| Dimension | Observed result | Limit |
|---|---|---|
| Lawfulness | Seven focused Rust gates pass; 252 native/Python position comparisons pass across all declarations and seats; all 336 driven decisions legal with independent leader/score agreement | Focused evidence plus code audit; not full repository certification |
| Latency | 12 complete hands; maximum four-play trick **24.238172 s**; maximum decision **12.796294 s**; zero gameplay fallbacks, no incomplete hands | M5 Max measurements on this small suite, not a universal runtime bound |
| Partnership behavior | With matched samples, the Gran 6-2/6-4 root changes from an L1 tie to a strict partner-model preference for 6-4 | The phone already chooses 6-4 with its own racing/refinement; this does not establish a repaired phone error |
| Strength | Across three unique fresh deals and two candidate team placements per deal: **0 favorable contract flips, 1 unfavorable flip, 5 unchanged outcomes** relative to the corresponding all-phone team; one extra mixed-team hand unchanged | Too small and restricted for a population win-rate claim; the observed regression is real |

All 12 hands total 181.833 seconds of gameplay computation in the instrument.
The 12 comprise nine primary comparison hands (three lineups on each of three
deals), two diagnostic reruns of one deal, and one mixed-team hand. They are
**three independent deal identities, not twelve fresh deals**. The candidate
team made 0/3 declaring contracts and set 2/3 defending contracts; the matching
phone teams made 0/3 and set 3/3. These contracts make this a weak strength
sample: all three all-phone references were set. Extra points do not count as
an improved contract outcome.

Phone decisions were 149 completed phone evaluations plus 68 forced moves;
partner-only decisions were 57 completed evaluations plus 34 forced moves.
The remaining 28 decisions used the baseline or all-L1 diagnostic lineups.
There were no hidden substitutions of a native fallback for a phone move.

## Actual play outcomes

Scores below are **declaring-team points** against bid 30. Each row uses an
identical deal, bidder, trump, and public decision seed across lineups.

| Deal | Bidder / trump | All phone | Candidate declaring pair | Candidate defending pair |
|---|---|---:|---:|---:|
| 420601 | S0 / sixes, preselected contract | 19, set | 13, set | 2, set |
| 420602 | S3 / ones | 4, set | 23, set | **36, made: regression** |
| 420603 | S1 / threes | 25, set | 24, set | 15, set |

After the first deal proved uninformative at the contract level, the next two
contracts were selected before play by a fixed fixture rule: longest pip-trump
hand, then trump double, then off-trump doubles, deterministic seat/pip ties.
This rule examines the referee deal only to freeze the common contract. It is
**a fixture generator, not a lawful auction algorithm or auction result**.
All player requests still carry only own-hand/public information.

On 420603, placing **only the bidder's partner** at the candidate seat, with
the bidder and both opponents still phone players, produced 15 declaring
points: set, the same contract outcome as the 25-point phone reference.

On the 420602 defensive regression, the fixed-sample weaker-partner ablation
held the declarer to **25**, still a set. Full L2 let the declarer make **36**,
matching the partner-only failure. Thus the loss is not explained solely by
removing the phone's racing/refinement. These are diagnostic repeats, not new
held-out successes or failures. All play continued legally through seven tricks.

The teams occupy both seat parities and declaring/defending roles. No additional
physical rotation of a deal was run; those symmetries remain untested here.
All gameplay comparisons used the frozen 40/8/2 native schedule. The contract
rule was fixed before outcomes on each fresh deal. No significance claim is
made from sequential inspection.

## Two decision witnesses

### Gran's actual information root

G1, sixes, bid 30. Jason leads 6-6, the next seat plays 6-0, and Gran must
choose 6-2 or the ten-count 6-4. This is the fully reconstructed historical
hand, with a **new experiment seed**, not a recreated phone random stream.

| Mode, same native 40/8 outer evidence | Estimated make after 6-2 | After 6-4 | Choice |
|---|---:|---:|---|
| Baseline | 33/40 | 33/40 | 6-2, index tie |
| Partner-only, n1=2 | 34/40 | 37/40 | 6-4 |
| Full L2, n1=2 | 38/40 | 36/40 | 6-2 |

The partner-only decision takes 0.790 seconds including its retained fallback.
The ten count is banked under the bidder's winner and 6-4 is made public; the
bidder's modeled future decisions can respond to that information. The ablation
shows that changing only the partner model changes the ranking. It does not
isolate revelation from score/resource effects, nor prove the candidate models
the bidder correctly. Full L2's reversal demonstrates that opponent modeling
also matters. The independent phone run selected 6-4 as well.

G3's later saturation root remains a four-way sampled tie under all three
native profiles. This experiment did **not** resolve that partnership ambiguity.

### A preserved failure after identical public play

New fixture `regression-602-t3-s0`: defender S0, ones trump, S3 bidding 30.
The public prefix is identical between the baseline and partner-only lineups
up to S3's 1-1 lead on trick 3. S0 can slough from
`3-2, 4-2, 6-2, 6-3, 6-5`.

| Mode | Declarer make estimate after 4-2 | After 6-5 | Defender's choice |
|---|---:|---:|---|
| Baseline | 20/40 | 21/40 | 4-2 |
| Partner-only | 20/40 | 19/40 | 6-5 |
| Full L2 | 17/40 | 16/40 | 6-5 |

The phone also chooses 4-2. The two deeper models turn a **one-world margin
out of forty** into the opposite choice. This small margin motivates a
sampling-stability check; it does not prove a structural defect in all L2.
The complete-policy outcome regresses from set to made. The first differing
tile is a localized witness, **not proof that this tile alone causes the
entire loss**, since subsequent policies and public histories also diverge.

## Deadlines, checks, and provenance

The wrapper derives legality locally before starting any child, retains the
lowest legal move, and attempts a complete 8/2 L1 fallback for up to 1.5 seconds.
It then gives the requested mode the remaining 14-second allowance with cleanup
reserve. Every response, including fallback, is checked against independent
legal/leader/point data. The native solver checks deadlines within recursion
and inner/outer sampling. A deadline abort discards the incomplete comparison;
it never inserts a fallback into the modeled-policy cache. The Python timeout
kills and reaps an overdue worker. All workers inherit the batch's process group.

Focused tests include cold and shared-cache hidden-completion invariance,
banked-score/own-hand cache separation, all-zero/all-one field equivalence,
CRN parity, deterministic complete actions, immediate native deadline refusal,
an actual subprocess timeout, invalid/private-input rejection, worker-startup
failure, malformed fallback rejection, and a 100 ms decision that returned its
legal fallback in **53.577 ms**. The complete G1 referee replay independently
reproduced 25–17. A separate end-to-end table smoke drove all seven human
prompts with legal input while the real computer players responded; it
completed in 6.840 seconds. That usability check is excluded from strength
counts. The final release checks passed again after adding the regression
fixture.

**Full Rust CI was deliberately not run**, under the launch instruction. The
focused Rust checks link the built library directly, avoiding unrelated probe
builds. No ingest verifier, Lean claim, global-optimality proof, GPU kernel,
or referenced mathematical companion program was claimed to have been run.

Hardware: M5 Max, 48 GiB, 18 cores; workers use six Rayon threads. Rust/cargo
1.95.0, Python 3.9.6, Node 26.0.0. The isolated native build took 4.440 seconds
on its final build run; worker startup and cache population are included in
decision times. No prior solve cache or GPU preparation was used. The measured
opening costs are dominated by modeled-policy calls and recursive nodes, not
the small outer sampler, so a GPU or full exact-integration port was not needed
to reach this timing milestone.

All builds, checks, and game batches use the supplied watchdog with allowances
at most 295 seconds. The expected two-second watchdog smoke timed out and
cleaned up its parent/child. An initial compile and test-compile failure were
fixed and retained in the logs. All subsequent declared gameplay batches
completed; no 300-second ceiling was exceeded. Scheduling prevents a
mathematical real-time guarantee; the legal fallback and reserve are the
operational boundary, and overruns are explicitly measured.

Base `9d6a5a2`; packet checkpoint `d840071`; player checkpoint `cfb0fb2` on
`codex/partnership-launch`. The original checkout was left untouched. Every
packet manifest entry was verified, as were the preserved phone artifact
digests. [SUMMARY.json](SUMMARY.json) is generated by
[`summarize.py`](summarize.py) from the raw run records; repeated hand names are
qualified by run id. Each run directory retains the exact command, elapsed
time, exit, stdout/stderr, binary identities, seeds/settings, and actual work
counters. Counter values can vary with parallel duplicate work; completed
action values retain deterministic semantics.

## What remains and the next small experiment

The artifact is usable and the investigated route has an honest outcome. It
has **not closed the partnership strength gap**. The immediate uncertainty is
whether the 420602 reversal reflects two-world inner modeling/sampling noise,
or a stable mismatch between the modeled partner and the actually executing
phone policy. Behavioral inference, full void-conditioned inner reasoning,
tie refinement for the deeper player, and human coordination remain open.

The next informative capped experiment is a preregistered stability panel on
the preserved failure: independent public tapes and n1=2 versus 4, holding the
outer 40/8 evidence scheme fixed, followed by actual paired continuations.
Only after selecting a change on that diagnostic should untouched seed 420604
be used for a fresh test. This session did not consume that final reserved deal
or continue tuning until a favorable strength result appeared.
