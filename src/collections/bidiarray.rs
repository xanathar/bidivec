use crate::bidiiter::{Iter, IterMut};
use core::slice::SliceIndex;
use std::default::Default;
#[rustversion::since(1.48)]
use std::ops::Range;
use std::ops::{Index, IndexMut};

use crate::*;

/// A contiguous bidimensional array type with heap-allocated contents,
/// based on an underlying `Box<[T]>`, non-growable (that is, preserving
/// the same size, but not necessarily the same width and height).
///
/// BidiArrays have immutable size after creation; if you need mutable size, use
/// a [`BidiVec<T>`] (which preserves linear layout) or a [`BidiGrowVec<T>`].
///
/// Note that BidiArrays can still mutate their width or height if
/// in-place transformations are applied (e.g. [`BidiArray::transpose()`]). What
/// is guaranteed to be constant is [`BidiArray::len()`], or the product of
/// width and height.
///
/// This bidimensional data structure lays out its elements linearly in memory,
/// for better interoperability with native code and greater efficiency (due to
/// memory locality) when the data structure is not changing.
///
/// Most methods will not implicitly panic if out-of-bounds accesses are attempted,
/// but they will return a [`BidiError`] for graceful error handling.
///
/// # Examples
///
/// You can explicitly create a [`BidiArray`] with [`BidiArray::new`]:
///
/// ```
/// # use bidivec::BidiArray;
/// let v: BidiArray<i32> = BidiArray::new();
/// ```
///
/// or by using the [`bidiarray!`] macro:
///
/// ```
/// # use bidivec::{BidiArray, bidiarray};
/// // this creates an empty bidiarray
/// let v: BidiArray<i32> = bidiarray![];
///
/// // this creates a 3x2 bidiarray, with all items equal to 1
/// let v = bidiarray![1; 3, 2];
///
/// // this creates a 3x2 bidiarray from items; the final '3' is the
/// // bidiarray's width
/// let v = bidiarray![1, 2, 3, 4, 5, 6; 3];
///
/// // this creates a 3x2 bidiarray, by listing the rows separately
/// let v = bidiarray!{
///     [1, 2, 3],
///     [4, 5, 6],
/// };
/// ```
///
/// BidiArrays support indexing with cartesian coordinates (through the [`Index`] and [`IndexMut`] traits);
/// note that coordinates out of range will cause the code to panic.
/// The [`get`][`BidiArray::get`] and [`get_mut`][`BidiArray::get_mut`] methods offer a safer way to access
/// the bidiarray contents, by returning an [`Option<T>`], in the same vein of `Vec<T>`.
///
/// ```
/// # use bidivec::{BidiArray, bidiarray};
/// let mut v = bidiarray!{[1, 2, 3], [4, 5, 6]};
/// let four = v[(0, 1)];
/// v[(1, 1)] = v[(1, 0)] + v[(2, 0)];
/// ```
#[derive(Clone, Default, Debug, PartialEq)]
pub struct BidiArray<T> {
    pub(crate) data: Box<[T]>,
    pub(crate) row_size: usize,
}

