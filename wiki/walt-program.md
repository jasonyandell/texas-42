[Home](Home.md) · owns: what the walt program is trying to do, every direction reset it has taken and why (2026-08-09 → 2026-09-07), and the working method that governs how it builds · Sources: [`walt/LOG.md`](../walt/LOG.md) (session index; the retired `walt/PLAN.md` survives at `git show 56e2173:walt/PLAN.md`), [`walt/MAP.md`](../walt/MAP.md) (walt on one page, 2026-09-04), the frozen bases and intakes under `walt/math/`, the append-only adjudication record [`walt/CENSUS-RULINGS.md`](../walt/CENSUS-RULINGS.md), the binding briefs and reports of record under [`walt/briefs/`](../walt/briefs/), [`walt/FACTOR-BELIEF.md`](../walt/FACTOR-BELIEF.md) (the running build record from 2026-08-30), [`walt/DISCREPANCIES.md`](../walt/DISCREPANCIES.md), `experiments/partnership/SESSION-STATUS.md`, and the repository at `c00717d1` (2026-09-07). Related: [walt hub](walt.md), [timeline](timeline.md), [vocabulary](vocabulary.md), [negative results](walt-negative-results.md), [seat play](walt-seat-play.md), [calculated evidence](walt-calculated-evidence.md), [counted belief and anytime](walt-counted-belief-era.md), [focal-horizon era](walt-focal-horizon-era.md), [Gran anchors](walt-gran-anchors.md), [partnership program](walt-partnership-program.md), [gym](walt-gym.md), [Scheme/Fix](walt-scheme-fix.md), [decision-sparse](walt-decision-sparse.md), [pre-pivot results](walt-pre-pivot-results.md), [architecture](walt-architecture.md), [instruments](walt-instruments.md), [math reference](walt-math-reference.md), [lineage](lineage.md).

# walt — the program, its resets, and its method

> **Epistemic tier: EXPLORATORY — below every tier on
> [Home](Home.md#evidentiary-tiers--never-promoted-never-blurred).** Everything
> under `walt/` and `experiments/` sits on walt's own frozen exploratory bases.
> Every number on this page is a probe record or a gate-pinned walt value, named
> as such; nothing here may be quoted in a brief, a dispatch,
> [FINDINGS](FINDINGS.md), or any claim-tier page. Repository state is described
> as of 2026-09-07 (`c00717d1`).

This page is the spine of Part II. It says what the program is for, then walks
every change of direction in order — fifteen of them in thirty days — with the
result that forced each, and closes with the working method, which is the part
of the program that has never been reset and is its most durable product.
A newcomer should read the first two sections and the reset table; a
mathematician the resets and the method; an engineer the method and
"Where the program stands", then [instruments](walt-instruments.md).

## The question walt exists to answer

[rob](rob.md) answers *what is exactly true* — the exact solver, receipts and
all. **walt is the seat**: the full-hand imperfect-information player that has to
act from one chair, seeing only what a chair legally sees. The name is continuity
rather than coincidence. In the predecessor project mk5, walt was the exact
four-tile endgame information-set solver — the first artifact that provably had a
plan and cashed it ([lineage](lineage.md)). This walt is that idea attempted at
full scale.

The obstacle is stated precisely on [the game page](game-of-42.md): a seat's
knowledge of the three hidden hands is exactly representable and cheap to count,
but knowledge is not enough to play by — the 90-world witness proves two
positions with identical exact knowledge can demand opposite play. So the seat
needs something else to carry, and the program is a sequence of increasingly
sharp guesses about what that something is. The guesses so far, in order: a
compressed descriptor of the hidden state (refuted); a small canonical situation
space (refuted by proof at the first play); a low-dimensional value closure
(refuted); a proof of the root action without solving every action (partly
built); a sampling player that simply plays (built, and it beat the champion);
adaptive settlement instead of fixed sample counts; exact integer masses instead
of estimates; an anytime proof state that returns a certified-regret
recommendation instead of a one-shot answer; the opponent model as a hidden
coordinate; one hierarchy of intervals indexed by how many focal decisions are
made exact; and, most recently, an expression language in which a situation can
be written down and searched for.

Two commitments have never moved. **Math first**: nothing is built before it is
adjudicated, and every construction goes to a walt-math consultant as a design
document before a line of code. **Exactness**: integers and rationals only, no
floating point anywhere near a value, caps that exclude rather than sample, and
every declared stop printed in the artifact that reports the run. A third
commitment was ruled on 2026-08-17 and has bound every player since: the
objective is **pmake**, P(make the bid), because 42 is scored in marks and trick
differential is only a proxy (`walt/SCENARIO-PLAYER.md` Def 6.2; the ruling is
recorded on [seat play](walt-seat-play.md)).

## The resets at a glance

| # | Date | Direction | What forced it | Record |
|---|---|---|---|---|
| 1 | 2026-08-09 | Freeze the basis (v0.4), build greenfield Rust, carry the dynamic control skeleton | the opening decision | [foundation era](walt-foundation-era.md) |
| 2 | 2026-08-10 | CDCL-style lesson factory as the outer loop | typed conflict objects from both checkers | [factory era](walt-factory-era.md) |
| 3 | 2026-08-10 (evening) | Re-tether: the lossless count-free equivariant quotient (§12.6A, v0.5); NO-RESCUE adopted | the build had come untethered; S4 compared interface alphabets raw | `walt/math/equivariant_lumpability_v0.5.md` |
| 4 | 2026-08-10 → 08-11 | Jason's bar: canonical situations of order 10^5, show or disprove | the bar's object migrated three times as measurement came in | [census era](walt-census-era.md) |
| 5 | 2026-08-12 | Predictive algebra v0.6: exact rank over the rationals | the first-play quotient is the identity (S5k) | [S6 era](walt-s6-era.md) |
| 6 | 2026-08-12 → 08-13 | Similarity of outcomes, then decision-sparse exact solving | Gate B refuted; 7/9 singleton frontiers with a STOPPED verdict | [decision-sparse](walt-decision-sparse.md) |
| 7 | 2026-08-17 | Build the seat that plays; pmake is the objective | the m4/S6 compression search frozen; grade 4 exhausted as a test-bed | [seat play](walt-seat-play.md) |
| 8 | 2026-08-24 | One walt: the seven-crate fold, the research producers archived | seventeen crates were three stacks and three orphans | `walt/UNIFICATION-CENSUS.md`, `walt/ARCHIVE.md` |
| 9 | 2026-08-24 | Calculated evidence: fixed sample counts leave the correctness path | the 40-vs-160 flip; phone-tier caps mistaken for statistics | [calculated evidence](walt-calculated-evidence.md) |
| 10 | 2026-08-30 | Counted belief: exact masses replace estimates; the sampled instrument becomes the fallback tier | the shadow instrument honestly Unresolved at tricks 1–3; the modeled minds are the bill | [counted belief](walt-counted-belief-era.md) |
| 11 | 2026-08-31 → 09-01 | Anytime proof state: a certified-regret recommendation, never a one-shot answer | RefineV1 refused the opening root by forecast | [counted belief](walt-counted-belief-era.md) |
| 12 | 2026-09-01 → 09-03 | Model belief and the unified player: "42 is 2 recursions running in opposite directions" | God-tightness at t5/t6; the fusion horizon; the carry was the wall | [focal-horizon era](walt-focal-horizon-era.md) |
| 13 | 2026-09-04 | Focal-horizon unification: three instruments become one object [L_k, U_k]; then consolidate | residual width = the tail's policy gap, not fusion price | [focal-horizon era](walt-focal-horizon-era.md) |
| 14 | 2026-08-24 carded; 2026-09-04/05 measured | The Gran anchors: one human partnership failure as the standing anchor | Jason's partner held the 6-4; the hoarding mechanism is a tie-break | [Gran anchors](walt-gran-anchors.md) |
| 15 | 2026-09-06 → 09-07 | Scheme for expression; the partnership program and the exact gym | "invented to compress; commissioned here to express"; no strength gain established | [partnership](walt-partnership-program.md), [gym](walt-gym.md), [Scheme/Fix](walt-scheme-fix.md) |

Standing across 8–15: **no default player changes without arena and
conformance gates, on Jason's word** (CE-A7/§20.16, restated verbatim at
CBS-A9, APS-A9, MB-A7 and FH-A10). The live default player, `walt_bridge` at
level 1 with the θ = 11/16 auction default, is unchanged by everything since
2026-08-19.

