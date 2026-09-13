[Home](Home.md) · owns: the dated record — every landing from the v0.7 ingest (2026-07-26) to HEAD c00717d1 (2026-09-07), by wave, with commit hash and/or PR number, what each established at which tier, and the page that owns it · Sources: `git log` of `main` (first-parent line through `2de8a05` and `2de8a05..c00717d1`, 197 commits), the GitHub merged-PR list #1–#88, `exchange/README.md` ledger, `walt/LOG.md`, `walt/MAP.md`, `walt/briefs/`, `walt/CENSUS-RULINGS.md`, `experiments/partnership/SCOPE.md`, the survey maps of 2026-09-07

# The record

This page is the book's calendar. Each entry gives the date, the short commit
hash and/or pull-request number, one line of what landed, its evidentiary tier
in brackets, and the page that owns the topic. It states nothing the owning
page does not; when the two disagree the owning page is wrong or this page is
stale, and either is a bug to report.

**How to read the tier brackets.** `[corpus]` — a package status; `[kernel]`
— a Lean theorem checked by the kernel; `[exchange CONFIRMED]` /
`[exchange PARTIAL]` — an adjudicated ChatGPT 5.6 Pro result; `[rob receipt]`
— a byte-diffed conformance reproduction; `[EXPLORATORY]` — everything under
`walt/`, `experiments/`, the ideas, analysis and field pages, which sits below
every other tier and is cited by nothing above it ([Home](Home.md), tier
ladder). `[process]` marks a rule, a document, or a queue change that
establishes no result. A walt number below is quotable only through the gate
file or record path named for it; where only a probe record exists the entry
says so.

**Dates.** Every date is the git author date on `main` (the machine's local
time, UTC−5). GitHub's `mergedAt` is UTC and pushes some late-evening merges to
the next calendar day (PRs #37–#42, #58–#59, #67–#69, #75–#77, #81–#87 read one
day later on GitHub). Where the two differ this page keeps the git date.

