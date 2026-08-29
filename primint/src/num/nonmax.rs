use core::fmt::{Display, Formatter};
use core::ops::{BitAnd, BitAndAssign};

use crate::UnsignedPrimInt;
use crate::num::NonZero;

/// An integer which is known not to be equal to the maximum value for the type.
///
/// Mirrors the [`core::num::NonZero`] type, but with a different forbidden value.
/// This offers an alternative to the types in the [`nonmax`](https://docs.rs/nonmax) crate,
/// which don't offer the nice generic syntax that [`core::num::NonZero`] does.
///
/// Currently, this type is limited to unsigned primitive integers.
/// This restriction may be lifted in a future version.
///
/// # Safety
/// As `T` is limited to builtin primitive integers,
/// The correctness of this type can be relied upon for unsafe code.
///
/// It is guaranteed that `Option<NonZero<T>>` is the same size as `T`.
/// However, the underlying representation of the type is not currently guaranteed.
/// Right now, it is the numeric value plus one (so `NonMax::new(0)` is represented as 1).
/// However, this may change in the future without notice (so even in patch versions).
///
/// Note that [`NonMax`] does not currently implement [`bytemuck::Contiguous`],
/// as that would expose the underlying representation.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[repr(transparent)]
pub struct NonMax<T: UnsignedPrimInt> {
    value_plus_one: NonZero<T>,
}
impl<T: UnsignedPrimInt> NonMax<T> {
    /// Creates a [`NonMax`] if the value is not zero,
    /// or `None` if the value is zero.
    #[inline]
    pub const fn new(val: T) -> Option<Self> {
        if crate::const_ops::is_max_val(val) {
            None
        } else {
            // SAFETY: Just verified we are not the maximum value
            Some(unsafe { Self::new_unchecked(val) })
        }
    }
    /// Creates [`NonMax`] without checking whether the value is in bounds.
    /// Undefined behavior occurs if the value is actually the maximum value for the type
    ///
    /// # Safety
    /// Value must be less than the maximum value of `T`.
    #[inline]
    pub const unsafe fn new_unchecked(val: T) -> Self {
        // since passing a max val is already UB, we are free to panic here
        debug_assert!(!crate::const_ops::is_max_val(val));
        // Using unchecked_add might be slightly faster, but would require an increased MSRV or rust version macros
        // Better to just wait for NonMax to be added to stdlib
        // SAFETY: Adding one to a non-max value ends up with something nonzero
        unsafe {
            NonMax {
                value_plus_one: NonZero::new_unchecked(crate::const_ops::wrapping_inc(val)),
            }
        }
    }

    /// Get the underlying integer value.
    ///
    /// The result will never be the maximum value of `T`.
    #[inline]
    pub const fn get(self) -> T {
        // use of unchecked_sub here might be faster, but see above for comments
        crate::const_ops::wrapping_dec(self.value_plus_one.get())
    }

    /// The minimum value for this type.
    ///
    /// This always matches the minimum value of the underlying integer type.
    pub const MIN: Self = NonMax {
        // SAFETY: For all integers, MIN + 1 is nonzero
        value_plus_one: unsafe { NonZero::new_unchecked(T::MIN_PLUS_ONE) },
    };
    /// The maximum value for this type.
    ///
    /// Always one less than the maximum value of the underlying type.y
    pub const MAX: Self = NonMax {
        // SAFETY: The maximum value is never zero.
        value_plus_one: unsafe { NonZero::new_unchecked(T::MAX) },
    };
}
impl<T: UnsignedPrimInt> Default for NonMax<T> {
    #[inline]
    fn default() -> Self {
        // SAFETY: We know that zero is not the maximum value for any primint
        unsafe { Self::new_unchecked(T::ONE) }
    }
}
impl<T: UnsignedPrimInt> Display for NonMax<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.get())
    }
}
impl<T: UnsignedPrimInt> BitAnd<T> for NonMax<T> {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: T) -> Self::Output {
        // SAFETY: Since we are non-maximum, at least one of our bits is unset.
        // It follows that one of the result bits is unset and so the int is not maximum
        unsafe { NonMax::new_unchecked(self.get() & rhs) }
    }
}
impl<T: UnsignedPrimInt> BitAnd for NonMax<T> {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        self & rhs.get()
    }
}
impl<T: UnsignedPrimInt> BitAndAssign<T> for NonMax<T> {
    #[inline]
    fn bitand_assign(&mut self, rhs: T) {
        *self = *self & rhs;
    }
}
impl<T: UnsignedPrimInt> BitAndAssign for NonMax<T> {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

macro_rules! primint_conversions {
    ($($prim:ident),+ $(,)?) => {
        $(
            impl From<NonMax<$prim>> for $prim {
                #[inline]
                fn from(x: NonMax<$prim>) -> Self {
                    x.get()
                }
            }
        )*
    }
}
primint_conversions!(u8, u16, u32, u64, u128, usize);

#[cfg(feature = "bytemuck")]
mod bytemuck_impls {
    use super::NonMax;
    use crate::UnsignedPrimInt;

