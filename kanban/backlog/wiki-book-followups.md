id: [[wiki-book-followups]]
opened: 2026-09-13

## What

Documentation debts the book rewrite of 2026-09-12/13 (branch
`worktree-wiki-book`, repository state as of 2026-09-07 `c00717d1`) found
and did not close, because they need a run, a ruling, or a file outside the
rewrite's scope. None is evidence; each names its owner file. Items that
turn out to be already fixed by another chapter of the rewrite: strike
them here with the date.

**Needs a run or a receipt**

- `walt/ci/check.sh` was not run on the 2026-09-06/07 commits
  (`d8400713..c00717d1`, session waiver); whether main is green at HEAD
  is unverified. `lake build` at `c00717d1` likewise has no fresh receipt
  (last `lean/` commit title says "does not build"; `wiki/lean.md` states
  the status).
- The committed browser oracles may be behind their source:
  `walt/walt-wasm/pkg/walt.wasm` last rebuilt at df3ffcd5 (2026-08-24),
  `walt2-wasm/pkg` at 33d541fe (2026-08-25), while 161b0195 (σ1-repair)
  changed both `api.rs` files and the `walt` crate changed again on
  2026-09-06. The book must state which binary plunge actually ships
  ([[plunge-walt-sync]]).
- `walt/walt/tests/solver_calibrate.rs:420`
  `v5_literal_count_timing_position_reconstructs` is still `#[ignore]`
  naming [[gran-anchor-reconstruction]] as its blocker although G1 is
  committed — decide whether G1 is that position and un-ignore or
  re-label.
- `walt/ci/run_test_binaries.py` `HEAVY_FIRST` lists `solver_focal_budget`,
  which does not exist under `walt/walt/tests/` — stale hint or planned
  suite.

**Source docs that drift from the records**

- `walt/FACTOR-BELIEF.md` line ~261 (Phases 4+5 paragraph) carries the
  same `[145,606]‰` mislabel corrected 2026-09-13 in
  `walt/probes/factor_belief/README.md` (it is action 4-1's stage-0
  interval; 3-1 starts `[66,830]‰`). The ledger is a dated record —
  add a bracketed correction beside it, as the doom paragraph has.
- `walt/walt/src/lib.rs` doc comment lists eight modules and omits `gym`
  and `policy_search` (both import `solver`; `policy_search` imports
  `gym`).
- Probe READMEs cite kanban cards by path that has since moved
  (`walt/probes/fieldswap_cancel` → `kanban/doing/slice3-cancellation-ladder.md`;
  `fieldswap_motifs`, `hazard_witness` → `kanban/backlog/slice3-deferred-producers.md`;
  all now under `kanban/done/`). Convention is `[[card-id]]`.
- `experiments/partnership` prose: `campaigns/bench-pool10/STATUS.md`
  labels 12 repeated seeds "fresh" (POOL.md and `pool-benchmark.json`
  say repeats); `campaigns/native-l1-vs-phone-620600-649/RESULTS.md`
  ends with the random campaign's seed-exclusion boilerplate that does
  not apply to 620600–649. Results files win; the labels are wrong.
- The intake companion
  `walt/math/targeted_level2_field_stability_v0.1_intake.md` §"Gran-anchor
  gap" still says the seeds are not in-repo; repoint at
  `walt/probes/gran/` ([[gran-anchor-reconstruction]] item).
- `walt/SCENARIO-PLAYER.md`'s O5 row now carries a note that two
  implementations exist; branch `walt-o5` carries a different rewrite of
  the same row — whichever merges first must reconcile
  ([[inner-voids-default]]).

**Ledgers with missing rows (cross-reference rule in CLAUDE.md)**

- `wiki/walt-math-reference.md` has no PANEL-A (x:019–023) or TRIPLE-A
  (x:024) rows.
- The four Lean obligation ledgers the rulings accept obligations into
  (CBS-O1..O15, PS-T1..T15, MB-O1..O20 / MB-I1..I10, SC §60) exist in no
  file under `lean/` — [[lean-catchup]] should own them explicitly.
- Errata owed since 2026-08-13/14 (S6 page: Corollary E4.1 §4.3; the
  seven FT + eight SR objects §9) are untracked by any card.
