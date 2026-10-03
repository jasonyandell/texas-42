# Open Problems and Boundaries (Merged)

[Home](Home.md) · owns: the merged OPEN inventory and its current statuses, plus the
fenced list of exploratory-tier open questions · Sources:
both packages `40_CLAIM_STATUS.md` §10 + Math §14 "Honest boundaries", merged per
[package-provenance](package-provenance.md); rec's rewrites of OPEN-01/OPEN-12 adopted;
exchange resolutions per [claim-ledger](claim-ledger.md). Re-synced 2026-09-12
against the repository as of 2026-09-07 (c00717d1).

Three kinds of entry live here, and the kind is always stated. **Genuinely open
mathematics** is a question the corpus poses and nobody has answered at any tier.
**Proved boundaries** are questions answered negatively or scoped, so that nobody
re-opens them by accident. **Exploratory-tier open questions** (last section) are
things the walt program does not know about its own instruments; they are pointers,
never claims, and nothing above the walt fence cites them.

## Genuinely open mathematics (UNRESOLVED)

- **OPEN-11 — the flagship**: the exact cardinality of the strictly Straight-reachable
  support image `R_Str^m` remains open, but the proved standalone interval is now
  **[36,45] bits** (corpus-proved [26,46]; narrowed at the exchange-adjudicated
  evidentiary tier by REACH-17 + REACH-18 — two structurally disjoint certified
  families totalling 36,913,384,410 > 2³⁵ reachable supports — and by REACH-19, the
  follower-supply-filtered outer census 33,297,009,347,414 < 2⁴⁵, the first ceiling
  movement since REACH-11; see [reachability](reachability.md)). The exact census and
  any *full* declaration class remain open — dispatch 006 explicitly disclaims closing
  either. **The no-void sub-slice is RESOLVED at the exchange-adjudicated tier**
  (REACH-20, dispatch 008, panel 2/3 SOUND + 1 UNVERIFIABLE-no-defect, carried as
  such): saturated, exactly **624,892,870**; the derived combined floor is then
  **36,978,961,138** (derived-from-REACH-20, not separately adjudicated; the
  interval is unchanged); the census over void contexts is what remains open. By the
  transport theorem (dispatch 004) the counting DP need only enumerate one pip-trump
  class plus DT and NT — the feasibility window is restated against
  `|R~| = 7·r_pip + |R_DT| + |R_NT|` rather than nine independent classes. Both
  packages still refuse to collapse the exact count by guesswork. rec's symbolic
  support DAG (REACH-16) is the obvious counting substrate; exchange 001/006's
  validated witness-generation + split-zeta upward-closure counter (independently
  checked against brute force) is a reusable substrate for further tightening. See
  [reachability](reachability.md) and [FINDINGS](FINDINGS.md) Q1.
  *Evidentiary note (2026-09-12).* The floor's first component (REACH-17) and the
  x:002/004/005 results are backed by rob `x-` receipt lines (S7–S10,
  [verification](verification.md)). **REACH-18, REACH-19 and REACH-20 — the results
  the [36,45] box actually rests on — are backed solely by the exchange programs**
  (`programs/006.py`, `007.py`, `008.py`; 006 and 007 re-run green 2026-09-13 with
  their recorded PASS counts, 008 not re-run — its 71.8 s exceeds the pass's 60 s
  budget); rob has reproduced none of them. Their reproduction in Rust is the
  named slice-03 target ([rob-slices](rob-slices.md)), and rob's second brief closes
  with "Do not begin slice 03" (`rob/BRIEF_SLICE_02.md` §13) — so the next receipt
  for OPEN-11's interval has an owner and no start date. Until it exists, quote the
  interval with its tier label, never as corpus-proved.
