# walt: spec-vs-reference discrepancies

Protocol (CLAUDE.md): where the frozen spec and a reference implementation
disagree, implement what the **receipt replay** forces, and record the conflict
here rather than silently picking a reading.

## Open discrepancies

### exp3A descriptor pin: blocked in S4 (atom semantics undefined)

v0.4 §14.4 reports Experiment 3A's winning static descriptor on the 90-world
design kernel:

> "Exhaustive search over subsets of size at most four found eight minimal
> four-observable solutions. One was D = {comp41, s3max2, team(2:0),
> team(4:2)}. It produced 33 cells, each pure for the eight-class root-Q
> target: 90 worlds -> 33 descriptor cells -> 8 responses."

The spec defines neither the "22-observable vocabulary" nor the semantics of
`comp41` or `s3max2` (each appears exactly once, in §14.4); §12.3 gives only
shape names ("companion, decisive-context partner strength, forced-follower
team, and beater team"), and no exp3A probe source survives -- the preserved
suite at `walt/probes/` holds exp5 only. Reproducing 90 -> 33 -> 8 would
require inventing atom semantics, which the ambiguity protocol forbids.
Blocked test: `walt-skeleton/tests/harness.rs::exp3a_static_descriptor_pin`
(`#[ignore]`d). Unblocks if the exp3A observable definitions are ever
preserved the way exp5's were.

S4 therefore built its own fully-specified registry in the same language
family (per-tile holder/team facts, beater counts; `walt-skeleton/src/atoms.rs`)
and pinned walt-tier numbers for it. Notably, that registry is UNSOUND on the
design kernel at every subset size <= 4 for all three targets
(`tests/synthesis_run.rs`) -- walt's holder-shaped vocabulary does not
reproduce 3A's four-atom success, which is evidence the missing 3A atoms
carried genuinely control-shaped content, not a substitute for them.

## Reconciled, not discrepancies

### exp5 census pins: blocked in S2, unblocked in S3.5

S2 recorded this as open: PLAN.md pinned "h1t3: 10 q_points classes; h3t3:
5345" but v0.4 §14 never defines exp5 or "q_points", and the probe scripts
were not in the repository, so there was nothing to reproduce without
guessing. The block dissolved when the exp5 probe suite was preserved at
`walt/probes/exp5/` (commit b3cb523). Extracted definitions
(`exp5_core.Solver`, `exp5_census.scalar_job`): a `q_points` class is an
exact PI root value vector under the real scoring differential -- each trick
worth +-(1 + count points of its four tiles), focal team minus opponents --
and the h1t3/h3t3 counts are censuses of a **sampled** world set: 10,000
exactly-uniform draws from the fiber at recorded seeds (42042013, 42042033),
marked `+` (lower bounds of the fiber census) in `exp5_results.md`.

Reproduction (`walt-strat/tests/exp5_census.rs`): the two samples were
regenerated with the probe's own sampler at the recorded seeds and their
distinct worlds frozen as fixtures (`walt-strat/tests/data/`); the distinct
counts (9,920 / 9,933) match the records' `n_distinct_worlds_solved` exactly,
fingerprinting the streams, and duplicates cannot add classes. walt's scalar
PI census over those worlds reproduces every recorded value: h1t3
q_points 10 / act_points 8 / q_trick 2 / act_trick 1; h3t3 q_points 5345 /
act_points 31 / q_trick 1007 / act_trick 31 -- plus both exhaustive
horizon-2/3 tables (13 kernels x 4 targets each) and the trick-6 `q_param`
row. All exploratory-tier regression pins, not axioms.

One statistic deliberately not pinned: `act_param`. The probe canonicalizes a
parametric optimal-action correspondence by *segment identity* on the upper
envelope; walt's `ArgmaxCorrespondence` is argmax-by-value with at-point
events (it distinguishes an isolated boundary tie from none; the probe's
does not). These are different statistics that often, but not provably
always, agree in count -- pinning one against the other would blur the
definitions rather than cross-validate one.

### §14.5 "future focal information-state counts" are choice states (S3)

§14.5 reports information-state counts 168 / 7,848 / 504 after roots
0-0 / 2-1 / 3-2 of the trick-5 kernel without saying whether forced states
(one legal action) are counted, and §10.9's `E_B(a)` definition does not
settle it either. Reproduction does: walt's canonical perfect-recall
partition reaches 60,360 / 69,600 / 164,088 future focal states, of which
exactly 168 / 7,848 / 504 offer two or more legal actions. The record counts
genuine choices. Pinned under that reading in
`walt-strat/tests/exp4_information.rs::information_state_counts_match_the_record`,
with the full totals frozen alongside as walt-tier pins consistent with (not
sourced from) the record.

Everything else walt implements is the v0.4 text; the two
places where the spec's phrasing and `rules42.py`'s code *look* different are
reconciled below, and both were checked exhaustively rather than argued.

### Rank of a mixed tile: "pip sum" (spec) vs. off-pip (reference)

v0.4 §1.3 says a mixed tile is "ranked by pip sum inside a nonzero tier";
`rules42.rank_key` ranks a mixed tile by its *off-pip* within the suit. These
are the same order. Ranks are only ever compared inside one tier, a tier fixes
one effective context `q`, and every mixed tile there carries pip `q`, so
`pip_sum = q + off_pip` is monotone in `off_pip`.

walt-core implements the spec's formulation (`Decl::rank` is a function of the
tile alone), which makes the rank table global. The safety condition that
buys -- distinct tiles in a shared nonzero tier never share a rank -- is
asserted exhaustively in `walt-core/tests/exhaustive.rs::ranks_are_injective_within_every_nonzero_tier`,
and the doubles sentinel (12, above the mixed maximum of 11) is what keeps
"a natural double is top in its effective natural suit" true.

Under doubles-trump the doubles are ranked by pip (0..=6), which numerically
overlaps mixed pip sums. That is harmless only because a double is always
tier 2 under DT and a mixed tile never is; the injectivity test is what pins it.

### Declaration coverage of the ground-truth bridge

`rob/receipts/verify_player.txt` contains pip-trump hands only (P0, P1, P3, P4,
P5, P6 -- no P2, no DT, no NT). The replay bridge therefore validates the
pip-trump path against rob and nothing else; DT and NT are pinned by the
exhaustive structural tests only. This is recorded honestly in
`walt-core/tests/receipt_replay.rs::receipt_declaration_coverage_is_pip_trump_only`,
which fails if the corpus ever changes shape.

Independently of CI, S1 ran a one-off differential of the whole rule algebra
against `rules42.py` over **all 737,100 four-tile tricks x 9 declarations**
(winner and trick points) and over all 2,016 follow predicates and 252 led
contexts: **zero mismatches**, DT and NT included. That is an exploratory-tier
cross-check between two implementations, not a promotion of anything; walt-core
carries no receipt yet.
