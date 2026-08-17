#include <metal_stdlib>
using namespace metal;

struct M3KernelControlV1 {
    uint words[16];
};

struct M3ParticleAbiV1 {
    uint words[24];
};

struct M3FieldSlotAbiV1 {
    uint words[28];
};

struct M3ReductionAbiV1 {
    uint words[12];
};

struct M3ReductionPairV1 {
    uint words[4];
};

static_assert(sizeof(M3KernelControlV1) == 64,
              "M3KernelControlV1 ABI size");
static_assert(alignof(M3KernelControlV1) == 4,
              "M3KernelControlV1 ABI alignment");
static_assert(sizeof(M3ParticleAbiV1) == 96,
              "M3ParticleAbiV1 ABI size");
static_assert(alignof(M3ParticleAbiV1) == 4,
              "M3ParticleAbiV1 ABI alignment");
static_assert(sizeof(M3FieldSlotAbiV1) == 112,
              "M3FieldSlotAbiV1 ABI size");
static_assert(alignof(M3FieldSlotAbiV1) == 4,
              "M3FieldSlotAbiV1 ABI alignment");
static_assert(sizeof(M3ReductionAbiV1) == 48,
              "M3ReductionAbiV1 ABI size");
static_assert(alignof(M3ReductionAbiV1) == 4,
              "M3ReductionAbiV1 ABI alignment");
static_assert(sizeof(M3ReductionPairV1) == 16,
              "M3ReductionPairV1 ABI size");
static_assert(alignof(M3ReductionPairV1) == 4,
              "M3ReductionPairV1 ABI alignment");

constant uint M3_ABI = 1u;
constant uint M3_OPCODE_EXPAND = 1u;
constant uint M3_OPCODE_REDUCE = 2u;
constant uint M3_OBJECTIVE_A = 1u;
constant uint M3_OBJECTIVE_B = 2u;
constant uint M3_TREATMENT_H = 1u;
constant uint M3_TREATMENT_C = 2u;
constant uint M3_STATUS_EMPTY = 0u;
constant uint M3_STATUS_VALID = 1u;
constant uint M3_STATUS_HARD = 2u;
constant uint M3_INVALID_TILE = 0xffffffffu;
constant uint M3_LOW_28_MASK = 0x0fffffffu;

constant uint M3_FIELD_BAD_ABI = 1u;
constant uint M3_FIELD_BAD_TASK = 2u;
constant uint M3_FIELD_BAD_OBJECTIVE = 3u;
constant uint M3_FIELD_BAD_TREATMENT = 4u;
constant uint M3_FIELD_BAD_MASK = 5u;
constant uint M3_FIELD_BAD_PACKED_STATE = 6u;
constant uint M3_FIELD_BAD_ACTOR = 7u;
constant uint M3_FIELD_BAD_LEGAL_DEGREE = 8u;
constant uint M3_FIELD_BAD_TILE = 9u;
constant uint M3_FIELD_REPEATED_TILE = 10u;
constant uint M3_FIELD_MASS_OVERFLOW = 11u;
constant uint M3_FIELD_BAD_EXPONENT = 12u;
constant uint M3_FIELD_TRICK_RESOLUTION = 13u;
constant uint M3_FIELD_LOSS_RANGE = 14u;
constant uint M3_FIELD_SLOT_RANGE = 15u;
constant uint M3_FIELD_INTERNAL = 16u;

constant uint M3_REDUCE_BAD_ABI = 1u;
constant uint M3_REDUCE_BAD_STATUS = 2u;
constant uint M3_REDUCE_BAD_ROW_ORDINAL = 3u;
constant uint M3_REDUCE_BAD_PLAN_INDEX = 4u;
constant uint M3_REDUCE_BAD_CARRY_FORM = 5u;
constant uint M3_REDUCE_ADD_OVERFLOW = 6u;
constant uint M3_REDUCE_INTERNAL = 7u;
static_assert(M3_REDUCE_INTERNAL == 7u,
              "M3 reduction error registry closes at INTERNAL=7");

static inline uint task_root(uint task)
{
    switch (task & 3u) {
        case 0u: return 4u;
        case 1u: return 7u;
        case 2u: return 9u;
        default: return 20u;
    }
}

static inline uint task_objective(uint task)
{
    return task < 4u ? M3_OBJECTIVE_A : M3_OBJECTIVE_B;
}

