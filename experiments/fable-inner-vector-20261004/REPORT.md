# Compressing Walt's inner level-0 vector at native resolution: first measured result

**Goal, per Jason (`CLAUDE.md`, received after this campaign ran):** the same
information position in, the full numerical vector the pinned lower-level Walt
computation returns out, at its native k/8 granularity, at lower computational
cost, to enable the ladder. Ties and action regret are diagnostics. The frozen
plan, data, weights and test below were preregistered before that steering
arrived and are not relabeled; this report's headline and assessment are
rewritten against the stated goal, the evidence is unchanged.

## Assessment against the native-compression goal

**Numerical fidelity to the exact production vector (test, 1,521 roots, legal
slots only, acting-side units, teacher output step = 0.125):**

| Arm | RMSE vs exact k/8 | in output steps | RMSE vs 128-world diagnostic mean | Mean pred / mean target (scale bias) | Pooled ECE |
|---|---:|---:|---:|---:|---:|
| inner-h32 (28,540 params) | .221 | 1.8 | .189 | .685 / .689 (−.004) | .014 |
| inner-h256 | .219 | 1.8 | .184 | .692 / .689 (+.003) | .026 |
| inner-d256 | .217 | 1.7 | .182 | .684 / .689 (−.005) | .026 |
| ref-h32 (trained on the diagnostic mean) | .213 | 1.7 | .181 | .686 / .689 | .016 |
| v0-h32 / v0-h256 (wrong boundary) | .301 / .306 | 2.4 / 2.5 | .278 / .282 | .520 / .517 vs .689 (−.17) | .170 / .172 |
| inner-feat (diagnostic encoding) | .171 | 1.4 | .124 | .688 / .689 | .009 |

Readings, computed evidence only:
- The raw inner-trained nets reproduce the **scale** of the native vector
  (bias under .005) and sit about **1.8 native steps** from the exact k/8
  values. The production seed is deterministic from the information position;
  alternative-seed variability does not establish an irreducible error floor
  for approximating that exact function. Against the smoother 128-world
  mean the raw nets are 1.5 steps off; the feature-encoding diagnostic is 1.0.
- Pooled ECE under .03 says the *averages* are on scale; it does not say an
  individual vector is within a step of the native one. Per-vector error
  distributions (fraction of legal slots within one step, worst slot per root)
  were **not computed** in this campaign and would be the right next metric.
- The wrong boundary (V0) is visibly wrong at native resolution: 2.4 steps off
  and biased by −.17. The corrected boundary is necessary; it is not yet
  sufficient for a faithful substitute.

**Cost, the limiting fact.** Measured on this host: tiny network-only median
5.33 µs; validated Python decision (encode + forward + legal argmax) 116.9 µs;
Rust seam in-process about 22 µs median (36.2 µs over the JSON pipe). **These
numbers do not demonstrate cheaper end-to-end compression.** The only path that
could be cheaper than the native 8-world recursion is a compiled in-recursion
input/forward/output path with the encoding fused, and that path is unmeasured.
The nested substitution test (net inside the outer recursion, paired outer
regret) is also undone. Everything about enabling the ladder is therefore
extrapolation at this point: a 28k-parameter dense forward is plausibly a few
microseconds in Rust, which would be below the 22 µs seam only if encoding and
masking cost nearly nothing, and that has to be built and measured, not assumed.

**Target.** Jason's stated native target is the exact production k/8 vector.
The 16-bundle mean is a declared diagnostic (noise reference) and `ref-h32` an
optional distinct experiment; neither is proposed as the target.

## Answer (decision-level diagnostics, preregistered)

