# Outcome-first Scheme discovery: preliminary results

Exploratory, 2026-09-18. **The discovery-and-ablation loop works; transfer evidence
is mixed.** This is a working instrument and a source of hypotheses, not a learned
threat model ready to influence Walt. No production/player change.

The [protocol](../THREAT-PROBE.md) was written before the outcomes were inspected.
The run used 7,200 already played bid30 games: 100 bidder hands, nine declarations,
eight trials. These contain **800 hidden completions**, since declarations share
the completion and policy seed. All receipts were hashed and independently
replayed/validated again. The live producer continued running throughout.

Discovery: 15 source deals, trials 0..3 (2,160 games). Same-hand validation:
their trials 4..7 (2,160 games). New-hand transfer: the other ten source deals,
trials 0..7 (2,880 games). Four bidder hands from one source deal stay together.
Using the complete first-eight slice avoids adaptive production screening.

## What the learner actually did

It selected 64 distinct observed losing discovery worlds by a fixed hash order,
described each by 21 hidden tile ownership facts relative to the bidder, and
searched all their one-, two-, and three-fact sub-conjunctions. It searched
**57,008 distinct clauses**, found 5,250 eligible positive-association clauses,
and froze eight candidates using discovery data alone. No tactical clauses,
count-offer rules, or labels from the reserved sets entered the constructor.

This first grammar deletes facts but retains physical tile identities. It does
not yet invent abstractions such as "highest remaining trump," nor learn action
sequences. The seed descriptions and removed facts are retained in
[evidence.json](evidence.json), along with every selected candidate and its
immediate discovery-set ablations. Emitted files are actual executable Schemes.

The first end-to-end run took 10.06 seconds on the Mac while production continued.
That includes reading, validating, and preserving the receipts, fitting,
permutation checks, and the native membership audit. This is a single observed
run, not a throughput benchmark.

## All eight frozen candidates

`after` is the opponent immediately after the opening bidder; `before` is the
opponent immediately before the bidder in cyclic play order; `mate` is partner.
Values below are **percentage-point failure excess over the matched mixture of
each hand/declaration cell's baseline**. They are not win rates or policy gains.
Each assessment baseline uses that assessment cell's outcomes, so this is an
association statistic, not a deployable prediction.

| Discovery rank / retained ownership | Discovery | New worlds, same hands | New hands | New-hand adjusted p |
|---|---:|---:|---:|---:|
| 0: after 4-4; mate 6-2 | +11.11 | +5.86 | +1.67 | .931 |
| 1: before 5-2 and 5-5 | +12.78 | +4.86 | +4.63 | .484 |
| 2: after 4-4; mate 2-1 and 6-2 | +16.67 | +1.39 | +15.28 | .390 |
| 3: after 1-1; mate 6-0; before 0-0 | +18.98 | +11.11 | +8.33 | .274 |
| 4: mate 6-2; before 5-5 | +11.27 | +7.32 | +6.61 | .052 |
| 5: mate 2-1; before 6-4 | +12.61 | +3.47 | +4.91 | .490 |
| 6: after 5-5 and 6-3; mate 2-0 | +15.97 | no matches | +3.61 | .898 |
| 7: mate 2-1; before 4-3 and 6-4 | +15.97 | +13.89 | +4.44 | .857 |

The primary candidate was **rank 0**, fixed before validation. Its new-hand
association is weak: only +1.67 points; unadjusted one-sided permutation p=.376.
It is not evidence of a generally reliable learned threat.

Rank 4 is the interesting secondary hypothesis. New-hand matches span 25 hidden
completions, 225 games, and all ten reserved source deals. It has 177 failures
(78.67%) against a matched-cell baseline of 72.06%. Same-hand validation had
22 matching completions/198 games, 80.30% failures against 72.98% baseline.
Its familywise adjusted permutation values are .004 on same-hand validation and
.052 on new hands. The latter is unresolved preliminary evidence, not a threshold
crossing to promote or a probability that the hypothesis is true.

