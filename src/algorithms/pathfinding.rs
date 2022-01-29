//! This module provides functionality to find the shortest path between two
//! points on a 2D map represented by a [`BidiView`][crate::BidiView], using Djikstra algorithm
//! on single source ([`pathfind_to_whole`]), multiple destinations and either Djikstra or A* for
//! singe-source, single-destination path findings ([`pathfind_to_dest`] and
//! [`pathfind_to_dest_heuristic`]).
//!
//! Parameters of the path are customizable, for example diagonal movement might
//! or might not be allowed (using [`BidiNeighbours`][crate::BidiNeighbours]); movement,
//! however, might happen only between adjacent or bordering tiles, that is,
//! from a tile to the 4 or 8 tiles directly surrounding it.
//!
//! The cost of travelling from one tile to another, if possible at all, is
//! customizable and can be expressed with any unsigned integer, with a floating
//! point number, or even custom types (whatever sense it might make). See
//! the [`PathFindCost`] trait for further details.
//!
//! To get started, start from the functions: [`pathfind_to_whole`], [`pathfind_to_dest`]
//! and [`pathfind_to_dest_heuristic`].
//!
//! # Examples
//!
//! ```
//! use bidivec::*;
//! use bidivec::pathfinding::*;
//!
//! // The width of the map
//! const WIDTH: usize = 9;
//!
//! // The map
//! const DATA: &[u8] = b"\
//!      ##########\
//!      ##  S#   #\
//!      ## ### # #\
//!      ##     # #\
//!      ## ### # #\
//!      ##   # #D#\
//!      ##########\
//! ";
//!
//! pub fn main() -> Result<(), BidiError> {
//!     // Load the map in a BidiSlice
//!     let map = BidiSlice::new(DATA, WIDTH)?;
//!
//!     // Find the source point
//!     let start = map.iter().with_coords().find_map(|(x, y, t)| {
//!         if *t == b'S' {
//!             Some((x, y))
//!         } else {
//!             None
//!         }
//!     }).unwrap();
//!
//!     // Find the destination point
//!     let dest = map.iter().with_coords().find_map(|(x, y, t)| {
//!         if *t == b'D' {
//!             Some((x, y))
//!         } else {
//!             None
//!         }
//!     }).unwrap();
//!
//!     // Find the shortest path
//!     let res = pathfind_to_dest(
//!         &map,
//!         start,
//!         dest,
//!         BidiNeighbours::Adjacent,
//!         |_, _, to, _| if *to == b'#' {
//!             None
//!         } else {
//!             Some(1u32)
//!         }
//!     ).unwrap();
//!
//!     // Print the result of the path
//!     if let PathFindDataResult::ShortestPathFound(cost) = res.result {
//!         for y in 0..map.height() {
//!             for x in 0..map.width() {
//!                 if map[(x, y)] == b' ' && res.tiles[(x, y)].in_shortest_path {
//!                     print!(".");
//!                 } else {
//!                     print!("{}", map[(x, y)] as char)
//!                 }
//!             }
//!             println!();
//!         }
//!
//!         println!("Path cost: {}", cost);
//!     } else {
//!         println!("Path not found 🤷");
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! This will print:
//!
//! ```text
//! $ cargo run --quiet
//! ##########
//! ##**S#***#
//! ##*###*#*#
//! ##*****#*#
//! ## ### #*#
//! ##   # #D#
//! ##########
//! Path cost: 16
//! ```

use crate::*;
use std::cmp::*;
use std::collections::BinaryHeap;
use std::num::FpCategory;
use std::ops::Add;

