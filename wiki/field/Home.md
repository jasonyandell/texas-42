# Field — rob beyond the repo

[Home](../Home.md) · owns: measurements and lessons from seating rob against
players outside this repository, and the direction map those encounters opened ·
Sources: none upward (this area cites; nothing above the Ideas tier ever cites it).
Related: [lineage](../lineage.md), [analysis](../analysis.md), [ideas](../ideas.md).

> **Epistemic tier: FIELD MEASUREMENT / EXPLORATORY — below every tier on
> [Home](../Home.md#evidentiary-tiers--never-promoted-never-blurred).** Numbers in
> this area are arena measurements computed in the sibling mk5 repository, not by
> the certified Rust, and not reproduced as receipts. They carry **zero evidentiary
> weight** for the mathematics: they may not be quoted in a brief, a dispatch,
> FINDINGS, or any claim-tier page. Their only legitimate downstream use is to
> *motivate* — an idea, a brief, or a dispatch, each of which then earns its own
> tier from scratch. This area does not reframe the project. The project is math
> first and math primary; only a discovery at the kernel tier could reframe it.

## Why this area exists

The mathematics was built to answer [the wall](../lineage.md) — and in July 2026
rob was, for the first time, physically seated against the champion it names.
Those encounters produced numbers, craft, and direction that are none of them
theorems, but all of them too valuable to lose to unversioned memory. This area is
the fence-and-filing for exactly that material: partitioned so the pure math stays
pure, committed so the record survives.

## The sequel, in one paragraph (2026-08-17; exploratory)

rob's opening was never replaced. Instead the project pivoted on 2026-08-17: [walt](../walt.md)
became the player, with P(make the bid) ruled as the objective, and `walt_bridge` — speaking
the same line protocol as `rob_bridge`, zero arena changes, ~15k decisions rules-clean — beat
the same E[Q] n=10 champion under the same dropped-30 3×384 protocol: **walt 630/1152 games
(54.7 %), McNemar z = +6.28 over 6,015 paired contracts**, every seed's mark-margin CI
excluding zero, losing ~4.7 points per hand and winning the marks
([walt-seat-play](../walt-seat-play.md); record `walt/probes/m3/arena_results_2026-08-17.txt`).
The same fence applies to it as to everything here: an arena outcome about play, at the
exploratory tier, against a modelled field; [lineage](../lineage.md) deliberately does not rule
whether it is the *demonstrated strategic reason* the wall demands. This area records rob's
encounters and stays rob's; walt's are owned by the walt pages.

## Pages

- [first-contact](first-contact.md) — the 2026-07-30 encounters with E[Q] n=10:
  the full-hand loss, the mid-hand dead heat, and the localization those two
  results pin down together.
- [lessons](lessons.md) — transferable craft from the encounters: protocol
  design, the bridge-as-conformance-instrument, interpretation discipline.
- [directions](directions.md) — the direction map stated after first contact:
  what sigma is and is not, the four shapes beyond rob, and the north star.
  (Ideas-tier capture; promotion paths per [ideas](../ideas.md).)

## Provenance convention

Every number here cites: the mk5 worktree branch and commit that produced it, the
artifact path (mk5 side), and the texas-42 commit of the `rob_bridge` binary that
played the games. mk5-side branches referenced from this area were **local-only**
at time of writing; the citation is to content, not to a public ref. Checked 2026-09-12:
the local-only branch `rob-vs-eq` still exists in `~/code/mk5-main`, and the write-ups
`arena/evidence/DROPPED30_RESULTS.md` and `arena/evidence/MIDHAND_RESULTS.md` plus the
drivers `arena/rob_play.py` and `arena/midhand_eval.py` are present on disk (dated
2026-07-30); the raw results directories `arena/results/dropped30_384*` and
`arena/results/midhand_t*_256` were not found on that checkout's working tree and were not
verified further in this pass.
