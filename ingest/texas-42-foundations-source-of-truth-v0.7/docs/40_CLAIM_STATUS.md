# Claim Status Ledger

## 0. Status vocabulary

| Status | Meaning |
|---|---|
| DEFINITION | Introduces an object, domain, or notation. |
| ADOPTED RULE | Normative for the Straight 42 profile. |
| CLARIFICATION | A precise consequence of adopted rules or definitions. |
| THEOREM — proved | A mathematical proof is supplied. |
| LEMMA — proved | A supporting mathematical proof is supplied. |
| THEOREM — finite verification | An exact program exhausts the stated finite domain. |
| FINITE VERIFICATION RECEIPT | An exact program checks a named finite corpus without claiming that the corpus exhausts the surrounding domain. |
| PROPOSITION | Proved under explicit additional assumptions. |
| COROLLARY — proved | A proved consequence of preceding definitions or theorems. |
| COROLLARY / SYNTHESIS | Organizes preceding results without claiming a new independent theorem. |
| COUNTEREXAMPLE | A concrete witness refutes an overstrong claim. |
| BOUNDARY | States where a theorem, model, or domain stops. |
| CONJECTURE | A precise statement proposed without proof. |
| UNRESOLVED | Not established; no stronger claim is made. |

Implementation agreement, speed, usefulness, and empirical performance are not
mathematical statuses. A row may join statuses with `/` when one statement
intentionally contains, for example, both a proved result and its scope
boundary.

## 1. Scope and normative rules

| ID | Claim | Exact statement | Assumptions | Status | Source |
|---|---|---|---|---|---|
| SCP-01 | Primary object | The complete profile is straight points-and-marks Texas 42. | Scope. | DEFINITION | Scope §2 |
| SCP-02 | Full hand | A contracted hand contains seven tricks and 28 plays. | Straight contracted hand. | ADOPTED RULE | Rules §§6–7 |
| SCP-03 | Special contracts excluded | Nello, plunge, splash, sevens, and related changes are outside the current object. | None. | DEFINITION / BOUNDARY | Scope §3 |
| CFG-01 | Finite mark cap | \(m_{\max}\) is a configured positive finite integer; no value is privileged. | Match configuration. | ADOPTED RULE | Rules §1 |
| CFG-02 | Match target | \(T\) is a configured positive finite integer; customary value 7. | Match configuration. | ADOPTED RULE | Rules §1 |
| CFG-03 | Reachable bid ceiling | The auction can reach at most \(\min(m_{\max},5)\) marks; every cap at least five induces the same legal auction tree. | Four one-time actions; opening ceiling two; +1 mark raises. | THEOREM — proved | Rules §3.5; Math §4.3 |
| RUL-01 | All-pass | Four passes abandon the deal, advance shaker, and begin a fresh deal attempt. | Current profile. | ADOPTED RULE | Rules §3.4 |
| RUL-02 | Baseline deal law | Ordered labeled deals are uniform. | Selected chance profile. | ADOPTED RULE | Rules §2 |
| RUL-03 | Cross-deal law | Conditional on pre-attempt history and non-deal latent state, each new deal is an independent uniform ordered deal under the baseline profile. | Selected chance profile. | ADOPTED RULE | Rules §2 |
| RUL-04 | Legal-game boundary | Illegal play and tournament adjudication are outside the legal transition graph. | Abstract game. | BOUNDARY | Rules §11 |
| RUL-05 | Match-global recall | A player remembers every privately observed hand and complete public history across redeals. | Observation rules. | ADOPTED RULE | Rules §10 |
| RUL-06 | Primitive/derived public record | Bids, declarations, and actor-attributed plays are primitive public events; winner, score, settlement, and similar facts are deterministic derivations. | Legal rules and configuration fixed. | DEFINITION / CLARIFICATION | Rules §10; Math §6.1 |

## 2. Finite declaration algebra

| ID | Claim | Exact statement | Assumptions | Status | Source |
|---|---|---|---|---|---|
| ALG-01 | Double-six cardinality | \(\lvert\mathcal D\rvert=28\). | Two-element multisets over seven pips. | THEOREM — proved | Math §2.1 |
| ALG-02 | Natural covering | Each natural incidence has size 7; doubles have one membership and mixed tiles two. | Natural incidence definition. | THEOREM — proved | Math §2.2 |
| ALG-03 | Pair intersection | \(\sigma_p\cap\sigma_q=\{p:q\}\) for distinct pips. | Natural incidence definition. | THEOREM — proved | Math §2.2 |
| ALG-04 | Count total | Count labels sum to 35. | Straight count table. | THEOREM — proved | Math §2.4 |
| ALG-05 | Straight declaration count | There are seven pip-trump layers, doubles-trump, and no-trump. | Straight scope. | DEFINITION | Math §3.1 |
| ALG-06 | Called absorption | A called domino belongs only to effective suit 7. | Declaration definitions. | THEOREM — proved | Math §3.3 |
| ALG-07 | Effective covering | Every tile has one or two effective memberships. | Straight declarations. | THEOREM — proved | Math §3.3 |
| ALG-08 | Follow exactness | \(F_\delta(d,q)\) is exactly effective-suit membership. | Straight follow rule. | DEFINITION / ADOPTED RULE | Math §3.4 |
| ALG-09 | Total declaration rank | Rank is total and used only in nonzero tiers. | Rank definition. | DEFINITION | Math §3.5 |
| ALG-10 | Lead nonzero tier | The lead is always tier 1 or 2. | Straight declaration rules. | THEOREM — proved | Math §3.6 |
| ALG-11 | Unique winner | Every four-distinct-domino trick with a designated lead has one unique winner. | Straight declarations. | THEOREM — proved | Math §3.6 |
| ALG-12 | Independent winner receipt | All 737,100 declaration/lead/three-subset cases have one algebraic maximum and agree with a separately coded prose-rule resolver. | Exact finite domain. | THEOREM — finite verification | Math §3.6; verification output |
| ALG-13 | BEATS exactness | A later tile takes current lead iff it lies in contextual `BEATS`. | Trick-key definition. | THEOREM — proved | Math §3.7 |
| ALG-14 | Threat monotonicity | Shrinking the live external set cannot add live threats. | Fixed declaration and tile. | THEOREM — proved | Math §3.7 |
| ALG-15 | Threat incompleteness | Equal when-led threat sets need not imply equal follow relations. | No-trump `0-0`/`1-1` witness. | COUNTEREXAMPLE | Math §3.7 |
| ALG-16 | Declaration selection | Declaration selects one relational layer for stable physical nodes. | Declaration bundle. | COROLLARY / SYNTHESIS | Math §3.8 |
| ALG-17 | Count-preserving pip classification | Only identity and swap \(2\leftrightarrow3\) preserve all count labels. | Pip endpoint permutations. | THEOREM — proved / THEOREM — finite verification | Math §3.9; verifier |
| ALG-18 | Scoped pip order transport | Swap \(2\leftrightarrow3\) preserves the game-semantic order structure exactly for declarations 2 and 3. | Count, suit, follow, led-context, and comparison transport. | THEOREM — proved / THEOREM — finite verification | Math §3.9; verifier |
| ALG-19 | Transport boundary | The swap does not preserve literal numeric ranks and fails every other Straight layer; the classification concerns count-preserving pip transports, not all abstract automorphisms. | Stated transport class. | THEOREM — proved / BOUNDARY | Math §3.9 |

