/-
Copyright (c) 2026 Jason Yandell. All rights reserved.
Released under Apache 2.0 license as described in the file LICENSE.
Authors: Jason Yandell
-/
import Texas42.Basic

/-!
# Texas 42 — the Straight auction

Layer B first slice (Math §4.3, ledger rows PA-B01–PA-B03): the configured
auction as a finite state machine with one action per seat, bid legality,
the deterministic transition, and the structural reachable mark ceiling
`min(maxMarkBid, 5)`.

Design (Handoff §Layer B): configuration values stay explicit parameters;
bids are plain data with legality a separate decidable predicate; the
transition is a total function whose legal-precondition is tracked by the
`LegalAuction` derivation predicate.
-/

namespace Texas42

/-- PA-B01: an auction action — pass, a point bid `P(n)` (`30 ≤ n ≤ 41`),
or a mark bid `M(m)` (`1 ≤ m ≤ maxMarkBid`). Range legality lives in
`AuctionState.legalBid`, not in the type (Math §4.3). -/
inductive Bid where
  | pass
  | point (n : ℕ)
  | mark (m : ℕ)
deriving DecidableEq, Repr

namespace Bid

/-- The bid order embedding `P(30) < ⋯ < P(41) < M(1) < M(2) < ⋯`
(Math §4.3), as a numeric value: `P(n) ↦ n`, `M(m) ↦ 41 + m`. `pass`
carries no order weight. -/
def value : Bid → ℕ
  | pass => 0
  | point n => n
  | mark m => 41 + m

end Bid

/-- PA-B01: auction configuration — the positive configured mark-bid cap
(Math §4.3). -/
structure AuctionConfig where
  maxMarkBid : ℕ
  maxMarkBid_pos : 0 < maxMarkBid

/-- PA-B01: one auction attempt — the shaker and the bids made so far, in
acting order (seat `shaker + 1 + k` acts `k`-th; each seat acts once). -/
structure AuctionState where
  shaker : Seat
  bids : List Bid
deriving DecidableEq

namespace AuctionState

/-- The seat to act (meaningful while `bids.length < 4`). -/
def actor (a : AuctionState) : Seat :=
  a.shaker + 1 + ⟨a.bids.length % 4, Nat.mod_lt _ (by norm_num)⟩

/-- The current high bid: the latest nonpass bid, if any. Legal nonpass
bids strictly exceed all predecessors, so the latest nonpass is the
maximum on legal histories. -/
def highBid (a : AuctionState) : Option Bid :=
  a.bids.foldl (fun acc b => match b with | .pass => acc | b => some b) none

/-- A candidate value exceeds the current high bid (vacuously, if none). -/
def Exceeds (hb : Option Bid) (v : ℕ) : Prop :=
  match hb with
  | none => True
  | some b => b.value < v

instance : ∀ (hb : Option Bid) (v : ℕ), Decidable (Exceeds hb v)
  | none, _ => inferInstanceAs (Decidable True)
  | some _, _ => inferInstanceAs (Decidable (_ < _))

/-- Mark-bid entry/overcall structure (Math §4.3): before any mark bid,
entry at `M(1)` or `M(2)`; after `M(r)`, the only legal mark overcall is
`M(r + 1)`. -/
def MarkEntry (hb : Option Bid) (m : ℕ) : Prop :=
  match hb with
  | some (.mark r) => m = r + 1
  | _ => m = 1 ∨ m = 2

instance : ∀ (hb : Option Bid) (m : ℕ), Decidable (MarkEntry hb m)
  | none, _ => inferInstanceAs (Decidable (_ ∨ _))
  | some .pass, _ => inferInstanceAs (Decidable (_ ∨ _))
  | some (.point _), _ => inferInstanceAs (Decidable (_ ∨ _))
  | some (.mark _), _ => inferInstanceAs (Decidable (_ = _))

/-- PA-B02: bid legality (Math §4.3). Each seat acts once (four actions
total); a point bid is in range and exceeds the high bid; a mark bid obeys
the cap and the entry/overcall structure (which implies exceedance). -/
def legalBid (cfg : AuctionConfig) (a : AuctionState) : Bid → Prop
  | .pass => a.bids.length < 4
  | .point n =>
      a.bids.length < 4 ∧ 30 ≤ n ∧ n ≤ 41 ∧ Exceeds a.highBid n
  | .mark m =>
      a.bids.length < 4 ∧ 1 ≤ m ∧ m ≤ cfg.maxMarkBid ∧ MarkEntry a.highBid m

instance (cfg : AuctionConfig) (a : AuctionState) :
    ∀ b : Bid, Decidable (a.legalBid cfg b)
  | .pass => inferInstanceAs (Decidable (_ < _))
  | .point _ => inferInstanceAs (Decidable (_ ∧ _))
  | .mark _ => inferInstanceAs (Decidable (_ ∧ _))

/-- PA-B02: the deterministic auction transition — append the bid. -/
def step (a : AuctionState) (b : Bid) : AuctionState :=
  ⟨a.shaker, a.bids ++ [b]⟩

/-- The auction attempt is complete after four actions. -/
def Complete (a : AuctionState) : Prop := a.bids.length = 4

instance (a : AuctionState) : Decidable a.Complete :=
  inferInstanceAs (Decidable (_ = _))

/-- The auction result: the winning seat and bid (the latest nonpass), or
`none` on all-pass (Math §4.3: four passes begin a new deal attempt). -/
def result (a : AuctionState) : Option (Seat × Bid) :=
  (a.bids.foldl
    (fun (acc : Seat × Option (Seat × Bid)) b =>
      (acc.1 + 1, match b with | .pass => acc.2 | b => some (acc.1, b)))
    (a.shaker + 1, none)).2

