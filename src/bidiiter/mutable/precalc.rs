use crate::BidiViewMutIterable;
use std::iter::Iterator;

/// An iterator type returning precalculated items.
/// Refer to the function that produced this iterator for
/// further details.
pub struct OnElements<'v, T: 'v, V: BidiViewMutIterable<Output = T>> {
    pub(super) view: &'v mut V,
    pub(super) points: Vec<(usize, usize)>,
    pub(super) started: bool,
}

impl<'v, T: 'v, V: BidiViewMutIterable<Output = T>> OnElements<'v, T, V> {
    /// Returns an iterator which yields the items with their original
    /// coordinates. Note that all the coordinates are relative to the
    ///[`BidiViewMutIterable`] (or other data structure) the iterator was created
    /// from.
    pub fn with_coords(self) -> super::super::mutable_xy::precalc::OnElements<'v, T, V> {
        if self.started {
            panic!("Can't call 'with_coords()' after enumeration has started.");
        }
        super::super::mutable_xy::precalc::OnElements {
            view: self.view,
            points: self.points,
        }
    }
}

impl<'v, T: 'v, V: BidiViewMutIterable<Output = T>> Iterator for OnElements<'v, T, V> {
    type Item = &'v mut T;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        self.started = true;
        if let Some((x, y)) = self.points.pop() {
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