/// A trait that is implemented by types that can be used to represent the cost in the
/// pathfinding algorithms. An implementation is offered out of the box for [`u16`],
/// [`u32`], [`u64`], [`u128`], [`usize`], [`f32`] and [`f64`]. Usage of one of the
/// unsigned integer types is recommended, however.
///
/// Types implementing this trait must implement at least [`PartialEq`], [`PartialOrd`],
/// [`Copy`], [`Clone`], [`Default`] and [`Add<Output = Self>`][Add].
///
/// Types that implement [`Ord`] and [`Eq`] and do not have negative values are free to
/// use an empty implementation.
///
/// Types that are not [`Ord`] should provide a [`normalize`][PathFindCost::normalize] method
/// that takes a value and returns [`None`] if that value would break [`Ord`].
/// For example, for floating point types, infinites and NaNs are normalized as [`None`]
/// (negative zero is treated as zero).
///
/// # Panics
/// If [`normalize`][PathFindCost::normalize] fails to normalize a value, the
/// pathfinding algorithm may panic or enter an infinite loop. This can never
/// happen with unsigned types correctly implementing [`Ord`].
pub trait PathFindCost:
    PartialEq + PartialOrd + Copy + Clone + Default + Add<Output = Self>
{
    /// Types implementing [`PathFindCost`] that do not have full ordering or that
    /// are signed, should implement `normalize` so that those values that are either
    /// negative or uncomparable, are either normalized to some value or to None.
    ///
    /// Implementors should handle all the edge cases of the types they are implementing
    /// the trait for.
    ///
    /// # Examples
    ///
    /// This is how `normalize` is internally implemented for floating point types.
    ///
    /// ```ignore
    /// // This is how `normalize` is internally implemented for f32.
    /// // This is provided as a reference, don't copy/paste this into your code!
    /// impl PathFindCost for f32 {
    ///     fn normalize(self) -> Option<Self> {
    ///         match self.classify() {
    ///             // Default infinites and NaNs to None (that is,
    ///             // no path is available).
    ///             FpCategory::Infinite | FpCategory::Nan => None,
    ///             // Handle zero separately, so that negative zero is
    ///             // handled well.
    ///             FpCategory::Zero => Some(0f32),
    ///             // Do not allow negative results: if negative it's
    ///             // unreachable
    ///             _ if self.is_sign_negative() => None,
    ///             // Other floating point numbers should be fine (at worst
    ///             // imprecise).
    ///             _ => Some(self),
    ///         }
    ///     }
    /// }
    /// ```
    fn normalize(self) -> Option<Self> {
        Some(self)
    }
}
impl PathFindCost for u16 {}
impl PathFindCost for u32 {}
impl PathFindCost for u64 {}
impl PathFindCost for u128 {}
impl PathFindCost for usize {}
impl PathFindCost for f32 {
    fn normalize(self) -> Option<Self> {
        match self.classify() {
            // Default infinites and NaNs to None (that is,
            // no path is available).
            FpCategory::Infinite | FpCategory::Nan => None,
            // Handle zero separately, so that negative zero is
            // handled well.
            FpCategory::Zero => Some(0f32),
            // Do not allow negative results: if negative it's
            // unreachable
            _ if self.is_sign_negative() => None,
            // Other floating point numbers should be fine (at worst
            // imprecise).
            _ => Some(self),
        }
    }
}
impl PathFindCost for f64 {
    fn normalize(self) -> Option<Self> {
        match self.classify() {
            FpCategory::Infinite | FpCategory::Nan => None,
            FpCategory::Zero => Some(0f64),
            _ if self.is_sign_negative() => None,
            _ => Some(self),
        }
    }
}

/// A single location in the result data of a pathfinding operation.
#[derive(Default)]
pub struct PathFindDataTile<C: PathFindCost> {
    /// The previous location in the path if the path to this
    /// tile was calculated, [`None`] otherwise.
    pub origin: Option<(usize, usize)>,
    /// The cost to reach this location if the path to this
    /// tile was calculated, [`None`] otherwise.
    pub cost: Option<C>,
    /// True if this tile is part of the optimal path (valid
    /// only for those pathfindings that lead to a single
    /// destination).
    pub in_shortest_path: bool,
}

/// The result type of the pathfinding operation
pub enum PathFindDataResult<C: PathFindCost> {
    /// The pathfinding was towards multiple destinations
    MultipleDestinations,
    /// The pathfinding was towards a single destination and
    /// a path was found (the cost is included in this enum).
    ShortestPathFound(C),
    /// The pathfinding was towards a single destination, and
    /// no valid path was found.
    PathNotFound,
}

