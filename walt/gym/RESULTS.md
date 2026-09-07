# First partnership gym: count offers

Follow-up: [Scheme-driven discovery](DISCOVERY.md) expands the collection to
170 distinct strict coordinates. The six-exercise pupil results below remain
the original starter measurement.

2026-09-06, `codex/partnership-launch`. Bid 30 throughout. Exactness here is
relative to the declared uniform mechanical belief and fixed seat-local field;
it is not perfect-information optimality or a general player-strength result.
The [guide](README.md) defines the exam and gives runnable commands.

## What was built and measured

The first 60 public-pattern candidates from the existing default-player
battery produced 60 complete, independently audited keys, with no failures or
world-cap skips. The final capped mining run took **1.305 seconds** with ten
workers. These deliberately small trick-5/6 coordinates contained 3,273 legal
worlds in total across coordinates and generated **9,554 complete action/world
replays**. These counts are work units, not independent statistical samples.

There were 21 coordinates with a strict count-offer advantage, 13 with a strict
disadvantage, and 26 with no qualifying strict pair. The
[mining manifest and all 60 summaries](mining.json) preserve candidates,
source hashes, field settings, action masses, and audit counts. Nonselected
full traces remain in the local directory recorded there. Published fixtures
are self-contained and include their full traces.

The gallery takes the first three stable-coordinate IDs in each category,
before pupil evaluation. The six exercise keys cover 308 worlds across
coordinates. Pupil choices never influenced selection.

| Exercise | Goal | Worlds | Offered count | Preferred play | Compared play | Preferred success | Compared success |
|---|---|---:|---|---|---|---:|---:|
| advantage-01 | set 30 | 36 | 5–0 | 5–0 | 5–3 | 30/36 | 8/36 |
| advantage-02 | make 30 | 12 | 5–5 | 5–5 | 6–6 | 12/12 | 10/12 |
| advantage-03 | make 30 | 150 | 5–5 | 5–5 | 5–4 | 92/150 | 83/150 |
| disadvantage-01 | set 30 | 36 | 5–0 | 3–3 | 5–0 | 3/36 | 1/36 |
| disadvantage-02 | make 30 | 50 | 3–2 | 1–1 | 3–2 | 35/50 | 28/50 |
| disadvantage-03 | make 30 | 24 | 5–5 | 3–0 | 5–5 | 24/24 | 17/24 |

These are paired examples, not universal rules. Every other legal action and
the complete optimal set remain in each key.

## First pupil results

All three named native players received the same six own-hand/public-history
requests with seed 420600. Ten workers, one native thread per worker. All 18
decisions completed without fallback in **0.201 seconds** of capped batch
time. These tiny late-game exercises do not estimate full-game decision cost.

| Player | Optimal root choices | Mean root regret | Mean decision time |
|---|---:|---:|---:|
| L1 default | 5/6 | 1/216 ≈ 0.463 percentage points | 0.028 s |
| L2 Partner default | 6/6 | 0 | 0.031 s |
| L2 Partner with voids | 5/6 | 1/216 ≈ 0.463 percentage points | 0.029 s |

The only differing grade is `disadvantage-01`. L1 and L2 with voids choose
2–0, worth 2/36 success; default L2 chooses 3–3, worth 3/36. Both alternatives
to offering count are better than 5–0, which is worth 1/36. The missed value
is therefore **1/36 on that exercise**, not the 2/36 gap of its displayed
offer/hold pair. Every other pupil choice is in the complete optimal set.

This is a functioning diagnostic score, not evidence that L2 generally beats
L1 or that voids hurt. The teacher uses one specified future teammate policy
and seed schedule; pupils approximate different models. Six outcome-selected
coordinates cannot settle general strength or the broader partnership gap.

[The saved run](benchmarks/starter-v1/manifest.json) pins pupil settings,
runner dependencies, binary, and catalog. All 18 response/grade records are
checked in. A second invocation resumed this run without changing any of the
18 item hashes. The report also rejects results from a different catalog or
a saved grade inconsistent with the chosen action.

## Two inspectable same-world witnesses

`show advantage-01`: twos are trump. Seat 3 defends, partner seat 1 has led
5–5, and seat 2 has played 1–1. We can follow with 5–0 or 5–3. In the displayed
compatible world, offering 5–0 lets partner capture 15 count; the declaring
team ends at 25. Playing 5–3 captures 10 count for partner and the declaring
team reaches 30. Across the whole legal belief, the preferred offer raises
set probability from 8/36 to 30/36.

`show disadvantage-01`: sixes are trump. Seat 2 defends and plays last, with
2–0, 3–3, and 5–0. Partner seat 0 is already winning with 5–4 over the led
4–3. In the displayed compatible world, withholding count with 3–3 leads
partner to lead 5–5 next; our saved 5–0 follows into that winning trick.
Partner captures 15 count over the suffix and the declaring team ends at 19.
Offering 5–0 immediately instead makes partner lead 4–4 next, which is trumped;
the declaring team reaches 30. Thus “partner is winning, give count” has a
concrete counterexample. These suffixes explain specimens; their exact
root values average all legal hidden worlds under the declared field.

## Verification and repair record

- **61 distinct focused Rust tests passed:** five new gym gates, 19 Scheme
  gates, six extraction gates, 11 factor-belief gates, five factor-recursion
  gates, eight model-belief gates, and seven model-belief recursion gates.
- **Nine Python gym tests passed:** independently enumerated support and full
  replay, scoring/ties, hidden-input rejection, malformed keys, native-key
  reproduction, manifest/catalog identity, failure/retry, interruption/resume,
  and duplicate-coordinator protection.
- Focused clippy with warnings denied passed. Formatting and diff checks
  passed. Native production binaries built successfully. Full repository CI
  was deliberately not run under the existing session waiver.

The first pilot exposed the previously documented zero-completion boundary:
single-field conditioning could classify an impossible hand while constructing
an action-likelihood factor. It now prunes zero-completion hands first, using
the same mathematical guard already applied by model-belief. Retained entries
keep their original factor weights. A dedicated non-unit-weight test prevents
accidentally substituting marginal weights and double-counting completions.

Two older tests deliberately expected the raw Level1 evaluator to refuse four
of six receipt roots. They initially failed after the repair because it now
completed. They were strengthened to require exact single-field/mixture parity
on all six roots. The independent impossible-frame witness remains in the
suite, so the change neither hides invalid states nor supplies a fallback
answer. The new 36-world third-hand gym regression exercises this boundary.

Run records are under `experiments/partnership/runs/`: `gym-focused-rust`,
`gym-weighted-regression`, `gym-final-python`, `gym-clippy`,
`gym-final-mining`, `gym-final-select`, `gym-final-pupils`, and `gym-resume`.
Earlier pilot/failure logs are retained separately in the same directory.
