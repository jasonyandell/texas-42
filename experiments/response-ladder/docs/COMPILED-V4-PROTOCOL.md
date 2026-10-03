# Larger teacher bundles, fixed representation

This is a named DEVELOPMENT control prepared after the v3 CPU/GPU panels. It
is prepared only; no v4 teaching, fitting, or H2H run has started.

The physical calibration corpus, request IDs, seeds, fixture groups, and
repeat-0 train/repeat-1 development split are copied byte-for-byte from
`results/compiled-calibration-v3`. The immutable inputs are present in both
`results/compiled-calibration-v4` and
`results/compiled-t1-calibration-v4`. Each directory contains only its
`manifest.json`, `requests.jsonl`, and `prepared-from.json` until teaching
begins. The manifest hash is
`168c3c1c82b53cef64ab3c09e33b92be655ac7806dfb87b0cb569b039912215a`; the
request hash is
`16bf03367b7b04e73cee2b1a07801d6887879c6f7ed3a88008b5434d88b14d2d`.

Use the sixteen-clause representation with separate declaring and defending
actors, at most three distinct clauses, and exact train-only selection. T0
uses `n=32` against the same Dice field. Freeze the resulting role-aware C0-v4
artifact before generating T1. T1 uses `n=8` against that frozen C0-v4 at all
modeled seats, then fit the role-aware C1-v4 artifact. Both stages use the
50ms request cap and preserve every complete vector, settled result, refusal,
and error. Refusals remain data and failed attempts are not silently retried.

The teaching sequence is:

1. Teach `results/compiled-calibration-v4` with `n=32`, `budget_ms=50`, and
the frozen compiled teacher; fit role-aware C0-v4 from train rows only.
2. Freeze C0-v4 and teach `results/compiled-t1-calibration-v4` with `n=8`,
`budget_ms=50`, and C0-v4 as the modeled field; fit role-aware C1-v4 from
train rows only.
3. Before any H2H, evaluate v3 and v4 actors on identical completed v4 rows,
separately by declaring/defending role, preserving physical-group weights,
optimal-set hits, canonical tie differences, and censored counts. These
local target comparisons do not claim whole-game strength or make raw cost
comparisons across different teacher bundles.

Each teaching cycle is bounded at 55 seconds under the existing 60-second
process-group watchdog. Resume only cooperative budget stops with the exact
same command and preserve all receipts and attempts. There is no fresh final
holdout; all rows remain CALIBRATION.

After the audit, if the v4 artifacts differ from v3, run the unchanged paired
development panels in namespace `compiled-v1-dev001` with 72 mirrored pairs,
six CPU threads, `outer=40`, `plans=1`, `horizon=7`, `work=2,000,000`,
`compiled_tail=true`, and 20ms candidate move budgets. Run CPU and GPU panels
sequentially, retaining complete games, failures, fallbacks, independent
replays, response reports, compute statistics, and all cycle receipts. Do not
stop on outcomes. If both role artifacts are byte-identical to v3, record the
null result and omit redundant H2H.

The current Walt anchor and all source/binary identities remain frozen. Offline
teaching work is not an online speed claim, and local teacher costs are not
whole-game regret.
