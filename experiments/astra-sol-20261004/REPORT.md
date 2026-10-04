# Walt: first jointly challenged research result and exact phone baseline

2026-10-04. **EXPLORATORY RESEARCH.** The abstract statements identified below
are Lean kernel checked; their application to the complete Walt implementation
is conditional. Arena observations are descriptive measurements, never theorem
or production-strength claims.

## Useful results first

1. **Safe approximation needs action control, not just accurate values inside
   a modeled mind.** Sol's finite counterexample has lower-mind action-value
   error and regret `1/(2n+1)`, arbitrarily small, yet replacing its chosen
   policy causes the outer seat's conditional success to fall from 1 to 0.
   Both modeled policies are lawful constants. This directly challenges a
   hoped-for cheap higher-rung player justified only by teacher value error.
2. **There is a precise conditional way to preserve the chosen action.** On
   the same weighted scenario bundle, bound payoff disagreement for every
   relevant lawful focal policy. Such bounds transport to the action maxima.
   If the incumbent's old count exceeds a competitor's by more than the sum
   of their error bounds, its strict preference survives. Equality preserves
   weak preference only; least-tile tie-breaking requires care. The lead
   formalized these components; Sol independently checked and reran them.
3. **The reusable baseline is the exact production phone WASM, not the old
   archive or a native proxy.** Four deals, four physical rotations each, and
   both partnership assignments completed 32 full games / 896 independently
   audited moves in 19.461 seconds. The first candidate simply disables the
   partner review. It produced 0 favorable / 0 unfavorable / 16 tied paired
   make outcomes. With only four source-deal clusters, the conservative 95%
   interval is `[-1,1]`; no strength or meaningful speed improvement is shown.

The demand-tree counterexample also shows that perfect memoization alone does
not establish useful linear cost in model level: distinct keys can still
produce `2^k` leaves. This is an abstract demand-count obstruction, **not** a
lower-bound theorem for every implementation of fixed finite Texas 42.

## Source identity and team

Shared worktree: `/Users/jason/Documents/Codex/2026-10-04/task/research-worktree`.
Its separate Git metadata is in sibling `source.git`, initially cloned with
read-only object sharing from the existing CPU checkout. No source checkout,
production branch, deployed artifact, or prepared-hand campaign was edited.
The worktree began at `3cf2536d`, then moved to the production player's exact
source after resolving the user's supplied Plunge commit.

| Input | Pinned identity |
|---|---|
| Supplied last-verified Plunge production | `a0d9fa806166b0e63fe016bb49d93f91f47b1af8` |
| Phone manifest's Rust source; final research base | `cb1ef3b23072e4c268f31f625f2b61d5facc1929` |
| Phone WASM SHA-256 | `40d7a1eea658627f7bab37d3c1888ad3bf00216fb23a5d2ceeb7d2d1f4219119` |
| Release source-v2 hash, independently recomputed over 202 files | `5bedf14af0e5a5ef354acca4765f67843469d074d163e8e4786631aa23436e32` |
| Local Plunge checkout inspected, older than supplied production | `aff966d65028e3a7813afdb6111562d3272f2556` |
| CPU-speedups checkout | `3cf2536dcbd6636bfff8ab814858ee00e8dc8b5e` |
| Located current registered texas-42 checkout | `/Users/jason/code/texas-42`, `9d6a5a2e12958183d49d0cd95c8617c4ef7a12e4` |
| Prior generator optimization, infrastructure only | `06bed7532de2015412e4808dae24a5cf6d95d38f` |

The production source and WASM hashes match the published manifest, and the
WASM, manifest, native profile, and settings Git blobs match the supplied
production commit's tree. Receipt: `results/production-identity/`. This pins
the user's last-verified release; we did not inspect the currently served
website or a physical phone. The older local WASM's one-game pilot is retained
under `results/smoke*` and explicitly excluded from production results.

The lead was delegated as **Astra**. The collaboration tool successfully
launched a collaborator explicitly configured as **gpt-6.1-sol**, high reasoning,
with a fresh context. Sol constructed its own proof and witnesses, challenged
the lead's proof, and independently audited the harness and complete pilot.
The lead independently reviewed and reran Sol's proof. The lead runtime does
not expose an independent model-identity attestation; do not infer one from
the role name alone. No separate Sol task is needed.