## The resets

Each reset was a deliberate call by Jason after a result came in, and each is
recorded with what forced it, because the sequence is the actual shape of the
program. Resets 1–7 are the research eras (2026-08-09 → 08-17); 8–15 are the
player eras (2026-08-24 → 09-07).

### 1. Freeze the basis and build the skeleton (2026-08-09)

The opening decision: freeze the mathematical basis at v0.4, build greenfield
Rust against it, and carry the **dynamic control skeleton from the jump** — a
descriptor is a transducer, state plus a closed update law, never a labeling. The
factory would grade every candidate on two axes, soundness for a response target
and lumpability of its update, both exhaustively checkable on finite kernels. If
no nontrivial lumpable skeleton existed on honest domains, that was to be a
reportable result rather than a failure. Delivered as
[the foundation era](walt-foundation-era.md); the first synthesis run returned
exactly the reportable negative the design had anticipated.

### 2. Conflict-driven lesson learning as the outer loop (2026-08-10)

With typed conflict objects coming out of both checkers, the factory's outer loop
was organized CDCL-style: harvest failure, generalize it, prune with it. This was
imported as a *stance*, not an algorithm — from the one community that made
exhaustive search industrial and whose safety culture (proof logging, independent
checkers, "never trust the solver") independently evolved this project's own
receipt discipline. The declared regime was few conflicts deeply analyzed, never
industrial throughput. Delivered as [the factory era](walt-factory-era.md).

### 3. The re-tethering: the lossless count-free equivariant quotient (2026-08-10, evening)

The hinge. After a fresh full read of the 3,820-line basis, Jason's diagnosis was
that the build had come untethered from the mathematics. Three findings supported
it. The basis had already ruled worldwise perfect-information classes the wrong
carrier for the hidden decision, so the label-fragility result had re-confirmed a
boundary drawn in ink. The basis already named the dynamic predictive quotient as
the target. And the lumpability instantiation compared observation and feature
alphabets **raw**, which is exactly why only world-reconstructing descriptors
could pass — the mathematics quotiented the state side but never the interface
alphabets.

Jason authored the missing theorem in session: **§12.6A, equivariant controlled
lumpability**, opening the v0.5 track and leaving v0.4 frozen. Two situations are
the same when, given what the seat knows and does not, one policy applied through
declared transports to any matching world produces the same outcome under the
quotient. The goal was restated as the **lossless count-free equivariant
quotient**, and the factory and economy infrastructure was frozen until the
compression question moved.

Two policies date from this session and still bind. The **no-rescue policy**: a
failure is a counterexample to carry back to the math, never a thing to fix,
spin, or engineer around. And its corollary on verification: verify against the
reference we have rather than in triplicate, and when independent mechanical
verification is genuinely needed the path is Lean, not Python — which retired the
planned Python checker and left the lesson economy's triggered deletions
mechanically blocked, safely, by design.

### 4. The bar, and the migration of its object (2026-08-10 through 2026-08-11)

With the goal restated, Jason declared the bar: **show or disprove that the count
of canonical situations is reasonably small, order 10^5** — either outcome a
result. The bar's *object* then moved three times as measurement came in, and
following that migration is the clearest way to read [the census
era](walt-census-era.md).

| The object the bar was tested against | What happened |
|---|---|
| World-level trick-six roots | Structural quotients merge none of 647; the retrograde quotient reaches 306 |
| World-level trick-five roots | Compression *weakens* going earlier, 1.25:1; the inventory is unconverged; trick-1 world-level roots extrapolate astronomically |
| The **seat**-level census at the first play | The bar's true object, per Jason's clarification: the situations facing the leader of the first hand |

