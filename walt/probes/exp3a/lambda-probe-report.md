# One-parameter exact solve — the integration doc's computational experiment (v0.1 §14, v0.2 §16, v0.31 §16.14–16.15)

**Tier: exploratory probe** (2026-08-08, extended 2026-08-09). Sits below
every evidentiary tier; numbers here become quotable only via amendment to a
verifier receipt. Implementation: `lambda_probe.py` + `lambda_probe_v2.py` +
`lambda_probe_v3.py` (stdlib Python, exact `Fraction`/integer arithmetic, no
floats). Source data: `rob/receipts/verify_player.txt` hand 0 — the probes
first **replay the full hand and validate every winner, point total, and
follow-legality against the receipt line** before any experiment runs. The
first half of this page is the v0.1 run; the v0.2 additions follow; the
**v0.31 additions (derived-table verification, Experiment 3A descriptor
synthesis, Experiment 3B horizon-3 breakpoint hunt, and a disclosed PWL
bugfix) are at the bottom**.

## The experiment (as specified by the document)

§14: give one tile a value λ (everything else at the symmetric baseline w = 0,
trick value 1), and compute exact policy lines, envelopes, rational
breakpoints, optimal-action intervals, ties, and the induced split of
baseline scheme cells.

## Choices (the document delegates these; all documented)

- **Domain**: last two tricks of hand 0 (threes trump). At trick-6 start every
  seat holds 2 tiles; **S1 leads**; S1's hidden fiber is exactly **90 worlds**
  (six unseen tiles into 2·2·2; the trump voids observed in tricks 1–5 are
  vacuous — no unseen tile contains a 3). No trump survives to the endgame, so
  it is a pure natural-suit domain.
- **Valued tile d = 4-1** (a five-count tile, unseen by S1, held by S2 in the
  true world, falls trick 7 in the receipt line).
- **Objective**: continuation differential for S1's team T1 = {S1, S3}:
  Ψ(λ) = (tricks T1 − tricks T0) + λ·(±1 for who captures the 4-1).
- **Part A** (§14.1–14.2 verbatim): fixed-field information-set solve —
  uniform belief over the 90-world fiber, the three hidden seats playing
  uniformly at random among legal tiles. S1's trick-7 play is forced, so each
  root action yields one exact line.
- **Part B** (§14.3 q4–q6): per-world parametric **minimax** (perfect
  information, both teams optimal) — exact piecewise-linear Q per lead per
  world, signature census across all 90.

## Results

### Part A — the fixed-field lines

| root action | E[Ψ](λ) |
|---|---|
| **lead 0-0** | **2/3 + (1/5)·λ** |
| lead 2-1 | −2/3 − (1/3)·λ |

The lines cross at **λ = −5/2**, outside the domain λ ≥ 0. Upper envelope for
λ ≥ 0 is the 0-0 line everywhere.

### Part B — 90-world parametric minimax census

Every world's Q(lead) is a **single exact line** (slopes ±1) — in this
two-trick domain, no world has an interior breakpoint: one valued tile never
changes any world's optimal *continuation* mid-λ. All the λ-structure lives
**across** worlds:

| class | worlds | Q(lead 0-0) | Q(lead 2-1) | 4-1 holder (S0/S2/S3) |
|---|---|---|---|---|
| 0 | 26 | 0 − λ | −2 − λ | 10/10/6 |
| 1 | 22 | 0 + λ | −2 − λ | 2/2/18 |
| 2 | 16 | 2 + λ | 2 + λ | 8/8/0 |
| 3 | 12 | 2 + λ | 0 − λ | 3/3/6 |
| 4 | 8 | 0 − λ | **0 + λ** | 4/4/0 |
| 5 | 2 | 0 − λ | 0 − λ | 1/1/0 |
| 6 | 2 | 0 + λ | 0 + λ | 1/1/0 |
| 7 | 2 | 2 + λ | 0 + λ | 1/1/0 |

