[Home](Home.md) · owns: the kernel mechanization — the `lean/` library as an artifact, the tier it occupies, the discipline that governs it, what it has proved, what those proofs do and do not certify about the rest of the repository, and how to build and extend it · Sources: [lean/README.md](../lean/README.md), [lean/PROOFS.md](../lean/PROOFS.md), the 27 files under `lean/Texas42/` (statements quoted from source, file:line as of 2026-09-07, commit c00717d1); **v0.7** `60_PROOF_ASSISTANT_HANDOFF.md` §§2, 9, 13, 14 and `65_MECHANIZATION_LEDGER.md`; **rec** `60_PROOF_ASSISTANT_KERNEL.md` §§3, 6, 8; `exchange/README.md` rows 011/013/015; `walt/ci/check.sh`, `walt/ci/check_m2_metal.sh`, `walt/ci/verify_m2_sources.sh`; `walt/CENSUS-RULINGS.md` GT1-A8, GT1-A17, GT1-A23, GT1-A24, FZ-A4; `walt/GPU-NATIVE-TRICK1-M3.md` §12; `kanban/backlog/lean-catchup.md`; git history of `lean/`. Related: [proof-assistant-plan](proof-assistant-plan.md) (the plan: trust boundary, K0–K15 spine, priorities, scoreboard), [lean-row-index](lean-row-index.md) (ledger row → declaration), [verification](verification.md), [rob](rob.md), [walt](walt.md), [walt-gpu-native-trick1](walt-gpu-native-trick1.md).

# The Kernel — `lean/`, the proof-assistant mechanization

**Tier labels used on this page.** Every substantive statement below carries
one: **[kernel]** — a declaration under `lean/Texas42/` that the Lean kernel has
checked over at most `propext`, `Classical.choice`, `Quot.sound`;
**[corpus]** — a status the ingest packages assign; **[exchange]** — an
exchange-adjudicated result; **[bookkeeping]** — a count taken against the
ledger or the source tree; **[engineering]** — a fact about scripts, builds,
timings, or commits; **[measured 2026-09-12]** — a fresh measurement run on
this machine for this page; **[exploratory]** — anything under `walt/`, cited by
nothing above it; **[unverified]** — a claim someone made that no artifact on
this machine supports.

---

## 1. What this is, in three registers

**For the newcomer.** Everything else in this repository produces *evidence*:
a Python verifier prints `PASS`, a Rust program reproduces a number, an outside
model's answer survives three referees. This directory produces *theorems*. A
statement that builds under `lean/` has been checked, step by step, by a small
program (the Lean kernel) against a fixed list of three logical axioms, and no
amount of testing, prose, or agreement substitutes for that. The statements
proved here are the game's rules and its exact information structure — how
many tiles there are, who wins a trick, what a player can and cannot know about
the hidden hands — not a strategy and not a player.

**For the mathematician.** `lean/` is a Lean 4 + mathlib4 Lake project
formalizing the Straight Texas 42 foundation as fixed by the two immutable
ingest packages and reconciled in this wiki. Its objects are the finite algebra
of 28 dominoes under nine declarations, the objective contracted hand, the
viewer's rule-derived hidden support as three dependent capacity cells, the
support normal form as the coarsest exact quotient, exact rational Bayes
conditioning on the fiber, strategic sufficiency of the belief state, and the
90-world witness that separates support from belief. All 42 priority-0 rows of
the v0.7 mechanization ledger are kernel-proved **[kernel; scoreboard
bookkeeping]** (2026-08-02, commit d190b264). The reflection targets and the
priority-1 rows listed in §10 are open.

**For the engineer.** 27 `.lean` files, 8,504 lines, 365 theorem/lemma
declarations **[bookkeeping, counted 2026-09-12]**; toolchain
`leanprover/lean4:v4.33.0-rc1`, mathlib at `79d0395a` **[engineering]**;
`lake build` from `lean/` after `lake exe cache get`; incremental full build of
the priority-0 library 47 s once the cache is warm **[engineering, commit
d190b264]**. Two trick-1 modules are built and axiom-audited by
`walt/ci/check.sh` on every run; the other 20 modules are built by no gate
(§7.3). Build instructions and the receipt format are in §12.

---

## 2. Status as of 2026-09-07 (c00717d1)

| Item | Status | Tier |
|---|---|---|
| Priority-0 ledger rows | 42 of 42 kernel-proved (13 A, 7 B, 9 C, 7 D, 5 E, 1 F) | kernel; ledger recount |
| Axiom hygiene | no `sorry`, no `native_decide`, no `axiom` declaration in any of the 27 files (grep, 2026-09-12); receipts for the priority-0 modules live in commit messages, for the M2 module in a committed file diffed by CI | engineering |
| Rows above priority 0 also proved | PA-A15, A16, A17, B03 in full; B05, C08, C15 in part | kernel |
| Open ledger rows | 20 priority-1 rows and three half-rows (§10); 11 at priority 2, one at priority 3, two at priority 4 | bookkeeping |
| Trick-1 modules (`Trick1Foundation`, `Trick1MetalFoundation`) | 25 + 8 theorems, kernel-checked as statements about Lean-defined arithmetic; built and receipt-diffed by `walt/ci/check.sh`; what they certify about walt is exploratory (§7) | kernel as arithmetic; exploratory as meaning |
| `Trick1PerfectRecallNet` tree (M3) | facade + 8 submodules, 103 `#print axioms` lines; **statements committed 2026-08-17 in a commit titled "does not build"; build and audit unverified as of 2026-09-07** (§7.4) | unverified |
| Constellation modules (x:013/x:015) | sorry-free, kernel-evaluated; self-contained, unreconciled, no ledger row (§8) | kernel; exchange provenance |
| Last commit touching `lean/` | 97ce321a, 2026-08-17 | engineering |
| Walt-era Lean programs | CBS-O1..O15, PS-T1..T15, MB-O1..O20, SC-O1..O15, the lean-catchup card: accepted, zero kernel coverage, ledgered in no file under `lean/` (§10) | exploratory |

---

## 3. The tier ladder and the trust boundary

The kernel is the **second** of the four evidentiary tiers, stated on
[Home](Home.md#evidentiary-tiers--never-promoted-never-blurred) as:

> 1. **Corpus statuses** — the packages' own labels ("Theorem — proved",
>    "Theorem — exhaustive finite verification", …); the ground truth.
> 2. **Proof-assistant kernel** — the target tier; external `PASS` is never imported
>    as an axiom (TRUST-01). First theorems landed in `lean/`.
> 3. **Exchange-adjudicated CONFIRMED** — external result; program executed ALL_PASS
>    plus 3/3 adversarial referees SOUND. Not a corpus theorem, not a kernel proof.
> 4. **rob conformance receipts** — byte-diffed Rust reproductions; `x-` prefixed
>    lines back exchange numbers. Evidence, never a status change.

Below all four sits the exploratory tier — everything under `walt/`,
`experiments/`, ideas and analysis — cited by nothing above it.

The trust boundary is **v0.7** Handoff §2, verbatim:

> Use the following hierarchy.
>
> ```text
> adopted Straight rule profile
>         ↓ definitions
> kernel-checked mathematical theorems
>         ↓ proved refinement
> extracted/reference executable functions
>         ↓ conformance
> external Python receipts and production implementations
> ```
>
> The two Python verifier entry points are valuable regression oracles and
> witness generators. They are outside the proof kernel. A finite result becomes
> a kernel theorem only when one of these holds:
>
> - it has a direct formal proof;
> - an internal Boolean decision procedure is proved equivalent to the
>   proposition and kernel evaluation closes the case;
> - a reflection procedure is proved sound and the kernel checks its certificate.
>
> Do not import `PASS` output as an axiom in the final foundation.

That is TRUST-01. Its working consequence, in the words of
[lean/PROOFS.md](../lean/PROOFS.md) hard rule 3: external receipts — Python
verifiers, rob, and (since 2026-08-16) walt's gates — are evidence *for us*,
never premises *for the kernel*. A number rob has verified does not arrive in
Lean by citation; if it is worth having in the kernel, the kernel recomputes it.

Two consequences people find counterintuitive, stated plainly:

- **The tiers order kinds of evidence, not amounts.** Lean does not outrank a
  rob receipt because it is newer or more thorough. A kernel proof is a
  mechanically checked derivation from stated axioms; a receipt is a report that
  a program agreed with an expected string. The former can be wrong only if the
  definitions do not say what the specification meant; the latter can be wrong
  in many more ways. This is why §4 fixates on definitions.
- **A green receipt is evidence, never a status change.** The reverse also
  holds: a kernel theorem about Lean-defined arithmetic does not certify the Rust
  or Metal program it was written beside. §7 is the worked example.

---

## 4. The discipline

The methodological position of `lean/` is not a style guide. It is recorded in
[lean/PROOFS.md](../lean/PROOFS.md) and restated here with the measurements that
justify it.

### 4.1 The five hard rules (PROOFS.md, verbatim)

> 1. **No `sorry` on the main branch.** In-progress statements live on PR
>    branches only. A slice merges when it builds sorry-free.
> 2. **No `native_decide`, no new axioms.** Every theorem must close over the
>    standard axioms only (`propext`, `Classical.choice`, `Quot.sound`).
>    Verify with `#print axioms` in a scratch file before committing —
>    this is the receipt; record it in the commit message.
> 3. **Never import an external `PASS` as an axiom** (TRUST-01). External
>    receipts (Python verifiers, rob) are evidence for *us*, never premises
>    for the kernel.
> 4. **Definitions are the trust surface.** The kernel checks proofs; nothing
>    checks that a definition matches the ingest spec except review. Every
>    definition carries a docstring citing its ingest source (`Math §x.y`,
>    ledger row `PA-xNN`). When two packages disagree, the wiki's resolution
>    (`wiki/discrepancies.md`) governs; note it in the docstring.
> 5. **Statement fidelity over proof convenience.** Do not weaken a statement
>    to make it provable without flagging it loudly. If a ledger row can only
>    be proved in a weaker form, that is a *finding* — surface it, don't bury
>    it.

