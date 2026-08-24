# The tilt audit (E0) — smoke design and first results

Status: SMOKE RUN 2026-08-19 (results at bottom), exploratory tier
throughout. Owns: the E0 experiment of
`math/signed_pivotal_geometry_v0.1.md` §9 as adopted and amended by the
signed-pivotal intake adjudication (`CENSUS-RULINGS.md` SP-A1..SP-A12).
Sources: the parent (verbatim, filed), the intake companion,
`SCENARIO-PLAYER.md` v0.1 + obligations O1–O9, O12–O19. Nothing in this
document is promoted by its own existence; estimates are never receipts;
not a P-A21 statement.

## What it answers

Why do small scenario samples already play strong 42 — and what should be
built next? The parent gives the instrument: for a frozen policy pair,
pivotal mass q (how often the choice matters), tilt τ (how decisively it
points one way when it does), gap g = qτ, fixed-pair hardness
H = 1/(qτ²) − 1. Four roads distinguished by measurement (parent §0):
sampling / counted-boundary / policy-library / search-instability. The
audit's falsifiers and decision gates are the parent's §9.9–9.10, adopted
as written.

## Objects (per the adjudication)

- **Frozen policy** (SP-A3): the seat's decision rule under a fixed freeze
  tuple (solver version, seed schedule, n, n0, field-model version,
  tie-refinement config). No DAG serialization (SP-A8): the tuple *is* the
  policy, its content hash is the policy ID, behavioral identity is decided
  by outcome bitsets on the panel.
- **Scenario** ξ = (ω, r): world ω = the hidden-hand assignment consistent
  with the root information state; tape r = a u64 seed from which ALL
  modeled-field randomness during replay derives (mixed with the record).
  Focal-policy randomness derives only from the freeze tuple and the
  observation record — information-consistent by construction (O1).
- **Panel**: a common scenario list, disjoint from every discovery sample
  (O13), on which every frozen policy is replayed. Hundreds scale, not
  10,000 (SP-A9: replay ≈ re-solve until explicit extraction exists).

## Smoke scope

Anchors (SP-A10), in order of attack:

1. **Mid/late-grade divergence positions** from the 900-hand / 4,156-
   decision miner corpus (2026-08-18) — per-decision solves are cheap
   there, so these get the full Phase A–D treatment first.
2. The **level-2 trick-1 saturation/tie episode** — regression anchor,
   reduced panel (early-grade, expensive).
3. A handful of **dropped-30 arena divergence positions** where walt beat
   the E[Q] champion — the "why did that work" cases.

Out of smoke scope: Phase E (world/tape decomposition) — BLOCKED on the
SP-A6 seed split, which does not exist in code yet; Phase F predicate
mining — offline, after the smoke's panels exist to mine.

## Phases (mapped to existing machinery)

- **Phase A — discovery replicates.** Per root position: solve with the
  existing level-1/level-2 configuration at n = 200, across 8 independent
  discovery seeds. Record chosen root action, runner-up, per-option basis
  points, and the freeze tuple per (seed, root action).
- **Phase B — common replay panel.** Fresh worlds sampled from the root
  fiber under panel-only seeds; tape seeds assigned per scenario. For every
  frozen policy that can affect the root winner: replay each scenario to
  terminal (focal decisions by the frozen-seed solver at its freeze tuple;
  field by the tape), record the make bit. Pairwise: N₊, N₋, N₀ →
  (q̂, τ̂, ĝ, Ĥ) with fixed-sample intervals. Single-look per panel; any
  extension is a new predeclared analysis (O14).
- **Phase C — subsampling calibration.** Panel prefixes at 25/50/100/200/…:
  winner-recovery rate vs predicted hardness Ĥ. The central plot is
  winner recovery vs 1/(q̂τ̂²), not make-rate error (parent §9.5).
- **Phase D — instability audit.** Across the 8 discovery seeds: root-action
  frequency; behaviorally distinct policies (outcome-bitset dedup);
  pairwise disagreement mass among same-action policies; held-out value
  spread; whether policy switching flips the root winner (parent §9.6).

## Driver shape

One new exploratory binary, `walt-m3-probe/src/bin/tiltaudit.rs`:

    tiltaudit <positions-file> <n_discovery=200> <seeds=8> <panel=400>

consuming root positions in the divergence-corpus format and emitting the
§13 persistence contract subset: position id, freeze tuples, per-policy
outcome bitsets (hex), N₊/N₋/N₀ per unresolved pair, (q̂, τ̂, ĝ, Ĥ),
instability table, and measured costs. Exact rationals for all statistics;
bitsets are the only packed representation.

