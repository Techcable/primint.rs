# Changelog

Notable changes to this project should be documented in this file.
Make sure it is up to date before performing a release.

This project follows the [Keep a Changelog](https://keepachangelog.com/en/2.0.0/) format wherever that is reasonable.

The "title" of each release should be its first line.
A title is required for publishing a github release, so all versions should have one.

## Unreleased

### Fixes
- Only define `NonZero` and `NonMax` aliases if those features have been enabled (qwnmsrvm)
  - Fixes compilation errors present in v0.1.6

## 0.1.6 - 2026-09-03
Guarantee that `NonMax<T>` and `NonZero<T>` implement `Hash` and `Ord` the same as `T`.

Make `Hash` impl for `NonMax` to satisfy this property.
All other trait implementations already satisfied this.

### Changed
- Officially guarantee that `NonMax<T>` and `NonZero<T>` implement `Hash` the same as `T` (zoxoyvnx)
- Similarly guarantee that `NonMax<T>` and `NonZero<T>` implement `Ord` the same as `T` (lzlwsrtp)
  - This has always been the case for both types.

### Fixes
- The `Hash` impl for `NonMax` now behaves the same as the underlying type `T`. (ppmqkvzx)
  - This property has always been true of `NonZero`, but has not been true for `NonMax`

## 0.1.5 - 2026-08-30
Add infallible `From` conversions for `NonMax`, `NonZero`.

### Added
- Add type aliases for `NonMax` and `NonZero` (i.e. `NonZeroU32`, `NonMaxU32`, etc.) (unqpkqks)
- Add infallible `From` conversions for `NonMax`, `NonZero` (xutqswvn)
  - No failable conversions yet, as that would require defining a `TryFromIntError` type.
- Add `BITS` associated constant to `NonMax`, `NonZero` (rslwssvw)
- Add `ONE`, `ZERO` constants to `NonMax` and `ONE` constant to `NonZero` (mkyykwvx)
- Require `PrimitiveInt` to implement int format traits ([`Binary`], [`LowerHex`], etc.) (kkmsmlmr)
  - This was always true of the underlying integer types, but not required by the trait.
- Implement int format traits for `NonZero`, `NonMax` ([`Binary`], [`LowerHex`], etc.) (mrlslmyw)
  - Always implemented by delegating to underlying type, just like stdlib and the [`nonmax` crate].

[`Binary`]: https://doc.rust-lang.org/core/fmt/trait.Binary.html
[`LowerHex`]: https://doc.rust-lang.org/core/fmt/trait.LowerHex.html
[`nonmax` crate]: https://docs.rs/nonmax/0.5/nonmax/

### Changes
- Delegate `Debug` impl for `NonZero`, `NonMax` to underlying type (qnlmtkkx)
  - Makes behavior consistent with stdlib and [`nonmax` crate].
  - Previous behavior for `NonZero` was to format as newtype tuple struct (`0` formats as `NonZero(0)`]
  - Previous behavior for `NonMax` was excessively verbose and exposed internal implementation details.
  - I don't consider this breaking since previous behavior was not guaranteed.
    The new behavior is guaranteed unless the stdlib changes it behavior.

[`nonmax` crate]: https://docs.rs/nonmax/0.5/nonmax/

### Fixes
- Fix `Default` impl for `NonMax` to return zero instead of one.

## 0.1.4 - 2026-08-25
Make `NonZero::new`, `NonMax::{new, get}` a `const fn`.

### Added
- Implement `Display` for `NonZero`, `NonMax` (svrxxntk)
- Implement `BitOr` for `NonZero`, `BitAnd` for `NonMax` (rrlpzwum)

### Changed
- Mark `NonZero::new`, `NonMax::{new, new_unchecked, get}` as `const fn` (vnvwkvzp)

## 0.1.3 - 2026-08-25
Hide `NonZero`, `NonMax` behind on-by-default features.

### Added
- Implement `Default` for `NonMax` (wmvtkkqk)

### Changed
- Hide `NonZero`, `NonMax` behind on-by-default feature flags (nktrlykq)
- Reduce MSRV from 1.71 back to 1.64 (rupzyqzu)

### Fixed
- Restore compatibility with rust versions before 1.83 by avoiding `const_refs_to_cell` feature (issue [#1][issue1]) (nktrlykq)

[issue1]: https://github.com/techcable/primint.rs/issues/1

## 0.1.2 - 2026-08-24
Add `NonZero` and `NonMax` types, generic over `PrimitiveInt`.

### Added
- Add `wrapping_add` and `wrapping_sub` functions (wznskrrp)
- Add `NonZero` struct to mirror `core::num::NonZero`, but made generic over `PrimitiveInt` (xooqvysu)
- Add `NonMax` struct to mirror the `NonZero` struct (msuozwwt)
  - Offers alternative to the `nonmax` crate.
  - Implemented in terms of `NonZero` type as stdlib doesn't offer it natively.

### Changed
- Bump MSRV to 1.71

## 0.1.1
Add `FromStr` bound to `PrimitiveInt`.

### Added
- Add `core::str::FromStr` bound to `PrimitiveInt` (xwvtvksx)

### Fixed
- Remove dead link to nonexistent `PrimitiveInteger` in crate docs (xxxzxqpn)
  The real trait name is actually `PrimitiveInt`.

## 0.1.0
Initial release.

Includes all functionality needed by the [intid.rs] project.

[intid.rs]: https://github.com/DuckLogic/intid.rs