static inline void decode_tile(uint tile, thread uint& high, thread uint& low)
{
    high = 0u;
    while (high < 7u) {
        const uint next = (high + 1u) * (high + 2u) / 2u;
        if (tile < next) {
            break;
        }
        ++high;
    }
    low = tile - high * (high + 1u) / 2u;
}

static inline bool tile_is_called(uint tile)
{
    uint high = 0u;
    uint low = 0u;
    decode_tile(tile, high, low);
    return high == 5u || low == 5u;
}

static inline uint tile_led_context(uint tile)
{
    uint high = 0u;
    uint low = 0u;
    decode_tile(tile, high, low);
    (void)low;
    return tile_is_called(tile) ? 7u : high;
}

static inline bool tile_follows(uint tile, uint context)
{
    if (context == 7u) {
        return tile_is_called(tile);
    }
    if (tile_is_called(tile)) {
        return false;
    }
    uint high = 0u;
    uint low = 0u;
    decode_tile(tile, high, low);
    return high == context || low == context;
}

static inline uint tile_rank(uint tile)
{
    uint high = 0u;
    uint low = 0u;
    decode_tile(tile, high, low);
    return high == low ? 12u : high + low;
}

static inline uint tile_tier(uint tile, uint context)
{
    if (tile_is_called(tile)) {
        return 2u;
    }
    return tile_follows(tile, context) ? 1u : 0u;
}

static inline uint tile_count(uint tile)
{
    uint high = 0u;
    uint low = 0u;
    decode_tile(tile, high, low);
    const uint sum = high + low;
    if (sum == 5u) {
        return 5u;
    }
    if (sum == 10u) {
        return 10u;
    }
    return 0u;
}

static inline uint legal_mask_for(uint hand, uint packed_trick, uint trick_length)
{
    if (trick_length == 0u) {
        return hand;
    }
    const uint led_tile = packed_trick & 31u;
    const uint context = tile_led_context(led_tile);
    uint following = 0u;
    uint remaining = hand;
    while (remaining != 0u) {
        const uint tile = ctz(remaining);
        remaining &= remaining - 1u;
        if (tile_follows(tile, context)) {
            following |= 1u << tile;
        }
    }
    return following == 0u ? hand : following;
}

static inline bool checked_mul_u256(
    device const M3ParticleAbiV1& input,
    uint factor,
    thread uint* result)
{
    ulong carry = 0ul;
    for (uint limb = 0u; limb < 8u; ++limb) {
        const ulong product = ulong(input.words[12u + limb]) *
                              ulong(factor) + carry;
        result[limb] = uint(product & 0xfffffffful);
        carry = product >> 32u;
    }
    return carry == 0ul;
}

static inline bool checked_add_u256(
    device const M3ReductionAbiV1& left,
    device const M3ReductionAbiV1& right,
    thread uint* result)
{
    ulong carry = 0ul;
    for (uint limb = 0u; limb < 8u; ++limb) {
        const ulong sum = ulong(left.words[4u + limb]) +
                          ulong(right.words[4u + limb]) + carry;
        result[limb] = uint(sum & 0xfffffffful);
        carry = sum >> 32u;
    }
    return carry == 0ul;
}

static inline void write_empty_slot(
    device M3FieldSlotAbiV1* outputs,
    uint output_index,
    uint parent,
    uint slot,
    uint legal_degree,
    uint task,
    uint source)
{
    for (uint word = 0u; word < 28u; ++word) {
        outputs[output_index].words[word] = 0u;
    }
    outputs[output_index].words[0] = M3_ABI;
    outputs[output_index].words[1] = M3_STATUS_EMPTY;
    outputs[output_index].words[3] = parent;
    outputs[output_index].words[4] = slot;
    outputs[output_index].words[5] = legal_degree;
    outputs[output_index].words[6] = M3_INVALID_TILE;
    outputs[output_index].words[7] = task;
    outputs[output_index].words[8] = source;
}

