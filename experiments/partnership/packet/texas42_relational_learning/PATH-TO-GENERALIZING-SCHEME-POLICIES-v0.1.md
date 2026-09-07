# From sampled decision tables to generalizing Scheme policies

Version 0.1 — 2026-09-07  
Inspected repository: `jasonyandell/texas-42`, `main` at `08fad7264fb3986878de67e1b4106b0028192325`  
Status: repository-grounded design and finite mathematical checks; not a shipped learner or a native training result.

## Executive judgment

There is a concrete path, but pricing is not an automatic generalization mechanism.

**Learn a small shared lawful controller, using regret rather than exact teacher-action agreement as its training cost. Use information prices to bound the alternatives that controller may have missed. Evaluate both on genuinely new starting hands and trajectories.**

Keep three objects separate:

1. The executable actor: one `PolicyProgram`, legal and based on own/public information, including lawful controller memory.
2. The teacher/evaluator: full information-state beliefs, fixed-field lawful searches, candidate-policy values and action-indexed upper bounds.
3. The information-price program: may inspect hidden scenarios inside an optimistic evaluator, but must be correctly centered at the actor's full information state.

The actor need not reconstruct the full belief or reproduce an exact decision table. The evaluator must not confuse the actor's chosen abstraction with the player's actual information.

## 1. What the new main actually delivers

The current commit adds typed Scheme dynamics, belief pushforward, rigid-role transport, backward preimages, serializable controllers, persistent sampled search, and a complete-donor composition experiment. The report carefully distinguishes finite-sample exactness from playing strength and across-hand generalization. [R1–R5]

The reported opening results are:

| Frozen field | Training makes | Independent test makes |
|---|---:|---:|
| `hash-legal-v1`, 32 roots, 200 training worlds/root | 4,979/6,400 (77.8%) | 2,100/8,192 (25.6%) |
| Native L0-8, 12 roots, 32 training worlds/root | 335/384 (87.2%) | 182/768 (23.7%) |

Those figures are repository reports, not measurements rerun for this note. The opening report says persistence saved 52.9% and 46.4% of complete paired schedule search time, respectively. Complete-donor composition produced the same completed policies and did not repay its construction cost. This is useful exact-search reuse, not evidence of a new learned policy class. [R1]

The implementation makes the mechanism particularly clear:

- `Search` owns one immutable root, field, and ordered sample stream. `walk` chooses one focal action for each observed history and current sample subset. Hidden branches group sampled worlds by the field's observed action. [R6]
- `TablePolicy` is a map from full root-relative public action histories to physical dominoes. An absent history chooses the lowest legal tile. [R6]
- `policy_search::program::export` creates `exact_rules`, with `bindings: vec![]` and `rules: vec![]`. The relational-policy facilities are not used by the constructor. [R7]
- The runtime checks exact rules before relational rules, then uses the mandatory `lowest-legal` fallback. Relational guards use `evaluate_viewer`, not the realized hidden world. [R8]

Complete-donor composition intentionally preserves the full finite training optimum for its stated Boolean/fixed-field scope. Requiring a new generalizer to reproduce that same sampled table would defeat much of the intended regularization. [R4]

**Retain persistence as a teacher optimization. Do not make exact table parity a gate for the learned actor.**

## 2. Diagnose two problems, not one

### 2.1 Within-hand generalization

At a fixed root, the empirical posterior can rapidly shrink to one or a few sampled worlds after public observations. A lawful policy can then exploit sample-specific correlations between an observation and a hidden completion. On the real posterior that observation can still be compatible with many other completions.

This need not be strategy fusion: different observed histories really are different information states. It is overconfident inference under a sparse empirical distribution.

A second mechanism is an explicit coverage failure: the saved program sees an unrecorded history and falls back. The opening report establishes substantial fitting but does not quantify how much of the gap is attributable to lookup misses versus poor decisions at matched keys. [R1, R6, R8]

### 2.2 Across-hand generalization

The present campaign constructs a different root-scoped policy for each starting hand. Repeating an algorithm on diverse hands is not training shared parameters across them. [R1, R2]

A single file containing every table would not fix this. The actor needs shared relational tests and selectors whose meaning applies on different hands.

