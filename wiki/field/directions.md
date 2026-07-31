# Directions — the map stated after first contact

[Field Home](Home.md) · owns: the direction-setting stated 2026-07-30, captured
so it is neither lost nor mistaken for a plan. **Ideas-tier capture** in the
sense of [ideas](../ideas.md): nothing here is a result, an expectation, or a
schedule; a direction leaves this page only by promotion to a brief or a
dispatch, where it earns its own tier from scratch.

## The frame

**rob is not the goal.** rob was the first idea that came to mind to exploit the
new mathematics — the most convenient shape, and a glorious one, but the
mathematics (exact fibers, canonical quotients, losslessness) is the asset.
"There's gold just laying on the ground in every direction; rob is just what we
picked up first." The long-term value inside rob is the **exact solver**.

## sigma, precisely

sigma is a **first-pass fast, deterministic, pure evaluator**. That is its whole
role. Its current use in rob's first few plays — jammed in so full games run
without intractable memory — is a convenience stand-in, **not** a role it will
keep, and explicitly **not worth optimization effort**. The first plays will be
done differently ([first-contact](first-contact.md) localizes the full-hand
deficit exactly there).

sigma-the-evaluator *is* worth investing in. The required properties (fast,
deterministic, pure) do not force simplicity: it can see trick context
(leading/following, trick locked, count on the table), partner vs opponent,
revealed voids from failed follows — all constant-time over information a
rollout already holds. A tiny quantized-integer net (NNUE-shape) fits the
no-floats discipline natively and still cranks positions like wildfire. Every
enrichment is measurable against the first-contact baselines: exact-window
agreement rates (repo-side, cheapest), the mid-hand rig with the takeover knob
walked earlier, and the full-hand −3.65pp benchmark as the endpoint.

## Four shapes beyond rob

1. **World selection.** Trying every world is silly; some matter, most don't.
   Choosing *which* must be done very carefully — selection coupled to solver
   output can bias the argmax — which is why it has not been attacked yet. Done
   right (exactness-preserving pruning by bounds over world-*classes*, or
   honestly-weighted sampling), it opens whole new classes of search. The
   natural unit is the fiber equivalence class, not the raw world
   ([idea-hierarchical-fibers](../idea-hierarchical-fibers.md) is the standing
   machinery candidate; its rung-3 decomposable bounds are the feasibility
   probe).
2. **Oracle swap inside E[Q].** The champion's construction is oracle-agnostic:
   it just needs *something* that evaluates positions. Distill something strong
   — e.g. exact info-set solutions from rob's exact window — and drop it in
   place of the perfect-information oracle. This attacks strategy fusion at its
   source while inheriting the champion's speed and harness.
3. **The plan solver walked home.** A rob-style path solver that argmaxes over
   *plans*, distilled from the earliest practical point, walked backward trick
   by trick to trick 0 — and then bidding. Subsumes the others: distillation
   needs the evaluator, the frontier solve needs tractable fibers, and each
   distilled stage is the oracle the next stage queries. The mid-hand rig's
   takeover knob is the empirical frontier-finder.
4. **Beliefs, then partnership dynamics.** Everything above is still the
   belief-free slice — dropped-30 deleted the auction precisely to make the
   players commensurable. Bidding, inference from bids and signals, and
   partnership convention are **where the real game gets played**; the ladder
   above is the construction of a player worth having beliefs about. (The
   foundation already proves support ≠ belief —
   [belief-vs-support](../belief-vs-support.md).)

## The north star

The dream, stated as a dream: a human-reasoning-like player. It looks at the
dominoes and goes *hmm*; **scenarios occur to it** and it evaluates them; it
plays them out; it looks back — *what could I have done better?* — and across
many games it notices that X tends to occur, that Y is a bit better, and
updates. All as **one thing**. No known method delivers the unified whole, and
this page does not pretend otherwise.

But the verbs are not mystical, and the new mathematics is the first vocabulary
in which the crucial one is even well-formed: a scenario that "occurs to" a
player is not a raw deal — it is an equivalence class of worlds, a fiber
statement. Scenario salience and careful world selection (shape 1) are the same
question. Evaluate = sigma's real role; play it out = rollout/solve; look back =
the counterfactual and paired-mirror rigs; update across games = learned priors
— beliefs (shape 4). The dream is not a work item. It is the direction the
compass points between work items.
