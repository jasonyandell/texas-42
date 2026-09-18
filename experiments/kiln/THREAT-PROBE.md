# Outcome-first Scheme discovery: first probe

Exploratory, 2026-09-18. The question is whether ownership facts retained from
observed losing worlds can describe other worlds with elevated failure rates.
No player, proposal distribution, production queue, or bidding rule changes.

## Frozen protocol (before inspecting results)

- Use exactly trials 0..7 of all 900 hand/declaration cells. This avoids the
  outcome-dependent 8/40-game screening. There are 7,200 played games, but only
  800 hidden completions: declarations share both completion and policy seed.
- Sort the 25 original deal seeds. Discovery uses the first 15 deals, trials
  0..3. Same-hand validation uses their trials 4..7. Transfer uses all eight
  trials of the remaining 10 deals. Never select or revise on either validation
  set. All four bidder hands from a source deal remain in the same deal group.
- Target: failure to make 30 under the actual saved bid30 player. Final scores,
  policies, and seeds are observations, not optimal-play or causal labels.
- Select up to 64 distinct losing discovery worlds by a fixed hash ordering.
  A world's initial description is its 21 hidden tile ownership facts, with
  chairs expressed relative to the bidder. Retain every one-, two-, and
  three-fact sub-conjunction. This is bounded deletion generalization, not a
  library of tactical patterns. Identical retained clauses are deduplicated.
- Physical tile identities are retained in this first grammar. No role lifting,
  beats/void predicates, negation, disjunction, declaration-specific clauses, or
  temporal predicates are searched. A null result is confined to this grammar.
- Fit only discovery labels. At least 36 matching games are required. Rank by
  positive squared within-cell failure excess divided by matching games;
  matches must span at least six discovery source deals;
  rational arithmetic, then fewer literals, then canonical order. Each cell
  contributes its own baseline failure rate, removing pure hand/declaration
  difficulty. Keep eight candidates, suppressing coverage Jaccard >= 4/5.
- Report both reserved sets in full, including reversals and nonmatches.
  The primary candidate is discovery rank 1, not the best validation result.
  Validation reports matched failure rates and matched-mixture cell baselines,
  plus exact-count excess. This is an association audit, not a deployable risk
  predictor (the assessment baseline uses each assessment cell's labels).
- A 999-draw permutation check shuffles trial labels within bidder hand, using
  the SAME permutation across declarations. It retains declaration dependence
  and cell difficulty. Report a one-sided Monte Carlo permutation p-value for
  the primary candidate and max-statistic adjusted values for all eight. No
  adaptive stopping or retuning after seeing validation.
- Emit executable Scheme sources and original seed descriptions. Check every
  published query's membership on every input world through the actual Rust
  Scheme runtime. Save source receipts, identities, frozen rows, split, search
  counts, and all selected results outside the worktree with a compact tracked
  report. The production database is opened read-only.

These are candidate **outcome associations**, not established threat mechanisms.
Both hidden completion and policy seed vary. A surviving association needs fresh
data and controlled continuations to test mechanism and usefulness. Hidden-world
queries are analyst/belief instruments, never direct executable player guards.
This probe does not yet ask whether a learned proposal reduces Walt's sample cost.

The motivating interpretation is approximate, decision-relevant regularity.
Successful small-sample play does not by itself prove an exact compressible world
representation; this experiment measures one possible useful regularity directly.
