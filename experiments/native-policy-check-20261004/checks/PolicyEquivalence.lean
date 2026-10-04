import Std
set_option autoImplicit false
namespace WaltResearch.NativePolicyCheck
/- Finite conditional refinement. S must contain the complete actor information
   state and immutable policy context. next must preserve ordered sample IDs,
   their multiplicity, and policy-selected branches. combine contains exactly
   the common nature SUM/focal MAX or MIN and terminal settlement semantics.
   These assumptions are not proved about Rust, RNG, the wire or deadlines. -/
def evaluate {S : Type} (terminal : S → Nat) (next : S → List S)
    (combine : S → List Nat → Nat) : Nat → S → Nat
  | 0, s => terminal s
  | h + 1, s => combine s ((next s).map (evaluate terminal next combine h))

theorem finite_bellman_refinement {S : Type} (terminalA terminalB : S → Nat)
    (nextA nextB : S → List S) (combineA combineB : S → List Nat → Nat)
    (term : ∀ s, terminalA s = terminalB s)
    (branches : ∀ s, nextA s = nextB s)
    (folds : ∀ s xs, combineA s xs = combineB s xs) :
    ∀ h s, evaluate terminalA nextA combineA h s = evaluate terminalB nextB combineB h s := by
  intro h
  induction h with
  | zero => exact term
  | succ h ih =>
    intro s
    simp only [evaluate]
    rw [branches s, folds s]
    congr 1
    exact List.map_congr_left (fun t _ => ih t)

theorem ordered_mass_preserved {W : Type} (rows : List W)
    (native wasm : W → Nat) (same : ∀ w ∈ rows, native w = wasm w) :
    (rows.map native).sum = (rows.map wasm).sum := by
  induction rows with
  | nil => rfl
  | cons w ws ih =>
    have hw := same w (by simp)
    have hws := ih (fun x hx => same x (by simp [hx]))
    simp [hw, hws]

theorem duplicate_mass (x y : Nat) : ([x, x, y] : List Nat).sum = 2*x+y := by
  simp; omega

theorem full_cache_key_refinement {V K A : Type} (encode : V → K)
    (decode : K → V) (lossless : ∀ v, decode (encode v) = v)
    (policy : V → A) (v : V) : policy (decode (encode v)) = policy v := by
  rw [lossless]

#print axioms finite_bellman_refinement
#print axioms ordered_mass_preserved
#print axioms duplicate_mass
#print axioms full_cache_key_refinement
end WaltResearch.NativePolicyCheck
