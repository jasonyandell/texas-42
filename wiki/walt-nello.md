# Walt and Nel-O: sampling, escape deals, and counterexample replanning

[Home](Home.md) · owns: the 2026-09-20–25 Nel-O player investigation, Ruby's
double-lead sample ladder, counterexample experiments, and the bounded preview ·
Sources: [implementation record](../walt/NELLO-PLAYER.md),
[sample ladder and raw panels](../walt/probes/nello-ruby-doubles-2026-09-21/LADDER.md),
[counterexample report and held-out panel](../walt/probes/nello-counterexample-2026-09-22/REPORT.md),
[timing panel](../walt/probes/nello-headroom-2026-09-24/REPORT.md).
Related: [seat play](walt-seat-play.md), [counted belief and doom](walt-counted-belief-era.md),
[instruments](walt-instruments.md), [architecture](walt-architecture.md).

> **Tier: EXPLORATORY.** This page records tested mechanics, sampled research,
> and an opt-in playing experiment. It establishes neither optimal Nel-O play
> nor a strength improvement. The straight-42 corpus and Lean results do not
> automatically extend to this contract. Results files outrank the prose.

## What we learned

Ruby's apparently attractive double leads became less attractive as the sample
grew. At both examined positions, every seed at 10,000 and 100,000 worlds chose
5–1 instead. A retained possible deal explains a concrete vulnerability: 6–6
lets a declarer without doubles discard the dangerous 5–4, whereas leading 5–1
forces that tile to win immediately under the modeled responses.

Counterexample search makes such failures visible at a much smaller budget.
It can break a sample's perfect tie, and the preview can retain a few failures
and replan around them. Its held-out experiment was mixed, however: additional
random deals were competitive, and the frozen plans needed substantial fallback
on unseen histories. That supports counterexamples as a diagnostic and a
playtesting hypothesis; it does not establish a stronger player.

## The contract fits the suit algebra

The original [Suit Algebra](../walt/math/SUIT_ALGEBRA_PURE.md) already has a
**doubles-suit declaration**: doubles follow doubles and rank by pip, but have
no trump power against a mixed lead. `Decl::DoublesSuit` implements it. The
Nel-O contract separately supplies three active seats, the inactive partner,
and the objective of avoiding all seven tricks. Any declarer win sets the bid.

There are still four physical hands. The inactive partner's seven tiles remain
hidden and occupy capacity in every sampled completion. Count points are kept
for display, not used to determine success: one retained make has only 37
played points. The declarer maximizes the zero-trick event; defenders minimize
it. Replay, inner models, cache identity and terminal checks share this contract.
Straight-only optimized paths and the nine-declaration auction retain their
scope. Computer Nel-O bidding is deferred.

The [mechanics tests](../walt/walt-player/tests/nello.rs) check all 235,872
ordered three-player winner cases across seat geometries, settlement and hidden
capacity fixtures, and cache separation. The
[native/WASM check](../walt/walt-player/nello-check.mjs) compares 96 fixture
positions, exact option vectors, malformed requests and deadline retention.
These establish implementation behavior, not strategic quality.

## The sample ladder: 40 through 100,000 worlds

Two decisions from the same shared hand were reconstructed from Ruby's own
hand and public history: before her 6–6 lead (six tiles played), and before
her 4–4 lead (nine tiles played). Each budget used seeds 1–8, fixed selection,
the same mechanically conditioned deal model, and eight inner worlds for
modeled players. The original decision receipt and seed were absent from the
shared link, so this is a reanalysis rather than exact reproduction of the
original choice.

| Outer worlds | Before 6–6: selected moves across eight seeds | Before 4–4: selected moves across eight seeds |
|---:|---|---|
| 40 | 3–1 ×1, 4–4 ×1, 5–1 ×1, 6–6 ×5 | 2–1 ×1, 3–1 ×2, 4–4 ×4, 5–1 ×1 |
| 160 | 4–4 ×3, 6–6 ×5 | 3–1 ×4, 4–4 ×2, 5–1 ×2 |
| 640 | 3–1 ×2, 4–4 ×3, 5–1 ×1, 6–6 ×2 | 3–1 ×2, 5–1 ×6 |
| 2,000 | 3–1 ×1, 4–4 ×4, 5–1 ×3 | 5–1 ×8 |
| 10,000 | 5–1 ×8 | 5–1 ×8 |
| 100,000 | 5–1 ×8 | 5–1 ×8 |

