import Std

/- Exploratory local realization and policy-family distinction.
   Removed cards are never legal again. A deterministic fixed world/tape
   supplies a unique public continuation after each selected action.
   The latter transition induction is not a Texas42 engine refinement here. -/
namespace AdversarialSol.Priority

def firstLegal {A : Type} (legal : A → Bool) : List A → Option A
  | [] => none
  | a :: rest => if legal a then some a else firstLegal legal rest

-- Prioritize the desired actual sequence chronologically: already-played
-- prefix cards are unavailable, desired next card legal, future cards later.
theorem chronological_priority_selects_next {A : Type} (legal : A → Bool)
    (removed suffix : List A) (next : A)
    (past_unavailable : ∀ a ∈ removed, legal a = false)
    (next_legal : legal next = true) :
    firstLegal legal (removed ++ next :: suffix) = some next := by
  induction removed with
  | nil => simp [firstLegal, next_legal]
  | cons a rest ih =>
    have ha := past_unavailable a (by simp)
    have hr : ∀ b ∈ rest, legal b = false := by
      intro b hb
      exact past_unavailable b (by simp [hb])
    simpa [firstLegal, ha] using ih hr

-- Any fixed priority list at these TWO PUBLICLY DISTINCT observations picks
-- one constant action because the remaining own hand/legal set is identical.
-- An observation-adaptive policy can choose differently without hidden input.
def reward (observation action : Bool) : Nat := if observation == action then 1 else 0
def commonPriorityValue (action : Bool) : Nat := reward false action + reward true action
def adaptiveValue : Nat := reward false false + reward true true

theorem every_common_first_action_has_value_one (action : Bool) :
    commonPriorityValue action = 1 := by
  cases action <;> decide

theorem adaptive_public_policy_strictly_beats_common_order (action : Bool) :
    commonPriorityValue action < adaptiveValue := by
  rw [every_common_first_action_has_value_one]
  decide

theorem any_shared_priority_list_is_incomplete (order : List Bool) (action : Bool)
    (_chosen : firstLegal (fun _ => true) order = some action) :
    commonPriorityValue action < adaptiveValue :=
  adaptive_public_policy_strictly_beats_common_order action

#print axioms chronological_priority_selects_next
#print axioms every_common_first_action_has_value_one
#print axioms adaptive_public_policy_strictly_beats_common_order
#print axioms any_shared_priority_list_is_incomplete
end AdversarialSol.Priority
