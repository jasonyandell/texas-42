# Texas 42 Foundations — the book on 42

The project: solve straight points-and-marks Texas 42 as an imperfect-information
game, on mathematics proved before code is trusted. Two immutable specification
packages in [`ingest/`](../ingest/) are the source of truth; this wiki is the
reconciled map over them — what is proved, at what evidentiary tier, what is open,
and what each other layer of the repo does about it. Nothing under `ingest/` is ever
modified (each package carries a verifying `MANIFEST.sha256`).

This wiki is **the book on 42** and this page is its table of contents: every page
is a chapter, one page owns each topic, pages link rather than restate, and every
substantive statement carries its evidentiary tier. Repository state described:
2026-09-07 (`c00717d1`); this page rewritten 2026-09-13.

The project's **player** is [walt](walt.md) — the imperfect-information seat, built
iteratively at the exploratory tier, and since 2026-08-17 actually playing full
hands. The project's **exact-truth engine** is [rob](rob.md) — the byte-diffed
executable specification whose receipt discipline remains the engineering bar the
walt build aspires to meet. Neither impersonates the other: rob answers *what is
exactly true*; walt is the seat that has to act.

## Three doorways

- **Never played 42, or here to find out what any of this is for?** Start at
  [the game of 42, mathematically](game-of-42.md) — the human-facing account of the
  game, what has been proved and measured about it, and what can be done with it
  now, written for a technical reader who has never played — then
  [lineage](lineage.md) for why the project exists. The rest of this page assumes
  the vocabulary those two build; [vocabulary](vocabulary.md) fixes each term.
- **Mathematician.** The tier ladder and citation convention below; then the
  claim-tier chapters — [FINDINGS](FINDINGS.md), [claim-ledger](claim-ledger.md)
  (the claim-tier inventory and every exchange result with its caveats verbatim)
  and [open-problems](open-problems.md), whose claim-tier content has not moved
  since the last exchange adjudication of 2026-08-25 —
  then the seat's mathematics at the EXPLORATORY tier:
  [walt-math-reference](walt-math-reference.md) (organized by idea, every object
  with the ruling that fixed it; its siblings [walt-math-intakes](walt-math-intakes.md)
  and [walt-math-freezes](walt-math-freezes.md), freezes 1–58) and
  [`walt/CENSUS-RULINGS.md`](../walt/CENSUS-RULINGS.md) (the append-only
  adjudication record, every ruling family in order — [walt](walt.md) lists the
  families); the kernel side is [lean](lean.md) and
  [`lean/PROOFS.md`](../lean/PROOFS.md).
- **Engineer who wants to run the player.**
  [walt-architecture](walt-architecture.md) (one crate, ten modules; the gate and
  what it costs; the epochs every number is relative to),
  [walt-instruments](walt-instruments.md) (every binary with its invocation, record
  path and gate; the seats you can play), and [`walt/MAP.md`](../walt/MAP.md) (walt
  on one page — what exists, what it costs, what is next). Read the agent rules in
  [`CLAUDE.md`](../CLAUDE.md) before starting anything long.

## The repo in eight layers

