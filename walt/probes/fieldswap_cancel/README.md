# Field-swap cancellation probe — slice 3: the ladder, pairwise masses, directional rungs

**EXPLORATORY INSTRUMENT OUTPUT** — sits below every evidentiary tier and
is cited by nothing above it. Every number here is a plain count or exact
rational over instrument records, never a receipt; no strength claim is
made or implied. The load-bearing gates are the tests
(`walt/walt/tests/solver_fieldswap_cancel.rs`); this directory is the
instrument view of the same machinery at the probe epoch's declared pair.
**[L2 thread]** (pairwise masses are CE-pairwise objects; the dominance
route is objective-level — PANEL-A7's thread labels).

Mathematical source: Part VI of the x:019–023 panel response
(`exchange/inbox/019-023-response-panel-and-cancellation-v0.1.md`
§§31–42), adopted by rulings **PANEL-A7/A8** (`walt/CENSUS-RULINGS.md`,
"The panel-response adjudication (2026-08-24)"); intake companion
`walt/math/response_walt_panel_and_cancellation_v0.1_intake.md`; card
`kanban/doing/slice3-cancellation-ladder.md`.
Producer: `walt/walt/src/bin/fieldswap_cancel.rs` at the commit that adds
this README, over `solver::exposure` (directional rungs, sampled E3,
exact-root producer) and `solver::field_swap` (ladder, pairwise masses,
six-label vocabulary, directional screen, Stage 4, Λ, traces).

**The interpretation rule (response §42; binding on every record here):**
cancellation may justify a value statement under one declared objective,
belief, and model — never pathwise safety, structural irrelevance,
dominance, or reweighting stability.

## The declared (σ0, σ1) epoch pair

The slice-2 probe epoch's, unchanged: **σ0** = `Level0 { n0 = 8 }`,
**σ1** = `Level1 { n_outer = 4, n0 = 2 }`, frozen focal candidates at
declared schedule `[8, 2]` (`ActionRule::PinnedThenLevel1`, one per legal
root action). Declared ε for the `EpsilonEquivalent` label: **1/20** (a
probe-epoch declaration, carried on every root record). Sampled-E3
stream prefix: worlds 0..64 at epoch 0. Both FieldIds ride every record;
a different schedule is a different FieldId and a different experiment.
(The tests declare the separate cheap pair `Level0{2}` / `Level1{2,2}`,
frozen `[2,2]`.)

## What one root's records are

1. **Ladders** (response §31) — per pinned candidate, the exact-fiber
   `(d, r, c⁺, c⁻, c)` with `|c| ≤ r ≤ d` asserted, the six-label
   classification at the declared ε, and the §9.1 pivotal evidence on
   the correction counts. Split aggregates (§10.1): first-split seat and
   trick histograms plus the conditional outcome difference.
2. **Pairwise masses** (§33–§34) — every ordered pair under each field:
   exact-fiber `(B, H, q, g)` (census asserted; `g` equals the frozen
   value gap by construction) and the dominance label (`Dominated` only
   via `H = 0 ∧ B > 0`, exact enumeration).
3. **Pair lifts** (§3.3, §32) — `Λ = c_a − c_b` with `|Λ| ≤ d_a + d_b`
   asserted. At default knobs the committed h8-t4 check value
   **Λ(pin-5-5, pin-3-3) = 31/1200** is asserted, not just printed
   (corrected 2026-08-24 from a 41/1200 mis-addition by the response
   §32; components were always right).
4. **Directional rungs** (§35, §38; PANEL-A8) — per action, the coupled
   fused solve's `(R⁺)^U, (R⁻)^U, (R^outcome)^U` beside exact E4, with
   the extended ladder `R± ≤ R^outcome ≤ R^exposure` and
   `R^exposure = R_a(E4)` asserted; the frozen-tier sandwich
   `V0 − (R⁻)^U ≤ V1 ≤ V0 + (R⁺)^U` asserted per action.
5. **Screens** — symmetric (E4 bounds) and directional (§37) at the
   exact-frozen-set tier (directional admissible ⊆ symmetric asserted;
   excluded actions strictly σ1-nonoptimal asserted), the §36
   winner-stability table (positive directional slack ⇒ σ1 order,
   asserted), and the directional screen at the **ExactRoot** tier over
   the exact optimized `Q⁰` values.
6. **Stage 4** (§8) — σ1 work confined to the survivors; settled-vs-
   selected comparison typed (`FieldDecisionChanged` exactly when the
   choice moved).
7. **Sampled E3** — the split-reach objective on the declared stream
   prefix, an estimate record that feeds nothing (see Deferred).
8. **ExactRoot** — exact optimized `Q⁰_a, Q¹_a` per action (the §35
   sandwich asserted on exact optimized values; optimizer ≥ its frozen
   candidate asserted).

## Files

- `cancel.jsonl` — all records above (141 lines, three roots).
- `summarize.py` — stdlib-only aggregator: recomputes every inequality
  and set from the raw counts with exact fractions; `ALL 386 CHECKS
  PASS` on the committed records.
- `run.log` — the producing run's stderr.

## Reproduction

From `walt/` (roots run in parallel; records are byte-deterministic
except the `micros` fields):

```
cargo build --release -p walt --bin fieldswap_cancel
./target/release/fieldswap_cancel run probes/fieldswap_cancel/cancel.jsonl
python3 probes/fieldswap_cancel/summarize.py probes/fieldswap_cancel/cancel.jsonl
```

Knobs (positional, after the output path): `n0_field0=8 n_outer_field1=4
n0_field1=2 n_outer_frozen=8 n0_frozen=2 stream_worlds=64`.

## Aggregate (2026-08-24, defaults above; regenerate with summarize.py)

| root | action | V⁰ | V¹ | d | r | c | label | (R⁺)^U | (R⁻)^U | R_a (E4) |
|---|---|---|---|---|---|---|---|---|---|---|
| h7-t5 (1680) | 1-0, 5-3, 5-5 | 0 | 0 | **0** | 0 | 0 | **NoFieldExposure** | 0 | 0 | 0 |
| h8-t4 (1200) | 2-1 | 1103/1200 | 71/80 | 1087/1200 | 1/10 | −19/600 | EpsilonEquivalent | 427/1200 | 241/600 | 14/15 |
| | 3-1 | 1049/1200 | 1019/1200 | 93/100 | 77/600 | −1/40 | EpsilonEquivalent | 149/400 | 541/1200 | 577/600 |
| | 3-3 | 367/400 | 179/200 | 1117/1200 | 39/400 | −9/400 | EpsilonEquivalent | 97/300 | 49/120 | 39/40 |
| | 5-5 | 547/600 | 183/200 | 569/600 | 7/150 | 1/300 | EpsilonEquivalent | 17/48 | 67/150 | 197/200 |
| h4-t6 (90) | 0-0 | 14/45 | 17/45 | 4/15 | 1/15 | +1/15 | Unresolved | 1/15 | **0** | 4/15 |
| | 1-1 | 13/15 | 13/15 | 2/45 | **0** | 0 | **OutcomeStable** | 0 | 0 | 2/45 |

Reading the three scales on h8-t4 (the response §32 shape, now per
action): the fields act differently on ~90–93% of worlds (`d`), change
the Boolean outcome on 5–13% (`r`), and the net corrections are within
1/20 (`c`; hence `EpsilonEquivalent` at the declared ε — a value
statement under this objective/belief/model, nothing more).

Highlights, honestly framed:

- **The corrected Λ reproduced from scratch:** Λ(pin-5-5, pin-3-3) on
  h8-t4 = c(5-5) − c(3-3) = 1/300 − (−9/400) = **31/1200** — the bin
  asserts it at default knobs; the committed-prose correction trail is
  `walt/probes/fieldswap/README.md` and `wiki/walt-calculated-evidence.md`.
- **Directional rungs are dramatically tighter than E4 on the h8-t4
  regime** (the PANEL-A8 motivation): (R±)^U ≈ 0.33–0.45 against
  R_a ≈ 0.93–0.99. Not yet tight enough to prune there — the frozen
  values sit within ~1/20 of each other, so the screen still reports
  `FieldSensitive` 4/4 — but the bound mass fell by ~2.3×.
- **First `FieldDecisionChanged` in the wild** (Stage 4, h8-t4): the
  σ0-settled 2-1 (V⁰ = 1103/1200) is not the σ1-best survivor — 5-5
  wins under σ1 (183/200 vs 71/80). A frozen-candidate-set statement at
  the declared pair, not a play-strength claim.
- **h4-t6 sharpens to the ExactRoot tier:** the exact optimized values
  coincide with the frozen ones here, the directional screen at
  ExactRoot tier reports **FieldStableExactRoot 1/2** — the first
  exact-root stability result (§15.3's missing tier, now produced) —
  and 1-1's label is `OutcomeStable` (r = 0 with d = 2/45 > 0: the
  fields act differently on 4 worlds and never change the outcome).
  Directional slack S⃗(1-1, 0-0) = 22/45 vs the symmetric 11/45.
- **First `Dominated` in the wild** (h4-t6): pin-1-1 strictly dominates
  pin-0-0 under both fields (`H = 0 ∧ B > 0` by exact enumeration —
  σ0: B = 50/90; σ1: B = 44/90). The §34 distinction observed: this is
  one-sided unforced risk, a different object from the h8-t4 rows'
  near-cancellation, and the vocabulary keeps them apart.
- **h7-t5 is `NoFieldExposure` per candidate** (d = 0 exactly), the
  ladder's zero-scale-1 — consistent with slice 2's root-action E0.
- **Sampled E3 tracks but proves nothing:** e.g. 3-3 and 5-5 read 1 on
  the 64-world prefix while exact R_a is 39/40 and 197/200 — the
  estimate tier's records carry `tier:"estimate"` and cannot enter any
  screen by type.

Cost (integer microseconds in the records; instrument-grade): h8-t4
baselines 31.0 s, ladders+rungs+E3 24.5 s (the coupled directional
branches to decided terminals are the §38 "costlier than split reach"
item), pairwise 0.14 s and exact-root 0.09 s (warm caches); h7-t5
0.10 s / 0.10 s / 0.06 s / 0.001 s; h4-t6 all under 0.08 s.

## Deferred LOUDLY (build-list items not landed in this slice)

1. **The δ-valid admissible-upper E3 producer.** §7.4 admits a sampled
   route into the screen only through an exact value, an admissible
   upper bound, or a valid structural over-approximation. The sampled
   E3 built here is the ESTIMATE sibling only — typed so it cannot feed
   screening. Constructing a δ-valid upper bound on a supremum needs
   evidence-engine design input (a sup is not a mean) and is deferred.
2. **The dominance valid-bound route.** PANEL-A7 admits `Dominated` via
   "exact enumeration or a valid bound"; only the exact-enumeration
   route has a producer. A structural hazard-bound type without a
   producer would invite misuse, so none was stubbed.
3. **Motif tags (§10 item 14).** `FieldSplitTrace` carries items 1–13
   — with one precision: item 11 (the public observation that
   distinguishes the relevant branches) has no dedicated field and is
   only implicit in (tile0, tile1, history); a motif design pass may
   want it made explicit. The structural motif vocabulary (e.g.
   "reveal-response") needs that design pass and is absent, not
   approximated. Aggregates ship seat/trick histograms and the
   conditional outcome difference only.

Nothing else on the PANEL-A7/A8 adoption list was capped: directional
rungs, extended ladder, sandwich, winner stability, directional
screening, all-five fixed-policy components, all-four pairwise
components, the six labels, the sampled-zero dominance lock, sampled E3
typing, Λ processes, trace aggregation, Stage 4, and the ExactRoot tier
all landed with gates.

## Reading the records honestly

- Everything is model-relative to the declared sampled inner minds and
  the declared `[8,2]` frozen candidates; nothing here says σ1 is a
  better mind (O36).
- The directional (R±)^U are valid UPPER bounds via cross-branch fusion
  in the safe direction only (the E2 caveat's direction) — never claimed
  exact, never playable policies, and never lower witnesses. The
  sandwich and winner-stability consequences hold a fortiori.
- `Dominated` appears exactly where §34 predicts its shape: on h4-t6,
  pin-1-1 strictly dominates pin-0-0 under BOTH fields (σ0: B = 50/90,
  H = 0; σ1: B = 44/90, H = 0) by exact enumeration — one-sided
  unforced risk, never "cancellation". The mirrored pairs (B = 0,
  H > 0) correctly read `Unresolved`: being dominated is not a label
  this vocabulary assigns. No h8-t4 or h7-t5 pair is dominated.
- Three roots, one declared pair: orientation for §17's success/
  falsifier watch, not a settlement.
