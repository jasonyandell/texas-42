/-
Copyright (c) 2026 Jason Yandell. All rights reserved.
Released under Apache 2.0 license as described in the file LICENSE.
Authors: Jason Yandell
-/
import Texas42.Trick1PerfectRecallNet.Evidence

/-!
# Freeze-57 perfect-recall information net

This facade is the integrated Lean gate required by
`GPU-NATIVE-TRICK1-M3.md` §12.  It proves the abstract codec/net/recurrence,
mass, objective, reduction, lineage, counter, and finite-bound obligations.

The trust boundary is intentionally honest: Rust-to-Lean codec and replay
correspondence, Metal-to-Rust kernel correspondence, general independent-oracle
correctness, and grade-4-to-trick-1 transport remain named implementation
debts.  Executable parity is evidence for those boundaries, not an axiom.
-/

namespace Texas42
namespace Trick1PerfectRecallNet

#print axioms decodeH_encodeH
#print axioms decodeC_encodeC
#print axioms encodeH_injective
#print axioms encodeC_injective
#print axioms encodeH_ne_encodeC
#print axioms scopedKey_eq_iff
#print axioms hKey_eq_determines_replay
#print axioms cKey_eq_determines_replay
#print axioms projectH_world_independent
#print axioms cKey_eq_iff
#print axioms child_strict_extension
#print axioms child_unique_parent_action
#print axioms root_sentinel_unique
#print axioms zeroField_successor_unique
#print axioms terminalChild_unique
#print axioms emission_group_eq_iff
#print axioms mem_mergeRuns_iff
#print axioms mergeRuns_monotone_left
#print axioms mergeRuns_sum
#print axioms revealed_group_world_eq
#print axioms blockPartition_fold_invariant
#print axioms blockSize_one_sixteen_identical
#print axioms witness_selector_injective
#print axioms distinct_groups_have_distinct_witnesses
#print axioms singleton_face_retained
#print axioms actionContribution_eq_sum_before_max
#print axioms legal_action_le_stateValue
#print axioms argmaxFace_nonempty
#print axioms lawful_policy_le_sumBeforeMax
#print axioms maximizingPolicy_lawful
#print axioms maximizingPolicy_value
#print axioms sumBeforeMax_eq_freeProduct
#print axioms argmax_reprice_eq_stateValue
#print axioms legal_degree_dvd_massBase
#print axioms cast_massBase_div_degree
#print axioms fieldStep_value
#print axioms field_children_mass_conservation
#print axioms focal_counterfactual_copies_mass
#print axioms propagate_value
#print axioms carried_posterior_invariant
#print axioms terminalScale_eq
#print axioms terminalScale_limbs_eq
#print axioms five_bucket_differential_bridge
#print axioms five_bucket_partition_root
#print axioms p30_make_iff_future_defender_le_eleven
#print axioms p30_make_iff_lossAllowance
#print axioms count_one_retires
#print axioms activeDestinationBound_eq
#print axioms three_mul_activeNextCount_le
#print axioms activeNextCounts_le_bound
#print axioms compaction_oldOrdinal_unique
#print axioms reduction_pair_range_separation
#print axioms reductionVolume_eq
#print axioms reductionVolume_lt_three_mul
#print axioms hiddenOrderCode_card
#print axioms allTwoOrderCode_card
#print axioms encodeFixedOrderContinuation_injective
#print axioms encodeAllTwoOrderContinuation_injective
#print axioms continuation_card_le_216
#print axioms all_two_order_continuation_card_le_432
#print axioms fixedOrderContinuation_card_le_216
#print axioms allTwoOrderContinuation_card_le_432
#print axioms continuation_numeric_identities
#print axioms semantic_ranges_lt_two_pow_21
#print axioms reduction_sequence_counts
#print axioms aggregate_reduction_base_eq_769
#print axioms aggregate_reduction_command_bounds
#print axioms treatment_command_bounds
#print axioms run_command_frame_bounds
#print axioms carrier_frame_constants
#print axioms revealed_world_caps
#print axioms abi_widths
#print axioms metal_live_byte_ledger
#print axioms h_host_phase_ledgers
#print axioms c_host_phase_ledger
#print axioms h_permanent_spill_base
#print axioms h_spill_phase_highs
#print axioms h_all_skeleton_base
#print axioms active_destination_at_cap
#print axioms h_reduction_lifecycle_highs
#print axioms replacement_growth_bound
#print axioms h_spill_high_water
#print axioms c_world_spill_charges
#print axioms c_spill_high_water
#print axioms family_content_caps
#print axioms massBucket_child_terminal_disjoint
#print axioms backward_child_terminal_disjoint
#print axioms two_family_level_zero_bound
#print axioms sign_lanes_partition
#print axioms family_scratch_noncoexistence
#print axioms epoch_partition
#print axioms focal_arrival_once_only
#print axioms accepted_visits_common
#print axioms accepted_censuses_common
#print axioms accepted_family_aggregate_bound
#print axioms global_visit_sum_sixteen
#print axioms selected_terminal_cpu_metal_equal
#print axioms evidence_fold_matches_sealed_root
#print axioms fieldSlotIndex_injective
#print axioms failed_conjunction_accepts_nothing
#print axioms successful_conjunction_accepts_all
#print axioms failed_check_accepts_nothing
#print axioms immutable_replacement_unique

end Trick1PerfectRecallNet
end Texas42
