# Remembering the observations that changed a bid

Exploratory instrument and research direction, authorized 2026-09-18. The running
player, random-world generator, sampling weights and screening rules are unchanged.

## What is retained

The database retains every completed game, including games that changed no
recommendation: the full deal, independent hidden-completion and policy seed
recipes, original acting-hand/public-history requests, all 28 responses, legal
alternatives and their final reported values, partner review, timing, score, and
immutable producer/source identity. Scores remain integers; decisions use the
exact 4/5 empirical cutoff. A greater sample count never replaces the old games.

`history.py` reconstructs every sample prefix in **trial order**, independent of
worker completion order. It records each recommendation change, the before/after
histograms and score tails, the added receipt, crossed thresholds, and hashes
binding both prefixes to their original receipts. Milestones at 8/40/160 are
included. Content-addressed reports are saved under the external campaign's
`history/` directory and are included automatically in subsequent backups.
No production database mutation, worker restart or extra Walt solve is needed.

```sh
python3 experiments/kiln/history.py report /Users/jason/data/texas-42/kiln-played-v1
python3 experiments/kiln/history.py explain /Users/jason/data/texas-42/kiln-played-v1 --cell 351 --n 17 --against-trial 0
```

The explanation packet contains the original game and producer metadata, all
seven tricks, both aggregate distributions and all prefix observation identities.
An explicitly requested contrast includes another original game, the changes in
each seat's hand, both opening choices and both scores. Contrasts are observations,
not causal attributions. The tool verifies receipt hashes, all plays and scoring
against the independent rules before emitting a packet. A report uses the already
validated database metadata; it does not claim to re-audit every receipt itself.

## First retained witness

Cell 351: original deal seed 420609, seat 2, no trump, own tile IDs
`[2,10,17,18,20,25,26]`. After 16 games, 13 reached 35 points: 13/16 clears 4/5.
The next game scored 33, leaving 13/17 at 35, while 14/17 reached 33. The
recommendation consequently fell from 35 to 33. The player still targeted 30
throughout every game; the added game itself made 30.

The first history snapshot covers 12,056 games in 900 cells. It reconstructs 511
recommendation changes after the first observation, 72 after at least eight
games. These counts describe threshold movement, not 511 independent discoveries
or confirmed strategic mechanisms. Initial recommendations are labeled separately.
Compact identities are retained in sample-history-evidence.json; full reports and
the witness packet live with the campaign, outside the worktree.

## The hypothesis to test later

Some kinds of hidden completion may matter disproportionately to a decision.
Learning to spend samples on those cases could identify a usable bid or move
sooner. The current evidence can help discover and test that hypothesis; it does
not establish that a recommendation-flipping observation is strategically rare
or especially informative on future hands. Some changes are ordinary cutoff noise.

Keep these experiments separate:

1. **Allocate empirical trials.** Learn which hand/declaration estimates need
   another complete game. Compare decision stability and cost on held-out hands.
2. **Change the world proposal.** Draw certain hidden completions more frequently
   while continuing to estimate the original target distribution. Record each
   proposal and its probability, target probability, stratum, and weight. For a
   fixed score indicator f, the finite-sum identity is
   `sum_w q(w) [p(w)/q(w)] f(w) = sum_w p(w) f(w)` when q has support wherever p
   does. Thus extra attention to rare losses need not falsely make them common.
   The present opening target has `choose(21,7)*choose(14,7)=399072960` labeled
   hidden completions, with intended uniform mass and unit weights. This is a
   Monte Carlo target description, not a theorem about finite PRNG frequencies.
   If policy-seed sampling changes as well, account for the joint proposal.
   This identity evaluates a fixed player; it does not by itself justify
   reweighting Walt's joint sample-bundle policy optimization.
3. **Change Walt's internal sampling.** This changes the player being measured and
   needs its own versioned comparison. The final per-move estimates and seeds are
   retained, but internal world lists/branch contributions and intermediate
   40-world opening action vectors are not all logged. Exact source plus seeds
   support targeted reconstruction; elapsed-time stops are not guaranteed to
   reproduce on another run. A dedicated trace must preserve those missing
   intermediate samples before claiming that one inner world caused a move flip.

Across current empirical trials both the hidden completion and policy seed vary.
To investigate a candidate explanation, cross them deliberately: keep one fixed
while varying the other, then compare paired continuations. Producer identities
and deadline outcomes remain part of that comparison. Scheme can describe a
candidate family after inspection; measure its failures and controls on fresh
hands rather than evaluating only the examples that suggested it. A future live
proposal may condition on the acting hand and public record, never actual unseen
cards. Later-trick proposals also need their own lawful conditional support and
probabilities; the opening uniform mass must not be reused indiscriminately.

This preserves a path from hindsight to cheaper informed guesses, while retaining
the original uniformly sampled evidence needed to find out whether it helped.
