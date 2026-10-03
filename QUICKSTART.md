# Quickstart

Orientation for a fresh session. Repository state described: **2026-09-20
(`afd46420`)** for the walt layer — curated 2026-09-20 by cycle 1 of the
[curator](wiki/curator.md); the claim tier and the era records still describe
`c00717d1` (2026-09-07); this page rewritten 2026-09-13. The wiki is **the book on 42** and
[wiki/Home.md](wiki/Home.md) is its table of contents; this page is the on-ramp.
Read this, then follow links only as your task needs them.

**New to 42 itself, or want the plain-language version of what this project knows?**
Read [the game of 42, mathematically](wiki/game-of-42.md) — the game, what has been
proved and measured about it, and what can be done with it now, written for a
technical reader who has never played. This page assumes that vocabulary
([wiki/vocabulary.md](wiki/vocabulary.md) fixes each term).

**Want walt (the imperfect-information seat) on one page — what exists, what it
costs, what is redundant, what is next?** Read [`walt/MAP.md`](walt/MAP.md); it is
the orchestrating session's page, rewritten at landings (last rewrite 2026-09-13,
for the state at `c00717d1`; its top paragraph carries the 2026-09-14 and 09-20
landing sessions' own notes, and `walt/LOG.md` has no entry after 2026-09-14).

**Want the whole book?** [wiki/Home.md](wiki/Home.md) — five parts, one line per
chapter, and three doorways (newcomer, mathematician, engineer).

## What this project is