## 3. Deal, auction, and contract

| ID | Claim | Exact statement | Assumptions | Status | Source |
|---|---|---|---|---|---|
| DEAL-01 | Ordered deal count | There are \(28!/(7!)^4\) labeled four-seat deals. | Seven tiles per labeled seat. | THEOREM — proved | Math §4.1 |
| DEAL-02 | Conditional assignment count | One known hand leaves \(21!/(7!)^3\) unconstrained assignments. | No further observations. | THEOREM — proved | Math §4.1 |
| DEAL-03 | Ideal shuffle proposition | Ideal Fisher–Yates plus slicing yields uniform ordered deals. | Independent uniform shuffle choices. | PROPOSITION | Math §4.2 |
| AUC-01 | Auction order | Left of shaker begins; each seat acts once clockwise. | Straight rules. | ADOPTED RULE | Rules §3.1 |
| AUC-02 | Point range | Point bids are 30 through 41. | Straight rules. | ADOPTED RULE | Rules §3.2 |
| AUC-03 | Opening mark ceiling | Before any mark bid, entry may be 1 or 2 marks subject to cap. | Straight rules. | ADOPTED RULE | Rules §3.3 |
| AUC-04 | Mark progression | After \(M(r)\), the only mark overcall is \(M(r+1)\), subject to cap. | Straight rules. | ADOPTED RULE | Rules §3.3 |
| AUC-05 | Attempt finiteness | One auction attempt is finite because it has depth four and every reachable node is finitely branching; a globally finite mark domain is not required for this theorem. | One-round progression rules. | THEOREM — proved | Math §4.3 |
| AUC-05A | Exact auction census | Terminal-history counts for caps 1–7 are 2380, 3060, 3196, 3213, 3214, 3214, 3214; reached maxima are 1,2,3,4,5,5,5. | Exact four-action enumeration. | THEOREM — finite verification | Math §4.3; verification output |
| AUC-06 | Pass-out unboundedness | Repeated all-pass deal attempts have no deterministic finite bound. | Reshake-next rule. | BOUNDARY | Math §4.3 |
| AUC-06A | Probabilistic contraction bound | If every attempt contracts with conditional probability at least \(\varepsilon>0\), attempts to the next contract have geometric tail bound and mean at most \(1/\varepsilon\); match expected attempts are at most \((2T-1)/\varepsilon\). | Uniform conditional lower bound. | PROPOSITION | Math §4.3 |
| AUC-07 | Straight action support | Bid and declaration legality impose no private-hand feasibility predicate. | No special-contract eligibility. | THEOREM — proved | Math §4.4 |
| AUC-08 | Auction/declaration evidence | Hand-sensitive field likelihood can reweight deals without changing rule support. | Chosen field model. | THEOREM — proved | Math §§4.4, 8 |
| K-01 | Point contract | \(P(n)\) has threshold \(n\) and stake 1. | Straight rules. | ADOPTED RULE | Math §4.5 |
| K-02 | Mark contract | \(M(m)\) has threshold 42 and stake \(m\). | Straight rules. | ADOPTED RULE | Math §4.5 |
| K-03 | Deterministic settlement | Contract and final declaring points determine receiver and amount. | Straight contract. | THEOREM — proved | Math §4.5 |
| K-04 | Mark threshold/sweep equivalence | In full play, declaring points equal 42 iff the declaring partnership wins all seven tricks. | Every trick has one base point; all 42 points awarded. | THEOREM — proved / CLARIFICATION | Rules §8; Math §4.5 |

## 4. Objective physical game

| ID | Claim | Exact statement | Assumptions | Status | Source |
|---|---|---|---|---|---|
| PLAY-01 | Location conservation | Every domino occupies one global location; play relocates it. | Full location state. | THEOREM — proved | Math §5.1 |
| PLAY-02 | Full/reduced distinction | Full location state and reduced contracted-play state are distinct exact objects. | Defined domains. | DEFINITION / BOUNDARY | Math §§5.1–5.2 |
| PLAY-03 | Match residue includes shaker | Shaker is required for continuation after hand settlement or pass-out. | Match continuation modeled. | THEOREM — proved | Math §§5.2, 5.5 |
| PLAY-04 | Exact legal set | Lead allows any held tile; otherwise followers if present, else any. | Straight follow-if-possible. | THEOREM — proved | Math §5.3 |
| PLAY-05 | One node spent | Every legal play removes one tile from the actor's remaining hand. | Legal play. | THEOREM — proved | Math §5.4 |
| PLAY-06 | Contracted physical Markov state | Reduced state determines within-hand physical continuation. | All defined fields retained. | THEOREM — proved | Math §5.5 |
| PLAY-07 | Match physical Markov state | Reduced play plus match residue determines settlement, next shaker, and next objective deal chance transition under the baseline independent-deal law. | Baseline chance law; match rules. | COROLLARY / BOUNDARY | Math §5.5 |
| PLAY-08 | Grade descent | Total remaining hand size falls by one each play. | Full play. | THEOREM — proved | Math §5.6 |
| PLAY-09 | Seven tricks | Full play has exactly 28 plays and seven tricks. | Four seats, seven tiles each. | THEOREM — proved | Math §5.6 |
| PLAY-10 | Score total | Final partnership hand points sum to 42. | Full play. | THEOREM — proved | Math §5.6 |
| PLAY-11 | Finite play DAG | The contracted-play graph is finite and acyclic. | Full play. | THEOREM — proved | Math §5.6 |
| ORC-01 | Backward-induction constructibility | Named finite node operators define exact recursive values on the complete-information history tree. | Fixed deal, contract, utility, operators. | THEOREM — proved | Math §5.7 |
| ORC-02 | State-DAG quotient scope | Memoization by reduced state is exact only when utility residue and node operators factor through that state. | Explicit congruence. | COROLLARY / BOUNDARY | Math §5.7 |
| ORC-03 | Oracle semantics required | “Oracle” is underspecified without utility, actor/node operators, exact value representation, and any claimed history-to-state quotient. | None. | BOUNDARY | Math §5.7 |
| ORC-04 | Effective exact backward induction | Finite recursion is an exact terminating computation when rewards and node operators are effectively computable in a closed exact representation class. | Effective computability and closure. | COROLLARY — proved | Math §5.7 |
| ORC-05 | Finiteness/computability boundary | A finite tree with mathematically defined values need not provide an algorithm for noncomputable utilities or operators. | No effectiveness assumption. | BOUNDARY | Math §5.7 |
| MATCH-01 | Contracted-hand bound | A 0–0 match to target \(T\) ends within \(2T-1\) contracted hands. | Each contracted hand awards at least one mark. | THEOREM — proved | Math §5.8 |
| MATCH-02 | Full-match horizon boundary | Repeated pass-outs prevent a deterministic finite-horizon bound on the full pre-contract match process. | Reshake-next profile. | BOUNDARY | Math §§4.3, 14 |

