//! Utilities for formatting integers, generic over a [`primint::PrimitiveInt`](crate::PrimitiveInt).

use core::fmt::{Debug, Display, Formatter};

use crate::PrimitiveInt;

/// Attempt to describe the specified [`PrimitiveInt`]
/// in a format suitable for debugging or panic messages.
///
/// This differs from the standard `Display` and `Debug` implementation,
/// because `T::MAX` and `T::MIN (for signed types)` are special-cased.
///
/// *WARNING*: This representation may change without warning in the future,
/// so the exact representation should not be relied upon.
///
/// ## Examples
/// ```
/// use primint::fmt::debug_desc;
/// assert_eq!(
///     debug_desc(3u32).to_string(),
///     "3"
/// );
/// assert_eq!(
///     debug_desc(u32::MAX).to_string(),
///     "u32::MAX"
/// );
/// assert_eq!(
///     debug_desc(i32::MIN).to_string(),
///     "i32::MIN"
/// );
/// assert_eq!(
///     debug_desc(0u32).to_string(),
///     "0"
/// );
/// ```
#[inline]
pub fn debug_desc<T: PrimitiveInt>(value: T) -> DebugDesc<T> {
    DebugDesc(value)
}

/// The description of an unsigned integer returned by [`debug_desc`].
#[derive(Clone)]
pub struct DebugDesc<T: PrimitiveInt>(T);
impl<T: PrimitiveInt> Display for DebugDesc<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        if self.0 == T::MAX {
            f.write_str(T::TYPE_NAME)?;
            f.write_str("::MAX")
        } else if self.0 == T::MIN && T::SIGNED {
            f.write_str(T::TYPE_NAME)?;
            f.write_str("::MIN")
        } else {
            <T as Display>::fmt(&self.0, f)
        }
    }
}
impl<T: PrimitiveInt> Debug for DebugDesc<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

#[cfg(test)]
mod test {
    use super::debug_desc;
    use crate::PrimitiveInt;

    #[track_caller]
    fn assert_debug_desc<T: PrimitiveInt>(val: T, expected: &str) {
        assert_eq!(debug_desc(val).to_string(), expected);
    }

    #[test]
    fn debug_desc_max() {
        // signed min is special cased
        assert_debug_desc(i32::MIN, "i32::MIN");
        assert_debug_desc(isize::MIN, "isize::MIN");
        // unsigned min should instead render as zero
        assert_debug_desc(u32::MIN, "0");
        assert_debug_desc(usize::MIN, "0");
    }

    #[test]
    fn debug_desc_min() {
        assert_debug_desc(i32::MAX, "i32::MAX");
        assert_debug_desc(i64::MAX, "i64::MAX");
        assert_debug_desc(u32::MAX, "u32::MAX");
        assert_debug_desc(usize::MAX, "usize::MAX");
    }
}