- **OPEN-01 (rec form) — RESOLVED, COLLAPSE** [exchange-adjudicated CONFIRMED
  (ALL_PASS 0.43s; 3/3 SOUND) — external tier, not a kernel proof
  ([claim-ledger](claim-ledger.md))]: the reduced viewer kernel `K = (δ, H_m, N, τ, α_U)` is proved
  *exact*, and the global transition minimum is *defined* (future-equivalence quotient
  per output contract, QUO-10). Dispatch 003 settles the equality question **negatively
  for the support-aware `P30_DECLARING_POINTS` contract**: `K` is strictly finer than
  the future-equivalence quotient — two reachable kernels differing only in raw fold
  ordinal (`r=7` vs `r=6`; NT, `P(30)`, viewer 0 / bidder 3) are machine-verified
  output-equivalent, so `r` is not an injective memoization coordinate. Mechanism: the
  dead-cut lemma. Witnesses `K₁/K₂`; source
  `exchange/inbox/003-kernel-vs-future-quotient.md`, verified program
  `exchange/adjudication/programs/003.py` + `witnesses/003.json`. See
  [reduced-viewer-kernel](reduced-viewer-kernel.md). The witness is rob slice 03's
  required regression, so it too waits on slice 03.
- **OPEN-12 (rec form)**: no closed-form *support-only* reachability criterion is
  known. Symbolic public-trace replay is exact and removes the hidden-deal
  requirement (REACH-14/15), but still carries legal ancestry.
- **OPEN-02/03**: minimal retained continuation record, minimal latent field-state
  representation, and minimal utility residue for arbitrary history-dependent
  fields/utilities — not established.
- **OPEN-04**: no general low-dimensional exact strategic quotient beyond the proved
  gauges (C₄/D₄, slot, `2↔3`, unscored transports) and the fixed-output future
  quotient.
- **OPEN-05**: extension of the cell/normal-form/reduced-kernel theorems to special
  contracts (nello, plunge, …) — structurally out of scope; nothing transfers
  automatically.

## Proved-boundary items (BOUNDARY — resolved negatively or scoped)

- **OPEN-06**: unrestricted native counting is cheap (CELL-10H/I); extensional
  enumeration, arbitrary predicate-restricted counting, and variable-seat systems
  keep separate computational boundaries.
- **OPEN-07**: off-path beliefs require an explicit assessment (Bayes undefined).
- **OPEN-08**: no canonical sampler from support alone — proved nonuniqueness
  (CELL-10D).
- **OPEN-09**: deterministic best response for arbitrary infinite private-signal
  models needs measurable-selection assumptions.
- **OPEN-10**: finiteness alone does not give effective exact algorithms for
  noncomputable utilities/operators.
- **Match horizon**: repeated all-pass attempts are unbounded without a termination
  assumption; almost-sure termination holds under a uniform per-attempt contract
  probability ε (geometric tail, `E[attempts] ≤ (2T−1)/ε`) [AUC-06/06A, MATCH-02].
- **No universal byte/runtime minimum** without a named cost model (CELL-29); no
  output-independent minimal "game state" (rec Math §12.10).

## Questions this wiki adds (not in either package)

1. **Gauge-reduction of the reachable census — RESOLVED (affirmative)**
   [exchange-adjudicated CONFIRMED (ALL_PASS 4.6s; 3/3 SOUND) — external tier]:
   **yes** — the unscored pip-trump transports biject reachable support images across
   the seven pip-trump declarations, so the REACH-11 declaration factor collapses
   from nine classes to **3** (one pip-trump class, doubles-trump, no-trump). The
   transport theorem `f_{t,u}(R_t)=R_u` is machine-certified (dispatch 004,
   `exchange/adjudication/programs/004.py`): the transport commutes with
   legal-prefix generation. Tagged census `|R~| = 7·r_pip + |R_DT| + |R_NT|`; the
   Step-15 quotient corollary's cocycle gap is closed by an in-house finite check
   (`programs/004-cocycle.py`, Claude-authored, all 343 ordered triples). The
   question as originally posed: do the unscored pip-trump transports (rec ALG-22)
   biject reachable support images across the seven pip-trump declarations? Support
   content depends only on unscored mechanics (follow/lead relations), so the
   expected answer was yes, and it is. See [reachability](reachability.md) and
   [FINDINGS](FINDINGS.md) Q4. Kernel status: none — PA-A16/A17 prove the `2↔3`
   transport of the algebra, not commutation with reachability.
