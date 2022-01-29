use crate::bidiiter::Iter;
use std::ops::Index;
#[rustversion::since(1.48)]
use std::ops::Range;

use crate::{BidiError, BidiView};

/// A bidimensional view over an immutable slice (for the mutable version,
/// see [`BidiMutSlice`][crate::BidiMutSlice]).
/// Items are expected to be linearly arranged per rows.
///
/// Most methods will not implicitly panic if out-of-bounds accesses are attempted,
/// but they will return a [`BidiError`] for graceful error handling.
///
/// # Examples
///
/// ```
/// use bidivec::BidiSlice;
///
/// let slice = [1, 2, 3, 4, 5, 6, 7, 8, 9];
/// let bslice = BidiSlice::new(&slice, 3).unwrap();
///
/// assert_eq!(bslice[(1, 1)], 5);
/// ```
#[derive(Debug)]
pub struct BidiSlice<'a, T> {
    pub(crate) data: &'a [T],
    pub(crate) row_size: usize,
}

impl<'a, T> BidiSlice<'a, T> {
    /// Constructs a new, empty `BidiSlice<T>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiSlice;
    ///
    /// let slice = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    /// let bslice = BidiSlice::new(&slice, 3).unwrap();
    /// ```
    #[inline]
    pub fn new(data: &'a [T], row_size: usize) -> Result<Self, BidiError> {
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
    /// use bidivec::BidiSlice;
    ///
    /// let slice = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    /// let bslice = BidiSlice::new(&slice, 3).unwrap();
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
    /// use bidivec::BidiSlice;
    ///
    /// let slice = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    /// let bslice = BidiSlice::new(&slice, 3).unwrap();
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
    /// use bidivec::BidiSlice;
    ///
    /// let slice = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    /// let bslice = BidiSlice::new(&slice, 3).unwrap();
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
    /// use bidivec::BidiSlice;
    ///
    /// let slice = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    /// let bslice = BidiSlice::new(&slice, 3).unwrap();
    ///
    /// assert!(!bslice.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Turns this bidislice back into the slice that was used to
    /// create it.
    pub fn into_slice(self) -> &'a [T] {
        self.data
    }

    /// Gets the slice that created this bidislice.
    pub fn as_slice(self) -> &'a [T] {
        self.data
    }

    /// Returns a raw pointer to the bidislice's buffer.
    ///
    /// The caller must ensure that the underlying slice outlives the pointer this
    /// function returns, or else it will end up pointing to garbage.
    ///
    /// The caller must also ensure that the memory the pointer (non-transitively) points to
    /// is never written to (except inside an [`UnsafeCell`][std::cell::UnsafeCell]) using this pointer or any pointer
    /// derived from it.
    pub fn as_ptr(&self) -> *const T {
        self.data.as_ptr()
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

    /// Accesses an element in the BidiSlice, using its cartesian coordinates.
    /// If coordinates are outside of range, [`None`] is returned.
    ///
    /// If the error is not going to be handled, direct indexing is an easier
    /// way to achieve the same results.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiSlice;
    ///
    /// let slice = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    /// let bslice = BidiSlice::new(&slice, 3).unwrap();
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

    /// Checks if the specified coordinates are inside the bidislice bounds
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiSlice;
    ///
    /// let slice = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    /// let bslice = BidiSlice::new(&slice, 3).unwrap();
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

    /// Converts this bidislice to an immutable [`BidiView`].
    pub fn as_bidiview(&self) -> &dyn BidiView<Output = T> {
        self
    }

    /// Returns an iterator over the items of the view
    pub fn iter(&self) -> Iter<T, Self> {
        Iter::new(self)
    }
}

impl<'a, T> Index<(usize, usize)> for BidiSlice<'a, T> {
    type Output = T;

    /// Accesses an element in the BidiSlice, using its cartesian coordinates.
    /// If coordinates are outside of range, it panics.
    ///
    /// If you want a more graceful error handling, see [`BidiSlice::get`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiSlice;
    ///
    /// let slice = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    /// let bslice = BidiSlice::new(&slice, 3).unwrap();
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

impl<'a, T> BidiView for BidiSlice<'a, T> {
    fn width(&self) -> usize {
        BidiSlice::<T>::width(self)
    }
    fn height(&self) -> usize {
        BidiSlice::<T>::height(self)
    }

    fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.get(x, y)
    }
}