impl<T> BidiArray<T> {
    /// Constructs a new, empty [`BidiArray<T>`].
    ///
    /// # Examples
    ///
    /// ```
    /// # #![allow(unused_mut)]
    /// use bidivec::BidiArray;
    ///
    /// let mut bvec: BidiArray<i32> = BidiArray::new();
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self {
            data: Vec::new().into_boxed_slice(),
            row_size: 0,
        }
    }

    /// Constructs a new [`BidiArray<T>`] with the specified size,
    /// cloning the specified item in every position.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds [`isize::MAX`] bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiArray;
    ///
    /// let mut bvec = BidiArray::with_elem(5, 3, 3);
    ///
    /// assert_eq!(bvec.len(), 9);
    /// assert_eq!(bvec.width(), 3);
    /// assert_eq!(bvec.height(), 3);
    /// assert_eq!(bvec[(1, 2)], 5);
    /// ```
    pub fn with_elem(value: T, width: usize, height: usize) -> Self
    where
        T: Clone,
    {
        Self {
            data: vec![value; width * height].into_boxed_slice(),
            row_size: width,
        }
    }

    /// Constructs a new [`BidiArray<T>`] with the specified size,
    /// using the default value in every position.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds [`isize::MAX`] bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiArray;
    ///
    /// let mut bvec = BidiArray::<i32>::with_size_default(3, 3);
    ///
    /// assert_eq!(bvec.len(), 9);
    /// assert_eq!(bvec.width(), 3);
    /// assert_eq!(bvec.height(), 3);
    /// assert_eq!(bvec[(1, 2)], 0);
    /// ```
    pub fn with_size_default(width: usize, height: usize) -> Self
    where
        T: Default,
    {
        Self::with_size_func(width, height, T::default)
    }

    /// Constructs a new [`BidiArray<T>`] with the specified size,
    /// using the specified closure to produce values.
    /// The order the closure is called when producing a new value is
    /// not guaranteed. If the item produced is depending on the its
    /// coordinates, use the slower `BidiArray<T>::with_size_func_xy`.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds [`isize::MAX`] bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiArray;
    ///
    /// let mut bvec = BidiArray::with_size_func(3, 3, ||137);
    ///
    /// assert_eq!(bvec.len(), 9);
    /// assert_eq!(bvec.width(), 3);
    /// assert_eq!(bvec.height(), 3);
    /// assert_eq!(bvec[(1, 2)], 137);
    /// ```
    pub fn with_size_func<F>(width: usize, height: usize, f: F) -> Self
    where
        F: FnMut() -> T,
    {
        let mut vec = Vec::with_capacity(width * height);
        vec.resize_with(width * height, f);

        Self {
            data: vec.into_boxed_slice(),
            row_size: width,
        }
    }

    /// Constructs a new [`BidiArray<T>`] with the specified size,
    /// using the specified closure to produce values.
    /// The order the closure is called when producing a new value is
    /// not guaranteed, but the closure will receive the item coordinates
    /// as an input. If the coordinates are not needed, `BidiArray::with_size_func`
    /// is faster and uses less temporary memory (this method uses up
    /// to a row or column size of temporary memory).
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds [`isize::MAX`] bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiArray;
    ///
    /// let mut bvec = BidiArray::with_size_func_xy(3, 3, |x,y| x+y);
    ///
    /// assert_eq!(bvec.len(), 9);
    /// assert_eq!(bvec.width(), 3);
    /// assert_eq!(bvec.height(), 3);
    /// assert_eq!(bvec[(1, 2)], 3);
    /// ```
    pub fn with_size_func_xy<F>(width: usize, height: usize, mut f: F) -> Self
    where
        F: FnMut(usize, usize) -> T,
    {
        let mut vec = Vec::with_capacity(width * height);

        for y in 0..height {
            for x in 0..width {
                vec.push(f(x, y));
            }
        }

        Self {
            data: vec.into_boxed_slice(),
            row_size: width,
        }
    }

    /// Creates a [`BidiArray<T>`] directly from the raw components of another vector.
    ///
    /// # Safety
    ///
    /// This is essentially the same to `std::slice::from_raw_parts_mut`, so the same
    /// caveats apply.
    ///
    /// As this is highly unsafe, please check the documentation of `std::slice::from_raw_parts_mut`
    /// before using this function.
    pub unsafe fn from_raw_parts(
        ptr: *mut T,
        length: usize,
        row_size: usize,
    ) -> Result<Self, BidiError> {
        if (length == 0 && row_size == 0) || (row_size != 0 && (length % row_size) == 0) {
            Ok(Self {
                data: Box::from_raw(std::slice::from_raw_parts_mut(ptr, length)),
                row_size,
            })
        } else {
            Err(BidiError::IncompatibleSize)
        }
    }

    /// Creates a bidiarray from a draining iterator, using the specified
    /// `row_size`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiArray, bidiarray};
    ///
    /// let mut vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
    /// let mut bvec = BidiArray::from_iterator(vec.drain(..), 3)?;
    ///
    /// let expected = bidiarray!{
    ///     [0, 1, 2],
    ///     [3, 4, 5],
    ///     [6, 7, 8],
    /// };
    ///
    /// assert_eq!(bvec, expected);
    /// # Ok::<(), bidivec::BidiError>(())
    /// ```
    pub fn from_iterator(
        iter: impl Iterator<Item = T>,
        row_size: usize,
    ) -> Result<Self, BidiError> {
        let vec = iter.collect::<Vec<T>>();
        Self::from_vec(vec, row_size)
    }

    /// Creates a bidiarray from another view iterator, using the specified
    /// mapping function to create and/or transform elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiArray, bidiarray};
    ///
    /// let from = bidiarray!{
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9],
    /// };
    ///
    /// let to = BidiArray::from_view_map(&from, |n| n - 1);
    ///
    /// assert_eq!(to, bidiarray!{
    ///     [0, 1, 2],
    ///     [3, 4, 5],
    ///     [6, 7, 8],
    /// });
    /// ```
    pub fn from_view_map<V, F>(view: &V, mapper: F) -> Self
    where
        V: BidiView,
        F: Fn(&V::Output) -> T,
    {
        Self::with_size_func_xy(view.width(), view.height(), |x, y| mapper(&view[(x, y)]))
    }

    /// Creates a [`BidiArray<T>`] from a [`Vec<T>`] and a specified row size.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiArray;
    ///
    /// let vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
    /// let mut bvec = BidiArray::from_vec(vec, 3)?;
    ///
    /// assert_eq!(bvec.len(), 9);
    /// assert_eq!(bvec.width(), 3);
    /// assert_eq!(bvec.height(), 3);
    /// assert_eq!(bvec[(1, 2)], 7);
    /// # Ok::<(), bidivec::BidiError>(())
    /// ```
    pub fn from_vec(vec: Vec<T>, row_size: usize) -> Result<Self, BidiError> {
        if (vec.is_empty() && row_size == 0) || row_size != 0 && (vec.len() % row_size) == 0 {
            Ok(Self {
                data: vec.into_boxed_slice(),
                row_size,
            })
        } else {
            Err(BidiError::IncompatibleSize)
        }
    }

    /// Returns the number of items contained in the bidiarray.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiArray, bidiarray};
    ///
    /// let mut bvec = bidiarray![5; 4, 3];
    ///
    /// assert_eq!(bvec.len(), 12);
    /// ```
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns the width (that is, the size of a row) in the bidiarray.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiArray, bidiarray};
    ///
    /// let mut bvec = bidiarray![5; 4, 3];
    ///
    /// assert_eq!(bvec.width(), 4);
    /// ```
    pub fn width(&self) -> usize {
        self.row_size
    }

    /// Returns the height (that is, the size of a column) in the bidiarray.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiArray, bidiarray};
    ///
    /// let mut bvec = bidiarray![5; 4, 3];
    ///
    /// assert_eq!(bvec.height(), 3);
    /// ```
    pub fn height(&self) -> usize {
        self.data.len() / self.row_size
    }

    /// Returns true if the bidiarray contains no elements (that
    /// implies that its width, height and len are all zero).
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiArray;
    ///
    /// let mut bvec = BidiArray::<i32>::new();
    ///
    /// assert!(bvec.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Extracts a slice containing the specified range of bidiarray contents,
    /// laid out linearly, by rows.
    pub fn as_slice<R: SliceIndex<[T]>>(&self, range: R) -> &R::Output {
        &self.data[range]
    }

    /// Extracts a slice containing the specified range of bidiarray contents,
    /// laid out linearly, by rows.
    pub fn as_mut_slice<R: SliceIndex<[T]>>(&mut self, range: R) -> &mut R::Output {
        &mut self.data[range]
    }

    /// Returns a raw pointer to the bidiarray's buffer.
    ///
    /// The caller must ensure that the bidiarray outlives the pointer this
    /// function returns, or else it will end up pointing to garbage.
    /// Modifying the bidiarray may cause its buffer to be reallocated,
    /// which would also make any pointers to it invalid.
    ///
    /// The caller must also ensure that the memory the pointer (non-transitively) points to
    /// is never written to (except inside an [`UnsafeCell`][std::cell::UnsafeCell]) using this pointer or any pointer
    /// derived from it. If you need to mutate the contents of the slice, use [`Vec::as_mut_ptr`].
    pub fn as_ptr(&self) -> *const T {
        self.data.as_ptr()
    }

    /// Returns an unsafe mutable pointer to the vector's buffer.
    ///
    /// The caller must ensure that the vector outlives the pointer this
    /// function returns, or else it will end up pointing to garbage.
    /// Modifying the vector may cause its buffer to be reallocated,
    /// which would also make any pointers to it invalid.
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
    /// See [`Vec<T>::as_ptr`] for warnings on using these pointers. The end pointer
    /// requires extra caution, as it does not point to a valid element in the
    /// slice.
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
    /// See [`Vec<T>::as_mut_ptr`] and [`Vec<T>::as_ptr`] for warnings on using these pointers.
    /// The end pointer requires extra caution, as it does not point to a valid
    /// element in the slice.
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

    /// Converts the vector into a `Box<[T]>` where items are linearly
    /// laid out by rows.
    pub fn into_boxed_slice(self) -> Box<[T]> {
        self.data
    }

    /// Converts the vector into a [`Vec<T>`] where items are linearly
    /// laid out by rows.
    pub fn into_vec(self) -> Vec<T> {
        self.data.into_vec()
    }

    /// Swaps two elements in the bidiarray. If any of the coordinates are out
    /// of range, [`BidiError::OutOfBounds`] is returned
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiArray, bidiarray};
    ///
    /// let mut bvec = bidiarray!{
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9],
    /// };
    ///
    /// assert_eq!(bvec[(2, 0)], 3);
    /// assert_eq!(bvec[(1, 2)], 8);
    ///
    /// bvec.swap((2, 0), (1, 2))?;
    ///
    /// assert_eq!(bvec[(2, 0)], 8);
    /// assert_eq!(bvec[(1, 2)], 3);
    /// # Ok::<(), bidivec::BidiError>(())
    /// ```
    #[inline(always)]
    pub fn swap(&mut self, a: (usize, usize), b: (usize, usize)) -> Result<(), BidiError> {
        let idx_a = self.calc_index(a.0, a.1)?;
        let idx_b = self.calc_index(b.0, b.1)?;

        self.data.swap(idx_a, idx_b);
        Ok(())
    }

    /// Accesses an element in the BidiArray, using its cartesian coordinates.
    /// If coordinates are outside of range, [`None`] is returned.
    ///
    /// If the error is not going to be handled, direct indexing is an easier
    /// way to achieve the same results.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiArray, bidiarray};
    ///
    /// let mut bvec = bidiarray!{
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9],
    /// };
    ///
    /// assert_eq!(*bvec.get(1, 1).unwrap(), 5);
    /// assert_eq!(bvec[(1, 1)], 5);
    /// ```
    #[inline(always)]
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        match self.calc_index(x, y) {
            Ok(idx) => Some(unsafe { self.data.get_unchecked(idx) }),
            Err(_) => None,
        }
    }

    /// Mutably accesses an element in the BidiArray, using its cartesian coordinates.
    /// If coordinates are outside of range, [`None`] is returned.
    ///
    /// If the error is not going to be handled, direct indexing is an easier
    /// way to achieve the same results.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiArray, bidiarray};
    ///
    /// let mut bvec = bidiarray!{
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9],
    /// };
    ///
    /// *bvec.get_mut(1, 1).unwrap() = 12;
    ///
    /// assert_eq!(*bvec.get(1, 1).unwrap(), 12);
    ///
    /// bvec[(1, 1)] = 13;
    ///
    /// assert_eq!(*bvec.get(1, 1).unwrap(), 13);
    /// ```
    #[inline(always)]
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        match self.calc_index(x, y) {
            Ok(idx) => Some(unsafe { self.data.get_unchecked_mut(idx) }),
            Err(_) => None,
        }
    }

    /// Checks if the specified coordinates are inside the bidiarray bounds
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiArray, bidiarray};
    ///
    /// let mut bvec = bidiarray!{
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9],
    /// };
    ///
    /// assert!(bvec.valid_coords(1, 1));
    /// assert!(!bvec.valid_coords(3, 3));
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
    /// use bidivec::{BidiArray, bidiarray};
    ///
    /// let mut bvec = bidiarray!{
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9],
    /// };
    ///
    /// assert_eq!(bvec[(2, 1)], 6);
    ///
    /// bvec.reverse_row(1).unwrap();
    ///
    /// assert_eq!(bvec[(2, 1)], 4);
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
    /// use bidivec::{BidiArray, bidiarray};
    ///
    /// let mut bvec = bidiarray!{
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9],
    /// };
    ///
    /// assert_eq!(bvec[(1, 2)], 8);
    ///
    /// bvec.reverse_col(1).unwrap();
    ///
    /// assert_eq!(bvec[(1, 2)], 2);
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

    /// Transposes the bidiarray, that is an operation that flips the bidiarray
    /// over its diagonal (or, more simply, switches the meaning of columns and
    /// rows). As such, the result of a transposition is as wide as the original
    /// was tall, and as tall as the original was wide.
    ///
    /// While this is performed in-place, it still requires O(n) additional memory
    /// if the bidiarray width and height are different (i.e. it's not a square).
    pub fn transpose(&mut self) {
        let mut slice = BidiMutSlice::new(&mut self.data, self.row_size).unwrap();
        slice.transpose();
        self.row_size = self.height();
    }

    /// Rotates the bidiarray 90°, counter-clockwise (or, 270° clockwise).
    /// The result of such a rotation is as wide as the original
    /// was tall, and as tall as the original was wide.
    ///
    /// While this is performed in-place, it still requires O(n) additional memory
    /// if the bidiarray width and height are different (i.e. it's not a square).
    pub fn rotate90ccw(&mut self) {
        self.transpose();
        self.reverse_columns();
    }

    /// Rotates the bidiarray 180°.
    pub fn rotate180(&mut self) {
        self.data.reverse();
    }

    /// Rotates the bidiarray 270°, counter-clockwise (or, 90° clockwise).
    /// The result of such a rotation is as wide as the original
    /// was tall, and as tall as the original was wide.
    ///
    /// While this is performed in-place, it still requires O(n) additional memory
    /// if the bidiarray width and height are different (i.e. it's not a square).
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

    /// Converts this instance into a [`BidiVec<T>`]
    /// This operation is `O(1)` in the worst case.
    pub fn into_bidivec(self) -> BidiVec<T> {
        BidiVec::<T>::from(self)
    }

    /// Converts this instance into a [`BidiGrowVec<T>`]
    /// This operation is `O(width*height)` in the worst case.
    pub fn into_bidigrowvec(self) -> BidiGrowVec<T> {
        BidiGrowVec::<T>::from(self)
    }

    /// Converts this bidiarray to an immutable [`BidiView`].
    pub fn as_bidiview(&self) -> &dyn BidiView<Output = T> {
        self
    }

    /// Converts this bidiarray to a mutable [`BidiView`].
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

