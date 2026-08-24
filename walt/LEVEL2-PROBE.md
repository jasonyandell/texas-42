# The level-2 probe — field-swap pivotal mass (SPEC, not started)

Status: SPEC 2026-08-23, exploratory tier throughout. Owns: the level-2
detection program — where a level-1 field model provably loses value, and
how to find those positions without building the full level-2 player
first. Sources: `TILT-AUDIT.md` (the instrument), `CENSUS-RULINGS.md`
SP-A1..SP-A12 (vocabulary), `math/signed_pivotal_geometry_v0.1.md`
(q/τ/g/H), two live plunge review hands (2026-08-23, screenshots in the
session record; positions to be re-derived from game seeds when the probe
runs). Nothing here is promoted by its own existence; estimates are never
receipts; not a P-A21 statement.

**Deliberately not started.** Prerequisites, in order: (1) walt
unification (one crate, archived computations queued for recompute);
(2) wiki re-synthesis; (3) Jason's adaptive-sampling mathematics lands
(replace fixed-n with dig-until-settled). Then this probe runs with
sampling noise diminished as a variable. GPU is the likely substrate for
the full level-2 field (multiplicative solve blowup) — not yet, on
Jason's word.

## The two motivating specimens (2026-08-23, plunge review)

Both from live hands with walt seats, via the "How'd I do? Ask walt"
review (options priced from the acting seat's fiber, level-0 field).

1. **The saturation-tie revelation (bid 31 sixes, made 36).** Trick 4,
   partner seat holds 5-5; all four of her options price 100%/160
   worlds — a full saturation tie, tiebreak decides. But the bidder's
   live fiber still contained worlds where opponents held the 5-5 and
   his off 5 dies; his play was shaped by that worry. Everything at
   stake in her choice (when to reveal the 5-5, relieving the bidder)
   was invisible to her evaluator, because the modeled bidder has no
   fiber and cannot be worried or relieved.

2. **The count-timing near-tie (bid 30 sixes, set 25–17).** Trick 1,
   partner holds 6-2 and 6-4 (ten count) under the bidder's winning 6-6
   lead: slough the 10 now vs hold the count-trump. 90% vs 80% at 40
   worlds; a 160-world rerun flips the pick — the near-tie/hardness
   regime the tilt audit characterized, behaving as predicted. The
   under-layer: a level-0 bidder model that does not reason about where
   the count went makes the two lines look nearly identical, so
   sampling noise decides. Trick 3 of the same hand (when to show the
   6-4, the tile the bidder feared) is specimen 1's pattern again at a
   77/72 near-tie.

The shared structure: the value of the acting seat's choice is carried
by the *partner's response to the information it conveys* — the channel
a level-0 field zeroes out. (The level-0 mind does condition on the
public record, but heuristically; it maintains no fiber, so revelation
prices at zero.) Human partnership conventions — show support early,
give count to partner's winner — are compiled level-2; walt cannot
currently derive any of them.

## The detector

A decision is **level-2-relevant** exactly where pivotal mass wakes up
under a field upgrade:

    q(level-0 field) ≈ 0   but   q(level-1 field) > 0

i.e. the acting seat's options are saturation-tied (or honestly
near-tied) when the field is the modeled heuristic, but separate when
the field seats are level-1 minds — minds that maintain their own
fibers, condition on the record epistemically, and re-evaluate. So the
full level-2 player is not needed to find where level 2 matters: run
the existing tilt instrument (`tiltaudit.rs`, phases A–B) with one
swap — replay's field decisions taken by level-1 evaluation instead of
`Solver::modeled_choice` — restricted to the logged tie/near-tie sets.

Where q stays ≈ 0 under the upgraded field, the indifference is honest
and level 1 loses nothing there. Where q wakes up, we hold the
position, the option pair, and the mechanism (what the partner's fiber
does with the revealed tile) — the anchor corpus for the level-2 build.

Anchor sources already logged or cheap to log: race-mode saturation-tie
disagreements (`level1_raced` eliminations that end tied), the level-2
trick-1 saturation episode (TILT-AUDIT smoke scope item 2), and plunge
review hands like the two specimens (re-derivable from game seeds).

## The hypothesis worth testing on the way

**Level 2 does not just pick better — it should make decisions easier
to sample.** If modeling the partner's response spreads the true values
of currently-tied options apart, the gap g grows and fixed-pair
hardness H = 1/(qτ²) − 1 drops: fewer worlds settle the choice. Part of
the near-tie fog at level 1 may be an artifact of a field model too
dead to feel the difference. If confirmed, the level-2 seat could be
*cheaper per decision at equal confidence* than its rollout cost
suggests — the extra modeling buys back samples. Exploratory
hypothesis; the probe measures it directly (paired H̄ under both fields
on the same positions).

