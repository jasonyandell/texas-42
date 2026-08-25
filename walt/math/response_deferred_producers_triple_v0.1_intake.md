# Intake companion — the x:024 response ("Three Deferred Producers")

**Parent (verbatim, never edited):**
`exchange/inbox/024-response-deferred-producers-triple-v0.1.md`
SHA-256 `337296a7e6241e2c55fc5ee0bfb37af7b10c65b978bc8f67fd19d1086cea61cf`.
Companion verifier (shipped beside the parent, filed verbatim):
`exchange/inbox/verify_deferred_producers_triple_v0_1.py`
SHA-256 `a4d23d50820812a9e0bd2bca7a73af505e251dc1f3815898b8624217464eddf1`.

**Status of this document:** exploratory intake record. The verifier is
**scratch tier at best**: useful, a reference, never a candidate for
import into the codebase; its PASS lines are session evidence, not
receipts (TRUST-01).

**Provenance:** Pro's response to dispatch x:024
(`exchange/outbox/024-deferred-producers-triple.md`), the three-part
design brief on the constructions slice 3 deferred LOUDLY. Jason
hand-ferried the dispatch 2026-08-24 and hand-delivered the response
2026-08-25 ("I think it's promising"). Adjudicated 2026-08-25 per the
exchange protocol: program executed, proofs step-checked, shipped-code
conformance checked against the response's load-bearing condition.

**Thread label:** **[L2]** throughout, with Part 1 consuming the CE
evidence engine through the sanctioned one-directional crossing (L2
consumes CE machinery; CE never consumes L2).

## Adjudication receipt (what was actually re-run)

- **Verifier executed** (stdlib Python 3, exact `fractions`, integers
  only in correctness logic, run with `-B` from the session scratch
  dir): **13/13 PASS, exit 0** (`ALL CHECKS PASS`). Includes the
  exhaustive Part-1 sweep: all 256 two-policy Boolean tables on four
  worlds × all 256 length-four streams (65,536 stream evaluations),
  worst finite-horizon undercoverage **11/128 < δ = 1/4**, and E3
  pathwise ≤ fused E2 on every prefix of every stream. Session record
  2026-08-25.
- **Specimen arithmetic re-derived independently:**
  E^>_{2,2}(1/4) = 1/3 + 1/2 + 3/10 = **17/15** < 4, so 3/4 survives at
  s = f = 2 — the R = 1/2 ≤ E3 = 3/4 < E2 = 1 specimen stands.
- **Endpoint monotonicity re-checked:** replacing one failure by one
  success scales the lower-mixture integrand by r(1−c)/(c(1−r)) ≤ 1 on
  r ≤ c, so E⁻ is nonincreasing in s and U_{δ,N} nondecreasing — the
  step that collapses the whole policy family to one integer count.
- **Load-bearing implementation condition verified in shipped code:**
  §1.4 requires the empirical count to be
  max over ONE information-consistent ρ shared across all worlds (a
  per-world choice would produce the fused E2 count instead). Our
  shipped `sampled_split_reach`
  (`walt/walt/src/solver/exposure.rs`) runs the E4 walk verbatim on the
  declared prefix — `max_count` maximizes over exactly one
  information-consistent continuation across the sample (the walk's
  same-action-per-public-node invariant). **The shipped statistic is
  already S\*_n.** No code change is needed to feed the new bound.
- **Vocabulary sweep:** zero occurrences of the banned bare
  "certificate" in the response; witness language throughout
  (`HazardExclusionWitness`).
