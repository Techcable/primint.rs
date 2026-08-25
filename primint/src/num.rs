//! Mirrors the [`core::num`] module.

#[cfg(feature = "nonmax")]
mod nonmax;
#[cfg(feature = "nonzero")]
mod nonzero;

#[cfg(feature = "nonmax")]
pub use nonmax::NonMax;
#[cfg(feature = "nonzero")]
pub use nonzero::NonZero;
