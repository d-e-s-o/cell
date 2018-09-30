// lib.rs

// Original work Copyright 2014 The Rust Project Developers.
// Modified work Copyright 2018 Daniel Mueller (deso@posteo.net).
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(optin_builtin_traits)]
#![deny(
  future_incompatible,
  missing_debug_implementations,
  missing_docs,
  rust_2018_compatibility,
  unused_import_braces,
  unused_results,
  warnings,
)]

//! A replacement of std::cell::RefCell adding advanced support for
//! mapping borrows.

#[allow(unused)]
mod cell;
mod fmt;

pub use cell::Ref;
pub use cell::RefCell;
pub use cell::RefMut;
