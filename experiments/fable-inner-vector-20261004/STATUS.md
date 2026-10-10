# Fable inner-vector distillation — STATUS

- **Accepted:** 2026-10-04, Claude Fable 5.1 (`claude-fable-5-1`), same session as the
  completed diagnosis (`../fable-tiny-net-20261004`, immutable) and the design phase
  (`../fable-walt-vector-20261004`). Jason: "yes please try that and start training
  again." Jeb's constraints from FABLE-HANDOFF.txt stay in force: every process tree
  under the 295 s wrapper, resumable batches, no cloud/paid/credentials/push/export,
  prior experiments immutable. Exploratory tier.
- **Hypothesis under test:** the corrected teacher boundary (Walt's inner level-0
  mind: own future committed per public node over a small bundle, others random)
  versus the pilots' V0 (everyone uniform after the root), on full legal-action
  vectors with a separate deterministic selector. Not "more vector information":
  the pilots already regressed full vectors.

## Source map: verified teacher contract (pinned Rust, `cb1ef3b2`, identical at HEAD)

The production phone (`walt-table-v2`, WASM `40d7a1ee…`) evaluates its root with
`partnership_wire` mode `baseline`, `n 40 / n0 8 / n1 2 / inner_belief 0 / selection 0`.
Every hidden seat inside those 40 worlds plays the **level-0 modeled mind**,
`Solver::pi(0, key, seat, hand, legal)` (`walt/walt/src/solver/mod.rs:1221`), public
through `Solver::modeled_choice(0, …)` (`mod.rs:1571`). Its generic path
(`mod.rs`, the tail of `pi`) is exactly:

```
rng    = SplitMix64(INNER_SEED ^ mix(seat) ^ mix(remaining_hand) ^ record_hash(public key))
sizes  = contract.sizes(key, boundary)                      # remaining hand sizes per seat
worlds = InnerBelief::Voidless.sample(dcl, seat, hand, key, sizes, n0 = 8, rng)
seeds  = 8 × rng.next_u64()                                 # one dice seed per world
values = Solver::new(sh, seat, hand, maximize = seat on T1, worlds, seeds, Field::Dice)
            .with_ordering(CaptureFirst).action_values(root, legal)   # Vec<(tile, k/8)>
choice = selection::Rule::Fixed → best_of(values, maximize)          # lowest tile on exact ties
```

Contract, stated for the label record:

| Coordinate | Pinned value | Note |
|---|---|---|
| Input | seat, **remaining** own hand mask, public key (played, leader, table plays, banked totals), legal mask | no request seed enters; the inner mind is a deterministic function of its information state |
| Perspective | `value = wins / 8` = fraction of the 8 worlds in which **T1 (declaring partnership) makes the bid** | declarers maximize, defenders minimize; the acting-side success is `v` or `1 − v` |
| Granularity | exact rationals with denominator 8 | ties are exact-count ties, common by construction |
| Own future | bundle recursion: one tile per public node over the worlds still alive | no per-world own choice (not fused), but bundle-coupled: optimistic as worlds thin out |
| Others | `Field::Dice`: each hidden seat plays a seeded uniform legal choice per world | no modeled mind below level 0 |
| Belief | `Voidless`: shuffle the unseen tiles into remaining hand sizes **ignoring public voids** | a superset of the lawful support; still information-lawful (own hand + public record only), but not support-conditioned and not a posterior |
| Selector | `best_of`, ascending tile on exact ties | the upper level consumes only this tile |
| Production fast paths | `cpu-speedups` (`stack-dice`, `bounded-choice`, `bypass-l0-cache`) compute the same choice | differential check in this phase: seam `best_of(action_values)` vs `modeled_choice(0)` on independent positions, both feature sets |

Deviations I introduce, named: (1) the seam returns the whole `values` vector that
production discards after selecting; (2) a declared **reference estimator** that
mixes an explicit extra seed into the RNG (`K` independent 8-world bundles) exists
for evaluation of noise, ties and gaps only; the primary label is the exact
seedless production vector. Anything else that differs from the table above is a
bug, not a design choice.

What is **not** being claimed: no posterior, no equivalence of the standalone C port
(`walt.c`, outer 30, no `walt_bridge` rotation, own sampler) with the phone, no
outer-level substitution result. Inner selection fidelity and paired outer-regret
substitution are different tests; this phase measures the first and leaves the
second explicitly undone unless a gate warrants.

## Log

- 2026-10-04 — accepted; seam located; contract written above before any build.
- 18.6 s / 18.1 s capped builds of the seam (production features, reference paths). Differential on 1,024 independent positions: 1,024/1,024 production agreement, 1,024/1,024 cross-build identical vectors, 22 µs median per call (`results/seam-check.json`).
- Plan frozen (`plan.json`); 7,168 fresh deals, zero collisions; 19,752 train+validation roots, 1,521 test roots; labels 14.9 seam-seconds total, production agreement on every labeled root.
- Eight arms trained (2.8 to 6.6 s each); frozen 18:13:56Z; test mined/labeled from 18:14:19Z; evaluated; audit green (`checks/audit-result.json`).
- **First result** (`REPORT.md`): inner-h256 beats v0-h256 by .0062 regret [−.0111, −.0014] on the same inner reference (boundary effect, resolved at ordinary capacity; tiny pair not resolved). The exact 8-world inner teacher is itself noisy (74% self-agreement with its 16-bundle majority; regret .0216 vs the 128-world mean). Capacity and loss form not established; BCE arms are calibrated (ECE ≤ .026). G3 (outer substitution) not run.
- Commits: 1315ba86 (pre-test state), d0f00775 (report). No background jobs.
- 2026-10-04 — **Jason's steering acknowledged** (`CLAUDE.md`): the goal is compressing the pinned lower-Walt computation at native k/8 resolution, full numerical vector at lower cost; ties and regret are diagnostics. Report headline rewritten against that goal without touching plan/data/weights/test. Native-fidelity reading: raw inner nets match scale (bias < .005) and sit 1.7 to 1.8 output steps from exact k/8 (teacher's own sampling spread about 1.3 steps); V0 nets are 2.4 steps off and biased −.17. **Cost not yet demonstrated cheaper:** network-only 5.33 µs, Python decision 116.9 µs, Rust seam about 22 µs in-process (36.2 µs pipe); compiled in-recursion path unmeasured; nested substitution undone. Exact native k/8 is the target; seed-averaged labels are an optional distinct experiment, not a pending decision.
