# Accepted findings for prefix-state continuation

Exploratory implementation evidence unless a conditional abstract Lean result
is explicitly identified. Numbering matches [REPORT](REPORT.md).
[Prior accepted findings](../native-frontier-20261004/ACCEPTED.md), the
[canonical accepted ledger](../adversarial-20261004/ACCEPTED.md), and
[C1–C3 source contradictions](../adversarial-20261004/CONTRADICTED.md) are retained
unchanged. No new sourced assertion is classified as contradicted.

- **1–3:** Pinned production identity; reviewed semantics and full cache keys;
  one occupied public State per full prefix, with conditional state/view/sum
  lemmas and independent finite parity. No Rust refinement proof is claimed.
- **4–6:** Exact child-list interning plus factoring beats equal-worker native
  Dice recursion on the measured40/128/512-world panels, including a49-root
  holdout. Eight-world batches lose. Pure hash factoring alone did not establish
  the same win. Reuse is counted separately from scenario work.
- **7:** Full cost, CPU and PID-specific RSS are measured. There is no uniform
  memory advantage, universal crossover or linearity theorem; reserved coordinate
  capacity still scales with projected row count.
- **8–9:** Full policy and hybrid vectors match. Full policy remains slower and
  unique actor demand grows sharply across fields0–2; field2 at40 worlds refuses
  one50k-query allowance. Exact choice-only hybrid completes. Stronger play,
  larger k, opening positions and a full phone adapter remain unestablished.
- **10:** Demand-limited shared choice with integer bounds is the concrete next
  experiment, with stated semantic and measurement obligations; it is not yet
  implemented or accepted as a speed gain.
