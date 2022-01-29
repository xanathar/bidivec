use crate::bidiiter::borderstate::IterBorderState;
use crate::BidiRect;
use crate::BidiRectSigned;
use crate::BidiViewMutIterable;
use std::iter::Iterator;

/// An iterator type returning items in a rectangular region.
pub struct OnBorder<'v, T: 'v, V: BidiViewMutIterable<Output = T>> {
    pub(crate) view: &'v mut V,
    pub(crate) rect: BidiRect,
    pub(crate) state: IterBorderState,
    pub(crate) border: BidiRectSigned,
}

impl<'v, T: 'v, V: BidiViewMutIterable<Output = T>> Iterator for OnBorder<'v, T, V> {
    type Item = (usize, usize, &'v mut T);

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        self.state.advance(&self.rect, &self.border);

        if let IterBorderState::Iterating(x, y) = self.state {
            let refptr = {
                let mutref = match self.view.get_mut_signed(x, y) {
                    Some(r) => r,
                    None => return None,
                };

                let refptr: *mut T = mutref;
                refptr
            };

            unsafe { Some((x as usize, y as usize, &mut *refptr)) }
        } else {
            None
        }
    }
}
