# Partnership launch: recentering against the original request

2026-09-06. Exploratory observations and local engineering status.

**Newest deliverables:** the expressive [Scheme/Fix runtime](../../walt/scheme/README.md)
and its first [partnership gym](../../walt/gym/README.md). Six verified
count-offer advantage/disadvantage exercises now have exact lawful answer keys,
same-world witnesses, and an interruptible native-player benchmark. Starter
optimal choices: L1 5/6, L2 Partner 6/6, L2 with voids 5/6. This diagnostic
gallery is not evidence of a general strength ranking. The existing playing
rules and default configurations remain as described below.

**2026-09-07:** [Scheme-driven discovery](../../walt/gym/DISCOVERY.md) expands
the gym to 170 distinct strict late-game coordinates. Three ordinary query
files select count offers, overtaking partner, and leads toward a possible
partner boss across 1,929 saved coordinates. Exact matching and grading remain
separate; no expanded pupil ranking has yet been measured.

**2026-09-07, outcome-only extension:** [433 bid-making exercises](../../walt/gym/BID-MAKING.md)
now select solely by strict make-probability differences. Among 837 exactly
graded declaring coordinates, 367 have a unique best play and 26 have a
certain make/set swing under the fixed field. The complete 1,929-coordinate
scan took 50.616 seconds with ten workers; exclusions and all-tied controls
are retained. No new pupil matchup or player change. The pre-extension work
through `b0c7c0aa` was merged directly into local `main`; development continues
on `codex/partnership-launch` in the same worktree.

**Latest full-game battery completed:** [default fixed-search results](campaigns/default-partner-battery/RESULTS.md),
400 games on 100 shared fresh deals. L2 Partner versus L1: **14 wins / 14 losses /
72 ties** (50% comparative contract wins). L2 Partner with voids versus without:
**12 / 17 / 71** (47.5%). Both rough intervals include 50%; no strength gain is
established. Mean time/move under this load: **0.228 seconds L1, 1.127 L2 Partner,
1.318 L2 Partner with voids**; fallback rates 0%, 1.2%, and 3.3%. All three
completed under the technical gate. The unchanged engine and binary were used.
See the [player family guide](PLAYERS.md) for the now-explicit names. No
experiment is running or scheduled. The foundation battery below remains a
separate historical result, using different search procedures.

The [launch brief](packet/texas42-partnership-launch-v0.1/EXPERIMENT-BRIEF.md)
asked for a lawful, playable partnership-aware player from trick 1, within
60 seconds per four-play trick on the M5 Max, with useful partnership behavior
and at least comparable strength to the actual phone Walt. Global optimality,
GPU use, full L2, and implementing every supplied mathematical proposal were
not requirements. Jason subsequently fixed all evaluation contracts at 30 and
authorized resumable campaigns and ten concurrent games.

## What has been delivered

| Original requirement | Current status | Evidence / limitation |
|---|---|---|
| Isolated worktree and preserved input packet | Complete | `codex/partnership-launch`; packet unpacked, manifest checked, and committed unchanged |
| A usable player from trick 1 | Complete | `player.py` decision interface and `table.py` human table; native baseline, partner-only, and full-L2 diagnostic modes |
| Explicit partner reasoning | Implemented, with a concrete decision witness | The focal solver models its partner at L1 and opponents at L0. Gran's validated 6-2/6-4 root distinguishes the partner upgrade from the matched native baseline. The phone already chooses 6-4 with its own search settings |
| Information consistency and legal mechanics | Focused checks pass | Own hand/public inputs, independent replay, hidden-world invariance, modeled-policy purity, cache separation; not a full mechanics proof or full-repository certification |
| Unified L1 foundation and reusable comparison arena | Complete for this scope | One shared selector at the root and inside modeled minds; independent belief/profile/budget choices; persistent bounded workers and mirrored per-seed/per-hand scoring |
| At most 60 seconds per trick | Met in measured play | Largest four-play trick in the 100-deal panel: 33.961 seconds; fixed-hand panel: 30.950 seconds. The wrapper reserves a legal move and limits each decision to 14 seconds |
| Frozen actual phone reference | Preserved locally | Original Plunge WASM and wrapper, 40/8 with racing and original refinement. The currently deployed phone version was not independently verified |
| At least comparable strength | Not established for the partner upgrade | Both larger voidless panels had more unfavorable contract flips than favorable flips; details below |
| Reproducible results and honest reporting | Complete for executed work | Named fixtures, source/binary identities, every move, clocks, fallbacks, paired make/set summaries, watchdog records, and local commits |

## Latest: foundation and fresh matched battery

The [foundation record](FOUNDATION.md) explains the shared selection authority,
cache and information boundaries, completed-comparison contract, and checks.
The [fresh battery](campaigns/foundation-battery/RESULTS.md) published 524 games /
14,672 independently verified moves in 13.74 minutes of capped foreground pool
time. All 4,639 moves saved at its first boundary survived resume unchanged.

- Native `l1-race` reproduces the recovered phone procedure: 64 direct checks,
  and identical 28-move trajectories in all 35 fallback-free mirrored pairs.
  All first divergences in the other pairs began at phone fallbacks.
- On 50 random deals, race versus fixed L1 scored 4 wins / 5 losses / 41 ties;
  refinement versus race scored 4 / 2 / 44. Their order is unresolved.
