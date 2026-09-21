# Eight-world support representation

The modeled sigma0 mind normally evaluates eight sampled worlds. The previous
representation interned a heap vector of surviving sample IDs for every field
partition, then locked and cloned shared pointers to recover that vector on
subsequent nodes. The small-support path uses an eight-bit set instead.

A bit names a **sample ID**, not a unique deal. If two samples contain identical
hands they still occupy different bits and contribute separate units of mass.
For n in 1..8, let F be the low-n-bits mask and M the surviving sample mask. The
solver-local support id is M XOR F. This is bijective, preserves empty support,
and maps full support to id0 as required by public roots. Iterating set bits in
ascending order equals the old sorted sample-ID vectors. Consequently field
partitions, sampled Dice seeds, success counts, probability denominators, viewer
actions and policy choices are unchanged. Larger supports retain the general
intern table. IDs are opaque and local to one Solver, never persistence keys.

The small serial Dice path partitions samples into 28 stack masks, then visits
nonempty tile buckets in the same increasing tile order. No subset vector,
intern lookup, reference-count clone or child-list allocation is needed. The
existing parallel path still works through the general partition routine. A
256-bit atomic presence table preserves the `alive_sets()` diagnostic count.

Validation exhaustively roundtrips every subset for n=1..8, including empty and
full support, and checks ascending iteration and exact distinct-set counts.
The candidate also matched 64 retained/fresh cases (including 4/12/40/160 worlds)
in exact fractions, nodes, modeled policy calls and inner sample totals. The
partnership, public-void sampler, frozen ordering and selection suites passed;
the non-parallel library build passed. See small-support-parity-summary.json.

This changes the representation of finite sampled support. It does not change
Walt's field model, hidden-information policy, or empirical calibration claims.

A paired test of 16 identical retained 160-world requests (four concurrent pairs,
alternating old/new order, bounded to 60 seconds) measured a 1.679x median and
1.672x geometric speedup relative to the preceding allocation-optimized binary.
All exact values and work counters matched. The one-minute production firing
retained 780 real prices with zero errors. See small-support-timing-summary.json.
