# The situation census — first (d, Θ) candidate under §12.6A

**Status:** experiment design, exploratory tier. Basis: v0.4 (frozen) +
`walt/math/equivariant_lumpability_v0.5.md` (§12.6A). Written 2026-08-10;
forks F1–F7 adjudicated by walt-math — **`walt/CENSUS-RULINGS.md` is
binding**, and the amendments below are folded in from it.
**Scope: pip-trump only** (all 13 receipt trick-six kernels are pip-trump;
DT/NT inclusion would implicitly claim cross-declaration-type transfer,
v0.4 §17.5 / §17.4 open problem 7 — F1 amendment).

**The bar (Jason):** show or disprove that the number of count-free
canonical situations is reasonably small — order 10^5 for the full game.
This census is the first measurement: the 13 trick-six kernels, where
everything is exact and (ECL) is exhaustively checkable. Both outcomes
are results; failures are counterexample pairs carried back to the math
(NO-RESCUE).

## Carrier (F1)

X = concrete situations x = K ⊕ ω pooled **across all 13 trick-six
receipt kernels** (the ambient frame spans kernels — cross-kernel merges
are the transfer signal). Focal seat = each kernel's decision seat; the
three non-focal seats play the declared **fixed uniform-legal field**
(v0.4 §7.4's field choice, as in every probe). Belief per kernel =
uniform over its fiber; the theorem pushes it forward, so quotienting
the world-level latent space is the correct move (§12.6A theorem,
conclusion 1). This is NOT worldwise-PI classing (§12.4): the
equivalence tested is dynamics equivalence (ECL), not response
equality.

Carrier contents: the kernel-root states (focal to lead, trick 6) plus
every reachable mid-trick and trick-7 state under the primitive step
model (F5). A state is: live tiles per seat, table state (unresolved
trick: tiles played in order — led context and current winner are
derived views), and actor. **The count-free banked increment is
emission, not state** (F5 amendment): it is emitted at trick-completing
plays and accumulated by the theorem's Σkᵢ; storing it in the state or
descriptor would duplicate a derived quantity and split trick-7 classes
by trick-6 outcome, destroying real merges.

## Candidate descriptor d (F2): the finest structural relabeling quotient

Two situations are equivalent iff there is a **structure-preserving
relabeling** of live tiles and live contexts matching them exactly:

- seats matched by the **forced rotation** aligning current actors:
  seat(actor_x + i) ↔ seat(actor_y + i), with focal ↔ focal as a match
  precondition (partnership is preserved automatically; **reflected
  matchings are forbidden** — reflection is not an automorphism of the
  oriented game and no orientation variable is adjoined, v0.4 §11.7)
  (F2 A4);
- the trump context matched to the trump context; **live** non-trump
  contexts matched to live non-trump contexts, where a context is live
  iff some live tile leads it (q ∈ ℓ[live]); membership in a context
  that can never be led again is dynamically inert and erased (F2 A3);
- tiles matched over **live tiles ∪ unresolved-trick tiles** (dead
  tiles in resolved tricks excluded — they never enter a future trick
  key), preserving: holder (relative seat) for live tiles / table
  position for played tiles, trick-key comparisons in every live
  context **plus the current led context** (future plays are compared
  against the table tiles, in particular the current winner) (F2 A1),
  follow membership in every live context (the covering structure —
  including two-context bridging), **the led-context map**:
  Θ^C(ℓ(d)) = ℓ(Θ^D(d)) for every live tile (follow membership alone
  does not determine which context a tile leads, and led context
  controls follow legality and the BEATS index) (F2 A2), and the
  double flag (not forced by pip-trump dynamics; retained per the
  finest-candidate design and named a first coarsening candidate for
  later declared rounds);
- the unresolved-trick play order matched (led context and current
  winner follow as derived views under the matchings above).

Formally: classes are isomorphism classes of the live-structure object;
d(x) is a canonical form computed by a deterministic canonicalization
(tie-broken labeling search — carrier is tiny, brute force is fine).
This is deliberately the **finest** lawful equivariant candidate: if it
fails (ECL), nothing coarser passes, and the counterexample tells us
what structure the game refuses to forget. If it passes, its class
count is the ceiling census, and coarser candidates can follow.

Count is excluded from the structure by design (count-free core). Pip
values appear ONLY through the relations above (rank comparisons,
follow membership), never as labels.

## Transports Θ (F3)

For d(x) = d(y): Θ_xy = c_y^{-1} ∘ c_x where c_x, c_y are the
canonicalization maps. Coherence is automatic for Θ^D, Θ^C, Θ^Q and the
derived Θ^A, Θ^obs alike (all factor through the canonical
representative — v0.5 Appendix A notes 1–2 discharged, F3). **Θ^D's
declared domain is all live ∪ unresolved-trick tiles** (the observation
language names tiles beyond any role interface, v0.5 App. A note 3).
Θ^A = Θ^D restricted to the actor's legal tiles — that it IS a
bijection A(x) ≃ A(y) is exactly ECL condition 1, checked, never
assumed. Θ^obs on observation tokens (relative-seat, tile,
lead/follow/slough class) = componentwise with Θ^D. No role interface
in this census (O_Σ = ∅ is lawful: ρ_x is the empty function, trivially
functionally instantiated; the rigid square holds vacuously; the only
preserved outcome channel is the e⋆ accumulator — extra ruling).

## Probability model (F4)

The operational (ECL) reading for a turn-taking game, fixed by F4 — the
builder implements this verbatim:

- **The actor is a function of the state.** One kernel per primitive
  step; (ECL) never mixes focal choice with hidden chance in one step.
- **Focal-to-act state x:** A(x) = the focal seat's legal tiles; for
  each a, K_a(x;·) is the Dirac point mass at the determined (k, o, x').
  ECL condition 1 checks A(y) = Θ^A(A(x)); condition 2 is the
  commutation check per action.
- **Non-focal-to-act state x:** A(x) is a singleton no-op; K(x;·) puts
  mass exactly 1/|L| on each legal move of the hidden actor, emitting
  that move's (k, o, x'). ECL compares the full joint law — which
  silently enforces |L_x| = |L_y| with tokenwise Θ^obs correspondence.
- All probabilities exact i128 rationals attached per primitive hidden
  play; never a probability on a focal action; exact equality, no
  tolerance.
- Count-free increment k ∈ {0, e⋆} at the trick-completing fourth play
  (including trick 7's final play, where the recursion closes); e⋆ is
  the trick coordinate of the transported focal partnership, guaranteed
  by A4's focal↔focal matching. A token is emitted for every play,
  focal included, uniformly (F5).

## The check (F5): primitive-step ECL, exhaustively

Per §12.6A: for every class with ≥2 members, take the canonical
representative r and every other member y with transport Θ_ry:

1. legality: A(y) = Θ^A_ry(A(r));
2. for every focal action a at focal-to-act states, and at
   non-focal-to-act states directly: the joint law of (k, transported
   observation, successor **class**) must match exactly (rational
   equality, no tolerance).

Successor classes are computed by canonicalizing successors —
step-then-canonicalize vs canonicalize-then-step commutation is exactly
what the check verifies. The recursion closes at hand end (trick 7
resolves). Every state reachable from any kernel root is in the
carrier, so the check is a finite exact verification of equivariant
lumpability on this domain.

## Outputs

`results/census_2026-08-10.txt` (regenerable by the runner):

- scope line: pip-trump only (cross-kernel merges are within-pip-trump
  by corpus construction — the corpus supplies no DT/NT);
- carrier size (states, by kernel and pooled; roots / mid-trick split);
- class count (the headline), same splits — **root-only and
  full-carrier counts reported separately** (the 10^5 bar is about
  situations; root counts are the comparable figure), plus
  singleton/vacuous-class counts so the ECL check's actual coverage is
  visible (F6); every count paired with its ECL verdict line — a count
  without PASS is a relabeling census, not a lawful compression
  measurement;
- **cross-kernel merges**: classes containing situations from ≥2
  distinct receipt hands (the beyond-the-particular-game signal);
- ECL verdict: PASS (every class checked, counts of checks) or FAIL
  with every counterexample pair (canonical form, divergent statistic,
  both concrete witnesses, exact values) — the math input, not a bug
  list;
- baseline comparison: class count under the identity-interface §12.6
  reading (no relabeling) on the same carrier — the equivariance
  dividend.

## Failure protocol (F7)

ECL failures are recorded, never patched in-run. No descriptor
refinement this session — the finest candidate either passes or its
counterexamples are the deliverable (drafted for a possible 5.6 Pro
dispatch that ONLY Jason can authorize). §12.9's counterexample-guided
refinement is the sanctioned *method* for later rounds, run as new
declared candidates, never as in-run fixes.

## Forks for walt-math (pre-build adjudication)

- **F1** carrier as stated (world-level latent, pooled frame, primitive
  steps) — sound instantiation of §12.6A? Any objection to pooling
  kernels with different declarations into one frame?
- **F2** the finest-structural-quotient candidate — is the listed
  invariant set exactly right for "structure-preserving"? Anything
  missing (e.g. does the trick-key rank comparison need to include
  dead-tile-relative ranks, or is live-only correct)? Anything that
  over-identifies (e.g. must non-trump context matching preserve
  something about the *called* structure under DT/NT kernels)?
- **F3** transports via canonicalization — confirm coherence and the
  empty-interface reading (rigid square vacuous).
- **F4** the uniform-legal probability model and where probabilities
  attach (per hidden play) — confirm against §7.4.
- **F5** primitive steps including mid-trick states in the carrier —
  confirm this matches §12.6A's step model, vs trick-level macro steps.
- **F6** nontriviality and reporting: is the identity-interface
  baseline the right control? What class-count statistic is quotable?
- **F7** failure protocol conformance with NO-RESCUE and §12.9.
