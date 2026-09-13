id: [[partnership-strength-question]]
opened: 2026-09-13 (the program's stated open question since 2026-09-06)

## What

Does a cost-matched partner model add playing strength over the level-1
default under a wall clock? The partnership program
(`experiments/partnership/`, 2026-09-06/07) built L2 Partner (partner
modeled at L1, opponents at L0) and measured no gain: L2 Partner vs L1
14 / 14 / 72 on 100 shared deals (400 games,
`campaigns/default-partner-battery/RESULTS.md`) at 1.127 s vs 0.228 s per
move — five times the cost; the earlier candidate-vs-phone panels were
unfavorable (15 / 23 / 156 random, `campaigns/random-420600-699/RESULTS.md`;
21 / 27 / 152 fixed-hand, `campaigns/worlds-520600-699/RESULTS.md`). The
named uncertainty is the **420602 reversal**: on seed 420602 (S3 declaring
ones) the phone holds the declarer to 4 and 23 (set) while the candidate
lets it make 36 — a defensive regression — and `REPORT.md` cannot say
whether that reflects two-world inner modeling / sampling noise or a stable
mismatch between the modeled partner and the executing policy. The gym's
composed exam points the other way in miniature (L2 Partner 26/30 vs L1
24/30, mean regret 0.4673 vs 0.8007 pp,
`walt/gym/PARTNERSHIP-COMPOSITION.md`) but is diagnostic, not strength.
Related and adverse: level 2 hoards the 6-4 MORE than level 1 on G1 (branch
`walt-g1-l2`), and at an exact tie no partner model can reach the decision
through pmake at all ([[gran-anchor-reconstruction]]).

## Done when

A matched-cost comparison (L1 given the L2 Partner's wall budget as extra
worlds, vs L2 Partner) on a fresh seed panel with a pre-declared stopping
rule states a sign or an honest null; the 420602 reversal is reproduced and
attributed (sampling noise vs model mismatch) by a controlled rerun with
matched inner draw streams; the answer is recorded in
`experiments/partnership/SESSION-STATUS.md` and
`wiki/walt-partnership-program.md` §9. No default changes on this card; a
default swap is Jason's word.

## Links

`experiments/partnership/REPORT.md` ("What remains"),
`experiments/partnership/PLAYERS.md`,
`experiments/partnership/campaigns/default-partner-battery/STRENGTH-ASSESSMENT.md`,
`wiki/walt-partnership-program.md`, [[inner-voids-default]],
[[plunge-walt-sync]].
