/-
Copyright (c) 2026 Jason Yandell. All rights reserved.
Released under Apache 2.0 license as described in the file LICENSE.
Authors: Jason Yandell
-/
import Texas42.Trick1PerfectRecallNet.LineageNetting

/-!
# Complete legal faces and the sum-before-max recurrence

The central result identifies independent state-local maximization with the
free product of lawful perfect-recall policies.  Arrival contributions are
summed inside `actionContribution` before any state maximum is taken.
-/

namespace Texas42
namespace Trick1PerfectRecallNet

/-- A complete nonempty legal face retained at one focal information state. -/
structure LegalFace (Action : Type*) [DecidableEq Action] where
  actions : Finset Action
  nonempty : actions.Nonempty

/-- A legal singleton remains a real face rather than disappearing. -/
theorem singleton_face_retained {Action : Type*} [DecidableEq Action]
    (action : Action) :
    (LegalFace.mk {action} (by simp)).actions.card = 1 := by
  simp

/--
Unnormalized action recurrence: terminal contribution plus every completely
netted child value (M3 contract §4; GT1-A21).
-/
def actionContribution {Child : Type*} [DecidableEq Child]
    (terminal : ℤ) (children : Finset Child) (childValue : Child → ℤ) : ℤ :=
  terminal + ∑ child ∈ children, childValue child

/-- The recurrence adds all children before exposing an action value. -/
theorem actionContribution_eq_sum_before_max {Child : Type*}
    [DecidableEq Child] (terminal : ℤ) (children : Finset Child)
    (childValue : Child → ℤ) :
    actionContribution terminal children childValue =
      terminal + ∑ child ∈ children, childValue child := rfl

/-- Exact state value after the complete legal face is known. -/
def stateValue {Action : Type*} [DecidableEq Action]
    (face : LegalFace Action) (actionValue : Action → ℤ) : ℤ :=
  face.actions.sup' face.nonempty actionValue

/-- Every legal action is bounded by the retained state maximum. -/
theorem legal_action_le_stateValue {Action : Type*} [DecidableEq Action]
    (face : LegalFace Action) (actionValue : Action → ℤ)
    {action : Action} (hlegal : action ∈ face.actions) :
    actionValue action ≤ stateValue face actionValue := by
  exact Finset.le_sup' actionValue hlegal

/-- A full argmax face retains all ties, not only its least representative. -/
def argmaxFace {Action : Type*} [DecidableEq Action]
    (face : LegalFace Action) (actionValue : Action → ℤ) : Finset Action :=
  face.actions.filter fun action =>
    actionValue action = stateValue face actionValue

/-- The retained argmax face is nonempty. -/
theorem argmaxFace_nonempty {Action : Type*} [DecidableEq Action]
    (face : LegalFace Action) (actionValue : Action → ℤ) :
    (argmaxFace face actionValue).Nonempty := by
  obtain ⟨action, hlegal, hmax⟩ :=
    face.actions.exists_mem_eq_sup' face.nonempty actionValue
  refine ⟨action, ?_⟩
  exact Finset.mem_filter.mpr ⟨hlegal, hmax.symm⟩

/-- A perfect-recall policy selects one action independently at every key. -/
abbrev PerfectRecallPolicy (State Action : Type*) := State → Action

/-- Lawfulness is exactly membership in each state's complete legal face. -/
def LawfulPolicy {State Action : Type*} [DecidableEq Action]
    (face : State → LegalFace Action)
    (policy : PerfectRecallPolicy State Action) : Prop :=
  ∀ state, policy state ∈ (face state).actions

/-- Value of one policy over a finite family of independent information states. -/
def policyValue {State Action : Type*} [DecidableEq State]
    (states : Finset State) (payoff : State → Action → ℤ)
    (policy : PerfectRecallPolicy State Action) : ℤ :=
  ∑ state ∈ states, payoff state (policy state)

