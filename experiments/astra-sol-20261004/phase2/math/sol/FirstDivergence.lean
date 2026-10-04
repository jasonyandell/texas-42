import Init

/- EXPLORATORY finite-horizon replay theorem. State may include a fixed physical
world and complete public history. Focal decisions only receive Observation.
Certificate expands every legal focal action and only the cheap field's edge.
This file does not derive Texas 42 mechanics or completion from a timed program.
-/
namespace WaltResearch.Phase2.Sol

structure Model where
  State : Type
  Action : Type
  Observation : Type
  settled : State → Bool
  focal : State → Bool
  legal : State → List Action
  next : State → Action → State
  observe : State → Observation
  payoff : State → Bool

def prepend (M : Model) (a : M.Action)
    (outcome : List M.Action × Bool) : List M.Action × Bool :=
  (a :: outcome.1, outcome.2)

def replay (M : Model) (field : M.State → M.Action)
    (policy : M.Observation → M.Action) : Nat → M.State → List M.Action × Bool
  | 0, s => ([], M.payoff s)
  | n + 1, s =>
    if M.settled s then ([], M.payoff s)
    else
      let a := if M.focal s then policy (M.observe s) else field s
      prepend M a (replay M field policy n (M.next s a))

def Certificate (M : Model) (cheap target : M.State → M.Action) :
    Nat → M.State → Prop
  | 0, _ => True
  | n + 1, s =>
    if M.settled s then True
    else if M.focal s then
      ∀ a ∈ M.legal s, Certificate M cheap target n (M.next s a)
    else
      cheap s = target s ∧ Certificate M cheap target n (M.next s (cheap s))

def Lawful (M : Model) (policy : M.Observation → M.Action) : Prop :=
  ∀ s, M.settled s = false → M.focal s = true →
    policy (M.observe s) ∈ M.legal s

theorem certified_complete_replay_equal (M : Model)
    (cheap target : M.State → M.Action) (policy : M.Observation → M.Action)
    (lawful : Lawful M policy) (n : Nat) (s : M.State)
    (certified : Certificate M cheap target n s) :
    replay M cheap policy n s = replay M target policy n s := by
  induction n generalizing s with
  | zero => rfl
  | succ n ih =>
    cases hs : M.settled s with
    | true => simp [replay, hs]
    | false =>
      cases hf : M.focal s with
      | true =>
        have hc : ∀ a ∈ M.legal s, Certificate M cheap target n (M.next s a) := by
          simpa [Certificate, hs, hf] using certified
        have ha := lawful s hs hf
        have heq := ih (M.next s (policy (M.observe s))) (hc _ ha)
        simpa [replay, hs, hf] using congrArg (prepend M (policy (M.observe s))) heq
      | false =>
        have hc : cheap s = target s ∧
            Certificate M cheap target n (M.next s (cheap s)) := by
          simpa [Certificate, hs, hf] using certified
        have heq := ih (M.next s (cheap s)) hc.2
        simpa [replay, hs, hf, ← hc.1] using congrArg (prepend M (cheap s)) heq

theorem certified_payoff_equal (M : Model)
    (cheap target : M.State → M.Action) (policy : M.Observation → M.Action)
    (lawful : Lawful M policy) (n : Nat) (s : M.State)
    (certified : Certificate M cheap target n s) :
    (replay M cheap policy n s).2 = (replay M target policy n s).2 := by
  exact congrArg Prod.snd
    (certified_complete_replay_equal M cheap target policy lawful n s certified)

theorem unequal_payoff_requires_uncertified (M : Model)
    (cheap target : M.State → M.Action) (policy : M.Observation → M.Action)
    (lawful : Lawful M policy) (n : Nat) (s : M.State)
    (different : (replay M cheap policy n s).2 ≠ (replay M target policy n s).2) :
    ¬ Certificate M cheap target n s := by
  intro certified
  exact different (certified_payoff_equal M cheap target policy lawful n s certified)

-- Operational callers can pessimistically flag refused/partial worlds.
-- All unflagged worlds must have completed the full certificate obligation.
theorem unflagged_world_coupling (M : Model)
    (cheap target : M.State → M.Action) (policy : M.Observation → M.Action)
    (lawful : Lawful M policy) (n : Nat) (roots : List M.State)
    (bad : M.State → Bool)
    (completed : ∀ s ∈ roots, bad s = false → Certificate M cheap target n s) :
    ∀ s ∈ roots, bad s = false →
      (replay M cheap policy n s).2 = (replay M target policy n s).2 := by
  intro s hs hb
  exact certified_payoff_equal M cheap target policy lawful n s (completed s hs hb)

#print axioms certified_complete_replay_equal
#print axioms certified_payoff_equal
#print axioms unequal_payoff_requires_uncertified
#print axioms unflagged_world_coupling

end WaltResearch.Phase2.Sol
