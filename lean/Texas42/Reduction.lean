/-
Copyright (c) 2026 Jason Yandell. All rights reserved.
Released under Apache 2.0 license as described in the file LICENSE.
Authors: Jason Yandell
-/
import Texas42.Cells

/-!
# Texas 42 — the canonical support reduction (generic cell kernel)

K7 groundwork and PA-C15 (Math §7.9): over a generic finite capacitated
cell system, the marginal holder support `P_h⋆ = ⋃_{ω ∈ Φ(C)} H_h(ω)`
defines the canonical support reduction `red(C)`. It preserves the
fiber, is contractive and idempotent, is the unique coordinatewise-least
possible-set system representing its fiber, and induces the coarsest
exact semantic quotient: `Φ(C) = Φ(Q) ↔ red(C) = red(Q)` (fixed pool and
capacities) — the backbone of the support normal form (PA-D05/D06).
-/

namespace Texas42

/-- K7: a generic finite capacitated cell system — one conserved pool,
per-holder allowed sets and exact quotas (rec Kernel §K7). -/
structure CellSys (H α : Type) [Fintype H] [DecidableEq H]
    [Fintype α] [DecidableEq α] where
  pool : Finset α
  allowed : H → Finset α
  cap : H → ℕ

namespace CellSys

variable {H α : Type} [Fintype H] [DecidableEq H] [Fintype α]
  [DecidableEq α]

