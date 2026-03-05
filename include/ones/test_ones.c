/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include "ones/ones.h"

static int failures = 0;

#define EXPECT_EQ(label, got, want) do { \
    uint64_t _g = (uint64_t)(got); \
    uint64_t _w = (uint64_t)(want); \
    if (_g != _w) { \
        fprintf(stderr, "FAIL %s: got %llu, want %llu\n", \
                (label), (unsigned long long)_g, (unsigned long long)_w); \
        failures++; \
    } else { \
        printf("ok   %s\n", (label)); \
    } \
} while(0)

int main(void) {
    ones_bitwidth_t bw4  = ones_bitwidth(4);
    ones_bitwidth_t bw8  = ones_bitwidth(8);

    /* complement ----------------------------------------------------------- */
    EXPECT_EQ("complement(5)  4-bit",  ones_complement(bw4, 5),   10);
    EXPECT_EQ("complement(0)  4-bit",  ones_complement(bw4, 0),   15);
    EXPECT_EQ("complement(15) 4-bit",  ones_complement(bw4, 15),   0);
    EXPECT_EQ("complement(0)  8-bit",  ones_complement(bw8, 0),  255);
    EXPECT_EQ("complement(255) 8-bit", ones_complement(bw8, 255),  0);

    /* macro complement ----------------------------------------------------- */
    EXPECT_EQ("ONES_COMPLEMENT_8(0x5A)",  ONES_COMPLEMENT_8(0x5A),  0xA5);
    EXPECT_EQ("ONES_COMPLEMENT_8(0xFF)",  ONES_COMPLEMENT_8(0xFF),  0x00);
    EXPECT_EQ("ONES_COMPLEMENT_16(0x5A5A)", ONES_COMPLEMENT_16(0x5A5A), 0xA5A5);
    EXPECT_EQ("ONES_COMPLEMENT_16(0xFFFF)", ONES_COMPLEMENT_16(0xFFFF), 0x0000);
    EXPECT_EQ("ONES_COMPLEMENT_32(0x5A5A5A5A)", ONES_COMPLEMENT_32(0x5A5A5A5A), 0xA5A5A5A5);
    EXPECT_EQ("ONES_COMPLEMENT_32(0xFFFFFFFF)", ONES_COMPLEMENT_32(0xFFFFFFFF), 0x00000000);
    EXPECT_EQ("ONES_COMPLEMENT_64(0x5A5A5A5A5A5A5A5A)", ONES_COMPLEMENT_64(0x5A5A5A5A5A5A5A5A), 0xA5A5A5A5A5A5A5A5);
    EXPECT_EQ("ONES_COMPLEMENT_64(0xFFFFFFFFFFFFFFFF)", ONES_COMPLEMENT_64(0xFFFFFFFFFFFFFFFF), 0x0000000000000000);

    /* add ------------------------------------------------------------------ */
    EXPECT_EQ("add(3,4)   4-bit no-carry",       ones_add(bw4, 3, 4),   7);
    EXPECT_EQ("add(6,12)  4-bit end-around",     ones_add(bw4, 6, 12),  3);
    EXPECT_EQ("add(127,1) 8-bit no-carry",       ones_add(bw8, 127, 1), 128);
    EXPECT_EQ("add(200,100) 8-bit end-around",   ones_add(bw8, 200, 100), 45);

    /* subtract ------------------------------------------------------------- */
    /* subtract(7,3) 4-bit: complement(3)=12; add(7,12)=19→(3)+1=4 */
    EXPECT_EQ("subtract(7,3)  4-bit",  ones_subtract(bw4, 7, 3),  4);
    /* subtract(5,5) 4-bit: complement(5)=10; add(5,10)=15<16 → 15 (neg zero) */
    EXPECT_EQ("subtract(5,5)  4-bit",  ones_subtract(bw4, 5, 5), 15);
    /* subtract(10,3) 8-bit: complement(3)=252; add(10,252)=262→(6)+1=7 */
    EXPECT_EQ("subtract(10,3) 8-bit",  ones_subtract(bw8, 10, 3),  7);

    if (failures) {
        fprintf(stderr, "%d test(s) FAILED\n", failures);
        return EXIT_FAILURE;
    }
    printf("All tests passed.\n");
    return EXIT_SUCCESS;
}
