import Init

/-
Exploratory abstract finite-model obstruction, not a Texas 42 theorem.
Two hidden types have masses n and n+1. The field wins by naming the
type, so its exact action values have denominator 2*n+1. An approximate
value table exchanges the two numerators (error 1/(2*n+1) per action).
The host conditions on the first type and wins exactly when the field errs.
All policies below are constant lawful policies; no hidden-type lookup enters
either field choice. Ratios are represented by exact numerator/denominator.
-/
namespace WaltResearch.Sol

def denominator (n : Nat) : Nat := 2 * n + 1

def exactMass (n : Nat) (action : Bool) : Nat :=
  if action then n else n + 1

def approximateMass (n : Nat) (action : Bool) : Nat :=
  if action then n + 1 else n

-- Canonical false-first tie rule.
def choose (falseValue trueValue : Nat) : Bool :=
  if falseValue < trueValue then true else false

def exactField (n : Nat) : Bool :=
  choose (exactMass n false) (exactMass n true)

def approximateField (n : Nat) : Bool :=
  choose (approximateMass n false) (approximateMass n true)

def hostSuccess (actualType fieldAction : Bool) : Nat :=
  if actualType = fieldAction then 0 else 1

theorem exact_field_false (n : Nat) : exactField n = false := by
  simp [exactField, exactMass, choose]

theorem approximate_field_true (n : Nat) : approximateField n = true := by
  simp [approximateField, approximateMass, choose]

-- Every action-table error is one unit of mass, in one direction or the other.
theorem action_error_one (n : Nat) (a : Bool) :
    exactMass n a + 1 = approximateMass n a ∨
    approximateMass n a + 1 = exactMass n a := by
  cases a <;> simp [exactMass, approximateMass]

-- The approximate policy's regret under the modeled belief is exactly 1/D.
theorem modeled_regret_one (n : Nat) :
    exactMass n (exactField n) = exactMass n (approximateField n) + 1 := by
  simp [exact_field_false, approximate_field_true, exactMass]

-- Under the host's conditional belief the same policy change loses all success.
theorem conditional_host_drop (n : Nat) :
    hostSuccess true (exactField n) = 1 ∧
    hostSuccess true (approximateField n) = 0 := by
  simp [exact_field_false, approximate_field_true, hostSuccess]

-- Cross multiplication: error 1/D is strictly below 1/m whenever 0<m<D.
-- Hence the value-error family is arbitrarily small without its outer loss shrinking.
theorem arbitrarily_small_error_full_drop (m : Nat) (_hm : 0 < m) :
    ∃ n : Nat, 0 < denominator n ∧ m < denominator n ∧
      (∀ a : Bool, exactMass n a + 1 = approximateMass n a ∨
        approximateMass n a + 1 = exactMass n a) ∧
      exactMass n (exactField n) = exactMass n (approximateField n) + 1 ∧
      hostSuccess true (exactField n) = 1 ∧
      hostSuccess true (approximateField n) = 0 := by
  refine ⟨m, ?_, ?_, action_error_one m, modeled_regret_one m,
    (conditional_host_drop m).1, (conditional_host_drop m).2⟩
  · exact Nat.zero_lt_succ _
  · have hle : m ≤ 2 * m := by
      calc
        m ≤ m + m := Nat.le_add_right m m
        _ = 2 * m := by simp [Nat.succ_mul]
    exact Nat.lt_succ_of_le hle

#print axioms exact_field_false
#print axioms approximate_field_true
#print axioms action_error_one
#print axioms modeled_regret_one
#print axioms conditional_host_drop
#print axioms arbitrarily_small_error_full_drop

end WaltResearch.Sol
