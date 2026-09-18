# Exact integer values inside the sampled solver

2026-09-18. Implementation proof sketch and exploratory regression evidence;
not a new corpus theorem or a claim of player strength.

`Solver` gives equal mass to every sampled world index, including duplicates.
At node K let A(K) be its alive index set and C(K) the number of successful
indices under the optimal information-consistent viewer continuation. Its
existing rational value is exactly C(K)/|A(K)|.

Induction on remaining plies:

1. A make terminal has C=|A|; a set terminal has C=0.
2. A viewer action retains the entire alive set. Each candidate therefore has
   the same denominator, so max/min of rational values equals max/min of C.
   The existing visit order and exact 0/1 stopping conditions are unchanged.
   One action is still chosen for the whole information set, never per world.
3. A modeled seat deterministically assigns each world to an observed-action
   bucket. This holds for the sampled Dice tapes and the deterministic Pi
   policies. Buckets partition A. Weighted averaging cancels denominators:
   sum_b (|A_b|/|A|)*(C_b/|A_b|) = sum_b C_b/|A|.
4. Thus the internal recurrence needs only integer success counts. The public
   API returns the same reduced BigRational, once at the boundary.

Counts are bounded by the number of stored u32 world indices. u64 addition
cannot overflow within this representation; the partition conservation and
mass upper-bound assertions remain active. No floats or approximate arithmetic
are introduced. Weighted beliefs in other solver modules are not changed.

The cache remains local to its exact world bundle: an alive-set intern id has
meaning only in that Solver. This change provides no permission to reuse node
values across a different bundle, objective, field, or sampler profile.

Validation: 117 retained price cases and four additional 4/12/40/160-world
comparisons matched prices, node counts, policy-call counts and inner-world
counts against the preserved rational implementation. Original binary SHA:
14939b7ab95452efb82be60e2678081fe5bf32bdee0f7fe88c665ed620b4d08f.
Raw comparison: `/Users/jason/data/texas-42/kiln-v1/counted-parity.json`.
The four Kiln tests (including SIGKILL recovery and all catalogue shuffle
seeds) and the 19 active sampler/partnership tests passed. The historical
sampler-fixture regeneration test remains intentionally ignored.
