# MB0-COLLISION-NOTES — independent audit findings, 2026-09-01

**Tier: walt-exploratory / scratch.** Nothing here is a receipt. Every
number below is a scratch measurement from an `#[ignore]`d probe test,
quotable as a result only if a gate receipt adopts it (`CLAUDE.md`, the
exploratory-stays-exploratory rule). Recorded here so the audit survives
an agent drop.

**Provenance.** Written by a third MB0 agent (`mb0-builder-3`) spawned on
the mistaken belief that `mb0-builder-2` had been killed. It had not
been: it was live-editing `walt/walt/src/solver/model_belief.rs` and
`walt/walt/tests/solver_model_belief.rs` throughout (1249 → 1352 → 1365
lines; test file 1433 → 1500; mtimes advancing 21:48 → 21:52), and went
on to create `walt/walt/src/bin/modelbeliefreport.rs`. Per the team
lead's ruling, `mb0-builder-2` owns the slice; this file is a
non-colliding audit record only. It is deliberately left UNSTAGED.

The findings below were established by reading, plus four `#[ignore]`d
scratch runs made before the tree stopped compiling under
`mb0-builder-2`'s in-flight `Arc<BehaviorType>` → `Rc<BehaviorType>`
migration.

---

## Finding 1 — the σ1 non-termination is real, and it is PRE-EXISTING

`walt/walt/src/solver/mod.rs:897-933`, `sample_belief`, is an unbounded
shuffle-and-reject sampler:

```rust
let unseen = FULL_MASK & !played & !viewer_hand;      // mod.rs:906
...
while out.len() < n {                                  // mod.rs:911
    // Fisher-Yates over `tiles`
    for &s in &others {
        w[s] = mask_slice(&tiles[off..off + sizes[s]]);
        off += sizes[s];
        if w[s] & voids[s] != 0 { ok = false; break; } // mod.rs:923
    }
    if ok { out.push(w); }
}
```

There is **no attempt cap and no feasibility precheck**. If the deduced
void frame admits no lawful deal, the acceptance probability is exactly
zero and the loop never terminates. This is a hang, not slowness.

**Reachability.** `FieldKind::Level1` reaches it through
`FieldModel::choose` → `level1_action` (`solver/field.rs:465`) →
`level1_evaluate` (`solver/mod.rs:1039`) → `sample_belief`.
`FieldKind::Level0` never does — it routes to `Level0Field::choose`.
So F₁ = `FieldKind::Level1` is exposed and F₀ = `FieldKind::Level0` is
not, which is why every F₀-only measurement below completes in
milliseconds.

**Why MB0 is the first caller to hit it.** MB0's mixture walk consults
the field at EVERY hand in the acting seat's factor support
(`branch_masses_via`, `factor_belief.rs:647-684`, which calls
`field_action` once per `actor_completion_weights` entry). That sweeps
far more information states than the live player's single-line play
ever does. The hazard is in shipped code; MB0 merely surfaces it.

### The live specimen

Caught by `mb0-builder-2`'s `GuardedF1` instrument, which enumerates the
acceptance region before delegating and panics instead of spinning:

```
UNSATISFIABLE frame for σ1: seat S3 hand {4-2 4-4}
  history [4-1, 4-3, 1-1]
  voids [16786368, 69173248, 33586176, 16786368]
  sizes [1, 1, 1, 2]
```

Three unseen tiles that cannot be dealt one each to the three other
seats under those void masks. Reproduce with the `#[ignore]`d
`scratch_guarded_f1_walk` over roots h5-t6 and h4-t6.

### The guard is FAITHFUL — this is not a false alarm

Worth stating explicitly, because the guard requires an EXACT partition
of `unseen` while `sample_belief` only slices a prefix
(`tiles[off..off + sizes[s]]`) and ignores any leftover. Those two
acceptance regions coincide, so the guard cannot over-report
infeasibility:

`ContinuationFrame::sizes()` (`solver/policy.rs:885-891`) returns
`boundary_hand_size` for all four seats, minus one for each seat that has
already played in the current trick. Hence `Σ_s sizes[s]` = total tiles
still in hand, and `played = 28 − Σ_s sizes[s]`. Therefore

```
|unseen| = 28 − played − |actor hand|
         = Σ_s sizes[s] − sizes[actor]
         = Σ_{s ≠ actor} sizes[s]
```

exactly. The prefix slicing always consumes `unseen` entirely, so
prefix-deal and exact-partition accept the same set of deals. ∎

### OPEN — why the frame is infeasible (the one experiment not run)

The exact-cover coupling *ought* to guarantee a satisfying deal: a hand
`A` is consulted only when `actor_completion_weights` gives it positive
weight `φ_s(A) · C_{-s}(U \ A)`, so a completion of the other hidden
seats exists, and the viewer's real hand completes the deal to all four
seats. That deal should satisfy every deduced void.

The most likely mismatch is between two constraint sets that are not the
same object:

- `continuation_frame` (`solver/policy.rs:898-950`) seeds `voids` from
  `root.voids` **for all four seats**, then adds history-deduced voids
  as the walk folds (`policy.rs:928-935`).
- the factor support uses `kernel.allowed(slot)`.

If `allowed()` does not encode everything `root.voids` does, the exact
cover can hand out deals that σ1's frame rejects — and when every
cover-legal deal is frame-illegal, σ1 spins.

