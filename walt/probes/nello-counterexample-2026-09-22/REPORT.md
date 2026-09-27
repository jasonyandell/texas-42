# Nel-O counterexample and replan probe

2026-09-22. Exploratory work in `codex/nello-player`, based on
`e4c549e2fd533c942f39125bf3134ac707c4e638`. This report records the offline
panel before live integration. The subsequent opt-in preview adapter is
documented in [NELLO-PLAYER.md](../../NELLO-PLAYER.md#counterexample-research);
the historical measurements below describe frozen completed policies.

**The proposed mechanism works, but this pilot does not establish better play.**
The probe finds public-history-consistent counterexamples, retains them across
rounds, and jointly replans every root action. A pinned tiny case breaks a
five-way 100% tie. Across the two Ruby positions with ordinary budgets 40 and
160, however, fresh-deal results are mixed and extra random samples are
competitive. The extracted plans also have poor coverage of fresh histories.

## A concrete double-lead vulnerability

At Ruby's first examined root (six tiles already played), one sampled remaining
deal consistent with her information is:

| Seat | Remaining tiles |
| --- | --- |
| Ruby, arena 3 | 2-1, 3-1, 4-4, 5-1, 6-6 |
| Declarer, arena 0 | 3-0, 3-2, 4-0, 5-4, 6-3 |
| Other defender, arena 1 | 0-0, 2-2, 5-0, 5-2, 5-3 |
| Inactive, arena 2 | 1-0, 1-1, 3-3, 5-5, 6-0, 6-1, 6-5 |

With **5-1**, the modeled declarer plays **5-4**, the other defender plays
**5-0**, and the declarer wins immediately: set. With **6-6**, the declarer
discards **5-4** and the other defender follows doubles with **2-2**. The
declarer escapes the replayed continuation.

The separate single-world search says **6-6 is doomed for Ruby's defense in
this deal against the fixed modeled field**: even revealing the deal and
searching all subsequent Ruby choices cannot recover a set. 5-1 is not doomed.
This is stronger than one bad continuation, but it is not a claim against all
possible declarer or other-defender strategies. It is also a sampled possible
deal, not the concealed deal from the original game.

Receipt: [ply6-n160-seed1.json](panel-v2/cases/ply6-n160-seed1.json), round 1,
second witness. Internal seats rotate arena seats by +1 modulo four; masks and
complete before-replanning traces are preserved.

## What the instrument does

The [library module](../../walt/src/solver/nello_counterexample.rs) and
[offline binary](../../walt-player/src/bin/nello-counterexample.rs) reuse the
existing Nel-O contract, own-suit doubles algebra, three-active-seat recurrence,
and deterministic modeled L0 field with inner budget 8. Root samples use the
deployed seed stream and mechanically condition on public history/voids. There
is no bidding-conditioned prior or clairvoyant declarer.

1. Draw 40 or 160 ordinary worlds and solve all legal root actions.
2. Extract one policy per action, keyed by observed tile history. Check its
   exact replay value against the solver on every training world.
3. In each of three rounds, inspect a fresh pool of 256 worlds. Rotate which
   actions are targeted and retain up to four new failures, preferring a world
   where another candidate policy succeeds. This is a bounded random search
   for counterexamples, not an exhaustive adversarial search.
4. Replan all candidates together on the ordinary bundle plus every retained
   witness. All entries have unit weight. Never optimize a different focal
   action separately in each hidden world.
5. Train a control with the same number of additional ordinary random worlds.
6. Freeze plans and choices, then draw a separate examination stream of 4,096
   worlds. Reuse it across candidate arms. Examination outcomes never train a
   plan or select a witness.

The deliberate mixture is a **stress score**, not a calibrated probability.
It does not acquire the natural probability mass of the selected failures.
Plans use only their own hand and observed history. An absent history falls
back to the fixed own/public **L0/8 policy**; this is not deployed L1 replanning.
That completion makes every plan executable and frozen, but limits what the
fresh-deal results can tell us about the live player.

## Main panel

Two roots × two ordinary budgets × seeds 1–8 = **32 completed cases**. Each
retained 12 witnesses. Each row below aggregates eight 4,096-world examinations
(32,768 evaluations). Percentages describe completed-policy replay sets.

| Root / ordinary worlds | Baseline | +12 targeted | +12 random | Targeted minus baseline |
| --- | ---: | ---: | ---: | ---: |
| Six played / 40 | 19,830 (60.516%) | 20,111 (61.374%) | 20,218 (61.700%) | +0.858 pp |
| Six played / 160 | 19,483 (59.457%) | 19,469 (59.415%) | 19,572 (59.729%) | −0.043 pp |
| Nine played / 40 | 19,848 (60.571%) | 19,868 (60.632%) | 19,392 (59.180%) | +0.061 pp |
| Nine played / 160 | 20,651 (63.022%) | 20,760 (63.354%) | 20,814 (63.519%) | +0.333 pp |

Pooled descriptive difference: targeted +396 sets / 131,072 evaluations
(+0.302 pp); random +184 (+0.140 pp). Targeted-only sets versus baseline-only
sets were 3,267 versus 2,871. This is not a broad strength estimate: there are
only two game roots, and the 40/160 cases share examination streams for each
root and seed. Those totals are not 131,072 distinct independent deals.

Targeted replanning changed the root lead in **4/32** cases. Five baselines
had tied leading actions; none of those ties changed the selected root action.
No baseline leader in this main panel scored 100%. Restricting selection to
originally tied leaders gave +71 replay sets overall, entirely from changed
continuations, not changed root choices. Thus the main panel has no evidence
of an improved root tie-break.

Of **384 witness retentions** across cases, the targeted action was certified
doomed against the fixed field in **194**. In the other 190, some focal defense
would succeed if the world were revealed. That does **not** imply those worlds
can all be saved by a single information-respecting plan. Retentions are not
necessarily distinct deals across cases.

After targeted replanning, **57.4–77.4% of subsequent non-forced Ruby choices**
on examinations used off-tree L0 completion (67.4% pooled). This is a major
coverage limitation. These replay percentages should not be compared directly
to the earlier 100k optimized in-sample values, nor treated as live Walt rates.

## Pinned 100% tie mechanism test

The separate [saturated demo](cases/saturated-demo.json) deliberately uses one
ordinary world at the six-play root, seed 1, one attack round, and four retained
witnesses. It is a mechanism demonstration, not additional strength evidence.

| Lead | Ordinary bundle set score | After retaining four counterexamples |
| --- | ---: | ---: |
| 2-1 | 1/1 | 3/5 |
| 3-1 | 1/1 | 3/5 |
| 4-4 | 1/1 | 4/5 |
| 5-1 | 1/1 | 5/5 |
| 6-6 | 1/1 | 4/5 |

The tie selects 5-1 after replanning. This preserves the mechanism Jason
suggested: a counterexample stops a tiny sample's 100% from implying universal
success. It does not make the remaining 5/5 a guarantee. The demo's 64-world
examination was 46 sets for the baseline choice versus 44 for the new choice,
so even this tie-breaking demonstration is not a performance win.

## Doom boundary

The existing [salvation math](../../math/salvation_complex_v0.1.md) and
[doom census](../../walt/src/solver/doom.rs) distinguish one policy failure from
a world no lawful focal continuation can save against a declared field. The
new module implements that **singleton fixed-field question for Nel-O** using
the Nel-O solver. It does not port the old four-player counted-class walker.

A certified doomed probability mass could bound success from above. Selected
witness counts here are not that mass, so they provide no numerical doom bound
on the natural distribution. Non-doomed policy failures may reflect policy
gap or conflicts between indistinguishable worlds; this probe does not
decompose those two contributions. No new Lean proof or full census is claimed.

## Cost and validation

Native release on this Apple M5 Max (48 GiB RAM), with at most two panel
processes running concurrently:

| Root / ordinary worlds | Baseline + three hunt/replan rounds | Full case including control and examinations |
| --- | ---: | ---: |
| Six played / 40 | 324–348 ms | 1,797–1,842 ms |
| Six played / 160 | 416–464 ms | 1,875–1,939 ms |
| Nine played / 40 | 114–124 ms | 517–535 ms |
| Nine played / 160 | 138–146 ms | 542–552 ms |

The 32-case panel took 20.13 seconds wall time. These are native research
timings, not phone measurements or compute-matched comparisons with random
augmentation. Every case had a 240-second internal budget and 295-second
process watchdog; all completed normally.

Validation receipts:

- [Three focused module tests](final-module-tests/stdout.log): singleton doom
  versus independent focal recursion on a pinned double trap; extracted-policy
  replay parity and own/public guards; invalid worlds and expired work refused.
- [Two binary tests](final-binary-tests/stdout.log): exact rational parity with
  the existing live wire on both Ruby roots, and the pinned 100% tie regression.
- [Portable module tests](final-portable-tests/stdout.log): same three pass
  without default features.
- [WASM compile check](final-wasm-check/stderr.log): player library compiles for
  wasm32 without parallel defaults; this is not a device runtime test.
- [Existing contract/Nel-O tests](existing-tests/stdout.log): five pass,
  including all 235,872 ordered three-player winner cases.
- Every extracted policy reproduces its exact rational training value before
  it can be used. All retained witnesses replay without off-tree fallback.

## Reproduce and artifacts

From the worktree root, build under the existing process watchdog, then run
the panel into a fresh directory:

```sh
python3 experiments/partnership/packet/texas42-partnership-launch-v0.1/tools/run_capped.py --seconds 295 --output-dir /tmp/nello-probe-build-fresh -- cargo build --locked --offline --release --manifest-path walt/Cargo.toml -p walt-player --bin nello-counterexample
python3 walt/probes/nello-counterexample-2026-09-22/run-panel.py panel-fresh
python3 walt/probes/nello-counterexample-2026-09-22/summarize.py
```

`summarize.py` rebuilds [summary.json](summary.json) from the retained `panel-v2`
cases, without new search. It also verifies all non-instrumentation results
match the initial panel in `cases/`; v2 added counters for non-forced choices.
[panel-run.json](panel-v2/panel-run.json) records source/binary hashes, process
status, and wall time. A test-only addition to the binary source followed that
panel; runtime logic did not change. Initial pilots and the initial compile
error receipt are retained separately rather than rewritten.

**Next useful experiment:** evaluate ordinary Walt replanning at subsequent
choices, with equal compute and fresh game roots, while retaining this witness
bank. The current result supports using counterexamples to expose and diagnose
overconfidence; it does not yet support shipping the mixture as a stronger
selection rule.
