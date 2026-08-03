/-
Copyright (c) 2026 Jason Yandell. All rights reserved.
Released under Apache 2.0 license as described in the file LICENSE.
Authors: Jason Yandell
-/
import Mathlib

/-!
# Texas 42 — Layer A finite algebra (first slice)

Kernel-checked foundations for the domino set, following the mechanization
ledger rows PA-A01 (finite types), PA-A02 (exactly 28 canonical dominoes),
PA-A03 (natural incidence covering and pair intersections), and PA-A04
(count-point function and total 35).

A domino is represented canonically as an ordered pair `(high, low)` with a
proof `low ≤ high`, per the handoff's recommendation (Layer A): prefer the
canonical pair over a quotient by pair permutation.
-/

namespace Texas42

/-- PA-A01: a pip value, `0` through `6`. -/
abbrev Pip := Fin 7

/-- PA-A01: a seat, one of four in play order. -/
abbrev Seat := Fin 4

/-- PA-A01: a team (fixed partnership). -/
abbrev Team := Fin 2

namespace Seat

/-- Seats advance cyclically: `s⁺ = s + 1 (mod 4)` (Math §2.3). -/
def next (s : Seat) : Seat := s + 1

/-- The fixed partnership map `θ(s) = s mod 2`: seats 0,2 vs 1,3 (Math §2.3). -/
def team (s : Seat) : Team := ⟨s.val % 2, Nat.mod_lt _ (by norm_num)⟩

end Seat

/-- PA-A01: a domino, as the canonical ordered pair of pips with `low ≤ high`. -/
structure Domino where
  high : Pip
  low : Pip
  le : low ≤ high
deriving DecidableEq

namespace Domino

/-- The canonical-pair representation is equivalent to the subtype of ordered
pip pairs; this transports `Fintype` and makes cardinality kernel-computable. -/
def equivSubtype : Domino ≃ { p : Pip × Pip // p.2 ≤ p.1 } where
  toFun d := ⟨(d.high, d.low), d.le⟩
  invFun p := ⟨p.1.1, p.1.2, p.2⟩

instance : Fintype Domino := Fintype.ofEquiv _ equivSubtype.symm

/-- PA-A02: there are exactly 28 canonical dominoes. -/
theorem card_domino : Fintype.card Domino = 28 := by
  rw [Fintype.card_congr equivSubtype]
  decide

/-- The pip sum of a domino. -/
def pipSum (d : Domino) : ℕ :=
  d.high.val + d.low.val

/-- PA-A04: the count-point function. The three tiles of pip sum 5 —
`(5,0)`, `(4,1)`, `(3,2)` — score 5; the two tiles of pip sum 10 —
`(5,5)`, `(6,4)` — score 10; every other tile scores 0. -/
def countPoints (d : Domino) : ℕ :=
  if pipSum d = 5 then 5
  else if pipSum d = 10 then 10
  else 0

/-- PA-A04: the count points over all 28 dominoes total 35. -/
theorem total_countPoints : ∑ d : Domino, countPoints d = 35 := by
  decide

/-! ## Natural incidence covering (PA-A03, Math §2.2) -/

/-- `d.hasPip p`: pip `p` appears on `d` — membership in the natural
incidence set `σ_p`. -/
def hasPip (d : Domino) (p : Pip) : Prop :=
  d.high = p ∨ d.low = p

instance (d : Domino) (p : Pip) : Decidable (d.hasPip p) :=
  inferInstanceAs (Decidable (_ ∨ _))

/-- A double: both ends carry the same pip. -/
def isDouble (d : Domino) : Prop :=
  d.high = d.low

instance (d : Domino) : Decidable d.isDouble :=
  inferInstanceAs (Decidable (_ = _))

/-- The canonical domino on pips `p` and `q`. -/
def ofPips (p q : Pip) : Domino :=
  if h : q ≤ p then ⟨p, q, h⟩ else ⟨q, p, (not_le.mp h).le⟩

/-- The natural incidence set `σ_p = {d : p ∈ d}` as a finset (Math §2.2). -/
def incidence (p : Pip) : Finset Domino :=
  Finset.univ.filter (fun d => d.hasPip p)

/-- PA-A03: `|σ_p| = 7` — each pip is incident to exactly seven dominoes. -/
theorem card_incidence : ∀ p : Pip, (incidence p).card = 7 := by
  decide

/-- PA-A03: a double lies in exactly one natural incidence set; a mixed
domino lies in exactly two. -/
theorem card_pip_memberships : ∀ d : Domino,
    (Finset.univ.filter fun p => d.hasPip p).card =
      if d.isDouble then 1 else 2 := by
  decide

/-- PA-A03: for `p ≠ q`, `σ_p ∩ σ_q = {p:q}` — the covering's pairwise
intersections are the single mixed dominoes. -/
theorem incidence_inter : ∀ p q : Pip, p ≠ q →
    incidence p ∩ incidence q = {ofPips p q} := by
  decide

/-! ## Injectivity helpers for the trick-key layer (used by PA-A10) -/

/-- Two dominoes sharing pip `p` with equal pip sums are equal: the shared
pip determines the other end from the sum. This is the arithmetic core of
rank injectivity inside a winning tier (Math §3.6). -/
theorem eq_of_hasPip_of_pipSum_eq {p : Pip} {d₁ d₂ : Domino}
    (h₁ : d₁.hasPip p) (h₂ : d₂.hasPip p)
    (h : d₁.pipSum = d₂.pipSum) : d₁ = d₂ := by
  obtain ⟨a, b, hab⟩ := d₁
  obtain ⟨c, e, hce⟩ := d₂
  simp only [hasPip, pipSum] at h₁ h₂ h
  simp only [Domino.mk.injEq]
  simp only [Fin.ext_iff] at h₁ h₂ ⊢
  simp only [Fin.le_def] at hab hce
  omega

/-- The only double containing pip `p` is `p:p`. -/
theorem eq_double_of_hasPip {p : Pip} {d : Domino}
    (hd : d.isDouble) (hp : d.hasPip p) : d = ⟨p, p, le_refl p⟩ := by
  obtain ⟨a, b, hab⟩ := d
  simp only [isDouble, hasPip] at hd hp
  simp only [Domino.mk.injEq]
  simp only [Fin.ext_iff] at hd hp ⊢
  omega

end Domino

end Texas42
