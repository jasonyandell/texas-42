import Std

/- Exploratory arithmetic bridge for pruning settled straight-42 positions.
   A caller must prove points never decrease and final team points total 42.
   No Texas42 transition implementation or tape compiler is verified here. -/
namespace AdversarialSol

theorem made_stays_made (bid nowBid finalBid : Nat)
    (settled : bid ≤ nowBid) (monotone : nowBid ≤ finalBid) :
    bid ≤ finalBid := Nat.le_trans settled monotone

theorem set_stays_set (bid nowDef finalDef finalBid : Nat)
    (validBid : bid ≤ 42) (settled : 42 - bid < nowDef)
    (monotone : nowDef ≤ finalDef) (total : finalBid + finalDef ≤ 42) :
    finalBid < bid := by omega

/- Each action retains the complete scenario bundle and its shared-policy Q.
   Partitioning actions changes only the schedule, provided workers return
   exactly those Q values. This assumption is not a parallel runtime proof. -/
theorem action_partition_preserves_Q {Action : Type} (actions : List Action)
    (sequential worker : Action → Nat) (correct : ∀ a ∈ actions, worker a = sequential a) :
    actions.map worker = actions.map sequential := by
  exact List.map_congr_left correct

#print axioms made_stays_made
#print axioms set_stays_set
#print axioms action_partition_preserves_Q
end AdversarialSol