Rule 2 now has two receipt forms **[engineering]**: the commit-message block
(every priority-0 commit; the house format is in §12.4) and, since 2026-08-17, a
committed file regenerated by `lake env lean <file>` and byte-diffed by CI
(`lean/trick1_metal_foundation_axioms_v1.txt`). PROOFS.md's text names only the
first form; the second is recorded in `lean/README.md`'s status note and here.

Rule 4 is the one that deserves emphasis. A mechanized proof transfers all the
risk to the statement: once the kernel accepts it, the only remaining way to be
wrong is to have proved something other than what the specification meant. That
is why every definition carries a citation to the ingest section it formalizes
(48 distinct `PA-` rows are cited in source **[bookkeeping]**; the two citation
gaps are in [lean-row-index](lean-row-index.md)), why
[discrepancies](discrepancies.md) governs when the packages disagree, and why
the source of truth is *ingest as reconciled by the wiki*
([lean/README.md](../lean/README.md) "Authority order").

### 4.2 `decide` versus `native_decide`: who computes

Both tactics discharge a decidable proposition by computing it. `decide`
reduces the proposition inside the Lean kernel — the small audited program the
whole edifice already rests on — so a `decide` proof adds no trusted component;
it *is* kernel evaluation, which is exactly what the trust boundary's second
bullet permits. `native_decide` compiles the proposition to machine code, runs
it, and asks the kernel to believe the result; that imports the Lean compiler,
the runtime, and the C toolchain into the trusted base and shows up as an extra
axiom under `#print axioms`. This library has none **[engineering, grep
2026-09-12]**.

PROOFS.md is therefore permissive rather than grudging:

> Kernel `decide` is a legitimate proof (it *is* kernel evaluation — allowed
> by the trust boundary, unlike `native_decide`). Use it freely for
> finite-arithmetic facts: cardinalities, census counts, witnesses, per-domino
> case sweeps.

Measured capacity: a 9 × 8 × 28 × 28 sweep (about 56,000 trick-key comparisons,
`swap23_transport_iff`) elaborates in seconds; permutation spaces
(`Equiv.Perm` enumeration) and 7⁷ function sweeps do not kernel-reduce in
reasonable time, so `countPreserving_iff` uses the specification's analytic
forcing argument instead **[engineering, PROOFS.md]**.

### 4.3 Where structure is required anyway

Permission to compute is not permission to stop thinking. The unique trick
winner is derived from key injectivity — a structural argument about shared
pips (PA-A10 → PA-A11) — with `decide` nowhere in the chain, because the spine
(rec Kernel §3 K3) demands the *reason*: "The formal proof should use
injectivity of nonzero trick keys, not case enumeration." The 737,100-case
agreement with an independent prose resolver stays a separate, deliberately
deferred reflection target (PA-A12, priority 2) rather than being conflated with
the theorem. The constellation modules (§8) make the opposite choice one level
down and the page says so.

### 4.4 `decide +kernel` and the cost model

Plain `decide` evaluates the proposition in the *elaborator* (`Meta.whnf`), not
the kernel — orders of magnitude slower, gigantic terms, no sharing.
`decide +kernel` produces the same proof term with the same axioms, evaluated
where it should be. The measurements, from commit d190b264 and PROOFS.md
**[engineering]**:

| Obligation | Under plain `decide` | Under `decide +kernel` |
|---|---|---|
| `Witness.lean` whole (180 rollouts + 90 replay checks) | out-of-memory kill after 16 h | 33 s |
| Full priority-0 library, incremental | — | 47 s |
| Four rollout anchors inside `refine ⟨by decide, …⟩` | a two-day hang at line 144 | seconds, once stated as top-level lemmas assembled with `exact ⟨…⟩` |
| 8-component replay fingerprint `DecidableEq` | "failed to synthesize" at the default instance-size cap of 128 | `set_option synthInstance.maxSize 2000 in` |

The rule that follows: use `decide +kernel` for anything that evaluates game
machinery. Read it as a rule for new work rather than a description of the
library — all 35 `decide +kernel` uses are in `Witness.lean` **[bookkeeping,
grep 2026-09-12]**; `Transport.lean`, the constellation modules and the trick-1
modules use plain `decide` for their finite sweeps.

Two further idioms the witness forced **[engineering]**: no `ℚ` inside `decide`
(`Rat` normalisation runs a well-founded `Nat.gcd` that does not kernel-reduce
— compute integer moments by `decide`, lift to exact rationals analytically),
and amortise repeated kernel evaluation through verified tables (`decide` each
expensive column once into a literal table, then every downstream sum is
cheap). About twenty such idioms are in PROOFS.md; §12.5 lists the headline
ones. Most exist because their absence produced a multi-hour failure once.

### 4.5 Statement fidelity, with two worked examples

Rule 5 says a weakened or corrected statement is a finding to report. The
library has two:

**The `hd_allowed` finding (PA-C07, `Cells.lean`) [kernel].** The completeness
half of the losslessness induction — every member of the derived cell fiber is
realized by a rule-compatible deal — needs a fact the prose leaves implicit: a
hidden seat's publicly played tile must respect that seat's previously recorded
voids. The docstring of `exists_deal_of_isWorld` (`Cells.lean:726–735`) says
so, and the proof derives it at line 884 (`have hd_allowed : d ∈ v.allowed …`)
from true-trajectory void soundness (`Coheres.voids_sound`) rather than
assuming it. The theorem is proved in full; the finding is about what the prose
proof silently used.

**The x:011 correction (constellation thread) [exchange provenance].**
Dispatch 011 asked for key injectivity as the unique-winner theorem's
statement. Pro's response — an honest refusal to fabricate a compilation claim
— caught the specification error: tier-zero trick keys tie by design, so the
theorem is unique *winner*, not key injectivity. The correction was accepted
and the staged build that followed (013, 015) proved `unique_winner` in that
form (`exchange/README.md` row 011). A mechanization attempt catching a
dispatch's mis-statement is the same phenomenon as `hd_allowed` from the other
side.

---

## 5. The spine as proved (K0–K15)

The rec Kernel §3 dependency spine, walked layer by layer with the headline
theorem's exact statement and location. Everything in the "Proved" column is
**[kernel]**; row IDs are the v0.7 ledger's; the "Not in the kernel" column
names what the layer leaves external or open. The full row-by-row map is
[lean-row-index](lean-row-index.md).

### K0–K1 Finite universe and count (`Basic.lean`, PA-A01–A04)

```lean
theorem card_domino : Fintype.card Domino = 28                         -- Basic.lean:60
theorem total_countPoints : ∑ d : Domino, countPoints d = 35           -- Basic.lean:77
theorem card_incidence : ∀ p : Pip, (incidence p).card = 7             -- Basic.lean:106
theorem incidence_inter : ∀ p q : Pip, p ≠ q → …  -- σ_p ∩ σ_q = {p:q}  -- Basic.lean:118
```

`Domino` is the canonical `(high, low)` pair with `low ≤ high`, exactly as
Handoff §3 recommends. Not in the kernel: nothing at this layer.

### K2–K3 Declaration mechanics and trick order (`Trick.lean`, PA-A05–A11, A13–A15)

```lean
theorem card_declaration : Fintype.card Declaration = 9                -- Trick.lean:38
theorem card_suit : Fintype.card Suit = 8                              -- Trick.lean:49
theorem tier_ledSuit_pos (δ : Declaration) (d : Domino) : …            -- Trick.lean:150  (PA-A09)
theorem eq_of_key_eq {δ : Declaration} {q : Suit} {d₁ d₂ : Domino}
    (h₁ : δ.tier q d₁ ≠ 0) (hk : δ.key q d₁ = δ.key q d₂) : d₁ = d₂    -- Trick.lean:234  (PA-A10)
theorem existsUnique_winner (δ : Declaration) (plays : Fin 4 → Domino)
    (hinj : Function.Injective plays) :
    ∃! i : Fin 4, ∀ j : Fin 4,
      δ.key (δ.ledSuit (plays 0)) (plays j)
        ≤ δ.key (δ.ledSuit (plays 0)) (plays i)                        -- Trick.lean:319  (PA-A11)
```

The winner theorem needs only distinctness and a designated lead — legality of
the three follower plays is not assumed. `beats_exact` (:277, PA-A13),
`threat_removal_mono` (:295, PA-A14) and the no-trump lead-threat
incompleteness witness `lead_threat_incomplete` (:304, PA-A15: `0:0` and `1:1`
have empty when-led threat sets yet different follow behaviour) close the
layer. Not in the kernel: the 737,100-case prose-resolver agreement (PA-A12,
REFLECT, priority 2) — **[corpus]** finite verification and **[rob]** receipt
`r_alg_prose_agreement` only.

### K4 Transports (`Transport.lean`, PA-A16–A17)

```lean
theorem countPreserving_iff (σ : Equiv.Perm Pip) :
    CountPreserving σ ↔ σ = 1 ∨ σ = Equiv.swap 2 3                     -- Transport.lean:95
theorem swap23_transport_iff : ∀ δ : Declaration,
    (∀ (q : Suit) (d e : Domino),
      (δ.mapPips swap23).Beats (q.mapPips swap23)
          (d.mapPips swap23) (e.mapPips swap23) ↔ δ.Beats q d e)
      ↔ (δ = .pip 2 ∨ δ = .pip 3)                                      -- Transport.lean:201
```

