//! A crate offering bidimensional arrays, vectors and slices, with batteries included.
//! The crate tries to be as generic as possible, and after this, to be reasonably optimized.
//!
//! # Features
//!
//! The crate supports the bidimensional containers in a simple to use way thanks to a mix
//! of macros, iterators and indexing.
//!
//! For example (see [`BidiVec`][crate::BidiVec], [`bidivec!`][crate::bidivec!],
//! [`BidiRect`][crate::BidiRect]):
//!
//! ```
//! use bidivec::{BidiVec, bidivec, BidiRect};
//!
//! // Create a new BidiVec<i32> using a macro
//! let mut bvec = bidivec!{
//!     [1, 2, 3],
//!     [4, 5, 6],
//!     [7, 8, 9],
//! };
//!
//! // Overwrite cell (1,1) - the 5 - with 7+8
//! bvec[(1, 1)] = bvec[(0, 2)] + bvec[(1, 2)];
//!
//! assert_eq!(bvec, bidivec!{
//!     [1, 2, 3],
//!     [4, 15, 6],
//!     [7, 8, 9]
//! });
//!
//! // Using iterators, collect the items in a vec
//! let v = bvec.iter().copied().collect::<Vec<i32>>();
//!
//! // Assert the result is the expected one
//! assert_eq!(v, vec![1, 2, 3, 4, 15, 6, 7, 8, 9]);
//!
//! // Change the sign of all items in the 2x2 rect located at (1,1)
//! for item in bvec
//!     .iter_mut()
//!     .on_rect(&BidiRect::new(1, 1, 2, 2,))
//! {
//!     *item = -(*item);
//! }
//!
//! assert_eq!(bvec, bidivec!{
//!     [1, 2, 3],
//!     [4, -15, -6],
//!     [7, -8, -9]
//! });
//! ```
//!
//! ## Data structures:
//!
//! All data structures offer (if appropriate) support for iterators, fast random access
//! to any specific bidimensional location, insertion and removal of columns and row, rotations,
//! transposition and cropping.
//!
//! The supported data structures are:
//! - [`BidiVec`]: a bidimensional wrapper over [`Vec<T>`] that maintains a linear layout for best
//!   interoperability with native code. Can be constructed with the [`bidivec!`][bidivec!] macro.
//! - [`BidiArray`]: a bidimensional wrapper over `Box<[T]>` that also maintains a linear layout for best
//!   interoperability with native code and remains constant length (but may vary width and height).
//!   Can be constructed with the [`bidiarray!`][bidiarray!] macro.
//! - [`BidiGrowVec`]: a bidimensional wrapper over a `Vec<Vec<T>>` that sacrifices memory
//!   layout and locality to offer better performances when inserting in the middle of the collection.
//!   Can be constructed with the [`bidigrowvec!`][bidigrowvec!]  macro.
//! - [`BidiMutSlice`]: a bidimensional wrapper over a `&mut [T]` slice, sacrificing some
//!   functionality to support an externally provided data store, including in-place transformations.
//! - [`BidiSlice`]: a bidimensional wrapper over a `&[T]` slice, with the same caveats as before,
//!   but immutable.
//!
//! ## Other functionalities:
//!
//! When possible, functionalities (in addition to sometimes being implemented in optimized ways by the appropriate
//! data structures) are applied to the [`BidiView`] and [`BidiViewMut`] traits, that are implemented by
//! all the data strucures and easily implementable by other types.
//!
//! Functionalities include:
//! - [Copy (blitting)][editing] of rectangles of one data structure to another, either through [`Copy`] and [`Clone`] traits
//!   (using the [`editing::copy`] and [`editing::clone_over`] methods) or using a custom blending function
//!   ([`editing::blend`]).
//! - Flood fill with customizable actions and comparisons ([`editing::flood_fill`]).
//! - Transformations implemented to view the data structures as [transposed][BidiView::to_transposed()],
//!   [cropped][BidiView::to_cropped()], [BidiView::to_rotated270ccw()], and more.
//! - In-place transformations for mutable data structures to [transpose][BidiArray::transpose()],
//!   [crop][BidiVec::crop], [rotate][BidiGrowVec::rotate90ccw], etc.
//! - [Iterators][bidiiter], including iterators over portions of data structures, and the possibility of enumerating the
//!   original coordinates together with items.
//! - [Pathfinding][pathfinding] algorithms for 2D tiled maps, doing Djikstra algorithm on single source, multiple destinations and
//!   either Djikstra or A* for singe-source, single-destination.

mod algorithms;
mod areas;
pub mod bidiiter;
mod bidiview;
mod collections;
mod error;
mod macros;

#[cfg(test)]
mod tests;

// areas
pub use crate::areas::bidirect::BidiRect;
pub use crate::areas::bidirect_signed::BidiRectSigned;
pub use crate::areas::neighbours::BidiNeighbours;

// data structures
pub use crate::collections::bidiarray::BidiArray;
pub use crate::collections::bidigrowvec::BidiGrowVec;
pub use crate::collections::bidimutslice::BidiMutSlice;
pub use crate::collections::bidislice::BidiSlice;
pub use crate::collections::bidivec::BidiVec;

// errors
pub use crate::error::BidiError;

// views
pub use crate::bidiview::transforming;
pub use crate::bidiview::{BidiFrom, BidiView, BidiViewMut, BidiViewMutIterable};

// algorithms
pub use algorithms::editing;
pub use algorithms::pathfinding;
