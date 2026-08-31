//! An abstraction over the primitive integer types.
//!
//! Originally written for the [intid](https://crates.io/crates/intid) crate,
//! but later extracted for independent use.
//!
//! This is lighter weight than [num-traits](https://github.com/rust-num/num-traits) crate.
//! More importantly, the [`primint::PrimitiveInt`] trait is sealed and
//! restricted only to primitive integers while [`num_traits::Primint`] can be implemented by any crate.
//!
//! All operations are implemented as module-level functions to avoid conflicts
//! with other traits and with inherent impls.
//! Even constants like zero are accessible through `const fn` rather than associated consts.
//!
//! [`primint::PrimitiveInt`]: crate::PrimitiveInt
//! [`num_traits::PrimInt`]: https://docs.rs/num-traits/0.2/num_traits/int/trait.PrimInt.html
#![cfg_attr(not(test), no_std)]

mod bounds;
mod casts;
#[cfg(any(feature = "nonzero", feature = "nonmax"))]
mod const_ops;
mod constants;
mod ops;
pub(crate) mod private;

pub mod fmt;
pub mod num;

pub use self::casts::*;
pub use self::constants::*;
#[cfg(feature = "nonmax")]
pub use self::num::NonMax;
#[cfg(feature = "nonzero")]
pub use self::num::NonZero;
pub use self::ops::*;

/// An primitive integer.
///
/// Most methods in this trait are only available as module-level functions in the crate root
/// to avoid conflict with inherent implementations and other traits.
///
/// You can get access to more functionality by enabling the following features:
/// - `num-traits-02` adds the [`num_traits_02::PrimInt`] bound
/// - `bytemuck` adds the [`bytemuck::Pod`] bound
/// - `serde` adds the [`serde::Serialize`] and [`serde::de::DeserializeOwned`] bound
///
/// This trait does not use the name `PrimInt` to avoid conflict with [`num_traits_02::PrimInt`].
///
/// # Safety
/// This trait is sealed and is only implemented by the builtin primitive integer types.
///
/// This means that unsafe code can trust all functionality to behave correctly.
pub trait PrimitiveInt:
    bounds::BasicBounds
    + bounds::IntFmtBounds
    + Default
    + core::str::FromStr<Err = core::num::ParseIntError>
    + bounds::ConvertPrimInts
    + private::PrivateInt
    + bounds::num_trait_02::PrimInt
    + bounds::bytemuck::Pod
    + bounds::bytemuck::Contiguous
    + bounds::IntOps
{
}

/// A primitive integer which is known to be unsigned.
pub trait UnsignedPrimInt: PrimitiveInt + bounds::num_trait_02::Unsigned {}

/// A primitive integer which is known to be signed.
pub trait SignedPrimInt: PrimitiveInt + bounds::num_trait_02::Signed + core::ops::Neg {}