## 5. Information and world domains

| ID | Claim | Exact statement | Assumptions | Status | Source |
|---|---|---|---|---|---|
| INFO-01 | Deal-local information | Current dealt hand plus current public prefix is safe for an isolated deal or conditional on fixed pre-deal private record. | Stated scope. | DEFINITION | Math §6.2 |
| INFO-02 | Match-global information | Ordered private hand observations plus full public history form the safe match record. | Observation model. | DEFINITION | Math §6.2 |
| INFO-03 | Perfect recall | The deal-local record is perfect-recall only under isolated/fixed-prehistory conditioning; the match-global record is perfect-recall for the full match. | Explicit scopes. | THEOREM — proved / BOUNDARY | Math §6.2 |
| INFO-03A | Derived public facts | Materializing deterministic public consequences adds no information and must remain validated against the primitive stream. | Rules/configuration fixed. | COROLLARY / BOUNDARY | Math §6.1 |
| INFO-04 | Current-deal support | \(\Omega_{r,t}^m(I_{r,t}^{m,\mathrm{deal}})\) is rule support on complete deals for the current attempt, not whole match histories. | One current deal. | DEFINITION / BOUNDARY | Math §6.3 |
| INFO-05 | Remainder map | \(\rho_{r,t}\) maps a complete current-attempt deal to current hidden remaining hands. | Actor-attributed current-deal history. | DEFINITION | Math §6.3 |
| INFO-06 | Domain distinction | A complete deal and a current remainder assignment are different types. | Definitions. | BOUNDARY | Math §6.3 |
| INFO-07 | Mechanical projection | A mechanical state retains exact physical/support residue for a named scope. | Listed reconstruction properties. | DEFINITION | Math §6.4 |
| INFO-08 | Objective congruence | Equal reduced objective states have equal physical continuations. | Complete reduced fields. | THEOREM — proved | Math §6.5 |
| INFO-09 | Viewer bundle | Mechanical state plus one current remainder reconstructs one reduced contracted-play state; full location history may require more public attribution. | Exact reconstruction fields. | THEOREM — proved / BOUNDARY | Math §6.5 |
| INFO-10 | Mechanical state not original information state | Distinct remembered bid paths can share a mechanical endpoint. | Path omitted. | COUNTEREXAMPLE | Math §6.6 |
| INFO-11 | Support not belief | Equal compatibility sets need not imply equal weights. | Discretionary action evidence. | THEOREM — proved | Math §6.7 |
| INFO-12 | Rule/chance support boundary | A chance law with structural zeros may produce physical belief support strictly smaller than the exact rule fiber. | Non-full-support chance law. | BOUNDARY | Math §6.7 |

## 6. Cells, exact support, minimality, and reachability