| Layer | What it is |
|---|---|
| [`ingest/`](../ingest/) | Two immutable spec packages, **v0.7** and **rec** (citation convention below). All definitions, theorems, and claim IDs live here. Unchanged since 2026-07-26. |
| `wiki/` (this) | The book: the reconciled synthesis — [merge order](package-provenance.md), [discrepancies](discrepancies.md), [claim tiers](claim-ledger.md), [current findings](FINDINGS.md), the dated [timeline](timeline.md), and the walt chapters (Parts II–IV below). |
| [`walt/`](walt.md) | **The project's player** — the imperfect-information seat. **Everything under it is EXPLORATORY, below every tier on this page**; [walt](walt.md) states the fence once. Objective ruled 2026-08-17: **P(make the bid)** — pmake; trick differential is a proxy. By era, each clause pointing at its chapter: the frozen-basis research programs of 2026-08-09 → 08-16 — the opening situation space does not compress, the decision side does ([walt-pre-pivot-results](walt-pre-pivot-results.md); the four era pages are provenance) · the pivot to play, 2026-08-17: the level-1 scenario-player seat that defeated the mk5 E[Q] champion under the dropped-30 3×384 protocol — 630/1152 pooled, McNemar z = +6.28, an arena outcome about play, never a statement about exact values (record `walt/probes/m3/arena_results_2026-08-17.txt`) — and has run live in plunge ([walt-seat-play](walt-seat-play.md)) · calculated evidence, 2026-08-24 → 08-29: anytime-valid adaptive settlement (CE-A1..A8, L2-A1..A7), the §22 build, the field swaps, the controller and waking seats ([walt-calculated-evidence](walt-calculated-evidence.md)) · counted belief and the anytime proof state, 2026-08-30 → 09-01 (CBS-A1..A9, APS-A1..A9, Phases 0–8): exact integer masses over the 399,072,960-world opening fiber and a certified-regret verdict at the receipt opening root — play 6-5, floor 732‰, at most 267‰ unclaimed, an honest UNRESOLVED at ε = 1/4 (record `walt/probes/factor_belief/openingreport_run1.txt`); the doom census's first reading of that 267‰ as "overwhelmingly the info-consistency price" was corrected 2026-09-03 — **the split is UNKNOWN** ([`walt/DISCREPANCIES.md`](../walt/DISCREPANCIES.md)) ([walt-counted-belief-era](walt-counted-belief-era.md)) · the focal horizon, 2026-09-01 → 09-05, under Jason's frame "**two recursions running in opposite directions**": model belief (MB0/MB1), the σ1 repair, the God-gap censuses (U0/U0b), the unified player (UP0/UP1a), the focal-horizon hierarchy FH0–FH5, audited and merged 2026-09-07 (PR #88); its finding: every live trick-4 coordinate settles by k ≤ 2 and the residual width is the tail's policy gap, not fusion price (record `walt/probes/factor_belief/focal_run1.txt`) ([walt-focal-horizon-era](walt-focal-horizon-era.md)) · the Gran anchors: the real "6-4" hands reconstructed, validated and played by three walts; exact indifference and the tie-break; obligation O5; two unmerged branches ([walt-gran-anchors](walt-gran-anchors.md)) · the partnership program, Scheme/Fix and the exact gym, 2026-09-06/07, landed CI-waived: no partner model has beaten its reference ([walt-partnership-program](walt-partnership-program.md), [walt-scheme-fix](walt-scheme-fix.md), [walt-gym](walt-gym.md)). The GPU side track holds portable M0/M1 and M2 Metal parity under freezes 55/56 and no player ([walt-gpu-native-trick1](walt-gpu-native-trick1.md)). The crate: **one crate `walt` of ten modules across six workspace members** ([walt-architecture](walt-architecture.md); the instruments on [walt-instruments](walt-instruments.md)). **No default change since 2026-08-19** (θ = 11/16; CE-A7/§20.16, restated by CBS-A9, APS-A9, MB-A7, FH-A10); code changes on the live path are parity-gated ([walt-seat-play](walt-seat-play.md) §8). Jason's ruling of 2026-09-04: **no new mathematical parent until the consolidation slice lands** ([[consolidation-slice]]). Refutations at [walt-negative-results](walt-negative-results.md); the resets at [walt-program](walt-program.md). |
| [`experiments/`](../experiments/partnership/) | `experiments/partnership/` — the 2026-09-06/07 partnership program: a Codex session under an explicit CI waiver (`walt/ci/check.sh` was not run on `d8400713..c00717d1`), the archived Plunge WASM as the frozen "phone" reference, the native player families (L1 default, L2 Partner, L2 Partner with voids), matched full-game batteries, the pool infrastructure, policy synthesis and relational learning. **EXPLORATORY, CI-waived**; the negative strength results are stated as such on the owning chapter, [walt-partnership-program](walt-partnership-program.md). Status ledger `experiments/partnership/SESSION-STATUS.md`. |
| [`rob/`](../rob/README.md) ([rob](rob.md)) | The Rust exact engine — an executable spec with proof receipts, and the **aspirational engineering example** walt's build is expected to grow into (byte-diffed receipts, frozen values, CI as the gate). Slices 01+02 green: **twelve byte-diffed receipts** reproducing every slice-01/02 ingest number ([verification](verification.md)) — ten stage receipts, the baseline's self-play transcript, and the P1–P5 player-track receipt `verify_rob.txt`; `rob/ci/check.sh` diffs all twelve (an hours-long gate — [rob](rob.md) §11). **rob the player** is the exact plan solver with rolling re-solve; it beat the baseline (the fixed-field Monte Carlo evening player v0) by **net +718** over 200 mirrored hands (`r_mat_paired`) — **a measurement, never a target**. Plus the HTML **game inspector** (per-seat perspectives, exact fiber counts and marginals, plan trees). Code dormant since 2026-08-01 (documentation passes only since); receipts last regenerated 2026-07-28. |
| [`exchange/`](../exchange/README.md) ([exchange](exchange.md)) | Courier channel to ChatGPT 5.6 Pro for adversarial research; [exchange](exchange.md) owns the chapter, `exchange/README.md` stays the operational ledger. **Dispatches 001–024; count 24.** 001–008 the foundation batch (adjudicated; 007/008 CONFIRMED with caveats — REACH-20's panel was 2/3 SOUND, carried verbatim); 009–015 the constellation batch (009 **PARTIAL**, 010 and 012 **CONFIRMED**, the Lean thread 011/013/015 iterated without a panel — Stages 1–2 GREEN, unreconciled; 014 an informal capture, UNADJUDICATED); 016–024 hand-ferried by Jason: 016/017 the decision-sparse thread, 019–023 the CE/L2 adversary panel (**PANEL-A1..A8**) and 024 the deferred-producers triple (**TRIPLE-A1..A7**), every one adjudicated same-day into **walt's exploratory tier**, never the CONFIRMED pipeline; **018 (correspondence) has no reply as of 2026-09-07, and whether it was delivered is not established from the record ([exchange](exchange.md) §6.4).** Seven further Pro parents (CE, L2, CBS, APS, MB, SC, FH) arrived by side channel, never as numbered dispatches, and are indexed only on [walt-math-intakes](walt-math-intakes.md); the FH response to Pro (`walt/briefs/FH-RESPONSE-TO-PRO.md`) was drafted 2026-09-04 for hand-ferry and is not a dispatch. Budget: monthly pacing cleared with Jason per batch (fixed lifetime cap retired 2026-08-01); the automated ceiling `HARD_CAP` remains 17 and the count legitimately exceeds it. |
| [`lean/`](../lean/README.md) ([lean](lean.md)) | Lean 4 + mathlib kernel formalization. **Priority-0 scoreboard: 42 of 42 rows kernel-proved** (2026-08-02) — the mechanization ledger's first-release target is closed. Spans the domino and declaration algebra through the unique trick winner, the objective hand machine and 42-point conservation, cell losslessness, the support normal form with compile/decode inverses, strategic sufficiency, and the **90-world posterior-flip witness** (PA-E10) internalized whole. No `sorry`, no `native_decide`, standard axioms only. The walt-era trick-1 tree: two modules (`Trick1Foundation`, `Trick1MetalFoundation`) built and axiom-audited by `walt/ci/check.sh` on every run — kernel as arithmetic, exploratory as meaning — and one tree (`Trick1PerfectRecallNet`, M3) whose build and audit are **unverified** as of 2026-09-07. Open: the priority-1 tiers and the PA-A12/B04 reflection targets ([proof-assistant-plan](proof-assistant-plan.md)); [[lean-catchup]] carries the reconciliation of the constellation files and the trick-1 tree. Last commit touching `lean/`: 2026-08-17. |
| `kanban/` | The work queue: one file per task, status = its directory (`backlog/` / `doing/` / `done/`), cards linked by greppable `[[card-id]]` tokens, never by path. walt's binding assignments are `walt/briefs/BRIEF-*.md` (reports of record `*-REPORT.md`); rob's remain `rob/BRIEF*.md`. **The board as of 2026-09-13** — `backlog/` (13): [[sigma0-read-key-study]], [[consolidation-slice]], [[ladder-policy-store]], [[gate-corpus-trim]], [[inner-voids-default]], [[partnership-strength-question]], [[scheme-compact-compiler]], [[wiki-book-followups]], [[lean-catchup]], [[m2-receipt-reearn]], [[m2-runner-trace]], [[gpu-level2]], [[plunge-walt-sync]]; `doing/` (2): [[gran-anchor-reconstruction]] (open on two items), [[hf-archive-upload]]; `done/` (12): [[adaptive-sampling-intake]], [[exchange-quota-reframe]], [[level2-field-swap-probe]], [[math-reorg]], [[panel-response-audits]], [[playable-controller-walt]], [[slice3-cancellation-ladder]], [[slice3-deferred-producers]], [[viewer-cross-fiber-review]], [[waking-seat-census]], [[walt-unification]], [[wiki-overhaul]]. A card is a queue entry, never evidence. |

[lineage](lineage.md) explains the prior project (mk5), the champion, and the
wall this repo exists to answer — context only; no code or definitions cross.

## The object in one paragraph

Straight points-and-marks Texas 42 is modeled as a **declaration-indexed physical
game** plus, per player, an **imperfect-information game over hidden deals and current
hidden remainders**. The load-bearing discovery chain: a viewer's rule knowledge about
the three hidden hands is *exactly* captured by three dependent capacity cells
(pool, per-seat allowed sets, capacities); that cell support has a **globally minimal
canonical normal form** (certain tiles + a determinate/binary/ternary ambiguity core);
the set of normal forms **legal play can actually reach** is a strict subset of the
Hall-feasible ones, its exact cardinality open inside a corpus-proved 26–46-bit
interval — narrowed to **[36,45] bits** at the exchange-adjudicated tier
([reachability](reachability.md)); and **support is not belief** — two legal histories
can share the same exact 90-world support yet require opposite optimal leads under
every named utility ([belief-vs-support](belief-vs-support.md)).

## Evidentiary tiers — never promoted, never blurred

1. **Corpus statuses** — the packages' own labels ("Theorem — proved",
   "Theorem — exhaustive finite verification", …); the ground truth.
