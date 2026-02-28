/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

#ifndef ONES_H
#define ONES_H

#include <stdint.h>

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

#endif /* ONES_H */
