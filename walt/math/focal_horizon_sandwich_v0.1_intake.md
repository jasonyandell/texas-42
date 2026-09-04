# Intake companion — The Focal-Horizon Hierarchy (parent title: "The Focal-Horizon Sandwich")

**Parent (verbatim, never edited):**
`walt/math/focal_horizon_sandwich_v0.1.md`
SHA-256 `892bc343f1ada12013b2bbd674d46962bc0256a55170aa48075fea17c2592f04`
(pinned by the `.sha256` beside it; re-hashed at intake, matches).
Companion verifier (shipped beside the parent, filed verbatim):
`walt/math/verify_focal_horizon_sandwich_v0.1.py`
SHA-256 `7700a35ec0a718ca674b8a56f6fec87f0bbeab6dbe9f11d9f9ec0d28881ee34b`
(re-hashed at intake, matches).

**Status of this document:** exploratory intake record, maintained under
dated provenance markers. The verifier is **scratch tier**: session
evidence, never a receipt, never imported into the codebase (TRUST-01).
This companion **governs wherever it repairs or narrows the parent**
(the DS-A17 citation rule). **Every statement in this document is
EXPLORATORY tier.** Nothing is promoted by being received, audited,
proved here, or indexed; a proof written here is a proof at the
exploratory tier until a kernel or a gate carries it.

**Provenance:** a Pro deliverable, hand-delivered by Jason 2026-09-04.
Not a courier dispatch — no number, the exchange ledger is untouched
(the calculated-evidence precedent). The upload arrived as
`DESIGN-walt-focal-horizon-sandwich-v0.1.md` with
`verify_walt_focal_horizon_sandwich_v0_1.py`; filed under the
`walt/math/` snake_case convention — recorded, not repaired. Jason's own
framing: "less 'I have a bunch of proofs let's gooo' and more 'here is
an idea to try with measurements'"; "I'm sure there is good math in
there. I'm not sure the tasks are as focused as usual." Intake and
walt-math review by one agent under `walt/briefs/BRIEF-FH0.md`
(2026-09-04). Rulings **FH-A1..A11** in `walt/CENSUS-RULINGS.md`.

**Thread labels (CE = sampling depth, L2 = model choice):** fixed-field
**L2-thread mathematics with no sampling anywhere in the hierarchy**.
The parent says so itself at §4 ("prefer an exactly evaluable lower
tail so the hierarchy itself contains no sampling risk") and §22 (the
exact-mass form). No CE machinery is consumed. The Ω×Θ lift (§27) is
deferred by the parent and **stays deferred** (FH-A2, FH-A10).

**Snapshot check:** the parent declares its basis as merged PR #87
`a80b98291a60c5fda22d44f528efc16016c53425`. Verified: that is exactly
`main` at intake (`git rev-parse main`, the PR #87 merge
"walt UP1a + U0b"). Every Part I empirical citation was checked against
the committed records — see the table below; all match; two are
restated loosely and are sharpened here.

## Adjudication receipt (what was actually re-run)

- **Verifier executed twice** (stdlib Python 3.12.13, every probability
  an exact `fractions.Fraction`; no floating-point type appears): once
  from `walt/math/` (the filed location) and once from the upload
  staging directory (`scratchpad/intake/`, where the upload pair
  `DESIGN-walt-focal-horizon-sandwich-v0.1.md` /
  `verify_walt_focal_horizon_sandwich_v0_1.py` re-hash to the pinned
  `892bc343…` / `7700a35e…` — the filed copies are byte-identical to
  the upload). Exact output both times, exit code 0 both times:

  ```
  24 CHECK FAMILIES
  EXHAUSTIVE TERMINAL SYSTEMS: 4096
  EXHAUSTIVE LOWER-TAIL CASES: 32768
  ALL CHECKS PASS
  ```

  No `__pycache__` was created by either run (the script imports only
  `fractions`); `walt/math/` was checked clean before commit.
- **The "24" is a printed label, not a count.** The line
  `print("24 CHECK FAMILIES")` is a literal. By count the script holds
  **31 `assert` statements in 18 comment-labelled blocks**: nine inside
  the exhaustive sweep and nine post-sweep specimens. **Never quote the
  parent's verifier as "24 theorem verifications."**
- **Load-bearing checks (the sweep, 4,096 Boolean payoff systems × 8
  lower-tail policies = 32,768 cases, a three-world two-focal-layer toy
  with one public partition between the layers):** (1) `Q ≤ U₁ ≤ U₀`
  and `U₂ = Q` — Theorem 2 and Theorem 4 on the upper side; (2) `U₁ =
  max_a Pr(S_a)` — Theorem 5; (3) action-indexed `Q_a ≤ U_{a,0}`,
  `U_{a,1} = Q_a ≤ U_{a,0}` — §16; (4) `L₀ ≤ L₁ ≤ Q` and `L₂ = Q` —
  Theorems 1 and 4 on the lower side; (5) the k = 1 lower replays as
  one information-consistent policy (`eval_policy(a0, a1map) == L₁`) —
  §7's materialization claim / gate FH5; (6) the exact policy replays
  to `Q`; (7) the root-action sandwich `L_{a,0} ≤ L_{a,1} = Q_a ≤
  U_{a,1} ≤ U_{a,0}` — Theorem 3 action-indexed; (8) `S₁ ⊆ S₀` and the
  exact maximizers survive both — Theorem 6 and gate FH4; (9) `0 ≤ Q −
  L₁ ≤ U* − L₁` — §19. These nine are genuine theorem checks on the
  toy.
- **Two genuine strictness witnesses, kept in view as specimens (not
  sweeps):** "public branches do not consume focal horizon" exhibits a
  system where a globally shared second action strictly loses
  (`< 1`) while per-observation choice attains 1 — §6's design choice
  has teeth; "merge-before-max" exhibits worldwise max strictly above
  the lawful value — the O34 fence, gate FH6's specimen in miniature.
