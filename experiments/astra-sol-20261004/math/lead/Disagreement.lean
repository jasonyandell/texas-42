import Std

/- A finite weighted coupling. No game semantics are assumed implicitly:
   callers must provide the same scenarios, weights, and payoff-coupling law.
   A true flag `d` permits disagreement; false requires equal payoffs. -/
namespace WaltResearch

def mass {S : Type} (w : S → Nat) (p : S → Bool) : List S → Nat
  | [] => 0
  | x :: xs => (if p x then w x else 0) + mass w p xs

theorem payoff_le_disagreement {S : Type} (w : S → Nat)
    (f g d : S → Bool) (xs : List S)
    (coupling : ∀ x ∈ xs, d x = false → f x = g x) :
    mass w f xs ≤ mass w g xs + mass w d xs := by
  induction xs with
  | nil => simp [mass]
  | cons x xs ih =>
    have hx := coupling x (by simp)
    have ht : ∀ y ∈ xs, d y = false → f y = g y := by
      intro y hy
      exact coupling y (by simp [hy])
    have hi := ih ht
    simp only [mass]
    cases hf : f x <;> cases hg : g x <;> cases hd : d x <;>
      simp_all <;> omega

theorem payoff_sandwich {S : Type} (w : S → Nat)
    (f g d : S → Bool) (xs : List S)
    (coupling : ∀ x ∈ xs, d x = false → f x = g x) :
    mass w f xs ≤ mass w g xs + mass w d xs ∧
    mass w g xs ≤ mass w f xs + mass w d xs := by
  constructor
  · exact payoff_le_disagreement w f g d xs coupling
  · apply payoff_le_disagreement w g f d xs
    intro x hx hd
    exact (coupling x hx hd).symm

theorem maximum_transport {P : Type} (f g : P → Nat) (vf vg error : Nat)
    (attained : ∃ p, f p = vf)
    (target_upper : ∀ p, g p ≤ vg)
    (uniform_error : ∀ p, f p ≤ g p + error) : vf ≤ vg + error := by
  obtain ⟨p, hp⟩ := attained
  have hf := uniform_error p
  have hg := target_upper p
  omega

/- Counts may be maxima over any SAME family of lawful focal policies.
   To instantiate this theorem for best-response values the two error bounds
   must hold for every policy, not just the current incumbent's replay. -/
theorem strict_action_transport (oldA oldB newA newB errA errB : Nat)
    (lowerA : oldA ≤ newA + errA)
    (upperB : newB ≤ oldB + errB)
    (gap : oldB + errA + errB < oldA) : newB < newA := by
  omega

theorem weak_action_transport (oldA oldB newA newB errA errB : Nat)
    (lowerA : oldA ≤ newA + errA)
    (upperB : newB ≤ oldB + errB)
    (gap : oldB + errA + errB ≤ oldA) : newB ≤ newA := by
  omega

/- Exact interval pruning: a later tile can also be discarded at equality.
   An earlier tile requires strict separation to preserve least-tile ties. -/
theorem canonical_interval_pruning (a b va vb lowerA upperB : Nat)
    (la : lowerA ≤ va) (ub : vb ≤ upperB)
    (separated : upperB < lowerA ∨ (upperB ≤ lowerA ∧ a < b)) :
    vb < va ∨ (vb ≤ va ∧ a < b) := by
  omega

#print axioms payoff_le_disagreement
#print axioms payoff_sandwich
#print axioms maximum_transport
#print axioms strict_action_transport
#print axioms weak_action_transport
#print axioms canonical_interval_pruning
end WaltResearch
