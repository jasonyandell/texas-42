import Std
set_option autoImplicit false

/- Conditional representation lemmas. Public transition is independent of the
   world, but private view, stochastic draw and information-set grouping are
   NOT assumed independent of it. This is not a Rust refinement proof. -/
namespace WaltResearch.PrefixState
variable {Q S A W V : Type}

def replay (root : Q → S) (step : Q → S → A → S) (q : Q) (history : List A) : S :=
  history.foldl (step q) (root q)

structure Row (Q S A W : Type) where
  query : Q
  state : S
  history : List A
  world : W

def Consistent (root : Q → S) (step : Q → S → A → S) (r : Row Q S A W) : Prop :=
  r.state = replay root step r.query r.history

def advance (step : Q → S → A → S) (r : Row Q S A W) (tile : A) : Row Q S A W :=
  { r with state := step r.query r.state tile, history := r.history ++ [tile] }

theorem replay_child (root : Q → S) (step : Q → S → A → S) (q : Q)
    (history : List A) (tile : A) :
    replay root step q (history ++ [tile]) = step q (replay root step q history) tile := by
  simp [replay, List.foldl_append]

theorem advance_consistent (root : Q → S) (step : Q → S → A → S)
    (r : Row Q S A W) (tile : A) (h : Consistent root step r) :
    Consistent root step (advance step r tile) := by
  unfold Consistent at h ⊢
  simp only [advance, replay_child]
  exact congrArg (fun state => step r.query state tile) h

theorem one_state_per_coordinate (root : Q → S) (step : Q → S → A → S)
    (r t : Row Q S A W) (hr : Consistent root step r) (ht : Consistent root step t)
    (hq : r.query = t.query) (hp : r.history = t.history) : r.state = t.state := by
  calc
    r.state = replay root step r.query r.history := hr
    _ = replay root step t.query t.history := by rw [hq, hp]
    _ = t.state := ht.symm

theorem advance_preserves_world (step : Q → S → A → S) (r : Row Q S A W) (tile : A) :
    (advance step r tile).world = r.world := rfl

theorem actor_view_preserved (root : Q → S) (step : Q → S → A → S)
    (view : W → V) (policy : Q → List A → S → V → A) (r : Row Q S A W)
    (h : Consistent root step r) :
    policy r.query r.history r.state (view r.world) =
      policy r.query r.history (replay root step r.query r.history) (view r.world) := by
  exact congrArg (fun state => policy r.query r.history state (view r.world)) h

-- Duplicates remain separate entries and therefore retain their unit mass.
-- This equality holds for ANY fixed payoff evaluator/action. Applying one
-- deterministic MAX/MIN/tie rule after these sums therefore preserves it.
theorem shared_sum_preserved (root : Q → S) (step : Q → S → A → S)
    (value : Q → List A → S → W → Nat) (rows : List (Row Q S A W))
    (h : ∀ r ∈ rows, Consistent root step r) :
    (rows.map (fun r => value r.query r.history r.state r.world)).sum =
      (rows.map (fun r => value r.query r.history (replay root step r.query r.history) r.world)).sum := by
  have eq : rows.map (fun r => value r.query r.history r.state r.world) =
      rows.map (fun r => value r.query r.history (replay root step r.query r.history) r.world) := by
    exact List.map_congr_left (fun r hr => congrArg (fun state => value r.query r.history state r.world) (h r hr))
  exact congrArg List.sum eq

#print axioms replay_child
#print axioms advance_consistent
#print axioms one_state_per_coordinate
#print axioms advance_preserves_world
#print axioms actor_view_preserved
#print axioms shared_sum_preserved
end WaltResearch.PrefixState
