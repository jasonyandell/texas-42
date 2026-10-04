import Std

/- Abstract finite DAG evaluation, conditional on a correct lawful graph.
   Node i refers only to earlier nodes through Fin i. SUM is a nonfocal
   bucket fold; MAX/MIN must cover shared focal actions, not hidden worlds.
   Cache below is an abstract persistent indexed store, not a timing model. -/
namespace WaltResearch.Phase3.Sol

inductive Node (i : Nat) where
  | leaf (mass : Nat)
  | sum (children : List (Fin i))
  | maximum (children : List (Fin i))
  | minimum (ceiling : Nat) (children : List (Fin i))

def Node.fold {i : Nat} (node : Node i) (values : Fin i → Nat) : Nat :=
  match node with
  | .leaf mass => mass
  | .sum children => (children.map values).sum
  | .maximum children => (children.map values).foldl Nat.max 0
  | .minimum ceiling children => (children.map values).foldl Nat.min ceiling

theorem fold_congr {i : Nat} (node : Node i) (f g : Fin i → Nat)
    (same : ∀ j, f j = g j) : node.fold f = node.fold g := by
  have maps : ∀ xs : List (Fin i), xs.map f = xs.map g := by
    intro xs
    exact List.map_congr_left (fun j _ => same j)
  cases node <;> simp only [Node.fold, maps]

def recursive (graph : (i : Nat) → Node i) (i : Nat) : Nat :=
  (graph i).fold (fun j => recursive graph j.val)
termination_by i
decreasing_by exact j.isLt

def compiled (graph : (i : Nat) → Node i) : Nat → (Nat → Nat)
  | 0 => fun _ => 0
  | n + 1 =>
      let previous := compiled graph n
      let value := (graph n).fold (fun j => previous j.val)
      fun i => if i = n then value else previous i

theorem compiled_prefix_agrees (graph : (i : Nat) → Node i) (n : Nat) :
    ∀ i, i < n → compiled graph n i = recursive graph i := by
  induction n with
  | zero => intro i h; omega
  | succ n ih =>
      intro i hi
      by_cases eq : i = n
      · subst i
        simp only [compiled, ↓reduceIte]
        rw [recursive]
        exact fold_congr (graph n) _ _ (fun j => ih j.val j.isLt)
      · have lt : i < n := by omega
        simpa only [compiled, if_neg eq] using ih i lt

theorem compiled_root_equals_recursive (graph : (i : Nat) → Node i)
    (size : Nat) (root : Fin size) :
    compiled graph size root.val = recursive graph root.val :=
  compiled_prefix_agrees graph size root.val root.isLt

-- Frozen graph, arbitrary changing leaf scores: evaluation equivalence is
-- uniform in every graph. Changing tapes/fields may change the graph itself.
theorem two_evaluators_agree_for_every_graph :
    ∀ (graph : (i : Nat) → Node i) (size : Nat) (root : Fin size),
      compiled graph size root.val = recursive graph root.val := by
  intros
  exact compiled_root_equals_recursive _ _ _

-- Two equally weighted worlds share one focal observation. The two lawful
-- constant actions have payoff vectors [0,1] and [1,0]. Maximizing each world
-- before summing invents two different actions at that same observation.
theorem world_first_max_changes_semantics :
    Nat.max (0 + 1) (1 + 0) < Nat.max 0 1 + Nat.max 1 0 := by decide

theorem world_first_min_changes_semantics :
    Nat.min 0 1 + Nat.min 1 0 < Nat.min (0 + 1) (1 + 0) := by decide

#print axioms fold_congr
#print axioms compiled_prefix_agrees
#print axioms compiled_root_equals_recursive
#print axioms two_evaluators_agree_for_every_graph
#print axioms world_first_max_changes_semantics
#print axioms world_first_min_changes_semantics

end WaltResearch.Phase3.Sol
