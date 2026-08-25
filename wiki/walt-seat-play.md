# walt — the seat plays: the scenario-player era

[Home](Home.md) · owns: the scenario-player line of the `walt/` build
(2026-08-17 onward) — the sampling-stack seat, its first games, the arena
match against the E[Q] champion, the level-2 stack, divergence mining, bid
calibration, the tilt audit, and live play in plunge ·
Sources: [`walt/SCENARIO-PLAYER.md`](../walt/SCENARIO-PLAYER.md) (the spec),
result files under [`walt/probes/m3/`](../walt/probes/m3/) and
[`walt/probes/bidcurve/`](../walt/probes/bidcurve/),
[`walt/TILT-AUDIT.md`](../walt/TILT-AUDIT.md),
[`walt/LEVEL2-PROBE.md`](../walt/LEVEL2-PROBE.md). (The solver itself was
built in the `walt-m3-probe` crate, folded 2026-08-24 into
`walt/walt/src/solver/` by the unification.) Related:
[walt](walt.md) (hub and fence), [walt-program](walt-program.md),
[walt-instruments](walt-instruments.md),
[walt-calculated-evidence](walt-calculated-evidence.md) (the 2026-08-24
adaptive-settlement era that instruments this track), [rob](rob.md).

> **Epistemic tier: EXPLORATORY — the hub's fence applies unchanged.** Every
> number on this page is a sampled ESTIMATE or an arena outcome; arena
> outcomes are external receipts about play, never statements about exact
> values; nothing here is quotable above the Ideas tier. The spec's
> obligations ledger (`SCENARIO-PLAYER.md` §10) is the graduation path.

## What happened

After the 2026-08-10 reset froze the m4/S6 compression search, the program
pivoted to Jason's standing direction ([walt-program](walt-program.md)): stop
compressing truth, **build the seat that plays**. In two days
(2026-08-17/18) the sampling-stack player went from first lawful hand to:

- **A playing seat.** Level-1 walt: exact best response over a sampled
  belief (void-conditioned fiber worlds) against a field of modeled level-0
  minds bottoming out in dice. Objective: P(make the bid) — Boolean, exact
  rationals, no floats. Playable at a web table (`webtable.rs`, dominoes,
  clockwise, trump deliberation visible); Jason has played it at length.
  His verdict after a sacrifice he first read as a blunder: "that's a good
  player buddy. a good player."
- **The match.** Via a bridge speaking the rob_bridge protocol (zero arena
  changes; ~15k decisions rules-cross-checked, zero divergences), level-1
  walt played the standing mk5 champion — the E[Q] n=10 lens that had
  beaten rob 6.5σ — under the same dropped-30, 3×384-game protocol.
  **Pooled: walt 630/1152 games (54.7%), McNemar z = +6.28 over 6,015
  paired contracts; every seed's mark-margin CI excludes zero.** The
  signature: walt loses ~4.7 points/hand and wins the marks — the pmake
  objective visible in data. Full numbers, parity audits, and honesty notes
  (a 4-game pilot that pointed the opposite way; one unresolved forensic):
  `arena_results_2026-08-17.txt`.
- **Level-2.** The field model is a parameter: a level-k mind best-responds
  to level-(k−1) minds. Level-2 — the first level whose modeled partner
  coordinates back — was laddered on the frozen carrier: it agrees with
  level-1 at every rung, and its opening lead (5-5, pulling trumps with the
  master) is unique at every sample size that separates ties, perfect at
  n=3200. Cost ≈ 25–50× level-1 per decision. `level2_results_2026-08-17.txt`.
- **The correction the parallel port caught.** Making level-2 parallel
  (≈5.6×, byte-identical across thread counts) exposed a purity bug: the
  modeled-mind cache key omitted the banked totals its objective conditions
  on. Serial runs had masked it as deterministic first-come aliasing. Fixed
  everywhere (post-match, so the 3×384 pool stays internally consistent on
  the pre-fix binary; post-fix play is a new baseline). The incident is the
  standing argument for the spec: a stated invariant, checked at review,
  catches drift that testing hides. `SCENARIO-PLAYER.md` §3.4.
