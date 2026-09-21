# Intake companion: outcome-grounded autonomous policy learning v0.1

**This is the maintained companion** to the received parent
`outcome_grounded_policy_learning_v0.1.md`, under the DS-A18/GT1-A1 convention:
the parent is verbatim and never edited; this companion is edited only under
dated provenance markers and governs wherever it repairs or narrows the parent.
First filed 2026-09-21 (the intake session). **Adjudication: PENDING — no
rulings family exists for this lineage yet.** Nothing in the parent or this
companion is quotable above walt's exploratory tier, and nothing here
authorizes a build.

## 1. Provenance

- **Received** 2026-09-21, hand-delivered by Jason via session upload
  (upload filename `dd7ce80a-MATHEMATICAL-CONTRACT.md`; the artifact's internal
  title is *Outcome-grounded autonomous policy learning for Texas 42*, Version
  0.1, dated September 21, 2026). Filed snake_case as
  `walt/math/outcome_grounded_policy_learning_v0.1.md`, recorded not repaired.
  Not a courier dispatch — the exchange ledger is untouched.
- **Checksum-pinned**: SHA-256
  `bd259effaf78972aea9c78f5ff93abfbee8628accce4423003ac4d8f6902389e`
  (`outcome_grounded_policy_learning_v0.1.sha256`, verified at filing).
- **Written against main exactly.** The parent's repository-sources section
  names `jasonyandell/texas-42` main `afd46420435713de4b3b2b98ae09778f3e85c971`;
  verified at intake to be exactly `origin/main` at filing. It also inspected
  PR #93 head `cd70c578…` (`experiments/response-ladder/`) — **not on main**;
  that PR was unmerged at intake.
- **The parent's own verification was NOT delivered.** Its §10 names
  `verify_learning_bridge.py` and `verification_results.json` as included;
  neither arrived with the upload. Until they are retrieved or declared
  unfiled (the SP-A11 precedent), §10's counts — 96 games, 576 gradient
  components, 288 remainder bounds — are the parent's own report, checked by
  nothing in this repository. The intake verifier below is a **separate,
  intake-authored** check, not a substitute for the missing pair.
- **Source lineage, audited.** The parent lists four "recovered mathematical
  sources". Their status in this repository:
  - `TEXAS42-UNIFIED-REVIEW-v0.1.md` (2026-09-05) — in the partnership packet,
    **MISSING intake** ([walt-math-intakes](../../wiki/walt-math-intakes.md) §9).
  - `PATH-TO-GENERALIZING-SCHEME-POLICIES-v0.1.md` (2026-09-07) — in the
    relational packet, **MISSING intake** (same §9).
  - `SCHEME-GEOMETRY-AND-SMOOTH-POLICIES-v0.1.md` (2026-09-07) — **not in this
    repository at all** (grep at filing finds no copy and no reference outside
    the parent). The parent says the relational policy and the
    outcome-gradient bridge "are already there"; that priority claim is
    unverifiable here.
  - `POLICY-ANTS-WALT-v0.1.md` (2026-09-12) — **never retrieved** (the failed
    retrieval is recorded at
    `experiments/partnership/packet/policy-ants/INTAKE-STATUS.md`). This
    parent is the first filed artifact carrying any of its content.

  Consequence: whatever the parent restates from the two absent sources enters
  this repository **only as the parent's own text**, with no authority
  inherited from the lost originals.
- **The consolidation ruling.** Jason ruled 2026-09-04: no new mathematical
  parent until the consolidation slice lands. This filing was made on Jason's
  explicit 2026-09-21 instruction — the same authority. The intake treats the
  ruling as still governing the **build**: the proposed program in §8 is
  queued behind adjudication, and the ruling's disposition is agenda item 2.
- The parent's own status line is tier-clean: "Not an implemented Texas 42
  learner, a strength result, or a kernel-verified theorem."

## 2. What the parent is

