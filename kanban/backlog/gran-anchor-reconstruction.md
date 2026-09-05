id: [[gran-anchor-reconstruction]]
opened: 2026-08-24

## What

Reconstruct the two Plunge "Gran 6-4" hands as named, reproducible
anchor positions: recover the game seeds and full public records from
the plunge side (the screenshots stay discovery artifacts, per
`walt/math/targeted_level2_field_stability_v0.1.md` §1.4 and L2-A6).
The two anchors: the bid-30 sixes hand ended 25–17 (6-4 retained; the
40-vs-160-world near-tie flip recorded on wiki/walt-seat-play.md) and
the bid-31 sixes hand ended 36–0 (6-4 revealed early; the all-100%
trick-4 saturation review). These feed the G1–G4 anchor experiments
(§11 of the parent) once the field-swap build exists.

## Update 2026-08-24 evening — artifacts in hand; seeds NOT required

Jason delivered the three screenshots (no seed capture existed when
they were taken; his tooling captures seeds now, so future anchors are
exact by construction). Archived per the data-home rule at
`~/data/texas-42/gran-anchors-2026-08-24/` with `MANIFEST.sha256`:

- `gran-failed-hand-trick1-40w.png` (`d21b0d0c…`) — bid 30 sixes,
  25–17; trick-1 panel: 40 worlds, 6-2 at 90% (walt's pick, played)
  vs 6-4 at 80%.
- `gran-failed-hand-trick3-160w.png` (`f528266a…`) — same hand;
  trick-3 panel: 160 worlds, 77% pick vs 72% played (6-4).
- `gran-made-hand-trick4-saturation.png` (`3262da08…`) — bid 31
  sixes, 36–0; trick-4 panel: four options all 100% on 160 worlds.

**Reconstruction path (no seed needed):** the "How it went" grid shows
every seat's tile for every trick — that IS the complete deal. So:
(1) transcribe both grids tile-by-tile; (2) validate the transcription
mechanically with the rules engine — 28 distinct tiles, 7 per seat,
legal follows under sixes, per-trick winners and +1/+6/+11 counts and
the final 25–17 / 36–0 must all reproduce exactly (any misread pip
fails validation and localizes itself); (3) commit the validated
records as the G1–G3 root identities. The 40/160 review panels are
then reproducible too: the bridge's belief sampling is a pure function
of the information state (per-decision seeds from own hand + record
hash), no wall-clock RNG anywhere.

## Done when

Exact root/fiber identities for G1 (failed-hand trick-1), G2
(successful-hand early-reveal decision point), and G3 (trick-4
saturation root) are committed with seed provenance, and the intake
companion's "Gran-anchor gap" note is updated to point at them. G4
(mechanism adjudication) belongs to the field-swap build, not this
card. Related: [[level2-field-swap-probe]], [[plunge-walt-sync]].

## Update 2026-09-04 — G1 reconstructed, validated and PLAYED; G2/G3 partial

Anchors live at `walt/probes/gran/` with the probe record in that
directory's `README.md` (EXPLORATORY tier). Runner: the new additive bin
`walt/walt/src/bin/granrun.rs` — a VARIANT surface that touches no
existing one (`waking_bridge`, `walt_bridge`, `controller_bridge`,
`solver::act`'s policy and the live default player are all unmodified).

**G1 (failed hand, bid 30 sixes, 25-17) — DONE.** Transcribed tile by
tile from both failed-hand screenshots (two independent reads of the same
grid) and validated mechanically: `granrun validate` runs
`rules::replay::replay_hand`, which trusts only the tiles and who played
them and re-derives the deal, every follow's legality under sixes, all
seven winners, all seven trick point values, the 25-17 totals and the set
verdict. Nothing was ambiguous; the 28-tile partition closed exactly.
Seat map Y=S0 (Jason, declarer), E=S1, G=S2 (Gran, the walt seat), R=S3.
NOT recovered: shaker, auction, prior mark ledger (fixture placeholders,
flagged in the file).

**G2/G3 (made hand, bid 31 sixes, 36-0) — COMMITTED, PARTIAL.** The grid
shows SIX tricks, not seven: the app ended the hand once 36 beat the bid
of 31. So 24 of 28 tiles are recorded and the DEAL IS NOT RECOVERED.
`granrun validate-partial` re-derives the whole prefix (every follow,
winner, trick value, the 36-0 and the made verdict) and enumerates the
residual honestly: unplayed tiles are 4-1 4-4 5-2 5-3, one per seat; the
trick-4 review panel pins S2's as the 5-2, so S2's full hand is known;
the assignment of {4-1, 4-4, 5-3} to S0/S1/S3 is 6-way ambiguous and
mechanically undecidable from this record (none is a six or a blank, and
every prefix void was on sixes or blanks). The G2 and G3 ROOTS are still
fully determined as INFORMATION SETS — a decision at S2 needs S2's hand
plus the public record, both known in full. What is unavailable is a
driven whole-hand run and a `replay_hand` validation.

**First waking-seat run on the real hand.** Replay mode (waking seat at
S2, other three seats on the record) at the `probes/waking` epoch: the
seat plays the **6-4** at trick 1 where the live level-1 seat played the
6-2, agreement with the record 6/7, whole run 25.04 s. But the wake did
NOT fire there — sigma0 already picks 6-4, the trick-1 fiber is 46.5 M so
the check took the sampled route and returned honestly open. The trick-1
flip is a baseline/epoch difference, not the waking mechanism. The one
real wake landed at trick 5 (fiber 300, exact route) and moved the play
onto 4-2, which is what the human game played. Driven mode (all four
chairs from the G1 deal): 26-16, still set, zero wakes, 166.11 s.

## Done when — status

G1 is committed and validated. G2/G3 are committed and validated over the
prefix, with the residual ambiguity stated rather than guessed; they do
NOT meet "exact root/fiber identity" for the full deal, only for the
information sets. Seed provenance remains unavailable for all three (no
seed capture existed when the screenshots were taken) — the card's own
"no seed needed" path is what was used. The intake companion's
"Gran-anchor gap" note has NOT yet been repointed at these files.
CARD STAYS OPEN on those two items.