The permutation audit uses 999 draws, moving trial outcome vectors together
across declarations, separately within each bidder hand. Cell difficulty and
the shared-declaration dependence are preserved. Max-statistic adjustment covers
the eight frozen hypotheses within each reserved set. No universal statistical
guarantee, causal interpretation, or out-of-domain validation is claimed.

## Post-hoc ablation: a useful example of the proposed loop

After seeing the complete reserved results, we evaluated **every** immediate
one-literal deletion of all eight candidates: 18 distinct queries. These
[post-hoc results](posthoc-ablations.json) are descriptive, not another held-out
confirmation or a basis for replacing the frozen primary candidate.

For rank 4:

| Description | Discovery excess | Same-hand excess | New-hand excess |
|---|---:|---:|---:|
| mate 6-2 AND before 5-5 | +11.27 | +7.32 | +6.61 |
| mate 6-2 alone | +4.39 | +2.46 | -0.61 |
| before 5-5 alone | +4.96 | +4.12 | +5.78 |

The simpler [before-double-five Scheme](posthoc-before-double-five.scheme)
retains an association; the 6-2 condition alone does not transfer. On new hands,
the simpler query matches 75 completions/675 games, with 535 failures (79.26%)
against a matched-cell baseline of 73.48%. This does **not** prove the pair adds
nothing, nor that possession of 5-5 caused these losses. It supplies an explicit,
testable candidate and an apparently unnecessary detail to challenge further.

No one supplied "opponent has double-five" as a tactical rule. The constructor
found retained ownership facts among actual losing worlds. Humans still chose
the primitive vocabulary, outcome target, and search bounds; this is bounded
discovery, not assumption-free learning.

## Verification and durable evidence

- Four Python checks cover cell-difficulty confounding, joint declaration
  permutations, genuine seed-world ablations, and held-out selection isolation.
- A Rust input check refuses impossible/overlapping opening worlds.
- The real Scheme engine agreed with the cached literal matcher on **115,200**
  primary/seed memberships and **129,600** post-hoc memberships: zero mismatches.
- All 7,200 original game receipts passed full independent play/score/request
  validation. This does not elevate outcome associations into mechanics proofs.
- Frozen rows, full compressed receipts, producers, fitted hypotheses, seed and
  learned Schemes, result identities, and a probe-source/binary snapshot live at
  `/Users/jason/data/texas-42/kiln-played-v1/threat-probe-v1/` (~35.5 MB), outside
  the checkout and inside the campaign's existing backup scope.

Primary result ID:
`e7afedea94dbfa9093dcfd5f99918f86b3b9ec1dba52027b97fefeda1ab7037a`.
Post-hoc result ID:
`a2361db76a7dc14196f9f97ac30ede0b810c17c1fcb73142839272d26694a799`.

Reproduce with Python 3.10+ (3.12 is already installed on this Mac):

```sh
cargo build --release --manifest-path walt/Cargo.toml -p walt --bin scheme_worlds
python3.12 experiments/kiln/threat_probe.py /Users/jason/data/texas-42/kiln-played-v1 --output /path/to/new-probe
python3.12 experiments/kiln/threat_probe.py /Users/jason/data/texas-42/kiln-played-v1 --output /path/to/new-probe --ablate
```

Completed primary results are not overwritten. New production games cannot
alter this first-eight protocol. Candidate choices and membership/outcome
statistics reproduce; compiler-dependent binary identities may differ.

## Next falsifiable question

Freeze the simpler candidates now and test them on additional worlds without
retuning. To extend coverage to cells production screened at eight, a fresh
uniformly scheduled audit batch is needed; using only surviving cells would
change the population. Test relational lifting against literal-only ablation
on a **new** discovery/validation split. Transform "this tile" into structural
roles relative to declaration and own hand, with the same facts-to-query
provenance. That is additional grammar/search work, not implemented here.

Only after replication should we cross fixed hidden worlds with policy seeds
and compare continuations to ask whether a candidate is a threat mechanism.
Then evaluate whether sampling with it saves work at equal decision quality.
This probe alone establishes neither causal threats nor cheaper/stronger play.