static inline void write_hard_slot(
    device M3FieldSlotAbiV1* outputs,
    uint output_index,
    uint error,
    uint parent,
    uint slot,
    uint task,
    uint source)
{
    for (uint word = 0u; word < 28u; ++word) {
        outputs[output_index].words[word] = 0u;
    }
    outputs[output_index].words[0] = M3_ABI;
    outputs[output_index].words[1] = M3_STATUS_HARD;
    outputs[output_index].words[2] = error;
    outputs[output_index].words[3] = parent;
    outputs[output_index].words[4] = slot;
    outputs[output_index].words[5] = 0u;
    outputs[output_index].words[6] = M3_INVALID_TILE;
    outputs[output_index].words[7] = task;
    outputs[output_index].words[8] = source;
}

static inline void rewrite_owned_hard(
    device M3FieldSlotAbiV1* outputs,
    uint first_output,
    uint error,
    uint parent,
    uint task,
    uint source)
{
    for (uint slot = 0u; slot < 7u; ++slot) {
        write_hard_slot(outputs, first_output + slot, error, parent, slot,
                        task, source);
    }
}

static inline void write_reduction_hard(
    device M3ReductionAbiV1* destination,
    uint output_index,
    uint error,
    uint row_ordinal)
{
    for (uint word = 0u; word < 12u; ++word) {
        destination[output_index].words[word] = 0u;
    }
    destination[output_index].words[0] = M3_ABI;
    destination[output_index].words[1] = M3_STATUS_HARD;
    destination[output_index].words[2] = error;
    destination[output_index].words[3] = row_ordinal;
}

