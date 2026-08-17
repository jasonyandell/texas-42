/-
Copyright (c) 2026 Jason Yandell. All rights reserved.
Released under Apache 2.0 license as described in the file LICENSE.
Authors: Jason Yandell
-/
import Texas42.Trick1PerfectRecallNet.Reduction

/-!
# Freeze-57 finite carrier, memory, and ABI bounds

This module kernel-checks the numeric identities used to reject an infeasible
run before allocation or dispatch.
-/

namespace Texas42
namespace Trick1PerfectRecallNet

/-- Exact support/horizon constants used by the task frame. -/
theorem carrier_frame_constants :
    supportWorlds = 1200 ∧ futureFieldMoves = 12 := by
  exact ⟨rfl, rfl⟩

/-- Complete revealed-world play-order and partial-node caps. -/
theorem revealed_world_caps :
    Nat.factorial 3 * (Nat.factorial 4) ^ 3 = 82944 ∧
      82944 * 16 = 1327104 := by
  norm_num [Nat.factorial]

/-- Exact fixed ABI widths in words and bytes. -/
theorem abi_widths :
    16 * 4 = 64 ∧ 24 * 4 = 96 ∧ 28 * 4 = 112 ∧
      12 * 4 = 48 ∧ 4 * 4 = 16 := by
  norm_num

/-- Exact maximum shared-Metal allocation ledger. -/
theorem metal_live_byte_ledger :
    64 + (524288 + 2) * 96 + (3670016 + 2) * 112 +
        2 * (524288 + 2) * 48 + (524288 + 2) * 16 = 520094400 ∧
      520094400 < 536870912 ∧
      536870912 - 520094400 = 16776512 := by
  norm_num

/-- H host-phase ledgers both remain under the 2 GiB cap. -/
theorem h_host_phase_ledgers :
    2 ^ 26 + 7 * 2 ^ 28 + 2 ^ 24 = 1962934272 ∧
      2147483648 - 1962934272 = 184549376 ∧
      2 ^ 26 + 32 * 2 ^ 23 + 2 * 96 * 2 ^ 19 +
        2 * 16 * 2 ^ 19 + 64 * 2 ^ 19 + 256 * 128 +
        (128 + 96 + 64) * 2 ^ 19 + 2 ^ 26 + 96 * 2 ^ 19 =
          755007488 ∧
      2147483648 - 755007488 = 1392476160 := by
  norm_num

/-- Sixteen-world production remains under the same host cap. -/
theorem c_host_phase_ledger :
    755007488 - 32 * 2 ^ 23 + 32 * 16 * 1327104 = 1166049280 ∧
      2147483648 - 1166049280 = 981434368 := by
  norm_num

/-- Exact permanent H spill payload. -/
theorem h_permanent_spill_base :
    96 * 2 ^ 23 + 128 * 2 ^ 23 + 96 * 7 * 2 ^ 23 +
      64 * 2 ^ 26 = 11811160064 := by
  norm_num

/-- Construction, key-sort, and edge-copy H phases. -/
theorem h_spill_phase_highs :
    11811160064 + 2 * (96 * 2073600) + 67108864 = 12276400128 ∧
      11811160064 + 805306368 + 67108864 = 12683575296 ∧
      11811160064 + 4294967296 + 67108864 = 16173236224 := by
  norm_num

/-- All-skeleton base used by both serial reduction families. -/
theorem h_all_skeleton_base :
    96 * 2 ^ 23 + 64 * 2 ^ 23 + 64 * 7 * 2 ^ 23 +
      64 * 2 ^ 26 = 9395240960 := by
  norm_num

/-- Exact active-destination census after singleton retirement. -/
theorem active_destination_at_cap :
    (2 * 2 ^ 26) / 3 = 44739242 := by
  norm_num

/-- Exact MASS_BUCKET and BACKWARD_VALUE H lifecycle bounds. -/
theorem h_reduction_lifecycle_highs :
    9395240960 + 48 * 67108864 + 48 * 44739242 + 67108864 =
        14831058912 ∧
      9395240960 + 6442450944 + 67108864 +
        (64 + 64) * 2 ^ 19 = 15971909632 := by
  norm_num

/-- Unique-witness replacement growth is bounded by `96 * 2^26`. -/
theorem replacement_growth_bound {completed unresolved : ℕ}
    (hcap : completed + unresolved ≤ 2 ^ 26) :
    96 * completed + 80 * unresolved ≤ 96 * 2 ^ 26 := by
  omega

/-- The nonoverlapping edge-copy phase is the H high water. -/
theorem h_spill_high_water :
    14831058912 ≤ 16173236224 ∧
      15971909632 ≤ 16173236224 ∧
      16173236224 < 17179869184 ∧
      17179869184 - 16173236224 = 1006632960 := by
  norm_num

/-- Exact one-world C spill and serial scratch charges. -/
theorem c_world_spill_charges :
    (96 + 128 + 96 + 64) * 1327104 = 509607936 ∧
      48 * 1327104 + 48 * ((2 * 1327104) / 3) = 106168320 ∧
      64 * 1327104 = 84934656 ∧
      96 * 1327104 = 127401984 ∧
      48 * 1327104 = 63700992 := by
  norm_num

/-- Corrected sixteen-world C high-water ledger. -/
theorem c_spill_high_water :
    16 * 509607936 + 16 * 106168320 + 16 * 84934656 + 398131200 +
        67108864 + 16 * 63700992 + 16 * 127401984 + 67108864 =
          14801371136 ∧
      14801371136 < 17179869184 ∧
      17179869184 - 14801371136 = 2378498048 := by
  norm_num

/-- All top-level independent content caps compose without overflow. -/
theorem family_content_caps :
    67108864 + 67108864 = 134217728 ∧
      134217728 = 2 ^ 27 ∧
      8388608 = 2 ^ 23 ∧ 67108864 = 2 ^ 26 := by
  norm_num

end Trick1PerfectRecallNet
end Texas42
