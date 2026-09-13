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
