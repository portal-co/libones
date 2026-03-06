/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

#ifndef ONES_H
#define ONES_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/* ones_bitwidth_t represents a fixed bit-width ones' complement arithmetic
 * context, mirroring the TypeScript Bitwidth class. */
typedef struct {
    int bits;
} ones_bitwidth_t;

/* ones_bitwidth
 * Construct an ones_bitwidth_t for the given bit width. */
static inline ones_bitwidth_t ones_bitwidth(int bits) {
    ones_bitwidth_t bw;
    bw.bits = bits;
    return bw;
}

/* ones_complement
 * Compute the ones' complement of a for the given bit width.
 * Equivalent to: (2^bits - 1) - a */
static inline uint64_t ones_complement(ones_bitwidth_t bw, uint64_t a) {
    uint64_t mod = (uint64_t)1 << bw.bits;
    return (mod - 1) - a;
}

/* ones_add
 * Add two ones' complement values a and b with end-around carry.
 * Equivalent to: (sum & mask) + carry */
static inline uint64_t ones_add(ones_bitwidth_t bw, uint64_t a, uint64_t b) {
    uint64_t mod = (uint64_t)1 << bw.bits;
    uint64_t sum = a + b;
    return (sum & (mod - 1)) + (sum >= mod ? 1 : 0);
}

/* ones_subtract
 * Subtract b from a in ones' complement by adding the complement of b. */
static inline uint64_t ones_subtract(ones_bitwidth_t bw, uint64_t a, uint64_t b) {
    return ones_add(bw, a, ones_complement(bw, b));
}

#ifdef __cplusplus
}
#endif

#ifdef __cplusplus
namespace ones {
    /* Templated ones' complement operations for C++.
     * These correctly handle the ones' complement logic for any unsigned
     * integer type, including proper end-around carry. */
    template<typename T>
    static inline T complement(T a) {
        return (T)~a;
    }

    template<typename T>
    static inline T add(T a, T b) {
        T sum = a + b;
        return (T)(sum + (sum < a));
    }

    template<typename T>
    static inline T subtract(T a, T b) {
        return add(a, complement(b));
    }
}
#define ONES_COMPLEMENT(a) ones::complement(a)
#define ONES_ADD(a, b) ones::add(a, b)
#define ONES_SUBTRACT(a, b) ones::subtract(a, b)
#else
/* Type-agnostic ones' complement operations using _Generic to handle
 * integer promotion for small types (uint8_t, uint16_t). */
#define ONES_COMPLEMENT(a) _Generic((a), \
    uint8_t:  (uint8_t)~(uint8_t)(a), \
    uint16_t: (uint16_t)~(uint16_t)(a), \
    uint32_t: (uint32_t)~(uint32_t)(a), \
    uint64_t: (uint64_t)~(uint64_t)(a), \
    default:  ~(a) \
)

#define ONES_ADD(a, b) _Generic((a), \
    uint8_t:  (uint8_t)((uint8_t)(a) + (uint8_t)(b) + ((uint8_t)((uint8_t)(a) + (uint8_t)(b)) < (uint8_t)(a))), \
    uint16_t: (uint16_t)((uint16_t)(a) + (uint16_t)(b) + ((uint16_t)((uint16_t)(a) + (uint16_t)(b)) < (uint16_t)(a))), \
    uint32_t: (uint32_t)((uint32_t)(a) + (uint32_t)(b) + ((uint32_t)((uint32_t)(a) + (uint32_t)(b)) < (uint32_t)(a))), \
    uint64_t: (uint64_t)((uint64_t)(a) + (uint64_t)(b) + ((uint64_t)((uint64_t)(a) + (uint64_t)(b)) < (uint64_t)(a))), \
    default:  ((a) + (b) + ((a) + (b) < (a))) \
)

#define ONES_SUBTRACT(a, b) ONES_ADD(a, ONES_COMPLEMENT(b))
#endif

/* Macros for fixed-width ones' complement for entire uintN_ts.
 * These forward to the type-agnostic implementations. */
#define ONES_COMPLEMENT_8(a)  ONES_COMPLEMENT((uint8_t)(a))
#define ONES_COMPLEMENT_16(a) ONES_COMPLEMENT((uint16_t)(a))
#define ONES_COMPLEMENT_32(a) ONES_COMPLEMENT((uint32_t)(a))
#define ONES_COMPLEMENT_64(a) ONES_COMPLEMENT((uint64_t)(a))

#define ONES_ADD_8(a, b)      ONES_ADD((uint8_t)(a), (uint8_t)(b))
#define ONES_ADD_16(a, b)     ONES_ADD((uint16_t)(a), (uint16_t)(b))
#define ONES_ADD_32(a, b)     ONES_ADD((uint32_t)(a), (uint32_t)(b))
#define ONES_ADD_64(a, b)     ONES_ADD((uint64_t)(a), (uint64_t)(b))

#define ONES_SUBTRACT_8(a, b)  ONES_SUBTRACT((uint8_t)(a), (uint8_t)(b))
#define ONES_SUBTRACT_16(a, b) ONES_SUBTRACT((uint16_t)(a), (uint16_t)(b))
#define ONES_SUBTRACT_32(a, b) ONES_SUBTRACT((uint32_t)(a), (uint32_t)(b))
#define ONES_SUBTRACT_64(a, b) ONES_SUBTRACT((uint64_t)(a), (uint64_t)(b))

#endif /* ONES_H */
