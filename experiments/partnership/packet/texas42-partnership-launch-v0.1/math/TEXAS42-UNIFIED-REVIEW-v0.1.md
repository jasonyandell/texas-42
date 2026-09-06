# Texas 42: A Unified Mathematical Core

## Review, corrections, and a falsifiable research program

**Version:** 0.1  
**Review date:** 2026-09-05  
**Status:** research synthesis and mathematical audit, not a replacement for the immutable rules/foundation packages and not an implementation specification.  
**Companions:** `verify_unification.py` and `verification_results.json`.

## Executive finding

The surviving work supports a coherent mathematical core:

> A finite family of information-consistent policies induces response vectors over hidden scenarios. Beliefs weight those vectors. Computation seeks an executable policy and enough valid upper constraints to certify its decision or bound its regret.

This is a stronger unification than a collection of compatible metaphors. It gives the same mathematical object to the belief, continuation, gluing, sampling, score, and model-belief work. It does **not** establish that opening-scale exact optimization is inexpensive.

Two questions must be separated permanently:

1. **Semantic question:** Can our ideas describe one correct imperfect-information decision problem? Yes, under explicit finite-game, information, field, and utility assumptions.
2. **Computational question:** Do typical early-game 42 decisions have certificates cheap enough to discover and verify without substantially traversing the enormous latent/policy space? That remains a substantive, testable hypothesis.

Several stronger compression hopes have already failed at their declared scope. Those failures do not refute the second question. Conversely, neither good sampled play nor the existence of a beautiful geometric description proves the second question.

The most promising surviving target is **small decision-proof complexity**, not a universally small representation of everything that could happen.

---

# 1. Review scope and evidence discipline

This review follows the recovered primary artifacts from the formal support/belief foundations through Scheme and transported roles, decision-sparse solving, pivotal sampling, counted beliefs, residual Bellman bounds, persistent model belief, Zora response geometry, and decision-safe frontier cuts. It also checks the connected repository's foundational proof index and negative-results index. It is not a claim to have reconstructed every message in every prior chat.

The review did not rebuild Lean, run the full Rust conformance suite, reproduce the arena results, or exhaustively adjudicate all theorems in the recovered manuscripts. Those are separate operations. Named proof declarations in the repository are reported as such; a theorem target in a packet is not treated as an existing kernel proof.

The companion program was newly written for this review. It checks small **generic finite response systems**, using integers and exact fractions. It does not simulate Texas 42 and is not evidence of opening-scale performance.

Use five different labels, rather than one undifferentiated word “proved”:

| Label in this review | Meaning |
|---|---|
| Repository-recorded formal result | The inspected proof index identifies a kernel theorem; not rebuilt here. |
| Finite theorem / proof below | The assertion follows from the stated hypotheses by the supplied argument; not thereby kernel-checked. |
| Checked here | The companion independently executed a declared finite verification. |
| Reported experiment | A recovered artifact reports the result on a specified corpus; not rerun here. |
| Hypothesis | A structural/economic claim still requiring proof or a properly designed experiment. |

The repository's own authority rules remain in force. In particular, exploratory Walt results are not silently promoted into foundational claims. The important trust boundary is also not merely “Lean versus Python”: a kernel verifies the theorem expressed by the definitions. Fidelity of those definitions to the intended rules and information model remains a review obligation. [S01, S02, S03]

---

# 2. The target must be an explicit mathematical contract

Fix a continuation target

\[
\mathcal T=(\text{rules},I_m,\beta,\sigma,U,\Pi,\text{identity conventions}).
\]

Here \(I_m\) is the focal seat's current information; \(\beta\) is its declared belief; \(\sigma\) is the nonfocal behavioral model, including the partner; \(U\) is terminal partnership utility; and \(\Pi\) is the permitted focal policy class. Identity conventions include relevant history, tie rules, seeds, fallback behavior, model versions, and any retained random state.

For a legal root action \(a\), let \(\Pi_a\subseteq\Pi\) contain policies taking \(a\) at the root. A policy is a complete contingent rule on focal information states, not just its opening domino.

A useful augmented scenario is

\[
\xi=(\omega,\theta,z)\in\Xi,
\]

where \(\omega\) is the physical deal, \(\theta\) a persistent behavioral-type profile, and \(z\) any persistent random tape. Deterministic fixed-field play is the special case with no nontrivial type/tape uncertainty.

**Putting a variable into the scenario does not reveal it to the focal player.** The information map still determines what a policy may read. A simulator can inspect a hidden world to price a policy; the policy cannot inspect it to select a move.

This target contract resolves many apparent contradictions in earlier work. A statement may be exact under one field but false under another; true for a fixed policy but not an optimized policy class; valid for expected score but not for making a contract. Bounds with different targets cannot simply be combined. [S05, S06, S12, S13, S14]

---

# 3. The foundation: possibility, probability, and behavior are different objects

## 3.1 Exact rule support

The capacity-cell foundation captures which hidden hands are legally compatible with the focal information and public history. Its losslessness theorem is substantial: the represented fiber equals the set of compatible deals, rather than merely an approximation to them.

The repository proof index identifies `Cells.losslessness`, the canonical reduction and normal-form equivalence, finite belief conditioning, and the strategic-sufficiency layer. It also records the 90-world witness demonstrating that identical support can accompany different posteriors and different optimal decisions. [S01]

These support results remain useful even if the ultimate policy solver changes completely.

## 3.2 Support is not belief

\[
\Omega(I_m)=\{\omega:\omega\text{ is compatible with }I_m\}
\]

