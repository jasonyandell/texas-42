# exp3a probe suite (exploratory tier)

The v0.1 / v0.2 / v0.31 lambda-probe chain, preserved verbatim from the
2026-08-09 session scratchpad — the same rescue the exp5 suite got (commit
b3cb523), done before /tmp cleanup could destroy the only copy.

**Why this matters:** `lambda_probe_v3.py` Part 1 is Experiment 3A — the
counterexample-guided descriptor synthesis whose winning four-atom
descriptor v0.4 §14.4 reports as `{comp41, s3max2, team(2:0), team(4:2)}`
(90 worlds → 33 cells → 8 responses). The atom **semantics** live only in
this file's registry (`comp41` at line ~268, `s3max2` at ~277, the full
22-observable vocabulary around them). S4's blocked pin
(`walt-skeleton/tests/harness.rs::exp3a_static_descriptor_pin`, `#[ignore]`d;
`walt/DISCREPANCIES.md` "exp3A descriptor pin") was blocked precisely
because these definitions were thought lost. They are not lost. The pin can
now be unblocked by porting the registry semantics and reproducing
90 → 33 → 8 against walt's checkers.

Contents:

- `lambda_probe.py` — v0.1 probe (§14 of the v0.1 doc). Carries the
  2026-08-09 `_combine` BUGFIX (the dropped-interval no-cross case); see the
  report's disclosure section.
- `lambda_probe_v2.py` — v0.2 probe (imports v1).
- `lambda_probe_v3.py` — v0.31 probe (imports v1 and v2). Part 0 = table
  verification; **Part 1 = Experiment 3A** (the atom registry and exhaustive
  subset search); Part 2 = Experiment 3B (horizon-3 breakpoint hunt,
  trick-5 fiber 1680).
- `v1_output_postfix.txt`, `v2_output_postfix.txt`, `v3_output_postfix.txt`
  — outputs of the post-bugfix re-runs (v1/v2 bit-identical to pre-fix; v3's
  spurious root switch at 1/5 gone).
- `v3_diag.py`, `v3_diag_postfix.txt` — the independent scalar validators
  that exposed the `_combine` bug, and their all-pass post-fix output.
- `lambda-probe-report.md` — the session report covering all three probes,
  including the bugfix disclosure and commentary.

Everything here is **exploratory probe tier**: stdlib-only Python, exact
Fractions/integers, no floats. Probe numbers are regression pins for walt,
never axioms (TRUST-01).

Run: `python3 lambda_probe_v3.py` from this directory (v3 imports v1/v2 by
path). Running creates `__pycache__` — this directory is not an ingest
package, but clean up anyway.
