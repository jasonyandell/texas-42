/-
Copyright (c) 2026 Jason Yandell. All rights reserved.
Released under Apache 2.0 license as described in the file LICENSE.
Authors: Jason Yandell
-/
import Texas42.Trick

/-!
# Texas 42 — pip transports and the scoped `2 ↔ 3` isomorphism

Layer A close-out: pip permutations acting on dominoes, suits, and
declarations (Math §3.9), the count-preserving classification (PA-A16 —
only the identity and the swap `2 ↔ 3` preserve every count label), and
the scoped game-order transport (PA-A17 — under the induced endpoint and
suit-context transport, the swap is an order isomorphism exactly between
the Straight layers of declarations 2 and 3).

Per Math §3.9's caveat, PA-A17 preserves the *game-semantic order reduct*
(called/powered sets, effective suits, led context, follow, contextual
comparison, count) — not literal numeric rank labels: `2:0` has rank 2 in
twos while its image `3:0` has rank 3 in threes.

The count-preserving classification is proved by the spec's analytic
forcing argument, not by enumerating the 5040 permutations (that
exhaustive receipt stays external).
-/

namespace Texas42

/-- Math §3.9: a pip permutation transports a domino by acting on both
ends (then renormalizing to the canonical `(high, low)` pair). -/
def Domino.mapPips (σ : Equiv.Perm Pip) (d : Domino) : Domino :=
  Domino.ofPips (σ d.high) (σ d.low)

/-- Math §3.9: the induced suit-context map fixes the called suit. -/
def Suit.mapPips (σ : Equiv.Perm Pip) : Suit → Suit
  | .natural p => .natural (σ p)
  | .called => .called

/-- Math §3.9: pip declarations transport by `p ↦ σ(p)`; `DT` and `NT`
are fixed. -/
def Declaration.mapPips (σ : Equiv.Perm Pip) : Declaration → Declaration
  | .pip p => .pip (σ p)
  | .doubles => .doubles
  | .notrump => .notrump

namespace Domino

/-- The pip sum of the canonical pair on `a, b` is `a + b`. -/
theorem pipSum_ofPips (a b : Pip) : (ofPips a b).pipSum = a.val + b.val := by
  unfold ofPips pipSum
  split
  · rfl
  · dsimp only; omega

end Domino

/-- Two permutations of the seven pips agreeing on every pip are equal —
the finite extensionality used to close the classification. -/
theorem perm_ext7 (σ τ : Equiv.Perm Pip)
    (h0 : σ 0 = τ 0) (h1 : σ 1 = τ 1) (h2 : σ 2 = τ 2) (h3 : σ 3 = τ 3)
    (h4 : σ 4 = τ 4) (h5 : σ 5 = τ 5) (h6 : σ 6 = τ 6) : σ = τ := by
  refine Equiv.ext fun p => ?_
  match p with
  | ⟨0, _⟩ => exact h0
  | ⟨1, _⟩ => exact h1
  | ⟨2, _⟩ => exact h2
  | ⟨3, _⟩ => exact h3
  | ⟨4, _⟩ => exact h4
  | ⟨5, _⟩ => exact h5
  | ⟨6, _⟩ => exact h6

/-- PA-A16: a pip permutation preserves every count label. -/
def CountPreserving (σ : Equiv.Perm Pip) : Prop :=
  ∀ d : Domino, (d.mapPips σ).countPoints = d.countPoints

/-- Extraction: a count value of 10 forces pip sum 10. -/
private theorem sum_of_count_ten {s : ℕ}
    (h : (if s = 5 then 5 else if s = 10 then 10 else 0 : ℕ) = 10) :
    s = 10 := by
  split_ifs at h <;> omega

/-- Extraction: a count value of 5 forces pip sum 5. -/
private theorem sum_of_count_five {s : ℕ}
    (h : (if s = 5 then 5 else if s = 10 then 10 else 0 : ℕ) = 5) :
    s = 5 := by
  split_ifs at h <;> omega