2. **Outer-language tightness — RESOLVED (negative)** [exchange-adjudicated CONFIRMED
   (ALL_PASS 0.9s; 3/3 SOUND)]: the witness `(NT, (6,6,6), V₁={6}, 18-tile pool)` passes
   **all four** outer checks (capacity shape, schedule, lead witness, Hall) yet is
   unreachable — so the outer language is *not* exact even at the `j=1` equal-capacity
   one-void phase. Exhaustion: 450 generators, 3 matches, 425,520 traces, 0 realizers.
   Source `exchange/inbox/002-outer-language-tightness.md`, verified program
   `exchange/adjudication/programs/002.py`; rob reproduces the exhaustion in
   `verify_unreachable` (`x-r_unr_002_*`). (Unlike REACH-10, which *fails*
   lead-witness, this witness passes it; the newly established follower-supply
   obstruction is the fifth necessary condition — see [reachability](reachability.md).)
3. **Minimality of the 90-world witness**: is 90 the smallest fiber exhibiting a
   same-support posterior action flip under all four lenses, or does a smaller legal
   witness exist? (The witness *itself* is now proof-assistant kernel tier — PA-E10,
   `ninety_world_witness`, 2026-08-02 — which settles that it holds, not that 90 is
   least; minimality is untouched. See [proof-assistant-plan](proof-assistant-plan.md).)
4. **Will C1 be stated in Lean?** The constellation suffix factorization (x:009,
   PARTIAL, proof chain step-checked at the external tier) was the reason the Lean
   thread was opened; x:011 refused to fabricate a build, x:013/x:015 delivered a
   sorry-free self-contained core and exact suffix minimax, and neither file states
   C1 or any factorization theorem. The two files are also unreconciled with the main
   spine. Owner: [lean](lean.md); ledger row: [claim-ledger](claim-ledger.md) (Lean
   thread). No plan is recorded.
5. **Kernel coverage of the weakest prose spots.** TRANS-08/09 (support-NF dynamic
   sufficiency, matching-minor ≡ conditioning) and PLAY-17 (reduced-kernel
   sufficiency) are the two places [FINDINGS](FINDINGS.md) §6 flags as prose proof +
   small-domain receipt; both are rec-only rows with no mechanization-ledger row at
   all, so no Lean work is even queued for them. Whether the v0.8 merge should add
   mechanization rows for rec's mathematics is undecided.

## Exploratory-tier open questions (walt; pointers only, nothing here is a claim)

> **Fence.** Everything in this section is EXPLORATORY — below corpus, kernel,
> exchange and rob tiers alike. These are questions the walt program has recorded
> about its own instruments, listed here so a reader of the ledgers can find them.
> Each entry names its owning page and its record path; **no number below is a
> result**, every number is a probe record or a session status quoted for
> identification, and nothing above the walt fence cites this section. Where the
> record itself says "not run", "unmerged" or "unrecorded", that is the status.