    #[cfg(feature = "bytemuck")]
    // SAFETY: Follows from the fact we wrap a NonZero
    unsafe impl<T: UnsignedPrimInt> bytemuck::PodInOption for NonMax<T> {}
    #[cfg(feature = "bytemuck")]
    // SAFETY: Follows from the fact we wrap a NonZero
    unsafe impl<T: UnsignedPrimInt> bytemuck::ZeroableInOption for NonMax<T> {}
    #[cfg(feature = "bytemuck")]
    // SAFETY: Follows from the fact we wrap a NonZero
    unsafe impl<T: UnsignedPrimInt> bytemuck::NoUninit for NonMax<T> {}
}

#[cfg(feature = "serde")]
mod serde_impls {
    use core::fmt::Formatter;
    use core::marker::PhantomData;
    use core::mem::size_of;

    use serde::Serializer;
    use serde::de::{Error, Unexpected};

    use super::NonMax;
    use crate::UnsignedPrimInt;

    impl<'a, T: UnsignedPrimInt> serde::Deserialize<'a> for NonMax<T> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'a>,
        {
            // serde uses a visitor https://github.com/serde-rs/serde/blob/v1.0.229/serde_core/src/de/impls.rs#L92-L100
            // this gives a better expecting() message but would be harder to implement
            let value: T = T::deserialize(deserializer)?;
            assert!(size_of::<T>() <= 16);
            Self::new(value).ok_or_else(|| {
                D::Error::invalid_value(
                    if size_of::<T>() == 16 {
                        Unexpected::Other(if T::SIGNED { "i128::MAX" } else { "u128::MAX" })
                    } else if T::SIGNED {
                        Unexpected::Signed(T::MAX.checked_cast().unwrap())
                    } else {
                        Unexpected::Unsigned(T::MAX.checked_cast().unwrap())
                    },
                    &ExpectedDesc::<T>(PhantomData),
                )
            })
        }
    }

    impl<T: UnsignedPrimInt> serde::Serialize for NonMax<T> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            self.get().serialize(serializer)
        }
    }

    /// Describes what we expect, implementing [`serde::de::Expected`]
    struct ExpectedDesc<T>(PhantomData<T>);
    impl<T: UnsignedPrimInt> serde::de::Expected for ExpectedDesc<T> {
        fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
            write!(f, "a {tname} (except {tname}::MAX)", tname = T::TYPE_NAME)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::UnsignedPrimInt;

    #[test]
    fn basic_roundtrip() {
        fn roundtrip<T: UnsignedPrimInt>(val: T) {
            let item = super::NonMax::new(val).unwrap_or_else(|| panic!("Maximum value {val}"));
            assert_eq!(item.get(), val);
        }
        macro_rules! basic_roundtrip {
            ($($t:ident),+) => {
                $(roundtrip::<$t>(crate::zero());
                roundtrip::<$t>(crate::one());
                roundtrip::<$t>(2u8.into());
                roundtrip::<$t>(crate::max_value::<$t>() - 2);
                roundtrip::<$t>(crate::max_value::<$t>() - 1);)*
            };
        }
        basic_roundtrip!(u8, u16, u32, u64, u128, usize);
    }
}
