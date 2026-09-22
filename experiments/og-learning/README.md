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

## The constructor and campaign og-v2

og-v1 froze the dictionary at the seed library; og-v2 opens the language
(parent §5). `src/constructor.rs` declares a fixed typed language over the
scheme registry's viewer predicates: 22 action fragments (double/master/
count-n and their negations, follow/boss/would-lead of the led context,
trump membership and trump-boss through the `Called` context literal, and
the `takes-trick` composite - action beats the current winning tile) x 13
qualifier fragments (partner/opponent currently winning, the four trick
positions through leader/successor/partner chains, and hand-size stage
markers through `quota`), assembled as `own-legal(action)` conjunctions:
each action fragment alone, crossed with each qualifier, and every pair of
action fragments - 539 expressions, every one compiled and text-round-trip
verified. A human declares the language and its caps; the campaign selects
within it. Per generation: dedup + constancy filtering on a declared prefix
of the versioned panel; candidate derivatives g_F at coefficient zero on a
fresh frozen-incumbent bundle (parent §3); top-3 admitted at weight 1 -
semantically invisible until the gradient moves them; dictionary cap 60.
Admitted expressions persist in `state.txt` as canonical single-line
s-expressions and re-compile at load.

**og-v2 declaration**: same target law as og-v1; every seed base shifted by
the campaign offset 100,000,000 (no deal shared with og-v1); constructor
on; train 4096 x 8 inner steps, dev 2048, stall 3; constructor scoring
bundles at region offset +50,000, capped at 2,048 deals; panel filter reads
the first 64 panel seeds (the 256-seed panel remains the versioned record).

**og-v3 declaration** (`og-v3/bid30-longest-pip/S0S2-learner/S1S3-level0-n8-v1`,
authorized by Jason 2026-09-21 "go - declare og-v3 against L0-8 and let it
run"): identical law to og-v2 except the fixed seats are walt's
`Level0Field::new(8)` - the σ0 modeled mind, n0 = 8 no-void belief worlds
from the frozen INNER_SEED derivation, best response against the Dice
field; a pure function of seat, hand and public record, so pairing and
replay determinism hold (cross-process replay verified at init). Campaign
offset 200,000,000; constructor on; all other budgets and the evidence
rule unchanged from og-v2. Measured cost at init: 0.70 ms/deal (make rate
of the uniform start 323/1000 on bench seeds vs 456/1000 under og-v1's
random-legal field - the harder lineup, at essentially no extra cost, so
the §6 multifidelity harness stays queued for genuinely expensive lineups
like the gym field).

## Layout

- `src/bounds.rs` - conservative exact-rational Hoeffding machinery.
- `src/rng.rs` - SplitMix64 + exact rational categorical sampling.
- `src/promotion.rs` - the registered evidence rule.
- `src/target.rs` - the campaign target: deal law, declaration rule, root.
- `src/features.rs` - the growing expression dictionary (seed + learned).
- `src/constructor.rs` - the typed language, panel dedup, admission.
- `src/actor.rs` - the rational multiplicative-weights softmax actor.
- `src/rollout.rs` - complete-deal driver on the walt rules engine.
- `src/gradient.rs` - the outcome-gradient estimator (LOO baselines).
- `src/campaign.rs` - the generation loop, state, records, exam.
- `src/bin/og_campaign.rs` - init | bench | train | panel | exam.
- `campaigns/<name>/` - run records; RESULTS.md + measurements govern prose.

**og-v4 declaration** (`og-v4/bid30-longest-pip/S0-learner/gym-field-l1p40-l0o8-v1`,
authorized by Jason 2026-09-22 "go - gym field target, use the multifidelity
harness"): the learner holds S0 ALONE; the other seats are walt's maintained
`GymField::new(S0, 40)` - S2 is the L1 fixed-40/8 partner, S1+S3 are
Level0Field(8); every fixed choice is a pure function of public state (seed
= mix(420600 ^ key digest)), so pairing and replay hold. Campaign offset
300,000,000. **Declared cheap proxy** (parent §6):
`og-v4-proxy/.../S1S2S3-level0-n8-v1` - same law and learner, all fixed
seats Level0Field(8). Training, construction and development screening run
on the proxy (selection only, no guarantee claimed); the promotion stream
is the two-batch estimator Delta_hat = mean_N(D_L) + mean_M(D_H - D_L) over
independent declared seed subregions (cheap pairs at promo_base+i,
correction quadruples at promo_base+70,000+i), judged by
`MfEvidenceRule::gym()`: delta = 1/20, practical threshold tau = 1/25
(declared for this campaign; gym resolution economics are recorded, not
hidden), checkpoints (N, M) = (16384, 256), (65536, 1024), (65536, 4096),
per-batch exact empirical-Bernstein radii (Maurer-Pontil; D_L range 2,
D_H - D_L range 4) at half the checkpoint alpha each. Every stream records
the measured proxy-target covariance, per-pair costs, and the §6 allocation
ratio computed from measured quantities. The exam is DIRECT gym-field
paired deals (the direct-target-only control), n = 2,048. Measured at init:
gym 24.7 ms/deal (rayon-parallel L1 partner), proxy 0.7 ms/deal; uniform S0
makes 9/20 on bench seeds beside the L1 partner.

**og-v4b declaration** (2026-09-22, pilot-informed continuation): identical
to og-v4 except promotion mode `direct-eb` (the og-v4 gen-0 pilot measured
Var(D_H - D_L) ~ 0.404 > Var(D_H) ~ 0.313 at correlation ~0.35, making MF
promotion ~1.8x worse than direct at measured costs - see
campaigns/og-v4/RESULTS.md): direct gym-field promotion streams with
empirical-Bernstein radii, tau = 1/100, checkpoints 2048/8192/32768;
campaign offset 400,000,000; proxy retained for training, construction and
screening only.

**og-v5 declaration** (2026-09-22, authorized "yes do that please" - the fair
second pilot plus the CE anytime harvester): same gym-field target lineup as
og-v4/og-v4b; declared proxy `GymField(S0, 4)` (same architecture, cheap L1
partner, 3.6 ms/deal vs 24.7) used for training, construction and screening
only. **The fair pilot** (og-v4b incumbent vs uniform, 1,024 paired seeds,
both lineups; record `campaigns/og-v4b/pilot-og-v5-proxy.json`): rho = 0.38,
Var(D_H-D_L) = 0.363 > Var(D_H) = 0.297, direct wins by 2.4x - the second
independent measurement that make/set paired differences are proxy-resistant
here. Promotion mode `anytime-direct`: the adjudicated CE-T4/T5 bounded-mean
betting mixtures (`walt::solver::evidence::BoundedMeanMixture` is the
authority; the count-based fast-exponent evaluation is gate-checked equal),
lambda grid {1/64..1/2} equal-weighted, tau = 1/100, one alpha_k =
delta/(k(k+1)) per candidate split across the promote (CE-T4) and futility
(CE-T5) sides, judged every 512 paired deals, stream cap 65,536 - optional
stopping valid at EVERY n, which is what harvests the 1-2% band the
fixed-checkpoint rules leave unresolved. Campaign offset 500,000,000.