2. **Proof-assistant kernel** — the target tier; external `PASS` is never imported
   as an axiom (TRUST-01). Priority 0 closed 2026-08-02, 42 of 42 rows
   ([lean](lean.md)); no `sorry`, no `native_decide`.
3. **Exchange-adjudicated CONFIRMED** — external result. The verdict rule actually
   applied ([claim-ledger](claim-ledger.md)): the response's program executed
   `ALL_PASS` on its own claims and the proof chain survived three adversarial
   referees with **no referee demonstrating a real flaw**. One panel, REACH-20, was
   2/3 SOUND + 1 UNVERIFIABLE that found no defect — the dissent travels verbatim and
   is never presented as 3/3. Not a corpus theorem, not a kernel proof.
4. **rob conformance receipts** — byte-diffed Rust reproductions; `x-` prefixed
   lines back exchange numbers. Evidence, never a status change.

Below all four, and cited by nothing above them: **EXPLORATORY** — everything under
`walt/` and `experiments/`, [ideas](ideas.md), [analysis](analysis.md), and
[field/](field/Home.md). A walt number is quotable only through the gate or test file
that pins it; otherwise it is labelled a probe record. Full vocabulary and the
per-result caveats: [claim-ledger](claim-ledger.md); the terms themselves:
[vocabulary](vocabulary.md).

