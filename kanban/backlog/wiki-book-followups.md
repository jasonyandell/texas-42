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

## Verification status after round 3 (2026-09-13)

Verified-and-fixed by independent agents in round 3 (about 1,980 numbers checked in total; 10 MAJOR fixed, 0 BLOCK): the twelve Part I core pages plus package-provenance and discrepancies (1 MAJOR: STR-07/08 mislabelled in a kernel block); claim-ledger, FINDINGS, open-problems, verification, exchange chapter and exchange/README hygiene (3 MAJOR); lean.md, lean-row-index, proof-assistant-plan, lean/README, rob.md, rob-slices, analysis, field/*, first-implementation-slice, lineage, rob READMEs (1 MAJOR: rob/README overclaim); game-of-42, timeline, vocabulary, walt-program, walt-negative-results, walt-pre-pivot-results, walt-decision-sparse (1 MAJOR: stale "no kanban card"); walt-math-reference, walt-math-open-questions, walt-math-intakes, walt-math-freezes, the four small math subpages, walt-calculated-evidence (4 MAJOR: a probe number presented as gate-cited; a false "hash not in history" claim; a false "nothing else names the obligation ledgers" claim; the ladder.rs basis note).

STILL unverified by an independent agent after round 3: walt-gran-anchors, walt-partnership-program, walt-gym, walt-scheme-fix, walt-gpu-native-trick1, walt-architecture, the doorways (Home, QUICKSTART, walt.md), ideas + idea-* pages, walt/MAP.md, walt/LOG.md and the kanban edits. Round 4 = those five bundles; round 5 = completeness critic against the survey maps, final checks, draft PR.

### Findings left open or handed to other pages by the round-3 verifiers

- [MINOR/not_a_defect] lean.md @ §7.4 Receipt-inconsistent bullet: receipt formatting 'consistent with an intermediate build during the overnight session before nine theorems were added — → Not a defect — flagged for the record: the page correctly labels this inference as unconfirmed; I verified only that the nine missing names are exactl | fix: None needed.
- [MINOR/left] first-contact.md @ Artifacts lines (mk5 commits 3193295f, 2d7375bc, 5: mk5-side commit hashes for the write-ups and driver → Cannot be verified from this worktree (git is confined to this repository); the mk5 files themselves exist on disk dated 2026-07-30 and their numbers  | fix: None; the provenance convention on field/Home.md already discloses that mk5 refs are local-only and co
- [MINOR/not_a_defect] reduced-viewer-kernel.md @ OPEN-01 COLLAPSE bullet: 'exchange-adjudicated CON: x:003 ran ALL_PASS in 0.43 s. → exchange/README.md row 003 records '0.4s'; the page's '0.43s' matches wiki/claim-ledger.md line 129, so the two ledgers differ only by rounding. Not a | fix: None needed; noted for the claim-ledger owner in cross_page_notes.
- [MINOR/not_a_defect] rules-profile.md @ Mechanization-status preamble on all twelve pages:: No sorry / native_decide / axiom under lean/Texas42 as of 2026-09-12. → Date is one day older than the required fresh-measurement date, but the statement is a true dated record and I re-ran the same grep today with the sam | fix: Left as-is (true, dated); today's re-verification recorded here.
- [MINOR/not_a_defect] walt-math-reference.md @ Sources line and Appendix C: '35 ruling families' → Suspected miscount; verified correct (31 ruling-ID families of the form XX-An plus the F, r3 Q, Y and shape-notion series = 35 table rows). | fix: None needed beyond the Appendix C row-count correction above
- [MINOR/not_a_defect] walt-math-intakes.md @ §7 pinned manifests: The nine parent digests listed in full match the .sha256 files and the parents → Suspected transcription drift; all nine 64-hex digests match the .sha256 files character for character and a fresh shasum of each parent (2026-09-13). | fix: None
- [MINOR/not_a_defect] walt-calculated-evidence.md @ 'What the shadow work found in the live player' §3: The standalone PiKey copy in playout.rs survives #83 unchanged at line 144 with five fields and no banked totals; still  → Suspected stale after commit 161b0195 rewrote playout.rs; verified the page states what the tree shows. | fix: None
- [MINOR/not_a_defect] walt-calculated-evidence.md @ 'The cap analysis, and Jason's 512 ruling' table a: 512 epoch: 67/113/3, 30 winners with 24/30 live agreement, 112/113 among survivors, settlement worlds 196/333/395; step  → Suspected prose-vs-record drift; every figure re-derived from the records (summarize.py on the two 512 jsonl files; a recount over records.jsonl; the  | fix: None
- [MINOR/not_a_defect] verification.md @ Caveat 1 (the __pycache__ trap): Known suspect: caveat 1 might still say the ingest copies 'contain __pycache__' → Not present — the caveat already states the checked-in tree is clean, quotes the withdrawn wording, and matches discrepancies.md D15 and FINDINGS §3 v | fix: None needed; only the reproduction date was refreshed to 2026-09-13.
- [MINOR/not_a_defect] README.md @ Status ledger rows 001–024: Only hygiene edits happened (row 017 SR-A37, dated 018 note, intakes pointer, ordering) → Confirmed: a row-sorted diff against c00717d1 shows only rows 017 and 018 differ, in exactly the stated places; 20 rows before and after. | fix: None.
- [MINOR/not_a_defect] README.md @ 'Where the drafts went' provenance table: Six former drafts now outbox 019–024; responses in inbox; PR #40 for the 024 draft → All six outbox files, both inbox responses, and PR #40 (4c452fe2) exist; links resolve. | fix: None.
- [MINOR/left] README.md @ Prompt discipline, line 64: 'Compute Z exactly and provide a certificate Claude can verify mechanically.' → Uses 'certificate' in a non-quotation template sentence (D3 vocabulary); pre-existing at c00717d1, not introduced by the rewrite; wiki/exchange.md §1. | fix: Left unchanged — altering the operational README's prompt template is beyond the hygiene-only scope the ass
- [MINOR/left] verification.md @ S10 stretch paragraph: 'eight-star admissible-module language (119 four-groups per star; 3,808 declaration/group/desired-winner cases …)' → 119 is not a receipt line — verify_floor.txt prints only 3,808 (= 8 stars × 119 × 4 winners); the figure comes from x:001's construction. Not wrong, b | fix: Left as is (arithmetically consistent and attributed to the x:001 con
- [MINOR/not_a_defect] game-of-42.md @ §5.3 and §5.9 table row 'The seat plays': McNemar z = +6.28 over 6,015 paired contracts; discordant 687 vs 473 → Suspect named by the assignment (the writer had been asked to verify 6,015). | fix: None needed.
- [MINOR/not_a_defect] walt-program.md @ §12, Jason's frame quotations: Quotations from BRIEF-MB1.md and BRIEF-UP0.md are verbatim. → Suspect named by the assignment. | fix: None needed.
- (cross-page) QUICKSTART.md line 245 still lists 'rob slice 03 targets' as if queued (rob-slices.md now says so explicitly); QUICKSTART's owner should add the pivot/open-call framing or point at rob-slices.md 'Named but never begun'.
- (cross-page) Home.md row 29 / QUICKSTART line 105 'twelve byte-diffed receipts reproducing every ingest number' remains the overclaim fixed here in first-implementation-slice.md and rob/README.md; owner of Home/QUICKSTART should change to 'every slice-01/02 ingest number'.
- (cross-page) wiki/claim-ledger.md (not mine): lean.md §8 and lean-row-index.md state the constellation modules carry no ledger row and that the C1 row 'Lean mechanization pending (dispatch 011)' is still accurate; the audit's suggested 'Lean thread (x:011/013/015)' row and
- (cross-page) wiki/verification.md (not mine): rob.md §3 now carries the INV-P enforcement table and names the three brief-promised tests that do not exist (inv_p2_partition, inv_p4_determinism, inv_p5_conservation); verification.md's owns-line still claims every verifier a
- (cross-page) wiki/walt-seat-play.md (verified by another agent) is the owner of the 630/1152, z=+6.28, 6,015 figures now quoted on field/Home.md, field/first-contact.md, rob.md §10 and lineage.md — all four quote the same arena_results_2026-08-17.txt values; any later corr
- (cross-page) lean/PROOFS.md (not in my file list): rule 2's text still names only the commit-message receipt form; lean/README.md's status note and lean.md §4.1/§12.4 document the file form, but PROOFS.md itself was not edited.
- (cross-page) wiki/claim-ledger.md line 129 says x:003 'ALL_PASS 0.43s' while exchange/README.md row 003 says '0.4s' — harmless rounding, but the ledger owner may want to align to the README (results file outranks).
- (cross-page) wiki/verification.md and wiki/FINDINGS.md §3 already carry the D15 correction (never-tracked __pycache__); discrepancies.md D15 now points to them as carrying the correction, not the stale sentence — no action needed by their owners, but do not reintroduce the
- (cross-page) wiki/lean-row-index.md and wiki/proof-assistant-plan.md are the pages the twelve Mechanization blocks link to for the row-to-declaration map; every row/priority I checked agrees with lean-row-index.md (27 p1 rows; 20 open + 3 half-rows), so if proof-assistant-
- (cross-page) wiki/open-problems.md item 3 (minimality of the 90-world witness) is what belief-vs-support.md and strategic-state.md cite as still open — keep that item number stable or update those two links.
- (cross-page) Untracked files 004c.out and 004c.time appeared at the worktree root during this run (not mine; likely another agent re-running exchange/adjudication/programs/004-cocycle.py) — the orchestrator should delete or gitignore them before committing.
- (cross-page) walt/CENSUS-RULINGS.md SC-A6 (2026-09-01) states the salvation parent's doom-census hash `eb5a459…` 'does not appear in main's history'; measured 2026-09-13, eb5a459d IS on main as the PR #79 squash commit (walt doom census, 2026-09-01). The rulings file is ap
- (cross-page) Any page repeating 'the obligation ledgers CBS-O1..O15 / PS-T1..T15 / MB-O / SC-O are named nowhere under wiki/lean.md or kanban/' is wrong: wiki/lean.md §10 and rows ~905/1069, wiki/lean-row-index.md ~225 and kanban/backlog/lean-catchup.md all name them (as a
- (cross-page) Any page saying the h8-t3 value 28859/29988 is asserted or cited by the anchors gate walt/walt/tests/solver_focal_anchors.rs is wrong: the gate covers seven h8-t4/h4-t4 coordinates, h8-t3 is PROBE-ONLY (gate header; FH4-AUDIT N5); no test file contains 28859. 
- (cross-page) Any page saying the pmake ruling's basis ladder.rs 'no longer exists on main' or 'the crate was deleted' is imprecise: the walt-m3-probe crate is gone but the file was folded as pure code motion into walt/walt/src/bin/ladder.rs at d1499d43 (2026-08-24) and exi
- (cross-page) Consumer counts for 'freeze 58' outside CENSUS-RULINGS are 14 briefs (case-insensitive; 12 exact-case) and 5 solver src/test files; walt-math-freezes.md's row 58 already says fourteen — other pages quoting 'eleven briefs and six files' should be aligned.
- (cross-page) The step-9 README's prose '13/18 pairs' vs its records.jsonl 12/1/5 is a record-vs-prose discrepancy in walt/probes/step9/README.md itself (not a wiki file); wiki pages now cite 12/18 with the recount.
- (cross-page) walt/probes/shadow/README.md aggregate section still prints only the 128-epoch aggregate (27; 23/27); the 512-epoch aggregate (30; 24/30; 112/113) is obtainable only by running summarize.py — walt-calculated-evidence.md says so; walt-instruments.md could note 
- (cross-page) wiki/idea-retrograde-rank.md line 236 still carries 'pending Lean' but is now annotated (2026-09-13) with 'C1 is stated in neither file'; exchange.md §10 reports it as annotated, not cleared — owner may want to retire the phrase outright.
- (cross-page) automation/submit.mjs header comment and .claude/skills/pro-exchange/SKILL.md still quote 'count 18 > cap 17' as their example; automation/finish-001.mjs keeps a dead 'count >= 10' guard; exchange/adjudication/workflow.js hard-codes REPO=/Users/jason/code/texa
- (cross-page) exchange/inbox/010-…FAILED.md (SIGTERM 2026-08-01T08:31:33Z) and exchange/outbox/017-…md.ready are housekeeping residue; not edited (not wiki pages, and deleting inbox/outbox files is beyond a hygiene edit).
- (cross-page) wiki/Home.md: the exchange row and tier-3 sentence flagged by the audit are already cleared in the working tree (no 'Dispatches 001–018', no '3/3 adversarial referees SOUND'); the Home owner should keep the tier-3 wording aligned with the claim-ledger rule ('p
- (cross-page) wiki/rob-slices.md line 74 names x:007/x:008 as slice-03 targets but not x:006 (REACH-18); exchange.md §2 and open-problems now say rob has reproduced none of REACH-18/19/20 — the rob-slices owner may want to add x:006 to the slice-03 target list or state why 
- (cross-page) wiki/walt-gym.md §3 'The three audits' supports open-problems' 'thrice-audited answer keys'; no change needed, noted for the gym owner in case that section is renamed.
- (cross-page) wiki/walt-s6-era.md (not mine) still carries the stale 'sequential timing rung unrun' sentences at its S6c section and discrepancy bullet (Appendix C row 9 of walt-pre-pivot-results now names it as the remaining stale page); the rung exists at walt/probes/fact
- (cross-page) walt/briefs/U0B-REPORT.md line 126 says h8-t3 was 'solved EXACTLY under σ0 in 14 min 13 s (289,407,472 …)' while the record horizon_run1.txt line 11932 prints wall 797 s (13 min 17 s) and the report's own line 155–156 separates the two; walt-focal-horizon-era.
- (cross-page) vocabulary.md cites two Jason rulings of 2026-09-04 (no hand-coded features; estimates must be loud) as 'session record, not yet filed in a repository document' — a repository filing (e.g. a MAP.md or CENSUS-RULINGS note) would let those entries cite a file; o
- (cross-page) No kanban card tracks the errata filing owed for Corollary E4.1 (§4.3) and the FT/SR/FF/FC objects (§9) — walt-decision-sparse.md and walt-program.md both say so and are consistent; if the kanban owner wants every standing debt carded, this is the one uncarded
- (cross-page) experiments/partnership/campaigns/random-420600-699/RESULTS.md and worlds-520600-699/WORLD-RESULTS.md print counts only; the −4.1 pp / −3.0 pp figures used across walt-partnership-program.md (not mine) are derived by the CALIBRATION.md convention — that page s