- **Divergence mining.** Rather than a head-to-head to detect where
  partnership modeling matters, 900 self-played hands with a level-2 shadow
  at one seat: 4,156 shadowed decisions, divergence concentrated in tricks
  1–4 and — at large value gaps — in the partner-bid and defense regimes
  (≈2× the self-bid rate at ≥1500bp), matching the prediction that level-2
  bites where "my partner will X" carries the decision. Top mined case:
  level-2 drops a five-count on the partner's winning trump pull where
  level-1 hoards it. Self-graded (level-2's own table); refereeing is an
  open design question. `divergence_results_2026-08-18.txt`, corpus under
  `mined/`.

## Since then (2026-08-19 through 2026-08-24)

- **Race-then-refine.** Applied to the seat as `level1_raced` (common-random-
  numbers block racing with exact binomial elimination — opening leads 745ms
  vs the full 1230ms at 100 worlds vs 40; disagreements are saturation ties
  only) and `level1_race_refined` (survivor ties get the 16× refinement).
  Shipped as a walt-wasm opt-in (`race 1`) with a full-hand conformance test;
  the default path stays byte-identical (Node smoke 28/28 against the frozen
  native trace). An arena gate at bid 30 (24 mirrored deals) found
  race-refined vs full a strength **dead heat** at slower mean decision cost
  in that tie-saturated regime — the racing edge is regime-dependent
  (openings, high bids), so the opt-in posture stands. Exploratory play
  policy, estimates never receipts.
- **Bid calibration.** A 3×200-hand bidcurve corpus (zero died cells;
  predeclared single-look analysis in `walt/probes/bidcurve/`) calibrated the
  bid threshold θ: at n=40 worlds against the n=200 reference, θ=1/2 overbids
  37/200 (the known saturation overbid, now quantified); **θ=11/16 is the
  first rung with 0 overbids and 0 missed bids** and became the walt-wasm and
  webtable default (θ stays a request parameter). n=12 is unfixable by θ;
  trump-declaration choice is noisier than bidding (159/200 n40-vs-n200
  agreement). The solo-auction protocol caveat travels with every number.
- **The tilt audit.** `walt/TILT-AUDIT.md` (smoke run 2026-08-19, under the
  signed-pivotal intake's SP-A rulings): the modeled field found
  deterministic — no tape, scenario = world — so the tilt phase is vacuous
  until a stochastic field exists; trick-6 is pure Case B; strong gaps
  recover at 25 worlds; and hand 0 caught a live discovery-selection error
  (the panel prefers 6-2 where the majority line is 6-5) — an instrument
  catch, not a verdict.
- **Live in plunge (2026-08-23).** Jason played walt at length inside the
  plunge product ("How'd I do? Ask walt" review). Two review specimens — a
  100%-saturation revelation tie and a 40-vs-160-world near-tie flip on a
  count-timing choice — motivated the **level-2 field-swap probe**,
  filed as **spec only** (`walt/LEVEL2-PROBE.md`): field-swap pivotal mass
  (q wakes up when the modeled field upgrades level-0 → level-1) as the
  detector. Deliberately not started — gated on Jason's incoming
  adaptive-sampling mathematics ([[adaptive-sampling-intake]]).
  *(Era context, 2026-08-24: that mathematics landed and was adjudicated the
  next day, the probe spec was amended to a detection layer, and both
  specimens now have owning machinery — see the calculated-evidence note
  below.)*
- **Unification (2026-08-24).** The seat's crates (`walt-m3-carrier`,
  `walt-m3-probe`, and the research stack) were folded into the one `walt`
  crate — pure code motion, trace-identical, wasm smoke 28/28 byte-identical
  — with the seat solver now at `walt/walt/src/solver/`. Freeze-56 was
  re-issued append-only as v2 at the unified layout.

## The calculated-evidence era arrives (2026-08-24)

The adaptive-sampling mathematics landed — twice, same Pro-session lineage,
both adjudicated same-day (CE-A1..A8, L2-A1..A7) — and its §22 build program
executed through step 7 in one day. That work has its own owning page,
[walt-calculated-evidence](walt-calculated-evidence.md); what belongs here is
what it did to *this* track's episodes (all exploratory / instrument-grade,
per that page's fences):

- **The 40/160 near-tie flip is resolved — as a near-tie, not as a winner.**
  The flip mode is exactly what fixed magic sample counts cost; the shadow
  instrument reproduced the phenomenon class beside the live player (4
  exact-route disagreements at small fibers where the live 200/8 estimate
  picked a different tile; 40 honest exact ties reported as ties), and **step
  8 landed the V5 flip repair and per-pair E0 calibration** (PR #31 /
  `e5a5f52`; instrument records at `walt/probes/step8/`, exploratory
  instrument output below every tier, cited by nothing above it). Re-running
  the count-timing shape family on one epoch and one common stream at an
  ascending cap ladder 40/160/640, the V5 law is asserted mechanically and
  **no cap-dependent flip occurs anywhere**: all six family members are
  honestly `Unresolved` at every cap, at q̂ ≈ 0.3 and |τ̂| ≈ 0.01–0.25. So the
  recorded episode was **a near-tie forced through a phone-tier cap**, not
  sampler inconsistency — the old player was never entitled to either answer.
  Three of the four shadow disagreements escalate to the exact winner (h4 d3 →
  2-1, h7 d5 → 6-2, driven h14 d4 → 2-1); h11 d4 (fiber 1750, τ = 11/175) is
  honestly Unresolved at all caps. The **literal** plunge position's seeds
  live plunge-side, so its reconstruction is a filed blocked test waiting on
  [[gran-anchor-reconstruction]]. Numbers and fences:
  [walt-calculated-evidence](walt-calculated-evidence.md).
- **The saturation-tie episode** keeps its protocol (Def 6.3, "look closer");
  the evidence path types such outcomes honestly (`ExactFrozenSet` with a null
  winner is a finding, not a defect).
- **The live player was audited, not changed.** The step-7 session findings
  (playout's standalone PiKey still omits banked totals; one-RNG threading
  through deal + belief sampling in playout/playtable/webtable contra O27;
  playout `all1` information-inconsistent; **walt_bridge clean**) are filed
  on the era page. Per CE-A7/§20.16 the old player remains the default until
  arena and conformance gates justify a change, on Jason's word. *(O27 was
  repaired 2026-08-24 in PR #37 / `23ba1c2` — all three bins now
  domain-separate the deal stream from the per-decision belief streams, so
  session output is record-grade; the §3.4 PiKey copy stays filed, untouched.)*
