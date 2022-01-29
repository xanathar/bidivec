use crate::bidiiter::rectstate::OnRectState;
use crate::BidiRect;
use crate::BidiView;
use std::iter::Iterator;

/// An iterator type returning items with their coordinates, on a
/// rectangular region.
pub struct OnRect<'v, T: 'v, V: BidiView<Output = T>> {
    pub(crate) view: &'v V,
    pub(crate) rect: BidiRect,
    pub(crate) state: OnRectState,
    pub(crate) by_column: bool,
}

impl<'v, T: 'v, V: BidiView<Output = T>> OnRect<'v, T, V> {
    /// Returns an iterator which yields the items by columns instead
    /// of by rows as it would otherwise do.
    pub fn by_column(mut self) -> Self {
        self.state.assert_not_started("by_column()");
        self.by_column = true;
        self
    }
}

impl<'v, T: 'v, V: BidiView<Output = T>> Iterator for OnRect<'v, T, V> {
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
