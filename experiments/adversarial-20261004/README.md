# Reproduce the constructive Walt checkpoint

Run from `/Users/jason/Documents/Codex/2026-10-04/task-2/research-worktree`.
Read [REPORT.md](REPORT.md) first. This directory owns this team's canonical
review; first-team originals under `experiments/astra-sol-20261004` are frozen.
`SHA256SUMS` pins this team's final source, reports and receipts, excluding
itself and ignored build/cache products. The Git commit pins the manifest.
Use a **fresh output directory** for each command. The runner rejects limits
over 295 seconds and kills the process group, including children.

No installation is required on this machine. NumPy commands use the existing
environment `/Users/jason/code/mk5-main/.venv/bin/python`; plain Python commands
are standard-library-only. Cargo uses existing offline dependencies. Lean is
core/Std, without a mathlib build. Native targets are ignored build products.

```sh
# Production artifact identity and source fingerprint.
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 30 --output-dir /tmp/walt-review-identity-NEW -- python3 experiments/astra-sol-20261004/tools/verify_identity.py

# Original 780-vector panel, with compilation and unused-coordinate counts.
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 290 --output-dir /tmp/walt-review-original-NEW -- python3 experiments/astra-sol-20261004/phase3/run_panel.py --out /tmp/walt-review-original-panel-NEW --limit 18 --worlds 40 --tapes 4

# Strongest new result: full-lawful-tree frontier, then fresh-deal scaling.
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 290 --output-dir /tmp/walt-frontier-log-NEW -- /Users/jason/code/mk5-main/.venv/bin/python experiments/adversarial-20261004/frontier.py --out /tmp/walt-frontier-NEW
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 290 --output-dir /tmp/walt-frontier-holdout-log-NEW -- /Users/jason/code/mk5-main/.venv/bin/python experiments/adversarial-20261004/frontier_holdout.py --out /tmp/walt-frontier-holdout-NEW

# Shared-ordering gap, then vectorized and public-bit refinements.
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 290 --output-dir /tmp/walt-orders-log-NEW -- python3 experiments/adversarial-20261004/shared_orders.py --out /tmp/walt-orders-NEW
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 290 --output-dir /tmp/walt-vector-log-NEW -- /Users/jason/code/mk5-main/.venv/bin/python experiments/adversarial-20261004/vector_orders.py --out /tmp/walt-vector-NEW
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 290 --output-dir /tmp/walt-refined-log-NEW -- /Users/jason/code/mk5-main/.venv/bin/python experiments/adversarial-20261004/refined_orders.py --out /tmp/walt-refined-NEW

# Concrete sampler counterexample and conservative repair.
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 30 --output-dir /tmp/walt-bad-support-NEW -- /Users/jason/code/mk5-main/.venv/bin/python experiments/adversarial-20261004/sol/verify_bidder_fixture.py
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 30 --output-dir /tmp/walt-repaired-support-NEW -- /Users/jason/code/mk5-main/.venv/bin/python experiments/adversarial-20261004/sol/repair_audit.py
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 30 --output-dir /tmp/walt-incoming-v2-NEW -- /Users/jason/code/mk5-main/.venv/bin/python experiments/adversarial-20261004/sol/final_packet_audit.py

# New small Lean lemmas (original six commands are in sol/receipts/*/run.json).
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 30 --output-dir /tmp/walt-priority-proof-NEW -- lean +leanprover/lean4:v4.33.0-rc1 experiments/adversarial-20261004/sol/PriorityRealization.lean
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 30 --output-dir /tmp/walt-pruning-proof-NEW -- lean +leanprover/lean4:v4.33.0-rc1 experiments/adversarial-20261004/sol/SettledPruning.lean
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 30 --output-dir /tmp/walt-nesting-proof-NEW -- lean +leanprover/lean4:v4.33.0-rc1 experiments/adversarial-20261004/sol/NestedPlans.lean

# Native demand census; this is the sampled core, not the phone wrapper.
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/walt-demand-build-NEW -- cargo +1.95.0 build --offline --release --manifest-path experiments/adversarial-20261004/native/Cargo.toml
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 290 --output-dir /tmp/walt-demand-log-NEW -- python3 experiments/adversarial-20261004/demand_panel.py --out /tmp/walt-demand-NEW --start 941000 --count 32 --min-ply 12

# Exact phone review-off holdout (four deals × four rotations × two teams).
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 290 --output-dir /tmp/walt-phone-log-NEW -- python3 experiments/astra-sol-20261004/tools/run_panel.py --output /tmp/walt-phone-NEW --seeds 910400 910401 910402 910403
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 30 --output-dir /tmp/walt-phone-summary-NEW -- python3 experiments/astra-sol-20261004/tools/arena.py summarize /tmp/walt-phone-NEW
```

`parallel_roots.py` can regenerate the initial new-root panel (`--count 256
--rounds 5 --min-ply 12 --ply-span 4`). Other scripts default to its retained
plan for reproducible paired comparisons. The holdout regenerates independent
deals and all sampled worlds itself. Every historical result has its own
`run.json`, stdout and stderr. Null-result searches and setup errors are
retained, labeled and excluded from positive claims.

Retained unsuccessful invocations: the first two native demand builds used an
incorrect relative dependency path; the first sampler attempt used Python
without NumPy; the first frontier stress harness accidentally duplicated its
output IDs. Each was corrected and rerun. The two nofusion searches deliberately
exit 1 when they find no counterexample; both are null results, not exceptions.
The first final-packet audit had a missing local import path, fixed before its
successful rerun. Raw downloaded files were never edited to resolve imports.

The versioned engineering packets are under `incoming/drive-0815/` and
`incoming/drive-final/`, with raw bytes,
Drive IDs/timestamps/URLs and SHA256 values in `PROVENANCE.json`. The earlier
nine files remain separately under the first-team snapshot. Do not replace
one version silently with another. The complete Claude conversation and the
reported `l2.py` caller were not in the preserved packet.

The final frontier source adds explicit rejection of duplicate output root IDs
and unsupported bid/declaration inputs. These guards do not alter any eligible
panel value; `results/frontier-final/` records a fresh parity run after them.

To check frozen source identity, saved phone move legality, versioned downloads
and watchdog receipts without rerunning the solvers:

```sh
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 30 --output-dir /tmp/walt-checkpoint-audit-NEW -- python3 experiments/adversarial-20261004/verify_checkpoint.py
```