- `exchange/README.md`: Pro's seven hand-delivered parents (CE, L2, CBS,
  APS, MB, SC, FH) have A-ruling families in `walt/CENSUS-RULINGS.md` but
  no courier-ledger row or pointer; dispatch 018's delivery is
  unconfirmed (24 days without a reply at `c00717d1`); the tally's
  semantics (24 = dispatches ever sent, by cancellation of a duplicate
  and a possible uncounted hand-ferry) — Jason's call.

**Numbers with two values in the tree**

- `wiki/game-of-42.md` §5 "z = +6.28 over 6,015 paired contracts" vs
  `walt/LOG.md` "pooled walt 630/1152" for the 2026-08-17 match — read
  `walt/probes/m3/arena_results_2026-08-17.txt` and make one of them
  match the results file.
- Step 9 README / wiki "pivotal mass drops on 13/18" vs the records' 12
  drops and one dq = 0.

**Structure**

- `walt/MAP.md`'s header promises "rewritten at every landing"; the
  2026-09-06 landings were prepended instead. The 2026-09-13 rewrite
  folded them in; the next landing must rewrite, not prepend.
- `wiki/walt-architecture.md` (NEW in the book's page map) was absent from
  the worktree at the start of this pass and present, uncommitted, by its
  end (2026-09-13); confirm it is committed before Home links to it.
- The instrument inventory (`wiki/walt-instruments.md`) has drifted on
  prose bin counts three times ("nineteen", "twenty-seven"); 54 bin
  sources at `c00717d1`. Consider a generated count.

## Done when

Every bullet is either fixed in its owner file (with the fix dated) or
carried by a card of its own; this card then moves to done/.

## Links

`wiki/Home.md`, `walt/LOG.md` (2026-09-12/13 entry), `walt/MAP.md`,
[[gran-anchor-reconstruction]], [[inner-voids-default]],
[[plunge-walt-sync]], [[lean-catchup]], [[hf-archive-upload]].

## Verification status after round 2 (2026-09-13)

Verified-and-fixed by an independent agent (about 210 numbers checked per page): walt-seat-play (3 MAJOR fixed), walt-focal-horizon-era (3 MAJOR fixed), walt-counted-belief-era (2 MAJOR fixed), walt-instruments (4 MAJOR fixed). Calibration: roughly three MAJOR defects per unverified chapter. NOT yet verified by an independent agent: every other page touched by the rewrite (core math pages, ledgers, exchange, lean, rob, game-of-42, timeline, vocabulary, walt-program, walt-negative-results, walt-pre-pivot-results, walt-math-*, walt-calculated-evidence, walt-gran-anchors, walt-partnership-program, walt-gym, walt-scheme-fix, walt-gpu-native-trick1, walt-architecture, and the doorways Home/QUICKSTART/walt). Next rounds: bundled verifiers (max 5 agents per round), then a completeness critic, then the draft PR.

### Findings left open or handed to other pages by the round-2 verifiers

- [MINOR/not_a_defect] walt-counted-belief-era.md @ §3 Slice G regime 1 note and §5.6 [145,606]‰ note: The page's bracketed corrections of the ledger (five/five vs six/four SETTLED/EQUIVALENT; [145,606]‰ is action 4-1's stage 0) → Both were suspects; both verified correct as written. | fix: None needed.
- [MINOR/not_a_defect] walt-counted-belief-era.md @ §7 doom paragraph: Original ledger sentence, first-pass restatement, the SC §8 decomposition, the DISCREPANCIES.md correction and the exact recovery list (all  → Suspect list demanded all of these; all present and verbatim against walt/DISCREPANCIES.md and doomreport_run1.txt. | fix: None needed.
- [MINOR/left] walt-seat-play.md @ §9 debts, 'CI waived 2026-09-06/07' row: 'focused suites only: 44 then 36 Rust tests, 14 Python tests' attributed to 'the partnership and Scheme sessions' → The 36-test figure is the 2026-09-06 InnerBelief session's focused run (INNER-BELIEF.md:124), i.e. still the partnership session, not the Scheme session; the wa | fix: Left as is — the counts and the waiver a
- [MINOR/left] walt-focal-horizon-era.md @ 'The five days, dated' table, last row: A 2026-09-06 'Postscript' row is placed after the 2026-09-07 PR #88 row → Chronological order is broken by one row; the row is explicitly labelled Postscript and its content (c59f1115 changes condition_via and rewrites G2/G8) is verif | fix: Left as is — moving it would break the writer's 'landing, then postscript' 
- [MINOR/not_a_defect] walt-focal-horizon-era.md @ Whole page — hardest numbers: MB1 eight-row Φ table, the 38/9600 pin, the trick-3 refusal figures, the h8-t3 exact-solve figures with both walls, all UP0/UP1a figures, th → Checked as listed in checks_done; every one matched its primary source (records outranking prose), so no further edit was warranted. | fix: None needed.
- [MINOR/not_a_defect] walt-instruments.md @ §4 scenario/level1/level2 row and §8 item 5: level-2 cost figure in prose ('25–50×') is not in the results file → Correctly flagged as unverified; I confirmed no such factor appears in level2_results_2026-08-17.txt (absolute timings only). Nothing to fix on this page; the p | fix: None here; noted for walt-seat-play's owner in cross_page_notes.
- (cross-page) walt/FACTOR-BELIEF.md lines 269–270 (Phases 4+5 paragraph) say gain equals the envelope at h8-t5; bellmanreport_run1.txt prints 26/32/27 against 33. The ledger is a dated running record (not mine to edit); any page restating the covers finding (walt-negative-r
- (cross-page) walt/FACTOR-BELIEF.md line 1169 says 'six SETTLED, four honest EQUIVALENT ties' for refine_run1.txt; the record is five and five. Same ledger-vs-record caveat for any page restating Slice G.
- (cross-page) walt/probes/factor_belief/README.md line 667 quotes '~2.5–9k walk nodes/s' for the opening doom leads; per-lead arithmetic from doomreport_run1.txt spans ≈0.8k–10.9k/s. The README is the CBS-A8 authority, so this is for the README owner to reconcile, not the w
- (cross-page) wiki/walt-focal-horizon-era.md is named as successor in the owns line now; its owner should confirm it links back here as predecessor (its line 1 owns clause was read and covers 2026-09-01 → 2026-09-05 as expected).
- (cross-page) walt/SCENARIO-PLAYER.md (owner: whoever holds the spec on this branch): the Def 6.3 bound note cites `solver/act.rs` lines 288 and 315 and `solver/mod.rs:1500` — verified correct at c00717d1 (act.rs:288/315 `tie_rule: TieRule::LowestTileIndex`, mod.rs:1500 `pu
- (cross-page) wiki/walt-scheme-fix.md owner: the seat-play page's CI-waiver row says '44 then 36 Rust tests' for 'the partnership and Scheme sessions'; the 36 is INNER-BELIEF.md's focused run (partnership session). If the Scheme session's focused suite count is recorded on 
- (cross-page) wiki/walt-partnership-program.md owner: seat-play now names `campaigns/default-partner-battery/STRENGTH-ASSESSMENT.md` as the source of the 'defensible operating default' quote and fences 'proved' inside it; your page quotes the same sentence at line ~336/467 
- (cross-page) wiki/lineage.md owner: seat-play attributes '6.5σ on 2026-07-30' to the lineage page, but lineage.md does not print the sigma figure (it is in walt/probes/m3/arena_results_2026-08-17.txt:113 and the dropped-30 record); harmless as a pointer, but if lineage.md 
- (cross-page) Owner of walt/DISCREPANCIES.md, walt/FACTOR-BELIEF.md (head note) and the walt-partnership-program chapter: the 2026-09-13 spot runs on the 2026-09-07 release binaries give identical masses but fewer field reads than every committed record compared (h8-t4 foca
- (cross-page) Any page that quotes the FH1 scout's read column (walt-instruments.md, walt-counted-belief-era.md, walt-tools-catalog survey) should be checked for the same non-record numbers (1.73M/2.55M/2.78M, 4.98M/8.34M, 0.13M); the record (focal_run0.txt) has 1.78M/2.63M
- (cross-page) kanban/backlog/consolidation-slice.md and kanban/backlog/sigma0-read-key-study.md were opened 2026-09-13 on this branch (6a319216); any chapter or open-questions page still saying these have no card (walt-math-open-questions.md, ideas/meta pages) is stale.
- (cross-page) walt/MAP.md's cost row now says solver/ was measured 2026-09-13 on c00717d1 (38,013 lines, 39 files); this page's Appendix says the same count was taken 2026-09-12 — both are true of c00717d1; no conflict, but the timeline page owner may want one date.
- (cross-page) ORCHESTRATOR: commit 6a319216 on the wiki-book branch (the 'round 1' wiki commit) edited four walt/ prose files — walt/LOG.md, walt/MAP.md, walt/SCENARIO-PLAYER.md, walt/probes/factor_belief/README.md — outside the wiki/ scope the hard rules assign to this rew
- (cross-page) walt/walt/tests/solver_fieldswap_cancel.rs:459 assertion message reads 'the directional sandwich holds exactly at {action}' — 'sandwich' used as an object name in a gate file (CBS-A3/FH-A2 vocabulary); a walt/ file, not a wiki edit. FH5 (b6de5a25) retired the 
- (cross-page) walt/walt-wasm/pkg/README.md:12 says walt.wasm is '~250 KB'; the committed file is 304,312 bytes. Stale README figure (walt/ file).
- (cross-page) walt/ci/run_test_binaries.py HEAVY_FIRST names a nonexistent suite `solver_focal_budget` (already on the page and in §8; walt/ file, not repaired).
- (cross-page) walt/probes/bundle/README.md:90 still says '~5–6x fewer nodes' where the record's ratio is 1,108,800/224,983 ≈ 4.9× (the page already says so; walt/ file).
- (cross-page) walt-seat-play.md owner: the 'level 2 costs ≈ 25–50× level-1 per decision' factor is not in level2_results_2026-08-17.txt (absolute timings only); this page flags it unverified in §8.
- (cross-page) walt-partnership-program.md / walt-gym.md owners: the 8/15/77 native-vs-phone figure lives in experiments/partnership/campaigns/native-l1-vs-phone-620600-649/RESULTS.md (not in foundation-battery); the 433/367/26 and 24/30 vs 26/30 figures live in walt/gym/BID
- (cross-page) walt-seat-play.md / walt-partnership-program.md owners: the `partnership` binary's request grammar is 'bare mode word on line 1, then field lines' (no `mode` field); if either page shows a request example, check it against experiments/partnership/player.py::na
- (cross-page) Home.md / walt.md owners: the walt-wasm and walt2-wasm committed binaries (df3ffcd5 2026-08-24, 33d541fe 2026-08-25) predate the σ1-repair (161b0195) and inner-belief (dbcc698f) changes to both crates' src/api.rs; whether the phone artifact is intentionally fr

### Doorways writer open items

- Ruling-family order: walt.md lists SEP, N4, EC, … in section-heading order (matching walt-math-reference Appendix C) rather than the assignment's SEP, EC, N4; by first ID occurrence EC-A (line 3942) precedes N4-A (4097) but EC's own section (line 4689) follows
- Home's layer table and doorway section link pages that also appear once in the Pages TOC; 'linked exactly once' was applied within the Pages section (checker: 65 pages, no missing, no duplicates).
- The literal `[[card-id]]` in each file's kanban-convention sentence is deliberately not a card; a strict card-token checker will flag it (it did in the previous versions too).
- The shared worktree carries uncommitted edits by other writers to wiki/walt-counted-belief-era.md, wiki/walt-focal-horizon-era.md, wiki/walt-instruments.md and wiki/walt-seat-play.md (git status); not touched by this assignment.
- QUICKSTART says the CE §22 build ran 'through step 9' (per walt-calculated-evidence's 'Step 9' section and timeline Wave 1), while walt-calculated-evidence's owns line still says 'steps 2–8' — flag for that page's owner.
- walt-program's closing section and walt-focal-horizon-era §7 both say the σ0 read-key study and consolidation slice carry 'no kanban card'; cards [[sigma0-read-key-study]] and [[consolidation-slice]] were opened 2026-09-13 and the doorways cite them — those tw
- walt/MAP.md counts 70 test files, the survey said 72; neither figure is quoted in the doorways.
