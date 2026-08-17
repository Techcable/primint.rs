use crate::PrimitiveInt;
use crate::private::PrivateInt;

/// Cast from one [`PrimitiveInt`] into another,
/// returning `None` if there is overflow.
#[inline]
pub fn checked_cast<T: PrimitiveInt, U: PrimitiveInt>(value: T) -> Option<U> {
    PrivateInt::checked_cast(value)
}

/// Cast from one [`PrimitiveInt`] into another,
/// wrapping around on overflow.
///
/// This has the same behavior as the `as` operator,
/// including performing appropriate sign extension when casting between signed integers.c
#[inline]
pub fn wrapping_cast<T: PrimitiveInt, U: PrimitiveInt>(value: T) -> U {
    T::wrapping_cast(value)
}

/// Convert a primitive integer to a [`usize`],
/// returning `None` if overflow occurs.
#[inline]
pub fn to_usize_checked<T: PrimitiveInt>(val: T) -> Option<usize> {
    T::to_usize_checked(val)
}

/// Convert a primitive integer to a [`usize`],
/// wrapping around on overflow.
#[inline]
pub fn to_usize_wrapping<T: PrimitiveInt>(val: T) -> usize {
    T::to_usize_wrapping(val)
}

/// Convert a primitive integer to a [`usize`],
/// returning `None` if overflow occurs.
#[inline]
pub fn from_usize_checked<T: PrimitiveInt>(val: usize) -> Option<T> {
    T::from_usize_checked(val)
}

/// Convert a primitive integer to a [`usize`],
/// wrapping around if overflow occurs.
#[inline]
pub fn from_usize_wrapping<T: PrimitiveInt>(val: usize) -> T {
    T::from_usize_wrapping(val)
}
