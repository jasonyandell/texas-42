#include <metal_stdlib>
using namespace metal;

struct ArithmeticInputV1 {
    uint words[20];
};

struct ArithmeticOutputV1 {
    uint words[16];
};

static_assert(sizeof(ArithmeticInputV1) == 80, "ArithmeticInputV1 ABI size");
static_assert(alignof(ArithmeticInputV1) == 4, "ArithmeticInputV1 ABI alignment");
static_assert(sizeof(ArithmeticOutputV1) == 64, "ArithmeticOutputV1 ABI size");
static_assert(alignof(ArithmeticOutputV1) == 4, "ArithmeticOutputV1 ABI alignment");

constant uint STATUS_SUCCESS = 1u;
constant uint STATUS_CHECKED_UNDEFINED = 2u;
constant uint HARD_BAD_ABI = 0x80000001u;
constant uint HARD_BAD_OPERATION = 0x80000002u;
constant uint HARD_BAD_OPERAND = 0x80000003u;
constant uint HARD_BAD_UNUSED_RHS = 0x80000004u;
constant uint HARD_BAD_EXPONENT = 0x80000005u;

constant uint OP_CHECKED_ADD = 1u;
constant uint OP_CHECKED_SUB = 2u;
constant uint OP_CHECKED_MUL_SMALL = 3u;
constant uint OP_CHECKED_MUL_POW_420 = 4u;
constant uint OP_COMPARE = 5u;

static inline void initialize_output(
    device ArithmeticOutputV1* outputs,
    uint gid,
    uint case_id,
    uint operation)
{
    for (uint word = 0u; word < 16u; ++word) {
        outputs[gid].words[word] = 0u;
    }
    outputs[gid].words[1] = case_id;
    outputs[gid].words[2] = operation;
}

static inline bool rhs_is_zero(device const ArithmeticInputV1& input)
{
    uint aggregate = 0u;
    for (uint limb = 12u; limb < 20u; ++limb) {
        aggregate |= input.words[limb];
    }
    return aggregate == 0u;
}

static inline bool checked_mul_small(
    thread const uint* lhs,
    uint factor,
    thread uint* result)
{
    ulong carry = 0ul;
    for (uint limb = 0u; limb < 8u; ++limb) {
        const ulong product = ulong(lhs[limb]) * ulong(factor) + carry;
        result[limb] = uint(product & 0xfffffffful);
        carry = product >> 32u;
    }
    return carry == 0ul;
}

kernel void u256_parity_v1(
    device const ArithmeticInputV1* inputs [[buffer(0)]],
    device ArithmeticOutputV1* outputs [[buffer(1)]],
    uint gid [[thread_position_in_grid]])
{
    const device ArithmeticInputV1& input = inputs[gid];
    const uint case_id = input.words[1];
    const uint operation = input.words[2];
    const uint operand = input.words[3];
    initialize_output(outputs, gid, case_id, operation);

    if (input.words[0] != 1u) {
        outputs[gid].words[0] = HARD_BAD_ABI;
        return;
    }
    if (operation < OP_CHECKED_ADD || operation > OP_COMPARE) {
        outputs[gid].words[0] = HARD_BAD_OPERATION;
        return;
    }
    if ((operation == OP_CHECKED_ADD || operation == OP_CHECKED_SUB ||
         operation == OP_COMPARE) && operand != 0u) {
        outputs[gid].words[0] = HARD_BAD_OPERAND;
        return;
    }
    if ((operation == OP_CHECKED_MUL_SMALL ||
         operation == OP_CHECKED_MUL_POW_420) && !rhs_is_zero(input)) {
        outputs[gid].words[0] = HARD_BAD_UNUSED_RHS;
        return;
    }
    if (operation == OP_CHECKED_MUL_POW_420 && operand > 21u) {
        outputs[gid].words[0] = HARD_BAD_EXPONENT;
        return;
    }

    uint result[8];
    for (uint limb = 0u; limb < 8u; ++limb) {
        result[limb] = 0u;
    }
    bool defined = true;

    if (operation == OP_CHECKED_ADD) {
        ulong carry = 0ul;
        for (uint limb = 0u; limb < 8u; ++limb) {
            const ulong sum = ulong(input.words[4u + limb]) +
                              ulong(input.words[12u + limb]) + carry;
            result[limb] = uint(sum & 0xfffffffful);
            carry = sum >> 32u;
        }
        defined = carry == 0ul;
    } else if (operation == OP_CHECKED_SUB) {
        ulong borrow = 0ul;
        for (uint limb = 0u; limb < 8u; ++limb) {
            const ulong lhs = ulong(input.words[4u + limb]);
            const ulong subtrahend = ulong(input.words[12u + limb]) + borrow;
            if (lhs >= subtrahend) {
                result[limb] = uint(lhs - subtrahend);
                borrow = 0ul;
            } else {
                result[limb] = uint((1ul << 32u) + lhs - subtrahend);
                borrow = 1ul;
            }
        }
        defined = borrow == 0ul;
    } else if (operation == OP_CHECKED_MUL_SMALL) {
        uint lhs[8];
        for (uint limb = 0u; limb < 8u; ++limb) {
            lhs[limb] = input.words[4u + limb];
        }
        defined = checked_mul_small(lhs, operand, result);
    } else if (operation == OP_CHECKED_MUL_POW_420) {
        for (uint limb = 0u; limb < 8u; ++limb) {
            result[limb] = input.words[4u + limb];
        }
        for (uint exponent = 0u; exponent < operand; ++exponent) {
            uint next[8];
            if (!checked_mul_small(result, 420u, next)) {
                defined = false;
                break;
            }
            for (uint limb = 0u; limb < 8u; ++limb) {
                result[limb] = next[limb];
            }
        }
    } else {
        uint ordering = 2u;
        for (uint reverse = 0u; reverse < 8u; ++reverse) {
            const uint limb = 7u - reverse;
            const uint lhs = input.words[4u + limb];
            const uint rhs = input.words[12u + limb];
            if (lhs < rhs) {
                ordering = 1u;
                break;
            }
            if (lhs > rhs) {
                ordering = 3u;
                break;
            }
        }
        outputs[gid].words[0] = STATUS_SUCCESS;
        outputs[gid].words[3] = 1u;
        outputs[gid].words[4] = ordering;
        return;
    }

    if (!defined) {
        outputs[gid].words[0] = STATUS_CHECKED_UNDEFINED;
        return;
    }
    outputs[gid].words[0] = STATUS_SUCCESS;
    outputs[gid].words[3] = 1u;
    for (uint limb = 0u; limb < 8u; ++limb) {
        outputs[gid].words[5u + limb] = result[limb];
    }
}