The seat-level question was then answered **by proof rather than by
enumeration**, and answered no: the first-play structural quotient is the
identity, so the count is exactly C(28,7) = 1,184,040, about 11.84× over the bar.
The insight that came with it reframed the program —
[structural compression is bought with deadness](walt-negative-results.md), and
nothing is dead at the first play.

### 5. Predictive algebra: the linear escape, and its refutation (2026-08-12)

Jason brought a new mathematical track, v0.6: exact predictive rank over the
rationals. The escape it offered from the rigidity result is real in principle —
linear rank can sit far below partition-lump size, so every behavioral row can be
distinct while the rank stays small. Adjudication immediately converted most of
the program into theorems (any closure with a constant in its terminal seed is
degenerate at full dimension, because complete records determine worlds), leaving
one measurable object. That object was measured, and the pre-declared payoff
criterion was **refuted**: the value closure saturates by grade three.

### 6. Similarity of outcomes, then decision-sparse exact solving (2026-08-12 to 2026-08-16)

Jason's response to the refutation redirected rather than retreated: his actual
hope was never exact low dimension but **similarity of outcomes** — that a hand
tends to land near the same place regardless of the path. The exact, adjudicable
fragment of that is the dual policy geometry, and measuring it produced the
program's most suggestive result: at seven of nine measured grade-3 pairs a
single policy weakly dominates every lawful alternative in all 1,680 worlds,
against raw plan counts up to 2^19930, while the two exceptions explode exactly
where the 42 is genuinely tense. The dissent travels with the number: those two
exceptions blew past the declared frontier cap, a partial frontier bounds
nothing, and **the probe's formal verdict is STOPPED with no verdict** — the
bimodality is the finding, never a seven-of-nine success. Read beside the
dimension census, **value richness and decision simplicity coexist**.

That pairing is the thesis of the decision-sparse track. Jason's
`decision_sparse_exact_solving_v0.1` moves the target from compressing truth to
**proving the root action**: a lawful lower witness against an
action-conditioned upper witness, L_{a*} ≥ U_a, certifying the opening play
without solving every action exactly. It has been filed verbatim, twice audited,
and its repaired mathematics maintained in an errata beside it. Its Experiment A
(deadness detection at census scale) and Experiment E (root-action separation)
are complete; Experiment D's two rungs — the fusion tax (S6k, one gluing-cut
closure at h6 with surplus 4930081/479001600) and the second rung (S6l, escape
actions present at 36 of 330 and 498 of 1,320 first-frontier states) — were
computed at grade 4, and Proposition SR-degen retired grade 4 as a test-bed. The
economy claim split into a primal half (answered at trick 4 on real deals by
four-word rules, S6j) and a full claim never tested. The seed survey (SS-A1..A18,
2026-08-15/16) was the branch's first carrier not selected by outcome. The whole
track is [decision-sparse](walt-decision-sparse.md); its results by result are
[pre-pivot results](walt-pre-pivot-results.md).

### 7. Build the seat that plays (2026-08-17)

The pivot that made walt the project's player. After the 2026-08-10 freeze of
the m4/S6 compression search and the exhaustion of grade 4, Jason's standing
direction was executed literally: stop compressing truth, **build the seat that
plays** — a sampling-stack player, iteratively improved, with the mathematics
catching up behind it on the record. Two rulings shaped it the same day. The
objective is **P(make the bid)** — pmake — because 42 is scored in marks and
trick differential is only a proxy; and the roadmap is explicitly *not* a
months-long best-response chase — the goal is a player that "doesn't play gross
42" (Jason's phrase, recorded in the session record of 2026-08-17 rather than in
a repository file), then beliefs, then partnership. In one day the seat went from
scenario harness to playing full hands; the same day, level-1 walt beat the mk5
E[Q] champion 630/1152 pooled (McNemar z = +6.28; `walt/probes/m3/arena_results_2026-08-17.txt`;
an arena outcome, exploratory) — losing about 4.7 points per hand and winning the
marks, the pmake objective visible in data. By 2026-08-19 the θ = 11/16 auction
default was calibrated and the seat was live in plunge. The whole arc is
[seat play](walt-seat-play.md); the spec-after-build and its obligations ledger
are `walt/SCENARIO-PLAYER.md`. The earlier tracks were not abandoned — the
decision-sparse program's two-sided witness is the direct ancestor of resets
10–13.

### 8. One walt (2026-08-24)

The unification census found that the "17 crates" were three disjoint stacks
plus three orphans, and that only one stack was the thing walt currently was —
the player that beat the champion and runs in plunge. The fold made seven crates
into seven modules of one crate (`d1499d4`), the research producers `walt-factory`
and `walt-skeleton` were deleted with their bytes preserved at commit `648f93a`
and their 65 result summaries relocated to `walt/probes/factory-results/`, seven
completed probe design docs were retired (bytes at `git show 2de8a05:walt/<NAME>.md`),
and the wiki was re-synthesized as the book's first pass (PR #9, base `2de8a05`).
Freeze 56 was re-issued append-only as v2 at the new layout (FZ-A1..A6). This is
the base every later reset builds on: one crate, `walt/walt/`, with the gate
`walt/ci/check.sh`. Details: `walt/UNIFICATION-CENSUS.md`, `walt/ARCHIVE.md`,
[architecture](walt-architecture.md).

### 9. Calculated evidence: magic sample counts leave the correctness path (2026-08-24)

The playing seat's numbers were sampled estimates at fixed counts — 40 worlds
here, 160 there, 200/8 at the auction — and a historical flip (a decision that
changed between 40 and 160 worlds) had no honest reading. Jason hand-ferried two
Pro parents the same day; both were intaken with exact stdlib verifiers (18/18
and 19/19), adjudicated (CE-A1..A8, L2-A1..A7), and built through CE §22 step 8
and L2 §21 steps 3–8 in one day. The ruling that names the reset is **CE-A5**:
"fixed sample counts leave the correctness path — they may persist only as
replay fixtures, heuristic-fallback defaults, historical coordinates, or
throughput batch sizes." What replaced them: an exact-rational evidence engine
(betting supermartingales, CE-T1..T5) with a run → decision → edge risk ledger, a
binding six-way result ladder (ExactFiberRoot / ExactFrozenSet / DeltaSettled /
EpsilonEquivalent / Unresolved / HeuristicFallback) in which a sample cap is a
resource limit, never a proof rule, and **Unresolved is a successful output**.
Jason's cap ruling the same day: world cap 512 is the way forward — 128/40/160
were phone-tier budget limits, not statistical choices.

