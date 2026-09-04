# FH3-REPORT — the report of record and the FH8 anchors, as measured

**Slice:** FH3, the parent's §38 report of record, §37 FH8 anchors and
§39–§41 success/falsifier verdict (`walt/math/focal_horizon_sandwich_v0.1.md`,
cited by title; the object is the FOCAL-HORIZON HIERARCHY, FH-A2) as
narrowed by the companion and the rulings FH-A1..A11 — the anchors'
coordinates and "what the soundness laws already say" per FH-A8
(`walt/CENSUS-RULINGS.md`) — under `walt/briefs/BRIEF-FH3.md`. Authorized
2026-09-04 by Jason. **Status: COMPLETE.** New `focalreport report` mode
(`src/bin/focalreport.rs`), record `walt/probes/factor_belief/focal_run1.txt`,
four gates in `walt/walt/tests/solver_focal_anchors.rs` (~85 s release
standalone), this report, the ledger paragraph and README entries. No
solver code changed; FH1's and FH2's gates untouched and green. Freeze
58 untouched; `ingest/` untouched; the math parent, verifier and
companion untouched.

**EXPLORATORY tier throughout.** Every number below is exploratory; the
anchors' answers were DISCOVERED by running, never pinned (FH-A8); the
live default player is untouched; no arena claim (FH-A10). **No §41
correctness failure fired anywhere** — every coordinate's containment,
nesting, survivor and verdict laws passed inside the record itself and
again in the gate.

---

## What this means and what it cost

