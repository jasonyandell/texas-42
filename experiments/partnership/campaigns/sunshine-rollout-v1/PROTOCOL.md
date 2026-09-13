# A bounded deployed-continuation partnership check

2026-09-13. Exploratory. Written before the candidate's measurements. The goal
is a reasonable, fast partner-aware player, with conditional skill measured
directly even if its opportunities are rare in ordinary games.

The gym supplied four missed useful count offers and six harmful offers by L1
in its development corpus. Another four mistakes choose a weaker non-offer.
An offer predicate is an opportunity to investigate, not a reward bonus.

## Candidate and hypothesis

`l1-partner-rollout` first completes ordinary fixed L1 40/8. If the declaring
player has two or three own tiles, the bid-30 contract is unresolved, and the
public count-offer Scheme query divides the legal choices into offers and
non-offers, compare **all** legal first moves. This applies whether the baseline
offered count or withheld it.

Each root move is followed by completed deployed fixed L1 40/8 decisions at all
seats, with exactly the original-hand/public-history seed formula. Each seat
sees its own original hand and public history. The simulator may use a sampled
world to distribute tiles and enforce rules; it never supplies that full world
to a continuation chooser. Stop each trajectory once make/set is settled.

Enumerate uniform mechanical support up to 400 worlds and shuffle without
replacement using an independently domain-separated public seed. Examine at
most 64 worlds, subject to a 500 ms outer allowance including startup. Count a
world only after every legal action's continuation completes. Require at least
eight completed worlds unless the whole support is smaller; prefer a strictly
higher observed make count and retain baseline ties. No count-point tiebreak.
The native soft deadline reserves 35 ms for output/cleanup inside the parent's
allowance; the whole player retains its existing 14-second ceiling.

This is deliberately a **fallible sampled guess**. A deadline may leave a
runtime-dependent prefix; it is not claimed to be an unbiased fixed-size
sample, a calibrated confidence interval, or a certified decision. A full
census is exact for this frozen continuation. Too little completed evidence,
support refusal, malformed output and worker failure retain the baseline.
All complete prefix values and paired wins/losses remain visible in the record.

Hypothesis: executing the future player's policy, rather than optimizing a
different focal continuation, will reduce conditional root regret at affordable
cost. The experiment can reject the candidate without discarding the instrument.

## Validation and development

Verify the native continuation adapter against the saved deployed gym: compare
its decisions by exact own/public input, all legal trajectories and make counts.
Use complete small fibers and three representative fixtures, including the
original useful 6-4 and 5-0 offers and a withholding case. Check rotation,
original-hand seed use, observed voids, duplicate information reuse, absorbing
outcomes and incomplete-world refusal. Keep the baseline evaluator unchanged.

Then run the candidate on all 143 declaring matched development roots, including
ties and the complete legal action vectors. Record baseline/candidate action
values, optimality, paired gains/losses and additional time. No values from the
answer key enter the player. If engineering changes or a policy revision are
needed, declare them before the fresh evaluation.

## Fresh evaluation

After development, freeze code and settings. Play 64 new mirrored random deals,
seeds 99210000–99210063: candidate versus default L1, team roles swapped, bid 30.
Every both-make or both-set pair ties. These seeds are the group units for
ordinary-game reporting; this panel checks side effects and operational cost,
not whether the rare conditional skill matters.

Use the baseline-declaring arm of those games as a separate source for the
unchanged public count-offer query. Because the candidate's review is restricted
to declaring players, that arm's continuation behavior is ordinary L1 at every
seat. Value every matched position within the 400-world cap using the generic
deployed-L1 gym. Keep tied positions and cap exclusions. Evaluate the frozen
candidate on all declaring matched roots, not just outcome-selected exercises.
Report root-averaged regret and paired same-world improvements/harms, with source
seed grouping. Do not turn hidden completions into independent source deals.

Use the existing ten-worker pool and atomic per-item records. Every workload
runs inside a separate watchdog allowance of at most 295 seconds; repeat slices
to resume rather than launching an unbounded job. No outcome-based adjustment
of the frozen candidate during the fresh evaluation. A useful result must
consider both missed help and harmful intervention, not just corrections on the
development examples. This tests one partnership skill, not general mastery.
