import Std

/- A student receives one constant observation. Safe pays 3 in each hidden
   world. Risk requires a later guess with the SAME observation and pays 4 iff
   correct. Values below are sums over the two equally weighted hidden worlds.
   They are twice expectations, so no floating point/rational axioms are used.
   This concerns clairvoyant continuation labels, NOT fixed lawful rollouts. -/
namespace WaltResearch.Phase2.Lead

def riskPayoff (world guess : Bool) : Nat := if world == guess then 4 else 0
def lawfulRisk (policy : Unit → Bool) : Nat :=
  riskPayoff false (policy ()) + riskPayoff true (policy ())
def safeValue : Nat := 3 + 3
def oracleRisk : Nat := riskPayoff false false + riskPayoff true true

theorem lawful_risk_exact (policy : Unit → Bool) : lawfulRisk policy = 4 := by
  unfold lawfulRisk
  cases policy () <;> simp [riskPayoff]

theorem oracle_label_ranks_risk : safeValue < oracleRisk := by decide

theorem every_lawful_policy_prefers_safe (policy : Unit → Bool) :
    lawfulRisk policy < safeValue := by
  rw [lawful_risk_exact]
  decide

-- Exact constant-input regression to these two oracle labels ranks Risk over
-- Safe, despite every information-measurable continuation doing worse there.
theorem exact_oracle_student_is_wrong (policy : Unit → Bool)
    (predSafe predRisk : Nat) (hs : predSafe = safeValue) (hr : predRisk = oracleRisk) :
    predSafe < predRisk ∧ lawfulRisk policy < safeValue := by
  constructor
  · simpa [hs, hr] using oracle_label_ranks_risk
  · exact every_lawful_policy_prefers_safe policy

-- Legal one-step guesses; training world weights (3,1) and deployment weights
-- (1,3) are both supported on the same two worlds. Lawful inputs alone do not
-- correct that posterior shift. The teacher here is not clairvoyant.
def weightedGuess (wFalse wTrue : Nat) (guess : Bool) : Nat :=
  wFalse * (if guess == false then 1 else 0) +
  wTrue * (if guess == true then 1 else 0)

theorem sampling_distribution_can_reverse_ranking :
    weightedGuess 3 1 true < weightedGuess 3 1 false ∧
    weightedGuess 1 3 false < weightedGuess 1 3 true := by decide

-- Absolute errors for the selected and optimal action imply a 2-epsilon
-- regret bound. This requires action-value accuracy; outcome classification
-- accuracy by itself supplies none of these hypotheses.
theorem chosen_action_regret_bound (qBest qChosen pBest pChosen error : Nat)
    (bestError : qBest ≤ pBest + error)
    (chosenError : pChosen ≤ qChosen + error)
    (greedy : pBest ≤ pChosen) : qBest ≤ qChosen + 2 * error := by omega

#print axioms lawful_risk_exact
#print axioms oracle_label_ranks_risk
#print axioms every_lawful_policy_prefers_safe
#print axioms exact_oracle_student_is_wrong
#print axioms sampling_distribution_can_reverse_ranking
#print axioms chosen_action_regret_bound

end WaltResearch.Phase2.Lead
