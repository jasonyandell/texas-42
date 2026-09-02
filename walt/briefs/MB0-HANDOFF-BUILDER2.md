# MB0-HANDOFF-BUILDER2 — audit notes from the mistakenly-spawned second builder

**Tier: walt-exploratory.** Nothing in this file is a receipt. Every
number below is an instrument reading from one machine on 2026-09-01;
quotable only if a gate receipt re-establishes it. Where a finding was
relayed to me rather than reproduced by me, it says so on the line.

**Provenance.** I am `mb0-builder-2`, spawned by the team lead on the
mistaken belief that the original `mb0-builder` had been killed. It had
not — the harness's task tracking dropped it silently. The lead's
corrected ruling gives the slice back to the original builder; this file
is my handover. A third agent, `mb0-builder-3`, was spawned on the same
false signal and owns `MB0-COLLISION-NOTES.md`; I have not touched that
file. Sources: `walt/briefs/BRIEF-MB0.md`,
`walt/math/model_belief_base_player_v0.1.md` §§5–20 and §§74–76, and its
intake companion.

## 1. Gate results on commit f4991cc

`f4991cc` (`walt MB0: WIP — model-belief slice draft (audit
checkpoint)`) is my snapshot of the working tree at the moment I found
it: the 1249-line `Arc`-based `model_belief.rs`, the 1280-line gate
file, and the `mod.rs` wiring. It is additive and destroyed nothing.

One run of `cargo test --release --test solver_model_belief` against
that snapshot, killed at **16 minutes** to stop taking CPU from the live
builder:

| Gate | Test | Outcome |
|---|---|---|
| G7 | `behavior_type_identity_tracks_every_coordinate` | pass |
| G5 | `fixed_policy_value_is_linear_in_the_model_belief` | pass |
| G2 | `point_mass_parity_reproduces_both_fixed_field_authorities` | FAIL, under 60 s |
| G3 | `posterior_closure_and_the_half_vs_quarter_persistence_specimen` | FAIL, under 60 s |
| G4 | `merge_before_max_and_no_hidden_type_policy_key` | FAIL, under 60 s |
| G1 | `enumeration_parity_over_augmented_pairs` | unfinished at 16 min |
| G6 | `separated_upper_bounds_the_mixture_response_and_zero_iff_common_optimizer` | unfinished at 16 min |

I do not have the panic text: cargo prints failure detail only in its
final section, which the kill pre-empted. All three failures are cheap
to re-surface with `--test-threads=1 --nocapture` and a per-gate filter,
since each fails inside the first minute.

The two gates that did not finish are exactly the two that sweep every
root under F₁ with the full eight-profile bundle — G1 enumerates (ω, θ)
pairs on three roots, and G6 runs `mixture_response` plus eight
`response_success_mass` calls per root action across six roots and two
type pairs. That runtime is itself a finding, and §2 explains it.

## 2. The σ1 hazard — an MB1-blocking flag

**Verified by me at the source.** `walt/walt/src/solver/mod.rs`,
`sample_belief` (the void-conditioned belief sampler, documented as
"uniform on the lawful-completion fiber by shuffle-and-reject",
`SCENARIO-PLAYER.md` §4.2), is an unbounded rejection loop:

```rust
while out.len() < n {
    // shuffle the unseen tiles
    // deal them out by `sizes`
    if w[s] & voids[s] != 0 { ok = false; break; }   // reject
}
```

There is no attempt cap and no feasibility precheck. On a frame where no
void-respecting completion exists, the loop spins forever. `FieldKind::
Level1` reaches it through `level1_action` → `level1_evaluate` →
`sample_belief`; `FieldKind::Level0` never does.

This is a **pre-existing hazard in shipped code**, not an MB0 bug. What
MB0 does is expose it, because the slice consults the field at every
hand in the factor support and so reaches frames the live player never
constructs. Two consequences for the finisher:

- Do **not** repair `solver/mod.rs`. It is upstream of the live player,
  and the brief puts the live player off limits. Keep any guard additive
  and inside the slice.
- Record the hazard explicitly in the `FACTOR-BELIEF.md` paragraph and
  in the final report. It is the kind of thing that must not be fixed
  silently, and it plausibly blocks MB1's larger roots.

**Relayed, not verified by me** (from `mb0-builder-3`): a live specimen
of the infeasible frame — seat S3, hand {4-2, 4-4}, history
[4-1, 4-3, 1-1], sizes [1, 1, 1, 2], voids [16786368, 69173248,
33586176, 16786368].

## 3. Two red gates are root-choice bugs, not mathematics

**Relayed from `mb0-builder-3`, not reproduced by me.** Both explanations
are consistent with my run's failure timing, and both concern *decided*
roots, where the argmax DAG is legitimately empty:

- **G4** panics on `"the walk reached a focal state"` at h10-t6 because
  that root is decided at the root, so `CountingPolicy` is never
  consulted. h5-t6 is undecided and cheap.
- **G2** fails on its first root, h12-t6, under **F₀** — not F₁ —
  with `left: None, right: Some(4-4)`. h12-t6 is decided, so
  `choice_at(&[])` is rightly `None`, while `raw_authority` still
  tie-breaks a "chosen" action. The **selected-action** half of G2 needs
  scoping to undecided roots; the **value** half is sound everywhere.

Note the gate file's own G1 comment already anticipates this ("the
h12-t6 root is decided, so its argmax DAG is rightly empty"), so the
discipline exists in the draft and simply was not carried into G2 and
G4. That is the cheapest reading, and it is consistent with G3's
neighbouring failure being a fixture-selection problem too — worth
checking before suspecting the recursion.