- Response classes at λ = 0: **4**. Parametric classes over λ ≥ 0: **8**.
- Argmax changes in exactly **8/90 worlds** (class 4): at λ = 0 the two leads
  tie at 0; for **any λ > 0**, leading the 2-1 becomes strictly optimal — a
  boundary-tie resolution at λ = 0⁺, not an interior crossing.

### Role-fact audit (which facts predict the curve)

| fact set | cells | worst cell | verdict |
|---|---|---|---|
| holder(4-1) | 3 | 8 signatures | overshoots |
| + holder(2-2) | 9 | 4 | overshoots |
| + holder(2-0) or holder(4-2) | 24 | 3 | overshoots |
| **smallest purpose-sound holder set** | — | 1 | **5 of the 6 unseen tiles** |

### True world (receipt holders)

Q(0-0) = Q(2-1) = 2 + λ — T1 wins both tricks and captures the 4-1 under
optimal play from either lead; the receipt's actual line (lead 0-0, T1 takes
both, 4-1 captured) sits on the optimal branch.

## The document's six questions (§14.3), answered exactly

1. **Is the no-count optimal action stable for all λ ≥ 0?** Yes at the
   information state (Part A: crossing at −5/2, outside domain). Per-world:
   stable in 82/90 worlds.
2. **First valuation at which another action becomes preferable?** None at
   the info state. In 8 worlds: **λ = 0⁺** — the valued tile breaks a
   baseline tie immediately rather than at an interior price.
3. **Which policies trade trick probability for capture probability?** None
   here: lead 0-0 dominates on both axes at the info state (higher trick
   expectation *and* positive capture slope). The domain is too small to
   force the trade; expect it at horizon 3+.
4. **Does a proposed Scheme group states with different breakpoint
   structure?** Yes — the natural cell "partner holds the 4-1" contains **3**
   distinct parametric signatures; "opponents hold it" contains all **8**.
   Both overshoot; neither is purpose-sound.
5. **Which role facts are necessary to predict the parametric Q-curve?**
   Essentially all of them: the smallest holder-fact set that pins the
   signature has **5 of the 6 unseen tiles**. Sharpest finding of the probe:
   **capture destiny ≠ holder** — in class 0, S3 (partner) holds the 4-1 in
   6 worlds yet the opponents capture it under optimal play (slope −1); in
   class 1 the opponents hold it in 4 worlds yet T1 takes it (+1). Who
   captures a tile is a *control* fact, not a *holding* fact.
6. **How many new constellation classes does one valued tile create?**
   4 → 8: **it exactly doubles the census** in this domain. Consistent with
   the monotone-refinement theorem (§9.2): pure refinement, no merges.

## Caveats

- The uniform-random field and support-uniform belief are named modeling
  choices (support ≠ belief); the fixed-field lines are relative to them.
- Two tricks is the smallest honest instance: per-world Q-curves are single
  lines, so upper hulls are degenerate. The growth path (§14.4 — 2 tiles,
  the five count tiles, role classes, horizon 3) is where interior
  breakpoints and genuine trick-vs-capture trades should first appear.
- Nothing here is promoted; the probe validates the receipt before running
  and touches nothing outside the scratchpad.

---

# v0.2 additions (doc §16, run 2026-08-08 on the same domain)

The v0.2 integration doc keeps the domain and valued tile, so the results
above stand as the baseline. `lambda_probe_v2.py` re-derives the Part A
lines from a full **terminal-law** representation (the §10 object: exact
distribution over (trick diff, set of tiles captured by T1) per root action)
and cross-checks them — **exact match** (2/3 + λ/5 and −2/3 − λ/3). Lead 0-0
induces 11 distinct terminal outcomes; lead 2-1 induces 26.

## Witness census — extensional vs witnessed counts (§16.4 q7)

| Scheme | extensional worlds | witnessed realizations | multiplicity by world |
|---|---|---|---|
| `F_master`: ∃ tile e, Live(e) ∧ AbsoluteMaster(e) | 90/90 (rule-certain) | 360 | exactly 4 everywhere: {0-0, 2-2, 4-4, 5-2} |
| `F_beat21`: ∃ opponent chair c, tile e: Holds(c,e) ∧ Beats(e, 2-1 led) | 90/90 (rule-certain) | 180 | 1 witness: 18 worlds; 2: 54; 3: 18 |