is a set. A belief assigns weights to that set. Bidding and play can alter relative likelihoods without eliminating a world. A capacity normal form may therefore be minimal for **support** without being sufficient for **decision-making**.

A reduced strategic state must retain whatever additional public evidence and latent belief variables are needed by the field and future likelihoods. “This field did not read the history in a sample of tests” is not a sufficiency proof.

## 3.3 Feasibility is not reachability

Static capacity consistency does not imply that legal play can generate the proposed support. The follower-supply obstruction and explicit unreachable witnesses survive as important boundaries.

The inspected repository overview reports the exchange-tier reachable-support interval as **[36,45] bits**, with lower count 36,913,384,410 and necessary-outer-profile upper count 33,297,009,347,414. This is narrower than the older [36,46] memory. These are bounds on a support census, not a solver-state size or runtime theorem. The no-void census of 624,892,870 carries a recorded panel dissent: two SOUND reviews and one UNVERIFIABLE review finding no defect. This review does not erase that qualification. [S03]

**Conclusion:** preserve the support algebra as the legal domain and exact counting substrate. Do not ask it to stand in for beliefs or policies.

---

# 4. One response system unifies the continuation mathematics

For a fixed target and deterministic lawful policy \(\rho\), define

\[
v_\rho(\xi)=U(\operatorname{terminal}(\xi;\rho,\sigma)).
\]

All policy randomness can instead be averaged conditionally or made explicit in a finite scenario, subject to the same information restrictions.

The lawful root-action value is

\[
\boxed{Q_a(\beta)=\max_{\rho\in\Pi_a}\langle\beta,v_\rho\rangle.}
\]

The order is essential:

\[
\max_\rho\mathbb E_\beta[v_\rho]
\ \le\
\mathbb E_\beta[\max_\rho v_\rho].
\]

The right side permits a different continuation in each hidden scenario. It is an optimistic information relaxation, not generally an executable player.

A revelation upper must keep the rest of the target aligned. An older full-information minimax oracle may also change the opponents' information and behavior; its output is not automatically an upper on best response to the fixed modeled field. A separate domination argument would be required.

A two-world example is enough. Two policies have success vectors \((1,0)\) and \((0,1)\). Under equal weights, the lawful value is \(1/2\); the world-informed upper is 1. No averaging trick repairs the illegal world-dependent choice.

Define the response polytope

\[
\mathcal R_a=\operatorname{conv}\{v_\rho:\rho\in\Pi_a\}.
\]

Then

\[
Q_a(\beta)=h_{\mathcal R_a}(\beta),
\]

where the support function is the maximum dot product in direction \(\beta\). Consequently, for this fixed response system, policy value is linear in belief and the optimized value is convex and piecewise linear. Randomizing among complete deterministic policies cannot improve a fixed-belief linear objective beyond the best member.

This is the mathematical core already made explicit in the Zora packet. It is a description of the object; it is not an instruction to materialize its entire matrix or convex hull. [S13]

## 4.1 Dynamic interpretation

At a focal information state, one common action is selected: a `max`.

At a publicly observed nonfocal action, the belief updates and expected child values add: a probability-weighted `sum`.

If two internal hidden cells generate the **same public observation**, they must be merged before the next focal maximum. Otherwise the implementation gives the focal player a hidden-cell observation it does not possess.

At the response-set level, focal choice is convexification of a union; distinguishable observation branches permit independent continuation choices and produce a product over the branch coordinates. These identities require the appropriate closure of the lawful continuation class. A frozen library of whole policies is not automatically closed under arbitrary branchwise recombination; a grammar allowing such recombination is a different, though potentially lawful, class. [S05, S13]

## 4.2 Relationship to POMDP mathematics

With the other seats represented by fixed information-pure behavioral functions, this is a finite partially observed control problem; history can be retained in the state wherever needed. The response vectors are the familiar policy/alpha-vector representation of finite-horizon belief-state value functions. The general literature already contains the convex value geometry and bound-driven online planning viewpoint. [E01]

The right distinction is not “Walt versus POMDP.” It is **a correct information-consistent model versus a tractable way to compute with it**. Naming the model class settles neither the 42-specific factoring opportunity nor the opening computation.

---

# 5. The many meanings of compression

Several disagreements in the corpus become straightforward once the preserved object is named.

| Compression target | What must be preserved | What the existing negative results do not establish |
|---|---|---|
| Rule-support normal form | Exactly the compatible hidden deals | That posterior probabilities or strategic values are determined |
| Structural transport quotient | Chosen mechanics, labels, interfaces, and transported actions | That no useful value-directed abstraction exists |
| Dynamically sufficient quotient | Future observation/transition behavior under all admitted controls | That every exact evaluator needs explicit worlds |
| Linear predictive representation | A specified linear family of future tests | A size lower bound for arbitrary nonlinear circuits |
| Response/decision representation | Relevant policy comparisons or exposed optima | A cheap way to find the representation |
| Decision certificate | One action or policy's bound against all relevant alternatives | All values, posteriors, or policies being solved |

The negative-results index is unusually useful because it preserves failed hypotheses with their scopes. [S04]

The first-play structural quotient is rigid under the declared pip-trump structure and focal-seat fixing. Its 1,184,040 focal-hand count is not the 399,072,960 hidden-deal count for one fixed focal hand. Those are different carriers.

Complete tile-attributed records can distinguish every physical world. Thus a universal linear observation closure containing the relevant constant can become full-dimensional. But full dimension does not imply large arithmetic-circuit size: even a simple product distribution can describe exponentially many distinguishable outcomes compactly.

