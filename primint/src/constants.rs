use crate::PrimitiveInt;

/// Check if the specified type is a signed integer,
/// or `false` if unsigned.
#[inline]
pub const fn is_signed<T: PrimitiveInt>() -> bool {
    T::SIGNED
}

/// Check if the specified type is an unsigned integer,
/// or `false` if signed.
#[inline]
pub const fn is_unsigned<T: PrimitiveInt>() -> bool {
    !T::SIGNED
}

/// Determine the zero value of the specified [`PrimitiveInt`].
///
/// This function always succeeds (a `NonZero` is not a primitive integer)
#[inline]
pub const fn zero<T: PrimitiveInt>() -> T {
    T::ZERO
}

/// Determine the one value of the specified [`PrimitiveInt`].
#[inline]
pub const fn one<T: PrimitiveInt>() -> T {
    T::ONE
}

/// Determine the maximum value of the specified [`PrimitiveInt`].
#[inline]
pub const fn max_value<T: PrimitiveInt>() -> T {
    T::MAX
}

/// Determine the maximum value of the specified [`PrimitiveInt`].
///
/// For unsigned integers, this is zero.
#[inline]
pub const fn min_value<T: PrimitiveInt>() -> T {
    T::MIN
}

/// Determine the number of bits needed to represent the specified [`PrimitiveInt`].
#[inline]
pub const fn bits<T: PrimitiveInt>() -> u32 {
    T::BITS
}
