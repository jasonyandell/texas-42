# Sunshine: from the gym to a Mac table

2026-09-13. **Exploratory engineering and conditional-policy evidence.**
The [protocol](PROTOCOL.md) preceded the override comparison. This cycle keeps
the measured player unchanged and makes it usable in Plunge, with original
decision evidence and a route back into the gym. [Operating guide](../../PLUNGE.md).

## Override study

Compare requiring one, two, or three extra sampled successes before changing
the L1 move. Every complete census can still act on any strict gain; insufficient
samples and ties keep L1. Select the lowest exact mean regret on the existing
143 development roots, breaking ties toward the smaller margin.

| Required sampled gain | Improved roots | Harmed roots | Changed roots | Mean regret (percentage points) |
|---|---:|---:|---:|---:|
| 1 (current rule) | 11 | 2 | 13 | 0.0608 |
| 2 | 10 | 1 | 11 | 0.0627 |
| 3 | 8 | 1 | 9 | 0.0740 |

**Keep margin one.** The stricter rules avoid some harm but suppress useful
changes too. On the 25 already-seen follow-up roots, margins two and three
avoid the previous one-root harm while retaining its one improvement. That is
a known regression check, not new held-out evidence, and does not override the
predeclared development selection.

Thirty-two fresh random subsets per known root also favor margin one on mean
development regret: 0.0287, 0.0356, and 0.0540 percentage points respectively.
These are sampling-sensitivity measurements on the same positions, **not
independent deals or calibrated confidence intervals**. Larger margins reduce
harms but miss more benefits. The small differences do not settle an optimal
threshold. The existing 8-world minimum, 64-world cap, support cap and deadline
remain unchanged. [Exact fractions and per-root rows](override-study.json).

## The playable instrument

Plunge's `codex/sunshine-table` branch starts from main `122ea7a5` and calls the
existing native `l1-default` or `l1-partner-rollout`. Computer declarations use
the existing Plunge hard player. Contracts are assigned 30 bids with rotating
bidders. All three computer seats use the selected native play preset.
The native player has no new tuning or game-strength claim from this work.
The implemented Plunge commit is `adfd7d4b3707f2324c395c88e5f7dbae1d3b315a`.

The bridge admits only the actor's original hand and public history. It saves
the full native response before replying; exact retries under the same
implementation reuse it. Human and computer moves can be flagged after a hand.
Flags retain the hand, note, legal suggested alternative and original receipt.
Examiner data enters a separate finished-hand endpoint and is independently
replayed. Plunge stops when a points contract settles; the importer checks that
stopping point instead of demanding artificial plays after the result.

Full comparisons use the existing deployed gym with selectable focal, partner
and opponent presets. They compare all root moves over a census of at most
400 compatible hands. Larger positions are saved but explicitly outside the
full-comparison scope. Each slice is externally capped, pause/resume retains
complete trajectories and decisions, and partial work never gets a census
label. Analysis reuse tracks the actual model settings and evaluation code,
independently of presentation changes. The local offline cache is disabled;
research API responses are never treated as static assets.

## First played examples

A browser-played 30/aces hand completed after six tricks, including a reload
and resume after the first trick. All 18 computer plays have saved native
receipts. Their total measured decision time was about 4.02 seconds, ranging
from forced moves below a millisecond to a 1.33-second early choice. This is
one integration hand, not a performance distribution or strength experiment.

The [first flagged move](examples/earl-risk-count.json) is Earl's 6–4 while
defending, with two dominoes left before playing. Ruby later takes the trick
with 1–1 and sets the bid. Complete L1 continuations give the 6–4 a set frequency
of **12/18**, compared with **6/18** for withholding count with 2–2. This is an
example of useful risk already recognized by L1; the declaring-side partnership
check was inactive. It is not an improvement attributable to the new check.

The [second flag](examples/gran-with-l2-partner.json) covers Gran's earlier
6–2 with three dominoes left. An L2 partner/L1-other-seats comparison over 60
compatible hands yields 41/60 makes for 6–2, 38/60 for 4–4, and 25/60 for 6–6.
These are named-model comparisons under a uniform mechanical belief, not
probabilities calibrated to the human who actually played the hand.

The initial 18-hand comparison took 0.25 seconds; the 60-hand L2-partner
comparison took 1.28 seconds including the watchdog. Both were faster than a
manual pause click. A separate automated interruption did pause a real native
180-trajectory comparison; all 49 previously saved files remained byte-identical
after resume. After tightening cache identity, a second interruption preserved
all 42 saved files and completed in 1.71 seconds including pause/resume and
audits. [Initial audit](live-audit.json), [final audit](live-audit-final.json).

## Validation and receipts

- 97 research Python tests passed, including the importer, strict live input
  boundary, immutable receipt checks, continuation identity, oversized-support
  refusal and resume behavior.
- 116 Plunge tests passed across native transport/cache, store/UI, share codes,
  request/explanation adapters and engine rules/invariants. Typecheck and the
  production build passed; the native-mode production build was also checked.
- The independent Python referee matched Plunge at all **132** recorded
  positions across all nine straight declarations. The checked
  [cross-engine fixture](codec-audit.json) is emitted explicitly by the Plunge
  native-table tests.
- Browser validation covered a complete hand, reload/resume, note and legal
  alternative selection, original-receipt display, flag reopening at the
  selected play, switching flagged links, and completed comparison display.
- All saved example keys passed the deployed-gym verifier, including trace
  legality, information-set inputs, role configurations, census coverage and
  reported action scores. Native failures are surfaced instead of selecting
  an unrecorded browser fallback.

[Run receipts](run-receipts.json) retain failed development checks as well as
the passing checks. Full logs are under
`/Users/jason/data/texas-42/sunshine-playable-v1/runs/`. Original hand receipts,
flags and complete gym keys are under
`/Users/jason/data/texas-42/plunge-sunshine/`;
[raw-key hashes](examples/raw-key-hashes.json) pin the full audits.

The next measurement can be a human observation: flag where partnership play
felt wrong or surprisingly good, state the alternative, and vary the named
continuation before deciding what the player should learn. We have not added
an ordinary-game strength claim or a new broad partnership mechanism.