/// The resulting data from a pathfinding run
pub struct PathFindData<C: PathFindCost> {
    /// The result of the run
    pub result: PathFindDataResult<C>,
    /// Data for the possible tiles
    pub tiles: BidiArray<PathFindDataTile<C>>,
}

#[derive(Clone, Debug, Hash, PartialEq)]
struct Adjacency<C: PathFindCost> {
    pub estimated_cost: C,
    pub actual_cost: C,
    pub position: (usize, usize),
    pub origin: (usize, usize),
}
impl<C: PathFindCost> Eq for Adjacency<C> {}

impl<C: PathFindCost> Ord for Adjacency<C> {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .estimated_cost
            .partial_cmp(&self.estimated_cost)
            .expect("non-normalized value in pathfinding")
    }
}

impl<C: PathFindCost> PartialOrd for Adjacency<C> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn pathfind_core<T, V, FC, FH, C>(
    view: &V,
    start: (usize, usize),
    dest: Option<(usize, usize)>,
    neighbouring: BidiNeighbours,
    cost_func: FC,
    heuristic: FH,
) -> Result<PathFindData<C>, BidiError>
where
    V: BidiView<Output = T> + Sized,
    C: PathFindCost,
    FC: Fn(&T, (usize, usize), &T, (usize, usize)) -> Option<C>,
    FH: Fn((usize, usize), (usize, usize)) -> C,
{
    let rect = view.bounding_rect();
    if !rect.contains(start.0, start.1) {
        return Err(BidiError::OutOfBounds);
    }

    if let Some(d) = dest {
        if !rect.contains(d.0, d.1) {
            return Err(BidiError::OutOfBounds);
        }
    }

    let mut adiacent = BinaryHeap::<Adjacency<C>>::new();

    let mut data = PathFindData {
        result: if dest.is_some() {
            PathFindDataResult::PathNotFound
        } else {
            PathFindDataResult::MultipleDestinations
        },
        tiles: BidiArray::with_size_default(view.width(), view.height()),
    };

    adiacent.push(Adjacency {
        estimated_cost: C::default(),
        actual_cost: C::default(),
        position: start,
        origin: start,
    });

    let mut neighbours = neighbouring.prealloc_vec();

    while let Some(adjacency) = adiacent.pop() {
        let cur_cost = {
            let cur_tile = &mut data.tiles[adjacency.position];

            if let Some(cost) = cur_tile.cost {
                if adjacency.actual_cost >= cost {
                    continue;
                }
            }

            cur_tile.origin = Some(adjacency.origin);
            cur_tile.cost = Some(adjacency.actual_cost);
            adjacency.actual_cost
        };

        if let Some(dest) = dest {
            if adjacency.position == dest {
                data.result = PathFindDataResult::ShortestPathFound(cur_cost);
                break;
            }
        }

        let from = &view[adjacency.position];
        neighbouring.generate_points_on(
            &mut neighbours,
            adjacency.position,
            rect.width,
            rect.height,
        );

        while let Some(neighbour) = neighbours.pop() {
            let to = &view[neighbour];
            let cost = match cost_func(from, adjacency.position, to, neighbour) {
                None => None,
                Some(c) => c.normalize(),
            };

            if let Some(cost) = cost {
                if match data.tiles[neighbour].cost {
                    None => true,
                    Some(old_cost) => cost < old_cost,
                } {
                    adiacent.push(Adjacency {
                        estimated_cost: if let Some(dest) = dest {
                            cur_cost
                                + cost
                                + heuristic(neighbour, dest).normalize().unwrap_or_default()
                        } else {
                            cur_cost + cost
                        },
                        actual_cost: cur_cost + cost,
                        position: neighbour,
                        origin: adjacency.position,
                    });
                }
            }
        }
    }

    if let PathFindDataResult::ShortestPathFound(_) = data.result {
        let mut pos = dest;

        while let Some(p) = pos {
            data.tiles[p].in_shortest_path = true;
            if p == start {
                break;
            }
            pos = data.tiles[p].origin;
        }
    }

    Ok(data)
}

