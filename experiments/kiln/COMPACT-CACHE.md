# Compact search caches

A two-second macOS sample of a production worker showed cache hashing and small
allocations among the prominent CPU consumers. Raw profile:
`/Users/jason/data/texas-42/kiln-v1/worker-profile.txt`. It is a workload snapshot,
not a universal time breakdown.

The optimization changes only private table storage:

- Memo keys and policy-cache keys store the unfinished trick inline. Starting
  with sentinel 1, append each tile as five bits. At most three tiles, each
  numbered 0–27, fit in 16 bits. The sentinel preserves length and leading zeros.
  Exhaustive testing checks injectivity of every sequence of lengths 0–3,
  including sequences that would not be legal domino histories.
- Every previous identity field remains in equality: public void state, played
  set, leader, trick order, both banked totals, and the appropriate alive-set id
  or private hand and modeled seat. Modeled level remains in the policy key.
- Private maps use a scalar word hash with full equality checks on collisions.
  Hashes route cache lookups only. They never seed sampling, choose actions,
  identify external records, or replace equality.
- Public Key/PiKey, game transitions, samplers, sample seeds, field policies,
  move order, tie choices and rational outputs remain unchanged.

The optimized worker matched 121 retained/fresh reference cases, including
4/12/40/160 worlds, in both exact prices and nodes/policy calls/inner sample counts.
See compact-cache-parity-summary.json. The packing test and the partnership,
sampler, ordering/frozen-value and selection suites passed. One historical
fixture-regeneration test remains intentionally ignored.

This is an implementation-equivalence argument for the current finite solver,
not a proof of the field model's predictive accuracy. The sixes/36 calibration
mismatch remains a separate finding.

Sixteen identical retained 160-world jobs were then timed with both binaries,
four concurrent pairs, alternating old/new order. All values and search counters
matched again. The median speedup was 1.2994x and geometric mean 1.2963x.
This is a paired workload measurement, not a claim about all game states or a
four-player match. The diagnostic is bounded to 60 seconds and saves each
completed pair. See compact-cache-timing-summary.json and benchmark.py.
The production one-minute firing also retained 404 actual prices with zero
errors; differing job difficulty prevents using it as a controlled speedup.