/-- Membership in the fiber `Φ(C)`: an assignment within the allowed
sets, at exact quotas, disjoint, conserving the pool (Math §7.3,
generic). -/
def IsWorld (C : CellSys H α) (A : H → Finset α) : Prop :=
  (∀ h, A h ⊆ C.allowed h ∧ (A h).card = C.cap h)
    ∧ (∀ h h' : H, h ≠ h' → Disjoint (A h) (A h'))
    ∧ Finset.univ.biUnion A = C.pool

instance (C : CellSys H α) (A : H → Finset α) : Decidable (C.IsWorld A) :=
  inferInstanceAs (Decidable
    ((∀ h, A h ⊆ C.allowed h ∧ (A h).card = C.cap h)
      ∧ (∀ h h' : H, h ≠ h' → Disjoint (A h) (A h'))
      ∧ Finset.univ.biUnion A = C.pool))

/-- The marginal holder support `P_h⋆` — the tiles the holder actually
receives in some conserved world (Math §7.9). -/
def marginal (C : CellSys H α) (h : H) : Finset α :=
  (C.allowed h).filter fun d => ∃ A, C.IsWorld A ∧ d ∈ A h

/-- The canonical support reduction `red(C)` (Math §7.9). -/
def red (C : CellSys H α) : CellSys H α :=
  ⟨C.pool, C.marginal, C.cap⟩

/-- The reduction is contractive: `C⋆ ≤ C` coordinatewise. -/
theorem marginal_subset (C : CellSys H α) (h : H) :
    C.marginal h ⊆ C.allowed h := by
  unfold marginal
  exact Finset.filter_subset _ _

/-- Math §7.9 item 1: the reduction preserves the fiber exactly. -/
theorem isWorld_red_iff (C : CellSys H α) (A : H → Finset α) :
    C.red.IsWorld A ↔ C.IsWorld A := by
  constructor
  · rintro ⟨hcell, hdisj, hcons⟩
    exact ⟨fun h => ⟨(hcell h).1.trans (C.marginal_subset h), (hcell h).2⟩,
      hdisj, hcons⟩
  · rintro ⟨hcell, hdisj, hcons⟩
    refine ⟨fun h => ⟨?_, (hcell h).2⟩, hdisj, hcons⟩
    intro d hd
    unfold red marginal
    rw [Finset.mem_filter]
    exact ⟨(hcell h).1 hd, A, ⟨hcell, hdisj, hcons⟩, hd⟩

/-- Math §7.9 item 2: any possible-set system with the same fiber
contains every marginal support — coordinatewise leastness. -/
theorem marginal_le_of_fiber_eq {C Q : CellSys H α}
    (hfib : ∀ A, C.IsWorld A ↔ Q.IsWorld A) (h : H) :
    C.marginal h ⊆ Q.allowed h := by
  intro d hd
  unfold marginal at hd
  rw [Finset.mem_filter] at hd
  obtain ⟨-, A, hA, hdA⟩ := hd
  exact (((hfib A).mp hA).1 h).1 hdA

/-- Math §7.9 item 5: the reduction is idempotent. -/
theorem red_red (C : CellSys H α) : C.red.red = C.red := by
  unfold red
  simp only [CellSys.mk.injEq, true_and, and_true]
  funext h
  ext d
  show d ∈ CellSys.marginal C.red h ↔ d ∈ C.marginal h
  unfold marginal
  rw [Finset.mem_filter, Finset.mem_filter]
  constructor
  · rintro ⟨hall, A, hA, hdA⟩
    have hA' : C.IsWorld A := (C.isWorld_red_iff A).mp hA
    exact ⟨(hA'.1 h).1 hdA, A, hA', hdA⟩
  · rintro ⟨hall, A, hA, hdA⟩
    refine ⟨?_, A, (C.isWorld_red_iff A).mpr hA, hdA⟩
    show d ∈ C.marginal h
    unfold marginal
    rw [Finset.mem_filter]
    exact ⟨(hA.1 h).1 hdA, A, hA, hdA⟩

/-- Math §7.9 item 7 (the canonical normal form / coarsest exact
quotient): two systems with common pool and capacities have equal fibers
iff they have equal reductions. -/
theorem fiber_eq_iff_red_eq {C Q : CellSys H α} (hpool : C.pool = Q.pool)
    (hcap : C.cap = Q.cap) :
    (∀ A, C.IsWorld A ↔ Q.IsWorld A) ↔ C.red = Q.red := by
  constructor
  · intro hfib
    unfold red
    simp only [CellSys.mk.injEq]
    refine ⟨hpool, ?_, hcap⟩
    funext h
    apply Finset.Subset.antisymm
    · intro d hd
      have hall := marginal_le_of_fiber_eq hfib h hd
      unfold marginal at hd ⊢
      rw [Finset.mem_filter] at hd ⊢
      obtain ⟨-, A, hA, hdA⟩ := hd
      exact ⟨hall, A, (hfib A).mp hA, hdA⟩
    · intro d hd
      have hall := marginal_le_of_fiber_eq (fun A => (hfib A).symm) h hd
      unfold marginal at hd ⊢
      rw [Finset.mem_filter] at hd ⊢
      obtain ⟨-, A, hA, hdA⟩ := hd
      exact ⟨hall, A, (hfib A).mpr hA, hdA⟩
  · intro hred A
    rw [← C.isWorld_red_iff A, ← Q.isWorld_red_iff A, hred]

end CellSys

/-! ## The game cells as a cell system -/

namespace ViewerCtx

/-- The viewer's derived cells as a generic cell system: the viewer's
own coordinate is normalized to the empty allowed set at quota zero. -/
def cellSys (v : ViewerCtx) (P : PubState) : CellSys Seat Domino where
  pool := v.pool P
  allowed := fun s => if s = v.viewer then ∅ else v.allowed P s
  cap := fun s => if s = v.viewer then 0 else capacity P s

/-- The concrete fiber membership of Math §7.3 agrees with the generic
cell-system fiber. -/
theorem isWorld_iff_cellSys (v : ViewerCtx) (P : PubState)
    (A : Seat → Finset Domino) :
    v.IsWorld P A ↔ (v.cellSys P).IsWorld A := by
  constructor
  · rintro ⟨hm, hcell, hdisj, hcons⟩
    refine ⟨fun s => ?_, hdisj, hcons⟩
    rcases eq_or_ne s v.viewer with rfl | hs
    · rw [hm]
      unfold cellSys
      simp
    · unfold cellSys
      simp only [if_neg hs]
      exact hcell s hs
  · rintro ⟨hcell, hdisj, hcons⟩
    have hm : A v.viewer = ∅ := by
      have := (hcell v.viewer).1
      unfold cellSys at this
      dsimp only at this
      rw [if_pos rfl] at this
      exact Finset.subset_empty.mp this
    refine ⟨hm, fun s hs => ?_, hdisj, hcons⟩
    have := hcell s
    unfold cellSys at this
    simp only [if_neg hs] at this
    exact this

end ViewerCtx

end Texas42