Retrograde classes can compress a completed inventory yet be expensive to discover. If the class identity depends on the entire future cone, computing it first may cost the very search it was supposed to avoid. The reported comparisons against ordinary memoization bear this out. A smaller quotient is not automatically a faster first solve.

**Revised hypothesis:** perhaps the information needed to *certify the chosen action* is far smaller than the information needed to reproduce all future observations. That is a new target with a real proof obligation, not a reinterpretation of the failed universal target. [S04, S06]

---

# 6. What remains of constellations, Scheme/Fix, roles, and valuation

These ideas should be assigned precise jobs, not discarded or promoted into a universal solver.

## 6.1 Relational predicates and roles

Scheme/Fix can describe relationships such as control, beaters, companions, and holder constraints more economically than lists of physical identities. A returned existential witness is not automatically a probability distribution; repeated existential queries do not guarantee identity through time. Rigid transported roles solve the identity problem when the required transport is proved.

The reported small lambda experiment found a four-atom relational descriptor giving 33 pure cells for eight responses over 90 worlds, while the tested holder-only vocabulary needed all 90. This supports the usefulness of consequence-shaped relations. It does not show that the descriptor is closed under every future observation or scales to the opening. [S07, S08]

## 6.2 Count-free structure versus arbitrary scoring

The rules can be studied independently of count valuation. A full additive terminal feature is

\[
\phi=(T,(X_d)_{d\in\mathcal D}),
\]

where \(T\) is focal-team tricks and \(X_d\) indicates capture of tile \(d\). For complete tricks,

\[
\sum_dX_d=4T.
\]

Hence

\[
(b,w)\sim(b-4c,w+c\mathbf1)
\]

gives identical additive scores \(bT+\sum_dw_dX_d\). This is an actual gauge redundancy, not a strategic compression theorem. It descends through a role interface only when that interface is capture-complete for the valuation. [S07]

A count-free optimal policy alone is insufficient for arbitrary later revaluation. Retaining a sufficiently rich feasible outcome/response object is what permits a later valuation direction to select another policy.

The order-exchange correction also stays: transported histories require transported policies. One arbitrary perfect-recall policy can lawfully react differently to two differently ordered public histories. Equality of achievable laws after policy transport is the correct conclusion, not literal equality under one unchanged history-indexed policy. [S06]

## 6.3 Contract probability is not expected score

For threshold \(c\), the objective is

\[
\Pr(S\ge c)=\mathbb E[\mathbf1\{S\ge c\}].
\]

Expected additive features alone do not determine this. An abstract score lottery \((0,42)\) and the constant score \((21,21)\), both with equal world weights, have the same expectation, but make threshold 30 with probabilities \(1/2\) and 0.

To revalue arbitrary contract thresholds exactly, preserve a sufficient **score distribution or scenario-level score response**, not merely its expectation. The small 42 count signature is valuable, but a small output alphabet is not itself a small strategic state.

Also, threshold nesting is only within a fixed response system. If the field reads the bid and changes behavior, changing the threshold changes more than the terminal readout. Even with a fixed field, the *fusion gap* need not be monotone in threshold. [S07, S11, S14]

---

# 7. Counted beliefs: a genuine representation change

Suppose the hidden root hands form a disjoint cover of the unseen pool, and the prior is

\[
W_0(H_1,H_2,H_3)
=\mathbf1\{\text{legal disjoint cover}\}\prod_s\phi_{s,0}(H_s).
\]

For a seat-local field, an observed action at public history \(h\) contributes a likelihood depending only on the acting seat's private hand, its modeled type if present, and that public history. Regrouping the likelihood product by actor gives

\[
\boxed{W_h(\mathbf H)
=\mathbf1\{\text{legal disjoint cover}\}\prod_s\phi_{s,h}(H_s).}
\]

The hidden hands are **not independent**: the disjoint-cover factor couples them. This is factorization of the unnormalized joint law, not an independence claim.

If \(Z_h=\sum_{\mathbf H}W_h(\mathbf H)\), then the exact next-action probability is \(Z_{ha}/Z_h\). Exact compatible-completion weights let one classify possible acting-seat hands rather than loop over complete deals. [S09]

At the unconstrained opening:

\[
\binom{21}{7}\binom{14}{7}
=116{,}280\cdot3{,}432
=399{,}072{,}960.
\]

A given hidden seat has 116,280 possible hands. Under the uniform root each has 3,432 completions. Thus its first-action distribution can be expressed exactly without a 399-million-world list.

This does not settle the recursive optimization bill. Evaluating the field on those hands may be costly; later weighted contractions may be harder; branch count and policy coupling remain. Correlated behavioral types or shared tapes require additional factors or mixture components and may enlarge contraction complexity.

The companion verifies posterior-factor regrouping independently on 90 six-piece covers and eight public histories. That supports the algebraic implementation example, not the runtime of the 42 opening.

---

# 8. Pivotal sampling is policy-difference geometry

For two **frozen** lawful policies \(\rho,\pi\), Boolean contract utility gives

\[
Y=v_\rho-v_\pi\in\{-1,0,1\}.
\]

Let benefit \(B=\Pr(Y=1)\), hazard \(H=\Pr(Y=-1)\), pivotal mass \(q=B+H\), and gap \(g=B-H\). Then

\[
\mathbb E[Y]=g,\qquad \operatorname{Var}(Y)=q-g^2.
\]

For \(q>0\), write \(g=q\tau\). This separates how often policies matter from which way their disagreements lean. [S05]