## Citation convention

- **v0.7** = [`texas-42-foundations-source-of-truth-v0.7`](../ingest/texas-42-foundations-source-of-truth-v0.7/) — the *proof-assistant boundary revision*.
- **rec** = [`texas-42-foundations-source-of-truth-v0.7-reconstructed`](../ingest/texas-42-foundations-source-of-truth-v0.7-reconstructed/) — the *reduced play/support foundation*.
- `Math §x` / `Rules §x` / `Exec §x` = that package's `20_MATHEMATICAL_FOUNDATION.md` / `10_RULES.md` / `30_EXECUTABLE_SPECIFICATION.md`; claim IDs like `CELL-14` refer to `40_CLAIM_STATUS.md`; `x:NNN` cites an exchange result by ledger number; ruling IDs like `CE-A1`, `CBS-A3`, `FH-A2` cite `walt/CENSUS-RULINGS.md`.
- Every walt number names its record path or gate file; results files outrank prose.
- Every substantive statement carries its tier label. Dates are absolute.

## Pages — the book's table of contents

Five parts. Part I is the claim-tier mathematics (corpus, kernel, exchange, rob
receipts). Parts II–IV are walt and sit at the EXPLORATORY tier, cited by nothing
above; their hub is the first line of Part II. Part V sits below every tier. A page marked
*provenance record* is a dated account of a program — its numbers are quotable
only from the results files it names.

