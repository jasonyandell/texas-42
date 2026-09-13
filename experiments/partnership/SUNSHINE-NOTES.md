# Sunshine notes: a fast player that understands partnership consequences

Discussion synthesis saved 2026-09-13. Source: Jason and Codex's conversation
after the relational-learning work was merged. **Exploratory ideas and proposed
methodology; not a new experiment, strength result, or implementation commitment.**

Follow-up on the same date: Jason authorized this next step, and the
[first completed learning cycle](campaigns/sunshine-partner-count-v1/RESULTS.md)
now implements and measures one bounded review. The text below preserves the
discussion as it stood before that experiment.

## The priority

A reasonable, fairly quick, partner-aware Texas 42 player on the Mac. It must
make a lawful choice within its computation budget. Guesses and occasional
mistakes are allowed. Spending the entire budget is optional.

Keep the existing bid-30, make/set objective for direct comparisons. Both-set
outcomes tie regardless of points. Auction optimization and predicting larger
bid amounts remain separate questions.

Partner awareness means representing how our own actions change what partner
can accomplish: capturing count, gaining or retaining a useful lead, preserving
resources, or setting up a later play. Giving a player the shared reward does
not guarantee that it represents or uses those relationships effectively.
Perfect information still has teamwork; the additional challenge here is making
a useful contribution without knowing exactly what partner can contribute.

## What was already built and measured

- A unified player with L1/L2 and belief-strategy options, plus a parallel,
  interruptible, resumable arena. The [default-player comparison](campaigns/default-partner-battery/RESULTS.md)
  found no established strength gain from L2 Partner or its void-aware option.
  L1 was the cheapest measured default in that panel.
- [Scheme/Fix](../../walt/scheme/README.md) expressions, finite transforms,
  belief operations, and executable policies. These express questions and
  programs; they do not imply that every query is cheap at an opening position.
- A [gym](../../walt/gym/README.md) with reproducible, composable exercise
  specifications and separate lawful evaluation under declared other-player
  policies. It can discover partnership-dependent bid-making decisions in
  bounded domains and retain the alternatives and their consequences.
- [Shared relational learning](campaigns/relational-learning-v1/RESULTS.md),
  teaching, provenance, and finite information-price experiments. The small
  shared actors remained weaker than sampled policy tables. A learned fallback
  had an uncertain positive signal in one endgame panel; another panel selected
  the empty baseline. The tested prices tightened no bounds.

The partnership gap is still open. The sunshine discussion proposed how to use
these instruments to investigate it; the layered player below is not yet built.

## Ideas worth retaining

### Approximate in ways that preserve useful consequences

Jason's phrase was **"vague in the right ways."** A model can omit substantial
detail while still predicting a relationship that helps choose a move. The
working hypothesis is that approximations should remain sensitive to partnership
consequences even when they are imprecise about individual hidden hands.

This is a hypothesis to test. It does not promise a universally compact policy,
lossless compression, or cheap recognition of every important decision. The
possibility of a worst-case hard problem does not itself answer whether useful
approximations work on the positions and time budgets we care about.

### Layer guesses and pointed checks

Keep a usable baseline move ready. A cheap, fallible detector can suggest a
concern or an opportunity and identify a rival play. A targeted Scheme query,
sampled investigation, or other bounded calculation can then investigate it.
For example: does this proposed lead create a count-capture opportunity for
partner, or is there a counterexample to the reasoning behind the favorite move?

An existential query may stop at a witness. Its operational outcomes must keep
**witness found**, **absence established**, and **unresolved within budget**
distinct. A witness establishes possibility, not frequency or improved make
probability. Both the interpretation of the answer and the choice to act on it
can be fallible and evaluated experimentally.

Deployment checks must derive their inputs from the player's own/public
information and any explicitly declared belief model. An examiner's access to
the actual hidden hands is not an input to the player. Existing examiner-only
predicates do not automatically become deployable actor predicates.

### Learn what to think about as well as what to play

Estimate where a mistake could be costly and whether additional investigation
is likely to improve the choice. That estimate need not be perfect. Learn which
concerns are worth raising, which checks help, and when to stop thinking early.
The deadline is a ceiling, not a target.

The resulting player earns its place by making better decisions at acceptable
elapsed cost. More elaborate reasoning or tighter bounds alone do not establish
stronger play. In particular, the completed price experiment showed that with
fixed continuation lowers, changing the common upper in its interval-cost rule
cannot change the ranking of candidate programs.

### Learn from hindsight without equating loss with error

"Should have taken that left turn at Albuquerque" can teach a better action or
a better habit of investigation. Ask what was overlooked, whether a different
question would have helped, and whether the extra computation was worthwhile.

A realized win or loss alone does not settle decision quality. In manageable
positions the gym can compare alternatives over compatible hidden hands under
a declared belief and other-player model. Keep that model-relative comparison
separate from one lucky or unlucky completed game.

### Accumulate skills around the existing player

An opportunity detector, a better continuation, a cheap relational program, or
an affordable exact calculation could each help. No one architecture was chosen
during the discussion. A useful route is to identify a recurring avoidable
mistake, try an affordable repair, and retain it only if fresh comparisons show
that it helps. This does not require replacing the entire player at once.

## The experimental method that emerged

The physics analogy was useful: the game rules are known, while useful predictive
relationships under hidden information and limited computation still need to be
discovered. A hypothesis should predict new measurements, with its conditions
of validity stated. Partner and opponent behavior are among those conditions.

For each investigation, record:

1. **Claim:** the relationship or improvement being proposed.
2. **Conditions:** positions, viewer information, hidden-hand distribution,
   declaration, bid, partner/opponent policies, and computation budget.
3. **Intervention:** what changes and what stays fixed in the comparison.
4. **Prediction and measurement:** the observable consequence, make/set effect,
   and elapsed computation it costs.
5. **Challenge:** counterexamples and untouched source-deal groups that could
   expose a failure or a narrower range of validity.

Scheme expresses hypotheses and situations. The gym supplies controlled cases.
The examiner supplies model-relative answers where tractable. The arena tests
whether the assembled player gains practical strength.

Discovery and prevalence measurement are separate jobs. Outcome-selected gym
cases can concentrate a phenomenon; they cannot by themselves establish how
often it occurs or how much a repair helps on ordinary deals. Distinct hidden
completions of the same visible position are useful matched experiments, not
independent source deals.

The same method can later support belief and dynamics studies: test how evidence
changes predictions and how beliefs and partnership opportunities evolve through
play. The partner-aware player remains the immediate application.

## Proposed next step, not yet executed

Use the existing player and gym to find one recurring family of avoidable
partnership mistakes. Explain what opportunity is lost and formulate a specific
question that could detect or investigate it without calling the target answer
inside the predicate. Test affordable ways to exploit that information.

Then compare the resulting player with the unchanged baseline on fresh matched
deals, recording both bid-making benefit and computation cost. Keep the selected
demonstrations separate from the untouched evaluation. A failure should help
separate a missing representation, an ineffective check, poor interpretation of
its answer, and an investigation that simply costs too much.

This is a proposed first learning cycle, not an assertion that the layered
architecture will win. It ties the next platform addition to an observable
partnership consequence while leaving room for alternative implementations.