The enormous number of possible deals does not appear directly in the variance of this bounded comparison. That is one reason samples can identify a useful ordering without approximating the full posterior or value table. But a claim that sampling works *because* 42 always has sparse, favorable pivotal geometry still needs representative evidence.

There is a crucial caution: if \(\tau\) is fixed and \(q\) becomes tiny, discovering the exact sign ordinarily becomes harder, not easier. The variance-to-squared-gap scale is

\[
\frac{q-g^2}{g^2}=\frac{1}{q\tau^2}-1.
\]

Rare pivots may make two policies practically equivalent within a tolerance, while making the exact winner hard to distinguish. A structural proof that \(H=0\) is much stronger than observing no hazards.

A proved pivotal cover \(P\), outside which the difference is zero, yields

\[
g=\beta(P)\mathbb E[Y\mid P].
\]

This connects relational predicates, exact counting, and concentrated sampling. Counting \(P\), generating from it with the right law, and proving it contains every pivot are different obligations. An approximate mined cover must retain a bound or sampling authority on its complement. [S05]

## 8.1 The sampling errors already caught

The correctness-first repair distinguishes a reoptimized block race from evaluation of frozen policies. If the policy changes on each sample block, the inferential target changes with it.

A block-sign majority also need not agree with expected value. For example, a difference of \(+1/8\) with probability \(3/4\) and \(-1/2\) with probability \(1/4\) wins most blocks but has mean \(-1/32\).

Repeated nominal fixed-alpha tests are not made sequentially valid by exact integer arithmetic. Use a justified confidence sequence or explicitly allocated finite checkpoint risks, including all relevant comparisons and selection. The general confidence-sequence literature provides the time-uniform framework; the sampling law and target purity must still be established for the actual evaluator. [S15, E02]

**Exact evaluation of a discovered library is not exact optimization over every lawful policy.** The omitted policies still need an upper-bound argument. Nor does exact enumeration of only statistically surviving candidates erase an earlier false-elimination risk. A zero-risk conclusion for the original library must restore every potentially eliminated contender or independently discharge its exclusion. [S05]

---

# 9. Salvation complexes identify what imperfect information costs

For Boolean utility let

\[
S_\rho=\{\xi:v_\rho(\xi)=1\}.
\]

The salvation complex is

\[
\mathcal K=\{A\subseteq\Xi:\exists\rho\in\Pi,\ A\subseteq S_\rho\}.
\]

A face is a collection of scenarios that **one common lawful policy** can save. Thus

\[
\boxed{Q(\beta)=\max_{A\in\mathcal K}\beta(A).}
\]

The complex is downward closed. Let \(\mathcal H\) be its minimal nonfaces: the smallest collections of scenarios that cannot all be saved together.

Then

\[
\boxed{1-Q(\beta)
=\min\{\beta(F):F\cap C\ne\varnothing\ \forall C\in\mathcal H\}.}
\]

### Proof

A set \(F\) hits every minimal nonface exactly when its complement contains no nonface. Since the domain is finite, every nonface contains a minimal one. Therefore the complement is a face exactly when \(F\) hits every conflict. Maximizing saved mass is minimizing the complementary lost mass. ∎

This explains why individually impossible worlds are only one source of loss. Such worlds are singleton conflicts. There may be no singleton conflicts at all, yet a substantial unavoidable loss because incompatible worlds require different choices at a shared information state. [S11, S13]

## 9.1 Conflict packing as an upper certificate

For every proved conflict \(C\), any success response satisfies

\[
\sum_{\xi\in C}v_\rho(\xi)\le |C|-1.
\]

If nonnegative rational weights \(\lambda_C\) obey

\[
\sum_{C\ni\xi}\lambda_C\le\beta_\xi
\quad\text{for every }\xi,
\]

then every failure set has mass at least \(\sum_C\lambda_C\), so

\[
Q(\beta)\le 1-\sum_C\lambda_C.
\]

This is a checkable certificate using only the conflicts actually needed. It need not reconstruct the complete complex. Conversely, fractional conflict inequalities need not describe the exact response hull, so failure of a weak relaxation to close is not a refutation of the underlying theorem.

## 9.2 No generic theorem of low conflict order

Take \(k\) worlds and \(k\) policies, policy \(i\) failing only world \(i\). Every proper subset of worlds is saveable, but all \(k\) are not. The sole minimal conflict has order \(k\).

Thus a universal “pairwise conflicts suffice” theorem is false for generic finite response systems. Low-order conflict sufficiency in the relevant 42 distribution remains a meaningful empirical hypothesis. The companion checks this family through \(k=8\); the construction proves it for all \(k\ge2\).

---

# 10. Two independent ways to relax information

A policy signature \(s:\Pi\to\Gamma\) records selected policy coordinates, such as actions at certain information states. Define its optimistic completion envelope

\[
e^s_\gamma(\xi)=\max_{\rho:s(\rho)=\gamma}v_\rho(\xi).
\]

Then

\[
U_s(\beta)=\max_\gamma\sum_\xi\beta_\xi e^s_\gamma(\xi)
\]

is an upper bound. It chooses one common signature, but lets the unspecified completion depend on the hidden scenario.

A finer signature enforces more common choices and lowers the bound. The identity signature recovers the lawful optimum; the constant signature gives the scenario-informed upper.

Independently, reveal a latent partition \(\mathscr P\) before signature selection:

\[
U_{\mathscr P,s}(\beta)
=\sum_{C\in\mathscr P}\max_\gamma
\sum_{\xi\in C}\beta_\xi e^s_\gamma(\xi).
\]

