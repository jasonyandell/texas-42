/-
Copyright (c) 2026 Jason Yandell. All rights reserved.
Released under Apache 2.0 license as described in the file LICENSE.
Authors: Jason Yandell
-/
import Texas42.Reduction

/-!
# Texas 42 — the exact support normal form

Math §7.10 (with the §7.11 linear validator), PA-D01–D05: over the
generic capacitated cell kernel of `Reduction.lean`, the marginal holder
relation of a feasible system splits into certain hidden-location marks
`K_s` and a delimited ambiguity component; with three holders the active
seats obey the `{0,2,3}` trichotomy (PA-D01). The validated normal form
`SupportNF` (PA-D02, handoff §6 "indexed or validated type") admits a
decoder reconstructing the reduced holder relation (PA-D03), compile and
decode are mutually inverse (PA-D04), and the `Empty`-tagged total form
classifies exact fibers over *all* systems, feasible or not (PA-D05).

No solver is imported: feasibility of well-formed normal forms is proved
from the §7.11 strict singleton Hall inequalities through one generic
capacitated Hall lemma (also the PA-C08 groundwork), whose matching side
is mathlib's `Finset.all_card_le_biUnion_card_iff_exists_injective`.
-/

namespace Texas42

namespace CellSys

variable {H α : Type} [Fintype H] [DecidableEq H] [Fintype α]
  [DecidableEq α]

/-! ## Feasibility and the marginal anatomy (Math §7.10) -/

/-- A system is feasible when its fiber `Φ(C)` is nonempty. -/
def Feasible (C : CellSys H α) : Prop := ∃ A, C.IsWorld A

instance (C : CellSys H α) : Decidable C.Feasible :=
  inferInstanceAs (Decidable (∃ A, C.IsWorld A))

/-- The exact holder set `A(d) = {s : d ∈ P_s⋆}` (Math §7.10). -/
def holders (C : CellSys H α) (d : α) : Finset H :=
  Finset.univ.filter fun s => d ∈ C.marginal s

theorem mem_holders {C : CellSys H α} {d : α} {s : H} :
    s ∈ C.holders d ↔ d ∈ C.marginal s := by
  unfold holders
  simp

/-- The certain hidden-location marks `K_s = {d : A(d) = {s}}`. -/
def certain (C : CellSys H α) (s : H) : Finset α :=
  C.pool.filter fun d => C.holders d = {s}

/-- The ambiguous pool `W = U \ ⨆_s K_s`. -/
def amb (C : CellSys H α) : Finset α :=
  C.pool \ Finset.univ.biUnion C.certain

/-- Residual capacities `r_s = k_s - |K_s|`. -/
def resid (C : CellSys H α) (s : H) : ℕ := C.cap s - (C.certain s).card

/-- The active seats `J⁺ = {s : r_s > 0}`. -/
def active (C : CellSys H α) : Finset H :=
  Finset.univ.filter fun s => 0 < C.resid s

theorem mem_active {C : CellSys H α} {s : H} :
    s ∈ C.active ↔ 0 < C.resid s := by
  unfold active
  simp

/-- Certain tiles sit in the pool with singleton holder set. -/
theorem mem_certain {C : CellSys H α} {d : α} {s : H} :
    d ∈ C.certain s ↔ d ∈ C.pool ∧ C.holders d = {s} := by
  unfold certain
  simp

/-- A world's hands stay inside the marginals. -/
theorem mem_marginal_of_world {C : CellSys H α} {A : H → Finset α}
    (hA : C.IsWorld A) {s : H} {d : α} (hd : d ∈ A s) :
    d ∈ C.marginal s := by
  unfold marginal
  rw [Finset.mem_filter]
  exact ⟨(hA.1 s).1 hd, A, hA, hd⟩

/-- Every pool tile of a world is held somewhere. -/
theorem exists_seat_of_world {C : CellSys H α} {A : H → Finset α}
    (hA : C.IsWorld A) {d : α} (hd : d ∈ C.pool) : ∃ s, d ∈ A s := by
  have : d ∈ Finset.univ.biUnion A := by rw [hA.2.2]; exact hd
  rw [Finset.mem_biUnion] at this
  obtain ⟨s, -, hs⟩ := this
  exact ⟨s, hs⟩

/-- In a feasible system every pool tile has a nonempty holder set. -/
theorem holders_nonempty {C : CellSys H α} (hC : C.Feasible) {d : α}
    (hd : d ∈ C.pool) : (C.holders d).Nonempty := by
  obtain ⟨A, hA⟩ := hC
  obtain ⟨s, hs⟩ := exists_seat_of_world hA hd
  exact ⟨s, mem_holders.mpr (mem_marginal_of_world hA hs)⟩

/-- The certain marks are pinned: every world assigns `K_s ⊆ A s`. -/
theorem certain_subset_world {C : CellSys H α} {A : H → Finset α}
    (hA : C.IsWorld A) (s : H) : C.certain s ⊆ A s := by
  intro d hd
  rw [mem_certain] at hd
  obtain ⟨t, ht⟩ := exists_seat_of_world hA hd.1
  have : t ∈ C.holders d := mem_holders.mpr (mem_marginal_of_world hA ht)
  rw [hd.2, Finset.mem_singleton] at this
  rwa [this] at ht

/-- In a feasible system `|K_s| ≤ k_s`. -/
theorem card_certain_le {C : CellSys H α} (hC : C.Feasible) (s : H) :
    (C.certain s).card ≤ C.cap s := by
  obtain ⟨A, hA⟩ := hC
  calc (C.certain s).card ≤ (A s).card :=
        Finset.card_le_card (certain_subset_world hA s)
    _ = C.cap s := (hA.1 s).2

theorem certain_disjoint (C : CellSys H α) {s t : H} (hst : s ≠ t) :
    Disjoint (C.certain s) (C.certain t) := by
  rw [Finset.disjoint_left]
  intro d hs ht
  rw [mem_certain] at hs ht
  have := hs.2.symm.trans ht.2
  rw [Finset.singleton_inj] at this
  exact hst this

theorem certain_subset_pool (C : CellSys H α) (s : H) :
    C.certain s ⊆ C.pool := Finset.filter_subset _ _

theorem amb_subset_pool (C : CellSys H α) : C.amb ⊆ C.pool :=
  Finset.sdiff_subset

theorem amb_disjoint_certain (C : CellSys H α) (s : H) :
    Disjoint C.amb (C.certain s) := by
  rw [Finset.disjoint_left]
  intro d hd hds
  unfold amb at hd
  rw [Finset.mem_sdiff] at hd
  exact hd.2 (Finset.mem_biUnion.mpr ⟨s, Finset.mem_univ s, hds⟩)

/-- Pool tiles split: certain somewhere, or ambiguous. -/
theorem mem_amb_or_certain {C : CellSys H α} {d : α} (hd : d ∈ C.pool) :
    d ∈ C.amb ∨ ∃ s, d ∈ C.certain s := by
  by_cases h : d ∈ Finset.univ.biUnion C.certain
  · rw [Finset.mem_biUnion] at h
    obtain ⟨s, -, hs⟩ := h
    exact Or.inr ⟨s, hs⟩
  · exact Or.inl (by unfold amb; rw [Finset.mem_sdiff]; exact ⟨hd, h⟩)

/-- In a feasible system an ambiguous tile has at least two holders. -/
theorem two_le_card_holders {C : CellSys H α} (hC : C.Feasible) {d : α}
    (hd : d ∈ C.amb) : 2 ≤ (C.holders d).card := by
  have hpool : d ∈ C.pool := C.amb_subset_pool hd
  obtain ⟨s, hs⟩ := holders_nonempty hC hpool
  by_contra hlt
  rw [not_le] at hlt
  have hsingle : C.holders d = {s} := by
    apply Finset.eq_singleton_iff_unique_mem.mpr
    refine ⟨hs, fun t ht => ?_⟩
    by_contra hts
    have : 2 ≤ (C.holders d).card := by
      calc 2 = ({t, s} : Finset H).card := by
            rw [Finset.card_insert_of_notMem (by simpa using hts),
              Finset.card_singleton]
        _ ≤ (C.holders d).card := by
            apply Finset.card_le_card
            intro u hu
            rw [Finset.mem_insert, Finset.mem_singleton] at hu
            rcases hu with rfl | rfl
            · exact ht
            · exact hs
    omega
  have : d ∈ C.certain s := mem_certain.mpr ⟨hpool, hsingle⟩
  exact Finset.disjoint_left.mp (C.amb_disjoint_certain s) hd this

/-- Conservation: in a feasible system the quotas sum to the pool. -/
theorem sum_cap_eq_card_pool {C : CellSys H α} (hC : C.Feasible) :
    ∑ s, C.cap s = C.pool.card := by
  obtain ⟨A, hA⟩ := hC
  calc ∑ s, C.cap s = ∑ s, (A s).card :=
        Finset.sum_congr rfl fun s _ => ((hA.1 s).2).symm
    _ = (Finset.univ.biUnion A).card :=
        (Finset.card_biUnion fun s _ t _ hst => hA.2.1 s t hst).symm
    _ = C.pool.card := by rw [hA.2.2]

