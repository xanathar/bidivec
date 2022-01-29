use crate::BidiViewMutIterable;
use std::iter::Iterator;

/// An iterator type returning precalculated items.
/// Refer to the function that produced this iterator for
/// further details.
pub struct OnElements<'v, T: 'v, V: BidiViewMutIterable<Output = T>> {
    pub(crate) view: &'v mut V,
    pub(crate) points: Vec<(usize, usize)>,
}

impl<'v, T: 'v, V: BidiViewMutIterable<Output = T>> Iterator for OnElements<'v, T, V> {
    type Item = (usize, usize, &'v mut T);

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if let Some((x, y)) = self.points.pop() {
            let refptr = {
                let mutref = match self.view.get_mut(x, y) {
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
