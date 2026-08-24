# Lineage — the mk5 project and the wall

[Home](Home.md) · owns: why texas-42 exists. **Context only** — no code, data, or
definitions cross from the old project into this one. The old corpus lives at
`~/code/mk5-main/wiki` (start at `topics/the-wall.md`).

## The short version

Jason has built 42 players since long before this repo (the dream dates to age
12). The mature prior project (mk5, January–July 2026) produced:

- **The champion**: E[Q] n=10 — expected Q over ten sampled worlds consistent
  with public history, evaluated per move, average-then-max. Undefeated at pure
  play against every learned challenger (zeb, burl, gus, jud's play half).
- **The wall** (the project's central question): *something that can reason
  with E[Q], do better, and have a plan that actually succeeds.* Learned
  approaches hit it repeatedly; the diagnosis matured into: the champion's
  distributions are "candlewax — gorgeous, honest, and mute on what to want."
- **The crack**: jud v0's bidding beat the champion on marks (first learned
  component to do so); **walt**, an exact endgame info-set solver (≤4 tiles),
  beat the greedy play head +3.00 marks/game and is the first artifact that
  provably *has a plan and cashes it*. The wall is an information-set problem.

## How texas-42 answers it

- The **evening player v0** (rob/crates/player) is the champion's law — one
  common continuation policy, average-then-max, never per-world re-optimization
  — rebuilt on *exact* fiber sampling (zero impossible worlds, by CELL-05).
- The foundation's core objects (exact support normal form, support ≠ belief ≠
  reachability, the 90-world witness, the reduced viewer kernel and its
  adjudicated collapse) are precisely the machinery walt's success says is
  needed: exact information-set state, scaled past the endgame.
- The old project's standard — paired-marks evidence, receipts, "state the
  question in question form" — is continued here as claim-tier discipline.

The wall's winning condition, restated for this repo: a texas-42 artifact
clears it only by beating the E[Q]-family champion in paired marks *for a
demonstrated strategic reason*. Everything else is instrument, not goal.

*Status note (2026-08-24, exploratory tier).* On 2026-08-17 the walt
scenario-player seat beat the E[Q] champion in paired marks under the dropped-30
protocol, with a legible strategic signature — it loses trick points and wins
marks, the shape its pmake objective predicts ([walt-seat-play](walt-seat-play.md)).
That is an arena outcome about play, at the exploratory tier, against a modeled
field; whether it constitutes the *demonstrated strategic reason* this condition
demands — a plan held and cashed, not a scoreline — is deliberately not ruled
here. The obligations ledger in `walt/SCENARIO-PLAYER.md` is the path from the
scoreline to the demonstration.