A finer revealed partition raises the upper. More policy gluing lowers it. These are opposite operations and should not share an ambiguous use of “refinement.” [S13]

Physical-world revelation, type revelation, and complete-scenario revelation are different. A tile record may identify the physical deal without identifying persistent type or tape.

## 10.1 Compatibility, not one selected optimizer

An unlawful relaxed optimizer shows a violated constraint worth examining. It does not prove the optimal face contains no lawful optimizer. Two different selected optimizers also do not prove disjoint optimal sets.

Equality with an atomwise upper requires a common pointwise optimizer on all positive-mass atoms. Equality with a type-separated upper requires a common policy attaining the *conditional expected* optimum in every positive-mass type. Do not strengthen the latter into pointwise world optimality. [S06, S13]

## 10.2 Glues can have complementary value

Let two unobserved binary decisions \(x,y\) succeed when \(x\oplus y\) equals the hidden world bit. Fixing only \(x\) allows \(y\) to fuse by world; fixing only \(y\) allows \(x\) to fuse. Each isolated glue leaves the upper at 1. Fixing both lowers it to \(1/2\).

A scheduler that permanently rejects every individually unproductive glue can miss useful small coalitions. No generic diminishing-returns assumption is licensed.

---

# 11. Additional correction from this review: support domination is not set containment

The signature-envelope formula above is sound. However, its raw envelope hull

\[
\mathcal E_s=\operatorname{conv}\{e^s_\gamma\}
\]

need not contain the lawful response polytope \(\mathcal R\).

Consider again lawful columns \((1,0)\) and \((0,1)\). A constant signature has a single envelope \((1,1)\). Thus

\[
\mathcal R=\operatorname{conv}\{(1,0),(0,1)\},
\qquad
\mathcal E_s=\{(1,1)\}.
\]

For nonnegative beliefs, the envelope gives a valid support upper. Yet \(\mathcal R\not\subseteq\mathcal E_s\); the two sets are disjoint. Intersecting this raw “upper” hull with the exact lawful hull produces the empty set, not an improved valid outer approximation.

For utilities in \([0,1]\), the repair is

\[
\downarrow\mathcal E_s
=\{x\in[0,1]^\Xi:\exists y\in\mathcal E_s,\ x\le y\}.
\]

Every lawful column lies below its signature envelope, so

\[
\mathcal R\subseteq\downarrow\mathcal E_s.
\]

Also, for nonnegative beliefs,

\[
h_{\downarrow\mathcal E_s}(\beta)=h_{\mathcal E_s}(\beta).
\]

Thus the bound is unchanged, but the corrected object is safe to use in containment-based outer intersections. For other bounded rewards, use the corresponding reward box or an explicit affine normalization.

The Zora packet already includes downward salvation corners, so this is a **typing clarification at the interface between its valid constructions**, not a rejection of signature gluing. This review has not established that the implementation makes this mistake.

Keep two APIs conceptually distinct:

- a scalar/support upper for a declared nonnegative direction;
- a certified set outer approximation containing every lawful response.

The first is sufficient for scalar minimum-of-uppers. The second is required before taking set intersections and claiming they retain all lawful responses.

---

# 12. Persistent model belief extends the same core

A fixed field is a point-mass belief over behavioral types. A model-belief target uses a joint distribution over physical worlds and persistent types. A single focal policy must work across both uncertainties.

With \(\nu_\theta=\Pr(\theta)\) and conditional world/tape belief \(\beta(\cdot\mid\theta)\), write the separated upper unambiguously as

\[
U_{\mathrm{type}}
=\sum_\theta\nu_\theta
\max_\rho\mathbb E[v_\rho\mid\theta].
\]

Alternatively, maximize each **unnormalized joint slice** and sum the results, without multiplying by \(\nu_\theta\) again. This normalization convention should be explicit in every implementation and theorem statement.

Public actions update type probabilities jointly with deal probabilities. With conditional seat-local types, the likelihood can be absorbed into the acting hand/type factor. Correlated conventions require an explicit common latent or a mixture over profiles. [S12, S13]

Three boundaries remain:

**Persistence.** A type sampled once is not equivalent to independently selecting a type before every move. Two A-like actions have probability \(1/2\) under a uniform persistent binary type but \(1/4\) under independent resampling.

**Support completeness.** If every modeled type assigns an observed legal action probability zero, Bayes conditioning is undefined. A support-complete residual must exist beforehand; adding it afterward is a model revision.

**Model-relative optimality.** Exact best response to a library does not prove the library describes the actual players. More reasoning levels need not produce monotonic improvement, equilibrium, or stable conventions. A deterministic finite update operator is eventually periodic, which includes cycles. [S05, S12, S13]

## 12.1 First-disagreement coupling prices a field change

A fixed field need not ignore signals: it is a fixed *function* of private information and public history, and can react to both. Its response may therefore change after a revealing focal move without the field identity changing.

For two fixed fields, couple their executions under the same focal policy and latent input. Before the first differing nonfocal action the public paths coincide. If they never differ, the terminal outcomes coincide. With utility in \([0,1]\),

\[
|V^{\sigma}(\rho)-V^{\widetilde\sigma}(\rho)|
\le \Pr_\rho(\text{a first field disagreement occurs}).
\]

Taking a valid uniform upper over \(\rho\in\Pi_a\) bounds the difference of the two optimized root-action values. Bounding only the incumbent policy's exposure does not bound every competing optimum. Directional benefit/hazard variants can be sharper. [S05, S10]