- **Does a cost-matched partner model add strength over level 1 under a wall
  clock?** The partnership program's stated open question. Its 400-game default
  battery (100 mirrored deals, bid 30) recorded L2 Partner vs L1 at 14/14/72 and L2
  Partner-with-voids vs L2 Partner at 12/17/71 (wins/losses/ties, first-named
  player), with fixed L2 Partner costing about five times default L1; every
  race/refine partner configuration crossed the >5% fallback gate. No strength gain
  is established either way, L1 remains the operating default, and profiling the
  partner-field decisions is the first open item with no experiment scheduled.
  Owner: [walt-partnership-program](walt-partnership-program.md); records
  `experiments/partnership/campaigns/default-partner-battery/01-level/MATCH.md`,
  `…/02-voids/MATCH.md`, `experiments/partnership/SESSION-STATUS.md` ("What remains
  open, in order", items 1–2), `experiments/partnership/PLAYERS.md`.
- **Is the 420602 reversal sampling noise or model mismatch?** On deal 420602 the
  deeper player turned a set into a made contract for the defence — one world in
  forty — and `REPORT.md` cannot say whether this is n1=2 inner-sampling noise or a
  stable mismatch between the modeled partner and the executing phone policy. The
  preregistered stability panel it proposes (independent public tapes, n1=2 versus 4)
  was never run. Owner: [walt-partnership-program](walt-partnership-program.md);
  records `experiments/partnership/REPORT.md` ("What remains and the next small
  experiment"), `experiments/partnership/campaigns/random-420600-699/RESULTS.md`.
- **The objective and the tie-break at exact indifference (Gran G2).** The made
  hand G2 is exactly locked from trick 3 — every one of its 280 trick-4 deals makes
  whatever the seat plays, at level 1 and level 2 with identical tie sets — so pmake
  pins at 1 and has no gradient; play then falls to `TieRule::LowestTileIndex` plus
  `best_of` keeping the incumbent, which deterministically hoards the highest-index
  count tile (the 6-4 is index 25 of 28). This is a source reading, never executed
  at a tied node: no harness drives `solver::act` at a constructed information state.
  The 2026-09-05 readout concludes another level is not the remedy — the levers are
  the objective (what replaces pmake at exact indifference) and the tie-break — and
  that neither is a feature; the design call (C) is Jason's and is pending. The
  level-2 runs live on the unmerged branch walt-g1-l2. Owner:
  [walt-gran-anchors](walt-gran-anchors.md); records
  `walt/briefs/MORNING-2026-09-05.md` items 1, 2, 4 and calls (A)–(F),
  `walt/probes/gran/`.
- **Does void-aware inner belief pay at the table (obligation O5)?** O5 is "the cost
  of the no-void inner simplification — measurement" (`walt/SCENARIO-PLAYER.md`
  obligations table). Every measurement so far is unresolved and non-composing: the
  foundation battery's voids-vs-voidless pairs were 9/5/36 on 50 random deals and
  5/8/37 on five focal hands × ten completions; the default battery's L2-voids vs L2
  was 12/17/71 over 100 mirrored deals; the O5 rerun on branch walt-o5 (unmerged)
  reads +262 of 6,048 at the live epoch and −226 of 16,128 at the reduced epoch,
  "dead heat" withdrawn, "correctness case strong, play case unmade", flag off. The
  voids-counted inner sampler also uses a different deterministic stream from the
  voidless one, so no result isolates void conditioning from stream change; a
  coupled-world ablation is unbuilt. Two implementations exist (walt-o5's
  `Level0Field::void_aware`; main's `InnerBelief` `voids-counted` strategy via
  `Kernel`/`FiberDp`) and which goes forward is undecided. Owner:
  [walt-gran-anchors](walt-gran-anchors.md) (with
  [walt-partnership-program](walt-partnership-program.md)); records
  `experiments/partnership/INNER-BELIEF.md`,
  `experiments/partnership/campaigns/foundation-battery/05-voids/MATCH.md`,
  `…/07-worlds/MATCH.md`, `…/default-partner-battery/02-voids/MATCH.md`,
  `walt/briefs/MORNING-2026-09-05.md` item 5.
- **The gym's untaken 433 and its unpublished defending side.** The bid-making-v1
  collection (433 declaring-side exercises with exact, thrice-audited answer keys,
  tricks 5–6, bid 30) has never been taken by any pupil — "this pupil comparison has
  not been run" — so no optimal-choice rate or mean root regret exists for L1, L2
  Partner or L2 Partner-voids on it. The 460 defending-side (set-30) strict keys were
  computed in the same discovery but never published; they exist only in the raw run
  under `/Users/jason/data/texas-42/bid-making-v1` (outside git), and
  `gym.py select --side defending` would publish them without new solving. Owner:
  [walt-gym](walt-gym.md); record `walt/gym/BID-MAKING.md`.
- **The divergence referee protocol has never been run.** The level-2 divergence
  mining of 2026-08-18 (900 deals, one seat shadowed by level 2 at every real
  decision) is self-graded on level 2's own value table; its header states that
  which mind is *right* "needs the mirrored-replay referee, not yet run". No referee
  run exists as of c00717d1. Owner: [walt-seat-play](walt-seat-play.md) (the level-2
  question); record `walt/probes/m3/divergence_results_2026-08-18.txt` (header) and
  its corpus under `mined/`.
- **G2/G3's exact deal, and G4.** The made hand was transcribed as a six-trick partial
  whose residual is six-way ambiguous; the S2 information set is fully known, but
  which of the six residual assignments is the real deal is undecidable from the
  screenshot, so a driven whole-hand G2 run and a `replay_hand` validation wait on
  the plunge side recovering the seed. G4 — the game Jason remembers, Gran locked at
  100s while holding the 6-4 — was never captured and does not exist as a record;
  the synthetic lock is explicitly never G4 (call (F)). Owner:
  [walt-gran-anchors](walt-gran-anchors.md); records `walt/probes/gran/`,
  `walt/briefs/MORNING-2026-09-05.md`.
- **The ladder policy store (19.4 GB).** The focal-horizon ladder copies a full
  policy table into every ancestor's lower fact (the FH-int requirement that every
  lower carries its witness); at h8-t3 that is 3.82M facts and 19.4 GB peak RSS, and
  the anchors gate peaks at 17.8 GB. The FH4 audit (N8) found two sinks — the fact
  store and the memo's `FactorBelief` clones — and rules "measure which dominates
  before fixing". Nothing has been measured; the card's done-when is under 2 GB at
  h8-t3 with every gate and the h8-t3 record byte-identical. Owner:
  [walt-focal-horizon-era](walt-focal-horizon-era.md); record
  `kanban/backlog/ladder-policy-store.md`, `walt/briefs/FH3-REPORT.md`.
