use super::*;
use crate::bidiiter::borderstate::IterBorderState;
use crate::bidiiter::rectstate::OnRectState;
use crate::*;
use std::iter::Iterator;

/// An immutable iterator over bidimensional data structures.
///
/// Bidimensional iterators are created by `iter` methods
/// on the data structure and view traits of this crate, such as:
/// * [`BidiView::iter()`]
/// * [`BidiArray::iter()`]
/// * [`BidiVec::iter()`]
/// * [`BidiGrowVec::iter()`]
/// * [`BidiMutSlice::iter()`]
/// * [`BidiSlice::iter()`]
///
/// Bidimensional iterators allow one to refine the iteration to
/// a subrectangle:
/// ```
/// # use bidivec::{BidiVec, bidivec, BidiRect};
///
/// let bvec = bidivec!{
///     [1, 2, 3],
///     [4, 5, 6],
///     [7, 8, 9],
/// };
///
/// let v = bvec.iter().on_row(1).copied().collect::<Vec<i32>>();
///
/// assert_eq!(v, vec![4, 5, 6]);
/// ```
///
/// Other methods allow similar restrictions to a specific range,
/// or to iterate by columns rather than by rows.
///
/// Additionally, the original coordinate can be returned, as in:
/// ```
/// # use bidivec::{BidiVec, bidivec, BidiRect};
///
/// let bvec = bidivec!{
///     [1, 2, 3],
///     [4, 5, 6],
///     [7, 8, 9],
/// };
///
/// for (x, y, i) in bvec.iter().with_coords() {
///     println!("The element {} is at {}, {}", i, x, y);
/// }
/// ```
///
/// # Panics
///
/// Calling the methods altering the iteration (`by_column`, `on_row`,
/// `on_column`, `on_rect` and `with_coords`) after the iteration has
/// been started will cause a panic.
pub struct Iter<'v, T: 'v, V: BidiView<Output = T>> {
    view: &'v V,
    rect: BidiRect,
    state: OnRectState,
    by_column: bool,
}

