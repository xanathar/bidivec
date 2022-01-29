//! A module containing iterator types for bidimensional data structures.
//!
//! Bidimensional iterators are created by `iter` and `iter_mut` methods
//! on the data structure and view traits of this crate, such as:
//! * [`BidiView::iter()`] / [`BidiViewMutIterable::iter_mut()`]
//! * [`BidiArray::iter()`] and [`BidiArray::iter_mut()`]
//! * [`BidiVec::iter()`] and [`BidiVec::iter_mut()`]
//! * [`BidiGrowVec::iter()`] and [`BidiGrowVec::iter_mut()`]
//! * [`BidiMutSlice::iter()`] and [`BidiMutSlice::iter_mut()`]
//! * [`BidiSlice::iter()`]
//!
//! # Immutable iterators
//!
//! Bidimensional iterators allow one to refine the iteration to
//! a subrectangle:
//! ```
//! # use bidivec::{BidiVec, bidivec, BidiRect};
//!
//! let bvec = bidivec!{
//!     [1, 2, 3],
//!     [4, 5, 6],
//!     [7, 8, 9],
//! };
//!
//! let v = bvec.iter().on_row(1).copied().collect::<Vec<i32>>();
//!
//! assert_eq!(v, vec![4, 5, 6]);
//! ```
//!
//! Other methods allow similar restrictions to a specific range,
//! or to iterate by columns rather than by rows.
//!
//! Additionally, the original coordinate can be returned, as in:
//! ```
//! # use bidivec::{BidiVec, bidivec, BidiRect};
//!
//! let bvec = bidivec!{
//!     [1, 2, 3],
//!     [4, 5, 6],
//!     [7, 8, 9],
//! };
//!
//! for (x, y, i) in bvec.iter().with_coords() {
//!     println!("The element {} is at {}, {}", i, x, y);
//! }
//! ```
//!
//! # Mutable iterators
//!
//! Mutable iterators work in a similar way to immutable ones, and
//! support the same kind of refinements that immutable iterators do:
//!
//! ```
//! # use bidivec::{BidiVec, bidivec, BidiRect};
//!
//! let mut bvec = bidivec!{
//!     [1, 2, 3],
//!     [4, 5, 6],
//!     [7, 8, 9],
//! };
//!
//! for item in bvec
//!     .iter_mut()
//!     .on_rect(&BidiRect::new(1, 1, 2, 2,))
//! {
//!     *item = -(*item);
//! }
//!
//! let v = bvec.iter().copied().collect::<Vec<i32>>();
//!
//! assert_eq!(v, vec![1, 2, 3, 4, -5, -6, 7, -8, -9]);
//! ```
//! # Panics
//!
//! Calling the methods altering the iteration ([`by_column`, `on_row`,
//! `on_column`, `on_rect`, `with_coords`, etc.) after the iteration has
//! been started will cause a panic.

#[cfg(doc)]
use crate::*;

mod borderstate;
pub mod immutable;
pub mod immutable_xy;
pub mod mutable;
pub mod mutable_xy;
mod rectstate;

pub use immutable::iter::Iter;
pub use mutable::iter::IterMut;
