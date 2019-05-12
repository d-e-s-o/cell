Unreleased
----------
- Added `RefValMut`, similar `RefVal` but with ref cell borrowed mutably


0.1.6
-----
- Added `map` method to `RefVal` struct


0.1.5
-----
- Enabled CI pipeline comprising building, testing, and linting of the
  project
- Added badges indicating pipeline status, current `crates.io` published
  version of the crate, current `docs.rs` published version of the
  documentation, and minimum version of `rustc` required


0.1.4
-----
- Updated original `cell` baseline to Rust 1.31.1
- Adjusted crate to use Rust Edition 2018
- Removed `#![deny(warnings)]` attribute and demoted lints prone to
  future changes from `deny` to `warn`


0.1.3
-----
- Dropped dependency on `optin_builtin_traits` feature and made crate
  compilable with stable toolchain


0.1.2
-----
- Added `crate` prefix to imports to make crate ready for 2018 edition


0.1.1
-----
- Added reference to Rust issue for discussion of the problem solved
  to README


0.1.0
-----
- Initial release
