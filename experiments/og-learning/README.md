# og-learning: outcome-grounded policy learning (exploratory instrument)

Implements the 2026-09-21 mathematical contract
(`walt/math/outcome_grounded_policy_learning_v0.1.md`; the maintained intake
companion beside it governs where it narrows the parent). Everything here is
**EXPLORATORY tier**: results files govern prose, no number here is quotable
above walt's exploratory fence, and no default player anywhere is touched.

This crate lives outside the walt workspace deliberately: it consumes the
`walt` crate read-only (rules engine, kernel, scheme runtime) and never
modifies walt sources, so walt's own gate is not invalidated by work here.
Gates for THIS crate are its `cargo test` suite, sized to laws (one coordinate
per law plus a PINNED strictness witness), not censuses.

## The campaign target T (declared before any run; frozen by digest in the run record)

Following the parent's §1, a campaign names every experimental authority up
front. The first campaign, `og-v1`:

- **Deal law**: uniform over complete 28-tile deals, seeded (`deal_seed` from
  the declared ranges below), dealt 7-7-7-7 to seats S0..S3.
- **Assignment**: S0 declares **sixes** at **bid 30**, S0 leads trick 1.
  No auction is modeled; this is a fixed-contract campaign target, exactly the
  regime the existing relational campaign used (bid 30, make/set only).
- **Learner**: the S0+S2 partnership - two decentralized invocations of ONE
  shared coefficient vector; each invocation sees only its own hand and the
  public record (the parent's §2: shared parameters are not shared private
  observations).
- **Fixed other players**: S1 and S3 play **uniform random legal** (the
  lineup's full policy semantics: at every decision, the uniform law over the
  legal set, drawn from a per-(deal, decision) derived stream). Changing this
  lineup is a NEW target.
- **Utility**: Y = 1 if the declaring side takes at least 30 points
  (count dominoes + tricks) when the hand resolves, else Y = 0. Make/set,
  never expected points (parent §1).
- **Observation semantics**: the actor consumes own remaining hand plus the
  full public record through `PolicyInput`; hidden-world predicates are
  excluded by the scheme runtime's `Access::World` rejection.
- **Actor law**: rational multiplicative-weights softmax over the versioned
  clause dictionary (`scheme-relational-actor-v1/grammar-v1/straight-v0.4`
  library, 14 clauses): weight(a) = prod_j r_j^{x_j(I,a)} over legal a, where
  x_j(I,a) = 1 iff clause j's answer relation at I contains a. All r_j are
  positive rationals; theta = log r is never materialized; every probability
  is exact-rational and every draw is exact (see `src/rng.rs`).
- **Budgets**: declared per run in the run record (generations, deals per
  gradient estimate, promotion checkpoint schedule, wall budget).

## The registered evidence rule

`EvidenceRule::standard()` (`src/promotion.rs`): total risk delta = 1/20
across the entire campaign; practical improvement threshold tau = 1/100 on
E[Y_cand - Y_inc]; paired bundles (one bundle = one fresh deal seed, both
arms coupled on exogenous randomness); deterministic checkpoints
n = 256, 1024, 4096, 16384, 65536; alpha[k, j] = delta/(k(k+1)j(j+1)) so
the double sum is exactly delta; two-sided Hoeffding radii computed as exact
rational UPPER bounds (`src/bounds.rs` - overstating the radius only delays
decisions, never spends unbudgeted risk). Promote iff mean - r > tau; resolve
NotPromoted iff mean + r < tau; exhausting checkpoints is **UNRESOLVED**, a
typed outcome, never a loss (parent §8-9).

## Seed ranges (disjoint by construction, declared here once)

- Training / gradient estimation: deal seeds `1_000_000 + g * 100_000 + i`
  for generation g < 20, deal i - never reused across generations.
- Development screening: seeds `3_000_000 + g * 10_000 + i` (fresh per
  generation; candidates and incumbent share them - paired screening).
- Dedup probe panel: seeds `500_000..500_256`, versioned with the dictionary.
- Final untouched exam: seeds `9_000_000..9_004_096`, touched exactly once,
  by the final incumbent, after the campaign stops.
- Promotion streams: seeds `20_000_000 + k * 100_000 + i` for frozen
  candidate k <= 100 (frozen before its stream starts).

## Layout

- `src/bounds.rs` - conservative exact-rational Hoeffding machinery.
- `src/rng.rs` - SplitMix64 + exact rational categorical sampling.
- `src/promotion.rs` - the registered evidence rule.
- (further modules land with the trainer: actor, features, rollout, gradient,
  generation loop, campaign binary)
- `campaigns/<name>/` - run records; RESULTS.md + measurements govern prose.
