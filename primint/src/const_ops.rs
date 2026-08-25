//! Implementation of arithmetic operations in a `const` context, generic over [`UnsignedPrimInt`].
//!
//! These are not exposed publicly, but are used by [`crate::NonMax`] and [`crate::NonZero`] internally.
//! The [`from_u128`] function has a simple and straightforward implementation, but [`to_u128`] is a little more complicated.
//! It took several tries to implement in a way that is both sound and works on the MSRV.

use crate::{PrimitiveInt, UnsignedPrimInt};

/// A value with explicit padding bytes
#[repr(C, packed)]
struct PaddedVal<T: PrimitiveInt> {
    val: T,
    padding: u128,
}
impl<T: PrimitiveInt> Copy for PaddedVal<T> {}
impl<T: PrimitiveInt> Clone for PaddedVal<T> {
    fn clone(&self) -> Self {
        *self
    }
}

#[repr(C)]
pub union TransmuteHelper<T: PrimitiveInt> {
    bits: u128,
    val: T,
    padded_val: PaddedVal<T>,
}
#[inline]
const fn to_u128<T: PrimitiveInt>(val: T) -> u128 {
    // current approach
    {
        let bits = TransmuteHelper {
            padded_val: PaddedVal { val, padding: 0 },
        };
        // SAFETY: Properly initialized with appropriate padding bytes
        unsafe { bits.bits }
    }
    // alternate approach 1
    #[cfg(any())]
    {
        // miri says this function is fine,
        // but it relies on `bits.val = val` leaving the padding bits alone (keeping them zero).
        let mut bits = TransmuteHelper { bits: 0 };
        bits.val = val;
        // SAFETY: Ensured we started with zero-initialized field
        unsafe { bits.bits }
    }
    // alternate approach 2
    #[cfg(any())]
    {
        // this is the precursor to the #[repr(packed)] approach.
        // It is less efficient without optimization but works fine on our MSRV
        union EvenSounderTransmuteHelper<T: PrimitiveInt> {
            repeated_val: [T; 16],
            bits: u128,
        }
        let mut bits = EvenSounderTransmuteHelper {
            repeated_val: [T::ZERO; 16],
        };
        unsafe {
            bits.repeated_val[0] = val;
            bits.bits
        }
    }
    // alternate approach 3
    #[cfg(any())]
    {
        // this is equally efficient as the current approach but requires Rust 1.83
        let mut bits = 0u128;
        unsafe {
            core::ptr::addr_of_mut!(bits).cast::<T>().write(val);
        }
        bits
    }
}
#[inline]
const fn from_u128<T: UnsignedPrimInt>(val: u128) -> T {
    unsafe { TransmuteHelper { bits: val }.val }
}
#[inline]
pub const fn wrapping_dec<T: UnsignedPrimInt>(val: T) -> T {
    from_u128(to_u128(val).wrapping_sub(1))
}
#[inline]
pub const fn wrapping_inc<T: UnsignedPrimInt>(val: T) -> T {
    from_u128(to_u128(val).wrapping_add(1))
}
#[inline]
pub const fn is_max_val<T: UnsignedPrimInt>(val: T) -> bool {
    to_u128(val) == to_u128(T::MAX)
}
#[inline]
pub const fn is_zero<T: PrimitiveInt>(val: T) -> bool {
    to_u128(val) == 0
}