**Relayed F₀-only walk costs, for sizing:** h5-t6 fixed 470 µs / respond
437 µs; h4-t6 767 / 1239 µs; h8-t5 4491 / 15013 µs. F₀ is cheap; the
cost is entirely F₁.

## 4. Mathematical audit of the draft — favourable

I reviewed the representation against §§5–20 before the collision
stopped me. It holds up:

- **Theorem 7.1 made concrete.** The belief over Ξ = Ω×Θ is stored as an
  exact per-profile expansion: one existing `FactorBelief` per type
  profile, an integer prior weight per profile, and one dispatching
  field per profile. Every mass, branch table, posterior and value is
  derived through the existing `factor_belief` authorities. Nothing
  forks the contraction, conditioning or recursion machinery, which is
  what §50's reuse ruling asks for.
- **Merge-before-max is structural, not asserted** (§13, §32, MB-I4).
  Hidden branching is by public action only. On the focal side the
  policy is consulted exactly **once per information state for the whole
  bundle**, so no policy — however stateful — can key its choice on the
  hidden type. That is a stronger guarantee than a runtime check, and it
  is the right reading of MB-I1's "make the API make it impossible".
- **The per-node max is the policy-class max.** Each focal information
  state is keyed by post-root public history, so it is reached along a
  unique path in the walk and the objective is additive over the tree.
  Nodewise maximisation therefore equals the global maximum over lawful
  focal policies. This is the same argument `grammar_success_mass`
  already documents, lifted to the weighted bundle.
- **Persistence is structural** (§9, MB-I2). Profile weights never
  change under conditioning; the evidence lives in the hand factors, and
  a profile can only be *dropped* when the observed action has zero mass
  under it. No API redraws a seat's type mid-hand, so the ¼ resampling
  semantics is unconstructible rather than merely untested.
- **The relaxation stays typed** (MB-I8). The type-revealed upper exists
  only as `separated_upper`, never as a policy the walk could follow.

What I could **not** establish is why G2, G3 and G4 fail — see §3 for
the most likely reading.

## 5. The probe binary

`walt/walt/src/bin/modelbeliefreport.rs`, untracked, left in the tree
for the finisher to adapt rather than rewrite. As of this handoff it
**builds clean**, passes `cargo clippy --release --bin modelbeliefreport
-- -D warnings -D clippy::float_arithmetic`, and is fmt clean against
the current `Rc`-based API. I have deliberately **never run it** — those
numbers belong to whoever owns the slice.

Design notes:

- One mode, `modelbeliefreport report <out.txt>`, matching the sibling
  probes. Intended output `walt/probes/factor_belief/modelbelief_run1.txt`.
- Emits the full §75 coordinate list per root: physical world mass;
  augmented mass with its profile factorisation; the prior; active types
  by seat; merged branch masses by public action with conservation
  checked against the parent; posterior type weights by seat after each
  observed action along a walked line; the fixed-policy mixture value;
  the exact mixture response; the separated upper; the model-fusion
  price; typed rows versus merged public branches; wall time; and the
  parity verdict.
- Closes with the five §76 criteria as explicit `YES`/`NO` lines, each
  with an evidence sentence, then a `GO` / `NO-GO` verdict line and the
  exploratory-tier footer. The criteria are accumulated in a `GoNoGo`
  struct during the run rather than asserted, so a red criterion prints
  as `NO` instead of panicking — the probe is meant to *report* the
  go/no-go, including a negative one.
- No floats: masses are exact `u128`, ratios are exact `BigRational`
  printed as ‰ (floored) **beside the exact integer pair**, wall time is
  integer microseconds.
- Memory follows the C2 house rule: a **declared accounting** over
  `size_of` (`declared_belief_bytes`) kept strictly beside a **measured**
  resident size from `/bin/ps -o rss=`, never merged into one figure.
- Criterion 3 (the mixture response differing nontrivially from a
  point-mass response) is detected by comparing the argmax policy's
  per-profile coordinates against the per-profile optima, so a specimen
  is captured with its exact rationals rather than asserted to exist.

Two things it still needs from the finisher: it must actually be run,
and its σ1 exposure is unbounded until the hazard in §2 is guarded — it
sweeps all six roots under F₁.

## 6. Things to size before calling MB0 done

- `walt/ci/check.sh` allocates a **fresh mktemp target directory** and
  then runs `cargo test --workspace --release` inside it. A green
  check.sh therefore means a full cold rebuild plus the entire workspace
  suite with these gates embedded. With G1 and G6 unbounded under F₁,
  that is not a background cost — budget it, or bring the F₁ gates
  under a cost ceiling first.
- The declared test epoch is F₁ = `Level1 { n_outer: 2, n0: 2 }`, which
  is already the cheapest lawful setting. The cost is not the sample
  counts; it is the number of distinct information states the factor
  support forces the field to classify. That points at support
  tightening rather than at the epoch.

## 7. One thing I broke

I ran `cargo fmt -- <my probe file>` meaning to format only my own
binary. `cargo fmt` ignores the path restriction and formatted the whole
crate, rewriting `model_belief.rs` and `tests/solver_model_belief.rs`
mid-edit. No content was lost — rustfmt changes whitespace only, is
idempotent, and produced exactly what check.sh's `cargo fmt --all
--check` demands, which now passes clean. The risk was to the original
builder's in-flight edits, and it has been told directly to re-read both
files before its next edit. Recorded here so the incident is in the
written record and not only in an agent message.
