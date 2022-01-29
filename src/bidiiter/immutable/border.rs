use crate::bidiiter::borderstate::IterBorderState;
use crate::BidiRect;
use crate::BidiRectSigned;
use crate::BidiView;
use std::iter::Iterator;

/// An iterator type returning items in a rectangular region.
pub struct OnBorder<'v, T: 'v, V: BidiView<Output = T>> {
    pub(super) view: &'v V,
    pub(super) rect: BidiRect,
    pub(super) border: BidiRectSigned,
    pub(super) state: IterBorderState,
}

impl<'v, T: 'v, V: BidiView<Output = T>> OnBorder<'v, T, V> {
    /// Returns an iterator which yields the items with their original
    /// coordinates. Note that all the coordinates are relative to the
    /// [`BidiView`] (or other data structure) the iterator was created
    /// from.
    pub fn with_coords(self) -> super::super::immutable_xy::border::OnBorder<'v, T, V> {
        self.state.assert_not_started("with_coords()");
        super::super::immutable_xy::border::OnBorder {
            view: self.view,
            rect: self.rect,
            border: self.border,
            state: IterBorderState::NotStarted,
        }
    }
}

impl<'v, T: 'v, V: BidiView<Output = T>> Iterator for OnBorder<'v, T, V> {
    type Item = &'v T;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        self.state.advance(&self.rect, &self.border);

        if let IterBorderState::Iterating(x, y) = self.state {
            self.view.get_signed(x, y)
        } else {
            None
        }
    }
}
