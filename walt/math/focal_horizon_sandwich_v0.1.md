# The Focal-Horizon Sandwich: a Canonical Anytime Refinement Calculus for Walt

**Status:** mathematical intake for engineering  
**Tier:** EXPLORATORY until adopted and gated in the repository  
**Repository basis:** `jasonyandell/texas-42`, after merged PR #87 (`a80b98291a60c5fda22d44f528efc16016c53425`)  
**Scope:** fixed declared field first; model-belief lift deliberately deferred  
**Objective:** `pmake` first; bounded-score generalization follows by the same Bellman algebra

---

## 0. Executive result

This intake proposes one canonical refinement hierarchy for the current Walt mathematics.

The lower side begins with one lawful executable continuation policy. The upper side begins with an admissible optimistic continuation, initially the world-revealed (`God`) continuation where affordable. One additional **focal information-consistent decision layer** is made exact at each refinement level.

For every public belief state \(B\) and focal horizon \(k\ge0\), the construction produces

\[
\boxed{L_k(B)\le Q(B)\le U_k(B)}
\]

with monotone nesting

\[
\boxed{L_0(B)\le L_1(B)\le\cdots\le Q(B)\le\cdots\le U_1(B)\le U_0(B).}
\]

If \(h_f(B)\) is the maximum number of focal decisions remaining on any continuation from \(B\), then

\[
\boxed{k\ge h_f(B)\Longrightarrow L_k(B)=Q(B)=U_k(B).}
\]

Thus this is not a heuristic rollout depth. It is a **finite exact hierarchy**. Tail quality controls only how early the sandwich closes.

For fixed root action \(a\),

\[
L_{a,k}\le Q_a\le U_{a,k}.
\]

Let

\[
B_k=\max_a L_{a,k},\qquad
\mathcal S_k=\{a:U_{a,k}\ge B_k\}.
\]

Then

\[
\boxed{\mathcal S_{k+1}\subseteq\mathcal S_k.}
\]

An exact action is established when

\[
\boxed{L_{b,k}>\max_{a\ne b}U_{a,k}.}
\]

A materialized lawful policy \(\widehat\rho_k\) has certified regret

\[
\boxed{Q^*-V(\widehat\rho_k)\le \max_aU_{a,k}-L(\widehat\rho_k).}
\]

The same hierarchy is simultaneously

\[
\boxed{\text{rollout improvement below}\quad+\quad\text{canonical information gluing above}.}
\]

It unifies rollout improvement, salvation masks, gluing, God uppers, exact suffix reuse, action-safe intervals, certified regret, and eventual exact best response.

The recommended engineering move is bold but bounded: **build one generic fixed-field focal-horizon engine for \(k=0,1,2\) before spending a larger campaign measuring arbitrary local refinements.** If it closes quickly, Walt gets the mechanism directly. If it does not, the same run produces the detailed frontier data needed for a selective scheduler.

---

# Part I. Empirical basis, not theorem premises

## 1. What merged PR #87 established

The mathematics below does not depend on these measurements. They explain why focal-decision depth is the refinement axis chosen here.

### 1.1 Lazy posterior carry is already settled engineering evidence

The unified player now records the public line and materializes model belief only when a model-sensitive tier reads it. On the declared 216-decision corpus, lazy and eager carry produced identical actions, evidence, refusals, frames, and join readings. The lean rung's previously unread posterior work disappeared.

This intake consumes that result. It does not propose another carry experiment.

### 1.2 A depth-only fusion horizon is too coarse

Uniform trick-5 receipt roots had appeared God-tight. The in-solve horizon census examined conditioned trick-5 beliefs actually reached inside trick-4 solves and found positive information price at substantial subsets of those nodes.

The local object is therefore

\[
\boxed{\Phi(B;c,\sigma)=U^{\mathrm{God}}(B;c,\sigma)-Q(B;c,\sigma)}
\]

and the zero-gap region

\[
\boxed{\mathcal Z_{c,\sigma}=\{B:\Phi(B;c,\sigma)=0\}.}
\]

Any exact substitution claim must bind the actual public belief state, contract, field identity, and utility.

### 1.3 Scalar value closeness is not decision safety

