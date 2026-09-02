# U0-REPORT — the God-gap census, as built

**Slice:** U0, assignment `walt/briefs/BRIEF-U0.md`. **Branch:**
`walt-u0`, base `161b019`, final commit `60c091e`. **Theory:**
`walt/math/salvation_complex_v0.1.md` §§4–9, §§36–40, §47–§48, §55,
under the governing companion
`walt/math/salvation_complex_v0.1_intake.md` and rulings SC-A1..A8
(`walt/CENSUS-RULINGS.md`). **Tier:** EXPLORATORY throughout — the
fusion horizon is a measurement on a declared corpus, never a theorem
(SC-A4).

This document is the slice's report of record: the canonical source for
the PR body, the ledger paragraph, and the independent audit.

## Status

Complete. Five commits, clean tree, not pushed. `walt/ci/check.sh`
PASS — cold rebuild, `fmt --check`, clippy `-D warnings
-D clippy::float_arithmetic`, the no-float greps and scanners, the full
workspace release suite, and the Lean trick-1 foundations with the
exact axiom audit. The U0 gate suite itself runs in 3.0 s.

| file | change |
|---|---|
| `walt/walt/src/solver/godgap.rs` | new, 916 lines |
| `walt/walt/tests/solver_godgap.rs` | new, 869 lines |
| `walt/walt/src/bin/godgapreport.rs` | new, 515 lines |
| `walt/probes/factor_belief/godgap_run1.txt` | new, 147 lines |
| `walt/walt/src/solver/mod.rs` | +1 (module registration) |
| `walt/FACTOR-BELIEF.md` | +82 (status line + U0 paragraph) |
| `walt/DISCREPANCIES.md` | +24 |

Untouched, verified by `git diff 161b019 HEAD`: `solver/doom.rs`
(§47), `solver/refine.rs` (freeze 58), `solver/model_belief.rs`
(MB0), `ingest/`.

## What was built

`solver/godgap.rs` computes the salvation parent's §8 three-part
failure decomposition per root-action coordinate, in exact rationals:

```text
1 − V(ρ) = d_phys + d_info + d_policy(ρ)
  d_phys      = β(D)      = |D| / Z      physical doom
  d_info      = U^God − Q                information price
  d_policy(ρ) = Q − V(ρ)                 policy gap
  U^God       = 1 − β(D)  = (Z − |D|)/Z
```

The God upper comes from `doom::doom_enumeration`'s per-world truth
where the fiber is enumerable — the tightest doom mass any sound doom
reasoning can certify — and from `doom::doom_census`'s certified
harvest otherwise. Exact `Q` is `factor_belief::response_success_mass`.
The executable incumbent is `factor_belief::extract_success_policy`,
re-priced through the INDEPENDENT fixed-policy evaluator
`factor_belief::viewer_success_mass` — a different recursion (a frozen
focal policy, not a max), which is what makes the §36 equality receipt
a cross-check rather than a restatement.

The four §48 result types are typed so that SC-A4 is structural rather
than conventional: `PositiveGodGap` holds its exact witness mass inside
the variant and cannot be constructed without it, while `GodUpper` and
`UnknownGodGap` are unit variants with no value field into which an
unmeasured gap could ever be written. The split between those two is
exactly whether the doom side produced anything — positive certified
doom leaves a real upper standing with an unknown gap beneath it; zero
certified doom leaves the vacuous upper 1 and no claim at all.

`GodGapProducer` installs through the open §49 registry. Installing a
God-tight coordinate's facts makes the closure itself show the
executable lower meeting the deterministic upper with `Γ = 0` — an
executable bar the upper-only doom store never had.

## The six gates (`walt/walt/tests/solver_godgap.rs`)

**G1 — `the_section_nine_table_is_re_derived_from_the_committed_record`**
(SC-A2, the binding gate). Parses the per-world truth column out of
`probes/factor_belief/doomreport_run1.txt` with a deliberately literal
parser, so a format drift fails the parse loudly rather than silently
yielding an empty table. Asserts exactly 14 rows. Then per coordinate:
the census walks the record's actions in the record's order; the fiber
matches; the upper's source is `PerWorldTruth`; the re-derived doom
truth equals the committed truth; **`d_info = 0` by exact rational
equality `Q = 1 − doomed/Z`**; the result type is `GodTightPolicy`;
`d_policy = 0`. It re-derives the class census's weaker harvest in the
same pass and asserts it never exceeds the truth. Closes on
`checked == 14` and an exact-vector assertion over the divergence list.

