#include <metal_stdlib>
using namespace metal;

struct OpeningTaskV1 {
    uint words[8];
};

struct OpeningChooseTableV1 {
    uint words[484];
};

struct OpeningSlotV1 {
    uint words[16];
};

static_assert(sizeof(OpeningTaskV1) == 32, "OpeningTaskV1 ABI size");
static_assert(alignof(OpeningTaskV1) == 4, "OpeningTaskV1 ABI alignment");
static_assert(sizeof(OpeningChooseTableV1) == 1936, "OpeningChooseTableV1 ABI size");
static_assert(alignof(OpeningChooseTableV1) == 4, "OpeningChooseTableV1 ABI alignment");
static_assert(sizeof(OpeningSlotV1) == 64, "OpeningSlotV1 ABI size");
static_assert(alignof(OpeningSlotV1) == 4, "OpeningSlotV1 ABI alignment");

constant uint STATUS_SKIP = 0u;
constant uint STATUS_VALID = 1u;
constant uint HARD_BAD_ABI = 0x80000001u;
constant uint HARD_BAD_MASK = 0x80000002u;
constant uint HARD_BAD_COUNT = 0x80000003u;
constant uint HARD_BAD_RESPONSE_ORDINAL = 0x80000004u;
constant uint HARD_TOO_MANY_STRATA = 0x80000005u;
constant uint HARD_CHOOSE_INDEX = 0x80000006u;
constant uint HARD_SUPPORT_OVERFLOW = 0x80000007u;
constant uint HARD_COEFFICIENT_OVERFLOW = 0x80000008u;
constant uint HARD_MASS_OVERFLOW = 0x80000009u;
constant uint MAX_CHOOSE_ENTRY = 352716u;
constant uint MAX_CELL_SUPPORT = 17153136u;
constant ulong MAX_CELL_COEFFICIENT = 74088000ul;
constant ulong MAX_CELL_MASS = 1270841539968000ul;

static inline void write_canonical(
    device OpeningSlotV1* slots,
    uint slot,
    uint status,
    uint task_ordinal)
{
    for (uint word = 0u; word < 16u; ++word) {
        slots[slot].words[word] = 0u;
    }
    slots[slot].words[0] = status;
    slots[slot].words[1] = task_ordinal;
    slots[slot].words[2] = slot;
}

static inline void rewrite_owned_hard(
    device OpeningSlotV1* slots,
    uint first_slot,
    uint status,
    uint task_ordinal)
{
    for (uint local = 0u; local < 10u; ++local) {
        write_canonical(slots, first_slot + local, status, task_ordinal);
    }
}

static inline uint descriptor_status(device const OpeningTaskV1& task)
{
    if (task.words[0] != 1u) {
        return HARD_BAD_ABI;
    }
    const uint pool_mask = task.words[3];
    const uint matching_mask = task.words[4];
    if ((pool_mask & 0xf0000000u) != 0u ||
        (matching_mask & 0xf0000000u) != 0u) {
        return HARD_BAD_MASK;
    }
    if ((matching_mask & ~pool_mask) != 0u) {
        return HARD_BAD_MASK;
    }
    const uint grade = task.words[2];
    if (grade < 1u || grade > 7u) {
        return HARD_BAD_COUNT;
    }
    const uint pool_count = task.words[5];
    if (popcount(pool_mask) != pool_count || pool_count != 3u * grade) {
        return HARD_BAD_COUNT;
    }
    if (popcount(matching_mask) > 6u) {
        return HARD_BAD_COUNT;
    }
    const uint expected_responses =
        pool_count * (pool_count - 1u) * (pool_count - 2u);
    if (task.words[6] != expected_responses) {
        return HARD_BAD_COUNT;
    }
    const uint expected_slots = expected_responses * 10u;
    if (task.words[7] != expected_slots || expected_slots > 79800u) {
        return HARD_BAD_COUNT;
    }
    return STATUS_VALID;
}