impl<T> BidiFrom<&dyn BidiView<Output = T>> for BidiArray<T>
where
    T: Clone,
{
    fn from_view(source: &dyn BidiView<Output = T>) -> Result<Self, BidiError> {
        Ok(BidiArray::<T>::with_size_func_xy(
            source.width(),
            source.height(),
            |x, y| source[(x, y)].clone(),
        ))
    }

    fn from_view_cut(source: &dyn BidiView<Output = T>, cut: &BidiRect) -> Result<Self, BidiError> {
        if cut.max_x() > source.height() || cut.max_y() > source.width() {
            return Err(BidiError::OutOfBounds);
        }

        Ok(BidiArray::<T>::with_size_func_xy(
            cut.width,
            cut.height,
            |x, y| source[(x + cut.x, y + cut.y)].clone(),
        ))
    }
}

impl<T> BidiFrom<BidiVec<T>> for BidiArray<T> {
    fn from_view(source: BidiVec<T>) -> Result<Self, BidiError> {
        Ok(Self::from(source))
    }

    fn from_view_cut(mut source: BidiVec<T>, cut: &BidiRect) -> Result<Self, BidiError> {
        source.crop(cut)?;
        Ok(Self::from(source))
    }
}

impl<T> BidiFrom<BidiGrowVec<T>> for BidiArray<T> {
    fn from_view(source: BidiGrowVec<T>) -> Result<Self, BidiError> {
        Ok(Self::from(source))
    }

