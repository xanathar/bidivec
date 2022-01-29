use super::super::rectstate::OnRectState;
use crate::BidiRect;
use crate::BidiViewMutIterable;
use std::iter::Iterator;

/// A mutable iterator type returning items with their coordinates, on a
/// rectangular region.
pub struct OnRect<'v, T: 'v, V: BidiViewMutIterable<Output = T>> {
    pub(crate) view: &'v mut V,
    pub(crate) rect: BidiRect,
    pub(crate) state: OnRectState,
    pub(crate) by_column: bool,
}

impl<'v, T: 'v, V: BidiViewMutIterable<Output = T>> OnRect<'v, T, V> {
    /// Returns an iterator which yields the items by columns instead
    /// of by rows as it would otherwise do.
    pub fn by_column(mut self) -> Self {
        self.state.assert_not_started("by_column()");
        self.by_column = true;
        self
    }
}

impl<'v, T: 'v, V: BidiViewMutIterable<Output = T>> Iterator for OnRect<'v, T, V> {
    type Item = (usize, usize, &'v mut T);

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

            unsafe { Some((x, y, &mut *refptr)) }
        } else {
            None
        }
    }
}