- **The GPU track's standing debts: M3 gate unbuilt, M4–M5 untouched, Lean
  correspondence debts.** Freeze 57 (GT1-A24) authorizes only the M3 gate and
  records no M3 result; the M3 production crates (walt-m3-net, walt-m3-oracle-a,
  walt-m3-metal) were deleted on 2026-08-24 as unbuildable WIP, and the surviving
  `walt/walt-metal/shaders/02_m3_wavefront.metal` and `build_m3_metallib.sh` have no
  consumer and no committed metallib. M4 (the representation-growth gate) and M5+
  (stopped controller, gluing, opening attempt) of `walt/GPU-NATIVE-TRICK1.md` have
  no design beyond the contract's own sections. On the Lean side the trick-1 modules
  are a foundation, not implementation refinement: "Rust-to-Lean, Metal-to-Rust,
  general independent-oracle correctness, and grade-4-to-trick-1 transport remain
  explicit correspondence debts; executable parity does not silently promote any of
  them" (`lean/README.md`). The whole track is parked on Jason's word
  (`kanban/backlog/gpu-level2.md`). Owner:
  [walt-gpu-native-trick1](walt-gpu-native-trick1.md).
- **The v2 M2 receipt re-earn.** The standing M2 Metal parity receipt
  (`walt/receipts/gpu_native_trick1_m2_v1/`) attests the freeze-56 v1 build identity,
  the pre-unification layout, and is carried forward explicitly labelled old-layout
  evidence (FZ-A3). Re-earning it under the v2 identity — the 614-task carrier, run
  twice, on hardware, via walt-m2-runner, filed beside and never replacing the v1
  receipt — waits on the track's unparking; walt-m2-runner's own live status is
  inferred, not confirmed. Owner: [walt-gpu-native-trick1](walt-gpu-native-trick1.md);
  records `kanban/backlog/m2-receipt-reearn.md`, `kanban/backlog/m2-runner-trace.md`,
  `walt/GPU-NATIVE-TRICK1-M2.md` §13.
