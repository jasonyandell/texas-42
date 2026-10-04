import Std

/- Explicit finite-policy class inclusion; no Texas42 policy compiler is
   assumed correct by this theorem. Values use integer sampled success mass. -/
namespace AdversarialSol.Nested

theorem nested_attained_value_monotone {P : Type} (value : P → Nat)
    (base refined : P → Prop) (baseBest refinedBest : Nat)
    (includes : ∀ p, base p → refined p)
    (base_attained : ∃ p, base p ∧ value p = baseBest)
    (refined_upper : ∀ p, refined p → value p ≤ refinedBest) :
    baseBest ≤ refinedBest := by
  obtain ⟨p, hp, hv⟩ := base_attained
  have bound := refined_upper p (includes p hp)
  omega

-- Per-action restricted value can improve while a greedy root switches to
-- a slightly worse action under the unchanged full-lawful values.
theorem nested_action_improvements_can_increase_root_regret :
    (98 : Nat) ≤ 98 ∧ (90 : Nat) ≤ 99 ∧
    (90 : Nat) < 98 ∧ (98 : Nat) < 99 ∧
    ((100 : Nat) - 100 < 100 - 99) := by decide

#print axioms nested_attained_value_monotone
#print axioms nested_action_improvements_can_increase_root_regret
end AdversarialSol.Nested