**This is a hypothesis, not a finding.** It is settled by two
`#[ignore]`d scratch tests left in
`walt/walt/tests/solver_model_belief.rs` (~line 1288):

- `scratch_preexisting_f1_response`
- `scratch_preexisting_f1_response_guarded`

Both call `raw_authority` — a plain `FactorBelief::uniform_root` plus
`response_success_mass`, with **no model belief anywhere** — against F₁'s
mind on h5-t6, h4-t6, h12-t6, h10-t6. If the guarded one panics, the
hazard is confirmed pre-existing in the shipped stack independent of
MB0, which is an honest finding to report rather than something the
slice should silently work around.

Neither was ever run: the build broke under the concurrent edit on both
attempts. Roughly two minutes of work, and it decides how the slice
reports F₁.

**Scope caveat.** `solver/mod.rs` is not under freeze 58, but it is live
player code, which `BRIEF-MB0.md` puts off limits ("No live-player
change"). Fixing `sample_belief` is therefore out of scope for MB0 —
report it, do not repair it here.

---

## Finding 2 — two gates fail for ROOT-CHOICE reasons, not math reasons

**h12-t6 and h10-t6 are DECIDED AT THE ROOT.** h5-t6, h4-t6 and h8-t5
are undecided. From `scratch_walk_size` under F₀ (`decided_early` /
`focal_nodes`): h5-t6 = 0 / 26, h4-t6 = 20 / 31, h8-t5 = 56 / 51.

**G4 `merge_before_max_and_no_hidden_type_policy_key`** panics
`"the walk reached a focal state"` (assert at the `!calls.is_empty()`
line). It runs on h10-t6, which `decided_success` settles at the root,
so `CountingPolicy` is never consulted and the MB-I1 one-consultation
law has nothing to witness. G4 needs an **undecided** root; h5-t6 is
undecided and cheap.

**G2 `point_mass_parity_reproduces_both_fixed_field_authorities`** fails
on the FIRST root, h12-t6, under **F₀ — not F₁**:

```
assertion `left == right` failed: the δ selected action equals the
fixed-field authority's (hand 12 trick 6, level0-modeled-mind-v1)
  left: None
  right: Some(4-4)
```

At a decided root the extracted argmax DAG is legitimately empty — G1's
own comment already says so ("the h12-t6 root is decided, so its argmax
DAG is rightly empty") — so `response.policy.choice_at(&[])` is `None`,
while `raw_authority` still tie-breaks a `chosen` action off equal
per-action masses. The **value** half of G2 is sound everywhere; only
the **selected-action** half needs scoping to undecided roots.

**G7 `behavior_type_identity_tracks_every_coordinate` passes green** as
written, unmodified.

---

## Finding 3 — a §76 go/no-go criterion no gate covers

§76's five criteria are the gate between the vertical slice and the
base-player program. The third reads:

> the mixture response differs nontrivially from at least one point-mass
> response on a specimen

**No drafted gate tests this.** G6 tests Theorem 18.1 (`Q(ν) ≤ U^sep`)
and Theorem 19.1's zero-iff-common-optimizer characterization, which is
a different statement: `Φ = 0` says the mixture response equals the
prior-weighted point-mass OPTIMA, not that `Q(ν)` differs from any
individual `Q(δ_θ)`.

Unless some gate witnesses `Q(ν) ≠ Q(δ_θ)` for some θ on some root, the
probe cannot honestly print YES on criterion 3 — and under the
exploratory-tier rule an unwitnessed YES is exactly the promotion the
brief forbids.

---

## Scratch measurements (F₀ only — F₁ walks never completed)

`scratch_walk_size`, release build, δ-prior over F₀, focal
`FixedPreference::lowest_first`. Masses are exact integer pairs.

| root | fixed µs | mass | respond µs | mass |
|---|---|---|---|---|
| h5-t6 | 470 | 12/27 | 437 | 12/27 |
| h4-t6 | 767 | 30/90 | 1239 | 78/90 |
| h8-t5 | 4491 | 64/92 | 15013 | 91/92 |

Walk census (fixed / respond), h8-t5: `focal_nodes` 51 / 353,
`hidden_nodes` 242 / 1293, `conditionings` 318 / 1599.

`scratch_level1_timing` — individual F₁ reads are FAST, which is what
makes the hang a hang rather than a cost problem: h5-t6 reads measured
1, 0, 0, 1870, 1344 µs; h4-t6 reads all 0 µs.

---

## Handover pointers

- `BRIEF-MB0.md` gates G1–G7 and the §75 report spec are the authority;
  the draft is not.
- The §75 report wants, per root: physical world mass; augmented mass;
  prior; active types by seat; branch masses by public action; posterior
  type weights after each observed action; fixed-policy mixture value;
  exact mixture response; separated upper; model-fusion price; distinct
  type actions vs merged public branches; wall time and declared memory;
  parity verdict — then the five §76 criteria as explicit YES/NO lines.
- The intake companion governs where it repairs the parent: F₀ =
  `FieldKind::Level0` = σ0, F₁ = `FieldKind::Level1`, Dice is the rung
  BELOW F₀ and is not σ0. The §8 erratum: `Q_a(δ_{F_k})` equals the
  ordinary exact best response to rung `F_k`.
