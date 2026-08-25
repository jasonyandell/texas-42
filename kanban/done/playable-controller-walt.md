id: [[playable-controller-walt]]
opened: 2026-08-24

## What

Make the §16.4 evidence/decision-controller walt an ACTING player behind
a stable API so plunge can consume it (CE thread). Library action policy
`solver::act` (settled winner played; honest tie → level-1 rank among
the tied set; Unresolved at the cap → level-1 rank among δ-survivors;
fallbacks recorded as ordering choices outside the correctness boundary,
never as settlements), `controller_bridge` speaking the walt_bridge line
protocol, controller seating in webtable/playtable (`ctrl`, `cap=N`,
interactive default 128), and the O27 deal/belief RNG split in
playout/playtable/webtable. Register: `walt/CONTROLLER-PLAYER.md`.
The playout PiKey/banked-totals copy (§3.4) stays filed, untouched.

## Done when

Route-gated fixture tests green; walt `ci/check.sh` green; draft PR to
main open; plunge can seat `controller_bridge` with zero external-side
changes.

## Links

[[panel-response-audits]], `walt/CONTROLLER-PLAYER.md`,
`walt/probes/shadow/README.md`

## Closed 2026-08-24

Merged as **PR #37**, main **`23ba1c2`** (CE thread); **central gate
green** (`walt/ci/check.sh`). Landed: `solver::act` with the six-label
route alphabet (exactly three settled; `settled` false on every
fallback), `bin/controller_bridge` on the unchanged walt_bridge line
protocol, `ctrl [cap=N]` seats in webtable/playtable (interactive cap
128 as a think-time budget), route-gated fixture tests in
`tests/solver_act.rs`, and the O27 deal/belief stream separation in
playout/playtable/webtable. The §3.4 playout PiKey/banked-totals copy
stays filed, untouched. Register `walt/CONTROLLER-PLAYER.md` owns the
surface; wiki synced on `wiki/walt-calculated-evidence.md` (era
section), `wiki/walt-instruments.md` (surfaces) and
`wiki/walt-seat-play.md` (the `ctrl` seats).
**No strength claim**: no arena run, no conformance gate, and the old
player remains the default per CE-A7/§20.16.