static inline bool select_set_bit(uint mask, uint ordinal, thread uint& physical)
{
    uint seen = 0u;
    for (uint bit = 0u; bit < 28u; ++bit) {
        if ((mask & (1u << bit)) != 0u) {
            if (seen == ordinal) {
                physical = bit;
                return true;
            }
            ++seen;
        }
    }
    return false;
}

static inline bool decode_response(
    uint pool_mask,
    uint pool_count,
    uint q,
    thread uint* response)
{
    const uint row = (pool_count - 1u) * (pool_count - 2u);
    const uint first_position = q / row;
    const uint remainder = q % row;
    const uint second_rank = remainder / (pool_count - 2u);
    const uint third_rank = remainder % (pool_count - 2u);
    const uint second_position =
        second_rank >= first_position ? second_rank + 1u : second_rank;
    const uint lower = min(first_position, second_position);
    const uint higher = max(first_position, second_position);
    uint third_position = third_rank;
    if (third_position >= lower) {
        ++third_position;
    }
    if (third_position >= higher) {
        ++third_position;
    }
    return select_set_bit(pool_mask, first_position, response[0]) &&
           select_set_bit(pool_mask, second_position, response[1]) &&
           select_set_bit(pool_mask, third_position, response[2]);
}

static inline bool choose_value(
    device const OpeningChooseTableV1& choose,
    uint n,
    uint k,
    thread uint& value)
{
    if (n > 21u || k > 21u) {
        return false;
    }
    value = choose.words[n * 22u + k];
    return value <= MAX_CHOOSE_ENTRY;
}

static inline uint compute_support(
    device const OpeningChooseTableV1& choose,
    uint remaining_matching,
    uint remaining_nonmatching,
    uint capacity,
    thread const uint* counts,
    thread uint& support)
{
    uint available_matching = remaining_matching;
    uint available_nonmatching = remaining_nonmatching;
    support = 1u;
    for (uint seat = 0u; seat < 3u; ++seat) {
        const uint matching = counts[seat];
        const uint nonmatching = capacity - matching;
        if (matching > available_matching || nonmatching > available_nonmatching) {
            support = 0u;
            return STATUS_VALID;
        }
        uint matching_choose = 0u;
        uint nonmatching_choose = 0u;
        if (!choose_value(choose, available_matching, matching, matching_choose) ||
            !choose_value(choose, available_nonmatching, nonmatching,
                          nonmatching_choose)) {
            return HARD_CHOOSE_INDEX;
        }
        ulong product = ulong(support) * ulong(matching_choose);
        if (product > ulong(MAX_CELL_SUPPORT)) {
            return HARD_SUPPORT_OVERFLOW;
        }
        support = uint(product);
        product = ulong(support) * ulong(nonmatching_choose);
        if (product > ulong(MAX_CELL_SUPPORT)) {
            return HARD_SUPPORT_OVERFLOW;
        }
        support = uint(product);
        available_matching -= matching;
        available_nonmatching -= nonmatching;
    }
    if (available_matching != 0u || available_nonmatching != 0u) {
        support = 0u;
    }
    return STATUS_VALID;
}

static inline uint compute_coefficient(
    uint grade,
    thread const bool* follower,
    thread const uint* counts,
    thread ulong& coefficient)
{
    coefficient = 1ul;
    for (uint seat = 0u; seat < 3u; ++seat) {
        const uint divisor = follower[seat] ? counts[seat] + 1u : grade;
        if (divisor == 0u || (420u % divisor) != 0u) {
            return HARD_COEFFICIENT_OVERFLOW;
        }
        const ulong factor = ulong(420u / divisor);
        if (factor != 0ul && coefficient > MAX_CELL_COEFFICIENT / factor) {
            return HARD_COEFFICIENT_OVERFLOW;
        }
        coefficient *= factor;
    }
    return STATUS_VALID;
}