A proposed mathematical contract for autonomous policy learning against a
declared target: campaign targets as complete experimental authorities (§1);
a weighted relational softmax actor over a bounded dictionary of typed Viewer
expressions (§2 — explicitly *not* the ordered first-match semantics of
PR #93); the outcome-gradient bridge — score-function gradients of expected
terminal utility with fixed other players, including two decentralized
teammates sharing parameters, and a covariance-form proposal score for fresh
expressions (§3); an elementary finite-step improvement bound with an
H(H−1)α² remainder and an L1 coefficient-to-TV bound (§4); an autonomous
constructor loop (§5); multifidelity reuse of cheap modeled players as
control variates, never as teachers (§6); placement of the existing
mathematics (§7); risk-budgeted promotion with frozen candidates and
deterministic checkpoints (§8); an old-data boundary and honest stopping
semantics (§9).

## 3. Mathematical audit

Every checkable claim was step-checked by hand at intake, and the central
algebra machine-checked exactly (§4 below). **No unsound claim was found.**
The parent is unusually well-fenced: every strength, optimality, and
generality disclaimer is already in its own text.

- **§2 (expressions, softmax consumer).** Definitions sound. The
  hidden-world-predicate prohibition for the deployed actor matches
  `walt::scheme`'s own/public `PolicyInput` discipline exactly. The history
  caveat ("the fact that a record stores history does not establish that the
  present expression evaluator exposes it") is correct against current code.
- **§3 (outcome-gradient bridge).** SOUND. The trajectory factorization
  p_θ(h) = c_T(h)·Π π_θ(a_t|I_t) with fixed reactive opponents inside c_T,
  the log-derivative identity, the score identity
  ∂_j log π(a|I) = x_j(I,a) − E_{b~π} x_j(I,b), baseline invariance under the
  conditional-zero identity, and the fresh-expression derivative
  g_F = E Σ_t Cov_{a~π(·|I_t)}(x_F(I_t,a), Q^π(I_t,a)) at coefficient zero —
  all verified exactly, including the decentralized shared-parameter case
  (both controlled seats differentiate through one coefficient vector while
  seeing only their own information). The parent's fences — Q^π is the frozen
  actor's forced-action value, not Q*; the covariance is a proposal score,
  not proof of gain — are kept verbatim.
- **§4 (finite-step bound).** SOUND. The exact telescope
  J(π′) − J(π) = Σ_t E_{d_t^{π′}} f_t and the remainder
  |J(π′) − J(π) − S_π(π′)| ≤ H(H−1)α² check out: |f_t| ≤ α because Q ∈ [0,1]
  and TV allows a free constant shift; the first-changed-action coupling
  gives TV(d_t^{π′}, d_t^π) ≤ (t−1)α; each term contributes at most
  2(t−1)α²; summing gives H(H−1)α². The L1 bound α ≤ min(1, ‖θ′−θ‖₁/4) is
  sound by the stated interpolation: the TV speed is
  ½ Σ_a π_a|δ_a − E_π δ| ≤ ¼·range(δ) (mean absolute deviation ≤ half the
  range), and range(δ) ≤ ‖θ′−θ‖₁ for features in [0,1]. The interpolation
  itself passes through irrational policies, so only its derivative-level
  engine is machine-checked; the integrated statement is hand-checked.
  The fence on the single-focal optimal-regret telescope (fixed field,
  coherent beliefs, single seat — do not silently apply it to a jointly
  optimized partnership) is consistent with the decision-sparse lineage's
  own assumptions. No conflict with any adjudicated object was found.
- **§6 (multifidelity).** SOUND. The expectation decomposition, the
  independence of the two batches, the exact variance split
  Var(Δ̂) = Var(D_L)/N + Var(D_H−D_L)/M, and the allocation ratio as the
  continuous stationary point all verify. The parent's own caveats carry the
  weight: pilot-estimated variances, a direct-target-only control arm, a
  poorly correlated proxy can cost more than it saves, and the transfer
  bound must hold for BOTH policies being compared. Its reading of PR #93 —
  cheap proxies valued by cost and correlation, never as stronger teachers;
  the "2.13x" figure is not a measurement of this estimator — is the correct
  tier discipline.
- **§7 (placement).** Verified as placement, not new claims. The pivotal
  identities are exact (checked exhaustively). The "important invariance"
  (tightening a common max-U shifts every actor's loss equally, so it cannot
  directly improve a ranking) is sound by inspection — a uniform additive
  shift. "Support does not determine a behavior-conditioned posterior"
  restates the 90-world lesson ([belief-vs-support]) correctly.
- **§8 (promotion).** SOUND as stated: the α[k,j] = δ/(k(k+1)j(j+1))
  allocation double-sums exactly to δ; the Hoeffding radius
  r = √(2·log(2/α)/n) is the standard two-sided bound for means of [−1,1]
  variables; the union bound is valid under the stated discipline (candidate
  frozen before its fresh promotion stream, deterministic checkpoints,
  bundles as the independence unit). The parent itself calls this the
  illustrative safe construction and invites variance-sensitive confidence
  sequences — which walt already has, adjudicated (CE-T1..T5); agenda item 6.
- **§9 (old-data boundary).** SOUND: deterministic logs have zero support
  for untaken actions and importance weighting cannot manufacture those
  outcomes; retained Walt games keep exactly the four listed roles. "Running
  out of compute is UNRESOLVED, not a loss" matches the CE result-type
  ladder discipline.

## 4. The intake verifier

`verify_outcome_grounded_policy_learning_v0.1_intake.py` — **intake-authored**
(the suffix marks it: this is not the parent's missing verifier), stdlib-only,
exact `Fraction` arithmetic throughout, no floats, no randomness,
deterministic enumeration; imports only `fractions`. Run twice at intake from
the filed location with `python3 -I -B`: both runs
`10 CHECK FAMILIES / ALL CHECKS PASS`, 0.72 s, exit 0, no `__pycache__`.
Scratch tier: session evidence, never a receipt (TRUST-01).

The game family is independent of the parent's described suite: three hidden
worlds, two controlled teammate seats sharing one coefficient vector, a
reactive fixed opponent, rational multiplicative-weight softmax (so every
probability, score, gradient and covariance is exact-rational; θ = log r is
never materialized).

| Family | Instances | What is checked (parent §) |
|---|---|---|
| SCORE | 120 | exact score identity via dual-number differentiation (§2/§3) |
| BRIDGE | 48 | grad J = E[Y·Σ score], shared-parameter teammates; baseline invariance; conditional-zero (§3) |
| COV | 32 | fresh expression at weight zero: g_F = Σ reach·Cov(x_F, Q^π), Q^π by forcing then frozen continuation (§3) |
| TELESCOPE | 24 | J(π′) − J(π) = Σ_t E_{d_t^{π′}} f_t exactly (§4) |
| REMAINDER | 24 | \|J′−J−S\| ≤ H(H−1)α² with α the max exact TV over every reachable controlled observation; \|f_t\| ≤ α pointwise; includes the α = 0 degenerate pair (§4) |
| TVSPEED | 3,500 | the L1 bound's engine: ½Σπ\|δ−μ\| ≤ ¼·range(δ) (§4) |
| MF | 8 | decomposition; two-batch estimator unbiased; exact variance split over enumerated independent batches; allocation stationarity by exact perturbation (§6) |
| DISAGREE | 3 | \|J_σ(P) − J_proxy(P)\| ≤ Pr(first differing field action), maximal per-decision coupling, two field decisions deep (§6) |
| PIVOT | 91 | E D = benefit − hazard; Var D = pivotal_mass − (E D)², exhaustive denominator-12 grid (§7) |
| RISK | 36 | α[k,j] partial sums = δ(1−1/(K+1))(1−1/(J+1)) ≤ δ, monotone (§8) |

3,886 exact instances in total. **Not checked mechanically:** the integrated
L1 bound (hand-checked only, see §3), the Hoeffding radius inversion (standard,
hand-checked), and everything in the parent's §10 (its verifier was not
delivered).

## 5. Vocabulary audit

- **"certificate"** — six stem-occurrences, all in negative or fence
  positions ("not … a global certificate", "does not certify",
  "cheap-to-certify", "certification or allocation", "not … population
  certificates"); none names an object. D3 does not bind received text; it
  binds anything built from it. The parent's promotion objects are lower
  bounds and intervals — already compliant naming.
- **"sandwich"** — zero occurrences (FH-A2 clean).
- **θ collision** — the parent's θ is the dictionary coefficient vector: a
  third sense beside the adjudicated θ = pivotal win share and ϑ = auction
  threshold (CE-A2). Proposed repair for any build: the coefficient vector
  is named `coefficients` (or `weights`) in code and prose; the bare symbol
  θ stays reserved. Agenda item 3.
- **"Viewer expression"** — consistent with walt's viewer (the acting seat's
  lawful view) and with `walt::scheme`'s own/public inputs; no collision.
  "Campaign target", "actor", "generation", "promotion" are fresh (grep at
  filing finds no conflicting reserved use).
- **support ≠ belief, feasible ≠ reachable** — respected throughout; §3's
  on-policy sampling paragraph and §7's support line state the distinctions
  correctly.

## 6. Code-boundary audit

- **Exists on main** (`afd46420`): `walt::scheme` — typed relational
  expressions, the 14-clause shared constructor, `PolicyProgram::compile`/
  `choose` over own/public `PolicyInput` (`walt/scheme/`);
  the partnership relational-learning campaign — ordered first-match
  programs, grouped splits, freeze-then-exam
  (`experiments/partnership/RELATIONAL-LEARNING.md`); the exact gym; the
  finite information-price teacher (`walt/scheme/INFORMATION-PRICES.md` —
  note its own tier hazard is already flagged on the index page); the
  solver's forced-root-action branch machinery; the adjudicated CE
  anytime-valid evidence machinery.
- **Not on main**: PR #93 (`experiments/response-ladder/`), which the parent
  assigns the cheap-proxy role. Nothing here may consume it until it lands;
  agenda item 7.
- **Green field** (nothing on main implements them): the weighted softmax
  consumer, the outcome-gradient estimator, the constructor generation loop,
  the promotion ledger, the multifidelity harness.
- **The float hazard, and its proposed discharge.** A softmax needs exp;
  walt's discipline denies floats near probabilities. Discharge: the
  **rational multiplicative-weights parameterization** — store
  r_j ∈ ℚ_{>0} and set π(a|I) ∝ base(I,a)·Π_j r_j^{x_j(I,a)} with binary
  features, so θ_j = log r_j is never materialized; gradient information is
  carried as the exact rational Σ score·Y statistics and updates act
  multiplicatively by bounded rational factors. Every probability, score,
  covariance and promotion statistic is then exact-rational. This is the
  parameterization the intake verifier itself uses, so the identities are
  verified in exactly the arithmetic the build would use. The parent's "fix
  the temperature/scale convention" is discharged by fixing the update
  grammar of r (bounded numerator/denominator — the parent's own
  coefficient-precision bound). Agenda item 4.

## 7. Proposed obligations (candidate; numbering and ledger are Jason's)

- **OG-O1** Every campaign target T is a frozen, content-addressed record
  (the FreezeTuple/PolicyId discipline); every reported number names its T.
- **OG-O2** The deployed actor consumes own-hand and public record only;
  hidden-world predicates are excluded by type, and any public-history
  exposure in the expression evaluator is explicit and versioned.
- **OG-O3** No floats: rational multiplicative weights with bounded
  coefficient precision (§6 discharge above).
- **OG-O4** An argmax conversion or symbolic distillation is a new actor and
  gets new complete-game evaluation — never inherits the incumbent's record.
- **OG-O5** Promotion only by the registered evidence rule; failed or
  unresolved candidates are never relabeled; out-of-compute is UNRESOLVED,
  a typed outcome.
- **OG-O6** The independent physical-deal bundle is the unit of independence;
  mirrored arms and derived branches are components, never extra
  observations.
- **OG-O7** Epoch discipline: seeds, sample counts, lineup identities and
  deadlines are part of every result's identity; epochs do not compose.
- **OG-O8** Retained old-Walt games are never relabeled as current-policy
  on-policy samples.
- **OG-O9** Proxy lineups enter only through the paired-correction estimator
  with a direct-target-only control arm alive in every campaign.
- **OG-O10** Dictionary edits and the deduplication probe panel are
  versioned; empirical agreement on the panel is never recorded as semantic
  equivalence.

## 8. Proposed build program (PENDING adjudication — nothing is authorized)

Proposed home: `experiments/` first (exploratory, results files govern
prose), with gates and any `walt/` residence only where a slice touches
walt code; no default-player change anywhere in the program; gates sized to
laws, one coordinate per law plus a PINNED strictness witness.

- **OG0 — the campaign target.** A frozen target record type (deal law,
  seats, lineup identities with full policy semantics, observation
  semantics, utility = make/set, budgets, deadlines, risk budget δ,
  practical-improvement threshold) plus the registered evidence rule.
  Smallest slice; everything downstream cites it.
- **OG1 — the actor.** Rational relational softmax over a bounded
  `walt::scheme` expression dictionary, consuming ordinary own/public
  `PolicyInput`; one shared coefficient vector may serve both partners, each
  invocation seeing only its own information. Gate: exact score identity and
  probability normalization on pinned fixtures.
- **OG2 — the estimator.** On-policy complete deals against the frozen
  lineup; exact per-trajectory score·(Y − baseline) statistics with
  pre-action baselines; seeded, declared deal bundles. Gate: the
  conditional-zero identity on a pinned bundle; a PINNED nonzero-gradient
  witness. OG0–OG2 is the minimum falsifiable unit: one generation, one
  coefficient update inside the α ≤ ‖Δ‖₁/4 small-step audit, one paired
  before/after evaluation.
- **OG3 — candidate scoring.** Branch Q^π by forcing legal root actions and
  executing the frozen actor (machinery the solver already has), centered
  features, covariance proposal scores for a bounded fresh-expression set.
- **OG4 — the constructor.** Bounded generation over the existing 14-clause
  library plus admissible thresholds; versioned dedup probe panel; block
  edits and retained exploration per the parent's §5 step 6.
- **OG5 — promotion.** Paired bundles, frozen finalists, the α[k,j]
  Hoeffding checkpoint ledger first (swap to the adjudicated CE confidence
  sequences if ruled — agenda item 6); incumbent retention; a final
  untouched exam for the reported learning curve.
- **OG6 — multifidelity.** Paired correction batches against a cheap lineup
  (PR #93's compiled players once landed, or σ0-family tables), pilot
  variance estimation, the allocation ratio, and the direct-target-only
  control arm.

**Dated provenance marker — 2026-09-21, later the same session.** Jason
authorized the build in-session ("my hope is to let you do this your way …
how would you make that happen all the way to running it?") — an explicit
instruction from the same authority this section was waiting on, superseding
the "nothing is authorized" line above for the build only. The instrument
was built at `experiments/og-learning/` (its README declares the campaign
target, the registered evidence rule, and the disjoint seed ranges) and the
first campaign ran to its registered stall stop the same day; the record of
record is `experiments/og-learning/campaigns/og-v1/RESULTS.md` (three
promotions in eight generations; untouched exam: learned 550‰ vs uniform
424‰, paired +125.9‰, exact CI [+83.3‰, +168.5‰] at α = 1/20 — EXPLORATORY,
target-specific). Two declared deviations from the §8 slice sketch: the
fixed 14-clause library was consumed whole as the campaign language (no
constructor-growth phase yet — OG4 remains open), and a generation takes
multiple audited inner steps with snapshot finalists (the parent's §5
steps 5–6 allow both). The parent's adjudication below remains PENDING;
nothing in the campaign record is quotable above walt's exploratory tier.

## 9. Adjudication agenda (for Jason; proposed rulings family OG-A)

1. Accept or decline the intake at instrument tier; disposition of the
   missing §10 verifier pair — retrieve, or declare unfiled (SP-A11
   precedent), in which case §10's counts stay uncited forever.
2. The consolidation ruling of 2026-09-04: does this lineage queue behind
   [[consolidation-slice]], or does the 2026-09-21 delivery amend the
   ruling? (Either way, filing ≠ build.)
3. Vocabulary: the θ symbol split (companion §5); do "campaign target",
   "actor", "generation", "promotion" enter the vocabulary page?
4. The rational multiplicative-weights float discharge: binding for any
   build?
5. Build home and gating: `experiments/` first as proposed, or under
   `walt/` with `walt/ci/check.sh` from the start?
6. Whether §8's promotion rule consumes the adjudicated CE anytime-valid
   machinery instead of the illustrative Hoeffding construction.
7. PR #93's role: register response-ladder compiled players as proxy
   lineups when that PR lands?
8. Obligations: accept/renumber OG-O1..O10 and name their ledger.

## 10. What this companion does NOT establish

No Texas 42 experiment was run; no strength claim exists at any tier; the
parent's §10 suite remains undelivered and unchecked; §5's constructor and
§7's producer placements are design prose, audited for consistency only. A
green intake verifier is evidence the algebra is right, never evidence the
learning program is economical — the parent says the same, and that sentence
travels with every citation of this lineage.
