# AUDIT-RESPONSE (Oct 4 2026)

Audit points received and accepted:

1. **`l1_orders` sampled-m branch is not a shared-plan estimator.** Each world draws its own lines, so a column is not a policy; summing by
   column mixes orderings and may still fuse. The "one-line swap (sum before min)" suggestion for that branch is RETRACTED.
   `flatplan.py` is the lawful version: plans drawn ONCE, independent of any hidden world, keyed by canonical ordering tuple (deduplicated
   in this revision), tiled across worlds; each world's tape row reused across plans.

2. **False-support sampler witness — confirmed in `batched.py` (v1).** The fallback filled unfillable slots with an unchecked candidate.
   `batched.v2.py` never emits an unchecked world; it returns a fill mask. `l2.py` (sandbox) now replaces an unfilled slot with a VERIFIED
   world of the same problem (duplicate weight) or raises. `consistent_worlds` (engine42.py) and `sample()` (walt.c) are replay/void-filtered
   and were not affected; void inference and legality replay define the same support (an off-suit play ⇒ holds no follower; a following
   play constrains nothing).

3. **Realization lemma scope.** For a fixed world and tape, first-legal-in-ordering over all orderings realizes exactly the leaves of the
   own-choice tree (per-world claim; what exact-tape rests on). A SHARED ordering across worlds is a strictly smaller class than
   history-dependent information-set policies. Nothing here shows otherwise.

4. **Measured class gap and cost** — shared orderings vs exact own-tree, same 128 worlds and tape, reference exact-tape@384, 50 positions/hand:
   - defending hand C: decision agrees with tree 86%; regret flat 0.0087 vs tree 0.0000; value lost to class 0.064; 26k rows; ~180 ms (numpy)
   - bidding hand A:   decision agrees 78% (trick 1: 25%); regret flat 0.0131 vs tree 0.0023 (trick 1: 0.034 vs 0.003); value lost 0.111 (trick 1: 0.34); 26k rows; ~270 ms
   - Walt's 8-world inner rung on the same hands: regret 0.018 (C), 0.023 (A).
   Conclusion: the shared-ordering class is lawful and fusion-free by construction, better than the 8-world inner for defenders, WORSE for a
   bidder on the opening lead. Not sufficient as a universal inner rung. Recommended design remains: keep the own-choice tree as a
   ply-synchronous frontier (no recursion), batched across all inner-rung problems per ply, with the group-by-history fold.