The first is proved by the specification's forcing argument (`5:5` forces
`σ(5) = 5`; `6:4` forces `{σ(6), σ(4)} = {6, 4}`; …), not by sweeping 5,040
permutations; the second is a 9 × 8 × 28 × 28 kernel `decide`. The docstring
caveat travels with it: PA-A17 preserves the game-semantic order reduct, not
numeric rank labels. Not in the kernel: K4's census of the three unscored
mechanics classes as a theorem, and the seat/slot gauges (PA-F01–F04).

### K5 Objective contracted hand (`Auction.lean`, `Deal.lean`, `Play.lean`, PA-B01–B03, B05–B10)

```lean
theorem mark_le_ceiling {cfg : AuctionConfig} {a : AuctionState}
    (h : LegalAuction cfg a) {m : ℕ} (hm : Bid.mark m ∈ a.bids) :
    m ≤ min cfg.maxMarkBid 5                                           -- Auction.lean:207  (PA-B03)
theorem mark_five_reachable {cfg : AuctionConfig} (hcap : 5 ≤ cfg.maxMarkBid) :
    ∃ a : AuctionState, LegalAuction cfg a ∧ Bid.mark 5 ∈ a.bids      -- Auction.lean:216  (PA-B03)
structure Deal where …  -- four labelled 7-tile pairwise-disjoint hands  -- Deal.lean:23    (PA-B05 define)
theorem inv_step {X : PlayState} (hX : X.Inv) {d : Domino}
    (hd : d ∈ X.legalSet) : (X.step d).Inv                             -- Play.lean:359    (PA-B09)
theorem gamma_init (ω : Deal) (K : Contract) : (init ω K).gamma = 28   -- Play.lean:526    (PA-B10)
theorem terminal_scores {X : PlayState} (hX : X.Inv) (hT : X.Terminal) :
    X.tricksDone = 7 ∧ X.score 0 + X.score 1 = 42                      -- Play.lean:559    (PA-B10)
```

`tricksDone` and `scoredTiles` are functions of the state, never fields — the
kernel-side form of the derived-views discipline (TYPE-02). Not in the kernel:
the deal cardinalities `28!/(7!)⁴ = 472,518,347,558,400` and `21!/(7!)³ =
399,072,960` *as cardinalities of `Deal`* (PA-B05's prove half — the second
number is proved only as a multinomial identity in `Trick1Foundation`, §7.1);
the auction census `2380…3214` for caps 1..7 (PA-B04, REFLECT, priority 3);
the graded play graph, Markov congruence and settlement rows (PA-B11–B14).

### K6 Information and remainder (`Cells.lean`, `Information.lean`, PA-C01–C04, F05)

`PubState` (the public record: played-by-seat, voids, declaration),
`ViewerCtx` with derived `pool`, `allowed`, `capacity`, `IsWorld`, `Compatible`
and `remainder` are the definitions (PA-C01–C04); and

```lean
theorem mech_not_injective :
    ∃ I₁ I₂ : DealLocalInfo, I₁ ≠ I₂ ∧ I₁.mech = I₂.mech               -- Information.lean:47  (PA-F05)
```

is Math §6.6's witness: two perfect-recall records differing only in which
losing seat opened `P(30)` have identical mechanical projections.

### K7 Generic capacitated cell kernel (`Reduction.lean`, `NormalForm.lean`, PA-C15 backbone, C08 groundwork)

```lean
theorem red_red (C : CellSys H α) : C.red.red = C.red                  -- Reduction.lean:90
theorem fiber_eq_iff_red_eq {C Q : CellSys H α} (hpool : C.pool = Q.pool)
    (hcap : C.cap = Q.cap) :
    (∀ A, C.IsWorld A ↔ Q.IsWorld A) ↔ C.red = Q.red                   -- Reduction.lean:112
theorem exists_partition_of_hall (W : Finset α) (r : H → ℕ) …          -- NormalForm.lean:383
```