### Front matter
- [Home](Home.md) — this page: the table of contents, the tier ladder, the citation convention, the three doorways, the layer table.
- [QUICKSTART](../QUICKSTART.md) — the on-ramp for a fresh session (repo root): the eight layers, the non-negotiables, the current state, the live frontier, the traps.
- [game-of-42](game-of-42.md) — the human-facing introduction to straight Texas 42 as a mathematical object: what the game is, what is proved, what is measured, and what the machinery can now do.
- [timeline](timeline.md) — the dated record: every landing from the v0.7 ingest (2026-07-26) to `c00717d1` (2026-09-07), by wave, with commit hash and/or PR number, what each established at which tier, and the page that owns it.
- [vocabulary](vocabulary.md) — the load-bearing vocabulary: one entry per term, its precise meaning, the typed distinction it guards, the ruling or source that fixed it, and the page that owns it.
- [lineage](lineage.md) — why texas-42 exists: the prior project (mk5), the champion, and the wall. Context only.

### Part I — The exact mathematics of hidden information
- [rules-profile](rules-profile.md) — the normative rules profile (byte-identical in both packages).
- [declaration-algebra](declaration-algebra.md) — the domino universe, the nine algebras, transports and mechanics classes.
- [support-fiber](support-fiber.md) — the cells, the fiber, the losslessness theorem.
- [capacity-dp](capacity-dp.md) — Hall feasibility, exact counting, the uniform sampler.
- [minimal-support-normal-form](minimal-support-normal-form.md) — the normal form, its minimality, the 81-bit census.
- [reachability](reachability.md) — feasible ≠ reachable, both witnesses, the [36,45]-bit interval, and rec's symbolic-reachability construction under its own name (necessary outer profiles — never "certificates", D3).
- [support-dynamics](support-dynamics.md) — (rec) the matching-minor calculus, monotonicity, the 63-edge budget.
- [reduced-viewer-kernel](reduced-viewer-kernel.md) — (rec) the four reductions, the kernel, future equivalence, the OPEN-01 collapse.
- [belief-vs-support](belief-vs-support.md) — the support/belief separation, Bayes machinery, the 90-world witness.
- [strategic-state](strategic-state.md) — the marked hand, the decision state (c, e, β), utility lenses, quotients and gauges.
- [package-provenance](package-provenance.md) — how v0.7 and rec relate, and which package wins on each topic.
- [discrepancies](discrepancies.md) — every disagreement found (D1–D17), with resolution and confidence.
- [claim-ledger](claim-ledger.md) — the status vocabulary (including the exchange-adjudicated tier definition), the merged claim inventory, and the per-row-group pointer to the kernel tier.
- [FINDINGS](FINDINGS.md) — the overall assessment: what the object is, strongest results, risks, next questions (the assessment itself unchanged since 2026-08-02).
- [open-problems](open-problems.md) — the merged OPEN inventory and its current statuses, plus the fenced list of exploratory-tier open questions.
- [verification](verification.md) — every verifier and receipt: ingest Python, rob Rust (all twelve receipts), exchange program runs.
- [exchange](exchange.md) — the adversary in the loop: the courier mechanism, the tier it produces, the quota protocol, every numbered dispatch 001–024 and what came of it, how to read an exchange result, the incidents and the ledger-drift errata.
- [lean](lean.md) — the kernel mechanization: the `lean/` library as an artifact, its tier, the discipline that governs it, what it has proved, what those proofs do and do not certify, and how to build and extend it.
- [lean-row-index](lean-row-index.md) — the map from mechanization-ledger rows to the Lean declarations that discharge them, and what the ledger still leaves open.
- [proof-assistant-plan](proof-assistant-plan.md) — the trust boundary, the K0–K15 spine, mechanization priorities, the scoreboard, and the queue of accepted Lean programs.
- [rob](rob.md) — the rob artifact: its place in the evidence hierarchy, the invariants it carries as types, the receipt discipline, the S1–S10 ladder and the P1–P5 player track, its instruments, its cost, and where it stands.
- [rob-slices](rob-slices.md) — what each rob brief assigned and what each stage established, and the ledger of what was named but never begun.
- [analysis](analysis.md) — the catalog of rob's probes, rigs, and instruments (display/exploratory tier; walt's instruments live on walt-instruments).
- [field/](field/Home.md) — rob beyond the repo: [first-contact](field/first-contact.md) (the 2026-07-30 encounters with the mk5 champion), [lessons](field/lessons.md) (the methods that made them fair), [directions](field/directions.md) (the direction-setting of 2026-07-30, captured not planned); field-measurement tier.
- [first-implementation-slice](first-implementation-slice.md) — the record of the original slice-01 assignment (historical; rob executed it).

