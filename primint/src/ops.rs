use crate::PrimitiveInt;
use crate::private::PrivateInt;

mod panics;

/// Add the specified value to the integer,
/// returning `None` if overflow occurs.
#[inline]
pub fn checked_add<T: PrimitiveInt>(left: T, right: T) -> Option<T> {
    PrivateInt::checked_add(left, right)
}

/// Subtract the specified value from the integer,
/// returning `None` if overflow occurs.
#[inline]
pub fn checked_sub<T: PrimitiveInt>(left: T, right: T) -> Option<T> {
    PrivateInt::checked_sub(left, right)
}

/// Multiply the specified values together,
/// returning `None` if overflow occurs.
#[inline]
pub fn checked_mul<T: PrimitiveInt>(left: T, right: T) -> Option<T> {
    PrivateInt::checked_mul(left, right)
}

/// Divide the `dividend` by the `divisor`,
/// returning `None` if overflow occurs or `divisor` is zero.
///
/// See [`i32::checked_div`] for more details.
#[inline]
pub fn checked_div<T: PrimitiveInt>(dividend: T, divisor: T) -> Option<T> {
    PrivateInt::checked_div(dividend, divisor)
}

/// Add the specified values together, wrapping around on overflow.
#[inline]
pub fn wrapping_add<T: PrimitiveInt>(left: T, right: T) -> T {
    PrivateInt::wrapping_add(left, right)
}

/// Subtract the specified values, wrapping around on overflow.
#[inline]
pub fn wrapping_sub<T: PrimitiveInt>(left: T, right: T) -> T {
    PrivateInt::wrapping_sub(left, right)
}

/// Take the remainder of dividing the `dividend` by the `divisor`,
/// returning `None` if overflow occurs or `divisor` is zero.
///
/// See [`i32::checked_rem`] for more details.
#[inline]
pub fn checked_rem<T: PrimitiveInt>(dividend: T, divisor: T) -> Option<T> {
    PrivateInt::checked_rem(dividend, divisor)
}

/// Raise `base` to the power of `power`,
/// returning `None` if overflow occurs.
///
/// See [`i32::checked_pow`] for more details.
#[inline]
pub fn checked_pow<T: PrimitiveInt>(base: T, power: u32) -> Option<T> {
    PrivateInt::checked_pow(base, power)
}

/// Get the number of trailing zeroes for the specified integer.
///
/// See [`u64::trailing_zeros`] for details.
#[inline]
pub fn trailing_zeros<T: PrimitiveInt>(val: T) -> u32 {
    PrivateInt::trailing_zeros(val)
}

/// Get the number of leading zeroes for the specified integer.
///
/// You may want to consider the [`ilog2`] function.
///
/// See [`u64::leading_zeros`] for details.
#[inline]
pub fn leading_zeros<T: PrimitiveInt>(val: T) -> u32 {
    PrivateInt::leading_zeros(val)
}

/// Get the number of trailing ones for the specified integer.
///
/// See [`u64::trailing_ones`] for details.
#[inline]
pub fn trailing_ones<T: PrimitiveInt>(val: T) -> u32 {
    trailing_zeros(!val)
}

/// Get the number of leading ones for the specified integer.
///
/// See [`u64::leading_ones`] for details.
#[inline]
pub fn leading_ones<T: PrimitiveInt>(val: T) -> u32 {
    leading_zeros(!val)
}

/// Count the number of one bits in the specified integer.
///
/// See [`u64::count_ones`] for details.
#[inline]
pub fn count_ones<T: PrimitiveInt>(val: T) -> u32 {
    PrivateInt::count_ones(val)
}

/// Count the number of zero bits in the specified integer.
///
/// See [`u64::count_zeros`] for details.
#[inline]
pub fn count_zeros<T: PrimitiveInt>(val: T) -> u32 {
    count_ones(!val)
}

/// Returns the base 2 logarithm of `value`, rounded down.
///
/// This function will panic if `value <= 0`.
///
/// See [`i64::ilog2`] for details,
/// and [`checked_ilog2`] to avoid a panic.
#[inline]
#[track_caller]
pub fn ilog2<T: PrimitiveInt>(val: T) -> u32 {
    match checked_ilog2(val) {
        Some(val) => val,
        None => panics::ilog_negative(),
    }
}

/// Returns the base 2 logarithm of `value`, rounded down.
/// Returns `None` if `value <= 0`.
///
/// See [`i64::checked_ilog2`] for details.
#[inline]
pub fn checked_ilog2<T: PrimitiveInt>(val: T) -> Option<u32> {
    // https://github.com/rust-lang/rust/blob/1.97.1/library/core/src/num/int_macros.rs#L3589-L3595
    if val <= crate::zero() {
        None
    } else {
        Some(T::BITS - 1 - PrivateInt::leading_zeros(val))
    }
}
