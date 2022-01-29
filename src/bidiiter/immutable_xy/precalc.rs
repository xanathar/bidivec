use crate::BidiView;
use std::iter::Iterator;

/// An iterator type returning precalculated items.
/// Refer to the function that produced this iterator for
/// further details.
pub struct OnElements<'v, T: 'v, V: BidiView<Output = T>> {
    pub(crate) view: &'v V,
    pub(crate) points: Vec<(usize, usize)>,
}

impl<'v, T: 'v, V: BidiView<Output = T>> Iterator for OnElements<'v, T, V> {
    type Item = (usize, usize, &'v T);

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        self.points.pop().map(|(x, y)| (x, y, &self.view[(x, y)]))
    }
}
