# Changelog

Notable changes to this project should be documented in this file.
Make sure it is up to date before performing a release.

This project follows the [Keep a Changelog](https://keepachangelog.com/en/2.0.0/) format wherever that is reasonable.

The "title" of each release should be its first line.
A title is required for publishing a github release, so all versions should have one.

## Unreleased

### Added
- Add `wrapping_add` and `wrapping_sub` functions (wznskrrp)

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
