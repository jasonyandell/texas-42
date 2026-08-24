# Shadow probe — §22 step 7: the controller beside the live player

**EXPLORATORY INSTRUMENT OUTPUT** — sits below every evidentiary tier and
is cited by nothing above it. Every number here is an estimate or a plain
count over instrument records, never a receipt; nothing in this directory
is a P-A21 statement, and no strength claim is made or implied (§20.16:
this is measurement, not a default change — the old player remains the
default; arena and conformance gates remain the bar for any change).

Parent: `walt/math/calculated_evidence_v0.1.md` §22 step 7 (adjudicated
CE-A1..A8, `walt/CENSUS-RULINGS.md`). Producer: `walt/walt/src/bin/shadow.rs`
at the commit that adds this README.

## What one record is

The shadow bin drives hands in the playout shape — the focal (bidder)
seat plays the library's `level1_evaluate` at the live 200/8 defaults (its
choice is always the played line), the other seats play the banked-correct
level-0 policy on their true hands — and at every focal decision with more
than one legal tile it ALSO runs the §16.4 decision controller on one
frozen level-1 continuation policy per legal root action
(`ActionRule::PinnedThenLevel1`, discovery worlds from the policy's own
domain-separated stream, declared inner schedule 8/2), under a run-scoped
risk plan: δ_run = 1/100 per hand, δ_d = δ_run/(d(d+1)) for the d-th
decision. Declared routing: fiber ≤ 2000 runs the exact frozen-set
endpoint directly (`route:"preroute"`; exact results spend no risk);
larger fibers run the adaptive controller (world cap 128 — a resource
limit producing honest `Unresolved`, never a settlement rule) with the
§11.3 escalation rule armed at 1:1 declared cost weights. Agreement
between the live choice and the controller winner is recorded, never
acted on. Every record carries the complete freeze-tuple identity
(inner-approximation visibility, §18 Phase-1 fence) and integer
wall-microseconds for the live and shadow evaluations.

Phase-1 fence, stated plainly: every candidate is model-relative to
sampled inner minds — level-1 continuation at declared 8/2 over a
level-0 field at declared n0 = 2 — so no record's winner is quotable
without its schedule. The exact routes are exact **for that frozen
candidate set under that declared field model**, nothing more. Where the
fiber was small enough that the exact endpoint ran, that record's exact
reference is its truth anchor.

## Files

- `receipt.jsonl` — the 13 frozen `verify_player` receipt hands
  (deterministic deal anchor; the bidder seat is driven by the live
  player, NOT the receipt's recorded line).
- `driven.jsonl` — 20 driven hands of playout's scenario (trump fives,
  P30 by T1, S1 the bidder holding the receipt-hand-8 tiles) from fixed
  seeds.
- `summarize.py` — stdlib-only aggregator that reproduces the counts
  below from the JSONL.

## Reproduction

From `walt/` (hands run in parallel; records are byte-deterministic
except the two `micros` fields, which are wall-clock measurements):

```
cargo build --release -p walt --bin shadow
./target/release/shadow receipt probes/shadow/receipt.jsonl
./target/release/shadow driven  probes/shadow/driven.jsonl 20
python3 probes/shadow/summarize.py probes/shadow/receipt.jsonl probes/shadow/driven.jsonl
```

Defaults baked into those commands (all positional knobs after the fixed
args, in order): `n_outer_live=200 n0_live=8 n_outer_frozen=8 n0_frozen=2
world_cap=128 exact_cap=2000`.

## Aggregate (plain counts; regenerate with summarize.py)

Combined (33 hands: 13 receipt + 20 driven), 2026-08-24, defaults above:

- decisions shadowed: **183** (70 receipt, 113 driven)
- result kinds: **ExactFrozenSet 67, Unresolved 116, DeltaSettled 0,
  EpsilonEquivalent 0** (ε-mode was not configured for this run)
- routes: preroute-exact 67, sampled 116; **controller §11.3 escalations
  fired: 0** (see the reading notes below)
- per-trick kinds: tricks 1–3 all Unresolved (33/33/29); trick 4 mixed
  (7 exact, 21 unresolved); tricks 5–6 all exact (30/30); trick 7 offers
  no multi-option focal decisions in these hands
- decisions with a controller winner: **27**, all from the exact route;
  live choice = controller winner in **23/27** (receipt 7/10, driven
  16/17). The 4 disagreements sit at fibers 28, 60, 1750 (receipt hands
  4, 7, 11) and one driven root — exact-for-the-frozen-set references
  where the live 200/8 estimate picked a different tile.
- exact ties among ExactFrozenSet: **40** of 67 (short-horizon roots
  where several pinned continuations win identical world counts — the
  path reports the tie instead of index-breaking)
- decisions left open at the cap: **116**, and the live player's choice
  was among the survivors in **116/116** — the controller never
  eliminated the live line at δ_run = 1/100
- settlement indices: no DeltaSettled occurred, so the settlement-index
  distribution is empty at world_cap 128 (T_edge = m(m−1)·100·d(d+1) is
  large at these m and d; the §8.5 refinement vectors in each Unresolved
  record carry the exact h±_min distances)
- wall-microsecond medians (integer): live-eval **≈ 0.21 s** median
  (max 1022 s — one trick-1 refinement cascade), shadow-eval
  **≈ 10.3 s** median (max 770 s at trick-1 roots)

Timing caveat: hands execute in parallel across cores (and the level-1
machinery is itself rayon-parallel), so the per-decision microsecond
fields include scheduler contention; medians are instrument-grade
orientation numbers only.

## Reading the records honestly

- `Unresolved` is a successful output (§1.5): the resource cap arrived
  before any stopping condition, and the record persists the §8.5
  refinement vectors saying where the next unit of compute would go.
- `ExactFrozenSet` with `winner:null` is an exact tie among the frozen
  candidates — honest, not a defect; index tie-breaking is exactly what
  this path refuses to do.
- Disagreements between the live choice and a controller winner conflate
  three things by design and settle none of them: outer sampling noise in
  the live player, the frozen candidates' smaller declared inner schedule
  (8/2 vs the live 200/8), and the declared evaluation field (level-0 at
  n0 = 2). Separating those is step 8's calibration work (V5, per-pair
  E0), not this instrument's claim.
- The controller's own §11.3 escalation almost never fires under honest
  1:1 cost weights (its h±-based lower bound is deliberately
  conservative); small fibers reach exactness through the declared
  pre-route instead. Both routes are recorded per record.
