//! A module containing functions to alter the contents of a [`BidiViewMut`][crate::BidiViewMut],
//! by copy/cloning rectangles of data from another view, flood-filling, etc.
//!
//! The most important entry points are:
//! - [`copy()`]: A function that copies a rectangle from a [`BidiView`][crate::BidiView]
//!   to a [`BidiViewMut`][crate::BidiViewMut]. Requires the type to be [`Copy`].
//! - [`clone_over()`]: A function that copies a rectangle from a [`BidiView`][crate::BidiView]
//!   to a [`BidiViewMut`][crate::BidiViewMut] and works if the type is [`Clone`].
//! - [`blend()`]: A function that operates on a rectangle from a [`BidiView`][crate::BidiView]
//!   and uses a customizable closure to alter a [`BidiViewMut`][crate::BidiViewMut]. Can be used
//!   for copying types that aren't [`Copy`]/[`Clone`], or to blend those types (e.g. alpha-blending),
//!   or for whatever operation the calling code decides.
//! - [`flood_fill()`]: Performs a flood-fill on the [`BidiViewMut`][crate::BidiViewMut], using a custom
//!   comparison closure and a custom action for painting/filling.

use crate::*;
use std::{cmp::min, collections::VecDeque};

/// Copies a rectangle from a [`BidiView`][crate::BidiView] to a [`BidiViewMut`][crate::BidiViewMut].
/// The type is required to be [`Copy`]; if the type is [`Clone`],  see
/// the [`clone_over()`] function. If the type is neither [`Copy`] nor [`Clone`], see the
/// [`blend()`] function.
///
/// # Examples
///
/// ```
/// use bidivec::{bidivec, editing, BidiRect};
///
/// let v1 = bidivec!{
///     [0, 1, 2, 3],
///     [4, 5, 6, 7],
///     [8, 9, 10, 11],
///     [12, 13, 14, 15],
/// };
///
/// let mut v2 = bidivec![-1; 4, 4];
///
/// editing::copy(
///     &v1,
///     &mut v2,
///     &BidiRect::new(0, 0, 2, 2),
///     (0, 0),
/// )?;
///
/// assert_eq!(v2[(0, 0)], 0);
/// assert_eq!(v2[(1, 0)], 1);
/// assert_eq!(v2[(0, 1)], 4);
/// assert_eq!(v2[(1, 1)], 5);
/// assert_eq!(v2[(2, 0)], -1);
/// assert_eq!(v2[(0, 2)], -1);
/// # Ok::<(), bidivec::BidiError>(())
/// ```
pub fn copy<S, D>(
    source: &S,
    dest: &mut D,
    from: &BidiRect,
    to: (usize, usize),
) -> Result<(), BidiError>
where
    S: BidiView,
    D: BidiViewMut<Output = S::Output>,
    S::Output: Copy + Sized,
{
    if from.x >= source.width()
        || from.y >= source.height()
        || to.0 >= dest.width()
        || to.1 >= dest.height()
    {
        return Err(BidiError::OutOfBounds);
    }

    for dy in to.1..min(to.1 + from.height, to.1 + source.height()) {
        let sy = dy - to.1 + from.y;
        for dx in to.0..min(to.0 + from.width, to.0 + source.width()) {
            let sx = dx - to.0 + from.x;
            println!("copying {},{} to {},{}", sx, sy, dx, dy);
            dest[(dx, dy)] = source[(sx, sy)];
        }
    }

    Ok(())
}

/// Clones a rectangle from a [`BidiView`][crate::BidiView] to a [`BidiViewMut`][crate::BidiViewMut].
/// The type is required to be [`Clone`]; if the type is also [`Copy`],  see
/// the [`copy()`] function which might be slightly faster.
/// If the type is neither [`Copy`] nor [`Clone`], see the
/// [`blend()`] function.
///
/// # Examples
///
/// ```
/// use bidivec::{bidivec, editing, BidiRect};
///
/// #[derive(Clone)]
/// struct Cloneable(i32);
///
/// let v1 = bidivec!{
///     [Cloneable(0), Cloneable(1), Cloneable(2), Cloneable(3)],
///     [Cloneable(4), Cloneable(5), Cloneable(6), Cloneable(7)],
///     [Cloneable(8), Cloneable(9), Cloneable(10), Cloneable(11)],
///     [Cloneable(12), Cloneable(13), Cloneable(14), Cloneable(15)],
/// };
/// let mut v2 = bidivec![Cloneable(-1); 4, 4];
///
/// editing::clone_over(
///     &v1,
///     &mut v2,
///     &BidiRect::new(0, 0, 2, 2),
///     (0, 0),
/// )?;
///
/// assert_eq!(v2[(0, 0)].0, 0);
/// assert_eq!(v2[(1, 0)].0, 1);
/// assert_eq!(v2[(0, 1)].0, 4);
/// assert_eq!(v2[(1, 1)].0, 5);
/// assert_eq!(v2[(2, 0)].0, -1);
/// assert_eq!(v2[(0, 2)].0, -1);
/// # Ok::<(), bidivec::BidiError>(())
/// ```
pub fn clone_over<S, D>(
    source: &S,
    dest: &mut D,
    from: &BidiRect,
    to: (usize, usize),
) -> Result<(), BidiError>
where
    S: BidiView,
    D: BidiViewMut<Output = S::Output>,
    S::Output: Clone + Sized,
{
    blend(source, dest, from, to, |s, d| *d = s.clone())
}

