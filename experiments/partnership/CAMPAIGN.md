# Resumable bid-30 campaign

Exploratory executed-policy evidence. Primary panel: seeds 420600–420699,
three matched games per seed: all phone, candidate declaring partnership,
candidate defending partnership. All bids are 30. A deterministic fixture
heuristic picks a bidder and pip trump from the generated deal before play;
this is not an auction-policy evaluation. The heuristic prefers trump length,
trump double, then off-trump doubles, with seat/pip tie breaks. It deliberately
selects plausible contracts; it is not a uniform random-contract population.

Each player receives only its original seven tiles and public history. The
public policy seed is always 420600, independent of the hidden deal seed.
Phone uses the preserved local WASM, 40/8 and racing. Candidate uses partner-only
40/8 with two inner worlds. The player retains its 14-second decision ceiling
and visible fallback behavior. No declaration or bid optimization is measured.

## Outcomes and review rules

Make means the declaring partnership takes at least 30 points. The candidate
declaring comparison wins if it makes where phone sets; the defending comparison
wins if it sets a contract phone allowed. Identical make/set outcomes tie,
regardless of points. Points over 30 are retained only as diagnostics.

Seeds 420601–420603 were previously inspected and are flagged as development
data. They appear in the full table but are excluded from fresh evidence.
Both role comparisons share a reference and are dependent. For a seed, the sum
of their signed outcomes is in {-1,0,1}; that seed is the statistical unit.

The downside monitor multiplies by 3/2 on a negative seed, 1/2 on a positive
seed, and 1 on a tied seed. Pause at value >=20 after at least ten fresh seeds.
This is a valid one-sided e-process only under the hypothesis that each fresh
seed's conditional expected signed outcome is nonnegative. Its <=5% crossing
interpretation depends on treating these pseudorandom generated deals as fresh
independent draws and fixing the player/generator. It is not a universal
Texas 42 strength claim. Exact integer fractions are stored. No efficacy stop
is claimed; all 100 seeds run unless a pause rule or user interruption applies.

Also pause for technical errors, fallback use above 5% after 20 nonforced
decisions, or an uninformative panel after 20 fresh seeds with no reference
makes and no make/set differences. The last rule indicates an unhelpful panel,
not equivalence. Review the accumulating table after roughly ten minutes;
do not spend the entire allowance on a plainly failed or uninformative run.

## Durability and parallel work

One seed is active at a time, with all three games concurrent. Native game
workers use a fixed configurable Rayon thread count; phone WASM remains single
threaded. Every completed decision is written through an atomic replacement
with file and directory synchronization. Each arm has a checkpoint and a final
result. Only all three complete arms produce a committed seed result. Restart
skips committed seeds and completed arms and replays partial public histories
through the independent referee. Hard interruption can discard an in-progress
decision, never a previously committed seed. A process lock rejects overlapping
runners for the same campaign. Source and executable hashes prevent silently
resuming with a changed player or runner.

Each foreground advance slice stops starting decisions at its time allowance;
an active decision then finishes under its existing deadline. The packet's
295-second process-group watchdog bounds every slice and reaps its descendants.
Use the watchdog for every advance. A stop marker is persistent and must be
explicitly cleared to resume. Time slice exhaustion is resumable without
clearing a marker. No asynchronous worker is left running between slices.

From the worktree root, initialize once:

```sh
python3 experiments/partnership/campaign.py init experiments/partnership/campaigns/random-420600-699 --threads 6
```

Advance with a new, unused output directory each time:

```sh
python3 experiments/partnership/packet/texas42-partnership-launch-v0.1/tools/run_capped.py --seconds 295 --output-dir experiments/partnership/campaigns/random-420600-699/caps/slice-01 -- python3 experiments/partnership/campaign.py advance experiments/partnership/campaigns/random-420600-699 --seconds 260
```

Inspect `campaigns/random-420600-699/STATUS.md` or `status.json` while running.
Each completed seed is in `results/`; detailed decisions are in `seeds/`.
Stop and resume commands:

```sh
python3 experiments/partnership/campaign.py stop experiments/partnership/campaigns/random-420600-699
python3 experiments/partnership/campaign.py resume experiments/partnership/campaigns/random-420600-699
```

Then launch another capped advance. Resume does not itself start computation.

## Same hand, different worlds

The optional `--panel worlds` generator groups ten completions per fixed opening
hand, with bidder, declaration, own seven tiles, empty public history, and policy
seed held constant. Only the other three hands change. Bidder rotates by group.
Each completion still has all three lineups. For each lineup, completed worlds
must have the same opening choice unless a recorded deadline fallback changes
the executed policy. This checks information isolation while later trajectories
can respond to different public play. These are clustered worlds, not 100
independent hands; the random-panel downside stopping test is disabled.
The generator is implemented and tested, but this panel is separate from the
requested primary 100-deal campaign.