PR #87 measured optimistic frontier substitutions with only a few permille of root-value error that nevertheless changed the selected root action. The exact h8-t3 fixed-field anchor likewise showed a coarse optimistic cut selecting a different action from the exact root solve.

Therefore no production approximation may treat small scalar error as sufficient evidence of action safety. The hierarchy here is action-indexed from the beginning.

### 1.4 The earlier computational wall is especially severe in augmented model space

The fixed-field h8-t3 response completed exactly, while the earlier eight-profile model-belief recursion at the same general depth had refused under its declared read budget. This supports a sequencing rule:

\[
\boxed{\text{fixed field first}\;\to\;\text{measure closure economics}\;\to\;\Omega\times\Theta\text{ lift}.}
\]

---

# Part II. Finite public-belief model

## 2. State categories

Fix one finite continuation problem under one declared semantics identity:

- public belief state \(B\);
- focal seat/team;
- fixed contract \(c\);
- fixed utility \(u\in[0,1]\), initially the `pmake` indicator;
- fixed field semantics \(\sigma\);
- finite physical posterior represented by Walt's exact factor belief.

Every reachable public belief state is one of:

1. **terminal/decided:** utility is fixed for every continuation;
2. **focal:** the focal player acts and must choose one legal action from public information only;
3. **modeled-seat/public-branch:** the declared field acts; its public action is observed and partitions the belief into child public beliefs.

For a modeled-seat node,

\[
p_t(B)=\Pr(t\mid B),\qquad \sum_t p_t(B)=1.
\]

In Walt's exact-mass form,

\[
\sum_t Z(B_t)=Z(B).
\]

Only positive-mass public branches participate.

---

## 3. Exact information-consistent value

Let \(Q(B)\) be the exact best-response value against the declared field over lawful focal policies only.

At terminal/decided states,

\[
Q(B)=u(B).
\]

At focal states,

\[
\boxed{Q(B)=\max_{a\in A(B)}Q(Ba).}
\]

At modeled-seat/public-branch states,

\[
\boxed{Q(B)=\sum_t p_t(B)Q(B_t).}
\]

The hidden branch is a sum **after public partition**. The focal choice is a max **after all hidden cells producing the same public history have been merged**. Reversing that order is strategy fusion.

---

# Part III. The two tails

## 4. Lawful lower tail

Choose one deterministic lawful continuation policy \(\pi\). It may be a frozen baseline, an extracted policy, or another named executable policy.

Let

\[
V^\pi(B)
\]

be its exact continuation value. Then

\[
\boxed{V^\pi(B)\le Q(B).}
\]

At a focal state,

\[
V^\pi(B)=V^\pi(B\pi(B))\le\max_aV^\pi(Ba).
\]

At modeled-seat public branches,

\[
V^\pi(B)=\sum_t p_t(B)V^\pi(B_t).
\]

For the first fixed-field slice, prefer an exactly evaluable lower tail so the hierarchy itself contains no sampling risk.

---

## 5. Optimistic upper tail

Choose an admissible continuation upper \(G(B)\) satisfying

\[
\boxed{Q(B)\le G(B).}
\]

For monotone focal-horizon refinement, require \(G\) to be a Bellman supersolution:

- terminal/decided exactness;
- public-branch harmonicity
  \[
  G(B)=\sum_t p_t(B)G(B_t);
  \]
- focal optimism
  \[
  G(B)\ge\max_aG(Ba).
  \]

The primary first-slice choice is

\[
G(B)=U^{\mathrm{God}}(B),
\]

the world-revealed continuation. At a focal node its supersolution law is exactly

\[
\sum_\omega\beta(\omega)\max_a q_\omega(a)
\ge
\max_a\sum_\omega\beta(\omega)q_\omega(a).
\]

Where exact God continuation is unaffordable, a branch may retain a previously valid upper or refuse. A truncated search result is not an upper merely because the budget ended.

---

# Part IV. Focal horizon

## 6. Remaining focal depth

Define

\[
h_f(B)=0
\]

for terminal/decided \(B\),

\[
\boxed{h_f(B)=1+\max_{a\in A(B)}h_f(Ba)}
\]

for focal \(B\), and

\[
\boxed{h_f(B)=\max_t h_f(B_t)}
\]

for modeled-seat/public-branch \(B\).

