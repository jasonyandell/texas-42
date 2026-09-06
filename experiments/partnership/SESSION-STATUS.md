# Partnership launch: recentering against the original request

2026-09-06. Exploratory observations and local engineering status.

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
| At most 60 seconds per trick | Met in measured play | Largest four-play trick in the 100-deal panel: 33.961 seconds; fixed-hand panel: 30.950 seconds. The wrapper reserves a legal move and limits each decision to 14 seconds |
| Frozen actual phone reference | Preserved locally | Original Plunge WASM and wrapper, 40/8 with racing and original refinement. The currently deployed phone version was not independently verified |
| At least comparable strength | Not established for the partner upgrade | Both larger voidless panels had more unfavorable contract flips than favorable flips; details below |
| Reproducible results and honest reporting | Complete for executed work | Named fixtures, source/binary identities, every move, clocks, fallbacks, paired make/set summaries, watchdog records, and local commits |

## What the larger panels actually established

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
  The option has compatibility and support checks, but no full-setting strength
  or latency campaign yet.
- A five-minute native-L1 / phone reference calibration, recorded separately
  [here](campaigns/native-l1-vs-phone-620600-649/CALIBRATION.md): 50 fresh deals,
  150 games, 4,200 independently verified moves, completed in **202.12 seconds**.
  Native L1: **8 favorable flips / 15 unfavorable / 77 ties**, a **−7.0-point**
  combined difference. Average decision time was **0.080 seconds native** versus
  **0.636 seconds phone** (0.121 versus 0.964 seconds for nonforced decisions).
  No native fallbacks; 15 phone fallbacks. This does not establish equivalence.

The native reference is suitable for routine controlled comparisons within the
same engine. Keep the archived phone as an occasional external strength anchor.
Do not substitute native L1 for historical phone results or treat native-relative
improvement as having met the original phone-strength criterion. Native L1 and
the partner upgrade were compared with the phone on **different panels**; their
reported deficits cannot be subtracted to establish a partner-model gain.

## What remains open, in order

1. Use explicitly named native L1 as the internal development reference;
   keep the archived phone and historical comparison intact. The five-minute
   calibration supports a practical separation of these roles, not equality.
2. Measure the new void-aware option's full-setting latency and actual contract
   outcomes. A useful next comparison holds the native engine, bid 30, paired
   deals, seat roles, and sample budgets fixed while varying belief and partner
   level. Changing the inner sampler also changes its deterministic sample
   stream; individual decision changes alone do not isolate the cause.
3. Establish whether the partner upgrade adds strength under those beliefs.
   More hidden completions per hand and more distinct opening hands answer
   different generalization questions. Neither expansion has been launched.
4. Add probability calibration scoring against a specified continuation policy
   if desired. Paired contract scoring is implemented; Brier scoring and
   oracle-makeability annotations were discussed but are not implemented.

Auction strength and bid prediction are deliberately deferred. A fitted human
partner model, broader strategic inference from observed actions, information
penalties, GPU acceleration, and global optimality remain possible research
directions, not requirements silently added to this milestone.

Full Rust CI was deliberately skipped under the launch instruction. The latest
change passed 36 focused Rust tests (one additional pre-existing ignored test),
Python integration/replay checks, native workspace compilation, and actual WASM
target compilation. Those checks have their stated scope.

The engineering deliverables are usable. The central empirical question—does
the richer partner model meet or improve on the reference's playing strength?—
remains open. The original packet explicitly permits a fast, lawful candidate
that loses to be a valuable experimental outcome.
