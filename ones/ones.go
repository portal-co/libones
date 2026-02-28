// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Package ones provides a ones' complement arithmetic wrapper type.
//
// OnesSigned[T] wraps a signed integer and performs ones' complement
// addition and subtraction with end-around carry/borrow correction,
// mirroring the behaviour of the Rust crate.
package ones

// Signed is the set of signed integer types that OnesSigned can wrap.
type Signed interface {
	~int | ~int8 | ~int16 | ~int32 | ~int64
}

// OnesSigned is a ones' complement signed integer.
// The zero value (OnesSigned[T]{}) is the ones' complement positive zero.
type OnesSigned[T Signed] struct {
	V T
}

// Add returns the ones' complement sum of x and y.
// After a normal addition, if the result is negative (wrapped past zero),
// the end-around carry subtracts 1 to maintain the ones' complement
// invariant (collapsing negative zero into the canonical representation).
func (x OnesSigned[T]) Add(y OnesSigned[T]) OnesSigned[T] {
	v := x.V + y.V
	if v < 0 {
		v = v - 1
	}
	return OnesSigned[T]{V: v}
}

// Sub returns the ones' complement difference of x and y.
// After a normal subtraction, if the result is positive when it should not
// be (the magnitude wrapped), the end-around borrow adds 1.
func (x OnesSigned[T]) Sub(y OnesSigned[T]) OnesSigned[T] {
	v := x.V - y.V
	if v > 0 {
		v = v + 1
	}
	return OnesSigned[T]{V: v}
}

// Bitwidth is a ones' complement arithmetic context for a fixed bit width,
// operating on unsigned bit patterns. It mirrors the TypeScript Bitwidth class.
type Bitwidth struct {
	bits uint
}

// NewBitwidth creates a Bitwidth for the given number of bits.
func NewBitwidth(bits uint) Bitwidth {
	return Bitwidth{bits: bits}
}

// Bits returns the bit width.
func (bw Bitwidth) Bits() uint {
	return bw.bits
}

// Complement returns the ones' complement of a: (2^bits - 1) - a.
func (bw Bitwidth) Complement(a uint64) uint64 {
	mod := uint64(1) << bw.bits
	return (mod - 1) - a
}

// Add returns the ones' complement sum of a and b with end-around carry:
// (sum & mask) + carry.
func (bw Bitwidth) Add(a, b uint64) uint64 {
	mod := uint64(1) << bw.bits
	sum := a + b
	carry := uint64(0)
	if sum >= mod {
		carry = 1
	}
	return (sum & (mod - 1)) + carry
}

// Subtract returns the ones' complement difference of a and b by adding the
// complement of b.
func (bw Bitwidth) Subtract(a, b uint64) uint64 {
	return bw.Add(a, bw.Complement(b))
}