- **Part-3 fixture histogram inspected:** the verifier's own primary
  partition came out Count = 6, Trump/Suit/Strength = 2 each — several
  fixtures *intended* for later coordinates were captured by the
  earlier count coordinate under first-difference priority (the
  program's comment anticipates this). The checks honestly assert
  coverage, not intended counts, and the histogram is itself a live
  demonstration of the response's own caveat: the ordering is a
  taxonomy convention, flags must travel with the primary label.
- **Caveat kept in view:** the exhaustive sweep is a finite-horizon
  sanity check; the *anytime* property rests on the step-checked proof
  ledger (P1–P3), not on the program.

## Verdicts by part

| Part | Deferral | Verdict |
|---|---|---|
| 1 | δ-valid admissible-upper E3 producer | **CONSTRUCTION delivered.** Theorem M1 (max-preserving upper CS, no Bonferroni split) + Corollary M2 (empirical-optimum collapse) + directional variants + P4 (E3 pathwise ≤ fused E2). P1–P4 step-checked sound. |
| 2 | Dominance valid-bound route | **CONDITION delivered.** No canonical weakest *local* rule exists; the weakest exact condition is hazard-terminal unreachability in the paired product, witnessed by a Hazard-Exclusion Invariant — sound (H1) and semantically complete (H2). One-round trump-extraction witness as first deliberately incomplete producer, with the demanded non-coverage instance (three-trick two-round extraction: dominance real, witness declines). P5 step-checked sound. |
| 3 | §10 motif tags (item 14) | **ALPHABET delivered** — and `RevealResponse` correctly refused. Six first-split morphology motifs + `Other` via least-differing coordinate of a six-coordinate local signature, orthogonal flags mandatory, root-frame resolution required (else `Other(missing_root_frame)`, never guessed). P6 step-checked sound. |

## The turn of the key (Part 1)

The deferral said "a valid upper bound on a supremum is not a mean-style
evidence problem." The response's answer: it **is** a one-mean problem —
for the *true maximizer*, which you never need to identify. Fix any true
maximizing ρ\* (belief-selected, not sample-selected); then
U\*_n = max_ρ U_{ρ,n} ≥ U_{ρ\*,n} covers R = μ_{ρ\*} at the *same* δ,
under arbitrary cross-branch dependence. Endpoint monotonicity then
collapses max_ρ U(S_{ρ,n}, n) to U(S\*_n, n) — one integer. The
bias-high property of the shipped estimator (E[max of sums] ≥ max of
means), which the dispatch flagged as the failure mode of any mean-style
treatment, is exactly the safe direction for an upper bound: the defect
of the estimate is the validity of the bound.

Dispositions of the dispatch's proposed routes: baseline (0) blessed
sound but pathwise dominated; route (a) valid but the δ-split it pays
is unnecessary; route (b) — the branch-mixture e-process — **refuted
for upper bounds** (a mixture tests the intersection null and yields
*lower* confidence on the maximum; the union null needs
intersection-union, i.e. the branchwise max); route (c) realized by the
theorem.

## Hypotheses that must stay declared (Part 1)

- Fixed policy class for the evidence epoch; no data-dependent mutation
  of policy identities inside an epoch.
- The declared world stream is i.i.d. from the fixed belief at the
  epoch (the standing CE evidence-stream construction).
- Grid validity needs the known fiber size N (the exact fiber counter
  supplies it), so every true mean lies on G_N.
- §1.8: the no-|Π_a| result is *inside one scalar maximum*. Risks
  across distinct screen inputs (per action, per direction, baselines)
  still sum against the screen budget — the declared-allocation ledger
  stands unchanged.
- Sampled E3 results carry (action, direction, upper, δ, stream epoch,
  prefix length, policy-class id) and are never described as tighter
  than exact E4 because a realized number happens to be smaller.

## What Part 2 fixes about our own framing

The deferral reasoned "a bound type with no producer invites misuse
beside the type lock." The response inverts the order of construction:
build the **general Hazard-Exclusion Invariant verifier first**, as the
single authority; pattern producers (one-round trump extraction being
the first) emit witnesses *for that authority* and never own dominance
semantics of their own. The type lock is preserved and sharpened:
`SampledPairwiseMasses` still has no dominance method, and
`StructuralHazardZero { hazard_upper = 0, delta = 0 }` is inhabited
only by verifier-checked witnesses — a sampled object can never reach
it. Honest boundaries retained: witness *language* is where
incompleteness lives (H2's completeness is semantic — the reachable set
itself is an invariant, but may not compress); no cross-field dominance
from a single-field witness (a field-action-family witness is strictly
stronger, never automatic).

## What Part 3 refuses, and what it unblocks

Buildable now, over current correction traces: the six-motif classifier
with flags and `Other`. Binding on every aggregate: current
`FieldSplitTrace` records exist only for u₀ ≠ u₁ worlds, so motifs
partition **correction mass, never field exposure** — "motif k accounts
for this fraction of exposure" is unsupported until non-pivotal exposed
worlds are also classified. The refused-aggregates list (causal claims,
pooled good/bad-play labels, exposure-by-motif from correction-only
traces, dominance from sampled motif hazards, cross-fiber pooling) is
adopted as written. No numerical residual-rate forecast is invented;
the residual is an instrument.

`RevealResponse` stays out: not decidable from the current trace (no
post-split suffixes; no but-for evidence). The prerequisite is raw
schema enrichment — persist `branch0_suffix`, `branch1_suffix`,
`root_semantics_hash` — which also closes our flagged item-11 gap (the
distinguishing public observation becomes explicit and replayable). Any
later second-layer label is `PartnerResponseCandidate`, never a causal
attribution; but-for would need an intervention-replay producer, a
separate construction nobody has asked for yet.

## Advisory implementation order (response §5, recorded not committed)

1. Part-1 E3 producer (exact upper-CS inversion + prefix minimum +
   result type + risk wiring; the count already exists).
2. Directional E3 with separate ledger entries.
3. General Hazard-Exclusion Invariant verifier *before* any pattern
   library.
4. One-round trump extraction as the first incomplete producer — its
   refusal path is part of its correctness.
5. Six-motif classifier over current correction traces, flags and
   `Other` included.
6. No `RevealResponse` until the suffix enrichment lands.
7. Three compact mechanization candidates (max-preserving upper CS;
   invariant soundness; first-difference partition) — recorded for
   Jason's unified Lean treatment with Pro, not a repo tranche.

## Rulings

TRIPLE-A1 through TRIPLE-A7, appended to `walt/CENSUS-RULINGS.md`
under "The deferred-producers adjudication (2026-08-25)". The card
`kanban/backlog/slice3-deferred-producers.md` now carries the
adjudicated designs; closure still requires gated producers (or
recorded not-wanted rulings), exactly as its done-when states.