/-- An ambiguous tile's holders are active seats (Math §7.10: a
zero-residual seat cannot hold an ambiguous domino). -/
theorem holders_subset_active {C : CellSys H α} {d : α}
    (hd : d ∈ C.amb) : C.holders d ⊆ C.active := by
  intro s hs
  rw [mem_holders] at hs
  unfold marginal at hs
  rw [Finset.mem_filter] at hs
  obtain ⟨-, A, hA, hdA⟩ := hs
  have hins : insert d (C.certain s) ⊆ A s := by
    intro x hx
    rw [Finset.mem_insert] at hx
    rcases hx with rfl | hx
    · exact hdA
    · exact certain_subset_world hA s hx
  have hdK : d ∉ C.certain s :=
    Finset.disjoint_left.mp (C.amb_disjoint_certain s) hd
  have hcard : (C.certain s).card + 1 ≤ C.cap s := by
    calc (C.certain s).card + 1 = (insert d (C.certain s)).card :=
          (Finset.card_insert_of_notMem hdK).symm
      _ ≤ (A s).card := Finset.card_le_card hins
      _ = C.cap s := (hA.1 s).2
  rw [mem_active]
  unfold resid
  omega

/-- A world's ambiguous share at `s`: `A s \ K_s` has `r_s` elements,
all ambiguous and marginally held by `s`. -/
theorem world_amb_share {C : CellSys H α} {A : H → Finset α}
    (hA : C.IsWorld A) (s : H) :
    A s \ C.certain s ⊆ C.amb.filter (fun d => d ∈ C.marginal s)
      ∧ (A s \ C.certain s).card = C.resid s := by
  constructor
  · intro d hd
    rw [Finset.mem_sdiff] at hd
    obtain ⟨hdA, hdK⟩ := hd
    have hpool : d ∈ C.pool := by
      rw [← hA.2.2]
      exact Finset.mem_biUnion.mpr ⟨s, Finset.mem_univ s, hdA⟩
    have hamb : d ∈ C.amb := by
      rcases mem_amb_or_certain hpool with h | ⟨t, ht⟩
      · exact h
      · rcases eq_or_ne t s with rfl | hts
        · exact absurd ht hdK
        · have hdt : d ∈ A t := certain_subset_world hA t ht
          exact (Finset.disjoint_left.mp (hA.2.1 t s hts) hdt hdA).elim
    rw [Finset.mem_filter]
    exact ⟨hamb, mem_marginal_of_world hA hdA⟩
  · rw [Finset.card_sdiff_of_subset (certain_subset_world hA s),
      (hA.1 s).2]
    rfl

/-- Math §7.11 (strict singleton Hall): in a feasible system every
active seat's ambiguous marginal neighborhood strictly exceeds its
residual capacity — `|N({s})| ≥ r_s + 1`. -/
theorem strict_singleton_hall {C : CellSys H α} (hC : C.Feasible) {s : H}
    (hs : s ∈ C.active) :
    C.resid s + 1 ≤ (C.amb.filter (fun d => d ∈ C.marginal s)).card := by
  obtain ⟨A, hA⟩ := hC
  set N := C.amb.filter (fun d => d ∈ C.marginal s) with hN
  have hle : C.resid s ≤ N.card := by
    rw [← (world_amb_share hA s).2]
    exact Finset.card_le_card (world_amb_share hA s).1
  rcases Nat.lt_or_ge N.card (C.resid s + 1) with hlt | hge
  · exfalso
    have heq : N.card = C.resid s := by omega
    -- equality forces every world to assign all of `N` to `s`
    have hall : ∀ B, C.IsWorld B → B s \ C.certain s = N := fun B hB =>
      Finset.eq_of_subset_of_card_le (world_amb_share hB s).1
        (by rw [heq, (world_amb_share hB s).2])
    -- `N` is nonempty since `s` is active
    have hNne : N.Nonempty := by
      rw [← Finset.card_pos, heq]
      exact mem_active.mp hs
    obtain ⟨d, hd⟩ := hNne
    have hdamb : d ∈ C.amb := (Finset.mem_filter.mp hd).1
    -- an ambiguous tile has a second holder `t ≠ s`
    have hsh : s ∈ C.holders d :=
      mem_holders.mpr (Finset.mem_filter.mp hd).2
    obtain ⟨t, ht, hts⟩ :=
      Finset.exists_mem_ne
        (s := C.holders d)
        (by have := two_le_card_holders ⟨A, hA⟩ hdamb; omega) s
    have := mem_holders.mp ht
    unfold marginal at this
    rw [Finset.mem_filter] at this
    obtain ⟨-, B, hB, hdB⟩ := this
    -- but every world pins `d` at `s`
    have hdBs : d ∈ B s := by
      have : d ∈ B s \ C.certain s := by rw [hall B hB]; exact hd
      exact (Finset.mem_sdiff.mp this).1
    exact Finset.disjoint_left.mp (hB.2.1 t s hts) hdB hdBs
  · exact hge

/-! ## PA-D01: the native active-seat trichotomy (Math §7.10) -/

/-- Determinacy: the ambiguous pool is empty iff no seat is active. -/
theorem amb_empty_iff_active_empty {C : CellSys H α} (hC : C.Feasible) :
    C.amb = ∅ ↔ C.active = ∅ := by
  have hsubK : Finset.univ.biUnion C.certain ⊆ C.pool := by
    intro d hd
    rw [Finset.mem_biUnion] at hd
    obtain ⟨s, -, hs⟩ := hd
    exact C.certain_subset_pool s hs
  have hcards : C.pool.card =
      (Finset.univ.biUnion C.certain).card + C.amb.card := by
    unfold amb
    rw [Finset.card_sdiff_of_subset hsubK]
    have := Finset.card_le_card hsubK
    omega
  have hKsum : (Finset.univ.biUnion C.certain).card =
      ∑ s, (C.certain s).card :=
    Finset.card_biUnion fun s _ t _ hst => C.certain_disjoint hst
  have hsum := sum_cap_eq_card_pool hC
  constructor
  · intro hamb
    rw [Finset.eq_empty_iff_forall_notMem]
    intro s hs
    have hlt := mem_active.mp hs
    unfold resid at hlt
    have hbound : ∀ t, (C.certain t).card ≤ C.cap t := card_certain_le hC
    have : ∑ t, (C.certain t).card < ∑ t, C.cap t :=
      Finset.sum_lt_sum (fun t _ => hbound t)
        ⟨s, Finset.mem_univ s, by omega⟩
    rw [hamb] at hcards
    simp at hcards
    omega
  · intro hact
    have hzero : ∀ s, C.resid s = 0 := by
      intro s
      by_contra h
      have : s ∈ C.active := mem_active.mpr (by omega)
      rw [hact] at this
      exact absurd this (Finset.notMem_empty s)
    have hge : ∀ s, C.cap s ≤ (C.certain s).card := by
      intro s
      have := hzero s
      unfold resid at this
      omega
    have : ∑ s, C.cap s ≤ ∑ s, (C.certain s).card :=
      Finset.sum_le_sum fun s _ => hge s
    rw [← Finset.card_eq_zero]
    omega

/-- One active seat is impossible in a feasible system. -/
theorem active_card_ne_one {C : CellSys H α} (hC : C.Feasible) :
    C.active.card ≠ 1 := by
  intro h1
  obtain ⟨s, hs⟩ := Finset.card_eq_one.mp h1
  have hne : C.amb ≠ ∅ := by
    intro hamb
    rw [(amb_empty_iff_active_empty hC).mp hamb] at hs
    exact absurd hs.symm (Finset.singleton_ne_empty s)
  obtain ⟨d, hd⟩ := Finset.nonempty_iff_ne_empty.mpr hne
  have hsub : C.holders d ⊆ C.active := holders_subset_active hd
  have : (C.holders d).card ≤ 1 := by
    rw [← h1]
    exact Finset.card_le_card hsub
  have := two_le_card_holders hC hd
  omega

/-- PA-D01 (Math §7.10 trichotomy): with three holders, the active-seat
count is 0, 2, or 3 — and it is 0 exactly when nothing is ambiguous. -/
theorem active_trichotomy {C : CellSys H α} (hH : Fintype.card H = 3)
    (hC : C.Feasible) :
    C.active.card = 0 ∨ C.active.card = 2 ∨ C.active.card = 3 := by
  have hle : C.active.card ≤ 3 := by
    rw [← hH]
    exact Finset.card_le_univ _
  have := active_card_ne_one hC
  omega

/-- PA-D01 (binary pattern): with two active seats every ambiguous tile
is possible at exactly both. -/
theorem holders_eq_active_of_two {C : CellSys H α} (hC : C.Feasible)
    (h2 : C.active.card = 2) {d : α} (hd : d ∈ C.amb) :
    C.holders d = C.active :=
  Finset.eq_of_subset_of_card_le (holders_subset_active hd)
    (by have := two_le_card_holders hC hd; omega)

/-- PA-D01 (ternary pattern): with three active seats every ambiguous
tile is possible everywhere or excludes exactly one seat. -/
theorem excl_card_le_one_of_three {C : CellSys H α} (hC : C.Feasible)
    (h3 : C.active.card = 3) {d : α} (hd : d ∈ C.amb) :
    (C.active \ C.holders d).card ≤ 1 := by
  have hsub := holders_subset_active (C := C) hd
  have h2 := two_le_card_holders hC hd
  rw [Finset.card_sdiff_of_subset hsub]
  omega