- **Weak checks (definitional illustrations on literals, NOT theorem
  verifications):** "scalar optimum equality does not imply
  selected-action safety" (two hardcoded dicts and a tie-rule pun);
  "exact-mass hidden-branch parity" (arithmetic on three literals);
  "Bellman supersolution condition is load-bearing" (asserts
  `1/2 < 3/4`, nothing more); "exact suffix substitution is
  compositional" (asserts an expression equals a byte-identical copy of
  itself — it verifies nothing); "partial interval propagation" and
  "nonexpansiveness / gap propagation" (hardcoded rationals); the
  "interval separation specimen" (a loop over a hand-picked grid —
  §18's logic in miniature, but on literals). Recorded so nobody
  quotes the specimen count as evidence for §22, §23 or §25: **the §23
  interruption rule and the §25 substitution theorem are not exercised
  by the sweep at all**; they are proved below by hand.
- **What the toy cannot see (recorded, not held against it):** no
  field node before the first focal decision (so the identity
  `U₀ ≡ G` through hidden nodes — Lemma 2.0 below — is never
  exercised); no decided-early cutoff; no forced actions; two focal
  layers only; no interruption; Boolean utility only. The FH1 endpoint
  gates and FH-A5's parity gates cover exactly the first of these on
  real roots.
- **Theorems step-checked sound, with full proofs supplied below
  (§ "Proofs"):** Theorem 1 (§9), Theorem 2 (§10, with the hypothesis
  discharged for the God tail by Proposition FH-God), Theorem 3 (§11),
  Theorem 4 (§12), Theorem 5 (§13, Boolean utility), Theorem 6 (§17)
  plus the exact-best-survives corollary the gates need, the §18
  exact-action criterion and its tie form (Proposition FH-tie), §19
  certified regret and its monotonicity under a **stated** meaning of
  "preserved facts", §22 exact-mass form, §23 the interruption rule
  (**the parent gives no proof; one is supplied**, Proposition FH-int,
  with the intersection discipline it needs), §25 continuation
  substitution with a **sharpened** identity list. Two further
  propositions delivered because the record demanded them: FH-cut
  (ply cut ≡ focal cut on viewer-lead roots) and FH-last (the forced
  last trick collapses the hierarchy one layer early).
- **Transcription errata:** none found. The parent is clean in
  transit; the only repair this companion makes to its text is
  vocabulary (FH-A2) and the narrowings listed under "Verdicts by
  part."
- **Vocabulary sweep:** no bare "certificate" (six occurrences of
  "certified regret", the APS term of art, all compatible); one
  occurrence of "floating-point", as a prohibition (§22), no float
  type named; "sandwich" seven times including the title — ruled at
  FH-A2 exactly as CBS-A3 ruled it for the sibling: **not a citable
  object name.** "God upper / God-tight" retained (SC-A4). "Strategy
  fusion", "information-consistent", "lawful", "focal" are the house
  words and are used the house way.

### Part I citations against the committed record

| Parent | Claim | Record | Verdict |
|---|---|---|---|
| §1.1 | 216-decision corpus; lazy ≡ eager on actions, evidence, refusals, frames, join readings; lean rung's unread posterior work gone | `walt/briefs/UP1A-REPORT.md` gate UC5 (216 = 72 decisions × 3 rungs, `unified_run2.txt`); lean carry 2,105,672 µs → 0 µs | MATCH |
| §1.2 | uniform trick-5 receipt roots looked God-tight; conditioned trick-5 nodes inside trick-4 solves carry positive price on substantial subsets | U0: fourteen t5/t6 coordinates God-tight; `horizon_run1.txt` cut 4 at bid 30: 40/466 (h8-t4), 171/779 (h3-t4), 384/1228 (h4-t4) positive-gap nodes, mass-weighted 13–14‰ | MATCH |
| §1.3 | a few permille of root-value error changed the root action | h8-t4 cut 8, bids 36 and 39: over-pricing 3/400 (7‰), exact argmax 2-1, cut argmax 5-5 | MATCH |
| §1.3 | the exact h8-t3 anchor: a coarse optimistic cut selects a different action | h8-t3 bid 30 cut 4: exact 28859/29988 (962‰) argmax 1-1; cut 29803/29988 (993‰) argmax 3-3; over-pricing 236/7497 (31‰) | MATCH |
| §1.4 | h8-t3 fixed-field response completed exactly; the eight-profile model-belief recursion refused under its read budget | `horizon_run1.txt`: 14 min 13 s standalone, 289,407,472 reads; `MB1-REPORT.md`: five typed refusals at the 7,000,000-read ceiling, 31.1 min | MATCH — "same general depth" is the same root, h8-t3 |
| §1.2 | "the local object is Φ(B; c, σ)" | U0b prices Φ per (root, contract, cut); contract variation is a different field (σ0 reads the bid) | MATCH; the identity must carry the contract (FH-A4) |

Restated loosely (sharpened, not wrong): §1.2's "positive information
price at substantial subsets" is 9–31% of frontier nodes by count and
13–14‰ by mass at the receipt contract — the mass figure is the one
that prices a cut; §1.3's "few permille" is exactly 7‰ at two rows of
thirty.

## Verdicts by part

| Part | Content | Verdict |
|---|---|---|
| 0 | Executive result: one canonical refinement hierarchy indexed by focal decisions | **ADOPTED** (FH-A1); every boxed statement proved below; "sandwich" retired as a name (FH-A2). |
| I (§1) | Empirical basis from PR #87 | **ACCURATE** against the committed records (table above). Correctly labelled "not theorem premises." |
| II (§2–3) | Finite public-belief model, exact information-consistent value | **SOUND.** The three node kinds are the §23 recursion's three cases; "focal max after merge" is the O34/CBS fence, which the factor-belief representation enforces by construction (a belief node IS the merge). |
| III (§4–5) | Lower tail (lawful policy), upper tail (Bellman supersolution) | **SOUND, with the hypothesis discharged.** "Admissible" is a consequence of the three supersolution laws (Lemma 2.1), not a separate hypothesis. The God tail IS a supersolution under a deterministic field (Proposition FH-God). "Retain a previously valid upper or refuse" narrowed at FH-A3: the trivial upper is a refusal, never a fact; installation is by intersection. |
| IV (§6–8) | Focal horizon h_f; the two recurrences | **ADOPTED as the definition** (FH-A6). Forced focal nodes consume a unit under the parent's convention — binding for the first slice; Proposition FH-last shows the hierarchy is exact one layer before h_f when the last layer is forced everywhere. |
| V (§9–12) | Theorems 1–4 | **SOUND**, full proofs below. The parent's sketches are right; the inductions need the outer-k / inner-structure order made explicit. |
| VI (§13–14) | Theorem 5, the canonical gluing staircase | **SOUND** for Boolean utility (pmake). For bounded score the "mask" becomes a per-world value, not a set. The §14 caveat (short coalitions may still be needed) is SC-A5 restated and stays binding. |
| VII (§15) | Rollout improvement as the lower dual | **SOUND.** The "same baseline serves as fallback, lower tail, response column" sentence is the reason σ0-as-focal is the primary tail (FH-A4). |
| VIII (§16–19) | Action intervals, survivor monotonicity, exact-action criterion, certified regret | **SOUND**, with §19's "preserved facts" given a meaning (FH-A9) and the tie form of §18 stated (Proposition FH-tie). |
| IX (§20–21) | Scalar closeness is not decision safety | **SOUND and already the house position** (U0b's finding 2 is the specimen). |
| X (§22) | Exact-mass form | **SOUND** (proof below); it is exactly the shape `viewer_success_mass` and `GodUpper` already have (FH-A5). |
| XI (§23–24) | Interruption rule; gap measurements | **SOUND under the intersection reading** (Proposition FH-int); as literally worded, sound only if "completed child" excludes any child whose upper came from a weaker tail. Δ's are byproducts, agreed. |
| XII (§25) | Continuation substitution | **SOUND with the identity list SHARPENED** (FH-A9): the key is the posterior (survivor set) and the public record *as read by the field*, never the tabular state alone. |
| XIII (§26–27) | Fixed field first; the Ω×Θ lift | **ADOPTED / DEFERRED.** The lift claim (public Bellman structure unchanged) is MB-A1's Theorem 7.1 and lifts every proof below verbatim; it stays behind the non-goals fence. |
| XIV (§28–29) | One generic engine; initial tails and k ∈ {0,1,2} | **ADOPTED as the slice shape** (the split is the orchestrator's, FH-A11). Tails ruled at FH-A4. |
| XV (§30–37) | Gates FH1–FH8 | **ADOPTED**, with additions the record makes free (FH-A5, FH-A8, FH-A11). |
| XVI (§38) | Report of record | **ADOPTED**; walt-math asks for two additional columns (review §W3). |
| XVII (§39–41) | Success, partial success, correctness failures | **ADOPTED VERBATIM**; the §41 list is the round's gate language. |
| XVIII | Non-goals | **BINDING** (FH-A10). |
| XIX | Compact theorem sheet | Consistent with the proofs below. |
| XX (§42) | The recommended move | Direction; the orchestrator's call. Walt-math concurs, with the structural observation that on trick-4 roots the question is k ∈ {0, 1} (review §W1). |

## The rulings, reasoned (binding text lives in `walt/CENSUS-RULINGS.md`, FH-A1..A11)

The brief's eight questions, answered here with the reasoning; the
one-line binding forms are the FH-A rows.

**Q1 — the lower tail π (FH-A4).** Primary tail for the report of
record: **σ0 driving the viewer seat** ("σ0-as-focal", the declared
field policy as the focal policy — `viewer_success_mass(oracle, belief,
focal = σ0, field = σ0, stats)`). It is lawful (reads own hand + public
record + frozen identity), exactly evaluable, already exercised
(`profile_run1.txt`: h4-t4 σ0-as-focal 769‰ vs lowest-first 722‰ at the
bid; h3-t4 280‰), and it is the parent's §15 "same baseline" — fallback,
tail and response column in one object. Its identity **includes the
contract** (σ0 reads the bid — the §44 reuse boundary, `profile` gate
3b): `L_k^π` is a per-(root, contract, field) number, never projected
across contracts. Gate-only second tail: `FixedPreference::lowest_first`
(the C0 tail): free, and its extraction is the existing
`ExtractedPolicy` unchanged (see Q5). Extracted policies from prior
solves are admissible tails by Theorem 1 (any lawful deterministic
policy) but are not first-slice tails: their identity is a content
address of a choice table plus its own tail, and stacking them is the
kind of glue the non-goals fence excludes. The report must run both
tails on the FH8 anchors so the tail-quality effect on the settling
horizon is a measurement, not a guess.

**Q2 — the upper tail is admissible (FH-A3).** Proposition FH-God
below: under a deterministic field the world-revealed continuation
`G(B) = Σ_ω β_B(ω)·g(ω, B)` satisfies terminal exactness, public-branch
harmonicity and focal optimism; admissibility `Q ≤ G` follows (Lemma
2.1). The parent asserts this in one line; it is now proved. On
"retain a previously valid upper or refuse": **the trivial upper 1 is a
refusal, never a fact** — the same discipline as `GodUpper::fact()`
returning `None` for a vacuous upper and `doom::census_fact`. On the
first pass there is no previously valid upper anywhere, so an
unaffordable God branch refuses, the enclosing child is *unfinished*,
and the whole root reports its typed refusal (FH1 has no partial
install until FH2). Installation of any later interval is **by
intersection** with what the proof state already holds, never by
replacement (Proposition FH-int) — under that reading §23 is sound
regardless of which tail produced the new interval; under the literal
"install its new [L, U]" it is sound only because a trivial upper is
not a completion.

**Q3 — what the existing instruments already are (FH-A5).**
(i) `U_{a,0}^{God}` at a root action IS `GodUpper.value = (Z − |D_a|)/Z`
of the U0 census coordinate (`solver/godgap.rs`, `god_gap`, doom
enumerated after `context.root_action`): Lemma 2.0 gives `U₀ ≡ G`
everywhere, and `G(B₀a)` in mass form is the undoomed count after `a`.
(ii) `U^{God}` at any belief node is `price_node`'s `upper` in
`solver/horizon.rs` (`doom_over_belief`, today private — exposure is
the builders' call, not a mathematical one). (iii) `L₀^π(B) =
viewer_success_mass(…)/Z(B)`. (iv) **Theorem 5 subsumes the queued U1
salvation-mask slice as an upper producer**: `U₁^{God}(B) = max_a
Pr(S_a(B))` and the hierarchy computes the masses `Pr(S_a)` without
materializing the sets. It does **not** subsume the set-valued
instrument — the salvation complex's conflict/transversal work (SC §10–16)
needs `S_a` as sets, and nothing in the hierarchy produces them. U1 as
"an upper" is retired; U1 as "the masks for gluing analysis" is a
separate question sequenced by evidence, as SC-A8 already says.
(v) **Ply cut vs focal cut — Proposition FH-cut**: at a root where the
viewer leads the root trick, the U0b census's cut at `cut_plays = 4m`
equals `U_{a,m−1}` for every root action `a`. So **cut-4 = U_{a,0}**
(the orchestrator's reading, confirmed: only hidden nodes lie between
the root action and ply 4, and `U₀ ≡ G` is harmonic there — which is
why U0b's cut-4 over-pricing reproduced U0's Φ) **and cut-8 = U_{a,1}**
— the orchestrator's "cut-8 is NOT U_{a,1} in general because the
trick-5 decision falls at a variable ply" is **corrected**: the ply
varies (4, 5, 6 or 7) but is strictly below 8 on every branch, so
exactly one focal layer is consumed before the frontier on every
continuation, which is all the identity needs. The identity fails when
the cut is not at a trick boundary or the viewer does not lead the root
trick (then the number of focal decisions before the frontier varies by
branch and the ply cut is a branch-wise mixture of horizons — still a
valid upper, but not a `U_k`). Consequence: **`horizon_run1.txt` already
holds `U_{a,0}` (cut 4) and `U_{a,1}` (cut 8) per action for h3-t4,
h4-t4, h8-t4 under five contracts, and `U_{a,0}` for h8-t3** — a free
parity oracle for FH1 (FH-A11).

**Q4 — `h_f` exposure (FH-A6).** §6 is the definition: terminal or
decided → 0; focal → 1 + max over legal actions; public branch → max
over positive-mass branches. "Decided" is the §5 arithmetic — the SAME
`decided_success` predicate the value recursions use — so a node
decided early has `h_f = 0` with plays remaining, and Theorem 4 holds
only if `h_f` is computed with that same predicate (a different cutoff
in the depth walk than in the value walk breaks the collapse gate).
Forced focal nodes (singleton legal set) consume a unit under the
parent's convention; that convention is binding for the FH1–FH3
numbers. Independent verification for FH3: (a) the walk with the
decided predicate; (b) the bound `h_f(B) ≤` viewer tiles remaining at
`B` (each focal decision plays one); (c) on viewer-lead uniform roots
with no early decision, `h_f` after the root action is exactly `7 − T`
at a trick-`T` root; (d) the mechanical form of "the tail is never
consulted": **tail consultations = 0** when `k ≥ h_f` — count them and
gate on zero. Proposition FH-last adds: when the deepest layer is forced
on every continuation (trick 7 always is), `L_{h_f−1} = Q = U_{h_f−1}`
already.

**Q5 — the tie rule (FH-A7).** `π_k` is materialized under
`TieRule::LowestTileIndex`: at each focal node of the first `k` layers,
the argmax of `L_{·}(Ba)` with ties to the lowest stable tile index;
below the `k`-th layer, `π`. The tie rule changes the policy, never the
value (`L_k` is a max). FH5 — `V^{π_k} = L_k` through
`viewer_success_mass` with `focal = π_k` — is exactly the lower-side
no-strategy-fusion gate: it fails iff some node's "max" was taken over
something other than one public information state. **Binding detail:**
the existing `ExtractedPolicy` completes off-DAG "by the same declared
rule" (lowest tile index); for `π_k` the off-DAG continuation must be
**the tail π** — identical to the existing extractor only when π is
lowest-first, which is one reason lowest-first is the gate tail.

**Q6 — the anchors (FH-A8).** Confirmed from `horizon_run1.txt`, with
FH-cut translating each cut into the hierarchy's own coordinate:
(i) **h8-t3, bid 30, Z = 59,976**: `Q* = 28859/29988` (962‰), argmax
1-1; cut 4 = `U_{a,0}`: 1-1 → 59503/59976 (992‰), 3-3 → 29803/29988
(993‰), 2-1 → 965‰, 3-1 → 936‰, 5-5 → 959‰; exact per action 962/928/
890/955/922‰. (ii) **h8-t4, bids 36 and 39, Z = 1,200**: exact 2-1
3/4 (750‰), 3-1 602‰, 3-3 723‰, 5-5 893/1200 (744‰); cut 8 = `U_{a,1}`:
2-1 451/600 (751‰), 3-1 603‰, 3-3 750‰, 5-5 303/400 (757‰). (iii)
**h4-t4, cut 4 = `U_{a,0}`, Z = 34,650**: over-pricing 157/11550,
1136/17325, 2951/34650, 122/1155, 584/17325 = 13/65/85/105/33‰ at bids
30/33/36/39/42; exact 980/889/794/734/217‰, argmax 6-5 at every bid;
at bid 39 per action exact 551/605/556/734‰ and `U_{a,0}` 592/655/591/
840‰. The parent's rule stands: **the focal-horizon answer is not
pinned beyond the soundness laws.** What the soundness laws already say
(law, not pin): at (ii) `max_a U_{a,1} = U_{5-5,1} = 757‰ > Q_{2-1} =
750‰ ≥ L_{2-1,k}` for every k, so **k = 1 cannot settle 2-1 at bids 36/39
under the exact-action criterion**; at (i) `U_{3-3,0} = 993‰ > Q_{1-1}`,
so k = 0 cannot settle 1-1; at (iii) bid 39, k = 0 settles 6-5 iff
`L_{6-5,0} > 655‰`, i.e. iff the tail is worth more than 655‰ after 6-5
— a tail-quality question the report answers. And by FH-last, **every
trick-4 root is exact at k = 2** (trick 7 is forced), so on anchors (ii)
and (iii) the experiment is k ∈ {0, 1} with k = 2 as a collapse gate;
anchor (i) is where k = 2 is a real test (t4, t5 consistent, t6
clairvoyant, t7 forced).

**Q7 — non-goals (FH-A10).** Restated as a ruling, binding on FH1–FH3.

**Q8 — the split (FH-A11, comment not ruling).** The mathematics
constrains the order in one place only: no budgeted run may *report*
partial intervals before the intersection-and-witness discipline of
Proposition FH-int exists in the proof-state facts. FH1 may therefore
land first **only as an affordable-or-refuse engine** (whole-root typed
refusal, no partial install). Under that fence FH1 → FH2 → FH3 is the
right order. Recommended additions to FH1's gates, all free from the
record: `U_{a,0}` parity with `godgap_run1.txt`; `U_{a,1}` parity with
the cut-8 per-action readings in `horizon_run1.txt` on the viewer-lead
trick-4 roots (FH-cut); `L₀` parity with `viewer_success_mass`; collapse
with tail-consultations = 0 at trick-6 roots (k = 0), trick-5 roots
(k = 1), trick-4 roots (k = 2) — FH3 on real roots by FH-last.

## Proofs

Dated provenance: written 2026-09-04 at intake, walt-math. All
EXPLORATORY tier. Notation: `B` a public belief node with posterior
`β_B` on hidden worlds and mass `Z(B) = Σ_ω β_B(ω)·Z` in the exact-mass
form (integer weights; under a deterministic field every surviving
world has weight 1 and `Z(B)` = number of surviving worlds — asserted
per world in `doom_over_belief`); `A(B)` the legal focal actions at a
focal node; `Ba` the child after focal action `a`; `B_t` the child of a
public-branch node after observed field action `t`, `p_t = Z(B_t)/Z(B)`,
only positive-mass branches; `u(B) ∈ [0,1]` at terminal/decided nodes.
A **lawful policy** `ρ` is a function from focal nodes (the viewer's
information states) to legal actions. Its value `V^ρ` and the exact
value `Q` are defined by the §3/§4 recursions. The continuation tree is
finite (at most 28 plays), so structural induction is available
throughout.

### P0. Preliminaries

**Lemma 0.1 (focal actions preserve the posterior).** `β_{Ba} = β_B`
and `Z(Ba) = Z(B)` for every legal `a`. *Proof.* The posterior is the
prior conditioned on the public record's likelihood under each world;
a focal action is chosen by the viewer from public information and its
own hand, which is known, so its likelihood factor is 1 in every
surviving world. ∎

**Lemma 0.2 (public branches partition mass).** Under a deterministic
field, each surviving world `ω` at a public-branch node determines the
field action `t(ω)`; the branch supports partition the support of
`β_B`; `β_B(ω) = p_t · β_{B_t}(ω)` for `ω` in branch `t`; and
`Σ_t Z(B_t) = Z(B)`. *Proof.* Likelihoods are 0/1; Bayes. ∎ (This is
the §2 exact-mass statement.)

**Lemma 0.3 (Q dominates every lawful policy and is attained).** For
every lawful `ρ` and node `B`, `V^ρ(B) ≤ Q(B)`; and the policy `ρ*`
choosing at each focal node an argmax of `Q(Ba)` (any fixed tie rule)
attains `V^{ρ*} = Q`. *Proof.* Structural induction. Terminal: equal.
Public branch: both recursions are the same positive combination of
children. Focal: `V^ρ(B) = V^ρ(Bρ(B)) ≤ Q(Bρ(B)) ≤ max_a Q(Ba) = Q(B)`;
for `ρ*` the middle inequality is equality by choice. ∎ Hence `Q` is
"the exact best-response value over lawful policies" as §3 says, and
randomized focal policies add nothing to the expectation objective (a
convex combination of pure values — the SC companion's mixture caveat
applies unchanged: this is for expectation, not credal criteria).

### P1. Theorem 1 (§9) — lower validity and monotonicity

**Claim.** For every `B` and `k ≥ 0`: `V^π(B) = L₀^π(B) ≤ L_k^π(B) ≤
L_{k+1}^π(B) ≤ Q(B)`.

*Proof.* (a) `L₀^π = V^π` everywhere: structural induction — terminal
both `u`; focal `L₀^π(B) = V^π(B)` by definition; public branch
`L₀^π(B) = Σ_t p_t L₀^π(B_t) = Σ_t p_t V^π(B_t) = V^π(B)` by §4's
harmonicity of `V^π` (itself Lemma 0.2 applied to the expectation
defining `V^π`).

(b) `L_k ≤ L_{k+1}` everywhere, by outer induction on `k` and inner
structural induction on `B`. Base `k = 0`: terminal equal; public
branch by inner induction and positivity of the `p_t`; focal
`L₁^π(B) = max_a L₀^π(Ba) = max_a V^π(Ba) ≥ V^π(Bπ(B)) = V^π(B) =
L₀^π(B)`, using `π(B) ∈ A(B)` (π lawful). Step: assume `L_k ≤ L_{k+1}`
everywhere; then at a focal node `L_{k+2}(B) = max_a L_{k+1}(Ba) ≥
max_a L_k(Ba) = L_{k+1}(B)`; the other node kinds as before.

(c) `L_k ≤ Q` everywhere, by outer induction on `k`. Base: `L₀ = V^π ≤ Q`
(Lemma 0.3). Step: at a focal node `L_{k+1}(B) = max_a L_k(Ba) ≤ max_a
Q(Ba) = Q(B)`; terminal and public-branch nodes by the inner
induction. ∎

### P2. Theorem 2 (§10) — upper validity and monotonicity

**Lemma 2.0 (`U₀^G ≡ G`).** If `G` is harmonic on public branches and
exact at terminals, then `U₀^G(B) = G(B)` at every node. *Proof.*
Structural induction: terminal `u = G`; focal `U₀ = G` by definition;
public branch `U₀(B) = Σ_t p_t U₀(B_t) = Σ_t p_t G(B_t) = G(B)`. ∎

**Lemma 2.1 (a Bellman supersolution is admissible).** If `G` is
terminal-exact, public-branch harmonic and focally optimistic, then
`Q ≤ G` everywhere. *Proof.* Structural induction: terminal equal;
public branch by harmonicity and positivity; focal `Q(B) = max_a Q(Ba)
≤ max_a G(Ba) ≤ G(B)`. ∎ So the parent's "admissible" is a consequence
of "supersolution"; the three laws are the whole hypothesis.

**Claim.** For a supersolution `G` and every `B`, `k`: `Q(B) ≤
U_{k+1}^G(B) ≤ U_k^G(B) ≤ U₀^G(B) = G(B)`.

*Proof.* (a) `U_{k+1} ≤ U_k` everywhere, outer induction on `k`. Base:
at a focal node `U₁(B) = max_a U₀(Ba) = max_a G(Ba) ≤ G(B) = U₀(B)` by
Lemma 2.0 and focal optimism; terminal equal; public branch by inner
induction. Step: focal `U_{k+2}(B) = max_a U_{k+1}(Ba) ≤ max_a U_k(Ba)
= U_{k+1}(B)`. (b) `Q ≤ U_k` everywhere, outer induction on `k`. Base:
`U₀ = G ≥ Q` (Lemmas 2.0, 2.1). Step: focal `U_{k+1}(B) = max_a U_k(Ba)
≥ max_a Q(Ba) = Q(B)`. ∎

### P3. Theorem 3 (§11) — the focal-horizon interval

Immediate from P1 and P2: `L_k^π(B) ≤ L_{k+1}^π(B) ≤ Q(B) ≤ U_{k+1}^G(B)
≤ U_k^G(B)`. ∎ This is the FH2 invariant, checked in exact rationals.

### P4. Theorem 4 (§12) — finite exact collapse

**Claim.** For every `B` and every `k ≥ h_f(B)`: `L_k^π(B) = Q(B) =
U_k^G(B)`, and neither tail is consulted on any continuation from `B`.

*Proof.* Structural induction on `B`, the statement quantified over all
`k ≥ h_f(B)`. Terminal/decided: `h_f = 0`, all three equal `u(B)`, no
tail. Public branch: `h_f(B) = max_t h_f(B_t) ≤ k`, so the hypothesis
applies to every positive-mass child; the three recursions are the
same positive combination. Focal: `h_f(B) = 1 + max_a h_f(Ba) ≤ k`
forces `k ≥ 1` and `h_f(Ba) ≤ k − 1` for every `a`; hence `L_k(B) =
max_a L_{k−1}(Ba) = max_a Q(Ba) = Q(B)` and likewise for `U`. The
horizon-zero focal case (where a tail would be consulted) is never
reached because an undecided focal node has `h_f ≥ 1`. ∎

Note the definition of `h_f` uses the same decided predicate as the
values; with a different cutoff in the depth walk the focal case's
`k ≥ 1` inference can fail (FH-A6).

### P5. Theorem 5 (§13) — the one-step God upper is the salvation-mask upper

For Boolean utility and the God tail, at a focal `B` and legal `a`:
`g(ω, Ba) ∈ {0,1}` equals 1 iff a world-aware continuation makes after
`a` in world `ω`, i.e. iff `ω ∈ S_a(B)`. By Lemma 0.1 `β_{Ba} = β_B`, so
`U₀^{God}(Ba) = G(Ba) = Σ_ω β_B(ω)·1_{S_a}(ω) = Pr_B(S_a(B))`, and
`U₁^{God}(B) = max_a U₀(Ba) = max_a Pr(S_a(B))`. ∎ In mass form,
`Z·U₀(Ba) = Z − |D_a|` with `D_a` the doomed worlds after `a` — the
U0 census's `GodUpper` numerator (FH-A5). For general `u ∈ [0,1]` the
set `S_a` is replaced by the per-world value `g(ω, Ba)`; the identity
`U₁ = max_a Σ_ω β g(ω, Ba)` still holds, but it is no longer a mask.

### P6. Theorem 6 (§17) — survivor monotonicity, and the exact best survives

With `B_k = max_a L_{a,k}` and `S_k = {a : U_{a,k} ≥ B_k}`:
(a) `B_{k+1} ≥ B_k` (Theorem 1 per action) and `U_{a,k+1} ≤ U_{a,k}`
(Theorem 2 per action); if `a ∈ S_{k+1}` then `U_{a,k} ≥ U_{a,k+1} ≥
B_{k+1} ≥ B_k`, so `a ∈ S_k`. Hence `S_{k+1} ⊆ S_k`. ∎
(b) Every exact optimal action `a*` lies in every `S_k`: `U_{a*,k} ≥
Q_{a*} = Q* ≥ Q_a ≥ L_{a,k}` for all `a`, so `U_{a*,k} ≥ B_k`. ∎ This is
gate FH4's second sentence, and it is why an exclusion is permanent.

### P7. The §18 exact-action criterion, and its tie form

If `L_{b,k} > max_{a≠b} U_{a,k}` then for every `a ≠ b`, `Q_b ≥ L_{b,k}
> U_{a,k} ≥ Q_a`, so `b` is the unique exact optimal action. ∎

**Proposition FH-tie (the exact survivor set under ties) — delivered
here.** Call action `a` *collapsed at k* if `L_{a,k} = U_{a,k}` (then
`= Q_a`). Suppose every `a ∈ S_k` is collapsed. Then `Q* = B_k` and the
exact optimal set is `{a ∈ S_k : Q_a = B_k}`. *Proof.* Let `a₀` attain
`B_k = L_{a₀,k}`; then `U_{a₀,k} ≥ L_{a₀,k} = B_k`, so `a₀ ∈ S_k`,
collapsed, `Q_{a₀} = B_k`, hence `Q* ≥ B_k`. Any `a ∉ S_k` has `Q_a ≤
U_{a,k} < B_k`; any `a ∈ S_k` has `Q_a = L_{a,k} ≤ B_k`. So `Q* = B_k`
and the optimal set is as stated. ∎ This is the parent's "retain the
exact survivor set once the relevant endpoints collapse", made
checkable: the survivor set is exact iff all its members are collapsed;
until then it is a superset (P6(b)).

### P8. §19 certified regret and its monotonicity

For any lawful `ρ̂` with exactly evaluated `V(ρ̂)`: `0 ≤ Q* − V(ρ̂)`
(Lemma 0.3) and `Q* = max_a Q_a ≤ max_a U_{a,k} = U_k*`, so `Q* − V(ρ̂)
≤ U_k* − V(ρ̂) = Γ_k`. ∎

**Monotonicity `Γ_{k+1} ≤ Γ_k` holds under either of the following, and
"preserved facts" is ruled to mean (ii) (FH-A9):**
(i) `ρ̂_k := π_k` at the root (the canonical materialization of
`B_k`): then `L_{exec,k} = B_k` is monotone by Theorem 1 and `U_k*` by
Theorem 2 — "deterministic exact tails" is what makes `V^{π_k} = L_k`
exact rather than estimated.
(ii) The proof state is append-only in the APS sense: the executable
lower fact for `ρ̂_k` **together with its stored policy** is never
discarded, and every upper fact is retained, so `L_{exec,k+1} =
max(L_{exec,k}, new lowers) ≥ L_{exec,k}` and `U*_{k+1} = min(U*_k, new
uppers) ≤ U*_k`. Without (ii), a later pass computing a lower with a
different (weaker) tail, or an upper with a refused God branch replaced
by the trivial 1, can move Γ the wrong way; that is not a theorem
failure, it is a discarded fact. Intersection makes the monotonicity
unconditional.

### P9. §22 exact-mass form

Define `M(B) = Z(B)·X(B)` for `X ∈ {Q, L_k, U_k}`. Public branch: `Z(B)
Σ_t p_t X(B_t) = Σ_t Z(B)·(Z(B_t)/Z(B))·X(B_t) = Σ_t M(B_t)` (Lemma
0.2). Focal: `Z(B) max_a X(Ba) = max_a Z(Ba) X(Ba) = max_a M(Ba)` (Lemma
0.1, `Z(Ba) = Z(B) > 0`). Terminal: `M = u·Z`. The invariant `M_k^L ≤
M_Q ≤ M_k^U` is Theorem 3 times `Z(B) > 0`. ∎ The tails are already in
this form: `viewer_success_mass` returns `Z·V^π` and `GodUpper` carries
`Z − |D|`. Every comparison in the engine is therefore an integer
comparison; the rational appears only at the report.

### P10. Proposition FH-God (the world-revealed continuation is a Bellman supersolution) — delivered here

Fix a deterministic field `σ`, contract, utility `u`, and viewer seat.
For a node `B` and surviving world `ω`, let `g(ω, B)` be the value at
`B` of the perfect-information game in which the viewer knows `ω` and
every other seat plays `σ` on its hand in `ω` (the viewer maximizes; the
field's moves are determined). Define `G(B) = Σ_ω β_B(ω)·g(ω, B)`. Then
`G` is terminal-exact, public-branch harmonic and focally optimistic;
hence (Lemma 2.1) `Q ≤ G`, and Theorem 2 applies with `U₀^G ≡ G`.

*Proof.* Terminal/decided: the indicator (or utility) is a constant of
every continuation of every surviving world, so `g(ω, B) = u(B)` for all
`ω` and `G(B) = u(B)`. Focal: in the perfect-information game the
viewer's node is an ordinary max node, `g(ω, B) = max_a g(ω, Ba)`; by
Lemma 0.1 `β_{Ba} = β_B`, so `G(B) = Σ_ω β_B(ω) max_a g(ω, Ba) ≥ max_a
Σ_ω β_B(ω) g(ω, Ba) = max_a G(Ba)` (max of sums ≤ sum of maxes, with
nonnegative weights). Public branch: in world `ω` the field's move is
the determined `t(ω)`, so the perfect-information continuation from `B`
in `ω` is the continuation from `B_{t(ω)}` in `ω`: `g(ω, B) = g(ω,
B_{t(ω)})`. By Lemma 0.2, `G(B) = Σ_t Σ_{ω ∈ branch t} p_t β_{B_t}(ω)
g(ω, B_t) = Σ_t p_t G(B_t)`. ∎

*Mass form.* With unit world weights, `Z(B)·G(B) = Σ_ω g(ω, B)`; for
Boolean utility this is the number of surviving worlds not doomed —
exactly what `line_can_make` counts ("can ANY viewer line make from
here, with every other seat playing the declared field on its known
hand"). So `price_node`'s `upper` and `GodUpper.value` ARE `G`.

*Remark (stochastic fields, out of slice).* Harmonicity survives a
stochastic field if the God player is hand-revealed but tape-blind
(`g(ω, B) = Σ_t P(t | ω, B) g(ω, B_t)` at a public branch, and the same
Bayes rearrangement goes through); the parent's `Ω×Θ×Z` lift restores
determinism outright. U0b's per-world enumeration asserts unit weights
and is the deterministic-only instrument; a tape coordinate is not this
slice's.

### P11. Proposition FH-int (the §23 interruption rule is sound and its lower is executable) — delivered here

**Setting.** A *fact* at node `C` is a pair `[L(C), U(C)]` with `L(C) =
V^{ρ_C}(C)` for a stored lawful policy `ρ_C` on `C`'s subtree and `Q(C)
≤ U(C)`, both established under the same semantics identity (FH-A9's
list). A never-priced node holds the trivial fact `[0, 1]` with `ρ_C :=
π` (the tail) — the lower 0 is attained by any lawful policy and the
upper 1 is not a fact but a placeholder; in mass form `[0, Z(C)]`.

**Claim.** Let a pass end with, at every node of some frontier-closed
set of nodes, a valid fact — new where the pass completed the node,
prior where it did not, and at every node **the intersection** `[max
(L_new, L_prior), min(U_new, U_prior)]` where both exist. Compose
upward: focal `[max_a L(Ba), max_a U(Ba)]` over **every** legal `a`;
public branch `[Σ_t p_t L(B_t), Σ_t p_t U(B_t)]` over **every**
positive-mass `t`. Then every composed interval contains `Q`, the
composed lower is attained by one lawful policy, and the composed
interval at the root is contained in the previous pass's root interval.

*Proof.* Validity: the intersection of two intervals containing `Q(C)`
contains `Q(C)`. Focal: `Q(B) = max_a Q(Ba) ∈ [max_a L(Ba), max_a
U(Ba)]` because `L(Ba) ≤ Q(Ba) ≤ U(Ba)` for each `a` — this needs every
legal `a` in the max on the upper side (omitting one could drop the true
maximizer); on the lower side omitting actions is sound but is not
`L_k`. Public branch: `Q(B) = Σ_t p_t Q(B_t)` lies between the
corresponding sums, needing every positive-mass branch (a dropped branch
of mass `p` shifts both endpoints by up to `p`). Executability: at a
focal node choose `a* ∈ argmax_a L(Ba)` under the declared tie rule and
set `ρ(B) = a*`, `ρ|_{Ba*} = ρ_{Ba*}`; then `V^ρ(B) = V^{ρ_{Ba*}}(Ba*) =
L(Ba*) = max_a L(Ba)`. At a public branch, distinct branches are
distinct public histories, so `ρ := ∪_t ρ_{B_t}` is one lawful policy
with `V^ρ(B) = Σ_t p_t V^{ρ_{B_t}}(B_t)`. Where the intersection kept
the prior lower, its stored policy is the one used — which is why the
policy must be retained with the fact. Containment in the previous
root interval: each node's fact is contained in its prior fact by
intersection, and max/Σ are monotone in every argument. ∎

*What this rules (FH-A3/FH-A9).* "Completed child: install its new
[L, U]" is read as *intersect*; "unfinished child: keep its prior" is
the intersection with nothing new; a budget that ends mid-branch
leaves that branch's fact untouched and typed-refused; "resume plus
completion equals uninterrupted completion" (gate FH7's last clause)
then holds because facts are a function of the set of completed nodes
and intersection is idempotent and order-independent — the same
derived-view law the §49 proof-state spike gated.

### P12. §25 continuation substitution, with the identity sharpened

**Claim.** Let `C` be a node reached inside a solve, and let a receipt
hold `L(C) = U(C) = Q(C)` established under a matching identity. Replace
`C`'s subtree by the terminal value `Q(C)` in **all three** recursions.
Then `Q` at every ancestor is unchanged; every ancestor's `L_k` is
replaced by a value in `[L_k, Q]` (a valid lower, attained by `π_k`
patched below `C` with the receipt's stored policy — executable iff that
policy is stored); every ancestor's `U_k` is replaced by a value in
`[Q, U_k]` (a valid, tighter upper). *Proof.* `Q(C)` is what the `Q`
recursion computes at `C`, so ancestors are unchanged. `L_j(C) ≤ Q(C) ≤
U_j(C)` for every residual horizon `j`, and max/Σ are monotone, so the
substituted values are sandwiched between the unsubstituted `L_k`/`U_k`
and `Q`. Executability as in P11. ∎

**The identity that must match (binding, FH-A9).** The receipt's value
is a function of exactly: the viewer seat; the viewer's remaining hand;
the **public record as read by the field** — for σ0 that is the full
play history, not the tabular state, because a level-0 modeled mind
reads the record (CBS-A6: "action = pure function of own hand + public
record + frozen identity"); the declaration/trump, contract and utility
(hence the decided predicate's inputs — banked points, leader, plays in
the current trick — all functions of the record); the field identity
with its frozen parameters; and **the posterior itself** — under a
deterministic field the survivor set of hidden-hand triples with their
weights. The last is the sharpening: two different root fibers can
reach the same public record with different survivor sets, so a receipt
keyed by record alone is the PiKey defect reborn (CBS-A6 §43). In mass
form the receipt must carry `Z(C)` or its value as a rational; the
inner solve uses its own `Z(C)`, which matches iff the posteriors do.
"Trick 6 is nearly exact" is not a receipt (parent §25, retained
verbatim).

### P13. Proposition FH-cut (a ply cut at a trick boundary is a focal cut on viewer-lead roots) — delivered here

Let `C_c(B)` be the value the U0b census computes with `cut_plays = c`:
at depth `≥ c`, `G(B)` (the per-world God price; exact where decided);
above the cut, `u` at decided nodes, max over legal actions at viewer
nodes, `Σ_t p_t` at field nodes (this is `Descent::walk` with the cut
side, `solver/horizon.rs`). **Claim.** If every continuation from `B`
either reaches depth `c` after exactly `j` viewer decisions or is
decided/terminal before, then `C_c(B) = U_j^{God}(B)`. *Proof.*
Structural induction. At depth `≥ c`: `j = 0` and `U₀ = G` (Lemma 2.0).
Decided before the cut: both `u`. Viewer node above the cut, `j ≥ 1`:
`C_c(B) = max_a C_c(Ba) = max_a U_{j−1}(Ba) = U_j(B)` since every
continuation from `Ba` has `j − 1` decisions left before the cut. Field
node: the sum, `j` unchanged. ∎

**Corollary.** At a uniform root where the viewer leads trick `T` (all
receipt roots are of this shape — U0b: "all uniform roots with the
viewer on lead"), for every root action `a` and every `m ≥ 1`:
`C_{4m}(B₀a) = U_{a,m−1}`. *Proof.* From `B₀a` (depth 1) to depth `4m`
the remaining plays of trick `T` are the field's, and each of tricks
`T+1, …, T+m−1` contains exactly one viewer decision at a depth `< 4m`;
so `j = m − 1` on every branch not decided earlier. ∎ In particular
**cut-4 = `U_{a,0}` and cut-8 = `U_{a,1}`**; the census's root argmax
under the cut is `argmax_a U_{a,m−1}` under the same tie rule. When the
cut is not a multiple of 4, or the root is mid-trick, or the viewer
does not lead, `j` varies by branch and `C_c` is a branch-wise mixture
of `U_j`'s — still a valid upper by the same induction, not a `U_k`.

### P14. Proposition FH-last (a forced last layer collapses the hierarchy one layer early) — delivered here

**Claim.** If on every continuation from `B` every focal node at focal
depth `h_f(B)` (the deepest layer) has a singleton legal set, then
`L_{h_f(B)−1}^π(B) = Q(B) = U_{h_f(B)−1}^G(B)` for every lawful tail `π`
and every supersolution `G` that is terminal-exact. More generally, the
horizon `k` suffices at `B` whenever every focal node beyond the `k`-th
layer on every continuation is forced. *Proof.* Below the `k`-th layer
the three recursions differ only at focal nodes, where all three take
the same unique action: `max` over a singleton is the identity for `L`
and `U`; `π` chooses the only legal tile; `G` at a forced focal node
equals `Σ_ω β g(ω, Ba)` for the unique `a` — no clairvoyance is
exercised — and remains harmonic below, so by the induction of Theorem 4
with the base case moved up one layer, all three coincide. ∎

**Corollary (Texas 42).** Trick 7 is forced for every seat (one tile
each). Hence at a viewer-lead uniform root at trick `T` with no early
decision, the hierarchy is exact at `k = 6 − T` after the root action:
**trick-6 roots at k = 0, trick-5 roots at k = 1, trick-4 roots at
k = 2, trick-3 roots at k = 3.** Follow-suit may force more nodes and
close earlier still; that is measured, not assumed. This is Lemma
SR-forced and Lemma FT-trunc ("the ladder truncates one rung early")
arriving in the focal-horizon calculus.

## The walt-math review (opinion, labelled as such)

Everything in this section is walt-math's opinion at the exploratory
tier — a review, not a rewrite; nothing here binds a builder unless it
is also an FH-A row.

**Headline.** The parent's one real idea is the right one: index
refinement by *focal decisions*, not plies, so that removing one layer
of clairvoyance and adding one layer of search are the same move seen
from both sides. That idea is sharp, correct, and cheap to prove, and
it turns three existing instruments (God-gap census, in-solve horizon
census, fixed-policy evaluator) into the endpoints of one object
without new mathematics. Where the parent is weak is in what it leaves
unsaid rather than in what it says: the interruption rule needs the
intersection discipline to be sound as worded, "preserved facts" was
undefined, the suffix-reuse identity omitted the posterior, and the
recommended experiment "k ∈ {0,1,2}" is, on the trick-4 corpus,
structurally "k ∈ {0,1}" because trick 7 is forced — the parent did not
notice that its own collapse theorem fires one layer early. None of
this is a correctness failure; all of it is what a builder would have
had to guess.

**W1 — What is sharp.** (a) §6: only focal decisions consume horizon.
This is the design choice that makes U0b's ply-cut table obsolete as a
*design* axis while making it *exactly reusable* as data (FH-cut).
(b) `U₀ ≡ G` (Lemma 2.0) is why Theorem 5 is a one-liner and why the
salvation-mask slice was never a separate subsystem. (c) Action
intervals from line one (§16, §20): the house learned this at U0b
finding 2; the parent builds it in rather than bolting it on. (d) §22's
mass form is already the code's shape. (e) The §41 failure list is
gate language as written.

**W2 — What is underspecified, and what a builder should not rely on
until it is stated more precisely.** (a) §5/§23 "previously valid
upper": valid *under which identity, and combined how?* Ruled:
intersection under the P12 identity. (b) §19 "preserved facts": ruled
at P8(ii). (c) §25's identity list: the posterior and the record-as-
read-by-the-field are the missing coordinates. (d) §6 forced nodes:
the parent's convention makes `h_f` overcount; harmless for soundness,
misleading for economics — FH-last is the correction. (e) §13 is
Boolean-only; the "bounded-score generalization by the same Bellman
algebra" (§0) is true for the interval laws but the God *tail* for a
score objective is a per-world max of a score, and the score-profile
machinery (Phase 2) is vector-valued — that generalization is a design,
not a corollary. (f) §7's materialization claim silently needs the
argmax keyed by the viewer's full information state (record + own
hand); `extract_success_policy`'s history keying is exactly that, but
the off-DAG completion rule must be the tail (Q5). (g) The verifier's
toy has no field node before the first focal decision, so the property
that does the most work on real roots — harmonicity of `G` through
hidden nodes — is untested by the sweep and rests on FH-God plus the
FH1 parity gates.

**W3 — What walt-math would add (opinion).** (a) Two report columns:
the per-action decomposition `U_{a,k} − L_{a,k} = (U_{a,k} − Q_a) +
(Q_a − L_{a,k})` wherever `Q_a` is affordable — the first term is the
remaining fusion price at horizon k, the second the tail's policy gap;
they answer different questions (glue vs better tail) and the parent's
`Δ`'s mix them. And the ply distribution of the k-th focal frontier per
root, so the U0b table and the hierarchy's table can be read against
each other. (b) The "settling-k lower bound from uppers alone":
`k_min(b) ≥ min{k : max_{a≠b} U_{a,k} < Q_b}` where `Q_b` is known —
computable from the record today at anchors (i) and (ii) (Q6) and a
cheap first screen on any root with an exact value. (c) Tail
consultations = 0 as the mechanical collapse statistic (Q4(d)). (d) A
forced-free horizon `h_f'` as a *declared* identity coordinate if a
builder ever wants the tighter numbers at the same k; not for this
slice. (e) A closing thought, not a proposal: the hierarchy's `k` is a
uniform depth, and the record says the price is concentrated (U0b: 9–31%
of frontier nodes carry all of it). The parent's own §24 says the
selective scheduler comes *after* the canonical layer is measured; walt-
math agrees, and notes that Proposition FH-int already makes a
non-uniform frontier sound — the scheduler will need no new
mathematics, only a work-selection rule, which is the parent's §33/§35
machinery from the APS lineage.

**W4 — Correctness failures found in the parent: none.** One hazard as
worded (§23's "install its new [L, U]" — repaired by the intersection
reading, FH-A3/FH-A9), one silent hypothesis (`h_f` and the values must
share a decided predicate, FH-A6), one omitted identity coordinate
(§25, FH-A9). The theorems are true as stated.

**W5 — What the builders must not do.** Install a trivial upper as a
fact; replace rather than intersect; drop a legal action from a focal
max or a positive-mass branch from a hidden sum on the upper side; key a
suffix receipt by public record without the survivor set; compute `h_f`
with a different cutoff than the values; complete `π_k` off-DAG with
anything but the tail; project an L2 tail's `L_k` across contracts; or
read cut-8 as "trick 6 is nearly exact" — it is `U_{a,1}`, and at h8-t4
bids 36/39 it still cannot settle the play.

## Adjudication

Ruled 2026-09-04 at **FH-A1..A11** (`walt/CENSUS-RULINGS.md`, § "The
focal-horizon adjudication (2026-09-04)"), with Propositions FH-God,
FH-int, FH-tie, FH-cut and FH-last delivered there in the standing
form. Indexed on [walt-math-intakes](../../wiki/walt-math-intakes.md).
The engineering split FH1–FH3 is the orchestrator's; walt-math's
comment on order is FH-A11. The live default player is untouched by
this lineage until arena and conformance gates justify a change on
Jason's word (CE-A7/CBS-A9/APS-A9/MB-A7 restated at FH-A10).