- Void-aware race versus voidless race scored 9 / 5 / 36 on 50 random deals,
  but 5 / 8 / 37 on five opening hands × ten hidden completions. There is no
  established general voids strength gain. All native L1 comparisons completed
  with zero fallbacks.
- Every tested refined partner configuration crossed the declared >5% fallback
  threshold, including smaller budgets and the configuration
  with fixed-rule modeled L1 minds. These short stopped panels reveal a cost
  problem; they cannot rank the intended deeper policy's strength.
- Per-move means under load were 0.137 seconds fixed L1, 0.313 refinement L1,
  0.502 racing L1, 0.535 void-aware racing L1, and 0.778 archived phone. These
  include forced moves and wrapper work. Defaults remain unchanged.

Use native `l1-race` as the internal counterpart of the recovered phone rule.
Retain the literal phone for occasional artifact checks. Measured parity does
not prove whole-program equivalence or retroactively replace the older fixed
L1 measurements. No experiment is running or scheduled.

## Earlier panels: original fixed-root candidate

- **Random panel:** 100 deals / 300 games; 97 fresh deal units after excluding
  three development seeds. Partner-only candidate: 15 favorable contract flips,
  23 unfavorable, 156 ties; combined improvement estimate **−4.1 percentage
  points**. [Results](campaigns/random-420600-699/RESULTS.md).
- **Fixed-hand panel:** ten opening hands × ten distinct hidden completions,
  three games per completion; 300 games. Candidate: 21 favorable flips,
  27 unfavorable, 152 ties; **−3.0 percentage points**. These are ten hand
  clusters, not 100 independent opening positions. Every lineup held its opening
  choice fixed across its ten worlds, while outcomes varied substantially.
  [Grouped results](campaigns/worlds-520600-699/WORLD-RESULTS.md).
- These compare the executed **voidless partner-only candidate** to the phone.
  They do not grade the new void-aware option or establish a broad full-L2 rank.
  Making 30 and making 42 tie; being set at 7 and at 29 tie.

## Useful infrastructure gained along the way

- A shared ten-game pool with monitoring, per-move checkpoints, retry records,
  and contiguous-seed reporting. Interruptions preserve completed work.
- Same-opening-hand / different-hidden-completion experiments, with explicit
  hand-group reports and opening information-invariance checks.
- A measured Mac throughput of 300 games in 9.58 minutes on the fixed-hand panel.
  Recorded average decision latency there: phone 0.704 seconds, native partner
  0.590 seconds including forced moves. These are different implementations and
  search schedules, not a controlled cost ratio of L1 versus L2.
- A shared [inner-belief strategy](INNER-BELIEF.md), retaining legacy behavior
  and adding opt-in void-conditioned counted sampling. Public voids persist
  through modeled continuations and both cache types; no second search engine.
  The subsequent foundation battery above adds full-setting L1 strength and
  latency observations; refined partner configurations stopped on cost.
- A five-minute native-L1 / phone reference calibration, recorded separately
  [here](campaigns/native-l1-vs-phone-620600-649/CALIBRATION.md): 50 fresh deals,
  150 games, 4,200 independently verified moves, completed in **202.12 seconds**.
  Native L1: **8 favorable flips / 15 unfavorable / 77 ties**, a **−7.0-point**
  combined difference. Average decision time was **0.080 seconds native** versus
  **0.636 seconds phone** (0.121 versus 0.964 seconds for nonforced decisions).
  No native fallbacks; 15 phone fallbacks. This does not establish equivalence.

The older fixed native L1 and original partner upgrade were compared with the
phone on **different panels**; their reported deficits cannot be subtracted to
establish a partner-model gain. The new matched comparisons preserve physical
deals across matchups and identify every selection rule explicitly.

## What remains open, in order

1. Profile the expensive partner-field decisions before enlarging that model.
   Fixed L2 Partner completed the follow-up with modest fallback rates, but
   cost about five times default L1 without a demonstrated gain. The stronger
   real-root/model schedules crossed the technical fallback gate earlier.
2. Establish whether a practical, cost-matched partner upgrade adds strength.
   Improving a completed model is not sufficient if more real decisions fall
   back. The fixed-root partner now has a direct matched result against default
   L1: tied on the 100-deal follow-up, with uncertainty about wider strength.
3. Resolve selection and void-option strength with more independent hands when
   requested. Changing the inner sampler also changes its deterministic sample
   stream; individual move changes alone do not isolate void conditioning.
   More hidden completions per hand and more distinct opening hands answer
   different generalization questions. No further expansion has been launched.
4. Add probability calibration scoring against a specified continuation policy
   if desired. Paired contract scoring is implemented; Brier scoring and
   oracle-makeability annotations were discussed but are not implemented.

Auction strength and bid prediction are deliberately deferred. A fitted human
partner model, broader strategic inference from observed actions, information
penalties, GPU acceleration, and global optimality remain possible research
directions, not requirements silently added to this milestone.

Full Rust CI was deliberately skipped under the launch instruction. The latest
change passed 44 focused Rust tests (one additional pre-existing ignored test),
14 Python tests, native clippy with warnings denied, workspace compilation,
and actual WASM target compilation, plus the decision parity and full replay
checks above. Those checks have their stated scope.

The engineering deliverables are usable. The central empirical question—does
the richer partner model meet or improve on the reference's playing strength?—
remains open. The original packet explicitly permits a fast, lawful candidate
that loses to be a valuable experimental outcome.