| ID | Claim | Exact statement | Assumptions | Status | Source |
|---|---|---|---|---|---|
| CELL-01 | Cell dependence | Possible-holder cells are coupled by capacities, disjointness, and conservation. | Shared unseen pool. | COUNTEREXAMPLE | Math §7.2 |
| CELL-02 | Remainder fiber | \(\Phi(\mathbf C)\) is the constrained set of current hidden hands. | Cell definitions. | DEFINITION | Math §7.3 |
| CELL-02A | Fiber factors through cells | \(\Phi(c)=\Phi(\mathbf C(c))\); equal cell systems give equal current-remainder support even when other mechanical fields differ. | Straight cell schema. | DEFINITION / COROLLARY — proved | Math §7.3 |
| CELL-03 | Fiber intensional | Enumeration is a query on the fiber, not its definition. | None. | DEFINITION | Math §7.3 |
| CELL-04 | Initial Straight cells | Before play, every hidden seat may hold every unseen tile with capacity 7. | Straight auction/declaration. | THEOREM — proved | Math §7.4 |
| CELL-05 | Exact cell losslessness | \(\Phi(c_{r,t})=\rho_{r,t}(\Omega_{r,t}(I_{r,t}^{\mathrm{deal}}))\) for legal Straight play prefixes in scope. | Explicit theorem scope. | THEOREM — proved | Math §7.5 |
| CELL-06 | No surviving positive clause | Successful follow evidence is carried by the removed played tile; no positive follower lower bound remains. | Straight play. | THEOREM — proved | Math §7.5 |
| CELL-07 | Fixed-history bijection | Current remainder assignments and compatible initial deals are bijective for fixed actor-attributed history. | Cell theorem scope. | COROLLARY — proved | Math §7.5 |
| CELL-07A | Reachable support parity corpus | Exact cell-fiber equality holds on 972 deterministic reachable prefixes spanning all nine declarations; 970 contain public voids. | Named verifier corpus, plays 20–28. | FINITE VERIFICATION RECEIPT | Math §7.5; verification output |
| CELL-08 | Cell theorem scope | Special contracts, hand eligibility, sitting out, draw/exchange, and private observations are excluded. | None. | BOUNDARY | Math §7.6 |
| CELL-09 | Hall feasibility | Seat-subset union inequalities characterize nonempty capacity fibers. | Finite cells, total capacity equals pool. | THEOREM — proved | Math §7.7 |
| CELL-10 | Hall finite receipt | 66,968 tiny three-seat systems agree with direct enumeration. | Universe size at most 4. | THEOREM — finite verification | Math §7.7; verification output |
| CELL-10A | Exact fiber-count coefficient | Fiber cardinality is the coefficient of \(\prod_sx_s^{k_s}\) in \(\prod_{d\in U}\sum_{s:d\in P_s}x_s\). | Finite cell system. | THEOREM — proved | Math §7.8 |
| CELL-10B | Exact deletion recurrence | Fiber count partitions exactly by the holder of any selected unseen tile. | Finite cell system. | COROLLARY — proved | Math §7.8 |
| CELL-10C | Fiber-count finite receipt | Direct assignment count, polynomial coefficient DP, and deletion recurrence agree on all 66,968 tiny systems. | Universe size at most 4. | THEOREM — finite verification | Math §7.8; verification output |
| CELL-10D | Support does not select probability | A nontrivial finite fiber admits multiple full-support normalized measures; a sampler needs an explicit law. | Fiber size at least two. | THEOREM — proved / BOUNDARY | Math §7.8 |
| CELL-10E | Uniform holder marginals | Under an explicitly selected uniform fiber law, \(\Pr(d\in H_s)=N(\mathbf C^{d\to s})/N(\mathbf C)\). | Nonempty finite fiber; uniform law selected. | THEOREM — proved | Math §7.8 |
| CELL-10F | Exact count-ratio sampler | Sequential holder choices weighted by successor fiber counts produce the exact uniform fiber law. | Exact counts and exact rational random choice. | THEOREM — proved / BOUNDARY | Math §7.8 |
| CELL-10G | Uniform-sampler finite receipt | The count-ratio recursion assigns every checked world probability \(1/N(\mathbf C)\), with 22,620 world probabilities checked across the 66,968 tiny systems. | Universe size at most 4. | THEOREM — finite verification | Math §7.8; verification output |
| CELL-10H | Exact capacity dynamic program | Occupancy-vector DP computes unrestricted fiber count exactly; under structural conservation, each bounded occupancy vector lies in one unique layer, giving at most \(\prod_s(k_s+1)\) states total, \(\lvert J\rvert(\prod_s(k_s+1)-1)\) candidate-holder checks, and \(\sum_s k_s\prod_{r\ne s}(k_r+1)\) capacity-eligible updates. | Finite cell system; resource bounds require \(\lvert U\rvert=\sum_sk_s\). | THEOREM — proved | Math §7.8 |
| CELL-10I | Native count bound | A native unrestricted fiber has at most 399,072,960 worlds; one exact count visits at most 512 occupancy states total, performs at most 1,533 candidate-holder checks and 1,344 capacity-eligible updates, and has at most 48 live states in one layer. | Three hidden seats, \(\lvert U\rvert=\sum_sk_s\le21\), \(k_s\le7\). | THEOREM — proved | Math §7.8 |
| CELL-10I1 | Native DP profile receipt | The instrumented unrestricted DP attains the derived state/check/update formulas on all 512 triples in \(\{0,\ldots,7\}^3\), with the stated maxima and exact 22-layer sequence at \((7,7,7)\). | Every holder edge allowed; three hidden seats. | THEOREM — finite verification | Math §7.8; verification output |
| CELL-10J | Local allowance not marginal possibility | A tile can lie in a seat's local possible set while no globally conserved fiber world assigns it there. | Explicit two-tile witness. | COUNTEREXAMPLE | Math §7.9 |
| CELL-10K | Exact holder-edge criterion | \(d\) is genuinely possible at seat \(s\) iff the forced successor \(\mathbf C^{d\to s}\) is Hall-feasible. | Finite cell system. | THEOREM — proved | Math §7.9 |
| CELL-10L | Canonical fixed-schema reduction | Replacing local possible sets by actual marginal holder supports preserves the fiber and is the unique coordinatewise least representative with fixed pool, capacities, and schema. | Fixed cell schema. | THEOREM — proved / BOUNDARY | Math §7.9 |
| CELL-10L1 | Reduction operator laws | The canonical reduction is contractive, monotone, and idempotent; fixed-schema systems denote the same fiber iff their reductions are equal. | Same pool and capacities for comparisons. | THEOREM — proved | Math §7.9 |
| CELL-10M | Fixed-schema reduction receipt | World projection and forced-successor Hall tests agree on 785,736 holder edges; reduction preserves each of 66,968 tiny-system fibers and satisfies idempotence and normal-form equivalence there. | Universe size at most 4. | THEOREM — finite verification | Math §7.9; verification output |
| CELL-10N | Reduction not transition-stable | A support-reduced predecessor can have a raw exact successor with newly unsupported holder edges; remaining reduced requires reducing again after transition. | Explicit three-tile witness. | COUNTEREXAMPLE / BOUNDARY | Math §7.9 |
| CELL-11 | Nonempty support recovery | A nonempty fiber uniquely recovers pool, labeled capacities, and marginal holder relation. | Three labeled hidden seats; nonempty fiber. | THEOREM — proved | Math §7.10 |
| CELL-12 | Active-seat trichotomy | After extracting certain tiles, an ambiguity component has 0, 2, or 3 active seats; binary tiles admit both active holders, ternary tiles admit all three or exclude exactly one. | Three hidden seats; support-reduced nonempty fiber. | LEMMA — proved | Math §7.10 |
| CELL-13 | Exact minimized feasible payload | Certain locations plus determinate/binary/ternary ambiguity data are in bijection with nonempty exact support fibers. | Native three-seat schema. | THEOREM — proved | Math §7.10 |
| CELL-14 | Global representation-minimal quotient | The total normal form, with one shared `Empty` state and one canonical feasible payload per nonempty fiber, is exactly the quotient by support equality; every exact deterministic support representation factors onto it. | Ordinary extensional empty-support semantics. | THEOREM — proved | Math §7.10 |
| CELL-15 | One-assignment SCC compiler | One feasible assignment plus one SCC pass recovers every marginal holder edge and the exact support normal form. | Feasible finite cell system. | THEOREM — proved | Math §7.10 |
| CELL-16 | Reachable witness/tag erasure | A certified objective state supplies a feasible current remainder witness; the witness and empty/feasible tag are not player-facing semantic state. | Legal reachable state; witness kept internal. | COROLLARY — proved | Math §7.10 |
| CELL-17 | Zero supplemental support state | If the containing mechanical coordinate retains every support-generating field, cells, fiber, and normal form are deterministic views and add zero semantic bits. | Exact containing coordinate. | COROLLARY — proved / BOUNDARY | Math §7.10 |
| CELL-18 | Strict Hall ambiguity | Every nonempty proper active-seat subset has at least one unit of Hall slack; the ambiguity graph has no nontrivial Hall-tight component. | Reduced ambiguity component. | THEOREM — proved | Math §7.11 |
| CELL-19 | Every ternary exclusion is essential | Removing any one stored ternary exclusion strictly enlarges the fiber. | Reduced ternary ambiguity. | THEOREM — proved | Math §7.11 |
| CELL-20 | Linear ternary validator | A ternary payload is nonempty and reduced iff capacities are positive, conserved, and \(n-n_s\ge r_s+1\) for all seats. | Three-seat ternary schema. | COROLLARY — proved | Math §7.11 |
| CELL-21 | Complete six-integer count signature | Ternary counting depends only on \((r_0,n_0,r_1,n_1,r_2,n_2)\), with unrestricted-category size derived. | Native ternary support. | THEOREM — proved | Math §7.12.1 |
| CELL-22 | Exact category-role symmetry | Simultaneous permutation of seats and their excluded-seat categories is the full structural automorphism group \(S_3\). | Ternary category incidence. | THEOREM — proved | Math §7.12.1 |
| CELL-23 | Stabilizer-orbit sampling | Matrix representatives weighted by orbit size times multinomial mass reproduce the exact labeled-world law. | Exact category sampler; canonical signature. | COROLLARY — proved | Math §7.12.1 |
| CELL-24 | Complement-elided worlds | A binary world needs one ambiguity mask and a ternary world two; the final hand is the complement. | Normal-form-local domino order. | COROLLARY — proved / BOUNDARY | Math §7.12.2 |
| CELL-25 | Minimal ordered completion automaton | Reachable and coaccessible residual-capacity vectors are the unique minimal partial deterministic acyclic automaton for a fixed ambiguous-tile order. | Fixed order and support normal form. | THEOREM — proved | Math §7.12.3 |
| CELL-25A | Universal native residual code | A native ternary residual vector has an injective nine-bit code, and nine bits are necessary across all native ternary automata. | Capacities at most seven. | COROLLARY — proved | Math §7.12.3 |
| CELL-26 | Fiber-local optimal rank | A world in a supplied fiber has an exact fixed-width rank of \(\lceil\log_2\lvert\Phi\rvert\rceil\) bits, at most 29 natively. | Fixed support and ordering. | THEOREM — proved / BOUNDARY | Math §7.12.4 |
| CELL-27 | Full-schema support census | The standalone native support quotient has 1,830,967,207,309,611,271,596,161 states, so 81 bits are necessary and sufficient. | Full native cell-schema domain, one extensional empty state. | THEOREM — proved / THEOREM — finite verification | Math §7.12.5; verification output |
| CELL-28 | Minimality verification receipt | All 66,968 tiny systems decode exactly through the total normal form; 22,620 feasible witness worlds agree with SCC compilation; 2,151 exclusions are checked essential; exact rank/unrank covers 22,620 worlds. Native signature and matrix censuses also match the stated totals. | Named finite domains. | THEOREM — finite verification | Math §7.12.5; verification output |
| REACH-01 | Reachable support image | \(\mathscr R_{\mathrm{Str}}^m\) is the image of legal Straight contracted-hand prefixes under the feasible exact support normal form. | Fixed viewer-relative hidden-seat frame. | DEFINITION | Math §7.13 |
| REACH-01A | Viewer-relative gauge | Seat rotation bijects all four viewer-indexed reachable-support domains; a viewer-relative support code stores no absolute viewer identifier. | Simultaneous rotation of complete legal witnesses. | COROLLARY — proved | Math §7.13 |
| REACH-02 | Reachable-domain minimality | Restricting to legal prefixes removes unrealized support classes but permits no merge of distinct reachable fibers; every exact deterministic reachable-support representation factors onto the restricted normal form. | Exact decoder on legal-prefix domain. | THEOREM — proved | Math §7.13 |
| REACH-03 | No reachability flag | Legal constructors preserve reachability inductively; external states require exact certification, after which a Boolean/tag or witness adds no semantic state. | Opaque certified type discipline. | COROLLARY — proved | Math §7.13 |
| REACH-03A | Reachable support is not transition-sufficient | Reachable support identifies an exact fiber but omits mechanical fields required for legal actions and successor transitions. | Standalone support identifier. | BOUNDARY | Math §7.13 |
| REACH-04 | Exact capacity profiles | Hidden capacities are reachable iff their range is at most one; exactly 50 labeled profiles occur. | Straight contracted-hand prefix. | THEOREM — proved / THEOREM — finite verification | Math §7.13.1; verification output |
| REACH-05 | Seven observable contexts | Every declaration has seven leadable contexts and lead-fiber sizes \(1,\ldots,7\); doubles-trump natural context 0 is nonempty but unleadable. | Straight declaration algebra. | THEOREM — proved / THEOREM — finite verification | Math §7.13.2; verification output |
| REACH-06 | Exact projected schedule language | A void-mask tuple is turn-schedule realizable exactly under the displayed completed-trick/current-follower criterion. | Tile identities, availability, and winner physics projected away. | THEOREM — proved / BOUNDARY | Math §7.13.3 |
| REACH-06A | Projected schedule census | Exhausting all \(8^7\) context-membership tuples gives the displayed \(A_j,T_{j,1},T_{j,2}\) counts. | Seven declaration-relative lead contexts. | THEOREM — finite verification | Math §7.13.3; verification output |
| REACH-07 | Lead-witness necessity | Every used public void context has at least one lead-fiber tile outside the current hidden pool. | Legal Straight prefix. | THEOREM — proved | Math §7.13.4 |
| REACH-08 | Exact witness criterion | A feasible support is reachable iff a valid deal, contract, legal actor-attributed prefix, and exact projection supply an accepted witness. | Straight support reachability definition. | THEOREM — proved | Math §7.13.4 |
| REACH-09 | Reachability decidable | Membership in the reachable support image is decidable by finite exhaustive witness generation. | Configured Straight profile; current contracted hand. | COROLLARY — proved / BOUNDARY | Math §7.13.4 |
| REACH-10 | Feasible is not reachable | The specified 18-tile, capacity-(6,6,6) reduced support is Hall-feasible but has no legal Straight ancestry. | Constructed support; all 450 static zero/one-context generators exhausted. | COUNTEREXAMPLE / THEOREM — finite verification | Math §7.13.5; verification output |
| REACH-11 | Reachable-support ceiling | 64,123,542,674,901 declaration-tagged necessary outer profiles cover every reachable support; hence standalone reachable support needs at most 46 bits. | Exact capacity/schedule/lead-witness outer language. | THEOREM — finite verification / BOUNDARY | Math §7.13.6; verification output |
| REACH-11A | Context-relative ceilings | Necessary-outer-profile ceilings are 43 bits with declaration supplied, 43 with capacity profile supplied, and 40 with both supplied. | Same outer language; not exact minima. | THEOREM — finite verification / BOUNDARY | Math §7.13.6; verification output |
| REACH-12 | Reachable-support floor | Four disjoint universally reachable no-void profile families contain 44,352,165 supports, so every universal standalone fixed-width code needs at least 26 bits. | Fixed viewer-relative frame. | THEOREM — proved / THEOREM — finite verification | Math §7.13.6; verification output |
| REACH-13 | Exact reachable census | The exact cardinality and optimal standalone bit count of \(\mathscr R_{\mathrm{Str}}^m\) are not closed; the proved interval is 26–46 bits. | Current theorem set. | UNRESOLVED / BOUNDARY | Math §7.13.6 |
| TRANS-01 | Hidden-action support map | Fixed-domino removal is a bijection from the legal predecessor subset onto the successor fiber. | Hidden actor; cell-theorem scope. | COROLLARY — proved | Math §7.14 |
| TRANS-02 | Viewer-action support map | Viewer play is the identity bijection on current hidden remainder worlds. | Viewer actor. | COROLLARY — proved | Math §7.14 |
| TRANS-03 | Fiber subset shorthand invalid | Pre- and post-hidden-action fibers generally have different world types. | Hidden actor. | BOUNDARY | Math §7.14 |
| TRANS-04 | Complete-deal nesting | Appending an observed legal action gives literal inclusion of compatible initial-deal sets within one attempt. | Fixed current deal domain. | THEOREM — proved | Math §7.14 |
| TRANS-05 | Typed cardinality refinement | Hidden actions cannot increase fiber cardinality; viewer actions preserve it exactly. | Cell-theorem scope. | COROLLARY — proved | Math §7.14 |
| TRANS-06 | Reachable transition corpus | The verifier checks 864 consecutive typed transitions: 648 hidden nonincrease and 216 viewer equality. | Named reachable corpus. | FINITE VERIFICATION RECEIPT | Math §7.14; verification output |
| TRANS-07 | Abstract typed-transition exhaustion | Exact predecessor-image/successor-fiber equality holds in 14,412 lead, 56,460 follow, and 56,460 slough cases over every three-seat cell system with at most three tiles. | Complete stated tiny domain. | THEOREM — finite verification | Math §7.14; verification output |
| CELL-29 | Minimality boundary | The exact support component is semantically minimal; no universal byte- or operation-minimum is claimed without a declared cost model, and the full mechanical/strategic state is not proved minimal. | Named support semantics. | BOUNDARY | Math §7.15 |