**G2 — `the_four_result_types_are_typed_by_what_was_actually_established`**
(SC-A4). Three parts. (a) h3-t5 at `exact_fiber_cap = 0`: zero
certified doom, vacuous upper, verdict `UnknownGodGap`, both absent
terms `None`, and `upper.fact()` returns nothing — a vacuous upper
installs nothing. (b) h12-t6 at the same cap: whole fiber doomed,
verdict `GodUpper`, upper exactly 0, `d_info` still `None`, fact
installable. (c) h8-t4 2-1 affordable: verdict `PositiveGodGap`,
`Φ > 0`, `Φ = U^God − Q` exactly, and the witness mass reproduced by an
independent `response_success_mass` call.

**G3 — `god_tight_policies_re_price_to_the_god_upper_under_a_bound_receipt`**
(§36). Over h5-t6, h4-t6, h8-t5: re-extracts the policy OUTSIDE the
census, asserts `policy.id()` equals the receipt's `policy_id`,
re-prices through `viewer_success_mass`, and asserts the value equals
the God upper. Then the §36 identity binding — `root_id`, `field_id`,
`contract`, `root_action`, `belief_id = "uniform-root"`, `utility_id`,
`fiber_mass`, `repriced_mass`, and `doomed_mass + repriced == Z`
("God-tightness is exactly: every saveable world saved"). Then installs
the coordinate's facts into a fresh `ProofState` and reads the equality
back off the closure: `view.lower == view.upper == U^God`, nothing
sampled. Closes on `receipts == 7`.

**G4 — `census_refusals_are_typed_and_nothing_is_silently_dropped`**
(§34/§35). h3-t5 under a cap of 0 and a 40-node/20-cap doom budget: one
coordinate per legal action (nothing dropped); every one carrying
`ExactValueUnaffordable` and `DoomTruthUnaffordable` by name with their
fiber and cap; both absent terms `None`; the result restricted to the
two honest variants; and `CensusLeftMassRefused` present somewhere.
Then the same root affordable: the exact-side refusals are gone and
both terms are `Some` — the refusal is a function of the declared
budget, not of chance.

**G5 — `the_census_consumes_doom_and_never_modifies_it`** (§47/SC-A3).
Per action on h5-t6: `doom_enumeration` before, the census,
`doom_enumeration` after, `assert_eq!(before, after)` — doom's
instrument is a pure function of its declared inputs, unchanged by the
census; and the God upper's doomed mass and value ARE doom's. Then both
producers into one store: `DoomCensusProducer` installs,
`GodGapProducer` installs beside it, the fact count strictly grows and
nothing is removed, every bound fact is `Deterministic` and `Upper`,
the closure gains an executable bar with `Γ = 0`, a second produce
proposes nothing new, and the §67.4 byte round trip holds.

**G6 — `the_fusion_horizon_table_counts_every_coordinate_and_names_its_exceptions`**
(§38). Over h8-t4, h12-t4, h5-t6: two strata in increasing depth order,
every coordinate counted once, per-stratum tallies partitioning, every
non-God-tight coordinate named in `exceptions`. Then the vacuity
discipline: t4 has 4 positive gaps and 4 God-tight, ALL FOUR of the
latter vacuous, so `substantively_fusion_free()` is false at t4; t6 has
0 vacuous and is substantively fusion-free; `earliest_fusion_free_trick`
returns 6 on this two-stratum slice.

## The fusion-horizon finding

Over 37 coordinates — the ten gated roots of the Slice F epoch plus the
h0-t1 opening root — **the earliest fusion-free depth on this corpus is
trick 5.**

```text
 trick | tested | God-tight (vacuous) | pos gap | GodUpper | Unknown | max Φ
-------+--------+---------------------+---------+----------+---------+-------
   t1  |     7  |       0 (  0)       |     0   |     0    |     7   | -
   t4  |    16  |       4 (  4)       |    12   |     0    |     0   | 43/1925 (22‰)
   t5  |     6  |       6 (  0)       |     0   |     0    |     0   | -
   t6  |     8  |       8 (  2)       |     0   |     0    |     0   | -
```

Exceptions are printed in full beneath the table: 7 at t1 (every
`h0-t1:<action>` is `UnknownGodGap`), 12 at t4 (all `PositiveGodGap`,
four each at h3-t4, h4-t4, h8-t4), none at t5 or t6. Two strata (t5,
t6) are SUBSTANTIVELY fusion-free.

The twelve measured information-consistency prices, exact:

| root | action | Φ | ‰ |
|---|---|---|---:|
| h3-t4 | 3-1 | 1/66 | 15 |
| h3-t4 | 4-1 | 79/11550 | 6 |
| h3-t4 | 4-4 | 43/1925 | 22 |
| h3-t4 | 6-4 | 23/1650 | 13 |
| h4-t4 | 2-1 | 47/4950 | 9 |
| h4-t4 | 4-0 | 376/17325 | 21 |
| h4-t4 | 5-1 | 17/1925 | 8 |
| h4-t4 | 6-5 | 157/11550 | 13 |
| h8-t4 | 2-1 | 1/150 | 6 |
| h8-t4 | 3-1 | 19/1200 | 15 |
| h8-t4 | 3-3 | 1/60 | 16 |
| h8-t4 | 5-5 | 1/50 | 20 |

