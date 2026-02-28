#![no_std]
use core::ops::{Add, Not, Sub};

#[cfg(feature = "num_traits")]
use num_traits::{Unsigned, WrappingAdd, WrappingSub};

// ---------------------------------------------------------------------------
// OnesOne — the "unsigned bit-pattern" marker/extension trait
// ---------------------------------------------------------------------------
//
// Without `num_traits`: a sealed trait with hand-rolled impls for the six
// unsigned primitives.  The seal is the only mechanism enforcing the
// unsigned-only constraint.
//
// With `num_traits`: a blanket extension trait over
// `Unsigned + WrappingAdd + WrappingSub + Not`.  Any type satisfying those
// bounds automatically implements `OnesOne`; `Unsigned` is what enforces the
// unsigned-only constraint in place of the seal.

/// Marker trait for types that can serve as the inner bit-pattern of a
/// [`OnesSigned`] value.
///
/// `OnesSigned<T>` stores a ones'-complement-encoded value as a raw unsigned
/// bit pattern. Two's-complement signed primitives are intentionally excluded
/// because on modern hardware they carry two's-complement semantics that would
/// interfere with the end-around carry logic.
///
/// ## Feature `num_traits`
///
/// Without this feature `OnesOne` is a **sealed trait** implemented only for
/// `u8`, `u16`, `u32`, `u64`, `u128`, and `usize`; `ONE`,
/// `ones_wrapping_add`, and `ones_wrapping_sub` are provided by those impls.
///
/// With this feature `OnesOne` becomes a **blanket extension trait** over
/// [`num_traits::Unsigned`] `+` [`num_traits::WrappingAdd`] `+`
/// [`num_traits::WrappingSub`] `+` [`core::ops::Not`].  Any type satisfying
/// those bounds automatically implements `OnesOne`; the three methods and the
/// constant are no longer part of the trait itself — they are supplied
/// directly by the `num-traits` bounds in [`OnesDeps`].
#[cfg(not(feature = "num_traits"))]
pub trait OnesOne: private::Sealed + Sized + Copy {
    /// The value `1` for this type.
    const ONE: Self;
    /// Wrapping addition — equivalent to `u*::wrapping_add`.
    fn ones_wrapping_add(self, rhs: Self) -> Self;
    /// Wrapping subtraction — equivalent to `u*::wrapping_sub`.
    fn ones_wrapping_sub(self, rhs: Self) -> Self;
}

#[cfg(feature = "num_traits")]
pub trait OnesOne: Unsigned + WrappingAdd + WrappingSub + Not<Output = Self> {}

// ---------------------------------------------------------------------------
// Impls of OnesOne
// ---------------------------------------------------------------------------

#[cfg(not(feature = "num_traits"))]
mod private {
    pub trait Sealed {}
    impl Sealed for u8    {}
    impl Sealed for u16   {}
    impl Sealed for u32   {}
    impl Sealed for u64   {}
    impl Sealed for u128  {}
    impl Sealed for usize {}
}

#[cfg(not(feature = "num_traits"))]
macro_rules! impl_ones_one {
    ($($t:ty),+) => {$(
        impl OnesOne for $t {
            const ONE: Self = 1;
            #[inline] fn ones_wrapping_add(self, rhs: Self) -> Self { self.wrapping_add(rhs) }
            #[inline] fn ones_wrapping_sub(self, rhs: Self) -> Self { self.wrapping_sub(rhs) }
        }
    )+};
}
#[cfg(not(feature = "num_traits"))]
impl_ones_one!(u8, u16, u32, u64, u128, usize);

/// Blanket impl: every type satisfying the `num-traits` bounds is
/// automatically a valid `OnesOne`.
#[cfg(feature = "num_traits")]
impl<T> OnesOne for T where T: Unsigned + WrappingAdd + WrappingSub + Not<Output = Self> {}

// ---------------------------------------------------------------------------
// OnesDeps — the combined bound used by OnesSigned's operator impls
// ---------------------------------------------------------------------------

/// Blanket bound required by [`OnesSigned<T>`]'s operator impls.
///
/// Without `num_traits`: requires [`OnesOne`] (which carries `ONE`,
/// `ones_wrapping_add`, and `ones_wrapping_sub`) plus the standard operator
/// traits.
///
/// With `num_traits`: requires [`OnesOne`] (which itself requires
/// [`Unsigned`] + [`WrappingAdd`] + [`WrappingSub`] + [`Not`]) plus
/// [`core::ops::Add`] and [`core::ops::Sub`] for the operator syntax used
/// inside the `Add`/`Sub` impls on `OnesSigned`.
#[cfg(not(feature = "num_traits"))]
pub trait OnesDeps:
    Sized
    + Copy
    + Default
    + PartialOrd
    + OnesOne
    + Not<Output = Self>
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
{
}

#[cfg(not(feature = "num_traits"))]
impl<T> OnesDeps for T where
    T: Sized
        + Copy
        + Default
        + PartialOrd
        + OnesOne
        + Not<Output = Self>
        + Add<Self, Output = Self>
        + Sub<Self, Output = Self>
{
}

#[cfg(feature = "num_traits")]
pub trait OnesDeps:
    Sized
    + Copy
    + Default
    + PartialOrd
    + OnesOne
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
{
}

#[cfg(feature = "num_traits")]
impl<T> OnesDeps for T where
    T: Sized
        + Copy
        + Default
        + PartialOrd
        + OnesOne
        + Add<Self, Output = Self>
        + Sub<Self, Output = Self>
{
}

// ---------------------------------------------------------------------------
// OnesSigned
// ---------------------------------------------------------------------------