Do not confuse this with hiding the player's own hand. The player is entitled to condition on that hand. Without a size restriction, a single arbitrary policy over all observations can simply contain every hand-specific policy. Generalization is a statistical and representation constraint, not a new rule of 42.

### 2.3 Mandatory first diagnostic

Add decision provenance to replay: exact-key hit, relational-rule hit (with rule identifier), or final fallback. Record controller state before/after and whether the contract is already decided. For unresolved play, report:

- probability of a first fallback by focal decision depth;
- matched-key rate and sampled posterior occupancy/weight concentration;
- first-action value under best lawful continuation, separately from actual full-program value;
- contribution of root units rather than treating all trajectories from one hand as independent hands.

Do not infer the causal effect of fallback merely from outcomes conditional on a miss. Bad situations can cause both misses and losses. Compare frozen identical exact tables with different fallback policies on paired fresh worlds, and replay the entire changed policy. A separate reoptimization arm answers a different question.

The exact gym already distinguishes first-action regret from the complete-policy gap: at 64 training worlds its report gives 0.436 and 1.008 percentage points respectively. Preserve that distinction. [R1]

## 3. The actor: a bounded shared relational program

Use the existing `PolicyProgram` as the deployment representation. Search a bounded grammar of ordered guards and action selectors. Start stateless; introduce modes and bindings only when there is evidence that memory helps.

For the first clean experiment:

- Require `exact_rules` to be empty.
- Exclude literal complete hands, histories, source seeds, and root identities.
- Prefer relative chairs, declaration-relative contexts, count values, and mechanically defined role relationships.
- Set an explicit code/AST budget; count thresholds, rule ordering, modes, binding definitions, and any learned predicate implementation.
- Keep the final legal fallback, but include meaningful general rules above it.

This is a restriction on the learned constructor, not a proposal to delete exact-rule support from the language. A hybrid exact-patch actor can be a separate arm with its patch size and coverage charged honestly.

A rule means a selector `f(I)` that can return different physical dominoes on different hands. Sharing does NOT mean forcing identical literal domino choices across roots.

### A current-syntax candidate, not a claimed tactic

The companion `partner-count-candidate.scheme` emits a ten-point tile, then a five-point tile, when partner is currently winning the partial trick; otherwise it uses the required legal fallback. It uses existing Viewer predicates and contains no hidden-hand query or literal domino.

This is only an example of a hypothesis the constructor can test and qualify. Partner currently winning does not imply partner will win the completed trick. The candidate has not been compiled or benchmarked in Rust here.

### Do not demand a strategically sufficient state quotient

A compact actor can ignore information without the environment, belief updater, or verifier discarding it. The actor may choose the same *rule* in multiple distinct states because their action consequences are close enough. This does not require their transitions, hidden support, future response sets, or optimal values to be identical.

This is the decision-relative compression perspective from the unification, applied to a deployable program instead of a universal world quotient.

## 4. Learn decision costs, not the teacher's arbitrary tie breaking

Fix rules, focal player, root posterior, utility in `[0,1]`, and all other seats' behavioral field. Fold their intervening moves into transitions between focal decisions. Let `I` retain the full lawful information history; persistent field states must be included where necessary.

Define:

\[
Q^*(I,a)=\text{best lawful continuation value after action }a,
\qquad V^*(I)=\max_a Q^*(I,a).
\]

The true cost of an action is

\[
c^*(I,a)=V^*(I)-Q^*(I,a).
\]

For any complete lawful program `P`, with its actual induced trajectory distribution,

\[
\boxed{V^*(I_0)-V^P(I_0)
=\mathbb E_P\sum_t c^*(I_t,P(I_t)).}
\tag{1}
\]

Here `P(I_t)` includes any controller memory determined by that history. No common continuation-policy witness is being silently substituted for the executed program.

**Proof.** At each focal step, `Q*(I_t,a_t)` is the conditional expectation of `V*(I_{t+1})` after imposing that action. Sum `V*(I_t)-Q*(I_t,a_t)` and take expectations under `P`. The intermediate values telescope, and the terminal optimal value equals the observed terminal utility. ∎

