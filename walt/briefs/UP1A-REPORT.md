# UP1A-REPORT — the lazy carry, as built

**Slice:** UP1a, the first of UP0's three named UP1 items ("decide what a
live player carries"). Authorized 2026-09-03 by Jason ("do the carry fix
and the horizon census, your way buddy"). **Status: COMPLETE.** Changes
confined to `solver/unified.rs` (additive fields, one reordering inside
tier (c)), one line in the UP0 gate driver, the transcript probe, a new
gate file `walt/walt/tests/solver_unified_carry.rs` (5 gates, 67 s), and
the re-run transcript `walt/probes/factor_belief/unified_run2.txt`.

**EXPLORATORY tier throughout.** Nothing here is a play-strength claim;
the live default player is untouched; arena and defaults stay on Jason's
word.

---

## THE CHANGE, IN ONE PARAGRAPH

UP0 advanced every open model-belief line at every ply. Its transcript
measured what that cost: on the lean rung 99.4% of the wall was
classifying acting-seat supports for a posterior no tier ever read. Gate
UP3 had already proved the posterior is a derived view of (root, public
line) and nothing else, which makes eager advancing a stored second copy
of a fact the line already holds. The player now RECORDS the line at
every ply — one `(seat, tile)` push per open line — and MATERIALIZES the
posterior only when a tier reads it, advancing it incrementally from
wherever it last stood to the head of the line. Consulted every ply, the
bill is UP0's; consulted never, it is zero. Every consultation the
lineage makes is charged to the decision that caused it, as the new
`Spend::carry_reads`, and the ledger total of a seat's lineage equals the
sum of its decisions' charges (gate UC2).

## THE NUMBERS

The committed transcript, re-run under the lazy carry
(`unified_run2.txt`, same corpus, same three rungs, same declared
library), against UP0's `unified_run1.txt`:

| rung  | UP0 deciding | UP0 carrying | UP1a deciding | UP1a recording | carry reads charged |
|-------|-------------:|-------------:|--------------:|---------------:|--------------------:|
| lean  |    12,117 µs | 2,105,672 µs |     11,605 µs |           0 µs |                   0 |
| ample | 20,001,864 µs |   904,569 µs | 21,559,803 µs |           2 µs |              17,724 |
| model |  6,712,759 µs | 1,616,137 µs |  8,077,029 µs |           6 µs |              12,164 |

Two readings.

**The lean rung's carry is gone.** 2.1 s of classification for a
posterior nothing read became 0 µs of recording; the rung's 72 decisions
cost 11.6 ms all in. The posterior is still carried — as the line — and a
final materialization after the hand still recovers every falsification
UP0 saw (gate UC3), so nothing was lost; it was deferred, and never
collected because nothing asked.

**On the rungs that read the posterior, the bill moved rather than
shrank.** The ample and model rungs' materialization now happens inside
`decide` at the consulting decisions, where it is charged as carry reads
(17,724 and 12,164 consultations). The deciding wall grew by roughly what
the carrying wall lost; the residual difference is wall noise — this run
shared the machine with the gate suites, and wall is the only number in
this report that should be read as approximate. Reads are exact and are
the measurement that matters.

**Nothing else moved.** 27 join readings, 9 value moves, 2 argmax flips —
identical to UP0's. Gate UC5 makes that a law rather than an observation:
an eager driver (materialize every line after every ply, from outside the
player) and the lazy player produce identical actions, evidence,
refusals, frames and join readings at all 216 decisions, and identical
posterior notes at every decision that consulted the posterior. Only the
carry charge moves.

**Falsifications are discovered when read, at the ply they happened.** Of
the transcript's nine library falsifications, two were discovered during
play by a consulting decision and seven at the final materialization
after the hand — the lean rung never learns its library was falsified,
and its provenance says so by reporting `materialized=k/n`. Gate UC3
checks the discovery against an independent eager replay: same history,
same seat, same tile, same supported set, materialized exactly through
the falsifying play.

## WHAT WAS BUILT

- `SeatLine` gains `line: Vec<(usize, Domino)>` and `materialized:
  usize`, plus a `Rc<ReadLedger>` handle so the lineage's ledger stays
  readable after the belief is retired (a falsifying classification is a
  real read and stays charged — the first version of the slice lost it,
  and gate UC2 caught that). New accessors: `line_plays`,
  `materialized`, `is_current`, `ledger_total`.
- `UnifiedPlayer::observe_play` records; `UnifiedPlayer::materialize_line`
  (new, public) folds recorded plays into the belief, idempotently.
- Tier (c)'s free structural checks — line exists, fiber cap, read cap —
  now come BEFORE materialization, so a budget that does not afford the
  tier refuses before a single classification is spent. The first version
  materialized first; gate UC1 caught the 2,178-read charge that
  produced on the lean rung. A read cap of zero is a typed
  `MixtureRefused` at zero spend.
- `PosteriorNote` gains `line_plays` and `materialized`; `Spend` gains
  `carry_reads` (included in `total()`).
- The transcript probe prints `carry=` reads, `materialized=k/n`, and
  marks each falsification as discovered during play or at the final
  materialization. The `carry` wall column is now `record`.

## THE GATES (`solver_unified_carry.rs`, 5, 67 s)

- **UC1** nothing read, nothing paid: lean rung, whole corpus — every
  decision charges zero carry, every line ends with ledger zero and
  nothing materialized, and the deferred bill is real (materializing
  afterwards spends reads on at least seven lines).
- **UC2** conservation: on the ample and model rungs, every seat's ledger
  total equals the sum over its decisions of `carry_reads +
  mixture_reads`; checked on lines that actually spent.
- **UC3** falsification at materialization: on h3-t5 and h8-t4, a lean
  walk learns nothing during play; the final materialization agrees with
  an independent eager replay on history, seat, tile and supported set,
  and stops exactly after the falsifying play.
- **UC4** idempotence and currency: after `materialize_line` the line is
  current (or retired), a second call spends nothing, and every decision
  whose note says the posterior was consulted reports it materialized to
  the head of the line.
- **UC5** lazy ≡ eager on every answer (above).

UP0's 18 gates stay green with ONE driver edit (`walk()` calls
`materialize_line` before reading a line's derived views — recorded here
as a deliberate gate change, not a weakening: the views UP3 checks are
the same views, read after the line is brought current).

## DEVIATIONS FROM THE PLAN, RECORDED

1. The ledger handle. Planned as "charge the ledger delta around
   materialization"; the ledger lived on the belief, which is dropped at
   falsification, so the falsifying read vanished from the charge and
   from the end-of-walk view. Fixed by holding the ledger on the line.
2. The tier (c) ordering. Planned as "materialize when tier (c) reads";
   built first as "materialize on entering tier (c)", which charged the
   lean rung. Fixed by putting the free checks first.
3. Provenance semantics changed, and the change is stated: `falsified`
   and `live_profiles` on a decision now reflect the posterior AS OF its
   materialized point, not as of the ply. `materialized`/`line_plays`
   make the difference legible on every row.

## FLAGS

- **The lean rung still opens lines it never reads.** The empty-library
  lever (UP0's UP4) remains the only way to spend nothing at all; with a
  library declared, the line is recorded for free and stays available.
  That is the right default: a later tier, a join reading, or a post-hoc
  audit can materialize it, and until then it costs a vector push.
- **The (b)-versus-(c) ordering and the trick-start proof-state boundary
  are untouched.** UP1's other two items; the specimens UP0 pinned still
  stand.

---

EXPLORATORY — below every evidentiary tier; quotable only via gate
receipts. Wall times are one machine, one run, shared with other work.
