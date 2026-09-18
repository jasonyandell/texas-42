# Sunshine notes: a fast player that understands partnership consequences

Discussion synthesis saved 2026-09-13. Source: Jason and Codex's conversation
after the relational-learning work was merged. **Exploratory ideas and proposed
methodology; not a new experiment, strength result, or implementation commitment.**

Follow-up on the same date: Jason authorized this next step, and the
[first completed learning cycle](campaigns/sunshine-partner-count-v1/RESULTS.md)
now implements and measures one bounded review. The text below preserves the
discussion as it stood before that experiment.

Jason's subsequent clarification: this particular count-offer competence is
expected to be uncommon, but an avoidable failure can be disproportionately
frustrating to a human partner. Measure success on the intended situations;
rarity in an ordinary arena is not a reason to abandon the skill. The
[targeted deployed-continuation replay](campaigns/sunshine-gym-replay-v1/RESULTS.md)
implements that follow-up and distinguishes useful corrections from teacher
labels that reverse with the actual future players.

The next platform step is now implemented: [continuation-selectable Scheme
recipes](../../walt/gym/SPECIFICATIONS.md) regenerate the same question under
named deployed players and reuse measured values when the query or filters
change. [Validation and model comparisons](campaigns/sunshine-recipes-v1/RESULTS.md)
make the continuation assumption a selectable experimental condition.

The next player step is now implemented and measured: the optional
[bounded partnership rollout](PARTNER-ROLLOUT.md) compares all legal moves
through completed L1 continuations. It improved 11 and harmed two development
decisions; on fresh sources it improved one and harmed one, with a larger
model-relative gain than loss. Mean investigation took about 0.11–0.12 seconds.
All 64 fresh mirrored make/set pairs tied. The
[results and counterexample](campaigns/sunshine-rollout-v1/RESULTS.md) preserve
the small-sample limit: a deadline-truncated comparison can favor the wrong
side of a close decision. This is an available experimental skill, not a new
default or a settled strength gain.

The next use step is now implemented: the [Mac Plunge table](PLUNGE.md) plays
the native L1 or L1 + partner check, saves original decision receipts, and
turns a flagged human-play observation into a continuation-selectable gym
comparison. The [bounded override study](campaigns/sunshine-playable-v1/RESULTS.md)
keeps the existing rule: larger required sampled gains traded fewer harms for
more missed help. Human play can now supply the next hypotheses and controls.