kernel void m3_field_expand_v1(
    device const M3KernelControlV1* control [[buffer(0)]],
    device const M3ParticleAbiV1* inputs [[buffer(1)]],
    device M3FieldSlotAbiV1* outputs [[buffer(2)]],
    uint gid [[thread_position_in_grid]])
{
    const device M3KernelControlV1& ctl = control[0];
    if (gid >= ctl.words[6]) {
        return;
    }

    const device M3ParticleAbiV1& input = inputs[gid + 1u];
    const uint parent = input.words[11];
    const uint task = ctl.words[3];
    const uint source_ordinal = input.words[3];
    const uint first_output = 1u + 7u * gid;

    for (uint slot = 0u; slot < 7u; ++slot) {
        write_empty_slot(outputs, first_output + slot, parent, slot, 0u,
                         task, source_ordinal);
    }

    if (ctl.words[0] != M3_ABI || input.words[0] != M3_ABI ||
        input.words[1] != M3_STATUS_VALID ||
        ctl.words[1] != M3_OPCODE_EXPAND || ctl.words[2] >= 32768u ||
        ctl.words[10] != 0u ||
        ctl.words[11] != 0u || ctl.words[12] != 0u ||
        ctl.words[13] != 0u || ctl.words[14] != 0u ||
        ctl.words[15] != 0u ||
        input.words[20] != 0u || input.words[21] != 0u ||
        input.words[22] != 0u || input.words[23] != 0u) {
        rewrite_owned_hard(outputs, first_output, M3_FIELD_BAD_ABI,
                           parent, task, source_ordinal);
        return;
    }

    if (task >= 8u || input.words[2] != task ||
        source_ordinal >= 1200u || ctl.words[9] != task_root(task)) {
        rewrite_owned_hard(outputs, first_output, M3_FIELD_BAD_TASK,
                           parent, task, source_ordinal);
        return;
    }
    if ((ctl.words[4] != M3_OBJECTIVE_A &&
         ctl.words[4] != M3_OBJECTIVE_B) ||
        ctl.words[4] != task_objective(task)) {
        rewrite_owned_hard(outputs, first_output, M3_FIELD_BAD_OBJECTIVE,
                           parent, task, source_ordinal);
        return;
    }
    if (ctl.words[5] != M3_TREATMENT_H &&
        ctl.words[5] != M3_TREATMENT_C) {
        rewrite_owned_hard(outputs, first_output, M3_FIELD_BAD_TREATMENT,
                           parent, task, source_ordinal);
        return;
    }

    uint hands_union = 0u;
    bool masks_valid = true;
    for (uint seat = 0u; seat < 4u; ++seat) {
        const uint hand = input.words[4u + seat];
        if ((hand & ~M3_LOW_28_MASK) != 0u || (hands_union & hand) != 0u) {
            masks_valid = false;
        }
        hands_union |= hand;
    }
    if (!masks_valid) {
        rewrite_owned_hard(outputs, first_output, M3_FIELD_BAD_MASK,
                           parent, task, source_ordinal);
        return;
    }

    const uint packed_state = input.words[8];
    const uint leader = packed_state & 3u;
    const uint next_actor = (packed_state >> 2u) & 3u;
    const uint trick_length = (packed_state >> 4u) & 7u;
    const uint completed = (packed_state >> 7u) & 7u;
    const uint future_t1_wins = (packed_state >> 10u) & 7u;
    const uint field_exponent = (packed_state >> 13u) & 15u;
    const uint record_length = (packed_state >> 17u) & 31u;
    const uint packed_trick = input.words[9];

    if ((packed_state & 0xffc00000u) != 0u || trick_length > 3u ||
        completed > 4u || (completed == 4u && trick_length != 0u) ||
        future_t1_wins > completed || field_exponent > 12u ||
        record_length != 12u + 4u * completed + trick_length ||
        record_length > 28u || (packed_trick & 0xfff00000u) != 0u) {
        rewrite_owned_hard(outputs, first_output, M3_FIELD_BAD_PACKED_STATE,
                           parent, task, source_ordinal);
        return;
    }

    uint seen_tiles = 0u;
    bool bad_tile = false;
    bool repeated_tile = false;
    for (uint lane = 0u; lane < 4u; ++lane) {
        const uint tile = (packed_trick >> (5u * lane)) & 31u;
        if (lane < trick_length) {
            if (tile >= 28u) {
                bad_tile = true;
            } else {
                const uint bit = 1u << tile;
                if ((seen_tiles & bit) != 0u) {
                    repeated_tile = true;
                }
                seen_tiles |= bit;
            }
        } else if (tile != 31u) {
            bad_tile = true;
        }
    }

    uint expected_sizes[4];
    for (uint seat = 0u; seat < 4u; ++seat) {
        expected_sizes[seat] = 4u - completed;
    }
    for (uint lane = 0u; lane < trick_length; ++lane) {
        const uint actor = (leader + lane) & 3u;
        if (expected_sizes[actor] == 0u) {
            masks_valid = false;
        } else {
            --expected_sizes[actor];
        }
    }
    for (uint seat = 0u; seat < 4u; ++seat) {
        if (popcount(input.words[4u + seat]) != expected_sizes[seat]) {
            masks_valid = false;
        }
    }
    if ((hands_union & seen_tiles) != 0u ||
        popcount(hands_union) + trick_length != 16u - 4u * completed) {
        masks_valid = false;
    }
    if (!masks_valid) {
        rewrite_owned_hard(outputs, first_output, M3_FIELD_BAD_MASK,
                           parent, task, source_ordinal);
        return;
    }

    if (next_actor != ((leader + trick_length) & 3u) ||
        next_actor == 1u || completed == 4u) {
        rewrite_owned_hard(outputs, first_output, M3_FIELD_BAD_ACTOR,
                           parent, task, source_ordinal);
        return;
    }
    const bool bad_led_tile = trick_length != 0u &&
                              (packed_trick & 31u) >= 28u;
    if (bad_led_tile) {
        rewrite_owned_hard(outputs, first_output, M3_FIELD_BAD_TILE,
                           parent, task, source_ordinal);
        return;
    }
    const uint legal_mask = legal_mask_for(
        input.words[4u + next_actor], packed_trick, trick_length);
    const uint legal_degree = popcount(legal_mask);
    if (legal_degree < 1u || legal_degree > 7u ||
        (420u % legal_degree) != 0u) {
        rewrite_owned_hard(outputs, first_output,
                           M3_FIELD_BAD_LEGAL_DEGREE,
                           parent, task, source_ordinal);
        return;
    }
    if (bad_tile) {
        rewrite_owned_hard(outputs, first_output, M3_FIELD_BAD_TILE,
                           parent, task, source_ordinal);
        return;
    }
    if (repeated_tile) {
        rewrite_owned_hard(outputs, first_output, M3_FIELD_REPEATED_TILE,
                           parent, task, source_ordinal);
        return;
    }
    for (uint slot = 0u; slot < 7u; ++slot) {
        write_empty_slot(outputs, first_output + slot, parent, slot,
                         legal_degree, task, source_ordinal);
    }

    uint focal_plays_in_prefix = 0u;
    for (uint lane = 0u; lane < trick_length; ++lane) {
        focal_plays_in_prefix += ((leader + lane) & 3u) == 1u ? 1u : 0u;
    }
    const uint expected_field_exponent =
        3u * completed + trick_length - focal_plays_in_prefix;
    uint scaled_mass[8];
    if (!checked_mul_u256(input, 420u / legal_degree, scaled_mass)) {
        rewrite_owned_hard(outputs, first_output,
                           M3_FIELD_MASS_OVERFLOW,
                           parent, task, source_ordinal);
        return;
    }
    if (field_exponent != expected_field_exponent ||
        field_exponent != ctl.words[7] || field_exponent >= 12u) {
        rewrite_owned_hard(outputs, first_output, M3_FIELD_BAD_EXPONENT,
                           parent, task, source_ordinal);
        return;
    }
    if (input.words[10] > 34u) {
        rewrite_owned_hard(outputs, first_output, M3_FIELD_LOSS_RANGE,
                           parent, task, source_ordinal);
        return;
    }
    if (ctl.words[6] > 524288u || ctl.words[6] > (0xffffffffu / 7u) ||
        ctl.words[8] != 7u * ctl.words[6] || gid >= ctl.words[6]) {
        rewrite_owned_hard(outputs, first_output, M3_FIELD_SLOT_RANGE,
                           parent, task, source_ordinal);
        return;
    }

    uint slot = 0u;
    uint remaining = legal_mask;
    while (remaining != 0u) {
        const uint tile = ctz(remaining);
        remaining &= remaining - 1u;
        if (slot >= 7u || tile >= 28u) {
            rewrite_owned_hard(outputs, first_output, M3_FIELD_INTERNAL,
                               parent, task, source_ordinal);
            return;
        }

        uint hands[4];
        for (uint seat = 0u; seat < 4u; ++seat) {
            hands[seat] = input.words[4u + seat];
        }
        const uint selected_bit = 1u << tile;
        if ((hands[next_actor] & selected_bit) == 0u) {
            rewrite_owned_hard(outputs, first_output, M3_FIELD_BAD_TILE,
                               parent, task, source_ordinal);
            return;
        }
        hands[next_actor] &= ~selected_bit;

        uint output_trick = packed_trick;
        output_trick &= ~(31u << (5u * trick_length));
        output_trick |= tile << (5u * trick_length);

        uint output_leader = leader;
        uint output_actor = (next_actor + 1u) & 3u;
        uint output_length = trick_length + 1u;
        uint output_completed = completed;
        uint output_t1_wins = future_t1_wins;
        uint output_spend = input.words[10];
        uint completed_winner = 4u;
        uint completed_points = 0u;
        uint transition_flags = 0u;

        if (output_length == 4u) {
            const uint led_tile = output_trick & 31u;
            if (led_tile >= 28u) {
                rewrite_owned_hard(outputs, first_output,
                                   M3_FIELD_TRICK_RESOLUTION,
                                   parent, task, source_ordinal);
                return;
            }
            const uint context = tile_led_context(led_tile);
            uint best_tier = 0u;
            uint best_rank = 0u;
            uint best_lane = 0u;
            completed_points = 1u;
            for (uint lane = 0u; lane < 4u; ++lane) {
                const uint played = (output_trick >> (5u * lane)) & 31u;
                if (played >= 28u) {
                    rewrite_owned_hard(outputs, first_output,
                                       M3_FIELD_TRICK_RESOLUTION,
                                       parent, task, source_ordinal);
                    return;
                }
                const uint tier = tile_tier(played, context);
                const uint rank = tile_rank(played);
                if (lane == 0u || tier > best_tier ||
                    (tier == best_tier && rank > best_rank)) {
                    best_tier = tier;
                    best_rank = rank;
                    best_lane = lane;
                }
                completed_points += tile_count(played);
            }
            if (completed_points > 31u) {
                rewrite_owned_hard(outputs, first_output,
                                   M3_FIELD_TRICK_RESOLUTION,
                                   parent, task, source_ordinal);
                return;
            }
            completed_winner = (leader + best_lane) & 3u;
            output_leader = completed_winner;
            output_actor = completed_winner;
            output_length = 0u;
            output_trick = 0x000fffffu;
            ++output_completed;
            if ((completed_winner & 1u) == 1u) {
                ++output_t1_wins;
            } else {
                if (output_spend > 34u - completed_points) {
                    rewrite_owned_hard(outputs, first_output,
                                       M3_FIELD_LOSS_RANGE,
                                       parent, task, source_ordinal);
                    return;
                }
                output_spend += completed_points;
            }
            transition_flags |= 1u;
        }

        if (output_completed > 4u || output_t1_wins > output_completed ||
            output_spend > 34u) {
            rewrite_owned_hard(outputs, first_output,
                               M3_FIELD_TRICK_RESOLUTION,
                               parent, task, source_ordinal);
            return;
        }
        if (output_completed == 4u) {
            transition_flags |= 2u;
        }
        if (output_actor == 1u) {
            transition_flags |= 4u;
        }
        bool objective_decided = output_completed == 4u;
        if (ctl.words[4] == M3_OBJECTIVE_B) {
            // P_live is the exact still-unbanked point supply: one point for
            // each remaining future trick plus every count tile in the hands
            // or the current incomplete trick.  This bit is diagnostic only.
            uint live_points = 4u - output_completed;
            for (uint seat = 0u; seat < 4u; ++seat) {
                uint live_hand = hands[seat];
                while (live_hand != 0u) {
                    const uint live_tile = ctz(live_hand);
                    live_hand &= live_hand - 1u;
                    live_points += tile_count(live_tile);
                }
            }
            for (uint lane = 0u; lane < output_length; ++lane) {
                const uint live_tile =
                    (output_trick >> (5u * lane)) & 31u;
                live_points += tile_count(live_tile);
            }
            objective_decided = output_spend > 11u ||
                                output_spend + live_points <= 11u;
        }
        if (objective_decided) {
            transition_flags |= 8u;
        }

        const uint output_state = output_leader |
            (output_actor << 2u) |
            (output_length << 4u) |
            (output_completed << 7u) |
            (output_t1_wins << 10u) |
            ((field_exponent + 1u) << 13u) |
            ((record_length + 1u) << 17u);

        const uint output_index = first_output + slot;
        for (uint word = 0u; word < 28u; ++word) {
            outputs[output_index].words[word] = 0u;
        }
        outputs[output_index].words[0] = M3_ABI;
        outputs[output_index].words[1] = M3_STATUS_VALID;
        outputs[output_index].words[2] = 0u;
        outputs[output_index].words[3] = parent;
        outputs[output_index].words[4] = slot;
        outputs[output_index].words[5] = legal_degree;
        outputs[output_index].words[6] = tile;
        outputs[output_index].words[7] = task;
        outputs[output_index].words[8] = source_ordinal;
        for (uint seat = 0u; seat < 4u; ++seat) {
            outputs[output_index].words[9u + seat] = hands[seat];
        }
        outputs[output_index].words[13] = output_state;
        outputs[output_index].words[14] = output_trick;
        outputs[output_index].words[15] = output_spend;
        outputs[output_index].words[16] = 0u;
        for (uint limb = 0u; limb < 8u; ++limb) {
            outputs[output_index].words[17u + limb] = scaled_mass[limb];
        }
        outputs[output_index].words[25] = completed_winner |
            (completed_points << 3u);
        outputs[output_index].words[26] = transition_flags;
        outputs[output_index].words[27] = 0u;
        ++slot;
    }

    if (slot != legal_degree) {
        rewrite_owned_hard(outputs, first_output, M3_FIELD_INTERNAL,
                           parent, task, source_ordinal);
    }
}