Formalized generically over a holder type `H` and tile type `α` first, then
specialized: `isWorld_iff_cellSys` (`Reduction.lean:152`) instantiates the
game's cells. The capacitated Hall lemma is by slot expansion into mathlib's
Hall theorem, exactly the route rec Kernel §3 K7 allows. Not in the kernel: the
game-level Hall/max-flow equivalence (PA-C08's remaining half), the exact
capacity-DP count recurrence (PA-C11), the count-ratio sampler (PA-C12), the
local-vs-marginal holder witness (PA-C13), and the marginal-edge
characterization (PA-C14).

### K8 Straight cell losslessness (`Cells.lean`, PA-C05–C07, C09, C10)

```lean
/-- PA-C07 (exact Straight 42 cell support, Math §7.5): for every legal
public play prefix, the derived cell fiber equals the remainder image of
the rule-compatible complete deals — **the losslessness theorem**. -/
theorem losslessness (ω : Deal) (K : Contract) (v : ViewerCtx)
    (hv : ω.hands v.viewer = v.hand0) {ds : List Domino}
    (hds : (PlayState.init ω K).LegalFrom ds) (A : Seat → Finset Domino) :
    v.IsWorld (PubState.replay K ds) A ↔
      ∃ ω' : Deal, v.Compatible K ds ω' ∧ v.remainder K ds ω' = A       -- Cells.lean:1060
theorem remainder_injective (v : ViewerCtx) (K : Contract)
    (ds : List Domino) {ω₁ ω₂ : Deal} (h₁ : v.Compatible K ds ω₁)
    (h₂ : v.Compatible K ds ω₂)
    (h : v.remainder K ds ω₁ = v.remainder K ds ω₂) : ω₁ = ω₂          -- Cells.lean:1075  (PA-C09)
```

Soundness comes directly from the coherence invariant; completeness is
`exists_deal_of_isWorld` (`Cells.lean:736`), the specification's four-case
induction — viewer action, hidden lead, hidden successful follow, hidden slough
— carrying the `hd_allowed` finding of §4.5. With `remainder_injective` the
remainder map is a bijection between compatible deals and the fiber.
`voids_mono` (`Cells.lean:282`) is the upper-bound-only observation update
(PA-C06). This layer is the keystone of [support-fiber](support-fiber.md)
(CELL-05/07 at corpus tier). `Cells.lean` is 1,098 lines and 37 theorems.

### K9 Marginal support and the normal form (`NormalForm.lean`, PA-D01–D05)

```lean
theorem active_trichotomy {C : CellSys H α} (hH : Fintype.card H = 3)
    (hC : C.Feasible) :
    C.active.card = 0 ∨ C.active.card = 2 ∨ C.active.card = 3          -- NormalForm.lean:349   (PA-D01)
theorem decode_compile {C : CellSys H α} (hH : Fintype.card H = 3)
    (hC : C.Feasible) : C.compile.decode = C.red                        -- NormalForm.lean:1126  (PA-D03/D04)
theorem compile_decode {N : SupportNF H α} (hwf : N.WellFormed) :
    N.decode.compile = N                                                -- NormalForm.lean:1251  (PA-D04)
theorem fiber_eq_iff_totalNF_eq (hH : Fintype.card H = 3)
    {C Q : CellSys H α} :
    (∀ A, C.IsWorld A ↔ Q.IsWorld A) ↔ C.totalNF = Q.totalNF           -- NormalForm.lean:1423  (PA-D05)
```

With `strict_singleton_hall` (:241, the §7.11 strict inequality),
`feasible_decode` (:849) and `decode_marginal` (:912). No solver is imported
anywhere: feasibility flows from the linear ternary validator through Hall.
The longest module: 1,455 lines, 56 theorems. Not in the kernel: global
factorization/minimality (PA-D06), the SCC marginal compiler (PA-D07), strict
Hall irreducibility and essential exclusions (PA-D08), and the 81-bit census
`1,830,967,207,309,611,271,596,161` (PA-D17, "REFLECT or keep external",
priority 4) — that number is **[corpus]** finite verification, **[exchange]**
x:005 and **[rob]** `r_nf_census_81`, never kernel.

### K10 Dynamic support, K12 folded kernel, K13 seat-frame gauges, K14 future-equivalence minimum

rec-only layers. **No kernel coverage.** The matching-minor calculus, the
63-edge budget, the reduced viewer kernel `K = (δ, H_m, N, τ, α_U)`, the D₄
frame gauges and the Myhill–Nerode quotient are corpus-tier (rec) and, for
OPEN-01, exchange-tier (x:003) only — see [support-dynamics](support-dynamics.md),
[reduced-viewer-kernel](reduced-viewer-kernel.md).

### K11 Symbolic reachability (`Reachability.lean`, PA-D09–D10)

```lean
def Reachable (K : Contract) (v : ViewerCtx) (P : PubState) : Prop :=
  ∃ (ω : Deal) (ds : List Domino),
    ω.hands v.viewer = v.hand0
      ∧ (PlayState.init ω K).LegalFrom ds
      ∧ PubState.replay K ds = P                                       -- Reachability.lean:23
def CertifiedState (K : Contract) (v : ViewerCtx) : Type :=
  { P : PubState // Reachable K v P }                                  -- Reachability.lean:33
theorem CertifiedState.ext … (h : c₁.val = c₂.val) : c₁ = c₂            -- proof irrelevance surfaced as subtype extensionality
```

Reachability is a proposition; equality, hashing and serialization factor
through the projection alone. This is the kernel counterpart of rob's INV-3 and
the wiki's D3 ruling (necessary outer profile, never certificate — the Lean
identifier `CertifiedState` is Handoff §5's own name and is quoted as such).
Not in the kernel: the reachable support image (PA-D11), the 50 hidden-capacity
profiles (PA-D12), seven leadable contexts (PA-D13), lead-witness necessity
(PA-D14), witness-validator soundness (PA-D15), the feasible-but-unreachable
witness (PA-D16), and the 26–46-bit reachable interval (PA-D18, priority 4) —
the interval and its exchange-tier narrowing are owned by
[reachability](reachability.md), whose dissent caveats travel with them.

### K15 Finite belief layer (`Belief.lean`, `Strategic.lean`, `Witness.lean`, PA-E01–E03, E07, E10)

```lean
theorem physicalBelief_support_isWorld (prior : FinPMF Deal)
    (v : ViewerCtx) (π : Seat → PolicyKernel)
    (hπ : ∀ s v P d, 0 ≤ π s v P d) (K : Contract) (ds : List Domino)
    (hZ : 0 < ∑ ω', prior.mass ω' * bayesWeight v π K ds ω')
    {A : Seat → Finset Domino}
    (hA : A ∈ (physicalBelief prior v π hπ K ds hZ).support) :
    v.IsWorld (PubState.replay K ds) A                                  -- Belief.lean:274     (PA-E03)
theorem beliefVal_eq_exp_latentVal (P : BeliefProc S A O L) (σ : S → A)
    (n : ℕ) (s : S) (β : FinPMF L) :
    P.beliefVal σ n s β = β.exp (P.latentVal σ n s)                    -- Strategic.lean:153  (PA-E07)
theorem bestResponse_eq (P : BeliefProc S A O L)
    (Cls : Finset (S → A)) (hne : Cls.Nonempty) (n : ℕ) (s : S)
    (β : FinPMF L) :
    Cls.sup' hne (fun σ => β.exp (P.latentVal σ n s))
      = Cls.sup' hne (fun σ => P.beliefVal σ n s β)                     -- Strategic.lean:284  (PA-E07)
```

`FinPMF` is an exact rational finite PMF; `condition_mul` (:97) is the Bayes
chain rule; `likelihoodFrom_append` (:171) the history-likelihood product.
`physicalBelief_support_isWorld` is the precise kernel form of *support is not
belief* read from the other side: the posterior's support lies **inside** the
cell fiber, so the fiber bounds belief exactly without determining the weights.
`beliefVal_eq_exp_latentVal` is strategic sufficiency for a generic
finite-horizon viewer process with latent state: the belief-Bellman recursion
integrates the ground-truth latent value at every horizon, so every fixed
admissible strategy's value and any finite-class best response is a function
of `B = (c, e, β)`. The Straight-42 instantiation — wiring `CertifiedState` and
`physicalBelief` into a concrete `BeliefProc` — is not done (PA-E08 and the
priority-1 E rows). The witness is §6. Not in the kernel: physics-only
posterior (PA-E04), exponential tilt (PA-E05), forced-action
nondiscrimination (PA-E06), best-response existence (PA-E08), the
coordinate-only factorization criterion (PA-E09), the context-free domino
value counterexample (PA-E11), the threshold-utility reversal (PA-E12).

### 5.1 Which numbers are kernel-proved and which remain external

| Number | Where it is kernel-proved **[kernel]** | Still external |
|---|---|---|
| 28 dominoes; 35 count points; 7 tiles per incidence; 9 declarations; 8 led contexts | `Basic.lean`, `Trick.lean` | — |
| 28 plays, 7 tricks, 42 points | `Play.lean` `gamma_init`, `terminal_scores` | — |
| mark ceiling `min(cap, 5)`, mark 5 reached | `Auction.lean` | auction history census 2380…3214 (PA-B04) |
| `{id, 2↔3}` count-preserving maps; transport exactly at δ ∈ {2, 3} | `Transport.lean` | — |
| 90 worlds; posterior weight sums 210 and 120; the four values and four make probabilities of §6 | `Witness.lean` | minimality of 90 ([open-problems](open-problems.md) item 3) |
| 56,448 = 9·8·28·28 key comparisons | `ConstellationCore.positive_key_injective` (and `swap23_transport_iff`'s sweep) | — |
| suffix values −11 and 16 | `ConstellationSuffix.lean` examples | the C1 factorization theorem (x:009, exchange tier) |
| 399,072,960 = 21!/(7!·7!·7!) | `Trick1Foundation.openingDealCount_eq_multinomial` — as a `Nat` identity only | as `Fintype.card` of a deal type (PA-B05) |
| 420 = lcm(1..7) divisibility; 2²¹¹ ≤ D < 2²¹²; 2²¹⁶ ≤ 42D < 2²¹⁷; opening-cell counts 7980, 1140, 2166, 3408, 5172, 7800, 11730 | `Trick1Foundation.lean` | the projector formulas that produce them in Rust (§7.1) |
| 79,800 slots; 5,109,296 and 2,359,424 arena bytes; 46 GradeMatching tasks; ≤ 10 matching vectors | `Trick1MetalFoundation.lean` | Rust/Lean and Metal/Rust correspondence (§7.2) |
| 737,100 unique-winner / prose-resolver cases | — | PA-A12 (REFLECT, priority 2): corpus finite verification, rob `r_alg_unique_winner`, `r_alg_prose_agreement` |
| 472,518,347,558,400 ordered deals | — | PA-B05 prove half |
| 50 hidden-capacity profiles | — | PA-D12 (priority 2), rob `r_nf_capacity_profiles` |
| 81-bit census 1,830,967,207,309,611,271,596,161 | — | PA-D17 (priority 4): corpus, x:005, rob `r_nf_census_81` |
| 26–46-bit reachable interval, its exchange-tier narrowing and stratum count | — | PA-D18 (priority 4): see [reachability](reachability.md) with its dissent |

---

## 6. The 90-world witness, internalized whole

`Witness.lean` (757 lines, 39 theorems; commit d190b264, 2026-08-02) is K15's
named theorem and PA-E10, and "internalized whole" means something specific:
not one number in it is imported. **[kernel throughout]**

**The scenario (Math §10.4, as defined in the file).** Contract `P(31)`
no-trump, bid by seat 3 (`theContract : Contract := ⟨3, BidKind.point, 31,
Declaration.notrump⟩`). A common public prefix of five tricks — twenty plays,
in play order:

```
6:3 6:1 6:4 6:0 | 0:0 2:2 5:0 2:0 | 4:3 4:2 4:0 5:4 | 1:1 3:0 3:3 2:1 | 1:0 6:6 5:2 5:1
```

The viewer (seat 3) holds `{4:1, 3:1}`; the unseen pool is
`{5:5, 4:4, 3:2, 6:5, 5:3, 6:2}`; at the endpoint the banked scores are 2 (team
0) and 18 (team 1), seat 3 to lead, no trick pending (the
`encodeState` tuple asserted by `replay_check`).

**What is proved, piece by piece.**

| Piece | Declaration | Statement |
|---|---|---|
| the fiber has 90 worlds | `card_worldPairs` (:99) | `worldPairs.card = 90` — 6!/(2!)³, seat 0's pair × seat 1's pair, seat 2 takes the rest |
| the fiber is exactly the cells | `isWorld_iff` (:590) | `v.IsWorld Pend A ↔ (A 0, A 1) ∈ worldPairs ∧ A 2 = poolW \ (A 0 ∪ A 1) ∧ A 3 = ∅` — the endpoint cells are computed from the public record by kernel evaluation, not asserted |
| every world is rule-realized | `replay_check` (:692), `deal_props`, `rule_fiber` (:708) | each of the 90 complete deals is a genuine `Deal` and legally replays the twenty plays to the common endpoint — 90 kernel replays |
| two legal histories, one endpoint | `auction_histories` (:506) | `histA = [pass, P30, pass, P31]` and `histB = [P30, pass, pass, P31]` are both `LegalAuction` from shaker seat 3, `histA ≠ histB`, both result in `some (3, .point 31)` |
| same full support | `same_full_support` (:233) | `∀ i : Fin 90, 0 < μA.mass i ∧ 0 < μB.mass i` — the posteriors `μA`, `μB` are exact-rational Bayes conditionings (PA-E02) of the uniform fiber prior by the auction likelihoods (`lA ∈ {1/9, 4/9, 2/9}`, `lB ∈ {2/3, 1/3}`, keyed on where `4:4` sits); integer weight sums 210 and 120 |
| the value columns | `Q31_table`, `Q41_table`, `anchor_values` (:147) | `Q w d` = the signed differential `(score 1 − 18) − (score 0 − 2)` after a 7-ply rollout of the committed `PlayState.step` under the lowest-ID field policy — 180 deterministic kernel rollouts; the two displayed worlds give `Q = 10, −22` and `−22, 22` |
| the expectations | `expected_differentials` (:400) | `μA`: `Q(3:1) = −160/21`, `Q(4:1) = 10/7`; `μB`: `Q(3:1) = −217/30`, `Q(4:1) = −52/5` |
| the contract lens | `make_probabilities` (:438) | make ⇔ `Q ≥ 4`; `μA`: `1/3`, `16/35`; `μB`: `1/3`, `1/5` |
| the reversal | `posterior_action_reversal` (:477) | under `μA` the `4:1` lead is strictly better on both lenses; under `μB` the `3:1` lead is |
| the named theorem | `ninety_world_witness` (:728) | the seven-way conjunction: distinct legal histories with equal auction result ∧ fiber = the 90 worlds ∧ `card = 90` ∧ every world rule-realized ∧ identical full support ∧ both reversals |

**What it establishes.** Two legal histories with the same mechanical
endpoint, the same 90-world fiber and the same posterior support have opposite
optimal leads; mechanical state alone is not an exact strategic state. The
mathematics is owned by [belief-vs-support](belief-vs-support.md); at corpus
tier it is a finite verification, and since 2026-08-02 it is also a kernel
theorem. The remaining two §10.4 lenses are positive affine transforms of the
two proved and are not separately stated.

**What it does not establish.** That 90 is the least fiber size exhibiting the
flip — minimality is open ([open-problems](open-problems.md) item 3). The
values `Q` are those of one named field policy (lowest-ID); the theorem is
about that field, exactly as §10.4 is.

---

## 7. What the kernel says about the player

Three modules were added in the walt GPU-native trick-1 program
([walt-gpu-native-trick1](walt-gpu-native-trick1.md)). This section is where
the book teaches the reader to separate *theorem proved* from *what it
certifies*.

The rule, stated once: **a theorem in these files is [kernel] as a statement
about Lean-defined arithmetic and Lean-defined structures. Its correspondence
to the Rust and Metal programs that motivated it is unproved and named as debt
in the files' own docstrings. Therefore everything walt cites these theorems
for stays [exploratory].** Neither `Trick1*` module carries a ledger row; their
citations are GT1-A rulings, not `PA-` rows.

### 7.1 `Trick1Foundation.lean` (2026-08-16, commit 3b4c6d60; 480 lines, 25 theorems)

Module docstring: "It deliberately does not formalize the GPU implementation
or claim that its search is complete." Imports `Texas42.Play`. Kernel-checked
**[kernel as arithmetic]**:

| Group | Theorems | Statement |
|---|---|---|
| contract arithmetic (GT1-A2) | `legal_point_lossAllowance_le_twelve`, `legal_bid_lossAllowance_le_twelve` | every legal nonpass bid's derived loss budget is ≤ 12, mark contracts having budget 0 |
| field scale (GT1-A3) | `card_one_to_seven_dvd_420`, `scaled_cardinality_exact`, `legalSet_card_one_to_seven`, `legalSet_card_dvd_fieldScale` | a live legal set has 1..7 members and its size divides 420 |
| hand cap | `handCapSeven_init`, `handCapSeven_step` | seven-tile hands, initialized and step-preserved |
| magnitudes (GT1-A3) | `openingDealCount_eq_multinomial` (`399072960 = 21!/(7!·7!·7!)`), `rootDenominator_bit_window` (`2^211 ≤ D < 2^212`, `D = 399072960·420^21`), `utilityMagnitude_bit_window` (`2^216 ≤ 42·D < 2^217`) | `norm_num` identities |
| point invariant | `stateUnbankedPoints_conservation`, `_step`, `trickTiles_subset_stateUnbankedTiles` | `score 0 + score 1 + unbanked = 42` with the unresolved current-trick tiles inside the unbanked set, preserved by every legal step (from `PlayState.Inv`) — the repaired mid-trick decidedness bound |
| cell counts (GT1-A4) | `openingCellCount_values`, `openingCellCount_le_11730` | `openingCellCount m` for `m = 0..6` is `7980, 1140, 2166, 3408, 5172, 7800, 11730`; max 11,730 |
| interval algebra (GT1-A7) | `componentwise_upper_sum`, `componentwise_upper_bounds_action`, `shared_policy_lower_bounds_action`, `action_dominance`, `nonstrict_interval_certifies_optimal_member`, `strict_interval_certifies_unique_optimal` | componentwise uppers sum; lowers sum only under one shared policy; dominance; non-strict membership vs strict uniqueness |

Not proved, per GT1-A8 verbatim: "the semantic `(response,e)` partition, the
`A/C/W` formulas or global conservation, posterior stratification/factorization,
the exact information-key equivalence, canonical least-index verdict, sparse-DP
or meet-in-the-middle refinement, Rust/Lean correspondence, or Metal/Rust
correspondence. Those are explicit proof debt." So the cell counts are a
kernel theorem about the counting *formula*, and the fact that walt's projector
emits those counts is a **[exploratory]** receipt
(`walt/receipts/gpu_native_trick1_m0_m1_v1/`).

### 7.2 `Trick1MetalFoundation.lean` (2026-08-17, commit 813d5e81; 141 lines, 8 theorems)

Module docstring: "They deliberately do not claim Rust/Lean or Metal/Rust
semantic correspondence, nor do they formalize the opening projector's `A/C/W`
formulas." The eight, with the axioms each closes over per the committed
receipt **[kernel as arithmetic]**:

| Theorem | Statement | Axioms |
|---|---|---|
| `candidateSlotCount_le_79800` | `10·(3g)(3g−1)(3g−2) ≤ 79800` for grades 1..7 | propext, Classical.choice, Quot.sound |
| `projectorArenaBytes_eq` | `32 + 1936 + (79800 + 2)·64 = 5109296` | propext |
| `arithmeticArenaBytes_eq` | `16384·80 + (16384 + 2)·64 = 2359424` | propext |
| `gradeMatching_covers_every_grade` | every grade 1..7 has its `m = 0` coordinate | all three |
| `gradeMatchingTaskCount_eq_46` | `∑_{g=1}^{7} (min 6 (3g) + 1) = 46` | all three |
| `matchingVectorCount_le_ten` | every admitted `(grade, mask, matching)` has ≤ 10 matching-count vectors — checked over the complete finite domain by `decide` | all three |
| `filter_preserves_occursBefore` | stable filtering preserves `OccursBefore` | propext |
| `failed_conjunction_accepts_nothing` | a failed conjunction accepts exactly `⟨0 tasks, 0 payload bytes⟩` | propext |

Four of the eight depend on `propext` alone — the accurate wording for the
library is "at most the three standard axioms", not "all three". The status
this module supports in walt, "M2 METAL PROJECTOR PARITY COMPLETE under freeze
56", is **[exploratory]** (GT1-A17; [walt-math-freezes](walt-math-freezes.md)
row 56): it is an arithmetic/projector parity receipt, not a correspondence
theorem.

### 7.3 The CI wiring **[engineering]**

Contrary to the 2026-08-13 version of this page, a Lean gate exists, since
813d5e81 (2026-08-17):

| Script | What it does with Lean | When |
|---|---|---|
| `walt/ci/check.sh` (lines 332–341) | step `== Lean trick-1 foundations and exact axiom audit`: `lake build Texas42.Trick1Foundation Texas42.Trick1MetalFoundation`, then `lake env lean Texas42/Trick1MetalFoundation.lean` and `diff -u trick1_metal_foundation_axioms_v1.txt <output>`; `lakefile.toml` and `lake-manifest.json` are also in the no-float TOML/JSON check's file list | every run, in a scrubbed environment with the fixed `~/.elan/bin/lake` path |
| `walt/ci/check_m2_metal.sh` (lines 669–688) | `== final Lean foundation and exact axiom audit`: the same build and diff from an immutable exact-HEAD source snapshot with the live `.lake/packages` bound in read-only | the elevated freeze-56 Metal gate (hardware) |
| `walt/ci/verify_m2_sources.sh` (lines 275–277, 373, 383–387) | closes the complete `lean/Texas42` tree plus `Texas42.lean`, `lakefile.toml`, `lake-manifest.json`, `lean-toolchain` and the M2 receipt by SHA-256 in the cumulative source manifest | freeze events only (FZ-A5) |

The receipt file `lean/trick1_metal_foundation_axioms_v1.txt` (8 entries) is
the mechanical form of PROOFS.md rule 2; it is pinned by digest in both
freeze-56 source manifests (`walt/math/gpu_native_trick1_m0_m2_sources_v1.sha256`,
23 `lean/` entries, pre-M3; `_v2.sha256`, 32 entries covering every module).
**[measured 2026-09-12]**: running `lean Texas42/Trick1MetalFoundation.lean`
from this worktree against the main checkout's built oleans (via `LEAN_PATH`,
no `lake`, nothing written) reproduced the committed receipt byte-identically,
exit 0, 27 s wall.