## Cost model and mitigations

Level-1 field = every field decision inside every rollout is itself a
fiber-sampling solve: multiplicative in (outer worlds × field decisions
× inner sample). This is why the probe is positions-restricted and why
the full level-2 seat points at the GPU (the exact workload factors
into GPU-shaped batches + CPU search — Jason's escape hatch, not yet).
Already-built mitigations: the field is deterministic per information
state (no tape — scenario = world), so repeated field solves are
shareable via the pi-cache; block racing eliminates dominated options
early; the detector confines level-2 spend to positions where it can
matter. The adaptive-sampling mathematics (to land before this runs)
replaces the fixed-n outer loop with dig-until-settled, removing the
40-vs-160 flip mode entirely.

## Cheap first-order UI detector (independent of all the above)

The plunge review can price the tapped seat's options from the
*viewer's* fiber alongside the actor's — two columns, "from her seat /
from yours." Same machinery, different root viewer, no new math. Rows
where the columns disagree are the human-visible flag of the same
asymmetry this probe mines (specimen 1 would have shown sub-100% from
the bidder's seat until the 5-5 shows). Worth shipping whenever the
viewer work is next touched; it is not gated on the probe.

Shipped on the walt side 2026-08-24 ([[viewer-cross-fiber-review]]):
`viewer_fiber_evaluate` in the solver (one authority), the optional
`viewer`/`viewer_hand` pair on walt-wasm `play`, and the webtable's
two-column reasoning panel with disagreement rows flagged. Plunge-side
wiring belongs to [[plunge-walt-sync]].

## Gates

- Probe runs only after unification + adaptive sampling land (above).
- Single-look discipline per panel (O14); tie sets predeclared from
  logs, not cherry-picked after pricing.
- Vocabulary: pivotal cover / pivotal win share / frozen policy per
  SP-A2/A3; "level-1 field" means field seats evaluated by the level-1
  machinery at a declared freeze tuple — that tuple is part of the
  probe's policy ID (SP-A8).
- Output contract: per position — q̂/τ̂/ĝ/Ĥ under both fields, paired;
  the wake-up set; per-pair mechanism notes. Estimates, never receipts.

## Amendment 2026-08-24 — the wake-up split and the true cost coordinate (CE-A6)

`math/calculated_evidence_v0.1.md` §14 corrects this spec's detector;
adopted at CE-A6 (`CENSUS-RULINGS.md`). Three distinct objects replace
the single "pivotal mass wakes up" detector:

- **Response wake-up:** `q₁ − q₀ > ε_q` — newly active response
  structure. NOT by itself a value statement: the upgraded field may
  create many disagreements whose signs balance exactly (q₁ > 0 with
  τ₁ = 0, g₁ = 0).
- **Value wake-up:** `g₁ − g₀ ≠ 0` (or by a declared amount), settled by
  the paired field-correction evidence of §14.6 (Zᵢ = Yᵢ⁽¹⁾ − Yᵢ⁽⁰⁾ on
  the same world, bounded-mean engine on Zᵢ/2).
- **Decision wake-up:** the selected action changes, or an
  unresolved/equivalent comparison becomes settled.

Sampling cost under each field is compared by the information rate
`𝓘_f = q_f · D_{1/2}(τ_f)` — never by q̂ alone and never by a noisy
plug-in Ĥ ordering. Exact-zero claims (q₀ = 0) require enumeration or a
structural proof; sampling supports only `q₀ ≤ ε_q` at declared risk —
the output contract keeps that distinction.

**Gate (updated):** unification (done, 2026-08-24) + the
calculated-evidence outer adaptive controller landed and green
(CE-A7 build program; [[adaptive-sampling-intake]]). The original
q̂/τ̂/ĝ/Ĥ output contract gains the paired-Z evidence columns and the
per-field 𝓘 estimates; "estimates, never receipts" stands.

## Amendment 2026-08-24 (second) — this probe is the detection layer of the targeted program (L2-A5)

`math/targeted_level2_field_stability_v0.1.md` (intaken and adjudicated
same day, L2-A1..A7 in `CENSUS-RULINGS.md`) owns the *targeting* layer
above this spec: the field-disagreement frontier `𝓕_{0,1}`, exposure
upper bounds by rung (E0–E4), the stability screen
`|Q_a^(1) − Q_a^(0)| ≤ R_a` with survivor-only field-1 optimization
(L2-T2..T4), first-split traces as explanations, and the cycle
discipline. This probe's paired q̂/τ̂/ĝ/𝓘 output contract is the
detection evidence consumed by that controller's Stages 1–2. One
field-swap program: detection here, targeting there — neither restates
the other. The gate line from the first amendment stands, with one
addition from L2-A6: the field-swap *build* enters after the
calculated-evidence shadow step (CE §22 step 7) merges.