At 100,000 worlds, mean modeled set values were 74.3785% for 5–1 versus
70.6365% for 6–6 at the first root, and 68.291625% for 5–1 versus 62.100125%
for 4–4 at the second. All eight seeds favored 5–1, but the values continued
falling from 10,000 to 100,000. Stable choices are not a convergence proof.
The [exact rational summary](../walt/probes/nello-ruby-doubles-2026-09-21/ladder-summary.json)
and [figure](../walt/probes/nello-ruby-doubles-2026-09-21/sampling-ladder.svg)
retain every action, budget and observed seed range.

**Why more samples can lower the scores.** Walt optimizes later focal choices
inside the same sampled information sets used to price them. A small bundle
can fragment into thin branches after observed plays, making its optimized
continuation look too well informed. The downward trend is consistent with
that mechanism; this ladder did not isolate it from ordinary sampling noise or
response-model error. Increasing outer worlds also leaves the eight-world
inner model unchanged. This one hand does not establish that Nel-O is generally
more sample-sensitive than straight 42.

40, 160 and 640 are engineering budgets, not natural combinatorial breakpoints.
The fixed sampler accepts integer counts; the original deployed adapter caps
them at 640. A fourfold ladder is convenient for ordinary fixed-estimator
sampling error, but supplies no confidence guarantee for the optimized plan.
The larger runs used isolated validation-limit patches, never a production
cap increase. The 100,000 run also allowed 240 seconds per decision.

On this Mac, the sixteen 100,000-world calls totaled 556.692 seconds, with
mean decision times of 63.240 seconds at the first root and 6.346 at the second.
Peak observed Node RSS was 1.497 GiB; WASM linear memory peaked at 1.431 GiB.
Fresh processes released memory between cases. These are
[host measurements](../walt/probes/nello-ruby-doubles-2026-09-21/100000/REPORT100000.md),
not phone performance or a runtime bound.

## Why the double objection matters

6–6 cannot catch the declarer on its lead trick. A declarer with a double can
play it safely; one without doubles can discard any mixed tile. Any benefit
must therefore come from later position or information, and needs evidence.
4–4 differs: it can force a higher double to win.

With no publicly revealed voids at these roots, the uniform legal-deal model
gives an exact no-double frequency of `C(12,5)/C(17,5) = 198/1547` before 6–6,
and `C(12,4)/C(15,4) = 33/91` before 4–4. These are combinatorial properties
of that model, not a bidding-conditioned human prior. The
[double-exposure diagnostic](../walt/probes/nello-ruby-doubles-2026-09-21/DOUBLE-EXPOSURE.md)
also counts the higher-double trap and explains the inactive hand.

The revealed original deal is a separate question: full-information analysis
found a forced set after every Ruby lead at both positions. It cannot justify
which lead Ruby should choose with hidden hands. The counterexample described
above is a *possible* hidden allocation, not the original concealed deal.

## Find failures, retain them, and jointly replan

The [instrument](../walt/walt/src/solver/nello_counterexample.rs) extracts a
policy for each root action, keyed by observed tile history, and verifies its
exact training replay value. It samples fresh candidate deals consistent with
the same public information, retains failures (preferring ones another action
can handle), and replans every root action over one common augmented bundle.
One focal policy must work across its indistinguishable worlds; there is no
independent per-world maximization at a hidden focal decision.

The offline experiment used three pools of 256 candidates, retaining up to four
new witnesses per round. Ordinary worlds and retained witnesses have unit
weight. This deliberately selected mixture is a **stress score**, separate
from the ordinary estimate. Hunting more candidates does not assign their
natural probability mass to the handful retained.

In a pinned one-world demonstration, all five leads scored 1/1. After four
witnesses, 5–1 scored 5/5, the doubles 4/5, and 2–1 and 3–1 scored 3/5.
The tie broke, but 5/5 was still no guarantee: the separate 64-world examination
gave 46 baseline sets versus 44 after refinement. The demonstration establishes
the mechanism and preserves its unfavorable examination outcome.

The main panel had two roots × two ordinary budgets (40/160) × eight seeds.
Each frozen policy was examined on 4,096 fresh worlds, with a +12-random-world
control. Targeted refinement changed four of 32 root choices; none of the five
initial leading ties changed choice. Its pooled descriptive gain was 396 sets
over 131,072 evaluations (+0.302 percentage points), versus 184 for random
augmentation. These are repeated evaluations of two roots with shared streams,
not 131,072 independent games. At six-play/160 the targeted arm regressed;
random augmentation won three of the four group comparisons.

