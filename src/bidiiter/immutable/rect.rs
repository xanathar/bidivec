use crate::bidiiter::rectstate::OnRectState;
use crate::BidiRect;
use crate::BidiView;
use std::iter::Iterator;

/// An iterator type returning items in a rectangular region.
pub struct OnRect<'v, T: 'v, V: BidiView<Output = T>> {
    pub(super) view: &'v V,
    pub(super) rect: BidiRect,
    pub(super) state: OnRectState,
    pub(super) by_column: bool,
}

impl<'v, T: 'v, V: BidiView<Output = T>> OnRect<'v, T, V> {
    /// Returns an iterator which yields the items with their original
    /// coordinates. Note that all the coordinates are relative to the
    /// [`BidiView`] (or other data structure) the iterator was created
    /// from.
    pub fn with_coords(self) -> super::super::immutable_xy::rect::OnRect<'v, T, V> {
        self.state.assert_not_started("with_coords()");
        super::super::immutable_xy::rect::OnRect {
            view: self.view,
            rect: self.rect,
            by_column: self.by_column,
            state: OnRectState::NotStarted,
        }
    }

    /// Returns an iterator which yields the items by columns instead
    /// of by rows as it would otherwise do.
    pub fn by_column(mut self) -> Self {
        self.state.assert_not_started("by_column()");
        self.by_column = true;
        self
    }
}

impl<'v, T: 'v, V: BidiView<Output = T>> Iterator for OnRect<'v, T, V> {
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
