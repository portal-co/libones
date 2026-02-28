// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

package ones_test

import (
	"testing"

	"github.com/portal-co/libones/ones"
)

// --- OnesSigned[T] tests ---

func TestOnesSigned_AddPositive(t *testing.T) {
	a := ones.OnesSigned[int]{V: 3}
	b := ones.OnesSigned[int]{V: 4}
	got := a.Add(b)
	if got.V != 7 {
		t.Fatalf("3 + 4 = %d, want 7", got.V)
	}
}

func TestOnesSigned_AddEndAroundCarry(t *testing.T) {
	// In an 8-bit ones' complement world the max positive is +127 (0x7F).
	// Using int8: 100 + 100 = 200 which overflows to -56 in two's complement;
	// ones' complement correction subtracts 1 → -57 (i.e. 0b11000111 = -56 - 1).
	a := ones.OnesSigned[int8]{V: 100}
	b := ones.OnesSigned[int8]{V: 100}
	got := a.Add(b)
	// raw int8 addition: 100+100 overflows to -56; -56 < 0 → subtract 1 → -57
	if got.V != -57 {
		t.Fatalf("100 + 100 (int8 ones') = %d, want -57", got.V)
	}
}

func TestOnesSigned_SubPositive(t *testing.T) {
	// In ones' complement the end-around borrow always applies when the result
	// is positive: 10 - 3 raw = 7; 7 > 0 → +1 = 8.
	a := ones.OnesSigned[int]{V: 10}
	b := ones.OnesSigned[int]{V: 3}
	got := a.Sub(b)
	if got.V != 8 {
		t.Fatalf("10 - 3 (ones') = %d, want 8", got.V)
	}
}

func TestOnesSigned_SubEndAroundBorrow(t *testing.T) {
	// 3 - 10: raw = -7; -7 is not > 0, no correction needed → -7
	a := ones.OnesSigned[int]{V: 3}
	b := ones.OnesSigned[int]{V: 10}
	got := a.Sub(b)
	if got.V != -7 {
		t.Fatalf("3 - 10 = %d, want -7", got.V)
	}
}

func TestOnesSigned_AddNegative(t *testing.T) {
	// (-3) + (-4) raw = -7; -7 < 0 → end-around carry subtracts 1 → -8.
	a := ones.OnesSigned[int]{V: -3}
	b := ones.OnesSigned[int]{V: -4}
	got := a.Add(b)
	if got.V != -8 {
		t.Fatalf("(-3) + (-4) (ones') = %d, want -8", got.V)
	}
}

func TestOnesSigned_SubZero(t *testing.T) {
	// 5 - 5 raw = 0; 0 is not > 0, no correction applied → 0.
	a := ones.OnesSigned[int]{V: 5}
	b := ones.OnesSigned[int]{V: 5}
	got := a.Sub(b)
	if got.V != 0 {
		t.Fatalf("5 - 5 (ones') = %d, want 0", got.V)
	}
}

// --- Bitwidth tests ---

func TestBitwidth_Complement4(t *testing.T) {
	bw := ones.NewBitwidth(4)
	// In 4-bit ones' complement: complement(0b0101=5) = 0b1111-0b0101 = 0b1010 = 10
	got := bw.Complement(5)
	if got != 10 {
		t.Fatalf("complement(5) in 4-bit = %d, want 10", got)
	}
}

func TestBitwidth_ComplementZero(t *testing.T) {
	bw := ones.NewBitwidth(4)
	// complement(0) = 0b1111 = 15 (negative zero)
	got := bw.Complement(0)
	if got != 15 {
		t.Fatalf("complement(0) in 4-bit = %d, want 15", got)
	}
}

func TestBitwidth_Add_NoCarry(t *testing.T) {
	bw := ones.NewBitwidth(4)
	// 3 + 4 = 7 (no end-around carry needed)
	got := bw.Add(3, 4)
	if got != 7 {
		t.Fatalf("3 + 4 in 4-bit = %d, want 7", got)
	}
}

func TestBitwidth_Add_EndAroundCarry(t *testing.T) {
	bw := ones.NewBitwidth(4)
	// 6 + 12 = 18; 18 >= 16 → (18 & 15) + 1 = 2 + 1 = 3
	got := bw.Add(6, 12)
	if got != 3 {
		t.Fatalf("6 + 12 in 4-bit = %d, want 3", got)
	}
}

func TestBitwidth_Subtract(t *testing.T) {
	bw := ones.NewBitwidth(4)
	// subtract(7, 3): complement(3)=12; add(7,12)=19; 19>=16 → (19&15)+1 = 3+1 = 4
	got := bw.Subtract(7, 3)
	if got != 4 {
		t.Fatalf("7 - 3 in 4-bit = %d, want 4", got)
	}
}

func TestBitwidth_Subtract_Self(t *testing.T) {
	bw := ones.NewBitwidth(4)
	// subtract(5, 5): complement(5)=10; add(5,10)=15; 15 < 16 → 15 (negative zero)
	got := bw.Subtract(5, 5)
	if got != 15 {
		t.Fatalf("5 - 5 in 4-bit = %d, want 15 (negative zero)", got)
	}
}

func TestBitwidth_8bit(t *testing.T) {
	bw := ones.NewBitwidth(8)
	// complement(0) = 255
	if c := bw.Complement(0); c != 255 {
		t.Fatalf("complement(0) in 8-bit = %d, want 255", c)
	}
	// 127 + 1 = 128 (no carry since 128 < 256)
	if s := bw.Add(127, 1); s != 128 {
		t.Fatalf("127 + 1 in 8-bit = %d, want 128", s)
	}
	// 200 + 100 = 300; 300 >= 256 → (300 & 255) + 1 = 44 + 1 = 45
	if s := bw.Add(200, 100); s != 45 {
		t.Fatalf("200 + 100 in 8-bit = %d, want 45", s)
	}
}
