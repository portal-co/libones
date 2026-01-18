#![no_std]
use core::ops::{Add, Sub};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
#[repr(transparent)]
pub struct OnesSigned<T>(pub T);
impl<T> Add for OnesSigned<T>
where
    T: Add<Output = T> + OnesDeps,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut v = self.0 + rhs.0;
        if v < T::default() {
            v = v - 1u8;
        }
        OnesSigned(v)
    }
}
pub trait OnesDeps:
    Sized + Default + PartialOrd + Sub<u8, Output = Self> + Add<u8, Output = Self>
{
}
impl<T> OnesDeps for T where
    T: Sized + Default + PartialOrd + Sub<u8, Output = Self> + Add<u8, Output = Self>
{
}
impl<T> Sub for OnesSigned<T>
where
    T: Sub<Output = T> + OnesDeps,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut v = self.0 - rhs.0;
        if v > T::default() {
            v = v + 1u8;
        }
        OnesSigned(v)
    }
}
