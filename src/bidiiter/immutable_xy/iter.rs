use super::*;
use crate::bidiiter::borderstate::IterBorderState;
use crate::bidiiter::rectstate::OnRectState;
use crate::BidiNeighbours;
use crate::BidiRect;
use crate::BidiRectSigned;
use crate::BidiView;
use std::iter::Iterator;

/// An iterator type returning items with their coordinates.
pub struct WithCoords<'v, T: 'v, V: BidiView<Output = T>> {
    pub(crate) view: &'v V,
    pub(crate) rect: BidiRect,
    pub(crate) state: OnRectState,
    pub(crate) by_column: bool,
}

impl<'v, T: 'v, V: BidiView<Output = T>> WithCoords<'v, T, V> {
    /// Returns an iterator which yields the items by columns instead
    /// of by rows as it would otherwise do.
    pub fn by_column(mut self) -> Self {
        self.state.assert_not_started("by_column()");
        self.by_column = true;
        self
    }

    /// Returns an iterator which yields the items on a given row.
    /// If the row is out of range, no data is yielded.
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
            points,
        }
    }
}

impl<'v, T: 'v, V: BidiView<Output = T>> Iterator for WithCoords<'v, T, V> {
    type Item = (usize, usize, &'v T);

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        self.state.advance(&self.rect, self.by_column);
        if let OnRectState::Iterating(x, y) = self.state {
            self.view.get(x, y).map(|v| (x, y, v))
        } else {
            None
        }
    }
}
