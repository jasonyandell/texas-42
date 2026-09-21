# Pre-evaluation engineering corrections

2026-09-18. No fresh per-game outcomes or associations had been inspected.

The original fit's complexity metadata counted extra role declarations along
with predicates. Corrected to count actual predicate atoms as specified. The
primary and four selected relational queries did not change; all 13 evaluated
query sources and their order were compared and were identical. Predicate counts
are 6, 6, 10, 10. Both versions remain outside git with the campaign.

- Earlier fit: `analysis/fit.json`, ID
  `01cc319cfcd98d5c80310fc1bd47062ee3fc8a21805ee35144933a46f5d4a1cc`.
- Corrected fit: `analysis-v2/fit.json`, ID
  `173bd310c97901a6f4629117bcc082492b49bff272c9b2abcaa649578f3fbe8a`.
- The control-variate pre-label extension references the corrected fit. The
  coefficients are unchanged: 50/549 (literal double-five), 56/311 (top trump).

The native Scheme library/selected-query memberships also remained identical.
This correction did not change the data, grammar, rankings selected, statistical
gates, or fresh sample size.
