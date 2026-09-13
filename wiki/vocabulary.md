[Home](Home.md) · owns: the load-bearing vocabulary — one entry per term, its precise meaning, the typed distinction it guards, the ruling or source that fixed it, and the page that owns it · Sources: v0.7 `00_THESIS_AND_SCOPE.md` §6 and `20_MATHEMATICAL_FOUNDATION.md` §1, §6–7; `wiki/discrepancies.md` D3; `walt/CENSUS-RULINGS.md` (F7, J-A10, CE-A2/A3/A7, L2-A3/A4/A7, CBS-A3/A4/A6, APS-A4/A5/A6/A7, MB-A3/A6/A7, SC-A1/A4, FH-A2/A4/A6/A7); `walt/SCENARIO-PLAYER.md` §3; `walt/MAP.md`; `walt/briefs/U0-REPORT.md`, `MB1-REPORT.md`, `MORNING-2026-09-05.md`; `walt/math/focal_horizon_sandwich_v0.1.md` §4–6; `experiments/partnership/PLAYERS.md`, `BASELINE.md`, `SCOPE.md`; `walt/scheme/README.md`; `walt/gym/README.md`; `wiki/walt-math-freezes.md`; `wiki/walt-program.md`; `CLAUDE.md`

# Vocabulary that is load-bearing

In this repository words are typed. Most of the disasters the record
describes — the PiKey defect, the "certificate" hazard, the withdrawn "dead
heat", the FH4 BLOCK on a single word — were a distinction that a sentence
blurred. Each entry below gives the plain meaning first, then the precise
object, then the distinction the word guards, then the ruling or source that
fixed it and the page that owns the topic. Entries are alphabetical by
headword; cross-references are in **bold**.

Every walt-side term is EXPLORATORY-tier vocabulary: it names an object in
walt's mathematics or code, never a promoted result. The package-side terms
(support, fiber, cell, reachable, necessary outer profile) are corpus
vocabulary and outrank everything below them.

**Words that are fenced.** *Certificate* — never, except in the one
explanatory phrase under **necessary outer profile**, in a verbatim quotation
of a source, and inside walt's own term of art **certified regret**.
*Sandwich* — never as the name of an object; see **sandwich (retired)**.
*Equilibrium*, *convergence*, *monotone improvement* for a level-2 result —
never; see **level 2**. *Laydown* bare — reserved for the universal type; see
**decided / decision-dead / laydown**. *Rank* for a standing — never; see
**constellation**. *Feature* — Jason's standing ruling (2026-09-04, session
record) is no hand-coded features in any player; declared marginals over
game-defined coordinates are not features.

---

### Bar `B` — proof bar `B^proof`, executable bar `B^exec`

*Plain.* The best value walt can currently guarantee for a root: the highest
lower endpoint among the root's actions.

*Precise.* In a **root interval** program the bar is `max_a L_a`. The
**proof bar** maxes over every lower fact including grammar lowers (sound
for exclusion); the **executable bar** `B^exec` maxes only over lowers that
carry a materialized policy — `B^exec ≤ B^proof ≤ Q*`. In the
**focal-horizon hierarchy** the bar at depth k is `B_k`.

*Guards.* A number you can prove from a number you can play. A grammar lower
enters `B^exec` only after argmax extraction and fixed-policy re-pricing.

*Fixed by.* APS-A6 (the split; the audit at `25b40d9` found the shipped bar
was the proof bar and nothing violated the split); FH-A2 (`B_k`).

*Owner.* [walt-counted-belief-era](walt-counted-belief-era.md),
[walt-math-reference](walt-math-reference.md).

### Cap

*Plain.* A budget — on worlds sampled, fiber size, reads, memory, wall.

*Precise.* A resource limit declared in a result's identity. A capped
computation returns a typed refusal or an `Unresolved`, never a value that
pretends the excluded work was done.