**2026-09-15 direction:** retain the [Sunshine harness](#sunshine-harness-a-scheme-workshop)
as a future workshop goal. The immediate focus returns to the playable game;
this records the workshop vision without starting its implementation.

## The priority

**2026-09-18 research lead:** preserve the samples that change a recommendation
and the ones that do not. The [Kiln sample-history instrument](../kiln/SAMPLE-HISTORY.md)
reconstructs all empirical prefixes and emits original-game explanation packets.
This supports later tests of more informative sample allocation or world proposals,
with retained probabilities/weights and fresh controls. It does not change the
player or claim that a threshold flip establishes a strategic mechanism.

A reasonable, fairly quick, partner-aware Texas 42 player on the Mac and phone. It must
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

The partnership gap is still open. The first bounded partnership layer is now
built; the broader architecture below remains a research direction.

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

## Original proposed first cycle (historical)

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

## Sunshine harness: a Scheme workshop

Added 2026-09-15. **Future product and research direction; not yet implemented.**
Jason wants a designed, interactive 3D analytics workshop for Walt: a place to
describe situations, express them with Scheme, discover examples, run scenes,
and investigate the game together with Codex. Rich visual design, animations,
and changes of perspective are part of the intended experience. The harness
supports the fast partner-aware player and later belief and dynamics studies.

The central object is a **saved, reproducible experiment**. Its coordinate or
search domain, Scheme expression, belief assumptions, named continuation
players, seeds, budgets, and implementation provenance connect every view and
result. Queries and filters reuse existing action values when their evaluation
conditions still match. A saved gallery remains attached to the specification
that generated it, so it can be filtered, extended, and regenerated.

Coordinated views should make the existing instruments accessible:

- **Table and history:** rotate into a seat's information perspective, scrub
  public history, and animate the flow of count, lead, and partnership resources.
  Clearly distinguish the actor's own/public information from an examiner's
  revealed hidden hands.
- **Compatible worlds:** hold the actor's hand and public history fixed while
  exploring different compatible hidden deals. Show which conclusions survive
  those changes and which depend on them.
- **Alternative continuations:** branch at a decision, compare plays side by
  side under named players, and locate where their consequences diverge.
- **Analytics and controls:** link candidate scores, coverage and uncertainty,
  timing, sampling progress, belief constraints, and player settings to the
  selected position. Use readable charts and tables alongside the 3D scene.

The first complete workflow to design is:

**Question → editable Scheme specification → matching positions → comparative
replay → saved gym exercise.**

For example: "Find positions where offering count to partner changes make/set
under our current player." The specification must make the continuation model
and search domain explicit. Scheme expresses the situation; measured or exact
action values under those conditions determine the outcome comparison. The
workshop should make exact versus sampled evidence, completed coverage, and
unresolved work visible. A discovered witness establishes possibility; frequency
and practical improvement require their own measurements.

Jobs should be bounded, monitorable, interruptible, and resumable, with durable
requests and completed results. A rich interface does not imply that opening
searches or arbitrary Scheme questions become cheap. Plunge observations and
original decision receipts should feed directly into this investigation loop.

**Codex at the same bench:** expose the workshop's operations through an MCP
adapter or another supported integration. Human and agent should inspect the
same experiment, edit its specification, run or pause investigations, select
views, and attach reproducible findings. Natural-language requests and agent
actions should remain visible in the workspace. An active agent can wait for
requests; waking an idle Codex session is a separate integration question.
Persistent job queues and a client-owned agent connection are options to
investigate, not capabilities already delivered by MCP alone.

When this direction resumes, design that complete workflow around one useful
partnership investigation. The immediate priority remains playing and improving
the game; the harness is retained here so the larger workshop vision survives
that pivot.

## Where the next sunshine cycle starts now

We can express a partnership concern, generate exercises with named
continuation players, and change the question while keeping applicable measured
values. The optional live rollout now uses completed deployed L1 continuations;
the original teacher-style review remains a separate experimental option.

The immediate next step is to play. Save mistakes and good partnership plays
with the original decision evidence. Separate failures of the public gate,
an inadequate sample, the modeled continuation, and the way an answer is used.
The human partner is itself a new continuation condition: an all-L1 census
does not automatically explain what happens with Jason at the table.

Use those examples to propose a small repair, test it on relevant positive and
withholding controls, and challenge it on new source-deal groups and different
named partners. Keep an affordable baseline ready. Hindsight should teach
which concerns deserve investigation, not just which tile happened to win once.
The priority remains a reasonable, fast, partner-aware player; neither a few
good demonstrations nor tied ordinary games settle its broader strength.

### Outcome-first Scheme discovery (2026-09-18)

The workshop also needs the reverse direction: **observed outcomes → candidate
Scheme → ablation/generalization → matching new worlds → measured continuation
outcomes**. Start from a specific world, remove details, and let independent
examples decide which abstractions deserve to survive. We choose mechanical
primitives and budgets; we need not prewrite every tactical pattern.

The first [bounded ownership-deletion probe](../kiln/threat-probe-v1/RESULTS.md)
runs this loop on saved actual-player games. It found mixed transfer evidence
and a simpler double-five ownership association worth challenging, with complete
negative/control results retained. The current grammar keeps physical tile
identities; relational lifting and learned temporal threat descriptions remain
future work. These queries inspect possible hidden worlds for analysis/belief
integration; they are not privileged inputs to the acting player. Validate
outcome prediction before claiming mechanism, and cheaper decisions before
claiming useful sample compression.

The [fresh follow-up](../kiln/mining-v2/RESULTS.md) completed ten unseen source
deals / 2,880 games. Two predeclared outcome associations replicated, including
a learned opponent-top-trump condition. A correction using its known uniform
opening prevalence reduced measured variance of fixed-player bid-success
estimates by 2.34% overall; a simpler double-five correction was inconclusive.
This is modest evidence of a useful shortcut for measurement, not a live-player
improvement. The frozen fit/evaluate/catalog tooling and retained failures are
the first mining-rig components. Next connect learned conditions to the gym's
paired alternative continuations, so unavoidable difficulty is separated from
decisions where helping partner can change the result.