    fn from_view_cut(mut source: BidiGrowVec<T>, cut: &BidiRect) -> Result<Self, BidiError> {
        source.crop(cut)?;
        Ok(Self::from(source))
    }
}

impl<T> BidiFrom<BidiArray<T>> for BidiArray<T> {
    fn from_view(source: BidiArray<T>) -> Result<Self, BidiError> {
        Ok(source)
    }

    fn from_view_cut(source: BidiArray<T>, cut: &BidiRect) -> Result<Self, BidiError> {
        let mut source = BidiVec::from(source);
        source.crop(cut)?;
        Ok(Self::from(source))
    }
}

impl<T> Index<(usize, usize)> for BidiArray<T> {
    type Output = T;

    /// Accesses an element in the BidiArray, using its cartesian coordinates.
    /// If coordinates are outside of range, it panics.
    ///
    /// If you want a more graceful error handling, see [`BidiArray::get`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiArray, bidiarray};
    ///
    /// let mut bvec = bidiarray!{
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9],
    /// };
    ///
    /// assert_eq!(bvec[(1, 1)], 5);
    /// ```
    #[inline(always)]
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let idx = self.calc_index(index.0, index.1).unwrap_or_else(|_| {
            panic!(
                "Indexes out of bidiarray bounds: ({},{}) out of {}x{}",
                index.0,
                index.1,
                self.width(),
                self.height()
            )
        });
        unsafe { self.data.get_unchecked(idx) }
    }
}