What the gate transitively builds: `Basic`, `Trick`, `Auction`, `Deal`, `Play`,
`Trick1Foundation`, `Trick1MetalFoundation` — 7 of 27 modules. **The other 20 —
`Transport`, `Cells`, `Reachability`, `Information`, `Reduction`, `NormalForm`,
`Belief`, `Strategic`, `Witness`, both constellation modules and the nine-file
M3 tree — are built by no gate.** Their axiom hygiene rests on the commit
receipts of 2026-07-28 → 08-02 (e.g. d190b264's `Receipts` block naming ten
theorems), the main checkout's `.lake` oleans dated 2026-08-03, and today's grep.
The `lean/.github/` workflows are inert (GitHub runs workflows only from a
repository root, and there is no `.github/` at the root) and are kept against a
possible split. `rob/ci/check.sh` never touches Lean.

### 7.4 `Trick1PerfectRecallNet` — statements committed; build and audit unverified

The facts **[engineering, verified 2026-09-12 unless dated otherwise]**:

- **Shape.** `Texas42/Trick1PerfectRecallNet.lean` (a 129-line facade of 103
  `#print axioms` lines, no theorems of its own) plus eight submodules under
  `Trick1PerfectRecallNet/`: `Types` (6 theorems), `CodecReplay` (9),
  `LineageNetting` (9), `Recurrence` (9), `MassObjectives` (13), `Reduction`
  (23), `Bounds` (16), `Evidence` (18) — 103 theorems, 1,555 lines with the
  facade. Import chain `Types ← CodecReplay ← LineageNetting ← Recurrence ←
  MassObjectives ← Reduction ← Bounds ← Evidence ← facade`; `Types` imports
  `Texas42.Trick1Foundation`. The facade docstring names the debts: "Rust-to-Lean
  codec and replay correspondence, Metal-to-Rust kernel correspondence, general
  independent-oracle correctness, and grade-4-to-trick-1 transport remain named
  implementation debts."
