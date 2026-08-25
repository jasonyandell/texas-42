# The playable controller walt — `solver::act` and its surfaces

**EXPLORATORY tier; CE thread** (CE = sampling depth — every result read
off these surfaces is labeled CE-thread). Sits below every evidentiary
tier and is cited by nothing above it. Estimates, never receipts; not a
P-A21 statement. No strength claim is made or implied: the old player
remains the default everywhere, and arena/conformance gates remain the
bar for any default change. Card: [[playable-controller-walt]].

Parent mathematics: `walt/math/calculated_evidence_v0.1.md` §16.4
(decision controller), §5/§6 (risk ledger and run allocation), §1.5
(caps are resource limits), §12 (frozen policies); rulings CE-A1..A8
(`walt/CENSUS-RULINGS.md`). The read-only precursor is the shadow
instrument (`walt/probes/shadow/README.md`) — this delivery makes that
controller ACT behind a stable API.

## What it is

`walt/walt/src/solver/act.rs` — one library entry point:

```
act(&DrivenState, &ActConfig, run_scope, d, δ_run) -> ActDecision
```

Per decision it builds one frozen level-1 continuation policy per legal
root action (`ActionRule::PinnedThenLevel1`, declared schedule
`n_outer_frozen`/`n0_frozen` — identity fields, CE-A5), runs the §16.4
controller under a run-scoped strict risk plan
(`δ_d = δ_run/(d(d+1))`, δ_run = 1/100 per hand by default), and then
applies THE ACTION POLICY:

| controller result | route label | tile chosen by | boundary |
|---|---|---|---|
| one legal tile | `forced` | the rules | inside (trivially) |
| `ExactFrozenSet`, unique max | `exact-winner` | the exact winner | inside |
| `DeltaSettled` | `delta-settled` | the δ-settled winner | inside |
| `ExactFrozenSet`, `winner:null` | `exact-tie-level1` | live `level1_evaluate` rank among the TIED maxima | **outside** |
| `Unresolved` at the cap | `unresolved-level1` | live `level1_evaluate` rank among the δ-SURVIVORS | **outside** |
| `EpsilonEquivalent` (ε-mode only; not produced by the strict plan used here) | `epsilon-level1` | live rank among the survivors | **outside** |

Stated plainly: the δ-safe eliminations are inside the correctness
boundary — a candidate is removed only by a settled directed edge at the
declared risk. The level-1 ranking among survivors or exact ties is a
scheduling/ordering choice OUTSIDE the correctness boundary (the
W7/filtration license: predictable ordering heuristics affect cost,
never truth). A fallback is never presented as a settled winner: every
`ActDecision` carries `route` (and `ActRoute::settled()` is `false` on
every fallback), and every surface logs which route chose the tile.

Routing before the policy: fibers ≤ `exact_cap` (default 2000) run the
exact frozen-set endpoint directly (`preroute` — exact spends no risk,
§6.1); larger fibers run the adaptive controller (`sampled`, or
`escalated` when the §11.3 switch fires). The controller record
(`SetEvaluation`) rides along in the decision as a derived view of the
evidence stream, never a second authority.

Decision ordinal `d`: surfaces use `d = plies played + 1`, stateless
from the public record. Any injective ordinal assignment is conservative
— `Σ δ_run/(d(d+1))` over any subset of ordinals telescopes under
δ_run (gated in `tests/solver_act.rs`).

Fallback determinism: the fallback's discovery stream is
`ACT_FALLBACK_SEED ^ mix(own remaining hand) ^ record_hash(record)` —
the walt_bridge information-consistent per-decision pattern (audited
CLEAN), a domain constant distinct from every other surface seed.

## Knobs (`ActConfig`)

- `world_cap` — controller cap in raw worlds. A THINK-TIME BUDGET:
  reaching it produces honest `Unresolved` → fallback, never a wrong
  settlement (§1.5). Interactive default **128**
  (`ActConfig::interactive()`); batch default **512**
  (`ActConfig::full()`, the shadow bin's epoch). Trick-1/2 decisions at
  cap 512 cost minutes; at 128 expect ~10 s-scale trick-1 decisions.
- `exact_cap` (2000) — preroute-exact fiber ceiling.
- `n_outer_frozen`/`n0_frozen` (8/2) — the frozen candidates' declared
  schedule; changing either is a new PolicyId and a new epoch.
- `fallback_n_outer`/`fallback_n0` (200/8) — the live level-1 ordering's
  sample counts.

## Surfaces (thin consumers)

- **`controller_bridge`** (`src/bin/controller_bridge.rs`) — the bridge
  plunge/mk5 can call: SAME line protocol as `walt_bridge.rs`
  (`rob:<path>` adapter, zero external-side changes). Play decisions run
  `solver::act`; the `declare` request keeps the walt_bridge level-1
  argmax-P(make 30) policy (trump naming is not a controller decision).
  Knobs argv/env (`WALT_CTRL_WORLD_CAP`, `WALT_CTRL_EXACT_CAP`,
  `WALT_N_OUTER`, `WALT_N0`, `WALT_N_DECLARE`, `WALT_PER_MOVE`,
  `WALT_CTRL_N_OUTER_FROZEN`, `WALT_CTRL_N0_FROZEN`,
  `WALT_DECLARE_FULL`). `WALT_CTRL_LOG=<base>` appends one JSONL record
  per decision (per-PID suffix) with route, settled flag, tag, consumed,
  among-set, and fallback opts in basis points — record-grade output.
- **`webtable`** — `webtable <port> [n_outer] [n0] [seed] [seat] ctrl
  [cap=N]`: seats the controller at every AI seat for the PLAY phase
  (auction and trump pricing stay level-1); each play's message carries
  the route. Browser UI unchanged.
- **`playtable`** — `playtable [seat] [n_outer] [n0] [seed] [fresh] ctrl
  [cap=N]`: terminal table, same seating, routes printed per play.

## O27 fix (in this delivery)

`playout`, `playtable`, and `webtable` previously shared ONE RNG across
deal and belief sampling (§12.3 audit finding O27). All three now
domain-separate: the deal stream deals and does nothing else; every
level-1 evaluation derives a per-decision stream from
(domain constant, own DEALT hand, record hash) — the walt_bridge
pattern. Session output is therefore record-grade: no decision's sample
depends on how many decisions preceded it. The separate playout
`PiKey`/banked-totals copy defect (§3.4) is **NOT** fixed here — it
stays filed.

## Gates

`walt/walt/tests/solver_act.rs` + unit tests in `solver/act.rs`: every
`SetResult` variant maps to exactly one route (no wildcard arm — a new
result kind is a compile error); forced/exact/tie/unresolved routes each
fixture-gated on frozen receipt roots (hand 0 trick 6, the exp5-pinned
fiber 90); the capped run is provably `Unresolved` (four worlds cannot
cross `T_edge ≥ 400`) and the fallback tile equals the live ordering's
choice; determinism per information state; the route alphabet is six
distinct labels with exactly three settled; the 28-ply ordinal
allocation stays under δ_run. Run via `walt` CI (`ci/check.sh`).