The largest limitation was continuation coverage: 67.4% of subsequent
non-forced focal choices used the frozen own/public L0 completion off the
extracted tree. The live player instead replans L1 at later decisions. Thus
the held-out rates neither validate the live preview nor directly compare
with the 100,000-world optimized in-sample scores. The
[report](../walt/probes/nello-counterexample-2026-09-22/REPORT.md) and
[recomputed summary](../walt/probes/nello-counterexample-2026-09-22/summary.json)
own the group results, paired counts and full traces.

## What doom contributes, and what it does not

The repository's [doom census](../walt/walt/src/solver/doom.rs) and
[salvation mathematics](../walt/math/salvation_complex_v0.1.md) already ask
whether every focal continuation fails against a specified field. The Nel-O
probe implements the singleton version: reveal one candidate world, search
all later focal choices, and keep the other modeled policies fixed.

That separates a failed extracted plan from a world that even a world-aware
focal defender cannot save against this field. Of 384 retained witnesses in
the offline panel, 194 were doomed for their targeted action; 190 admitted
some world-aware rescue. The latter need not admit one lawful shared rescue
policy across worlds. Neither label quantifies over every possible declarer
or defending-partner strategy.

The live preview hunts **policy failures**; it does not run the singleton doom
classification or the old counted-class census. A verified doomed *mass* could
supply an upper bound, but selected witness counts are not that mass. No doom
probability bound or new Lean proof follows from this experiment. The earlier
counted-doom scope correction remains on
[the counted-belief page](walt-counted-belief-era.md).

## The preview being prepared for merge

As of 2026-09-25, Texas 42 provides an explicit
`nello_counterexamples:true` opt-in on the shared player call. It applies only
to Nel-O defenders. Plunge exposes the whole feature as one default-off
**Advanced → Nel-O · Preview** setting: declarations and defender refinement
are enabled together. Computer auctions remain straight-only.

| Allocation | Current preview |
|---|---|
| Ordinary Ask Walt / ordinary later play | 40 worlds, eight inner worlds |
| Browser explicit Think deeper | 350 worlds; first retains a completed 40-world comparison |
| Counterexample pools | 540 candidates per round, up to three rounds |
| Retained failures | At most four per round / twelve total, unit weight |
| Reserved counterexample time | Up to 4.2 seconds within the existing 14/20-second total; at most half a short caller budget |

Each completed joint replan emits a checkpoint. A partial round never replaces
the last complete move or vector. Ordinary `evaluation` remains intact and
`counterexample_result` reports the separate mixture. Fresh Ask Walt and
How it went analysis use the same defender path; saved original evidence keeps
its original settings. Declarer and straight analysis use ordinary sampling.

The first preview had 256 candidates and a two-second remainder cap. A
headroom trial raised that to 768 candidates, a six-second reservation and
500 deeper worlds. The [five-position Node panel](../walt/probes/nello-headroom-2026-09-24/REPORT.md)
completed all work at roughly 3.1 times the old aggregate deeper time.
Browser playtesting then led to the current roughly 30% reduction. On the
same Chrome replay spot check, 350 worlds plus all twelve witnesses completed
in 5.20 seconds versus 7.20 seconds for the prior configuration. This is a
single-host observation, not a general speed ratio or physical-phone benchmark.

The shared source is staged as [mechanics PR #90](https://github.com/jasonyandell/texas-42/pull/90),
then [counterexamples and research PR #96](https://github.com/jasonyandell/texas-42/pull/96).
[Plunge PR #4](https://github.com/jasonyandell/plunge/pull/4) owns the app flag,
WASM import, receipts UI and isolated preview deployment. Merging shared source
does not by itself publish a new Plunge artifact or change its production flag.

## Next experiment and reproduction

The next discriminating test is a fresh-root, matched-deal comparison with
equal total compute and **live L1 replanning at subsequent turns**. Compare
ordinary sampling, additional random sampling and retained counterexamples;
report actual sets, reversals, timing, fallback frequency and witness coverage.
Separately vary inner-model budget and bidding-conditioned beliefs. Keep
examination worlds out of discovery and avoid tuning solely on Ruby's two roots.

The [archive README](../walt/probes/nello-ruby-doubles-2026-09-21/README.md)
explains the retained raw panels, source hashes and research-only patches.
`compare-panels.py` and the counterexample probe's `summarize.py` regenerate
both summaries byte for byte without new search (verified 2026-09-25).
The [merge-readiness record](../walt/NELLO-MERGE-READINESS.md) owns current
validation and integration order; the historical reports retain their original
parameters and conclusions.
