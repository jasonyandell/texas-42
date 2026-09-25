id: [[sigma0-read-key-study]]
opened: 2026-09-13 (named next-in-order in `walt/MAP.md` since 2026-09-04)

## What

σ0 (`walt/walt/src/solver/field.rs`, the level-0 modeled mind for the
other three seats) reads the bid and the FULL public record, so its cache
key contains the record: cross-history cache reuse measured exactly 0 in
the Slice C1 cache study (2026-08-30,
`walt/probes/factor_belief/cache_run1.txt`), and classifying acting-seat
hands through σ0 is ≈ 99% of every recursion's bill (`walt/MAP.md` row 1).
FH3 measured a warm σ0 instance running a pass ~15× faster at identical
reads (`walt/briefs/FH3-REPORT.md`) — the field cache is the lever.

The study: does σ0's answer depend on the full record, or on a sufficient
statistic of it (live tiles, public voids, leader, the current trick's
prefix, banked counts)? If the latter, the key coarsens to that statistic
and every recursion — exact response, the focal-horizon hierarchy, model
belief — gets cheaper by the reuse factor (MAP's estimate 10–100×,
unmeasured). Method: enumerate σ0's decision over a corpus of public
records, group nodes by each candidate statistic, count answer collisions;
a zero-collision statistic is a valid key. Exactness must not move — the
field's answers are unchanged, only the key is.

## Done when

A probe record states, for the receipt-root corpus (13 hands) and the G1
line, which candidate statistic reproduces σ0's decision at every node with
zero collisions (or the smallest counterexample if none does); one gate pins
that identity on a named coordinate plus one pinned strictness witness (the
gate-sizing rule); if a coarser key exists, a second measurement states the
read-count and wall reduction on h8-t4 / h4-t4 (FH1 baseline 0.66M / 10M
reads, `focal_run0.txt`) and on h8-t3 (289M reads). No field semantics
change; `walt/ci/check.sh` green.

## Links

`walt/MAP.md` ("Next, in order"), `wiki/walt-focal-horizon-era.md` §7,
[[consolidation-slice]] (follows this card), [[ladder-policy-store]],
[[gate-corpus-trim]].
