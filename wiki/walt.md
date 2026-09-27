# walt — the imperfect-information seat

[Home](Home.md) · owns: the hub for the [`walt/`](../walt/) build and the
[`experiments/`](../experiments/partnership/) directory — the fence around them,
what walt is now, the map of Parts II–IV of the book, the sources under `walt/`,
the workspace in one line, the ruling families, and the unmerged branches ·
Sources: none upward (this page cites; nothing above the Ideas tier ever cites
it); [`walt/MAP.md`](../walt/MAP.md), [`walt/LOG.md`](../walt/LOG.md),
[`walt/CENSUS-RULINGS.md`](../walt/CENSUS-RULINGS.md), `walt/Cargo.toml`,
`ls walt/` and `git log main..<branch>` measured 2026-09-13 on the tree at
`c00717d1` (2026-09-07). Related: [ideas](ideas.md),
[idea-seat-context](idea-seat-context.md),
[idea-retrograde-rank](idea-retrograde-rank.md), [lineage](lineage.md),
[analysis](analysis.md), [field/](field/Home.md), [rob](rob.md), [lean](lean.md),
[exchange](exchange.md), [timeline](timeline.md), [vocabulary](vocabulary.md).

> **Epistemic tier: EXPLORATORY — below every tier on
> [Home](Home.md#evidentiary-tiers--never-promoted-never-blurred), cited by
> nothing above [ideas](ideas.md).** Everything under `walt/` and
> `experiments/` is exploratory: the frozen mathematical bases in `walt/math/`,
> the Rust workspace, every pin, every measured number, every probe record, every
> gate, and every receipt-shaped artifact (each marked exploratory in its own
> header). walt's cross-implementation pins are **regression pins against probe
> records, never axioms** — TRUST-01 applies unchanged. A walt number is
> quotable only through the gate or test file that pins it; otherwise it is a
> probe record, and results files outrank prose. Nothing under this hub may be
> quoted in a brief, a dispatch, [FINDINGS](FINDINGS.md), or any claim-tier
> page. Promotion path: independent re-verification, and per the no-rescue
> policy of 2026-08-10 (F7) new mechanical verification goes through **Lean**
> ([lean](lean.md)); the preserved Python probe *records* remain frozen
> cross-implementation pins. The fence is stated here once; every other walt
> page inherits it.

## What walt is now

[rob](rob.md) answers *what is exactly true*. **walt is the seat** — the
full-hand imperfect-information player that has to act from one chair, seeing
only what a chair legally sees. It is one decision procedure
([walt-seat-play](walt-seat-play.md) §1): at its turn the seat imagines many
complete deals consistent with what it has seen, plays each out exactly against
its model of the other three seats, and chooses the tile that makes the bid in
the largest share of them — **pmake**, the objective ruled 2026-08-17, with trick
differential explicitly a proxy. Every walt that has ever played a hand is that
procedure at a point on **three axes**: the *field model* (`Dice`, `Level(k)`,
`SeatLevels`), the *inner belief* a modeled mind samples (`Voidless`,
`VoidsCounted`), and *selection* (`Fixed`, `Refine`, `RaceRefine`). **The live
default is level-1 walt** — `walt_bridge` in the arena, `walt-wasm` on the
phone, `webtable`/`playtable` at the tables — and **no default has changed since
2026-08-19** (θ = 11/16 at `9a056f20`); the ruling lineage is CE-A7/§20.16 →
CBS-A9 → APS-A9 → MB-A7 → FH-A10, each restating that the old defaults remain
until arena and conformance gates justify a change, on Jason's word. The
variants that coexist and are not defaults: level 2 in the browser
(`walt2-wasm`), the controller seat (`solver::act`, `controller_bridge`, the
`ctrl` seats), the waking seat (`waking_bridge`, `granrun`), the unified player
(`solver/unified.rs`), and the partnership families L2 Partner and L2 Partner
with voids under `experiments/partnership/`. Code changes on the live path since
2026-08-25 are parity-gated ([walt-seat-play](walt-seat-play.md) §8). Behind the
player stands the counted backward stack — the exact instruments that walk in
from the last trick: counted belief, exact response, God uppers, proof states
with certified regret, model belief, the focal-horizon hierarchy — so that the
crate is organized around Jason's frame "**two recursions running in opposite
directions**" ([walt-architecture](walt-architecture.md) §2). Since 2026-08-17
walt is also the project's primary build, with rob's receipt discipline as the
engineering bar it is expected to grow into.

The name is continuity: in the predecessor project, walt was the exact four-tile
endgame solver, the first artifact that provably had a plan and cashed it
([lineage](lineage.md)). The ingest packages never name walt — implementation
names are deliberately excluded from the corpus
([package-provenance](package-provenance.md)) — so this hub cites `walt/`'s own
documents only.

New here? Read [the program and its resets](walt-program.md) first; it is the
narrative spine. For the game itself in plain language, and what any of this is
for, read [the game of 42, mathematically](game-of-42.md). To run something,
read [walt-instruments](walt-instruments.md); to change something, read
[walt-architecture](walt-architecture.md) first.

## The map of Parts II–IV

| Page | Owns | Dates |
|---|---|---|
| **Part II — the seat's mathematics** | | |
| [walt-program](walt-program.md) | The goal, every direction reset and why, the working method, and where the program stands | 2026-08-09 → 09-07 |
| [walt-math-reference](walt-math-reference.md) | The map of walt's mathematics by idea — every named object, the ruling that fixed it, the measurement that last moved it; siblings: [structure and transport](walt-math-structure-transport.md), [information geometry](walt-math-information-geometry.md), [decision-deadness](walt-math-deadness.md), [decision-sparse witnesses](walt-math-decision-sparse.md), [received artifacts and intakes](walt-math-intakes.md), [the freeze register](walt-math-freezes.md) (freezes 1–58), [open questions](walt-math-open-questions.md) | through 2026-09-07 |
| [walt-pre-pivot-results](walt-pre-pivot-results.md) | What the frozen-basis programs established, by result: what a seat cannot compress, decision sparsity, the interval at four tricks, two theorems about the rules, the lesson factory — with the prose-versus-artifact disagreements tabled | 2026-08-09 → 08-16 |
| [walt-negative-results](walt-negative-results.md) | The refutations and negative results as first-class findings — what each refuted, at what scope, under which pre-declared criterion | 2026-08-09 → 09-07 |
| [walt-decision-sparse](walt-decision-sparse.md) | The decision-sparse architecture, its objects, audits and experiments, and its lineage into root intervals, certified regret and focal-horizon intervals | 2026-08-13 → 08-16; lineage to 09-04 |
| **Part III — the programs, as records** | | |
| [walt-nello](walt-nello.md) | Nel-O mechanics, sample-size reversal, retained escape deals, fixed-field doom, held-out limitations and the opt-in preview | 2026-09-20 → 09-25 |
| [walt-foundation-era](walt-foundation-era.md) | *Provenance record.* S1–S4.5: the rules-to-operators stack, the control skeleton and its two exhaustive checkers | 2026-08-09 |
| [walt-factory-era](walt-factory-era.md) | *Provenance record.* S5a–S5d: the conflict-driven lesson factory, the label-fragility discovery, the lesson economy, the re-tethering | 2026-08-10 |
| [walt-census-era](walt-census-era.md) | *Provenance record.* S5e–S5k: the situation censuses, the retrograde quotient and railyard, the fiber and endgame probes, the seat census resolved by proof | 2026-08-10 → 08-11 |
| [walt-s6-era](walt-s6-era.md) | *Provenance record.* S6a–S6n: the predictive-rank census, policy geometry, deadness detectors, the separation and economy probes, the trick-1 and lay-down theorems, the fusion-tax, second-rung, feature-fee and fee-correlation chapters | 2026-08-12 → 08-16 |
| [walt-seat-play](walt-seat-play.md) | How walt plays: the decision procedure and its three axes, every coexisting configuration, the 2026-08-17 match, the level-2 question, bidding and declaring, the variant seats, surfaces and gates, the live-code changes with no default change, debts | 2026-08-17 → 09-07 |
| [walt-calculated-evidence](walt-calculated-evidence.md) | *Provenance record.* The CE and L2 threads: adaptive settlement (CE-A1..A8, L2-A1..A7), the §22 build, the shadow instrument, the field-swap slices, the controller and waking seats, the speed campaign, walt2-wasm | 2026-08-24 → 08-29 |
| [walt-counted-belief-era](walt-counted-belief-era.md) | *Provenance record.* The CBS C→G exact-mass ladder, the anytime proof-state Phases 0–8 (CBS-A1..A9, APS-A1..A9, freeze 58), the doom census, and the opening-root diagnosis with its 2026-09-03 correction | 2026-08-30 → 09-01 |
| [walt-focal-horizon-era](walt-focal-horizon-era.md) | *Provenance record.* Two recursions running in opposite directions: the book-one closing intakes (MB-A1..A8, SC-A1..A8), model belief (MB0, σ1-repair, MB1), the God-gap censuses (U0, U0b), the unified player (UP0, UP1a), the focal-horizon hierarchy (FH-A1..A11; FH0–FH5, CI1, the FH4 audit), the consolidation ruling | 2026-09-01 → 09-05; merged 09-07 (PR #88) |
| [walt-gran-anchors](walt-gran-anchors.md) | The 6-4 problem: the Gran anchors G1/G2/G3 and the synthetic lock, the waking seat's first real hand, exact indifference and the tie-break, obligation O5 and both void-aware inner-belief implementations, branch status | 2026-09-04 → 09-06 |
| [walt-partnership-program](walt-partnership-program.md) | `experiments/partnership/`: the launch packet, the "phone" reference artifact, the player families, every battery and its numbers, the pool, policy synthesis, relational learning, what is settled and open | 2026-09-06 → 09-07 |
| [walt-gym](walt-gym.md) | The exact partnership gym: coordinates, answer keys, Scheme-driven discovery, the 6 → 170 → 433 → 30 ladder, reproduction | 2026-09-06 → 09-07 |
| [walt-scheme-fix](walt-scheme-fix.md) | The Scheme/Fix relational expression language as implemented (`walt::scheme`), and the archived descriptor research that preceded it | 2026-09-06/07; research 2026-08 |
| [walt-gpu-native-trick1](walt-gpu-native-trick1.md) | The GPU side track: M0/M1, M2 parity, the M3 gate, the freeze-56 v2 re-issue, why it never became a player | 2026-08-16 → 08-24 |
| **Part IV — the tools** | | |
| [walt-architecture](walt-architecture.md) | One crate, ten modules; the two stacks and their seams; invariants carried by types; the gate and its cost; the declared epochs; named debts; freezes by path | tree at `c00717d1` |
| [walt-instruments](walt-instruments.md) | The instrument catalog by program: every binary with its invocation, record path and gate; the seats you can play; how to read a record; the archive-only historical inventory | tree at `c00717d1` |

The dated spine across all of these is [timeline](timeline.md); the terms are
fixed on [vocabulary](vocabulary.md).

## The sources under `walt/` (and `experiments/`)

| Piece | What it is |
|---|---|
| [`walt/NELLO-PLAYER.md`](../walt/NELLO-PLAYER.md) | Human-called own-suit Nel-O candidate: contract and active-seat extension, native/WASM parity, retained paired defense failures, and phone validation boundary |
| [`walt/MAP.md`](../walt/MAP.md) | walt on one page, for Jason: the objects that exist and what each costs, the tree-shake list, the cost trend, what is next in order. Rewritten at landings by the orchestrating session (last rewrite 2026-09-13 for the state at `c00717d1`) |
| [`walt/LOG.md`](../walt/LOG.md) | The session index. Entries through 2026-08-24 are summary lines pointing at the owning page here; the 2026-08-25 → 09-01 entries were logged retroactively; the 2026-09-06/07 entries are multi-paragraph session records. The full per-session records live in git history |
| [`walt/CENSUS-RULINGS.md`](../walt/CENSUS-RULINGS.md) | The append-only adjudication record — every ruling, freeze and theorem that governs a probe (14,358 lines at `c00717d1`); the families are listed below and mapped with ranges on [walt-math-reference](walt-math-reference.md) Appendix C |
| [`walt/FACTOR-BELIEF.md`](../walt/FACTOR-BELIEF.md) | The running build record from 2026-08-30: the status ledger for Slices A–G, anytime Phases 0–8, the doom census, then MB0/σ1-repair/U0/MB1/UP0/UP1a/U0b and the FH program, with every gate suite and probe record named |
| [`walt/DISCREPANCIES.md`](../walt/DISCREPANCIES.md) | Spec-versus-reference reconciliations, same protocol as the corpus: never pick a plausible reading silently. Carries the 2026-09-03 doom-census correction (the 267‰ split is UNKNOWN) |
| [`walt/SCENARIO-PLAYER.md`](../walt/SCENARIO-PLAYER.md) | The spec-after-build of the playing seat (2026-08-18), with its proof-obligations ledger (§10) and the 2026-09-06 inner-belief note — the graduation path for everything on [walt-seat-play](walt-seat-play.md) |
| [`walt/CONTROLLER-PLAYER.md`](../walt/CONTROLLER-PLAYER.md) | The register for the controller variant (`solver::act`, its route labels, the `controller_bridge` / `ctrl` surfaces, the knobs and gates). Estimates never receipts, no strength claim, never the default |
| [`walt/briefs/`](../walt/briefs/) | The binding assignments `BRIEF-*.md` (CI1, FH0–FH3, FH4-AUDIT, MB0, MB1, SIGMA1-REPAIR, U0, UP0) and the reports of record `*-REPORT.md` / `FH4-AUDIT.md`; the readout `MORNING-2026-09-05.md` (the main-side record of the two unmerged branches, with `walt/LOG.md`'s 2026-09-05 entry); `FH-RESPONSE-TO-PRO.md` (drafted 2026-09-04 for hand-ferry, not a dispatch) |
| [`walt/math/`](../walt/math/) | The frozen bases, never edited (`unified_information_geometry_v0.4.md`, `equivariant_lumpability_v0.5.md`, `predictive_algebra_v0.6.md`); the received parents byte-for-byte with `.sha256` pins, `_intake.md` companions and scratch-tier `verify_*.py` (decision-sparse and its errata and second audit, signed-pivotal, calculated evidence, targeted level 2, the panel and triple responses, counted belief, anytime proof state, model belief, salvation complex, focal horizon); the 2026-08-17 pmake question/ruling pair (`WALT-MATH-QUESTION-…` / `WALT-MATH-RULING-…`); the GPU guides and rebriefs; the derived `implementers_guide.md` (non-authoritative). Indexed on [walt-math-intakes](walt-math-intakes.md) |
| [`walt/probes/`](../walt/probes/) | The frozen probe records, one directory per program, each README carrying its own fence: `bidcurve/` (bid calibration), `bundle/` (bundled evaluator vs per-world exact), `exp3a/` and `exp5/` (the rescued Python suites — frozen validators, never source), `factor_belief/` (Slices C–G, Phases 1–8, doom, godgap, horizon, model belief, unified, focal — the `*_run1.txt` records), `factory-results/` (the pre-pivot result summaries; producers archive-only at `648f93a`), `field_cache/` (the two hot-path levers), `fieldswap/`, `fieldswap_screen/`, `fieldswap_cancel/`, `fieldswap_motifs/` (the four field-swap slices), `grammar_residual/` (Slice B), `gran/` (the Gran anchors), `hazard_witness/` (slice 4b), `l2_controller/` (the targeted field-1 controller), `m3/` (the seat-play result files incl. the 2026-08-17 arena), `ordering/` (reorder-not-cull), `root_interval/` (Slice A), `shadow/` (step 7), `step8/` (V5 flip repair), `step9/` (the level-2 detection layer), `waking/` (the waking-seat profile); plus `tilt_arena_2026-08-19.log` |
| [`walt/gym/`](../walt/gym/) | The exact partnership gym's artifacts: `README`, `RESULTS`, `DISCOVERY`, `BID-MAKING`, `SPECIFICATIONS`, `PARTNERSHIP-COMPOSITION`; `queries/*.scheme`, `scenarios/`, `collections/`, `specs/`, `benchmarks/`, `mining.json` — [walt-gym](walt-gym.md) |
| [`walt/scheme/`](../walt/scheme/) | The Scheme/Fix guides: `README`, `DYNAMICS`, `POLICIES`, `RELATIONAL`, `COMPOSITION`, `INFORMATION-PRICES`, `VALIDATION`, `examples/` — [walt-scheme-fix](walt-scheme-fix.md) |
| [`walt/receipts/`](../walt/receipts/) | The GPU track's committed receipts: the Gate-0 NO-GO record (`gpu_native_trick1_gate0_2026-08-16.txt`, a true observation of its old environment), `gpu_native_trick1_m0_m1_v1/`, `gpu_native_trick1_m2_v1/` — executable fixed-carrier evidence, never a theorem or player artifact |
| [`walt/ci/`](../walt/ci/) | `check.sh` (the gate: bootstrap, immutable M0/M1 history, guide checksum, receipt replay, fmt, clippy `-D warnings -D float_arithmetic`, the no-float scans, all workspace release tests run concurrently, doc tests, the Lean trick-1 build and axiom audit), `check_m2_metal.sh` (the freeze-56 integrated gate), `verify_m2_history.sh`, `verify_m2_sources.sh`, the no-float checkers, `run_test_binaries.py`, `render_m2_failure.py` — [walt-architecture](walt-architecture.md) §4 and §7 |
| `walt/audits/` | `panel_response_conformance.md` — the 2026-08-25 audit (PR #48) of the panel response's Claim-D repair, W7–W11 and the τ coupling against PANEL-A3/A5/A6, at base `51eac3f`, with eight conformance gates; the believed-by-construction claims are the ones it checks |
| `walt/viewer/` | `walt_viewer.html`, `walt_table_viewer.html` — the browser viewers over records and playouts |
| The standing design docs `walt/*.md` | `SEPARATION-PROBE` (SEP-A1..A19), `SEPARATION-RUNG-N4`, `ECONOMY-SUCCESSOR`, `POLICY-GEOMETRY`, `TILT-AUDIT`, `LEVEL2-PROBE` (the detection layer), `GPU-NATIVE-TRICK1` / `-M2` / `-M3` (the adjudicated GPU contracts), `UNIFICATION-CENSUS` (the 2026-08-24 fold), `ARCHIVE` (where the untracked outputs live, the recompute queue, producer commit `648f93a`). Seven completed-probe design docs were retired 2026-08-24; each era page cites the preserved bytes by commit hash (`git show 2de8a05:walt/<NAME>.md`) |
| [`experiments/partnership/`](../experiments/partnership/) | The partnership program directory (2026-09-06/07, CI-waived): `SCOPE.md` (the waiver's own words), `SESSION-STATUS.md` (the status ledger), `REPORT.md`, `PLAYERS.md`, `BASELINE.md` (the "phone"), `INNER-BELIEF.md`, `POLICY-SYNTHESIS.md`, `RELATIONAL-LEARNING.md`, the Python drivers, `campaigns/` and `runs/` (the results files that govern), `packet/` (the launch packet, preserved byte-for-byte) — [walt-partnership-program](walt-partnership-program.md) |
| `kanban/` (repo root) | The forward queue since 2026-08-24: one card per task, status = directory, `[[card-id]]` links; the board is listed on [Home](Home.md). `walt/PLAN.md` is retired — its historical content is at `git show 56e2173:walt/PLAN.md` |

## The workspace in one line

`walt/Cargo.toml` has **six members** — `walt`, `walt-gpu-ref`, `walt-metal`,
`walt-m2-runner`, `walt-wasm`, `walt2-wasm` — and the `walt` crate has **ten
modules** (`rules`, `kernel`, `geom`, `strat`, `spec`, `carrier`, `solver`,
`scheme`, `gym`, `policy_search`) and **54 binaries plus `webtable.html`** under
`src/bin/` (counts from `ls` on 2026-09-13 at `c00717d1`); the fold of
2026-08-24 (`d1499d4`: seven crates → seven modules of one crate, trace-identical,
after the census had sorted the workspace's seventeen crates into three stacks
plus orphans and the stage-1 deletions) is recorded in
`walt/UNIFICATION-CENSUS.md`. [walt-architecture](walt-architecture.md) owns
every detail — module sizes and import order, the two stacks, the gate's stages
and cost (about 308 s wall; the anchors suite 18.2 GB standalone at the FH4 audit
and 8.8 GB in-gate after FH5's job cap; not run on the 2026-09-06/07 commits),
the declared epochs, the debts.

## The ruling families, in `walt/CENSUS-RULINGS.md` order

Every family exists in the file (checked by `grep -o` of the prefixes on
2026-09-13); ranges and dates are on [walt-math-reference](walt-math-reference.md)
Appendix C. In section order: **SEP** (the separation probe), **N4** (the n = 4
rung, two sections), **EC** (the economy successor), **T1** (the trick-1 witness),
**LD** (lay downs), **RW** (the map-free rule walk), **FT** (the fusion tax, x:016),
**SR** (the second rung, x:017), **FF** (the feature-fee audition), **FC** (the
fee-correlation chapter), **SS** (the seed survey), **GT1** (GPU-native trick 1,
three sections), **SP** (signed pivotal), **FZ** (the freeze-56 v2 amendment),
**CE** (calculated evidence), **L2** (targeted level-2 field stability), **PANEL**
(the x:019–023 panel response), **TRIPLE** (the x:024 deferred producers),
**CBS** (counted belief), **APS** (the anytime proof state), **MB** (the
model-belief base player), **SC** (the salvation complex), **FH** (the focal
horizon, 2026-09-04 — the last section). Before SEP the file carries the census
fork rulings **F1–F7** (F7 = the NO-RESCUE protocol), the r3 quotient **Q1–Q5**,
the railyard **Y1–Y3**, and the probe families **P-A**, **X-A**, **E-A**, **S-A**,
**R-A**, **PG-A**, **J-A**, **DS-A** (2026-08-10 → 08-13). Ranges are append-only
and never move; sections are located by heading, never by line number.

## The unmerged branches

Measured with `git log main..<branch>` on 2026-09-13. Both fork from the
`walt-gran` line, which is itself in `main` (`git merge-base`: `walt-o5` at
`a0d594b2`, `walt-g1-l2` at `8174fa83`; the `walt-gran` tip is `9d6a5a2e`);
their results reach `main` only
through `walt/briefs/MORNING-2026-09-05.md`; [walt-gran-anchors](walt-gran-anchors.md)
owns the status.

| Branch | Ahead of `main` | Contents |
|---|---|---|
| `walt-o5` | **9 commits** (`0b65efb9` … `2981e090`) | O5 measured: the modeled minds' void-aware inner belief (`Level0Field::void_aware`), its gates and bins, the mirrored match whose "dead heat" is withdrawn after a seed repair; the readout records `check.sh PASS` on the branch. The void-aware *belief* itself reached `main` separately at `dbcc698f` (2026-09-06, default off) — the two-implementation fork is open ([[inner-voids-default]]) |
| `walt-g1-l2` | **8 commits** (`2d3907bd` … `6abdd78f`) | Level 2 at Gran's seat holds the 6-4 at both G1 nodes; G2 locked from trick 3; the synthetic lock; the tie-break `TieRule::LowestTileIndex` identified as the hoarding mechanism at exact indifference; the over-claim withdrawn |

`walt-gran` and `walt-fh` (PR #88) are fully contained in `main`. Jason's calls
(A)–(F) in the readout are open.

## Where it stands (2026-09-07, `c00717d1`)

The frame, twice-measured: **42 is two recursions running in opposite
directions**, and on the receipt corpus they trade dominance between trick 4 and
trick 5 — from trick 5 in, exact instruments are effectively free; trick 4 is
minutes per root; trick 3 is the wall (one root, h8-t3: 289M field reads and
14 min for the single-field exact solve, record
`walt/probes/factor_belief/horizon_run1.txt`; the FH3 record run that includes
it peaked at 19.4 GB, `focal_run1.txt`); earlier than that the
seat plays forward on sampled evidence with a certified regret it cannot yet
close (the opening root: play 6-5, floor 732‰, at most 267‰ unclaimed, the
split of the 267‰ UNKNOWN). At k ≥ 1 the remaining width is the tail's policy
gap, so the next money is a better lawful tail and a cheaper σ0 read key, not a
deeper search ([walt-focal-horizon-era](walt-focal-horizon-era.md);
[walt-program](walt-program.md) "Where the program stands").

What is ruled and queued, in order (`walt/MAP.md`): the σ0 read-key study
([[sigma0-read-key-study]]) → the consolidation slice ([[consolidation-slice]]:
retire `godgap.rs`, `horizon.rs`, `extraction.rs`, `refine.rs` as endpoints of
the focal-horizon hierarchy) → **no new mathematical parent until the
consolidation lands** (Jason, 2026-09-04). Beside that queue: the Gran calls
(A)–(F), the partnership strength question
([[partnership-strength-question]]), the void flag default
([[inner-voids-default]]), and the standing debts [[ladder-policy-store]] and
[[gate-corpus-trim]].

The partnership program (2026-09-06/07) is a local experiment under a CI
waiver: a lawful partner-aware player exists and is playable within the trick
target, and **no tested partner model has beaten the phone or L1** (L2 Partner
vs L1 14/14/72 at about 5× the cost; results file
`experiments/partnership/campaigns/default-partner-battery/RESULTS.md`);
Scheme/Fix and the exact gym are its instruments; defaults are unchanged
([walt-partnership-program](walt-partnership-program.md) §9). The GPU side track
holds portable M0/M1 and M2 Metal parity under freezes 55/56 and no player
([walt-gpu-native-trick1](walt-gpu-native-trick1.md)).

Every count above is carrier-relative, coordinate-relative, epoch-relative and
exploratory. Full numbers, scope caveats and dissents live on the era pages;
the refutations are collected at [negative results](walt-negative-results.md);
the results of the pre-pivot programs at
[walt-pre-pivot-results](walt-pre-pivot-results.md).