**`d_policy = 0` at every one of the twelve** — the incumbent IS the
argmax, re-priced and receipted. So the remainder at trick 4 is neither
a bad policy nor physical doom. It is the price of not knowing, and no
amount of further counterexample counting can reach it.

A stratification caveat is printed above the table in the probe rather
than left implicit: §48 asks for trick, grade, contract, trump
structure, count state and field level, but on this corpus the last
four are constant (one declared field, contract 30 at every root, the
receipt's own trump per root, field level 0). Trick depth is the only
axis that varies; grade is the per-coordinate fiber mass in Part 1.

## Degenerate God-tightness, typed apart

Where the WHOLE fiber is physically doomed, `U^God = Q = 0` and
Theorem 7.1's common intersection is an intersection over an empty
index set: every lawful policy is God-tight, and the equality carries
no information about the price of blindness. These are real receipts
but degenerate evidence.

`GodTightPolicy::nothing_saveable()` flags them,
`FusionStratum::god_tight_vacuous` counts them, and
`substantively_fusion_free()` additionally requires at least one
God-tight coordinate with something left to save. This changes a
headline: without the distinction, h12-t4's four receipts would have
made trick 4 look partly fusion-free.

**Six of the eighteen extracted God-tight policies are of this kind**
(h12-t4's four, h12-t6's two), so twelve are substantive. The ledger
and the PR body should use the twelve.

## God-tight policies extracted and persisted

18 of 37 coordinates, 14 with a persisted 43-bin score profile. The
four h12-t4 receipts declined the profile against the declared
`profile_fiber_cap = 12_000` and say so with a typed
`ProfileUnaffordable` refusal rather than dropping it.

## The opening-root verdict

`UnknownGodGap` on all seven actions — the SC-A4 floor, and the honest
answer rather than a disappointment. The exact side is unaffordable
(fiber 399,072,960 against the declared cap) and the doom side
certifies zero, so the God upper is the vacuous 1 and NOTHING is
claimed about `d_info` or `d_policy` there.

What would change it, in increasing order of realism: an exact `Q` at
the opening (out of reach); any nonvacuous opening doom mass — which
§47 declines to chase and which the committed doom record argues is
near zero anyway (two hand-built crusher worlds and a 228-point stride
grid all let the world-aware viewer make); or, the route the
mathematics actually points at, an information-consistency-aware upper.
That is U1/U2 territory, and the census's own numbers say it is the
only door.

## Wall times (release, one run, 37 coordinates, 286.3 s total)

| root | fiber | coords | total | mean |
|---|---:|---:|---:|---:|
| h3-t4 | 11,550 | 4 | 8.78 s | 2.19 s |
| h4-t4 | 34,650 | 4 | 29.11 s | 7.28 s |
| h8-t4 | 1,200 | 4 | 2.79 s | 0.70 s |
| h12-t4 | 34,650 | 4 | 0.002 s | 0.5 ms |
| h8-t5 | 92 | 3 | 0.025 s | 8.5 ms |
| h3-t5 | 200 | 3 | 0.088 s | 29 ms |
| h12-t6 | 6 | 2 | 0.23 ms | 0.1 ms |
| h10-t6 | 19 | 2 | 1.4 ms | 0.7 ms |
| h5-t6 | 27 | 2 | 1.3 ms | 0.6 ms |
| h4-t6 | 90 | 2 | 5.5 ms | 2.8 ms |
| h0-t1 | 399,072,960 | 7 | 245.5 s | 35.1 s |

Two readings worth keeping. **86% of the census's wall went to
coordinates that returned `UnknownGodGap`** — the opening root's seven
refusals cost 245 s because the doom census still walks its priority
classes before the exact side declines. And **h12-t4 is free** (0.5 ms
on a 34,650-world fiber) because every world is already decided at the
root: the §17-dual zero-cost path surfacing in the God-gap census
exactly as it does in doom. The whole enumerable corpus outside h4-t4
costs under 12 seconds.

## The discrepancy G1 surfaced

Re-deriving the class census's harvest beside the per-world truth
turned up a correction to the intake companion. The committed record
carries THREE truth-vs-census divergences where the companion names
two: **h8-t5 0-0, where the class census certified 17 of 21** (809‰,
printed in the record's own recovery column), joins h4-t6 0-0 (56
against 60) and h8-t5 5-3 (0 against 1).

Nothing above it moves. The §9 table cites the TRUTH column at every
one of its fourteen coordinates, so every `d_info = 0` inference stands
exactly as adjudicated under SC-A1, and the class census's one-sided
soundness is unaffected (a certified harvest never exceeds the truth —
asserted per coordinate in the same gate). What is one short is the
companion's COUNT of divergence points. Recorded rather than repaired,
since the companion is a dated intake record; the full wording is in
`walt/DISCREPANCIES.md` under "Reconciled, not discrepancies", and G1
now asserts all three by exact value, so the number cannot drift again
without a red suite.

## Design decision: `godgap.rs`, not an extension of `doom.rs`

The brief allowed either. New sibling module, for three reasons in
order of weight.

1. §47/SC-A3 preserves the doom census AS the God-upper ground truth,
   and a byte-identical `doom.rs` is the cheapest possible proof of
   that — `git diff` against the base shows it untouched, and no
   argument is required.
2. The dependency runs one way only. The census consumes
   `doom_enumeration`/`doom_census` and additionally knows
   `extraction`, `factor_belief`, and `proof_state`, none of which doom
   knows about. Extending doom would have inverted that and dragged the
   exact recursions into a module whose entire value is being
   deterministic, δ-free, and cheap.
3. The new-core deletability rule (§67.10) survives: `godgap` imports
   its siblings and is imported by nothing but the crate root, so the
   group stays removable together.

## Flags

### For U1 (salvation masks)

Twelve ready specimens, all typed and reproducible. §40 asks
positive-gap coordinates to retain the earliest focal information state
at which worldwise successful completions become incompatible, plus
exact salvation masks, minimal joint glue sets, and first-split motifs.
**U0 records none of that** — it measures `Φ` and stops. That is the
honest boundary of this slice and the first thing U1 adds.

The shape of the finding is encouraging for mask work: the gaps are
THIN (6–22‰ against God uppers of 278–994‰), so the conflict structure
is a narrow sliver. A mask producer either localises it quickly or the
sliver is genuinely diffuse, and either answer is informative.

**h8-t4 is the right development fixture** — fiber 1,200, four positive
gaps, 0.70 s per coordinate, already gated here as the positive-gap
specimen. h3-t4 (11,550, 2.19 s) is the natural second rung.

### For MB2 (the sep upper)

The t4 numbers give it a target with teeth. At h4-t4 the God upper sits
at 793–994‰ while exact `Q` is 784–980‰; a separation upper that cannot
get inside an 8–21‰ band adds nothing at these coordinates, because the
deterministic doom upper is already that tight. Conversely U0 hands MB2
exact `Q` at all sixteen t4 coordinates as free ground truth to measure
looseness against — cheaper than anything MB2 would build for itself,
and already committed in the probe.

### For MB1 / the model-belief program

Flagged from SC-A7's field typing, NOT from MB1's brief, which the U0
builder has not read — treat this as an implication to check rather
than a finding about MB1.

**God-tightness is field-SPECIFIC and does not transport.** Every
object here is an equality against a doom upper computed against one
declared σ0, and §41's tower changes the field and therefore the
complex. Under SC-A7's three-way typing these are field-specific facts,
the strictest class. `SalvationContext` carries `field_id` on every
object and the equality receipt binds it, which is the mechanism — but
nothing in U0 ENFORCES non-transport, because nothing in U0 transports.
If the model-belief program begins lifting facts to Ξ = Ω×Θ, the
God-gap objects need an explicit coupling proof at that boundary, and
the boundary is worth a gate on the MB side before the first transport,
not after.

### For the unified player

Nothing here touches the live default player, and this slice claims
nothing about play strength. But the census does say something concrete
about a future exact endgame backend. §39's fusion-cut substitution
needs frontier records carrying exact continuation values and
executable continuation policies attaining them — and at t5 and t6 that
is exactly what a God-tight receipt IS. Twelve substantive ones exist,
fourteen with persisted profiles, re-priced and installed.

The horizon says that substitution becomes available at trick 5 and NOT
at trick 4 on this corpus. The shape for a unified walt is therefore:
an exact, cheap backend from trick 5 inward, with the middlegame at
trick 4 needing information-consistency work before the same trick is
available there. That is a sequencing input, not an authorization.

### One structural gap for the PR body

The corpus varies only trick depth. The fusion horizon is measured
along a single axis, so the cheapest real falsifier attempt on the
fusion-free-suffix hypothesis is a corpus that varies the CONTRACT —
same machinery, different roots, no new mathematics. SC-A4 already
forbids theorem language; this is the specific experiment that would
earn or break the hypothesis.

---

EXPLORATORY — below every evidentiary tier; quotable only via gate
receipts. The fusion horizon is a measurement on the declared corpus,
never a theorem (SC-A4).
