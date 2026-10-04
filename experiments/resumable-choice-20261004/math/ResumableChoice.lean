import Std
set_option autoImplicit false
namespace WaltResearch.ResumableChoice
/- Conditional arithmetic and representation laws only. These premises do not
   establish Rust scheduling, lawful actor views, sampler/RNG equivalence, cache
   completeness, overflow safety, privacy, or complexity. -/
theorem nature_delta_exact (siblings oldChild newChild : Nat) :
    (siblings + oldChild) - oldChild + newChild = siblings + newChild := by omega

theorem max_lower_incremental (siblings oldChild newChild : Nat)
    (monotone : oldChild ≤ newChild) :
    max (max siblings oldChild) newChild = max siblings newChild := by omega

theorem min_upper_incremental (siblings oldChild newChild : Nat)
    (monotone : newChild ≤ oldChild) :
    min (min siblings oldChild) newChild = min siblings newChild := by omega

theorem max_upper_unchanged (siblings oldChild newChild : Nat)
    (decrease : newChild ≤ oldChild) (notTop : oldChild < max siblings oldChild) :
    max siblings newChild = max siblings oldChild := by omega

theorem min_lower_unchanged (siblings oldChild newChild : Nat)
    (increase : oldChild ≤ newChild) (notBottom : min siblings oldChild < oldChild) :
    min siblings newChild = min siblings oldChild := by omega

theorem unchanged_ancestor {B A : Type} (fold : B → A) (oldBound newBound : B)
    (same : oldBound = newBound) : fold oldBound = fold newBound := congrArg fold same

theorem completed_choice_recycling {K A T : Type} (cache : K → Option A)
    (key : K) (choice : A) (tree : T) (saved : cache key = some choice) :
    (fun (_ : T) => cache key) tree = some choice := saved

#print axioms nature_delta_exact
#print axioms max_lower_incremental
#print axioms min_upper_incremental
#print axioms max_upper_unchanged
#print axioms min_lower_unchanged
#print axioms unchanged_ancestor
#print axioms completed_choice_recycling
end WaltResearch.ResumableChoice