/// A ones'-complement-encoded value whose bit pattern is stored in an
/// unsigned integer `T`.
///
/// *Positive zero* is `T::default()` (`0`); *negative zero* is `!T::default()`
/// (all bits set).  The end-around carry/borrow in `Add`/`Sub` keeps results
/// in the canonical ones'-complement range.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
#[repr(transparent)]
pub struct OnesSigned<T>(pub T);

#[cfg(not(feature = "num_traits"))]
impl<T: OnesDeps> Add for OnesSigned<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let v = self.0.ones_wrapping_add(rhs.0);
        // End-around carry: if the true mathematical sum ≥ 2^N the wrapped
        // result is less than either addend.
        if v < self.0 {
            OnesSigned(v.ones_wrapping_add(T::ONE))
        } else {
            OnesSigned(v)
        }
    }
}

#[cfg(feature = "num_traits")]
impl<T: OnesDeps> Add for OnesSigned<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let v = self.0.wrapping_add(&rhs.0);
        // End-around carry: if the true mathematical sum ≥ 2^N the wrapped
        // result is less than either addend.
        if v < self.0 {
            OnesSigned(v.wrapping_add(&T::one()))
        } else {
            OnesSigned(v)
        }
    }
}

impl<T: OnesDeps> Sub for OnesSigned<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        // Ones'-complement subtraction is addition of the bitwise complement:
        //   a - b  ≡  a + !b   (with end-around carry)
        // For an N-bit unsigned type, !b == 2^N − 1 − b, which is exactly the
        // ones' complement of b.
        self + OnesSigned(!rhs.0)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn w8(v: u8)   -> OnesSigned<u8>   { OnesSigned(v) }
    fn w16(v: u16) -> OnesSigned<u16>  { OnesSigned(v) }
    fn w32(v: u32) -> OnesSigned<u32>  { OnesSigned(v) }
    fn w64(v: u64) -> OnesSigned<u64>  { OnesSigned(v) }

    // --- Add ---

    #[test]
    fn add_no_carry_u8() {
        // 3 + 4 = 7; no overflow, no end-around carry.
        assert_eq!((w8(3) + w8(4)).0, 7);
    }

    #[test]
    fn add_end_around_carry_u8() {
        // 0b11000000 (192) + 0b11000000 (192): true sum = 384, wraps to 128;
        // 128 < 192 → end-around carry → 128 + 1 = 129.
        assert_eq!((w8(192) + w8(192)).0, 129);
    }

    #[test]
    fn add_max_plus_one_u8() {
        // 0xFF + 0x01: wraps to 0x00; 0x00 < 0xFF → carry → 0x01.
        assert_eq!((w8(0xFF) + w8(0x01)).0, 0x01);
    }

    #[test]
    fn add_max_plus_max_u8() {
        // 0xFF + 0xFF: wraps to 0xFE; 0xFE < 0xFF → carry → 0xFF.
        // (All-ones plus all-ones returns all-ones — negative zero is idempotent.)
        assert_eq!((w8(0xFF) + w8(0xFF)).0, 0xFF);
    }

    #[test]
    fn add_zero_identity_u8() {
        // 0 + 0 = 0; no carry.
        assert_eq!((w8(0) + w8(0)).0, 0);
    }

    #[test]
    fn add_no_carry_u32() {
        // 100 + 200 = 300; no overflow.
        assert_eq!((w32(100) + w32(200)).0, 300);
    }

    #[test]
    fn add_end_around_carry_u32() {
        // u32::MAX + u32::MAX: wraps to u32::MAX - 1; carry → u32::MAX.
        assert_eq!((w32(u32::MAX) + w32(u32::MAX)).0, u32::MAX);
    }

    #[test]
    fn add_no_carry_u64() {
        assert_eq!((w64(1_000_000) + w64(2_000_000)).0, 3_000_000);
    }

    // --- Sub (a - b ≡ a + !b with end-around carry) ---

    #[test]
    fn sub_u8_basic() {
        // 7 - 3: !3 = 0xFC; 7 + 0xFC wraps to 3; 3 < 7 → carry → 4.
        assert_eq!((w8(7) - w8(3)).0, 4);
    }

    #[test]
    fn sub_self_gives_negative_zero_u8() {
        // 5 - 5: !5 = 0xFA; 5 + 0xFA = 0xFF; 0xFF ≮ 5 → no carry → 0xFF
        // (negative zero — all bits set).
        assert_eq!((w8(5) - w8(5)).0, 0xFF);
    }

    #[test]
    fn sub_zero_from_zero_gives_negative_zero_u8() {
        // 0 - 0: !0 = 0xFF; 0 + 0xFF = 0xFF; 0xFF ≮ 0 → 0xFF.
        assert_eq!((w8(0) - w8(0)).0, 0xFF);
    }

    #[test]
    fn sub_u16_basic() {
        // 10 - 3: !3u16 = 0xFFFC; 10 + 0xFFFC wraps to 6; 6 < 10 → carry → 7.
        assert_eq!((w16(10) - w16(3)).0, 7);
    }

    #[test]
    fn sub_u32_basic() {
        // 200 - 100: !100u32 = u32::MAX - 100;
        // 200 + (u32::MAX - 100) wraps to 99; 99 < 200 → carry → 100.
        assert_eq!((w32(200) - w32(100)).0, 100);
    }

    // --- Complement properties ---

    #[test]
    fn value_plus_complement_is_negative_zero_u8() {
        // a + !a must always equal all-ones (negative zero) with no carry,
        // because a + (2^N - 1 - a) = 2^N - 1 exactly — no overflow.
        let v = 0b10110011u8;
        assert_eq!((w8(v) + w8(!v)).0, 0xFF);
    }

    #[test]
    fn value_plus_complement_is_negative_zero_u32() {
        let v = 0xDEAD_BEEFu32;
        assert_eq!((w32(v) + w32(!v)).0, u32::MAX);
    }
}
