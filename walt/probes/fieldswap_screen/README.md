# Field-swap screen probe — §21 steps 6–8: rungs E0–E2, exact split reach, the admissible set

**EXPLORATORY INSTRUMENT OUTPUT** — sits below every evidentiary tier and
is cited by nothing above it. Every number here is a plain count or exact
rational over instrument records, never a receipt; no strength claim is
made or implied. The load-bearing O32/O38 parity gates are the tests
(`walt/walt/tests/solver_fieldswap_screen.rs`); this directory is the
instrument view of the same machinery at the probe epoch's declared pair.

Parent: `walt/math/targeted_level2_field_stability_v0.1.md` §5, §7, §8
Stages 1–3, §12.1–12.2 (adjudicated L2-A1..A7, `walt/CENSUS-RULINGS.md`;
obligations O31/O32/O34/O38, `walt/SCENARIO-PLAYER.md` §10).
Producer: `walt/walt/src/bin/fieldswap_screen.rs` at the commit that adds
this README, over `solver::exposure` (the rung producers) and
`solver::field_swap` (the admissible screen) with baselines from
`solver::controller`'s cold exact endpoint.

## The declared (σ0, σ1) epoch pair

Slice 1 left the σ0 inner schedule an open question; it is resolved BY
DECLARATION here: **one (σ0, σ1) pair per experiment epoch**, and this
probe epoch declares

- **σ0** = `Level0 { n0 = 8 }` (the banked-correct level-0 modeled mind,
  the same field the step-7 shadow bin drives non-focal seats with);
- **σ1** = `Level1 { n_outer = 4, n0 = 2 }` (the level-1 machinery per
  non-focal seat, seeded per state under `FIELD_DOMAIN`);
- frozen focal candidates at declared schedule `[8, 2]`,
  `ActionRule::PinnedThenLevel1`, one candidate per legal root action.

This is the same pair slice 1's smoke declared. The σ0 inner schedule n0
is a FieldId identity component; both FieldIds ride every JSONL record. A
different schedule is a different FieldId and a different experiment
epoch. (The tests declare a separate cheap pair — `Level0{2}` /
`Level1{2,2}`, frozen `[2,2]` — likewise carried by its FieldIds.)

## What one root's records are

For each root (the three exact parity roots: receipt-h7-t5 fiber 1680,
receipt-h8-t4 fiber 1200, receipt-h4-t6 fiber 90):

1. **Stage 1** — exact frozen-set baseline under σ0: `V_0(ρ_a)` for every
   legal root action's pinned candidate, one enumeration pass through
   `solver::controller` (tier `exact-frozen-set`; the screen's statements
   are about this frozen candidate set, never the optimized root).
2. **Stage 2** — the rung ladder per action, every bound rung-labeled:
   - E1 counted structural covers (trivial ≡ 1; forced-non-focal, sound
     because a seat holding ≤ 1 tile is forced everywhere);
   - E0/E2 from one shared pre-split reach walk (free focal branching —
     safe-direction strategy fusion — over the complete fiber; E0 fires
     exactly when NO reachable non-focal state disagrees, proving
     `R_a = 0`);
   - E4, the exact split-reach solve: the §7.4 hit-frontier objective
     maximized over information-consistent continuations (per-node
     action choice on the public-history tree — no strategy fusion,
     O34); its optimal value IS `R_a`, exactly.
3. **Stage 3** — the admissible screen twice: at the cheapest sound
   bounds (E0 where it fires, else E2) and at the exact E4 bounds.
   `L^(1) = L^(0) − R^U`, `U^(1) = U^(0) + R^U`, bar `B̄ = max L^(1)`,
   `𝓐₁ = {a : U^(1) ≥ B̄}` (L2-T4), the full ordered-pair slack table
   `S_{a,b} = L_a^(0) − U_b^(0) − R_a^U − R_b^U` (§12.2 routing).
4. **Parity** — exact `V_1(ρ_a)` under σ1 for the SAME frozen candidates
   (the naive full-σ1 pass), with the exclusion audit (every excluded
   action strictly σ1-nonoptimal in the frozen set) and L2-T2
   (`|Q^(1) − Q^(0)| ≤ R_a`) asserted with exact numbers.
5. **Cost** (§12.1) — integer microseconds for baseline, rungs, and the
   σ1 all-action pass.

