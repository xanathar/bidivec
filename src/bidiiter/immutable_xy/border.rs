use crate::bidiiter::borderstate::IterBorderState;
use crate::BidiRect;
use crate::BidiRectSigned;
use crate::BidiView;
use std::iter::Iterator;

/// An iterator type returning items in a rectangular region.
pub struct OnBorder<'v, T: 'v, V: BidiView<Output = T>> {
    pub(crate) view: &'v V,
    pub(crate) rect: BidiRect,
    pub(crate) state: IterBorderState,
    pub(crate) border: BidiRectSigned,
}

impl<'v, T: 'v, V: BidiView<Output = T>> Iterator for OnBorder<'v, T, V> {
    type Item = (usize, usize, &'v T);

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        self.state.advance(&self.rect, &self.border);

        if let IterBorderState::Iterating(x, y) = self.state {
            self.view
                .get_signed(x, y)
                .map(|v| (x as usize, y as usize, v))
        } else {
            None
        }
    }
}