kernel void opening_project_v1(
    device const OpeningTaskV1* task [[buffer(0)]],
    device const OpeningChooseTableV1* choose [[buffer(1)]],
    device OpeningSlotV1* slots [[buffer(2)]],
    uint q [[thread_position_in_grid]])
{
    if (q >= task[0].words[6]) {
        return;
    }
    const uint task_ordinal = task[0].words[1];
    const uint first_slot = q * 10u;
    for (uint local = 0u; local < 10u; ++local) {
        write_canonical(slots, first_slot + local, STATUS_SKIP, task_ordinal);
    }

    const uint descriptor = descriptor_status(task[0]);
    if (descriptor != STATUS_VALID) {
        rewrite_owned_hard(slots, first_slot, descriptor, task_ordinal);
        return;
    }

    uint response[3];
    if (!decode_response(task[0].words[3], task[0].words[5], q, response)) {
        rewrite_owned_hard(
            slots, first_slot, HARD_BAD_RESPONSE_ORDINAL, task_ordinal);
        return;
    }

    bool follower[3];
    uint follower_count = 0u;
    for (uint seat = 0u; seat < 3u; ++seat) {
        follower[seat] =
            (task[0].words[4] & (1u << response[seat])) != 0u;
        follower_count += follower[seat] ? 1u : 0u;
    }
    const uint matching_count = popcount(task[0].words[4]);
    if (follower_count > matching_count) {
        rewrite_owned_hard(slots, first_slot, HARD_BAD_RESPONSE_ORDINAL,
                           task_ordinal);
        return;
    }
    const uint capacity = task[0].words[2] - 1u;
    const uint remaining_matching = matching_count - follower_count;
    const uint void_count = 3u - follower_count;
    const uint total_nonmatching = task[0].words[5] - matching_count;
    if (void_count > total_nonmatching) {
        rewrite_owned_hard(slots, first_slot, HARD_BAD_RESPONSE_ORDINAL,
                           task_ordinal);
        return;
    }
    const uint remaining_nonmatching = total_nonmatching - void_count;

    uint local_slot = 0u;
    for (uint first = 0u; first <= capacity; ++first) {
        if (!follower[0] && first != 0u) {
            continue;
        }
        for (uint second = 0u; second <= capacity; ++second) {
            if (!follower[1] && second != 0u) {
                continue;
            }
            for (uint third = 0u; third <= capacity; ++third) {
                if (!follower[2] && third != 0u) {
                    continue;
                }
                if (first + second + third != remaining_matching) {
                    continue;
                }
                if (local_slot >= 10u) {
                    rewrite_owned_hard(
                        slots, first_slot, HARD_TOO_MANY_STRATA, task_ordinal);
                    return;
                }
                const uint counts[3] = { first, second, third };
                uint support = 0u;
                uint status = compute_support(
                    choose[0], remaining_matching, remaining_nonmatching,
                    capacity, counts, support);
                if (status != STATUS_VALID) {
                    rewrite_owned_hard(slots, first_slot, status, task_ordinal);
                    return;
                }
                if (support == 0u) {
                    continue;
                }
                ulong coefficient = 0ul;
                status = compute_coefficient(
                    task[0].words[2], follower, counts, coefficient);
                if (status != STATUS_VALID) {
                    rewrite_owned_hard(slots, first_slot, status, task_ordinal);
                    return;
                }
                if (coefficient != 0ul &&
                    ulong(support) > MAX_CELL_MASS / coefficient) {
                    rewrite_owned_hard(
                        slots, first_slot, HARD_MASS_OVERFLOW, task_ordinal);
                    return;
                }
                const ulong mass = ulong(support) * coefficient;
                const uint slot = first_slot + local_slot;
                write_canonical(slots, slot, STATUS_VALID, task_ordinal);
                slots[slot].words[3] = response[0];
                slots[slot].words[4] = response[1];
                slots[slot].words[5] = response[2];
                slots[slot].words[6] = first;
                slots[slot].words[7] = second;
                slots[slot].words[8] = third;
                slots[slot].words[9] = support;
                slots[slot].words[10] = uint(coefficient);
                slots[slot].words[11] = uint(coefficient >> 32u);
                slots[slot].words[12] = uint(mass);
                slots[slot].words[13] = uint(mass >> 32u);
                ++local_slot;
            }
        }
    }
}