Read instructions included CLAUDE.md, QUICKSTART.md, wiki/Home.md, walt/MAP.md,
SCENARIO-PLAYER.md, existing partnership/response-ladder machinery, local
Codex memory pointers, and the prior win-endgame thread's accessible result.
Local checkouts had no `.agents/skills`. The production commit does contain
`plunge-generate-hands`; its SKILL.md and workflow were retrieved and preserved.
Its scope is prepared-hand generation, which this task does not perform.

## What the pinned code solves

The conceptual ladder in [issue 99](https://github.com/jasonyandell/texas-42/issues/99)
is a useful definition: L0 responds to Dice, and Lk responds to lower-rung
modeled seats. It is distinct from the deployed budgeted wrapper and from the
fixed-field focal-horizon hierarchy. This distinction follows the current code,
not historical labels saying “live L1.”

For straight 42, let `S` be a fixed finite multiset of sampled hidden deals and
frozen random tapes, `F` a declared field of modeled non-focal policies, and
`P_a` the family of lawful focal continuation policies starting with `a`.
For positive total scenario weight, the intended sampled response objective,
in focal-team success orientation, is

`V_F(a) = max[p in P_a] sum[s in S] w_s * 1{focal team succeeds under (p,F,s)} / sum w_s`.

Physical world state is available to the simulator; a decision policy receives
only its own holding, public information, and deal-independent randomness.
Focal choices are shared across indistinguishable scenarios. Modeled actions
partition scenarios into public-observation buckets, which are summed; focal
nodes optimize once over their current bucket. A modeled seat samples again
from its own chair. Reusing the outer seat's private sample as that modeled
seat's belief is not an authorized shortcut.

The ordinary completed L1 comparison models all other seats at L0, including
the partner. `PartnerOnly` raises only the partner to L1, whereas `AllLevel1`
raises all three others. Those are different fields, not equivalent levels.
The focal seat optimizes alone; neither field is a joint optimization over
partners' private strategies, nor a minimax guarantee against arbitrary legal
defense. Code: `walt/walt/src/solver/partnership.rs`, `solver/mod.rs::pi`.

Outer sampling obeys own hand, capacities, and observed mechanical voids.
The deployed modeled minds use `InnerBelief::Voidless`: they deliberately
ignore historical voids when drawing their own worlds. An optional counted
void-conditioned strategy exists but is not the deployed default. Neither
strategy fits policy likelihoods to the actual observed prefix. Within a
sampled future search, a modeled public action does refine the alive scenario
set. Support, sampled belief, and a calibrated real-game posterior differ.
Code: `solver/inner_belief.rs`, `partnership.rs::sample_belief_bounded`.

“Exact” here means a completed integer/rational comparison under a declared
finite bundle, represented information state, fixed field, and selection rule.
It does not remove sampling error, validate beliefs about real players, or
prove equilibrium strength. The implementation's complete cache-key and
game-to-model correspondence obligations are not discharged by our new Lean
files. A perfect-information oracle may supply an upper relaxation; its
world-dependent optimal choices cannot be used as a lawful imperfect-information
policy. A root certificate, a complete action vector, an executable contingent
policy, and actual playing strength are different claims.

The production phone default is `native-partner`, with `thinkDeeper=false`:

- On the opening bidder play: 160 requested outer worlds, 20 seconds, review off.
- Otherwise: 40 worlds, 14 seconds, partner review on; ordinary inner budget 8.
- It publishes a legal reserve immediately, tries a complete 8/2 L1 fallback,
  then complete ordinary 40/8, then larger requested comparison when applicable.
  Only a completed comparison replaces the retained move.
- The optional partner check is a separate, bounded paired continuation review,
  at most 500 ms. Its continuations use deployed L1; a sampled support prefix is
  explicitly heuristic, not a calibrated confidence statement. It is not the
  exact `PartnerOnly` L2 solve. A census is a distinct result.
- Host interruption retains a completed checkpoint. Thus actual behavior can
  depend on the time budget/device, even though completed modeled policies
  use fixed seeds and do not silently choose a timed-out inner approximation.

References: pinned `reference/production-phone/native.ts::livePlayerCall`,
`store.ts::DEFAULT_SETTINGS`, `client.ts`; source `walt/walt-player/src/lib.rs`,
`walt/walt/src/policy_search/partner_rollout.rs`.

Production also has a Nel-O route: the bidder's partner is inactive but its
seven cards remain hidden; three active players per trick, first bidder-won
trick is failure, avoiding every trick is success. Optional defender
counterexample search uses a deliberately biased witness mixture. It is not
a calibrated probability. This pilot covers straight bid-30 play only, not
Nel-O, auctions, bidding accuracy, or complete marks matches.

## Proof assumptions, challenge, and actual verification

`math/lead/Disagreement.lean` contains six kernel-checked lemmas:

- `payoff_le_disagreement`, `payoff_sandwich`: for nonnegative integer scenario
  weights and Boolean payoffs, equal payoffs outside a flagged disagreement
  set imply absolute success-mass difference at most its weight. Repeated
  scenario entries retain their multiplicity.
- `maximum_transport`: an attained source value, an upper bound for every
  target policy value, and a uniform one-sided policy error imply the same
  one-sided bound on the attained value. Apply in both directions to attained
  finite maxima over the same policy family.
- `strict_action_transport`, `weak_action_transport`: action error bounds and
  a sufficiently large old gap preserve strict/weak preference, respectively.
- `canonical_interval_pruning`: an action upper bound below an incumbent lower
  bound permits removal; equality also permits removal when the incumbent has
  the lower tile index. For defenders use focal success or complement bounds.

The common-replay interpretation requires an additional game coupling argument:
if the field never differs on a replay under one fixed lawful focal policy,
payoff is unchanged. The Lean theorem assumes that payoff-coupling premise;
it does not formalize the whole Texas 42 transition system or construct a cheap
uniform disagreement detector. This is the remaining application work.

Sol challenged an attempted practical shortcut with a two-scenario witness:
old A=`[1,0]`, old B=`[0,0]`, new A=`[1,0]`, new B=`[1,1]`. Incumbent A has
zero replay disagreement, yet B becomes better. Therefore incumbent-only
rollouts cannot stand in for action- and policy-uniform error control.

`math/sol/ArgmaxAmplification.lean` proves six components of the counterexample
family in the first result. The host knows a hidden type that the field does
not. This is consistent with different seats' information; it does not refute
optimal-value continuity under one unchanged belief. The lead checked this
distinction and independently reran the file.

| Actual check | Result / elapsed | Receipt |
|---|---|---|
| Lead final Lean file | pass, 0.322 s | `results/lean-disagreement-final/` |
| Sol independent final lead check | pass, 0.314 s | `math/sol/lead-final-independent-lean-run/` |
| Sol Lean file | pass, 0.150 s | `math/sol/lean-run/` |
| Lead independent Sol check | pass, 0.148 s | `results/lead-recheck-sol/` |
| Independent finite weighted checks | 5,832 couplings; 11,025 admissible transports; pass | `results/lead-small-cases/` |
| Sol exact rational/demand/cycle/counterexample checks | pass | `math/sol/witness-final-run/` |

Toolchain is Lean 4.33.0-rc1, commit `62eed1db4d67327ec8120be05f1a1b0847d74561`,
arm64 macOS. These files need only bundled Init/Std, not a mathlib download.
All declarations are proved, with no `sorry`, `admit`, custom axioms, or
`native_decide`. Printed dependencies are only standard `propext`, `Quot.sound`,
and (one lead theorem) `Classical.choice`; Sol's six use only `propext`.
Every verification's exact command, limit, exit, stdout, and stderr is retained.

## Reusable evaluation baseline and measured limits

Reused infrastructure: independent `experiments/partnership/rules.py`, existing
watchdog `run_capped.py`, its process-group design, and phone ABI/checkpoint
conventions. The old `phone.mjs` deliberately loads archived racing WASM, so it
cannot serve as the new production baseline. Historical response-ladder and
full-game-speed drivers informed the protocol, not a new phone-strength claim.

The new `tools/arena.py` loads exact pinned WASM through `phone_worker.mjs`.
It starts a fresh WASM instance per decision and reuses compilation in the
Node host. Only time and checkpoint imports are available. Requests contain
exactly the acting seat's original hand, chronological public record, contract,
seat/bidder, declaration and public policy seed `7042104`. That seed is fixed
independently of all hidden deal seeds; it is an experimental public stream,
not a claim to reproduce an arbitrary saved phone game's app-derived seed.

The sequential pilot predeclared seeds `104200–104203` and declarations
`6,0,7,9`, respectively. Each whole deal was physically rotated four ways and
the candidate swapped between declarer and defender partnerships. Both sides
have identical requested budgets: 6,368,000 ms each over 448 moves. The ablation
uses the same exact binary, sample budgets, and stages, with review disabled.
Games play all 28 moves, including the settled suffix. This is actual play
against the phone binary, not teacher-label agreement or an oracle contest.

| Mac Node/WASM observation | Production phone | L1 review-off ablation |
|---|---:|---:|
| Decisions | 448 | 448 |
| Mean host ms/decision | 18.540 | 18.151 |
| Median host ms/decision | 1.657 | 1.137 |
| p95 host ms/decision | 66.464 | 65.645 |
| Ordinary completed L1 routes | 285 | 288 |
| Forced routes | 161 | 160 |
| Completed review changed move | 2 | 0 |
| Fallbacks / host interruptions | 0 / 0 | 0 / 0 |

Timing includes sampling, stages, review, IPC and first worker startup in the
corresponding call. Compilation timing is retained separately in move receipts;
the overall panel also includes refereeing, process startup and JSON records.
These are single sequential pilot timings on macOS 26.5.1 arm64, not a phone
benchmark, a controlled speed win, or tail-latency evidence for hard hands.

Sol independently reconstructed all 32 games, all 896 strict information
requests, shuffled deals/rotations, profiles, legal sets, scores, outcomes and
hashes (`math/sol/panel-audit-run`, pass in 0.265 s). Before that, its 144 forced
suffix checks covered all nine straight declarations and four bidder rotations.
A further exact-phone-versus-itself control completed eight games / 224 moves
in 6.460 s: each paired game had identical requests, choices and score.

The paired contrast is candidate-declaring make minus phone-declaring make,
averaged over four rotations of each source deal. Rotations are not independent
samples. With four source clusters, mean 0 and distribution-free Hoeffding 95%
interval `[-1,1]`, no strength ordering is established. The interval assumes
independent draws of source deals with fixed declared profiles; predefined
declaration strata may differ. Exact finite chosen-panel outcomes need no such
population assumption. The host timing can affect policy at budget boundaries.

All planned attempts are recorded before execution. Incomplete blocks contribute
no pair counts and cause population-interval refusal; wholly absent planned
seeds are detected. Independent failure tests verified that error/EOF retains
a checkpoint, explicit result errors remain fatal, and incomplete panels cannot
silently receive a complete-panel interval. A timeout is not a loss or a draw.

## Hard bounds and next research steps

All experiments and Lean runs used a process-group watchdog at **295 seconds
or less**. The full-game batch used 290 seconds; its own scheduling limit is
280 and each game at most 240. The deliberate watchdog test killed a parent
and grandchild at 0.25 seconds, returning expected timeout status at 0.260 s.
`ps -p 6917` was denied by the sandbox; a permitted `kill(pid,0)` liveness check
confirmed that the grandchild no longer existed. No process work is pending.
See `results/RUN-INDEX.json` for every run's cap and elapsed time.

The initial useful result and reusable baseline are complete. Open work is
substantive research, not an access blocker:

1. Instrument unique complete lower-rung information keys and dependency edges
   on small fixed bundles. Existing `pi_calls_by_level` counts are useful;
   measure actual DAG size before asserting memoization makes k cheap.
2. Construct action-uniform disagreement or dominance bounds, ideally sharing
   lower-rung classifications. A compiled lower policy is a different frozen
   field unless equality or transported action bounds are proved. Small teacher
   regret alone is now explicitly ruled out as sufficient justification.
3. Compare model depth and focal horizon separately on bounded endgame panels.
   Retained bounds can improve monotonically for one fixed field and bundle;
   replacing the field, resampling, or changing its budget changes the target.
4. Test candidates on fresh, larger whole-deal holdouts, preserving this exact
   phone baseline, equal budgets, recorded fallbacks, balanced assignments,
   and separate speed/strength reporting. Physical phone timing is a later test.

The old 4.4x generator-tail scheduling improvement is reusable infrastructure
evidence only. It neither accelerates a single phone decision nor proves
higher-k scaling. The prior faster-win endgame discussion remains a separate
unimplemented refinement; no secondary tie preference was changed here.