## 7. Belief and filtering

| ID | Claim | Exact statement | Assumptions | Status | Source |
|---|---|---|---|---|---|
| BEL-01 | Behavioral field | Legality indicator and normalized discretionary action kernel are distinct factors. | Behavioral field. | DEFINITION | Math §8.1 |
| BEL-02 | General history likelihood | Exact correlated behavior is represented by a joint likelihood or a sufficient augmented latent-state kernel. | Field model named. | DEFINITION / BOUNDARY | Math §8.1 |
| BEL-03 | Product-likelihood scope | A local product is valid only when its factors are the exact chain-rule conditionals; latent state paths must otherwise be integrated. | Sufficient retained state. | PROPOSITION / BOUNDARY | Math §8.1 |
| BEL-03A | Kernel/tape equivalence scope | A full-random-tape representation is equivalent to a stochastic kernel only after a measurable randomization realization is specified; conditioned tape likelihoods are zero-one. | Realization assumption. | PROPOSITION / BOUNDARY | Math §8.1 |
| BEL-04 | Current-attempt posterior | Posterior is inherited prior × current-deal rule compatibility × modeled within-attempt history likelihood on one augmented current-attempt domain. | Positive observation probability. | THEOREM — proved | Math §8.2 |
| BEL-05 | Remainder belief | Current physical belief is the pushforward of the augmented current-attempt posterior through \(\rho_{r,t}\). | Defined remainder map. | DEFINITION / THEOREM — proved | Math §8.3 |
| BEL-06 | Augmented current belief | Field state must remain coupled to world uncertainty when not separable. | Memoryful/correlated field. | DEFINITION / BOUNDARY | Math §8.3 |
| BEL-06A | Map/kernel representation | Deterministic pushforward notation requires a valid random-tape realization; otherwise field-state evolution is a Markov-kernel image. | General stochastic field. | DEFINITION / BOUNDARY | Math §§8.1, 8.6 |
| BEL-06B | Conditional-kernel factorization | On finite/countable domains every joint augmented belief factors exactly as physical marginal times conditional field-state kernel; graph and constant-state forms are special cases. | Finite/countable domain; regular conditional required in general spaces. | PROPOSITION — proved / BOUNDARY | Math §8.3 |
| BEL-07 | Uniform physics fiber | A current-deal marginal uniform conditional on the viewer hand pushes its normalized rule-only restriction to a uniform fixed-history fiber. | Cell theorem scope; no action likelihood. | THEOREM — proved | Math §8.4 |
| BEL-07A | Physics-only holder marginals | Under the uniform-physics assumptions, a tile's hidden-seat probability is the corresponding successor fiber count divided by current fiber count, and count-ratio sampling is exact. | BEL-07 assumptions. | COROLLARY — proved | Math §§7.8, 8.4 |
| BEL-08 | Exponential tilt | Posterior equals normalized rule-only belief times history likelihood, or its Radon–Nikodym analogue. | Positive denominator and required measurability. | THEOREM — proved | Math §8.5 |
| BEL-09 | Within-attempt filter | A public action reweights augmented current-attempt worlds without changing the current initial deal. | Exact field likelihood. | THEOREM — proved | Math §8.6 |
| BEL-10 | Current-world filter | Augmented current belief is a typed kernel image with predecessor masses summed; the physical marginal alone need not suffice. | Exact reconstruction/transition. | THEOREM — proved / BOUNDARY | Math §8.6 |
| BEL-11 | Three update effects | Public action can change physics, support typing, and likelihood weights separately. | Exact model. | THEOREM — proved | Math §8.6 |
| BEL-11A | New-deal domain transition | An all-pass attempt is filtered first, then a persistent-state/new-deal chance kernel creates the next current-deal domain. | Cross-attempt model named. | DEFINITION / BOUNDARY | Math §8.6 |
| BEL-12 | Forced action | At a singleton actor information state, normalized discretionary likelihood is one. | Behavioral field. | THEOREM — proved | Math §8.7 |
| BEL-13 | Own-action cancellation | Own action cancels from own deal posterior under private randomization based only on known information and independent of hidden uncertainty. | Explicit independence assumptions. | PROPOSITION | Math §8.7 |
| BEL-14 | Evidence model-relative | Legality fixes zero likelihood for impossible actions and normalized policy fixes one for forced actions; discretionary likelihood ratios beyond that are field-relative. | Alternative field models allowed. | THEOREM — proved | Math §8.8 |
| BEL-15 | Off-path boundary | Zero-probability histories require an assessment or off-path rule. | Bayes normalizer zero. | BOUNDARY | Math §8.8 |