/-! ## The generic capacitated Hall lemma (PA-C08 groundwork)

Quota-respecting exact partitions exist whenever the grouped Hall
condition holds; proved by slot expansion `(s : H) × Fin (r s)` into
mathlib's `Finset.all_card_le_biUnion_card_iff_exists_injective`. -/

omit [Fintype α] in
theorem exists_partition_of_hall (W : Finset α) (r : H → ℕ)
    (allowed : H → Finset α) (hsub : ∀ s, allowed s ⊆ W)
    (hsum : ∑ s, r s = W.card)
    (hhall : ∀ R : Finset H, ∑ s ∈ R, r s ≤ (R.biUnion allowed).card) :
    ∃ A : H → Finset α, (∀ s, A s ⊆ allowed s ∧ (A s).card = r s)
      ∧ (∀ s t : H, s ≠ t → Disjoint (A s) (A t))
      ∧ Finset.univ.biUnion A = W := by
  -- slot expansion: one vertex per unit of quota
  have hall : ∀ S : Finset ((s : H) × Fin (r s)),
      S.card ≤ (S.biUnion fun p => allowed p.1).card := by
    intro S
    set R := S.image Sigma.fst with hR
    have hunion : S.biUnion (fun p => allowed p.1) = R.biUnion allowed := by
      ext d
      rw [Finset.mem_biUnion, Finset.mem_biUnion]
      constructor
      · rintro ⟨p, hp, hd⟩
        exact ⟨p.1, Finset.mem_image_of_mem Sigma.fst hp, hd⟩
      · rintro ⟨s, hs, hd⟩
        rw [hR, Finset.mem_image] at hs
        obtain ⟨p, hp, rfl⟩ := hs
        exact ⟨p, hp, hd⟩
    have hcard : S.card ≤ ∑ s ∈ R, r s := by
      have hstep : S.card ≤
          (R.sigma fun s => (Finset.univ : Finset (Fin (r s)))).card := by
        apply Finset.card_le_card
        intro p hp
        rw [Finset.mem_sigma]
        exact ⟨Finset.mem_image_of_mem Sigma.fst hp, Finset.mem_univ _⟩
      have hsig : (R.sigma fun s => (Finset.univ : Finset (Fin (r s)))).card
          = ∑ s ∈ R, r s := by
        rw [Finset.card_sigma]
        exact Finset.sum_congr rfl fun s _ => by
          rw [Finset.card_univ, Fintype.card_fin]
      omega
    rw [hunion]
    exact hcard.trans (hhall R)
  obtain ⟨f, hfinj, hfmem⟩ :=
    (Finset.all_card_le_biUnion_card_iff_exists_injective
      (fun p : (s : H) × Fin (r s) => allowed p.1)).mp hall
  refine ⟨fun s => Finset.univ.image fun i : Fin (r s) => f ⟨s, i⟩,
    fun s => ⟨?_, ?_⟩, ?_, ?_⟩
  · intro d hd
    rw [Finset.mem_image] at hd
    obtain ⟨i, -, rfl⟩ := hd
    exact hfmem ⟨s, i⟩
  · rw [Finset.card_image_of_injective _
      (fun i j hij => by
        have := hfinj hij
        rw [Sigma.mk.injEq] at this
        exact eq_of_heq this.2),
      Finset.card_univ, Fintype.card_fin]
  · intro s t hst
    rw [Finset.disjoint_left]
    intro d hds hdt
    rw [Finset.mem_image] at hds hdt
    obtain ⟨i, -, hi⟩ := hds
    obtain ⟨j, -, hj⟩ := hdt
    have := hfinj (hi.trans hj.symm)
    rw [Sigma.mk.injEq] at this
    exact hst this.1
  · apply Finset.eq_of_subset_of_card_le
    · intro d hd
      rw [Finset.mem_biUnion] at hd
      obtain ⟨s, -, hd⟩ := hd
      rw [Finset.mem_image] at hd
      obtain ⟨i, -, rfl⟩ := hd
      exact hsub s (hfmem ⟨s, i⟩)
    · rw [Finset.card_biUnion, ← hsum]
      · apply le_of_eq
        apply Finset.sum_congr rfl
        intro s _
        rw [Finset.card_image_of_injective _
          (fun i j hij => by
            have := hfinj hij
            rw [Sigma.mk.injEq] at this
            exact eq_of_heq this.2),
          Finset.card_univ, Fintype.card_fin]
      · intro s _ t _ hst
        show Disjoint _ _
        rw [Finset.disjoint_left]
        intro d hds hdt
        rw [Finset.mem_image] at hds hdt
        obtain ⟨i, -, hi⟩ := hds
        obtain ⟨j, -, hj⟩ := hdt
        have := hfinj (hi.trans hj.symm)
        rw [Sigma.mk.injEq] at this
        exact hst this.1

end CellSys

/-! ## PA-D02: the validated support normal form (Math §7.10, handoff §6)

The handoff sanctions "an indexed or validated type"; this is the
validated form — a record of certain marks, ambiguous pool, residual
capacities, and the sparse exclusion function, with `WellFormed`
carrying the branch invariants including the §7.11 linear ternary
validator. -/

/-- The feasible exact support normal form `𝒩 = ((K_s)_s, 𝒦_amb)`:
certain hidden-location marks, the ambiguous pool `W`, residual
capacities `r_s`, and the sparse exclusion `ε : W ⇀ H` (`none` = the
tile is possible at every active seat). -/
structure SupportNF (H α : Type) [Fintype H] [DecidableEq H]
    [Fintype α] [DecidableEq α] where
  certain : H → Finset α
  amb : Finset α
  resid : H → ℕ
  excl : α → Option H

namespace SupportNF

variable {H α : Type} [Fintype H] [DecidableEq H] [Fintype α]
  [DecidableEq α]

/-- The active seats of a normal form. -/
def active (N : SupportNF H α) : Finset H :=
  Finset.univ.filter fun s => 0 < N.resid s

theorem mem_active {N : SupportNF H α} {s : H} :
    s ∈ N.active ↔ 0 < N.resid s := by
  unfold active
  simp

/-- The declared possible ambiguous tiles of a seat. -/
def ambAllowed (N : SupportNF H α) (s : H) : Finset α :=
  if 0 < N.resid s then N.amb.filter (fun d => N.excl d ≠ some s) else ∅

theorem mem_ambAllowed {N : SupportNF H α} {s : H} {d : α} :
    d ∈ N.ambAllowed s
      ↔ 0 < N.resid s ∧ d ∈ N.amb ∧ N.excl d ≠ some s := by
  unfold ambAllowed
  split_ifs with h
  · rw [Finset.mem_filter]
    tauto
  · simp [h]

theorem ambAllowed_subset_amb (N : SupportNF H α) (s : H) :
    N.ambAllowed s ⊆ N.amb := fun _ hd =>
  (mem_ambAllowed.mp hd).2.1

/-- PA-D03 (decoder): the cell system a normal form presents — pool,
allowed sets, and quotas reconstructed by complement and conservation
(Math §7.10 decode). -/
def decode (N : SupportNF H α) : CellSys H α where
  pool := Finset.univ.biUnion N.certain ∪ N.amb
  allowed := fun s => N.certain s ∪ N.ambAllowed s
  cap := fun s => (N.certain s).card + N.resid s

/-- PA-D02: well-formedness — the handoff §6 common and branch
invariants; the ternary branch carries the §7.11 linear validator
`r_s + n_s + 1 ≤ |W|`. -/
structure WellFormed (N : SupportNF H α) : Prop where
  certain_disj : ∀ s t : H, s ≠ t → Disjoint (N.certain s) (N.certain t)
  amb_disj : ∀ s, Disjoint N.amb (N.certain s)
  resid_sum : ∑ s, N.resid s = N.amb.card
  excl_mem : ∀ d s, N.excl d = some s → d ∈ N.amb ∧ 0 < N.resid s
  branch : N.active = ∅
    ∨ (N.active.card = 2 ∧ ∀ d, N.excl d = none)
    ∨ (N.active.card = 3 ∧ ∀ s ∈ N.active,
        N.resid s + (N.amb.filter fun d => N.excl d = some s).card + 1
          ≤ N.amb.card)

/-- Determinacy: no active seat forces an empty ambiguous pool. -/
theorem amb_eq_empty {N : SupportNF H α} (hwf : N.WellFormed)
    (h : N.active = ∅) : N.amb = ∅ := by
  have : ∀ s, N.resid s = 0 := by
    intro s
    by_contra hs
    have : s ∈ N.active := mem_active.mpr (by omega)
    rw [h] at this
    exact absurd this (Finset.notMem_empty s)
  have hsum := hwf.resid_sum
  rw [Finset.sum_congr rfl fun s _ => this s] at hsum
  simp at hsum
  rw [← Finset.card_eq_zero]
  omega

section ForcedPartition

variable {N : SupportNF H α} {d₀ : α} {s₀ : H}

/-- The primed quotas: one unit of `s₀`'s residual is spent on `d₀`. -/
private def resid' (N : SupportNF H α) (s₀ : H) (s : H) : ℕ :=
  if s = s₀ then N.resid s - 1 else N.resid s

