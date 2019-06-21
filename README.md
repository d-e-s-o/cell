[![pipeline](https://gitlab.com/d-e-s-o/cell/badges/master/pipeline.svg)](https://gitlab.com/d-e-s-o/cell/commits/master)
[![crates.io](https://img.shields.io/crates/v/cell.svg)](https://crates.io/crates/cell)
[![Docs](https://docs.rs/cell/badge.svg)](https://docs.rs/cell)
[![rustc](https://img.shields.io/badge/rustc-1.35+-blue.svg)](https://blog.rust-lang.org/2019/05/23/Rust-1.35.0.html)

cell
====

- [Documentation][docs-rs]
- [Changelog](CHANGELOG.md)

**cell** is a crate providing a revised `RefCell` implementation with
additional mapping capabilities. It can be used as a drop-in replacement
for `std::cell::RefCell` where this additional functionality is required.


The Problem
-----------

A borrowed [`RefCell`][rust-ref-cell] is represented as a
[`Ref`][rust-ref]. Such a `Ref` contains a reference to the data
contained in the `RefCell`. To extend the borrow to a different member
in the original data, the [`map`][rust-ref-map] method can be used.
Using this method a new `Ref` object can be created that contains a
**reference** to a member in the borrowed data.

While having a direct reference to such a data member is appropriate in
many cases, there are some where this is insufficient, and an actual
object that contains such a reference is required.

#### Example

The most prominent example is an iterator. While an iterator internally
keeps a reference to the object being iterated over, it is more than
just a reference to it: it contains state about the progress of the
iteration.

If such an iterator is to be exposed for an object contained in a
`RefCell` that is currently borrowed, the `Ref::map` function is
insufficient:

```rust
struct RefStrings(RefCell<Vec<String>>);

impl RefStrings {
    fn iter(&self) -> Ref<Iter<String>> {
        Ref::map(self.0.borrow(), |x| x.iter())
    }
}
```

```
error[E0308]: mismatched types
 |  Ref::map(self.0.borrow(), |x| x.iter())
 |                                ^^^^^^^^ expected reference, found struct `std::slice::Iter`
 |
 = note: expected type `&_`
            found type `std::slice::Iter<'_, std::string::String>`
```
(Note that required lifetimes have been elided in the example for brevity)


A Solution
----------

This crate provides alternative `RefCell` and `Ref` implementations that
solve this problem by introduction of another mapping method: `map_val`.
This method returns a `RefVal` object. `RefVal` is a new type that is
similar to `Ref` but, instead of embedding a reference to its `T` (a
type parameter), it embeds a value of it. `T` in turn would contain the
actual reference to the borrowed object.

In the above example the only changes that need to happen are the
replacement of `std::cell::RefCell` with `cell::RefCell`, that of
`std::cell::Ref` with `cell::Ref`, and the usage of `Ref::map` instead
of `Ref::map_val`.

```patch
--- test.rs
+++ test.rs
@@ -1,13 +1,14 @@
-use std::cell::Ref;
-use std::cell::RefCell;
+use cell::Ref;
+use cell::RefCell;
+use cell::RefVal;
 use std::slice::Iter;


 struct RefStrings(RefCell<Vec<String>>);

 impl RefStrings {
-    fn iter<'t, 's: 't>(&'s self) -> Ref<'t, Iter<String>> {
-        Ref::map(self.0.borrow(), |x| x.iter())
+    fn iter<'t, 's: 't>(&'s self) -> RefVal<'t, Iter<String>> {
+        Ref::map_val(self.0.borrow(), |x| x.iter())
     }
 }

```

Similar functionality exists for mutable borrow with `RefValMut`.


#### Alternative Implementations

The possibility of providing this functionality by means of a trait has
been investigated but no viable solution has been identified. The main
problem stems from the fact that we require access to `Ref` internals in
order to provide this functionality. Such a trait would alleviate the
need for providing alternative `RefCell` and `Ref` implementations.

No other existing solutions for this problem have been found.

A discussion around the upstreaming of this functionality is tracked by
Rust [issue #54776][rust-issue-54776].


[docs-rs]: https://docs.rs/crate/cell
[rust-ref-cell]: https://doc.rust-lang.org/std/cell/struct.RefCell.html
[rust-ref]: https://doc.rust-lang.org/std/cell/struct.Ref.html
[rust-ref-map]: https://doc.rust-lang.org/std/cell/struct.Ref.html#method.map
[rust-issue-54776]: https://github.com/rust-lang/rust/issues/54776