- **What it states** (the obligations of `walt/GPU-NATIVE-TRICK1-M3.md` §12):
  H/C codec injectivity and disjointness; replay determination; unique
  parent/action lineage; complete-run netting; block-partition invariance;
  complete legal faces; sum-before-max as the free product of lawful
  perfect-recall policies; the carried-posterior mass `(1/1200)·∏(1/d)`;
  terminal scale/U256 limbs; the five-bucket differential and P30-make bridges
  (make ⇔ future defender spend ≤ 11); 216/432 continuation-code injections;
  the 769 aggregate reduction base; 168/12,600 sequence counts; 936/13,368
  command bounds; `< 2^21` semantic ranges; ABI/live-memory/spill ledgers;
  four-epoch and sixteen-treatment counters; all-or-nothing evidence
  composition.
- **How it was committed.** 97ce321a, 2026-08-17 09:45 −0500, titled "WIP: M3
  perfect-recall net scaffolding (mid-flight, does not build)"; body: "Committed
  only so the work survives the worktree." The same commit added the module to
  `lean/Texas42.lean`'s import list and the M3 paragraph to
  [proof-assistant-plan](proof-assistant-plan.md). It is the last commit to
  touch `lean/`.
- **No build artifact.** The main checkout's `lean/.lake/build/lib/lean/Texas42/`
  holds oleans for all 16 priority-0-era modules (2026-08-03) and for the two
  gated trick-1 modules (2026-09-03 23:09, a CI run) and **none** for
  `Trick1PerfectRecallNet` or its submodules; the survey of 2026-09-07 found none
  in any of 16 worktree `.lake` directories either. This worktree has no
  `.lake`.
- **No gate.** `grep -rn 'PerfectRecallNet\|perfect_recall' walt/ci rob/ci`
  returns nothing.
- **Receipt inconsistent.** `lean/trick1_perfect_recall_net_axioms_v1.txt`
  (committed in 97ce321a; 108 lines, 94 entries) lists 94 of the facade's 103
  names. Missing: `accepted_censuses_common`, `accepted_family_aggregate_bound`,
  `activeNextCounts_le_bound`, `allTwoOrderContinuation_card_le_432`,
  `encodeAllTwoOrderContinuation_injective`,
  `encodeFixedOrderContinuation_injective`, `failed_check_accepts_nothing`,
  `fixedOrderContinuation_card_le_216`, `three_mul_activeNextCount_le` — all
  present as theorems in `Reduction.lean` and `Evidence.lean`. The receipt names
  no axiom outside `propext`, `Classical.choice`, `Quot.sound`; its formatting
  looks like genuine Lean output, consistent with an intermediate build during
  the overnight session before nine theorems were added — unconfirmed. It is
  consumed by no script and is nonetheless pinned by digest in the v2 manifest,
  so regenerating it is a freeze event (FZ-A5).
- **Partial positive evidence.** **[measured 2026-09-12]** `Types.lean`
  elaborates against the main checkout's `Trick1Foundation.olean` with no
  diagnostics, exit 0, about 3 s (read-only `lean` invocation). The other seven
  submodules cannot be checked this way without intermediate oleans.
- **The rulings.** GT1-A23 makes "must build and pass the axiom audit" an
  *obligation* of the M3 gate; GT1-A24 (freeze 57) authorizes the gate and
  records **no M3 result**; the sentence "M3 PERFECT-RECALL NET PARITY COMPLETE
  under freeze 57" has never been issued. The M3 Rust/Metal crates were deleted
  as unbuildable WIP at the 2026-08-24 unification (ad355e9); the Lean tree was
  kept, FZ-A4 calling it "kernel-audited freeze-57 mathematics, protected work,
  not scaffolding". That phrase is a ruling's wording; no artifact on this
  machine supports "kernel-audited", and this page records the gap rather than
  the phrase. [proof-assistant-plan](proof-assistant-plan.md)'s M3 paragraph
  carried the same overclaim from 2026-08-17 until this rewrite.

**Status, in one sentence: statements committed 2026-08-17; build and axiom
audit unverified as of 2026-09-07 [unverified].** Settling it needs
`lake build Texas42.Trick1PerfectRecallNet` in a warm-cache checkout and a
regenerated receipt with all 103 names — a freeze-event action, not a wiki
edit. Until then the tree counts as exploratory for every purpose, including
the "sorry-free" and "standard axioms only" sentences elsewhere, which are
grep-true for it but build-unverified.

---

## 8. The Pro-authored constellation thread

Two modules came from the exchange's Lean thread on 2026-08-01, under the
iteration policy (refine in-conversation, no adversary panel until
finalizable) — [exchange](exchange.md) owns the dispatch table. **[kernel]** for
the theorems that build; **[exchange provenance]** for how they arrived.

| Dispatch | Sent → harvested | Outcome |
|---|---|---|
| x:011 `constellation-lean-formalization` | 2026-08-01 07:38Z → 14:27Z | honest refusal to fabricate a compilation claim; proposed the staged build; caught the dispatch spec error (tier-zero keys tie by design — the theorem is unique winner, not key injectivity); correction accepted |
| x:013 `constellation-lean-stage1` | 14:28Z → 15:10Z | **STAGE 1 GREEN**: 278-line core; `lake build` clean under the pin after two mechanical local fixes (`set_option … in` chaining → section scope); `unique_winner` kernel-checked, zero sorries, including the 56,448-case `decide` and two worked tricks; wired into `Texas42.lean` (commit 1d36ddb8) |
| x:015 `constellation-lean-stage2` | 15:15Z → 15:58Z | **STAGE 2 GREEN after local repair**: suffix positions, mid-trick legality, fuel-indexed exact minimax, `value_k1_forced`; the kernel evaluates two `k = 1` values, `−11` (fours trump) and `16` (no-trump). Local fixes: `prefix` reserved-keyword rename → `pending`; `step_remaining` proof restructured (`fin_cases` → pointwise `by_cases` + 4-way `omega`); a `LinearOrder Domino` lift instance; `SuffixPos` namespace (commit f0de9333) |

Division of labour: Pro writes, this side lake-builds under the pin and returns
the log.

What landed: `ConstellationCore.lean` (272 lines; its own `Constellation`
namespace with `Domino` as a subtype, `trickKey : Fin 42`, `winner_maximal`,
`positive_key_injective` by `decide` over every declaration, context and tile
pair, `unique_winner`) and `ConstellationSuffix.lean` (274 lines; `SuffixPos k`,
the `MidState` machine, `legalMoves_subset_hand`, `step_remaining`, `minimax`,
`value`, `value_k1_forced`, and `example : value pipTrumpK1 = -11 := by decide`,
`example : value noTrumpK1 = 16 := by decide`).

What did not: the C1 suffix-factorization theorem the thread was built toward
(suffix minimax factors through the declaration-free constellation key) was
never stated in Lean. It remains exchange-tier only (x:009, PARTIAL — the proof
chain survived 3/3, the corroboration artifacts were quarantined;
[claim-ledger](claim-ledger.md)'s C1 row "Lean mechanization pending
(dispatch 011)" is still accurate; [idea-retrograde-rank](idea-retrograde-rank.md)
holds the idea).

Two things a reader comparing files should know. The modules are
**self-contained** — they re-derive their own domino and declaration algebra
rather than importing `Basic`/`Trick`, and are not reconciled with them; they
carry no ledger row and are not part of the priority-0 scoreboard, though they
are in the default build target so the hygiene greps cover them. And they
differ from the spine one level below the headline: `Trick.lean` proves key
injectivity by the shared-pip argument, `ConstellationCore` discharges it by a
56,448-case `decide`. Both routes are legitimate under the trust boundary; the
choice of how much of a theorem should have a reason rather than a computation
is unresolved, not an oversight. Whether the thread is parked or dead is an
open question (§10).

---

## 9. Layout

27 files under `lean/Texas42/`, 8,504 lines, 365 theorem/lemma declarations
**[bookkeeping, counted 2026-09-12]**; 19 top-level modules imported by the
root `lean/Texas42.lean` (whose import list includes the M3 tree, so the
*default* target demands the unverified module — §12.2). Only `Basic.lean` and
`ConstellationCore.lean` `import Mathlib` wholesale. Per-declaration detail is
in [lean/README.md](../lean/README.md); ledger rows in
[lean-row-index](lean-row-index.md).

