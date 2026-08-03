[Home](Home.md) · owns: the seat-level context frame (the player's maintained state) · Sources: conversation-tier (Jason + Claude, 2026-08-01→03); builds on [idea-retrograde-rank](idea-retrograde-rank.md); informal capture [`exchange/informal/2026-08-03-domino-constellations-theory.md`](../exchange/informal/2026-08-03-domino-constellations-theory.md) (unadjudicated)

# Idea: seat-level context — the player's state as a fold over history

**EXPLORATORY / IDEA TIER.** Nothing on this page is adjudicated, kernel-proved, or receipt-backed. It records a framing developed in conversation. The central question (§2) is **deliberately unresolved at Jason's request** — do not treat either reading as settled.

## 1. The reframe: neither pip-shaped nor decl-shaped

42's fundamental theory is not about pips and not organized around declarations. Pips are nails — a true and indispensable *representation* (the realization/witness layer computes with them), demoted from *the point*. Trump is a special suit: declared early, it shapes the hand, but it is absorbed into standings at declaration and is not a parameter of the ongoing dynamics. Consistent whispers from adjudicated territory (cited as motivation only, no promotion implied): the k=1 census separates all values with a key that pools all nine declarations (zero collisions), and C1 factors suffix minimax through the constellation. What is fundamental: the relationships among dominoes — their standings now, and how those standings morph through past and future.

Field evidence of the native factorization: experienced players, told "we evaluate by the relationships between dominoes, not the pips," respond the way one responds to being told a stop sign is red — obviously. They then immediately speak strategy fluently in the frame (finesse patterns keyed to relational situations). Obvious-to-natives is the signature of a captured ubiquitous language; the mathematical content is that the obvious factorization provably suffices for the exact value.

## 2. God's-eye vs. seat's-eye — the deliberately unresolved question

The constellation as built is a **God's-eye object**: perfect-information, all four hands realized — the substrate where the census, C1, and the kernel proofs live. A seat never sees one. What a seat maintains is: **(own hand exactly) + (constraint residue on everyone else) + (model-gated beliefs)** — a *query whose extension is a weighted set of constellations*, accessed lazily through salience.

Two readings, held open:
- Constellations are the substrate — fundamental as the things seat-level queries range over and the exact value is a function of — and "it" (the theory of 42 as played) is the seat-level object living on top. Semantics vs. pragmatics.
- Jason's gut is flagging something deeper that neither reading names yet.

Distinguishing evidence, someday: if a player restricted to the seat-level object plays boring-competent against the exact value, the filtration lost nothing (see §7).

## 3. Memory = the residue, cached; the cost inversion

Phenomenology (cold read of a live position, 2026-08-01): attention radiates outward from the hand lazily — hand first, trump consulted like checking the wind, stakes before contract details, history loaded only on anomaly; irrelevant tiles never enter the mental context at all (salience filtration). Interpreting history is not a glance but a **replay** — a play-by-play reconstruction.

Frame: the seat's context is a **derived view of the history stream, not of the position** — a fold, `context_{t+1} = update(context_t, event_t)`. It is cheap to maintain incrementally and expensive to reconstruct cold; a cold read is a cache miss. This is fully inside the project's "derived views, never stored state" discipline — there is no second authority; only the domain changes (history, not position) and the natural evaluation strategy (incremental caching).

**Cost inversion:** on a random unremarkable play the decision is nearly free; the expensive thing is context. The intelligence lives almost entirely in the maintained state — the reverse of the standard game-AI cost model. Every existing piece one-shots (rob is God's-eye exact; sigma is an aggregate; E[Q] gathers futures into an EV) — all excellent, none carries context. PIMC-family wackiness is memorylessness observable from across the table: worlds redealt per decision, no carried inferences, no cross-trick consistency.

## 4. The two-sort constraint store (what we are narrowing, expressibly)

The fold's steps are not cheap, and each is forward-from-its-point. The narrowing splits into two sorts:

- **Sort 1 — structural narrowings.** Played-set (which morphs standings — the constellation substrate) and the void matrix ("didn't follow suit ⇒ has no X"). Monotone, commutative, order-free, glanceable from the aggregate record. Lattice-shaped; propagates for free.
- **Sort 2 — evidential narrowings.** One per observed action: a revealed-preference constraint — "actor chose a from the alternatives available *at that state*, under their information, read through my model of them." Order-indexed, graded, defeasible, model-gated (an unreliable-opponent model discounts Sort 2 wholesale while Sort 1 stands). Statable only by replaying to reconstruct each step's context; each step is itself a small evaluation. This is why history is replayed, and why context is expensive: its cost is the sum of per-step evaluations of everyone's past choices.

Candidate expressible shape: a **constraint store over the realization fiber** of the current constellation — structural constraints propagate; evidential constraints weight/filter. Resonances: the 009 backward-commutation postmortem independently called its failure "embeddability-shaped in exactly this micro-CSP sense" (same smell from the backward side); constraint stores / abstract-interpretation-style factored domains are precisely a shape computers are good at.

**Open middle:** memory visibly *compiles* Sort-2 facts into flat Sort-1 form ("so they don't have 1s"). Does that compilation have a general, factored, cheap form? This is the sharpened statement of "a useful and efficient factorization of constraints so we know when a constellation could apply" — one question with a forward face (smallest carried state making play cheap) and a backward face (smallest residue making constellation-applicability local). Same object, two ends.

## 5. Feedback: fold intermediates as the "this" hindsight attaches to

Keeping the fold's intermediates gives credit assignment a target. Each intermediate is a typed claim (a standing claim, a graded possibility, a protection relation, a model-gated inference) — so hindsight can adjudicate claims *individually* instead of smearing outcome over an opaque (decision, outcome) pair.

Verdict taxonomy = an **error factoring**, each factor with a different repair target:
- *worked / didn't work* — calibrates the graded evaluations;
- *didn't work because of bad luck* — ex-ante sound, realized world in a correctly-weighted tail; repairs **nothing** (possible ≠ probable doing feedback work; knowing when to repair nothing is half of intuition);
- *had more danger than I considered* — not an evaluation error: the store never loaded the relevant query. Audits the **salience filtration**; detectable only because what *was* considered is recorded.

Honesty guarantee: intermediates are forward-from-their-point (each carries its information state), so revealed information flows into the *assessment of recorded claims*, never into play — not cheater vision, not Monday-morning quarterbacking. Consonant with the repo's evidence discipline: outcome is evidence, never a status change for what was known.

What learns: **store policies only** — salience policy, player models, grading of evidential constraints, Sort-2→Sort-1 compilation rules. The exact substrate never updates. Frozen semantics, learned pragmatics.

Transfer: verdicts are indexed by relational (constellation-shaped) keys, which is what makes lessons portable across hands with different tiles — the quotient as the generalization key of experience. "Memorable" standings (the 1-0 boss) suggest surprise-gated storage.

The hindsight adjudicator is exactly computable at reveal (42 reveals everything at hand's end): rob scores recorded alternatives ex post; the recorded ex-ante store supplies weights; all four verdicts get operational definitions.

## 6. Recast roles of the existing pieces

- **rob** (exact solver): the truth of the substrate; additionally, the oracle *inside* fold steps ("which worlds make that past play reasonable?") and the ex-post adjudicator behind them. Argmax-over-plans moves inside the step; plans are artifacts of the value.
- **E[Q]**: the graded/soft version of the same two jobs.
- **sigma**: disposable scaffolding (unchanged from prior direction).
- The parked **extension-type census** (e(c), κ(c), predecessor-spectrum uniformity conditioned on (carrier-skeleton, constellation)) is the first empirical probe of the residue question — it measures the one-trick backward explosion per constellation and tests the first candidate residue. Parked awaiting Jason's go.

## 7. Acceptance test

Expectiminimax over [constellations and ranked actions] from trick 0, tractable, yielding a **boring competent player** ⇒ the factorization is correct. Boring is the point: no brilliancies (which would suggest the quotient smuggles privileged information) and no howlers (which would mean it discarded something load-bearing). A dull, sound player is what "the abstraction leaks nothing" looks like from outside.

## 8. Open questions

1. Smallest sufficient residue (forward face = backward face, §4).
2. Does Sort-2 compilation have a general factored form, or does each evidential constraint stay an opaque re-ask-the-oracle-at-step-t?
3. Formalize salience (lazy query evaluation over the store; relation to 014's salience filtration and upper-set promotion calculus — idea-tier concepts, see the ledger's 014 row).
4. Whether "is it it?" (§2) resolves, and in which direction.
