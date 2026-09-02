# BRIEF-MB1 — the model-belief recursion joins the solver (M1+M2 merged)

**Authorized:** 2026-09-02, Jason's §76 GO ("join if possible. an honest
attempt") and overnight build grant. **Binding theory:**
`walt/math/model_belief_base_player_v0.1.md` §§21–33 (the model-space
recursion, merge-before-max in model space §32, the §33 sparse
counterfactual frontier), §§16–20 (response vectors, sep upper), under
rulings MB-A1..A8 and the intake companion's corrections. MB0
(`solver/model_belief.rs`, PR #82) is the foundation — extend it, never
fork it. Read `walt/FACTOR-BELIEF.md` first (MB0, sigma1-repair, and U0
paragraphs are the newest state), then `walt/briefs/U0-REPORT.md` for
the census findings this brief inherits.

## Mission

Take MB0's exact mixture machinery from "evaluated at roots" to "runs
inside the recursion": the model-belief posterior maintained down the
walk, mixture values computed by the §23-style factorized recursion over
Ξ rather than per-root enumeration, on the receipt corpus plus earlier
(pre-t4) roots where affordable with honest refusals where not.

Jason's official framing governs root selection: "42 is 2 recursions
running in opposite directions" — late tricks are enumerable and hold no
model uncertainty (MB0 criterion 4's censused absence, now measured by
U0: the fusion horizon sits at trick 5 on the receipt corpus, and the
whole t4 gap is information price, d_policy = 0). The model-belief
physics lives earlier. MB1's job is to reach earlier honestly.

Build items (restructure freely if the math says so — report deviations):
1. Posterior-carrying recursion: thread the (H,θ) factor posterior
   through the existing solver walk (proof_state facts + derived views,
   never stored duplicate state). Theorem 12.1 updates at each observed
   action; merge-before-max in model space (§32) structurally, as MB0
   did at roots.
2. Response-vector caching (MB0 flag 4): MixtureOutcome already carries
   per-profile response vectors — reprice any ν by dot product (§16/§23)
   instead of re-walking. Gate the identity.
3. Adopt the positive-support tightening for EVERY sampled mind inside
   exact recursions (MB0 flag 3) — the repaired sampler
   (`solver::sample_belief`, typed refusals) is the fallback, tightening
   is the default. No unbounded walks anywhere.
4. Budgeted F₁ reads (MB0 flag 2): ~1.5–2 ms/read, zero cross-history
   reuse — declare the budget per root in the probe; refuse typed when
   exceeded (§34/§35 style).
5. Earlier-root probe: at least two pre-t4 receipt-root coordinates
   (affordability permitting) evaluated under the registered F₀/F₁
   mixture — the first place Φ > 0 COULD appear. Finding either way:
   strict fusion-price specimen, or the censused absence extended
   earlier with exact evidence.
6. Non-product priors are already accepted by from_profile_prior (MB0
   flag 5) — do not build correlation machinery, but do not break the
   interface either; one gate keeps it honest.
7. **Field-identity fence (U0's flag, MANDATORY before any transport):**
   God-tightness and every doom-derived bound is field-SPECIFIC (SC-A7
   strictest class) — an equality against a doom upper computed under
   one declared σ0. If MB1 lifts any fact to Ξ = Ω×Θ or reads a
   fixed-field authority inside the model-space recursion, the boundary
   crossing needs an explicit coupling check gated on THIS side: a fact
   consumed across a field identity must either carry a matching
   field_id or arrive through a typed coupling proof. Gate the fence
   even if (especially if) MB1 never actually transports — the gate is
   what keeps the first future transport honest.

## Gates (`walt/walt/tests/solver_model_belief_recursion.rs`)

- M1 recursion-vs-enumeration parity: the posterior-carrying walk equals
  MB0's per-root enumeration on the full MB0 corpus (values, posteriors,
  actions), exact.
- M2 response-vector repricing identity: dot-product reprice equals the
  full walk for swept ν on a rational grid.
- M3 point-mass collapse inside the recursion: δ endpoints reproduce
  fixed-field authorities at depth, not just at roots.
- M4 budget refusals typed and honest; no silent truncation.
- M5 MB0's eight gates + sigma1-repair's seven + U0's six stay green
  untouched.
- M6 the earlier-root finding gated: whatever Φ result appears at the
  pre-t4 coordinates, pin it (strict specimen or censused-absence
  extension).
- M7 the field-identity fence (item 7): unconstructible-by-API where
  possible, gated where not.

## Probe (`modelbelief_recursion_report` → `walt/probes/factor_belief/modelbelief_recursion_run1.txt`)

Per root: depth reached, posterior evolution summary, mixture value vs
MB0 root value (must match), F₁ read count vs budget, wall time; the
earlier-root Φ table; findings language only.

## Discipline

Unchanged house rules: check.sh green; no floats; vocabulary greps;
EXPLORATORY tier; checkpoint-commit per item, never squash; final commit
`walt MB1:`; no push/PR (orchestrator lands); ambiguity protocol;
FACTOR-BELIEF.md paragraph; independent audit follows (mb0-builder-3).
Never touch ingest/, refine.rs (freeze 58), doom.rs (§47), and now
godgap.rs is consume-only the same way doom.rs is (U0's census is a
frozen instrument for you — extend beside it, never inside it).
Report progress and your final report via the SendMessage tool ONLY —
plain final text is never delivered across sessions. Also commit your
full report as `walt/briefs/MB1-REPORT.md` before declaring done.

## Report back

Slice status; gates; the earlier-root Φ finding (THE number of the
slice); recursion overhead vs fixed-field walks; what the unified player
needs from this module (API shape, budget knobs); MB2 flags.