/-- PA-A16 (count-preserving pip classification, Math §3.9): a pip
permutation preserves every count label iff it is the identity or the swap
`2 ↔ 3`. Forward direction by the spec's forcing argument: `5:5` forces
`σ(5) = 5`; `6:4` forces `{σ(6), σ(4)} = {6, 4}`; `5:0` forces `σ(0) = 0`;
`4:1` forces `σ(4) = 4`, hence `σ(6) = 6` and `σ(1) = 1`; `3:2` leaves
exactly fixing or swapping pips 2 and 3. -/
theorem countPreserving_iff (σ : Equiv.Perm Pip) :
    CountPreserving σ ↔ σ = 1 ∨ σ = Equiv.swap 2 3 := by
  constructor
  · intro hσ
    have hne : ∀ a b : Pip, a ≠ b → (σ a).val ≠ (σ b).val := fun a b hab h =>
      hab (σ.injective (Fin.ext h))
    -- count preservation at the five count dominoes, as pip-sum facts
    have h55 : ((⟨5, 5, le_refl 5⟩ : Domino).mapPips σ).countPoints = 10 := by
      rw [hσ]; decide
    have h64 : ((⟨6, 4, by decide⟩ : Domino).mapPips σ).countPoints = 10 := by
      rw [hσ]; decide
    have h50 : ((⟨5, 0, by decide⟩ : Domino).mapPips σ).countPoints = 5 := by
      rw [hσ]; decide
    have h41 : ((⟨4, 1, by decide⟩ : Domino).mapPips σ).countPoints = 5 := by
      rw [hσ]; decide
    have h32 : ((⟨3, 2, by decide⟩ : Domino).mapPips σ).countPoints = 5 := by
      rw [hσ]; decide
    simp only [Domino.mapPips, Domino.countPoints, Domino.pipSum_ofPips] at h55 h64 h50 h41 h32
    have hx55 := sum_of_count_ten h55
    have hx64 := sum_of_count_ten h64
    have hx50 := sum_of_count_five h50
    have hx41 := sum_of_count_five h41
    have hx32 := sum_of_count_five h32
    -- bounds and the injectivity facts the forcing argument uses
    have b0 := (σ 0).isLt; have b1 := (σ 1).isLt; have b2 := (σ 2).isLt
    have b3 := (σ 3).isLt; have b4 := (σ 4).isLt; have b5 := (σ 5).isLt
    have b6 := (σ 6).isLt
    have n65 := hne 6 5 (by decide); have n45 := hne 4 5 (by decide)
    have n20 := hne 2 0 (by decide); have n21 := hne 2 1 (by decide)
    have n24 := hne 2 4 (by decide); have n25 := hne 2 5 (by decide)
    have n26 := hne 2 6 (by decide); have n30 := hne 3 0 (by decide)
    have n31 := hne 3 1 (by decide); have n34 := hne 3 4 (by decide)
    have n35 := hne 3 5 (by decide); have n36 := hne 3 6 (by decide)
    -- the forced values
    have e5 : (σ 5).val = 5 := by omega
    have e0 : (σ 0).val = 0 := by omega
    have e4 : (σ 4).val = 4 := by omega
    have e6 : (σ 6).val = 6 := by omega
    have e1 : (σ 1).val = 1 := by omega
    have e23 : (σ 2).val = 2 ∨ (σ 2).val = 3 := by omega
    have v0 : ((0 : Pip)).val = 0 := rfl
    have v1 : ((1 : Pip)).val = 1 := rfl
    have v2 : ((2 : Pip)).val = 2 := rfl
    have v3 : ((3 : Pip)).val = 3 := rfl
    have v4 : ((4 : Pip)).val = 4 := rfl
    have v5 : ((5 : Pip)).val = 5 := rfl
    have v6 : ((6 : Pip)).val = 6 := rfl
    have g0 : σ 0 = (0 : Pip) := Fin.ext (by omega)
    have g1 : σ 1 = (1 : Pip) := Fin.ext (by omega)
    have g4 : σ 4 = (4 : Pip) := Fin.ext (by omega)
    have g5 : σ 5 = (5 : Pip) := Fin.ext (by omega)
    have g6 : σ 6 = (6 : Pip) := Fin.ext (by omega)
    rcases e23 with e2 | e2
    · left
      have e3 : (σ 3).val = 3 := by omega
      have g2 : σ 2 = (2 : Pip) := Fin.ext (by omega)
      have g3 : σ 3 = (3 : Pip) := Fin.ext (by omega)
      exact perm_ext7 σ 1 g0 g1 g2 g3 g4 g5 g6
    · right
      have e3 : (σ 3).val = 2 := by omega
      have g2 : σ 2 = (3 : Pip) := Fin.ext (by omega)
      have g3 : σ 3 = (2 : Pip) := Fin.ext (by omega)
      exact perm_ext7 σ (Equiv.swap 2 3) g0 g1 g2 g3 g4 g5 g6
  · rintro (rfl | rfl) <;> (unfold CountPreserving; decide)