impl<'v, T: 'v, V: BidiView<Output = T>> Iter<'v, T, V> {
    pub(crate) fn new(view: &'v V) -> Self {
        let rect = view.bounding_rect();
        Self {
            view,
            rect,
            state: OnRectState::NotStarted,
            by_column: false,
        }
    }

    /// Returns an iterator which yields the items with their original
    /// coordinates. Note that all the coordinates are relative to the
    /// [`BidiView`] (or other data structure) the iterator was created
    /// from.
    ///
    /// # Examples
    /// ```
    /// # use bidivec::{BidiVec, bidivec, BidiRect};
    ///
    /// let bvec = bidivec!{
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9],
    /// };
    ///
    /// for (x, y, i) in bvec.iter().with_coords() {
    ///     println!("The element {} is at {}, {}", i, x, y);
    /// }
    /// ```
    pub fn with_coords(self) -> super::super::immutable_xy::iter::WithCoords<'v, T, V> {
        self.state.assert_not_started("with_coords()");
        super::super::immutable_xy::iter::WithCoords {
            view: self.view,
            rect: self.rect,
            by_column: self.by_column,
            state: OnRectState::NotStarted,
        }
    }

    /// Returns an iterator which yields the items by columns instead
    /// of by rows as it would otherwise do.
    ///
    /// # Examples
    /// ```
    /// # use bidivec::{BidiVec, bidivec, BidiRect};
    ///
    /// let bvec = bidivec!{
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9],
    /// };
    ///
    /// let v = bvec.iter().by_column().copied().collect::<Vec<i32>>();
    ///
    /// assert_eq!(v, vec![1, 4, 7, 2, 5, 8, 3, 6, 9]);
    /// ```
    pub fn by_column(mut self) -> Self {
        self.state.assert_not_started("by_column()");
        self.by_column = true;
        self
    }

    /// Returns an iterator which yields the items on a given row.
    /// If the row is out of range, no data is yielded.
    ///
    /// # Examples
    /// ```
    /// # use bidivec::{BidiVec, bidivec, BidiRect};
    ///
    /// let bvec = bidivec!{
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9],
    /// };
    ///
    /// let v = bvec.iter().on_row(1).copied().collect::<Vec<i32>>();
    ///
    /// assert_eq!(v, vec![4, 5, 6]);
    /// ```
    pub fn on_row(self, row: usize) -> rect::OnRect<'v, T, V> {
        self.state.assert_not_started("on_row()");
        let rect = self
            .rect
            .intersect(&BidiRect::new(0, row, self.rect.width, 1));
        rect::OnRect {
            view: self.view,
            rect,
            by_column: self.by_column,
            state: OnRectState::NotStarted,
        }
    }

    /// Returns an iterator which yields the items on a given column.
    /// If the row is out of range, no data is yielded.
    ///
    /// # Examples
    /// ```
    /// # use bidivec::{BidiVec, bidivec, BidiRect};
    ///
    /// let bvec = bidivec!{
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9],
    /// };
    ///
    /// let v = bvec.iter().on_column(1).copied().collect::<Vec<i32>>();
    ///
    /// assert_eq!(v, vec![2, 5, 8]);
    /// ```
    pub fn on_column(self, column: usize) -> rect::OnRect<'v, T, V> {
        self.state.assert_not_started("on_column()");
        let rect = self
            .rect
            .intersect(&BidiRect::new(column, 0, 1, self.rect.height));
        rect::OnRect {
            view: self.view,
            rect,
            by_column: self.by_column,
            state: OnRectState::NotStarted,
        }
    }

    /// Returns an iterator which yields the items that are inside a
    /// given rectangle.
    ///
    /// # Examples
    /// ```
    /// # use bidivec::{BidiVec, bidivec, BidiRect};
    ///
    /// let bvec = bidivec!{
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9],
    /// };
    ///
    /// let v = bvec.iter()
    ///     .on_rect(&BidiRect::new(1, 0, 2, 3))
    ///     .copied()
    ///     .collect::<Vec<i32>>();
    ///
    /// assert_eq!(v, vec![2, 3, 5, 6, 8, 9]);
    /// ```
    pub fn on_rect(self, rect: &BidiRect) -> rect::OnRect<'v, T, V> {
        self.state.assert_not_started("on_rect()");
        let rect = self.rect.intersect(rect);
        rect::OnRect {
            view: self.view,
            rect,
            by_column: self.by_column,
            state: OnRectState::NotStarted,
        }
    }

    /// Returns an iterator which yields the items on the border of a
    /// given rectangle. The rectangle is signed, so that it can be
    /// offset'ed before the (0, 0) point and be cropped correctly.
    /// Iteration starts from the top-left corner of the rectangle and
    /// goes clockwise.
    ///
    /// # Examples
    /// ```
    /// # use bidivec::{BidiVec, bidivec, BidiRectSigned};
    ///
    /// let bvec = bidivec!{
    ///     [ 1,  2,  3,  4],
    ///     [ 5,  6,  7,  8],
    ///     [ 9, 10, 11, 12],
    ///     [13, 14, 15, 16],
    /// };
    ///
    /// let v = bvec.iter()
    ///     .on_border(&BidiRectSigned::new(1, 1, 3, 3))
    ///     .copied()
    ///     .collect::<Vec<i32>>();
    ///
    /// assert_eq!(v, vec![6, 7, 8, 12, 16, 15, 14, 10]);
    /// ```
    pub fn on_border(self, border: &BidiRectSigned) -> border::OnBorder<'v, T, V> {
        self.state.assert_not_started("on_border()");
        border::OnBorder {
            view: self.view,
            rect: self.rect,
            border: border.clone(),
            state: IterBorderState::NotStarted,
        }
    }

    /// Returns an iterator which yields the items directly surrounding
    /// a given coordinate.
    /// Iteration starts from the element direct above the specified
    /// coordinates and proceeds clockwise.
    ///
    /// # Examples
    /// ```
    /// # use bidivec::{BidiVec, bidivec, BidiNeighbours};
    ///
    /// let bvec = bidivec!{
    ///     [ 1,  2,  3,  4],
    ///     [ 5,  6,  7,  8],
    ///     [ 9, 10, 11, 12],
    ///     [13, 14, 15, 16],
    /// };
    ///
    /// let bordering = bvec.iter()
    ///     .on_neighbours(1, 1, BidiNeighbours::Bordering)
    ///     .copied()
    ///     .collect::<Vec<i32>>();
    ///
    /// assert_eq!(bordering, vec![2, 3, 7, 11, 10, 9, 5, 1]);
    ///
    /// let adjacent = bvec.iter()
    ///     .on_neighbours(1, 1, BidiNeighbours::Adjacent)
    ///     .copied()
    ///     .collect::<Vec<i32>>();
    ///
    /// assert_eq!(adjacent, vec![2, 7, 10, 5]);
    /// ```
    pub fn on_neighbours(
        self,
        x: usize,
        y: usize,
        neighbours: BidiNeighbours,
    ) -> precalc::OnElements<'v, T, V> {
        self.state.assert_not_started("on_neighbours()");
        let mut points = neighbours.prealloc_vec();
        neighbours.generate_points_on(&mut points, (x, y), self.view.width(), self.view.height());
        precalc::OnElements {
            view: self.view,
            started: false,
            points,
        }
    }
}

impl<'v, T: 'v, V: BidiView<Output = T>> Iterator for Iter<'v, T, V> {
    type Item = &'v T;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        self.state.advance(&self.rect, self.by_column);
        if let OnRectState::Iterating(x, y) = self.state {
            self.view.get(x, y)
        } else {
            None
        }
    }
}
