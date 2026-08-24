id: [[wiki-overhaul]]
opened: 2026-08-24

## What

Deep re-synthesis of the wiki — much has happened since last sync.
Specifically ruled by Jason 2026-08-24:

- **Reframe primacy**: walt is the primary player now, built
  iteratively; rob is still framed as primary and that's out of date.
  rob's mathematical engineering remains the aspirational example.
- **Consolidate scattered docs**: walt/'s ~20 root .md files grew
  organically; fold what's historical into wiki pages, keep what's
  load-bearing, retire what's superseded (PLAN.md already ruled gone).
- Update the math pages ([[math-reorg]] owns the math side).
- Home.md exchange status line and cross-reference ledgers
  (claim-ledger, FINDINGS, open-problems) re-synced.

Runs AFTER [[walt-unification]] so it synthesizes the unified shape
once.

## Done when

wiki/Home.md maps the current project truthfully; every walt-era result
has an owning page at its correct tier; no doc contradicts the unified
crate layout.

## Links

[[walt-unification]], [[math-reorg]], wiki/Home.md

## Closed 2026-08-24

Executed on branch `wiki-overhaul` (a750e84..659a818, off merged main
2de8a05), five chunks + a lineage note: primacy reframed (walt = the
player, rob = the exact-truth exemplar; Home.md now seven layers with
kanban), seven closed probe design docs retired from walt/ (each
folded into its owning era/reference page first; bytes at
`git show 2de8a05:walt/<NAME>.md`; ARCHIVE.md records it),
walt-instruments rewritten to the unified five-crate layout, stale
`walt-factory/results/` citations resolved (~10 live pointers
rewritten, ~25 historical mentions covered by page-level layout
notes), ledgers re-synced (exchange count 18 everywhere; 016–018
hand-ferried; x:018 awaiting Pro's reply), freeze register through 57
+ 56-v2. Corrections found with evidence: the freeze-43 deadness
timing rung WAS run (deadness_rung_2026-08-13.txt — 17 and 42 ns/call
quotable; the "unrun/25 ns" caveat was stale, verified by re-reading
the file); "M3–M5 untouched" updated to freeze-57/GT1-A24 chapter-
closed on three pages. Deep math gathering deliberately deferred to
[[math-reorg]]; Lean tree reconciliation to [[lean-catchup]]; budget
prose to [[exchange-quota-reframe]].
