# walt mathematics — the freeze register

[Home](Home.md) · owns: the register of walt's determinism freezes 1–57 —
number, content, version, and the ruling that fixed each · Sources:
`walt/CENSUS-RULINGS.md` (every freeze is declared in a ruling there);
cross-checked against the `Freeze NN:` doc comments in the factory's
`examples/*.rs` (archive-only at commit `648f93a` since the 2026-08-24
unification), the results-file headers (now under
`walt/probes/factory-results/`), for freeze 55 the
canonical descriptor in `walt-gpu-ref/src/receipt.rs`, and for freeze 56 the
canonical descriptor in `walt-gpu-ref/src/m2_receipt/receipt.rs`. Related:
[the reference map](walt-math-reference.md),
[information geometry](walt-math-information-geometry.md),
[decision-sparse witnesses](walt-math-decision-sparse.md).

> **Tier: EXPLORATORY throughout**, below every tier on
> [Home](Home.md#evidentiary-tiers--never-promoted-never-blurred).

## What a freeze is, and what it is not

A **freeze** is a declared constant, encoding or ordering that a quoted number
depends on. Freezes exist so that a reported figure is reproducible from the
repository alone. They are *not* mathematics: nothing is proved by freezing it,
and a freeze may be replaced by a later adjudication.

Four standing rules, and they are the reason the register exists at all:

- **Numbers are never reused.** A spent number stays spent even when the build
  it served is parked. Freezes 18–21 belong to the parked seat-census build;
  R-A22 declined to recycle them, precisely because S-A2 cites "freeze 18"
  inside the licensing argument of Lemma S-fold.
- **Freeze-relativity is declared, not assumed.** A predictive dimension is
  freeze-independent; the basis, the closure matrices and every sparsity figure
  are freeze-dependent (R-A21). Which side of that line a number falls on is
  part of what the results file says.
- **A stored artifact carries the freeze-set digest.** A record whose digest
  differs from the running freeze set is **corrupt, not stale**: the cache is
  discarded **entire**, never partially reused (DS-A30, X-A6(i), P-A17, E-A18).
- **A freeze clause states a constant *or* a generating rule, never both**
  (FT-A23(v), bought with the freeze-50(a) defect: an explicit five-element
  carrier list sat beside a sort rule that did not generate it). Where a future
  freeze wants both — a rule a successor can extend, a list this run executes —
  the **rule** is authoritative, the list is printed as a **derived check**, and
  the two are **asserted equal in-run**. An unasserted redundant derivation
  beside a constant is not documentation; it is a second authority for one
  object.

## The register

All 56 issued numbers are accounted for: 54 spent, and **39 and 40 still
RESERVED**. "Fixed at" names the declaring ruling; where a number has been
amended, the version and its amending ruling are named in the same cell.

**How to follow a pointer.** Locate a ruling by its **ID and section heading**,
never by line number — `CENSUS-RULINGS.md` is append-only and line numbers drift
with every append (the discipline is stated on [the reference
map](walt-math-reference.md#the-ruling-families)). Line areas given below are
dated conveniences, correct as of **2026-08-16**, and the heading governs if they
disagree.

| # | Content | Fixed at |
|---|---|---|
| 1 | The content-addressed class encoding — a class identity is the 128-bit FNV-1a hash of its signature bytes, and a signature names successors by *their* hashes, so identity is a function of the future cone alone | r3 Q4/Q5.3 |
| 2 | The per-state canonical move order — moves sorted by (increment, classification, successor class hash), ties broken by the state's concrete tile order | r3 Q5.3 |
| 3 | The yard tree encoding — step and handoff-leaf byte forms, children sorted by (increment, classification, child encoding) | yard v1 pass, registered at P-Q6 |
| 4 | The shape canonical form — leaf colours refined to a fixpoint, then the minimum encoding over orderings still tied, with a declared ceiling past which the run STOPS rather than approximating | yard v1 pass, registered at P-Q6 |
| 5 | The suffix cut — a depth-*d* suffix replaces everything below by a hole carrying that subtree's exact **interned** identity (interning, not hashing); the equality pattern over holes is recomputed locally inside each suffix | shape v2, registered at P-Q6 |
| 6 | The open variant — at unconstrained nodes only, options deduplicated by (increment, classification, child suffix) after children are already in open form, bottom up | shape v2, registered at P-Q6 |
| 7 | The fiber enumeration order, stated precisely enough to reproduce an index → world map (hidden slots in offset order, each slot's k-combinations in lexicographic order, slot 0 outermost) | P-A18 |
| 8 | The decimation rule (g, W) per rung, **for the fiber probe (S5h)** — indices {i·g mod N}, gcd(g,N) = 1 asserted in-run, no prefix, no RNG. Distinct from freeze 25, which is the predictive-rank track's own constants | P-A18 |
| 9 | The fold weighting definition — an exact integer, stated offset-from-focal, never orientation-flavoured | P-A18 (definition at P-A13) |
| 10 | The operator and the valuation | P-A18 |
| 11 | The per-arm key functions — the packed semantic-state key at trick boundaries, and the r3 128-bit signature | P-A18 |
| 12 | Each exclusion predicate's definition, declared **intensionally** — never as a list of class hashes, which are themselves freeze-dependent — printing the set size beside the definition | X-A11 |
| 13 | The flag keying: a flag is keyed by (predicate id, freeze-set id) | X-A11 (keying at X-A6(i)) |
| 14 | The store record format and its freeze-set digest | X-A11, first implemented at E-A19 |
| 15 | The canonical-form key definition and its byte encoding — internal to r1 before, a persistent key now, therefore a freeze | E-A19 |
| 16 | The floor domain and its closed-form count | E-A19 |
| 17 | The declared coordinate order for the warm arm and the receipt stride | E-A19 |
| 18 | The seat-side form: the holder sort, the S-A1 invariant list, **the S-A2 comparison reading** (winner-determining order per live context, tier-0 tiles collapsed to one bottom class), and the byte encoding | S-A19 |
| 19 | The declaration-fold maps and whether the run reports folded, unfolded, or both (both mandatory) | S-A19 |
| 20 | The interface-element encoding — the ordered play record — and the record and hand enumeration orders | S-A19 |
| 21 | The landing form's observation content: the support-normal-form void encoding, the leader-offset encoding, and the empty declaration set | S-A19 |
| 22 | The information-interface encoding — live set, capacity-cell system, leader offset, grade, empty output interface and no monitor, no accumulated outcome — plus the coordinate enumeration order | R-A22 |
| 23 | The closure discipline — primitive-step granularity, per-contract terminal seed sets, the deterministic pivot rule (first nonzero in kernel world enumeration order), basis storage order, index convention | R-A22 |
| 24 | The observation-label encoding, with the derived transition tuple γ = (leader offset from focal; three followers' follow/slough classifications; count-free increment) | R-A22 (γ defined at R-A13) |
| 25 | The decimation constants (g, W) per grade, **for the predictive-rank track (S6a) and everything built on its coordinates (S6b, S6c, S6d)** — implemented `[(7919,12), (104729,6), (1299709,3)]`. Same P-A15 form as freeze 8, different constants and a different track: do not cross-wire them | R-A22 |
| 26 | **The concrete authority**: the H solver and its version, budget, valuation, fiber weighting and observation model — `ScalarHidden::action_values_dag` (dag-v1), `trick_only`, uniform fiber weighting, observation contract = the full public record, `AUTHORITY_BUDGET = 200_000_000`; **the bridge Q_diff = 2·Q_count − grade**; **the tie rule: least domino index among the argmax** | R-A22; the bridge and tie rule ratified as freeze-26 content at SEP-A3(iii)/SEP-A8 |
| 27 | The vector encoding — world order = the kernel world order of freeze 23, exact rationals — and the dedup order | PG-A14 |
| 28 | The dominance-check order and the incremental-fold order over observation branches — a determinism freeze because the stop point depends on it | PG-A14 |
| 29 | The exposure programme and its pivot rule — exact-rational primal simplex with Bland's rule | PG-A14 (programme at PG-A9) |
| 30 | The caps: per-interface frontier cap, per-partial-sum cap, per-coordinate budget, and the grade-3 conditionality rule | PG-A14 |
| 31 | The policy-counting convention — plans versus reduced | PG-A14 (convention at PG-A3) |
| 32 | The detector predicates — D0 in Proposition J-0's form (including the potential-leader quantifier and the still-leadable-context definition), D1-sym in J-1's form, D1-win in J-win's form — and their bitset encodings. Explicitly: **no exhaustion-margin constant exists to freeze** | J-A16 |
| 33 | The detector call sites and the charging rule | J-A16 (rule at J-A13) |
| 34 | The ground-truth classifier: which denominator, computed how | J-A16 (per J-A10) |
| 35 | The harvest arms, rungs, coordinates, budget unit, and the control's solver identification | J-A16 |
| 36 | **The candidate-policy library v1** — see below. **Now v2**: clause (e)'s transport is opened from identity-only to identity plus the declaration fold | **SEP-A4** (reserved at DS-A13); **(e) amended at EC-A8 — freeze 36 v2** |
| 37 | **The action-conditioned upper witness and its solver identification** — see below | **SEP-A6** (reserved at DS-A13) |
| 38 v1, clause (d) clarified at v1.1(d) | **The gluing cut, v1 — RESERVATION DISCHARGED.** The three things DS-A13 reserved the number for: a cut language (a *cut* is a declared partition of the latent world set of **one named focal information state**, forcing one common action per block — "an identification of action variables. It removes no world, alters no world's mass, and asserts nothing about reachability"), the validity obligation and its blanket discharge for the canonical family, and the cut ordering. **Scoped**: the reveal-delay ladder and first-frontier partitions only. **Clause (d)'s induced order was exhibited at v1.1(d)** — a clarification with no new content; v1 is not amended and **v2 is not opened**. See below | Reserved at DS-A13; **FILLED at FT-A17** (§ "The fusion tax: inbox 016 adjudicated", ≈ 7297); **(d) exhibited at SR-A21(ii)** (§ "The second rung: inbox 017 adjudicated", ≈ 9024) |
| 39 | The circuit representation and its evaluation order — exact rational arithmetic is order-insensitive in value, but reported node and operation counts are not — **RESERVED** | DS-A13 |
| 40 | The reachable-belief family defining W_reach, with its deal-level typing — **RESERVED** | DS-A13, as revised by DS-A23 |
| 41 | The checkpoint record format and its freeze-set digest; a record whose digest differs is **corrupt, not stale**, and the cache is discarded entire | DS-A36 (detail at DS-A30(i)) |
| 42 | The unit identity and the canonical assembly order | DS-A36 |
| 43 | The sequential timing rung's selection rule and its W = 1 requirement — declared before the parallel pass, by rule over the canonical unit order, **never by result** | DS-A36 (discipline at DS-A33/A34) |
| 44 v2 | **The walk-step unit and the budgeted-walk contract**: one walk-step per (particle, node) visit, charged `bag.len()` at each `walk` entry before any child call; `walk` takes `budget: &mut u64` and returns `Option`, charge-then-descend, and on exhaustion **no partial fold of any kind is retained**; the stop point is a function of (kernel, budget) alone. Clause (b) binds every `walk`-based evaluator, not only the six then enumerated. Constants unchanged except the v2 amendment: **B = 10,000,000,000** walk-steps per (coordinate, action), **4B** whole-call for `revealed_summary`, **P_max v2 = 192,000,000** partition states per (coordinate, action), and **g = 15,485,863** for the §5 rung. `P_max v2` is applied to the count-only pass's **completed exact count before any map is allocated**; a count above it is `NOT PRICED`, with no partial partition or verdict. The insertion check remains only as a defensive stop and is never a receipt. Clauses (a)–(d), (f) and (g) are unchanged | v1 at **N4-A1**; clause (b) clarified at **RW-A8**; **v2, clause (e) only, at N4-A16(vi)** |
| 45 | **The n = 4 coordinate identity**: grade 4, declaration pip, the viewer's hand and pool as canonical ascending-domino-index tile lists, leader offset from focal asserted **0**, \|X\| = 34,650 asserted against `kernel.count()`, and the freeze-7/23 fiber enumeration order. Corpus hand id and trick number are **provenance only**, never identity components; the kernel is rebuilt in-run from the printed identity and asserted equal. **No library entry is written at any n = 4 coordinate** | N4-A3 |
| 46 | **The economy-successor arm list, CLOSED**: X (exact control, the H-argmax seed recomputed in-pass — receipts, not measurements, g = 0 by Corollary E4.1(2)); T (transport, the four library entries by φ to p′ = 6 plus idx = 0 to p′ ∈ {1..5} — receipts under Corollary S-fold-val); P1 least-tile, P2 greatest-tile, P3 beat-if-able, P4 trump-hoard; and R, the heuristic re-key, labelled **HEURISTIC RE-KEY (NOT A TRANSPORT)** on every row. Plus the transport and image-key construction, the canonical run order, and the results-file column set. An open arm list is not a freeze: a later arm is a **freeze-46 v2** fixed by a later adjudication | EC-A1 |
| 47 | **The trick-1 carrier**. **(a)** Arm A: the drawing family of T1-A6, **all 294 coordinates**, in the canonical order *declaration pip ascending; then t descending; then the non-trump doubles' pips ascending lexicographically*, each coordinate's identity printed in freeze 45's form and the kernel rebuilt in-run and asserted equal. **(b)** Arm B: the **13 `verify_player.txt` hands at trick 1**, by corpus index ascending, with the bidder asserted to be the trick-1 leader at each. **(c)** The reduced-grade cross-check ladder of (T1-R2): grades 2, 3, 4 mandatory, grade 5 attempted with a declared stop. **(d)** No library entry is written at any coordinate of either arm. Belief and field are **not** re-declared — freeze 26 and freeze 37(d), cited unchanged | T1-A11 (§ "The trick-1 witness: the bounded sandwich, refuted and replaced", ≈ 5931) |
| 48 | **The lay-down catalogue**: "the hand enumeration order (ascending canonical domino index, lexicographic), the declaration order (pip ascending), the catalogue record format with the freeze-set digest, and the phase-2 search order (full-suit hand by pip ascending, then extensions in catalogue order)" | LD-A9(iii) (§ "Lay downs: the characterization, and the four-laydown question", ≈ 6230) |
| 49 | **The n4 economy carrier**: the coordinate list (the nine n4 coordinates in freeze-44(f) order), the action set (all four per coordinate, ascending), the arm subset (**P1..P4**, with X printed structurally unavailable where the cap bars it and T/R out of scope), the **rule argument list `(record, legal)`** of RW-A1(i)–(ii), the canonical run order, and the results-file column set (arm, action, L_rule, Q^H, gap, margin, separation cell with its typing, reached count, walk-steps, residual). **No freeze is amended by it** — freeze 44(b)'s contract already binds every `walk`-based evaluator | RW-A8 (§ "The map-free rule walk, and what h9 already decided", ≈ 6544) |
| 50 v1.1 | **The fusion-tax probe carrier.** **(a)** The carrier is **five negative-margin n = 4 coordinates** in exactly this enumerated order — pip 3 `[00 21 32 53]` (h0); pip 4 `[11 40 43 53]` (h6); pip 5 `[21 33 53 54]` (h2); pip 4 `[30 41 54 61]` (h9); pip 0 `[20 30 40 65]` (h12) — **with no generating rule** (v1.1(a), see the amendment note below). **(b)** Per coordinate the run is over the **binding pairs only**: every (a⋆, a) whose filed margin is negative. **(c)** The frontier emission format — one row per (coordinate, a, I) carrying the record, `p_I`, \|X_I\|, \|A(I)\|, `δ_I` as an exact rational **in the count convention**, the complete argmax set, and one minimal fusion core where `δ_I > 0`; **v1.1(c)** cuts the emission **by content**, see below. **(d)** The per-coordinate summary: `T_a` (asserted 0), `Δ_a^(1)`, `U_a^(1)`, `Δ_a^(2)`, the decision cell and FT-tie's fraction-required column. **(e)** Belief and field are **not** re-declared — freeze 26 and 37(d), no decimation anywhere inside any L, U or tax. **(f)** No library entry at any coordinate. **(g)** The freeze-set digest travels on every record | v1 at **FT-A18(vi)**; **(a) amended at FT-A23(iv)**, **(c) amended at FT-A24(viii)** (same section, ≈ 7389 / 7616 / 7677) |
| 51 | **The depth-two probe carrier.** **(a)** The carrier, **enumerated with no generating rule** (the FT-A23 rule applied at the point of declaration this time, not after a defect): **arm 1, mandatory** — coordinate h2, pip 5, hand `[21 33 53 54]`, both freeze-50 units, competitor `a = 53` then `a = 54`, in that order; **arm 2, attempted after arm 1 completes, with a declared stop** — coordinate h9, pip 4, hand `[30 41 54 61]`, units `a = 41` then `a = 54`. h0, h6 and h12 are **out of scope**. Coordinate identity asserted first in freeze 45's form, kernel rebuilt in-run and asserted equal. **(b)** Why that order: h2 is the carrier's smallest first frontier and the coordinate the received note itself nominates; h9 is the second smallest, carries the branch's largest exact negative, and is the coordinate the exact primal route **cannot price**. **(c)** The frontier-2 convention, fixed by Lemma SR-forced: the second frontier is the focal seat's **next decision after b, forced or not**; a forced second state is **counted, not skipped**, matching rung one — the alternative is not wrong, it is a different object, and two objects with one name is how a chapter goes bad. The early-terminal mass and Θ are **asserted zero** at grade 4 and the assertion is contentful. **(d)** Emission cut by content per the freeze-50 v1.1(c) pattern: committed branch and state rows, a regenerable uncommitted companion under its SHA-256 and byte/line counts, and **seven accounting integers per unit** making the omission auditable. **(e)** Every column in the **count** convention, with the two bridges kept separate. **(f)** Belief and field **not** re-declared — freeze 26 and 37(d), no decimation inside any L, U, s, d or δ. **(g)** No library entry at any coordinate. **(h)** The freeze-set digest travels on every record | SR-A22(iii) (§ "The second rung: inbox 017 adjudicated", ≈ 9076) |
| 52 v1.4 | **The feature-fee audition carrier.** **(a)** The carrier, enumerated with no generating rule: **arm 1** — h0, pip 3, hand `[00 21 32 53]`, unit `a = 00` (the cheapest unit in the carrier *and* the hand the candidate feature came from); **arm 2, attempted after arm 1, with a declared stop** — h2, pip 5, hand `[21 33 53 54]`, units `a = 53` then `a = 54`. **(b)** The measured object, per frontier state with a **positive** local tax and per feature: the per-action centre for every action, the breakpoint count, the optimal fee coefficient, the residual and the captured amount. Zero-tax states are **counted and skipped** — there is nothing to capture there. **(c)** The fee is centred **per action**, and that is mandatory and receipted: a single per-state centre fails the penalty theorem's hypothesis and the resulting number would bound nothing in either direction. **(d) Exact minimisation only** — enumerate the breakpoints, evaluate, take the least. **No grid, no search, no float**; every denominator that must divide is asserted to divide and an arithmetic overflow is stop-and-report, never a wrap. **(e)** The tie rule, declared before the run: the **smallest** breakpoint attaining the minimum, ascending; no breakpoints means a coefficient of zero. **(f)** Reporting in the **count** convention: the per-unit residual, the capture as an exact rational, a three-way census of states by how much was captured, the leading/following split, and the count of states with no outstanding trump. (The capture is named **oracle-θ capture** and never "capture" unqualified — that naming rule is Proposition FF-oracle's, carried here because it binds every such column.) **(g)** Belief and field not re-declared; no library entry; the freeze-set digest travels. **(h)** Budgets unchanged; on exhaustion no partial capture and no partial residual. **Four amendments, all below** | v1 at **FF-A6**; **v1.1 at FF-A15(i)**, **v1.2 at FF-A20(iii)**, **v1.3 at FF-A23(iv)**, **v1.4 at FF-A33(iii)** (§ "The feature-fee audition: Jason's control feature, specified", ≈ 10122 / 10447 / 10609 / 10689 / 11117) |
| 53 | **The fee-correlation diagnostic.** **(a)** The carrier is the **same three units as freeze 52 v1.1** — h0, pip 3, hand `[00 21 32 53]`, unit `a = 00`; h2, pip 5, hand `[21 33 53 54]`, units `a = 53` then `a = 54` — enumerated with no generating rule. **No new coordinate is introduced.** **(b)** Four features with their roles fixed: the **null control**, exempt from the v1.4 screen and run **first and blocking**; the boss-keyed binary feature and **its graded form**, the latter **diagnostic-only** (no capture figure computed for it); and the amended beatability feature. **(c)** The measured object per swept state and per feature: the two one-sided slopes, the **clairvoyant-argmax cardinality profile**, the distance to the nearest breakpoint on the descending side and which side it lies on, and the **drop bound**. Swept states only — the domain census still emitted over every frontier state. **(d)** The **frozen comparison table**: the previous run's per-state captured amounts, transcribed into the probe source with a provenance line and **never re-parsed from results text**. They are what the two contentful receipts compare against. **(e)** Exact rationals only, no float, checked arithmetic, every divisibility asserted; **count convention on every tax column**, with the two bridges kept separate. **(f)** One row per (unit, feature, state), committed entire with no companion, and **no count or capture figure without its state set named in the same sentence**. **(g)** Belief and field not re-declared; no library entry; the freeze-set digest travels. **(h)** Budgets unchanged; on exhaustion no partial fold | FC-A5 (§ "The fee-correlation chapter: what a fee bites on, measured", ≈ 11330) |
| 54 | **The seed-survey carrier and measured objects.** **Carrier generating rule:** seeds `0..99`; `index(n) = (n*A) mod D`, with `A = 292,032,399,099,041` and `D = 472,518,347,558,400`; standard mixed-radix combinadic deal unranking; declaration `PipTrump(n mod 7)`; seat 0 leads; freeze-26 least-index policy plays three tricks; focal seat is the trick-3 winner; every legal focal action is one unit. The generated 100 coordinates are printed in freeze-45 form and are not a redundant authority list. **Measured per unit:** exact `Q^H`, complete H-optimal face and separation cell, `U^C` and fusion gap, first-frontier/tie-multiplicity census, the two-rung tax split, four frozen rule values, and the count-only partition pass against `P_max v2`; exact arithmetic only, with the committed-summary/companion split and checkpoint order fixed at SS-A4 | **SS-A4** (§ "The seed survey: a hundred fresh coordinates, designed"); range closed at SS-A18 |
| 55 | **The GPU-native trick-1 portable M0/M1 authority and deterministic encodings.** Fixes the bytes of `GT1_FREEZE_SET_DESCRIPTOR_V1` and the v1 encoding rules it names: received-v0.2 identity and v0.3/GT1 authority; the exact `OpeningRootV1` profiles; `U256MassV1` (`8 x LE-u32`) at field scale 420; `SemanticTablesCanonicalV2`; the opening-cell generator/order and 11,730 cap; `ReducedOpeningCarrierV1` grades 2..5, the 100,000 direct-world cap and 756,756-world zero-output grade-5 stop; tasks `M1OpeningResponseProjectorV1` and `M1OpeningDirectParityDeclaredStopV1`; and the canonical run-envelope, declared-stop and source-manifest-derived build-identity schemas. Generated hashes, lengths and counts are asserted derived checks, never parallel authorities. Freezes 7/23/26/47 are cited unchanged; 39/40 remain reserved; freeze 44 and M2+ are excluded | **GT1-A9** (§ "GPU-native trick-1: the bounded portable foundation") |
| 56 | **The binding M2 Metal parity authority and deterministic encodings.** Fixes the exact 899 ASCII bytes of `GT1-M2-FREEZE-SET-V1`, SHA-256 `7bdc5e05513fd1d7e7b6c26870cf9bd4a16966c5daf48963729d999c4b6b28cf`, naming: binding contract `GPU-NATIVE-TRICK1-M2-v1` at SHA-256 `aacb6df5e9106b3b6bf00ccfb496c71f762c0fb4644c13a17f76d2ac2f0326e3`; parent freeze 55 at SHA-256 `9b181092045b003893cae7c09cc7b7c8b57f75c3c5c4cf7043b8d428df738efa` and commit `3b4c6d60fef371e3050de151ccf9eaefbc2d2da7`; received guide at SHA-256 `ee2e78da20eb7d087fb121f467a56bafc0179a45fb692ca0b938f4c4210b6a44` and exact rebrief at SHA-256 `9183132529a42289a104a73d8f7e196eb95058ac2edda60bb42c715f1f8a139a`; tasks `U256MetalParityV1` and `OpeningProjectorMetalParityV1`; `U256MassV1`, `U256MetalCorpusV1`, `M2MetalAbiV1`, `OpeningChooseTableV1`, `M2OpeningParityCarrierV1`, `ReducedEvidenceBindingV1`, `PhysicalActionBindingV1`, `M2SequentialRunnerV1`, `M2MetalCompilerProfileV1`, `M2MetalParityReceiptV1`, `M2SourceManifestV1` and `Texas42.Trick1MetalFoundation`. It keeps 39/40 reserved and excludes action value, selected lead, information net, K-OPEN4+ continuation, performance and player. Generated receipt/build hashes, byte lengths, counts and observed limits remain asserted consequences, never parallel authorities | **GT1-A17** (§ "GPU-native trick-1: the binding M2 Metal parity gate") |

## Freeze 52's amendment chain: v1 → v1.4

Four amendments in one day. The first three came out of the first run and all
trace to faults in the freeze's own text rather than to anything the build did;
the fourth came out of a compliance question and caught a defect that had never
fired. They are recorded together because the sequence is the instructive part.

**v1.1, at FF-A15(i) — the domain clause, scoped to the features that use it.**
The frozen feature list defined "outstanding trump" and then said that *if no
trump is outstanding, every feature below is 0 for every action*. That fallback
was attached to all three features, but only two of them reference the
boss-trump holder at all; the third is perfectly well defined with no trump
outstanding, and that is precisely when it is most interesting, because control
then turns entirely on suit rank. **The clause voided six of the twelve
(feature, unit) cells by construction**, including the one measurement the run
most needed. v1.1 scopes the fallback to features that reference the boss-trump
holder, requires **every feature to carry its own domain clause**, and repairs a
second, smaller ambiguity in the third feature's definition at following states.

**v1.2, at FF-A20(iii) — the domain screen, before the sweep.** Every unit now
emits its feature-domain census **before** sweeping, and a unit whose domain is
empty at every swept state is declared an **EMPTY TEST**: it is not swept, and
it contributes to no capture figure and no outcome gate. An empty test is
reported as a unit that did not run, **never as a zero**. For a boss-keyed
feature the screen is computable from the coordinate and root action alone with
no traversal — if exactly one trump sits outside the focal hand and the root
action is a trump, the field must follow, the lone boss burns on trick 1 in
every world, and the domain is provably empty at every frontier state.

**v1.3, at FF-A23(iv) — the sweep and the census count different sets, and both
say which.** The sweep runs over states with a positive tax, because a state
with no tax has nothing to price; the domain census runs over **every** frontier
state, because its job is to characterise whether the feature has content at the
coordinate at all, and a screen restricted to swept states would answer a
different question. Both are printed and **both are labelled with the set they
count**. Nothing numeric turns on the distinction — the capture figures are
identical under either reading, since the skipped states contribute zero to both
numerator and denominator — which is exactly why choosing silently would have
been a reportable deviation rather than a private one.

**v1.4, at FF-A33(iii) — the screen applies per (unit, feature) cell, and the
null control is exempt.** v1.2 said *a **unit** whose domain is empty is declared
an EMPTY TEST*, but **domain-emptiness is a property of a (unit, feature) pair,
not of a unit**. Read literally, v1.2 would have barred the null control from
running at the coordinate where its own domain is empty — contradicting the
mandate that the control run first and blocking at *every* unit precisely because
it is what makes the other numbers trustworthy. The build had resolved this
correctly and silently by running the control everywhere, so nothing was owed;
v1.4 makes the resolution the rule. **The null control is exempt from the screen
in all cases**, because its job is to test the harness rather than the feature,
and a harness check that skips the states where the feature is empty is exactly a
harness check that has not been run where it is cheapest to run.

## Freeze 50's amendment: v1 → v1.1

Two clauses moved, both on the same day and both because the **build** found a
defect in the adjudicator's text and reported it rather than picking a reading.
Nothing else in freeze 50 changed, and no number was reused — content is
versioned by ruling, the freeze-36-v2 / freeze-44-v2 pattern.

**(a), at FT-A23 — the enumeration governs, the sort clause is struck.** Clause
(a) as first written carried an explicit five-element carrier list *and* a
trailing sort rule, and **the rule does not generate the list**: the enumerated
order h0, h6, h2, h9, h12 is neither pip-ascending, nor hand-lexicographic, nor
the corpus-hand-id order of the filed separation results. It is the order in
which the adjudicator discussed the coordinates in the prose above it, and it is
generated by no rule at all. EC-A8's principle decides which survives — *a freeze
is a constant, not a rule* — so **the five-element list is the constant** and the
sort clause is **struck, not repaired**; re-sorting the carrier would change the
emitted file for zero informational gain. The emitted order stands and no
re-emission was owed on this account: nothing numeric crosses coordinates, and no
budget stop occurred. **It would not have been merely presentational had a stop
occurred**, since which units complete is a function of the order — which is the
whole reason a carrier order is frozen, and the reason this had to be ruled
rather than shrugged at. The standing discipline it yields is the fourth rule at
the top of this page.

**(c), at FT-A24 — the emission is cut by content, not by size.** Of the run's
**281,542** frontier rows, **12,639 carry `δ_I > 0` and 268,903 carry
`δ_I = 0`**; a zero row contributes exactly zero to `Δ^(1)`, cannot appear in the
concentration ranking, and has no fusion core. So the **committed** named file
carries every positive-tax row together with the complete optimal-face material,
every receipt, every summary, every pair verdict and every fence, and the full
table including the zero rows is a **regenerable, NOT COMMITTED companion**
named with its regeneration command, its **SHA-256** and the freeze-set digest.
Two accounting assertions per unit make the omission auditable — the printed
rows' `δ_I` sum to `Δ^(1)` exactly, and printed plus suppressed equals the
frontier state count already asserted on the `P_max` admission line — with the
suppressed count split into forced states and unforced-with-a-common-optimum.
A **declared cap** stops a future run smuggling a truncation: past 20,000
positive rows in one unit the file carries the top 20,000 by descending `δ_I`
plus the residual tail's exact count and summed `δ`, printed in place, never
silently. The split is produced by the probe's own emitter in a re-run and
**never by post-processing emitted text**, and that re-run bought a receipt that
did not exist before — (FT-R7), the re-emission determinism receipt.

## Freezes 36, 37 and 38 in full

All three were reserved by DS-A13 in the intake audit: 36 and 37 were fixed by
the Experiment E adjudication, 38 sat reserved through four further sections and
was filled by the fusion-tax adjudication. They are the load-bearing freezes of
the live track and the ones a successor is most likely to need.

### Freeze 36 — the candidate-policy library, v1 (SEP-A4)

**(a) Key:** (grade, base index, declaration ∈ {0..6}, root action) under the
S6a unranking; **pip is derived** from the index by that unranking and is a
printed field, never a key component. **(b) Body:** a total map from observation
record to chosen tile over the built information partition, serialised as
(observation record, chosen tile) pairs sorted lexicographically by record under
the canonical ascending domino-index order, the record being the plays since the
kernel decision point with the root action first; **the in-process information
state handle never appears in a stored entry**. **(c) Frame, mandatory on every
entry:** observation contract, field, belief, |X|, and the freeze-set digest.
**(d)** Stored content is a policy and its provenance only — no value, rank,
verdict or dominance status; the file is a cache, never an authority; a loaded
entry is re-priced before use; a digest mismatch is corruption and the file is
discarded entire. **(e) Transport:** identity only in v1 — **amended to v2 at
EC-A8**, see below. **(f) Seed rule:** the
argmax-recording pooled H solve over the same partition, **unmemoized**, with
freeze 26's tie rule **cited, not restated**; the seed contributes no number to
any reported L. **(g)** The DS-A16 header note: entries remain valid
primal-witness sources under count re-entry; their count-free quality verdicts
do not survive.

**Freeze 36 v2 (EC-A8) — clause (e) only.** Transport becomes: *identity, and
the declaration fold φ_{p→p′} of Lemma S-fold — image key computed by the
freeze-46(b) construction, R9 receipts asserted in-run, values licensed by
Corollary S-fold-val and verdict transport by Lemma E7 with β′ = T_*β. Any
further transport re-enters with its own adjudication.* Note what was
**rejected**: the class formulation, which would have admitted "transports with
an exhibited isomorphism" as a class — a freeze is a constant, not a rule, and a
class clause would delegate future adjudications to the freeze. Conditions:
transported candidates are in-process objects; **no image entry is written to
the library file** in the successor run, which stays at its four entries; a
transported policy is re-priced before anything is reported. Numbers are never
reused — versioning content by ruling, as here, is the pattern (freeze 46(e)
cites this precedent for its own future v2).

### Freeze 37 — the action-conditioned upper witness (SEP-A6)

**(a) Evaluator:** U_a := the per-root-action column of the revealed summary,
read at the declared direction, identified as E_β[V*_a]. **(b)** The relaxation
is named treatment **C**, not C⁺ — on this carrier the latent is ξ = ω and the
two coincide. **(c)** The declared direction is the count-free focal trick
differential; the reporting convention is the count convention and the freeze-26
bridge is asserted exactly **at the reporting boundary only**; the bridge is
affine with positive slope, so verdicts are convention-invariant. **(d)** Belief
uniform over the full enumerated fiber, identical on both sides; **no decimated
world set appears inside any L or U**. **(e)** Conditions (C1)–(C4) asserted
in-run. **(f)** The per-action price is the existing per-root continuation-price
column, asserted nonnegative; its aggregate siblings are named once and never
confused with it. **(g)** The envelope and scalar H solvers are in the **same
units** and are asserted **equal exactly, with no bridge**; the root is asserted
trick-leading so their action lists coincide. **(h)** Budget honesty: the scalar
authority is budgeted and its exhaustion is a declared stop; the envelope path
carries no budget and no stop, and the results file says so in place.

### Freeze 38 — the gluing cut, v1, scoped (FT-A17)

Reserved at DS-A13 for "the gluing-cut language, the validity-proof obligation,
and the cut ordering", and confirmed reserved and untouched at SEP-A18, T1-A11,
LD-A9 and RW-A8 in turn. The received nonanticipativity-taxes note supplied all
three, **correctly typed**, so the number is now spent.

**(a) The cut language.** A *cut* is a declared partition Π_I of the latent world
set X_I of **one named focal information state** I, constraining the relaxed
controller to one common action per block. It is an **identification of action
variables**: it removes no world, alters no world's mass, and asserts nothing
about reachability. A construct that excludes worlds is a **declared exclusion
remnant** and is not a cut. This typing is what let the number be filled at all —
it is the line between a cut in Theorem E6.5's sense, which carries E6.5's
guarantees, and a world-level exclusion, which carries none of them.

**(b) The canonical family, v1.** (1) The **reveal-delay ladder**: `C^(k)` is the
one-block partition at every focal frontier of depth ≤ k and the singleton
partition below, with `C^(0) = C` and `C^(N−1) = C^(N) = H` by Lemma FT-trunc.
(2) **Within the first frontier only**, arbitrary partitions Π_I, priced by the
exact merge cost `c_I(B₁,B₂) = v_I(B₁) + v_I(B₂) − v_I(B₁∪B₂)`.

**(c) The validity obligation, and its discharge for (b).** Every cut must be an
information equality satisfied by every lawful policy. For family (b) it is
discharged **once and for all**: a lawful policy chooses one action per
information state and therefore satisfies every block identification within that
state. Any cut outside (b) re-enters with its own proof and its own freeze
version.

**(d) The cut ordering — which is why this is a determinism freeze.** Layers
ascending, k = 1, 2, …; within a layer, frontier information states in
**ascending observation-record order** (freeze 36(b)'s lexicographic order over
the canonical ascending domino index); within a state, actions in ascending
domino index; block merges, where used, in **descending exact merge cost** with
ties broken by the smallest record in the block. **Declared before the run and
never chosen by result.**

**(d) at v1.1(d) — the induced order exhibited, not re-specified.** Clause (d)
already generates an order at every rung, but FT-A23's lesson is that a rule
which does not visibly generate its list is a defect, so the rung-two order it
induces was written out: **first states in ascending record order; within a first
state, first actions in ascending domino index; within a (first state, first
action) pair, second states in ascending record order; within a second state,
second actions in ascending domino index.** Second-frontier records are
information states of layer 2, ordered by the same lexicographic rule, and a
second state's record strictly extends its parent's — so this is a well-defined
total order, declared before the run. **This is a clarification with no new
content**: v1 is not amended, v2 is not opened, and the depth-two cut needed no
new cut authority because the reveal-delay ladder of (b)(1) already has k = 2 as
a member and (c) discharges its validity along with the rest of the family.

**(e) The stop rule.** Theorem E6.5(G2)'s exposed-face criterion, instantiated as
the zero-tax test computed from **complete** argmax sets. A single tie-broken
optimiser is not evidence of a conflict, and **freeze 26's least-index tie rule
is not used here** — it exists to make the *authority* deterministic and is not a
statement about the optimal face. **At rung two a second, strictly stronger rule
sits on top of this one**: the complete optimal face is *also* not sufficient,
because the minimum may be attained by an action outside it. Both bind and they
are different rules — see [the decision-sparse
track](walt-decision-sparse.md#the-second-rung-chapter-s6l-2026-08-14).

**(f) Arithmetic and reporting.** Exact rationals throughout over a common
integer denominator; no float. Taxes are reported in the **count convention**,
obtained from the differential evaluators by the exact inverse of freeze 26's
bridge: a differential tax is **twice** its count-convention value, and **a tax
compared against a margin in the other convention is void**.

**(g) Scope of v1, stated so a successor knows what is not frozen.** Feature
penalties, multi-stage/martingale penalties, adaptive block search beyond the
first frontier, and any cost model for the decision-relative distance `κ_a(T)`
are **NOT** in freeze 38 v1. They re-enter as **freeze 38 v2** fixed by a later
adjudication. No number is reused; **39 and 40 remain reserved and untouched**.

## Addendum 2026-08-24 — freeze 57, and freeze-56's v2 re-issue

Two register events since the table above was written; pointers only, the
rulings govern.

| # | Content (one line) | Fixed by |
|---|---|---|
| 57 | **The binding M3 perfect-recall-net gate.** Fixes the exact 962 ASCII bytes of `GT1-M3-FREEZE-SET-V1` (SHA-256 `e5efe6ce…`), naming binding contract `GPU-NATIVE-TRICK1-M3-v1` at SHA-256 `79de73e9…`, parent freeze 56, the M3 objectives/treatments ABI, carrier profile `M3CarrierProfileV1` (h8, roots 21-31-33-55), and `Texas42.Trick1PerfectRecallNet` as the proof boundary. It authorizes only the gate and records **no M3 result**; the GT1 ruling range is re-frozen at A1..A24 and the chapter closed. | **GT1-A24** |
| 56 v2 | **Append-only re-issue at the unified layout** (the fold, 2026-08-24). New cumulative source manifest `walt/math/gpu_native_trick1_m0_m2_sources_v2.sha256` (identity `8a780895…`) beside the byte-immutable v1; a 32-entry fold-translation table as verifier amendment; full-closure checking demoted to freeze-event verification; the standing M2 receipt explicitly **old-layout evidence**, its re-earning deferred to [[m2-receipt-reearn]]. | **FZ-A1..FZ-A6** (§ "freeze-56 v2") |

## Addendum 2026-08-31 — freeze 58, the RefineV1 semantic freeze

One register event; pointer only, the ruling governs. This is a freeze of an
implementation's *semantics*, in the frozen-policy-identity lineage: it exists
so that "reproduces RefineV1" is a checkable sentence.

| # | Content (one line) | Fixed by |
|---|---|---|
| 58 | **RefineV1.** `walt/walt/src/solver/refine.rs` as merged at main `25b40d9` (PR #69) is the semantically frozen reference controller: no new fields, enum variants, or work items, ever; bug fixes only with independent justification; its four gates (`walt/walt/tests/solver_factor_refine.rs`) never weaken. The anytime proof-state core must reproduce RefineV1's results wherever their scopes overlap (same producers, same goal) before any promotion, and remains removable without touching it. New capability goes to the new core, never to this file — the growing-enum temptation is refused by rule, not judgment. | **APS-A9** (number issued here) |

## Two discrepancies on the record

Both were found by cross-checking the rulings against the code and the published
results headers. Neither has been resolved by editing anything, per the
project's ambiguity protocol.

1. **Freeze 1 vs 2 ordering.** The r3 Q5.3 passage names them in the order
   "canonical move order and content-addressed encoding", but the numbering in
   the implementation and in the X-Q7 citation is the reverse: content-addressed
   encoding is freeze 1, canonical move order is freeze 2. The code and X-Q7
   agree with each other; the prose ordering in r3 Q5.3 is the odd one out.
2. **Freeze 2's sort key.** The census-era results headers describe the order's
   first component as `k` — the r3, t5, pruned, yard and yard-v2 files all do —
   while the implementation and the fiber-probe results header say `increment`.
   The two names denote the same quantity in the census-era files, which define
   `k` as the count-free increment, but the register records the divergence
   because the freeze is quoted by name across both eras. **The code is
   authoritative.**
