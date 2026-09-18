# Kiln

Kiln produces a versioned opening bidding book for Plunge using the same lawful
L1/8-inner-world, voidless evaluator as the deployed player. This is exploratory
model pricing, not calibrated odds or a perfect-information oracle.

## Active objective and completion ledger

- [ ] 1,000 complete deals; all four seats, nine declarations, targets 30–42.
- [ ] Progressive 8 → 40 → 160 outer samples; every stop recorded honestly.
- [ ] Measured throughput tuning on the M5 Max, then long observable production.
- [x] Interrupt/restart and abrupt-death recovery verified; atomic durable results.
- [x] Calibration: at least 100 fresh hidden-hand completions of selected bids,
      including a sixes/36 example near 0.78 if the catalogue produces one.
      Compare original estimates with executed player outcomes and uncertainty.
      Completed: 21/100 versus 121/160 for one sixes/36 hand; see
      [CALIBRATION-SIX36.md](CALIBRATION-SIX36.md).
- [ ] Versioned complete book independently validated against its receipts.
- [ ] Integrate catalogue deals and instant bidding into Plunge, preserving saves,
      receipts, questions, regular auction cadence, and existing lawful play.
- [ ] Deploy and verify the actual production site with browser automation.

The larger [Sunshine goals](../partnership/SUNSHINE-NOTES.md) remain active:
a fairly quick, lawful, partner-aware player; guesses checked by actual
continuations; original evidence and human questions feeding the Scheme gym;
the future Sunshine workshop for beliefs, dynamics, and visual exploration.
Kiln removes bidding latency; it does not close the partnership gap.

## Commands

Run from the repository root:

```sh
cargo build --release --locked --manifest-path walt/Cargo.toml -p walt-player --bin kiln-worker
python3 experiments/kiln/kiln.py init /Users/jason/data/texas-42/kiln-v1
python3 experiments/kiln/kiln.py run /Users/jason/data/texas-42/kiln-v1 --workers 12 --threads 1 --seconds 60
python3 experiments/kiln/kiln.py status /Users/jason/data/texas-42/kiln-v1
python3 experiments/kiln/kiln.py export /Users/jason/data/texas-42/kiln-v1 /Users/jason/data/texas-42/kiln-v1/book.json
```

`--seconds 0` runs until work finishes or interruption. Ctrl-C/SIGTERM stops active
workers and returns unfinished jobs to the queue. An exclusive OS lock prevents
two coordinators. After abrupt termination a new lock owner recovers leases.
The database uses WAL, FULL synchronous writes, macOS fullfsync, and one
transaction per completed price plus its refinement decision. At most unfinished
worker jobs are lost; no whole panel is discarded. `status.json` and run logs are
views, not authority. The database and WAL must stay together while running.

A persistent native process per worker accepts only own-hand auction requests.
`RAYON_NUM_THREADS` controls inner concurrency independently of process count.
Start with one inner thread and tune completed prices/second, RAM and errors;
CPU utilization alone is not a throughput result. Every native result includes
node counts, inner evaluations, cache counts, elapsed time, and exact fractions.
The coordinator externally bounds each job even if the solver's deadline fails.

## Sampling and screening contract

Seeds 420600–421599 are selected before evaluation. Numeric seed → first Plunge
hand uses its existing Mulberry32 shuffle and initial-shaker draw. Every deal is
retained regardless of estimated quality. Evaluation seeds depend on the own
hand, seat and fixed profile, never the actual unseen opposing hands. All
prices explicitly condition on that own hand under Walt's uniform opening model,
not the smaller distribution induced by limiting play to a finite catalogue.

The initial run prioritized coverage. `--order coverage` retains that scheduling;
`--order depth` prioritizes pending refinements; `--order deal` finishes deals
in seed order, deeper stages first within a deal. These change scheduling only.
Eight-world prices at most 1/8 stop provisionally; prices
at 40 worlds below 1/2 stop provisionally. Other entries advance to 160. A
preselected, deterministic 2% of *all cells* bypass screening and reach 160 for
missed-opportunity audits. Screening is heuristic and can miss viable bids;
0/8 is not impossibility. No score is inferred for another target, and none is
imputed from neighboring bids. Sampled optimized policies can change as the
bundle grows; independently optimized scores are never averaged as if they were
one larger solve. The initial generator retains no cross-job solver cache.

The shipping book contains only deals with all 468 entries present. Each cell
retains its actual sample count and disposition; own-hand sampling seeds are
included separately as decimal strings; a covered deal can be exported
while refinements remain pending. Final completion additionally requires every
retained cell to reach its planned depth and no failed/pending jobs remaining.

## Validation and continuation

`python3 -m unittest discover -s experiments/kiln -v` checks all 1,000 generator
seeds against Plunge's actual RNG, native prices against the existing auction
endpoint, and queue identity/transaction/refinement/export invariants. The test's
Plunge checkout currently defaults to `/Users/jason/code/plunge-sunshine`.

See `PROGRESS.md` for last verified runs and immediate next actions. Raw data,
SQLite, logs, and full books live outside Git under `/Users/jason/data/texas-42/`.
Check the live process before treating any old status snapshot as running.

## Release audit

The read-only auditor pins a consistent SQLite snapshot, checks every receipt
against its own-hand identity, samples, counters and exact fraction, checks every
refinement chain and screening decision, verifies immutable binary/source bundles,
and reports the preselected screening audit. With a book, it also checks all book
cells against original saved receipts. A partial audit never certifies completion.

```sh
python3 experiments/kiln/audit.py /Users/jason/data/texas-42/kiln-v1 /Users/jason/data/texas-42/kiln-v1/audit-partial.json --partial
python3 experiments/kiln/audit.py /Users/jason/data/texas-42/kiln-v1 /Users/jason/data/texas-42/kiln-v1/audit-final.json --book /Users/jason/data/texas-42/kiln-v1/book.json
```

The second command is a release gate: all 1,000 deals and every scheduled
refinement must be done. Run it before installing the book in Plunge. Original
model calibration remains separate from structural verification.