## 8. Marked hand and strategic state

| ID | Claim | Exact statement | Assumptions | Status | Source |
|---|---|---|---|---|---|
| HAND-01 | Owned induced structure insufficient | Same owned structure can have different live external threats and holder constraints. | Nontrivial ambient complement. | COUNTEREXAMPLE | Math §9.1 |
| HAND-02 | Native hand ambient | The exact hand is an owned marking in the full declaration algebra with public and hidden boundary data. | Straight play. | DEFINITION | Math §9.2 |
| HAND-02A | Local/marginal holder distinction | Raw rule-derived cells expose local holder allowance; actual possible holders are the marginal support obtained from the whole dependent fiber or its canonical reduction. | Exact cell fiber. | DEFINITION / THEOREM — proved | Math §§7.9, 9.2 |
| HAND-03 | Boundary recoverability | Ambient marked structure determines induced hand and boundary queries; converse fails. | Defined expansion. | THEOREM — proved | Math §9.3 |
| HAND-04 | Auction bundle | Before declaration, one physical hand is embedded in all nine algebra layers. | Straight auction. | COROLLARY / SYNTHESIS | Math §9.4 |
| HAND-05 | Action as expenditure | A legal owned node induces physical, retained-record, support-typing, and belief transition. | Exact state/filter. | DEFINITION / SYNTHESIS | Math §9.5 |
| HAND-06 | Slot gauge | Re-encoding local slots leaves physical value invariant and permutes slot-indexed Q. | Encoding/output transported together. | THEOREM — proved | Math §9.6 |
| HAND-07 | No intrinsic tile scalar | The same `4-1` action has exact pointwise values -22 and 22 in anchor worlds. | Witness field/utility. | COUNTEREXAMPLE | Math §§9.7, 10.4 |
| HAND-08 | Additive split noncanonical | The displayed interaction split is nonidentifiable absent additional identifying constraints. | Arbitrary additive decomposition. | THEOREM — proved | Math §9.8 |
| STR-00 | Ambient domain versus belief measure | \(\Xi(c,e)\) records hard admissibility; correlation, null sets, and concentration belong to \(\beta\). Positive-mass support is canonical on finite/countable domains; general topological support requires a declared topology. | Exact strategic model. | DEFINITION / BOUNDARY | Math §10.1 |
| STR-01 | Exact strategic sufficiency | Relative to a fixed rules profile, continuation model, utility, and allowed strategy class, fixed-strategy values and every well-defined fixed-field best-response value are functions of \((c,e,\beta)\); when a maximum is attained, its best-response correspondence is determined there too. | Seven explicit reconstruction, Markov, filtering, horizon, measurability, and integrability assumptions. | THEOREM — proved | Math §10.1 |
| STR-02 | Exact shorthand boundary | \((c,\beta)\) is shorthand only when \(e\) is trivial/fixed; exact factorizations include graph form \((\mu,f)\) and conditional-kernel form \((\mu,K)\), while bare marginals require proved independence. | Explicit factorization/disintegration conditions. | PROPOSITION / BOUNDARY | Math §§8.3, 10.1 |
| STR-03 | Bellman accounting | A bundled observation kernel, cumulative reward, retained-record update, and successor filter must use the same segment boundary. | Named recursion boundary and utility representation. | DEFINITION / BOUNDARY | Math §10.2 |
| STR-04 | Coordinate-only N&S criterion | Exact scalar value factors through a coordinate image iff it is constant on every nonempty fiber of the projection. | Fixed decision problem. | THEOREM — proved | Math §10.3 |
| STR-04A | Scalar/action factorization gap | Equal optimized scalar values on a coordinate do not imply equal action-value vectors or optimal actions. | Explicit two-state/two-action witness. | COUNTEREXAMPLE | Math §10.3 |
| STR-05 | Structural equality only sufficient | Posterior, retained-record, field, utility, and transition-preserving isomorphism is sufficient but not necessary for equal value. | Fixed problem. | PROPOSITION / BOUNDARY | Math §§10.3, 12.2 |
| STR-06 | Full history witness fiber | The common endpoint has exactly 90 legal current remainder worlds. | Specified no-trump prefix. | THEOREM — finite verification | Math §10.4; verifier |
| STR-07 | Pointwise anchors | Two fiber members have Q tables \((10,-22)\) and \((-22,22)\). | Lowest-legal field; signed remaining differential. | THEOREM — finite verification | Math §10.4; verifier |
| STR-08 | Same-support posterior flip | Two legal auctions give positive mass to all 90 worlds but opposite optimal leads. | Specified stochastic bidding field. | THEOREM — finite verification / COUNTEREXAMPLE | Math §10.4; verifier |
| STR-09 | Four-lens action flip | The best lead flips under expected declaring points, signed differential, contract success, and one-mark hand utility. | Same witness. | THEOREM — finite verification | Math §10.4; verifier |
| STR-10 | No strategy fusion in witness | After the root lead, seat 3 has one tile and every later own action is forced. | Two tricks remain. | THEOREM — proved / FINITE VERIFICATION RECEIPT | Math §10.4 |
| STR-11 | On-path higher-order hierarchy | Common prior/model and complete private types induce every finite-order belief hierarchy at positive-probability histories. | Finite/countable model, or standard-Borel model with required regular conditionals; common knowledge. | THEOREM — proved | Math §10.5 |
| STR-12 | Off-path hierarchy boundary | Zero-probability histories require an assessment. | Bayes undefined. | BOUNDARY | Math §10.5 |

