# walt — the instrument catalog

[Home](Home.md) · owns: the catalog of walt's executable instruments — every
binary under `walt/walt/src/bin/` (54 `.rs` files plus `webtable.html` at
`c00717d1`, 2026-09-07 — unchanged at this branch's HEAD `6a319216`, whose
walt-side edits are prose files only) with its purpose, exact invocation, record path, gate
file, cost and status, organized by role and by program; the seats a person
can play; how to read a walt record; the historical (archive-only) instruments
of 2026-08-09 → 08-24 · Sources: the binary headers under
[`walt/walt/src/bin/`](../walt/walt/src/bin/) (every invocation grammar below is
read from `main()`, not from prose); the probe records under
[`walt/probes/`](../walt/probes/) (`m3/`, `bidcurve/`, `shadow/`, `step8/`,
`step9/`, `fieldswap*/`, `hazard_witness/`, `l2_controller/`, `waking/`,
`ordering/`, `field_cache/`, `bundle/`, `root_interval/`, `grammar_residual/`,
[`factor_belief/`](../walt/probes/factor_belief/README.md),
[`gran/`](../walt/probes/gran/README.md), `factory-results/`, `exp3a/`, `exp5/`,
`tilt_arena_2026-08-19.log`); the briefs and reports of record under
[`walt/briefs/`](../walt/briefs/); [`walt/MAP.md`](../walt/MAP.md);
[`walt/LOG.md`](../walt/LOG.md); [`walt/DISCREPANCIES.md`](../walt/DISCREPANCIES.md);
[`walt/SCENARIO-PLAYER.md`](../walt/SCENARIO-PLAYER.md),
[`walt/CONTROLLER-PLAYER.md`](../walt/CONTROLLER-PLAYER.md),
[`walt/TILT-AUDIT.md`](../walt/TILT-AUDIT.md); [`walt/gym/`](../walt/gym/README.md),
[`walt/scheme/`](../walt/scheme/README.md), [`experiments/partnership/`](../experiments/partnership/README.md);
[`walt/ci/check.sh`](../walt/ci/check.sh), [`walt/ci/run_test_binaries.py`](../walt/ci/run_test_binaries.py),
[`walt/ci/check_m2_metal.sh`](../walt/ci/check_m2_metal.sh);
[`walt/UNIFICATION-CENSUS.md`](../walt/UNIFICATION-CENSUS.md), [`walt/ARCHIVE.md`](../walt/ARCHIVE.md);
`walt/walt-wasm/`, `walt/walt2-wasm/`, `walt/walt-m2-runner/src/main.rs`,
`walt/walt-metal/tests/metal_device.rs`; the survey maps of 2026-09-07; fresh
measurements made 2026-09-13 on this machine (labelled as such).
Related: [walt hub](walt.md), [walt-architecture](walt-architecture.md) (the
crate: modules, seams, invariants, the gate — this page does not restate them),
[walt-seat-play](walt-seat-play.md), [walt-calculated-evidence](walt-calculated-evidence.md),
[walt-counted-belief-era](walt-counted-belief-era.md),
[walt-focal-horizon-era](walt-focal-horizon-era.md),
[walt-gran-anchors](walt-gran-anchors.md), [walt-partnership-program](walt-partnership-program.md),
[walt-gym](walt-gym.md), [walt-scheme-fix](walt-scheme-fix.md),
[walt-gpu-native-trick1](walt-gpu-native-trick1.md),
[walt-negative-results](walt-negative-results.md), [vocabulary](vocabulary.md),
[timeline](timeline.md).