Only focal decisions consume horizon. Public observations do not.

This is the essential design choice. The relaxation error being removed is clairvoyant focal choice, not lack of observation of a public field action.

---

## 7. Lower focal-horizon value

Define \(L_k^\pi(B)\) by structural recursion on the finite continuation tree.

At terminal/decided states:

\[
L_k^\pi(B)=u(B).
\]

At a focal state with zero remaining explicit horizon:

\[
\boxed{L_0^\pi(B)=V^\pi(B).}
\]

At a focal state with \(k+1\) remaining focal layers:

\[
\boxed{L_{k+1}^\pi(B)=\max_{a\in A(B)}L_k^\pi(Ba).}
\]

At a modeled-seat/public-branch state:

\[
\boxed{L_k^\pi(B)=\sum_t p_t(B)L_k^\pi(B_t).}
\]

### Interpretation

\(L_k^\pi\) is the value of the best lawful policy that may optimize the next \(k\) focal decision layers on each public trajectory and then returns to \(\pi\).

With a deterministic tie rule, the maximizing choices form one materialized information-consistent policy \(\pi_k\), because each max occurs at one real public information state and distinct public branches are distinguishable histories.

Therefore

\[
\boxed{V^{\pi_k}(B)=L_k^\pi(B).}
\]

Every lower endpoint is executable, not merely existential.

---

## 8. Upper focal-horizon value

Define \(U_k^G(B)\) similarly.

At terminal/decided states:

\[
U_k^G(B)=u(B).
\]

At a focal state with zero remaining explicit horizon:

\[
\boxed{U_0^G(B)=G(B).}
\]

At a focal state with \(k+1\) remaining focal layers:

\[
\boxed{U_{k+1}^G(B)=\max_{a\in A(B)}U_k^G(Ba).}
\]

At a modeled-seat/public-branch state:

\[
\boxed{U_k^G(B)=\sum_t p_t(B)U_k^G(B_t).}
\]

With \(G=U^{\mathrm{God}}\):

- \(U_0\) allows clairvoyant focal continuation immediately;
- \(U_1\) requires the current focal choice to be one common public-information action, then permits clairvoyance;
- \(U_2\) requires the current and next focal decision layers to be information-consistent;
- and so on.

Each increase in \(k\) removes one canonical layer of strategy fusion.

---

# Part V. The sandwich theorems

## 9. Theorem 1 — lower validity and monotonicity

For every \(B\) and \(k\ge0\),

\[
\boxed{V^\pi(B)=L_0^\pi(B)\le L_k^\pi(B)\le L_{k+1}^\pi(B)\le Q(B).}
\]

**Proof sketch.** Horizon \(k+1\) can emulate every horizon-\(k\) policy and additionally optimize one more focal layer. Every horizon policy remains lawful. Equivalently, use structural induction with focal `max` monotonicity and nonnegative public-branch summation. ∎

---

## 10. Theorem 2 — upper validity and monotonicity

Assume \(G\) is an admissible Bellman supersolution. Then

\[
\boxed{Q(B)\le U_{k+1}^G(B)\le U_k^G(B)\le U_0^G(B)}
\]

at focal states, with the corresponding hidden-node propagation for arbitrary \(B\).

**Proof sketch.** At horizon zero, \(G\) is admissible. Adding a focal layer requires one action to be common before access to the optimistic tail and therefore restricts the relaxation. The base focal inequality is \(G(B)\ge\max_aG(Ba)\). Induction uses `max` monotonicity and public-branch linearity. ∎

---

## 11. Theorem 3 — focal-horizon sandwich

For every public belief state \(B\) and \(k\ge0\),

\[
\boxed{L_k^\pi(B)\le Q(B)\le U_k^G(B).}
\]

Moreover,

\[
\boxed{L_k^\pi(B)\le L_{k+1}^\pi(B)\le Q(B)\le U_{k+1}^G(B)\le U_k^G(B).}
\]

This is the primary implementation invariant.

---

## 12. Theorem 4 — finite exact collapse

If

\[
k\ge h_f(B),
\]

then the tail is never consulted on any continuation and

\[
\boxed{L_k^\pi(B)=Q(B)=U_k^G(B).}
\]

