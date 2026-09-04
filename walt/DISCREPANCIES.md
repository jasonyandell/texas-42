# walt: spec-vs-reference discrepancies

Protocol (CLAUDE.md): where the frozen spec and a reference implementation
disagree, implement what the **receipt replay** forces, and record the conflict
here rather than silently picking a reading.

## Open discrepancies

**None as of S5a.**

## Reconciled, not discrepancies

### doom-census ledger paragraph: "overwhelmingly the info-consistency price" outruns what was established (UP1a/U0b, 2026-09-03)

The doom-census paragraph of `walt/FACTOR-BELIEF.md` (2026-09-01) closes
its opening-root diagnosis with: "the plateau's remaining Γ ≈ 267‰ is
overwhelmingly the INFO-CONSISTENCY PRICE — purchasable by floor work
... and info-consistency-aware uppers, never by counterexample
counting." The salvation parent adjudicated the same session (SC-A1)
states the correction in its §8–§9: a zero doom census moves only
`d_phys` and "does **not**, by itself, prove that the remaining gap is
information-consistency price" — the unclaimed mass is `d_info +
d_policy`, and zero doom does not distinguish them. U0 then typed the
opening root `UnknownGodGap` on all seven actions, claiming nothing
about either term (SC-A4). The ledger sentence therefore overclaims
relative to the mathematics adjudicated the same day.

What the later evidence suggests, without settling it: U0's twelve
trick-4 information prices are 6–22‰ with `d_policy = 0` at every one,
and U0b's in-solve census finds the trick-5 frontier's mass-weighted
price at 13–14‰ under the trick-4 roots; the opening upper of 999‰ is a
512-world sampled optimization lock, not a doom bound. The honest
statement is that the 267‰ is UNKNOWN in its split, with the sampled
upper's looseness and the policy gap both live candidates. Recorded
here rather than rewritten in the ledger (the ledger is a dated
running record); the doom paragraph carries a one-line pointer to this
entry.

### salvation-complex intake companion: two divergence points named, three in the record (U0, 2026-09-02)

The intake companion `walt/math/salvation_complex_v0.1_intake.md` records
that the parent's §9 table "correctly cites truth, not census recovery, at
both divergence points (h4-t6 0-0: truth 60 where the census certified 56;
h8-t5 5-3: truth 1 where the census certified 0)". U0's G1 gate re-derives
the whole table mechanically from the committed record
(`walt/probes/factor_belief/doomreport_run1.txt`) and finds **three**
coordinates where the class census's certified mass falls short of the
per-world truth: the two named, plus **h8-t5 0-0, where the census
certified 17 of 21** — a divergence the record itself prints in its own
recovery column (809‰).

Not a conflict, and nothing above it moves: the §9 table cites the TRUTH
column at every one of its fourteen coordinates, so every `d_info = 0`
inference stands exactly as adjudicated (SC-A1), and the class census's
one-sided soundness is unaffected (a certified harvest never exceeds the
truth — asserted per coordinate in the same gate). What is one short is
the companion's COUNT of divergence points. Recorded rather than repaired
(the companion is a dated intake record); the gate
`walt/walt/tests/solver_godgap.rs::the_section_nine_table_is_re_derived_from_the_committed_record`
now asserts all three by exact value, so the number cannot drift again
without a red suite.

### exp3A descriptor pin: blocked in S4, unblocked in S4.5

S4 recorded this as open: v0.4 §14.4 reports the winning static descriptor
on the 90-world design kernel --

> "Exhaustive search over subsets of size at most four found eight minimal
> four-observable solutions. One was D = {comp41, s3max2, team(2:0),
> team(4:2)}. It produced 33 cells, each pure for the eight-class root-Q
> target: 90 worlds -> 33 descriptor cells -> 8 responses."

-- but the spec defines neither the "22-observable vocabulary" nor the
semantics of `comp41`/`s3max2`, and the exp3A probe source was thought
lost. The block dissolved when the lambda-probe chain was preserved at
`walt/probes/exp3a/` (commit 9357536): `lambda_probe_v3.py` Part 1
(`build_atoms`) carries the full 22-observable registry. Extracted
semantics: `comp41` = the tile sharing the valued tile 4-1's holder's
two-tile hand; `s3max2` = the partner seat's best rank in the decisive
suit-2 context (the context the viewer's remaining tile 2-1 forces at
trick 7), with the probe's ad-hoc suit ranking (double top, then by pip);
`team(t)` = whether the partner seat holds `t`; the other 19 atoms are
holder coordinates and suit-2 control relations (opponent strength/top,
opponent beater count, best-keep trick-7 winner, boss/floor companions).

Reproduction (`walt-skeleton/src/atoms.rs::Exp3aAtom`,
`tests/harness.rs`): the semantics were reimplemented from the probe's
definitions (probes are validators, never source; nothing was copied), at
the partition level -- walt's `Decl::rank` is order-isomorphic to the
probe's ranking and every atom feeds only equality cells and strict order
comparisons, so the induced world-partitions are identical. Through walt's
own §12.1 checker: **D = {comp, focal-max, team(2-0), team(4-2)}
reproduces 90 -> 33 -> 8 exactly**, and the full <= 4 search reproduces
the probe's entire Part 1 record -- minimal size 4, exactly eight
solutions (the {comp | comp-rank} x {holder | team} family) at
69/53/53/33 cells, for BOTH the 8-class parametric target and the 3-class
action-correspondence target. All exploratory-tier regression pins, not
axioms.

One reading in the port is walt's own, recorded here: on the twelve
non-design kernels the vocabulary's parameters (decisive tile/context) are
derived by walt's generalization rule -- decisive tile = the viewer tile
whose led context touches the most hidden-pool tiles, ties to the higher
tile -- which lands exactly on the probe's constants (2-1, suit 2) for the
design kernel but is NOT probe-backed elsewhere; the corpus-wide table in
`tests/synthesis_run.rs::exp3a_registry_search_over_the_trick6_corpus` is
walt-tier only.

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