Solve straight points-and-marks Texas 42 as an imperfect-information game, on
mathematics proved *before* code is trusted. Two immutable spec packages under
`ingest/` are the ground truth; everything else reconciles, reproduces, or extends
them. Why this project exists at all: [lineage](wiki/lineage.md) — the prior project
(mk5) hit "the wall" (E[Q] players that can't hold a plan), and this repo answers it
with exact information-set machinery.

## The eight layers

| Layer | What it is | Touch it? |
|---|---|---|
| `ingest/` | Two immutable spec packages, **v0.7** and **rec**; unchanged since 2026-07-26 | **Never modify.** Each has a verifying `MANIFEST.sha256` |
| `wiki/` | The book: what's proved, at what tier, what's open; [Home](wiki/Home.md) is the TOC | Yes — it's the living synthesis; keep the page convention (line 1: the Home link · owns: … · Sources: …) and the cross-referenced ledgers in step |
| `walt/` | **The project's player** — the imperfect-information seat that acts from one chair. One crate `walt` of ten modules across six workspace members ([walt-architecture](wiki/walt-architecture.md)) (corrected 2026-09-20: eleven modules across eight members — `walt-player`, the seat deployed on the phone and the Mac since 2026-09-14, and `walt-cpu-bench`; the v34 CPU speedups are the native default since 2026-09-20); hub [walt](wiki/walt.md) | Yes, per `kanban/` and the binding briefs `walt/briefs/BRIEF-*.md` — but **everything under it is EXPLORATORY tier**, below every tier below |
| `experiments/` | `experiments/partnership/` — the 2026-09-06/07 partnership program (player families, matched batteries, the pool, policy synthesis, relational learning); owned by [walt-partnership-program](wiki/walt-partnership-program.md); its own entry point is [`experiments/partnership/README.md`](experiments/partnership/README.md). Since 2026-09-13 also Sunshine (the partner-aware presets and the Mac table; [walt-partnership-program §11](wiki/walt-partnership-program.md#11-sunshine-2026-09-13--09-15-the-partner-aware-live-player)), and since 2026-09-18 `experiments/kiln/` — the opening bidding book Plunge bids from and the mining studies on its played corpus ([walt-kiln](wiki/walt-kiln.md); entry point [`experiments/kiln/README.md`](experiments/kiln/README.md)) | Yes — but it is **EXPLORATORY and CI-waived** (`walt/ci/check.sh` was not run on its commits); its results files govern its prose |
| `rob/` | The Rust exact engine: executable spec + twelve byte-diffed receipts, rob the exact plan solver, the demoted baseline (evening player v0), the HTML inspector — the receipt discipline walt aspires to. Artifact guide: [rob](wiki/rob.md) | Yes, per its BRIEFs; code dormant since 2026-08-01 |
| `exchange/` | Courier channel to ChatGPT 5.6 Pro for adversarial research; dispatches authorized in batches, quota agreed per batch (count in `exchange/submission_count.txt`; batch ceiling `HARD_CAP` in `automation/submit.mjs`). Chapter: [exchange](wiki/exchange.md) | Per the [pro-exchange protocol](exchange/README.md); **never submit without Jason's explicit go** |
| `lean/` | Lean 4 + mathlib kernel formalization — all 42 priority-0 rows kernel-proved (2026-08-02). Artifact guide: [lean](wiki/lean.md) | Yes, per [lean/PROOFS.md](lean/PROOFS.md) |
| `kanban/` | The work queue: one file per task, status = directory (`backlog/` / `doing/` / `done/`), linked by `[[card-id]]` tokens | Yes — move the file to move the status; a card is never evidence |

## Non-negotiables (every session, every task)

- **Never edit `ingest/`.** Discrepancies between the packages are resolved in the
  wiki ([discrepancies](wiki/discrepancies.md)), never by editing sources.
- **Evidentiary tiers are never promoted or blurred** ([Home](wiki/Home.md#evidentiary-tiers--never-promoted-never-blurred)):
  corpus statuses > proof-assistant kernel > exchange-adjudicated CONFIRMED > rob
  conformance receipts. A green receipt is *evidence*, never a status change; "PASS"
  is never imported as an axiom (TRUST-01). Label every substantive claim you write.
  Dissents and corrections travel verbatim (REACH-20 is 2/3 SOUND; the doom-census
  sentence was corrected 2026-09-03 — see the traps).
- **Citation convention** ([Home](wiki/Home.md#citation-convention)): **v0.7** = the
  type-discipline package, **rec** = the new-mathematics package; claim IDs like
  `CELL-14`; `x:NNN` cites an exchange result; ruling IDs like `CE-A1`, `CBS-A3`,
  `FH-A2` cite `walt/CENSUS-RULINGS.md`. Every walt number names its record path or
  gate file; results files outrank prose; dates are absolute.
- **Merge rule** ([package-provenance](wiki/package-provenance.md)): rec's mathematics
  under v0.7's type discipline. Concretely: derived views, never stored cells;
  reachability proof-irrelevant (no identity-bearing certificates); say "necessary
  outer profile," never "certificate"; no floats near ranks or probabilities.
- **Vocabulary is load-bearing** ([vocabulary](wiki/vocabulary.md)): support ≠
  belief, feasible ≠ reachable, possible ≠ probable, estimate ≠ receipt — typed
  distinctions, not emphasis. "Sandwich" is never the name of an object (CBS-A3,
  FH-A2); pmake is the objective (ruled 2026-08-17); "level 2" is a best response to
  a named σ1, never equilibrium.
- **Exploratory stays exploratory**: [ideas](wiki/ideas.md),
  [analysis](wiki/analysis.md), [field/](wiki/field/Home.md), and everything under
  [walt/](wiki/walt.md) and `experiments/` sit below every tier and are cited by
  nothing above them. A walt number is quotable only through the gate or test file
  that pins it; otherwise label it a probe record. A walt number never appears in a
  brief, a dispatch, [FINDINGS](wiki/FINDINGS.md), or any claim-tier page.
- **The gates cost what they cost.** `rob/ci/check.sh` is hours. `walt/ci/check.sh`
  is about **308 s wall and memory-hungry**: the anchors suite measured 18.2 GB
  standalone at the FH4 audit, 8.8 GB in-gate after FH5 capped its in-flight
  h4-t4 jobs (`b6de5a25`, in `main`), and the whole gate still runs more than
  20 GB of test-binary RSS on the 48 GB machine
  ([walt-architecture](wiki/walt-architecture.md) §4 owns the numbers). **Full walt
  CI was waived on the 2026-09-06/07 landings** (`d8400713..c00717d1`); whether
  `main` is green under the full gate at `c00717d1` is not known from any record
  — and it is not recorded run on any of the 73 landings `c00717d1..afd46420`
  (2026-09-13 → 09-20) either; the v34 receipts say `full_workspace_ci: "not
  run"` ([walt-instruments §3.7](wiki/walt-instruments.md#37-native-cpu-speedups-v34-and-the-phone-release-2026-09-18--09-20)).
- **Never end a turn with background work pending** (`CLAUDE.md`, "Agents"): a
  subagent that yields while a `run_in_background` job runs is never woken. Long jobs
  run in the foreground under the tool's timeout, split under it, or polled in a
  foreground loop.
- **Briefs**: walt's binding assignments are `walt/briefs/BRIEF-*.md` and the reports
  of record are `walt/briefs/*-REPORT.md` (since 2026-09-01); rob's remain
  `rob/BRIEF*.md`. `kanban/` is the queue, the briefs are the scope.

## The mathematical object, in one breath

Declaration selects one of nine relational algebras over the 28 dominoes
([declaration-algebra](wiki/declaration-algebra.md) — only 3 classes for count-blind
mechanics). A viewer's exact knowledge of the three hidden hands is a capacity cell
system whose fiber is lossless ([support-fiber](wiki/support-fiber.md), CELL-05 — the
keystone), countable and exactly samplable without enumeration
([capacity-dp](wiki/capacity-dp.md)), with a globally minimal canonical normal form —
81 bits standalone, 0 bits given mechanical state
([minimal-support-normal-form](wiki/minimal-support-normal-form.md)). Play updates it
as a monotonically deleting 63-edge graph
([support-dynamics](wiki/support-dynamics.md)). Legal play reaches strictly fewer
supports than Hall allows; the exact count is the flagship open problem, boxed to
**[36,45] bits** at the exchange tier ([reachability](wiki/reachability.md), OPEN-11).
And **support is not belief**: the 90-world witness gives two legal histories with
identical support but opposite optimal leads
([belief-vs-support](wiki/belief-vs-support.md)) — the theorem that guards every
shortcut. The minimal exact decision state is the reduced viewer kernel, proved
strictly finer than the true quotient via the dead-cut lemma
([reduced-viewer-kernel](wiki/reduced-viewer-kernel.md), x:003).

## Current state (2026-09-20 for the walt layer; the claim tier as of 2026-09-07)

- **The claim tier has not moved.** Nothing at the corpus, kernel, exchange-CONFIRMED
  or rob-receipt tier has changed since the Lean priority-0 close (2026-08-02) and
  the ledger's last row (x:024, 2026-08-25); not one commit since 2026-08-24 touches
  `ingest/`, `rob/`, or a Lean theorem ([timeline](wiki/timeline.md) Part D; through
  `afd46420` the only touches under `rob/` and `lean/` are PR #91's README edits —
  Part E). Every wave below is **EXPLORATORY**, one pointer each.
- **Seat play and the match** (EXPLORATORY; [walt-seat-play](wiki/walt-seat-play.md)).
  Objective ruled 2026-08-17: **P(make the bid)** — pmake; trick differential is a
  proxy. The level-1 scenario-player seat played its first full hands 2026-08-17,
  then defeated the mk5 E[Q] champion under the dropped-30 3×384 protocol — 630/1152
  pooled, McNemar z = +6.28, losing points while winning marks (record
  `walt/probes/m3/arena_results_2026-08-17.txt`; an arena outcome about play, never a
  statement about exact values). Jason has played it live at the plunge table. **No
  default has changed since 2026-08-19** (θ = 11/16); every later change on the live
  path is parity-gated. (Corrected 2026-09-20: true through `c00717d1`; since
  2026-09-14 the seat Plunge ships is the shared `walt-player` crate — the next
  bullets — under conformance receipts, with no record citing the ruling for it.)
- **Calculated evidence, 2026-08-24 → 08-29** (EXPLORATORY;
  [walt-calculated-evidence](wiki/walt-calculated-evidence.md)). Jason's two
  hand-ferried parents adjudicated same-day (CE-A1..A8, L2-A1..A7): anytime-valid
  adaptive settlement, the §22 build (steps 2–9), the shadow instrument beside the
  live player, four field-swap slices, the controller made seatable (a capability,
  never a comparison), the waking seat, the speed campaign, level 2 in the browser
  (walt2-wasm, never a default). CE = sampling depth, L2 = model choice; L2 consumes CE
  baselines, never the reverse.
- **Counted belief and the anytime proof state, 2026-08-30 → 09-01** (EXPLORATORY;
  [walt-counted-belief-era](wiki/walt-counted-belief-era.md)). CBS-A1..A9 and
  APS-A1..A9, Phases 0–8: exact integer masses over the 399,072,960-world opening
  fiber, append-only proof states, certified regret Γ = U* − B_exec. **The
  opening-root verdict**: play 6-5, floor 732‰, at most 267‰ unclaimed — an honest
  UNRESOLVED at ε = 1/4 (record `walt/probes/factor_belief/openingreport_run1.txt`).
  **Its correction (2026-09-03, `walt/DISCREPANCIES.md`)**: the doom census's reading
  of the 267‰ as "overwhelmingly the info-consistency price" outran what was
  established — the split of the 267‰ between information price and policy gap is
  UNKNOWN.
- **The focal horizon, 2026-09-01 → 09-05** (EXPLORATORY;
  [walt-focal-horizon-era](wiki/walt-focal-horizon-era.md)). Jason's frame, now
  measured: "**42 is two recursions running in opposite directions**" — exactness
  walking backward from the last trick, sampled play walking forward — and on the
  receipt corpus they trade dominance between trick 4 and trick 5. The book-one
  closing intakes (MB-A, SC-A), model belief (the fusion price proved strictly
  positive at trick 4, gate-pinned), the σ1 repair, the God-gap censuses, the unified
  player, then the focal-horizon hierarchy FH0–FH5 (audited, merged 2026-09-07 as
  PR #88): one object [L_k, U_k] per root action. **The direction-changing finding**:
  every live trick-4 coordinate settles by k ≤ 2, and at k ≥ 1 the remaining width is
  the tail's policy gap, not fusion price — a better lawful tail buys more than a
  deeper search (record `walt/probes/factor_belief/focal_run1.txt`). The trick-3
  anchor h8-t3 settles only at k = 3, the collapse; that root is the wall — the
  single-field exact solve costs 289M field reads and 14 min (record
  `walt/probes/factor_belief/horizon_run1.txt`), and the FH3 record run that
  includes it peaked at 19.4 GB (`focal_run1.txt`; `walt/briefs/FH3-REPORT.md`).
- **The Gran anchors, 2026-09-04/05** (EXPLORATORY;
  [walt-gran-anchors](wiki/walt-gran-anchors.md)). The real "6-4" hands from the
  phone were reconstructed tile-by-tile, validated by the rules engine, and played by
  three walts. The answer is not what the question expected: at trick 1 the choice is
  a near-tie that different samplers decide differently; the level-2 player hoards
  *more*, not less; the hoarding mechanism at exact indifference is the live seat's
  tie-break (`TieRule::LowestTileIndex`). Obligation O5 (the modeled minds' no-void
  fiber) was measured on the unmerged branch `walt-o5`; "dead heat" is withdrawn.
- **Partnership, Scheme/Fix and the gym, 2026-09-06/07** (EXPLORATORY, CI-waived;
  [walt-partnership-program](wiki/walt-partnership-program.md),
  [walt-scheme-fix](wiki/walt-scheme-fix.md), [walt-gym](wiki/walt-gym.md)). A
  partner-aware sampling-stack player exists and is playable within the trick target;
  **no tested partner model beat the phone or L1** — L2 Partner vs L1 is 14/14/72 at
  about 5× the cost, voids vs voidless 12/17/71 (results file
  `experiments/partnership/campaigns/default-partner-battery/RESULTS.md`); the
  strength question stays open. Scheme/Fix is implemented as `walt::scheme` (typed
  relational expressions over kernel worlds, exact event probabilities, finite
  dynamics, executable policies). The exact gym grew 6 → 170 → 433 → the 30-case
  composed exam (L1 24/30, L2 Partner 26/30). The relational learner's shared actors
  still trail sampled tables. No default changed.
- **Sunshine, 2026-09-13 → 09-15** (EXPLORATORY, CI-waived; [walt-partnership-program §11](wiki/walt-partnership-program.md#11-sunshine-2026-09-13--09-15-the-partner-aware-live-player),
  [walt-gym §11](wiki/walt-gym.md#11-sunshine-2026-09-13-the-gym-under-deployed-continuations-and-the-live-move-intake), [walt-scheme-fix §10.4](wiki/walt-scheme-fix.md#104-sunshine-2026-09-13--09-15-the-count-offer-query-as-a-live-gate-and-recipes-with-a-selectable-continuation); added 2026-09-20). Two optional partner-aware presets
  around L1 — `l1-partner-count-review` (250 ms, valued under the gym teacher's
  field) and `l1-partner-rollout` (500 ms, every legal root action through
  completed deployed L1; Plunge's "L1 + partner check") — neither a default. Every
  ordinary-game panel tied (32 + 100 + 64 mirrored pairs, 192 conditional): "no
  demonstrated strength gain". Conditional skill is model-relative: the review
  24/30 → 29/30 on the teacher's exam, two useful / four reversed under deployed
  L1; the rollout 11 / 2 development, 1 / 1 fresh, the fresh harm a
  deadline-truncated prefix (results files `experiments/partnership/campaigns/sunshine-*/RESULTS.md`).
  The gym now names its continuation; the Mac table feeds flagged human-play moves
  into it; the workshop harness is recorded (2026-09-15), not built.
- **The shared deployed player, 2026-09-14** (EXPLORATORY, conformance receipts;
  [walt-seat-play §8A](wiki/walt-seat-play.md#8a-the-shared-deployed-player-walt-player-2026-09-14-onward); added 2026-09-20). The deployed Walt is **one crate on two hosts**:
  `walt-player` — its WASM in Plunge on the phone, its native `walt-table` behind
  the Mac bridge — fixed L1 40/8 with an 8/2 reserve and the optional 500 ms partner
  rollout in 14 s (level 1, voidless, fixed selection), and a once-around auction at
  threshold 3/4, "deliberately an uncalibrated bidding policy". Native/WASM exact
  agreement on nine declarations, a late root and the 64-world partner prefix;
  "The phone itself has not yet been timed" (`walt/walt-player/README.md`). No
  strength claim; no arena run.
- **Kiln, 2026-09-18 → 09-19** (EXPLORATORY; [walt-kiln](wiki/walt-kiln.md); added
  2026-09-20). Plunge's table bids catalogue hands by lookup in Kiln's played book
  since 2026-09-19 (Plunge `c7a1215d`): 500 hands / 125 deals / 4,500 panels /
  **305,440 games** of the deployed player at bid 30, the recommendation the
  highest 30–42 target reached in 4/5 of games, 75 panels `capped-unsettled`;
  playing WASM unchanged; production stopped (`experiments/kiln/played-bidder-release.json`).
  The scalar model survey that preceded it is frozen (1,001,348 receipts / 851
  settled deals) after its one calibration check: forecast 121/160, executed
  21/100 — "a candidate explanation, not a cause established by the experiment"
  (`CALIBRATION-SIX36.md`). Four mining studies on the played corpus replicated two
  opening outcome associations on fresh deals and changed no move rule (§8); the
  solver work measured through the Kiln bench (§9) is inside the baseline v34
  measured against.
- **CPU speedups v34 and the phone release, 2026-09-20** (EXPLORATORY, conformance
  receipts; [walt-instruments §3.7](wiki/walt-instruments.md#37-native-cpu-speedups-v34-and-the-phone-release-2026-09-18--09-20); added 2026-09-20). The `cpu-speedups` umbrella (26
  features) is the native default in `walt`, `walt-player` and `walt-cpu-bench`:
  12/12 paired L2 Partner 40/8/2 games exact-equal, median **13.13×**
  (`walt/receipts/cpu-speedups-v34/comparison.json`); shipped to Plunge the same
  day (app `65f8f68b…`, WASM `b3016e18…`; 15/15 shipped-WASM parity; hosted
  opening 265 ms — "Mac Chrome timings, not Pixel measurements"). "Finite
  conformance receipts, not universal equivalence proofs"; no strength claim;
  `full_workspace_ci: "not run"`. Every walt number computed in a default build
  after 2026-09-20 runs the v34 paths.
- **Exchange: dispatches 001–024** (count 24; 016–024 hand-ferried by Jason;
  chapter [exchange](wiki/exchange.md)). 001–008 the foundation batch (adjudicated;
  007/008 CONFIRMED with caveats); 009 **PARTIAL**, 010 and 012 **CONFIRMED**; the
  Lean thread 011/013/015 iterated without a panel (Stages 1–2 GREEN, unreconciled);
  014 an informal capture (unadjudicated); 016/017 the decision-sparse thread and
  **019–023 the CE/L2 adversary panel (PANEL-A1..A8) and 024 the deferred-producers
  triple (TRIPLE-A1..A7)** all adjudicated same-day into **walt's exploratory tier**,
  never the CONFIRMED pipeline; **018 (correspondence) has no reply in the inbox as
  of 2026-09-07, and whether it was ever delivered is not established from the
  record** ([exchange](wiki/exchange.md) §6.4). Seven further Pro parents (CE, L2, CBS, APS, MB, SC, FH) came by side
  channel and are indexed only on [walt-math-intakes](wiki/walt-math-intakes.md); the
  FH response to Pro was drafted 2026-09-04 and not dispatched. Results table:
  [claim-ledger](wiki/claim-ledger.md). Standing headlines: interval [36,45] bits
  (x:001/006/007), no-void stratum exactly 624,892,870 (x:008), outer language not
  tight + fifth condition (x:002), kernel-vs-quotient COLLAPSE (x:003), transport
  9→3 collapse (x:004), all 19 census integers independently reproduced (x:005).
- **Lean: all 42 priority-0 rows kernel-proved** (2026-08-02; last row PA-E10, the
  90-world witness internalized whole; no `sorry`, no `native_decide`, standard
  axioms only). Since then: the constellation thread's self-contained Stage 1/2 files
  (x:013/x:015, not reconciled with the main layers) and the walt-era trick-1 tree —
  two modules gated by `walt/ci/check.sh`, one tree (`Trick1PerfectRecallNet`)
  unverified to build; [[lean-catchup]] tracks the reconciliation. Last commit
  touching `lean/`: 2026-08-17. [proof-assistant-plan](wiki/proof-assistant-plan.md)
  has the scoreboard; [lean](wiki/lean.md) is the artifact guide.
- **rob slices 01+02 green**: twelve byte-diffed receipts reproducing every
  slice-01/02 ingest number, plus the x:001 floor family (S10). Full inventory:
  [verification](wiki/verification.md); artifact guide: [rob](wiki/rob.md). Player
  track P1–P5 green: **rob (the exact plan solver, rolling re-solve) beats the
  baseline (evening player v0) net +718** over 200 mirrored hands (`r_mat_paired`) — a
  frozen measurement, never a target; the inspector (`rob/inspector/`) shows every
  decision's plan tree; probes and rigs are cataloged in [analysis](wiki/analysis.md).
  No code change since 2026-08-01.

## The live frontier

- **As of `afd46420` (2026-09-20; added 2026-09-20).** The deployed seat is
  `walt-player` + v34 + the Kiln bid book on the phone and the Mac
  ([walt-seat-play §8A](wiki/walt-seat-play.md#8a-the-shared-deployed-player-walt-player-2026-09-14-onward)); what the cycle's chapters name as open: the partnership gap ("The
  partnership gap is still open", `SUNSHINE-NOTES.md`; whether a bounded partner
  check should override L1 on sampled evidence at all — partnership §9 item 10;
  the human partner as a continuation condition no census models), the Kiln
  calibration gap's cause, and whether Plunge's forced last bid (2026-09-20)
  belongs to the straight-42 rules profile (a question for Part I). **PR #90**
  (`codex/nello-player`, "Add doubles-suit Nel-O to the shared Walt player") is
  open and unmerged as of 2026-09-21; the book lists Nel-O as a formal exclusion,
  so a merge would be the first foundation change under the
  [curator](wiki/curator.md)'s sorting rule ([walt](wiki/walt.md), "The unmerged
  branches").
- **Jason's ruling of 2026-09-04**: "follow through on what we have, then invest in a
  simplification/unification attempt" — **no new mathematical parent until the
  consolidation slice lands.** The order (`walt/MAP.md`): the σ0 read-key study
  ([[sigma0-read-key-study]] — does σ0's answer depend on the full record? if not,
  every recursion gets 10–100× cheaper) → the consolidation slice
  ([[consolidation-slice]] — retire `godgap.rs`, `horizon.rs`, `extraction.rs`,
  `refine.rs` as endpoints of the focal-horizon hierarchy). Neither is started as of
  `c00717d1`.
- **Open beside the line, on Jason's word**: the partnership strength question
  ([[partnership-strength-question]]), the void-aware inner-belief default
  ([[inner-voids-default]]), a compact Scheme compiler ([[scheme-compact-compiler]]),
  the ladder's memory ([[ladder-policy-store]] — 19.4 GB at h8-t3), gate suites sized
  like censuses ([[gate-corpus-trim]]), and the documentation debts of this rewrite
  ([[wiki-book-followups]]). In `doing/`: [[gran-anchor-reconstruction]] (open on two
  items) and [[hf-archive-upload]]. Older backlog: [[lean-catchup]],
  [[m2-receipt-reearn]], [[m2-runner-trace]], [[gpu-level2]], [[plunge-walt-sync]].
- **Two unmerged branches** from the 2026-09-05 readout (`walt/briefs/MORNING-2026-09-05.md`):
  `walt-o5` (9 commits: O5 measured, the mirrored match with the withdrawn "dead
  heat") and `walt-g1-l2` (8 commits: level 2 at Gran's seat holds the 6-4; the
  synthetic lock; the tie-break). Jason's calls (A)–(F) in the readout are open;
  [walt-gran-anchors](wiki/walt-gran-anchors.md) owns the status.
- **walt's obligations ledger** (`walt/SCENARIO-PLAYER.md` §10): the seat leapt
  first; the bridge is being built behind it, deliberately and on the record. The
  decision-sparse economy claim is preserved on
  [walt-decision-sparse](wiki/walt-decision-sparse.md) and was overtaken by the
  counted-belief and focal-horizon programs; its lineage is recorded there.
- **OPEN-11** — exact reachable-support census; void-context strata still open
  ([open-problems](wiki/open-problems.md), [FINDINGS Q1](wiki/FINDINGS.md)). No
  activity since x:008 (2026-07-27).
- **The constellation direction** (idea tier —
  [idea-retrograde-rank](wiki/idea-retrograde-rank.md)): backward induction over
  constellations, with exchange-tier anchors C1 (x:009 proof chain), R1 (x:010), and
  the x:012 staircase; its Lean files are GREEN but unreconciled ([[lean-catchup]]).
  No probe work since 2026-08-03. The seat-level frame at
  [idea-seat-context](wiki/idea-seat-context.md) is deliberately unresolved;
  [ideas](wiki/ideas.md) records each idea's walt descendants.
- **rob slice 03 targets** (no activity since 2026-08-01): reproduce x:007 (filtered
  census) and x:008 (no-void slice) in Rust; the belief/filtering layer with the
  90-world regression is the unassigned slice 6
  ([first-implementation-slice](wiki/first-implementation-slice.md)).
- **Mechanization** (no activity since 2026-08-17): the priority-1 tiers and the
  PA-A12/B04 reflection targets ([proof-assistant-plan](wiki/proof-assistant-plan.md)
  — priority 0 is closed).

## Traps that have bitten before

- **The `__pycache__` trap (D15, as now stated).** Running the rec verifiers *in
  place* with a default `python3` writes `verification/__pycache__`, after which
  `audit_package.py` fails its no-transients check. Run them from a copy
  (`cp -r ingest/<package> /tmp/…`) or with `python3 -B`; either keeps the audit
  green. The checked-in `ingest/` tree never contained a `__pycache__`
  ([discrepancies D15](wiki/discrepancies.md)).
- rec's executable spec contradicts rec's own math on stored state — v0.7's
  discipline controls (D1/D2).
- x:001's "exactly 559,316,142" is a grammar-subfamily count, not the no-void slice
  (D17). One exchange panel (REACH-20) was 2/3 SOUND, not 3/3 — carry the dissent.
- Frozen generator values (`FROZEN_WITH_VOIDS` 970, the `verify_player` and
  `verify_rob` transcripts) are rob-internal determinism freezes, **not** ingest
  numbers.
- **`rob/ci/check.sh` is an hours-long job, not a quick check.** Where the time goes
  ([rob](wiki/rob.md) §11): `verify_rob` solves 756 positions exactly (58,609,267
  nodes), plays three mirrored 200-hand matches with rolling re-solve at every rob
  decision, and re-solves all 756 positions to round-trip 6,001,465,196 bytes of plan
  JSON. The earlier explanation on this page — that it "re-derives a
  44,722,908,161-state census" — was a misreading: `r_pos_census` is merely the sum of
  the 756 capacity-DP fiber counts, computed cheaply (corrected 2026-09-12). A run on
  2026-08-13 had spent four CPU-hours and was still going; the date of the last
  complete green `PASS` is not recorded anywhere. It is not hung — that is what the
  gate costs. Budget for it, and don't start one casually late in a session.
- **`walt/ci/check.sh` is seconds-to-minutes but memory-hungry**: about 308 s wall
  with the test binaries run concurrently, more than 20 GB of test-binary RSS on the
  48 GB machine, and the anchors suite alone 18.2 GB standalone before FH5 capped
  its in-flight jobs (8.8 GB in-gate after `b6de5a25`); the FH3 record run peaked
  at 19.4 GB ([walt-architecture](wiki/walt-architecture.md) §4). Don't run two
  gates at once.
  And it was **not run on the 2026-09-06/07 commits** (session waiver), nor is it
  recorded run on any landing through `afd46420` (2026-09-20).
- **The wedge.** An agent that ends its turn with a background job running is never
  woken when the job finishes — the project's recurring stall (2026-09-04: gates and
  record finished at 02:11, agent silent for five hours). Foreground with the tool's
  timeout, split, or poll (`CLAUDE.md`, "Agents").
- **In `walt/` and `experiments/`, the results files outrank the prose.** Several
  headline numbers in the session log disagree with the artifacts they cite; the known
  cases are tabled at [walt-pre-pivot-results](wiki/walt-pre-pivot-results.md)
  Appendix C (the seven prose-versus-artifact disagreements of the S6 era) and in
  [`walt/DISCREPANCIES.md`](walt/DISCREPANCIES.md) (the 2026-09-02/03 entries,
  including the doom-census correction). Check the artifact before quoting a walt
  number. Pre-pivot result summaries live at `walt/probes/factory-results/`; their
  producers are archive-only — regenerating one means checking out producer commit
  `648f93a` first (`walt/ARCHIVE.md`).
- **Epochs do not compose.** Every walt number is relative to a declared epoch — the
  σ0/σ1 field identities, sample counts, caps and seeds — and the epoch is part of
  the result's identity. The `l2_controller` probes run σ0 at `n0 = 8`; the acting,
  waking and Gran epoch runs σ0 at `n0 = 2`; the Plunge panel's forty-world
  percentages compose with neither ([walt-architecture](wiki/walt-architecture.md)
  §5, the declared-epochs table).
- **rob the player vs the baseline.** The +718 belongs to rob (the exact plan
  solver); the evening player v0 *is* the baseline it beat. An earlier version of
  this page had the attribution backwards.

For the full assessment — strongest results ranked, suspicious spots, what to build
next — read [FINDINGS](wiki/FINDINGS.md). For the human-facing account of the game
and what can be done with it, read [game-of-42](wiki/game-of-42.md). For the dated
record of every landing, read [timeline](wiki/timeline.md).