Equation (1) is the finite performance-difference identity specialized to our target, not a novel general result. It identifies the correct reduction: cost-sensitive sequential policy learning. Related work includes DAgger for learner-induced state distributions, AggreVaTe for cost-to-go learning, VIPER for value-guided policy extraction, and programmatic-policy search. None automatically provides the missing 42-specific learner or guarantees for its heuristic search. [E1–E4]

### Certified costs from intervals

Suppose the teacher maintains valid bounds for every relevant legal action:

\[
L(I,a)\le Q^*(I,a)\le U(I,a).
\]

Bounds may be clipped into `[0,1]`. Then

\[
\boxed{\bar c(I,a)=\min\{1,\max_b U(I,b)-L(I,a)\}}
\tag{2}
\]

satisfies `0 <= c*(I,a) <= cbar(I,a)`. Consequently,

\[
\boxed{V^*(I_0)-V^P(I_0)
\le\mathbb E_P\sum_t\bar c(I_t,P(I_t)).}
\tag{3}
\]

A lower can come from an actual lawful continuation; an upper can come from prices, gluing, residual bounds, or trivial bounded utility. A sampled rollout mean is not an exact lower bound: attach correct statistical authority or retain the empirical label.

The local witnesses used to obtain different lowers need not stitch into the final actor. They are only bounding `Q*`; the proof of (3) evaluates the final actor through its own induced actions and observations.

For each state, maintain two different sets:

\[
A_{\mathrm{safe},\epsilon}(I)=\{a:\bar c(I,a)\le\epsilon\},
\]

\[
A_{\mathrm{not\ ruled\ out},\epsilon}(I)=
\{a:\max_b L(I,b)-U(I,a)\le\epsilon\}.
\]

The first certifies low regret. The second merely says the evidence has not excluded it. Do not conflate them.

If no action is certified safe, that can be an oracle-width problem, not a missing feature. Tighten decisive bounds before expanding the actor to fit uncertain distinctions.

### Why labels are the wrong invariant

Two states can have action values `(0.90,0.89)` and `(0.89,0.90)`. Exact teacher labels disagree. A constant selector nevertheless loses at most 0.01 at either state. Requiring label agreement forces an unnecessary split.

Conversely, a rare 0.50-regret error can matter much more than numerous harmless tie disagreements. Train on consequence gaps and evaluate whole trajectories.

## 5. An implementable learning loop

This loop can begin without useful information prices. Exact small endgames and lawful-policy rollouts supply the first teacher data; pricing is an optional oracle accelerator to test separately.

1. **Freeze the target and data split.** Keep source deals, starting-hand groups, field identity, contracts, and declarations explicit. A final test group is untouched by rule/price selection.
2. **Propose a small program.** Use a typed bounded library of current Viewer guards and action selectors. Beam search or enumerative search over ordered lists is a reasonable first constructor; no claim of globally optimal program synthesis is needed.
3. **Roll out that program on new discovery worlds.** Retain the information states it actually reaches, including modes and binding state.
4. **Ask an information-consistent teacher about costly decisions.** Reconstruct the posterior from the learner's own hand and public history under the declared model. Draw additional compatible worlds or enumerate the fiber. Do not give the teacher only the realized rollout world, and do not label its perfect-information action as a lawful answer.
5. **Fit a short program to action costs.** A surrogate is weighted empirical decision cost plus code length and measured execution cost. Share the same rule parameters across hands. Direct full-program rollout evaluation decides promotion.
6. **Refine using the right counterexamples.** Add a visible relation or mode only when it resolves a material, supported error. Add fresh discovery roots and repeat.
7. **Freeze and audit.** Evaluate the selected program on independent whole-deal/own-hand groups, plus exact fibers on the small diagnostic panel.

Training only on the teacher's trajectories is insufficient: the student changes the histories it sees. This is the reason for on-policy discovery/aggregation, not a promise that a bounded heuristic beam search inherits a no-regret theorem. [E1, E2]

Use the current persistent solver as a teacher whose exact memo remains root/field/sample scoped. Reusing those memos across new roots, new posterior weights, different controller restrictions, or changed fields without a preservation proof is invalid. Cache compiled predicates and feature evaluations separately under their complete semantic inputs. [R4, R6]

A rollout continuation of the current student can supply a feasible lower for an alternative first action, provided its controller state is advanced consistently. It is not an upper on optimal continuation and is not automatically a replacement for `Q*`.