**Exploratory local experiment, 2026-10-04.** Claude Fable 5.1 (`claude-fable-5-1`),
same session throughout. Authorized by Jason ("yes please try that and start
training again"). All prior experiments immutable; everything here is new under
`experiments/fable-inner-vector-20261004/`. Not a strength claim, not a theorem,
not a status change at any tier.

## Answer

The pinned production Walt's **inner level-0 modeled mind** was extracted as a
vector-returning seam without modifying production code, and verified
differentially: on 1,024 independent positions plus every one of the 21,273
labeled roots, the seam's `best_of(action_values)` equals the production
fast-path `modeled_choice(0)`, and the production-feature build and the
reference-path build give byte-identical vectors. Nets trained on this
teacher's full legal vectors, with a separate legal argmax selector, were
tested once on 512 fresh deals (1,521 roots, all four seats) after freezing:

- **Teacher boundary (the hypothesis).** On the same 128-world inner-type
  reference, the inner-trained regular-sized net beats its V0-trained twin by
  .0062 regret, paired 95% [−.0111, −.0014], resolved; the tiny pair differs by
  .0047 [−.0101, +.0003], not resolved. V0-trained nets are also miscalibrated
  as evaluators of this level (mean prediction .52 against .69): rankings
  partly transfer, values do not, exactly the message-24 caveat.
- **The estimator is sensitive to its sampled bundle.** The exact 8-world inner call agrees
  with its own 16-bundle majority on 74% of roots (87% on resolved gaps), has
  regret .0216 against the 16-bundle mean, and ties at the top on 60% of roots.
  A net trained on these native labels can score *below* the single exact call
  on the reference: the diagnostic feature net does (.0209 vs .0216). Nets
  land in the exact teacher's tie set 71 to 77% of the time; the 128-world
  reference lands there on 82%. That comparison is not a fidelity ceiling.
- **Capacity and loss, not established.** 256-wide one- and two-layer raw nets
  are numerically better than the tiny net (.0382/.0370 vs .0406) with
  intervals spanning zero; the late band reverses the order. Centered
  advantages (the pilots' loss) vs absolute BCE: −.0026 [−.0081, +.0030] on
  decisions, null; only the BCE arms yield calibrated evaluator numbers
  (ECE .014 to .026). Representation again separates: the diagnostic feature
  scorer wins by .0197 [+.0153, +.0244].
- **Not done:** the nested substitution test (net inside the outer recursion,
  paired outer regret). Nothing here certifies the outer consumer.

## Teacher: verified contract (see STATUS.md for the full table)

`WALT-INNER-L0MIND-n8-DICE-VOIDLESS-FIXED-BID30-STRAIGHT-DECLMAKE-v1`: the generic
tail of `Solver::pi(0, …)` in pinned `walt/walt/src/solver/mod.rs` (`cb1ef3b2`,
identical at HEAD). Deterministic function of (seat, remaining hand, public key):
RNG `INNER_SEED ^ mix(seat) ^ mix(hand) ^ record_hash(key)`; 8 worlds by the
**Voidless** shuffle (ignores public voids: a superset of the lawful support,
information-lawful, not a posterior); one dice seed per world; `Field::Dice`
others (seeded uniform legal per world and node); own future committed per
public node over the alive bundle (bundle-coupled, not fused); `action_values`
over the legal set as exact `k/8` rationals of **declarer-make**; `best_of`:
declarers maximize, defenders minimize, lowest tile on exact ties. The request
seed is unused by this level. Production discards the vector after selecting;
the seam keeps it. Adapter: [adapter/src/lib.rs](adapter/src/lib.rs); builds
under 19 s each ([receipts](results/)); differential: [seam-check.json](results/seam-check.json).

Declared, non-production estimators: the **reference** mixes an extra seed term
into that RNG and averages 16 bundles (128 worlds); the **V0 control** is the
pilots' kernel (byte-identical to the ladder pilot), 128 common worlds and
tapes, all-future uniform, acting-partnership outcome.

Net orientation: targets are acting-side success (`p_make` for declarers,
`1 − p_make` for defenders), recovered exactly for display; selector is the
legal argmax of the acting-side score, lowest tile on ties. The two are
equivalent to the teacher's max/min rule.

## Frozen design ([plan.json](plan.json), written before any label)

7,168 fresh deals (zero collisions against 29,188 prior partitions including
every phone-game deal): blocks 0 to 2 train (6,144 deals, 18,236 roots), block 3
validation (512 deals, 1,516 roots), block 4 test (512 deals, 1,521 roots) mined,
deduplicated and labeled only after the freeze at 18:13:56Z (earliest test
receipt 18:14:19Z; audit). Eight uniform-play histories per deal, one live
nonforced root per ply band, actor = seat on turn (train seats 4,935/4,936/
4,958/4,923). Roots disjoint from every prior experiment's information sets.

Labels per root: exact inner vector and choice (with the production check on
every call), 16 reference bundles, V0-128 packed outcomes. Raw 862 encoding
primary; 32 lawful features diagnostic. Eight arms, matched 6,000 updates,
Adam .002, batch 256, selection by each arm's own validation loss on a 10-update
grid:

| Arm | Arch | Target | Loss | Params | Selected update |
|---|---|---|---|---:|---:|
| inner-h32 | 862→32→28 | exact k/8 | soft BCE | 28,540 | 1,210 |
| inner-h256 | 862→256→28 | exact k/8 | soft BCE | 228,124 | 720 |
| inner-d256 | 862→256→256→28 | exact k/8 | soft BCE | 293,916 | 350 |
| inner-h32-centered | 862→32→28 | exact k/8 | pilots' centered MSE | 28,540 | 140 |
| v0-h32 / v0-h256 | as above | V0 k/128 | soft BCE | | 1,430 / 460 |
| ref-h32 | 862→32→28 | 16-bundle mean | soft BCE | 28,540 | 1,710 |
| inner-feat (diagnostic) | 32→64→64→1 per action | exact k/8 | soft BCE | 6,337 | 4,780 |

Soft-target BCE is a proper scoring rule here, not an exact likelihood for
bundle-coupled counts. No softmax anywhere; illegal tiles masked.

## Fresh test, all bands (1,521 roots, 512 deals; 205 resolved-gap roots)

Regret is against the 128-world reference mean in acting-side make units.
"Tie set" = chosen tile among the exact teacher's top-count tiles; "exact" =
same tile as the production call; "resolved" = tie-set rate on resolved gaps.

| Rule | Regret [95%] | Tie set | Exact | Resolved | RMSE vs k/8 | RMSE vs ref | ECE |
|---|---:|---:|---:|---:|---:|---:|---:|
| Uniform random | .0657 [.0613, .0705] | | | | | | |
| Highest tile | .0621 | .646 | .149 | .532 | | | |
| **Exact inner teacher (production tile)** | .0216 [.0193, .0239] | 1 | 1 | 1 | | | |
| 16-bundle majority | .0109 | .781 | .740 | .946 | | | |
| 128-world reference argmax | 0 | .819 | .632 | .951 | | | |
| inner-h32 | .0406 [.0361, .0453] | .707 | .388 | .683 | .221 | .189 | .014 |
| inner-h256 | .0382 [.0341, .0426] | .719 | .401 | .673 | .219 | .184 | .026 |
| inner-d256 | .0370 [.0327, .0418] | .723 | .413 | .683 | .217 | .182 | .026 |
| inner-h32-centered | .0432 [.0375, .0491] | .712 | .396 | .580 | n/a | n/a | n/a |
| v0-h32 | .0453 [.0405, .0505] | .695 | .361 | .683 | .301 | .278 | .170 |
| v0-h256 | .0444 [.0397, .0497] | .706 | .364 | .702 | .306 | .282 | .172 |
| ref-h32 | .0379 [.0338, .0422] | .728 | .399 | .712 | .213 | .181 | .016 |
| inner-feat (diagnostic) | **.0209** [.0182, .0237] | .766 | .456 | .854 | .171 | .124 | .009 |

RMSE against exact deterministic k/8 is the primary native-compression error.
RMSE against the alternative-seed 128-world mean measures a different, smoother
target. Neither the bundle-sensitive choice comparison nor a binomial variance
heuristic apportions the observed .22 error into sampling and approximation
components. Paired differences (positive = first worse):

| Comparison | All bands | Late band (497 roots) |
|---|---:|---:|
| inner-h32 − v0-h32 | −.0047 [−.0101, +.0003] | −.0006 [−.0088, +.0076] |
| inner-h256 − v0-h256 | **−.0062 [−.0111, −.0014]** | +.0023 [−.0072, +.0111] |
| inner-h32 − inner-h32-centered | −.0026 [−.0081, +.0030] | −.0100 [−.0234, +.0024] |
| inner-h32 − inner-h256 | +.0024 [−.0022, +.0070] | −.0047 [−.0141, +.0047] |
| inner-h32 − inner-d256 | +.0036 [−.0006, +.0079] | −.0023 [−.0117, +.0070] |
| inner-h32 − ref-h32 | +.0027 [−.0006, +.0061] | −.0006 [−.0073, +.0058] |
| inner-h32 − inner-feat | **+.0197 [+.0153, +.0244]** | **+.0193 [+.0109, +.0285]** |

Late band ([test-late](results/test-late/summary.json)): teacher noise is
lower there (exact vs majority 79%, 92% resolved), the raw nets cluster at
.043 to .047 with every pairwise interval spanning zero, and the feature net
reaches .0232 (teacher .0189).

## Reading

1. **The boundary matters for values more than for rankings.** V0 nets rank
   nearly as well as inner nets on the inner reference (tie-set .70 vs .71)
   but their probabilities are off by .17 on average. If the consumer only
   takes the tile, V0 was a tolerable stand-in; if any consumer reads the
   numbers, it was the wrong teacher. The resolved paired gain at 256 width
   says the inner labels do carry extra decision signal at ordinary capacity.
2. **The 8-world inner mind is a noisy estimator of itself.** Its single-bundle
   choice is in its own 16-bundle majority only 74% of the time, and V0's
   128-world argmax scores better against the inner expectation (.019) than the
   inner single call does (.022). A distilled net is an average over positions
   and can beat the single call it was trained on; the feature net already
   does. Whether that helps the outer player is the undone G3 question: the
   outer currently consumes a noisy but *seed-consistent* inner tile, and a
   smoother substitute changes the outer's statistics in ways only the paired
   outer test can measure.
3. **Capacity is not the lever on raw inputs, again,** but it is also not ruled
   out: the 256-wide nets are numerically better on all bands and worse late,
   intervals spanning zero both ways. **Loss form** did not change decisions;
   it changes what the output means (BCE arms are calibrated, ECE ≤ .026).
   **Seed averaging as a target** (ref-h32) was numerically best among tiny raw
   arms and not resolved against the exact-label arm.
4. **Representation** remains the strongest tested factor on this teacher too
   (feature scorer −.0197, resolved, and tie-set .766 close to the .78 to .82
   ceilings). It is a diagnostic, not a mandate; it says the raw encoding is
   sample-inefficient for this evaluator as it was for V0.

## Costs, size, latency

| Item | Observed |
|---|---:|
| Seam builds (production features / reference paths) | 18.6 s / 18.1 s under cap |
| Labels, 21,273 roots: exact + 16 reference bundles | 14.9 summed seam-seconds (about 22 µs per in-process call; 36 µs over the JSON pipe) |
| V0-128 control labels | 10.3 summed seconds |
| Mining 7,168 deals | about 70 s across capped blocks (≤ 16.8 s each) |
| Training per arm | 2.8 to 6.6 s loop, 6,000 updates, MLX Metal |
| inner-h32 / h256 / d256 / feat | 114 KB / 912 KB / 1.18 MB / 25 KB raw; network-only 5.3 / 7.1 / 9.8 / 9.6 µs; validated decision 117 / 116 / 121 / 166 µs |
| Receipts | 37, all completed, largest 18.6 s, every allowance ≤ 295 s |

A net replacing the inner call must beat the seam's about 22 µs in-process
cost to be worth it on speed. The measured paths (5.33 µs network-only, 116.9 µs
validated Python decision, 36.2 µs seam over the JSON pipe) do not show that;
a compiled forward with the encoding fused is the real comparison, not run here.

## Audit and limitations

[checks/audit.py](checks/audit.py) ([result](checks/audit-result.json)) re-verified
under its own capped receipt: source and information-set disjointness against
all prior experiments and across splits; all receipts within caps; all eight
frozen hashes; freeze preceding every test receipt; 42 sampled test roots
replayed byte-identically through both seam builds with production agreement
and identical 16-seed reference counts; referee legality and seat-parity
perspective for all 1,521 test roots; one headline mean recomputed. Same-agent
independent code paths, not an independent reviewer.

Limitations: teacher-relative fidelity to a named model estimator under a
Voidless belief, not posterior, not strength; the reference is the inner-type
estimator, which favors inner-trained arms on calibration by construction (the
decision comparison is on a common scale but the V0 nets were trained toward a
different objective); the 8-world granularity makes exact-tile agreement a
weak metric (ties on 60% of roots); alternative-seed variation does not prove
an error floor for the deterministic native teacher; many comparisons, no multiplicity
correction; G3 (outer substitution) and a Rust/C in-recursion forward path are
not done; the standalone C port was not used and is not claimed equivalent to
the phone.

## Evidence, extrapolation, next engineering step

- **Computed:** the seam reproduces production exactly; inner-trained raw nets
  match the native scale and sit 1.7 to 1.8 steps from exact k/8 (1.5 from the
  128-world mean); V0-trained nets are off-scale; decision diagnostics as
  tabled; latencies as tabled; one initialization per arm; six nested domains
  only (bid 30, straight pip trumps, uniform-play positions, Voidless inner
  belief, 8 inner worlds, this host).
- **Extrapolation, not evidence:** that a compiled forward would undercut the
  22 µs native inner call; that per-vector errors are tolerable for the outer
  recursion; that the feature encoding's gain transfers to a fused compiled
  path.
- **Next engineering step:** a Rust (or C) in-recursion forward for the frozen
  `inner-h32` and `inner-h256` weights with fused encoding and legal masking,
  measured against the seam on the same positions; then G3, the nested
  substitution on identical outer worlds against the pinned baseline, measuring
  outer vector error and paired outer regret against a high-budget outer
  reference. Per-vector step-error distributions should be added to the
  evaluation before that run. None of this was started here.
