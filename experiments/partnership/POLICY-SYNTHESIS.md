# Persistent and compositional policy synthesis

## Question

Can a policy constructor retain or compose useful work as its sample grows,
while matching or improving the policy obtained by solving the accumulated
sample afresh?

This first experiment studies one information root at a time. A fixed-root-hand
campaign repeats hidden worlds beneath one supplied seven-tile hand. A
random-own-hand campaign draws an independent full deal and starting hand for
each seed. Aggregating those independent roots tests whether the mechanism is
useful across starting hands; it does not construct one policy generalized
across deals.

## Objects under comparison

For each seed and each prefix of the same ordered training-world stream, the
native laboratory produces three arms:

- `fresh` solves the entire accumulated prefix with an empty cache.
- `persistent` extends the preceding prefix and retains valid exact search
  state. Its completed policy and value must equal the fresh solution.
- `compose` collects successful alternatives from single-world donors and then
  searches the accumulated prefix within that restricted grammar. Donor work
  and restricted-search work both count toward its reported cost.

Every arm is evaluated on the same independent heldout draw stream. Sampling
is with replacement, so physical worlds may coincide across streams. Training
prefixes are nested, so the result at 16 samples extends the
same stream used at 8 rather than drawing a new panel. `policy_id` identifies
the extracted policy program and makes exact-policy parity observable rather
than inferred from equal scores.

The field is frozen across all arms. `hash-legal` is a deterministic,
seat-local legal policy intended to isolate synthesis behavior cheaply. It is
not a strength baseline. `l0-8` provides the existing native L0 field when that
cost is warranted. Field identity is part of the immutable campaign manifest.

## Evidence and interpretation

The primary comparisons are paired within seed and sample size:

- persistent versus fresh node count and elapsed time, conditional on exact
  value and policy parity;
- composition versus fresh training and heldout make counts;
- composition's total charged work versus fresh work, over the whole prefix
  schedule as well as at each stage;
- failure, budget, and policy-size behavior as the prefix grows.

Persistence succeeds only if it preserves the fresh result and reduces total
work over the schedule. The implemented complete-donor composition preserves
the training optimum (see `walt/scheme/COMPOSITION.md`); partial donor grammars
would need a weaker claim. Equal make counts can conceal different
policies, which is why value and `policy_id` parity are reported separately.

The solver is exact only on the supplied finite sample against the declared
frozen field. Heldout performance is sampled evidence for that root and field.
Neither result establishes optimal Texas 42 play, a universal policy quotient,
or convergence over the full deal distribution.

The separate `policy_gym.py` quality panel applies the constructors to the 30
maintained bid-making gym roots. It uses the gym's frozen field and evaluates
each serialized policy over the entire compatible root fiber. The report keeps
the policy's first-action regret against the exact gym key separate from the
full policy's make-rate gap to the best root value. The latter includes every
later policy decision; it is not another estimate of root-action Q. The gym
field has a deterministic action cache shared during one case, so this panel
supports quality and exact replay comparisons, not arm timing comparisons.

## Reproducible campaign

Build the release `policy_lab` binary, then initialize once:

```sh
python3 experiments/partnership/policy_campaign.py init \
  --output experiments/partnership/runs/policy-random-pilot \
  --binary walt/target/release/policy_lab \
  --mode random-own-hand --field hash-legal \
  --seeds 700000:700020 --tiles 4 \
  --samples 1,2,4,8,16,32,64 \
  --test-worlds 256 --node-budget 100000 \
  --decl 6 --workers 10 --seed-timeout 120
```

Run or resume it beneath the repository watchdog:

```sh
python3 experiments/partnership/packet/texas42-partnership-launch-v0.1/tools/run_capped.py \
  --seconds 295 --output-dir experiments/partnership/runs/policy-random-cap-01 -- \
  python3 experiments/partnership/policy_campaign.py run \
  --output experiments/partnership/runs/policy-random-pilot --seconds 285
```

The manifest pins configuration, runner source, and binary bytes. Each valid
seed is committed atomically under `results/`; failed or interrupted attempts
remain receipts under `seeds/` and are retried. `status.json` includes pending
and active seeds. `summary.json` contains per-arm metrics, paired results by
sample size, and total work across the schedule. Resume refuses changed inputs
or damaged completed results.

Run the exact quality panel under the same outer watchdog:

```sh
python3 experiments/partnership/packet/texas42-partnership-launch-v0.1/tools/run_capped.py \
  --seconds 295 --output-dir experiments/partnership/runs/policy-gym-cap-01 -- \
  python3 experiments/partnership/policy_gym.py \
  --gallery /Users/jason/data/texas-42/partnership-bid-making-v1 \
  --output experiments/partnership/runs/policy-gym-v1 \
  --binary walt/target/release/policy_lab --workers 10 --seconds 285
```

## Evidence

The [completed report](campaigns/policy-synthesis-v1/RESULTS.md) covers 32
opening roots through 200 samples, 12 opening roots under native L0-8, and the
30-case exact partnership gym. Persistence approximately halved opening search
work while preserving completed policies. Donor composition preserved policies
but did not repay its setup cost. Exact tables still show substantial fitting
to training samples; no cross-hand relational generalization is claimed.