## 9. Utility, equivalence, and restrictions

| ID | Claim | Exact statement | Assumptions | Status | Source |
|---|---|---|---|---|---|
| UTIL-01 | Value derived | V and Q are expectations determined by physical state, retained record, belief, field, and utility. | Fixed problem. | COROLLARY / SYNTHESIS | Math §11.2 |
| UTIL-02 | Points/differential affine | \(U_{diff}=2U_{pts}-42\). | Full 42-point hand. | THEOREM — proved | Math §11.3 |
| UTIL-03 | Threshold reversal | Expected points and contract success can rank lotteries oppositely. | Explicit threshold-31 example. | COUNTEREXAMPLE | Math §11.3 |
| UTIL-04 | Information-set best response | One strategy is selected per information state, not per hidden world. | Fixed field/belief. | DEFINITION | Math §11.4 |
| UTIL-04A | Deterministic best-response existence | In a contracted continuation with finitely many reachable information records, bounded utility, and fixed field, an optimal deterministic contingent policy exists. | Native finite information structure or finite augmentation. | THEOREM — proved | Math §11.4 |
| UTIL-04B | Private-randomization boundary | Randomization independent of hidden world and fixed field cannot beat the best deterministic plan; correlated randomness is an added signal/device. | Independence assumption. | THEOREM — proved / BOUNDARY | Math §11.4 |
| UTIL-04C | Infinite-signal extension boundary | The finite-policy proof does not cover arbitrary infinite private-signal models without measurability and selection assumptions. | Added nonnative signal domain. | BOUNDARY | Math §11.4 |
| TEAM-01 | Shared utility not shared information | A controller observing both partner hands defines a different information structure unless added information is proved redundant. | No communication; no separate equivalence theorem. | PROPOSITION / BOUNDARY | Math §11.5 |
| QUO-01 | Physical congruence | Reduced play state preserves within-hand physical transition. | Exact retained fields. | THEOREM — proved | Math §12.1 |
| QUO-02 | Strategic isomorphism | Action, physical/record transition, augmented belief, field-state continuation, and utility-preserving isomorphism preserves value. | Finite contracted continuation; named field and utility. | THEOREM — proved | Math §12.2 |
| QUO-03 | Current support attribution quotient | Completed-tile actor attribution can be omitted from current fiber if exact support residue remains. | Straight cell scope. | PROPOSITION | Math §12.3 |
| QUO-04 | Evidence not preserved | The same omission can lose deal reconstruction and field likelihood. | History-sensitive field. | BOUNDARY / COUNTEREXAMPLE | Math §12.3 |
| QUO-05 | Original information-key test | Exact representation of original information sets cannot merge distinct observation records. | Original extensive form. | COROLLARY / SYNTHESIS | Math §12.4 |
| QUO-06 | Coarser key possibility | A coarser key may be a named abstraction or value-preserving quotient. | Separate proof. | BOUNDARY | Math §12.4 |
| SYM-01 | Seat rotations | Complete simultaneous seat transport gives a \(C_4\) symmetry. | Utility/team orientation transported. | THEOREM — proved | Math §12.5 |
| SYM-02 | Bidder anchoring | Post-auction bidder anchoring is an exact gauge with every relative field retained. | Bidder exists. | THEOREM — proved | Math §12.6 |
| SYM-03 | Reflection failure | Reflection reverses clockwise successor and is not same-rule symmetry. | Oriented game. | COUNTEREXAMPLE | Math §12.7 |
| FILT-01 | Exact predicate restriction | Intersecting with a named predicate does not redefine the unrestricted object. | Predicate explicit. | COROLLARY / SYNTHESIS | Math §12.8 |
| FILT-02 | Stopping frontier | `traverseUntil` returns a frontier, not terminal values. | No quotient supplied. | BOUNDARY | Math §12.8 |
| QUO-07 | Current-hand outcome quotient | Outcome-fixed state preserves contract success, hand award, score update, and whether match ends now. | Completed-trick boundary. | THEOREM — proved | Math §12.9 |
| QUO-08 | Future match quotient scope | Full later-hand match value is preserved only if omitted observations cannot affect future behavior. | Reset/independence assumptions. | PROPOSITION / BOUNDARY | Math §12.9 |
| FAC-01 | Exact current decision factorization | Declaration algebra, ambient marked hand, certified mechanical/support residue, its derived exact reachable support, retained continuation record, augmented belief, typed transition, field, and utility compose the exact current decision object within their named scopes. | Straight cell/reachability scope; fixed continuation problem; §10.1 assumptions. | COROLLARY / SYNTHESIS | Math §15 |
| TYPE-01 | Proof-irrelevant reachability | Reachability is a proposition about a semantic state; different witnesses or proof terms do not distinguish otherwise equal reachable states. | Legal replay semantics; proof/witness not exposed to the game. | DEFINITION / BOUNDARY | Executable §10, §18, §25; proof-assistant handoff §4 |
| TYPE-02 | Derived support views | Rule cells, reduced support, normal form, and fiber are deterministic views of a sufficient mechanical state and are excluded from semantic equality when cached. | Mechanical fields of Math §6.4 retained. | COROLLARY — proved / BOUNDARY | Math §7.10; Executable §§15–16, 20, 25 |
| TYPE-03 | Normal-form well-formedness | The common partition/capacity conditions plus the branch-specific determinate, binary, or ternary conditions are necessary and sufficient for a valid native feasible support normal form; compile/decode are inverse at the exact-support quotient. | Native three-hidden-seat Straight schema. | DEFINITION / THEOREM — proved | Math §§7.10–7.11; Executable §15; proof-assistant handoff §6 |
| TRUST-01 | External verification boundary | Python receipts support finite claims but do not become proof-assistant kernel theorems unless reified and checked by a proved reflection theorem or replaced by an internal proof. | Any proof-assistant formalization. | BOUNDARY | Proof-assistant handoff §§2, 9 |