### Counterexample taxonomy

| Situation | Appropriate response |
|---|---|
| Two hidden worlds at the SAME full lawful information state require incompatible actions | Improve the belief/price/conflict analysis; do not add the hidden discriminator to the actor. |
| Different lawful information states are merged by a guard and need materially different selectors | Add a lawful relational distinction or memory, if it pays for its complexity. |
| Bounds overlap widely or sample labels disagree without a decisive gap | Spend more teacher work or collect fresh evidence before splitting. |
| An unseen history reaches the final fallback | Improve reusable coverage and on-policy data; measure full-policy effects. |

Every adversarial or outcome-selected counterexample is a discovery example, not an unbiased estimate of the deployment distribution.

## 6. Where information prices enter

For an examiner-side Scheme event `phi_j(I,omega)`, use

\[
p_\theta(I,a,\omega)=\sum_j\theta_j(I,a)
\left[\phi_j(I,\omega)-\mu_j(I)\right],
\qquad
\mu_j(I)=\mathbb E_{\beta_I}\phi_j(I,\omega).
\tag{4}
\]

The event may use `Access::World` inside the evaluator. Coefficients and centers used for a lawful action must not depend on unavailable realized information. With correct conditional centers, every lawful policy has zero expected accumulated price.

A sound relaxed continuation maximum of terminal utility minus accumulated prices supplies an upper on the lawful optimum. This is the information-relaxation duality already used in the earlier work. [P1, P2, E5]

**What can generalize across hands:** the feature vocabulary, relational coefficient rule, and small numerical parameter vector. **What must remain information-specific:** the posterior and the conditional centers.

The actor and price program need not use the same vocabulary. The earlier cross-hand coholding price was useful precisely because it described hidden relations; that does not authorize an executable actor to inspect those relations. A belief-probability actor would need an explicit lawful belief-summary input, not a relaxation of the World-access prohibition.

### Price useful alternatives, not already-fixed choices

For an action-conditioned root upper, the root action is fixed. Subtracting a centered root-only charge integrates to zero; it does not improve that upper. Price the future choices at which the relaxed evaluator is given forbidden information.

A price fitted over a finite library of candidate policies certifies only that library unless an additional upper covers undiscovered continuations. A good feasible relaxed path is a lower on the relaxed maximum, not a valid upper on it. Preserve both inner-optimization and outer-expectation obligations. [P1, P2]

### A zero empirical dual gap does not imply generalization

If an empirical posterior has one sampled world per observed history, even its exact lawful optimum can exploit that sample-specific certainty. A perfect information price then certifies the *empirical* optimum, not the real one.

The verifier includes an exact one-choice example. A public context has a reusable visible bit; the hidden correct answer matches it with probability 3/4. A public nuisance identifier permits a sampled table to memorize every training outcome. On new public contexts, that table's fixed fallback achieves 1/2 while the short shared rule achieves 3/4. The table's empirical optimum and perfect empirical dual both equal 1.

This is a demonstration of a logical limit, not a Texas 42 benchmark. It proves that pricing cannot replace statistical validation or an inductive restriction on policy size.

### Do not center over an actor's coarsened descriptor

The player may still legally distinguish information states that its current program treats similarly. Prices must have zero conditional mean at the full lawful information state, not merely when averaged over a learned cell.

A minimal exact counterexample: public `X` is equally likely 0 or 1, and reward is `1{A=X}`. The lawful optimum is 1. Centering each action's reward over both public states gives `p_A = 1{A=X}-1/2`. Every adjusted action reward is 1/2, so the purported relaxed upper is 1/2—below the lawful optimum. Correctly conditioning on `X` instead makes these public-only prices zero and restores the valid upper of 1.

**Share the price formula. Do not silently share its center across observably different situations.**

### Counting can make those centers practical

When a price event is a small disjoint union or suitably corrected combination of products of hand-local constraints, its expectation can use the previous exact-cover contraction route. Fixed capacities justify the saturated covering transform in its proved scope. Shared syntax does not guarantee such a decomposition; query overlap and existential witness multiplicity still require set semantics. [P1, P2]

Always measure the cost of computing centers, evaluating prices, and solving the modified inner problem. An expressive price can destroy useful solver structure. Economic success, not expressive possibility alone, is the test.

