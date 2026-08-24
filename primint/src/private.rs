use core::mem::size_of;

use crate::{PrimitiveInt, SignedPrimInt, UnsignedPrimInt};

#[allow(dead_code)]
type LargestSignedInt = i128;
#[allow(dead_code)]
type LargestUnsignedInt = u128;

pub trait WrappingCastFrom<U> {
    fn wrapping_cast_from(value: U) -> Self;
}

pub trait Siblings {
    type Signed: SignedPrimInt;
    type Unsigned: UnsignedPrimInt;
}

/// Implemented for [`core::num::NonZero`] whenever the underlying type is a [`PrimitiveInt`].
///
/// # Safety
/// Must be valid to transmute back and forth from the underlying integer type,
/// provided that the value is nonzero.
///
/// Must satisfy the requirements of [`bytemuck::PodInOption`] and [`bytemuck::NoUninit`],
/// although it doesn't need to actually implement the traits.
pub unsafe trait NonZeroInner: super::bounds::BasicBounds {}

/// A type which has an associated [`NonZeroInner`]
///
/// Used to emulate the generic type [`core::num::NonZero`] (which requires Rust 1.79).
pub trait NonZeroAble {
    type NonZero: NonZeroInner;
}
macro_rules! non_zero_able {
    ($($target:ident => $nonzero:ident),+ $(,)?) => {
        $(impl NonZeroAble for $target {
            type NonZero = core::num::$nonzero;
        }
        // SAFETY: We know NonZero can be transmuted back/forth from $target
        unsafe impl NonZeroInner for core::num::$nonzero {}
        )*
        const _VERIFY_NONZERO_SIZES: () = {
            $(
                assert!(size_of::<$target>() == size_of::<core::num::$nonzero>());
            )*
        };
    };
}
non_zero_able! {
    u8 => NonZeroU8,
    u16 => NonZeroU16,
    u32 => NonZeroU32,
    u64 => NonZeroU64,
    u128 => NonZeroU128,
    usize => NonZeroUsize,
    i8 => NonZeroI8,
    i16 => NonZeroI16,
    i32 => NonZeroI32,
    i64 => NonZeroI64,
    i128 => NonZeroI128,
    isize => NonZeroIsize,
}

