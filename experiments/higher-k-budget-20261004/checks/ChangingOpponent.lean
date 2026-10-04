import Std
set_option autoImplicit false

/- A generic two-action, two-opponent finite payoff example. This is not a
   Texas42 witness and makes no claim about any measured rung k or Rust solver.
   It isolates why exact best response does not imply improving performance
   against an unchanged reference after the modeled opponent changes. -/
namespace WaltResearch.ChangingOpponent

def payoff (action opponent : Bool) : Nat :=
  if action == opponent then 100 else 0

def BestResponse (action opponent : Bool) : Prop :=
  ∀ other : Bool, payoff other opponent ≤ payoff action opponent

theorem exact_best_responses_can_worsen_fixed_reference :
    BestResponse false false ∧ BestResponse true true ∧
    payoff true false < payoff false false := by
  constructor
  · intro other
    cases other <;> decide
  · constructor
    · intro other
      cases other <;> decide
    · decide

#print axioms exact_best_responses_can_worsen_fixed_reference
end WaltResearch.ChangingOpponent