Both example events are **rule-certain yet witness-ambiguous** — the cleanest
possible illustration that witnesses ≠ probability. (2-1's beaters are
{2-2, 4-2, 5-2}, all unseen; S3 can hide at most two, so some opponent always
holds one.) Fix-branch overlap, measured: |E(opp holds 2-2)| = 60,
|E(opp holds 5-2)| = 60, intersection 36, union 84 ≠ naive branch sum 120 —
a Fix is a union, not a mixture.

## Naive witness-weighting (§16.4 q8)

Reweighting the 90-world belief by witness count of `F_beat21` (the "no
canonical lift" mistake made deliberately) moves both action lines —
lead 0-0 goes from 2/3 + λ/5 to 8/15 + λ/10, lead 2-1 from −2/3 − λ/3 to
−5/6 − (47/120)λ — **but does not flip the optimal action anywhere on λ ≥ 0**
(crossing moves from −5/2 to −164/59, still outside the domain). So here the
bias is real but not decision-relevant; the doc's warning is measured, not
merely hypothetical.

## Rigid transport vs fresh re-query (§16.4 q9)

Through the actual receipt trick 6 (S1 0-0, S2 5-2, S3 4-4, S0 2-0):

- Masters before: {0-0, 2-2, 4-4, 5-2}. Rigid transport: **three of the four
  witnesses die in the observation** (0-0, 4-4, 5-2); only 2-2 survives as a
  master.
- Fresh re-query after: masters = {2-2, **4-2**} — 4-2 is a **new witness**
  (it was dominated by 4-4, which just died). The two operators disagree on
  4 of 5 tiles involved. This is the doc's witness-switching phenomenon,
  realized on the receipt line.
- Exact support cut: the trick-6 observation collapses 90 → **6** worlds
  (three tiles revealed; slough legality already forces the void structure).
  Hindsight anchor "the 4-1-holder role = S2" (learned at trick 7) filters
  6 → **2** — exact backward preimage, no forward leakage.

## Banked-accumulator identity (§16.3) and gauge (§8.3)

Banked after 5 tricks: T1 has 3 tricks + 5 count points; T0 has 2 tricks +
25 count points. Verified exactly, for straight count and for λ·e₍₄₋₁₎:
**Q_full = ev_w(bank) + Q_future, and the action order is identical** in both
modes. On all 37 enumerated full-hand terminal outcomes: every trick carries
exactly 4 captured tiles, the gauge relation (b, w) ~ (b−4c, w+c·𝟙) holds
for c ∈ {1, 3, −2}, and uniform tile value collapses to P = (1+4u)·t.

## Policy census (§16.4 q10)

Honest degeneracy: with trick 7 forced, S1 has exactly one contingent policy
per root action — full set 2, cone-exposed on λ ≥ 0: 1. Feature points:
lead 0-0 → (E[t_T1], P(T1 captures 4-1)) = (13/3, 3/5); lead 2-1 →
(11/3, 1/3). Lead 0-0 dominates on both axes; hull structure needs horizon 3+.

## Second experiment — role-valued defect (§16.5)

Role: **absolute master at trick-6 start** (witness set {0-0, 2-2, 4-4, 5-2}
in every world — kernel-certain, so the lens comparison is pure). Give "the
master" an extra λ; the meaning of "the" is the lens:

| lens | Q(lead 0-0) | Q(lead 2-1) | crossing |
|---|---|---|---|
| ALL witnesses carry λ | 2/3 + (8/5)λ | −2/3 − (4/3)λ | −5/11 |
| CANONICAL (least tile = 0-0) | 2/3 + 1·λ | −2/3 − (1/3)λ | −1 |
| PROBABILISTIC (uniform χ) | 2/3 + (2/5)λ | −2/3 − (1/3)λ | −20/11 |
| ADVERSARIAL (worst witness) | 2/3 + (1/5)λ | −2/3 − (1/3)λ | −5/2 |
| AT-LEAST-ONE captured | 2/3 + (1/3)λ | −2/3 − (1/3)λ | −2 |

Findings: the lens changes the capture slope of the 0-0 lead by a factor of
**8** (1/5 → 8/5) — "role value" is mostly selection semantics here, not
geometry — yet **no lens changes the optimal action** on λ ≥ 0 (every
crossing is negative). The per-witness slopes explain the spread: leading
0-0 captures the 0-0-master with certainty (slope 1 — a master led is a
sure trick, no live trumps), while every master under the 2-1 lead has the
same slope −1/3. The doc's prediction that role value decomposes into
geometry + selection is confirmed and quantified on this domain.

## v0.2 caveats

- Same tiny-domain caveat as v0.1: lines not hulls; the growth path
  (two tiles → five count tiles → role classes → horizon 3) is unchanged.
- The witness-weighting and lens results are relative to the same named
  belief/field choices as Part A.
- The rigid/fresh demonstration is kernel-level (mastery is holder-free
  here); a holder-dependent role would add world-relative switching on top.

---

# v0.31 additions (doc v0.3 §16.14–16.15, run 2026-08-09)

Implementation: `lambda_probe_v3.py` (+ `v3_diag.py` independent validators).
The v0.3 doc consolidates v0.1/v0.2, derives several new tables from the
reported data without re-running the scripts, and specifies two new
experiments: **3A** — mine the existing 90-world corpus for a compact
relational descriptor of the 8-class response target; **3B** — move to
horizon 3 and hunt for the first genuine interior breakpoint λ\* > 0.
Both ran. Everything below is exact rationals/integers.

## Disclosed bugfix in the shared PWL machinery

Building Experiment 3B exposed a bug in `lambda_probe.py`'s piecewise-linear
`_combine` (present since v0.1): in the no-crossing case the interval's true
endpoint was dropped, so the winning segment was probed at a point possibly
*outside* the interval. The bug is provably harmless when combining single
lines — which is all v0.1/v0.2 ever did — and **after the fix, the complete
v0.1 and v0.2 outputs were re-run and are bit-identical**. It bit only on
multi-segment PWLs (the horizon-3 fixed-field envelopes), where it produced
one spurious root switch in the first v3 run (at λ = 1/5), now gone. All
post-fix results below were validated against **independent scalar
re-implementations** (12 fixed-field points across all segments, 216
perfect-information spot solves): every check exact-matches. The doc's Phase
G1 insistence on independent re-runs is vindicated by its own experiment.

## Part 0 — every v0.31-derived table verified

The v0.3 doc derived tables from our reported data without executing the
scripts. All of them check out exactly:

- **Class census** (§16.5): sizes (26, 22, 16, 12, 8, 2, 2, 2) with the
  stated Q-lines and holder splits (10/10/6, 2/2/18, 8/8/0, 3/3/6, 4/4/0,
  1/1/0 ×3) — exact match.
- **Action-correspondence census**: 62 worlds strict 0-0 / 20 always tied /
  8 tie-at-0-then-2-1; 2 classes at λ=0 → 3 parametric — exact match.
- **Holder ladder** (§16.7): (3 cells, worst 8) → (9, 4) → (24, 3) → (24, 3);
  smallest sound holder set = 5 of 6 — exact match.
- **Entropy**: 90·H ∈ [226, 227] bits by exact integer bracket (no floats),
  i.e. H ≈ 2.52 bits as stated.
- **Feature-to-line conversions** (§16.4) and the **per-master
  capture-differential vectors** ḡ(0-0) = (1, 1/5, 1/5, 1/5),
  ḡ(2-1) = (−1/3)·𝟙 (§16.12) — exact match.
- **Control ≠ location counts** (§12.12): 6 worlds where the partner holds
  4-1 yet opponents capture it; 4 worlds the other way — exact match.

## Experiment 3A — the first run of the R\* = R̄∘D synthesis loop

Registered vocabulary: 22 mechanical / local-continuation atoms (holders,
team-of-holder, the 4-1 holder's companion tile, suit-2 strength per seat,
beater counts, forced-follow markers, one keep-best trick-7 motif).
Target R8 = the 8-class parametric root-Q signature (with single-line Q's the
first-order jet target coincides with R8); target R3 = the 3-class action
correspondence.

**Headline: a sound 4-atom relational descriptor exists.** Exhaustive search
over all subsets of size ≤ 4 finds 8 minimal solutions, e.g.

> `{comp41, s3max2, team(2-0), team(4-2)}` — 33 cells, purpose-sound for R8

where `comp41` = which tile shares the 4-1 holder's hand, `s3max2` = the
partner's best suit-2 rank, and the two team atoms say which side holds the
forced follower 2-0 and the mid beater 4-2. Compare the holder-only basis:
its smallest sound set (5 of 6 holders) has **90 cells — one per world, zero
compression** — while the relational basis is sound at 33 cells. Every
minimal solution is control-shaped: each contains a suit-2-strength atom and
the companion atom (who travels with the valued tile), exactly the
"continuation control rather than current ownership" coordinates §12.12
predicted. No smaller descriptor exists even for the 3-class target R3 in
this vocabulary (its minimal solutions are the same four-atom sets).

The counterexample-guided greedy loop (start from `team(4-1)`, add the
impurity-minimizing separator per counterexample pair) converges but
overshoots: sound after 7 refinements with 8 atoms / 35 cells. On this
corpus, exhaustive minimality beats greedy synthesis — worth remembering when
the loop is scaled.

Honest caveats: the vocabulary was hand-registered after seeing the domain,
and n = 90 is small enough that a 4-atom / 33-cell descriptor can overfit;
the real test is transfer of these *atom shapes* (companion, suit-strength,
forced-follow) to a fresh kernel. And 33 cells against an 8-class target is a
strict approximant, not purpose-exactness.

## Experiment 3B — horizon 3: where the first interior breakpoint lives

Domain: trick-5 start of hand 0. S1 leads holding {0-0, 2-1, **3-2**} — the
last live trump, so the trick-5 state has exactly one absolute master and a
genuine 3-action root. The 9 unseen tiles contain no trump, the observed
voids are vacuous, and the fiber is the full **1680 worlds** (true world
present).

### Perfect information: completely affine — a surprise

Per-world parametric minimax over 1680 worlds × 3 roots × 12 valued-tile
directions = **60,480 exact parametric solves: every root Q is a single
line**. Zero multi-segment values, zero interior argmax switches, in any
direction. The v0.3 doc expected horizon 3 to produce genuine interior
policy crossings; under perfect information it does not — capture destiny
under optimal play is λ-invariant across this entire domain, exactly as it
was at horizon 2. (Validated by 216 independent scalar minimax spot checks.)

### Imperfect information: the breakpoints appear, richly

The fixed-field information-set solve (uniform belief over 1680 worlds,
uniform-random legal field; S1's only decision after the root is its trick-6
play, so each Q(root; λ) is a **sum over info-sets of convex envelopes** —
its breakpoints are exposed-vertex changes of the action polytope):

For d ∈ {**1-1, 4-1, 4-4, 5-1**} — exactly the four unseen tiles *without* a
2-pip, the tiles that can never be forced into the suit-2 endgame:

> Q(0-0) = 37/21 + (22/35)λ on [0, 1/5); 26/15 + (27/35)λ on [1/5, 4);
> 176/105 + (11/14)λ on [4, ∞) — **three exposed segments**
> Q(2-1) = 5/3 + (20/21)λ; Q(3-2) = 37/21 + (4/7)λ — single lines
> **ROOT SWITCH at λ\* = 7/19**: lead 0-0 → lead 2-1

Both Q(0-0) vertices are genuine trick-versus-capture trades: at λ = 1/5 the
exposed policy gives up trick value 37/21 → 26/15 to raise the capture slope
22/35 → 27/35, and again at λ = 4 (26/15 → 176/105 for 27/35 → 11/14). At
λ\* = 7/19 S1 abandons the safe double lead entirely for the 2-1 gamble,
whose capture slope 20/21 beats anything the 0-0 line can expose. The other
eight directions (every tile with a 2-pip, plus S1's own holdings) stay
single-line: their destinies are pinned by follow obligations.

This establishes, at probe tier, item 10 of the doc's not-established list:
**a positive interior valuation breakpoint in an information-set-consistent
horizon-three solve** — with the polytope showing ≥ 2 exposed vertices, a
genuine trade, and the full terminal laws retained. One structural aside:
the info-set counts per root are 0-0: 168, 2-1: **7848**, 3-2: 504 — leading
the weak tile creates ~47× the epistemic branching of leading a master; safe
leads keep the observation tree small.

## The probe's answers to §16.15's search criteria

| criterion | verdict |
|---|---|
| λ\* > 0 with a suboptimal-at-0 policy becoming optimal | **yes** — λ\* = 7/19 (root), plus policy vertices at 1/5 and 4 |
| one root-action polytope with ≥ 2 exposed λ≥0 vertices | **yes** — Q(0-0) has 3 exposed segments |
| genuine trick-versus-capture trade | **yes** — both vertices |
| full terminal laws available | yes (retained per info-set) |
| world-dependent answer bundle | yes at horizon 3 (mastery after trick 5 depends on the hidden deal) |
| …under perfect information | **no** — all 60,480 PI solves are single lines |

## Weighing in (Claude's commentary, invited)

1. **The compression result is the one to sit with.** This was the first
   actual run of the doc's R\* = R̄∘D loop, and it converged: four
   control-shaped relational atoms explain what five-of-six holder facts
   (i.e., the entire hidden world) were needed to express. The atoms that
   win — who travels with the valued tile, who is strong in the suit that
   decides the endgame, which side holds the forced follower — are the
   beginnings of the game's own ontology surfacing from exact behavior, which
   is precisely what the constellation program is for. I'd resist declaring
   victory at n = 90: the vocabulary was written after seeing the domain.
   The transferable claim worth testing next is that *atom shapes* (companion,
   suit-strength, forced-follow) stay sound across kernels, not these
   particular four atoms.

2. **The PI/fixed-field split looks like a finding, not a failure.** The doc
   predicted horizon 3 would produce interior PI breakpoints; instead all
   60,480 PI solves are affine while the imperfect-information solve produced
   breakpoints immediately. On this evidence, valuation sensitivity in this
   domain is an *epistemic* phenomenon: with full information, optimal play
   never has to trade tricks against capture; under uncertainty the trade is
   real and priced (λ\* = 7/19). I'd log "PI parametric minimax Q is affine
   on small trick-suffix domains" as a new OPEN conjecture to refute or prove
   — two horizons and one receipt hand are consistent with it, and either
   resolution is informative. If it holds broadly, the polytope layer's
   interesting geometry lives entirely above the support layer, which is a
   pleasing echo of "constellations need the decision carrier, not the world
   carrier."

3. **Where the sensitivity localizes is beautiful**: the four breakpoint
   directions are exactly the unseen tiles that can never be forced to
   follow. Tiles pinned by follow obligations have valuation-inert destinies;
   free tiles are the ones play can fight over. That is control-versus-
   location again, now in the valuation geometry — the same lesson from three
   independent instruments.

4. **The bugfix earns a mention in the doc's own terms.** A live PWL bug
   survived two probe generations because it was unobservable on single
   lines; the first multi-segment domain exposed it within hours, the fix
   changed nothing upstream (re-verified bit-identically), and independent
   scalar validators now cover both solvers. That is Phase G1 working as
   designed, and a good argument for keeping the "attach and independently
   rerun" bar before any promotion.

## v0.31 caveats

- Everything here remains exploratory-probe tier: one receipt hand, two
  suffix domains, named belief/field choices (uniform × uniform).
- The 3A search is exhaustive only to size 4 over a 22-atom registry;
  minimality claims are relative to that registry.
- The horizon-3 fixed-field solve exploits S1 having a single post-root
  decision; deeper domains need the general per-info-set DP.
- The PI affineness observation is an empirical pattern over 60,480 solves,
  not a theorem; treat as OPEN.