pub trait PrivateInt: Sized + Siblings + NonZeroAble {
    const BITS: u32;
    const ZERO: Self;
    const ONE: Self;
    const MAX: Self;
    const MIN: Self;
    /// The minimum value of the type, minus one.
    ///
    /// Used for the [`crate::num::NonMax`] abstraction.
    const MIN_PLUS_ONE: Self;
    const SIGNED: bool;
    fn checked_cast<U: PrimitiveInt>(self) -> Option<U>;
    fn wrapping_cast<U: PrimitiveInt>(self) -> U;
    /// The type name as a short unqualified string.
    const TYPE_NAME: &'static str;
    fn checked_add(self, other: Self) -> Option<Self>;
    fn checked_sub(self, other: Self) -> Option<Self>;
    fn checked_mul(self, other: Self) -> Option<Self>;
    fn checked_div(self, other: Self) -> Option<Self>;
    fn checked_rem(self, other: Self) -> Option<Self>;
    fn checked_pow(self, exp: u32) -> Option<Self>;
    fn wrapping_add(self, other: Self) -> Self;
    fn wrapping_sub(self, other: Self) -> Self;
    fn trailing_zeros(self) -> u32;
    fn leading_zeros(self) -> u32;
    fn count_ones(self) -> u32;
    fn from_usize_checked(val: usize) -> Option<Self>;
    fn from_usize_wrapping(val: usize) -> Self;
    #[allow(clippy::wrong_self_convention)]
    fn to_usize_wrapping(this: Self) -> usize;
    #[allow(clippy::wrong_self_convention)]
    fn to_usize_checked(this: Self) -> Option<usize>;
}
macro_rules! impl_primint {
    (@wrapping_cast $target:ident <= [$($src:ident),+ $(,)?]) => {
        $(impl WrappingCastFrom<$src> for $target {
            #[inline]
            fn wrapping_cast_from(value: $src) -> Self {
                value as $target
            }
        })*
    };
    ($($target:ident),+ $(,)?) => ($(
        impl super::PrimitiveInt for $target {}
        #[deny(unconditional_recursion)]
        impl PrivateInt for $target {
            const TYPE_NAME: &'static str = stringify!($target);
            const ZERO: Self = 0;
            const SIGNED: bool = $target::MIN != 0;
            const BITS: u32 = $target::BITS;
            const ONE: Self = 1;
            const MAX: Self = $target::MAX;
            const MIN: Self = $target::MIN;
            const MIN_PLUS_ONE: Self = Self::MIN + 1;
            #[inline]
            fn checked_cast<U: super::PrimitiveInt>(self) -> Option<U> {
                U::try_from(self).ok()
            }
            #[inline]
            fn wrapping_cast<U: PrimitiveInt>(self) -> U {
                <U as WrappingCastFrom<$target>>::wrapping_cast_from(self)
            }
            #[inline]
            #[cfg(any())]
            fn wrapping_cast<U: PrimitiveInt>(self) -> U {
                // implemented in terms of bitcasts,
                // this uses unsafe code in the hopes of reducing compile time
                // turns out it doesn't help much at all
                if Self::SIGNED {
                    // after expanding to the largest signed int,
                    // truncation becomes valid
                    //
                    // the expansion can't use truncation directly
                    // due to potential need for sign extension
                    let expanded = (self as LargestSignedInt);
                    // SAFETY: We know dest <= src and both types are bytemuck::Pod
                    unsafe {
                        core::mem::transmute_copy(&expanded)
                    }
                } else {
                    let value = (self as LargestUnsignedInt);
                    // SAFETY: We know dest <= src and both types are bytemuck::Pod
                    unsafe { core::mem::transmute_copy(&value) }
                }
            }
            #[inline]
            fn checked_add(self, other: Self) -> Option<Self> {
                <$target>::checked_add(self, other)
            }
            #[inline]
            fn checked_sub(self, other: Self) -> Option<Self> {
                <$target>::checked_sub(self, other)
            }
            #[inline]
            fn checked_mul(self, other: Self) -> Option<Self> {
                <$target>::checked_mul(self, other)
            }
            #[inline]
            fn checked_div(self, other: Self) -> Option<Self> {
                <$target>::checked_div(self, other)
            }
            #[inline]
            fn checked_rem(self, other: Self) -> Option<Self> {
                <$target>::checked_rem(self, other)
            }
            #[inline]
            fn checked_pow(self, exp: u32) -> Option<Self> {
                <$target>::checked_pow(self, exp)
            }
            #[inline]
            fn wrapping_add(self, other: Self) -> Self {
                <$target>::wrapping_add(self, other)
            }
            #[inline]
            fn wrapping_sub(self, other: Self) -> Self {
                <$target>::wrapping_sub(self, other)
            }
            #[inline]
            fn count_ones(self) -> u32 {
                <$target>::count_ones(self)
            }
            #[inline]
            fn leading_zeros(self) -> u32 {
                <$target>::leading_zeros(self)
            }
            #[inline]
            fn trailing_zeros(self) -> u32 {
                <$target>::trailing_zeros(self)
            }
            #[inline]
            fn from_usize_checked(val: usize) -> Option<Self> {
                <$target>::try_from(val).ok()
            }
            #[inline]
            #[allow(clippy::cast_possible_truncation)] // desired functionality
            fn from_usize_wrapping(val: usize) -> Self {
                val as $target
            }
            #[inline]
            #[allow(clippy::cast_possible_truncation)] // desired functionality
            fn to_usize_wrapping(this: Self) -> usize {
                this as usize
            }
            #[inline]
            fn to_usize_checked(this: Self) -> Option<usize> {
                usize::try_from(this).ok()
            }
        }
        impl_primint!(
            @wrapping_cast $target <= [
                i8, i16, i32, i64, i128, isize,
                u8, u16, u32, u64, u128, usize
            ]
        );
        impl crate::bounds::ConvertPrimInts for $target {}
        impl crate::bounds::IntOps for $target {}
    )*
    const _ASSERT_COMMON: () = {
        $(
            assert!(size_of::<$target>() <= size_of::<LargestSignedInt>());
            assert!(size_of::<$target>() <= size_of::<LargestUnsignedInt>());
            assert!(size_of::<$target>() == size_of::<<$target as Siblings>::Signed>());
            assert!(size_of::<$target>() == size_of::<<$target as Siblings>::Unsigned>());
        )*
    };
    );
}
impl_primint!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize,);
macro_rules! impl_signed {
    ($($target:ident),+ $(,)?) => {
        $(
        impl super::SignedPrimInt for $target {}
        )*
        const _ASSERT_SIGNED: () = {
            $(assert!(<$target as PrivateInt>::SIGNED);)*
        };
    };
}
macro_rules! impl_unsigned {
    ($($target:ident),+ $(,)?) => {
        $(impl super::UnsignedPrimInt for $target {})*
        const _ASSERT_UNSIGNED: () = {
            $(assert!(!<$target as PrivateInt>::SIGNED);)*
        };
    };
}
impl_signed!(i8, i16, i32, i64, i128, isize);
impl_unsigned!(u8, u16, u32, u64, u128, usize);
macro_rules! impl_siblings {
    ($($unsigned:ident => $signed:ident),+ $(,)?) => {
        $(impl Siblings for $unsigned {
            type Signed = $signed;
            type Unsigned = $unsigned;
        }
        impl Siblings for $signed {
            type Signed = $signed;
            type Unsigned = $unsigned;
        })*
    };
}
impl_siblings! {
    u8 => i8,
    u16 => i16,
    u32 => i32,
    u64 => i64,
    u128 => i128,
    usize => isize,
}
