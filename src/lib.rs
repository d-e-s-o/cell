// lib.rs

// Original work Copyright 2014 The Rust Project Developers.
// Modified work Copyright 2018-2019 Daniel Mueller (deso@posteo.net).
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![warn(
  future_incompatible,
  missing_debug_implementations,
  missing_docs,
  rust_2018_compatibility,
  unused_import_braces,
  unused_results,
)]

//! A replacement of std::cell::RefCell adding advanced support for
//! mapping borrows.

#[allow(unused)]
mod cell;
mod fmt;

pub use crate::cell::Ref;
pub use crate::cell::RefCell;
pub use crate::cell::RefMut;
pub use crate::cell::RefVal;
pub use crate::cell::RefValMut;