**Proof sketch.** Induct on the finite continuation tree. Public branches consume no horizon. A focal state consumes one unit, and the definition of \(h_f\) ensures the induction hypothesis applies to every child. Both recurrences become the exact Bellman recurrence. ∎

### Consequence

The hierarchy is complete regardless of tail quality. Tail quality affects only how early the lower and upper meet.

---

# Part VI. Salvation masks and gluing

## 13. Theorem 5 — one-step God upper is the salvation-mask upper

Let \(B\) be focal. For legal action \(a\), define

\[
S_a(B)=\{\omega:\text{after common action }a,\text{ a world-aware continuation can make}\}.
\]

Then

\[
U_0^{\mathrm{God}}(Ba)=\Pr(S_a(B)),
\]

so

\[
\boxed{U_1^{\mathrm{God}}(B)=\max_a\Pr(S_a(B)).}
\]

Thus salvation masks are not another subsystem. They are the first nontrivial member of the upper focal-horizon hierarchy.

Doom is the empty-mask case: worlds belonging to no \(S_a\) contribute to no action's salvageable mass.

---

## 14. Higher horizons are canonical multi-state gluing

At horizon two, the current focal action and every next focal action are information-consistent within their own public information states. Horizon three glues one more focal layer, and so on.

Therefore

\[
\boxed{U_0^{\mathrm{God}}\ge U_1^{\mathrm{God}}\ge U_2^{\mathrm{God}}\ge\cdots\ge Q}
\]

is a canonical information-consistency staircase.

This does not imply arbitrary unary glues have diminishing returns. Short glue coalitions may still be necessary. The point of the focal hierarchy is that engineering need not choose such coalitions before testing the canonical next layer.

---

# Part VII. Rollout improvement is the lower dual

## 15. Targeted reasoning over a boring tail

\[
L_0^\pi=V^\pi
\]

is the boring lawful continuation.

\[
L_1^\pi
\]

optimizes the current focal decision and then returns to \(\pi\).

\[
L_2^\pi
\]

optimizes the current and next focal decision layers before returning to \(\pi\).

This is targeted search around a lawful policy, not another global best-response ladder.

The same baseline may serve as executable fallback, lower tail, response column, and post-horizon continuation.

---

# Part VIII. Action-indexed proof state

## 16. Root-action intervals

Let \(B_0\) be focal. Fix root action \(a\), producing child \(B_0a\). Define

\[
\boxed{L_{a,k}=L_k^\pi(B_0a),\qquad U_{a,k}=U_k^G(B_0a).}
\]

Here \(k\) counts **additional** focal decision layers after the root action.

Then

\[
\boxed{L_{a,k}\le Q_a\le U_{a,k}}
\]

and

\[
L_{a,k}\le L_{a,k+1},\qquad U_{a,k+1}\le U_{a,k}.
\]

---

## 17. Theorem 6 — survivor monotonicity

Let

\[
B_k=\max_aL_{a,k},\qquad
\mathcal S_k=\{a:U_{a,k}\ge B_k\}.
\]

Then

\[
\boxed{\mathcal S_{k+1}\subseteq\mathcal S_k.}
\]

**Proof.** The bar can only rise and every action upper can only fall. ∎

No safely excluded action may return at a larger focal horizon.

---

## 18. Exact action criterion

If

\[
\boxed{L_{b,k}>\max_{a\ne b}U_{a,k},}
\]

then \(b\) is the unique exact optimal root action.

For exact ties, retain the exact survivor set once the relevant endpoints collapse.

---

## 19. Certified executable regret

Let \(\widehat\rho_k\) be the best materialized lower policy available, with

\[
L_{\mathrm{exec},k}=V(\widehat\rho_k).
\]

Let

\[
U_k^*=\max_aU_{a,k}.
\]

Then

\[
\boxed{0\le Q^*-V(\widehat\rho_k)\le U_k^*-L_{\mathrm{exec},k}.}
\]

Define

\[
\boxed{\Gamma_k=U_k^*-L_{\mathrm{exec},k}.}
\]

With deterministic exact tails and preserved facts,

\[
\Gamma_{k+1}\le\Gamma_k.
\]

Thus Walt may stop before exact action selection and still return a playable policy with a rigorous regret bound.

---

