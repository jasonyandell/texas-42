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