## 7. What dynamics and persistent modes contribute

The new APIs make it possible to describe a *plan across observations*, not only a single move:

- `step_frame` and `step_belief` give the legality-aware forward evolution.
- `transport_answers` carries object identities without pretending their old predicates remain true.
- `compare_answers` distinguishes persistence, extinction, and birth.
- `anchor_back` localizes the predecessors of a selected later event. It is examiner hindsight, not early player knowledge. [R5]

These can support a discovery question such as: which known resource must survive until the public observation that distinguishes two plans? That connects to the earlier information-deadline interpretation. Use it to propose reusable guarded plans and focused tests, not to introduce oracle-only features into live play.

### Current controller limits matter

Rigid bindings initialize once and require exactly one answer. They are not a general run-time variable-binding mechanism. A general actor cannot assume every starting hand has a unique master. Modes change only after a successful legal relational selection; exact actions leave the mode unchanged. Current guards do not receive the rigid-binding map as query parameters. [R3, R8]

Begin with one mode and no bindings. A later two-mode pilot can use always-defined initializers or an explicit optional-binding extension. Arbitrary run-time rebinding or reacting to every opponent event would be new functionality, not already implemented by the existence of a controller struct.

Do not cold-start a controller at a later sampled frame and call that evaluation of its whole-game behavior. Supply its reachable memory from the opening, or explicitly define and evaluate a continuation-initialization protocol.

If a stateful policy becomes part of the opponent field, retain its hidden controller state in the latent model. `step_belief` currently merges physical remaining hands, so a physical marginal alone need not support correct stateful-field updates. [R5]

## 8. A small but important observation-interface extension

The present `PolicyInput` contains full public history, banked score, bid, and declaring team. The exact key uses them. But `CompiledPolicy::choose` evaluates each relational guard with `guard.evaluate_viewer(input.frame, budget)`. Standard Viewer predicates receive `Frame`, not those extra policy fields. [R8, R9]

Therefore a richer constructor alone cannot express every legal score/history-dependent distinction through the current guard surface. This is an interface boundary, not evidence that such distinctions must all be added before a first pilot.

Proposed extension: a versioned read-only `PolicyObservation`/policy-guard context exposing only legally known information. Candidate mechanical predicates include contract points remaining, declaring/defending role, public void evidence, own remaining holdings, and bounded actor-attributed history facts. Add only what demonstrated counterexamples require.

Do not capture per-example score or labels in a callback registered as Viewer. The extension contract forbids undeclared information. No predicate may call the target solver or inspect its response label. [R9]

The standard `void(c,q)` predicate is World-access, even though some voids are publicly established. Add a separately named public-evidence predicate rather than weakening the existing access classification.

A future posterior feature—such as the probability partner holds a named class—would need an explicit model-tagged belief-summary interface with exact/statistical authority and charged work. It is not present in current actor guards.

## 9. Why a code budget and a root split belong in the mathematics

Let `R` independent root bundles be drawn from the declared deployment distribution. Each bundle has a public starting hand/root and conditionally generated evaluation worlds. Let `Y_i(P)` be a fixed program's average utility within root bundle `i`, in `[0,1]`, and let `J(P)=E[Y_i(P)]`.

For a fixed, countable, prefix-coded policy grammar with code length `ell(P)` bits and Kraft sum at most one, Hoeffding plus a union bound gives, with probability at least `1-delta`, simultaneously for every program:

\[
\boxed{J(P)\ge\widehat J_R(P)
-\sqrt{\frac{\ell(P)\ln2+\ln(1/\delta)}{2R}}.}
\tag{5}
\]

**Proof.** For a fixed `P`, the lower-tail failure probability at radius in (5) is at most `delta * 2^{-ell(P)}`. Sum over the fixed code and apply Kraft's inequality. ∎

This elementary bound is usually conservative. It is not a recommendation to estimate program size from raw source bytes and declare a useful numerical guarantee. A valid code must include learned predicate definitions/parameters; the grammar and any prior weights must not be retrospectively selected from the test data. Other statistical analyses can be tighter.

Its conceptual message is precise: many worlds beneath one hand improve that hand's conditional estimate, but they are not many independent hands for cross-hand generalization. The number of independent root/deal groups matters. Program size is a statistical complexity term, not just an aesthetic preference.