## 10. Unresolved mathematical boundaries

| ID | Statement | Status |
|---|---|---|
| OPEN-01 | A representation-minimal complete mechanical state is not established; only the exact support component has been minimized. | UNRESOLVED |
| OPEN-02 | A minimal required retained continuation record and minimal latent field-state representation for arbitrary fields/utilities are not established. | UNRESOLVED |
| OPEN-03 | A minimal utility residue for arbitrary history-dependent utilities is not established. | UNRESOLVED |
| OPEN-04 | No general low-dimensional exact strategic quotient is claimed beyond proved gauges. | UNRESOLVED |
| OPEN-05 | Extension of the cell theorem to special contracts is not established. | UNRESOLVED |
| OPEN-06 | Native unrestricted fiber counting has the explicit bounded DP of CELL-10H/I; full extensional enumeration, arbitrary predicate-restricted counting, and generalized variable-seat systems retain separate computational boundaries. | BOUNDARY |
| OPEN-07 | Off-path beliefs are not unique without an assessment. | BOUNDARY |
| OPEN-08 | No canonical sampler is determined by a support fiber; every sampler must name a law. | BOUNDARY — proved nonuniqueness |
| OPEN-09 | Best-response existence for arbitrary added infinite private-signal models is not claimed without measurable-selection conditions. | BOUNDARY |
| OPEN-10 | Finite backward recursion does not by itself imply an effective exact algorithm for noncomputable utilities/operators. | BOUNDARY |
| OPEN-11 | The exact cardinality and optimal standalone code of the strictly Straight-reachable support quotient remain open inside the proved 26–46-bit interval. | UNRESOLVED |
| OPEN-12 | No closed-form local criterion is claimed to replace exact legal-witness validation for arbitrary externally supplied support states. | UNRESOLVED / BOUNDARY |
