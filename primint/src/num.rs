//! Mirrors the [`core::num`] module.

#[cfg(any(feature = "nonzero", feature = "nonmax"))]
mod noncommon;
#[cfg(feature = "nonmax")]
mod nonmax;
#[cfg(feature = "nonzero")]
mod nonzero;

#[cfg(feature = "nonmax")]
pub use nonmax::NonMax;
#[cfg(feature = "nonzero")]
pub use nonzero::NonZero;

macro_rules! nonzero_aliases {
    ($($alias:ident => $prim:ident),+ $(,)?) => {
        $(#[doc = concat!("A [`", stringify!($prim), "`] that is known to not equal zero.")]
        ///
        #[doc = concat!("This is a convenience alias for [`primint::num::NonZero<", stringify!($prim), ">`](crate::num::NonZero).")]
        pub type $alias = NonZero<$prim>;)*
    };
}

macro_rules! nonmax_aliases {
    ($($alias:ident => $prim:ident),+ $(,)?) => {
        $(#[doc = concat!("A [`", stringify!($prim), "`] that is known to not equal [`", stringify!($prim), "::MAX`].")]
        ///
        #[doc = concat!("This is a convenience alias for [`primint::num::NonMax<", stringify!($prim), ">`](crate::num::NonMax).")]
        pub type $alias = NonMax<$prim>;)*
    };
}

nonzero_aliases! {
    NonZeroU8 => u8,
    NonZeroU16 => u16,
    NonZeroU32 => u32,
    NonZeroU64 => u64,
    NonZeroU128 => u128,
    NonZeroUsize => usize,
    NonZeroI8 => i8,
    NonZeroI16 => i16,
    NonZeroI32 => i32,
    NonZeroI64 => i64,
    NonZeroI128 => i128,
    NonZeroIsize => isize,
}

nonmax_aliases! {
    NonMaxU8 => u8,
    NonMaxU16 => u16,
    NonMaxU32 => u32,
    NonMaxU64 => u64,
    NonMaxU128 => u128,
    NonMaxUsize => usize,
}