impl<T> IndexMut<(usize, usize)> for BidiArray<T> {
    /// Mutably accesses an element in the BidiArray, using its cartesian coordinates.
    /// If coordinates are outside of range, it panics.
    ///
    /// If you want a more graceful error handling, see [`BidiArray::get_mut`]
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiArray, bidiarray};
    ///
    /// let mut bvec = bidiarray!{
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9],
    /// };
    ///
    /// bvec[(1, 1)] = 12;
    ///
    /// assert_eq!(bvec[(1, 1)], 12);
    /// ```
    #[inline(always)]
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let idx = self.calc_index(index.0, index.1).unwrap_or_else(|_| {
            panic!(
                "Indexes out of bidiarray bounds: ({},{}) out of {}x{}",
                index.0,
                index.1,
                self.width(),
                self.height()
            )
        });
        unsafe { self.data.get_unchecked_mut(idx) }
    }
}

impl<T> BidiView for BidiArray<T> {
    fn width(&self) -> usize {
        BidiArray::<T>::width(self)
    }
    fn height(&self) -> usize {
        BidiArray::<T>::height(self)
    }

    fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.get(x, y)
    }
}

impl<T> BidiViewMut for BidiArray<T> {
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.get_mut(x, y)
    }
}

unsafe impl<T> BidiViewMutIterable for BidiArray<T> {}

impl<T> From<BidiVec<T>> for BidiArray<T> {
    /// Creates a new instance of [`BidiArray<T>`] from an existing [`BidiVec<T>`].
    /// This operation is `O(1)` in the worst case.
    fn from(other: BidiVec<T>) -> Self {
        Self {
            data: other.data.into_boxed_slice(),
            row_size: other.row_size.unwrap_or(0),
        }
    }
}

impl<T> From<BidiGrowVec<T>> for BidiArray<T> {
    /// Creates a new instance of [`BidiArray<T>`] from an existing [`BidiVec<T>`].
    /// This operation is `O(width*height)` in the worst case.
    fn from(other: BidiGrowVec<T>) -> Self {
        let other = BidiVec::<T>::from(other);
        Self {
            data: other.data.into_boxed_slice(),
            row_size: other.row_size.unwrap_or(0),
        }
    }
}