/// Finds the shortest path between `start` and `dest` in the given
/// `view` using the given `cost_func` to evaluate the cost of a
/// given movement.
///
/// Under the hood, this uses the Djikstra algorithm.
///
/// `cost_func` is a closure like:
/// `fn(from_elem: &T, from_pos: (usize, usize), to_elem: &T, to_pos: (usize, usize)) -> Option<C>`.
///
/// It should return the cost of a movement between `from_pos` to `to_pos` (the elements
/// are also passed for convenience) and should return [`None`] if no such path exists, or
/// the cost C if it does.
pub fn pathfind_to_dest<T, V, FC, C>(
    view: &V,
    start: (usize, usize),
    dest: (usize, usize),
    neighbouring: BidiNeighbours,
    cost_func: FC,
) -> Result<PathFindData<C>, BidiError>
where
    V: BidiView<Output = T> + Sized,
    C: PathFindCost,
    FC: Fn(&T, (usize, usize), &T, (usize, usize)) -> Option<C>,
{
    pathfind_core(view, start, Some(dest), neighbouring, cost_func, |_, _| {
        C::default()
    })
}

/// Finds the shortest path between `start` and `dest` in the given
/// `view` using the given `cost_func` to evaluate the cost of a
/// given movement and an `heuristic` to estimate what is the best
/// direction to go.
///
/// Under the hood, this uses the A* algorithm.
///
/// `cost_func` is a closure like:
/// `fn(from_elem: &T, from_pos: (usize, usize), to_elem: &T, to_pos: (usize, usize)) -> Option<C>`.
///
/// It should return the cost of a movement between `from_pos` to `to_pos` (the elements
/// are also passed for convenience) and should return [`None`] if no such path exists, or
/// the cost C if it does.
///
/// `heuristic` is a closure like:
/// `fn(from_pos: (usize, usize), to_pos: (usize, usize)) -> C`.
///
/// It should return the estimated cost of a movement between `from_pos` to `to_pos`.
///
/// The heuristic does not need, obviously, to provide and accurate estimate.
///
/// Underestimations have a performance impact, but they don't impact the ability to
/// find a solution (in fact, `pathfind_to_dest` simply uses zero as the heuristic).
/// Overestimations will improve runtime performances, but the result will not be the
/// optimal one.
pub fn pathfind_to_dest_heuristic<T, V, FC, FH, C>(
    view: &V,
    start: (usize, usize),
    dest: (usize, usize),
    neighbouring: BidiNeighbours,
    cost_func: FC,
    heuristic: FH,
) -> Result<PathFindData<C>, BidiError>
where
    V: BidiView<Output = T> + Sized,
    C: PathFindCost,
    FC: Fn(&T, (usize, usize), &T, (usize, usize)) -> Option<C>,
    FH: Fn((usize, usize), (usize, usize)) -> C,
{
    pathfind_core(view, start, Some(dest), neighbouring, cost_func, heuristic)
}

/// Finds the shortest path between `start` and every other element in
/// the given `view` using the given `cost_func` to evaluate the cost of
/// a given movement.
///
/// Under the hood, this uses the Djikstra algorithm.
///
/// `cost_func` is a closure like:
/// `fn(from_elem: &T, from_pos: (usize, usize), to_elem: &T, to_pos: (usize, usize)) -> Option<C>`.
///
/// It should return the cost of a movement between `from_pos` to `to_pos` (the elements
/// are also passed for convenience) and should return [`None`] if no such path exists, or
/// the cost C if it does.
pub fn pathfind_to_whole<T, V, FC, C>(
    view: &V,
    start: (usize, usize),
    neighbouring: BidiNeighbours,
    cost_func: FC,
) -> Result<PathFindData<C>, BidiError>
where
    V: BidiView<Output = T> + Sized,
    C: PathFindCost,
    FC: Fn(&T, (usize, usize), &T, (usize, usize)) -> Option<C>,
{
    pathfind_core(view, start, None, neighbouring, cost_func, |_, _| {
        C::default()
    })
}
