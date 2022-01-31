# bidivec

![GitHub Workflow Status](https://img.shields.io/github/workflow/status/xanathar/bidivec/Rust?label=CI) ![Crates.io](https://img.shields.io/crates/v/bidivec) ![docs.rs](https://img.shields.io/docsrs/bidivec) ![Crates.io](https://img.shields.io/crates/d/bidivec) ![Crates.io](https://img.shields.io/crates/l/bidivec) 

A crate offering bidimensional arrays, vectors and slices, with batteries included.
The crate tries to be as generic as possible, and after this, to be reasonably optimized.

# Features

The crate supports the bidimensional containers in a simple to use way thanks to a mix
of macros, iterators and indexing.

For example:

```rust
use bidivec::*;

// Create a new BidiVec<i32> using a macro
let mut bvec = bidivec!{
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9],
};

// Overwrite cell (1,1) - the 5 - with 7+8
bvec[(1, 1)] = bvec[(0, 2)] + bvec[(1, 2)];

assert_eq!(bvec, bidivec!{
    [1, 2, 3],
    [4, 15, 6],
    [7, 8, 9]
});

// Using iterators, collect the items in a vec
let v = bvec.iter().copied().collect::<Vec<i32>>();

// Assert the result is the expected one
assert_eq!(v, vec![1, 2, 3, 4, 15, 6, 7, 8, 9]);

// Change the sign of all items in the 2x2 rect located at (1,1)
for item in bvec
    .iter_mut()
    .on_rect(&BidiRect::new(1, 1, 2, 2,))
{
    *item = -(*item);
}

assert_eq!(bvec, bidivec!{
    [1, 2, 3],
    [4, -15, -6],
    [7, -8, -9]
});
```

## Data structures:

All data structures offer (if appropriate) support for iterators, fast random access
to any specific bidimensional location, insertion and removal of columns and row, rotations,
transposition and cropping.

The supported data structures are:
- `BidiVec`: a bidimensional wrapper over `Vec<T>` that maintains a linear layout for best
  interoperability with native code. Can be constructed with the `bidivec!` macro.
- `BidiArray`: a bidimensional wrapper over `Box<[T]>` that also maintains a linear layout for best
  interoperability with native code and remains constant length (but may vary width and height).
  Can be constructed with the `bidiarray!` macro.
- `BidiGrowVec`: a bidimensional wrapper over a `Vec<Vec<T>>` that sacrifices memory
  layout and locality to offer better performances when inserting in the middle of the collection.
  Can be constructed with the `bidigrowvec!`  macro.
- `BidiMutSlice`: a bidimensional wrapper over a `&mut [T]` slice, sacrificing some
  functionality to support an externally provided data store, including in-place transformations.
- `BidiSlice`: a bidimensional wrapper over a `&[T]` slice, with the same caveats as before,
  but immutable.

## Other functionalities:

When possible, functionalities (in addition to sometimes being implemented in optimized ways by the appropriate
data structures) are applied to the `BidiView` and `BidiViewMut` traits, that are implemented by
all the data strucures and easily implementable by other types.

Functionalities include:

- Copy (blitting) of rectangles of one data structure to another, either through `Copy` and `Clone` traits
  (using the `editing::copy` and `editing::clone_over` methods) or using a custom blending function
  (`editing::blend`).
- Flood fill with customizable actions and comparisons (`editing::flood_fill`).
- Transformations implemented to view the data structures as transposed, cropped, rotated, and more.
- In-place transformations for mutable data structures to transpose, rotate, crop, etc.
- Iterators, including iterators over portions of data structures, and the possibility of enumerating the
  original coordinates together with items.
- Path-finding algorithms for 2D tiled maps, doing Djikstra algorithm on single source, multiple destinations and
  either Djikstra or A* for singe-source, single-destination.

# Performances

Performances are reasonably fast, for most algorithms, even if raw performances are not the focus of this crate.

On a mobile Ryzen 9, pathfinding between opposite corners on a 2000x2000 map (4 million nodes), without heuristics, takes about 1500ms to find the shortest path, and flood-filling the same size takes 110ms to fill the whole, 70ms to fill a checkerboard pattern.

On a 5000x5000 map (25 million nodes) pathfinding is around 10 seconds and flood-filling around 500ms.

On a 200x200 map (20000 nodes), which is a more reasonable size for this kind of algorithms, pathfinding goes down to 10ms, and flood-filling floats around 1ms.

