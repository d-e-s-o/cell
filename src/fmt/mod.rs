// Copyright 2013-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
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

use crate::cell::Ref;
use crate::cell::RefCell;
use crate::cell::RefMut;
use crate::cell::RefVal;
use crate::RefValMut;


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

impl<T: ?Sized + Debug> Debug for Ref<'_, T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Debug::fmt(&**self, f)
    }
}

impl<T: ?Sized + Debug> Debug for RefMut<'_, T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Debug::fmt(&*(self.deref()), f)
    }
}

impl<T: Sized + Debug> Debug for RefVal<'_, T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Debug::fmt(&*(self.deref()), f)
    }
}

impl<T: Sized + Debug> Debug for RefValMut<'_, T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Debug::fmt(&*(self.deref()), f)
    }
}

// If you expected tests to be here, look instead at the run-pass/ifmt.rs test,
// it's a lot easier than creating all of the rt::Piece structures here.
