id: [[lean-catchup]]
opened: 2026-08-24

## What

The Lean mechanization completed all 42 P0 rows (kernel-proved, commit
d190b26) but has catch-up work: the walt-era mathematics (signed
pivotal geometry identities, pmake objective, seat-census rulings) has
no kernel coverage, and the P1+ backlog was never triaged after P0
closed. Scope the next Lean tranche against [[math-reorg]]'s index once
it exists — the index decides what's worth kernel time. decide+kernel
discipline per lean/PROOFS.md is load-bearing.

## Done when

A triaged P1 list exists in lean/ referencing the math index; at
minimum the signed-pivotal boxed identities are assessed for
mechanization cost.

## Links

[[math-reorg]], lean/PROOFS.md