Training distribution aggregation, adversarial counterexamples, and adaptively queried teachers do not make their rows i.i.d. Use untouched independently generated root bundles for simple final guarantees. When several recorded roots derive from one original deal, group them together at splitting and uncertainty estimation.

No arbitrary pip permutation is a valid augmentation by default. Counts, rank comparisons, declaration semantics, and even the frozen field can break a proposed symmetry. Only use transformations preserving the entire tested target, or label a transformed problem as a new task. This includes the hash field's dependence on identities/history. [R6, R9]

## 10. Minimal native implementation slices

### Slice A — Explain the current gap

Add `choose_traced` or an equivalent audit wrapper to report decision provenance and mode transitions. Extend campaign metrics with first fallback before contract resolution, full-policy score, and grouped root summaries. Perform matched frozen-table fallback swaps. Do not change search semantics yet.

### Slice B — One shared relational actor, no prices required

Build a bounded grammar constructor that emits the existing `PolicyProgram` with empty exact rules. Pool training roots across starting hands. Start in replay-certified small endgames under one frozen field/contract/declaration profile. Retain the current exact search as a teacher, and use all-action values or intervals rather than single chosen labels.

A practical initial grammar budget is a handful of ordered clauses in one mode with no bindings; that number is an experimental choice, not a theorem. Use named, versioned mechanical feature primitives and actual run-time work measurements. Select program complexity on development roots.

### Slice C — On-policy regret-guided discovery

Replay the shared actor on new discovery deals. Reconstruct information-consistent teacher roots, identify costly choices, and aggregate them. Beam-search additions/deletions/reorderings of clauses. A mode extension should specify its transitions and be assessed by complete rollouts, not by independently labeled rows alone.

### Slice D — Price-assisted teaching

Add a separate examiner-side price consumer. Keep its feature basis/coefficients shared but recenter at each full information state. Compare price-assisted action bounds against the same-budget unpriced teacher. Count exact moment work, inner optimization, discovery, and verification overhead. Refuse or explicitly mark bounds lacking valid centers, inner uppers, or outer authority.

### Slice E — Deployment as a complete baseline and witness

The result is one frozen compiled actor with a lawful fallback and bounded operational behavior. It may play directly or supply a continuation/witness to Walt. Search can improve decisions around it. A saved-program benchmark and live budgeted replanning are different modes; report both rather than interpreting one as the other.

Do not change the opponent field silently when inserting the learned player into a training or evaluation role. Freeze field identity per campaign and invalidate incompatible teacher caches.

## 11. Decisive experimental panel

Use at least these distinguishable arms:

| Arm | Question it answers |
|---|---|
| Existing exact sampled table + existing fallback | Current baseline |
| Same frozen table + shared relational fallback | How much is unsupported-history behavior responsible? |
| One shared relational program, no exact patches | Is there actual across-hand rule learning? |
| Same actor class + price-assisted teacher | Do prices improve quality or reduce total learning/certification work? |

The shared actor must have one program digest across every test hand, no training-deal lookup, and its actual continuation must be replayed. Freeze all selection before the final root-group split is evaluated.

Begin with the existing exact gym and new roots generated under its declared target, splitting by source deal; it is a diagnostic, not a sufficient opening benchmark. Move to larger hidden supports and opening roots after the small-domain reduction is validated. Native L0 and the gym's L1-partner/L0-opponent field are separate target panels. The hash field is useful for cheap mechanism tests, not the sole test of relational strategic generalization. [R1, R2]

Report equal-root-weight full-program make rate; first-action regret separately; whole-policy regret where exact truth exists; no-match rate; code size; inference work; total construction/teacher/price work; failures and work-cap behavior; and source-deal-group uncertainty. At population scale a training-vs-test gap is diagnostic, not an estimator of optimal regret.

Success does not require preserving the sampled optimum. Lower training value with higher unseen full-policy value, a genuinely shared short program, and acceptable execution cost is a meaningful success. A low price gap on the training sample alone is not.

## 12. What was and was not verified here

The GitHub connector was used to inspect the pinned main commit, source files, documentation, and focused policy tests. A local clone could not resolve the GitHub host and no Cargo executable was found on PATH, so no native implementation or campaign was run in this session.