/-- PA-B02: legal auction states — the derivation predicate from an empty
attempt through legal bids. -/
inductive LegalAuction (cfg : AuctionConfig) : AuctionState → Prop
  | init (shaker : Seat) : LegalAuction cfg ⟨shaker, []⟩
  | step {a : AuctionState} {b : Bid} (ha : LegalAuction cfg a)
      (hb : a.legalBid cfg b) : LegalAuction cfg (a.step b)

/-- `highBid` after a step: a pass leaves it, a nonpass replaces it. -/
theorem highBid_step (a : AuctionState) (b : Bid) :
    (a.step b).highBid =
      match b with | .pass => a.highBid | b => some b := by
  cases b <;> simp [step, highBid, List.foldl_append]

/-- The auction invariant behind the mark ceiling: at most four bids, every
mark bid bounded by the cap and by `1 + length`, and a high mark bid
bounded by `1 + length`. -/
private def MarkInv (cfg : AuctionConfig) (a : AuctionState) : Prop :=
  a.bids.length ≤ 4
    ∧ (∀ m : ℕ, Bid.mark m ∈ a.bids → m ≤ cfg.maxMarkBid ∧ m ≤ 1 + a.bids.length)
    ∧ (∀ r : ℕ, a.highBid = some (.mark r) → r ≤ 1 + a.bids.length)

private theorem markInv_of_legal {cfg : AuctionConfig} {a : AuctionState}
    (h : LegalAuction cfg a) : MarkInv cfg a := by
  induction h with
  | init shaker =>
      refine ⟨by simp, by simp, ?_⟩
      intro r hr
      simp [highBid] at hr
  | @step a b ha hb ih =>
      obtain ⟨hlen, hmarks, hhigh⟩ := ih
      have hlen' : (a.step b).bids.length = a.bids.length + 1 := by
        simp [step]
      have hlt : a.bids.length < 4 := by
        cases b with
        | pass => exact hb
        | point n => exact hb.1
        | mark m' => exact hb.1
      -- a legal mark bid is bounded by the cap and by `2 + length`
      have hmark_new : ∀ m' : ℕ, b = .mark m' →
          m' ≤ cfg.maxMarkBid ∧ m' ≤ 2 + a.bids.length := by
        intro m' hbm
        subst hbm
        obtain ⟨-, -, hcap, hentry⟩ := hb
        refine ⟨hcap, ?_⟩
        unfold MarkEntry at hentry
        cases hhb : a.highBid with
        | none => rw [hhb] at hentry; omega
        | some hbid =>
            cases hbid with
            | pass => rw [hhb] at hentry; omega
            | point n => rw [hhb] at hentry; omega
            | mark r =>
                rw [hhb] at hentry
                have := hhigh r hhb
                omega
      refine ⟨?_, ?_, ?_⟩
      · rw [hlen']; omega
      · intro m hm
        rw [hlen']
        simp only [step, List.mem_append, List.mem_singleton] at hm
        rcases hm with hm | hm
        · exact ⟨(hmarks m hm).1, (hmarks m hm).2.trans (by omega)⟩
        · obtain ⟨h₁, h₂⟩ := hmark_new m hm.symm
          exact ⟨h₁, by omega⟩
      · intro r hr
        rw [highBid_step] at hr
        rw [hlen']
        cases b with
        | pass => exact (hhigh r hr).trans (by omega)
        | point n => simp at hr
        | mark m' =>
            simp only [Option.some.injEq, Bid.mark.injEq] at hr
            obtain ⟨-, h₂⟩ := hmark_new m' rfl
            omega

/-- PA-B03 (structural reachable mark ceiling, Math §4.3): on any legal
auction state, every mark bid is at most `min(maxMarkBid, 5)`. A first mark
bid is at most two and each overcall adds one across at most three later
actors. -/
theorem mark_le_ceiling {cfg : AuctionConfig} {a : AuctionState}
    (h : LegalAuction cfg a) {m : ℕ} (hm : Bid.mark m ∈ a.bids) :
    m ≤ min cfg.maxMarkBid 5 := by
  obtain ⟨hlen, hmarks, -⟩ := markInv_of_legal h
  have := hmarks m hm
  omega

/-- PA-B03 (ceiling reached): whenever the cap permits, the chain
`M(2), M(3), M(4), M(5)` is a legal auction reaching mark five. -/
theorem mark_five_reachable {cfg : AuctionConfig} (hcap : 5 ≤ cfg.maxMarkBid) :
    ∃ a : AuctionState, LegalAuction cfg a ∧ Bid.mark 5 ∈ a.bids := by
  refine ⟨⟨0, [.mark 2, .mark 3, .mark 4, .mark 5]⟩, ?_, by simp⟩
  have h0 : LegalAuction cfg ⟨0, []⟩ := .init 0
  have h1 : LegalAuction cfg ⟨0, [.mark 2]⟩ :=
    .step h0 ⟨by simp, by omega, by omega, Or.inr rfl⟩
  have h2 : LegalAuction cfg ⟨0, [.mark 2, .mark 3]⟩ :=
    .step h1 ⟨by simp, by omega, by omega, rfl⟩
  have h3 : LegalAuction cfg ⟨0, [.mark 2, .mark 3, .mark 4]⟩ :=
    .step h2 ⟨by simp, by omega, by omega, rfl⟩
  exact .step h3 ⟨by simp, by omega, by omega, rfl⟩

end AuctionState

end Texas42
