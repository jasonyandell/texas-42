# From gym action grades to deployed partnership play

The gym's original answer asks how well a first move can do with an optimal
lawful focal continuation and a specified partner/opponent model. This replay
instrument asks what happens when the deployed players actually continue.
It targets competence on selected partnership situations, including rare
mistakes that need not noticeably move ordinary-game win rates.

The [completed results](campaigns/sunshine-gym-replay-v1/RESULTS.md) include
per-root values and concrete paired win/loss examples.

The [first fixed protocol](campaigns/sunshine-gym-replay-v1/PROTOCOL.md) covers
the six missed composed exercises, the already-correct certain case, and 38
strict withholding controls. The portable
[panel](campaigns/sunshine-gym-replay-v1/panel.json) retains the exact own/public
requests, source coordinates, original teacher grades and outcomes, and every
mechanically compatible remaining deal. The independent rules verifier checks
support equality against the old teacher traces when preparing the panel.

## Three comparisons on the same hidden hands

- **Fixed offer versus baseline:** force the original teacher-preferred offer
  or baseline first move, then use ordinary L1 at every seat. This asks whether
  the proposed repair helps under actual L1 continuation, even if the reviewer
  fails to select it within its budget.
- **Reviewed first move versus baseline:** take the deployed reviewer's actual
  first choice, then use ordinary L1 at every seat. This isolates the first
  decision. Every legal action is replayed, so its rank can also be inspected.
- **Complete reviewed partnership versus baseline:** use the reviewer's first
  choice and reviewed players on both declaring seats afterward, against L1
  opponents. This tests the assembled procedure, including later reviews.

Only the examiner forces test actions. Every player receives exactly its own
original seven-tile hand, public plays, declaration, bid, bidder, seat and seed.
It receives no teacher answer or other actual hands. Decisions are cached by
the exact request and complete player configuration using the earlier
conditional runner's cache. Thus all worlds sharing information get the same
frozen realized decision, including any timeout or fallback.

## Run and resume

The existing release `partnership` and `partner_review` workers are used without
changes. From the worktree root, use a fresh external output directory:

```sh
python3 experiments/partnership/packet/texas42-partnership-launch-v0.1/tools/run_capped.py --seconds 295 --output-dir /tmp/gym-replay-slice-1 -- python3 experiments/partnership/gym_replay.py run experiments/partnership/campaigns/sunshine-gym-replay-v1/panel.json /tmp/gym-replay-results
```

Repeat with the same panel/result directory and a new watchdog output directory
to resume. Completed trajectories and individual decisions remain unchanged.
SIGINT stops scheduling and drains active trajectories. Exit 75 means an
incomplete but resumable run. A hard stop can lose active trajectories; already
saved decisions survive. Changed code, binaries, panel, or player configuration
refuses silent resume. Defaults allow ten continuations and two native threads
per worker; `--workers` can reduce concurrency.

`status.json` reports saved jobs, active job IDs, and failures. Failed jobs retry
on the next invocation; old failure files are historical, so consult completed
items and current status as well. The job ID identifies root, world, and either
the forced action or the deployed reviewed arm.

The report command performs the full audit without calling a player:

```sh
python3 experiments/partnership/gym_replay.py report experiments/partnership/campaigns/sunshine-gym-replay-v1/panel.json /tmp/gym-replay-results
```

It refuses a partial report and verifies every trajectory, exact world, request,
player configuration, cached response hash, root intervention, score and make
flag. `report.json` contains per-root values, paired gains/losses/ties, separate
group summaries, witnesses, and hashes for all used raw artifacts.

## Interpretation

Makes are exact counts over the full selected mechanical support for the frozen
procedures. This is not a claim about all possible partner behaviors or a
posterior weighted by their earlier play. No sampling confidence interval is
needed for enumerating these fixed supports; population generalization remains
separate. Equal-root means and pooled world counts answer different questions.

Keep gains and losses as well as net differences. An offer with higher make
probability can hurt in some individual hidden hands. A negative control was
negative under the original teacher; replay can discover that the deployed
players reverse its ranking. Report that reversal rather than treating the old
label as an immutable fact.

Cached replay speed and unique-call timing are operational diagnostics. They do
not estimate live arena latency or average ordinary-game strength. Root calls
retain their actual elapsed time and the review's complete/incomplete status.