### Part II — The seat's mathematics (walt; EXPLORATORY)
- [walt](walt.md) — the hub: the fence, what walt is now, the map of Parts II–IV, the sources under `walt/`, the ruling families, the unmerged branches.
- [walt-program](walt-program.md) — what the walt program is trying to do, every direction reset it has taken and why (2026-08-09 → 2026-09-07), and the working method.
- [walt-math-reference](walt-math-reference.md) — the map of walt's mathematical corpus organized by idea, every named object with the ruling that fixed it and the measurement that last moved it; its siblings by object family: [structure and transport](walt-math-structure-transport.md) (Lemmas V, X, E, S and the S-corollaries), [information geometry](walt-math-information-geometry.md) (Lemma R, Lemma G, G-flat and the cardinality ladder), [decision-deadness](walt-math-deadness.md) (Lemma J and the J-propositions), [decision-sparse witnesses](walt-math-decision-sparse.md) (the errata's E-series), [received artifacts and intakes](walt-math-intakes.md) (every frozen parent, companion and ruling range), [the freeze register](walt-math-freezes.md) (freezes 1–58), [open questions](walt-math-open-questions.md) (each with the ruling that left it open).
- [walt-pre-pivot-results](walt-pre-pivot-results.md) — what the frozen-basis programs of 2026-08-09 → 08-16 established, by result: what a seat cannot compress, decision sparsity, the interval at four tricks, two theorems about the rules, the lesson factory; the era pages are provenance.
- [walt-negative-results](walt-negative-results.md) — walt's negative results as first-class findings, 2026-08-09 → 2026-09-07: what each refuted, at what scope, under which pre-declared criterion, and what it did not rule out.
- [walt-decision-sparse](walt-decision-sparse.md) — the decision-sparse exact-solving architecture: thesis, objects, theorem inventory, audit history, the experiment program, and its lineage into root intervals, certified regret and focal-horizon intervals.