- **The three focal-horizon questions to Pro.** After building the focal-horizon
  hierarchy whole in a day and finding that at k ≥ 1 the residual width is the tail's
  policy gap rather than the fusion price, the 42 team's letter asks Pro three
  things, in the order they would help: (1) **the tail** — what is the cheapest lawful
  tail that closes most of the policy gap, and is there a monotone tail-improvement
  ladder on the lower side that converges to Q faster than k does; (2) **the field's
  sufficient statistic** — which coordinates of the public record a level-0 modeled
  mind's decision actually depends on, since a small statistic would make every
  recursion ten to a hundred times cheaper without changing a value; (3) **the live
  decision at tricks 1–3** — whether "tail + k = 0 or 1 interval + certified regret"
  is the right live decision and what its honest guarantee to a player is. The
  letter is a draft for hand-ferry dated 2026-09-04, not a courier dispatch (the
  exchange ledger is untouched); whether it was sent, and any reply, is unrecorded
  in the repository as of c00717d1. Owner:
  [walt-focal-horizon-era](walt-focal-horizon-era.md); record
  `walt/briefs/FH-RESPONSE-TO-PRO.md` §4.
- **The Kiln calibration gap: why did the model forecast 121/160 for a hand the
  table made 21/100?** (Added 2026-09-20, cycle 1.) The scalar-price survey's one
  calibration check — a sixes/36 hand selected before any outcome was played —
  forecast 121/160 = 75.625 % and made 21 of 100 fresh hidden completions when the
  deployed L1+partner player actually played all four seats (Wilson 95 %
  14.17–29.98 %); 1,808 decisions were replayed through Plunge's engine. The
  diagnostic follow-up confirmed two structural mismatches from source (the
  forecast prices against modeled level-0 seats while actual seats used the
  deployed L1 player and partner review; the forecast optimizes over its original
  sampled worlds while actual play samples anew at each decision) and stops there:
  "This policy mismatch is a candidate explanation, not a cause established by the
  experiment"; "The causal contributions remain unmeasured". The isolation it
  proposes (held-out completions with the deployed bidder against the same modeled
  field, then changed partner/opponent policies) has not been run; the survey was
  frozen and replaced by the actual-play book. Owner: [walt-kiln](walt-kiln.md)
  §1.4, §6; records `experiments/kiln/CALIBRATION-SIX36.md`,
  `calibration-six36-summary.json`, commit `466db3f1`.
- **Should a bounded partner check override L1 on sampled evidence at all?**
  (Added 2026-09-20, cycle 1.) The partner rollout's one fresh harm: at the 500 ms
  deadline 52 complete paired worlds favored withholding 34–33, the full 210-world
  census favored offering 140/210 to 139/210, and the same shuffled stream at 64
  worlds reads 41–40 for offering — "the budgeted prefix happened to stop while
  the small difference pointed the other way." The override study (require one,
  two or three extra sampled successes before changing L1) keeps margin one on
  development regret (0.0608 / 0.0627 / 0.0740 pp) and says "The small
  differences do not settle an optimal threshold." Every ordinary-game panel
  tied, so no arena evidence bears on it. Owner:
  [walt-partnership-program](walt-partnership-program.md) §11.3, §11.5, §9 item
  10; records `experiments/partnership/campaigns/sunshine-rollout-v1/RESULTS.md`
  (`summary.json` `prefixes`), `campaigns/sunshine-playable-v1/override-study.json`.
