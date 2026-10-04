import Std

/- An outer focal player knows hidden bit x=false. A modeled opponent has
   one constant observation and does not know x. Its own belief contains four
   equally weighted worlds: one x=false, three x=true. Outer mechanical support
   conditioned on the focal information contains no x=true world. Optimizing
   the modeled actor over that outer support silently grants focal information.
   This is an abstract coverage obstacle, not a concrete Texas42 deal. -/
namespace WaltResearch.Phase3.Sol

def guessCount (falseMass trueMass : Nat) (guess : Bool) : Nat :=
  if guess then trueMass else falseMass

theorem outer_restriction_reverses_modeled_response :
    guessCount 1 0 true < guessCount 1 0 false ∧
    guessCount 1 3 false < guessCount 1 3 true := by decide

theorem outer_support_omits_inner_world :
    guessCount 1 0 true = 0 ∧ 0 < guessCount 1 3 true := by decide

#print axioms outer_restriction_reverses_modeled_response
#print axioms outer_support_omits_inner_world

end WaltResearch.Phase3.Sol
