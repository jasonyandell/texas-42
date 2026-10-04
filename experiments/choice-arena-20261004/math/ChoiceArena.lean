import Std
set_option autoImplicit false
namespace WaltResearch.ChoiceArena
/- These are conditional representation facts. The premises demand the same
   complete ordered world group and the same per-row values. They do not prove
   the Rust scheduler, sampled distribution, RNG, refusal behavior or privacy. -/
theorem indexed_group_values {W : Type} (ids : List Nat) (world : Nat → W)
    (value : W → Nat) (arenaValue : Nat → Nat)
    (same : ∀ i ∈ ids, arenaValue i = value (world i)) :
    ids.map arenaValue = (ids.map world).map value := by
  induction ids with
  | nil => simp
  | cons i ids ih =>
    have hi := same i (by simp)
    have hs := ih (fun j hj => same j (by simp [hj]))
    simp [hi, hs]

theorem indexed_group_sum {W : Type} (ids : List Nat) (world : Nat → W)
    (value : W → Nat) (arenaValue : Nat → Nat)
    (same : ∀ i ∈ ids, arenaValue i = value (world i)) :
    (ids.map arenaValue).sum = ((ids.map world).map value).sum := by
  rw [indexed_group_values ids world value arenaValue same]

theorem ordered_group_bounds (groups : List (List Nat)) (lo hi value : Nat → Nat)
    (covered : ∀ g ∈ groups, ∀ w ∈ g, lo w ≤ value w ∧ value w ≤ hi w) :
    (groups.flatten.map lo).sum ≤ (groups.flatten.map value).sum ∧
    (groups.flatten.map value).sum ≤ (groups.flatten.map hi).sum := by
  have rows : ∀ w ∈ groups.flatten, lo w ≤ value w ∧ value w ≤ hi w := by
    intro w hw
    obtain ⟨g, hg, hwg⟩ := List.mem_flatten.mp hw
    exact covered g hg w hwg
  have sumBound : ∀ ws : List Nat, (∀ w ∈ ws, lo w ≤ value w ∧ value w ≤ hi w) →
      (ws.map lo).sum ≤ (ws.map value).sum ∧ (ws.map value).sum ≤ (ws.map hi).sum := by
    intro ws coveredRows
    induction ws with
    | nil => simp
    | cons w ws ih =>
      have hw := coveredRows w (by simp)
      have hs := ih (fun x hx => coveredRows x (by simp [hx]))
      simp only [List.map_cons, List.sum_cons]; omega
  exact sumBound groups.flatten rows

theorem deduplicated_complete_view {V A : Type} (policy : V → A)
    (specs : List V) (unique : Nat → V) (route : V → Nat)
    (complete : ∀ s ∈ specs, unique (route s) = s) :
    specs.map (fun s => policy (unique (route s))) = specs.map policy := by
  induction specs with
  | nil => simp
  | cons s ss ih =>
    have hs := complete s (by simp)
    have ht := ih (fun t ht => complete t (by simp [ht]))
    simp [hs, ht]
/- Public future-known tiles and sampled unknown tiles must be precisely the
   disjoint (or overlapping) union tested by the original following obligation.
   This logical equivalence alone establishes neither that partition nor RNG. -/
theorem following_absence_split {T : Type} (futureKnown unknown follows : T → Prop) :
    (∀ t, (futureKnown t ∨ unknown t) → ¬ follows t) ↔
    (∀ t, futureKnown t → ¬ follows t) ∧ (∀ t, unknown t → ¬ follows t) := by
  constructor
  · intro h; exact ⟨fun t ht => h t (Or.inl ht), fun t ht => h t (Or.inr ht)⟩
  · intro h t ht; cases ht with
    | inl ht => exact h.1 t ht
    | inr ht => exact h.2 t ht

theorem all_observations_split {O T : Type} (known unknown follows : O → T → Prop) :
    (∀ o t, (known o t ∨ unknown o t) → ¬ follows o t) ↔
    (∀ o t, known o t → ¬ follows o t) ∧ (∀ o t, unknown o t → ¬ follows o t) := by
  constructor
  · intro h; exact ⟨fun o t ht => h o t (Or.inl ht), fun o t ht => h o t (Or.inr ht)⟩
  · intro h o t ht; cases ht with
    | inl ht => exact h.1 o t ht
    | inr ht => exact h.2 o t ht
#print axioms following_absence_split
#print axioms all_observations_split
#print axioms indexed_group_values
#print axioms indexed_group_sum
#print axioms ordered_group_bounds
#print axioms deduplicated_complete_view
end WaltResearch.ChoiceArena
