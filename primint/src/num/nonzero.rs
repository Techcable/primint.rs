use core::cmp::Ordering;
use core::hash::{Hash, Hasher};
use core::mem::size_of;
use core::ops::{BitOr, BitOrAssign};

use crate::PrimitiveInt;

/// An integer which is known not to be zero.
///
/// This allows `Option<NonZero<T>>` to take the same space as `T`.
/// Equivalent to [`core::num::NonZero`], but generic over [`PrimitiveInt`].
///
/// # Safety
/// As `T` is limited to builtin primitive integers,
/// The correctness of this type can be relied upon for unsafe code.
///
/// The representation is guaranteed to exactly match [`core::num::NonZero`].
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct NonZero<T: PrimitiveInt> {
    inner: <T as crate::private::NonZeroAble>::NonZero,
}
impl<T: PrimitiveInt> NonZero<T> {
    /// Creates a [`NonZero`] if the value is not zero,
    /// or `None` if the value is zero.
    #[inline]
    pub const fn new(val: T) -> Option<Self> {
        if crate::const_ops::is_zero(val) {
            None
        } else {
            // SAFETY: Just verified we are not zero
            Some(unsafe { Self::new_unchecked(val) })
        }
    }

    /// Creates a [`NonZero`] without checking whether the value is zero.
    /// Undefined behavior occurs if the value is actually zero.
    ///
    /// # Safety
    /// Value cannot be zero.
    #[inline]
    pub const unsafe fn new_unchecked(val: T) -> Self {
        // since passing a max val is already UB, we are free to panic here
        debug_assert!(!crate::const_ops::is_zero(val));
        // This function must be const so to implement Self::MIN and Self::MAX
        // SAFETY: Caller guarantees zero cannot happen and we trust private::NonZeroInner
        #[allow(clippy::incompatible_msrv)] // nonzero requires Rust 1.74
        unsafe {
            let helper = TransmuteHelper { val };
            helper.nonzero
        }
    }

    /// Get the underlying integer value.
    ///
    /// The result will never be zero.
    #[inline]
    pub const fn get(self) -> T {
        // SAFETY: The private::NonZeroInteger trait guarantees this is valid
        unsafe {
            let helper = TransmuteHelper { nonzero: self };
            helper.val
        }
    }

    /// The minimum value for this type.
    ///
    /// For signed integers, this is the minimum value ([`crate::min_value`]).
    /// For unsigned integers, this is the number `1`.
    pub const MIN: Self = if T::SIGNED {
        // SAFETY: For a signed integer, the min value is nonzero
        unsafe { Self::new_unchecked(T::MIN) }
    } else {
        // SAFETY: We know that one is not zero
        unsafe { Self::new_unchecked(T::ONE) }
    };

    /// The maximum value for this type.
    ///
    /// Always equals the maximum value of the underlying type.
    pub const MAX: Self = {
        // SAFETY: The maximum value is never zero.
        unsafe { Self::new_unchecked(T::MAX) }
    };

    /// The number of bits needed to represent this type.
    ///
    /// Always equals the number of bits of the underlying type.
    pub const BITS: u32 = crate::bits::<T>();

    /// The constant number `1`.
    pub const ONE: Self = {
        // SAFETY: One is never zero
        unsafe { Self::new_unchecked(T::ONE) }
    };
}

impl<T: PrimitiveInt> BitOr<T> for NonZero<T> {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: T) -> Self::Output {
        // SAFETY: bitwise or always nonzero if one of its arguments is
        unsafe { NonZero::new_unchecked(self.get() | rhs) }
    }
}
impl<T: PrimitiveInt> BitOr for NonZero<T> {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        self | rhs.get()
    }
}
impl<T: PrimitiveInt> BitOrAssign<T> for NonZero<T> {
    #[inline]
    fn bitor_assign(&mut self, rhs: T) {
        *self = *self | rhs;
    }
}
impl<T: PrimitiveInt> BitOrAssign for NonZero<T> {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

/// Hashes the underlying value.
///
/// This is guaranteed to give the same result as the underlying type:
/// ```
/// # use std::{hash::BuildHasher, collections::hash_map::RandomState};
/// # #[rustversion::since(1.71)] // need hash_one() function
/// # fn main() {
/// let hasher = RandomState::new();
/// assert_eq!(
///     hasher.hash_one(primint::NonZero::new(3u32).unwrap()),
///     hasher.hash_one(3u32)
/// );
/// # }
/// # #[rustversion::before(1.71)]
/// # fn main() {}
/// ```
///
/// This property has been true in all released versions,
/// but not guaranteed until
impl<T: PrimitiveInt> Hash for NonZero<T> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        // behaves the same as the derive impl since get() is just transmute
        <T as Hash>::hash(&self.get(), state)
    }
}