# Part IX. Why scalar cut error is insufficient

## 20. Value-close does not imply decision-safe

Suppose an approximation reports only a scalar optimum \(\widetilde Q^*\) with

\[
|\widetilde Q^*-Q^*|\le\varepsilon.
\]

This does not bound the regret of the action selected by \(\widetilde Q\). A losing action may be overestimated into a tie or narrow win while the maximum scalar value remains exact or nearly exact.

Therefore every focal-horizon producer used for action selection must preserve either:

1. action-indexed intervals \([L_{a,k},U_{a,k}]\), or
2. an independently evaluated executable policy with a global upper against which regret is computed.

---

## 21. Margin form

If

\[
\widetilde Q_a-\varepsilon_a\le Q_a\le\widetilde Q_a,
\]

then approximate winner \(b\) is exact only when

\[
\boxed{\widetilde Q_b-\varepsilon_b>\max_{a\ne b}\widetilde Q_a.}
\]

A few permille of looseness can be irrelevant under a wide action margin and decisive under a narrow one.

---

# Part X. Exact-mass form

## 22. Division-free implementation

For `pmake`, let \(Z(B)\) be exact posterior mass and define

\[
M_Q(B)=Z(B)Q(B),\quad
M_k^L(B)=Z(B)L_k(B),\quad
M_k^U(B)=Z(B)U_k(B).
\]

At a public modeled-seat branch,

\[
\boxed{M(B)=\sum_tM(B_t).}
\]

At a focal state, each legal focal action preserves the hidden posterior mass, so

\[
\boxed{M(B)=\max_aM(Ba).}
\]

The central invariant is

\[
\boxed{M_k^L(B)\le M_Q(B)\le M_k^U(B).}
\]

No floating-point arithmetic is required in the exact fixed-field hierarchy.

---

# Part XI. Interruption and residuals

## 23. Sound interruption rule

A budget may prevent completion of a requested horizon. A refusal must never manufacture a partial value.

Use the strongest previously valid child intervals:

- completed child: install its new \([L,U]\);
- unfinished child: keep its prior valid \([L,U]\);
- focal parent:
  \[
  [\max_aL_a,\max_aU_a];
  \]
- modeled-seat public parent:
  \[
  [\sum_tp_tL_t,\sum_tp_tU_t].
  \]

This yields a sound partially refined proof state even when the requested global \(k+1\) horizon is not finished everywhere.

A producer may return a completed horizon, partial action intervals plus a resumable frontier, or a typed affordability refusal leaving prior intervals unchanged.

---

## 24. Gap measurements become a byproduct

The hierarchy automatically measures

\[
\boxed{\Delta_{a,k}^L=L_{a,k+1}-L_{a,k}\ge0}
\]

and

\[
\boxed{\Delta_{a,k}^U=U_{a,k}-U_{a,k+1}\ge0.}
\]

Those quantities, together with survivor movement and work cost, are the raw material for a later selective scheduler if canonical full-layer refinement proves too expensive.

No measurement-only campaign is required before the producer exists.

---

# Part XII. Exact suffix reuse

## 25. Continuation substitution theorem

Suppose reachable public belief state \(C\) has a receipt establishing

\[
L(C)=U(C)=Q(C)
\]

under matching physical/public state, field, contract, utility, belief weights/representation, and focal-seat identity.

Then replacing the entire continuation below \(C\) by exact scalar \(Q(C)\) preserves every ancestor value under the same Bellman recursion.

If a policy attaining \(Q(C)\) is stored, the substitution also preserves construction of a complete executable lower witness.

Thus exact late-state receipts should become terminals **inside** earlier focal-horizon solves, not merely answers at decision roots.

A coarse statement such as "trick 6 is nearly exact" is never enough for exact substitution. The receipt is state-specific.

---

# Part XIII. Model-belief lift

## 26. Fixed field first

The first slice should remain under one declared fixed field \(\sigma\). The exact physical response authority and God continuation already exist, and PR #87 supplied an exact trick-3 fixed-field anchor.

---

## 27. Later lift to persistent behavioral types

For model belief, augment latent state to

\[
\Xi=\Omega\times\Theta
\]

or \(\Omega\times\Theta\times Z\) with explicit stochastic tapes.