/-! ## PA-A17: the scoped `2 ↔ 3` transport -/

/-- The nontrivial count-preserving swap `s = (2 3)`. -/
def swap23 : Equiv.Perm Pip := Equiv.swap 2 3

/-- The swap's domino transport is a bijection on the 28 dominoes. -/
theorem swap23_bijective : Function.Bijective (Domino.mapPips swap23) := by
  decide

/-- PA-A17 (called-set transport): the swap carries the called set of
declaration 2 exactly onto the called set of declaration 3. -/
theorem swap23_called : ∀ d : Domino,
    (Declaration.pip 3).called (d.mapPips swap23)
      ↔ (Declaration.pip 2).called d := by
  decide

/-- PA-A17 (effective-suit transport): effective membership is preserved
under the swap's domino and suit-context transports. -/
theorem swap23_effMem : ∀ (d : Domino) (q : Suit),
    (Declaration.pip 3).effMem (d.mapPips swap23) (q.mapPips swap23)
      ↔ (Declaration.pip 2).effMem d q := by
  decide

/-- PA-A17 (led-context transport): the led context commutes with the
swap's transports — the numeric-`high` subtlety of Math §3.9's proof. -/
theorem swap23_ledSuit : ∀ d : Domino,
    (Declaration.pip 3).ledSuit (d.mapPips swap23)
      = ((Declaration.pip 2).ledSuit d).mapPips swap23 := by
  decide

/-- PA-A17 (count transport): the swap preserves count labels. -/
theorem swap23_countPreserving : CountPreserving swap23 :=
  (countPreserving_iff _).mpr (Or.inr rfl)

/-- PA-A17 (scoped game-order transport, Math §3.9): under the swap's
endpoint and suit-context transports, contextual comparison is preserved
from layer `δ` to layer `δ` transported **exactly when** `δ` is
declaration 2 or declaration 3 — the two directed transports
`𝒢₂ ≅ 𝒢₃` and `𝒢₃ ≅ 𝒢₂`. Every other Straight layer has a reversed pair
(no-trump: `0:2`/`0:3` in context 0; doubles-trump: `2:2`/`3:3` in the
called context; pip `p ∉ {2,3}`: the powered `p:2`/`p:3`). -/
theorem swap23_transport_iff : ∀ δ : Declaration,
    (∀ (q : Suit) (d e : Domino),
      (δ.mapPips swap23).Beats (q.mapPips swap23)
          (d.mapPips swap23) (e.mapPips swap23)
        ↔ δ.Beats q d e)
      ↔ (δ = .pip 2 ∨ δ = .pip 3) := by
  decide

end Texas42
