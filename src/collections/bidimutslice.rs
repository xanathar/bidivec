use crate::bidiiter::{Iter, IterMut};
use std::iter::Iterator;
#[rustversion::since(1.48)]
use std::ops::Range;
use std::ops::{Index, IndexMut};

use crate::*;

/// A bidimensional view over a mutable slice (for the immutable version,
/// see [`BidiSlice`]).
/// Items are expected to be linearly arranged per rows.
///
/// Most methods will not implicitly panic if out-of-bounds accesses are attempted,
/// but they will return a [`BidiError`] for graceful error handling.
///
/// # Examples
///
/// ```
/// use bidivec::BidiMutSlice;
///
/// let mut slice = [1, 2, 3, 4, 5, 6, 7, 8, 9];
/// let mut bslice = BidiMutSlice::new(&mut slice, 3).unwrap();
///
/// assert_eq!(bslice[(1, 1)], 5);
///
/// bslice[(1, 1)] = 12;
///
/// assert_eq!(bslice[(1, 1)], 12);
/// ```
#[derive(Debug)]
pub struct BidiMutSlice<'a, T> {
    pub(crate) data: &'a mut [T],
    pub(crate) row_size: usize,
}

impl<'a, T> BidiMutSlice<'a, T> {
    /// Constructs a new, empty [`BidiMutSlice<T>`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiMutSlice;
    ///
    /// let mut slice = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    /// let bslice = BidiMutSlice::new(&mut slice, 3).unwrap();
    /// ```
    #[inline]
    pub fn new(data: &'a mut [T], row_size: usize) -> Result<Self, BidiError> {
        if (data.is_empty() && row_size == 0) || row_size != 0 && (data.len() % row_size) == 0 {
            Ok(Self { data, row_size })
        } else {
            Err(BidiError::IncompatibleSize)
        }
    }

    /// Returns the number of items contained in the bidislice.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiMutSlice;
    ///
    /// let mut slice = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    /// let bslice = BidiMutSlice::new(&mut slice, 3).unwrap();
    ///
    /// assert_eq!(bslice.len(), 9);
    /// ```
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns the width (that is, the size of a row) in the bidislice.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiMutSlice;
    ///
    /// let mut slice = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    /// let bslice = BidiMutSlice::new(&mut slice, 3).unwrap();
    ///
    /// assert_eq!(bslice.width(), 3);
    /// ```
    pub fn width(&self) -> usize {
        self.row_size
    }

    /// Returns the height (that is, the size of a column) in the bidislice.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiMutSlice;
    ///
    /// let mut slice = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    /// let bslice = BidiMutSlice::new(&mut slice, 3).unwrap();
    ///
    /// assert_eq!(bslice.height(), 3);
    /// ```
    pub fn height(&self) -> usize {
        self.data.len() / self.row_size
    }

