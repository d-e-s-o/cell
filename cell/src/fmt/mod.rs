// mod.rs

// Original work Copyright 2013-2015 The Rust Project Developers.
// Modified work Copyright 2018 Daniel Mueller (deso@posteo.net).
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Utilities for formatting and printing strings.

use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;
use std::ops::Deref;

use cell::Ref;
use cell::RefCell;
use cell::RefMut;
use cell::RefVal;


impl<T: ?Sized + Debug> Debug for RefCell<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self.try_borrow() {
            Ok(borrow) => {
                f.debug_struct("RefCell")
                    .field("value", &borrow)
                    .finish()
            }
            Err(_) => {
                // The RefCell is mutably borrowed so we can't look at its value
                // here. Show a placeholder instead.
                struct BorrowedPlaceholder;

                impl Debug for BorrowedPlaceholder {
                    fn fmt(&self, f: &mut Formatter) -> Result {
                        f.write_str("<borrowed>")
                    }
                }

                f.debug_struct("RefCell")
                    .field("value", &BorrowedPlaceholder)
                    .finish()
            }
        }
    }
}

impl<'b, T: ?Sized + Debug> Debug for Ref<'b, T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Debug::fmt(&**self, f)
    }
}

impl<'b, T: ?Sized + Debug> Debug for RefMut<'b, T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Debug::fmt(&*(self.deref()), f)
    }
}

impl<'b, T: Debug> Debug for RefVal<'b, T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Debug::fmt(&*(self.deref()), f)
    }
}