| Module | Lines | Thms | Imports | Proves | Tier of meaning |
|---|---|---|---|---|---|
| `Basic.lean` | 149 | 7 | Mathlib | dominoes as canonical pairs; 28; 35; incidence covering (K0–K1) | kernel |
| `Trick.lean` | 345 | 18 | Basic | nine declarations, eight led contexts, follow, rank, tier, key, key injectivity, **unique trick winner**, BEATS, threat monotonicity, lead-threat witness (K2–K3) | kernel |
| `Transport.lean` | 209 | 9 | Trick | count-preserving maps `{id, 2↔3}`; the scoped 2↔3 transport (K4) | kernel |
| `Auction.lean` | 230 | 3 | Basic | Straight auction: bids, decidable legality, deterministic step, mark ceiling `min(cap, 5)` | kernel |
| `Deal.lean` | 72 | 5 | Basic | ordered deal worlds, computable owner (cardinalities open) | kernel |
| `Play.lean` | 585 | 20 | Trick, Auction, Deal | contract; reduced play state; legal-set characterization; invariant-preserving step; 28 plays / 7 tricks / 42 points (K5) | kernel |
| `Cells.lean` | 1,098 | 37 | Play | public record, upper-bound-only voids, derived cells and fiber, **losslessness**, remainder bijection (K6, K8) | kernel |
| `Reachability.lean` | 59 | 3 | Cells | `Reachable`, `CertifiedState`, extensionality (K11 predicate) | kernel |
| `Information.lean` | 58 | 1 | Cells | mechanical ≠ information-state (§6.6 witness) | kernel |
| `Reduction.lean` | 180 | 6 | Cells | generic capacitated cell kernel: `red`, idempotence, coarsest exact quotient; game instantiation (K7) | kernel |
| `NormalForm.lean` | 1,455 | 56 | Reduction | trichotomy, strict Hall, generic capacitated Hall, `SupportNF`, compile/decode inverses, total form classifies fibers (K9) | kernel |
| `Belief.lean` | 291 | 11 | Cells | exact rational `FinPMF`, Bayes conditioning, likelihoods, posterior, pushforward with support inside the fiber (K15) | kernel |
| `Strategic.lean` | 294 | 7 | Belief | `BeliefProc`, strategic sufficiency, best response as a function of `B` (K15) | kernel |
| `Witness.lean` | 757 | 39 | Strategic | **the 90-world witness**, internalized whole (K15, PA-E10) | kernel |
| `ConstellationCore.lean` | 272 | 3 (+2 ex.) | Mathlib | self-contained core; `unique_winner` by 56,448-case `decide` (x:013) | kernel; no ledger row |
| `ConstellationSuffix.lean` | 274 | 4 (+2 ex.) | ConstellationCore | suffix positions, exact minimax, `−11` and `16` (x:015) | kernel; no ledger row |
| `Trick1Foundation.lean` | 480 | 25 | Play | trick-1 arithmetic: loss allowance ≤ 12, 420, 399,072,960, 212/217-bit windows, unbanked-point invariant, cell counts to 11,730, interval algebra (GT1-A8) | kernel as arithmetic; exploratory as meaning; gated |
| `Trick1MetalFoundation.lean` | 141 | 8 | Trick1Foundation | 79,800 slots, arena bytes, 46 tasks, ≤ 10 vectors, stable filter, fail-closed acceptance (GT1-A17, freeze 56) | kernel as arithmetic; exploratory as meaning; gated + receipt-diffed |
| `Trick1PerfectRecallNet.lean` + 8 submodules | 1,555 | 103 | Trick1Foundation (via `Types`) | the freeze-57 M3 abstract obligations (§7.4) | **unverified** — statements committed, build and audit pending |

Spine (14 modules): 5,782 lines, 222 theorems. Constellation: 546 lines, 7
theorems and 4 examples. Trick-1 modules: 621 lines, 33 theorems. M3 tree:
1,555 lines, 103 theorems.

Two receipt files sit beside the sources: `trick1_metal_foundation_axioms_v1.txt`
(8 entries; live, CI-diffed) and `trick1_perfect_recall_net_axioms_v1.txt` (94
entries; stale, unconsumed). Neither is mentioned in PROOFS.md; both are pinned
in the v2 manifest.

---

## 10. What remains

### 10.1 The ledger queue **[bookkeeping against v0.7 `65_MECHANIZATION_LEDGER.md`]**

Ledger: 83 `PA-` rows (17 A, 14 B, 15 C, 18 D, 12 E, 7 F); 42 at priority 0,
all proved; 27 at priority 1, of which 4 are fully proved (A15, A16, A17, B03)
and 3 partly.

**Open priority-1 rows, 20:**

| Row | Route | Target |
|---|---|---|
| PA-B11 | PROVE | fixed-hand play graph is finite and graded |
| PA-B12 | PROVE | objective physical Markov congruence |
| PA-B13 | PROVE | deterministic contract/mark settlement |
| PA-C11 | PROVE | exact capacity-DP count recurrence and soundness |
| PA-C13 | WITNESS | local possible holder is not marginal possible holder |
| PA-C14 | PROVE | marginal holder edge iff forced successor feasible |
| PA-D06 | PROVE | global deterministic support minimality/factorization |
| PA-D11 | PROVE | reachable support image and restricted minimality |
| PA-D13 | PROVE | seven observable lead contexts per declaration |
| PA-D14 | PROVE | lead-witness necessity |
| PA-D15 | PROVE | witness validator soundness/completeness |
| PA-D16 | WITNESS | feasible-but-unreachable support |
| PA-E04 | PROVE | physics-only posterior under uniform chance assumptions |
| PA-E05 | PROVE | finite exponential-tilt form |
| PA-E06 | PROVE | forced-action world-nondiscrimination |
| PA-E08 | PROVE | deterministic information-set best-response existence |
| PA-E09 | PROVE | coordinate-only scalar factorization criterion |
| PA-E11 | WITNESS | context-free domino value counterexample |
| PA-F01 | PROVE | local hand-order invariant/equivariant gauge |
| PA-F07 | PROVE/BOUNDARY | shared utility does not centralize partner information |

**The three half-rows (priority 1):** PA-B05 — `Deal` and its lemmas are
defined, the cardinalities `28!/(7!)⁴` and `21!/(7!)³` as cardinalities of the
type are open; PA-C08 — the generic capacitated Hall lemma
(`exists_partition_of_hall`) is proved, the game-level Hall/max-flow feasibility
equivalence is open; PA-C15 — the reduction's fiber preservation, idempotence
and coarsest-exact-quotient theorem are proved, and the row's dependence on the
open PA-C14 marks what the docstrings and [lean-row-index](lean-row-index.md)
call the "backbone" as the proved part.

**Reflection targets and census rows:** PA-A12 (737,100 winner cases,
priority 2), PA-B04 (auction history counts, priority 3), PA-D17 (81-bit
census, priority 4, "REFLECT or keep external"), PA-D18 (26–46-bit interval,
priority 4). Whether reflection is still the intended route or these stay
"visibly external", as the ledger's definition of done permits, is undecided.
Remaining priority-2 rows: B14, C12, D07, D08, D12, E12, F02, F03, F04, F06.

**Also open:** the Straight-42 instantiation of `BeliefProc`; the minimality of
the 90-world witness; reconciliation of the constellation modules with the
spine; the two citation gaps (PA-C05 uncited in source, PA-C03 cited only in a
section-header comment at `Cells.lean:450`).

### 10.2 Walt-era programs with zero kernel coverage **[exploratory]**

Every item below is accepted mathematics or an accepted obligation list in
walt's documents. **None has a file under `lean/`, none has a ledger row, and
the "Lean side-project ledger" they are said to enter exists in no file** —
its only trace is the sentence accepting each program.

| Program | Where the obligations are stated | Acceptance | Size |
|---|---|---|---|
| [[lean-catchup]] card | `kanban/backlog/lean-catchup.md` (opened 2026-08-24) | done when a triaged P1 list exists in `lean/` and the signed-pivotal boxed identities are assessed for mechanization cost; neither exists; also names the pmake objective and seat-census rulings as uncovered | — |
| CBS-O1..O15 | `walt/math/counted_belief_sandwich_v0.1.md` Part XIII §54, with a candidate module map in §55 (`Walt/RootSandwich.lean` … `Walt/EmpiricalMaxUpper.lean`, `Texas42/CurrentRemainderFactor.lean`, `Texas42/PmakeCellBounds.lean` — filenames quoted from the source) | CBS-A9, 2026-08-30: "accepted into the Lean side-project ledger (Jason + Pro track; deterministic finite theorems before the probability formalization)" | 15 obligations, 13 candidate files |
| PS-T1..T15 | `walt/math/anytime_proof_state_score_v0.1.md` Part XV (§75, plus a 42 instance layer in §76) | APS intake Part XV row, 2026-08-31: "ACCEPTED into the Lean side-project ledger (Jason + Pro track)" | 15 theorems, game-independent |
| MB-O1..O20 | `walt/math/model_belief_base_player_v0.1.md` Part XII §70 | MB intake Part XII row: "ADOPTED verbatim as the round's gate language"; MB-O20 (common-information prescription equivalence) "queued behind everything" | 20 obligations |
| SC-O1..O15 and the §60 Lean order | `walt/math/salvation_complex_v0.1.md` §59 (obligations) and §60 (`Walt/Salvation/Incidence.lean` … `Texas42/WaltSalvationInstance.lean` — filenames quoted) | SC-A1..A8 rulings; the generic finite set-system layer is named "an attractive next formalization tranche" | 15 obligations, 8 candidate files |
| C1 suffix factorization | [idea-retrograde-rank](idea-retrograde-rank.md), x:009 | exchange tier; "Lean mechanization pending (dispatch 011)" since 2026-08-01 | 1 theorem |

The mechanization-cost lessons of §4.4 are what make this queue expensive:
anything that evaluates game machinery needs `decide +kernel`, verified tables
and integer moments, and a silent long run is a hang until proven otherwise.

---

## 11. What Lean is for

**The independent verification path.** Lean is the route by which a finite
claim stops being "we ran a program and it agreed" and becomes a theorem. It is
independent because it may not read anyone else's answers — not the ingest
Python verifiers, not exchange programs, not [rob](rob.md), not walt's gates.
When [walt](walt.md) needs mechanical verification, its standing no-rescue
policy routes it here ([walt-math-open-questions](walt-math-open-questions.md)'s
promotion path), and §7 shows the price: a theorem about Lean-defined arithmetic
certifies the Rust beside it only through a correspondence proof nobody has
written.

**The place where definitions get pinned down.** Mechanizing a section forces
every implicit assumption in the prose into the open; `hd_allowed` and the
x:011 correction (§4.5) are the two findings of that kind so far. Rule 5 exists
so that such findings are reported, not hidden.

---

## 12. Building and extending

### 12.1 Toolchain pins **[engineering]**

