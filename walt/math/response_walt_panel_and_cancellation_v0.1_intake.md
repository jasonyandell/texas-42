# Intake companion — the x:019–023 panel response ("Cancellation and Irrelevance")

**Parent (verbatim, never edited):**
`exchange/inbox/019-023-response-panel-and-cancellation-v0.1.md`
SHA-256 `a3f468aa8742fd03bf6fc9044caa916c0b0d7a1f1eb7ed5ca4888081e93c2685`.
Companion verifier (byte-identical to the parent's embedded program
modulo one trailing blank line):
`exchange/inbox/verify_walt_panel_response_v0_1.py`
SHA-256 `262b991c009dd0e6a4ed74084668f8bc09bcf3ffef5017c25f37ed4048d1a2ce`.

**Status of this document:** exploratory intake record. The verifier is
**scratch tier at best** (Jason, at delivery): useful, helpful, a
reference — never a candidate for import into the codebase, and its
PASS lines are session evidence, not receipts (TRUST-01).

**Provenance:** one consolidated Pro response to the five adversary-panel
briefs Jason hand-ferried 2026-08-24 as an authorized batch (dispatches
019–023, `exchange/outbox/`). Adjudicated same day per the exchange
protocol: program executed, arithmetic step-checked, the one concrete
correction re-verified independently from our own raw records.

## Adjudication receipt (what was actually re-run)

- **Verifier executed** (stdlib Python 3, exact `fractions`, no floats,
  fixed seeds only for grid generation): **36/36 PASS, exit 0**
  (`ALL CHECKS PASS`). Log: session record 2026-08-24.
- **Embedded-vs-shipped diff:** the parent's embedded program differs
  from the shipped `.py` by one trailing blank line only.
- **The Λ correction re-verified independently** (the response's only
  claim against committed prose): counting all 2,400 world records for
  `receipt-h8-t4` in `walt/probes/fieldswap/fieldswap.jsonl` directly —
  reveal c⁺=30, c⁻=26 (c = +1/300); retain c⁺=45, c⁻=72 (c = −9/400);
  |Λ| = |4/1200 − (−27/1200)| = **31/1200**. The committed prose said
  41/1200 in two places (probe README, wiki era page); the component
  counts were always right, the difference was mis-added. Both sites
  corrected in this intake's branch with correction notes in place.
- **Claim-D counterexample arithmetic checked:** three individually
  valid one-shot e-values (mass 1/8 at 8), retrospective opening of a
  crossed one ⇒ false-cross probability 1 − (7/8)³ = 169/512 > 1/4 =
  δ_dec. The adversary is real; Claim D as written falls.
- **O26 ambiguity fixture checked:** the 5-world, 3-candidate table
  diverges between canonical per-index liveness and a batch-start
  liveness reading (E⁺₄,₁ = 19/10 crosses T = 3/2 for a canonically
  dead candidate). W1–W6 as written under-determine the invariant.

## Verdicts, labeled by thread (per Jason's 2026-08-24 framing ask)

| Dispatch | Thread | Verdict (response's own words, abridged) |
|---|---|---|
| 019 evidence process | **CE (depth)** | CERTIFIED, claims A–I |
| 020 bounded mean | **CE (depth)** | CERTIFIED A–E; task-D sign-safe wording narrowed (unrestricted class is not sign-safe; subclasses are) |
| 021 risk ledger / escalation | **CE (depth)** | A–C, G–H certified; **Claim D counterexampled as written** (repairs: future-only opening or preallocation); E conditional; F's "refund" replaced by final-result typing |
| 022 execution order | **CE (depth)** | Under-specified but repairable: W5 needs full per-index liveness; W7–W11 proposed; conditional-null question resolved positively under predictable activation |
| 023 L2 coupling theorems | **L2 (model)** | CERTIFIED L2-T1..T5 with the coupling definition repaired (first split defined on the common prefix via the stopping time τ); T3's optimality hypothesis redundant; T4's interval hypotheses made explicit |
| Part VI (unsolicited new math) | **both, typed** | cancellation ladder \|c\| ≤ r ≤ d (L2, fixed-policy); pairwise (B,H,q,g) masses (CE, pairwise); directional bounds R±_a and directional screening (L2, root-action); dominance theorem (objective-level, thread-free) |

## What the response adds (Part VI, the substantive new mathematics)

1. **The cancellation ladder** |net value correction| ≤ outcome-change
   mass ≤ field-exposure mass, with three distinct zeros (behavioral
   irrelevance d=0, outcome irrelevance r=0, value neutrality c=0) that
   must never be collapsed into one "close" label.
2. **Pairwise benefit/hazard masses** B(a|b), H(a|b) with g = B−H,
   q = B+H: small |g| from near-agreement vs. from heavy exchange are
   different objects; reports must retain (B,H,q,g), not g alone.
3. **Dominance theorem:** H(a|b)=0 (with B>0) is strict dominance —
   Jason's high-trump-versus-vulnerable-double example is one-sided
   unforced risk, not cancellation; zero observed hazards in a sample
   never proves H=0 (exact enumeration or a valid bound required).
4. **Directional root-action bounds** R⁺_a, R⁻_a with
   Q⁰_a − R⁻_a ≤ Q¹_a ≤ Q⁰_a + R⁺_a, directional winner stability
   (only "winner worse" and "rival better" can flip), and directional
   screening — the natural tightening for split-heavy roots where the
   symmetric exposure bound prunes nothing (our h8-t4 regime).
5. **Rung ladder extension:** R±_a ≤ R^outcome_a ≤ R^exposure_a — the
   coupled branches must run to terminal for the directional rungs
   (more expensive than split-reach, potentially much tighter).

## Required amendments (response §40) and their disposition

1. 41/1200 → 31/1200 — **done in this intake** (both sites, with notes).
2. Sign-safe wording narrowed — recorded against dispatch 020's task D;
   binding on future CE prose (PANEL-A2).
3. Claim D repair (future-only / preallocated opening) — accepted as a
   binding obligation on the controller (PANEL-A3); current shipped
   controller behavior to be audited against it (open item below).
4. "Refund" → final-result typing + pathwise future-allocation rule —
   accepted (PANEL-A4); matches the existing "exact results spend no
   risk" preroute semantics, which are final-result-scoped already.
5. W5 → canonical per-index liveness replay; W7–W11 — accepted
   (PANEL-A5); shipped controller's batch semantics to be audited.

Full rulings: `walt/CENSUS-RULINGS.md`, "The panel-response adjudication
(2026-08-24)", PANEL-A1..A8.

## Open items this intake creates

- **Audit the shipped controller** (solver::controller / adaptive)
  against W7–W11 and the Claim-D repair: does any code path open an
  edge retrospectively on already-consumed evidence, and is per-index
  liveness canonically replayed under batching? (Both are believed to
  hold by construction — single-stream, preallocated all-pairs α — but
  belief is not a receipt; card the audit.)
- **Slice 3 consumes Part VI:** directional rungs R±_a next to the E4
  exact split-reach route; (d, r, c⁺, c⁻, c) retained per fixed-policy
  report; (B,H,q,g) per pairwise report; the six result labels
  (NoFieldExposure / OutcomeStable / ValueNeutral / EpsilonEquivalent /
  Dominated / Unresolved) typed into the field-swap kinds.
- **τ-based coupling definition** back-ported into the L2 parent's
  working notes (the slice-2 implementation already forks at the first
  common-prefix disagreement — audit that it matches the response's
  stopping-time definition exactly; believed yes).