kernel void m3_u256_reduce_pass_v1(
    device const M3KernelControlV1* control [[buffer(0)]],
    device const M3ReductionAbiV1* source [[buffer(1)]],
    device const M3ReductionPairV1* plan [[buffer(2)]],
    device M3ReductionAbiV1* destination [[buffer(3)]],
    uint gid [[thread_position_in_grid]])
{
    const device M3KernelControlV1& ctl = control[0];
    if (gid >= ctl.words[6]) {
        return;
    }

    uint output_index = gid + 1u;
    uint output_row = ctl.words[9] + gid;
    if (output_row < ctl.words[9]) {
        output_row = 0xffffffffu;
    }

    if (ctl.words[0] != M3_ABI || ctl.words[1] != M3_OPCODE_REDUCE ||
        ctl.words[2] >= 32768u ||
        ctl.words[3] >= 8u ||
        (ctl.words[4] != M3_OBJECTIVE_A &&
         ctl.words[4] != M3_OBJECTIVE_B) ||
        ctl.words[4] != task_objective(ctl.words[3]) ||
        (ctl.words[5] != M3_TREATMENT_H &&
         ctl.words[5] != M3_TREATMENT_C) ||
        ctl.words[6] == 0u || ctl.words[6] != ctl.words[8] ||
        ctl.words[6] != ctl.words[12] || ctl.words[6] > 524288u ||
        ctl.words[10] >= 21u || ctl.words[11] == 0u ||
        ctl.words[11] > 524288u || ctl.words[6] > ctl.words[11] ||
        ctl.words[13] == 0u || ctl.words[13] > ctl.words[6] ||
        ctl.words[14] != 0u || ctl.words[15] != 0u ||
        ctl.words[7] + ctl.words[11] < ctl.words[7] ||
        ctl.words[9] + ctl.words[12] < ctl.words[9]) {
        write_reduction_hard(destination, output_index, M3_REDUCE_BAD_ABI,
                             output_row);
        return;
    }

    const device M3ReductionPairV1& pair = plan[gid + 1u];
    const uint left_index = pair.words[0];
    const uint right_index = pair.words[1];
    const uint destination_index = pair.words[2];
    const uint flags = pair.words[3];
    const bool right_is_sentinel = right_index == 0xffffffffu;
    const bool left_in_range = left_index < ctl.words[11];
    const bool right_in_range = right_is_sentinel ||
                                right_index < ctl.words[11];
    if (!left_in_range || !right_in_range) {
        write_reduction_hard(destination, output_index,
                             M3_REDUCE_BAD_PLAN_INDEX, output_row);
        return;
    }

    const device M3ReductionAbiV1& left = source[left_index + 1u];
    if (left.words[0] != M3_ABI ||
        (!right_is_sentinel &&
         source[right_index + 1u].words[0] != M3_ABI)) {
        write_reduction_hard(destination, output_index, M3_REDUCE_BAD_ABI,
                             output_row);
        return;
    }
    if (left.words[1] != M3_STATUS_VALID || left.words[2] != 0u ||
        (!right_is_sentinel &&
         (source[right_index + 1u].words[1] != M3_STATUS_VALID ||
          source[right_index + 1u].words[2] != 0u))) {
        write_reduction_hard(destination, output_index,
                             M3_REDUCE_BAD_STATUS, output_row);
        return;
    }
    const uint expected_left_row = ctl.words[7] + left_index;
    if (left.words[3] != expected_left_row ||
        (!right_is_sentinel &&
         source[right_index + 1u].words[3] != ctl.words[7] + right_index)) {
        write_reduction_hard(destination, output_index,
                             M3_REDUCE_BAD_ROW_ORDINAL, output_row);
        return;
    }

    if (destination_index >= ctl.words[12] || destination_index != gid) {
        write_reduction_hard(destination, output_index,
                             M3_REDUCE_BAD_PLAN_INDEX, output_row);
        return;
    }

    const bool carry = (flags & 1u) != 0u;
    if ((flags & ~1u) != 0u || carry != right_is_sentinel ||
        (!carry && right_index != left_index + 1u)) {
        write_reduction_hard(destination, output_index,
                             M3_REDUCE_BAD_CARRY_FORM, output_row);
        return;
    }

    uint result[8];
    if (carry) {
        for (uint limb = 0u; limb < 8u; ++limb) {
            result[limb] = left.words[4u + limb];
        }
    } else {
        const device M3ReductionAbiV1& right = source[right_index + 1u];
        if (!checked_add_u256(left, right, result)) {
            write_reduction_hard(destination, output_index,
                                 M3_REDUCE_ADD_OVERFLOW, output_row);
            return;
        }
    }

    for (uint word = 0u; word < 12u; ++word) {
        destination[output_index].words[word] = 0u;
    }
    destination[output_index].words[0] = M3_ABI;
    destination[output_index].words[1] = M3_STATUS_VALID;
    destination[output_index].words[2] = 0u;
    destination[output_index].words[3] = output_row;
    for (uint limb = 0u; limb < 8u; ++limb) {
        destination[output_index].words[4u + limb] = result[limb];
    }
}