walt's focal-horizon hierarchy now has its report of record, and the
three anchors the parent named as stress tests are answered by
measurement. On the whole trick-4 corpus — fourteen live (root, contract)
coordinates across five contracts — the hierarchy settles EVERY one by
k ≤ 2: five at k = 0 (no search at all: the σ0 tail beats every other
action's God upper), six more at k = 1 (one focal layer exact, certified
regret at most 45‰ anywhere), and the last three at k = 2, which is the
exact solve at trick 4 (FH-last). The two ply-cut flips the record found
at h8-t4 (bids 36 and 39) are not flips of the hierarchy: at k = 1 the
verdict stays honestly `Unresolved` — exactly what FH-A8's law said it
must, since the cut's action 5-5 keeps an upper of 757‰ over 2-1's exact
750‰ — and at k = 2 it settles 2-1; neither cut argmax (3-3 at cut 4,
5-5 at cut 8) is ever certified, here or at any of the 33 coordinates.
The contract-sensitive h4-t4 specimen settles 6-5 at k = 0 at four of
five contracts and needs k = 1 only at bid 39, where the tail is worth
651‰ against 4-0's 655‰ upper — the tail-quality question of FH-A8
answered by four per mille. The real test, the exact trick-3 root
h8-t3, does NOT settle at k = 2: the survivor set shrinks 5 → 5 → 3 and
Γ falls 141‰ → 100‰ → 34‰, with the lower policy already playing the
exact 1-1 from k = 1 on but uncertified; k = 3 — the FH-last collapse,
run because k = 2 came in under the brief's ten-minute gate — settles
1-1 at `28859/29988`, the record's fourteen-minute exact value, cited
and reproduced. What it cost, up front as the finding it is: **memory is
the cost axis that grew again**. The ladder's fact store at h8-t3 holds
3.82M facts and the record run peaks at 19.4 GB RSS (the standalone
h8-t3 ladder 17.1 GB; five concurrent h4-t4 coordinates 7.8 GB; the
anchors gate 17.8 GB with five h4-t4 ladders in flight) against FH2's
662 MB at h3-t4. Reads per horizon are unchanged from FH1/FH2 (the
engine was not touched): h8-t3 costs 27.2M reads at k = 0, 73.5M at
k = 1, 69.7M at k = 2 and 20.0M at k = 3 with reuse — 190M reads for the
exact answer against the record's 289M for `response_success_mass`, and
the direct engine without reuse pays 27.2M and 79.2M for k = 0 and 1.
Wall: the record runs in 23 min 55 s (1,435 s) (corpus on six workers ≈ 4 min; h8-t3
alone: direct k = 0 163 s, k = 1 460 s; ladder passes 161 s /
276 s / 113 s / 8.7 s (the k = 3 pass rides the ladder's own warm instance and its 3.68M collapsed receipts)); the gate adds 85 s standalone and 200 s in-gate, and **`check.sh` grew from 230 s to 308 s wall** (sum of suite walls 1,367 s → 2,126 s; the FH1 suite beside it slowed 141 s → 231 s under the contention). One wall
finding: a ladder pass on a σ0 instance the direct engine already
warmed runs 15× faster at identical reads (h8-t3 k = 0: 154 s cold, 10 s
warm), so the record gives every direct run a fresh field instance and
the ladder its own — the cross-horizon cache reuse the MAP said was
"measured 0" across histories is large within one coordinate.

---

## THE QUESTION

FH1 built the interval, FH2 made it anytime. Neither answered the
parent's own experiment: on the corpus, and at the three PR #87 anchors
where a ply cut had picked the wrong action, what is the smallest
horizon that settles, or ties exactly, or what does the `Γ_k` ladder look
like when neither happens — and does the hierarchy ever select an action
it cannot certify? FH-A8 fixed the coordinates and stated what the
soundness laws already imply (k = 1 cannot settle 2-1 at (ii); k = 0
cannot settle 1-1 at (i); at (iii) bid 39 k = 0 settles iff the tail is
worth more than 655‰ after 6-5; every trick-4 anchor is exact at k = 2).
Everything else was to be discovered.

## WHAT WAS BUILT

- **`focalreport report <out.txt> [h<hand>-t<trick> ...]`** — the report
  of record. Corpus: T4 {h3, h4, h8, h12}-t4 × contracts {receipt, 33,
  36, 39, 42}; T56 {h8-t5, h3-t5, h12-t6, h10-t6, h5-t6, h4-t6} ×
  {receipt, 36} (a contract equal to the receipt's is deduplicated);
  then anchor (i) h8-t3 alone. Per coordinate, for k ∈ {0, 1, 2, 3}: the
  DIRECT engine (`focal_horizon`, reads without reuse, FH1-comparable,
  each run on a fresh `FieldModel`) and ONE ladder walked sequentially
  with the suffix memo on (reads with reuse, one field instance, cold at
  k = 0). Per action: `L_{a,k}`, `U_{a,k}`, `U − L`, `Δ^L_{a,k}`,
  `Δ^U_{a,k}`, the survivor mark, `Q_a` and the split `(U − Q) + (Q −
  L)`. Per horizon: bar, survivors, verdict, `π_k` id (both ids where the
  ladder's differs on a lower-side tie), `L_exec`, `U*`, `Γ_k`, the lower
  policy's root action, the direct spend (field/tail reads,
  conditionings, focal/hidden nodes, tail evaluations, forced), the
  ladder's spend (reads, hits/lookups, receipts, facts, collapsed,
  new/revisited/tightened, refused frontier count/mass), wall. Then the
  exact column (`Q_a`, `Q*`, exact argmax, its wall and source), the
  changes by horizon (π_k's root play at each k and where it changed;
  survivors and Γ by k; first `Settled`; first `Equivalent`), the ply
  cut's argmax under Proposition FH-cut on the viewer-lead trick-3/4
  roots (cut-4 = argmax `U_{a,0}`, cut-8 = argmax `U_{a,1}`, flagged
  when not an exact maximizer, and whether ANY horizon certified it),
  the completed focal depth with `h_f` after each action (the
  independent walk at trick 4–6; FH-A6's law `7 − T` cited at h8-t3,
  where the walk is an exact-size pass), the first suffix hit, and the
  affordability decision for k = 3 at h8-t3. `Q_a` is a fresh
  `response_success_mass` everywhere except h8-t3, where the record's
  values (`horizon_run1.txt`, 14 min) are cited and asserted against
  this root's legal set and `Z`. The §41 laws are asserted per
  coordinate inside the mode: containment, no lower falls, no upper
  rises, survivors shrink, Γ never rises, ladder ≡ direct in every value
  at every shared k, `Settled ⇒` unique exact argmax, `Equivalent ⇒` the
  exact tie set, `Unresolved ⊇` the maximizers — a failure panics naming
  the coordinate. Corpus coordinates run on six workers (walls
  contended; reads exact); the anchor runs alone after them; the file is
  re-flushed after every coordinate. Two summary tables close the record:
  one row per (root, contract, k), and the anchors' verdict/Γ/reads by k.
- **Gate file `solver_focal_anchors.rs`** (below).
- No solver module was touched.

## THE GATES (`solver_focal_anchors.rs`, 4, ~85 s release wall standalone, 1,120 s CPU, 17.8 GB peak RSS)

One `LazyLock` fixture: at (ii) h8-t4 × {36, 39} and (iii) h4-t4 × {30,
33, 36, 39, 42} the direct engine at k = 0, 1, 2; `Q_a` by
`response_success_mass` (independent); the live `horizon_census` at cut
8 and cut 4 for (ii), cut 4 for (iii); the memo-on ladder walked k = 0,
1, 2. Nothing pins WHICH k settles or whether a cut flips.

- **FHA1** sandwich, nesting, containment: `L_{a,k} ≤ Q_a ≤ U_{a,k}` at
  every action and k; no lower falls and no upper rises with k; bar, `U*`
  and `Γ` monotone; survivors only shrink; at k = 2 every action is
  collapsed to `Q_a` with every tail consultation forced (FH-last).
- **FHA2** `Settled ⇒ exact argmax`: a `Settled{b}` names the unique
  exact maximizer and `Q* > U_{a,k}` for every other `a`; an `Equivalent`
  lists exactly the maximizers at `Q*`; an `Unresolved` survivor set
  contains every maximizer. FH-A8's law in conditional form: wherever
  some other action's `U_{a,k} ≥ Q_b`, the verdict is not `Settled{b}`.
- **FHA3** the ply-cut comparison on the same (root, contract): the
  census's per-action cut readings equal `U_{a,0}` (cut 4) and `U_{a,1}`
  (cut 8) — FH-cut, so the cut's argmax IS `argmax_a U_{a,m−1}`; its
  exact readings equal the independent `Q_a` and its exact argmax the
  first exact maximizer; the flip flag equals "cut argmax not exact";
  and wherever it is wrong, no horizon's verdict is `Settled` on it and
  no `Equivalent` set contains it — the ladder never selects the ply
  cut's wrong action.
- **FHA4** reuse parity: the sequential memo-on ladder's derived view at
  each k equals the direct engine's on every value — intervals, bar,
  survivors, verdict, `U*`, `Γ`, the lower policy's root action.

**Anchor (i) is probe-only** (the brief's "measure, then decide"): before
the gate was sized, the h8-t3 ladder measured k = 0 at 154 s / 27.2M
reads, k = 1 at 272 s, k = 2 at 116 s, 17.1 GB peak — no split of k ≤ 2
fits under ~3 minutes, and k = 0 alone is over half of it. Its numbers
live in the record and here.

## THE FINDINGS (`focal_run1.txt`, σ0 tail, node cap 40,000, nothing refused anywhere)

### Survivors by k, trick-4 corpus and the trick-3 anchor

| root | bid | k = 0 | k = 1 | k = 2 | k = 3 |
|---|---:|---|---|---|---|
| h3-t4 | 30 | 3-1 4-4 6-4 | **3-1** S | 3-1 | 3-1 |
| h3-t4 | 33 | 3-1 4-4 | **3-1** S | 3-1 | 3-1 |
| h3-t4 | 36 | 3-1 4-4 | **3-1** S | 3-1 | 3-1 |
| h3-t4 | 39 | 3-1 4-4 6-4 | **4-4** S | 4-4 | 4-4 |
| h3-t4 | 42 | **6-4** S | 6-4 | 6-4 | 6-4 |
| h4-t4 | 30 | **6-5** S | 6-5 | 6-5 | 6-5 |
| h4-t4 | 33 | **6-5** S | 6-5 | 6-5 | 6-5 |
| h4-t4 | 36 | **6-5** S | 6-5 | 6-5 | 6-5 |
| h4-t4 | 39 | 4-0 6-5 | **6-5** S | 6-5 | 6-5 |
| h4-t4 | 42 | **6-5** S | 6-5 | 6-5 | 6-5 |
| h8-t4 | 30 | 2-1 3-1 3-3 5-5 | 2-1 3-3 5-5 | **3-3** S | 3-3 |
| h8-t4 | 33 | 2-1 3-1 3-3 5-5 | **2-1** S | 2-1 | 2-1 |
| h8-t4 | 36 | 2-1 3-1 3-3 5-5 | 2-1 3-3 5-5 | **2-1** S | 2-1 |
| h8-t4 | 39 | 2-1 3-1 3-3 5-5 | 2-1 3-3 5-5 | **2-1** S | 2-1 |
| h8-t4 | 42 | decided at the root: `Equivalent` at 0 | | | |
| h12-t4 | all | decided at the root: `Equivalent` at 0 | | | |
| h8-t3 | 30 | 1-1 2-1 3-1 3-3 5-5 | 1-1 2-1 3-1 3-3 5-5 | 1-1 2-1 3-3 | **1-1** S |

S = the first `Settled`. T56: every coordinate is `Settled` at k = 0 or
`Equivalent` (an exact tie set) by k ≤ 1 (h8-t5 at 36 goes `Unresolved`
→ `Equivalent {0-0 5-0 5-3}` at k = 1); h_f = 2 at trick 5 and 1 at
trick 6 (FH-A6), intervals constant beyond.

### Γ_k by k (‰)

| root | bid | Γ_0 | Γ_1 | Γ_2 | Γ_3 |
|---|---:|---:|---:|---:|---:|
| h3-t4 | 30 / 33 / 36 / 39 / 42 | 76 / 62 / 48 / 37 / 11 | 12 / 13 / 7 / 8 / 3 | 0 | 0 |
| h4-t4 | 30 / 33 / 36 / 39 / 42 | 30 / 132 / 177 / 189 / 61 | 10 / 28 / 37 / 29 / 6 | 0 | 0 |
| h8-t4 | 30 / 33 / 36 / 39 | 100 / 170 / 225 / 226 | 39 / 45 / 43 / 43 | 0 | 0 |
| h8-t3 | 30 | 141 | 100 | 34 | 0 |

### The anchors' per-action intervals (‰), Δ^L = L_{k+1} − L_k, Δ^U = U_k − U_{k+1}

(ii) h8-t4 bid 36 (Z = 1,200; exact 2-1 750, 3-1 602, 3-3 723, 5-5 744):

| action | [L_0, U_0] | Δ^L_0 / Δ^U_0 | [L_1, U_1] | Δ^L_1 / Δ^U_1 | [L_2, U_2] |
|---|---|---|---|---|---|
| 2-1 | [626, 775] | 87 / 24 | [714, 751] | 35 / 1 | [750, 750] |
| 3-1 | [461, 636] | 83 / 33 | [545, 603] | 57 / 0 | [602, 602] |
| 3-3 | [559, 852] | 118 / 101 | [677, 750] | 45 / 27 | [723, 723] |
| 5-5 | [537, 846] | 170 / 89 | [707, **757**] | 36 / 13 | [744, 744] |

Bid 39 differs from bid 36 by at most 1‰ per entry (exact rationals in
the record). k = 1: bar 714 (2-1), `U* = 757` (5-5), Γ 43‰, `Unresolved {2-1 3-3
5-5}`; the cut-4 argmax is 3-3 (`U_0` 852) and the cut-8 argmax 5-5
(`U_1` 757 > 751): both flips, neither ever certified (FHA3, and the
record's per-coordinate line).

(iii) h4-t4 bid 39 (Z = 34,650; exact 2-1 551, 4-0 605, 5-1 556, 6-5 734):

| action | [L_0, U_0] | Δ^L_0 / Δ^U_0 | [L_1, U_1] | Δ^L_1 / Δ^U_1 | [L_2, U_2] |
|---|---|---|---|---|---|
| 2-1 | [500, 592] | 36 / 37 | [536, 555] | 15 / 3 | [551, 551] |
| 4-0 | [527, **655**] | 60 / 38 | [587, 616] | 17 / 11 | [605, 605] |
| 5-1 | [474, 591] | 62 / 32 | [537, 558] | 19 / 2 | [556, 556] |
| 6-5 | [**651**, 840] | 58 / 100 | [710, 739] | 24 / 4 | [734, 734] |

k = 0 is `Unresolved {4-0 6-5}` by 4‰ (bar 651 vs `U_{4-0,0}` 655); k = 1
`Settled 6-5` with Γ 29‰. At bids 30/33/36/42 the k = 0 bar beats every
other upper (964 vs ≤ 869 at 30). The cut-4 argmax is 6-5 at every
contract: the record's 13–105‰ over-pricing never moved the argmax here.

(i) h8-t3 bid 30 (Z = 59,976; exact, cited: 1-1 962, 2-1 928, 3-1 890, 3-3 955, 5-5 922):

| action | L_0 → L_1 → L_2 → L_3 | U_0 → U_1 → U_2 → U_3 | survives through |
|---|---|---|---|
| 1-1 | 837 → 876 → 928 → 962 | 992 → 977 → 963 → 962 | k = 3 (`Settled`) |
| 2-1 | 764 → 843 → 903 → 928 | 965 → 941 → 931 → 928 | k = 2 |
| 3-1 | 736 → 794 → 861 → 890 | 936 → 912 → 892 → 890 | k = 1 |
| 3-3 | 852 → 872 → 927 → 955 | 993 → 968 → 956 → 955 | k = 2 |
| 5-5 | 685 → 813 → 909 → 922 | 959 → 941 → 923 → 922 | k = 1 |

π_k plays 3-3 at k = 0 (the cut-4 argmax, by the LOWER side — the tail
after 3-3 is worth 852‰) and 1-1 from k = 1 on; the bar 852 → 876 → 928
→ 962. At k = 2 the width per action is 13–34‰ and the split is the
FH1 pattern again — `U − Q` 1–3‰, `Q − L` 13–34‰: the residual is the
tail's policy gap, not fusion price. `h_f = 4` after the root action
(FH-A6's law; not walked).

### Cost by k in reads (direct without reuse / ladder with reuse), facts and hits

| root | bid | k = 0 | k = 1 | k = 2 | k = 3 | facts at k = 3 |
|---|---:|---|---|---|---|---:|
| h3-t4 | 30 | 1.78M / 1.78M | 2.63M / 1.47M | 2.83M / 0.42M | 2.78M / 0 | 89,923 |
| h4-t4 | 30 | 5.13M / 5.13M | 8.61M / 5.20M | 10.2M / 1.62M | 9.99M / 0 | 321,284 |
| h4-t4 | 36 | 5.25M / 5.25M | 8.81M / 6.31M | 10.5M / 2.53M | 10.3M / 0 | 412,001 |
| h8-t4 | 36 | 0.27M / 0.27M | 0.49M / 0.38M | 0.56M / 0.15M | 0.55M / 0 | 21,827 |
| h8-t3 | 30 | 27.2M / 27.2M | 79.2M / 73.5M | — / 69.7M | — / 20.0M | 3,817,605 |

The h8-t3 ladder's suffix hits: 0 / 6,266 / 151,400 / 206,935; the
k = 3 pass is 20.0M reads because 3.68M of its facts were already
collapsed receipts. The trick-4 k = 3 passes cost 0 reads (every node
completed at k = 2 returns its fact; the four hits are the root
children). Every direct k = 3 run equals FH1's k = 2 field reads with
zero tail reads (FH-A6's "consultations = 0 at k = h_f", seen at every
trick-4 coordinate).

### The three-layer picture, in the hierarchy's vocabulary

On the trick-4 corpus the layers are horizons: k = 0 (σ0's tail below,
God's cut at the trick-5 frontier above) already settles 5 of 14 live
coordinates; k = 1 (the trick-5 decision exact, the trick-6 frontier
priced — U0b's cut-8 upper with a lawful lower under it) settles 11 of
14 with Γ ≤ 45‰ everywhere; k = 2 (the trick-6 decision exact, trick 7
forced) is the exact solve. At the trick-3 root the same three layers
sit one trick deeper and the picture is the same shape at larger
numbers: Γ 141‰ / 100‰ / 34‰ at k = 0/1/2 with the survivor set 5 / 5
/ 3, and only the collapse settles. The record's ply-cut flips live
entirely on the upper side: the cut's argmax is `argmax U_{a,m−1}`,
which the hierarchy carries as an UPPER and never as a verdict; the
lower policy `π_k` — a lawful, replayable policy — picks the exact
action from k = 1 at h8-t3 and from k = 0 at every trick-4 coordinate
except two: h3-t4 at bid 30 (π_0 plays 4-4, π_1 the exact 3-1) and
h8-t4 at bid 30 (π_0 3-3, π_1 2-1, π_2 3-3 — the FH1 specimen, where the
bar's argmax wanders while the verdict stays honest).

### The §39 / §40 verdict, in the parent's terms

§39 asks whether k ≤ 2 "frequently yields a singleton exact survivor,
exact tie set, accepted certified regret, large upper movement at
tractable cost, or reusable exact suffixes that materially shorten
earlier solves." On this corpus: singleton exact survivors at k ≤ 2 at
14 of 14 live trick-4 coordinates and exact tie sets at every decided
or trick-5/6 one; certified regret at k = 1 of 3–45‰ at trick 4 and
34‰ at k = 2 at trick 3; upper movement at k = 0 → 1 of 24–101‰ per
action at the h8-t4 anchor; suffix reuse cutting the k = 2 pass to
15–30% of the direct engine's reads at trick 4 and the k = 3 pass at
trick 3 to 20.0M reads. Against the honest reading of §40: every
sandwich gate holds, every lower re-prices as one lawful policy (FH5 /
PS2 in the FH1/FH2 suites; the ladder's `π_k` here equals the direct
engine's in value at every coordinate), uppers nest, and the record
localizes the remaining width to the tail's policy gap. The one place
k ≤ 2 does not settle is the trick-3 root, where it leaves three
survivors and Γ 34‰ — §40's "honest partial success" at exactly the
coordinate the parent flagged as the real test, with k = 3 (the
collapse) affordable at 190M reads with reuse.

## WALL

The record: 23 min 55 s (1,435 s) total (corpus on six workers, contended, ≈ 4 min;
h8-t3 alone ≈ 19.7 min: direct k = 0 163 s, k = 1 460 s, ladder
161 s / 276 s / 113 s / 8.7 s (the k = 3 pass rides the ladder's own warm instance and its 3.68M collapsed receipts), `Q_a` cited at 30 µs). The gate ~85 s
standalone (1,120 s CPU on 18 threads; the h4-t4 ladder walks are the
long jobs). Peak RSS: record 19.4 GB; h8-t3 ladder standalone 17.1
GB; gate 17.8 GB. Wall is the only approximate number; reads are exact
and in the record per run.

## DEVIATIONS AND BOUNDARIES

1. **Anchor (i) is probe-only** (measured, then decided — above). Its
   `Q_a` are the record's, never recomputed here (the brief), asserted
   only for shape (legal set, `Z`); the §41 containment check at h8-t3
   is therefore against `horizon_run1.txt`'s numbers, and it passed at
   every k, which is an independent reproduction of that exact value by
   the k = 3 collapse (`L = U = 28859/29988` at 1-1).
2. **The direct engine at h8-t3 stops at k = 1.** k = 2 direct would
   re-walk ~70M+ reads without reuse for a number the ladder already
   holds; the record says "not run at this horizon (ladder only)" and
   the reads column is the ladder's.
3. **Cold walls.** Each direct run and the exact pricing use a fresh
   `FieldModel`; the ladder's passes share one. The first record run
   used one instance per coordinate and its ladder walls were cache-warm
   by 15× at h8-t3 — replaced, and the header states the discipline.
   Reads were identical in both runs.
4. **The corpus runs in parallel** (six workers); walls there are
   contended and say so in the header. The anchor runs alone.
5. **`h_f` at h8-t3 is cited from FH-A6**, not walked (the walk is an
   exact-size pass). The trick-4/5/6 rows walk it.
6. **The ladder's `π_k` id differs from the direct engine's** at k ≥ 1
   on most coordinates (prior-wins-ties, FH2); the record prints both
   ids and the value equality is asserted. The direct engine's table is
   larger (it records every argmax on its DAG; the ladder's root view
   unions the children's stored tables).
7. **Table widths** in the record are fixed-format; the `Equivalent`
   verdicts at decided roots list the whole tie set.
8. **Memory** is reported from `/usr/bin/time -l` on the record and gate
   processes (the code measures reads, not bytes) — the numbers are
   this machine's, approximate like wall.
9. **Gate sizing inside `check.sh`**: the anchors suite is 85 s standalone
   and 17.8 GB peak; under the concurrent runner with nine binaries in
   flight its wall and memory contend with the other focal suites — the
   gate wall is reported in the closing section of this report.

## THE §41 ITEMS

None fired. Every coordinate's record line reads `containment ✓ |
nesting ✓ | survivors shrink ✓ | Γ never rose ✓ | ladder ≡ direct ✓ |
verdict vs exact argmax ✓`, and FHA1–FHA4 pass at the seven gated
coordinates.

## `check.sh`

`walt/ci/check.sh` PASS, foreground, 5 min 8 s wall: 123 binaries, 0
failed, sum of suite walls 2,126 s. The cost trend, stated as the finding
it is: the gate wall grew from CI1/FH2's 230 s to 308 s and the sum of
suite walls from 1,367 s to 2,126 s. The anchors suite is 200.6 s
in-gate (85 s standalone) and, running beside it, `solver_focal_horizon`
went from 141 s in-gate (CI1) to 230.7 s — the new fixture's eighteen
threads and 17.8 GB contend with the other focal suites under the
nine-at-a-time runner. `solver_focal_ladder` 32.6 s. Lean built (8,665
jobs). The corpus-trim card the MAP already carries now has a second
customer.

---

EXPLORATORY — below every evidentiary tier; quotable only via gate
receipts. The settling horizons are measurements on the declared corpus,
never theorems.
