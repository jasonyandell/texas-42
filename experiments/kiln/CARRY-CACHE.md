# Carry completed inner-policy answers between refinements

Kiln's persistent native worker can now keep one completed policy cache. The
ordinary 8 → 40 → 160 sequence recomputes the outer solve at each sample size,
while reusing completed inner-policy answers where their inputs match. It does
not average earlier prices, reuse an outer policy, or substitute a score from
another target. The existing bidder and phone entry points remain stateless.

## Why reuse preserves the implemented evaluator

As implemented, `Solver::pi` seeds its own belief from the fixed `INNER_SEED`,
level, acting seat, remaining hand and public record. Its key includes the actor's
hand, seat, played tiles, current leader/partial trick, banked points and voids.
It does not use the enclosing world's unseen hands, outer sampling seed or outer
sample count. The shared context supplies declaration, bid, inner sample counts,
boundary played mask/hand size, belief strategy and modeled selection rule.

`Shared::take_policy_cache_from` requires every one of those seven shared context
coordinates to agree. It takes exclusive mutable ownership of both old and fresh
contexts and moves the cache, leaving the old one empty. It never copies deadlines,
death flags, counters, outer samples, search memo tables or incomplete answers.
`pi` already inserts only after a complete comparison finishes within its budget.
Thus a hit supplies the same completed answer that a fresh calculation would
produce. This is a claim about this finite sampled implementation, not a theorem
that its information representation solves Texas 42 generally.

The worker retains only its last context and drops retention above 100,000 policy
entries. Different contexts start fresh. A restarted worker starts fresh too;
saved price receipts and refinement decisions still live in SQLite. Cache reuse
is optional acceleration and introduces no additional recovery requirement.

## Work accounting and evidence

`carried_policy_entries` records the starting cache population. Nodes, policy
calls and inner worlds count only newly executed work, so warm and cold work
counters can differ. The independent auditor checks that the final cache contains
at least the carried entries and at most carried entries plus new policy calls.
Older receipts implicitly carry zero. A cold replay reproduces the price; its
work counts need not reproduce a warm process's history.

The first candidate passed 36 retained/fresh cold exact-fraction and counter
comparisons. Thirty-two paired persistent-worker ladders matched all 96 prices
at 8/40/160 worlds and measured **1.244x median / 1.226x geometric speedup**.
They ran with production stopped, four concurrent alternating-order pairs, and
a 60-second limit. Raw evidence is carry-cache-{cold-parity,timing}.json in the
campaign. The candidate's immutable source/binary snapshot is
77f81f56770c3cf166b15ab6b3742a1f43462b9c5814e2ee541a732373c2cbc4.

Rust tests also compare a warm sequence with cold prices after changing the
declaration, bid, outer seed, seat and hand, test all seven context rejection
conditions, and ensure the deadline/death/counters reset. Thirteen Python tests
passed, including carried-counter validation, abrupt-death recovery and the
existing auction parity check. The later production firing and final producer
identity are recorded in PROGRESS.md.
