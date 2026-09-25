id: [[consolidation-slice]]
opened: 2026-09-13 (Jason's ruling of 2026-09-04)

## What

The focal-horizon hierarchy (`walt/walt/src/solver/focal_horizon.rs`, FH1)
is the one object: U0's God-gap census is `U_{a,0}`, U0b's in-solve ply cut
is `U_{a,m−1}` on viewer-lead roots (Proposition FH-cut), the never-built
salvation-mask upper is `U_{a,1}` (Theorem 5), rollout improvement is
`L_k`, argmax extraction is `π_k`, and the exact endpoint is the collapse.
The tree-shake list from `walt/MAP.md`: retire `godgap.rs` (933 lines),
`horizon.rs` (635) and `extraction.rs` (135) as measurement scaffolding
around one recursion, and `refine.rs` (917 — freeze 58 is a *semantic*
freeze of RefineV1, so its oracle values must survive as a byte-diffed
fixture, not as live code); `doom.rs` (1,048) stays as the God tail's
engine. Every gate those modules carry is re-expressed against the
hierarchy or shown redundant; every probe record they produced stays
quotable from its file, untouched.

Jason's ruling (2026-09-04): "follow through on what we have, then invest
in a simplification/unification attempt" — **no new mathematical parent
until this lands.** Runs after [[sigma0-read-key-study]].

## Done when

The four modules are gone or reduced to thin views over
`focal_horizon.rs` / `focal_ladder.rs`; the U0 / U0b / Phase-6 findings
reproduce from the hierarchy on their pinned coordinates (the h8-t4 3-1
fusion price 38/9600, the h8-t3 ply-cut over-price 236/7497, h3-t4's
Γ 83‰ → 0‰); freeze 58's RefineV1 oracle is preserved as a fixture the
gate byte-diffs; `solver/` shrinks and the gate wall does not grow;
`walt/ci/check.sh` green; `walt/MAP.md` rewritten at the landing.

## Links

`walt/MAP.md` ("What the hierarchy makes redundant"),
`wiki/walt-focal-horizon-era.md` §7, `walt/briefs/FH3-REPORT.md`,
`walt/briefs/FH4-AUDIT.md`, [[sigma0-read-key-study]],
[[ladder-policy-store]], [[gate-corpus-trim]].
