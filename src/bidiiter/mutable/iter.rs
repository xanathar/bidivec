use super::super::borderstate::IterBorderState;
use super::super::rectstate::OnRectState;
use super::*;
use crate::*;
use std::iter::Iterator;

/// A mutating iterator over bidimensional data structures.
///
/// Bidimensional iterators are created by `iter_mut` methods
/// on the data structure and view traits of this crate, such as:
/// * [`BidiViewMutIterable::iter_mut()`]
/// * [`BidiArray::iter_mut()`]
/// * [`BidiVec::iter_mut()`]
/// * [`BidiGrowVec::iter_mut()`]
/// * [`BidiMutSlice::iter_mut()`]
///
/// Bidimensional mutating iterators allow one to refine the iteration to
/// a subrectangle:
/// ```
/// # use bidivec::{BidiVec, bidivec, BidiRect};
///
/// let mut bvec = bidivec!{
///     [1, 2, 3],
///     [4, 5, 6],
///     [7, 8, 9],
/// };
///
/// for item in bvec
///     .iter_mut()
///     .on_rect(&BidiRect::new(1, 1, 2, 2,))
/// {
///     *item = -(*item);
/// }
///
/// let v = bvec.iter().copied().collect::<Vec<i32>>();
///
/// assert_eq!(v, vec![1, 2, 3, 4, -5, -6, 7, -8, -9]);
/// ```
///
/// Other methods allow similar restrictions to a specific range,
/// or to iterate by columns rather than by rows.
///
/// Additionally, the original coordinate can be returned, as in:
/// ```
/// # use bidivec::{BidiVec, bidivec};
///
/// let mut bvec = bidivec!{
///     [1, 2, 3],
///     [4, 5, 6],
///     [7, 8, 9],
/// };
///
/// for (x, y, item) in bvec
///     .iter_mut()
///     .with_coords()
/// {
///     *item = (x * 10 + y) as i32;
/// }
///
/// let v = bvec.iter().copied().collect::<Vec<i32>>();
///
/// assert_eq!(v, vec![0, 10, 20, 1, 11, 21, 2, 12, 22]);
/// ```
///
/// # Panics
///
/// Calling the methods altering the iteration (`by_column`, `on_row`,
/// `on_column`, `on_rect` and `with_coords`) after the iteration has
/// been started will cause a panic.
pub struct IterMut<'v, T: 'v, V: BidiViewMutIterable<Output = T>> {
    view: &'v mut V,
    rect: BidiRect,
    state: OnRectState,
    by_column: bool,
}

impl<'v, T: 'v, V: BidiViewMutIterable<Output = T>> IterMut<'v, T, V> {
    pub(crate) fn new(view: &'v mut V) -> Self {
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
    /// [`BidiViewMutIterable`] (or other data structure) the iterator was created
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
    pub fn with_coords(self) -> super::super::mutable_xy::iter::WithCoords<'v, T, V> {
        self.state.assert_not_started("with_coords()");
        super::super::mutable_xy::iter::WithCoords {
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
    /// let mut bvec = bidivec!{
    ///     [100, 100, 100],
    ///     [100, 100, 100],
    ///     [100, 100, 100],
    /// };
    ///
    /// for (idx, item) in bvec
    ///     .iter_mut()
    ///     .by_column()
    ///     .enumerate()
    /// {
    ///     *item = idx as i32;
    /// }
    ///
    /// let v = bvec.iter().copied().collect::<Vec<i32>>();
    ///
    /// assert_eq!(v, vec![0, 3, 6, 1, 4, 7, 2, 5, 8]);
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
    /// let mut bvec = bidivec!{
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9],
    /// };
    ///
    /// for item in bvec
    ///     .iter_mut()
    ///     .on_row(1)
    /// {
    ///     *item = -(*item);
    /// }
    ///
    /// let v = bvec.iter().copied().collect::<Vec<i32>>();
    ///
    /// assert_eq!(v, vec![1, 2, 3, -4, -5, -6, 7, 8, 9]);
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
    /// let mut bvec = bidivec!{
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9],
    /// };
    ///
    /// for item in bvec
    ///     .iter_mut()
    ///     .on_column(1)
    /// {
    ///     *item = -(*item);
    /// }
    ///
    /// let v = bvec.iter().copied().collect::<Vec<i32>>();
    ///
    /// assert_eq!(v, vec![1, -2, 3, 4, -5, 6, 7, -8, 9]);
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
    /// let mut bvec = bidivec!{
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9],
    /// };
    ///
    /// for item in bvec
    ///     .iter_mut()
    ///     .on_rect(&BidiRect::new(1, 1, 2, 2,))
    /// {
    ///     *item = -(*item);
    /// }
    ///
    /// let v = bvec.iter().copied().collect::<Vec<i32>>();
    ///
    /// assert_eq!(v, vec![1, 2, 3, 4, -5, -6, 7, -8, -9]);
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
    /// let mut bvec = bidivec!{
    ///     [ 1,  2,  3,  4],
    ///     [ 5,  6,  7,  8],
    ///     [ 9, 10, 11, 12],
    ///     [13, 14, 15, 16],
    /// };
    ///
    /// for item in bvec
    ///     .iter_mut()
    ///     .on_border(&BidiRectSigned::new(0, 0, 4, 4))
    /// {
    ///     *item = 0;
    /// }
    ///
    /// let v = bvec.iter().copied().collect::<Vec<i32>>();
    ///
    /// assert_eq!(v, vec![
    ///     0,  0,  0, 0,
    ///     0,  6,  7, 0,
    ///     0, 10, 11, 0,
    ///     0,  0,  0, 0,
    /// ]);
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
    /// let mut bvec = bidivec!{
    ///     [ 1,  2,  3,  4],
    ///     [ 5,  6,  7,  8],
    ///     [ 9, 10, 11, 12],
    ///     [13, 14, 15, 16],
    /// };
    ///
    /// for item in bvec
    ///     .iter_mut()
    ///     .on_neighbours(2, 2, BidiNeighbours::Bordering)
    /// {
    ///     *item = 0;
    /// }
    ///
    /// let v = bvec.iter().copied().collect::<Vec<i32>>();
    ///
    /// assert_eq!(v, vec![
    ///     1, 2, 3, 4,
    ///     5, 0, 0, 0,
    ///     9, 0, 11, 0,
    ///     13, 0, 0, 0,
    /// ]);
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

impl<'v, T: 'v, V: BidiViewMutIterable<Output = T>> Iterator for IterMut<'v, T, V> {
    type Item = &'v mut T;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        self.state.advance(&self.rect, self.by_column);
        if let OnRectState::Iterating(x, y) = self.state {
            let refptr = {
                let mutref = match self.view.get_mut(x, y) {
                    Some(r) => r,
                    None => return None,
                };

                let refptr: *mut T = mutref;
                refptr
            };

            unsafe { Some(&mut *refptr) }
        } else {
            None
        }
    }
}