This gives compiled or cheaper fields a precise job: solve cheaply, then pay a certified transfer allowance for possible consequential disagreement. A model change is not licensed merely by a high action-agreement percentage on a sampled corpus.

Separate physical and model fusion prices also need not add. If success requires guessing the XOR of two independent hidden bits, revealing either bit alone has zero value while revealing both has value \(1/2\). If the bits are identical, either revelation is already sufficient and their effects are redundant. Joint latent geometry matters.

---

# 13. The actual solver object: a persistent decision proof

For every root action maintain

\[
L_a\le Q_a\le U_a.
\]

A lower may come from exactly pricing a materialized lawful policy or from a sound lower producer with an attaining policy extraction procedure. An upper may come from revelation, partial gluing, conflicts, score ceilings, omitted-policy bounds, or a sound frontier relaxation.

For bounds about the same target,

\[
L_a=\max_jL_{a,j},\qquad U_a=\min_jU_{a,j}.
\]

A root action \(a^\star\) is proved optimal when

\[
L_{a^\star}\ge\max_{b\ne a^\star}U_b.
\]

Strict inequality proves uniqueness. Equality proves optimal membership but may not implement a particular tie-break rule.

If \(\widehat\rho\) is the actual executable policy, then

\[
\boxed{0\le Q^*-V(\widehat\rho)
\le\max_aU_a-L_{\widehat\rho}.}
\]

An exact root action does not automatically give an exact continuation policy. A lower used for executable regret must describe the policy that will actually be executed, including fallback behavior. [S06, S10, S13, S14]

## 13.1 The optimization obligation

A grammar restricts available policy choices. Its exact optimum is a lower for the unrestricted problem. If \(G\) is the grammar optimum and \(R\) the optimum over its genuine complement, then

\[
Q=\max(G,R).
\]

A valid upper on first off-grammar deviations can discharge the residual obligation. Merely observing that several sampled policies look similar cannot do so. [S10]

## 13.2 Root-value accuracy is not action safety

Let true values be \(Q_0=0,Q_1=1\), while optimistic estimates are \(U_0=U_1=1\). The maximum optimistic value is exactly correct. A tie rule choosing action 0 nevertheless incurs regret 1.

Therefore small aggregate root overpricing cannot certify the action selected by an upper. Use action-indexed intervals or an independently valued executable recommendation. This is an already-filed frontier correction, reinforced here by a direct finite check. [S14]

---

# 14. Frontier cuts, local gaps, and work allocation

A sound frontier interval \([\ell_C,u_C]\) propagates upward through the exact tree:

\[
[L,U]_{\mathrm{focal}}=[\max_aL_a,\max_aU_a],
\]

\[
[L,U]_{\mathrm{public}}=
[\sum_op_oL_o,\sum_op_oU_o].
\]

This is why an incomplete consequence partition can still be useful. It need not resolve every field action exactly; it needs a valid effect on the root proof.

The newest recovered frontier report is important negative evidence against a universal “fusion-free after trick 5” shortcut: uniform receipt roots and the conditioned states reached inside earlier solves behaved differently. The valid object is a region of target-bound information states, not a trick number. [S14]

If a work item can reduce a frontier upper by at most \(r_C\), propagate those allowances by `max` at focal nodes and weighted `sum` at public nodes. The resulting root quantity bounds how much that work can improve the current upper. It is a certificate of **possible leverage**, not a promise the work will realize it.

A sharper counterfactual calculation substitutes the best-case refined frontier and recomputes the root upper. This captures max switches and improvements hidden behind other unresolved branches.

The scheduler should consider joint prerequisites and small glue coalitions. An item with little isolated leverage can be necessary for a useful combination. Conversely, a large local ambiguity can be irrelevant to the current move.

---

# 15. A difference-first robustness theorem

The existing decision subspace is

\[
D=\operatorname{span}\{v_\rho-v_\pi:\rho,\pi\in\Pi\}.
\]

If \(\beta-\widetilde\beta\in D^\perp\), every policy ordering is unchanged. This is the exact decision quotient of belief within a **fixed response system**. [S13]

The following elementary extension provides an approximate version useful for unification.

Define

\[
\epsilon_D(\beta,\widetilde\beta)
=\max_{\rho,\pi\in\Pi}
\left|\langle\beta-\widetilde\beta,v_\rho-v_\pi\rangle\right|.
\]

### Theorem

If \(\widehat\rho\) is optimal under \(\widetilde\beta\), then

\[
\boxed{Q(\beta)-V_\beta(\widehat\rho)
\le\epsilon_D(\beta,\widetilde\beta).}
\]

### Proof

Choose \(\rho^*\) optimal under \(\beta\). Then

\[
\begin{aligned}
V_\beta(\rho^*)-V_\beta(\widehat\rho)
&=\langle\beta-\widetilde\beta,v_{\rho^*}-v_{\widehat\rho}\rangle\\
&\quad+\langle\widetilde\beta,v_{\rho^*}-v_{\widehat\rho}\rangle\\
&\le\epsilon_D,
\end{aligned}
\]

since the second term is nonpositive. ∎

This statement links exact difference quotients, pivotal sampling, approximate beliefs, and regret. One need not preserve every probability or absolute policy value equally well to preserve a decision.

But the maximum covers the declared full policy class. Estimating it only for discovered policies does not close the omitted-policy problem. Nor is a small value guaranteed. The theorem supplies the right error quantity, not a cheap general estimator.

