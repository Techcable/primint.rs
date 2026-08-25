# Changelog

Notable changes to this project should be documented in this file.
Make sure it is up to date before performing a release.

This project follows the [Keep a Changelog](https://keepachangelog.com/en/2.0.0/) format wherever that is reasonable.

The "title" of each release should be its first line.
A title is required for publishing a github release, so all versions should have one.

## Unreleased

### Added
- Implement `Default` for `NonMax` (wmvtkkqk)

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