*Guards.* Resource from proof: "a sample cap is a resource limit, never a
proof rule" (CE-A3). "Caps exclude, they never sample": a cap removes work
from scope and the excluded set travels with the result; unmeasured is never
zero ([walt-program](walt-program.md)). The world-cap 512 ruling
(2026-08-24, PR #32) is engineering, not mathematics — 128 was the phone
budget.

*Fixed by.* CE-A3; the census-era house rule; FH-A3 (an unaffordable God
branch is a typed refusal, and the trivial upper 1 is never installed as a
fact).

*Owner.* [walt-program](walt-program.md), [walt-instruments](walt-instruments.md).

### Cell · fiber · remainder · support normal form

*Plain.* What a seat can deduce about where the unseen tiles are, exactly,
before any probability enters.

*Precise.* Three domains that must not be conflated (Math §6.3, §7):

- a **complete deal** is fixed for the attempt;
- a **current hidden remainder** is what is left in the three hidden hands
  after the public plays — its type changes as tiles are played;
- the **current-remainder fiber** is the set of remainders compatible with
  the public history under the rules (Hall-feasible with the public voids
  and capacities).

The **cells** are the derived views a viewer computes from the mechanical
state: the unseen pool `U`, each hidden seat's void set `V_s`, allowed set
`P_s` and capacity `k_s`; cells are dependent through one conserved pool.
The **support normal form** splits the pool into certain marks `K_s` (exact
hidden-location knowledge) and the ambiguous pool `W` with residual
capacities; it is the globally minimal exact representation of the support
(Math §7.10–7.12, CELL-12/CELL-29). rob computes all of these as functions
of the semantic state, never as stored authorities.

*Guards.* A fiber is a set, not a distribution (see **support vs belief**);
a deal is not a remainder; a normal form is a representation of support, not
a game state.

*Fixed by.* Math §1 and §6.3 (both packages); D2 (derived views, v0.7);
CELL-12; rob's code discipline (`CLAUDE.md`).

*Owner.* [support-fiber](support-fiber.md),
[minimal-support-normal-form](minimal-support-normal-form.md),
[capacity-dp](capacity-dp.md).

### Certified regret `Γ = U* − B_exec`

*Plain.* How much pmake walt might be leaving on the table by playing its
recommended action, given everything it has proved so far.

*Precise.* `Γ^exec = U* − B^exec`, where `U*` is the best upper endpoint over
root actions and `B^exec` the **executable bar**. It bounds the recommended
executable policy's pmake regret on the joint-validity event; it is monotone
under refinement (`U*` never rises, `B^exec` never falls); `Γ ≤ ε` certifies
ε-optimality under the declared field and belief. The `[L_k, U_k]` ladder
reports `Γ_k` per depth.

*Guards.* This is walt's own term of art, internal to the exploratory tier.
"Certified" here means "bounded by a proof state's facts under a declared
identity" — it is never the D3 sense of certificate and never a corpus
status. A number is Γ only when both endpoints are facts in a proof state;
a settled root may keep a vacuous upper (cross-action dominance), so Γ can be
honestly positive at a settled root.

*Fixed by.* APS-A7 (adopted as the finite-budget deliverable; the §33
recommendation order — pmake floor primary); FH-A2 ("certified regret"
stays; no bare "certificate").

*Owner.* [walt-counted-belief-era](walt-counted-belief-era.md),
[walt-focal-horizon-era](walt-focal-horizon-era.md).

### Constellation

*Plain.* The shape of an endgame once you forget which tiles are which.

*Precise.* The canonical class object: the arrangement of **standings** (a
tile's relational position among the *living* tiles — who it follows, who it
sloughs against, who takes precedence, per lead context; partially ordered),
counts, holders and lead. Identity lives in the relations among the points,
not in which tiles fill them; a **realization** is a concrete assignment of
tiles and a declaration. Round 1's finding: the suffix minimax value is a
function of the constellation, not the tiles.

*Guards.* Standing is never called *rank* — `competitive_ordinal` owns that
word (static 1..13 over all 28 tiles); suit labels are colors, unordered.
A constellation is a rule-free carrier skeleton: feasibility, not
reachability (x:012's caveat).

*Fixed by.* Settled with Jason 2026-07-31 → 08-01; x:009 PARTIAL, x:010 and
x:012 CONFIRMED.

*Owner.* [idea-retrograde-rank](idea-retrograde-rank.md).

### Coordinate (gym)

*Plain.* One exercise position for the partnership gym.

*Precise.* One original seven-tile hand plus the actor-attributed public
history, declaration, bidder and seat to move. It does **not** contain the
other hands. The initial belief is uniform over all mechanically compatible
remaining deals (public voids and exact capacities respected) — an explicit
experimental prior; earlier observed actions are not reweighted under an
assumed historical player. The fixed future field is declared with it.

*Guards.* A coordinate is a question with an exact key, not a world; the
answer key is exact only relative to the declared field and prior.

*Fixed by.* `walt/gym/README.md` "What an answer means" (2026-09-06).

*Owner.* [walt-gym](walt-gym.md).

### Decided · decision-dead · laydown

Three neighbours a reader will confuse; none implies another.

- **Decided** — the make indicator is already determined at a public state
  for *every* continuation by arithmetic alone: the declaring side has
  banked its bid (monotone), or the unbanked remainder of the 42-point pool
  cannot reach it. `decided_success` (`solver/adaptive.rs`) is the one
  predicate every value recursion and the focal-depth walk use (FH-A6). This
  is device P1 of the pmake ruling (2026-08-17). A decided node has
  `h_f = 0` with plays remaining.
- **Decision-dead** — every information-consistent policy from the node has
  the identical value function on the node's fiber (`N_vec = 1`). Ladder:
  forced ⊂ dead ⊂ dominant, both inclusions strict on the S6b evidence.
  Forced nodes (one legal action) are excluded from every deadness count.
  A **one-deviation tie** is weaker — see that entry.
- **Laydown** — the universal type only: every legal continuation makes
  (∀ω ∀π ∀σ). The three weaker typed results are always named in full:
  `PolicyCertainMake` (one π, one σ, ∀ω), `AdversarialPolicyMake` (∀ω ∀σ),
  `ForcedMake` (∃π ∀ω ∀σ). A model-relative pmake = 1 is never called a
  laydown; no sampled route constructs any of the four. Lay downs at the
  rules level (301 per declaration, T1/LD chapters) are the LD theorem's
  object, proved relative to walt's `rules.rs`.

*Fixed by.* walt-math ruling 2026-08-17 (P1); J-A typing (2026-08-12);
APS-A5 (2026-08-31); FH-A6.

*Owner.* [walt-math-deadness](walt-math-deadness.md),
[walt-counted-belief-era](walt-counted-belief-era.md).

### Declared knobs

*Plain.* Every setting a result was measured under, stated with the result.

*Precise.* Sample counts, budgets, caps, seeds, tie rule, field identity,
belief identity, contract, epoch — each part of the result's identity. A
knob inherited silently is a defect; a knob changed is a new identity, never
"the same statistic improving" (a cap at one budget semantics and a
measurement at another is a semantics change).

*Guards.* Identity from convenience. Under MB-A6 every behavior-affecting
coordinate (registry, priors, correlation, persistence scope, seeds, tie
rule, fallback, solve budget, quality claim) is identity.

*Fixed by.* The instruments page's knob table; CE result identities; MB-A6.

*Owner.* [walt-instruments](walt-instruments.md).

### Epoch

*Plain.* One run of evidence under one fixed candidate set and
configuration.

*Precise.* In calculated evidence an evaluation epoch begins on fresh worlds
whenever the candidate set changes (CE §5.3); changing configuration,
replacing an action, raising a discovery budget or re-solving under a new
seed creates a new `PolicyId` and a new epoch (CE §17). In the sampling
stack an epoch is the declared sample-size triple — the level-2 ladder's
epoch of record is n₁ = 8, n₀ = 4 (`level2_results_2026-08-17.txt`) at
n_outer 200; the live/reduced O5 epochs are n_outer 50/n₀ 8 and 16/4
(`MORNING-2026-09-05.md`); the partnership defaults are 40 / 8 / 2
(`PLAYERS.md`).

*Guards.* **Numbers from different epochs do not compose.** A verdict measured
at one epoch is not the same statistic at another (the O5 match reversed
sign between the live and reduced epochs; the l2_controller's σ0 at n₀ = 8
and the waking seat's at n₀ = 2 are different minds).

*Fixed by.* CE §5.3/§17 (CE-A1); the arena and probe records' headers.

*Owner.* [walt-calculated-evidence](walt-calculated-evidence.md),
[walt-seat-play](walt-seat-play.md).

### Estimate · receipt · witness

*Plain.* Three kinds of "I have a number": a sampled one, a reproduced one,
an exhibited one.

*Precise.*
- **Estimate** — a sampled quantity carrying declared risk; it is marked as
  sampled and never silently upgraded ("a sampled basis is always marked as
  sampled", [walt-program](walt-program.md)). Jason's ruling of 2026-09-04
  — never mix certainty and uncertainty; estimates must be loud — is on the
  session record and not yet filed in a repository document (open item).
- **Receipt** — a byte-diffed reproduction that CI compares (rob's twelve;
  walt's pinned records and gate outputs). A green receipt is evidence,
  never a status change. "By construction is not a receipt" (PG-A8): an
  assertion that cannot fail proves nothing. An orchestrator's byte-diff is
  an audit note, never a receipt (FT-R7c).
- **Witness** — an exhibited object whose existence proves the claim: a
  world, a policy, a state, a counterexample pair (the 90-world witness; a
  lower witness = a policy whose value is the bound; an upper witness = a
  relaxation). "A cross-check is not a witness": agreement between two
  computations of the same quantity does not manufacture the object the
  pipeline could not build (FT-R1 at h9, NOT PRICED stands).

*Guards.* Each from the others; in particular a receipt of the dual side says
nothing about the primal side.

*Fixed by.* PG-A8; FT-A18/FT-R1; the tier ladder ([Home](Home.md));
Jason's 2026-09-04 ruling (session record).

*Owner.* [verification](verification.md), [walt-math-reference](walt-math-reference.md).

### Exact

*Plain.* Computed over every world of the fiber with rational arithmetic —
no sampling, no floats.

*Precise.* Two typed exactnesses in the six-way ladder: **`ExactFiberRoot`**
(exact over the whole fiber at the root — "exact root") and
**`ExactFrozenSet`** (exact over a frozen, enumerated world set — "exact for
the frozen set"). The exact response `response_success_mass` is the oracle
every walt gate checks against; the trick-3 root h8-t3 is the deepest exact
value on record (28859/29988, 289M reads, 14 min); trick 1 is unreachable by
that path (`walt/MAP.md`). Jason's ruling of 2026-09-04 (session record):
the exact machine is theory and grader, not a player — "if we say it can
play from trick 3 we don't have a player"; "exact when small" is the
seat-side rule.

*Guards.* Exact-for-the-frozen-set from exact root; exact from estimate;
scalar closeness from decision safety (a trick-6 cut is 0–7‰ off and flips
the play twice — U0b).

*Fixed by.* CE-A3; `walt/MAP.md`; U0B-REPORT; the 2026-09-04 ruling.

*Owner.* [walt-calculated-evidence](walt-calculated-evidence.md),
[walt-focal-horizon-era](walt-focal-horizon-era.md).

### Feasible vs reachable

*Plain.* "Could the hidden tiles sit this way?" versus "could a legal game
have produced exactly this knowledge?"

*Precise.* **Hall-feasible**: a hidden assignment consistent with the public
voids and capacities exists now. **Reachable**: a valid deal plus a legal,
actor-attributed prefix produced exactly this support (`R_Str^m`, REACH-01).
A Hall-feasible support is not automatically reachable; the exact reachable
cardinality is open inside a corpus-proved 26–46-bit interval, narrowed to
[36,45] at the exchange tier. Reachability is a proof-irrelevant proposition
— no identity-bearing certificates; equality through projected state only.

*Guards.* Feasible from reachable; "possible by rule now" from "generated by
legal play".

*Fixed by.* Math §7.13 (both packages), REACH-01..03; D1 (proof
irrelevance); x:001/006/007/008.

*Owner.* [reachability](reachability.md).

### Field `σ` · level-k mind · `σ0` · `σ1` · `F₀ / F₁ / F₂`

*Plain.* walt's model of the other three seats.

*Precise.* A **field** is a behavioral action law for the seats walt does
not control, with any latent continuation state it needs (Thesis §6). A
**level-0 mind** at a **PiKey** draws n₀ worlds from its information state
and best-responds to a **Dice** field (uniform draws); a **level-k mind**
does the same with n_k worlds against level-(k−1) minds (SCENARIO-PLAYER
Def 3.2/3.3). The binding rung table (MB-A3): `D = FieldModel::Dice`;
**`F₀ = BR(D) = σ0`**, the banked-correct level-0 modeled mind
(`solver/field.rs`), deterministic, reads the bid and the full public record;
`F₁ = BR(F₀)` = level-1 walt; `F₂ = BR(F₁)` = the unbuilt level-2 rung. `σ1`
names the level-1 mind used as a field by the waking seat and the level-2
probes. Under model belief the field itself is hidden state: Ξ = Ω×Θ, one
factor belief per type profile; a fixed field is the point mass ν = δ_θ.

*Guards.* The registration "Dice = σ0" proposed at first reading was WRONG and
retracted (MB-A3). σ0's per-hand classification is 99% of every bill
(`walt/MAP.md`).

*Fixed by.* SCENARIO-PLAYER §3; MB-A1/A3; CBS-A6 (fields must be seat-local
or represented as explicit factors).

*Owner.* [walt-seat-play](walt-seat-play.md),
[walt-focal-horizon-era](walt-focal-horizon-era.md).

### Focal decision · focal depth `h_f`

*Plain.* A decision walt itself must make; how many of them are left.

*Precise.* A **focal** node is one where the viewer seat chooses; public
observations and modeled-seat plays consume no horizon. `h_f(B) = 0` at a
terminal or decided node; `h_f(B) = 1 + max_a h_f(Ba)` at a focal node;
`h_f(B) = max_t h_f(B_t)` at a public branch (FH §6). Forced focal nodes
consume a unit under the parent's convention (binding for FH1–FH3).
Independent checks: `h_f ≤` viewer tiles remaining; `h_f = 7 − T` after the
root action at an undecided viewer-lead trick-T root; tail consultations = 0
whenever `k ≥ h_f`.

*Guards.* Focal depth from ply depth: the relaxation error being removed is
clairvoyant focal choice, not lack of observation of a public action. A ply
cut at a trick boundary equals a focal cut only on viewer-lead roots
(FH-cut).

*Fixed by.* FH §6; FH-A5 (FH-cut), FH-A6.

*Owner.* [walt-focal-horizon-era](walt-focal-horizon-era.md),
[walt-math-reference](walt-math-reference.md).

### Focal-horizon hierarchy · focal-horizon interval `[L_k(B), U_k(B)]`

*Plain.* One object that says, for each opening action, "at least this much,
at most this much," and tightens as walt is allowed to think exactly about
more of its own decisions.

*Precise.* For each root action `a` and depth `k`, the **action interval**
`[L_{a,k}, U_{a,k}]`: below, the value of the best lawful policy that may
optimize the next k focal layers on each public trajectory and then returns
to a fixed lawful **tail** π; above, the same with the world-revealed God
continuation as tail. `L_k ≤ Q ≤ U_k`; both collapse to the exact `Q` when
`k ≥ h_f` (Theorem 6); with trick 7 forced the collapse is one layer early —
trick-4 roots exact at k = 2 (FH-last). k = 0 is `viewer_success_mass` below
and the **God upper** above; U0's census is `U_{a,0}`, U0b's ply cut is
`U_{a,m−1}` on viewer-lead roots, the never-built salvation-mask upper is
`U_{a,1}`; rollout improvement is `L_k`; argmax extraction is `π_k`
(`walt/MAP.md`, the tree-shake list). The **bar** `B_k`, **survivor set**
`S_k` and `Γ_k` are derived per depth.

*Guards.* "Sandwich" is not a citable object name; gate names and ledgers say
*containment* (the FH4 BLOCK, fixed at `b6de5a25`). The hierarchy is a
finite exact hierarchy, not a rollout depth; a cut's argmax is carried as an
upper and never as a verdict.

*Fixed by.* FH-A2 (names), FH-A3 (God tail admissible), FH-A5 (identities),
FH-A9 (intersection discipline: a fact is the intersection of prior and new;
every lower carries its policy; resume ≡ uninterrupted).

*Owner.* [walt-focal-horizon-era](walt-focal-horizon-era.md).

### Freeze

*Plain.* A numbered, dated pin on something later work must reproduce.

*Precise.* A freeze clause states a **constant or a generating rule, never
both** (bought with the freeze-50(a) defect: an explicit list beside a sort
rule that did not generate it). Where both are wanted the rule is
authoritative, the list is a derived check, and the two are asserted equal
in-run. A stored artifact carries the freeze-set digest; a digest mismatch
means *corrupt*, not stale — the cache is discarded entire. The register runs
1–58 (39 and 40 reserved; 58 = the RefineV1 semantic freeze of
`solver/refine.rs` at `25b40d9`, issued by the register and the briefs —
CENSUS-RULINGS never names the number).

*Guards.* A pin from a result: a freeze authorizes a gate and records no
finding (freeze 57 is the standing example).

*Fixed by.* FT-A23(v); the freezes page's four standing rules; APS-A9.

*Owner.* [walt-math-freezes](walt-math-freezes.md).

### Fusion horizon

*Plain.* The trick depth beyond which not knowing the hidden hands stops
costing anything.

*Precise.* The depth past which, on a declared corpus, the **information
price** `d_info = U^God − Q` is zero — beyond it physical doom is the only
unavoidable failure and one lawful policy saves every saveable world (the
fusion-free-suffix hypothesis, SC §37–38). Measured, never proved: trick 5 at
the uniform receipt roots (U0, `godgap_run1.txt`); NOT fusion-free inside a
trick-4 solve (13–14‰ mass-weighted, U0b).

*Guards.* An empirical object first (SC-A4): a theorem may be proposed only
after adversarial counterexample search; the corpus varies only trick depth,
so the horizon is a measurement along one axis.

*Fixed by.* SC-A4; U0-REPORT; U0B-REPORT.

*Owner.* [walt-focal-horizon-era](walt-focal-horizon-era.md).

### Fusion price · information price · policy gap (and the model-fusion price Φ)

*Plain.* Three different reasons walt fails to make a bid that a clairvoyant
would make.

*Precise.* At a root with fiber size Z, doomed set D, exact information-
consistent value Q and a lawful policy ρ (U0-REPORT):

```
1 − V(ρ) = d_phys + d_info + d_policy(ρ)
  d_phys      = |D| / Z         physical doom (no policy can save these worlds)
  d_info      = U^God − Q       information price (the God gap)
  d_policy(ρ) = Q − V(ρ)        policy gap (this policy is not the best lawful one)
```

The **fusion price** in the FH reports is `U_k − Q` (the upper side of the
interval); the **policy gap** is `Q − L_k` (the tail's shortfall). The
**model-fusion price** `Φ = U^sep − Q(ν)` is the analogous gap when the
*field* is hidden (MB1): zero at every t5/t6 coordinate, strictly positive at
trick 4 (38/9600 at h8-t4 3-1, gate M6).

*Guards.* The three terms are typed apart because they buy different work: a
zero doom census moves only `d_phys` and does not prove the remainder is
information price (SC-A1); at k ≥ 1 the residual width is the policy gap
(9–41‰), not fusion price (0–3‰) — a better lawful tail buys more than a
deeper search (FH1/FH3). **The doom-census ledger sentence "overwhelmingly
the info-consistency price" was corrected 2026-09-03: the 267‰ split at the
opening root is UNKNOWN** (`walt/DISCREPANCIES.md`).

*Fixed by.* SC-A1 (§13 split); U0-REPORT; MB1-REPORT; the 2026-09-03
correction.

*Owner.* [walt-focal-horizon-era](walt-focal-horizon-era.md),
[walt-counted-belief-era](walt-counted-belief-era.md).

### Gate · probe record · receipt

*Plain.* Three places a walt number can live, in decreasing strength.

*Precise.* A **gate** is a test under `walt/walt/tests/` that asserts the
number and fails the suite when it drifts — sized to its law: one coordinate
per law plus a PINNED strictness witness, never a census. A **probe record**
is a committed output file under `walt/probes/` or `walt/briefs/` produced by
a named binary; it is reproducible but not asserted. A **receipt** (rob
sense) is byte-diffed in CI. A walt number is quotable as a result only
through the gate that pins it or the verifier receipt that carries it;
otherwise it is "probe record, not gate-pinned". Expensive oracle values a
suite needs in several gates are computed once in a shared fixture —
independence is between code paths, never between recomputations.

*Guards.* Gate-not-prose: read the assertion, never the summary; prose
drifts toward the tidier claim (CBS-A8: the parent's §0 softened three
probes' verdicts; the READMEs are the authority).

*Fixed by.* `CLAUDE.md` code discipline (gate sizing, 2026-09-04); CBS-A8;
the `[[gate-corpus-trim]]` card.

*Owner.* [walt-architecture](walt-architecture.md),
[walt-instruments](walt-instruments.md).

### God upper · God-tight · God gap

*Plain.* The best a seat could do if it could see every hidden hand — and
whether walt already does that well.

*Precise.* The **God upper** at a node is the per-world clairvoyant make
check: `1 − doomed/Z` bounds `Q` from above (`doom.rs`, `godgap.rs`,
`horizon.rs`); at a root action it is `U_{a,0}`. A coordinate is
**God-tight** when the exact `Q` equals the God upper — every saveable world
saved; Theorem 7.1: iff the saveable worlds have a common intersection.
The **God gap** is `d_info = U^God − Q`. Typed verdicts: `GodUpper`,
`GodTightPolicy`, `PositiveGodGap` (needs the exact `Q` as witness),
`UnknownGodGap` (zero certified doom with no exact `Q` — never
`PositiveGodGap`). Degenerate God-tightness (nothing left to save) is typed
apart from substantive.

*Guards.* God-tightness is field-specific and does not transport; a God-tight
receipt is a policy, not a theorem.

*Fixed by.* SC Thm 6.1/7.1 (SC-A1); SC-A4 (`UnknownGodGap`); U0-REPORT
(vacuity discipline); FH-A3 (Proposition FH-God).

*Owner.* [walt-focal-horizon-era](walt-focal-horizon-era.md).

### Information set · PiKey

*Plain.* Everything a seat knows at a decision; the key a modeled mind's
answer is cached under.

*Precise.* A perfect-recall **information state** is the private-observation
record plus the public history (Math §6.2); a mechanical projection is not
automatically one (Math §6.6). In the sampling stack a modeled mind's
decision state is **`PiKey = (s, h_s, κ(R))`** — chair, remaining hand, the
reduced record *including banked totals* (Def 3.1); every cached π value must
be a pure function of that key (Def 3.4). From `level1.rs`'s birth through
2026-08-18 PiKey omitted banked — serial execution masked it as first-come
aliasing; the rayon port made it racy within hours; the fix restored purity
(receipt on `f5fff91`). Since 2026-09-06 `Key`/`PiKey` also carry a voids
coordinate (`InnerBelief`).

*Guards.* A cache key that omits any field or factor coordinate is the PiKey
defect reborn (CBS-A6, FH-A9): a suffix receipt is keyed by seat, hand, the
record *as read by the field*, contract, field identity, and the posterior
itself.

*Fixed by.* SCENARIO-PLAYER Def 3.1/3.4; CBS-A6; FH-A9.

*Owner.* [walt-seat-play](walt-seat-play.md), [strategic-state](strategic-state.md).

### Intake — parent · companion · verifier · rulings

*Plain.* How outside mathematics enters walt without being trusted.

*Precise.* The **parent** is filed verbatim with a `.sha256` pin and never
edited; the **companion** (`*_intake.md`) records where it narrows and what
governs; the **verifier** is a stdlib Python check re-run at intake and
recorded as session evidence, scratch tier (TRUST-01 — a PASS is never an
axiom); same-day **rulings** (`XX-A1..An`) in `walt/CENSUS-RULINGS.md`
adopt, amend or refuse each part; obligations go to a ledger. Transit
damage is recorded, not repaired (MB-A4). Hand-delivered parents are not
courier dispatches — no number, ledger untouched.

*Guards.* An intake accepted "at instrument tier" promotes nothing; a
parent's §0 engineering summaries are unciteable as probe conclusions
(CBS-A8).

*Fixed by.* The CE precedent (2026-08-24) restated in every later
authorization note.

*Owner.* [walt-math-intakes](walt-math-intakes.md).

### Level 2

*Plain.* walt modeling a seat that is itself modeling walt.

*Precise.* A level-2 result is a **best response to a named σ1** — the level-1
mind with its declared epoch — never "equilibrium", "convergence" or monotone
improvement (L2-A7, O36). Only worlds and branches that can reach a
field-disagreement state can carry any level-2 correction: level 2 is a
calculated refinement, never a universal re-solve (L2-A1). Recurrence claims
are typed root / behavioral / local exact / global exact and never promoted
across those lines; no damping, mixtures or robust-cycle policies without a
separate intake. In the partnership program "L2" means **L2 Partner**
(partner at L1, opponents at L0) unless stated; **L2 All** is a separate
family.

*Guards.* Model choice from sampling depth (the **thread** labels); a rung
from a fixed point.

*Fixed by.* L2-A1/A7 (2026-08-24); `PLAYERS.md` (2026-09-06).

*Owner.* [walt-seat-play](walt-seat-play.md),
[walt-partnership-program](walt-partnership-program.md).

### Necessary outer profile

*Plain.* A cheap check that every reachable support passes — but passing it
proves nothing.

*Precise.* `ReachabilityOuterNecessaryProfile` (v0.7): the 46-bit
upper-bound object whose check is necessary only; it "cannot construct a
reachable state" and provably admits members that decode to infeasible or
unreachable support (Math §7.13.6). rec called the same object an "outer
reachability certificate"; the numbers agree; the name does not.

*Guards.* Necessary from sufficient. **"Certificate" is a soundness hazard
and is never used** — rob's CI greps enforce this; walt's decision-sparse
audit barred it forward (DS-A1..A18); the only permitted occurrences are
this explanatory phrase, a verbatim quotation of a source, and **certified
regret**.

*Fixed by.* D3 (`wiki/discrepancies.md`, v0.7 naming, confidence high);
x:005's response used the deprecated name — cosmetic only.

*Owner.* [discrepancies](discrepancies.md), [reachability](reachability.md).

### NO-RESCUE · F7 both-outcomes

*Plain.* When something fails, that is a finding, not a bug to engineer
around.

*Precise.* **NO-RESCUE** (Jason, 2026-08-10; stood at the top of the retired
`walt/PLAN.md`): a failure is a counterexample to carry back to the
mathematics, never a thing to fix, spin or assist with engineering — "if the
whole thing falls on its face that's FINE"; verify against the reference we
have, not in triplicate, and when independent mechanical verification is
needed the path is Lean, not Python. **F7** (census-fork ruling): both
outcomes of a declared experiment are results; a FAIL emits the canonical
form, the divergent statistic and both concrete witnesses in exact rationals
and continues to the next class — no descriptor edit, no re-run with altered
invariants. Every later chapter's "stop and report per NO-RESCUE; never
patch" is this rule; a detector disagreement with a proved proposition is an
implementation defect, never a new finding (J-A11).

*Guards.* A negative result from a defect; a refutation from a rescue.

*Fixed by.* F7 (`walt/CENSUS-RULINGS.md`), the NO-RESCUE policy
([walt-factory-era](walt-factory-era.md) §, [walt-program](walt-program.md)).

*Owner.* [walt-program](walt-program.md), [walt-negative-results](walt-negative-results.md).

### Obligation (O-numbers)

*Plain.* A proof or audit debt the code owes, numbered so it cannot be
forgotten.

*Precise.* The SCENARIO-PLAYER obligations ledger O1–O38 (O1–O9 the
spec-after-build; O12–O19 signed-pivotal; O20–O28 calculated evidence;
O29–O38 level-2 field stability). Obligations are debts, not results
(CE-A4, L2-A2). Sibling ledgers: CBS-O1..O15, PS-T1..T15, MB-O1..O20 and the
SC §60 Lean tranche, accepted into a Lean side-project ledger that exists in
no file.

*Owner.* [walt-seat-play](walt-seat-play.md), [walt-math-open-questions](walt-math-open-questions.md).

### One-deviation tie

*Plain.* Changing one move from a reference policy does not change the
value.

*Precise.* Argmax-indifference at a node: single deviations from one
reference policy tie. The set of one-deviation-tied states is a **superset**
of the **decision-dead** states (the two coincide only if every one-deviation
from *every* policy ties). Recall is reported against both denominators
where both are computable, each labelled, never summed or averaged. 51% of
49.5M classified mid-game call sites are one-deviation ties (S6c).

*Guards.* Indifference from deadness — "all policies tie" is the stronger
claim.

*Fixed by.* J-A10 (two denominators, never one); J-A11.

*Owner.* [walt-math-deadness](walt-math-deadness.md).

### Phone

*Plain.* The walt that runs in the plunge app on Jason's phone.

*Precise.* The archived Plunge WASM artifact `walt.wasm` (SHA-256
`af0200af…`) with its `walt.ts` wrapper, preserved byte-for-byte under
`experiments/partnership/reference/phone/`; added to Plunge at `1810da20`
(2026-08-22) and identified byte-for-byte as texas-42 **`9a056f20`**
(2026-08-19, θ = 11/16 default, race-then-refine opt-in). It plays with its
original race/refinement procedure at n = 40, n₀ = 8. **L1 Race**
(`l1-race`) is the native counterpart whose completed decisions matched it
in 64 checks and 35/35 fallback-free pairs; it is not an alias of **L1
default**.

*Guards.* The artifact from the current native player; whether plunge still
deploys this exact binary is the `[[plunge-walt-sync]]` question.

*Fixed by.* `BASELINE.md`, `HEAD-TO-HEAD.md`, `PLAYERS.md` (2026-09-06).

*Owner.* [walt-partnership-program](walt-partnership-program.md).

### Player families

*Plain.* The names for which seats walt models as thinkers.

*Precise.* The family says whose thinking is modeled; any departure from its
default search or belief is named beside it (`PLAYERS.md`):

| Family | Preset | Models |
|---|---|---|
| **L1 default** | `l1-default` (= `l1-fixed`) | all three other seats at L0; fixed search; legacy inner beliefs |
| **L2 Partner default** | `l2-partner-default` | partner at L1, opponents at L0; fixed search at root and in modeled minds |
| **L2 Partner with voids** | `l2-partner-voids` | as above with counted sampling respecting public voids inside modeled minds |
| **L2 All** | `--mode all-l1` | all three other seats at L1 — a separate family |

Independent settings: root search and modeled search each **Fixed / Refine /
Race**; inner belief **voidless / voids-counted**; sample budgets root / L0 /
L1 (default 40 / 8 / 2); a 14 s execution allowance. A **fallback**
(`l1-fallback`, `legal-fallback`) is an outcome of execution, never a family.
"Default" means the fixed-search settings, not the best-scoring policy. The
strength assessment ruled L1 default the operating default (14/14/72 and
12/17/71 establish no gain).

*Guards.* Modeled level from search procedure from belief — three
independent axes; this vocabulary names the sampling-stack player only, not
the proof-state player `unified.rs`.

*Fixed by.* `PLAYERS.md`, `STRENGTH-ASSESSMENT.md` (2026-09-06).

*Owner.* [walt-partnership-program](walt-partnership-program.md).

### pmake

*Plain.* The probability of making the bid — the only thing walt is trying to
maximize.

*Precise.* `Q(B) = max` over lawful policies of P(declaring side banks its
bid), the max taken after hidden worlds with the same public history are
merged (see **strategy fusion**), against a declared field. A Boolean
payoff: `u_ρ ∈ {0,1}`. Trick differential is a proxy, never the target;
expected points is a secondary objective under the score layer (APS-A2).
Consequences: the exact Boolean pruning devices P1–P4; the arena signature
(walt loses points and wins marks); at an exact tie P(make) has no gradient,
so play falls to the tie rule (the Gran 6-4 hoarding mechanism).

*Guards.* Objective from proxy; a model-relative pmake = 1 from a laydown.

*Fixed by.* Jason's ruling 2026-08-17; walt-math-12
(`WALT-MATH-RULING-2026-08-17-pmake-and-the-walk-to-trick-1.md`); APS-A2/A7
(pmake floor primary).

*Owner.* [walt-seat-play](walt-seat-play.md), [walt-math-reference](walt-math-reference.md).

### Possible vs probable

*Plain.* "Could be" versus "how likely".

*Precise.* **Possible** is a rule-support statement: which hidden worlds
remain compatible with the public history (the fiber; a Boolean predicate).
**Probable** is a belief statement: a measure on those worlds from the
chance law and a policy model's action likelihoods. The current remainder
fiber is not a probability distribution (Thesis §6); physics-only
uniformity holds only under the uniform deal law with no action-likelihood
tilt (BEL-07). Capped or unmeasured mass is possible and unweighted:
"unmeasured is never zero".

*Guards.* Support from belief (next entry); a world excluded by a cap from
a world with zero mass.

*Fixed by.* Thesis §6; Math §1; BEL-07/07A; the caps rule.

*Owner.* [belief-vs-support](belief-vs-support.md).

### Root interval · survivor set

*Plain.* For each opening action, the exact bracket walt has proved; the
actions that are still in the running.

*Precise.* A **root interval** `[L_a, U_a]` per root action, a fixed lawful
policy below and a relaxation above; the **survivor set** is the set of
actions whose upper is not below the **bar** (`max_a L_a`); exclusion is
permanent under refinement (Theorem 2.1 of the counted-belief parent, the
successor of Theorem E6.3). Seven typed result kinds extend the six-way
ladder; `HeuristicFallback` is never serialized as a settled winner.

*Guards.* These are the adopted names; "sandwich" collides with adjudicated
names (E6.3's value sandwich; the REFUTED T1-A bounded sandwich; SP-A7) and
is not a citable object name.

*Fixed by.* CBS-A3 (2026-08-30).

*Owner.* [walt-counted-belief-era](walt-counted-belief-era.md).

### Sandwich (retired)

*Plain.* The word for "a value trapped between a lower and an upper bound"
— retired as a name.

*Precise.* Permitted only as: the adjudicated name of Theorem E6.3 ("value
sandwich", decision-sparse errata); the title of a parent document quoted as
such (`counted_belief_sandwich_v0.1.md`, `focal_horizon_sandwich_v0.1.md`);
and the REFUTED T1-A "bounded sandwich". The objects are **root interval**,
**survivor set**, **focal-horizon hierarchy/interval**; gates and ledgers
say *containment*.

*Fixed by.* CBS-A3; FH-A2; the FH4 audit's one BLOCK (B1), fixed at
`b6de5a25` (2026-09-04).

*Owner.* [walt-focal-horizon-era](walt-focal-horizon-era.md).

### Scheme · Fix · role · presence mass

*Plain.* A small language for saying things about a 42 position exactly —
"my partner can overtake this trick", "an offer of count is on the table" —
and evaluating them over every hidden world.

*Precise.* **Scheme** is the executable relational language `walt::scheme`
(2026-09-06) over exact worlds; a **Fix** is an expression
`(fix (roles (sort name) ...) (out name ...) case ...)` — typed **roles**
over the finite sorts (28 dominoes, four chairs, eight contexts), cases of
registered relations, an output projection; distinct role names of the same
sort denote distinct objects within a case unless identified; omitted roles
are existential. A query's summary over a belief returns the exact event
probability, **per-answer presence masses** (the belief mass on worlds where
a given answer binding is present), the distribution of entire answer sets,
and separately named certainty properties; conditioning refuses an
impossible event. Enumeration has explicit world/work caps that exclude,
never sample.

*Guards.* Expression from compression: Scheme was invented (v0.4 §3–5) to
compress and was commissioned to express; no player, no general
compression claim, no move-scoring bonuses.

*Fixed by.* `walt/scheme/README.md`, `VALIDATION.md`; Jason's direction of
2026-09-06 ("invented to compress, use to express").

*Owner.* [walt-scheme-fix](walt-scheme-fix.md).

### Six-way result ladder

*Plain.* Every sampled evaluation says exactly what kind of answer it is.

*Precise.* `ExactFiberRoot` / `ExactFrozenSet` / `DeltaSettled` /
`EpsilonEquivalent` / `Unresolved` / `HeuristicFallback` — mechanically
distinct in every new API, log and report. **`Unresolved` is a successful
output.** Fixed-count play paths are `HeuristicFallback` until retyped. The
field-swap kinds (`FieldStableExactRoot` … `FieldUnresolved`) and the
root-interval kinds extend it; no UI or bridge flattens them into one
unlabeled percentage.

*Fixed by.* CE-A3 (2026-08-24); L2-A3; CBS-A3.

*Owner.* [walt-calculated-evidence](walt-calculated-evidence.md).

### Strategy fusion (merge-before-max)

*Plain.* The mistake of planning as if you will later know which hidden
world you are in — the thing 42 players punish.

*Precise.* `Q(B)` is a max over lawful policies taken *after* hidden worlds
with the same public history are merged; reversing the order (max per world,
then average) is strategy fusion and gives an upper bound, not a value. The
fence is O34 and is restated at every level: two disjoint seed policies give
two lower witnesses and a grammar, never coverage of omitted policies
(CBS-A4); in model space, branch by public action, never by hidden type —
a policy keyed on hidden type is unconstructible (MB-A7, MB-I4); the
`V^{π_k} = L_k` gate fails iff some max was taken over something other than
one public information state (FH-A7). The **fusion price** is what the
relaxation costs.

*Guards.* A value from an upper bound; lawful from clairvoyant.

*Fixed by.* O34 (SCENARIO-PLAYER); CBS-A4; MB-A7; FH-A7; `walt/MAP.md`
"The one question".

*Owner.* [walt-math-reference](walt-math-reference.md).

### Substantive vs vacuous

*Plain.* Whether a bound or a receipt actually says anything.

*Precise.* An upper that includes every remaining point is **vacuous by
inspection** (APS-A3, §70 falsifier); a God-tight receipt with nothing left
to save is **degenerate** and typed apart from a **substantive** one (U0:
twelve substantive, six degenerate of eighteen); a verifier check that
asserts `1/2 < 3/4` is a literal illustration carrying no weight (FH-A1);
conformance that cannot fail is the W10 caveat. The vacuity discipline
keeps a corpus of vacuous zeros from diluting a horizon measurement.

*Fixed by.* APS-A3; U0-REPORT; FH-A1; PG-A8.

*Owner.* [walt-focal-horizon-era](walt-focal-horizon-era.md).

### Support vs belief vs analyst conditioning

*Plain.* What the rules allow, what the seat thinks likely, and what the
experimenter decided to hold fixed.

*Precise.* **Rule support** is a Boolean compatibility predicate: the
compatible complete deals, the current remainder fiber, the legal-prefix
reachable image. **Belief** is a probability measure on compatible latent
worlds from the chance law and the likelihood of discretionary actions under
a policy model; two histories can have identical support and different
likelihoods (INFO-11); different valid models reverse Bayes factors on the
same action (BEL-14); kernel fields and random-tape fields must never be
mixed (BEL-03A). **Analyst conditioning** is the experimenter's declared
choice of world set and prior — a fixed hand, a frozen world list, a uniform
prior over all mechanically compatible deals with earlier actions *not*
reweighted (the gym's "explicit experimental prior"), a deal stream split
from the belief stream (O27) — recorded in the result's identity, and
neither the rules nor the seat's belief.

*Guards.* The layer table: physics / information / rule support / belief /
field / value, each a different object (Thesis §6; Math §1). Belief is not a
physical coordinate; policy is not legality.

*Fixed by.* Thesis §6; Math §1, §6.7, §8; O27; `walt/gym/README.md`.

*Owner.* [belief-vs-support](belief-vs-support.md), [walt-gym](walt-gym.md).

### Tail

*Plain.* What walt assumes it will do after it stops thinking exactly.

*Precise.* The **lower tail** is a fixed lawful policy π (executable,
information-consistent) that the hierarchy returns to after k exact focal
layers; the report of record's primary tail is σ0 driving the viewer seat
(σ0-as-focal), whose identity includes the contract; a gate-only second tail
is `FixedPreference::lowest_first`. The **upper tail** is the world-revealed
God continuation G (a Bellman supersolution, FH-God). Tail quality controls
only how early `L_k` and `U_k` meet; the hierarchy is complete regardless.
Off-DAG, the extractor completes by lowest tile index; for a σ0 tail the
off-DAG continuation must be σ0.

*Guards.* Tail from search: at k ≥ 1 the residual width is the tail's
policy gap — a better lawful tail buys more than a deeper search (FH3).

*Fixed by.* FH §4–5; FH-A3, FH-A4, FH-A7.

*Owner.* [walt-focal-horizon-era](walt-focal-horizon-era.md).

### Thread — CE-thread · L2-thread

*Plain.* Two questions that must not be mixed: how many worlds to sample,
and which model of the other seats to use.

*Precise.* **CE** = sampling depth (calculated evidence: anytime-valid
settlement, information rate, epochs); **L2** = model choice (the field as a
level, exposure rungs, the field as hidden state). Every adjudication note
labels its thread; the only mixing point is that L2 consumes CE baselines
through the sanctioned crossing (§20's risk-ledger rule). The
focal-horizon hierarchy is fixed-field L2-thread mathematics with CE
consumed nowhere.

*Fixed by.* Jason's standing framing (2026-08-24); the "Threads:" line of
every authorization note since.

*Owner.* [walt-calculated-evidence](walt-calculated-evidence.md).

### Tie rule (`TieRule::LowestTileIndex`)

*Plain.* What walt plays when two tiles are exactly as good.

*Precise.* At an exact tie the extractor materializes `π_k` under
`TieRule::LowestTileIndex`, and `best_of` keeps the incumbent, so a tied seat
plays the lowest-index tile. The tie rule changes the policy, never the
value (FH-A7). Because pmake pins at 1 in a locked hand, count is hoarded
deterministically at ties — the Gran 6-4 (index 25 of 28) is held because
it is a high-index tile (a source reading, not executed; the readout of
2026-09-05). The two levers at exact indifference are the objective and the
tie rule; neither is a feature.

*Fixed by.* FH-A7; `MORNING-2026-09-05.md` items 1–4.

*Owner.* [walt-gran-anchors](walt-gran-anchors.md).

### Tier labels

*Plain.* How much to trust a statement, stamped on the statement.

*Precise.* In order, never promoted or blurred:

1. **corpus** — the packages' own statuses ("Theorem — proved", "Theorem —
   exhaustive finite verification"); the ground truth;
2. **kernel** — a Lean theorem checked by the kernel (42/42 P0 rows;
   external `PASS` is never imported as an axiom, TRUST-01);
3. **exchange CONFIRMED** — program executed ALL_PASS plus 3/3 adversarial
   referees SOUND (x:008 is 2/3 and says so; x:009 is PARTIAL);
4. **rob receipt** — a byte-diffed Rust reproduction; evidence, never a
   status change;
5. **EXPLORATORY** — everything under `walt/`, `experiments/`, the ideas,
   analysis and field pages; below every tier above and cited by nothing
   above it. Inside it, a gate outranks a probe record outranks prose.

Every substantive statement in the wiki carries its label; dissents travel
verbatim (REACH-20's 2/3 panel is never presented as 3/3).

*Fixed by.* [Home](Home.md); `CLAUDE.md` hard rules.

*Owner.* [Home](Home.md), [claim-ledger](claim-ledger.md).

### Two recursions running in opposite directions

*Plain.* Jason's frame for what 42 is, computationally.

*Precise.* Backward enumerable exactness from trick 7 (free at t7,
microseconds at t6, seconds at t5, minutes at t4, the wall at t3) against
forward sampled/structural play from trick 1; on the receipt corpus they
trade dominance between trick 4 and trick 5 (U0). Recorded in
`walt/FACTOR-BELIEF.md` (the U0 paragraph), BRIEF-MB1, BRIEF-UP0 and the
`unified.rs` module doc (`Recursion::direction`). A frame, not a theorem.

*Owner.* [walt-focal-horizon-era](walt-focal-horizon-era.md), [walt-program](walt-program.md).

### θ vs ϑ

*Plain.* Two thresholds that used to share a letter.

*Precise.* **θ** is the pivotal win share `(1 + τ)/2` in calculated
evidence; **ϑ** is an auction/policy threshold — the calibrated bidding
default 11/16 is a ϑ. Code fields named `theta` that mean ϑ keep their
serialized names; docs and new code use the split.

*Fixed by.* CE-A2 (2026-08-24), superseding the signed-pivotal companion's
never-bare-θ proposal.

*Owner.* [walt-calculated-evidence](walt-calculated-evidence.md), [walt-seat-play](walt-seat-play.md).