**Shape of the record.** Before 2026-08-24 the history is a single line of
direct commits plus eight merged pull requests (#1, #3–#8); after the
unification base `2de8a055` (PR #8, 2026-08-24) it is 197 commits and 78
merged pull requests #9–#88 (#10 and #11 were closed unmerged — they carry the
same titles as #12 and #13 and were superseded by them). Not one commit after
`2de8a05` touches `ingest/`, `rob/` or `lean/`: the claim-tier mathematics is
where the 2026-08-24 wiki left it, and every landing after that date is
EXPLORATORY.

---

## Part A — the pre-unification arc, 2026-07-23 → 2026-08-24 (compressed)

### A1. Foundations, the exact engine, the adversary, the kernel (07-23 → 08-03)

| Date | Commit / PR | What it established | Tier | Owner |
|---|---|---|---|---|
| 2026-07-23 | (v0.7 provenance) | The research mandate, verbatim in `provenance/RESEARCH_MANDATE_2026-07-23.md`; demotes the mk5 implementation gates to evidence. | corpus (provenance) | [lineage](lineage.md), [package-provenance](package-provenance.md) |
| 2026-07-26 | `4e238ed6` | The two v0.7 packages (`texas-42-foundations-source-of-truth-v0.7` and `-reconstructed`) ingested under `MANIFEST.sha256`; never modified since. | corpus | [package-provenance](package-provenance.md), [discrepancies](discrepancies.md) |
| 2026-07-26 | `c442a27d`, `b760caef`, `e61133db`, `5c482b25` | The wiki's first pass: Home, provenance verdict, the seventeen-item discrepancy ledger, the ten core mathematics pages, claim ledger, verification, proof-assistant plan, FINDINGS. | process | [Home](Home.md), [FINDINGS](FINDINGS.md), [claim-ledger](claim-ledger.md) |
| 2026-07-26 | `e8ccc176` | rob slice-01 brief and README; the engine's discipline fixed (derived views, proof-irrelevant reachability, no floats). | process | [rob](rob.md), [first-implementation-slice](first-implementation-slice.md) |
| 2026-07-27 | `91b7c991`, `b18d6359`, `651824fb`, `6639c833` | rob slice 01 green: S1 declaration algebra (737,100 unique-winner tricks), S2 objective hand machine, S3 cells and losslessness (972 prefixes, the 90-world support witness), S4 support normal form and capacity DP. Four receipts. | rob receipt | [rob-slices](rob-slices.md), [verification](verification.md) |
| 2026-07-27 | `05d1df86`, `513a2e01`, `dd40bfa6`, `e5a69d5c`, `4ce6e16c`, `fa027ab7` | rob slice 02 green: S5 matching-minor calculus (1,331 normal forms, 170,058 observations), S6 symbolic trace validator (3,024 three-way agreements), S7 necessary outer language and censuses, S8 unreachability regressions (425,520 traces, 0 realizers), S9 transport-aware census (9→3 declaration classes), S10 stretch — x:001's floor 17,668,066,045 reproduced, interval [35,46]. Eleven receipts in all. | rob receipt | [rob-slices](rob-slices.md), [reachability](reachability.md) |
| 2026-07-27 | `34b46b66`, `4b9f3344`, `8ea2ff24`, `8d3651ef` | The exchange channel opens: courier protocol, ledger, five dispatches 001–005 (from FINDINGS §8) submitted 05:07–05:10Z. | process | [exchange](exchange.md) |
| 2026-07-27 | `dfd148fd`, `661ef732`, `4d2c4018`, `5b49172e` | 001–005 harvested ~13:02Z and adjudicated the same day (xhigh adversarial workflow, 30 agents): all five CONFIRMED, 3/3 SOUND each. x:001 floor 17,668,066,045 → [35,46]; x:002 the outer language is not tight (fifth necessary condition); x:003 OPEN-01 collapses (reduced kernel strictly finer); x:004 transport–reachability commutation (cocycle gap closed in-house); x:005 all 19 census integers reproduced, Burnside decomposition 136,514 / 2,156 / 35 → 23,842. | exchange CONFIRMED | [exchange](exchange.md), [claim-ledger](claim-ledger.md), [reachability](reachability.md) |
| 2026-07-27 | `b3b57e51` … `9b6ff34d` | 006 (double-sent by two operators ~28 s apart; count corrected; single-operator rule adopted) CONFIRMED: disjoint two-void family 19,245,318,365, combined floor 36,913,384,410 → [36,46]. | exchange CONFIRMED | [exchange](exchange.md), [reachability](reachability.md) |
| 2026-07-27 | `0727f9db`, `407af451`, `4f4550b5` | 007 CONFIRMED: filtered outer census 33,297,009,347,414 < 2^45 ⇒ ceiling 45, interval **[36,45]** (REACH-19). 008 CONFIRMED with a **2/3 SOUND panel** (one referee UNVERIFIABLE, no defect): the no-void slice saturated at exactly 624,892,870 (REACH-20). The 2/3 travels with the result. | exchange CONFIRMED | [reachability](reachability.md), [claim-ledger](claim-ledger.md) |
| 2026-07-27 | `f65d7ee6`, `43d7831d`, `460a1ac6`, `41555382` | Home rewritten as the front door (five layers, tier ladder); math pages get one-line owns headers; the lineage page (mk5, the champion, the wall). | process | [Home](Home.md), [lineage](lineage.md) |
| 2026-07-27 | `a4fff49c`, `b22d4b3d`, `fb8c3b99`, `9cdcc5e0` | INV-6 enforcement tests; the evening player v0 (fixed-field Monte Carlo best response, `verify_player` transcript, frozen value 970 as a rob-internal determinism freeze); the HTML game inspector with trumps and URL-hash state. | rob receipt / process | [rob](rob.md), [analysis](analysis.md) |
| 2026-07-27 | `8ea2ff24` (per the Lean survey) | `lean/` created: Lake project, mathlib pin v4.33.0-rc1. | process | [lean](lean.md) |
| 2026-07-28 | `48107b6f`, `0c7f03fb` | Player Brief 01: rob as exact endgame plan solver over the fiber, amended the same day to whole-hand play from trick 1 under a budgeted exact window. | process | [rob](rob.md) |
| 2026-07-28 | `ae6af953`, `392bdee8`, `4140d203`, `6d3409f1` | rob player track green: P1 GreedySigma field policy, P2 boundary position corpus, P3 exact dual-engine W0 solver, P4 rob at the table beats the baseline, P5 the contingency book. `verify_rob.txt` is the twelfth receipt; no receipt has been regenerated since. | rob receipt | [rob-slices](rob-slices.md), [verification](verification.md) |
| 2026-07-28 | `cf75e71f`, `e0414c06`, `0754cb07`, `4a7536ec`, `ee98cd6a`, `cc956ca5`, `27082f48` | Ablation probes (depth is the margin), the openings table, `solve_opening` gate and nickel autopsy, the σ-counterfactual rig; the analysis hub and the ideas hub (hierarchical fibers) opened. | EXPLORATORY | [analysis](analysis.md), [ideas](ideas.md), [idea-hierarchical-fibers](idea-hierarchical-fibers.md) |
| 2026-07-28 | `01d33951` | Lean K2–K3: declaration algebra and the unique trick winner. | kernel | [lean](lean.md), [lean-row-index](lean-row-index.md) |
| 2026-07-28 | `0f63f760`, `9167fd09` | `CLAUDE.md` per-session rules and `QUICKSTART.md`. | process | [../QUICKSTART](../QUICKSTART.md) |
| 2026-07-29 | `8ea14e8a`, `6f7d3469` | Hierarchical fibers round 1: rung 1 built, rung 2 falsified, rung 3 measured; `rob_bridge` foreign-harness seat. | EXPLORATORY | [idea-hierarchical-fibers](idea-hierarchical-fibers.md), [field/Home](field/Home.md) |
| 2026-07-29 | `95ffcb24` and same-day commits | Lean Layer A complete (BEATS/threat, witnesses, 2↔3 transport), Layer B core, Layer C stage 1, the losslessness theorem, reachability and certified states, the reduction kernel and finite belief layer. | kernel | [lean](lean.md) |
| 2026-07-30 | `e3f15d89`, `828bcf25`, `a962f8ad` | **First contact**: rob's evening player loses to the mk5 E[Q] champion, 525/1152 games (45.6%), McNemar z = −6.52; ~180k decisions rules-conformant; mid-hand dead heat. CC0 licence. | EXPLORATORY (field measurement) | [field/first-contact](field/first-contact.md), [lineage](lineage.md) |
| 2026-07-31 | `54fb2acc` | Lean: the support normal form (PA-D01–D05) and strategic sufficiency. | kernel | [lean](lean.md) |
| 2026-07-31 | `ddd2f96a` (PR #1 `50e12177`, 2026-08-01) | Retrograde rank probe round 1: substitution quotient holds at the endgame, zero divergences. | EXPLORATORY | [idea-retrograde-rank](idea-retrograde-rank.md) |
| 2026-08-01 | `0f22c4dc` … `3eaacfee`, `ae9e4bb4` | Exchange batch 2: 009 **PARTIAL** (C1 proof chain 3/3; backward commutation REFUTED-confirmed), 010 CONFIRMED (realizable = reachable at k = 1; 31,197 witnesses referee-replayed), 012 CONFIRMED (carrier staircase, Burnside), 011 an honest refusal that caught a dispatch spec error, 013/015 Lean Stages 1–2 GREEN after local repair, 014 informal (unadjudicated). Jason retires the fixed lifetime cap. | exchange CONFIRMED / PARTIAL; kernel (Stages 1–2) | [exchange](exchange.md), [claim-ledger](claim-ledger.md) |
| 2026-08-01 | `fb6d1721`, `4cf3b464`, `cd51ce2e` (PR #3 `4c2d072a`) | Constellation vocabulary rework; k = 1 census (2,211,300, zero collisions); k = 2 probe (817,896 checks, zero divergences). **The last change to rob's code.** | EXPLORATORY | [idea-retrograde-rank](idea-retrograde-rank.md), [rob](rob.md) |
| 2026-08-02 | `d190b264` | Lean: the 90-world witness (PA-E10) — **priority-0 scoreboard 42/42 kernel-proved**. | kernel | [lean](lean.md), [lean-row-index](lean-row-index.md) |
| 2026-08-03 | `0d1bf7ea`, `eec4e3be`, `df33387f`, `10122fb1`, `fba7de56`, `d8e779ad`, `988faa07` | Lean P0 merged to `main`; wiki synced to 42/42; the informal-capture convention; fixed-cap framing retired in docs; staleness sweep; the seat-context idea page. | process | [lean](lean.md), [idea-seat-context](idea-seat-context.md), [exchange](exchange.md) |

### A2. walt's research arc on frozen bases (08-09 → 08-16)

All EXPLORATORY. Sessions S1–S6n are dated in the era pages; the commits below
are the anchors that survive in `main`'s history (most session work arrived
through PR #4 `b54f2f3e`, 2026-08-16, and PR #5 `44f321d5`, 2026-08-18).

| Date | Commit | What it established | Owner |
|---|---|---|---|
| 2026-08-09 | `b3cb5230` (exp5 probes); S1–S3 | Reset 1: the basis frozen at `unified_information_geometry_v0.4.md`; walt-core and walt-kernel as separate crates (unique winner over 737,100 tricks × 9 declarations, receipt replay of rob's hands); geometry, PI minimax, the trick-6 census; H/C/F operators, information prices, the fiber of 1,680. | [walt-foundation-era](walt-foundation-era.md) |
| 2026-08-10 | `9357536e` (exp3A rescue), `0995212b` (Jason's §12.6A), `60cb15f9` (implementer's guide) | S4 control skeleton and the first-class negative (only world-reconstructing skeletons are lumpable); Reset 2 the lesson factory (S5a–S5c: label fragility discovered); Reset 3 to the lossless count-free equivariant quotient; the **NO-RESCUE** policy adopted; the census-fork rulings F1–F7. | [walt-factory-era](walt-factory-era.md), [walt-census-era](walt-census-era.md), [walt-math-structure-transport](walt-math-structure-transport.md) |
| 2026-08-11 | (S5f–S5k, in PR #4) | The seat census answered by proof: Corollary S-rigid — the first-play seat quotient is the identity, COUNT 1 = C(28,7) = 1,184,040; fiber-crush, fiber-refinement and endgame-store programs run to their negatives (the class DAG is not an accelerator; zero bite; the tablebase loses). | [walt-census-era](walt-census-era.md), [walt-pre-pivot-results](walt-pre-pivot-results.md), [walt-negative-results](walt-negative-results.md) |
| 2026-08-12 | `53b903cc` | Predictive algebra v0.6 filed and audited (Gate B REFUTED: dim V^val reaches full rank 1,680 at grade 3); policy geometry (7/9 singleton, verdict STOPPED — the dissent travels); the deadness rulings J-A1..A18. | [walt-s6-era](walt-s6-era.md), [walt-math-deadness](walt-math-deadness.md) |
| 2026-08-13 | `8ee1c9e5`, `55855c87` | The decision-sparse parent filed verbatim and audited (DS-A1..A36; "certificate" barred forward); the deadness run (174,250,255 detector calls, zero false positives); the separation probe (three grade-3 coordinates SEPARATED). Same day the rob/Lean documentation pass corrects the receipt count to twelve. | [walt-decision-sparse](walt-decision-sparse.md), [walt-math-decision-sparse](walt-math-decision-sparse.md), [verification](verification.md) |
| 2026-08-14 | `6dcd7fdf` (016 cleared), `08f1b615` (FC emission) | S6e–S6n in one day: trick-1 draw (287 first-trick plays exact over all 399,072,960 worlds), lay downs (exactly 301 per declaration; no four-laydown deal), the n = 4 pass, the fusion-tax and second-rung probes from hand-ferried x:016/x:017 (adjudicated FT-A1..A29, SR-A1..A37 into walt's tier), the feature-fee audition and fee-correlation chapter; x:018 staged (still unanswered). | [walt-s6-era](walt-s6-era.md), [walt-math-intakes](walt-math-intakes.md), [exchange](exchange.md) |
| 2026-08-15/16 | (SS-A1..A18) | The seed survey: 100 seeds, association NOT reportable; the U prediction refuted. | [walt-s6-era](walt-s6-era.md), [walt-negative-results](walt-negative-results.md) |
| 2026-08-16 | `c230949c`, `3b4c6d60` (PR #4 `b54f2f3e`) | GPU-native trick-1: the Pro implementer's guide intaken (GT1-A1..A9, freeze 55); portable M0/M1 COMPLETE with `Trick1Foundation.lean`; Metal Gate 0 NO-GO. | [walt-gpu-native-trick1](walt-gpu-native-trick1.md) |

### A3. The pivot to play (08-17 → 08-23)

All EXPLORATORY unless marked.

| Date | Commit | What it established | Owner |
|---|---|---|---|
| 2026-08-17 | `813d5e81`, `a6df853c`, `20a9fecc`, `e6cd9586`, `97ce321a` | M2 Metal parity gate implemented and closed under freeze 56 (`a6df853c` is the immutable closure commit); the M3 perfect-recall-net gate frozen (freeze 57 — a gate, no result); M3 scaffolding committed WIP "does not build". | [walt-gpu-native-trick1](walt-gpu-native-trick1.md), [walt-math-freezes](walt-math-freezes.md) |
| 2026-08-17 | `171cd22c` (ladder), walt-math-12 ruling | **pmake ruled the objective**: P(make the bid); trick differential a proxy. Devices P1–P4 (decided cutoffs, viewer early exit, pmake key reduction, gcd-normalized posteriors) ruled SOUND; the path-dependence counterexample (posterior 1:1 vs 3:4 on the same reduced key). | [walt-seat-play](walt-seat-play.md), [walt-math-reference](walt-math-reference.md) |
| 2026-08-17 | `c585b031`, `20a95221`, `178722e8`, `f5fff915`, `30f14098` | **The seat plays**: scenario solver → level-1 solver (the field becomes level-0 minds; 5-5 survives) → level-2 solver (rayon port exposes and fixes the PiKey banked-aliasing defect) → divergence miner. | [walt-seat-play](walt-seat-play.md) |
| 2026-08-17/18 | (PR #5 `44f321d5`) | **Arena**: walt level-1 beats the mk5 E[Q] champion under the dropped-30 3×384 protocol — pooled 630/1152 games (54.7%, game-level z = +3.18), McNemar z = **+6.28** over 6,015 paired contracts (`walt/probes/m3/arena_results_2026-08-17.txt`; pre-PiKey-fix binary, internally consistent). An arena outcome about play, never a value statement. | [walt-seat-play](walt-seat-play.md) |
| 2026-08-18 | `1fc23196`, `e9b8c263`, `f8121ee7`, `eaf9b23a`, `94b3df30` | Banked PiKey fix applied to the play binaries (new baseline); `walt-wasm` browser oracle for plunge; `bidcurve` P(make b) substrate; signed-pivotal geometry intaken and adjudicated (SP-A1..A12). `SCENARIO-PLAYER.md` written after the build. | [walt-seat-play](walt-seat-play.md), [walt-math-intakes](walt-math-intakes.md) |
| 2026-08-19 | `f60b2e33`, `9a056f20`, `cef13367` | Race-then-refine shipped as opt-in; θ calibrated to 11/16 (first zero-overbid rung of the 200-hand bidcurve corpus) and made the walt-wasm/webtable default — **the `walt.wasm` at `9a056f20` is the artifact plunge deploys and the partnership program's "phone"**; the race-vs-full arena gate: a strength dead heat. | [walt-seat-play](walt-seat-play.md), [walt-partnership-program](walt-partnership-program.md) |
| 2026-08-22 | plunge `1810da20` | Plunge copies `walt.wasm`/`walt.ts` into `src/ai/walt/` (play n = 40, n0 = 8, race on). | [walt-partnership-program](walt-partnership-program.md) |
| 2026-08-23 | `648f93ae` | Jason plays walt live in plunge ("How'd I do? Ask walt"); two review specimens (a 100%-saturation tie; a 40-vs-160-world flip on a count-timing choice) motivate `LEVEL2-PROBE.md`. The unification census (three stacks, three orphans, fold plan). | [walt-gran-anchors](walt-gran-anchors.md), [walt-architecture](walt-architecture.md) |
| 2026-08-23/24 | PR #6 `97321343`, PR #7 `dcdc14b0` | The phone package and signed-pivotal drop merged; the exp5 probe suite preserved. | [walt-instruments](walt-instruments.md) |
| 2026-08-24 | `ad355e93`, `fa3fe743`, `d1499d43`, `c92175ae` → PR #8 `2de8a055` | **Unification**: closure-clean deletions (the unbuildable M3 crates, `PLAN.md`); walt-factory and walt-skeleton archived with 65 summaries relocated to `walt/probes/factory-results/`; **THE FOLD** — seven crates become seven modules of one `walt` crate, trace-identical, wasm smoke 28/28; freeze-56 v2 re-issued append-only at the unified layout (FZ-A1..A6, 282-entry manifest). The base every later wave cites. | [walt-architecture](walt-architecture.md), [walt-gpu-native-trick1](walt-gpu-native-trick1.md), [walt-math-freezes](walt-math-freezes.md) |

---

## Part B — the six waves after the unification, 2026-08-24 → 2026-09-07 (in full)

Every entry in Part B is EXPLORATORY unless its tier bracket says otherwise.

### Wave 1 — the calculated-evidence era in one day (2026-08-24/25; PRs #9–#59)

Two hand-ferried parents adjudicated same day, the §22 build through step 9,
three field-swap slices, the panel batch and its producers, a playable
controller, the waking seat, the speed campaign, walt2-wasm. Owner:
[walt-calculated-evidence](walt-calculated-evidence.md).

| Date | PR · merge | What it established | Owner |
|---|---|---|---|
| 08-24 | #9 `c408b80d` | Wiki overhaul: walt is the player, rob the exact-truth exemplar; seven closed probe designs retired (bytes at `git show 2de8a05:walt/<NAME>.md`); ledgers re-synced. `[process]` | [Home](Home.md), [walt](walt.md) |
| 08-24 | #12 `c9a99df4`, #13 `019d39af`, #14 `4231cb24`, #18 `01c3f7e7` | Batch-quota protocol the only exchange framing; the math-reorg (walt-math-intakes with 17 artifacts, the reference map, the eight walt-math pages); kanban closes. `[process]` | [exchange](exchange.md), [walt-math-reference](walt-math-reference.md) |
| 08-24 | #15 `328ba023`, #16 `125cb722` | `calculated_evidence_v0.1.md` filed verbatim (SHA `9b32b14f…`), verifier 18/18; adjudicated **CE-A1..A8** — CE-T1..T5 sound, θ/ϑ split, the six-way result ladder, "a sample cap is a resource limit, never a proof rule", obligations O20–O28. | [walt-math-intakes](walt-math-intakes.md), [walt-math-information-geometry](walt-math-information-geometry.md) |
| 08-24 | #17 `c9c95e5c` | Viewer cross-fiber review (`viewer_fiber_evaluate`, webtable two-column review). | [walt-seat-play](walt-seat-play.md) |
| 08-24 | #19 `5baad992`, #20 `bf432be1`, #21 `636d306e` | `solver::evidence` + `solver::adaptive` (steps 2–3, the A.6 vertical slice); `solver::policy` FreezeTuple/PolicyId (step 4); `solver::controller` m-candidate controller with safe elimination and exact endpoints (steps 5–6). | [walt-calculated-evidence](walt-calculated-evidence.md), [walt-architecture](walt-architecture.md) |
| 08-24 | #22 `ed082963`, #23 `9dd89f17` | `targeted_level2_field_stability_v0.1.md` filed (SHA `597d33c3…`), model-checked 19/19; adjudicated **L2-A1..A7** — level 2 is a calculated refinement, never a universal re-solve; a level-2 result is a best response to a named σ1, never "equilibrium"; the Gran anchors G1–G4 carded. | [walt-math-intakes](walt-math-intakes.md), [walt-gran-anchors](walt-gran-anchors.md) |
| 08-24 | #24 `0794ff86` | Step 7: `bin/shadow` beside the live player — 183 decisions shadowed (70 receipt, 113 driven) at cap 128 (`walt/probes/shadow/README.md`). | [walt-calculated-evidence](walt-calculated-evidence.md) |
| 08-24 | #25 `31b6a885` | The three Gran Plunge screenshots archived at `~/data/texas-42/gran-anchors-2026-08-24/` with `MANIFEST.sha256`; no-seed reconstruction carded. | [walt-gran-anchors](walt-gran-anchors.md) |
| 08-24 | #26 `ffdc002b`, #30 `ca0483d7` | Field-swap slices 1–2: `solver::field` + `solver::exposure`, fixed-policy smoke; exposure rungs E0–E2, exact split-reach E4, the L2-T4 admissible screen (`walt/probes/fieldswap/`, `fieldswap_screen/`). | [walt-calculated-evidence](walt-calculated-evidence.md) |
| 08-24 | #27 `70799503`, #28 `d22e03b8` | The era page and the LOG entry. `[process]` | [walt-calculated-evidence](walt-calculated-evidence.md) |
| 08-24 | #29 `74b26d29`, #35 `5ef7975d` | Five adversary-panel briefs drafted; x:019–023 hand-ferried as one authorized batch and the consolidated response adjudicated same day at **PANEL-A1..A8** — Claim D counterexampled (169/512 > 1/4), W7–W11 adopted, Part VI cancellation mathematics adopted, the lift corrected 41/1200 → **31/1200**. | [exchange](exchange.md), [walt-math-intakes](walt-math-intakes.md) |
| 08-24 | #31 `e5a5f521`, #33 `07d44154` | Step 8: `solver::calibrate`, the V5 flip fixtures, the cap-ladder law, per-pair E0 calibration (`walt/probes/step8/`); wiki sync. | [walt-calculated-evidence](walt-calculated-evidence.md) |
| 08-24 | #32 `6e00528d`, #34 `039b11df`, #36 `80f94a0e` | Jason's cap ruling: shadow `world_cap` 128 → 512 (128 was the phone budget); 512-epoch receipt and driven reruns — three settlements, the proxy forecast retracted, anti-live in trick 2. | [walt-calculated-evidence](walt-calculated-evidence.md) |
| 08-24 | #37 `23ba1c22` | The playable controller player (`solver::act`, `controller_bridge`, `ctrl [cap=N]` seats, the O27 deal/belief RNG split; `walt/CONTROLLER-PLAYER.md`) — a capability, not a comparison: no arena run on it. | [walt-seat-play](walt-seat-play.md), [walt-instruments](walt-instruments.md) |
| 08-24 | #38 `151ea4f3`, #39 `162f8081`, #41 `7f15763f` | Slice 3: the cancellation ladder, pairwise masses, directional rungs; Λ(pin-5-5, pin-3-3) = 31/1200 asserted in-gate (`walt/probes/fieldswap_cancel/README.md`); first `FieldDecisionChanged` / `FieldStableExactRoot` / `Dominated`. | [walt-calculated-evidence](walt-calculated-evidence.md) |
| 08-24 | #40 `4c452fe2`, #42 `d8a2d70e` | Dispatch x:024 (the deferred-producers triple) drafted and shipped, hand-ferried; the count reaches 24 and has not moved since. `[process]` | [exchange](exchange.md) |
| 08-25 | #43 `60f01dcd`, #47 `51eac3fc` | x:024 response adjudicated **TRIPLE-A1..A7** (max-preserving upper CS; the Hazard-Exclusion Invariant; six-motif first-split morphology). Last edit of `claim-ledger.md` to date. | [exchange](exchange.md), [claim-ledger](claim-ledger.md) |
| 08-25 | #44 `43967f30`, #45 `668c5fb6`, #46 `cbce1ae3` | All three producers built the same night: slice 4c `solver/motif.rs` (453 correction worlds, residual 0), 4a `solver/upper_cs.rs`, 4b `solver/hazard.rs` (40 pairs, 0 accepts, 40 declines). | [walt-calculated-evidence](walt-calculated-evidence.md) |
| 08-25 | #48 `0224471f` | Panel-response conformance audits: eight gates, all CONFORMS, the W10 vacuous-conformance caveat (`walt/audits/panel_response_conformance.md`). | [walt-calculated-evidence](walt-calculated-evidence.md) |
| 08-25 | #49 `43017541`, #50 `6110b163` | Step 9: `solver::wakeup`, the level-2 detection layer (CE-A6/L2-A5), run on the predeclared corpus (`walt/probes/step9/`). | [walt-calculated-evidence](walt-calculated-evidence.md) |
| 08-25 | #51 `68f9d040`, #52 `0b6094da` | `solver::targeted` — the assembled targeted field-1 controller; E4 never paid on the first corpus (`walt/probes/l2_controller/`). | [walt-calculated-evidence](walt-calculated-evidence.md) |
| 08-25 | #53 `aff53213`, #55 `a28ddcef`, #56 `5bead5c8` | The speed campaign: reorder-not-cull (E-A15), the two surgical levers (cached field in `act`, decided cutoff in the per-world replay), the bundled world evaluator — verdict "the modeled minds are the bill"; the probe READMEs, not the parent's summaries, are the authority (CBS-A8). | [walt-calculated-evidence](walt-calculated-evidence.md), [walt-instruments](walt-instruments.md) |
| 08-25 | #54 `93d99563`, #57 `586588da` | **The waking seat**: `solver::waking` + `waking_bridge`, wake-gated σ1 escalation; the two-hand natural-play profile — σ0 baseline 729‰ / wake check 269‰ / escalation 1‰ of wall (`walt/probes/waking/README.md`). | [walt-seat-play](walt-seat-play.md), [walt-gran-anchors](walt-gran-anchors.md) |
| 08-25 | #58 `33d541fe`, #59 `a1d2219b` | `walt2-wasm`: the level-2 browser oracle for plunge. | [walt-instruments](walt-instruments.md) |
| 08-29 | — | The era closes (LOG and the era page); no commits on `main` between 08-26 and 08-29. | [walt-calculated-evidence](walt-calculated-evidence.md) |

### Wave 2 — counted belief and the anytime proof state (2026-08-30/31; PRs #60–#78)

Owner: [walt-counted-belief-era](walt-counted-belief-era.md). The mathematics
by idea: [walt-math-reference](walt-math-reference.md).

| Date | PR · merge | What it established | Record |
|---|---|---|---|
| 08-30 | #60 `e4c10ead` | `counted_belief_sandwich_v0.1.md` filed verbatim with verifier (20/20); adjudicated **CBS-A1..A9** — "sandwich" retired as an object name, **root interval** and **survivor set** adopted; the O34 strategy-fusion fence restated; CBS-O1..O15 to the Lean side-project ledger (no Lean file exists). | `walt/math/counted_belief_sandwich_v0.1_intake.md` |
| 08-30 | #61 `a99fcb9f` | Slice A: root intervals, survivor sets, the pmake empirical-max upper; `walt/FACTOR-BELIEF.md` opened as the running record. | `walt/probes/root_interval/README.md` |
| 08-30 | #62 `eba8c940` | Slice C0: the factor belief and `ExactCoverOracle` (backend zero) — the opening root's exact branch table in **8.7 ms** over the 399,072,960-world fiber via 116,280 acting-seat hands. | `walt/probes/factor_belief/README.md` |
| 08-30 | #63 `2d6eb442` | Slice B: the two-policy grammar, the residual split, the §8 identity made mechanical. | `walt/probes/grammar_residual/README.md` |
| 08-30 | #64 `bd86c18a` | Slice C1: the cache study — once-per-state laws hold; cross-history cache reuse is exactly 0 (the record is in the key). | `walt/probes/factor_belief/cache_run1.txt` |
| 08-30 | #65 `8d260844` | Slice D: the factorized fixed-policy recursion (`SupportOracle`, the §23 make-mass recursion V = M/Z), parity with the bundled walk at every gated root. | `recursion_run1.txt` |
| 08-30 | #66 `8bea94ca` | Slice C2: the opening-root report — all seven §46 coordinates in one run. | `c2_run1.txt` |
| 08-30 | #67 `1cae2419` | Slice E: the factorized grammar best response — at trick-4 depth the grammar mix strictly beats every single source. | `response_run1.txt` |
| 08-30 | #68 `246cb260` | Slice F: consequence CEGAR (§28 hand classes, §30 witness pairs) — mass concentrates but the tail fragments. | `cegar_run1.txt` |
| 08-30 | #69 `25b40d9f` | Slice G: `solver/refine.rs`, the integrated refinement controller — **the C→G program COMPLETE in one day**; the opening root returns honest UNRESOLVED at the affordability cliff. | `refine_run1.txt` |
| 08-31 | #70 `aed2a38b` | `anytime_proof_state_score_v0.1.md` intaken, verifier 36/36; adjudicated **APS-A1..A9** — the score layer, the proof-bar/executable-bar split, **certified regret Γ = U* − B_exec** as the finite-budget deliverable, the typed laydown hierarchy, `refine.rs` frozen as the RefineV1 reference. | `walt/math/anytime_proof_state_score_v0.1_intake.md` |
| 08-31 | #71 `8545bab1` | Phases 0+2: **freeze 58** (RefineV1 semantic freeze) and the exact 43-bin score profile (one run buys the whole bid-threshold curve). | `profile_run1.txt`; [walt-math-freezes](walt-math-freezes.md) |
| 08-31 | #72 `d6320b36` | The §49 spike PASSED: `solver::proof_state`, facts-only authority with derived views and an open producer registry; RefineV1 reproduced as the frozen oracle. | six gates |
| 08-31 | #73 `a1b24d09` | Phase 3: contract projection and certified regret — `recommend()`; Γ = 0 at h5-t6 (floor 444‰); Γ = 83‰ at h3-t4 (floor 267‰, upper 350‰). | `proofreport_run1.txt` |
| 08-31 | #74 `1586ff78` | Phase 6: argmax extraction and residual policy bounds — the h3-t4 gap closed **exactly** (83‰ → 0‰; the recommendation switches 4-4 → 3-1). | `extractreport_run1.txt` |
| 08-31 | #75 `1ceadb7c` | Phase 1: the work frontier — declared goals, Z/3Z costs, the §42 assert per purchase. | `frontierreport_run1.txt` |
| 08-31 | #76 `1ee6669a` | Phases 4+5: the residual Bellman staircase and count-threat covers; first-generation covers vacuous at rich roots (§70). | `bellmanreport_run1.txt` |
| 08-31 | #77 `a8987fd1` | Phase 7: the typed laydown hierarchy (Fixed/Exists/All), deterministic-only producer; h10-t6 a real receipt-root Laydown witness. | `laydownreport_run1.txt` |
| 09-01 | #78 `e14c1b35` | Phase 8: the §65 opening-root iterative run — **play 6-5, floor 732‰, at most 267‰ unclaimed, honest UNRESOLVED at ε = 1/4**; the sampled tier plateaus at p = 512. **PHASES 0–8 COMPLETE.** | `openingreport_run1.txt` |

### Wave 3 — the doom census, book-one closing intakes, model belief, the unified player (2026-09-01 → 09-03; PRs #79–#87)

Owner: [walt-focal-horizon-era](walt-focal-horizon-era.md) (MB0 onward);
[walt-counted-belief-era](walt-counted-belief-era.md) (the doom census and
first-pass wiki). Reports of record are under `walt/briefs/` from #81 on.

| Date | PR · merge | What it established | Report of record |
|---|---|---|---|
| 09-01 | #79 `eb5a459d` | The doom census (`solver/doom.rs`): counterexample mass as deterministic uppers, the ∀-fail dual of the laydown hierarchy. Its ledger sentence "the remaining Γ ≈ 267‰ is overwhelmingly the info-consistency price" was **corrected 2026-09-03** in `walt/DISCREPANCIES.md`: the split of the 267‰ is UNKNOWN. | `doomreport_run1.txt`; `walt/DISCREPANCIES.md` |
| 09-01 | #80 `08fe3d2d` | First-pass wiki synthesis of the counted-belief/anytime era. `[process]` | [walt-counted-belief-era](walt-counted-belief-era.md) |
| 09-01 | #81 `c8110ea6` | Book-one closing intakes: the model-belief base player (**MB-A1..A8**, verifier 40/40; the rung table D / F₀ = σ0 / F₁ / F₂ corrected at intake) and the salvation complex (**SC-A1..A8**; 1 − Q = minimum transversal; the fusion horizon an empirical object first). `walt/briefs/` convention begins. | `BRIEF-MB0.md`, `BRIEF-U0.md` |
| 09-01 | #82 `90255f52` | MB0: `solver/model_belief.rs`, the exact vertical slice over Ξ = Ω×Θ; Φ = 0 at fourteen t5/t6 coordinates; the σ1 sampler hazard discovered; three builders collided. | `MB0-COLLISION-NOTES.md`, `MB0-HANDOFF-BUILDER2.md`; `modelbelief_run1.txt` |
| 09-02 | #83 `161b0195` | σ1-repair: the legacy sampler's empty acceptance region terminated (`sample_belief` returns a typed `InfeasibleFrame`); five copies deduplicated onto the library. | `BRIEF-SIGMA1-REPAIR.md` |
| 09-02 | #84 `c5d2f32a` | U0: the God-gap census (`solver/godgap.rs`) — 1 − V = d_phys + d_info + d_policy; **the fusion horizon measured at trick 5** on the receipt corpus (a measurement, never a theorem, SC-A4); twelve t4 prices 6–22‰ with d_policy = 0; the opening root typed `UnknownGodGap`. | `U0-REPORT.md`; `godgap_run1.txt` |
| 09-02 | #85 `bfbb45ce` | MB1: `solver/model_recursion.rs` — **the model-fusion price is strictly positive at trick 4**: Φ = 38/9600 at h8-t4 3-1 (8323/9600 vs 8361/9600), pinned by gate M6; h8-t3 refuses at 7M reads. | `MB1-REPORT.md`; `modelbelief_recursion_run1.txt` |
| 09-02 | #86 `3b4105ca` | UP0: `solver/unified.rs`, the unified player — one five-tier decision cascade, provenance always; 99.4% of the lean-rung wall spent carrying an unread posterior; two pinned argmax flips; the declared type library falsified by the player's own play (UP0-REPORT §4). Not the live default; no arena result. | `UP0-REPORT.md`; `unified_run1.txt` |
| 09-03 | #87 `a80b9829` (`62abe028`) | UP1a the lazy carry (2.1 s → 0 µs) and U0b the in-solve horizon census (`solver/horizon.rs`): **h8-t3 solved exactly, Q* = 28859/29988 (962‰), argmax 1-1, 289,407,472 field reads, 14 min 13 s**; the trick-5 frontier is NOT fusion-free inside a trick-4 solve (13–14‰); a trick-6 cut is 0–7‰ yet flips the play twice. | `UP1A-REPORT.md`, `U0B-REPORT.md`; `horizon_run1.txt`, `unified_run2.txt` |

### Wave 4 — the focal-horizon program and the Gran anchor (2026-09-04; branch `walt-fh` → PR #88, merged 2026-09-07 by fast-forward)

All on 2026-09-04. Owner: [walt-focal-horizon-era](walt-focal-horizon-era.md)
(FH), [walt-gran-anchors](walt-gran-anchors.md) (Gran). The `walt-fh` commits
are in `main`'s first-parent line; GitHub records PR #88 MERGED
2026-09-07T06:17Z with merge commit = branch head `a0d594b2` (fast-forward,
no merge commit).

| Commit | What it established | Report of record |
|---|---|---|
| `4cefa5a2`, `4a41e831`, `405b523b` | FH0: `focal_horizon_sandwich_v0.1.md` filed verbatim (SHA `892bc343…`); intake and walt-math review — **FH-A1..A11**; Theorems 1–6 proved in full in the companion; five delivered propositions FH-God / FH-int / FH-tie / FH-cut / FH-last (trick-4 roots exact at k = 2 because trick 7 is forced). | `BRIEF-FH0.md`; `walt/math/focal_horizon_sandwich_v0.1_intake.md` |
| `4c9df5a0`, `1e213bdb` | FH1: `solver/focal_horizon.rs` — three instruments (U0, U0b, extraction) become one refinement object `[L_k, U_k]`, k ∈ {0,1,2}, affordable-or-refuse; 10 gates. Finding: at k = 1 the residual width is the tail's policy gap (Q − L 9–41‰), not fusion price (U − Q 0–3‰). | `FH1-REPORT.md`; `focal_run0.txt` |
| `8efe6923` | **The background-work wedge becomes a CLAUDE.md rule** (gates finished 02:11, the agent silent five hours). `[process]` | `CLAUDE.md` Agents |
| `bbcdb709`, `1c66bb02`, `e53752b5`, `508cc4af` | CI1: the gate-sizing rule; concurrent test binaries (121, gate wall 367 s) then recompute-once fixtures (367 → 230 s); `[[gate-corpus-trim]]` carded. `[process]` | `BRIEF-CI1.md`, `CI1-REPORT.md` |
| `f132f4c9`, `dc515ac0` | FH2: `solver/focal_ladder.rs` — budgeted passes that stop and resume on a store of node facts (FH-int), exact suffix reuse; k = 2 reads 2.83M → 0.42M at h3-t4; memory 411 → 662 MB; 9 gates (a tenth, FH-C, added by FH5). | `FH2-REPORT.md`; `focal_ladder_run1.txt` |
| `2265f819`, `fc171e1f` | FH3: **the report of record** `focal_run1.txt` at 33 (root, contract) coordinates × k ≤ 3 — every live trick-4 coordinate settles by k ≤ 2 (5/6/3 at k = 0/1/2; Γ₁ ≤ 45‰); h8-t3 settles only at k = 3 (survivors 5/5/3/1, Γ 141/100/34/0‰); U0b's ply-cut flips live entirely on the upper side, no wrong action ever certified; peak RSS 19.4 GB, 3.82M facts; four anchor gates; gate wall 230 → 308 s. | `FH3-REPORT.md`; `tests/solver_focal_anchors.rs` |
| `2e6b678d`, `608c1feb`, `5003543e` | `walt/MAP.md` written — walt on one page (objects, costs, the tree-shake list, the trend); `[[ladder-policy-store]]` carded. `[process]` | `walt/MAP.md` |
| `07f0e34e`, `8aae7c79` | FH4: the independent audit — **one BLOCK (FH-A2: "sandwich" as an object name in a gate name, the ledger and FH3-REPORT), thirteen NOTEs**; every record reproduces; `check.sh` honest at 306.65 s over 123 binaries; the anchors gate 18.2 GB standalone. | `BRIEF-FH4-AUDIT.md`, `FH4-AUDIT.md` |
| `b6de5a25`, `5bc5a068`, `2a4f17f0`, `f098c4df`, `a0d594b2` | FH5: post-audit fixes — B1 vocabulary ("containment", never "sandwich" as a name), N3 the in-pass cap-refusal gate, N13 the anchors-fixture memory cap (18.2 → 8.8 GB), the report and ledger numbers, MAP rows. **Jason's ruling: no new mathematical parent until a consolidation slice lands.** | `FH2-REPORT.md` (N3), `walt/MAP.md` |
| `35d76796`, `a2623bbe` | The response to Pro drafted for hand-ferry (not a numbered dispatch; the count stays 24); the LOG entry. `[process]` | `FH-RESPONSE-TO-PRO.md` |
| `32aa14f1`, `8174fa83` (branch `walt-gran`) | **Gran G1** transcribed and mechanically validated by rules replay (`bin/granrun.rs`, `walt/probes/gran/g1.receipt.txt`); the waking seat plays the real 6-4 hand from the opening lead — agreement with the record 6 of 7, the one wake at trick 5 reproduces the human play; G2/G3 committed as validated partial. | `walt/probes/gran/README.md` |

### Wave 5 — the morning readout and the two unmerged branches (2026-09-04 evening → 09-05)

Owner: [walt-gran-anchors](walt-gran-anchors.md). The readout is on `main`;
the measurements it summarizes are on branches that are **not**.

| Date | Commit(s) | What it established | Where |
|---|---|---|---|
| 09-04 22:10–22:52 | `0b65efb9` … `00633c0c` (branch `walt-o5`) | O5 stages 1–5: the dead-sample census (exact, no solver change), void-aware inner minds behind a typed epoch marker, gates one-coordinate-per-law with pinned strictness witnesses, the `o5flip`/`o5level1` probes, the probe record and the `inner-voids` card. | branch only |
| 09-04 22:38 → 09-05 00:39 | `2d3907bd` … `6abdd78f` (branch `walt-g1-l2`) | Level 2 on the real Gran G1 record keeps the 6-4 at both legal nodes (trick 3 exact: 5-2 654 vs 6-4 640); G2 at Gran's seat — the trick-1 6-4 was forced, exact lock from trick 3, identical tie sets at both levels; the constructed synthetic lock; the over-claim ("saturation and the 6-4 mattering are mutually exclusive") withdrawn in-record. | branch only |
| 09-05 00:05 → 00:42 | `331fcfbc`, `43790099`, `2981e090` (branch `walt-o5`) | The mirrored match first reported a DEAD HEAT; a label defect fixed (P(declaring side makes), no decision changed); **the match seed lacked the deal index** — repaired and rerun, the verdict changed (live epoch 33/19/20 pairs aware, +262 of 6048; reduced epoch 76/83/33 blind, −226 of 16128); "dead heat" WITHDRAWN; the void flag stays off. | branch only |
| 09-05 02:09 | `9d6a5a2e` (on `main`) | **The 6-4 readout**: G2 exactly locked from trick 3 (all 280 deals at trick 4 make whatever Gran plays; level 1 gives the identical tie set — at an exact tie P(make) has no gradient); tie-break = `TieRule::LowestTileIndex` + `best_of` keeps the incumbent, so count is hoarded deterministically at ties (a source reading, not executed); design conclusion: another rung is not the remedy — the levers are the objective and the tie-break at exact indifference; Jason's open calls (A)–(F). | `walt/briefs/MORNING-2026-09-05.md` |
| 09-05 | (packet notes dated) | The Pro packet notes `TEXAS42-UNIFIED-REVIEW-v0.1` and `TEXAS42-IMPROVISATION-v0.1` are dated; neither has been intaken. | [walt-math-intakes](walt-math-intakes.md) |

### Wave 6 — the partnership program, Scheme, and the gym under a CI waiver (2026-09-06/07; 31 commits `d8400713..c00717d1`, fast-forwarded into `main`)

A Codex session on branch `codex/partnership-launch` (from `9d6a5a2e`), landed
by plain fast-forward. **The full Rust gate was deliberately waived for the
session** (`experiments/partnership/SCOPE.md`: "Full Rust CI is deliberately
waived for this experiment; focused checks replace it"; every LOG entry
repeats "full CI remains waived") — see Part D. Owners:
[walt-partnership-program](walt-partnership-program.md),
[walt-scheme-fix](walt-scheme-fix.md), [walt-gym](walt-gym.md).

| Date | Commit | What it established | Record |
|---|---|---|---|
| 09-06 | `d8400713` | The launch packet preserved byte-for-byte (6/6 SHA-256). | `experiments/partnership/packet/` |
| 09-06 | `cfb0fb25`, `f5b6e11d` | The bounded partner-aware player (`solver::partnership`: partner at L1, opponents at L0; 40/8/2 samples; 14 s wrapper) with the archived phone WASM as reference (SHA `af0200af…`) and an independent Python referee; the launch batch (three fresh deals: 0 favorable / 1 unfavorable contract flips / 5 unchanged; 12 hands, 336 driven decisions all legal) and a reproducible defensive regression fixture. | `README.md`, `SCOPE.md`, `BASELINE.md`, `REPORT.md` |
| 09-06 | `d60bad42` … `766168b7` | Resumable parallel campaign runner with independent replay verification; the 100-seed random campaign — 300 games, **15 favorable / 23 unfavorable / 156 ties** (no strength gain). | `campaigns/random-420600-699/RESULTS.md` |
| 09-06 | `755d1869`, `1bab636b`, `13255635`, `fe17548c` | Shared ten-game pool (2.96× on the benchmark); the fixed-hand hidden-world panel (10 hands × 10 completions; 21/27/152 per the survey map — `WORLD-RESULTS.md` not re-read in this pass). | `POOL.md`; `campaigns/worlds-520600-699/WORLD-RESULTS.md` |
| 09-06 | `dbcc698f` | Selectable void-aware inner beliefs on `main` (`InnerBelief::{Voidless, VoidsCounted}`; `Key`/`PiKey` carry a voids coordinate); nine pre-change decisions reproduced exactly; default stays Voidless. | `INNER-BELIEF.md`; `walt/SCENARIO-PLAYER.md` O5 row |
| 09-06 | `548ab6e8`, `c51ff020` | Native L1 vs the phone calibration: 150 games in 202.12 s, **8 favorable / 15 unfavorable / 77 ties** (−7.0 pp); **the phone artifact identified byte-for-byte as texas-42 `9a056f20` (2026-08-19)**; the paired head-to-head design. | `campaigns/native-l1-vs-phone-620600-649/CALIBRATION.md`, `HEAD-TO-HEAD.md` |
| 09-06 | `9236ca7f`, `e310e6ff` | The foundation: `solver/selection.rs` unifies Fixed/Refine/Race for the real root and the modeled minds; a resumable paired arena; **the foundation battery — 524 games / 14,672 verified moves in 13.74 min; `l1-race` matches the phone in 35/35 fallback-free pairs (3/1/46)**; refined partner configurations exceed the fallback gate. | `FOUNDATION.md`; `campaigns/foundation-battery/RESULTS.md` |
| 09-06 | `10950845`, `1c99cf6a`, `0b65e5b1`, `5cbede4e` | The player families named (L1 default / L2 Partner default / L2 Partner with voids; L2 All separate); **the default battery — 400 games on 100 shared deals: L2 Partner vs L1 14/14/72; voids vs without 12/17/71; 0.228 / 1.127 / 1.318 s per move**; the strength assessment: L1 default is the operating default. | `PLAYERS.md`; `campaigns/default-partner-battery/RESULTS.md`; `STRENGTH-ASSESSMENT.md` |
| 09-06 | `0be750a9`, `b764665f` | The Scheme gym assessment; **Scheme/Fix implemented as `walt::scheme`** (19 tests; "invented to compress, commissioned to express"; no player, no compression claim). | `walt/scheme/README.md`, `VALIDATION.md` |
| 09-06 | `c59f1115` | **The first exact partnership gym** (`walt::gym`, `bin/partnership_gym.rs`, `gym.py`): six count-offer exercises with exact keys and same-world replays; pupils L1 5/6, L2 Partner 6/6, L2-voids 5/6 (diagnostic only). Also repairs `factor_belief.rs::condition_via` to prune zero-completion hands before policy consultation; raw/mixture level-1 parity now holds on all six receipt roots (LOG). | `walt/gym/README.md`, `RESULTS.md`; `walt/FACTOR-BELIEF.md` head note |
| 09-07 | `b0c7c0aa` | Scheme-driven discovery: three query files over 1,929 coordinates → 176 strict memberships / **170 distinct exercises** in ~13 s; 245 above-cap states skipped, never sampled. | `walt/gym/DISCOVERY.md` |
| 09-07 | `67f1e4ab` | Outcome-only bid-making: a four-line all-legal query — **433 exact declaring exercises (367 with one best play, 26 certain make-vs-set) in 50.616 s**. | `walt/gym/BID-MAKING.md` |
| 09-07 | `75b6a3f1` | Specification-first collections: `specs/bid-making.json` regenerates the 433 in 67.707 s (367 in 13.991 s; 26 in 1.921 s). | `walt/gym/SPECIFICATIONS.md` |
| 09-07 | `1df741db` | The composed partnership/bid-making exam: 30 positions from 21 seeds; **L1 24/30 (mean regret 1643/205200), L2 Partner 26/30 (959/205200)**; 60 decisions in 1.458 s, no fallback. | `walt/gym/PARTNERSHIP-COMPOSITION.md` |
| 09-07 | `08fad726` | Scheme dynamics and persistent sampled policy synthesis: persistence saves 52.9% / 46.4% of search time at exact parity; donor composition costs 6.3% / 13.2% more than fresh. | `walt/scheme/DYNAMICS.md`, `POLICIES.md`, `COMPOSITION.md`; `campaigns/policy-synthesis-v1/RESULTS.md` |
| 09-07 | `14e01322` | The Astra relational-learning proposal packet preserved unchanged (unintaken). | `packet/texas42_relational_learning/` |
| 09-07 | `c00717d1` (**HEAD**) | The shared relational learner and information-price examiner: shared actors trail sampled tables by 5.47 pp; one hybrid fallback +0.771 pp (uncertain); the price basis tightens 0 of 64,806 bounds; 8,694 independent full-game replays pass. | `RELATIONAL-LEARNING.md`; `campaigns/relational-learning-v1/RESULTS.md` |

---

## Part C — pull requests #1–#88: merge hash, branch, report of record

Git author date of the merge on `main`; GitHub `mergedAt` (UTC) where it reads
a day later is noted in the header of this page. "Report of record" is the
brief/report under `walt/briefs/` or the probe README/record that the PR's own
subject names; "—" means the PR body and the owning wiki page are the record.

| PR | Date | Merge | Branch | Landed | Report of record |
|---|---|---|---|---|---|
| #1 | 08-01 | `50e12177` | worktree-retrograde-rank-probe | Retrograde rank probe round 1 | [idea-retrograde-rank](idea-retrograde-rank.md) |
| #2 | — | — | — | (an issue, not a PR: the focused-session handoff) | — |
| #3 | 08-01 | `4c2d072a` | worktree-constellation-language-rework | Constellations: language rework, k = 1 census, dispatch 009 draft | [idea-retrograde-rank](idea-retrograde-rank.md) |
| #4 | 08-16 | `b54f2f3e` | worktree-walt-s2 | walt S1 onward: workspace, rules core, fibers, replay-validated | era pages |
| #5 | 08-18 | `44f321d5` | walt-trick1-first-play | The seat that plays: sampling-stack player, arena win, web table | `walt/probes/m3/*_2026-08-17.txt` |
| #6 | 08-23 | `97321343` | walt-wasm-phone | The phone package and the signed-pivotal drop | `walt/TILT-AUDIT.md` |
| #7 | 08-23 | `dcdc14b0` | worktree-exp5-probes | exp5 census-curve probe suite preserved | `walt/probes/exp5/` |
| #8 | 08-24 | `2de8a055` | walt-unify | One clean walt: the fold, freeze-56 v2, kanban, the archive | `walt/UNIFICATION-CENSUS.md`, `walt/ARCHIVE.md` |
| #9 | 08-24 | `c408b80d` | wiki-overhaul | Wiki overhaul, primacy reframe | — |
| #10 | — | closed | — | superseded by #12 (same title) | — |
| #11 | — | closed | — | superseded by #13 (same title) | — |
| #12 | 08-24 | `c9a99df4` | exchange-quota-reframe | Batch-quota protocol the only framing | `exchange/README.md` |
| #13 | 08-24 | `019d39af` | math-reorg | The math is its own project: one map, intakes first-class | [walt-math-intakes](walt-math-intakes.md) |
| #14 | 08-24 | `4231cb24` | kanban-close | Cards closed | — |
| #15 | 08-24 | `328ba023` | adaptive-sampling-intake | CE parent filed and verified | `walt/math/calculated_evidence_v0.1_intake.md` |
| #16 | 08-24 | `125cb722` | adaptive-sampling-intake | CE-A1..A8 | `walt/CENSUS-RULINGS.md` |
| #17 | 08-24 | `c9c95e5c` | viewer-cross-fiber | Cross-fiber review | `walt/walt-wasm/pkg/README.md` |
| #18 | 08-24 | `01c3f7e7` | kanban-close-2 | Card closed | — |
| #19 | 08-24 | `5baad992` | calculated-evidence-slice | evidence + adaptive | — |
| #20 | 08-24 | `bf432be1` | calculated-evidence-step4 | Frozen policies | — |
| #21 | 08-24 | `636d306e` | calculated-evidence-step5 | m-candidate controller | — |
| #22 | 08-24 | `ed082963` | level2-field-stability-intake | L2 parent filed | `walt/math/targeted_level2_field_stability_v0.1_intake.md` |
| #23 | 08-24 | `9dd89f17` | level2-field-stability-adjudication | L2-A1..A7 | `walt/CENSUS-RULINGS.md` |
| #24 | 08-24 | `0794ff86` | calculated-evidence-step7 | Shadow instrument | `walt/probes/shadow/README.md` |
| #25 | 08-24 | `31b6a885` | gran-anchor-artifacts | Gran screenshots archived | `kanban/backlog/gran-anchor-reconstruction.md` |
| #26 | 08-24 | `ffdc002b` | field-swap-slice | Field-swap slice 1 | `walt/probes/fieldswap/README.md` |
| #27 | 08-24 | `70799503` | wiki-ce-era | Era page | — |
| #28 | 08-24 | `d22e03b8` | walt-log-ce-era | LOG entry | `walt/LOG.md` |
| #29 | 08-24 | `74b26d29` | panel-briefs-draft | Five panel drafts | `exchange/drafts/` |
| #30 | 08-24 | `ca0483d7` | field-swap-slice2 | Field-swap slice 2 | `walt/probes/fieldswap_screen/README.md` |
| #31 | 08-24 | `e5a5f521` | calculated-evidence-step8 | Step 8 calibration | `walt/probes/step8/README.md` |
| #32 | 08-24 | `6e00528d` | world-cap-512 | Cap ruling 128 → 512 | — |
| #33 | 08-24 | `07d44154` | wiki-step8-slice2 | Wiki sync | — |
| #34 | 08-24 | `039b11df` | shadow-512-epoch | 512-epoch receipt rerun | `walt/probes/shadow/` |
| #35 | 08-24 | `5ef7975d` | panel-response-intake | x:019–023, PANEL-A1..A8 | `walt/math/response_walt_panel_and_cancellation_v0.1_intake.md` |
| #36 | 08-24 | `80f94a0e` | shadow-512-driven | 512-epoch driven rerun | `walt/probes/shadow/` |
| #37 | 08-24 | `23ba1c22` | (agent worktree) | Playable controller player | `walt/CONTROLLER-PLAYER.md` |
| #38 | 08-24 | `151ea4f3` | (agent worktree) | Slice 3 cancellation ladder | `walt/probes/fieldswap_cancel/README.md` |
| #39 | 08-24 | `162f8081` | (agent worktree) | Wiki sync | — |
| #40 | 08-24 | `4c452fe2` | (agent worktree) | Dispatch 024 draft | `exchange/drafts/` |
| #41 | 08-24 | `7f15763f` | fix-trace-item-coverage | Docs precision | — |
| #42 | 08-24 | `d8a2d70e` | dispatch-024-bookkeeping | 024 shipped | `exchange/README.md` |
| #43 | 08-25 | `60f01dcd` | exchange-024-response-intake | TRIPLE-A1..A7 | `walt/math/response_deferred_producers_triple_v0.1_intake.md` |
| #44 | 08-25 | `43967f30` | worktree-walt-s4c-motifs | Slice 4c motifs | `walt/probes/fieldswap_motifs/README.md` |
| #45 | 08-25 | `668c5fb6` | worktree-walt-slice4a-e3 | Slice 4a E3 upper | `tests/solver_e3_upper.rs` |
| #46 | 08-25 | `cbce1ae3` | worktree-walt-slice4b | Slice 4b hazard | `walt/probes/hazard_witness/README.md` |
| #47 | 08-25 | `51eac3fc` | wiki-slice4-sync | Wiki sync | — |
| #48 | 08-25 | `0224471f` | walt-panel-audits | Panel conformance audits | `walt/audits/panel_response_conformance.md` |
| #49 | 08-25 | `43017541` | worktree-walt-step9-detect | Step 9 detection layer | `walt/probes/step9/README.md` |
| #50 | 08-25 | `6110b163` | wiki-sync-step9 | Wiki sync | — |
| #51 | 08-25 | `68f9d040` | walt-l2-controller | Targeted field-1 controller | `walt/probes/l2_controller/README.md` |
| #52 | 08-25 | `0b6094da` | wiki-sync-l2-controller | Wiki sync | — |
| #53 | 08-25 | `aff53213` | worktree-walt-ordering | Reorder-not-cull | `walt/probes/ordering/README.md` |
| #54 | 08-25 | `93d99563` | walt-waking-seat | The waking seat | `walt/probes/waking/README.md` |
| #55 | 08-25 | `a28ddcef` | walt-field-cache | Two surgical levers | `walt/probes/field_cache/README.md` |
| #56 | 08-25 | `5bead5c8` | worktree-walt-bundling | Bundled evaluator | `walt/probes/bundle/README.md` |
| #57 | 08-25 | `586588da` | wiki-sync-walt-day | Wiki sync; era closed at 08-29 | — |
| #58 | 08-25 | `33d541fe` | walt2-wasm | Level-2 browser oracle | `walt/walt2-wasm/pkg/README.md` |
| #59 | 08-25 | `a1d2219b` | wiki-sync-walt2 | Wiki sync | — |
| #60 | 08-30 | `e4c10ead` | intake-counted-belief-sandwich | CBS-A1..A9 | `walt/math/counted_belief_sandwich_v0.1_intake.md` |
| #61 | 08-30 | `a99fcb9f` | intake-counted-belief-sandwich | Slice A | `walt/probes/root_interval/README.md` |
| #62 | 08-30 | `eba8c940` | walt-slice-c0 | Slice C0 | `walt/probes/factor_belief/README.md` |
| #63 | 08-30 | `2d6eb442` | walt-slice-b | Slice B | `walt/probes/grammar_residual/README.md` |
| #64 | 08-30 | `bd86c18a` | walt-slice-c1 | Slice C1 | `factor_belief/cache_run1.txt` |
| #65 | 08-30 | `8d260844` | walt-slice-d | Slice D | `factor_belief/recursion_run1.txt` |
| #66 | 08-30 | `8bea94ca` | walt-slice-c2 | Slice C2 | `factor_belief/c2_run1.txt` |
| #67 | 08-30 | `1cae2419` | walt-slice-e | Slice E | `factor_belief/response_run1.txt` |
| #68 | 08-30 | `246cb260` | walt-slice-f | Slice F | `factor_belief/cegar_run1.txt` |
| #69 | 08-30 | `25b40d9f` | walt-slice-g | Slice G | `factor_belief/refine_run1.txt` |
| #70 | 08-31 | `aed2a38b` | walt-aps-intake | APS-A1..A9 | `walt/math/anytime_proof_state_score_v0.1_intake.md` |
| #71 | 08-31 | `8545bab1` | walt-aps-phase2 | Phases 0+2, freeze 58 | `factor_belief/profile_run1.txt` |
| #72 | 08-31 | `d6320b36` | walt-aps-spike | §49 spike | — |
| #73 | 08-31 | `a1b24d09` | walt-aps-phase3 | Phase 3 certified regret | `factor_belief/proofreport_run1.txt` |
| #74 | 08-31 | `1586ff78` | walt-aps-phase6 | Phase 6 extraction | `factor_belief/extractreport_run1.txt` |
| #75 | 08-31 | `1ceadb7c` | walt-aps-phase1 | Phase 1 frontier | `factor_belief/frontierreport_run1.txt` |
| #76 | 08-31 | `1ee6669a` | walt-aps-phase45 | Phases 4+5 | `factor_belief/bellmanreport_run1.txt` |
| #77 | 08-31 | `a8987fd1` | walt-aps-phase7 | Phase 7 laydowns | `factor_belief/laydownreport_run1.txt` |
| #78 | 09-01 | `e14c1b35` | walt-phase8-opening | Phase 8 opening root | `factor_belief/openingreport_run1.txt` |
| #79 | 09-01 | `eb5a459d` | walt-doom-census | Doom census | `factor_belief/doomreport_run1.txt`; `walt/DISCREPANCIES.md` |
| #80 | 09-01 | `08fe3d2d` | wiki-first-pass-counted-belief | Wiki first pass | — |
| #81 | 09-01 | `c8110ea6` | worktree-walt-s3 | MB-A1..A8, SC-A1..A8 | `walt/briefs/BRIEF-MB0.md`, `BRIEF-U0.md` |
| #82 | 09-01 | `90255f52` | walt-mb0 | MB0 | `walt/briefs/MB0-COLLISION-NOTES.md`, `MB0-HANDOFF-BUILDER2.md` |
| #83 | 09-02 | `161b0195` | walt-sigma1 | σ1-repair | `walt/briefs/BRIEF-SIGMA1-REPAIR.md` |
| #84 | 09-02 | `c5d2f32a` | walt-u0 | U0 God-gap census | `walt/briefs/U0-REPORT.md` |
| #85 | 09-02 | `bfbb45ce` | walt-mb1 | MB1 | `walt/briefs/MB1-REPORT.md` |
| #86 | 09-02 | `3b4105ca` | walt-up0 | UP0 | `walt/briefs/UP0-REPORT.md` |
| #87 | 09-03 | `a80b9829` | walt-up1a-u0b | UP1a + U0b | `walt/briefs/UP1A-REPORT.md`, `U0B-REPORT.md` |
| #88 | 09-07 (ff) | `a0d594b2` | walt-fh | FH0–FH5, CI1, FH4 audit | `walt/briefs/FH1-REPORT.md`, `FH2-REPORT.md`, `FH3-REPORT.md`, `FH4-AUDIT.md`, `CI1-REPORT.md` |

---

## Part D — the state of the tree as of 2026-09-07 (`c00717d1`)

### Unmerged as of 2026-09-07

Measured with `git log main..<branch>` on the worktree at `c00717d1`
(2026-09-12). Both branches fork from `walt-gran` (`9d6a5a2e`), which is
itself in `main`. Their results reach `main` only through
`walt/briefs/MORNING-2026-09-05.md`; Jason's calls (A)–(F) in that readout
are unanswered in-tree. Everything on them is EXPLORATORY.

| Branch | Head | Commits ahead of `main` | Contents | Owner |
|---|---|---|---|---|
| `walt-o5` | `2981e090` | **9** (`0b65efb9` … `2981e090`, 2026-09-04 22:10 → 09-05 00:42) | O5 void-aware inner minds (stages 1–5), gates, `o5flip`/`o5level1`, the mirrored match with the withdrawn "dead heat" and the seed repair. The readout records `check.sh PASS incl. Lean` on this branch. The void-aware *belief* itself reached `main` separately at `dbcc698f` (2026-09-06) by the partnership session, default off. | [walt-gran-anchors](walt-gran-anchors.md) |
| `walt-g1-l2` | `6abdd78f` | **8** (`2d3907bd` … `6abdd78f`, 2026-09-04 22:38 → 09-05 00:39) | `level2.rs` gains carrier/field-level parameters and a fixture mode; level 2 on G1 (holds the 6-4 at both nodes); G2 at Gran's seat; the synthetic lock; the over-claim withdrawn. | [walt-gran-anchors](walt-gran-anchors.md) |

Also present on the remote but not in `main`: `origin/walt-slice-c2`
(`9c8c49ca`), `origin/dispatch-024-bookkeeping` (`9203f091`), and the
agent worktree branches — all superseded by merged PRs; not verified
individually in this pass.

### The CI waiver on the 2026-09-06/07 commits

The 31 commits `d8400713..c00717d1` were landed under an explicit,
user-authorized waiver: "Full Rust CI is deliberately waived for this
experiment; focused checks replace it. No merge, publication, or deployment
is requested" (`experiments/partnership/SCOPE.md`; the packet's
`EXPERIMENT-BRIEF.md` adds that the waiver "do[es] not waive rule fidelity or
information consistency"). Each LOG entry for the session records what ran in
place of the gate — focused Rust tests (19 Scheme; 8 gym + 19 Scheme; 61
distinct focused tests at the first gym), clippy, formatting, WASM library
compilation, Python tests, and independent full-game replays (8,694 at HEAD)
— and repeats
"full CI remains waived". **No `walt/ci/check.sh` run is recorded on any of
these commits**, so whether `main` at `c00717d1` is green under the full gate
is unknown from the record (open item; see
[walt-architecture](walt-architecture.md) for the gate and its cost). The
work also modified shared solver files (`factor_belief.rs::condition_via`,
`solver/selection.rs`, `solver/inner_belief.rs`, `Key`/`PiKey`) beside the
new `scheme`, `gym`, `partnership` and `policy_search` modules, so the waiver
covers more than the experiment directory.

### What did not move

- `ingest/` — unchanged since `4e238ed6` (2026-07-26); manifests verify.
- `rob/` — no code change since `cd51ce2e` (2026-08-01); documentation only
  on 2026-08-13 and 2026-08-24; the twelve receipts never regenerated.
- `lean/` — no theorem change since the P0 merge `0d1bf7ea` (2026-08-03)
  apart from the walt-era Trick1 tree of 2026-08-16/17 (`3b4c6d60`,
  `813d5e81`, `97ce321a`); the `[[lean-catchup]]` card stands.
- The exchange count — 24 since `d8a2d70e` (2026-08-24); x:018 (sent
  2026-08-14) still has no reply on record; `FH-RESPONSE-TO-PRO.md` is a
  draft for hand-ferry, not a numbered dispatch.
- The live default player — untouched by every wave since 2026-08-17's seat
  (CE-A7, CBS-A9, APS-A9, MB-A7, FH-A10 restate it); the partnership
  program changed no phone default (`walt/MAP.md`).
