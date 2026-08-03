/-
Copyright (c) 2026 Jason Yandell. All rights reserved.
Released under Apache 2.0 license as described in the file LICENSE.
Authors: Jason Yandell
-/
import Texas42.Basic

/-!
# Texas 42 — complete deal worlds

PA-B05 (define): the ordered deal — four labeled seven-domino hands
partitioning the 28 dominoes (Math §4.1). The deal-count theorems
(`N_deal = 472,518,347,558,400`; `21!/(7!)³` conditional) are separate
PROVE targets, not yet formalized.
-/

namespace Texas42

/-- PA-B05: a complete deal world for one deal attempt — an ordered
partition of the 28 dominoes into four labeled seven-domino hands
(Math §4.1). Disjointness plus the card count force the union to be
everything (`Deal.biUnion_eq_univ`). -/
structure Deal where
  hands : Seat → Finset Domino
  card_hands : ∀ s, (hands s).card = 7
  disjoint : ∀ s t : Seat, s ≠ t → Disjoint (hands s) (hands t)

namespace Deal

/-- The four hands exhaust the domino set. -/
theorem biUnion_eq_univ (ω : Deal) :
    Finset.univ.biUnion ω.hands = Finset.univ := by
  apply Finset.eq_univ_of_card
  rw [Finset.card_biUnion (fun s _ t _ hst => ω.disjoint s t hst)]
  simp [ω.card_hands, Domino.card_domino]

/-- Every domino sits in exactly one hand. -/
theorem existsUnique_mem (ω : Deal) (d : Domino) :
    ∃! s : Seat, d ∈ ω.hands s := by
  have hd : d ∈ Finset.univ.biUnion ω.hands := by
    rw [ω.biUnion_eq_univ]; exact Finset.mem_univ d
  rw [Finset.mem_biUnion] at hd
  obtain ⟨s, -, hs⟩ := hd
  refine ⟨s, hs, fun t ht => ?_⟩
  by_contra hts
  exact Finset.disjoint_left.mp (ω.disjoint t s hts) ht hs

/-- The owner of a domino — the unique seat holding it (computable via
finite choice). -/
def owner (ω : Deal) (d : Domino) : Seat :=
  Fintype.choose _ (ω.existsUnique_mem d)

theorem mem_owner (ω : Deal) (d : Domino) : d ∈ ω.hands (ω.owner d) :=
  Fintype.choose_spec _ (ω.existsUnique_mem d)

theorem owner_eq (ω : Deal) {d : Domino} {s : Seat} (h : d ∈ ω.hands s) :
    ω.owner d = s :=
  ((ω.existsUnique_mem d).unique (ω.mem_owner d) h)

/-- Deals are determined by their hands. -/
theorem ext {ω₁ ω₂ : Deal} (h : ω₁.hands = ω₂.hands) : ω₁ = ω₂ := by
  cases ω₁ with
  | mk f hc hd =>
      cases ω₂ with
      | mk g hc' hd' =>
          dsimp only at h
          subst h
          rfl

end Deal

end Texas42