What the era measured decided the next reset. The shadow instrument beside the
live player (183 decisions over 33 hands at cap 128: 67 exact / 116 Unresolved /
0 δ-settled; at cap 512 three settlements, two against the live choice) was
honestly Unresolved at every trick-1 to trick-3 decision; the historical
40-vs-160 flip dissolved into an honest near-tie at every cap (step 8, V5 law
gated); the waking-seat profile put 729‰ of decision compute in the σ0 baseline
and 926‰ in tricks 1–2; and the four value-identical sharing levers of the speed
campaign bought about 4%, about 3% and about 1.04× on that regime (all
[negative results](walt-negative-results.md#the-four-sharing-levers-are-a-wash)).
The conclusion, in the era page's words: the modeled minds are the bill; sharing
levers are exhausted. The level-2 program (field swap, exposure rungs, the
cancellation ladder, the wake-up detector, the targeted controller, the waking
seat) and the three deferred producers from x:024 were built in the same window;
"level 2" is a best response to a named σ1, never equilibrium (L2-A7/O36).
Era page: [calculated evidence](walt-calculated-evidence.md).

### 10. Counted belief: exact masses replace estimates (2026-08-30)

If the modeled minds are the bill, stop paying it per world. Jason's
counted-belief parent (CBS-A1..A9) turned the trick-1 root's 399,072,960 worlds
into 116,280 acting-seat hands × 3,432 exact-cover completions (pure
combinatorics, gate-pinned in `walt/walt/tests/solver_factor_belief.rs`), and
Theorem 20.1 — conditioning on an observed hidden action multiplies only the
acting seat's factor — kept the posterior a product of seat factors. The C→G
ladder was built in one day (PRs #61–#69): root intervals and survivor sets
(Slice A; CBS-A3 retired "sandwich" as an object name), the grammar/residual
triple (B), the factor belief and exact-cover oracle (C: the opening root's exact
one-ply branch table in 8.7 ms under a trivial field, 5.36 s under σ0, no world
materialized — record `walt/probes/factor_belief/c2_run1.txt`), the factorized
fixed-policy recursion V = M/Z in integers (D), the grammar best response (E),
consequence CEGAR (F), and the integrated controller `refine_root` (G, frozen as
RefineV1 at freeze 58). **The sampled instrument of reset 9 became the fallback
tier**: Slice A consumes the CE one-mean engine through the sanctioned crossing,
and every exact route refuses by forecast rather than sampling. The honest
negatives of the ladder — cross-history cache reuse exactly zero, the tail
fragmenting to 116,280 singletons, the recursion slower than the bundled walk at
small fibers — are on the [negative results](walt-negative-results.md) page.
Era page: [counted belief and anytime](walt-counted-belief-era.md).

### 11. The anytime proof state: a certified-regret recommendation, not an answer (2026-08-31 → 2026-09-01)

RefineV1 at the opening root with a budget of 100,000 refused every exact item
by forecast and returned UNRESOLVED with fallback 0-0 named and never promoted.
Jason's second hand-delivered parent (APS-A1..A9) changed what kind of thing the
solver returns: a persistent, append-only **proof state** whose closure is a
derived view, and a **certified regret Γ = U* − B_exec** — the gap between the
best upper bound over actions and the executable bar witnessed by a materialized
policy — with the recommendation block of §33. "Certified regret" is walt's own
term of art (APS-A6, FH-A2), deliberately checked not to collide with the D3 ban
on "certificate". Phases 0–8 landed in two days (PRs #71–#78): freeze 58, the
43-bin score profile, the §49 architecture spike, Phase 3 regret, Phase 6 argmax
extraction (h3-t4's Γ 83‰ → 0‰ exactly, recommendation 4-4 → 3-1), the work
frontier, the residual-Bellman staircase and covers, the typed laydown hierarchy
(bare "laydown" reserved for the universal type), and the §65 opening-root
ladder. Its verdict at receipt root h0-t1 (contract 30, seven legal leads,
p = 16/64/256/512): **play 6-5, floor 732‰, at most 267‰ unclaimed, honest
UNRESOLVED at ε = 1/4**, the sampled tier plateauing at p = 512 (record
`walt/probes/factor_belief/openingreport_run1.txt`; the ladder's laws gated in
`solver_opening.rs`, the numbers probe-only). §65's first target was exactly
that — a useful certified-regret recommendation under a playable budget, not a
seven-trick exact solve — and it was met in that form.

The doom census followed (PR #79): counterexample mass as deterministic uppers,
recovering 809–1000‰ of per-world doom on enumerable roots but certifying zero
doomed worlds on all seven opening leads. The ledger paragraph's gloss that the
remaining 267‰ was "overwhelmingly the info-consistency price" was **corrected
2026-09-03** in `walt/DISCREPANCIES.md`: a zero doom census moves only d_phys,
and "the honest statement is that the 267‰ is UNKNOWN in its split". That
correction travels with every citation of the opening verdict.

### 12. Model belief and the unified player: two recursions running in opposite directions (2026-09-01 → 2026-09-03)

The salvation-complex intake (SC-A1..A8) named the era's "structural saturation"
as God-tightness — the per-world doom truth and the exact lawful value close at
the God upper at every trick-5/6 receipt coordinate — and the U0 census made
the decomposition 1 − V = d_phys + d_info + d_policy mechanical: on the receipt
corpus the earliest fusion-free depth is trick 5, every saveable trick-4
coordinate carries a positive information price of 6–22‰ with d_policy = 0, and
the opening is typed UnknownGodGap (record `godgap_run1.txt`, six gates,
never theorem language per SC-A4). Jason's frame is on the record in the
briefs verbatim — `walt/briefs/BRIEF-MB1.md`: *Jason's official framing governs
root selection: "42 is 2 recursions running in opposite directions" — late
tricks are enumerable and hold no model uncertainty … The model-belief physics
lives earlier.* And `walt/briefs/BRIEF-UP0.md`: *Jason's frame, now
twice-measured: "42 is 2 recursions running in opposite directions." UP0 is the
first artifact that PLAYS both recursions — enumerable exactness walking
backward, sampled/structural play walking forward — and knows at every depth
which one it is standing in.*

What was built on that frame. **MB0/MB1** made the opponent model a persistent
hidden coordinate, Ξ = Ω × Θ, with exact integer priors and merge-before-max:
the model-fusion price Φ is zero at all fourteen trick-5/6 root-action
coordinates (MB0's criterion 4 an honest NO) and **strictly positive at trick 4**
— eight of eight substantive coordinates, h8-t4 3-1 Φ = 38/9600 pinned by gate
M6 — while a trick-3 mixture coordinate refuses at the declared 7,000,000-read
ceiling. The **σ1 repair** (PR #83) found and terminated the level-1 sampler's
empty acceptance region and deduplicated five byte-identical copies. **UP0**
built the unified player — one total five-tier cascade, decided arithmetic →
endgame exact → middlegame mixture → certified regret → σ0 fallback, with
unforgeable provenance and `Recursion::direction()` answering backward or
forward — and measured that 99.4% of the lean rung's wall was carrying a
posterior nothing read; **UP1a** made the carry lazy (2.1 s → 0 µs, lazy ≡ eager
on all 216 decisions, gate UC5). **U0b** found the trick-5 frontier inside a
trick-4 solve is not fusion-free, that a trick-6 cut is 0–7‰ in value yet flips
the root play in two of thirty rows, and solved h8-t3 exactly under σ0 for the
first time: Q* = 28859/29988 (962‰), argmax 1-1, 289,407,472 field reads,
797.43 s (`horizon_run1.txt`). Scalar closeness is not decision safety — a
measured fact from this reset on. Era page:
[focal-horizon era](walt-focal-horizon-era.md).

### 13. The focal-horizon unification, and the consolidation ruling (2026-09-04)

Pro's focal-horizon note was intaken at instrument tier (FH-A1..A11; Theorems
1–6 step-checked; **FH-A2 retires "sandwich" as an object name** — the objects
are the focal-horizon hierarchy, the focal-horizon interval [L_k, U_k], the
action interval, the bar B_k, the survivor set S_k). In one day the hierarchy
was built (FH1), given a stop/resume ladder over a store of node facts (FH2), run
as a report of record at 33 (root, contract) coordinates (FH3), audited
independently (FH4: one BLOCK, vocabulary; thirteen NOTEs; every record
reproduces), and repaired (FH5). Three instruments became one object: the
God-gap census is U_{a,0}, the ply-cut census is U_{a,m−1} on viewer-lead roots
(Proposition FH-cut), the never-built salvation-mask upper is U_{a,1}, rollout
improvement is L_k, argmax extraction is π_k, and the exact endpoint is the
collapse (Proposition FH-last).

The measurement that changed the direction: **at k = 1 the remaining width is
the tail's policy gap, not fusion price** — Q − L is 9–41‰ per action while
U − Q is 0–3‰ (`focal_run0.txt`, `focal_run1.txt`; probe record, no gate pins
the split). Every live trick-4 coordinate settles by k ≤ 2 (five at k = 0, six at
k = 1, three at k = 2; Γ₁ ≤ 45‰ everywhere); h8-t3 settles only at the k = 3
collapse, reproducing the 14-minute exact value; neither ply-cut argmax is ever
certified (`solver_focal_anchors.rs`). So a better lawful tail buys more than a
deeper search on this corpus — better tails first, then consolidation. The cost
was stated as a finding: 19.4 GB peak RSS at h8-t3, the gate 230 → 308 s.

Jason's ruling of 2026-09-04, recorded in `walt/MAP.md`: **"No new mathematical
parent until the consolidation lands"** — *Jason, 2026-09-04: "follow through on
what we have, then invest in a simplification/unification attempt"*. The
consolidation slice is named there: `godgap.rs` (933 lines), `horizon.rs` (635)
and `extraction.rs` (135) are measurement scaffolding around one recursion, and
`refine.rs` (917, freeze 58) was already declared removable; `doom.rs` stays as
the God tail's engine. Before it, the σ0 read-key study (does σ0's answer depend
on the full public record? if not, every recursion gets 10–100× cheaper). As of
`c00717d1` neither had a brief or a commit; both were carded on 2026-09-13 in the
book rewrite (`kanban/backlog/sigma0-read-key-study.md`,
`kanban/backlog/consolidation-slice.md`). The walt-fh branch
merged as PR #88 on 2026-09-07 (fast-forward, tip `a0d594b2`).

### 14. The Gran anchors: one human failure as the standing anchor (2026-08-24 → 2026-09-05)

On 2026-08-23/24 Jason, declaring 30 on sixes in a live Plunge game, watched his
walt partner ("Gran", the live level-1 seat) hold the ten-count 6-4 and play the
6-2 at trick 1; the hand went 25–17, set. Three screenshots were archived with a
manifest (PR #25) and the L2 intake carded the anchors G1–G4 (L2-A6). On
2026-09-04 the failed hand was transcribed tile-by-tile and mechanically
validated by the rules layer (28 tiles, every follow, every winner, the 25–17
verdict re-derived; `walt/probes/gran/g1.receipt.txt`, `granrun validate`,
re-run 2026-09-12 on this machine: VALIDATED in under a second), and the made
hand committed as a six-trick partial whose residual is six-way ambiguous
(G2/G3). The waking seat played G1: it plays the 6-4 at trick 1, **but the wake
did not fire** — σ0 already chose 6-4, so the trick-1 flip is an epoch/baseline
difference against the live seat's 40-world sampler, not partner modelling; its
one real wake, at trick 5, reproduced the human play.

The 2026-09-05 morning readout (`walt/briefs/MORNING-2026-09-05.md`, the only
main-side record of two branches that remain unmerged at `c00717d1`) then
located the mechanism: G2 is exactly locked from trick 3 (all 280 deals at trick
4 make whatever Gran plays; level 1 gives the identical tie set); at an exact tie
at pmake = 1 there is no gradient, and the seat's tie-break
(`TieRule::LowestTileIndex` plus `best_of` keeping the incumbent) discards
ascending by domino index — the 6-4 is index 25 of 28, so count is hoarded
deterministically. Level 2 hoards *more* (holds the 6-4 at both legal G1 nodes,
trick 3 exact 654 vs 640). The design conclusion, on the record: another rung
of modelling is not the remedy; the levers are the objective (pmake pins at 1)
and an explicit tie-break at exact indifference. Jason's calls (A)–(F) from that
readout are open. The anchor's page is [Gran anchors](walt-gran-anchors.md).

### 15. Scheme for expression; the partnership program and the gym (2026-09-06 → 2026-09-07)

A two-day Codex session (31 commits, `d8400713` … `c00717d1`) run under an
explicit **CI waiver** — every landing says "full CI deliberately skipped under
the session waiver" and no `walt/ci/check.sh` run is recorded on these commits.
Three things landed, all EXPLORATORY, none changing a default:

- **Scheme/Fix as an executable language** (`walt::scheme`, commit `b764665f`):
  typed role schemas, a versioned predicate registry, exact finite-world belief
  events, explicit selectors, counterexamples. Jason's direction, in the LOG's
  words: *Invented to compress; commissioned here to express.* No compression,
  player-strength or compact-transducer claim. The receipt example (partner
  holds the count tile at hand 0, trick 6: probability 1/3) re-run 2026-09-12 on
  this machine reproduces the README exactly. Page: [Scheme/Fix](walt-scheme-fix.md).
- **The partnership program** (`experiments/partnership/`): the archived phone
  WASM pinned as the strength reference (SHA-256 `af0200af…`, byte-identical to
  `walt-wasm` at `9a056f20`, 2026-08-19); a bounded partner-aware sampling-stack
  player (partner at level 1, opponents at level 0; 40/8/2; 14 s wrapper); a
  shared ten-game pool with resume proofs and an independent replay verifier;
  matched batteries of 300, 300, 150, 524 and 400 games. **No strength gain was
  established anywhere** — L2 Partner ties L1 14/14/72 at about five times the
  cost — and L1 stays the operating default. Page:
  [partnership program](walt-partnership-program.md).
- **The exact partnership gym** (`walt/gym`, driven by `experiments/partnership/gym.py`):
  six starter count-offer exercises with exact answer keys, then Scheme-driven
  discovery of 170 distinct strict coordinates, 433 outcome-only bid-making
  exercises from one specification, and a 30-position composed exam (L1 24/30,
  L2 Partner 26/30 — a diagnostic, never a strength ranking). The six-exercise
  report re-run 2026-09-12 reproduces `walt/gym/RESULTS.md`. Page: [gym](walt-gym.md).

Two consumed-outside-the-courier packets (the launch packet `d8400713` and the
Astra relational-learning proposal `14e01322`) are preserved unchanged and cited
as packets, not as exchange results; neither appears in `exchange/`. Whether
this program runs beside or after the consolidation ruling of reset 13 is not
written down anywhere in the repository; it opened no new mathematical parent
and built no new solver module in `solver/`.

## The working method

The disciplines below are not house style; each was adopted after something went
wrong, and several are enforced in Rust's type system rather than by convention.
They have never been reset. Every later era inherits them verbatim, and the two
newest (the audit as instrument, the background-work wedge rule) are dated.

**Adjudicate before building.** Every probe is written as a design document —
question, construction, receipts, declared stops, and the criterion that would
count as failure — and adjudicated by a walt-math consultant before any code.
Since 2026-09-01 the binding assignment is a brief (`walt/briefs/BRIEF-*.md`)
and the deliverable a report of record (`*-REPORT.md`). The rulings are binding
and are recorded in an append-only file (`walt/CENSUS-RULINGS.md`; the ruling
families are indexed on [math reference](walt-math-reference.md), the latest
being FH-A of 2026-09-04); a superseded ruling is marked, never rewritten
(DS-A28). This has repeatedly paid: proposed comparisons ruled
strawmen (P-Q2), proposed chains rejected in favour of a direct theorem, proposed
receipts rejected as vacuous, five gaps in a proposed invariant list caught
before the build (S5k), and a wrong one-line assertion in a received parent
replaced by a delivered proof (FH-A3).

**Both outcomes are results (F7); NO-RESCUE.** A probe is specified so that
either outcome is reportable before it runs. "A FAIL emits (canonical form,
divergent statistic, both concrete witnesses, exact rationals) and continues to
the next class; no descriptor edit, no re-run with altered invariants" (F7,
2026-08-10). A failure is a counterexample to carry back to the mathematics,
never a thing to fix, spin, or assist past. The S6f gate NO-GO was filed as a
result under exactly this rule; so was the seed survey's non-reportable
association.

**Declare the criterion in advance, and report the verdict, not the texture.**
The clearest instance is the policy-geometry probe, whose formal verdict is
STOPPED with no verdict even though seven of its nine measured pairs collapsed to
a singleton. Its descendant is the six-way result ladder of reset 9, in which
Unresolved is a successful output and a cap is a resource limit, never a proof
rule.

**Caps exclude, they never sample.** A budget cap or fiber cap removes work from
scope and the excluded set travels with every result; a sampled basis is always
marked as sampled and never silently upgraded; a refused coordinate proposes
nothing at all (MB1 gate M4: a wholly refused census returns an empty proposal
list, never a bound). Unmeasured is never zero.

**Grades and labels travel with every verdict.** A verdict is worthless without
the operator pair and weighting it was measured at, because equivalence is
label-relative; a lesson never quotes above its grade; nothing measured at grade
4 is quoted for trick 1 (P-A21); numbers from different epochs (σ0 n0 = 2 versus
n0 = 8; the live seat's 40-world sampler versus the waking seat's) do not
compose, and the record says so at every such site.

**The selection fence.** A carrier chosen by outcome is a carrier, not a sample:
"five coordinates chosen by negative margin are a CARRIER, not a sample"
(FT-A26(iii), restated at SR-A25(iii)). The seed survey (freeze 54) was the
first carrier generated by rule rather than selected by result, and it broke a
screening chain the selected carriers had suggested.

**A cross-check is not a witness.** "Agreement between two computations of the
same quantity does not manufacture the object the pipeline could not build"
(SR-A25); "independent verification means an independent predicate, not an
independent party — two agents running one grep are one check, however many
agents" ([S6 era](walt-s6-era.md)). FH4's NOTE N7 applied the same rule to the
hierarchy: the lower-half parity check is the same call at the same node —
plumbing parity, not independence.

**Determinism is declared, not hoped for.** Numbered freezes fix seeds, orders,
formats and stop criteria (58 issued, 39 and 40 reserved); a freeze clause states
a constant or a generating rule, never both; a record whose freeze-set digest
differs is corrupt, not stale; every stop criterion is a deterministic count
rather than a wall clock; contended timings are recordable but not quotable, with
the bias direction named (DS-A32). Stop and resume must be byte-identical to the
uninterrupted run (§67.5; gated in `solver_opening.rs` and `solver_focal_ladder.rs`).
The register is [the freeze table](walt-math-freezes.md).

**Application outside a verified domain is unconstructible.** A verdict's scope
is its verified domain, gated in types — the same shape as the project's rule
that an external PASS is never imported as an axiom, applied one level down.
The calculated-evidence era made this literal: exposure tiers are mechanically
distinct types, a sampled lower witness is never an upper bound, `Provenance` and
`Decision` have no public constructor, `WakeEvidence` cannot be forged
(compile-fail locks in `solver_waking.rs`).

**Gate, not prose.** Every number a report quotes must name the assertion that
makes it true; "a number in prose with no gate behind it is a NOTE; a number
presented as gated that is not is a BLOCK" (`walt/briefs/BRIEF-FH4-AUDIT.md`).
Gates are sized to their laws, not to a census — one coordinate per law plus a
pinned strictness witness; a corpus sweep belongs in a probe record
(`CLAUDE.md`, 2026-09-04; suites still sized like censuses are tracked at
[[gate-corpus-trim]]). Expensive oracle values a suite needs in several gates
are computed once in a shared fixture (CI1).

**Results files outrank prose.** Where a survey, a ledger paragraph or a wiki
page disagrees with the results file, the results file wins and the
disagreement is recorded, never repaired in place: the S6 era's "the results
file governs" notes (19/3 not 92/15; h2's 8257248/479001600 below h6's), the
shadow README's retraction of the "~108/116 settle by 512" mining proxy, the
doom paragraph's correction in `walt/DISCREPANCIES.md`. A ledger is a dated
running record; corrections travel beside it.

**The audit is an instrument, and independence is what makes it one.** "The
builders cannot audit themselves — the standing audit-independence policy:
never build what you audit; the independence IS the instrument"
(`BRIEF-FH4-AUDIT.md`, 2026-09-04, restating the policy of 2026-09-01). An audit
changes no tier; its BLOCKs are fixed before landing (FH5) and its NOTEs are
fixed or carded. The only fully independent God computation on the walt-fh
branch is a test-local walker (FH6), and the audit said so.

**Probes are validators, never source.** The rescued Python suites are frozen
regression records; walt reimplements from definitions and pins against them, and
a pin detects drift in walt while conferring status on nothing. The verifier
companions that ship with received parents are session evidence at scratch tier,
never receipts (TRUST-01).

**Never end a turn with background work pending.** An agent that yields while a
background job is running is not woken when it finishes; this is the project's
recurring wedge (2026-09-04, FH1: gates and record finished at 02:11, the agent
silent for five hours). Long jobs run in the foreground under the tool's timeout,
split under it, or polled by a foreground loop; for an orchestrator, an idle
notification mentioning running jobs is a stall signal, not a status
(`CLAUDE.md`, commit `8efe6923`).

**The no-default rule.** The old player remains the default until arena and
conformance gates justify a change, on Jason's word (CE-A7/§20.16; restated at
CBS-A9, APS-A9, MB-A7, FH-A10). As of `c00717d1` no arena or conformance run
exists for the controller, waking, unified or partner-aware players, so none of
them carries a strength number and none is the default. (Corrected 2026-09-20:
true through `c00717d1`. On 2026-09-14 the seat Plunge ships became the shared
`walt-player` procedure on conformance receipts alone — level 1, voidless,
fixed selection, an 8/2 reserve, an optional 500 ms partner check — and no
record cites this rule or an arena gate for the change; it still carries no
strength number. The rule stands for the controller, waking, unified and
partner-aware variants, none of which is a default; the Sunshine presets of
2026-09-13 are optional. [walt-seat-play §8A](walt-seat-play.md#8a-the-shared-deployed-player-walt-player-2026-09-14-onward).)

## Where the program stands (2026-09-07, `c00717d1`; addendum as of `afd46420`, 2026-09-20)

The standing epistemic frame, unchanged since S5d: the mathematics has proven
that the object exists; it has not proven its utility. If the utility turns out
bad, that is a conversation rather than a rescue, and the instruments stay
valuable for other explorations either way.

The live question is no longer the economy claim of the decision-sparse era
(that framing is preserved on [decision-sparse](walt-decision-sparse.md) and was
overtaken when the counted-belief era computed the opening root's exact branch
table and Phase 8 returned a certified-regret recommendation there). It is the
frame of reset 12 with the measurement of reset 13 under it: **42 is two
recursions running in opposite directions**, and on this corpus they trade
dominance between trick 4 and trick 5 — from trick 5 in, exact instruments are
effectively free and God-tight receipts certify optimality; trick 4 is minutes
per root; trick 3 is the wall (one root, 289M reads, 14 minutes, 19.4 GB); and
earlier than that the seat plays forward on sampled evidence with a certified
regret it cannot yet close (the opening root: floor 732‰, at most 267‰
unclaimed, the split of the 267‰ UNKNOWN). At k ≥ 1 the remaining width is the
tail's policy gap, so the next money is a better lawful tail and a cheaper σ0
read key, not a deeper search.

What is ruled and what is queued, in order (`walt/MAP.md`): the σ0 read-key
study → the consolidation slice (retire `godgap.rs`, `horizon.rs`,
`extraction.rs`, `refine.rs` as endpoints of the focal-horizon hierarchy) →
**no new mathematical parent until the consolidation lands**. Both items are
carded (`kanban/backlog/sigma0-read-key-study.md`,
`kanban/backlog/consolidation-slice.md`, opened 2026-09-13); neither has a brief
or a commit. Beside that queue, not sequenced against it: the Gran calls
(A)–(F) of 2026-09-05 (more live-epoch match deals; the declarer-side
measurement; the objective and tie-break design at exact indifference; the void
flag default; the shape of the two unmerged branches `walt-o5` and `walt-g1-l2`;
G4 when the real game recurs); the partnership program's open strength question
(profile the partner-field decisions; a coupled-world ablation to isolate void
conditioning); the sampled-table generalization gap (77.8% train vs 25.6% test);
and the standing debts — [[ladder-policy-store]] (memory), [[gate-corpus-trim]]
(gate size), the errata filing for E4.1 and the FT/SR objects (owed since
2026-08-13/14), the corpus check T1-A12 before any rules-level theorem is cited
outside walt, and the FH response to Pro drafted 2026-09-04 and not recorded as
sent.

**Addendum, as of `afd46420` (2026-09-20; cycle 1 of the [curator](curator.md),
curated 2026-09-20).** Between 2026-09-13 and 09-20 the program took no
sixteenth reset: Jason's 2026-09-04 ruling holds, the σ0 read-key study and
the consolidation slice are still uncommitted, and no new mathematical parent
arrived. The movement was on the seat, in three areas, none of them a change
to what 42 *is* for the book. **Sunshine** (2026-09-13 → 09-15;
[walt-partnership-program §11](walt-partnership-program.md#11-sunshine-2026-09-13--09-15-the-partner-aware-live-player)) asked for "a reasonable, fairly quick, partner-aware Texas 42
player on the Mac and phone" and built two optional, bounded second looks
around L1 — a 250 ms partner review valued under the gym teacher's field and
a 500 ms partner rollout valued under the deployed players' own continuations
— then measured them the way the method above demands: on the intended
situations (the review 24/30 → 29/30 on the teacher's exam, of which two of
the six corrections hold and four reverse once deployed L1 plays on; the
rollout 11 improved / 2 harmed development roots and 1 / 1 fresh, the fresh
harm a deadline-truncated prefix that the full census would have reversed)
and on ordinary games, where every panel tied — 32 + 100 + 64 mirrored pairs
and 192 conditional. "No demonstrated strength gain"; neither preset is a
default; the exact gym now names the continuation its answer key assumes,
and the Mac table feeds flagged human-play moves into it ([walt-gym §11](walt-gym.md#11-sunshine-2026-09-13-the-gym-under-deployed-continuations-and-the-live-move-intake)). On
2026-09-15 the notes recorded the workshop harness as a future goal and
turned to the playable game. **The shared deployed player** (2026-09-14;
[walt-seat-play §8A](walt-seat-play.md#8a-the-shared-deployed-player-walt-player-2026-09-14-onward)): the seat Plunge ships and the seat the Mac bridge runs became
one crate, `walt-player` — level 1, voidless, fixed selection with an 8/2
reserve and the optional partner check, and a once-around auction at
threshold 3/4 that its own record calls "deliberately an uncalibrated
bidding policy, not a new strength finding" — under conformance receipts
only. This is where the no-default rule above became stale as a description
of Plunge: the level, the belief and the objective did not move, the
deployed procedure did, and no record cites CE-A7 or an arena gate for it.
**Kiln** (2026-09-18 → 09-19; [walt-kiln](walt-kiln.md)): the model-price
survey of opening bids was frozen after its one calibration check — a
sixes/36 hand the model forecast at 121/160 made 21/100 when the table
actually played it — and replaced, on Jason's direction that "the unit is
the player making bids", by playing the deployed player 305,440 times at
bid 30 over 500 catalogue hands and reading off score tails; Plunge bids
from that book since 2026-09-19 (a lookup: the highest 30–42 target reached
in 4/5 of games), play unchanged. The calibration gap's cause is unmeasured
("a candidate explanation, not a cause established by the experiment",
`CALIBRATION-SIX36.md`). Then, on 2026-09-20, the CPU campaign's **v34**
speedups became the native default (13.13× median on completed fixed solves,
exact-equal on 12/12 paired games) and went to the phone the same day —
portability evidence, "not evidence of improved game strength" — while the
full walt gate remained unrun on every landing since 2026-09-06
([walt-instruments §3.7](walt-instruments.md#37-native-cpu-speedups-v34-and-the-phone-release-2026-09-18--09-20)). *Stopped:* Kiln production and its monitor. *Frozen:* the
scalar survey (never installed); RefineV1 (freeze 58) as before. *Deployed:*
`walt-player` with v34 and the Kiln book, on the phone and the Mac. *Open,
as the chapters name it:* the partnership gap ("The partnership gap is still
open"; whether a bounded partner check should override L1 on sampled
evidence at all; the human partner as a continuation condition no census
models); the calibration gap; whether Plunge's forced last bid of 2026-09-20
belongs to the straight-42 rules profile — a question for Part I; and PR
#90's Nel-O, which would be the first foundation change ([walt](walt.md),
"The unmerged branches"). The queue of the paragraph above is otherwise as
it was.

The summary of results by result lives on [the hub](walt.md) and
[pre-pivot results](walt-pre-pivot-results.md); the refutations at
[negative results](walt-negative-results.md); the dated record at
[timeline](timeline.md); how to run any of it at [instruments](walt-instruments.md).