The companion checks the bound for every nonempty Boolean response family on three atoms, every ordered pair of beliefs on the denominator-four simplex grid, and every tied maximizer: 57,375 family/belief-pair cases.

A further caveat matters: if changing \(\beta\) also reconstructs the modeled field, then the response vectors change. That is a different comparison requiring an additional field-transfer bound.

---

# 16. Three independent obligations before calling an answer exact

It is useful to carry three explicit obligations with every result.

**Measure.** Are the represented masses the intended belief? If sampled, what risk guarantee covers them? Are random tapes, behavioral types, and correlations represented correctly?

**Response.** Does execution of the fixed candidate or bound respect the actual rules, field identity, utility, and information restrictions? A perfect-information maximization is not fixed-policy evaluation.

**Optimization.** Have all omitted lawful policies been covered or bounded? Exact masses and exact candidate responses do not alone prove an unrestricted optimum.

These are independent of a fourth, empirical question: whether the declared model is useful against actual people or programs.

A player can be lawful but approximate, exact for a sampled empirical belief, exact for a frozen library, probabilistically certified, exactly optimal for a fixed field, or optimal for an augmented model belief. Those are different accomplishments. Naming them precisely does not devalue any of them.

---

# 17. What has survived, failed, and remains open

| Idea or claim | Review disposition |
|---|---|
| Capacity-cell support and canonical reduction | Foundational asset; proof index records formal results. Not a belief representation by itself. |
| Information-consistent fixed-field best response | Correct semantic core; use one action per actual focal information state. |
| Scheme/Fix and transported roles | Useful relational/query and certificate vocabulary; existential semantics and identity transport must remain explicit. |
| Count-free structure followed by revaluation | Valid with sufficient preserved outcome data and valuation transport; one old optimum or expected score alone is insufficient. |
| Universal small opening structural quotient | Refuted for the declared structural equivalence; not a refutation of every decision-specific reduction. |
| Universal low linear predictive rank | Refuted/theoretically blocked under the declared observation closures; nonlinear circuit complexity not settled. |
| Retrograde quotient as a first-build accelerator | Negative in reported comparisons; reuse/storage remains a different purpose. |
| Sparse policy frontiers | Strong small-coordinate observations, with important capped cases; no opening-scale theorem. |
| Signed pivotal comparison | Exact identity for frozen policy pairs; does not justify reoptimized block-sign racing. |
| Exact counted posterior factors | Sound under explicit local-likelihood/prior assumptions; recursive computational economy still needs measurement. |
| Salvation/conflict duality | Exact finite theorem; efficient discovery of useful conflicts remains open. |
| Partial information gluing | Valid upper-bound hierarchy; complementary glues and optimal-face compatibility matter. |
| Persistent model belief | Same finite core on a larger latent domain; type realism and computational cost unresolved. |
| Uniform fusion horizon | Not justified; conditioned-state counterevidence requires target-local certificates. |
| Persistent proof state and action intervals | Coherent integration point for all sound producers. |
| Small early-game decision proofs | Central surviving hypothesis; not established by current unification. |
| Equilibrium or monotone best-response improvement | Outside the established core; do not infer from deeper reasoning or cycles. |

---

# 18. The strongest research program now

Do not start another player or another universal compression scheme merely to unify the vocabulary. Zora already names the generic response mathematics; Walt is its 42 decision system. The next improvement should connect existing authorities through one target-bound proof object.

## Experiment A: decision-proof complexity, not solved-value similarity

Use a predeclared corpus stratified by early/middle/late play, bidder/defender role, contract, trump regime, and posterior-conditioning type. Include actual conditioned successors, not only synthetic uniform roots. Keep training/mining roots separate from held-out evaluation.

For roots that can be independently solved, obtain exact references. For larger roots, report validated lower/upper intervals rather than manufacturing ground truth.

Measure: executable lower; every action upper; certified regret; root survivors; policy columns used; active glues/conflicts and their order; contraction/classification/search cost; bound construction cost; and total work to reach each goal.

**Success:** on held-out early roots, the method closes decisions or useful regret bands before reproducing most of the expensive full computation.

**Failure:** useful bounds remain broad until nearly full expansion, or preprocessing/classification costs exceed the work avoided. That falsifies the tested economic route, not the finite response identities.

## Experiment B: threshold-relevant structure versus exact field reproduction

Hold the target fixed. Compare refinement aimed at reproducing the field's exact action on every hand with refinement aimed at action-indexed score ceilings, salvation masks, and root decision debt.

**Success:** the threshold-relevant proof stabilizes while many exact field actions remain unresolved.

**Failure:** the threshold structure fragments just as severely, or its verification costs erase the gain.

This directly tests the most important escape from the observed singleton tail. It does not assume that a small score alphabet makes the answer simple.

## Experiment C: low-order conflict and policy coverage

Start with cheap lawful policies and cheap information relaxations, not a hidden full exact solve used to manufacture the initial witness. Add verified conflict/mask/glue information. Allow bounded-size glue coalitions.

**Success:** a small set of held-out-valid policy witnesses and low-order constraints closes much of the gap.

**Failure:** high-order conflicts or large policy sets are routinely essential, or proving each conflict costs nearly the original solve.

The distinction between certificate size and certificate discovery cost is part of the experiment, not a footnote.

---

# 19. Verification performed for this review

Run:

```sh
python verify_unification.py
```

The recorded result is PASS. The program uses no network, third-party package, floating-point arithmetic, or project implementation.