| Pin | Value | Where |
|---|---|---|
| Lean | `leanprover/lean4:v4.33.0-rc1` (Lean 4.33.0-rc1, commit 62eed1db; Lake 5.0.0 — `~/.elan/bin/lean --version` from `lean/`, 2026-09-12) | `lean/lean-toolchain` |
| mathlib | `rev = "v4.33.0-rc1"`, resolved to `79d0395a1825a6264ad5d269e35e60537518955e` | `lean/lakefile.toml`, `lean/lake-manifest.json` |
| Lake options | `relaxedAutoImplicit = false`, `weak.linter.mathlibStandardSet = true`, `maxSynthPendingDepth = 3`, default target `Texas42` | `lean/lakefile.toml` |
| Packages pulled by mathlib | aesop, batteries, Cli, importGraph, LeanSearchClient, plausible, proofwidgets, Qq | `lean/.lake/packages/` |

`elan` lives in `~/.elan/bin` and must be on `PATH` (or invoked by absolute
path; running `lean` outside `lean/` fails with "no default toolchain
configured" because the pin is read from `lean-toolchain`).

### 12.2 Building

```sh
cd lean
lake exe cache get   # prebuilt mathlib artifacts, multi-GB, once
lake build           # default target = every module in lean/Texas42.lean, INCLUDING the M3 tree
```

- **The cache step is not optional in practice** — without it `lake build`
  compiles mathlib from source.
- **The default target has not demonstrably built since 2026-08-17**, when the
  M3 tree entered the root import list; the main checkout's root
  `Texas42.olean` is dated 2026-08-03. To build what is known to build, name
  the modules: `lake build Texas42.Witness Texas42.NormalForm Texas42.Transport
  Texas42.Reachability Texas42.Information Texas42.ConstellationSuffix
  Texas42.Trick1MetalFoundation` covers 18 of the 19 top-level modules through
  the import graph of §9 (everything but `Trick1PerfectRecallNet`). The gated
  pair alone is `lake build Texas42.Trick1Foundation
  Texas42.Trick1MetalFoundation`.
- **In a fresh worktree, clone the artifacts instead of refetching.** On APFS,
  `cp -Rc <main>/lean/.lake <worktree>/lean/.lake` is instant and
  copy-on-write; a never-built worktree has no `.lake` at all (this one has
  none). The main checkout's `.lake` is 7.5 GB.
- **Never run two `lake build`s in the same directory concurrently.**
- Budget: once the cache is warm, an incremental full build of the spine is
  20–60 s (47 s measured at d190b264); `Witness.lean` alone 33 s. Budget for
  that loop rather than for interactive elaboration.
- A read-only check of a single file against existing oleans, without `lake`,
  is `LEAN_PATH=<main>/lean/.lake/build/lib/lean:<each package>/.lake/build/lib/lean
  ~/.elan/bin/lean Texas42/<File>.lean` — it writes nothing and is how the two
  2026-09-12 measurements on this page were taken.

### 12.3 Adding a proof (the PROOFS.md per-slice checklist, with mechanics)

1. **Read the ingest sections being formalized** — both packages if they
   overlap. [discrepancies](discrepancies.md) resolves conflicts; do not resolve
   them in the Lean file.
2. **Create `lean/Texas42/<Name>.lean`** with the house preamble: the copyright
   block, an import of the nearest layer (not `Mathlib` wholesale unless truly
   needed), a module docstring naming the Math sections and ledger rows the file
   covers, `namespace Texas42`.
3. **Register it** by adding `import Texas42.<Name>` to `lean/Texas42.lean` in
   dependency order; no lakefile edit is needed.
4. **Land the definitions first, with citations** (rule 4), keeping them
   minimal: derived views are functions, not fields. For a match-defined
   predicate you will `decide` over, write the `Decidable` instances
   per-constructor.
5. **Prove the row's theorems**, each docstring naming its `PA-` row (or, for
   walt-facing work, its ruling ID). Kernel `decide` for finite arithmetic;
   `decide +kernel` for anything that evaluates game machinery; top-level
   lemmas assembled with `exact`, never a heavy `decide` inside `refine`.
6. **Build clean and check axioms.** `lake build`, then `#print axioms` on
   every new theorem in a scratch file. Expected: a subset of
   `[propext, Classical.choice, Quot.sound]`.
7. **Update the doc surfaces** — the layout list in
   [lean/README.md](../lean/README.md), the status in
   [proof-assistant-plan](proof-assistant-plan.md), and the row map in
   [lean-row-index](lean-row-index.md) — and commit with the receipt.

A proof *counts* only when all of that holds: on main, sorry-free,
`native_decide`-free, closing over at most the three standard axioms with the
receipt recorded, definitions cited, and the statement not quietly weakened.

### 12.4 Receipt formats

**Commit-message form** (every priority-0 commit; d190b264 verbatim):

```
Receipts (#print axioms, all [propext, Classical.choice, Quot.sound]):
anchor_values, card_worldPairs, isWorld_iff, rule_fiber,
auction_histories, same_full_support, expected_differentials,
make_probabilities, posterior_action_reversal, ninety_world_witness.
No sorry, no native_decide.
```

**File form** (the M2 module): `#print axioms <name>` lines at the end of the
module, `lake env lean Texas42/<File>.lean > <file>_axioms_v1.txt` committed
beside the sources, byte-diffed by the gate, regenerated only through Lean,
never hand-edited. Output lines look like
`'Texas42.Trick1MetalFoundation.projectorArenaBytes_eq' depends on axioms: [propext]`.
The two trick-1 commits (3b4c6d60, 813d5e81) carry no receipt in their bodies;
the file is their receipt.

### 12.5 Idiom pointers (PROOFS.md carries the full list)

`Fin` arithmetic → destructure, `simp only [… Fin.ext_iff …]`, `omega`; ground
every non-literal atom for `omega` first; `ofLex_toLex` in the simp set for
lex keys; `WithTop ℕ` ranks via `coe_ne_top`/`coe_inj`; per-constructor
`Decidable` instances; `simp only`/`unfold` rather than `rw` for plain defs;
probe mathlib names with `lake env lean probe.lean` and `exact?` before
guessing; `show … from congrArg _ h` for projections of computed structures;
never `rw` into proof-dependent terms; `Finset.choose` under a `dif` guard for
order-free extraction; function-level `have`s near binders; capacitated
matching by slot expansion into mathlib's Hall; index concrete families by
`Fin n` into a computable `List`, never a `Finset`-membership subtype; no `ℚ`
inside `decide`; opaque indicators (`makeInd`) inside expectations; one-pass
state fingerprints (`encodeState`); verified tables; `decide +kernel`; no heavy
`decide` inside `refine`; `synthInstance.maxSize` for wide products; bound every
declaration with default heartbeats in a diagnostic pass and use `sample <pid>`
to tell elaborator frames from kernel evaluation.

---

## 13. Timeline **[engineering, from `git log -- lean/` and the exchange ledger]**

| Date | Event | Commit |
|---|---|---|
| 2026-07-27 | `lean/` created: Lake skeleton, mathlib pin v4.33.0-rc1, `Basic.lean` (65 lines), inert template workflows | 8ea2ff24 |
| 2026-07-28 | K2–K3: declaration algebra and the unique trick winner (`Trick.lean`) | 01d33951 |
| 2026-07-29 | Layer A complete (BEATS/threat, witnesses, 2↔3 transport); Layer B core (auction, deal, contract, play); Layer C stage 1; **the losslessness theorem**; reachability + certified states, mech ≠ info; generic reduction kernel + finite belief layer | 95ffcb24, 10ae1a4b, c9b195b5, a904c163, d82e385c, 8512a56d |
| 2026-07-31 | **the support normal form** (PA-D01–D05); strategic sufficiency (PA-E07) | 54fb2acc, 38031a9e |
| 2026-08-01 | x:011 honest refusal (caught the spec error); x:013 Stage 1 GREEN; x:015 Stage 2 GREEN after local repair; constellation modules land sorry-free | 1d36ddb8, f0de9333 |
| 2026-08-02 | **the 90-world witness** (PA-E10) — priority 0 complete, 42/42; `decide +kernel` idioms recorded | d190b264 |
| 2026-08-03 | merge into main; oleans for all 16 modules built 09:07–09:08 | 0d1bf7ea |
| 2026-08-13 | `wiki/lean.md` and `wiki/lean-row-index.md` first written | 55855c87 |
| 2026-08-16 | `Trick1Foundation.lean` with freeze 55 (GT1-A8); `lake build Texas42.Trick1Foundation` joins the M0/M1 conjunction | 3b4c6d60 |
| 2026-08-17 | `Trick1MetalFoundation.lean` + committed axiom receipt + the Lean step in `walt/ci/check.sh` and `check_m2_metal.sh` (freeze 56); later the same day the M3 `Trick1PerfectRecallNet` tree committed as WIP "does not build" with its 94-entry receipt and the plan page's M3 paragraph; freeze 57 (GT1-A24) authorizes the M3 gate, records no result | 813d5e81, 97ce321a |
| 2026-08-24 | freeze-56 v2 re-issue absorbs the `Texas42.lean` drift and pins all 32 `lean/` entries by digest (FZ-A4); closure check demoted to freeze events (FZ-A5); M3 crates deleted, Lean tree kept; [[lean-catchup]] opened | c92175ae, ad355e9 |
| 2026-08-30 | CBS-A9: CBS-O1..O15 accepted into the Lean side-project ledger (no file) | — |
| 2026-08-31 | APS intake: PS-T1..T15 accepted into the Lean side-project ledger (no file) | — |
| 2026-09-03 | `Trick1Foundation`/`Trick1MetalFoundation` oleans rebuilt in main's `.lake` at 23:09 (a CI run); no M3 outputs | — |
| 2026-09-07 | survey for this book: 27 files, 8,504 lines, 365 theorems; no sorry/native_decide/axiom; 32/32 manifest digests match | c00717d1 |
| 2026-09-12 | this page: counts re-taken; M2 receipt reproduced byte-identically read-only (27 s); M3 `Types.lean` elaborates (3 s); no `lean/` commit since 2026-08-17 | — |