/-- State-local sum-before-max value. -/
def sumBeforeMax {State Action : Type*}
    [DecidableEq State] [DecidableEq Action]
    (states : Finset State) (face : State → LegalFace Action)
    (payoff : State → Action → ℤ) : ℤ :=
  ∑ state ∈ states, stateValue (face state) (payoff state)

/-- Every lawful free-product policy is bounded by sum-before-max. -/
theorem lawful_policy_le_sumBeforeMax {State Action : Type*}
    [DecidableEq State] [DecidableEq Action]
    (states : Finset State) (face : State → LegalFace Action)
    (payoff : State → Action → ℤ)
    (policy : PerfectRecallPolicy State Action)
    (hlawful : LawfulPolicy face policy) :
    policyValue states payoff policy ≤ sumBeforeMax states face payoff := by
  unfold policyValue sumBeforeMax
  apply Finset.sum_le_sum
  intro state hstate
  exact legal_action_le_stateValue (face state) (payoff state)
    (hlawful state)

/-- Choose one maximizing member of every complete legal face. -/
noncomputable def maximizingPolicy {State Action : Type*}
    [DecidableEq Action] (face : State → LegalFace Action)
    (payoff : State → Action → ℤ) : PerfectRecallPolicy State Action :=
  fun state => Classical.choose
    ((face state).actions.exists_mem_eq_sup' (face state).nonempty (payoff state))

/-- The pointwise maximizing policy is lawful. -/
theorem maximizingPolicy_lawful {State Action : Type*}
    [DecidableEq Action] (face : State → LegalFace Action)
    (payoff : State → Action → ℤ) :
    LawfulPolicy face (maximizingPolicy face payoff) := by
  intro state
  exact (Classical.choose_spec
    ((face state).actions.exists_mem_eq_sup' (face state).nonempty
      (payoff state))).1

/-- The pointwise maximizing policy attains sum-before-max exactly. -/
theorem maximizingPolicy_value {State Action : Type*}
    [DecidableEq State] [DecidableEq Action]
    (states : Finset State) (face : State → LegalFace Action)
    (payoff : State → Action → ℤ) :
    policyValue states payoff (maximizingPolicy face payoff) =
      sumBeforeMax states face payoff := by
  unfold policyValue sumBeforeMax maximizingPolicy stateValue
  apply Finset.sum_congr rfl
  intro state hstate
  exact (Classical.choose_spec
    ((face state).actions.exists_mem_eq_sup' (face state).nonempty
      (payoff state))).2.symm

/--
Sum-before-max is exactly the maximum over the free product of lawful
perfect-recall policies: it is an upper bound and one lawful policy attains it.
-/
theorem sumBeforeMax_eq_freeProduct {State Action : Type*}
    [DecidableEq State] [DecidableEq Action]
    (states : Finset State) (face : State → LegalFace Action)
    (payoff : State → Action → ℤ) :
    ∃ policy : PerfectRecallPolicy State Action,
      LawfulPolicy face policy ∧
      policyValue states payoff policy = sumBeforeMax states face payoff ∧
      ∀ other, LawfulPolicy face other →
        policyValue states payoff other ≤ policyValue states payoff policy := by
  refine ⟨maximizingPolicy face payoff, maximizingPolicy_lawful face payoff,
    maximizingPolicy_value states face payoff, ?_⟩
  intro other hother
  rw [maximizingPolicy_value states face payoff]
  exact lawful_policy_le_sumBeforeMax states face payoff other hother

/-- Repricing a selected full-argmax member reproduces the state value. -/
theorem argmax_reprice_eq_stateValue {Action : Type*} [DecidableEq Action]
    (face : LegalFace Action) (actionValue : Action → ℤ)
    {selected : Action} (hselected : selected ∈ argmaxFace face actionValue) :
    actionValue selected = stateValue face actionValue := by
  simpa [argmaxFace] using (Finset.mem_filter.mp hselected).2

end Trick1PerfectRecallNet
end Texas42