> **Epistemic tier: EXPLORATORY — below every tier on
> [Home](Home.md#evidentiary-tiers--never-promoted-never-blurred).** Every
> binary on this page opens with the same sentence in its own header — "sits
> below every evidentiary tier and is cited by nothing above it" — and every
> number it prints is computed evidence at a declared configuration on a
> declared finite domain: never a corpus status, never a kernel proof, never an
> exchange adjudication, never a rob receipt. Timings are machine facts about
> one run, not properties of the mathematics. A probe number becomes quotable
> as a result only through the gate file under `walt/walt/tests/` that pins
> it, or by a brief amendment that adds it to a verifier receipt; this page
> names the gate beside every number it quotes, and says "probe record, not
> gate-pinned" where there is none. A green gate is evidence, never a status
> change (TRUST-01).

Vocabulary that this page keeps typed: "certificate" appears only as walt's
own historical §16.11 record type (`certificate-schema.md`, preserved under
`walt/probes/factory-results/`) and as walt's term of art **certified regret**
`Γ = U* − B_exec`; the D3 concept is the **necessary outer profile** and no walt
object is an identity-bearing witness of reachability. Support ≠ belief;
feasible ≠ reachable; estimate ≠ receipt; exact-for-the-frozen-set ≠ exact
root; decision-dead ≠ decided ≠ laydown; the objective is **pmake** (ruled
2026-08-17); "level 2" is a best response to a *named* σ1, never an
equilibrium. The one object of the root-interval / focal-horizon work is the
interval `[L, U]`, the survivor set, and the focal-horizon hierarchy — never a
"sandwich" (CBS-A3, FH-A2; the adjudicated theorem name and the package
filenames may be quoted).

## 0. How to use this page

Three readers, three doorways.

- **A newcomer** who wants to see walt play: read §1 (the seats), then run the
  eight-step first-run order at the end of §5 — every step finishes in under a
  minute and prints something readable.
- **A mathematician** who wants the instrument behind a number: §3 maps each
  binary to its module, its gate file, its record and its owning chapter;
  §4 puts every headline number beside the honest negative that travelled
  with it.
- **An engineer** who wants to reproduce a record: §2 says what a record's
  identity consists of (the declared epoch is part of the result), §3 gives
  the exact argv and env knobs, §5 says what it costs. The gate itself is
  owned by [walt-architecture](walt-architecture.md).

The unified crate holds **54 binaries plus `webtable.html`** (counted at
`c00717d1`; `ls walt/walt/src/bin/`). Earlier revisions of this page said
"nineteen" and then "twenty-seven"; both were true when written and both went
stale, so this page now counts by directory listing. The 54 are catalogued
exactly once each: six play surfaces in §1 and forty-eight instruments in §3.

## 1. The seats you can play

**The standing fence (CE-A7 / `CONTROLLER-PLAYER.md` §20.16):** level-1 walt
through `walt_bridge` / `webtable` / `playtable` is the default and has been
untouched by every program since 2026-08-24 (FH-A10 restates it); every other
seat below is a **variant surface with no strength claim**, seated only by an
explicit flag or by launching a different binary. Nothing here is a strength
claim; the one arena outcome (§4) is an exploratory result about play.

| Surface | Library entry point | How it is spoken to | Declared defaults | Per-move cap | Status |
| --- | --- | --- | --- | --- | --- |
| `webtable` (+ `webtable.html`) | `solver::level1_evaluate` for auction, trump and play; `solver::act` at the AI chairs when `ctrl` is given (play phase only) | Single-process localhost HTTP, page served by `include_str!` of `walt/walt/src/bin/webtable.html`; routes `/ /state /step /play /pick /bid /hint /review /auto /new` | `webtable [port=4242] [n_outer=100] [n0=8] [seed=42] [human_seat=0] [ctrl] [cap=N]` (flags may sit anywhere; `cap` default 128); bid threshold θ = 11/16 as the constants `THETA_NUM/THETA_DEN` (calibrated 2026-08-19, `probes/bidcurve/ANALYSIS-2026-08-19.txt`) | 120 s | live default surface — the table Jason plays; real auction, all-pass redeals, `hint` from the human's chair only, per-play `review` cross-fiber column, routes logged per play under `ctrl` |
| `playtable` | `solver::level1_evaluate`; `solver::act` under `ctrl` | Terminal prompt: a tile, `hint`, `auto`, `quit` | `playtable [human_seat=0] [n_outer=100] [n0=8] [seed=42] [fresh] [ctrl] [cap=N]`; contract frozen to receipt hand 8 (trump fives, P30 by T1, S1 leads); `fresh` re-deals all four seats | 180 s | live default surface (interactive); no auction |
| `walt_bridge` | `solver::level1_evaluate` (decisions identical to `playtable`'s policy); declaration = argmax P(make 30) over the bidder's belief sample | The arena line protocol (mk5's `rob:<path>` adapter, unchanged): per stdin line `seat decl bidder h0..h6 n (actor domino)*n` → `domino leader points0 points1` (walt's own leader/points let the harness assert rules conformance on every decision); `declare bidder h0..h6` → declaration id. Seats are rotated so the bidding team is internal T1. Falls back to lowest legal on a `Level1Refusal` | `walt_bridge [n_outer=50] [n0=8] [per_move_secs=120] [n_declare=100]`; env `WALT_N_OUTER / WALT_N0 / WALT_PER_MOVE / WALT_N_DECLARE`; `WALT_DECLARE_FULL=1` adds doubles/no-trump (pip trumps 0..6 only otherwise); `WALT_BRIDGE_LOG=<base>` appends one JSONL line per decision (per-PID suffix, bp only); the outer-belief seed is the compiled constant `BRIDGE_SEED` (`0xB7E1_5162_8AED_2A6B`, mixed with the hand and the record hash), not a knob | 120 s | **the** live default surface for external harnesses — the seat that beat the mk5 E[Q] champion on 2026-08-17 (`probes/m3/arena_results_2026-08-17.txt`: n_outer=50, n0=8, 120 s/move, 527 hands in 20.8 min); audited CLEAN (O27) |
| `controller_bridge` | `solver::act` — one decision in, one `ActDecision` out carrying the `route` that chose the tile and a `settled` flag that is false on every fallback; δ-safe elimination inside the correctness boundary, level-1 ranking among ties/δ-survivors outside it (δ_run = 1/100) | The same line protocol as `walt_bridge` — an external consumer seats it with zero changes on its side; empty stdin exits 0 | `controller_bridge [world_cap=128] [exact_cap=2000] [n_outer=200] [n0=8] [n_declare=100] [per_move_secs=120]`; env `WALT_CTRL_WORLD_CAP / WALT_CTRL_EXACT_CAP / WALT_N_OUTER / WALT_N0 / WALT_N_DECLARE / WALT_PER_MOVE`; env-only `WALT_CTRL_N_OUTER_FROZEN=8 / WALT_CTRL_N0_FROZEN=2`; `WALT_DECLARE_FULL=1`; `WALT_CTRL_LOG=<base>` (one JSONL record per decision: route, settled flag, among-set, fallback options) | 120 s | variant surface (2026-08-24, #37); touched by the σ1-repair (#83) and the 2026-09-06 inner-belief option; never a default; the owning register is `CONTROLLER-PLAYER.md` |
| `waking_bridge` | `solver::waking::WakingSeat` — act's σ0 baseline always, a hard-budgeted wake check, wake-gated σ1 escalation through `solver::targeted`; one seat per dealt hand so the σ1 action cache amortizes | The same line protocol; plus `waking_bridge driven <out.jsonl> [n_hands=2] [same knobs]` (whole fresh-deal hands with the waking seat at all four chairs; `WAKING_DRIVEN_SEED` mixed per hand, bidder rotating, bid 30) | Same six positional/env knobs as `controller_bridge`; env-only `WALT_WAKING_N_OUTER_FROZEN=8 / WALT_WAKING_N0_FROZEN=2 / WALT_WAKE_WORLDS=24 / WALT_WAKE_EXACT_CAP=1024 / WALT_WAKE_ESC_EXACT_CAP=4096 / WALT_WAKE_ESC_BASELINE=128 / WALT_WAKE_ESC_E3=24`; census JSONL to `$WALT_WAKING_LOG.<pid>` | 120 s | variant surface + instrument (2026-08-25, #54); record `probes/waking/driven.jsonl` + `summary.txt` (2 hands, 56 decisions, 283,899,641 µs of decision compute — "not affordable as-is"); gates `tests/solver_waking.rs` (9 tests; the 2 `compile_fail` typing locks are doctests on `src/solver/waking.rs`) |
| `partnership` (`--stream`) | `solver::partnership` + `solver::selection` (one fixed/refine/race-refine selection authority shared by the real root and the modeled minds) and `solver::inner_belief` (voidless / voids-counted) | One request on stdin → one JSON line; `partnership --stream` answers blank-line-separated requests one after another (protocol errors as `{"status":"error","error":…}`, 16 KB request cap). **The first line of a request is the bare mode word** — `status`, `baseline`, `partner` or `all-l1` (there is no `mode` field; a first line `mode status` is rejected as `unknown mode`) — and every following line is `field value…` over exactly these fields: `decl` (0..7, 9), `bid` (30..42), `seat`, `bidder`, `hand` (7 ids), `plays` (actor/tile pairs), `seed`, `n` (1..640), `n0` (1..64), `n1` (1..64), `budget_ms` (≤ 14000), `inner_belief` (0 voidless / 1 voids-counted), `selection` / `modeled_selection` (0 fixed / 1 refine / 2 race-refine); even `status` needs the sampling fields to parse. Reply: `legal, leader, points, trick`, and for the play modes `choice, options [[tile, "numer", "denom"]…], selection, modeled_selection, inner_belief, outer_worlds, outer_draw_attempts, pi_calls_by_level, inner_worlds_by_level, nodes, solver_us`. There is no `--help`; a bare run prints `missing mode` and exits 2. `experiments/partnership/player.py::native_text` writes this grammar | The families are declared by `experiments/partnership/player.py` (`BINARY = walt/target/release/partnership`): L1 = n 40 / n0 8; L2 Partner = 40/8/2 (partner modeled at level 1, opponents level 0) | 14 s (the wrapper's ceiling) | live surface of the [partnership program](walt-partnership-program.md) — the L1 / L2 Partner / L2 All families behind the 524- and 400-game batteries; exploratory; not a default anywhere (2026-09-06, `cfb0fb25`, `9236ca7f`, `dbcc698f`). Measured 2026-09-13 (release binary of the main checkout): `status` on the opening root answers `{"legal":[0, 1, 2, 3, 4, 5, 6],"leader":1,"points":[0, 0],"trick":1}` in 3 ms; a `partner` request at n 8 / n0 2 / n1 2 answers in 0.63 s (`solver_us` 626,070; 4,409,364 nodes) |

The **phone** is the seat plunge ships, and it is not one of the 54 binaries:

| Oracle | Entry point | Defaults | Build and verify | State of the committed binary |
| --- | --- | --- | --- | --- |
| `walt-wasm` (`pkg/walt.wasm`, 304,312 bytes committed — the pkg README's "~250 KB" is stale — zero imports; `pkg/walt.ts`) | `api::handle` — request kinds `play` / `bid` / `declare` over a string API; play fields `seat, decl, bid, bidder, hand, plays, seed, n, n0, race, viewer, viewer_hand`; bid fields `hand, need, theta num den` | `n=40`, `n0=8`, θ = 11/16, `race` opt-in (`race: true` → race-then-refine, response marked `raced`), `budgetMs` inert in wasm (no monotonic clock — budget by `n`) | `sh walt/walt-wasm/build.sh` (needs `rustup target add wasm32-unknown-unknown`; Node ≥ 23.6) builds, copies to `pkg/`, runs `smoke.mjs`, which drives a full hand and byte-compares with the native `tests/full_hand.rs` trace (28/28); `cargo test --release -p walt-wasm` | `pkg/walt.wasm` was last rebuilt at `df3ffcd5` (2026-08-24). `src/api.rs` changed after that at `161b0195` (2026-09-02, σ1-repair) and `dbcc698f` (2026-09-06, inner-belief option), and the `walt` crate it links changed with them — **the committed binary is behind its source**. Whether that is intentional is an open item: `experiments/partnership/BASELINE.md` deliberately freezes the phone artifact plunge actually runs (the Plunge checkout at `122ea7a5`, whose `walt.wasm` was added at plunge `1810da20` on 2026-08-22; byte-for-byte texas-42 `9a056f20` of 2026-08-19) as an external anchor |
| `walt2-wasm` (`pkg/walt2.wasm`, 282,019 bytes committed; `pkg/walt2.ts`) | same ABI export names and request mapping, `walt2` request magic; play evaluated against modeled level-1 minds (`Field::Level(1)`, `n_inner=[n0,n1]`) over the same outer worlds and seed formula as walt-wasm (common random numbers across levels); bid/declare are walt-wasm's auction handlers, byte-identical by a native equality test | `n=8`, `n1=4`, `n0=2` — "thinks for a bit"; the pkg README carries a measured trick-1 latency grid | `sh walt/walt2-wasm/build.sh`; `cargo test --release -p walt2-wasm` | last rebuilt at `33d541fe` (2026-08-25, #58); its `src/api.rs` changed at `161b0195` and `dbcc698f` likewise. Never a default; whether small-knob level 2 beats big-knob level 1 at equal latency is an open head-to-head the README states |

Two HTML viewers under [`walt/viewer/`](../walt/viewer/) consume the records
the seats write: `walt_viewer.html` renders `playout` JSON games ("level-1
playouts"); `walt_table_viewer.html` renders the `playtable`/`webtable`
cross-fiber review ("the informed table"). Open either in a browser and load a
record; no server is needed.

Three facts about the live player that a reader must not infer wrongly from
older text:

- **The σ1-repair (`161b0195`, 2026-09-02, #83)** made `solver::sample_belief`
  the one void-conditioned belief sampler. Before it, `walt_bridge`, `playout`,
  `playtable`, `divergence` and the library each carried a copy, and the
  shuffle-and-reject loop could fail to terminate at zero-joint-mass
  information states (the pinned specimen: seat S3 hand {4-2 4-4} after
  [4-1, 4-3, 1-1]; diagnosed by MB0, `probes/factor_belief/modelbelief_run1.txt`,
  whose "four un-deduplicated copies remain exposed" sentence is therefore
  *historical*). Gates R1–R6 in `tests/solver_sigma1_repair.rs` (eight
  `#[test]` functions: the six R-gates named in the file header, an
  oracle-semantics check, and the `#[ignore]`d fixture capture): the before-side
  fixture `tests/data/sigma1_before_v1.txt` was captured once against the
  unpatched sampler and is byte-reproduced through the repaired path; the
  dedup is witnessed by a source grep and by the compile itself (a local
  `fn sample_belief` would now be an E0255 collision). `grep -rn "fn
  sample_belief(" walt/walt/src` returns `solver/mod.rs:1165` only (checked
  2026-09-13; the bare pattern also hits `solver/partnership.rs:147`, a
  differently named `sample_belief_bounded` wrapper over the same authority,
  not a second sampler).
- **The 2026-09-06 inner-belief option (`dbcc698f`)** added
  `InnerBelief::VoidsCounted` to the shared solver behind `Shared::with_inner_belief`;
  the default stays `Voidless`, the phone rejects the option, and
  `bidcurve`/`controller_bridge`/the play surfaces gained the plumbing without
  a default change ([walt-gran-anchors §8](walt-gran-anchors.md)).
- **`playout`'s local `PiKey` still omits banked totals** — the filed §3.4
  finding of the shadow audit — and is deliberately not patched there
  ([walt-calculated-evidence](walt-calculated-evidence.md) carries the filing).

## 2. How to read a record

Every instrument writes a record whose first line is its own tier line
("EXPLORATORY … estimates, never receipts; not a P-A21 statement"). Five rules
make a record readable, and each of them is a typed distinction the code
enforces rather than a convention:

1. **The declared epoch is part of the result's identity.** The field models
   (`FieldId`s σ0/σ1), the frozen focal schedules (`PolicyId`s), the seeds and
   the caps ride every record; a different schedule is a different experiment,
   not the same statistic improving (CE-A3). Two σ0 epochs coexist in the
   records and **do not compose**:

   | Epoch | σ0 | σ1 | frozen focal candidates | act / wake caps | Instruments |
   | --- | --- | --- | --- | --- | --- |
   | L2-thread probe epoch | `Level0{n0=8}` | `Level1{n_outer=4, n0=2}` | `[8, 2]` | — | `fieldswap`, `fieldswap_screen`, `fieldswap_cancel`, `fieldswap_motifs`, `hazard_witness`, `l2_controller` |
   | counted-belief / anytime / horizon / model-belief / focal epoch | `Level0{n0=2}` under `SupportOracle` | `Level1{n_outer=2, n0=2}` where a σ1 appears (MB0/MB1's F₁) | `PinnedThenLevel1[2,2]` | — | `rootinterval` … `focalreport` (§3.3–§3.5), `wakeup` (its σ0 is step 8's exact field, L2-A6) |
   | act / waking / gran epoch | `Level0{n0=2}` | `Level1{n_outer=4, n0=2}` | `[8, 2]` | `ActConfig::interactive`: world cap 128, exact cap 2000, fallback 200×8; wake budget 24, exact wake cap 1024, escalation 4096/128/24 | `controller_bridge`, `waking_bridge`, `granrun`, `webtable`/`playtable` under `ctrl` |

   The Plunge panel that started the Gran program (40 sampled worlds, 6-2 at
   90% vs 6-4 at 80%) is a fourth epoch and composes with none of these
   (`probes/gran/README.md`). This table is the instruments' view; the
   crate-wide declared-epochs table — which adds the batch controller, the
   gym field and the argv/env defaults of every live level-1 surface, each
   with the source line that declares it — is
   [walt-architecture §5](walt-architecture.md), and the two agree row for
   row (checked 2026-09-14). One more typed fact about the first row: the
   **gate files** of the L2-thread family (`tests/solver_fieldswap*.rs`,
   `solver_hazard_witness.rs`, `solver_targeted.rs`) run at σ0 =
   `Level0{n0=2}`, σ1 = `Level1{n_outer=2, n0=2}` — the second row's epoch,
   not the bins' — so the records' n0 = 8 numbers are bin-asserted probe
   output; what those gates pin are the laws (containment, parity, exclusion
   soundness, winner stability) and the one rational `Λ = 31/1200`, which
   `solver_fieldswap_cancel.rs` asserts at the gate's own epoch (checked
   2026-09-13).
2. **A cap is a resource limit, never a settlement rule** (CE-A3, §1.5). A low
   cap yields more honest `Unresolved` results and level-1 fallbacks, never a
   wrong settlement; an over-budget exact measurement returns nothing and is
   recorded as refused, never quietly replaced by a sample (`Unmeasured` is
   never `Measured(0)`). Every refusal is typed and printed with its reason.
3. **No aggregate is hand-maintained.** The JSONL-writing instruments ship a
   stdlib-only `summarize.py` beside their records (`probes/{shadow, step8,
   step9, waking, l2_controller, fieldswap, fieldswap_screen, fieldswap_cancel,
   fieldswap_motifs, gran}/summarize.py`); the published tables are recomputed
   from the records, and the README's "Reproduction" section gives the exact
   bin command and knobs.
4. **A probe number becomes quotable only by a gate.** The `report`-mode
   instruments assert their laws inside the run (parity, conservation, the
   §41 laws) and the gate files re-assert the pinned specimens in CI; the
   *numbers* a record prints (walls, per-root permilles) are probe output
   unless a test names them. §3's tables say which.
5. **Walls are single-machine readings; reads and nodes are the exact units.**
   Records that are meant to be byte-deterministic carry no wall fields at
   all (`v5flip`, `wakeup`); the `report` files print integer microseconds
   beside exact read counts and say "wall is the only approximate number
   here".

**Worked example — the shadow instrument's two epochs.** `shadow`'s
`world_cap` default became **512** at `fe26c88d` (2026-08-24, Jason's cap
ruling: the 128/40/160 caps were phone-tier budget limits, not calibrated
choices; `6e00528d`, which earlier revisions of this page cited, is the merge
commit of that PR, #32, and carries the same change). The committed `probes/shadow/receipt.jsonl` and `driven.jsonl`
are the **128-epoch** (33 hands, 183 decisions: ExactFrozenSet 67 /
Unresolved 116 / DeltaSettled 0; 27 controller winners, live choice equal in
23/27; live choice among survivors 116/116). Reproducing them byte-identically
now requires passing `128` explicitly. The **512-epoch** reruns are separate
records, `receipt_512.jsonl` (28/40/2 — the instrument's first sampled-route
settled disagreement: hand 10 trick 1 settles at world 395 with winner 5-5
against the live 6-3) and `driven_512.jsonl` (39/73/1); they supersede nothing,
being a different epoch by construction, and the forecast that "~108/116 would
settle by 512" was wrong (2 of 42 receipt-side). Both settled disagreements sit
in tricks 1–2, where the arena localized the live player's deficit —
suggestive, instrument-grade only. Nothing in the shadow records is gate-pinned
except the library pieces (`tests/solver_shadow.rs`).

## 3. Instruments by program

Each program has two tables: the **map** (binary → module → gate file → record
→ owning chapter) and the **rows** (purpose, exact invocation, runtime and
output, status, and the one honest sentence the instrument earned). All
invocations are given for the release binaries (`cargo run --release -p walt
--bin <name> -- <args>` from `walt/`, or `walt/target/release/<name>` directly);
`report`-mode binaries print usage and exit 2 on a bare invocation. **Five
binaries start long runs on no arguments** — `bidcurve` (a 10-hand run at
120 s per cell), `divergence` (a 200-hand run), `level1`, `level2` and `ladder`
(all four boundaries t=4…1 in turn, the last of which dies or is refused) —
and `scenario` (all four boundaries at 10,000 scenarios), `e0cal`, `v5flip`,
`wakeup` and the `fieldswap*` family write their default record on a bare run
too. Read the row before pressing enter.

### 3.1 Seat play, 2026-08-17 → 08-19 (owned by [walt-seat-play](walt-seat-play.md))

These are the probes of the day walt first played: the frozen hand-8 carrier
solved exactly, then sampled, then given a field that thinks. The play
surfaces of this era (`webtable`, `playtable`, `walt_bridge`) are in §1.

| Binary | Module it exercises | Gate file | Record | Owning chapter |
| --- | --- | --- | --- | --- |
| `m3probe` | `walt::carrier` (freeze-57 M3 carrier, KAT-pinned in `tests/carrier.rs`) + its own integer solver | none pins the four values (carrier digests only) | `probes/m3/results_2026-08-17.txt` | walt-seat-play §1 |
| `ladder` | `walt::carrier`; own exact solver | its own in-binary assertions (`frozen_t4_expectations`); no CI gate | `probes/m3/ladder_results_2026-08-17.txt` | walt-seat-play §1 |
| `scenario` | `walt::carrier`; own sampled solver | none | `probes/m3/scenario_results_2026-08-17.txt`, `sampling_results_2026-08-17.txt` | walt-seat-play §1 |
| `level1` | `walt::carrier`; self-contained serial level-1 solver | none (superseded by `walt::solver`) | `probes/m3/level1_results_2026-08-17.txt` | walt-seat-play §1 |
| `level2` | `walt::carrier`; rayon-parallel level-2 solver with the banked-totals `PiKey` | none | `probes/m3/level2_results_2026-08-17.txt` | walt-seat-play §4 |
| `playout` | `walt::solver` (level-1) + `walt::carrier::VIEWER` | `tests/solver_sigma1_repair.rs` (its sampler import) | JSON games for `walt/viewer/walt_viewer.html` (none committed) | walt-seat-play §7 |
| `divergence` | own copy of the level-2 core + library `sample_belief` | none | `probes/m3/divergence_results_2026-08-18.txt`; corpus `probes/m3/mined/div_2026-08-18_hands_{000-299,300-899}.jsonl.gz` | walt-seat-play §4 |
| `bidcurve` | `walt::solver` (level-1, `n_inner=[8]`) | none | `probes/bidcurve/{small-n12,live-n40,ref-n200}.log`, `ANALYSIS-2026-08-19.txt`, `analyze.py`, `run_calibration.sh` | walt-seat-play §5 |
| `tiltaudit` | `walt::solver` (`level1_race`, `level1_raced`, `level1_race_refined`) | none (design `walt/TILT-AUDIT.md`, SP-A1..A12) | `walt/TILT-AUDIT.md` smoke section; `probes/tilt_arena_2026-08-19.log` | walt-seat-play §2, §6 |

| Binary | Purpose | Invocation | Runtime and output | Status | The sentence it earned |
| --- | --- | --- | --- | --- | --- |
| `m3probe` | Exact lawful (treatment H, perfect recall) and clairvoyant (C) solve of the frozen M3 carrier: receipt hand 8 before trick 4, S1 leads holding [2-1, 3-1, 3-3, 5-5], trump fives, 1,200 uniform worlds, field uniform-random-legal; objectives M3A (future-trick differential) and M3B (P30 make). Integer-only: root mass 1200·12¹² | `m3probe` (no arguments) | ≈ 30 s single-threaded; prints both objectives with every argmax member; **measured 2026-09-13: 28.85 s, output identical to the 2026-08-17 record** | live, historical probe; the seat-play era's one exact-truth anchor | Under pmake the lawful lead is 3-3 at 16078667/16588800 (≈ 96.92%); under trick differential it is 5-5 — the objective changes the play (probe record; the four fractions are not gate-pinned) |
| `ladder` | Walks the exact H solve from the trick-4 carrier back toward trick 1 with uniform posterior over the void-consistent support, decided cutoffs, viewer early exit, gcd-normalized interned posteriors; asserts the t=4 support equals the carrier and the four t=4 fractions equal the frozen M3B values; optional frozen-seed support sampling (estimates) | `ladder [t in 1..=4] [budget_secs=600] [sample_size]` — **no args runs t=4,3,2,1 in turn** | t=4 4.305 s (8,154,532 nodes); t=3 (59,976 worlds) killed at ≈ 600 s after 3·10⁸ nodes; t=2 died at 120 s; t=1 refused (399,072,960 > the 30M cap) | historical probe; still builds and its t=4 cross-check still runs | Exact posterior-carrying enumeration solves trick 4 in seconds and dies at trick 3 |
| `scenario` | External sampling of both random axes — the deal and a frozen per-scenario seed for the field's uniform-legal draws — with the solve exact on the sample and S1 keyed on public record + own hand only (no fusion) | `scenario [t in 1..=4] [budget_secs=600] [n_scenarios=10000]` — no args runs all four boundaries | 64.0 s at t=1, n=256,000 (124,463,989 nodes, linear in n) | historical probe; the device that made trick 1 reachable | The opening lead 5-5 is the argmax at every n from 4,000 to 256,000 with a stable ordering 55 > 33 > 11 > 54 > 21 > 52 > 31; against a non-random skilled field none of these numbers apply (the header's own caveat) |
| `level1` | The field seats become level-0 minds (own hand + public record, uniform no-void inner belief of n0 worlds, dice model of others, T1 max / T0 min pmake, lowest-index ties); S1 best-responds with lawful perfect recall; where the support enumerates (t=4) the outer solve is exact over the support given the policy; alive-set partition at field nodes = Bayes on policy-consistent deals | `level1 [t in 1..=4] [budget_secs=300] [n_outer=2000] [n0=64]` — **no args runs all boundaries** | t=1 with n_outer=2000, n0=16: 80.8 s, 299,375 π0 evaluations | historical probe; superseded by the library solver in `walt::solver` | 5-5 survives every rung: at t=1 lead 5-5 at 2000/2000 deals, every other lead losing ≥ 4 (sampled estimate, `level1_results_2026-08-17.txt` lines 22–45) |
| `level2` | The field model becomes a parameter (Dice at the bottom, Level(k) above): field seats are level-1 minds (n1 worlds) whose fields are level-0 minds (n0 worlds); rayon-parallel outer solve with a 64-shard cross-level `PiKey` cache carrying the banked totals; result lines identical at any thread count | `level2 [t in 1..=4] [budget_secs=600] [n_outer=2000] [n1=8] [n0=4]` — **no args runs all boundaries, very long** | t=1 n=3200: 960.2 s, 18.4·10⁹ nodes | historical probe (serial at `178722e`, parallel at `f5fff91`); the core was generalized into `divergence` and then the library | Level 2 agrees with level 1 at every rung on the frozen carrier (5-5 at 3200/3200, unique); the parallel port caught the `PiKey` banked-totals aliasing bug (values moved up to ≈ 1.7 points, argmax unchanged) and a three-way saturation tie at n=200 broke only at n=800 — saturation ties are refined, never index-broken |
| `playout` | Full games of receipt hand 8's declaration from trick 1: S1 = level-1 walt (sampled belief, no peeking), S0/S2/S3 = level-0 on their true hands, or `all1` = every seat level-1 from its own chair (filed information-inconsistent); every decision logged with the decider's options; emits a JSON array for the viewer | `playout [n_games=3] [budget_secs=300] [n_outer=200] [n0=8] [all1] > games.json` (progress to stderr; stops emitting at the budget) | seconds per hand at the defaults | historical/instrument; O27-repaired 2026-08-24; sampler deduplicated 2026-09-02; carries the filed §3.4 `PiKey` defect | The viewer's data source; the `all1` mode is known-inconsistent and stays labelled so |
| `divergence` | Level-2 divergence miner: self-play random deals with four level-1 walts (`n_inner=[8]`) and a level-2 shadow (`n_inner=[4,8]`) at one seat cycling self-bid / partner-bid / defense by hand index; the shadow never steers; both evaluate the same CRN worlds with one 4× tie refinement; emits the positions where level-1's choice is sub-optimal on level-2's table with the gap in basis points | `divergence [n_hands=200] [start=0] [n_outer=50] [budget_l1_secs=60] [budget_l2_secs=300] > out.jsonl` — **no args starts a 200-hand run** (the committed run used n_outer=40 over 900 hands, miner commit `30f1409`) | hours for hundreds of hands | historical probe; the referee protocol (mirrored replay, McNemar on discordant pairs) proposed 2026-08-18 was never run | Over 900 hands and 4,156 shadowed decisions the divergence rate is 1154/4156 = 27.8%, concentrated in tricks 1–4 (38–42% at t1–t3, 3.2% at t6) and, at gaps ≥ 1500 bp, in partner-bid + defense (2.44%) over self-bid (1.40%, z ≈ 2.3) — self-graded on level-2's own table, sampled |
| `bidcurve` | Deals random 7-tile hands to internal bidder S1 and prices P(make b) for all nine declarations × b ∈ 30..=42 at the auction point over common random worlds per hand (level-1 seat, field = level-0 minds `n_inner=[8]`, rayon); monotonicity in b checked softly (`[mono-viol xN]`) | `bidcurve [n_hands=10] [start=0] [n_outer=50] [per_cell_budget_secs=120]` — **no args starts a 10-hand run**; the corpus script is `probes/bidcurve/run_calibration.sh` (`bidcurve 200 0 <n> 120`) | 117 cells per hand at up to 120 s each — hours for a corpus | probe; substrate of baseline bidding (`SCENARIO-PLAYER.md` §6.2) | θ = 11/16: three passes over the same 200 hands at n = 12/40/200, scored by the n=200 reference — θ = 1/2 overbids 37/200 at n40 while 11/16 is the first rung with 0 overbids and 0 missed bids (`ANALYSIS-2026-08-19.txt`); this became the `webtable`/`walt-wasm` default |
| `tiltaudit` | Signed-pivotal E0 smoke (SP-A1..A12): fresh self-played hands under frozen seeds; find S1's first ≥ 2-legal decision at trick ≥ t_target, discover the choice across discovery seeds (Phase A), freeze the implied policies, replay pairs on a disjoint common panel, report pivotal mass q, tilt τ, gap g, hardness H in exact bp/‰. Subcommands `bench` (racing evaluator vs full evaluator head-to-head) and `arena` (mirrored deals at bid 30, race-then-refine team vs full-evaluator team, paired makes) | `tiltaudit [n_hands=4] [t_target=4] [n_seeds=4] [panel_n=200] [n_disc=200] [n0=8] [secs=120]` \| `tiltaudit bench [n_hands=12] [t_target=3] [n_full=40] [n0=8] [n_max=100] [n_self=20] [secs=120]` \| `tiltaudit arena [n_hands=24] [n=40] [n0=8] [secs=120]` | minutes (the 24-hand arena ≈ 132 s of decisions) | instrument (2026-08-19 smoke, `81a1943`/`f60b2e3`); runnable; Phase E vacuous until a stochastic field exists | The modeled level-0 field is deterministic in (seat, hand, record), so scenario = world; the race wins on wall (745 ms vs 1230 ms at the opening lead) but the arena is a dead heat (paired makes race-only 1 / full-only 2 / both 11) and the race *loses* on decision cost in saturation-heavy bid-30 self-play (177 ms vs 116 ms mean) — the opt-in `race 1` posture stayed opt-in |

### 3.2 Calculated evidence and the targeted level-2 thread, 2026-08-24 → 08-25 (owned by [walt-calculated-evidence](walt-calculated-evidence.md))

Two threads share these instruments and are labelled per row: **[CE]** =
sampling depth (anytime-valid adaptive settlement), **[L2]** = model choice
(field swaps against a named σ1). The play surfaces of this era
(`controller_bridge`, `waking_bridge`) are in §1.

| Binary | Module | Gate file | Record | Owning chapter |
| --- | --- | --- | --- | --- |
| `shadow` [CE] | `solver::adaptive` (`driven_root`), `solver::policy` (`ActionRule::PinnedThenLevel1`), `solver::field` (`Level0Field`) | `tests/solver_shadow.rs` (10; the library pieces only) | `probes/shadow/{receipt,driven,receipt_512,driven_512}.jsonl` + README + `summarize.py` | walt-calculated-evidence "The shadow instrument" |
| `v5flip` [CE] | `solver::calibrate` (`assert_cap_ladder`, `FLIP_FIXTURES`, `CountTimingSpec`) | `tests/solver_calibrate.rs` (11) | `probes/step8/v5.jsonl` + README + `summarize.py` | "Step 8" |
| `e0cal` [CE] | `solver::calibrate`, `solver::adaptive::evaluate_pair` | `tests/solver_calibrate.rs` | `probes/step8/e0.jsonl` | "Step 8" |
| `wakeup` [L2] | `solver::wakeup` | `tests/solver_wakeup.rs` (9 + compile_fail doctests) | `probes/step9/records.jsonl` + README + `summarize.py` | "Step 9" |
| `fieldswap` [L2] | `solver::field`, `solver::exposure` (`FrozenPolicyExposure`) | `tests/solver_fieldswap.rs` (11) | `probes/fieldswap/fieldswap.jsonl` + README + `summarize.py` | "field-swap slice 1" |
| `fieldswap_screen` [L2] | `solver::exposure` rungs E0–E2/E4, `solver::field_swap` (the L2-T4 screen) | `tests/solver_fieldswap_screen.rs` (9; O32/O38 parity) | `probes/fieldswap_screen/screen.jsonl` + README + `summarize.py` | "slice 2" |
| `fieldswap_cancel` [L2] | `solver::exposure` cancellation ladder, `solver::upper_cs` | `tests/solver_fieldswap_cancel.rs` (12) | `probes/fieldswap_cancel/cancel.jsonl` + README + `summarize.py` | "slice 3" |
| `fieldswap_motifs` [L2] | `solver::motif` | `tests/solver_fieldswap_motifs.rs` (6) | `probes/fieldswap_motifs/motifs.jsonl` + README + `summarize.py` | "slice 4" |
| `hazard_witness` [L2] | `solver::hazard` (`verify_hazard_witness`) | `tests/solver_hazard_witness.rs` (7) | `probes/hazard_witness/records.jsonl` + README | "slice 4" |
| `l2_controller` [L2] | `solver::targeted` (`targeted_root`) | `tests/solver_targeted.rs` (7) | `probes/l2_controller/records.jsonl` + README + `summarize.py` | "The targeted field-1 controller" |
| `ordering_bench` [CE] | `solver::mod` (`solve_viewer` visit order, `MoveOrdering`) | `tests/solver_ordering.rs` (4) | `probes/ordering/README.md` | "The speed campaign" |
| `field_cache_bench` [CE] | `solver::act`'s hot path (`FieldModel`, decided cutoff, `replay_viewer_success`) | `tests/solver_field_cache.rs` (5) | `probes/field_cache/README.md`, `bench_2026-08-25.log` | "The speed campaign" |
| `bundle_bench` [CE] | `solver::bundle` (`bundled_set_outcomes`) | `tests/solver_bundle.rs` (5) | `probes/bundle/README.md` | "The speed campaign" |
| σ1-repair (no binary; `161b0195`, 2026-09-02) | `solver::sample_belief` — the one sampler authority | `tests/solver_sigma1_repair.rs` (R1–R6 over 8 test fns, one `#[ignore]`d; fixture `tests/data/sigma1_before_v1.txt`) | `walt/briefs/BRIEF-SIGMA1-REPAIR.md`, `MB0-COLLISION-NOTES.md` | [walt-focal-horizon-era §2](walt-focal-horizon-era.md) |

| Binary | Purpose | Invocation | Runtime and output | Status | The sentence it earned |
| --- | --- | --- | --- | --- | --- |
| `shadow` | The §22 step-7 controller run *beside* the live player: hands driven in the playout shape (focal seat = live `level1_evaluate` at 200/8, other seats level-0 on true hands); at every multi-option focal decision the §16.4 controller also evaluates one frozen `PinnedThenLevel1` continuation per legal action (declared 8/2) under a run-scoped risk plan (δ_run = 1/100, δ_d = δ_run/(d(d+1))); fibers ≤ exact_cap run the exact frozen-set endpoint; agreement recorded, never acted on | `shadow receipt <out.jsonl> [n_outer_live=200 n0_live=8 n_outer_frozen=8 n0_frozen=2 world_cap=512 exact_cap=2000]` \| `shadow driven <out.jsonl> [n_hands=20] [same knobs]` — pass `128` explicitly to reproduce the committed 128-epoch records | summed shadow micros ≈ 1.5 h (128-epoch), ≈ 4.15 h (512 receipt), ≈ 16.3 h (512 driven); live eval median ≈ 0.21 s, shadow eval median ≈ 10.3 s; hands in parallel under rayon | instrument (2026-08-24, #24 / `0794ff8`; cap default 512 since `fe26c88d`) | At cap 128 the controller never eliminated the live line (116/116 among survivors) and agreed with it in 23/27 settled cases; at 512 it produced its first settled disagreements, both in tricks 1–2 (§2's worked example) |
| `v5flip` | The V5 law made mechanical: flip-shaped roots re-run under the adaptive controller on one epoch and one common indexed stream at the cap ladder 40/160/640, `assert_cap_ladder` asserting monotone outcomes (unresolved may settle later; settled stays settled identically; exact stays exact identically); exact frozen-set endpoint recorded beside the ladder where the fiber permits; specimens = the four step-7 exact-route disagreements and the six-member count-timing shape family (6-2 vs 6-4) | `v5flip [out.jsonl=v5.jsonl] [count_timing_n=6]` (committed: `v5flip probes/step8/v5.jsonl 6`); byte-deterministic records with no wall fields | runtime not recorded; the count-timing units evaluate at cap 640 on a 46,558,512-world fiber | instrument (2026-08-24, #31); the V5 law itself is gated in `tests/solver_calibrate.rs` | No cap-dependent flip anywhere — "the flip mode is gone"; h11 d4 (fiber 1750) stays an honest Unresolved at every cap (τ = 11/175) and the count-timing family stays Unresolved with q̂ ≈ 0.3 |
| `e0cal` | The §19 V6 per-fixed-pair E0 calibration — per pair, never pooled: exact coordinates (q, τ, g, H) by full-fiber enumeration, the initial evidence state, §7 information-rate and leading-order forecasts as exact rational interval bounds (24 series terms), the §8.4 DP forecast at γ = 1/2 and 9/10, and observed settlement indices of `evaluate_pair` over replicate declared streams; δ_pair = 1/200 (T = 400) | `e0cal [out.jsonl=e0.jsonl] [reps=3] [world_cap=1024] [dp_h_max=192]` — a bare run writes `e0.jsonl` | seconds to minutes (18 pairs × 3 replicates) | instrument (2026-08-24, #31) | 54 runs: 45 DeltaSettled, 9 honest Unresolved (the three small-\|τ\| pairs); every settled winner agreed with the sign of the exact τ (45/45) — forecasts are forecasts, settlement is by the exact evidence threshold |
| `wakeup` | The step-9 detection layer: per predeclared root and frozen pair, paired detection evidence under σ0 and σ1 — q̂/τ̂/ĝ under both fields, §14.6 paired-Z evidence, per-field 𝓘 = q·D₁/₂(τ) interval bounds with the §14.5 verdict, the three wake-ups (response / value / decision) kept distinct, exact-zero vs practical-zero typed; exact route (complete-fiber coupled enumeration, budget 4096) on the four flip fixtures (18 pairs), sampled dig-until-settled route (cap 256, min 64) on the six count-timing members; asserts the σ0 leg reproduces step 8's exact wins | `wakeup run [out.jsonl=wakeup.jsonl] [n0_field0=2 n_outer_field1=4 n0_field1=2 n_outer_frozen=8 n0_frozen=2 world_cap=256 min_worlds=64 exact_budget=4096]` (committed: `wakeup run probes/step9/records.jsonl`); byte-deterministic, timing to stderr only | runtime not recorded | instrument (2026-08-25, #49) | Exact route: value wake-up 18/18, decision wake-up 8/18, response wake-up > ε_q on only 2/18 with pivotal mass *dropping* under σ1 on 13/18 (h4 reaching q₁ = 0 exactly); §14.4 separation observed in the wild (h7 6-2 v 6-3: dq = 0, τ flips +3/5 → −1); sampled route honestly open 6/6 on response/value |
| `fieldswap` | The §21 step-5 field-swap smoke: `scan` lists receipt roots where the bidder leads (trick, declaration, fiber, bidder hand with count and trump marks); `run` computes `FrozenPolicyExposure` — coupled σ0/σ1 replay to the first field split — for two contrasting `PinnedThenLevel1` focals (reveal-5-5 vs retain) on the driven trick-1 root (stream prefix 64 of the 399M fiber) and receipt roots h8-t4 and h7-t5 over their exact fibers | `fieldswap scan` (the default mode, ≈ 1 s: 34 roots) \| `fieldswap run [out.jsonl=fieldswap.jsonl] [n0_field0=8 n_outer_field1=4 n0_field1=2 n_outer_frozen=8 n0_frozen=2 stream_worlds=64]` | scan ≈ 1 s; run minutes | instrument (2026-08-24); detector work only — never a root-action bound, never screening input (L2-A4, O31) | Three regimes on one smoke: h8-t4 reveal-5-5 exposure d = 1138/1200 with corrections +30/−26 (ĉ = +1/300), retain-3-3 d = 1117/1200 (ĉ = −9/400); h7-t5 both pins d = 0 exactly; driven h0-t1 63/64 and 64/64 exposed |
| `fieldswap_screen` | Steps 6–8 on the three parity roots (h7-t5 fiber 1680, h8-t4 fiber 1200, h4-t6 fiber 90): Stage 1 exact frozen-set σ0 baseline; Stage 2 the rung ladder per action (E1 trivial/forced-non-focal covers, E0/E2 from the clairvoyant reach walk, E4 = exact split reach, R_a exactly); Stage 3 the L2-T4 admissible screen at the cheapest sound bounds and at E4 with the full ordered-pair slack table; the exact σ1 parity audit asserted (every excluded action strictly σ1-nonoptimal); the §12.1 targeted-vs-naive cost note | `fieldswap_screen run [out.jsonl=fieldswap_screen.jsonl] [n0_field0=8 n_outer_field1=4 n0_field1=2 n_outer_frozen=8 n0_frozen=2]` (the default mode is `run`) | h8-t4: σ0 baseline 4.7 s, rungs 10.9 s, σ1 all-action 17.9 s | instrument (2026-08-24) | E0 fires on every h7-t5 action (R_a = 0, an exact zero); h8-t4's exact R_a = 14/15, 577/600, 39/40, 197/200 admit all four actions; h4-t6 excludes 0-0 (R_a 4/15) and admits 1-1 (2/45) — the first singleton |
| `fieldswap_cancel` | Slice 3 (x:019–023 Part VI §§31–42, PANEL-A7/A8): on the three parity roots exact frozen-set baselines under σ0/σ1, per-action cancellation ladders (d, r, c⁺, c⁻, c) with six-label classification at ε = 1/20, split aggregates, directional rungs (R±)^U beside exact E4 with the containment asserted, sampled E3 estimates, pairwise (B, H, q, g) masses with dominance labels under both fields, pair lifts Λ, symmetric and directional screens with exclusion soundness replayed, §36 winner stability, Stage-4 survivor-only σ1 work, the ExactRoot tier | `fieldswap_cancel run [out.jsonl=fieldswap_cancel.jsonl] [n0_field0=8 n_outer_field1=4 n0_field1=2 n_outer_frozen=8 n0_frozen=2 stream_worlds=64]` (a bare run runs) | ≈ 1 min (h8-t4 baselines 31 s + rungs 24.5 s) | instrument (2026-08-24, #38) | Λ(pin-5-5, pin-3-3) on h8-t4 = 1/300 − (−9/400) = 31/1200 (corrected 2026-08-24 from the response's 41/1200 mis-addition) is *asserted* by the bin at its default knobs and by `tests/solver_fieldswap_cancel.rs` at the gate's n0 = 2 / [2,2] epoch — the same rational at both; the first `FieldDecisionChanged` in the wild (h8-t4 Stage 4: σ0-settled 2-1 → σ1-best 5-5) and the first `Dominated` (h4-t6 pin-1-1 over pin-0-0, H = 0); the binding interpretation rule (§42): a value statement under one objective/belief/model, never pathwise safety or dominance |
| `fieldswap_motifs` | Slice 4c (x:024 Part 3, TRIPLE-A6/A7): six-motif first-split morphology over the exact-fiber correction traces of the three cancel roots — raw trace specimens capped per action (no motif tag persisted), the exact per-motif decomposition (m_k±, r_k, c_k, τ_k) with identities asserted against the cancel ladders, coordinate-difference flag counts, split-actor relation, the §3.6 residual report | `fieldswap_motifs run [out.jsonl=fieldswap_motifs.jsonl] [n0_field0=8 n_outer_field1=4 n0_field1=2 n_outer_frozen=8 n0_frozen=2 specimen_cap=8]` | ≈ 30 s (h8-t4 7–9 s per action) | instrument (2026-08-25, #44) | 453/453 correction worlds classified, residual fraction 0 on this corpus; h8-t4 2-1 is count-led (50/120 CountCommitmentFork) while 5-5 is shape-led (45/56 SuitShapeFork); `StrengthCommitmentFork` never occurred — and every number partitions **correction mass, never exposure** (TRIPLE-A6) |
| `hazard_witness` | Slice 4b (x:024 Part 2, TRIPLE-A4/A5): the one-round trump-extraction producer over every ordered pair of pinned candidates on the three cancel roots under both fields; each accept's witness verified by the single authority `verify_hazard_witness` and cross-checked against exact enumeration (H = 0); declines carry the failed hypothesis; a summary record with the histogram | `hazard_witness run <out.jsonl> [n0_field0=8 n_outer_field1=4 n0_field1=2 n_outer_frozen=8 n0_frozen=2]` (a bare run panics with the usage string) | seconds | instrument (2026-08-25, #46) | 40 ordered pairs, 0 accepts, 40 declines (LeadNotHighestTrump 28, HostileTrumpsExceedOneRound 6, HostileSuitBeater 4, LeadNotVulnerableNontrump 2) — the adopted narrowness, not a failure; the accept path is exercised only by the gates' void-engineered specimens |
| `l2_controller` | The assembled targeted field-1 controller (§8 Stages 1–5) over the predeclared corpus — three receipt roots, four step-9 flip fixtures, six count-timing shape-family members at fiber 46,558,512 — with schedule-controlled rung spend, provably-useless refusals, δ-valid degradation over the exact cap (screen budget 1/50, per-baseline side 1/800, per-E3 1/400, prefixes 128/24, ε 1/20); serializes root/row/screen/directional/spend/stage4/refusal/risk records | `l2_controller run [out.jsonl=l2_controller.jsonl] [n0_field0=8 n_outer_field1=4 n0_field1=2 n_outer_frozen=8 n0_frozen=2 exact_cap=4096 baseline_prefix=128 e3_prefix=24 ct_members=6]` | minutes (roots in parallel) | instrument + library layer only (2026-08-25, #51); never a default (CE-A7) | h8-t4 `FieldDecisionChanged` (2-1 → 5-5) with rung spend refused as provably useless; h4-t6 pruned 1/2 with no σ1 work; all four flip fixtures `FieldSensitive`; count-timing g0–g5 honestly open 6/6 |
| `ordering_bench` | Reorder-not-cull A/B (E-A15): exact frozen-set endpoints on three cheap receipt roots (h4-t6, h8-t5, h10-t6) under σ0 and σ1; `level1_evaluate` at two synthetic seed-`0x9E3779B9` roots; the same roots through a bench-owned `Solver` for both `MoveOrdering` arms (TileIndex vs CaptureFirst) with readable children/legal break counters; `hard` adds h8-t4 (fiber 1200) under σ0; values asserted identical | `ordering_bench [hard]` | ≈ 1.5 s (hard adds ≈ 3.1 s); **measured 2026-09-13: 1.10 s, all values and counters identical to the README** | instrument (2026-08-25, #53) | Values byte-identical across arms; the counters are the signal — shared children/legal 28957/44113 → 27014/42537 (deal 1) and 25552/33961 → 22803/30924 (deal 2) under CaptureFirst |
| `field_cache_bench` | A/B/C of the two surgical levers in `solver::act`'s hot path — bare `Level0Field` full replay vs cached `FieldModel` full replay vs cached with the decided cutoff — over the identical (world × candidate) grid on h4-t6 (fiber 90), h11-t5 (fiber 1120) and the sampled trick-1 regime (first 128 worlds × 7 act-shaped frozen level-1 continuations at [8, 2]); wins vectors asserted identical | `field_cache_bench` (no arguments) | ≈ 2 min (the trick-1 arms ≈ 43 s each) | instrument (2026-08-25, #55) | 3,820 → 1,729 µs on h4-t6 (2.2×), 364,791 → 303,289 µs on h11-t5 (1.2×), 42,911,487 → 41,191,341 µs at trick 1 (≈ 1.04×) — value-identical levers, honestly benched; the trick-1 regime barely moves |
| `bundle_bench` | The per-world `replay_viewer_success` loop vs `solver::bundle::bundled_set_outcomes` on three receipt roots (h4-t6 fiber 90 m=3, h11-t5 fiber 1120 m=4, h11-t4 fiber 23100 m=3) under two field configurations (cached `Level0{n0=2}`, lowest-first), fresh cold instances per route, wins totals asserted equal, exact node/field-query/settlement counters | `bundle_bench` (no arguments) | ≈ 2 s | instrument (2026-08-25, #56) | h11-t4: per-world 874,333 µs vs bundled 839,684 µs (≈ 1.04×), field queries 626,028 vs 295,433, bundled nodes 224,983 against ≤ 1,108,800 per-world plays (plays/nodes ≈ 3.3–4.9×, not the "5–6×" some prose says) — the primitive survives, the speedup does not |

### 3.3 Counted belief and the anytime proof state, 2026-08-30 → 09-01 (owned by [walt-counted-belief-era](walt-counted-belief-era.md))

The route from 399,072,960 opening-root worlds to 116,280 acting-seat hands,
then the proof state that turns exact masses into certified regret. All under
σ0 = `Level0{n0=2}` and `SupportOracle`; every binary here takes exactly
`report <out.txt>` (or `run <out.txt>`) and prints usage + exit 2 otherwise.

| Binary | Module | Gate file | Record | Owning chapter |
| --- | --- | --- | --- | --- |
| `rootinterval` (Slice A) | `solver::root_interval`, `solver::upper_cs` | `tests/solver_root_interval.rs` (6) | `probes/root_interval/run1.txt` + README | walt-counted-belief-era §3 |
| `grammarsplit` (Slice B) | `solver::grammar` | `tests/solver_grammar.rs` (8) | `probes/grammar_residual/run1.txt` + README | §3 |
| `factorbelief` (Slice C, C0–C2) | `solver::factor_belief` | `tests/solver_factor_belief.rs` (11) | `probes/factor_belief/{run1,opening_level0_run1,cache_run1,c2_run1}.txt` | §1 |
| `factorrecursion` (Slice D) | `solver::factor_belief` (`SupportOracle`, `viewer_success_mass`) | `tests/solver_factor_recursion.rs` (5) | `probes/factor_belief/recursion_run1.txt` | §2 |
| `factorresponse` (Slice E) | `solver::factor_belief` (`grammar_success_mass`) | `tests/solver_factor_response.rs` (4) | `probes/factor_belief/response_run1.txt` | §2 |
| `factorcegar` (Slice F) | `solver::factor_belief` (`refine_to_action_exact`) | `tests/solver_factor_consequence.rs` (4) | `probes/factor_belief/cegar_run1.txt` | §2 |
| `factorrefine` (Slice G) | `solver::refine` (`refine_root`; frozen as RefineV1, freeze 58) | `tests/solver_factor_refine.rs` (4) | `probes/factor_belief/refine_run1.txt` | §3 |
| `factorprofile` (Phase 2) | `solver::factor_belief` (`viewer_score_profile`) | `tests/solver_factor_profile.rs` (5) | `probes/factor_belief/profile_run1.txt` | §5 |
| `proofreport` (Phase 3) | `solver::proof_state` (`ProofState::recommend`) | `tests/solver_proof_regret.rs` (5); `tests/solver_proof_state.rs` (6, the §49 spike) | `probes/factor_belief/proofreport_run1.txt` | §4–5 |
| `frontierreport` (Phase 1) | `solver::frontier` | `tests/solver_frontier.rs` (6) | `probes/factor_belief/frontierreport_run1.txt` | §5 |
| `bellmanreport` (Phases 4/5) | `solver::residual`, `solver::covers` | `tests/solver_residual.rs` (6), `tests/solver_covers.rs` (3) | `probes/factor_belief/bellmanreport_run1.txt` | §5 |
| `extractreport` (Phase 6) | `solver::extraction` | `tests/solver_extraction.rs` (6) | `probes/factor_belief/extractreport_run1.txt` | §5 |
| `laydownreport` (Phase 7) | `solver::laydown` | `tests/solver_laydown.rs` (4) | `probes/factor_belief/laydownreport_run1.txt` | §5 |
| `openingreport` (Phase 8) | `solver::opening` (`OpeningLadder`) | `tests/solver_opening.rs` (5) | `probes/factor_belief/openingreport_run1.txt` | §6 |
| `doomreport` | `solver::doom` | `tests/solver_doom.rs` (8) | `probes/factor_belief/doomreport_run1.txt` | §7 |

| Binary | Purpose | Invocation | Runtime and output | Status | The sentence it earned |
| --- | --- | --- | --- | --- | --- |
| `rootinterval` | Slice A (§44): on six affordable receipt roots (h4-t6, h10-t6, h5-t6, h12-t6, h8-t5, h3-t5) per legal action the exact Q_a beside the δ-valid root interval — the pmake empirical-max upper (epoch 0) over a frozen `PinnedThenLevel1[2,2]` lower witness (epoch 1, provenance FIXED), δ = 1/20 per endpoint — with shortfall Q − L and excess U − Q, survivor-set evolution by prefix, worlds-to-singleton, the typed decision (DeltaRootWinner / DeltaRootSet / UnresolvedRootSet), wall per phase | `rootinterval run <out.txt> [prefix=16]` | ≈ 0.2 s; **measured 2026-09-13: 0.18 s; output identical to run1 except the six `wall-us` lines** | instrument (2026-08-30, #61) | h4-t6 settles `DeltaRootWinner{1-1; bar=7/10}` after 8 sampled worlds (the exact optimum, 13/15 vs 1/3); the four exact ties (h10-t6, h5-t6, h12-t6, h3-t5) come out `UnresolvedRootSet`, never a forced winner; all 14 rows satisfy L ≤ Q ≤ U (gated) |
| `grammarsplit` | Slice B (§45): per fixture and grammar G1 = {lowest}, G2 = {lowest, highest}, G3 = {level-1 pinned [2,2], σ0 mind, count-preservation} the exact §12 triple free/gram/dev per legal root action, verdicts (closes / ties / counterexample / root-off-grammar), lazy first-deviation witnesses, the grammar census, the root-closure line; Section B the sampled route at a declared prefix and the §8 residual-upper identity asserted | `grammarsplit run <out.txt> [prefix=64]` (a bare run panics with the usage string) | seconds (G3 ≤ ≈ 105 ms per split) | instrument (2026-08-30, #63) | G2 and G3 attain the exact optimum on all six t5/t6 fixtures; the singleton G1 fails root closure at h4-t6 (30 vs 78) and h8-t5 (64 vs 91) with a depth-4 witness after 0-0, 3-0, 6-0, 6-6; the residual-upper identity holds byte-identically |
| `factorbelief` | Slice C, four modes: `run` (C0 — one-ply branch masses by contraction over acting-seat hands vs complete-world enumeration, route parity and Z_h = Σ_t Z_ht asserted per row, six t5/t6 roots under trivial and σ0 fields plus the opening root under the trivial field); `opening-level0` (σ0 classification of all 116,280 opening hands); `cache` (C1 — first/repeat/bundled costs, extensional cache identity, cross-history sharing, opening-root identity cost); `c2` (C2 — all seven §46 coordinates from one opening-root run under σ0, declared byte accounting beside measured RSS via `/bin/ps`) | `factorbelief run <out.txt>` \| `opening-level0 <out.txt>` \| `cache <out.txt>` \| `c2 <out.txt>` | `run` seconds; `opening-level0` ≈ 5.6 s; `c2` ≈ 5.4 s cold | instrument (2026-08-30, #62 / #64 / #66) | The 399,072,960-world opening fiber stands behind 116,280 acting-seat hands (3,432 worlds per hand; gated): branch table in 8,671 µs under the trivial field; under σ0 the cold pass is 5,361,549 µs of which classification is 5,339,731 µs (45 µs/hand, 99%), the warm repeat 21,818 µs (187 ns/query, reuse ×245), cross-history cache hits 0 of 36, conservation exact at 399,072,960; memory as a declared 23,563,392-byte accounting beside a measured 63,340,544-byte maximum resident size, never one dressed as the other |
| `factorrecursion` | Slice D (§23/§47): the factorized fixed-policy recursion over `SupportOracle` vs the bundled complete-world walk, value parity asserted per row; six t5/t6 roots × two focals × two fields plus the four trick-4 roots (16 post-root plies); prints M/Z, per-route µs, the node census, σ0 states materialized; the opening root deliberately not attempted | `factorrecursion report <out.txt>` | ≈ 10 s (h4-t4 under σ0: 7,172,768 µs) | instrument (2026-08-30, #65) | Parity on every row; deepest row h4-t4 M/Z = 25039/34650 (722‰) with 121,868 conditionings — and the honest negative: the bundled route is faster (2,367,679 µs) at worlds/hands ≈ 3 |
| `factorresponse` | Slice E (§48): `grammar_success_mass` per grammar root action under σ0 checked against the Slice B enumeration split (parity asserted; free/dev from the split only — the §48 fence), two- and three-source grammars, dominance rows under the trivial field, trick-4 rows | `factorresponse report <out.txt>` | ≈ 10 s (h4-t4 under σ0 9,777,288 µs) | instrument (2026-08-30, #67) | At h4-t4 under the trivial field the two-source grammar certifies Q^G = 34650/34650 — certain make — against 34,170 (986‰) for the best source; at every t5/t6 root the grammar never exceeds the best source, and the enumeration split is 30–40× faster there (h8-t5 0-0: 12,330 µs vs 365 µs) |
| `factorcegar` | Slice F (§27–31/§49): consequence CEGAR at the field-classification bottleneck — `refine_to_action_exact` over §28 hand classes with §30 witness-pair refinement; per-root stage tables on ten gated roots under σ0 and the trivial field, then the opening root with operating points at ≥ 500/800/900/950‰ exact mass; pays one classification per support hand; measures representational compression only | `factorcegar report <out.txt>` | ≈ minutes (the opening root dominates) | instrument (2026-08-30, #68) | At the opening root under σ0, ≥ 800‰ action-exact at 36,923 classes (3 hands/class; residual floor 194‰, width 81‰) and ≥ 500‰ at 5,387 classes — but zero residual costs full fragmentation to 116,280 singletons; the record's stage table prints these operating points as exact mass 805‰ / 513‰ (stages 10 and 6) and its summary lines as residual 194‰ / 486‰ — the same two points, not four numbers |
| `factorrefine` | Slice G (§32–37/§50): `refine_root` — Section A exact-only ladder at ample budget with full traces (§35 scheduling, §34 refusals, exclusions) and typed intervals; Section B two-tier ladder (prefix 16, δ 1/20, scope 4/5); Section C budget ladder on h3-t4 and h8-t4 (§53 width-vs-cost); Section D the opening root at budget 100,000 (the affordability cliff); work units are declared forecasts, never wall | `factorrefine report <out.txt>` | ≈ 1 min (h3-t4 8.9 s, h4-t4 19.9 s, opening 11,990,554 µs) | instrument (2026-08-30, #69); `refine.rs` frozen as the RefineV1 reference (freeze 58) | The exact ladder settles every gated root (twice *without* escalating the winner); the sampled tier settles small fibers first but its trick-4 uppers are too loose to prune; at the opening root every exact item is refused by forecast and the result is UNRESOLVED 7/7 with fallback 0-0 named and never promoted |
| `factorprofile` | Phase 2 (§18, APS-A2): per gated root (ten) and two frozen focals the exact 43-bin declaring-score profile under σ0 — nonzero bins, the tail-permille curve k = 1..42, pmake at the contract (asserted equal to `viewer_success_mass`), exact expected score (§3 tail-sum), §10/§11 rescue and fragile-make bands at d = 1/5/10, the profile walk's cost beside the decided-cutoff walk | `factorprofile report <out.txt>` | seconds (h12-t4 profile walk 444,922 µs) | instrument (2026-08-31, #71) | Certain outcomes carry their explanation (h12-t6's miss is exactly 20 points in every world); the σ0-as-focal profile spikes exactly at the bid (445‰ at s = 30 on h8-t5); cross-contract reuse is void under the bid-reading σ0 and exact under bid-blind semantics (the frozen gate specimen) |
| `proofreport` | Phase 3 (§33, APS-A6/A7): for seven roots (six enumerable + h3-t4) run RefineV1 two-tier ample (prefix 16, δ 1/20, scope 1/2), install the facts in a `ProofState`, add exact lowest-first continuation profiles for up to three actions, print the §33 block: action, policy id, pmake floor B_exec, global upper U*, certified regret Γ = U* − B_exec, proof class, score floor/ceiling with fragile/rescue d = 1 bands, risk scopes | `proofreport report <out.txt>` | 14.1 s in run1 (h3-t4 13.35 s); **measured 2026-09-13: 7.35 s (h3-t4 6,826,743 µs), same survivors and recommendations** | instrument (2026-08-31, #73) | Certified regret zero far from certain make (h5-t6 Γ = 0 at 444‰); on h3-t4 the settled action is 3-1 (Q = 350‰) but the recommended *executable* policy starts 4-4 at floor 267‰, Γ = 83‰ — pmake belongs to the policy, not the first tile (the ‰ values are probe output; the regret laws are gated) |
| `frontierreport` | Phase 1 (§35, §39–44): per root × goal (SelectAction, RecommendEpsilonPolicy(0), StrengthenToExact) the Frontier starts from the zero-fact top state and buys work items (baseline profile, exact value, extraction, residual interval, the §41 exact-survivors macro) under the declared Z/3Z forecast cost model, printing each purchase with its §42 bound and debt trajectory, the refusal census, the §33 recommendation | `frontierreport report <out.txt>` | ≈ 13 s (total 13,340,983 µs) | instrument (2026-08-31, #75) | Goal separation is real money: h10-t6 and h3-t5 certify Γ = 0 for 1Z while SelectAction costs 7Z and 10Z; h3-t4 SelectAction settles at 16Z without buying any extraction while its ε-goal spends 28Z (≈ 15Z uppers-first — a poor forecast wastes, never weakens) |
| `bellmanreport` | Phases 4/5 (§61/§62, APS-A8/A9): per root × action the §61 F staircase (stage intervals, exact mass, class census to the action-exact endpoint) beside the exact §36 response and the stage-1 tail envelope's §7 straddle, then the §62 cover table (verified movement bound vs the §5 arithmetic envelope, named resources, derived rescue-band upper, COLLAPSED/OPEN) on the six enumerable roots plus h3-t4 | `bellmanreport report <out.txt>` | ≈ 15.7 s (total 15,696,614 µs, h3-t4 dominates) | instrument (2026-08-31, #76) | The staircase is a real anytime object: **h3-t4 action 3-1 walks [66,830]‰ (70 classes, 31 exact) → [350,350]‰ in six stages (0–5); action 4-1 starts at [145,606]‰ (50 classes) and reaches [271,271]‰** (record lines 133–147; `probes/factor_belief/README.md` line 531 carried the [145,606]‰ start under 3-1 until this branch's `6a319216` corrected it against the record on 2026-09-13); h12-t6's cover certifies V* = 0 (gain 0 collapses both actions); h4-t6's cover gain 11 beats the arithmetic envelope 12 by one point with the 134‰ hazard visible; first-generation covers are vacuous at rich roots (the §70 caveat, live) |
| `extractreport` | Phase 6 (§30/§63, APS-A9): per root install RefineV1 two-tier facts plus lowest-first baseline profiles, read Γ, run the `argmax-extraction-v1` producer (full legal set, lowest-tile tie rule), read Γ again; prints the executable bar's rise, DAG sizes, the post-extraction §33 block, the §63 residual verdicts (closure / escape / empty-class) for one-, two- and three-source grammars | `extractreport report <out.txt>` | ≈ 29 s (total 29,183,927 µs) | instrument (2026-08-31, #74) | h3-t4 certified regret 83‰ → 0‰ *exactly* (the 3-1 continuation is a 12,420-state DAG; B_exec 267‰ → 350‰; the recommendation switches 4-4 → 3-1); h8-t5 Γ 282‰ → 10‰; h4-t6 keeps Γ = 133‰ honestly (cross-action dominance leaves the winner's upper vacuous until a §36 upper fact prices it); the two-source grammar escapes on every h3-t4 action while every multi-source t5/t6 verdict is empty-class — Slice E's "ties free" explained as saturation at ≤ 2-tile focal states |
| `laydownreport` | Phase 7 (§15–17/§64, APS-A9): `classify_root`'s typed census — Laydown / ForcedMake(+witness) / AdversarialPolicyMake(lowest-first) / PolicyCertainMake — with universal-walk node counts and wall on three synthetic fixtures (boss-chain bid 33, already-made bid 20, loose-boss bid 33; sixes trump, 3-tile hands) and the four t6 receipt roots | `laydownreport report <out.txt>` | < 1 s (boss chain 373,469 µs) | instrument (2026-08-31, #77) | The boss-chain fixture certifies as a TRUE Laydown in 1,492,276 walk nodes (proved by walk, not phrase); the already-made root classifies in 3 nodes (§17 zero-cost); the loose-boss counterexample refutes fail-fast in 280 nodes; receipt root h10-t6 is a real Laydown with forced witness 2-2; the universal walk ranges over a per-seat relaxation — certification-sound, possibly conservative |
| `openingreport` | Phase 8 (§65): one append-only `ProofState` at the receipt opening root h0-t1 (Z = 399,072,960, 7 legal leads, contract 30 on threes) driven through `OpeningLadder` stops — zero budget; sampled p = 16; p = 64; p = 256 with census and frontier Z/2; p = 512 with frontier Z/2 — printing the full §65 panel at every stop (per-action intervals, survivors, proof/exec bars, U*, Γ, width debt, cylinders, count-threat cells, covers, risk, recommendation, verdict) and the serialized state size; δ = 1/100 per endpoint, scope 3/5, ε = 1/4; flushes after every stop | `openingreport report <out.txt>` | stop walls 69 µs / 5.7 s / 30 s / 10.5 min / 76 min — ≈ 1.5 h total | instrument (2026-09-01, #78) | The bar climbs 0 → 407 → 594 → 732‰ and Γ falls 1000 → 592 → 405 → 267‰ with the recommendation migrating 0-0 → 2-1 → 6-5; the sampled tier PLATEAUS p = 256 → 512 (U* 1000 → 999‰ only); all 29 frontier refusals are pure affordability; 56 facts / 10,439 bytes resumed bytewise (§67.5 resume ≡ uninterrupted, gated); verdict honest UNRESOLVED at ε = 1/4 (Γ 267‰ vs 250‰) — §65's first target met in its certified-regret form. The δ endpoints differ from RefineV1's Section D because that section declared δ = 1/20 (stated in the header, not drift) |
| `doomreport` | The doom census (`solver::doom`, the §70 structural producer, ∀-fail dual of the §16 hierarchy): Part 1 the census vs per-world enumeration truth on six enumerable roots; Part 2 the h0-t1 priority census per lead (node budget 500,000, walk cap 100,000, max level 3, top-8 descent); Part 3 the God grid (two hand-built crusher worlds + a stride-512 grid over the S2 support); then the composed §65 panel (sampled stops p16/p64 plus doom uppers). Scout modes for one action or an enumeration-cost prefix | `doomreport report <out.txt>` \| `doomreport scout <action_idx> <node_budget> <walk_cap> <max_level> [top_k]` \| `doomreport enumscout <action_idx> <outer_limit>` | ≈ 45 s of sampled stops plus census walks (grid 3,158,068 µs; p64 stop 34,631,624 µs) | instrument (2026-09-01, #79) | Where doom lives the census recovers 809–1000‰ of it (h8-t5 0-0: 17 of 21; h4-t6 0-0: 56 of 60; h12-t6 whole-fiber in one decided read); at the opening root total certified doom is 0 worlds, and the God grid shows the zero is real (0 of 228 grid worlds and both crushers let the world-aware viewer make against σ0), so doom uppers cannot move the h0-t1 999‰ ceilings. **Corrected reading (walt/DISCREPANCIES.md, 2026-09-03):** the era's sentence "the plateau's Γ is overwhelmingly the info-consistency price" overclaims — a zero doom census moves only d_phys and does not by itself distinguish d_info from d_policy (SC-A1); U0 typed the opening root `UnknownGodGap` on all seven actions (SC-A4). The honest statement is that the 267‰ is UNKNOWN in its split, with the sampled upper's looseness and the policy gap both live candidates; the census's working domain is the endgame and in-play middlegame |

### 3.4 Model belief and the unified player, 2026-09-01 → 09-03 (owned by [walt-focal-horizon-era](walt-focal-horizon-era.md) §2–§4)

The field itself as hidden state (Ξ = Ω × Θ), the price of fusing over it, the
God-gap censuses that located the fusion horizon, and the one decision core
that runs every instrument as a tier. "The unified player" (`solver/unified.rs`,
UP0) is distinct from "the unified crate" (the 2026-08-24 fold).

| Binary | Module | Gate file | Record | Owning chapter |
| --- | --- | --- | --- | --- |
| `modelbeliefreport` (MB0) | `solver::model_belief` | `tests/solver_model_belief.rs` (8, G1–G8) | `probes/factor_belief/modelbelief_run1.txt`; `walt/briefs/BRIEF-MB0.md`, `MB0-COLLISION-NOTES.md`, `MB0-HANDOFF-BUILDER2.md` | §2 |
| `modelbeliefrecursionreport` (MB1) | `solver::model_recursion` | `tests/solver_model_belief_recursion.rs` (7; gate M6 pins the h8-t4 3-1 specimen) | `probes/factor_belief/modelbelief_recursion_run1.txt`; `walt/briefs/MB1-REPORT.md` | §2 |
| `unifiedreport` (UP0 / UP1a) | `solver::unified` (`UnifiedPlayer`) | `tests/solver_unified.rs` (18), `tests/solver_unified_carry.rs` (5) | `probes/factor_belief/unified_run1.txt` (UP0), `unified_run2.txt` (UP1a); `walt/briefs/UP0-REPORT.md`, `UP1A-REPORT.md`; `walt/MAP.md` object #8 | §4 |
| `godgapreport` (U0) | `solver::godgap` | `tests/solver_godgap.rs` (6) | `probes/factor_belief/godgap_run1.txt`; `walt/briefs/U0-REPORT.md` | §3 |
| `horizonreport` (U0b) | `solver::horizon` | `tests/solver_horizon.rs` (5) | `probes/factor_belief/horizon_run1.txt` (12,613 lines); `walt/briefs/U0B-REPORT.md` | §3 |

| Binary | Purpose | Invocation | Runtime and output | Status | The sentence it earned |
| --- | --- | --- | --- | --- | --- |
| `modelbeliefreport` | MB0's §75 first report: per enumerable receipt root — physical and augmented masses, prior/active types by seat, exact branch masses by public action along one observed line with posterior type weights, fixed-policy mixture value, exact mixture response, type-revealed separated upper U^sep, per-action model-fusion price Φ, typed-vs-merged census, wall, declared memory accounting, (ω, θ)-enumeration parity; then the §76 go/no-go criteria as YES/NO lines; also a labelled SYNTHETIC carrier mixture. Epoch F₀ = `Level0{n0=2}`, F₁ = `Level1{n_outer=2, n0=2}`, prior (1/2, 1/2) per hidden seat | `modelbeliefreport report <out.txt>` (exactly two arguments; asserts otherwise) | 5.23 s (total wall 5,229,376 µs, single-threaded; RSS 18,768 KB) | instrument / MB0 report of record (2026-09-01, #82) | Φ_a = 0 on all 14 root-action coordinates of the t5/t6 corpus; §76 criteria: point-mass parity SCOPED YES (F₁ exact only on {h12-t6, h10-t6}, the raw σ1 authority's entire terminating domain — the caveat is part of the result's name), posterior closure YES, mixture ≠ point-mass on 3 root actions YES, point-mass upper sometimes strict NO, type dimension small YES (≤ 8 profiles live); the run also found the σ1 sampler hazard that became #83 (§1) |
| `modelbeliefrecursionreport` | MB1: per root the posterior-carrying model-space recursion — depth reached, posterior evolution, mixture value vs MB0's root value (must match), F₁ read count vs declared cap, wall; then the earlier-root Φ table (h8-t4, h12-t4, h3-t4) and the strictly pre-t4 attempt (h8-t3) under declared read caps (4M at MB0 roots; 12M trick 4; 7M trick 3); then the field-identity fence census | `modelbeliefrecursionreport report <out.txt>` \| `modelbeliefrecursionreport measure <hand> <trick> <cap>` (bare affordability probe on one root; cap 0 = uncapped) | ≈ 40 min for the declared run (`MB1-REPORT.md`: 2,379 s total — h8-t4 97.7 s, h3-t4 412.2 s [record line 247: 412,206,767 µs], h8-t3 refused after 1,864 s) | instrument / MB1 report of record (2026-09-02, #85) | The model-fusion price is STRICTLY POSITIVE at trick 4: h8-t4 2-1 Φ = 47/9600 (4‰), 3-1 = 38/9600 = 19/4800 (3‰, gate M6's pinned specimen), 3-3 90/9600 (9‰), 5-5 58/9600 (6‰); h3-t4 3-1 173/46200, 4-1 157/92400, 4-4 37/6600, 6-4 7/2200 — eight substantive coordinates, eight strict; MB0's 14 zeros re-typed 7 substantive / 7 vacuous; h8-t3 (Z = 59,976) refused on all five actions at the 7,000,000-read cap (35,000,039 reads spent) — the wall for the model-space recursion is trick 3 |
| `unifiedreport` | The UP0/UP1a TRANSCRIPT: walks MB0's six roots + h8-t4 to terminal with `UnifiedPlayer` choosing every seat's action under three declared budget rungs — lean (enumeration cap 32, mixture off), ample (40,000 / 256 / 4M reads, join reading on), model (8 / 256 / 4M — caps swapped so tier (c) answers) — recording per decision the tier (a)–(e), recursion direction/space, reads (enum/mix/carry/field), typed refusals, posterior consultation/materialization, wall; then a census table, spend and wall per rung, the JOIN readings, the refusal census. Explicitly not an evaluation | `unifiedreport report <out.txt>` \| `unifiedreport walk <hand> <trick> [lean\|ample]` (one root to stdout) | ≈ 32 s for `report` (`unified_run2.txt`: lean 2,215,942 µs wall = 11,605 µs deciding + 0 µs recording; ample 21,565,653 µs; model 8,573,990 µs); **measured 2026-09-13: `walk 8 5 lean` 0.18 s, 12 decisions, declaring MADE, every decision printed with its tier and refusals** | instrument / transcript of record for UP0 (#86, 2026-09-02) re-run under UP1a (2026-09-03) | Under the lazy carry the lean rung spends 0 µs recording the line where UP0 had spent 2,105,672 µs — 99.4% of its wall — carrying a posterior no tier read (`unified_run1.txt`); at the join (27 readings where both recursions priced the same state) 9 value moves and 2 argmax flips: h3-t5 t6 p2 s2 model 6-4 vs fixed-field 3-1 (600‰ vs 833‰), h8-t4 t4 p2 s3 model 0-0 vs 6-2 (714‰ vs 800‰); refusal census 63 ProofStateUnavailable / 27 EnumerationUnaffordable / 12 MixtureUnaffordable / 2 PosteriorFalsified; no play-strength claim; totality, refusal discipline, lazy ≡ eager on all 216 decisions and the first two flips are gated |
| `godgapreport` | U0 (SC-A4): per root-action coordinate the fiber mass, doomed mass, God upper U^God, exact Q where affordable (exact fiber cap 40,000, profile cap 12,000), the §8 decomposition (d_phys, d_info, d_policy), the §48 result type (GodTightPolicy / PositiveGodGap / GodUpper / UnknownGodGap), the extracted God-tight policy with its equality receipt, refusals and the cost bill; Part 2 the h0-t1 opening root on a cheap priority census (50k/20k/top-8); Part 3 the §38 fusion-horizon table by trick; Part 4 a two-regime summary | `godgapreport scout <hand> <trick> [exact-cap] [profile-cap]` \| `godgapreport report <out.txt>` | minutes (h3-t4 coordinates ≈ 2 s each; the opening-root doom census 26–41 s per lead, record lines 157–177) | instrument (2026-09-02, #84) | The fusion horizon on the ten-root corpus is trick 5: t4 16 coordinates, 4 God-tight (all whole-fiber doom), 12 positive gap, max Φ = 43/1925 (22‰); t5 6/6 God-tight; t6 8/8 (2 vacuous); h0-t1 7/7 UnknownGodGap; God-tight policies extracted 18 of 37; e.g. h3-t4 3-1: Q = 1349/3850 (350‰), U^God = 2111/5775 (365‰), d_phys 634‰, d_info 1/66 (15‰), d_policy 0 — a measurement on a declared corpus, never a theorem |
| `horizonreport` | U0b: per (root, contract, cut depth) every frontier belief node's mass, doomed mass, exact Q, God upper and price Φ; the God-tight / vacuous / positive-gap tally, max and mass-weighted Φ; the root consequence — exact value vs a §39 fusion cut at that depth, and whether the cut flips the root play. Corpus: the four trick-4 roots cut at 4 and 8 plays under the receipt contract and the sweep {30, 33, 36, 39, 42}, the two trick-5 roots cut at 4, h8-t3 cut at 4 under node cap 12,000 | `horizonreport scout <hand> <trick> <cut> [contract] [node-cap]` \| `horizonreport report <out.txt>` | hours for `report` (h8-t3 alone 797,430,769 µs — 13.3 min, "14 minutes" in prose — with 289,407,472 reads; record line 11932) | instrument (2026-09-03) | h3-t4 cut 4 prices 779 frontier nodes (477 God-tight substantive, 131 vacuous, 171 positive gap), max Φ 1/3, mass-weighted 14‰, root over-pricing 1/66 (15‰), no flip; **h8-t3 (Z = 59,976) solved exactly under σ0: Q* = 28859/29988 (962‰) argmax 1-1, while a cut at its trick-4 frontier prices 29803/29988 (993‰) argmax 3-3 — over-pricing 236/7497 (31‰) and a root-play FLIP** (record line 11934) |

### 3.5 The focal-horizon hierarchy and the Gran anchors, 2026-09-04 → 09-05 (owned by [walt-focal-horizon-era §5](walt-focal-horizon-era.md) and [walt-gran-anchors](walt-gran-anchors.md))

| Binary | Module | Gate file | Record | Owning chapter |
| --- | --- | --- | --- | --- |
| `focalreport` (FH1/FH2/FH3) | `solver::focal_horizon` (the engine), `solver::focal_ladder` (budgeted passes over a fact store) | `tests/solver_focal_horizon.rs` (10), `tests/solver_focal_ladder.rs` (10), `tests/solver_focal_anchors.rs` (4; anchors (ii)/(iii) — anchor (i) is probe-only) | `probes/factor_belief/focal_run0.txt` (FH1 scout-corpus), `focal_ladder_run1.txt` (FH2), `focal_run1.txt` (FH3, the report of record); `walt/briefs/FH1-REPORT.md`, `FH2-REPORT.md`, `FH3-REPORT.md`, `FH4-AUDIT.md`; `probes/factor_belief/README.md` FH sections | walt-focal-horizon-era §5 |
| `granrun` | `solver::waking::WakingSeat` driven directly under the `waking_bridge` epoch; `rules::replay::replay_hand` | `tests/solver_waking.rs` (the seat); the fixtures are validated by rules replay, not by a gate | `probes/gran/{g1.receipt.txt, g2g3.receipt.txt, g1-replay.jsonl, g1-driven.jsonl, summary-replay.txt, summary-driven.txt, README.md, summarize.py}`; `walt/briefs/MORNING-2026-09-05.md` | walt-gran-anchors §3–§7 |

| Binary | Purpose | Invocation | Runtime and output | Status | The sentence it earned |
| --- | --- | --- | --- | --- | --- |
| `focalreport` | The focal-horizon hierarchy instrument (FH-A1..A11): `scout` runs `solver::focal_horizon` at one (root, trick, k [, contract, node cap, tail σ0\|lowest, exact]) printing per-action [L_{a,k}, U_{a,k}], width, the (U − Q) + (Q − L) split when priced, verdict, π_k, L_exec, U*, Γ_k, focal depth and the spend line; `scout-corpus` is FH1's record over the four trick-4 roots × k ∈ {0, 1, 2}; `ladder` runs `solver::focal_ladder` through a schedule of k:ceiling steps (suffix memo on unless `nomemo`; `cap=N`) printing outcome, reads, residual frontier, fact-store movement, suffix hits and the derived root view; `ladder-record` is FH2's pinned record (h8-t4, h3-t4); `report` is THE REPORT OF RECORD (FH3): 33 (root, contract) coordinates × k ≤ 3 with the direct engine and the memo-on ladder, the §41 laws (containment, nesting, survivors shrink, Γ never rises, ladder ≡ direct) asserted per coordinate, the FH8 anchors, run on 6 workers then the h8-t3 anchor alone (its Q_a *cited* from `horizon_run1.txt`, never recomputed) | `focalreport scout <hand> <trick> <k> [contract] [node-cap] [sigma0\|lowest] [exact]` \| `scout-corpus <out.txt>` \| `ladder <hand> <trick> <contract\|receipt> [nomemo] [cap=N] <k:ceiling>...` \| `ladder-record <out.txt>` \| `report <out.txt> [h<hand>-t<trick> ...]` | scout-corpus minutes (h4-t4 16.6–26.5 s per horizon, h3-t4 6.2–8.5 s, h8-t4 1.7–2.9 s); `report` 1,435 s (23 min 55 s) with **peak RSS 19.4 GB** (17.1 GB for the h8-t3 ladder alone; README lines 750–753) — a limit of the current instrument's per-node policy store, not of the mathematics | instrument (2026-09-04; FH1 `1e213bdb`, FH2 `dc515ac0`, FH3 `fc171e1f`, FH5 fixes `b6de5a25`) | FH1: h3-t4 k = 0 UNRESOLVED {3-1 4-4 6-4} Γ 76‰ → k = 1 SETTLED 3-1 Γ 12‰ → k = 2 Γ 0; h4-t4 SETTLED 6-5 at k = 0; h8-t4 needs k = 2 (survivors 4 → 3 → 1); at k = 1 the width is policy gap (Q − L 9–41‰), not fusion price (U − Q ≤ 3‰). FH2: at h3-t4 the memo-on ladder's k = 2 pass costs 421,175 reads against FH1's direct engine's 2,829,306 (`focal_run0.txt` line 126) and an interrupted k = 1 pass at ceiling 1.2M reads is already SETTLED 3-1. FH3: over 33 coordinates every live trick-4 coordinate settles by k ≤ 2 with Γ₁ ≤ 45‰; the h8-t3 anchor (bid 30) goes k = 0 Γ 141‰ (5 survive) → k = 1 100‰ → k = 2 34‰ ({1-1 2-1 3-3}) → k = 3 SETTLED 1-1 at 28859/29988; the cut-4/cut-8 argmaxes at h8-t4 36/39 (3-3, 5-5) are never certified |
| `granrun` | The Gran-anchor probe runner — a VARIANT surface driving the waking seat directly (σ0 `Level0{n0=2}`, σ1 `Level1{4,2}`, frozen [8, 2], `ActConfig::interactive`, wake budget 24, exact wake cap 1024, escalation caps 4096/128/24; the same `WALT_*` env knobs as `waking_bridge`): `validate` replays a hand-transcribed Plunge anchor through `rules::replay::replay_hand` (deal, every follow, winner, points, verdict re-derived); `validate-partial` does the same over a record that stops early and enumerates every residual-tile assignment the prefix admits (`known Sn: h-l` lines pin tiles); `replay` seats the waking seat in one chair against the recorded line, one census record plus one compare line per decision; `driven` plays the whole hand with the waking seat at all four chairs from the anchor's deal | `granrun validate <fixture.txt>` \| `granrun validate-partial <fixture.txt>` \| `granrun replay <fixture.txt> <seat e.g. S2> <out.jsonl>` \| `granrun driven <fixture.txt> <out.jsonl>`; fixtures `probes/gran/g1.receipt.txt`, `g2g3.receipt.txt`; `python3 walt/probes/gran/summarize.py <out.jsonl>` | validate < 1 s; replay 25.04 s on G1 (trick 1 12.55 s); driven 166.11 s | instrument / variant surface (2026-09-04, `32aa14f1`, `8174fa83`) | G1 (the failed hand: bidder S0, bid 30 on sixes; S2 "Gran" = 1-0 2-2 4-1 4-2 5-2 6-2 6-4; T0 25 – T1 17, set) replays mechanically — VALIDATED; the waking seat at S2 agrees with the record on 6 of 7 decisions and plays the 6-4 at trick 1 where Gran played 6-2, **but no wake fired there** (fiber 46,558,512, sampled-open after 24 paired worlds; σ0's own `unresolved-level1` fallback chose 6-4), so the trick-1 flip is an epoch/baseline difference, not partner-modelling; the one real wake (trick 5, fiber 300, exact route) selected 4-2 off σ0's 1-0 and matched the human game; driven all-four-chairs T0 26 – T1 16, still set, 28 decisions, zero wakes. G2/G3 (the made hand): 24 of 28 tiles recorded, S2's information set fully known, residual {4-1 4-4 5-3} 6-way ambiguous across S0/S1/S3 |

**Two unmerged branches carry instruments this page does not catalogue**
(they are not at `c00717d1`; [walt-gran-anchors §10](walt-gran-anchors.md)
owns their status): `walt-g1-l2` adds a `fixture` mode to `level2` (the
2026-08-17 level-2 player run on G1, G2 and a synthetic threes-trump lock —
level 2 holds the 6-4 at both legal G1 nodes; the release `level2` binary on
this machine is main's build and has no such mode), and `walt-o5` adds the O5
probes `voidcensus`, `o5flip`, `o5level1`, `o5match`, `o5anchor` with their
gate `tests/solver_inner_voids.rs` (7 tests). Neither is a default anywhere.

### 3.6 Scheme, the gym, and learning, 2026-09-06 → 09-07 (owned by [walt-scheme-fix](walt-scheme-fix.md), [walt-gym](walt-gym.md), [walt-partnership-program](walt-partnership-program.md))

The `partnership` oracle behind the batteries is in §1. Every binary here was
landed with the full walt gate deliberately waived (the partnership packet's
295 s watchdog discipline); the focused suites named below are what was run.

| Binary | Module | Gate file | Record | Owning chapter |
| --- | --- | --- | --- | --- |
| `scheme` | `walt::scheme` (`syntax`, `registry`, `eval`, `belief`, `dynamics`, `policy`; imports rules/kernel only, no solver) | `tests/scheme.rs` (19), `tests/scheme_dynamics.rs` (10), `tests/scheme_policy.rs` (6) | `walt/scheme/README.md` runnable examples (`walt/scheme/examples/*.scheme`) | walt-scheme-fix §2–§13 |
| `partnership_gym` | `walt::gym` (`GymField`: L1 partner at 40/8 voidless, L0-8 opponents, seed 420600, lowest ties) | `tests/gym.rs` (8) + 25 Python tests (`experiments/partnership/test_gym*.py`) | `walt/gym/{mining.json, scenarios/, collections/, specs/, benchmarks/, queries/, RESULTS.md, DISCOVERY.md, BID-MAKING.md, SPECIFICATIONS.md, PARTNERSHIP-COMPOSITION.md}` | walt-gym §1–§8 |
| `policy_lab` | `walt::policy_search` (`request`, `program`; the sampled-table constructor) + `walt::scheme::policy` | `tests/policy_search.rs` (9), `tests/scheme_policy.rs`, `tests/scheme_dynamics.rs` | `experiments/partnership/campaigns/policy-synthesis-v1/RESULTS.md`; `walt/scheme/COMPOSITION.md` | walt-partnership-program §8; walt-scheme-fix §9–§10 |
| `relational_lab` | `walt::policy_search::{relational, learning_io, learning_eval, prices}` | `tests/relational_learning.rs` (6), `tests/relational_runtime.rs` (6), `tests/information_prices.rs`, `tests/learning_io.rs` | `experiments/partnership/campaigns/relational-learning-v1/RESULTS.md`; `walt/scheme/RELATIONAL.md`, `INFORMATION-PRICES.md` | walt-partnership-program §8; walt-scheme-fix §11 |

| Binary | Purpose | Invocation | Runtime and output | Status | The sentence it earned |
| --- | --- | --- | --- | --- | --- |
| `scheme` | Offline Scheme/Fix v1 query runner (no player code, no private-policy inputs): reads a Fix query file and a receipt coordinate (hand, trick, seat via `ReceiptDecision`), enumerates the uniform legal support under a world cap and work budget, optionally conditions on another query, summarizes event probability / certainty / answer-presence masses, optional explicit selector, optional comparison with a second query (answers \| existence) yielding a counterexample; `--registry` lists the 28 versioned predicates (`scheme-v1/straight-v0.4`, 5 of them World-access); the whole report is buffered so a budget refusal prints no partial result | `scheme --query FILE [--receipt FILE (default rob/receipts/verify_player.txt)] [--hand N=0] [--trick 1..7 =6] [--seat 0..3 =0] [--max-worlds N=40000] [--work N=10000000] [--condition FILE] [--compare FILE] [--comparison answers\|existence] [--selector first\|uniform]` \| `scheme --registry` \| `scheme --help` | milliseconds; **measured 2026-09-13: the partner-count example on h0-t6-S0 prints `support-worlds: 6`, `event-probability: 1/3`, `answer-presence: [count-tile=4-1] probability=1/3`, `work: 1084` in ≈ 10 ms (11 ms wall)** | instrument / expression tool (2026-09-06, `b764665f`; layers added `08fad726`, `c00717d1`) | Scheme was "invented to compress, commissioned to express" — an exact finite-domain query over a seat's belief, with the Viewer/World access split typed and every computation charged a work budget that refuses rather than truncates |
| `partnership_gym` | One atomic exact gym assessment per process: own hand + public play in (`decl`, `bid` = 30 only, `bidder`, `seat`, `hand`, `plays`, `seed`), JSON out with schema `partnership-gym-v1` — `root_id`, `worlds` (the exact fiber), `legal`, `offers` (the `OFFER_QUERY` Fix evaluated), `leader`, `prefix`, `banked`, `trick`, `remaining`, `scheme_identity`; then per legal action `success_mass`, `score_bins`, `policy_id`/states and same-world traces under `GymField`, and `best`. `--inspect` stops after the header; `--query FILE` matches an arbitrary Scheme query; `--check-query FILE` compiles a query and prints its identity | `partnership_gym [--inspect] [--query FILE] [--max-worlds N=400] [--query-work N=20000000] [--partner-worlds N=40 (1..640)] < request` \| `partnership_gym --help` \| `--check-query FILE`. The outer runner is `python3 experiments/partnership/gym.py {report\|show ID\|verify\|mine\|discover\|select\|run\|generate}`, long modes under `experiments/partnership/packet/texas42-partnership-launch-v0.1/tools/run_capped.py --seconds 295` | header instantly; a full assessment seconds (60 keys in 1.305 s on 10 workers); **measured 2026-09-13: `--inspect` on an opening-root request answers `worlds 399072960, legal [0..6], offers []` in under 10 ms; `gym.py show advantage-01` prints "5-0: 30/36 success optimal; 5-3: 8/36" in 0.04 s** | instrument / exam evaluator (2026-09-06 `c59f1115`; extensions 2026-09-07 through `1df741db`) | Exact answer keys relative to a declared field: 60 mined candidates → 60 audited keys (21 strict count-offer advantages, 13 disadvantages, 26 none); pupils L1 5/6, L2 Partner 6/6; Scheme-driven discovery found 170 strict late-game coordinates and 433 bid-making exercises (367 unique best plays, 26 certain make/set swings); the composed 30-position exam scored L1 24/30 vs L2 Partner 26/30 (mean regret 1643/205200 vs 959/205200) — finite-domain evidence, explicitly not a general strength ranking |
| `policy_lab` | One seed's paired policy-construction experiment: for a fixture root (random own hand, fixed root hand with `--hand`, or an existing gym `--request`) and a nested training-world stream, three arms per schedule prefix — fresh (empty cache), persistent (retained exact search state), compose (singleton-donor `ActionPool` union) — under a node budget and a frozen field (`hash-legal` \| `l0-8` \| `gym`), replaying each extracted `TablePolicy` on train/test worlds, exporting it as a Scheme `PolicyProgram`, re-parsing/compiling/re-replaying the serialized program (parity asserted), optional `--exact-test` full-fiber repricing (fiber ≤ 10,000), optional `--artifact-dir`; prints `policy-lab-v1` JSON rows plus replay records | `policy_lab --seed N --tiles 2..7 --samples 1,2,4,8 --test-worlds N --node-budget N --decl 0..7\|9 --mode random-own-hand\|fixed-root-hand [--hand seven,ids] [--field hash-legal\|l0-8\|gym] [--artifact-dir PATH]` or `policy_lab --request FILE --seed N [--exact-test 1]`; `policy_lab --help`. Defaults: seed 420600, tiles 7, samples 1,2,4,8,16,32, test 128, node budget 2,000,000, decl 0. Outer runners `experiments/partnership/policy_campaign.py`, `policy_gym.py` | 32 opening roots on the hash-legal field 17.189 s (10 workers); 12 roots on the native L0-8 field 64.590 s | instrument (2026-09-07, `08fad726`) | The persistent arm roughly halves search with exact completed-policy parity (287 + 72 three-arm comparisons agree); complete singleton-donor composition preserves the training optimum but does not repay its setup cost; exact table policies fit training worlds far better than held-out worlds |
| `relational_lab` | Bounded native jobs for the shared-policy campaign: `generate` finds the first qualifying own/public root in a seed range (tiles 2..4, support ≤ `--max-worlds`, unresolved bid 30, ≥ 2 legal) and prints a `relational-root-v1` request; `fit` learns a shared one-mode `RelationalLearner` program from a lessons file under clause/beam/AST/work caps and writes `candidate-N.policy` files with digests and metrics; `evaluate` runs `learning_eval` over a request + policies directory with a field, seed, samples, work and optional information prices | `relational_lab generate [--seed 910000] [--tiles 3] [--max-worlds 512] [--attempts 1000] [--decl 6]` \| `relational_lab fit --lessons FILE --output DIR [--clauses 3] [--beam 6] [--ast 256] [--work 2000000]` \| `relational_lab evaluate --request FILE --policies DIR --output DIR [--field gym] [--seed 0] [--samples 16] [--work 2000000] [--prices on]`; `relational_lab --help`. Outer runners `experiments/partnership/relational_campaign.py`, `relational_exam.py`, `verify_relational.py` | generate milliseconds; the gym-field pipeline 39.411 s (10 workers), the L0-8 pipeline 11.352 s (8 workers) | instrument (2026-09-07, `c00717d1` = HEAD) | Learned actors have ≤ 3 clauses from a fixed 14-clause grammar and still trail sampled tables; one frozen-table fallback substitution gains an uncertain 0.771 pp; L0 development selects the empty baseline; the fixed price basis tightens no tested bound, and with fixed lowers a state-common upper cannot rerank actors (now tested explicitly); 8,694 independent full-game replays pass — positive engineering, negative strength |

## 4. What each instrument established

Headline numbers, each with its record and its tier, beside the honest
negative that travelled with it. Everything is EXPLORATORY; "gated" names the
test file that pins the *law*, never a promotion of the number.

| Instrument | Headline | Record | Pinned by | The honest negative beside it |
| --- | --- | --- | --- | --- |
| `m3probe` | Exact pmake lead on the frozen carrier: 3-3 at 16078667/16588800 | `probes/m3/results_2026-08-17.txt`; reproduced byte-for-byte 2026-09-13 | probe record, not gate-pinned | One frozen trick-4 root; not a trick-1 statement (P-A21) |
| `ladder` | Exact enumeration at t=4 in 4.3 s | `probes/m3/ladder_results_2026-08-17.txt` | in-binary assertion | Dies at t=3 (59,976 worlds), refused at t=1 |
| `scenario` / `level1` / `level2` | The opening lead 5-5 survives every rung and every level | `probes/m3/{scenario,level1,level2}_results_2026-08-17.txt` | none | Sampled; support-on-sample ≠ certainty; level-2 cost figure in prose ("25–50×") is not in the results file |
| `walt_bridge` (arena) | 32/48 games against the mk5 E[Q] champion; pooled 630/1152 (54.7%), McNemar z = +6.28 over 6,015 paired contracts; zero rules divergences over ≈ 15k decisions | `probes/m3/arena_results_2026-08-17.txt` | exploratory arena outcome about play, never about values | walt loses points and wins marks — the pmake objective visible in data; the 3×384 pool is internally consistent on the pre-PiKey-fix binary |
| `divergence` | 27.8% divergence, in tricks 1–4 and (at large gaps) partner-bid/defense | `probes/m3/divergence_results_2026-08-18.txt` | none | Self-graded on level-2's own table; the referee protocol was never run |
| `bidcurve` | θ = 11/16 (0 overbids, 0 missed bids at n40 vs the n200 reference) | `probes/bidcurve/ANALYSIS-2026-08-19.txt` | none (a constant in `webtable.rs` and `walt-wasm`) | Monotonicity in b violated 1357/1822/1536 times — adjacent cells are different games |
| `tiltaudit` | The race is faster per lead and the arena is a dead heat (the audit's own word for paired makes race-only 1 / full-only 2 / both 11 over 24 deals on 2026-08-19 — not the O5 match sentence that `SCENARIO-PLAYER.md`'s O5 row later withdrew) | `walt/TILT-AUDIT.md`; `probes/tilt_arena_2026-08-19.log` | none | The race loses on decision cost in saturation-heavy self-play; Phase E vacuous (deterministic field) |
| `shadow` | 183 decisions: 67 exact / 116 unresolved / 0 δ-settled at cap 128; 23/27 agreement | `probes/shadow/README.md` (+ `summarize.py`) | library pieces only (`solver_shadow.rs`) | The live line was never eliminated at 128; at 512, 2 of 42 receipt-side settled (the "~108/116" forecast was wrong) |
| `v5flip` | No cap-dependent flip anywhere on 40/160/640 | `probes/step8/README.md` | `solver_calibrate.rs` (the V5 law) | The count-timing family stays honestly Unresolved at every cap |
| `e0cal` | 45/54 settled, all with the sign of the exact τ | `probes/step8/README.md` | `solver_calibrate.rs` | 9 honest Unresolved on the small-\|τ\| pairs |
| `wakeup` | Value wake-up 18/18; decision 8/18 | `probes/step9/README.md` | `solver_wakeup.rs` (type locks, the §14.4 fixture) | Response wake-up > ε on 2/18; pivotal mass *drops* under σ1 on 13/18 |
| `waking_bridge` | 729‰ of compute in the σ0 baseline; wake rate 1/34; agreement with σ0 55/56 | `probes/waking/summary.txt` | `solver_waking.rs` (the seat) | Not affordable as-is: 283.9 s for 56 decisions; tricks 1–2 carry 926‰ |
| `fieldswap_cancel` | Λ = 31/1200 asserted; first `FieldDecisionChanged` and first `Dominated` in the wild | `probes/fieldswap_cancel/README.md` | `solver_fieldswap_cancel.rs` (ladder inequalities, containment, winner stability, and Λ = 31/1200 itself — at the gate's n0 = 2 epoch, §2) | Directional rungs ≈ 2.3× tighter than E4 and still no prune |
| `hazard_witness` | 40 pairs, 0 accepts | `probes/hazard_witness/README.md` | `solver_hazard_witness.rs` | The narrowness is adopted (TRIPLE-A4/A5); the accept path lives only in engineered specimens |
| `l2_controller` | h8-t4 2-1 → 5-5 with spend refused as provably useless | `probes/l2_controller/README.md` | `solver_targeted.rs` | Count-timing g0–g5 open 6/6; epoch n0=8 does not compose with the n0=2 census |
| `ordering_bench` / `field_cache_bench` / `bundle_bench` | Values byte-identical across every arm; levers 2.2× / 1.2× / ≈ 1.04× | `probes/{ordering,field_cache,bundle}/README.md` | `solver_ordering.rs`, `solver_field_cache.rs`, `solver_bundle.rs` | The bundled evaluator's speedup does not survive (≈ 1.04×); the trick-1 regime barely moves |
| `rootinterval` | L ≤ Q ≤ U on all 14 rows; h4-t6 winner after 8 worlds | `probes/root_interval/run1.txt`; reproduced 2026-09-13 (every value identical; only the `wall-us` lines differ) | `solver_root_interval.rs` (incl. the 11/128 mirror sweep) | Exact ties come out `UnresolvedRootSet` — the instrument refuses to force a winner |
| `factorbelief` | 399,072,960 worlds → 116,280 hands; 45 µs/hand; reuse ×245 | `probes/factor_belief/c2_run1.txt` | `solver_factor_belief.rs` (the count, conservation, the four cache laws) | Cross-history cache reuse is exactly 0 (the record is in the key) |
| `factorrecursion` / `factorresponse` | Parity on every row; at h4-t4 the grammar mix certifies certain make | `recursion_run1.txt`, `response_run1.txt` | `solver_factor_recursion.rs`, `solver_factor_response.rs` | The bundled walk / enumeration split is faster where worlds/hands ≈ 3 |
| `factorcegar` | ≥ 800‰ action-exact at 36,923 classes | `cegar_run1.txt` | `solver_factor_consequence.rs` (Thm 30.1 nesting) | Zero residual = 116,280 singletons under a sampled mind |
| `factorrefine` | The exact ladder settles all ten gated roots | `refine_run1.txt` | `solver_factor_refine.rs` | The opening root is UNRESOLVED 7/7 at budget 100,000, fallback never promoted |
| `proofreport` / `extractreport` | h3-t4 Γ 83‰ → 0‰ exactly via a 12,420-state DAG | `proofreport_run1.txt`, `extractreport_run1.txt` | `solver_proof_regret.rs`, `solver_extraction.rs` | h4-t6 keeps Γ = 133‰ (vacuous winner's upper); settled action ≠ best materialized policy |
| `frontierreport` / `bellmanreport` / `laydownreport` | Γ = 0 for 1Z at two roots; a six-stage exact staircase; a real receipt-root Laydown (h10-t6) | `frontierreport_run1.txt`, `bellmanreport_run1.txt`, `laydownreport_run1.txt` | `solver_frontier.rs`, `solver_residual.rs`, `solver_covers.rs`, `solver_laydown.rs` | Vacuous uppers cannot rank extractions (28Z vs ≈ 15Z); first-generation covers vacuous at rich roots |
| `openingreport` | Opening root: bar 732‰, Γ 267‰, recommendation 6-5, honest UNRESOLVED at ε = 1/4 | `openingreport_run1.txt` | `solver_opening.rs` (resume ≡ uninterrupted; the cliff buys nothing) | The sampled tier plateaus at p = 512; ≈ 1.5 h |
| `doomreport` | 809–1000‰ of doom recovered where doom lives; an honest zero at the opening root | `doomreport_run1.txt` | `solver_doom.rs` | The split of the remaining 267‰ is UNKNOWN (DISCREPANCIES.md, 2026-09-03) |
| `modelbeliefreport` / `modelbeliefrecursionreport` | Φ = 0 at t5/t6; strictly positive at trick 4 (3–9‰ on h8-t4) | `modelbelief_run1.txt`, `modelbelief_recursion_run1.txt` | `solver_model_belief.rs`, `solver_model_belief_recursion.rs` (M6) | h8-t3 refused at 7M reads per action — trick 3 is the wall for the model-space recursion; F₁ exact only on two roots |
| `unifiedreport` | Lean rung 11.6 ms deciding for 72 decisions; 2 argmax flips at the join | `unified_run2.txt` | `solver_unified.rs`, `solver_unified_carry.rs` (lazy ≡ eager on 216 decisions) | A transcript, not an evaluation; 63 ProofStateUnavailable refusals |
| `godgapreport` / `horizonreport` | Fusion horizon at trick 5; h8-t3 exact 28859/29988 in 797 s, a trick-4 cut over-prices by 31‰ and flips the play | `godgap_run1.txt`, `horizon_run1.txt` | `solver_godgap.rs`, `solver_horizon.rs` | A measurement on a declared corpus, never a theorem; h0-t1 UnknownGodGap 7/7 |
| `focalreport` | Every live trick-4 coordinate settles by k ≤ 2; h8-t3 at k = 3 | `focal_run0.txt`, `focal_ladder_run1.txt`, `focal_run1.txt` | `solver_focal_horizon.rs`, `solver_focal_ladder.rs`, `solver_focal_anchors.rs` | 19.4 GB peak RSS and 1,435 s for the record; anchor (i) probe-only |
| `granrun` | G1 validated; the waking seat replays it 6/7 with one wake at trick 5 matching the human | `probes/gran/README.md`, `summary-replay.txt` | fixture by rules replay; the seat by `solver_waking.rs` | The trick-1 6-4 came from the baseline epoch, not from waking; the Plunge panel's numbers do not compose |
| `partnership` (batteries) | L2 Partner vs L1 14/14/72 (50.0%) on 100 shared deals; native `l1-race` reproduces the phone in 64 checks and 35 pairs | `experiments/partnership/campaigns/{native-l1-vs-phone-620600-649,foundation-battery,default-partner-battery}/RESULTS.md` (8/15/77 is the first's) | `solver_partnership.rs` (12), `solver_selection.rs` (7); independent replay by `verify_campaign.py` | **No strength gain established anywhere**; native fixed L1 is behind the phone (8/15/77) because the phone races |
| `partnership_gym` | 6 → 170 → 433 → 30 exercises with exact audited keys; L1 24/30 vs L2 Partner 26/30 | `walt/gym/RESULTS.md` | `tests/gym.rs` + 25 Python tests | Field-relative, bid 30, tricks 5–6 only; not a general strength ranking |
| `policy_lab` / `relational_lab` | Persistence ≈ halves search with exact parity | `campaigns/{policy-synthesis-v1,relational-learning-v1}/RESULTS.md` | `policy_search.rs`, `relational_learning.rs`, `relational_runtime.rs` | Donor composition unpaid; shared relational actors trail sampled tables |

## 5. What it costs

Walls are single-machine readings at the declared knobs (the record's own
figure unless marked otherwise); reads and nodes are the exact units. The
ladder runs from microseconds at trick 6 to hours at the opening root, and the
opening root is reached only by contraction or sampling — never by enumeration
(`ladder` t=1 refused at 399,072,960 worlds).

| Instrument (knobs) | Cost | Source |
| --- | --- | --- |
| `scheme` partner-count on h0-t6-S0 | ≈ 10 ms (11 ms wall; work 1084) | measured 2026-09-13 |
| `rootinterval run` (prefix 16) | 0.17–0.18 s | run1; measured 2026-09-13: 0.18 s (0.177 s wall) |
| `bundle_bench` | ≈ 2 s | `probes/bundle/README.md` |
| `ordering_bench` (default items) | 1.1–1.5 s; `hard` adds ≈ 3.1 s | measured 2026-09-13: 1.10 s |
| `laydownreport` | < 1 s | `laydownreport_run1.txt` |
| `factorbelief c2` (opening root under σ0) | 5.4 s cold, 22 ms warm | `c2_run1.txt` |
| `modelbeliefreport` | 5.2 s | `modelbelief_run1.txt` |
| `proofreport` | ≈ 14 s (run1's per-root walls sum to 14.1 s; no total line); 7.2–7.4 s measured 2026-09-13 (two runs) | `proofreport_run1.txt` |
| `frontierreport` / `bellmanreport` / `extractreport` | 13 s / 15.7 s / 29 s | the run1 records |
| `m3probe` | ≈ 30 s single-threaded (28.85 s measured 2026-09-13) | `results_2026-08-17.txt` |
| `unifiedreport report` | ≈ 32 s (lean 2.2 s, ample 21.6 s, model 8.6 s) | `unified_run2.txt` |
| `fieldswap_motifs` / `fieldswap_cancel` / `field_cache_bench` | ≈ 30 s / ≈ 1 min / ≈ 2 min | the probe READMEs |
| `scenario` t=1, n = 256,000 | 64.0 s | `scenario_results_2026-08-17.txt` |
| `factorrefine` (Sections A–D) | ≈ 1 min (opening Section D 12.0 s) | `refine_run1.txt` |
| `granrun replay` / `driven` (G1) | 25 s / 166 s | `probes/gran/README.md` |
| `tiltaudit arena` (24 hands) | ≈ 132 s of decisions | `tilt_arena_2026-08-19.log` |
| `waking_bridge driven` (2 hands, 56 decisions) | 284 s of decision compute (283,899,641 µs) | `probes/waking/summary.txt` |
| `focalreport scout-corpus` | minutes (h4-t4 16.6–26.5 s per horizon) | `focal_run0.txt` |
| `horizonreport` h8-t3 exact | 797 s (13.3 min), 289,407,472 reads | `horizon_run1.txt` line 11932 |
| `focalreport report` (33 coordinates + the h8-t3 anchor) | 1,435 s, peak RSS 19.4 GB | `probes/factor_belief/README.md` lines 750–753 |
| `modelbeliefrecursionreport report` | ≈ 40 min (2,379 s; h8-t3 refused after 1,864 s) | `MB1-REPORT.md` |
| `openingreport` (stops p16 → p512) | ≈ 1.5 h (5.7 s / 30 s / 10.5 min / 76 min) | `openingreport_run1.txt` |
| `shadow` at cap 512 | ≈ 4.15 h summed (receipt), ≈ 16.3 h (driven) | `probes/shadow/README.md` |
| `bidcurve` corpus (200 hands × 3 passes) | hours (117 cells/hand at ≤ 120 s) | `run_calibration.sh` |
| `divergence` (900 hands) | hours (overnight) | `divergence_results_2026-08-18.txt` |
| Interactive caps | `webtable` 120 s/move; `playtable` 180 s/move; `walt_bridge`/`controller_bridge`/`waking_bridge` 120 s declare-path budget; `partnership` 14 s | the binaries' `main()` |
| The gate (`walt/ci/check.sh`) | 230 s wall, 121 test binaries (CI1, down from 367 s serial); 308 s with FH3's anchors gate inside; 18.22 GB peak RSS for `solver_focal_anchors` standalone (FH3 said 17.8 GB) | `walt/briefs/CI1-REPORT.md`; `walt/briefs/FH3-REPORT.md` and `walt/MAP.md` (230 → 308 s); `walt/briefs/FH4-AUDIT.md` (18.22 GB). Not re-run here |

**Newcomer first-run order** — each step run 2026-09-13 (the two previously untimed parts of steps 5 and 7 on 2026-09-14) from the worktree root
with the release binaries under `walt/target/release/` (built 2026-09-07 in
the main checkout; the worktree has no `target/`) and `timeout 60`; walls are
wall-clock seconds on this machine:

1. `scheme --query walt/scheme/examples/partner-count.scheme --hand 0 --trick 6 --seat 0` — real 0.01 s; prints the coordinate (viewer S0 hand {2-0 4-2}, 6 support worlds), `event-probability: 1/3`, `answer-presence: [count-tile=4-1] probability=1/3`, `work: 1084`. Teaches the coordinate/belief vocabulary.
2. `rootinterval run /tmp/ri.txt` — real 0.18 s; six roots, exact Q beside [L, U] per action; `h4-t6 … decision: DeltaRootWinner{action=1-1;bar=7/10}`, `worlds-to-singleton: 8`; ties typed `UnresolvedRootSet`. Identical to `run1.txt` except the six `wall-us` lines.
3. `ordering_bench` — real 1.10 s; twelve items, values byte-identical across arms, counters `1198/1308 … 22803/30924` identical to the README. The counters are the signal.
4. `proofreport report /tmp/pr.txt` — real 7.35 s; seven roots; `h3-t4 … survivors=[3-1]`, `recommended action=4-4` at floor 267‰, CERTIFIED REGRET 83‰ — certified regret on real roots.
5. `unifiedreport walk 8 5 lean` — real 0.18 s; 12 decisions to terminal, each with its tier ((e) field-fallback, (a) decided-arithmetic, (b) endgame-exact), reads and typed refusals; "no line was falsified on this walk". Then `walk 8 4 ample` — real 13.0 s (run 2026-09-14): 16 decisions, 8 at tier (a) and 8 at tier (b), join reading on; at the h8-t4 root the mixture is *refused* (`MixtureUnaffordable { fiber: 1200, cap: 256 }`) and tier (b) answers, and the walk ends with a `LIBRARY FALSIFIED at line play 6` line (seat 3 played 5-0 where the carried library supported {2-0 5-3 6-0} — the UP1a lazy carry catching a falsified posterior during play, the honest output, not an error). An earlier revision of this step said `ample` shows "the mixture tier"; it does not — `walk` takes only `lean|ample` (a bare fourth argument other than `lean` is `ample`), and tier (c) answers only under `report`'s `model` rung, whose caps are swapped for that purpose (§3.4).
6. `m3probe` — real 28.85 s; the one exact anchor of the seat-play era, `lawful play under M3B P30 make probability: 33`, output identical to the 2026-08-17 record.
7. `webtable` then open `http://127.0.0.1:4242` and play a hand; `webtable ctrl` to watch routes per play. (Interactive; the hand itself is not timed.) Run 2026-09-14 as `webtable 4737 8 2 42 0` (a spare port, small knobs): the process prints its tier line ("walt web table — EXPLORATORY; estimates, never receipts"), `n_outer=8 n0=2 seed=42`, and serves within 2 s — `GET /` answers HTTP 200 with the 23,173-byte page (`<title>walt table`), `GET /state` answers `{"phase":"auction","hand_no":1,"human":0,"bidder":1,"bid":30,…,"sizes":[7,7,7,7],…}` with seven tile ids in `hand`. Pick a free port first: the binary panics on `bind localhost port … AddrInUse` if the port is taken (observed on the first port tried on 2026-09-14; a `webtable` process 27 days old was still running on this machine).
8. `partnership_gym --inspect < request` (a seven-line request: `decl 5 / bid 30 / bidder 1 / seat 1 / hand 0 1 2 3 4 5 6 / plays / seed 1`) — real 0.01 s; the `partnership-gym-v1` header with `worlds 399072960`; then `python3 experiments/partnership/gym.py show advantage-01` — real 0.04 s; the exercise with its answer key (`5-0: 30/36 success optimal`, `5-3: 8/36`) and the examiner-only witness. (The `partnership` oracle itself wants its mode as the bare first line — `status`, then the field lines — see §1.)

Nothing in the list exceeded 30 s; the only steps skipped for time are the
`report` modes of §3.4–§3.5 and `openingreport`.

## 6. Everything else that runs under `walt/`

**The workspace beyond the unified crate.** Six crates in one Cargo workspace
at `walt/Cargo.toml`: `walt` (the unified crate — ten modules `rules, kernel,
scheme, geom, strat, spec, carrier, solver, gym, policy_search`; `lib.rs`'s doc
comment still lists eight and omits `gym`/`policy_search`), `walt-wasm` and
`walt2-wasm` (§1), and the GPU trio. [walt-architecture](walt-architecture.md)
owns the module table, the two solver stacks and their seams, the invariants
and the gate; [walt-gpu-native-trick1](walt-gpu-native-trick1.md) owns the GPU
track's status. What this page adds about them is only what runs:

| Crate / script | What runs | Notes |
| --- | --- | --- |
| `walt-gpu-ref` | the portable M1 reference projector, the complete M2 carrier, bindings and canonical receipt codecs; the example `generate_m0_m1_receipts` is what `check.sh` byte-diffs against `walt/receipts/gpu_native_trick1_m0_m1_v1/` | rob appears only as a development-time prose-rules bridge (dev-dependency) |
| `walt-metal` | the only Metal/Objective-C boundary (`abi`, `bridge`, `runtime`, `error`; `shaders/00_u256.metal`, `01_opening_projector.metal`, `02_m3_wavefront.metal`; the checked-in metallib) | **its three device tests in `tests/metal_device.rs` are `#[ignore]`d** ("requires elevated access to the local Apple GPU; run in release mode") — the portable gate never exercises Metal; only `check_m2_metal.sh` does |
| `walt-m2-runner` | the supervised freeze-56 executable: `walt-m2-runner COMMAND …` with **ten modes** — `descriptor-render ROOT`, `descriptor-verify ROOT`, `run-smoke ROOT OUTPUT`, `run-official ROOT OUTPUT`, `child-smoke`, `child-official` (the supervised child profiles `run-smoke`/`run-official` spawn), `validate-smoke FILE`, `validate-receipt ROOT FILE`, `validate-failure FILE`, `adjudicate-receipts ROOT FIRST SECOND COMMITTED CHECKSUM FAILURE` (`main.rs` `usage()`); five of them (`descriptor-verify`, `run-smoke`, `run-official`, `validate-receipt`, `adjudicate-receipts`) are the user-facing ones earlier revisions of this page listed | typed progress/timeout/no-partial semantics; constructs or adjudicates the closed receipt |
| `walt/ci/check.sh` | the portable gate: re-execs in a clean environment, immutable M0/M1 history check, the frozen GPU trick-1 guide checksum, the byte-diffed M0/M1 receipt replay, `cargo fmt --check`, clippy `-D warnings -D clippy::float_arithmetic`, the no-float greps (walt and rob's core/player/verify, MSL and TOML), `cargo test --workspace --release --no-run --message-format=json \| run_test_binaries.py`, doc tests, the Lean `Trick1Foundation`/`Trick1MetalFoundation` build with the axiom audit diff | `/bin/bash -p walt/ci/check.sh [FAILURE_OUTPUT]`; do not run casually (release build, full suite, Lean) |
| `walt/ci/run_test_binaries.py` | the gate's test scheduler since CI1 (`e53752b`, 2026-09-04): runs every test executable concurrently with max(2, cpu/2) workers, heavy suites first, PASS/FAIL per binary, the eight slowest and the sum of walls | purely a scheduler; its `HEAVY_FIRST` list names a `solver_focal_budget` suite that does not exist under `walt/walt/tests/` (the focal suites are `solver_focal_anchors`, `solver_focal_horizon`, `solver_focal_ladder`) — a leftover, flagged, not repaired here |
| `walt/ci/check_m2_metal.sh` | the elevated native gate: the portable conjunction, the host/tool descriptor, the metallib rebuilt twice and compared with the committed library, canonical Gate 0, the full U256 corpus, malformed/timeout/no-partial controls, the complete 614-task carrier twice from fresh process state | requires the exact native toolchain and a real device; whether it still runs green after the fold has not been traced (open item) |
| `walt/ci/verify_m2_sources.sh` | the cumulative source-manifest closure | a **freeze-event** check only (FZ-A5), because the unified crate contains the actively developed solver |
| `experiments/partnership/*.py` | `player.py` (the JSON wrapper over `partnership`), `table.py`, `experiment.py`, `campaign.py`, `pool.py`, `match.py`, `verify_campaign.py`, `gym.py`, `gym_spec.py`, `policy_campaign.py`, `policy_gym.py`, `relational_campaign.py`, `relational_exam.py`, `verify_relational.py`, `checks.py`; every long job under `packet/texas42-partnership-launch-v0.1/tools/run_capped.py --seconds 295` | owned by [walt-partnership-program §10](walt-partnership-program.md) and [walt-gym §8](walt-gym.md) |

**`walt::carrier`, corrected.** The frozen hand-8 receipt carrier (freeze-57
M3 gate profile; two constructors that must agree byte-for-byte, KAT pins and
digests in `tests/carrier.rs`) is **consumed by the fixed-carrier probes only**
— `ladder`, `level1`, `level2`, `m3probe`, `scenario` — plus `playout`, which
imports `carrier::VIEWER`. No live player surface reads it; an earlier
revision's "the seat player's data source" is stale.

**The probe records — every directory under `walt/probes/`:**

| Directory | What it holds | Written by |
| --- | --- | --- |
| `bidcurve/` | `small-n12.log`, `live-n40.log`, `ref-n200.log`, `driver.log`, `ANALYSIS-2026-08-19.txt`, `analyze.py`, `run_calibration.sh` | `bidcurve` |
| `bundle/` | README with the readings | `bundle_bench` |
| `exp3a/` | the rescued Python suite: `lambda_probe{,_v2,_v3}.py`, `v3_diag.py`, four `*_output_postfix.txt` runs, `lambda-probe-report.md` (Experiment 3A's 22-observable atom registry) | 2026-08-09 scratchpad, preserved verbatim; frozen validator, never source |
| `exp5/` | `exp5_core.py`, `exp5_rules.py`, `exp5_census.py`, `exp5_validate.py`, `exp5_report.py`, `exp5_pwl.py`, `exp5_exact.py`, `exp5_records.jsonl` (566 records), `exp5_results.md` | the designated second implementation; its census vectors (h1t3 = 10, h3t3 = 5,345) and 52 fiber sizes are regression pins in `tests/strat_exp5_census.rs` and `tests/kernel_known_fibers.rs` |
| `factor_belief/` | README (the quotable authority for the era's numbers) and the `*_run1.txt` records of §3.3–§3.5 (`run1`, `opening_level0_run1`, `cache_run1`, `c2_run1`, `recursion_run1`, `response_run1`, `cegar_run1`, `refine_run1`, `profile_run1`, `proofreport_run1`, `frontierreport_run1`, `bellmanreport_run1`, `extractreport_run1`, `laydownreport_run1`, `openingreport_run1`, `doomreport_run1`, `modelbelief_run1`, `modelbelief_recursion_run1`, `unified_run1`, `unified_run2`, `godgap_run1`, `horizon_run1`, `focal_run0`, `focal_ladder_run1`, `focal_run1`) | the fifteen counted-belief/anytime bins, MB0/MB1, UP0/UP1a, U0/U0b, FH1–FH3 |
| `factory-results/` | the relocated result summaries of the archive-only factory (§7), with their provenance README and the sixteen §16.11 records under `certificates_2026-08-10/` | the deleted `walt-factory` examples at `648f93a` |
| `field_cache/` | README, `bench_2026-08-25.log` | `field_cache_bench` |
| `fieldswap/`, `fieldswap_screen/`, `fieldswap_cancel/`, `fieldswap_motifs/` | `fieldswap.jsonl` / `screen.jsonl` / `cancel.jsonl` / `motifs.jsonl`, README, `summarize.py` | the field-swap family |
| `grammar_residual/` | `run1.txt`, README | `grammarsplit` |
| `gran/` | `g1.receipt.txt`, `g2g3.receipt.txt`, `g1-replay.jsonl`, `g1-driven.jsonl`, `summary-replay.txt`, `summary-driven.txt`, README, `summarize.py` (the screenshot sources live under `~/data/texas-42/gran-anchors-2026-08-24/` with a `MANIFEST.sha256`, not in the repo) | `granrun` |
| `hazard_witness/` | `records.jsonl`, README | `hazard_witness` |
| `l2_controller/` | `records.jsonl`, README, `summarize.py` | `l2_controller` |
| `m3/` | `results_2026-08-17.txt`, `ladder_results_2026-08-17.txt`, `scenario_results_2026-08-17.txt`, `sampling_results_2026-08-17.txt`, `level1_results_2026-08-17.txt`, `level2_results_2026-08-17.txt`, `arena_results_2026-08-17.txt`, `divergence_results_2026-08-18.txt`, `mined/` | the seat-play probes and the arena |
| `ordering/` | README with the before/after counters | `ordering_bench` |
| `root_interval/` | `run1.txt`, README | `rootinterval` |
| `shadow/` | `receipt.jsonl`, `driven.jsonl`, `receipt_512.jsonl`, `driven_512.jsonl`, README, `summarize.py` | `shadow` |
| `step8/` | `e0.jsonl`, `v5.jsonl`, README, `summarize.py` | `e0cal`, `v5flip` |
| `step9/` | `records.jsonl`, README, `summarize.py` | `wakeup` |
| `waking/` | `driven.jsonl`, `summary.txt`, README, `summarize.py` | `waking_bridge driven` |
| `tilt_arena_2026-08-19.log` | the 24-hand race-vs-full arena log | `tiltaudit arena` |

None of these records is byte-diffed by CI, and none becomes a claim-tier
result by existing. The Python suites' framing is load-bearing: they are
frozen validators, never source — walt reimplements from the frozen
mathematical basis and pins its own results against the probe records; a
disagreement is a discrepancy to record, never a reason to copy probe code
into the implementation. Both suites are stdlib-only Python 3.12 with exact
`Fraction`/integer arithmetic; running them creates `__pycache__` — clean it up
(D15).

## 7. The historical inventory (2026-08-09 → 08-24; archive-only since the fold)

Everything in this section is what the pre-pivot programs built and measured.
It is kept because the machinery and the numbers are still cited by the era
pages ([walt-foundation-era](walt-foundation-era.md),
[walt-factory-era](walt-factory-era.md), [walt-census-era](walt-census-era.md),
[walt-s6-era](walt-s6-era.md)) and by [walt-pre-pivot-results](walt-pre-pivot-results.md),
and because several of its negative results left working machinery behind on
purpose: the fiber-crush probe found the class DAG slower than a plain cache at
first build ("the class store is a storage/transport object, never a
first-build accelerator", S5h); the fiber-refinement probe found its declared
exclusions biting near-zero worlds ("the predicate ENGINE is proven either
way", S5i); the endgame store lost to the plain cache on speed and still
produced the first direct size data for the seat-level census (S5j). The
retired `PLAN.md` said it plainly: "Nothing is discarded: the memoized H solver
+ tree cross-validation are retained as the seat-label ground-truth
instrument."

**The fold (2026-08-24, `d1499d43`).** Seven crates became the seven original
modules of one `walt` crate (`rules` ← walt-core, `kernel` ← walt-kernel,
`geom` ← walt-geom, `strat` ← walt-strat, `spec` ← walt-gpu-spec, `carrier` ←
walt-m3-carrier, `solver` ← walt-m3-probe), and the `walt-factory` /
`walt-skeleton` crates — every probe binary in the tables below — were
**deleted**. Their code is archive-only at producer commit `648f93a` (deletion
commits `ad355e9` / `fa3fe74`); their tracked result summaries were relocated
to `walt/probes/factory-results/`; regeneration follows the recompute queue in
[`walt/ARCHIVE.md`](../walt/ARCHIVE.md): `git switch --detach 648f93a`, then
`cargo run --release -p walt-factory --example NAME [subcommand]`
(`walk_corpus` was a `src/bin/` binary), and verify against the archive
manifest digest afterwards — frozen seeds make byte-identity the expected
outcome, and a mismatch is a finding. `walt-skeleton` held the
`ControlSkeleton` trait, the §12.1 soundness and §12.6 lumpability checkers,
both atom vocabularies, the §12.9 synthesis search and the §12.6A equivariant
census machinery (class DAG, railyard, suffix library); `walt-factory` held the
regret walker, the conflict/lesson/basin vocabulary, the lesson database with
its watched-feature index and rent ledger, §16.11 record emission, and all 24
probe examples plus `walk_corpus`.

### 7.1 The research-era solvers (still in the crate, live only in three census gates)

Five operators exist in `walt::strat`, and §10.8's rule is enforced socially
and by type: a theorem for one operator never silently transfers to another.
PI-averaged action values are the information-relaxed diagnostic, **not** the
seat's hidden value Q^H; the gap is the strategy-fusion gap and it is
action-specific.

| Solver | File | What it computes |
| --- | --- | --- |
| Symbolic parametric PI | `walt/walt/src/strat/pi.rs` | Worldwise perfect-information backward induction over the whole valuation ray; every root action value is a continuous PWL envelope |
| Scalar PI | `walt/walt/src/strat/scalar.rs` | The same PI operator at one integer valuation, with a trick-boundary cache keyed on semantic state; the workhorse for whole-fiber and census work |
| Symbolic H | `walt/walt/src/strat/hidden.rs` | The actual hidden-information fixed-field treatment at the root, exact on the whole ray (pooled maximization decomposes because the canonical partition is a tree) |
| Scalar H (`dag-v1`) | `walt/walt/src/strat/hidden_scalar.rs` | Exact Q^H per legal action at arbitrary (including mid-trick) decision points, unit-fraction particle weights, budgeted, with pooled-state boundary memoization |
| Revealed C and F | `walt/walt/src/strat/revealed.rs` | Continuation- and root-revelation with the field held fixed, aggregated at the support level so no polytope is materialized |

The sixth solver family — the scenario-player stack in `walt::solver`, exact
best response over sampled fiber worlds against modeled level-k minds under the
pmake objective — is the live one since 2026-08-17 and is owned by
[walt-seat-play](walt-seat-play.md) and `walt/SCENARIO-PLAYER.md`.

Measured facts about the research-era solvers, each exploratory and each
attached to its source (the `results/...` files now live under
`walt/probes/factory-results/`; the `examples/*.rs` producers are archive-only
at `648f93a`):

- **Ordinary transposition memoisation is the manyfold, and it compounds with
  depth.** Arm A1 (identity-key boundary cache) against A0 (plain tree) has wall
  medians 0.166, 0.024, 0.010 at n = 4, 5, 6 tricks remaining — roughly 6× to
  100×. Source: `fiber_probe_2026-08-11.txt`, produced by `examples/fiber_probe.rs`.
- **The class DAG is not a first-build accelerator.** Arm B (r3-signature
  content-addressed class DAG) against A1 is ≈ 4.3–4.9 at every rung (medians
  4.7 / 4.3 / 4.9) — identical values at about five times the cost. The reason
  is structural: class identity is a function of the future cone, so it is
  computable only after full expansion. Interior collapse is nonetheless real
  (n=4 hand 0: 1.50M situations to 129k classes). Same source file.
- **Canonicalization dominates in the endgame store.** The symmetry-reduced
  tablebase arms run 1.57–2.69× slower than the plain A1 cache: about 4.6 µs
  per canonical form against about 0.1 µs per state-key probe. Convergence
  itself is real (830,399 form hits, 38–73% form-hit rates). Source:
  `endgame_store_2026-08-11.txt`.
- **Closed-form last-trick resolution beats a floor table.** Floor-table lookup
  1,430 ns against the closed-form control 35 ns — a 41× negative, reported as
  one; the closed-form bottom is the one arm that beat the control end to end
  (0.88–0.99 of T0). Source: `endgame_floor_2026-08-11.txt`.
- **The memoized H solver is value-transparent and much cheaper.** `dag-v1`
  did 13–125× less work than the unmemoized `tree-v0` walk on the fiber-probe
  coordinates, and 28–122× fewer steps on the four big-fiber cross-validation
  decisions (tree side 4.2e9 to 6.5e10 steps). Byte-identical Q^H was a
  CI-pinned invariant while the factory existed
  (`walt-factory/tests/h_value_transparency.rs`, sixteen decisions, including
  the `Q^H(2-1) = 80/7` vs `Q^H(3-2) = 202/21` pins); that test is archive-only
  at `648f93a`. The offline cross-validation receipt is
  `h_tree_crossval_2026-08-10.txt`, produced by the `#[ignore]`d
  `tests/h_dag_probe.rs::crosscheck_tree_uncapped`.
- **Cold treatment H completed at four tricks remaining.** The seat's actual
  pooled hidden-information solve completed on full 34,650-world void-free
  fibers at every eligible n=4 coordinate, in roughly 7 to 17 seconds each,
  inside a declared 200M particle-step budget. No tractability claim follows
  (SEP-A15(iii), R-A23): one wall-clock observation at declared coordinates
  under a declared budget. Source: `fiber_probe_h_2026-08-11.txt` (9
  coordinates COMPLETED and 4 out of scope; `walt/LOG.md`'s "8 of 13" was an
  error, corrected here).
- **Store-based exclusion predicates are essentially free.** A predicate pass
  over a built store costs 0.1–3.7 ms against multi-second builds, roughly
  100–1000× cheaper than the cheapest storeless route (200–960 ms);
  reachability and confinement predicates have no storeless alternative at
  all. Source: `fiber_refine_2026-08-11.txt`.
- **Deadness detection is cheap, and has a quotable instrument.** The often
  repeated "about 25 ns per detector call" is **contended and not quotable**,
  and is not even in the results file: `deadness_2026-08-12.txt` records a
  RESUMED run and prints `0 ns over 0 calls`. Freeze 43's sequential timing
  rung (DS-A33) — the only quotable timing instrument — **was subsequently
  run**: `deadness_rung_2026-08-13.txt` records 17 ns/call over 384 calls at
  the declared grade-3 unit and 42 ns/call over 3,540,143 calls at the declared
  n=4 unit, single uninterrupted process, declared selection rule, run
  complete. Those are the quotable figures; the 25 ns stays retired. (An
  earlier revision of this page recorded the rung file as
  referenced-but-nonexistent in one place and as run in another; the file
  exists at `walt/probes/factory-results/` and the table below now says so.)
- **The weighted H re-solve over a pre-built class DAG — the number the
  belief/policy-iteration platform claim rests on — is still unmeasured.** The
  existing H solvers take a uniform fiber weighting only and the K-bar
  integration is unbuilt; stated in the results file itself (P-A14).

### 7.2 The factory probe binaries (archive-only at `648f93a`)

| Binary | What it measures | Results file (under `walt/probes/factory-results/`) | Session |
| --- | --- | --- | --- |
| `bin/walk_corpus` | Full corpus regret walk, 13 hands × 4 seats, whole transcripts; resumable by `[start_hand [start_seat [max_pairs]]]`, seeds a fixed function of (base seed, hand, seat, trick) so parts concatenate | `full_walk_2026-08-10*.txt` (part 1, part 2, assembled) | S5a |
| `gen_fixtures` | Regenerates the frozen walker fixtures; never hand-edit the outputs | `tests/data/ci_corpus_pins.txt`, `tests/data/walk_h0_S1.txt` (archive-only with the crate) | S5a |
| `thread_independence` | One-off determinism check: designated walk under one worker thread vs full parallelism must be byte-identical | none (prints) | S5a |
| `lesson_run` | Generalizes walker conflicts into lessons and measures basins on the tricks-5–6 exhaustive domain | `lesson_basins_2026-08-10*.txt`, `tests/data/lesson_h0_S1_t5.txt` | S5b |
| `falsification_run` | The falsification test proper on the tricks-3–6 fiber-capped domain, with relaxation ladders and cut refinement | `falsification_2026-08-10*.txt` | S5c-m1 |
| `label_transfer_run` | Re-measures every lesson basin at (H, fixed-uniform-legal); mode `r3` re-measures only the capped decisions at a raised declared budget | `label_transfer_2026-08-10{,_r2,_r3}.txt` | S5c-m2 / m3 |
| `economy_run` | Lesson DB as a working set: watched index, dual-ledger rent epochs, deletion rule with the checker block, restart-with-retention, §16.11 record emission | `economy_2026-08-10.txt` + `certificates_2026-08-10/` | S5c-m3 |
| `economy_run_r2` | The same 16-lesson working set re-priced at `dag-v1` / 10^9 | `economy_2026-08-10_r2.txt` | S5c-m3c |
| `census_run` | The §12.6A situation census. Subcommands: (default) r1 finest quotient, `r2` declared coarsenings, `r3` retrograde coarsest, `t5` the trick-five climb, `prune` live sub-DAG, `yard` the railyard factoring, `yard2` the suffix library, `a1` the complete level-one alphabet | `census_2026-08-10{,_r2,_r3}.txt`, `census_t5_2026-08-10.txt`, `census_pruned_2026-08-10.txt`, `census_yard_2026-08-10.txt`, `census_yard_v2_2026-08-10.txt`, `census_a1_complete_2026-08-11.txt` | S5e–S5g |
| `fiber_probe` | Default: the three-arm cost ladder (A0 / A1 / B) at n = 4, 5, 6. Subcommands: `h` cold treatment H, `refine` declared exclusion remnants, `endgame` symmetry-reduced tablebase, `floor` the level-1 floor table | `fiber_probe_2026-08-11.txt`, `fiber_probe_h_2026-08-11.txt`, `fiber_refine_2026-08-11.txt`, `endgame_store_2026-08-11.txt`, `endgame_floor_2026-08-11.txt` | S5h–S5j |
| `predictive_rank` | Dimension census of the value closure V^val at grades 1–3 (v0.6 Gate B) | `predictive_rank_2026-08-12.txt` | S6a |
| `policy_geometry` | Policy-geometry probe (Gate E): the four never-conflated cardinalities N_pol / N_vec / N_par / N_exp, with an exact-rational simplex under Bland's rule | `policy_geometry_2026-08-12.txt` | S6b |
| `policy_inspect` | Exploratory diagnostic, cited by nothing: reads out the dominant policies at the singleton-frontier roots and interrogates them against trivial rules | none (prints) | S6b |
| `deadness_probe` | Three one-sided deadness detectors (D0, D1-sym, D1-win) at census scale against the one-deviation tie classifier; parallel and resumable with per-unit checkpoints; the freeze-43 sequential timing rung (DS-A33) | `deadness_2026-08-12.txt`; `deadness_rung_2026-08-13.txt` (the rung **has been run** — 17 ns/call and 42 ns/call, §7.1) | S6c |
| `separation_probe` | Experiment E: exact root-action certification by a primal witness against an action-conditioned upper witness; writes candidate library v1 | `separation_2026-08-13.txt` | S6d |

### 7.3 Frozen artifacts, receipts, and what CI checked then and checks now

**Byte-frozen fixtures.** The factory's three (`walk_h0_S1.txt`,
`ci_corpus_pins.txt`, `lesson_h0_S1_t5.txt`, asserted for exact string
equality by its walker/lesson tests) are archive-only at `648f93a` with the
crate that checked them. The unified crate's surviving frozen fixtures are the
exp5 census samples under `walt/walt/tests/data/` (`exp5_sample_h1t3.txt`,
`exp5_sample_h3t3.txt`, asserted by `strat_exp5_census` and
`kernel_known_fibers`), the σ1-repair before-side fixture
`sigma1_before_v1.txt` (§1), and the frozen native trace the wasm smoke
byte-compares.

**Results artifacts.** The tracked result summaries — every dated `.txt` the
era pages cite, each opening with its own tier line, its binding rulings, its
declared scope and (for the later ones) its exact regenerate command — were
relocated intact to `walt/probes/factory-results/` (provenance README there),
including `certificates_2026-08-10/`: sixteen §16.11 records, one per lesson
in the S5c-m3 working set (ten `cert_refutation_*`, five `cert_win_*`, one
`cert_checker_*`, filenames deterministic from content keys), written against
the self-contained `certificate-schema.md` beside them (schema-v1, the
historical filename that keeps walt's own "certificate" name) so an
independent implementation can check them. The untracked bulk (8.3G of raw
outputs, 514M of stores) lives at `~/data` and HuggingFace per
`walt/ARCHIVE.md`, never in the repo.

**GPU-track comparands.** Portable M0/M1 has the canonical envelope, declared
stop and summary under `walt/receipts/gpu_native_trick1_m0_m1_v1/`. The
separate `gpu_native_trick1_gate0_2026-08-16.txt` is retained unchanged: its
NO-GO remains a true observation of the old Command-Line-Tools-only
environment. Freeze 56 has one committed binary receipt and external checksum
under `walt/receipts/gpu_native_trick1_m2_v1/`. That M2 receipt is executable
evidence, not a Lean theorem and not a persisted value for a solver to consume.
The GPU side supports exactly one status sentence — **M2 METAL PROJECTOR
PARITY COMPLETE under freeze 56**, re-issued append-only at the unified layout
as freeze-56 v2 (FZ-A1..A6) with the standing M2 receipt explicitly old-layout
evidence — and it covers arithmetic/projector parity only: no action value,
selected lead, optimal set, information net, continuation, performance claim
or player.

**Three accurate statements about walt's receipt discipline**, which differ
from rob's:

1. The legacy probe artifacts (`walt/probes/factory-results/*.txt`) are
   **not** diffed by CI, and since the factory's deletion their byte-equality
   coverage by ordinary tests is archive-only too; none becomes a claim-tier
   result merely by existing. The same is true of every JSONL and `*_run1.txt`
   record of §3.
2. Portable M0/M1 does have a byte-diffed receipt stage: `ci/check.sh`
   regenerates the complete canonical directory in fresh state and compares it
   recursively with the committed comparands.
3. M2 has a stricter native stage: two fresh complete receipts must match each
   other, their external checksum and the immutable HEAD comparand, with typed
   failure output and zero partial acceptance. Receipt-shaped walt artifacts
   remain exploratory and say so in their own headers — the factory's
   `report.rs` and `lesson_report.rs` existed to make rendering byte-stable.
   A green walt run is evidence at a declared configuration; the M2 receipt is
   likewise executable evidence rather than a theorem.

**Stores.** The factory's gitignored caches (`store/endgame_l2.store` — the
level-2 endgame form store; `store/deadness_ckpt` — per-unit run checkpoints
with a freeze digest; `store/candidate_library.txt` — candidate library v1,
freeze 36: observation-record keys, no values, no verdicts, identity transport
only, cache never authority) went to the local archive with the crate
(`~/data/texas-42/walt-factory-archive-2026-08-24/store/`). Every headline
number keeps a cold-regenerate path that starts by deleting its store — at the
producer commit, per the recompute queue.

### 7.4 What was mechanically blocked, and why that was safe

The lesson economy's deletion rule fired on three lessons in the re-priced run
(the empty-basin refutation and both h1 S2 t4 lessons, all measured-zero at
the seat-facing label). All three deletions are **TRIGGERED and each
mechanically BLOCKED**. The block was enforced by type in
`walt-factory/src/ledger.rs`: executing an H-priced deletion requires an
`HCheckerToken`, whose only constructor is `HCheckerRegistry::token`, which
returns one exactly when an independent H checker is registered. An empty
registry can only produce `DeletionBlocked` records. The uncapped tree
cross-validation receipt is deliberately **not** a registered checker — it is
context only, and every at-collection stamp stays SINGLE-IMPLEMENTATION.

The intended independent checker was "m4", a Python H checker. It is
**retired** by the NO-RESCUE policy (`PLAN.md`): if independent mechanical
verification of H is ever genuinely needed, the path is Lean, not Python.
Until then the triggered deletions stay blocked — safe by design, since
deletion is an economy action over working-set membership only; the archive is
append-only, readmission is cheap, and no evidence is lost by the block. While
the factory existed the machinery was intact and exercised in CI
(`tests/economy_pins.rs`); since 2026-08-24 the whole apparatus — the lesson
DB, the watched-feature index under its candidate-completeness contract
(exhaustively cross-checked, 179 × 16 = 2,864 pairs), the dual H-primary rent
ledger with "unmeasured is never zero", and §16.11 record emission with
per-record checker-coverage annotations and H rows honestly marked
UNCHECKED-EXTERNALLY — is archive-only at `648f93a`, with the emitted records
preserved under `walt/probes/factory-results/`.

### 7.5 Declared knobs of the factory era, and the discipline that goes with them

These were constants or CLI arguments in the factory sources (archive-only at
`648f93a`), and every one of them is part of the declared inputs its result is
quoted under:

| Knob | Where | Declared value |
| --- | --- | --- |
| H particle-step budget | `label_transfer_run` CLI arg 2; `ledger::H_DAG_BUDGET_PARTICLE_STEPS` | 10^8 default, 10^9 in the r3 supplement |
| H budget semantics | `label_transfer::BudgetSemantics` | `tree-v0` (unmemoized) or `dag-v1` (memoized); a cap at one and a measurement at the other is a semantics change, never the same statistic improving |
| Cold-H and authority-receipt budgets | `fiber_probe.rs::H_BUDGET`; `AUTHORITY_BUDGET` in `predictive_rank.rs`, `policy_geometry.rs`, `separation_probe.rs` | 200,000,000 particle-steps |
| Frontier cap / unpruned bound / LP pivot caps | `policy_geometry.rs` | 16,384 / 1,024 / 200,000 and 4,000,000 |
| Deadness support bound and receipt budget | `deadness_probe.rs::GT_SUPPORT_BOUND`, `RECEIPT_BUDGET` | 400 / 50 |
| Carrier and state stops | `census_run.rs::T5_CARRIER_STOP`, `A1_STATE_STOP` | 20,000,000 / 100,000,000 |
| Walker exhaustive threshold, sample draws, base seed | `WalkerConfig::{ci, fixture}`; `walk_corpus` header line | CI: exhaustive ≤ 40,000, 64 draws, seed `0x5ea7425a`; full walk: 1,000,000 / 2,000 draws |
| Domain fiber cap | `basin::DomainSpec` | tricks 3–6, fiber ≤ 40,000 in the falsification and economy domains |
| Deterministic decimation | `predictive_rank.rs`, `policy_geometry.rs::DECIMATION` | `(7919, 12)`, `(104729, 6)`, `(1299709, 3)` — deterministic strides, adopted after prefix sampling was rejected |
| Freeze digests | `deadness_probe.rs::FREEZE_DIGEST`, `fiber_probe.rs::STORE_FREEZES`, `separation_probe.rs::LIB_DIGEST` | Written into every checkpoint and store so a stale cache cannot be silently reused |

The discipline that must survive any reuse, and that §2 restates for the live
instruments:

- **Caps are exclusion, never sampling.** An over-budget H measurement returns
  nothing and the decision is recorded as capped; `Unmeasured` is never
  `Measured(0)`.
- **Every declared stop is printed.** Carrier stops, budget exhaustion,
  out-of-scope coordinates and excluded decisions all appear in the results
  file with their counts, and control-bias annotations travel with capped
  domains (the fiber-cap exclusions skew low-control).
- **Where sampling does happen it is marked in the type.** The walker's
  above-threshold fibers use the kernel's exact uniform sampler at a recorded
  per-decision seed and every downstream quantity is graded `Sampled`.
- **Determinism is structural.** Reductions are exact integer sums and counts,
  so thread partition and schedule cannot move a result (checked by
  `thread_independence`); caches store exact values of projected states, so
  trimming one cannot change an output; the deadness runner's deterministic
  block is byte-identical across invocations and survived a mid-run kill at
  41/45.
- **Raising a budget is lawful; coarsening a key is not.** The r3 supplement is
  the worked example — same solver, same semantics, larger declared budget,
  recorded as a budget change.

### 7.6 Caveats before reaching for something old

- **Several historical instruments lived only inside example binaries, and all
  of those are archive-only now.** The three deadness detectors (`d0`,
  `d1_sym`, `d1_win`), the exclusion-predicate engine, the endgame form store
  and floor table, and the railyard level-step drivers were functions inside
  `deadness_probe.rs`, `fiber_probe.rs` and `census_run.rs`; the r3 retrograde
  class machinery and yard/suffix-library routines were library code in
  `walt-skeleton::equivariant`. Reusing a detector now means retrieving it from
  `648f93a` and lifting it into the unified crate first. The exp3A atom
  vocabulary was ported into `walt-skeleton::atoms::Exp3aAtom` with the
  90 → 33 → 8 reproduction as a live test — archive-only with the skeleton;
  the frozen Python records under `probes/exp3a/` remain in place.
- **Legacy receipt-corpus statistics are pip-trump.** The corpus
  (`rob/receipts/verify_player.txt`) has no doubles-trump and no no-trump
  hand, so every statistic derived from it — including every §3 instrument
  that reads its roots — validates the pip-trump path and nothing else. The
  one exception is the complete level-one alphabet run, which enumerates its
  own carrier and is still declared pip-trump only. The independently
  generated freeze-56 M2 carrier has its own frozen scope and is not typed by
  this caveat.
- **Discipline carried by types, not by promises**, then and now: policies map
  info-state ids, so a world-peeking policy will not compile; caps exclude
  rather than sample; and in the archived factory the ledger's deletion path
  needed a token whose only constructor was a registry with a checker in it.

## 8. Open items on this page

- Whether `pkg/walt.wasm` / `pkg/walt2.wasm` should be rebuilt after the
  2026-09-02 and 2026-09-06 `api.rs` changes, or whether the committed
  binaries are intentionally frozen beside `experiments/partnership/reference/phone/`
  — and which binary plunge actually runs today. Not settled here.
- Per-hand runtimes for `shadow`, `v5flip`, `wakeup` and `l2_controller` are
  not recorded anywhere (the records are byte-deterministic by design); a
  one-off measurement is needed if a per-instrument cost column must be
  complete.
- `walt/ci/run_test_binaries.py`'s `HEAVY_FIRST` names a nonexistent
  `solver_focal_budget` suite; and whether `check_m2_metal.sh` still runs green
  after the fold has not been traced. Both are `walt/` files, flagged here, not
  repaired.
- `walt/probes/factor_belief/README.md` line 531 mislabelled bellmanreport's
  [145,606]‰ start as action 3-1 (it is 4-1's); the record wins. The README
  was corrected at this branch's `6a319216` (2026-09-13) — a wiki-round
  commit that also touched `walt/LOG.md`, `walt/MAP.md` and
  `walt/SCENARIO-PLAYER.md`; all four are `walt/` prose files, none is a
  record or a gate.
- The "level-2 costs ≈ 25–50× level-1 per decision" figure some prose carries
  is not in `level2_results_2026-08-17.txt` and was not verified in this pass.
- The `report` modes of §3.4–§3.5 and `openingreport` were not re-run on
  2026-09-13 (minutes to hours each); their figures are the committed
  records'.
- The L2-thread probe records (§3.2: `fieldswap*`, `hazard_witness`,
  `l2_controller`) were bin-run at σ0 = `Level0{n0=8}`, while every
  instrument since the counted-belief round, the act/waking/gran seats and
  the L2 gates themselves run at `Level0{n0=2}` (§2). No record re-runs the
  L2-thread bins at `n0 = 2`, so their numbers compose with nothing later;
  whether such a re-run is wanted is an open call (survey of 2026-09-07),
  not scheduled anywhere.
