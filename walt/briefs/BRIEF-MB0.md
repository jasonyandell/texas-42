# BRIEF-MB0 — the model-belief exact vertical slice (the §76 go/no-go)

**Authorized:** 2026-09-01, Jason's "full go" (rulings MB-A1..A8,
`walt/CENSUS-RULINGS.md`). **Binding theory:**
`walt/math/model_belief_base_player_v0.1.md` §§5–15 (types, Ξ, reduction,
persistence, hand-type factors, posterior closure, branch masses,
residual), §§16–20 (response vectors, sep upper, fusion price), §§74–76
(the bounded assignment, the required report, the go/no-go), plus
obligations MB-O1..O9, MB-O12, MB-O16 and gates MB-I1..I10 where they
touch this slice. **The intake companion governs where it repairs the
parent** (`walt/math/model_belief_base_player_v0.1_intake.md` — note
the §8 and §34 transcription errata and the corrected rung table).
Read `walt/FACTOR-BELIEF.md` (the status ledger) before writing code.

## Mission

Build ONE exact finite-type vertical slice — the §74 assignment,
restructured only where noted. No live-player change. No new rules
engine (reuse `solver::{field, factor_belief, kernel, proof_state}`
authorities through their existing interfaces). In-crate additive:
new module `walt/walt/src/solver/model_belief.rs` plus a gate file and
a probe binary, nothing else restructured.

1. Register the two existing deterministic solver fields as persistent
   behavior types: **F₀ = `FieldKind::Level0` (σ0)** and
   **F₁ = `FieldKind::Level1`** at their existing declared parameters.
   `BehaviorTypeId` = content address per §51 (construction, parent
   field identity, tie rule, persistence scope — deterministic fields:
   no tape). Changing any behavior-affecting coordinate must change
   the identity (gate).
2. Rational prior ν = (1/2, 1/2), independent across hidden seats.
   Persistence scope = per hand: the type is fixed once per seat for
   the whole hand, NEVER resampled per action (MB-I2).
3. Extend hidden-seat factors from H to (H, θ): `HandTypeFactor` per
   §52, wrapping the existing factor-belief machinery — do not fork
   it. All exact integers/rationals; the crate denies floats.
4. Exact public-action branch masses Z_ht (§13): group by public
   action, NEVER by hidden type (MB-I4, merge-before-max §32).
   Branch masses must sum exactly to the parent mass (MB-I6).
5. Condition the posterior after one observed action via Theorem 12.1
   (multiply only the acting seat's factor by its kernel likelihood).
6. Evaluate one frozen focal policy (reuse an existing extracted or
   frozen SlicePolicy) through one full small root under the mixture:
   V_ν(ρ) = Σ_θ ν(θ) V_θ(ρ). Fixed-policy value must be exactly
   linear in ν (gate).
7. **Parity against explicit augmented-state enumeration**: on small
   receipt roots (use the enumerable t5/t6 roots from the receipt
   corpus — h12-t6, h10-t6, h5-t6 are the cheapest), enumerate
   (ω, θ) pairs explicitly and check every mass, posterior, and value
   to exact equality.
8. **Point-mass parity both ways** (MB-I5/MB-O2): ν = δ_{F₀}
   reproduces the existing σ0 fixed-field value AND selected action
   on every tested root; same for ν = δ_{F₁} against the Level1
   authority. This is the ladder-demotion theorem made mechanical.
9. Compute the type-revealed separated upper U^sep = Σ_θ ν(θ) q(θ)
   (§18) and the exact mixture response Q(ν) (max over lawful focal
   policies — reuse the existing exact response machinery on the
   augmented space via Theorem 7.1's reduction: solve the
   fixed-semantics problem on Ξ).
10. Record the model-fusion price Φ = U^sep − Q(ν) per root action.
    Verify Q(ν) ≤ U^sep everywhere (Thm 18.1) and the §19
    zero-iff-common-optimizer characterization on at least one root
    where it is zero and, if the corpus provides one, one where it is
    strictly positive.

## Gates (one test file, `walt/walt/tests/solver_model_belief.rs`)

- G1 exact (ω,θ) enumeration parity on ≥ 2 receipt roots (masses,
  posteriors, fixed-policy values, exact mixture response).
- G2 point-mass parity: both δ endpoints reproduce the existing
  fixed-field authorities (value AND selected action).
- G3 posterior closure: observed action multiplies only the acting
  seat's factor; branch masses conserve exactly; the persistent-vs-
  resampled distinction is witnessed on a two-action fixture (the §9
  ½ vs ¼ specimen, built from real types on a real root or a minimal
  synthetic carrier — either is fine, label which).
- G4 merge-before-max: types choosing the same public action stay in
  one branch; a focal policy keyed on hidden type must be
  unconstructible or rejected (MB-I1 — make the API make it
  impossible, then gate the rejection).
- G5 linearity of V_ν(ρ) in ν (exact, on a swept rational grid).
- G6 sep upper: Q(ν) ≤ U^sep on every tested root; equality iff one
  common policy is pointwise optimal on support (checked against the
  enumeration).
- G7 identity: every behavior-affecting parameter change produces a
  new BehaviorTypeId; equal construction produces equal id.

## Probe (`modelbeliefreport`, output to `walt/probes/factor_belief/modelbelief_run1.txt`)

The §75 report per test root: physical world mass; augmented mass;
prior; active types by seat; branch masses by public action; posterior
type weights after each observed action; fixed-policy mixture value;
exact mixture response; sep upper; model-fusion price; distinct type
actions vs merged public branches; wall time and peak declared memory;
parity verdict. Plus the §76 criteria stated as explicit YES/NO lines
at the end — this probe IS the go/no-go evidence.

## Discipline

- Run `walt/ci/check.sh` green before calling it done (fmt, clippy
  `-D warnings -D float_arithmetic`, no-float grep, vocabulary greps,
  release tests, receipt byte-diffs).
- Never touch `ingest/`, `solver/refine.rs` (freeze 58), or the live
  player. Everything is walt-exploratory tier; the probe numbers are
  quotable only via the gate receipts.
- Append a status paragraph to `walt/FACTOR-BELIEF.md` (match the
  house style: what landed, gate file + count, probe findings with
  exact rationals, discoveries and honest negatives).
- Commit on the current branch with a message starting `walt MB0:`.
  Do not push, do not open a PR — the orchestrating session reviews
  and lands it.

## Report back (your final message)

Slice status; gate count green; the §76 five criteria each YES/NO with
one line of evidence; the fusion-price findings (which roots, what
values, exact rationals); any ambiguity hit (per the ambiguity
protocol: blocked test + exact conflicting passages, never a silent
reading); anything you'd flag for MB1.