### Part III — The programs, as records (EXPLORATORY)
- [walt-nello](walt-nello.md) — Nel-O mechanics, the 40-to-100,000-world double-lead investigation, counterexample/doom distinctions, held-out failures, and the bounded browser preview (2026-09-20 → 09-25).
- [walt-foundation-era](walt-foundation-era.md) — *provenance record*: S1–S4.5, the rules-to-operators stack and the control-skeleton checkers.
- [walt-factory-era](walt-factory-era.md) — *provenance record*: S5a–S5d, the conflict-driven lesson factory, the label-fragility discovery, the re-tethering.
- [walt-census-era](walt-census-era.md) — *provenance record*: S5e–S5k, the situation censuses, the retrograde quotient and railyard, the fiber and endgame probes, the seat census resolved by proof.
- [walt-s6-era](walt-s6-era.md) — *provenance record*: S6a–S6n, the predictive-rank census, policy geometry, deadness detectors, the separation and economy probes, the trick-1 and lay-down theorems, the fusion-tax, second-rung, feature-fee and fee-correlation chapters.
- [walt-seat-play](walt-seat-play.md) — how walt plays: the one decision procedure and its three axes, every configuration that coexists, the 2026-08-17 match, the level-2 question, bidding and declaring, the variant seats, surfaces and gates, the live-code changes since 2026-08-25 with no default change, debts.
- [walt-calculated-evidence](walt-calculated-evidence.md) — *provenance record*: 2026-08-24 → 08-29 — the CE/L2 threads, the §22 build, the shadow instrument, the field-swap slices, the controller and waking seats, the speed campaign, walt2-wasm.
- [walt-counted-belief-era](walt-counted-belief-era.md) — *provenance record*: 2026-08-30 → 09-01 — the CBS C→G ladder, the anytime proof-state Phases 0–8, the doom census, and the opening-root diagnosis with its 2026-09-03 correction.
- [walt-focal-horizon-era](walt-focal-horizon-era.md) — *provenance record*: 2026-09-01 → 09-05 — two recursions running in opposite directions: the book-one closing intakes (MB-A, SC-A), model belief (MB0, σ1-repair, MB1), the God-gap censuses (U0, U0b), the unified player (UP0, UP1a), the focal-horizon hierarchy (FH0–FH5, CI1, the FH4 audit), the consolidation ruling.
- [walt-gran-anchors](walt-gran-anchors.md) — the 6-4 problem: the Gran anchors G1/G2/G3 and the synthetic lock, the waking seat's first real hand, exact indifference and the tie-break, obligation O5 and both void-aware inner-belief implementations, branch status.
- [walt-partnership-program](walt-partnership-program.md) — `experiments/partnership/` 2026-09-06/07: the launch packet, the "phone" reference, the player families, every battery and its numbers, the pool infrastructure, policy synthesis and relational learning, what is settled and open.
- [walt-gym](walt-gym.md) — the exact partnership gym: what a coordinate is, how an answer key is defined and audited, Scheme-driven discovery, the 6 → 170 → 433 → 30 results ladder, reproduction.
- [walt-scheme-fix](walt-scheme-fix.md) — the Scheme/Fix relational expression language as implemented (`walt::scheme`, 2026-09-06/07: grammar, semantics, beliefs and counterexamples, finite dynamics, executable policies) and the archived descriptor research that preceded it.
- [walt-gpu-native-trick1](walt-gpu-native-trick1.md) — the GPU side track: the authority chain, M0/M1, M2 Metal parity, the frozen-but-unbuilt M3 net, the freeze-56 v2 re-issue, and why it never became a player.

### Part IV — The tools (engineering)
- [walt-architecture](walt-architecture.md) — the `walt` crate: the six-crate workspace and the one crate's ten modules, the two solver stacks and their seams, invariants carried by types and CI, the gate and what it costs, the declared epochs, named debts and freezes by path.
- [walt-instruments](walt-instruments.md) — the instrument catalog: every binary under `walt/walt/src/bin/` with its purpose, invocation, record path, gate, cost and status, by program; the seats a person can play; how to read a record; the archive-only historical inventory.
- The tool sections of the Part I chapters: rob §12 "Running rob" and §11 (the cost of the gate), lean §12 "Building and extending", and analysis (rob's probes) — linked above.

### Part V — Ideas (below every tier)
- [ideas](ideas.md) — the list of promising-but-unproven directions and, per idea, the dated record of which later objects descend from it.
- [idea-hierarchical-fibers](idea-hierarchical-fibers.md) — the pooled-fiber proposal and its admissibility conditions (2026-07-28), with its walt descendants noted.
- [idea-retrograde-rank](idea-retrograde-rank.md) — the retrograde direction: endgame quotients keyed on constellations, walked backward, and its probe record; exchange anchors x:009/010/012.
- [idea-seat-context](idea-seat-context.md) — the seat-level context frame (the player's maintained state); deliberately unresolved.