    /// Returns true if the bidislice contains no elements (that
    /// implies that its width, height and len are all zero).
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiMutSlice;
    ///
    /// let mut slice = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    /// let bslice = BidiMutSlice::new(&mut slice, 3).unwrap();
    ///
    /// assert!(!bslice.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Turns this bidislice back into the slice that was used to
    /// create it.
    pub fn into_slice(self) -> &'a mut [T] {
        self.data
    }

    /// Returns a raw pointer to the bidislice's buffer.
    ///
    /// The caller must ensure that the underlying slice outlives the pointer this
    /// function returns, or else it will end up pointing to garbage.
    ///
    /// The caller must also ensure that the memory the pointer (non-transitively) points to
    /// is never written to (except inside an [`UnsafeCell`][std::cell::UnsafeCell]) using this pointer or any pointer
    /// derived from it. If you need to mutate the contents of the slice, use [`Vec::as_mut_ptr`].
    pub fn as_ptr(&self) -> *const T {
        self.data.as_ptr()
    }

    /// Returns an unsafe mutable pointer to the bidislice's buffer.
    ///
    /// The caller must ensure that the underlying slice outlives the pointer this
    /// function returns, or else it will end up pointing to garbage.
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.data.as_mut_ptr()
    }

    /// Returns the two raw pointers spanning the slice.
    ///
    /// The returned range is half-open, which means that the end pointer
    /// points *one past* the last element of the slice. This way, an empty
    /// slice is represented by two equal pointers, and the difference between
    /// the two pointers represents the size of the slice.
    ///
    /// This function is useful for interacting with foreign interfaces which
    /// use two pointers to refer to a range of elements in memory, as is
    /// common in C++.
    ///
    /// Requires rustc 1.48 or later.
    #[rustversion::since(1.48)]
    pub fn as_ptr_range(&self) -> Range<*const T> {
        self.data.as_ptr_range()
    }

    /// Returns the two mutable raw pointers spanning the slice.
    ///
    /// The returned range is half-open, which means that the end pointer
    /// points *one past* the last element of the slice. This way, an empty
    /// slice is represented by two equal pointers, and the difference between
    /// the two pointers represents the size of the slice.
    ///
    /// This function is useful for interacting with foreign interfaces which
    /// use two pointers to refer to a range of elements in memory, as is
    /// common in C++.
    ///
    /// Requires rustc 1.48 or later.
    #[rustversion::since(1.48)]
    pub fn as_mut_ptr_range(&mut self) -> Range<*mut T> {
        self.data.as_mut_ptr_range()
    }

    /// Swaps two elements in the bidislice. If any of the coordinates are out
    /// of range, [`BidiError::OutOfBounds`] is returned
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiMutSlice;
    ///
    /// let mut slice = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    /// let mut bslice = BidiMutSlice::new(&mut slice, 3).unwrap();
    ///
    /// assert_eq!(bslice[(2, 0)], 3);
    /// assert_eq!(bslice[(1, 2)], 8);
    ///
    /// bslice.swap((2, 0), (1, 2)).unwrap();
    ///
    /// assert_eq!(bslice[(2, 0)], 8);
    /// assert_eq!(bslice[(1, 2)], 3);
    /// ```
    #[inline(always)]
    pub fn swap(&mut self, a: (usize, usize), b: (usize, usize)) -> Result<(), BidiError> {
        let idx_a = self.calc_index(a.0, a.1)?;
        let idx_b = self.calc_index(b.0, b.1)?;

        self.data.swap(idx_a, idx_b);
        Ok(())
    }

    /// Accesses an element in the BidiMutSlice, using its cartesian coordinates.
    /// If coordinates are outside of range, [`None`] is returned.
    ///
    /// If the error is not going to be handled, direct indexing is an easier
    /// way to achieve the same results.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiMutSlice;
    ///
    /// let mut slice = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    /// let bslice = BidiMutSlice::new(&mut slice, 3).unwrap();
    ///
    /// assert_eq!(*bslice.get(1, 1).unwrap(), 5);
    /// assert_eq!(bslice[(1, 1)], 5);
    /// ```
    #[inline(always)]
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        match self.calc_index(x, y) {
            Ok(idx) => Some(unsafe { self.data.get_unchecked(idx) }),
            Err(_) => None,
        }
    }

    /// Mutably accesses an element in the BidiMutSlice, using its cartesian coordinates.
    /// If coordinates are outside of range, [`None`] is returned.
    ///
    /// If the error is not going to be handled, direct indexing is an easier
    /// way to achieve the same results.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiMutSlice;
    ///
    /// let mut slice = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    /// let mut bslice = BidiMutSlice::new(&mut slice, 3).unwrap();
    ///
    /// *bslice.get_mut(1, 1).unwrap() = 12;
    ///
    /// assert_eq!(*bslice.get(1, 1).unwrap(), 12);
    ///
    /// bslice[(1, 1)] = 13;
    ///
    /// assert_eq!(*bslice.get(1, 1).unwrap(), 13);
    /// ```
    #[inline(always)]
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        match self.calc_index(x, y) {
            Ok(idx) => Some(unsafe { self.data.get_unchecked_mut(idx) }),
            Err(_) => None,
        }
    }

    /// Checks if the specified coordinates are inside the bidislice bounds
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiMutSlice;
    ///
    /// let mut slice = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    /// let bslice = BidiMutSlice::new(&mut slice, 3).unwrap();
    ///
    /// assert!(bslice.valid_coords(1, 1));
    /// assert!(!bslice.valid_coords(3, 3));
    /// ```
    #[inline(always)]
    pub fn valid_coords(&self, x: usize, y: usize) -> bool {
        self.calc_index(x, y).is_ok()
    }

    #[inline(always)]
    fn calc_index(&self, x: usize, y: usize) -> Result<usize, BidiError> {
        let idx = y * self.row_size + x;
        if x >= self.row_size || idx >= self.data.len() {
            Err(BidiError::OutOfBounds)
        } else {
            Ok(idx)
        }
    }

    /// Reverses the order of the items in the specified row.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiMutSlice;
    ///
    /// let mut slice = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    /// let mut bslice = BidiMutSlice::new(&mut slice, 3).unwrap();
    ///
    /// assert_eq!(bslice[(2, 1)], 6);
    ///
    /// bslice.reverse_row(1).unwrap();
    ///
    /// assert_eq!(bslice[(2, 1)], 4);
    /// ```
    pub fn reverse_row(&mut self, row: usize) -> Result<(), BidiError> {
        if row >= self.height() {
            return Err(BidiError::OutOfBounds);
        }

        let width = self.width();

        for x in 0..(width / 2) {
            self.swap((x, row), (width - 1 - x, row)).unwrap();
        }

        Ok(())
    }

    /// Reverses the order of the items in the specified column.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiMutSlice;
    ///
    /// let mut slice = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    /// let mut bslice = BidiMutSlice::new(&mut slice, 3).unwrap();
    ///
    /// assert_eq!(bslice[(1, 2)], 8);
    ///
    /// bslice.reverse_col(1).unwrap();
    ///
    /// assert_eq!(bslice[(1, 2)], 2);
    /// ```
    pub fn reverse_col(&mut self, col: usize) -> Result<(), BidiError> {
        if col >= self.width() {
            return Err(BidiError::OutOfBounds);
        }

        let height = self.height();

        for y in 0..(height / 2) {
            self.swap((col, y), (col, height - 1 - y)).unwrap();
        }

        Ok(())
    }

    /// Transposes the bidislice, that is an operation that flips the bidislice
    /// over its diagonal (or, more simply, switches the meaning of columns and
    /// rows). As such, the result of a transposition is as wide as the original
    /// was tall, and as tall as the original was wide.
    ///
    /// While this is performed in-place, it still requires O(n) additional memory
    /// if the bidislice width and height are different (i.e. it's not a square).
    pub fn transpose(&mut self) {
        let width = self.width();
        let height = self.height();

        if width == height {
            for x in 0..width {
                for y in (x + 1)..height {
                    self.swap((x, y), (y, x)).unwrap();
                }
            }
        } else if width != 0 && height != 0 {
            let last = self.data.len() - 1;
            let mut visited = vec![false; self.data.len()].into_boxed_slice();

            visited[0] = true;
            visited[last] = true;

            let mut next_index = Some(1);
            while let Some(mut i) = next_index {
                let cycle_start = i;
                loop {
                    let next = (i * width) % last;

                    if visited[next] {
                        visited[i] = true;
                        break;
                    }

                    self.data.swap(next, i);
                    visited[i] = true;
                    i = next;

                    if i == cycle_start {
                        break;
                    }
                }

                next_index = visited
                    .iter()
                    .enumerate()
                    .find(|(_, v)| !**v)
                    .map(|(idx, _)| idx);
            }
            self.row_size = height;
        }
    }

    /// Rotates the bidislice 90°, counter-clockwise (or, 270° clockwise).
    /// The result of such a rotation is as wide as the original
    /// was tall, and as tall as the original was wide.
    ///
    /// While this is performed in-place, it still requires O(n) additional memory
    /// if the bidislice width and height are different (i.e. it's not a square).
    pub fn rotate90ccw(&mut self) {
        self.transpose();
        self.reverse_columns();
    }

    /// Rotates the bidislice 180°.
    pub fn rotate180(&mut self) {
        self.data.reverse();
    }

    /// Rotates the bidislice 270°, counter-clockwise (or, 90° clockwise).
    /// The result of such a rotation is as wide as the original
    /// was tall, and as tall as the original was wide.
    ///
    /// While this is performed in-place, it still requires O(n) additional memory
    /// if the bidislice width and height are different (i.e. it's not a square).
    pub fn rotate270ccw(&mut self) {
        self.transpose();
        self.reverse_rows();
    }

    /// Reverse the order of items in all columns. This is equivalent to flipping
    /// the data structure over its horizontal axis.
    pub fn reverse_columns(&mut self) {
        for col in 0..self.width() {
            self.reverse_col(col).unwrap();
        }
    }

    /// Reverse the order of items in all rows. This is equivalent to flipping
    /// the data structure over its vertical axis.
    pub fn reverse_rows(&mut self) {
        for row in 0..self.height() {
            self.reverse_row(row).unwrap();
        }
    }

    /// Converts this bidislice to an immutable [`BidiView`].
    pub fn as_bidiview(&self) -> &dyn BidiView<Output = T> {
        self
    }

    /// Converts this bidislice to a mutable [`BidiView`].
    pub fn as_bidiview_mut(&mut self) -> &dyn BidiViewMut<Output = T> {
        self
    }

    /// Returns an iterator over the items of the view
    pub fn iter(&self) -> Iter<T, Self> {
        Iter::new(self)
    }

    /// Returns a mutable iterator over the items of the view
    pub fn iter_mut(&mut self) -> IterMut<T, Self> {
        IterMut::new(self)
    }
}