It checks all 255 nonempty Boolean response families on three atoms, under all 27 weights in \(\{0,1,2\}^3\): 6,885 weighted cases. The checks include maximum saved-face value, blocker equivalence, exact minimum lost-transversal mass, doom decomposition, the common-optimizer criterion with zero-weight atoms, and signature-refinement endpoints/monotonicity.

It separately checks the belief-difference regret theorem on 57,375 family/belief-pair cases, factor posterior regrouping on 90 six-piece covers and eight histories, and counterexamples for strategy fusion, raw-envelope set containment, scalar root-value versus action safety, block sign versus expectation, high-order conflicts, complementary glues, joint revelation interactions, score expectation versus contract probability, nonmonotone threshold fusion gaps, and type persistence.

These checks support the finite mathematical audit and provide runnable regressions. They are not kernel proofs, not tests of the real rules implementation, and not a validation of gameplay strength or scalability.

---

# 20. Final assessment

The unified object is now clear enough to be useful:

\[
\boxed{
\text{lawful policy responses}
+\text{counted belief}
+\text{information-compatible bounds}
\longrightarrow
\text{certified action or executable regret}.
}
\]

The foundational work tells us what is possible. The belief layer tells us how much each possibility matters. The response layer tells us what a common policy can accomplish. The conflict/gluing layer tells us why individually attractive possibilities may be incompatible. Sampling estimates selected comparisons. Exact counting and structural proofs discharge parts of them. The persistent proof state assembles the evidence without pretending that unfinished work has been completed.

The dream does not require every world to become “the same.” It requires a way to stop distinguishing worlds and policies **once their remaining distinctions cannot defeat the decision certificate**.

That is a coherent target, a falsifiable target, and a target that continues to support a useful math-core player even where exact proof closure is expensive.

---

# Source register

Source keys preserve provenance without assuming access to a particular chat renderer. “Reviewed” does not mean every line or every imported claim was re-adjudicated.

| Key | Source and relevant location | Status used here |
|---|---|---|
| S01 | `jasonyandell/texas-42`, `lean/README.md`; Cells, Reduction, NormalForm, Belief, Strategic, Witness module entries | Connected repository proof index inspected; no rebuild |
| S02 | Same repository, `lean/PROOFS.md`; hard rules and definitions trust surface | Connected repository discipline inspected |
| S03 | Same repository, `wiki/game-of-42.md` and `QUICKSTART.md`; support, belief, reachability, evidence tiers | Repository synthesis; attached qualifications retained |
| S04 | Same repository, `wiki/walt-negative-results.md`; structural, predictive, performance, and capped-probe negatives | Exploratory, declared-scope negative index |
| S05 | `MATHEMATICS-OF-WALT-v0.1.md`, 2026-08-24; Parts II–VIII and formal/open-boundary sections | Exploratory unified mathematics |
| S06 | `straight_42_decision_sparse_second_audit_v0.1.md`, 2026-08-13; §§1–4 | Exploratory audit with repaired theorem statements |
| S07 | `straight_42_math_fluent_implementers_guide_v0.1.md`; roles, outcome interface, additive gauge and stabilizer boundaries | Prose mathematical/interface contracts |
| S08 | `straight_42_scheme_parametric_model_v0.3.md` and `_v0.3_amendment_A.md`, 2026-08-09; lambda/control-descriptor experiments | Reported finite experiments, not rerun |
| S09 | `DESIGN-walt-counted-belief-sandwich-v0.1.md`; §§18–23 and grammar/refinement boundaries | Factorization proofs and design |
| S10 | `DESIGN-walt-residual-bellman-calculus-v0.1.md`; grammar complement, partial-field Bellman and compiled-field transfer | Prose mathematics and reported small verification |
| S11 | `DESIGN-walt-anytime-proof-state-and-score-calculus-v0.1.md`, 2026-08-31; executive ruling and score/contract distinctions | Exploratory design and reported implementation history |
| S12 | `THEORY-walt-model-belief-base-player-v0.1.md`, 2026-09-01; persistent types, local factors, residual support | Exploratory theory; inspected-source basis stated in original |
| S13 | `ZORA-PROOF-ASSISTANT-PACKET-v0.1.md`; finite response algebra, salvation/blocker, signatures, Bellman geometry, decision quotient, model lift, conjectures | Proof targets and finite arguments, not assumed completed formalization |
| S14 | `DESIGN-walt-decision-safe-frontier-geometry-v0.1.md`; Parts I–VI, based on PR #87 head `62abe02841100ff3ad6ef9eab8a5f220629b4fec` | Reported conditioned-frontier measurements and prose interval proofs |
| S15 | `HANDOFF-walt-correctness-first-repair-and-unification-v0.1.md`, 2026-08-19; §3 repairs R1–R7 | Prior audit of frozen-policy/block-racer/statistical distinctions |
| E01 | Ross, Pineau, Paquet, Chaib-draa, *Online Planning Algorithms for POMDPs*, JAIR 32 (2008), pp. 663–704; especially pp. 667–668 | Primary research source for general POMDP value geometry and bounds |
| E02 | Howard, Ramdas, McAuliffe, Sekhon, *Time-uniform, nonparametric, nonasymptotic confidence sequences*, arXiv:1810.08240, HTML v9; Introduction | Primary research source for time-uniform confidence coverage |

## Carry-forward obligations

The next revision should attach actual theorem names/build receipts where formalization has advanced beyond the inspected packet; include exact independent references for any new 42 corpus measurements; resolve any disputed mathematical statements without deleting their provenance; and keep the universal finite theorems separate from claims of economical certificate discovery.
