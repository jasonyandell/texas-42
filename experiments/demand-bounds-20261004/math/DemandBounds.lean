import Std
set_option autoImplicit false

/- Conditional arithmetic guarantees, not a Rust refinement or a privacy proof.
   Bounds must cover the exact shared-group totals from a lawful fixed sampler. -/
namespace WaltResearch.DemandBounds
variable {n : Nat}

def MaxCertificate (lo hi : Fin n → Nat) (a : Fin n) : Prop :=
  (∀ b, b < a → hi b < lo a) ∧ (∀ b, a < b → hi b ≤ lo a)
def MinCertificate (lo hi : Fin n → Nat) (a : Fin n) : Prop :=
  (∀ b, b < a → hi a < lo b) ∧ (∀ b, a < b → hi a ≤ lo b)

theorem max_choice_sound (lo hi value : Fin n → Nat) (a : Fin n)
    (covered : ∀ b, lo b ≤ value b ∧ value b ≤ hi b)
    (cert : MaxCertificate lo hi a) :
    (∀ b, b < a → value b < value a) ∧ (∀ b, value b ≤ value a) := by
  constructor
  · intro b h; have hb := covered b; have ha := covered a; have hc := cert.1 b h; omega
  · intro b
    by_cases h : b < a
    · have hb := covered b; have ha := covered a; have hc := cert.1 b h; omega
    · by_cases h' : a < b
      · have hb := covered b; have ha := covered a; have hc := cert.2 b h'; omega
      · have eq : b = a := Fin.ext (by omega); rw [eq]; exact Nat.le_refl _

theorem min_choice_sound (lo hi value : Fin n → Nat) (a : Fin n)
    (covered : ∀ b, lo b ≤ value b ∧ value b ≤ hi b)
    (cert : MinCertificate lo hi a) :
    (∀ b, b < a → value a < value b) ∧ (∀ b, value a ≤ value b) := by
  constructor
  · intro b h; have hb := covered b; have ha := covered a; have hc := cert.1 b h; omega
  · intro b
    by_cases h : b < a
    · have hb := covered b; have ha := covered a; have hc := cert.1 b h; omega
    · by_cases h' : a < b
      · have hb := covered b; have ha := covered a; have hc := cert.2 b h'; omega
      · have eq : b = a := Fin.ext (by omega); rw [eq]; exact Nat.le_refl _

theorem sum_bounds {W : Type} (rows : List W) (lo hi value : W → Nat)
    (covered : ∀ w ∈ rows, lo w ≤ value w ∧ value w ≤ hi w) :
    (rows.map lo).sum ≤ (rows.map value).sum ∧
    (rows.map value).sum ≤ (rows.map hi).sum := by
  induction rows with
  | nil => simp
  | cons w ws ih =>
    have hw := covered w (by simp)
    have hws := ih (fun x hx => covered x (by simp [hx]))
    simp only [List.map_cons, List.sum_cons]; omega

theorem unit_mass_bounds {W : Type} (rows : List W) (value : W → Nat)
    (binary : ∀ w ∈ rows, value w ≤ 1) :
    0 ≤ (rows.map value).sum ∧ (rows.map value).sum ≤ rows.length := by
  constructor
  · exact Nat.zero_le _
  · induction rows with
    | nil => simp
    | cons w ws ih =>
      have hw := binary w (by simp)
      have hws := ih (fun x hx => binary x (by simp [hx]))
      simp only [List.map_cons, List.sum_cons, List.length_cons]; omega

theorem pair_max_bounds (al ah av bl bh bv : Nat)
    (a : al ≤ av ∧ av ≤ ah) (b : bl ≤ bv ∧ bv ≤ bh) :
    max al bl ≤ max av bv ∧ max av bv ≤ max ah bh := by omega

theorem pair_min_bounds (al ah av bl bh bv : Nat)
    (a : al ≤ av ∧ av ≤ ah) (b : bl ≤ bv ∧ bv ≤ bh) :
    min al bl ≤ min av bv ∧ min av bv ≤ min ah bh := by omega

theorem pure_view_preserved {V A : Type} (policy : V → A) (v w : V) (same : v = w) :
    policy v = policy w := congrArg policy same

#print axioms max_choice_sound
#print axioms min_choice_sound
#print axioms sum_bounds
#print axioms unit_mass_bounds
#print axioms pair_max_bounds
#print axioms pair_min_bounds
#print axioms pure_view_preserved
end WaltResearch.DemandBounds