Budget sanity: a mid/late-grade replay is ≤ 4 remaining focal decisions at
ms-scale solves → a 400-scenario panel per policy runs in seconds; 20
anchor positions × 8 seeds × ~3 live root actions stays inside an
afternoon of CPU. Early-grade anchors get panel = 100 and one position at
a time.

## Gates (parent §9.10, restated for the smoke)

- Envelope→cover audit and any conditional sampler: built only if Phase F
  predicts a real wall-clock gain (SP-A2 vocabulary: pivotal **cover**).
- Policy/facet reuse: built if Phase D shows behaviorally recurrent
  policies.
- Search stabilization: prioritized if instability dominates evaluation
  noise.
- Honest near-ties: positions where |τ̂| stays near zero after large
  pivotal evidence are declared practically indifferent, not re-sampled
  into submission (O14).

## Smoke results (2026-08-19, `tiltaudit.rs`, commits 81a1943/f60b2e3)

All numbers exploratory estimates from the driver's own frozen seeds.

**Structural finding first: the modeled field has no tape.** The level-0
mind (`Solver::modeled_choice`) is a pure function of (seat, hand, public
record) — its inner randomness is derived from state, not drawn. So under
the current field model a scenario IS a world, every world is tape-stable
by construction, and Phase E is vacuous until a stochastic field model
exists. The response lock's tape component is closed for free; SP-A6's
seed split becomes relevant only when a stochastic field is introduced.

**Trick-6 roots (2 hands, panel 30, 2 seeds):** textbook Case B — q =
2000–3000bp, τ = +1000 per-mille exactly (every pivotal world points at
the discovered choice), H ≤ 4, policies behaviorally identical across
seeds. Winner recovered at 25 worlds.

**Trick-4 roots (8 hands, panel 100, 4 seeds):** strong-gap hands
(pooled D = 25–32/400) recover the winner 4/4 at 25 worlds; near-ties
(D = 4–9/400) are honestly near-tied and recovery hovers at chance until
~100 worlds. Behavioral instability exists but is small (max pairwise
disagreement 8/100 worlds; usually ≤ 4). One discovery-selection error
caught in the act (§3's bias, live): hand 0's discovery majority picked
65 while the 800-replay panel prefers 62.

**Racing (the solver application), `tiltaudit bench`:**

- *Replay-race* (frozen policies on a common panel, exact binomial
  elimination): the audit-faithful object, but replay ≈ re-solve without
  explicit extraction — SP-A9's cost correction, now measured (5.9s vs
  full's 0.46s at trick 2). Notably its hand-0 choice sides with the
  audit panel against the full evaluator.
- *Block-race* (`level1_raced`): candidates bundle-solved on CRN blocks
  of 8, paired per-block sign test, losers stop consuming worlds.
  Opening leads: **745ms vs full's 1230ms while processing 100 worlds vs
  40**. Every disagreement with the full mode is a saturation tie
  (multiple candidates at or near 10000bp — bid-30 trick-1 saturation).
- *Race-then-refine* (`level1_race_refined`): block-race eliminates,
  surviving saturation ties go to the existing 16× refinement. Wired
  into walt-wasm as opt-in `race 1` (default path byte-identical; race
  full-hand test lawful/deterministic/conformant).

**Arena gate (`tiltaudit arena`, 24 mirrored deals at bid 30, n=40):**
strength is a dead heat — paired makes as bidder: race-only 1, full-only
2, both 11 (3 pivotal deals decide nothing at this sample; race mode
does not degrade play). Speed in game conditions: race mean 177ms vs
full 116ms per decision — the race's 2n cap plus refinement fallback
LOSES on saturation-heavy bid-30 hands, where nearly every pair ties and
elimination never bites. The bench's opening-lead win (745 vs 1230ms) is
real but regime-dependent: the race pays off exactly where evaluation is
expensive and candidates separate (openings, higher bids where P(make)
spreads); fixed-bid-30 self-play is its worst case everywhere else.
Opt-in wiring is therefore the right default posture: `race 1` for
opening leads / high-bid contracts, full evaluator elsewhere, strength
equal either way.

**Road verdict (parent §0, so far):** sampling road confirmed for
mid/late grades (Case B/D dominate; H small); search-instability road
mostly quiet (small behavioral spread, but hand 0 shows it is not zero);
counted-boundary road untested (Phase F not run); plan-library road
untested. Next gates: Phase F predicate mining over the stored panels,
the level-2 saturation-tie episode as an anchor, and a high-bid arena
(calibrated θ walks contracts to ~40, where ties thin and racing should
bite — the regime the bid-30 gate cannot see).

**Level-2 successor:** the field-swap probe — where pivotal mass wakes
up when the field is upgraded from level-0 to level-1 minds — is
specified in `LEVEL2-PROBE.md` (SPEC only, deliberately not started;
gated on walt unification and the adaptive-sampling mathematics).