/-- The primed allowed sets: `d₀` is claimed. -/
private def ambAllowed' (N : SupportNF H α) (d₀ : α) (s : H) : Finset α :=
  (N.ambAllowed s).erase d₀

private theorem resid'_le (s : H) : resid' N s₀ s ≤ N.resid s := by
  unfold resid'
  split_ifs <;> omega

private theorem resid'_of_ne {s : H} (hs : s ≠ s₀) :
    resid' N s₀ s = N.resid s := if_neg hs

private theorem sum_resid' (hwf : N.WellFormed) (hd₀ : d₀ ∈ N.amb)
    (hs₀ : 0 < N.resid s₀) :
    ∑ s, resid' N s₀ s = (N.amb.erase d₀).card := by
  have hsplit :=
    Finset.sum_erase_add Finset.univ (resid' N s₀) (Finset.mem_univ s₀)
  have hsplit₀ :=
    Finset.sum_erase_add Finset.univ N.resid (Finset.mem_univ s₀)
  have hcongr : ∑ s ∈ Finset.univ.erase s₀, resid' N s₀ s
      = ∑ s ∈ Finset.univ.erase s₀, N.resid s := by
    apply Finset.sum_congr rfl
    intro s hs
    exact resid'_of_ne (Finset.mem_erase.mp hs).1
  have hr's₀ : resid' N s₀ s₀ = N.resid s₀ - 1 := if_pos rfl
  have hsum := hwf.resid_sum
  rw [Finset.card_erase_of_mem hd₀]
  omega

/-- Math §7.11 (validator sufficiency, with the essential-exclusion
forcing): a well-formed normal form realizes every declared ambiguous
edge — the ambiguous pool splits along the residual quotas with `d₀`
placed at the allowed active seat `s₀`. -/
theorem exists_forced_partition (hwf : N.WellFormed) (hd₀ : d₀ ∈ N.amb)
    (hs₀ : 0 < N.resid s₀) (hex : N.excl d₀ ≠ some s₀) :
    ∃ G : H → Finset α,
      (∀ s, G s ⊆ N.ambAllowed s ∧ (G s).card = N.resid s)
        ∧ (∀ s t : H, s ≠ t → Disjoint (G s) (G t))
        ∧ Finset.univ.biUnion G = N.amb ∧ d₀ ∈ G s₀ := by
  have hsuble : ∀ R : Finset H,
      ∑ s ∈ R, resid' N s₀ s ≤ ∑ s, resid' N s₀ s := fun R =>
    Finset.sum_le_sum_of_subset (Finset.subset_univ R)
  have hr'zero : ∀ s, N.resid s = 0 → resid' N s₀ s = 0 := by
    intro s hs
    have := resid'_le (N := N) (s₀ := s₀) s
    omega
  have hsum' := sum_resid' hwf hd₀ hs₀
  -- the grouped Hall condition for the primed system, by branch
  have hhall : ∀ R : Finset H, ∑ s ∈ R, resid' N s₀ s
      ≤ (R.biUnion (ambAllowed' N d₀)).card := by
    rcases hwf.branch with hdet | ⟨h2, hnone⟩ | ⟨h3, hval⟩
    · -- determinate: contradicts `d₀ ∈ W`
      exact absurd hd₀
        (by rw [amb_eq_empty hwf hdet]; exact Finset.notMem_empty d₀)
    · -- binary: any active seat's allowed set is the whole pool
      intro R
      by_cases hact : ∃ t ∈ R, 0 < N.resid t
      · obtain ⟨t, htR, htact⟩ := hact
        have hallt : N.ambAllowed t = N.amb := by
          unfold ambAllowed
          rw [if_pos htact]
          apply Finset.filter_true_of_mem
          intro d _
          rw [hnone d]
          simp
        have hcover : N.amb.erase d₀ ⊆ R.biUnion (ambAllowed' N d₀) := by
          intro d hd
          rw [Finset.mem_biUnion]
          refine ⟨t, htR, ?_⟩
          unfold ambAllowed'
          rw [hallt]
          exact hd
        calc ∑ s ∈ R, resid' N s₀ s ≤ ∑ s, resid' N s₀ s := hsuble R
          _ = (N.amb.erase d₀).card := hsum'
          _ ≤ _ := Finset.card_le_card hcover
      · have hzero : ∀ s ∈ R, resid' N s₀ s = 0 := fun s hs =>
          hr'zero s (by by_contra h; exact hact ⟨s, hs, by omega⟩)
        rw [Finset.sum_eq_zero hzero]
        exact Nat.zero_le _
    · -- ternary: two actives cover everything; a lone active seat is
      -- carried by the linear validator with its unit of slack
      intro R
      by_cases htwo : ∃ s ∈ R, ∃ t ∈ R,
          0 < N.resid s ∧ 0 < N.resid t ∧ s ≠ t
      · obtain ⟨s, hsR, t, htR, hsact, htact, hst⟩ := htwo
        have hcover : N.amb.erase d₀ ⊆ R.biUnion (ambAllowed' N d₀) := by
          intro d hd
          obtain ⟨hdne, hdamb⟩ := Finset.mem_erase.mp hd
          rw [Finset.mem_biUnion]
          by_cases hds : N.excl d = some s
          · refine ⟨t, htR, Finset.mem_erase.mpr
              ⟨hdne, mem_ambAllowed.mpr ⟨htact, hdamb, ?_⟩⟩⟩
            intro hdt
            rw [hds] at hdt
            exact hst (Option.some.inj hdt)
          · exact ⟨s, hsR, Finset.mem_erase.mpr
              ⟨hdne, mem_ambAllowed.mpr ⟨hsact, hdamb, hds⟩⟩⟩
        calc ∑ u ∈ R, resid' N s₀ u ≤ ∑ u, resid' N s₀ u := hsuble R
          _ = (N.amb.erase d₀).card := hsum'
          _ ≤ _ := Finset.card_le_card hcover
      · by_cases hone : ∃ s ∈ R, 0 < N.resid s
        · obtain ⟨s, hsR, hsact⟩ := hone
          have hsum_R : ∑ u ∈ R, resid' N s₀ u = resid' N s₀ s := by
            have hsplit := Finset.sum_erase_add R (resid' N s₀) hsR
            have hz : ∑ u ∈ R.erase s, resid' N s₀ u = 0 := by
              apply Finset.sum_eq_zero
              intro u hu
              obtain ⟨hus, huR⟩ := Finset.mem_erase.mp hu
              apply hr'zero
              by_contra h
              exact htwo ⟨u, huR, s, hsR, by omega, hsact, hus⟩
            omega
          have hval_s := hval s (mem_active.mpr hsact)
          have hAA : (N.ambAllowed s).card
              = N.amb.card
                - (N.amb.filter fun d => N.excl d = some s).card := by
            unfold ambAllowed
            rw [if_pos hsact]
            have hsplit := Finset.card_filter_add_card_filter_not
              (s := N.amb) (p := fun d => N.excl d = some s)
            have hne : (N.amb.filter fun d => N.excl d ≠ some s)
                = N.amb.filter fun d => ¬(N.excl d = some s) := rfl
            rw [hne]
            omega
          have herase : N.resid s ≤ ((N.ambAllowed s).erase d₀).card := by
            have hpred := Finset.pred_card_le_card_erase
              (s := N.ambAllowed s) (a := d₀)
            omega
          have hsubs : (N.ambAllowed s).erase d₀
              ⊆ R.biUnion (ambAllowed' N d₀) :=
            Finset.subset_biUnion_of_mem (ambAllowed' N d₀) hsR
          calc ∑ u ∈ R, resid' N s₀ u = resid' N s₀ s := hsum_R
            _ ≤ N.resid s := resid'_le s
            _ ≤ ((N.ambAllowed s).erase d₀).card := herase
            _ ≤ _ := Finset.card_le_card hsubs
        · have hzero : ∀ s ∈ R, resid' N s₀ s = 0 := fun s hs =>
            hr'zero s (by by_contra h; exact hone ⟨s, hs, by omega⟩)
          rw [Finset.sum_eq_zero hzero]
          exact Nat.zero_le _
  -- solve the primed system
  obtain ⟨G', hG'cell, hG'disj, hG'union⟩ :=
    CellSys.exists_partition_of_hall (N.amb.erase d₀) (resid' N s₀)
      (ambAllowed' N d₀)
      (fun s => Finset.erase_subset_erase d₀ (N.ambAllowed_subset_amb s))
      hsum' hhall
  have hG'd₀ : ∀ u, d₀ ∉ G' u := fun u hu =>
    (Finset.mem_erase.mp ((hG'cell u).1 hu)).1 rfl
  have hG'amb : ∀ u, G' u ⊆ N.ambAllowed u := fun u =>
    ((hG'cell u).1).trans (fun x hx => Finset.mem_of_mem_erase hx)
  -- ambiguity membership through the primed system
  have hG'mem : ∀ u d, d ∈ G' u → d ∈ N.amb := fun u d hd =>
    N.ambAllowed_subset_amb u (hG'amb u hd)
  -- reinstate `d₀` at `s₀`
  refine ⟨Function.update G' s₀ (insert d₀ (G' s₀)), ?_, ?_, ?_, ?_⟩
  · intro s
    rcases eq_or_ne s s₀ with hs | hs
    · rw [hs, Function.update_self]
      constructor
      · rw [Finset.insert_subset_iff]
        exact ⟨mem_ambAllowed.mpr ⟨hs₀, hd₀, hex⟩, hG'amb s₀⟩
      · rw [Finset.card_insert_of_notMem (hG'd₀ s₀)]
        have hc := (hG'cell s₀).2
        rw [show resid' N s₀ s₀ = N.resid s₀ - 1 from if_pos rfl] at hc
        omega
    · rw [Function.update_of_ne hs]
      exact ⟨hG'amb s, by
        have hc := (hG'cell s).2
        rwa [resid'_of_ne hs] at hc⟩
  · intro s t hst
    rcases eq_or_ne s s₀ with hs | hs
    · have htne : t ≠ s₀ := fun h => hst (hs.trans h.symm)
      rw [hs, Function.update_self, Function.update_of_ne htne,
        Finset.disjoint_left]
      intro d hd hdt
      rcases Finset.mem_insert.mp hd with rfl | hd'
      · exact hG'd₀ t hdt
      · exact Finset.disjoint_left.mp
          (hG'disj s₀ t fun h => hst (hs.trans h)) hd' hdt
    · rcases eq_or_ne t s₀ with ht | ht
      · rw [ht, Function.update_self, Function.update_of_ne hs,
          Finset.disjoint_left]
        intro d hd hdt
        rcases Finset.mem_insert.mp hdt with rfl | hd'
        · exact hG'd₀ s hd
        · exact Finset.disjoint_left.mp (hG'disj s s₀ hs) hd hd'
      · rw [Function.update_of_ne hs, Function.update_of_ne ht]
        exact hG'disj s t hst
  · ext d
    rw [Finset.mem_biUnion]
    constructor
    · rintro ⟨u, -, hd⟩
      rcases eq_or_ne u s₀ with hu | hu
      · rw [hu, Function.update_self] at hd
        rcases Finset.mem_insert.mp hd with rfl | hd'
        · exact hd₀
        · exact hG'mem s₀ d hd'
      · rw [Function.update_of_ne hu] at hd
        exact hG'mem u d hd
    · intro hd
      rcases eq_or_ne d d₀ with hdd | hdd
      · exact ⟨s₀, Finset.mem_univ _, by
          rw [hdd, Function.update_self]; exact Finset.mem_insert_self _ _⟩
      · have hmem : d ∈ Finset.univ.biUnion G' := by
          rw [hG'union]
          exact Finset.mem_erase.mpr ⟨hdd, hd⟩
        rw [Finset.mem_biUnion] at hmem
        obtain ⟨u, -, hu⟩ := hmem
        rcases eq_or_ne u s₀ with hus | hus
        · exact ⟨s₀, Finset.mem_univ _, by
            rw [Function.update_self]
            exact Finset.mem_insert_of_mem (hus ▸ hu)⟩
        · exact ⟨u, Finset.mem_univ _, by
            rw [Function.update_of_ne hus]; exact hu⟩
  · rw [Function.update_self]
    exact Finset.mem_insert_self _ _

end ForcedPartition

/-! ## Decode: worlds, feasibility, and exact marginals (PA-D03) -/

theorem decode_pool (N : SupportNF H α) :
    N.decode.pool = Finset.univ.biUnion N.certain ∪ N.amb := rfl

theorem decode_allowed (N : SupportNF H α) (s : H) :
    N.decode.allowed s = N.certain s ∪ N.ambAllowed s := rfl

theorem decode_cap (N : SupportNF H α) (s : H) :
    N.decode.cap s = (N.certain s).card + N.resid s := rfl

/-- An ambiguity partition assembles into a decode world. -/
theorem decode_isWorld_of_partition {N : SupportNF H α}
    (hwf : N.WellFormed) {G : H → Finset α}
    (hcell : ∀ s, G s ⊆ N.ambAllowed s ∧ (G s).card = N.resid s)
    (hdisj : ∀ s t : H, s ≠ t → Disjoint (G s) (G t))
    (hunion : Finset.univ.biUnion G = N.amb) :
    N.decode.IsWorld (fun s => N.certain s ∪ G s) := by
  have hGamb : ∀ s, G s ⊆ N.amb := fun s =>
    ((hcell s).1).trans (N.ambAllowed_subset_amb s)
  refine ⟨fun s => ⟨?_, ?_⟩, ?_, ?_⟩
  · rw [decode_allowed]
    exact Finset.union_subset_union subset_rfl (hcell s).1
  · rw [decode_cap,
      Finset.card_union_of_disjoint
        (Finset.disjoint_of_subset_right (hGamb s) (hwf.amb_disj s).symm),
      (hcell s).2]
  · intro s t hst
    rw [Finset.disjoint_union_left]
    constructor
    · rw [Finset.disjoint_union_right]
      exact ⟨hwf.certain_disj s t hst,
        Finset.disjoint_of_subset_right (hGamb t) (hwf.amb_disj s).symm⟩
    · rw [Finset.disjoint_union_right]
      exact ⟨Finset.disjoint_of_subset_left (hGamb s) (hwf.amb_disj t),
        hdisj s t hst⟩
  · rw [decode_pool, ← hunion]
    ext d
    rw [Finset.mem_biUnion, Finset.mem_union, Finset.mem_biUnion,
      Finset.mem_biUnion]
    constructor
    · rintro ⟨s, hs, hd⟩
      rcases Finset.mem_union.mp hd with h | h
      · exact Or.inl ⟨s, hs, h⟩
      · exact Or.inr ⟨s, hs, h⟩
    · rintro (⟨s, hs, hd⟩ | ⟨s, hs, hd⟩)
      · exact ⟨s, hs, Finset.mem_union_left _ hd⟩
      · exact ⟨s, hs, Finset.mem_union_right _ hd⟩

/-- Every declared edge — certain or ambiguous — occurs in a decode
world, with the certain marks pinned everywhere. -/
theorem exists_decode_world_forced {N : SupportNF H α} (hwf : N.WellFormed)
    {d₀ : α} {s₀ : H} (hd₀ : d₀ ∈ N.amb) (hs₀ : 0 < N.resid s₀)
    (hex : N.excl d₀ ≠ some s₀) :
    ∃ A, N.decode.IsWorld A ∧ d₀ ∈ A s₀ := by
  obtain ⟨G, hcell, hdisj, hunion, hmem⟩ :=
    exists_forced_partition hwf hd₀ hs₀ hex
  exact ⟨fun s => N.certain s ∪ G s,
    decode_isWorld_of_partition hwf hcell hdisj hunion,
    Finset.mem_union_right _ hmem⟩

/-- PA-D03 (feasibility): well-formed normal forms decode to nonempty
fibers. -/
theorem feasible_decode {N : SupportNF H α} (hwf : N.WellFormed) :
    N.decode.Feasible := by
  rcases Finset.eq_empty_or_nonempty N.amb with hamb | ⟨d₀, hd₀⟩
  · -- empty ambiguity: the certain assignment is the world
    have hres : ∀ s, N.resid s = 0 := by
      have hsum := hwf.resid_sum
      rw [hamb] at hsum
      simp only [Finset.card_empty] at hsum
      intro s
      exact (Finset.sum_eq_zero_iff.mp hsum) s (Finset.mem_univ s)
    refine ⟨fun s => N.certain s ∪ ∅,
      decode_isWorld_of_partition hwf
        (fun s => ⟨Finset.empty_subset _, by simp [hres s]⟩)
        (fun s t _ => by simp)
        ?_⟩
    rw [hamb]
    ext d
    simp
  · -- nonempty ambiguity: force any allowed edge of `d₀`
    have hact : ∃ s₀, 0 < N.resid s₀ ∧ N.excl d₀ ≠ some s₀ := by
      rcases hwf.branch with hdet | ⟨h2, hnone⟩ | ⟨h3, hval⟩
      · exact absurd hd₀
          (by rw [amb_eq_empty hwf hdet]; exact Finset.notMem_empty d₀)
      · have hne : N.active.Nonempty := by
          rw [← Finset.card_pos, h2]; omega
        obtain ⟨s₀, hs₀⟩ := hne
        exact ⟨s₀, mem_active.mp hs₀, by rw [hnone d₀]; simp⟩
      · rcases hexc : N.excl d₀ with _ | u
        · have hne : N.active.Nonempty := by
            rw [← Finset.card_pos, h3]; omega
          obtain ⟨s₀, hs₀⟩ := hne
          exact ⟨s₀, mem_active.mp hs₀, by simp⟩
        · obtain ⟨s₀, hs₀, hne⟩ :=
            Finset.exists_mem_ne (s := N.active) (by rw [h3]; omega) u
          refine ⟨s₀, mem_active.mp hs₀, fun h => ?_⟩
          exact hne (Option.some.inj h).symm
    obtain ⟨s₀, hs₀, hex⟩ := hact
    obtain ⟨A, hA, -⟩ := exists_decode_world_forced hwf hd₀ hs₀ hex
    exact ⟨A, hA⟩

/-- Certain marks are pinned in every decode world. -/
theorem certain_subset_decode_world {N : SupportNF H α}
    (hwf : N.WellFormed) {A : H → Finset α} (hA : N.decode.IsWorld A)
    (s : H) : N.certain s ⊆ A s := by
  intro d hd
  have hpool : d ∈ N.decode.pool := by
    rw [decode_pool]
    exact Finset.mem_union_left _
      (Finset.mem_biUnion.mpr ⟨s, Finset.mem_univ s, hd⟩)
  obtain ⟨t, ht⟩ := CellSys.exists_seat_of_world hA hpool
  rcases eq_or_ne t s with rfl | hts
  · exact ht
  · exfalso
    have hall : d ∈ N.decode.allowed t := (hA.1 t).1 ht
    rw [decode_allowed] at hall
    rcases Finset.mem_union.mp hall with h | h
    · exact Finset.disjoint_left.mp (hwf.certain_disj t s hts) h hd
    · exact Finset.disjoint_left.mp (hwf.amb_disj s)
        (N.ambAllowed_subset_amb t h) hd

/-- PA-D03 (reducedness): the decoded system's marginal holder relation
is exactly the declared one — no unsupported edges, nothing missing
(Math §7.10 decode / §7.11 essential exclusions). -/
theorem decode_marginal {N : SupportNF H α} (hwf : N.WellFormed) (s : H) :
    N.decode.marginal s = N.certain s ∪ N.ambAllowed s := by
  apply Finset.Subset.antisymm
  · rw [← decode_allowed]
    exact N.decode.marginal_subset s
  · intro d hd
    rcases Finset.mem_union.mp hd with h | h
    · obtain ⟨A, hA⟩ := feasible_decode hwf
      exact CellSys.mem_marginal_of_world hA
        (certain_subset_decode_world hwf hA s h)
    · obtain ⟨hs, hdamb, hex⟩ := mem_ambAllowed.mp h
      obtain ⟨A, hA, hmem⟩ := exists_decode_world_forced hwf hdamb hs hex
      exact CellSys.mem_marginal_of_world hA hmem

end SupportNF

/-! ## Compile: the normal form of a feasible system (PA-D02/D03) -/

namespace CellSys

variable {H α : Type} [Fintype H] [DecidableEq H] [Fintype α]
  [DecidableEq α]

/-- Structure extensionality for cell systems. -/
theorem cellSys_ext {C Q : CellSys H α} (hpool : C.pool = Q.pool)
    (hallowed : C.allowed = Q.allowed) (hcap : C.cap = Q.cap) : C = Q := by
  cases C
  cases Q
  dsimp only at hpool hallowed hcap
  subst hpool hallowed hcap
  rfl

/-- Certain tiles are marginally held by their seat. -/
theorem mem_marginal_of_certain {C : CellSys H α} {d : α} {s : H}
    (hd : d ∈ C.certain s) : d ∈ C.marginal s := by
  rw [← mem_holders, (mem_certain.mp hd).2]
  exact Finset.mem_singleton_self s

theorem marginal_subset_pool (C : CellSys H α) (s : H) :
    C.marginal s ⊆ C.pool := by
  intro d hd
  unfold marginal at hd
  rw [Finset.mem_filter] at hd
  obtain ⟨-, A, hA, hdA⟩ := hd
  rw [← hA.2.2]
  exact Finset.mem_biUnion.mpr ⟨s, Finset.mem_univ s, hdA⟩

/-- The certain marks and the ambiguous pool exhaust the pool. -/
theorem biUnion_certain_union_amb (C : CellSys H α) :
    Finset.univ.biUnion C.certain ∪ C.amb = C.pool := by
  apply Finset.Subset.antisymm
  · apply Finset.union_subset
    · intro d hd
      rw [Finset.mem_biUnion] at hd
      obtain ⟨s, -, hs⟩ := hd
      exact C.certain_subset_pool s hs
    · exact C.amb_subset_pool
  · intro d hd
    rcases mem_amb_or_certain hd with h | ⟨s, hs⟩
    · exact Finset.mem_union_right _ h
    · exact Finset.mem_union_left _
        (Finset.mem_biUnion.mpr ⟨s, Finset.mem_univ s, hs⟩)

/-- Pool cardinality splits into certain marks and ambiguity. -/
theorem card_pool_split (C : CellSys H α) :
    C.pool.card = (∑ s, (C.certain s).card) + C.amb.card := by
  have hsubK : Finset.univ.biUnion C.certain ⊆ C.pool := by
    intro d hd
    rw [Finset.mem_biUnion] at hd
    obtain ⟨s, -, hs⟩ := hd
    exact C.certain_subset_pool s hs
  have hK : (Finset.univ.biUnion C.certain).card
      = ∑ s, (C.certain s).card :=
    Finset.card_biUnion fun s _ t _ hst => C.certain_disjoint hst
  have hle := Finset.card_le_card hsubK
  have : C.amb.card = C.pool.card
      - (Finset.univ.biUnion C.certain).card := by
    unfold amb
    rw [Finset.card_sdiff_of_subset hsubK]
  omega

/-- Extract the unique element of a one-element set (computably). -/
private def theUnique (X : Finset H) (h : X.card = 1) : H :=
  X.choose (fun _ => True) (by
    obtain ⟨a, ha⟩ := Finset.card_eq_one.mp h
    refine ⟨a, ⟨by rw [ha]; exact Finset.mem_singleton_self a, trivial⟩,
      ?_⟩
    rintro b ⟨hb, -⟩
    rw [ha, Finset.mem_singleton] at hb
    exact hb)

omit [Fintype H] [DecidableEq H] in
private theorem eq_singleton_theUnique {X : Finset H} (h : X.card = 1) :
    X = {theUnique X h} := by
  obtain ⟨a, ha⟩ := Finset.card_eq_one.mp h
  have hmem : theUnique X h ∈ X := (Finset.choose_spec _ _ _).1
  set u := theUnique X h with hu
  rw [ha] at hmem ⊢
  rw [Finset.mem_singleton] at hmem
  rw [hmem]

/-- The compiler `𝒩(C)` (Math §7.10): the normal form read off the
marginal holder relation — certain marks, ambiguous pool, residual
capacities, and the sparse exclusion of the unique missing active
holder. -/
def compile (C : CellSys H α) : SupportNF H α where
  certain := C.certain
  amb := C.amb
  resid := C.resid
  excl := fun d =>
    if h : d ∈ C.amb ∧ (C.active \ C.holders d).card = 1
    then some (theUnique _ h.2)
    else none

theorem compile_certain (C : CellSys H α) :
    C.compile.certain = C.certain := rfl

theorem compile_amb (C : CellSys H α) : C.compile.amb = C.amb := rfl

theorem compile_resid (C : CellSys H α) : C.compile.resid = C.resid := rfl

theorem compile_active (C : CellSys H α) :
    C.compile.active = C.active := rfl

/-- The compiled exclusion characterized. -/
theorem compile_excl_eq_some {C : CellSys H α} {d : α} {s : H} :
    C.compile.excl d = some s
      ↔ d ∈ C.amb ∧ C.active \ C.holders d = {s} := by
  show (if h : d ∈ C.amb ∧ (C.active \ C.holders d).card = 1
      then some (theUnique _ h.2) else none) = some s ↔ _
  split_ifs with h
  · rw [Option.some_inj]
    constructor
    · rintro rfl
      exact ⟨h.1, eq_singleton_theUnique h.2⟩
    · rintro ⟨-, hX⟩
      exact Finset.singleton_inj.mp
        ((eq_singleton_theUnique h.2).symm.trans hX)
  · constructor
    · rintro ⟨⟩
    · rintro ⟨hd, hX⟩
      exact absurd ⟨hd, by rw [hX]; exact Finset.card_singleton s⟩ h

theorem compile_excl_eq_none {C : CellSys H α} {d : α}
    (h : ¬(d ∈ C.amb ∧ (C.active \ C.holders d).card = 1)) :
    C.compile.excl d = none := dif_neg h

/-- PA-D02: the compiled normal form of a feasible three-holder system
is well-formed — the branch invariants hold, with the ternary validator
supplied by the §7.11 strict singleton Hall inequality. -/
theorem wellFormed_compile {C : CellSys H α} (hH : Fintype.card H = 3)
    (hC : C.Feasible) : C.compile.WellFormed := by
  refine ⟨fun s t hst => C.certain_disjoint hst,
    fun s => C.amb_disjoint_certain s, ?_, ?_, ?_⟩
  · -- conservation of residuals
    show ∑ s, C.resid s = C.amb.card
    have hsplit := card_pool_split C
    have hcap := sum_cap_eq_card_pool hC
    have hle : ∀ s, (C.certain s).card ≤ C.cap s := card_certain_le hC
    have hsum_resid : ∑ s, C.resid s
        = (∑ s, C.cap s) - ∑ s, (C.certain s).card := by
      unfold resid
      rw [eq_tsub_iff_add_eq_of_le (Finset.sum_le_sum fun s _ => hle s),
        ← Finset.sum_add_distrib]
      apply Finset.sum_congr rfl
      intro s _
      have := hle s
      omega
    omega
  · -- exclusions live on the ambiguous pool at active seats
    intro d s h
    obtain ⟨hd, hX⟩ := compile_excl_eq_some.mp h
    have hs : s ∈ C.active \ C.holders d := by
      rw [hX]
      exact Finset.mem_singleton_self s
    rw [Finset.mem_sdiff, mem_active] at hs
    exact ⟨hd, hs.1⟩
  · -- the branch trichotomy
    rcases active_trichotomy hH hC with h0 | h2 | h3
    · exact Or.inl (by
        show C.active = ∅
        exact Finset.card_eq_zero.mp h0)
    · refine Or.inr (Or.inl ⟨h2, fun d => ?_⟩)
      apply compile_excl_eq_none
      rintro ⟨hd, hcard⟩
      rw [holders_eq_active_of_two hC h2 hd, Finset.sdiff_self] at hcard
      simp at hcard
    · refine Or.inr (Or.inr ⟨h3, fun s hs => ?_⟩)
      show C.resid s
        + (C.amb.filter fun d => C.compile.excl d = some s).card + 1
        ≤ C.amb.card
      have hhall := strict_singleton_hall hC (s := s) (by exact hs)
      -- the excluded tiles and the marginal neighborhood are disjoint
      have hdisj : Disjoint
          (C.amb.filter fun d => C.compile.excl d = some s)
          (C.amb.filter fun d => d ∈ C.marginal s) := by
        rw [Finset.disjoint_left]
        intro d hd hd'
        rw [Finset.mem_filter] at hd hd'
        obtain ⟨-, hX⟩ := compile_excl_eq_some.mp hd.2
        have : s ∈ C.active \ C.holders d := by
          rw [hX]; exact Finset.mem_singleton_self s
        rw [Finset.mem_sdiff, mem_holders] at this
        exact this.2 hd'.2
      have hunion : (C.amb.filter fun d => C.compile.excl d = some s)
          ∪ (C.amb.filter fun d => d ∈ C.marginal s) ⊆ C.amb :=
        Finset.union_subset (Finset.filter_subset _ _)
          (Finset.filter_subset _ _)
      have hcards := Finset.card_le_card hunion
      rw [Finset.card_union_of_disjoint hdisj] at hcards
      omega

/-- PA-D03 (compile then decode): the decoded compiled form *is* the
canonical reduction — `decode (𝒩(C)) = red(C)` (Math §7.10). -/
theorem decode_compile {C : CellSys H α} (hH : Fintype.card H = 3)
    (hC : C.Feasible) : C.compile.decode = C.red := by
  apply cellSys_ext
  · show Finset.univ.biUnion C.certain ∪ C.amb = C.pool
    exact C.biUnion_certain_union_amb
  · funext s
    show C.certain s ∪ C.compile.ambAllowed s = C.marginal s
    have hamb_char : ∀ d, d ∈ C.compile.ambAllowed s
        ↔ 0 < C.resid s ∧ d ∈ C.amb ∧ C.compile.excl d ≠ some s :=
      fun d => SupportNF.mem_ambAllowed
    apply Finset.Subset.antisymm
    · apply Finset.union_subset
      · intro d hd
        exact mem_marginal_of_certain hd
      · intro d hd
        rw [hamb_char d] at hd
        obtain ⟨hs, hdamb, hex⟩ := hd
        rw [← mem_holders]
        by_contra hsh
        -- `s` active but not a holder: the exclusion must name `s`
        rcases active_trichotomy hH hC with h0 | h2 | h3
        · rw [Finset.card_eq_zero] at h0
          have : s ∈ C.active := mem_active.mpr hs
          rw [h0] at this
          exact Finset.notMem_empty s this
        · have : s ∈ C.holders d := by
            rw [holders_eq_active_of_two hC h2 hdamb]
            exact mem_active.mpr hs
          exact hsh this
        · have hle := excl_card_le_one_of_three hC h3 hdamb
          have hmem : s ∈ C.active \ C.holders d :=
            Finset.mem_sdiff.mpr ⟨mem_active.mpr hs, hsh⟩
          have hcard : (C.active \ C.holders d).card = 1 := by
            have := Finset.card_pos.mpr ⟨s, hmem⟩
            omega
          have hsing := eq_singleton_theUnique hcard
          have : C.compile.excl d = some s := by
            rw [compile_excl_eq_some]
            refine ⟨hdamb, ?_⟩
            apply Finset.eq_singleton_iff_unique_mem.mpr
            refine ⟨hmem, fun t ht => ?_⟩
            rw [hsing, Finset.mem_singleton] at hmem ht
            rw [ht, ← hmem]
          exact hex this
    · intro d hd
      have hpool := C.marginal_subset_pool s hd
      rcases mem_amb_or_certain hpool with hdamb | ⟨t, ht⟩
      · apply Finset.mem_union_right
        rw [hamb_char d]
        have hsh : s ∈ C.holders d := mem_holders.mpr hd
        have hact : s ∈ C.active := holders_subset_active hdamb hsh
        refine ⟨mem_active.mp hact, hdamb, fun hex => ?_⟩
        obtain ⟨-, hX⟩ := compile_excl_eq_some.mp hex
        have : s ∈ C.active \ C.holders d := by
          rw [hX]; exact Finset.mem_singleton_self s
        exact (Finset.mem_sdiff.mp this).2 hsh
      · apply Finset.mem_union_left
        have hht := (mem_certain.mp ht).2
        have hsh : s ∈ C.holders d := mem_holders.mpr hd
        rw [hht, Finset.mem_singleton] at hsh
        rwa [hsh]
  · funext s
    show (C.certain s).card + C.resid s = C.cap s
    have := card_certain_le hC s
    unfold resid
    omega

end CellSys

/-! ## PA-D04: compile and decode are mutually inverse -/

namespace SupportNF

variable {H α : Type} [Fintype H] [DecidableEq H] [Fintype α]
  [DecidableEq α]

/-- Structure extensionality for normal forms. -/
theorem ext' {M N : SupportNF H α} (h1 : M.certain = N.certain)
    (h2 : M.amb = N.amb) (h3 : M.resid = N.resid)
    (h4 : M.excl = N.excl) : M = N := by
  cases M
  cases N
  dsimp only at h1 h2 h3 h4
  subst h1 h2 h3 h4
  rfl

/-- A well-formed ambiguous tile is declared at two distinct seats. -/
theorem exists_two_ambAllowed {N : SupportNF H α} (hwf : N.WellFormed)
    {d : α} (hd : d ∈ N.amb) :
    ∃ s t : H, s ≠ t ∧ d ∈ N.ambAllowed s ∧ d ∈ N.ambAllowed t := by
  rcases hwf.branch with hdet | ⟨h2, hnone⟩ | ⟨h3, hval⟩
  · exact absurd hd
      (by rw [amb_eq_empty hwf hdet]; exact Finset.notMem_empty d)
  · obtain ⟨a, b, hab, hactive⟩ := Finset.card_eq_two.mp h2
    have ha : a ∈ N.active := by
      rw [hactive]; exact Finset.mem_insert_self a _
    have hb : b ∈ N.active := by
      rw [hactive]
      exact Finset.mem_insert_of_mem (Finset.mem_singleton_self b)
    exact ⟨a, b, hab,
      mem_ambAllowed.mpr ⟨mem_active.mp ha, hd, by rw [hnone d]; simp⟩,
      mem_ambAllowed.mpr ⟨mem_active.mp hb, hd, by rw [hnone d]; simp⟩⟩
  · rcases hexc : N.excl d with _ | u
    · obtain ⟨a, ha, b, hb, hab⟩ :=
        Finset.one_lt_card.mp
          (show 1 < N.active.card by rw [h3]; omega)
      exact ⟨a, b, hab,
        mem_ambAllowed.mpr ⟨mem_active.mp ha, hd, by rw [hexc]; simp⟩,
        mem_ambAllowed.mpr ⟨mem_active.mp hb, hd, by rw [hexc]; simp⟩⟩
    · have hcard : 1 < (N.active.erase u).card := by
        have := Finset.pred_card_le_card_erase
          (s := N.active) (a := u)
        rw [h3] at this
        omega
      obtain ⟨a, ha, b, hb, hab⟩ := Finset.one_lt_card.mp hcard
      obtain ⟨hau, ha'⟩ := Finset.mem_erase.mp ha
      obtain ⟨hbu, hb'⟩ := Finset.mem_erase.mp hb
      refine ⟨a, b, hab,
        mem_ambAllowed.mpr ⟨mem_active.mp ha', hd, fun h =>
          hau (Option.some.inj (hexc.symm.trans h)).symm⟩,
        mem_ambAllowed.mpr ⟨mem_active.mp hb', hd, fun h =>
          hbu (Option.some.inj (hexc.symm.trans h)).symm⟩⟩

/-- PA-D04: compiling the decoded system returns the same normal form —
the stored payload is exactly the marginal content of its fiber. -/
theorem compile_decode {N : SupportNF H α} (hwf : N.WellFormed) :
    N.decode.compile = N := by
  -- the decoded holder relation is the declared one
  have hhold : ∀ (d : α) (t : H),
      t ∈ N.decode.holders d ↔ d ∈ N.certain t ∨ d ∈ N.ambAllowed t := by
    intro d t
    rw [CellSys.mem_holders, decode_marginal hwf, Finset.mem_union]
  -- certain marks agree
  have hcert : CellSys.certain N.decode = N.certain := by
    funext s
    ext d
    rw [CellSys.mem_certain]
    constructor
    · rintro ⟨hpool, hsingle⟩
      rw [decode_pool, Finset.mem_union] at hpool
      rcases hpool with hK | hamb
      · rw [Finset.mem_biUnion] at hK
        obtain ⟨t, -, ht⟩ := hK
        have : t ∈ N.decode.holders d := (hhold d t).mpr (Or.inl ht)
        rw [hsingle, Finset.mem_singleton] at this
        rwa [this] at ht
      · exfalso
        obtain ⟨a, b, hab, hda, hdb⟩ := exists_two_ambAllowed hwf hamb
        have ha : a ∈ N.decode.holders d := (hhold d a).mpr (Or.inr hda)
        have hb : b ∈ N.decode.holders d := (hhold d b).mpr (Or.inr hdb)
        rw [hsingle, Finset.mem_singleton] at ha hb
        exact hab (ha.trans hb.symm)
    · intro hd
      refine ⟨by
        rw [decode_pool]
        exact Finset.mem_union_left _
          (Finset.mem_biUnion.mpr ⟨s, Finset.mem_univ s, hd⟩), ?_⟩
      apply Finset.eq_singleton_iff_unique_mem.mpr
      refine ⟨(hhold d s).mpr (Or.inl hd), fun t ht => ?_⟩
      rcases (hhold d t).mp ht with h | h
      · by_contra hts
        exact Finset.disjoint_left.mp (hwf.certain_disj t s hts) h hd
      · exact absurd hd (Finset.disjoint_left.mp (hwf.amb_disj s)
          (N.ambAllowed_subset_amb t h))
  -- ambiguous pools agree
  have hamb : CellSys.amb N.decode = N.amb := by
    unfold CellSys.amb
    rw [hcert, decode_pool]
    ext d
    rw [Finset.mem_sdiff, Finset.mem_union]
    constructor
    · rintro ⟨hK | hamb, hnK⟩
      · exact absurd hK hnK
      · exact hamb
    · intro hd
      refine ⟨Or.inr hd, fun hK => ?_⟩
      rw [Finset.mem_biUnion] at hK
      obtain ⟨t, -, ht⟩ := hK
      exact Finset.disjoint_left.mp (hwf.amb_disj t) hd ht
  -- residuals agree
  have hresid : CellSys.resid N.decode = N.resid := by
    funext s
    unfold CellSys.resid
    rw [decode_cap, hcert]
    omega
  -- exclusions agree
  have hexcl : (CellSys.compile N.decode).excl = N.excl := by
    funext d
    rcases hexc : N.excl d with _ | u
    · -- nothing is declared excluded: nothing is compiled excluded
      apply CellSys.compile_excl_eq_none
      rintro ⟨hd, hcard⟩
      rw [hamb] at hd
      obtain ⟨t, ht⟩ := Finset.card_pos.mp (by omega :
        0 < (CellSys.active N.decode \ N.decode.holders d).card)
      rw [Finset.mem_sdiff] at ht
      obtain ⟨htact, hthol⟩ := ht
      rw [CellSys.mem_active, hresid] at htact
      exact hthol ((hhold d t).mpr (Or.inr
        (mem_ambAllowed.mpr ⟨htact, hd, by rw [hexc]; simp⟩)))
    · -- a declared exclusion is recompiled verbatim
      obtain ⟨hd, hu⟩ := hwf.excl_mem d u hexc
      rw [CellSys.compile_excl_eq_some]
      refine ⟨by rwa [hamb], ?_⟩
      apply Finset.eq_singleton_iff_unique_mem.mpr
      constructor
      · rw [Finset.mem_sdiff, CellSys.mem_active, hresid]
        refine ⟨hu, fun hhol => ?_⟩
        rcases (hhold d u).mp hhol with h | h
        · exact Finset.disjoint_left.mp (hwf.amb_disj u) hd h
        · exact (mem_ambAllowed.mp h).2.2 hexc
      · intro t ht
        rw [Finset.mem_sdiff, CellSys.mem_active, hresid] at ht
        obtain ⟨htact, hthol⟩ := ht
        by_contra htu
        exact hthol ((hhold d t).mpr (Or.inr (mem_ambAllowed.mpr
          ⟨htact, hd, fun h =>
            htu (Option.some.inj (hexc.symm.trans h)).symm⟩)))
  exact ext' hcert hamb hresid hexcl

end SupportNF

/-! ## PA-D05: the total normal form classifies exact fibers -/

namespace CellSys

variable {H α : Type} [Fintype H] [DecidableEq H] [Fintype α]
  [DecidableEq α]

/-- Math §7.10 (total exact support normal form): one `Empty` tag for
every infeasible system, the compiled payload otherwise. -/
def totalNF (C : CellSys H α) : Option (SupportNF H α) :=
  if C.Feasible then some C.compile else none

/-- Math §7.10 (nonempty support recovery, world half): equal nonempty
fibers force equal pools and capacities — both are read off any common
world. -/
theorem pool_cap_eq_of_fiber_eq {C Q : CellSys H α} (hC : C.Feasible)
    (hfib : ∀ A, C.IsWorld A ↔ Q.IsWorld A) :
    C.pool = Q.pool ∧ C.cap = Q.cap := by
  obtain ⟨A, hA⟩ := hC
  have hQA := (hfib A).mp hA
  constructor
  · rw [← hA.2.2, ← hQA.2.2]
  · funext s
    rw [← (hA.1 s).2, ← (hQA.1 s).2]

/-- The compiler factors through the canonical reduction. -/
theorem compile_eq_of_red_eq {C Q : CellSys H α} (hred : C.red = Q.red) :
    C.compile = Q.compile := by
  have hpool : C.pool = Q.pool :=
    show C.red.pool = Q.red.pool from congrArg _ hred
  have hmarg : C.marginal = Q.marginal :=
    show C.red.allowed = Q.red.allowed from congrArg _ hred
  have hcap : C.cap = Q.cap :=
    show C.red.cap = Q.red.cap from congrArg _ hred
  have hholders : CellSys.holders C = CellSys.holders Q := by
    funext d
    unfold holders
    rw [hmarg]
  have hcertain : CellSys.certain C = CellSys.certain Q := by
    funext s
    unfold certain
    rw [hpool, hholders]
  have hamb : CellSys.amb C = CellSys.amb Q := by
    unfold amb
    rw [hpool, hcertain]
  have hresid : CellSys.resid C = CellSys.resid Q := by
    funext s
    unfold resid
    rw [hcap, hcertain]
  have hactive : CellSys.active C = CellSys.active Q := by
    unfold active
    rw [hresid]
  have hexcl : C.compile.excl = Q.compile.excl := by
    funext d
    rcases hq : Q.compile.excl d with _ | u
    · rcases hc : C.compile.excl d with _ | u
      · rfl
      · exfalso
        obtain ⟨hd, hX⟩ := compile_excl_eq_some.mp hc
        rw [hamb] at hd
        rw [hactive, hholders] at hX
        have h2 : Q.compile.excl d = some u :=
          compile_excl_eq_some.mpr ⟨hd, hX⟩
        rw [hq] at h2
        simp at h2
    · obtain ⟨hd, hX⟩ := compile_excl_eq_some.mp hq
      rw [← hamb] at hd
      rw [← hactive, ← hholders] at hX
      exact compile_excl_eq_some.mpr ⟨hd, hX⟩
  exact SupportNF.ext' hcertain hamb hresid hexcl

/-- PA-D05 (Math §7.10 global classification): two systems — feasible
or not — present the same exact fiber iff they have the same total
support normal form. The forward direction runs through world recovery
and the §7.9 coarsest-quotient theorem; the backward direction decodes. -/
theorem fiber_eq_iff_totalNF_eq (hH : Fintype.card H = 3)
    {C Q : CellSys H α} :
    (∀ A, C.IsWorld A ↔ Q.IsWorld A) ↔ C.totalNF = Q.totalNF := by
  constructor
  · intro hfib
    by_cases hC : C.Feasible
    · have hQ : Q.Feasible := by
        obtain ⟨A, hA⟩ := hC
        exact ⟨A, (hfib A).mp hA⟩
      obtain ⟨hpool, hcap⟩ := pool_cap_eq_of_fiber_eq hC hfib
      have hred : C.red = Q.red := (fiber_eq_iff_red_eq hpool hcap).mp hfib
      unfold totalNF
      rw [if_pos hC, if_pos hQ, Option.some_inj]
      exact compile_eq_of_red_eq hred
    · have hQ : ¬Q.Feasible := fun ⟨A, hA⟩ => hC ⟨A, (hfib A).mpr hA⟩
      unfold totalNF
      rw [if_neg hC, if_neg hQ]
  · intro htot A
    unfold totalNF at htot
    by_cases hC : C.Feasible <;> by_cases hQ : Q.Feasible
    · rw [if_pos hC, if_pos hQ, Option.some_inj] at htot
      have hred : C.red = Q.red := by
        rw [← decode_compile hH hC, ← decode_compile hH hQ, htot]
      rw [← C.isWorld_red_iff A, hred, Q.isWorld_red_iff A]
    · rw [if_pos hC, if_neg hQ] at htot
      exact absurd htot (Option.some_ne_none _)
    · rw [if_neg hC, if_pos hQ] at htot
      exact absurd htot.symm (Option.some_ne_none _)
    · exact ⟨fun hA => absurd ⟨A, hA⟩ hC, fun hA => absurd ⟨A, hA⟩ hQ⟩

end CellSys

end Texas42