/// Blends a rectangle from a [`BidiView`][crate::BidiView] to a [`BidiViewMut`][crate::BidiViewMut]
/// using a closure to customize the behavior.
/// This can be used to implement copies for types that aren't [`Copy`]/[`Clone`],
/// or to support alpha-blending or similar algorithms, being the `blender`
/// function entirely customizable.
///
/// The blender function is a [`FnMut(&S::Output, &mut D::Output)`][FnMut], where the
/// first argument is the element in the source [`BidiView`][crate::BidiView], and
/// the second element is a mutable element in the destination [`BidiViewMut`][crate::BidiViewMut].
///
/// # Examples
///
/// ```
/// use bidivec::{bidivec, editing, BidiRect};
///
/// let v1 = bidivec!{
///     [0, 1, 2, 3],
///     [4, 5, 6, 7],
///     [8, 9, 10, 11],
///     [12, 13, 14, 15],
/// };
/// let mut v2 = bidivec![100; 4, 4];
///
/// editing::blend(
///     &v1,
///     &mut v2,
///     &BidiRect::new(0, 0, 2, 2),
///     (0, 0),
///     |src, dst| *dst = src + 2 * (*dst),
/// )?;
///
/// assert_eq!(v2[(0, 0)], 200);
/// assert_eq!(v2[(1, 0)], 201);
/// assert_eq!(v2[(0, 1)], 204);
/// assert_eq!(v2[(1, 1)], 205);
/// assert_eq!(v2[(2, 0)], 100);
/// assert_eq!(v2[(0, 2)], 100);
/// # Ok::<(), bidivec::BidiError>(())
/// ```
pub fn blend<S, D, F>(
    source: &S,
    dest: &mut D,
    from: &BidiRect,
    to: (usize, usize),
    mut blender: F,
) -> Result<(), BidiError>
where
    S: BidiView,
    D: BidiViewMut,
    D::Output: Sized,
    F: FnMut(&S::Output, &mut D::Output),
{
    if from.x >= source.width()
        || from.y >= source.height()
        || to.0 >= dest.width()
        || to.1 >= dest.height()
    {
        return Err(BidiError::OutOfBounds);
    }

    for dy in to.1..min(to.1 + from.height, to.1 + source.height()) {
        let sy = dy - to.1 + from.y;
        for dx in to.0..min(to.0 + from.width, to.0 + source.width()) {
            let sx = dx - to.0 + from.x;

            blender(&source[(sx, sy)], &mut dest[(dx, dy)]);
        }
    }

    Ok(())
}

/// Performs a flood-fill like operation, using custom comparisons and
/// custom painter.
///
/// A flood-fill starts from a coordinate (`pos`) and expands in all
/// the directions (according to the `neighbouring` argument) over all
/// the values for which the comparison function (`comparer`) returns
/// true.
/// All those values are then "painted" using the `painter` function.
///
/// The `comparer` function is a [`Fn(&V::Output, &V::Output, &V::Output) -> bool`][Fn]
/// that takes a pair of elements and returns `true` if the fill should
/// expand from the second element to the third. The first element is
/// always the element from which the flood-fill started.
///
/// The `painter` function is a [`FnMut(&mut V::Output, (usize, usize))`][FnMut]
/// that takes the element to be written as the first argument, and its
/// coordinates as the second argument.
///
/// Returns the number of elements that have been passed to the painter
/// function (that is, the number of elements to which the flood-fill
/// expanded to, including the starting position).
///
/// # Examples
///
/// ```
/// use bidivec::{bidivec, editing, BidiNeighbours};
///
/// let mut v = bidivec!{
///     [0, 0, 1, 1],
///     [0, 0, 1, 0],
///     [1, 0, 1, 1],
///     [1, 0, 0, 1],
/// };
///
/// editing::flood_fill(
///     &mut v,
///     (0, 0),
///     BidiNeighbours::Adjacent,
///     |_, val1, val2| val1 == val2,
///     |val, _| { *val = 5; },
/// )?;
///
/// assert_eq!(v, bidivec!{
///     [5, 5, 1, 1],
///     [5, 5, 1, 0],
///     [1, 5, 1, 1],
///     [1, 5, 5, 1],
/// });
/// # Ok::<(), bidivec::BidiError>(())
/// ```
pub fn flood_fill<V, FC, FF>(
    dest: &mut V,
    pos: (usize, usize),
    neighbouring: BidiNeighbours,
    comparer: FC,
    mut painter: FF,
) -> Result<usize, BidiError>
where
    V: BidiViewMut,
    V::Output: Sized,
    FC: Fn(&V::Output, &V::Output, &V::Output) -> bool,
    FF: FnMut(&mut V::Output, (usize, usize)),
{
    #[derive(Copy, Clone, PartialEq)]
    enum FloodFillState {
        Unvisited,
        Border,
        Paint,
    }

    if pos.0 >= dest.width() || pos.1 >= dest.height() {
        return Err(BidiError::OutOfBounds);
    }

    let (width, height) = (dest.width(), dest.height());
    let initial_elem = &dest[pos];
    let mut queue = VecDeque::new();
    let mut neighbours = neighbouring.prealloc_vec();

    let mut visited = BidiArray::with_elem(FloodFillState::Unvisited, width, height);

    visited[(pos)] = FloodFillState::Paint;
    queue.push_back(pos);

    while let Some(point) = queue.pop_front() {
        let cur_val = &dest[point];
        neighbouring.generate_points_on(&mut neighbours, point, width, height);

        while let Some(neighbour) = neighbours.pop() {
            if visited[neighbour] != FloodFillState::Unvisited {
                continue;
            }

            if comparer(initial_elem, cur_val, &dest[neighbour]) {
                queue.push_back(neighbour);
                visited[neighbour] = FloodFillState::Paint;
            } else {
                visited[neighbour] = FloodFillState::Border;
            }
        }
    }

    for (x, y, elem) in visited.iter().with_coords() {
        if *elem == FloodFillState::Paint {
            painter(&mut dest[(x, y)], (x, y));
        }
    }

    Ok(visited.len())
}