The independent standard-library `verify.py` checks exact rational mathematics:

- 128 finite three-decision stochastic games and 64 policies each: **8,192 policy checks**, each verifying the performance-difference identity and the action-interval regret certificate.
- A sparse empirical-model example with perfect empirical dual recovery but poor table generalization.
- A public-information pooling counterexample that invalidates coarse conditional centering.
- An example where different exact teacher labels do not justify a more complex low-regret actor.

All checks passed. They support the derivations and guard against specific logical mistakes. They are not evidence that a particular relational constructor generalizes in Texas 42, and do not replace the proposed native experiment.

## 13. Final research hypothesis

The useful shared object may be neither a universal compressed game nor an exact policy table. It may be:

> **A small lawful program plus a small family of information-price and regret certificates explaining which distinctions that program can afford to ignore.**

The new runtime supplies the executable half. The next missing piece is a constructor whose optimization pressure rewards reusable, low-regret decisions rather than exact reproduction of sample-specific choices.

## Source register

All repository sources below are pinned to `08fad7264fb3986878de67e1b4106b0028192325`.

- [R1] Campaign results: https://github.com/jasonyandell/texas-42/blob/08fad7264fb3986878de67e1b4106b0028192325/experiments/partnership/campaigns/policy-synthesis-v1/RESULTS.md
- [R2] Campaign definition: https://github.com/jasonyandell/texas-42/blob/08fad7264fb3986878de67e1b4106b0028192325/experiments/partnership/POLICY-SYNTHESIS.md
- [R3] Policy semantics: https://github.com/jasonyandell/texas-42/blob/08fad7264fb3986878de67e1b4106b0028192325/walt/scheme/POLICIES.md
- [R4] Donor composition: https://github.com/jasonyandell/texas-42/blob/08fad7264fb3986878de67e1b4106b0028192325/walt/scheme/COMPOSITION.md
- [R5] Dynamics: https://github.com/jasonyandell/texas-42/blob/08fad7264fb3986878de67e1b4106b0028192325/walt/scheme/DYNAMICS.md
- [R6] Search implementation: https://github.com/jasonyandell/texas-42/blob/08fad7264fb3986878de67e1b4106b0028192325/walt/walt/src/policy_search.rs
- [R7] Export and replay: https://github.com/jasonyandell/texas-42/blob/08fad7264fb3986878de67e1b4106b0028192325/walt/walt/src/policy_search/program.rs
- [R8] Controller implementation: https://github.com/jasonyandell/texas-42/blob/08fad7264fb3986878de67e1b4106b0028192325/walt/walt/src/scheme/policy.rs
- [R9] Scheme registry and boundaries: https://github.com/jasonyandell/texas-42/blob/08fad7264fb3986878de67e1b4106b0028192325/walt/scheme/README.md
- [R10] Focused policy tests: https://github.com/jasonyandell/texas-42/blob/08fad7264fb3986878de67e1b4106b0028192325/walt/walt/tests/scheme_policy.rs
- [P1] Conversation artifact, `SCHEME-INFORMATION-PRICES-v0.1.md`, especially sections 3, 6, 7, and 8.
- [P2] Conversation artifact, `TEXAS42-IMPROVISATION-v0.1.md`, especially sections 7–13.
- [P3] Conversation artifact, `TEXAS42-UNIFIED-REVIEW-v0.1.md`.
- [E1] Ross, Gordon, Bagnell (2011), A Reduction of Imitation Learning and Structured Prediction to No-Regret Online Learning: https://proceedings.mlr.press/v15/ross11a.html
- [E2] Ross, Bagnell (2014), Reinforcement and Imitation Learning via Interactive No-Regret Learning: https://arxiv.org/abs/1406.5979
- [E3] Bastani, Pu, Solar-Lezama (2018), Verifiable Reinforcement Learning via Policy Extraction: https://arxiv.org/abs/1805.08328
- [E4] Verma et al. (2018), Programmatically Interpretable Reinforcement Learning: https://proceedings.mlr.press/v80/verma18a.html
- [E5] Brown, Smith, Sun, Information Relaxations and Duality in Stochastic Dynamic Programs: https://optimization-online.org/2008/03/1927/