impl<'a, T> Index<(usize, usize)> for BidiMutSlice<'a, T> {
    type Output = T;

    /// Accesses an element in the BidiMutSlice, using its cartesian coordinates.
    /// If coordinates are outside of range, it panics.
    ///
    /// If you want a more graceful error handling, see [`BidiMutSlice::get`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiMutSlice;
    ///
    /// let mut slice = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    /// let bslice = BidiMutSlice::new(&mut slice, 3).unwrap();
    ///
    /// assert_eq!(bslice[(1, 1)], 5);
    /// ```
    #[inline(always)]
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let idx = self.calc_index(index.0, index.1).unwrap_or_else(|_| {
            panic!(
                "Indexes out of bidislice bounds: ({},{}) out of {}x{}",
                index.0,
                index.1,
                self.width(),
                self.height()
            )
        });
        unsafe { self.data.get_unchecked(idx) }
    }
}

impl<'a, T> IndexMut<(usize, usize)> for BidiMutSlice<'a, T> {
    /// Mutably accesses an element in the BidiMutSlice, using its cartesian coordinates.
    /// If coordinates are outside of range, it panics.
    ///
    /// If you want a more graceful error handling, see [`BidiMutSlice::get_mut`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiMutSlice;
    ///
    /// let mut slice = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    /// let mut bslice = BidiMutSlice::new(&mut slice, 3).unwrap();
    ///
    /// bslice[(1, 1)] = 12;
    ///
    /// assert_eq!(bslice[(1, 1)], 12);
    /// ```
    #[inline(always)]
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let idx = self.calc_index(index.0, index.1).unwrap_or_else(|_| {
            panic!(
                "Indexes out of bidislice bounds: ({},{}) out of {}x{}",
                index.0,
                index.1,
                self.width(),
                self.height()
            )
        });
        unsafe { self.data.get_unchecked_mut(idx) }
    }
}

impl<'a, T> BidiView for BidiMutSlice<'a, T> {
    fn width(&self) -> usize {
        BidiMutSlice::<T>::width(self)
    }
    fn height(&self) -> usize {
        BidiMutSlice::<T>::height(self)
    }

    fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.get(x, y)
    }
}

impl<'a, T> BidiViewMut for BidiMutSlice<'a, T> {
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.get_mut(x, y)
    }
}

unsafe impl<'a, T> BidiViewMutIterable for BidiMutSlice<'a, T> {}