A sampled lower witness is never an upper bound (§7.4, O34): every bound
in these records is exhaustive over its declared domain, and the
fixed-policy tier (`FrozenPolicyExposure`, slice 1's probe) cannot enter
the screen by type (L2-A4/O31).

## Files

- `screen.jsonl` — root/baseline/exposure/screen/parity/cost records.
- `summarize.py` — stdlib-only aggregator: recomputes every screen from
  the baseline and exposure records with exact fractions, re-verifies
  bar/admissible/slack, replays the L2-T2 and exclusion audits, prints
  the table below.

## Reproduction

From `walt/` (roots run in parallel; records are byte-deterministic
except the `micros` fields):

```
cargo build --release -p walt --bin fieldswap_screen
./target/release/fieldswap_screen run probes/fieldswap_screen/screen.jsonl
python3 probes/fieldswap_screen/summarize.py probes/fieldswap_screen/screen.jsonl
```

Knobs (positional, after the output path): `n0_field0=8 n_outer_field1=4
n0_field1=2 n_outer_frozen=8 n0_frozen=2`.

## Aggregate (2026-08-24, defaults above; regenerate with summarize.py)

| root | action | Q⁽⁰⁾ | Q⁽¹⁾ | R_a exact (E4) | E2 cover | E0 | screen (E4 bounds) |
|---|---|---|---|---|---|---|---|
| receipt-h7-t5 (fiber 1680) | 1-0 | 0 | 0 | **0** | 0 | **fires** | admitted |
| | 5-3 | 0 | 0 | **0** | 0 | **fires** | admitted |
| | 5-5 | 0 | 0 | **0** | 0 | **fires** | admitted |
| receipt-h8-t4 (fiber 1200) | 2-1 | 1103/1200 | 71/80 | 14/15 | 1127/1200 | — | admitted |
| | 3-1 | 1049/1200 | 1019/1200 | 577/600 | 1159/1200 | — | admitted |
| | 3-3 | 367/400 | 179/200 | 39/40 | 1181/1200 | — | admitted |
| | 5-5 | 547/600 | 183/200 | 197/200 | 1 | — | admitted |
| receipt-h4-t6 (fiber 90) | 0-0 | 14/45 | 17/45 | 4/15 | 4/15 | — | **excluded** |
| | 1-1 | 13/15 | 13/15 | 2/45 | 2/45 | — | admitted |

Screen results (identical at cheapest and E4 bounds on all three roots):

- **receipt-h4-t6: `FieldStableExactFrozenSet`, admissible 1/2** — the
  first singleton in the wild. 1-1's lower field-1 bound (37/45) clears
  0-0's upper field-1 bound (26/45); slack S(1-1, 0-0) = 11/45 > 0. The
  σ1 parity pass confirms: V₁(0-0) = 17/45 < V₁(1-1) = 13/15. The
  excluded action consumed no σ1 optimization.
- **receipt-h7-t5: rung E0 FIRES at the root-action level for all three
  legal actions** — slice 1's d = 0 specimen upgraded from a two-policy
  observation to a proof over ALL information-consistent continuations:
  the two declared fields choose the same tile at every reachable
  non-focal information state, so R_a = 0 exactly and the level-1 upgrade
  cannot move this root's frozen-set values at all. The screen still
  reports `FieldSensitive` 3/3: all three candidates tie exactly at
  V₀ = 0 (with R = 0 the admissible set is exactly the σ0-argmax set) —
  an honest exact tie, not field sensitivity in the value sense.
- **receipt-h8-t4: `FieldSensitive`, admissible 4/4** — exposure bounds
  near 1 (R_a exact between 14/15 and 197/200; E2 covers larger still)
  degenerate the screen to the naive survivor set, exactly the parent's
  §8.1 expectation for a trick-4 root whose field disagreement is
  reachable almost everywhere. The screen prunes nothing here and is
  honest about it; every excluded-action guarantee is vacuously
  maintained, and the parity audit (L2-T2, exclusion soundness) still
  passes with exact numbers.

## Targeted-vs-naive cost note (§12.1; instrument-grade microseconds)

| root | C₀ baseline σ0 | ΣC_R rungs | σ1 all-action (naive share) | 𝓐₁/legal |
|---|---|---|---|---|
| receipt-h7-t5 | 0.05 s | 0.13 s | 0.008 s | 3/3 |
| receipt-h8-t4 | 4.7 s | 10.9 s | 17.9 s | 4/4 |
| receipt-h4-t6 | 0.008 s | 0.06 s | 0.003 s | 1/2 |

Reading honestly: on h7-t5 the rung walk is nearly free (the walk never
splits, so the whole free-branching tree stays one merged field line) and
E0's proof is the cheapest possible screen input. On h8-t4 the rungs cost
about half the naive σ1 pass and prune nothing — at split-heavy roots the
screen does not yet earn its keep, exactly the §17.2 falsifier direction
to keep watching. On h4-t6 the screen prunes half the survivor set, but
at fiber 90 the naive σ1 pass (0.003 s) is far cheaper than the rungs
(0.06 s) — at tiny fibers the screen's value is the typed stability
result, not saved compute; §12.1's economy question only becomes real at
fibers where C₁(a) dominates. Three roots at one declared pair —
orientation, not a conclusion.

## Reading the records honestly

- Everything is model-relative to the declared sampled inner minds and
  the declared `[8,2]` frozen candidates. A different schedule is a
  different FieldId/PolicyId and a different experiment. Nothing here
  says σ1 is a better mind (O36).
- Baseline tier is `exact-frozen-set`: every screen statement is about
  the named frozen candidate set. It is NOT an exact field-stable root
  (§15.3); the ExactRoot tier needs the exact root optimizer, a later
  slice.
- `R_a exact` is the §7.4 optimum over information-consistent
  continuations, solved exactly on the enumerated fiber (rung E4). The
  E2 column is the clairvoyant cover — always ≥ R_a, labeled, never a
  playable policy.
- The h8-t4 admissible sets did not shrink, and the h7-t5 result is a
  three-way exact tie at value 0. Only h4-t6 produced a pruning
  singleton. Small corpus, one epoch pair: these feed §17's success/
  falsifier watch, they do not settle it.