The public Bellman structure is unchanged: focal max, modeled-seat public partition/sum. Therefore the focal-horizon theorem lifts unchanged.

The difference is computational cost, not mathematical validity.

---

# Part XIV. First engineering slice

## 28. One generic engine

Conceptual interface:

```text
focal_horizon(
    belief_state,
    focal_depth,
    lower_tail,
    upper_tail,
    target_identity,
    work_budget,
) -> FocalHorizonResult
```

Reuse existing authorities for legal actions, state transition, factor conditioning, field action, decided arithmetic, score/contract semantics, exact-cover mass accounting, and fixed-policy replay.

Do not fork them.

Minimal semantic result:

```text
FocalHorizonResult {
    action_intervals,
    executable_policy,
    executable_lower,
    global_upper,
    certified_regret,
    survivors,
    completed_focal_depth,
    residual_frontier,
    spend,
    refusals,
    identity,
}
```

Names are proposals. Semantics are not.

---

## 29. Initial tails and horizons

### Lower tail

Use one existing lawful deterministic policy \(\pi\) with an independent exact fixed-policy evaluator.

### Upper tail

Use exact world-revealed continuation where affordable. On an unaffordable branch, retain a previously valid upper or refuse; do not install an unfinished number.

### Horizons

Run

\[
\boxed{k\in\{0,1,2\}}
\]

first.

The question is not whether two layers are universally sufficient. It is whether one or two explicit focal layers close enough of the actionable gap to justify the hierarchy economically.

---

# Part XV. Mandatory gates

## 30. FH1 — endpoint parity

On affordable exact roots, independently rederive

\[
\boxed{L_0^\pi=V^\pi,\qquad U_0^{\mathrm{God}}=U^{\mathrm{God}}.}
\]

---

## 31. FH2 — sandwich and nesting

For every completed horizon:

\[
\boxed{L_k\le L_{k+1}\le Q\le U_{k+1}\le U_k.}
\]

Use exact integer/rational comparison.

---

## 32. FH3 — exact collapse

When the test horizon covers all remaining focal decisions:

\[
\boxed{L_k=Q=U_k.}
\]

Expose and independently verify the remaining focal depth.

---

## 33. FH4 — action containment

For every legal root action \(a\):

\[
\boxed{L_{a,k}\le Q_a\le U_{a,k}.}
\]

The exact best action must remain in the survivor set at every horizon.

---

## 34. FH5 — executable lower witness

Extract \(\pi_k\), replay it through the independent fixed-policy evaluator, and require

\[
\boxed{V^{\pi_k}=L_k.}
\]

This is the lower-side no-strategy-fusion gate.

---

## 35. FH6 — merge before max

Use a specimen with two hidden cells producing the same public action but requiring different focal continuations for success. The lawful engine must merge them before the focal max. A test-local cellwise-max implementation must be strictly optimistic and rejected.

---

## 36. FH7 — budget honesty

Under a budget too small to finish the next focal layer:

- no child is dropped;
- unfinished children retain previous valid intervals;
- the root interval still contains exact \(Q\) on an affordable fixture;
- the refusal names the actual boundary;
- resume plus completion equals uninterrupted completion.

---

## 37. FH8 — PR #87 anchors

At minimum include:

1. the exact h8-t3 fixed-field root, where the coarse optimistic cut selected a different action from exact play;
2. one trick-6 frontier row where a few-permille optimistic cut changed the root action;
3. one contract-sensitive trick-5 frontier specimen.

Do not pin the focal-horizon answer in advance beyond the soundness laws. The experiment is to discover the smallest \(k\) that settles or \(\varepsilon\)-settles these anchors.

---

# Part XVI. Report of record

## 38. Measurements produced by the mechanism

For each root/action/horizon, report

\[
L_{a,k},\qquad U_{a,k},\qquad U_{a,k}-L_{a,k},
\]

plus survivor set, exact action where independently known, extracted policy id, executable lower, global upper, certified regret, lower-policy action, action changes by horizon, exact field reads, conditioned nodes, exact-suffix receipt hits, completed focal depth, refused frontier mass/count, and approximate wall time.

The report should make the lower and upper increments explicit:

\[
\Delta_{a,k}^L=L_{a,k+1}-L_{a,k},
\qquad
\Delta_{a,k}^U=U_{a,k}-U_{a,k+1}.
\]

If the hierarchy disappoints, these are exactly the measurements needed to build the more selective local-glue scheduler.

---

# Part XVII. Success and falsifiers

## 39. Strong practical success

The hierarchy is a strong practical success if \(k\le2\) frequently yields a singleton exact survivor, exact tie set, accepted certified regret, large upper movement at tractable cost, or reusable exact suffixes that materially shorten earlier solves.

No success rate is assumed in advance.

---

## 40. Honest partial success

Even if \(k\le2\) settles few early positions, the slice succeeds mathematically if all sandwich gates hold, lower witnesses materialize lawfully, uppers nest, and the report localizes the remaining width.

---

## 41. Correctness failures

Stop and investigate if any of the following occurs:

1. a lower cannot replay as one lawful information-consistent policy;
2. exact \(Q_a\) falls outside a reported action interval;
3. a lower falls as \(k\) increases;
4. an upper rises as \(k\) increases;
5. a modeled-seat public branch consumes focal horizon;
6. hidden cells producing the same public action are maximized separately before merging;
7. a budget refusal returns a truncated number without independent bound authority;
8. a suffix receipt is reused under mismatched semantics identity.

These are correctness failures, not disappointing measurements.

---

# Part XVIII. Deliberate non-goals for this slice

Do not combine the first focal-horizon build with arbitrary glue-coalition selection, the full \(\Omega\times\Theta\) hierarchy, a new residual behavior type, joint partnership prescriptions, a global response-polytope representation, a new rules engine, a live default change, or a broad arena claim.

The bold move is one canonical hierarchy, not every future idea at once.

---

# Part XIX. Compact theorem sheet

For lawful tail \(\pi\) and admissible Bellman supersolution \(G\):

\[
L_0^\pi=V^\pi,\qquad U_0^G=G
\]

at exhausted focal horizon;

\[
L_{k+1}^\pi(B)=\max_aL_k^\pi(Ba),
\qquad
U_{k+1}^G(B)=\max_aU_k^G(Ba)
\]

at focal nodes;

\[
L_k^\pi(B)=\sum_tp_tL_k^\pi(B_t),
\qquad
U_k^G(B)=\sum_tp_tU_k^G(B_t)
\]

at public modeled-seat branches.

Therefore

\[
\boxed{L_k^\pi(B)\le Q(B)\le U_k^G(B),}
\]

\[
\boxed{L_k^\pi(B)\le L_{k+1}^\pi(B),\qquad U_{k+1}^G(B)\le U_k^G(B),}
\]

\[
\boxed{k\ge h_f(B)\implies L_k^\pi(B)=Q(B)=U_k^G(B),}
\]

\[
\boxed{L_{b,k}>\max_{a\ne b}U_{a,k}\implies b\text{ is the unique exact optimal action},}
\]

and

\[
\boxed{Q^*-V(\widehat\rho_k)\le\max_aU_{a,k}-L(\widehat\rho_k).}
\]

With a God tail,

\[
\boxed{U_1^{\mathrm{God}}(B)=\max_a\Pr(\text{world remains individually salvageable after common }a).}
\]

---

# Part XX. Engineering ruling requested by this intake

## 42. Recommended next move

Build the fixed-field focal-horizon vertical slice now, with \(k=0,1,2\), existing exact authorities as cross-checks, and the merged PR #87 anchors as stress tests.

The purpose is to test the strongest canonical refinement available **without first spending a separate engineering campaign choosing which local refinements to try**.

If it closes quickly, Walt gains a tractable targeted-reasoning mechanism directly.

If it does not, the same run returns the actionwise interval movement, residual frontiers, and exact costs needed to choose selective salvation/gluing work intelligently.

The fallback measurement program is preserved rather than discarded.

> **Central implementation sentence:** Make one more focal decision information-consistent on both sides of the proof state, not one more arbitrary trick of the tree exact.

> **Central mathematical sentence:** A boring lawful tail and a clairvoyant optimistic tail define a monotone finite sandwich around exact best response; each focal-horizon step improves the policy below, removes one canonical layer of strategy fusion above, and eventually collapses to the exact game.
