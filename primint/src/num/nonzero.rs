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
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[repr(transparent)]
pub struct NonZero<T: PrimitiveInt> {
    inner: <T as crate::private::NonZeroAble>::NonZero,
}
impl<T: PrimitiveInt> NonZero<T> {
    /// Creates a [`NonZero`] if the value is not zero,
    /// or `None` if the value is zero.
    #[inline]
    pub fn new(val: T) -> Option<Self> {
        // this comparison prevents this function from being const
        if val == T::ZERO {
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
        // Use of core::ptr::read here requires rust 1.71
        // This function must be const so to implement Self::MIN and Self::MAX
        // SAFETY: Caller guarantees zero cannot happen and we trust private::NonZeroInner
        unsafe { core::ptr::addr_of!(val).cast::<Self>().read() }
    }

    /// Get the underlying integer value.
    ///
    /// The result will never be zero.
    #[inline]
    pub const fn get(self) -> T {
        // SAFETY: The private::NonZeroInteger trait guarantees this is valid
        unsafe { core::ptr::addr_of!(self).cast::<T>().read() }
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
