id: [[ladder-policy-store]]
opened: 2026-09-04

## What

The focal-horizon ladder (`walt/walt/src/solver/focal_ladder.rs`, FH2)
stores a full policy table beside every node's lower fact — the
FH-int requirement that every lower carries its witness — by copying
the subtree's choice table into each ancestor's fact. At h8-t3 that is
3.82M facts and 19.4 GB peak RSS (FH3 record; the anchors gate peaks at
17.8 GB with five concurrent h4-t4 ladders). Replace the copies with a
version-referenced policy store: each fact references the child
policies by content id, the total policy is a derived view assembled on
demand. Same facts, same views, same gates; memory should fall to the
size of the distinct choice tables.

## Done when

FH2's nine gates and FH3's anchors gate pass unchanged; the h8-t3
ladder record reproduces byte-for-byte; peak RSS at h8-t3 is under
2 GB and the anchors gate under 4 GB (measured and stated in the
report).

## Links

`walt/briefs/FH2-REPORT.md` (the deviation that named the fix),
`walt/briefs/FH3-REPORT.md` (the numbers), `walt/MAP.md` (the cost
axis), [[gate-corpus-trim]].
