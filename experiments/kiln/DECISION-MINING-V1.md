# Learn where another look helps

Exploratory protocol, 2026-09-18. Frozen before evaluating alternative actions.
This is a bounded next Sunshine cycle, not a change to the phone player.

## Question

Do the previously learned opponent-top-trump Schemes identify decisions where
the shared deployed Walt misses a better play? Can a small, lawful action
description learned from several such decisions transfer to new deals?

## Domain and separation

- Development: source deals 420600..420609, trial 0, all four bidder hands and
  all nine declarations from the completed actual-play campaign.
- Fresh: new source deals 420635..420644, trial 0, the same 360-game design and
  pinned full-game producer. Generate these separately; no earlier mining run
  has used them. Freeze the selected rule before assessing their alternatives.
- Extract declaring-team decisions at plies 20..23 (two dominoes in the acting
  hand), with two legal moves and an unsettled bid30 contract. Deduplicate own/
  public coordinates. Select at most 12 per source deal by a fixed hash of the
  coordinate, without outcome, original choice or threat filtering. Retain the
  whole selection ledger, including ties and nonmatches.
- Every assessed root uses its entire uniform mechanically compatible support
  (at most 90 worlds). This is a declared prior, not a behavior-conditioned
  posterior. Actual recorded hidden hands and the source's terminal outcome are
  never inputs to the player or the learner's feature view.
- Two policy seeds per coordinate: the original source seed and an independently
  labelled hash. All root actions and worlds share each seed. The measured
  policy is the actual shared `walt_player::decide`, 40 worlds, partner review,
  14 seconds. After forcing one root action, all four seats finish normally.
  Exact own/public calls are cached within an assessment, including their full
  first response. Only completed assessments are published; every decision and
  terminal history is independently audited. Report any budget/fallback use.

## Description and learning

Re-evaluate the frozen `relational-0` and `relational-1` Schemes at the current
position: presence of the *live* top trump with either opponent. Integrate each
over the entire support. This is an explicit temporal extension of the opening
association, whose usefulness here remains a hypothesis.

For legal actions, use witnessed single atoms and conjunctions of two atoms
from a fixed mechanical vocabulary: called/not called, double/not double,
count 0/5/10, and boss/not boss in the context that action would lead. These
are executable Viewer-only Schemes, with no physical tile identities. No
handcrafted score bonuses, attention model or count-offer override.

A candidate preserves the baseline unless its Scheme selects exactly one legal
action. Candidate gates are: always; maximum of the two opponent-boss presences
at least 1/3; at least 2/3. Also retain the empty baseline candidate.
Rank by average paired make-probability gain, weighting coordinates equally and
averaging their two seeds. Require at least eight changed coordinates across
at least three source deals, positive mean gain under each seed, and positive
gain in at least three source deals. Ties prefer fewer atoms, then canonical
identity. If none qualify, freeze the empty baseline. Publish every candidate's
score, not merely the winner. This is supervised policy-fragment search; its
training gains are selected and optimistic.

## Fresh checks and outcomes

1. Primary: selected fragment's equal-coordinate paired make-probability gain
   on the new deals; report helped/harmed/tied roots, override coverage, both
   seed results and source-deal clustered descriptive uncertainty. A positive
   mean alone is not a settled strength claim.
2. Detector diagnostic: missed-opportunity frequency and baseline regret by
   the predeclared 1/3 and 2/3 threat gates, including ungated controls.
3. Preserve useful and harmful concrete witnesses: same compatible world,
   same policy seed, alternative initial action, complete ordinary continuations.
4. Sample-efficiency diagnostic: using the retained full-support paired outcome
   vectors, examine whether threat strata explain action-difference variation.
   Do not promote this to a new sampling method without a separate test.

The intervention is one root move. These results cannot establish the effect
of applying a learned fragment repeatedly during a game. Hidden event labels
remain examiner data; the candidate uses only their support-integrated presence
derived from own/public inputs. No selected override is deployed automatically.

## Execution

External artifacts live under `~/data/texas-42/kiln-played-v1/decision-mining-v1`.
Run parallel, minute-sized resumable batches with process-group watchdogs.
Store root inputs, source receipts, query text, binaries/source hashes, original
responses, support worlds, paired traces, immutable fit and fresh evaluation.
An interruption loses only assessments still in flight; completed roots survive.
If fresh results are unhelpful, retain the negative result and stop this cycle.