- **The tables can now seat the controller [CE thread].** `webtable` and
  `playtable` grew `ctrl [cap=N]` seats, and `controller_bridge` speaks the
  same line protocol as `walt_bridge` so plunge/mk5 consume it unchanged
  (PR #37 / `23ba1c2`). The play surfaces are the only part of that delivery
  that belongs on this page; the acting player itself is owned by the
  in-repo register [`walt/CONTROLLER-PLAYER.md`](../walt/CONTROLLER-PLAYER.md)
  and read out on
  [walt-calculated-evidence](walt-calculated-evidence.md). Each play reports
  the route that chose it, and the cap is a **think-time budget** — a low cap
  buys more honest fallbacks, never a wrong settlement. **Nothing here says
  the controller seat plays better than the seat this page describes**: no
  arena run, no conformance gate, no strength number of any kind exists, and
  the default is unchanged.
- **The Gran review specimens became carded anchors.** Three Plunge
  screenshots are pinned with a manifest, and the reconstruction path needs
  **no seeds** — the "How it went" grid is the complete deal, transcribed
  and rules-engine-validated ([[gran-anchor-reconstruction]]). Discovery
  artifacts until reconstructed; the G1–G4 experiments stay gated.
- **`walt/LEVEL2-PROBE.md` is no longer "gated on the intake"** — it is the
  detection layer inside the targeted level-2 controller (L2-A5), and three
  field-swap slices have landed [L2 thread]: the fixed-policy smoke (never
  root-action screening — L2-A4); at PR #30 / `ca0483d`, the exposure rungs
  E0–E2 with the exact split-reach route E4 and the L2-T4 admissible screen;
  and at PR #38 / `151ea4f`, the Part VI cancellation ladder with pairwise
  masses and directional rungs (PANEL-A7/A8). On the h7-t5 root the smoke's
  "the fields never split" observation became an **exact zero over all
  information-consistent continuations**, one root produced the first pruning
  singleton in the wild, and slice 3 added the first `FieldDecisionChanged`,
  the first `FieldStableExactRoot` and the first `Dominated` — all
  frozen-candidate-set statements at a declared field pair, never play-strength
  claims and never "σ1 is a better mind" (O36). Instrument records at
  `walt/probes/fieldswap_screen/` and `walt/probes/fieldswap_cancel/`, read
  out on [walt-calculated-evidence](walt-calculated-evidence.md).

## Where this sits in the program

The seat is real and it wins — and every one of its numbers is a sampled
estimate against a *modeled* field, not an exact value and not an
equilibrium claim (`SCENARIO-PLAYER.md` §7). The program's next moves, per
Jason 2026-08-18 and the state since 2026-08-24: pay down the spec debt (the
obligations ledger, now O1–O9, O12–O38), finish calculated-evidence step 8,
then the level-2 probe as the detection layer
([walt-calculated-evidence](walt-calculated-evidence.md)). [rob](rob.md)
remains the exact-truth solver; walt remains the seat; neither impersonates
the other.