union TransmuteHelper<T: PrimitiveInt> {
    val: T,
    nonzero: NonZero<T>,
}

macro_rules! primint_conversions {
    ($($prim:ident => $alias:ident),+ $(,)?) => {
        $(
            impl From<core::num::$alias> for NonZero<$prim> {
                #[inline]
                fn from(x: core::num::$alias) -> Self {
                    // SAFETY: We know the value is nonzero
                    unsafe {
                        NonZero::new_unchecked(x.get())
                    }
                }
            }
            impl From<NonZero<$prim>> for core::num::$alias {
                #[inline]
                fn from(x: NonZero<$prim>) -> Self {
                    // SAFETY: We know the value is nonzero
                    unsafe {
                        core::num::$alias::new_unchecked(x.get())
                    }
                }
            }
            impl From<NonZero<$prim>> for $prim {
                #[inline]
                fn from(x: NonZero<$prim>) -> Self {
                    x.get()
                }
            }
        )*
        const _CONVERT_ASSERT: () = {
            $(
                assert!(size_of::<$prim>() == size_of::<core::num::$alias>());
            )*
        };
    }
}
primint_conversions!(
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
);
impl<T: PrimitiveInt> PartialOrd for NonZero<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
    #[inline]
    fn lt(&self, other: &Self) -> bool {
        self.inner.lt(&other.inner)
    }
    #[inline]
    fn le(&self, other: &Self) -> bool {
        self.inner.le(&other.inner)
    }
    #[inline]
    fn gt(&self, other: &Self) -> bool {
        self.inner.gt(&other.inner)
    }
    #[inline]
    fn ge(&self, other: &Self) -> bool {
        self.inner.ge(&other.inner)
    }
}
/// The ordering of a `NonZero<t>` is equivalent to that of the underlying value `T`.
impl<T: PrimitiveInt> Ord for NonZero<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.inner.cmp(&other.inner)
    }
    #[inline]
    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        NonZero {
            inner: self.inner.max(other.inner),
        }
    }
    #[inline]
    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        NonZero {
            inner: self.inner.min(other.inner),
        }
    }
    #[inline]
    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
    {
        NonZero {
            inner: self.inner.clamp(min.inner, max.inner),
        }
    }
}

#[cfg(feature = "bytemuck")]
mod bytemuck_impls {
    use super::NonZero;
    use crate::{PrimitiveInt, UnsignedPrimInt};

    /// For unsigned integers, valid [`NonZero`] instances operate a contiguous range of values.
    ///
    /// For signed integers, there is a whole at zero so this trait cannot be implemented.
    // SAFETY: This is correct for unsigned integers, which we are limited too
    unsafe impl<T: UnsignedPrimInt> bytemuck::Contiguous for NonZero<T> {
        type Int = T;
        const MAX_VALUE: Self::Int = Self::MAX.get();
        const MIN_VALUE: Self::Int = Self::MIN.get();
    }

    // SAFETY: The private::NonoZeroInner trait guarantees this is true
    unsafe impl<T: PrimitiveInt> bytemuck::PodInOption for NonZero<T> {}
    // SAFETY: Follows from the implementation of PodInOption
    unsafe impl<T: PrimitiveInt> bytemuck::ZeroableInOption for NonZero<T> {}
    // SAFETY: The private::NonoZeroInner trait guarantees this is true
    unsafe impl<T: PrimitiveInt> bytemuck::NoUninit for NonZero<T> {}
}

#[cfg(feature = "serde")]
mod serde_impls {
    use core::fmt::Formatter;
    use core::marker::PhantomData;

    use serde::Serializer;
    use serde::de::{Error, Unexpected};

    use super::NonZero;
    use crate::PrimitiveInt;

    impl<'a, T: PrimitiveInt> serde::Deserialize<'a> for NonZero<T> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'a>,
        {
            // serde uses a visitor https://github.com/serde-rs/serde/blob/v1.0.229/serde_core/src/de/impls.rs#L92-L100
            // this gives a better expecting() message but would be harder to implement
            let value: T = T::deserialize(deserializer)?;
            Self::new(value).ok_or_else(|| {
                D::Error::invalid_value(
                    if T::SIGNED {
                        Unexpected::Signed(0)
                    } else {
                        Unexpected::Unsigned(0)
                    },
                    &ExpectedDesc::<T>(PhantomData),
                )
            })
        }
    }

    impl<T: PrimitiveInt> serde::Serialize for NonZero<T> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            self.get().serialize(serializer)
        }
    }

    /// Describes what we expect, implementing [`serde::de::Expected`]
    struct ExpectedDesc<T>(PhantomData<T>);
    impl<T: PrimitiveInt> serde::de::Expected for ExpectedDesc<T> {
        fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
            write!(f, "a nonzero {}", T::TYPE_NAME)
        }
    }
}
