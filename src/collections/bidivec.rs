use core::slice::SliceIndex;
use std::cmp::{min, Ordering};
#[rustversion::since(1.57)]
use std::collections::TryReserveError;
use std::default::Default;
use std::iter::Iterator;
#[rustversion::since(1.48)]
use std::ops::Range;
use std::ops::{Index, IndexMut};

use crate::bidiiter::{Iter, IterMut};
use crate::*;

#[cfg(debug_assertions)]
macro_rules! check_consistent {
    ($e:expr) => {{
        let this = $e;
        match this.row_size {
            None => if !this.data.is_empty() {
                panic!("BidiVec consistency failed: vec has {} elements, row_size is {:?}", this.data.len(), this.row_size);
            }
            Some(l) => if this.data.is_empty() || (this.data.len() % l) != 0 {
                panic!("BidiVec consistency failed: vec has {} elements, row_size is {:?}", this.data.len(), this.row_size);
            },
        };
    }};
}

#[cfg(not(debug_assertions))]
macro_rules! check_consistent {
    ($e:expr) => {};
}

/// A contiguous growable bidimensional array type with heap-allocated contents,
/// based on an underlying `Vec<T>`.
///
/// BidiVecs have `O(1)` indexing, amortized `O(1)` push of rows (to the end) and
/// `O(row_length)` pops (from the end).
///
/// This bidimensional data structure lays out its elements linearly in memory,
/// for better interoperability with native code and greater efficiency (due to
/// memory locality) when the data structure is not changing. This comes at a price
/// of higher cost for column based insertions and removals and mid-collection row
/// insertions and removals. For a more efficient type for these scenarios, see
/// [`BidiGrowVec`].
///
/// Most methods will not implicitly panic if out-of-bounds accesses are attempted,
/// but they will return a [`BidiError`] for graceful error handling.
///
/// # Examples
///
/// You can explicitly create a [`BidiVec`] with [`BidiVec::new`]:
///
/// ```
/// # use bidivec::BidiVec;
/// let v: BidiVec<i32> = BidiVec::new();
/// ```
///
/// or by using the [`bidivec!`] macro:
///
/// ```
/// # use bidivec::{BidiVec, bidivec};
/// // this creates an empty bidivec
/// let v: BidiVec<i32> = bidivec![];
///
/// // this creates a 3x2 bidivec, with all items equal to 1
/// let v = bidivec![1; 3, 2];
///
/// // this creates a 3x2 bidivec from items; the final '3' is the
/// // bidivec's width
/// let v = bidivec![1, 2, 3, 4, 5, 6; 3];
///
/// // this creates a 3x2 bidivec, by listing the rows separately
/// let v = bidivec!{
///     [1, 2, 3],
///     [4, 5, 6],
/// };
/// ```
///
/// You can push rows onto the end of a bidivec (which will grow the bidivec
/// as needed):
///
/// ```
/// # use bidivec::{BidiVec, bidivec};
/// let mut v = bidivec![1, 2; 1];
///
/// v.push_row([3, 4]);
/// ```
///
/// Popping rows works in much the same way:
///
/// ```
/// # use bidivec::{BidiVec, bidivec};
/// let mut v = bidivec![1, 2; 1];
///
/// let one_and_two = v.pop_row();
/// ```
///
/// BidiVecs support indexing with cartesian coordinates (through the [`Index`] and [`IndexMut`] traits);
/// note that coordinates out of range will cause the code to panic.
/// The [`BidiVec::get`] and [`BidiVec::get_mut`] methods offer a safer way to access the bidivec contents,
/// by returning an `Option`, in the same vein of `Vec<T>`.
///
/// ```
/// # use bidivec::{BidiVec, bidivec};
/// let mut v = bidivec!{
///     [1, 2, 3],
///     [4, 5, 6],
/// };
/// let four = v[(0, 1)];
/// v[(1, 1)] = v[(1, 0)] + v[(2, 0)];
/// ```
#[derive(Clone, Default, Debug, PartialEq)]
pub struct BidiVec<T> {
    pub(crate) data: Vec<T>,
    pub(crate) row_size: Option<usize>,
}

impl<T> BidiVec<T> {
    /// Constructs a new, empty [`BidiVec<T>`].
    ///
    /// # Examples
    ///
    /// ```
    /// # #![allow(unused_mut)]
    /// use bidivec::BidiVec;
    ///
    /// let mut bvec: BidiVec<i32> = BidiVec::new();
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            row_size: None,
        }
    }

    /// Constructs a new, empty [`BidiVec<T>`] with the specified capacity.
    ///
    /// The bidivec will be able to hold exactly `capacity` elements without
    /// reallocating. If `capacity` is 0, the bidivec will not allocate.
    /// See the documentation of `Vec<T>` for further details.
    ///
    /// Note that the capacity is expressed in terms of elements, so to
    /// store, for example, a 3x3 bidivec, the capacity to request should be
    /// 9.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds [`isize::MAX`] bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiVec;
    ///
    /// let mut bvec = BidiVec::with_capacity(9);
    ///
    /// // The bidivec contains no items, even though it has capacity for more
    /// assert_eq!(bvec.len(), 0);
    /// assert_eq!(bvec.capacity(), 9);
    ///
    /// // These are all done without reallocating...
    /// bvec.push_row([0, 1, 2]).unwrap();
    /// bvec.push_row([3, 4, 5]).unwrap();
    /// bvec.push_row([6, 7, 8]).unwrap();
    /// assert_eq!(bvec.len(), 9);
    /// assert_eq!(bvec.width(), 3);
    /// assert_eq!(bvec.height(), 3);
    /// assert_eq!(bvec.capacity(), 9);
    ///
    /// // ...but this may make the bidivec reallocate
    /// bvec.push_row([9, 10, 11]).unwrap();
    /// assert_eq!(bvec.len(), 12);
    /// assert_eq!(bvec.width(), 3);
    /// assert_eq!(bvec.height(), 4);
    /// assert!(bvec.capacity() >= 12);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            row_size: None,
        }
    }

    /// Constructs a new, empty [`BidiVec<T>`] with the capacity to handle the
    /// specified size without reallocating.
    ///
    /// Note that the specified size doesn't put any constraint on the size
    /// that the bidivec will have.
    ///
    /// See the documentation of `Vec<T>` for further details.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds [`isize::MAX`] bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiVec;
    ///
    /// let mut bvec = BidiVec::with_capacity_size(3, 3);
    ///
    /// // The bidivec contains no items, even though it has capacity for more
    /// assert_eq!(bvec.len(), 0);
    /// assert_eq!(bvec.capacity(), 9);
    ///
    /// // These are all done without reallocating...
    /// bvec.push_row([0, 1, 2]).unwrap();
    /// bvec.push_row([3, 4, 5]).unwrap();
    /// bvec.push_row([6, 7, 8]).unwrap();
    /// assert_eq!(bvec.len(), 9);
    /// assert_eq!(bvec.width(), 3);
    /// assert_eq!(bvec.height(), 3);
    /// assert_eq!(bvec.capacity(), 9);
    ///
    /// // ...but this may make the bidivec reallocate
    /// bvec.push_row([9, 10, 11]).unwrap();
    /// assert_eq!(bvec.len(), 12);
    /// assert_eq!(bvec.width(), 3);
    /// assert_eq!(bvec.height(), 4);
    /// assert!(bvec.capacity() >= 12);
    /// ```
    pub fn with_capacity_size(width: usize, height: usize) -> Self {
        Self {
            data: Vec::with_capacity(width * height),
            row_size: None,
        }
    }

    /// Constructs a new [`BidiVec<T>`] with the specified size,
    /// cloning the specified item in every position.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds [`isize::MAX`] bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiVec;
    ///
    /// let mut bvec = BidiVec::with_elem(5, 3, 3);
    ///
    /// assert_eq!(bvec.len(), 9);
    /// assert_eq!(bvec.capacity(), 9);
    /// assert_eq!(bvec.width(), 3);
    /// assert_eq!(bvec.height(), 3);
    /// assert_eq!(bvec[(1, 2)], 5);
    /// ```
    pub fn with_elem(value: T, width: usize, height: usize) -> Self
    where
        T: Clone,
    {
        Self {
            data: vec![value; width * height],
            row_size: if width == 0 { None } else { Some(width) },
        }
    }

    /// Constructs a new [`BidiVec<T>`] with the specified size,
    /// using the default value in every position.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds [`isize::MAX`] bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiVec;
    ///
    /// let mut bvec = BidiVec::<i32>::with_size_default(3, 3);
    ///
    /// assert_eq!(bvec.len(), 9);
    /// assert_eq!(bvec.capacity(), 9);
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

    /// Constructs a new [`BidiVec<T>`] with the specified size,
    /// using the specified closure to produce values.
    /// The order the closure is called when producing a new value is
    /// not guaranteed. If the item produced is depending on the its
    /// coordinates, use the slower `BidiVec<T>::with_size_func_xy`.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds [`isize::MAX`] bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiVec;
    ///
    /// let mut bvec = BidiVec::with_size_func(3, 3, ||137);
    ///
    /// assert_eq!(bvec.len(), 9);
    /// assert_eq!(bvec.capacity(), 9);
    /// assert_eq!(bvec.width(), 3);
    /// assert_eq!(bvec.height(), 3);
    /// assert_eq!(bvec[(1, 2)], 137);
    /// ```
    pub fn with_size_func<F>(width: usize, height: usize, f: F) -> Self
    where
        F: FnMut() -> T,
    {
        let mut this = Self {
            data: Vec::with_capacity(width * height),
            row_size: None,
        };
        this.resize_with(width, height, f);
        this
    }

    /// Constructs a new [`BidiVec<T>`] with the specified size,
    /// using the specified closure to produce values.
    /// The order the closure is called when producing a new value is
    /// not guaranteed, but the closure will receive the item coordinates
    /// as an input. If the coordinates are not needed, `BidiVec::with_size_func`
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
    /// use bidivec::BidiVec;
    ///
    /// let mut bvec = BidiVec::with_size_func_xy(3, 3, |x,y| x+y);
    ///
    /// assert_eq!(bvec.len(), 9);
    /// assert_eq!(bvec.capacity(), 9);
    /// assert_eq!(bvec.width(), 3);
    /// assert_eq!(bvec.height(), 3);
    /// assert_eq!(bvec[(1, 2)], 3);
    /// ```
    pub fn with_size_func_xy<F>(width: usize, height: usize, f: F) -> Self
    where
        F: FnMut(usize, usize) -> T,
    {
        let mut this = Self {
            data: Vec::with_capacity(width * height),
            row_size: None,
        };
        this.resize_with_xy(width, height, f);
        this
    }

    /// Creates a [`BidiVec<T>`] directly from the raw components of another vector.
    ///
    /// # Safety
    ///
    /// This is essentially the same to `Vec<T>::from_raw_parts`, so the same
    /// caveats apply.
    ///
    /// As this is highly unsafe, please check the documentation of `Vec<T>::from_raw_parts`
    /// before using this function.
    ///
    /// [`Vec<T>::from_raw_parts`]: crate::vec::Vec::from_raw_parts
    pub unsafe fn from_raw_parts(
        ptr: *mut T,
        length: usize,
        row_size: usize,
        capacity: usize,
    ) -> Result<Self, BidiError> {
        if length == 0 && row_size == 0 {
            Ok(Self {
                data: Vec::from_raw_parts(ptr, 0, capacity),
                row_size: None,
            })
        } else if row_size != 0 && (length % row_size) == 0 {
            Ok(Self {
                data: Vec::from_raw_parts(ptr, length, capacity),
                row_size: Some(row_size),
            })
        } else {
            Err(BidiError::IncompatibleSize)
        }
    }

    /// Creates a bidivec from a draining iterator, using the specified
    /// `row_size`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiVec, bidivec};
    ///
    /// let mut vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
    /// let mut bvec = BidiVec::from_iterator(vec.drain(..), 3).unwrap();
    ///
    /// let expected = bidivec!{
    ///     [0, 1, 2],
    ///     [3, 4, 5],
    ///     [6, 7, 8],
    /// };
    ///
    /// assert_eq!(bvec, expected);
    /// ```
    pub fn from_iterator(
        iter: impl Iterator<Item = T>,
        row_size: usize,
    ) -> Result<Self, BidiError> {
        let vec = iter.collect::<Vec<T>>();
        Self::from_vec(vec, row_size)
    }

    /// Creates a bidivec from another view iterator, using the specified
    /// mapping function to create and/or transform elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiVec, bidivec};
    ///
    /// let from = bidivec!{
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9],
    /// };
    ///
    /// let to = BidiVec::from_view_map(&from, |n| n - 1);
    ///
    /// assert_eq!(to, bidivec!{
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

    /// Creates a [`BidiVec<T>`] from a `Vec<T>` and a specified row size.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiVec;
    ///
    /// let vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
    /// let mut bvec = BidiVec::from_vec(vec, 3).unwrap();
    ///
    /// assert_eq!(bvec.len(), 9);
    /// assert_eq!(bvec.width(), 3);
    /// assert_eq!(bvec.height(), 3);
    /// assert_eq!(bvec[(1, 2)], 7);
    /// ```
    pub fn from_vec(vec: Vec<T>, row_size: usize) -> Result<Self, BidiError> {
        if vec.is_empty() && row_size == 0 {
            Ok(Self {
                data: vec,
                row_size: None,
            })
        } else if row_size != 0 && (vec.len() % row_size) == 0 {
            Ok(Self {
                data: vec,
                row_size: Some(row_size),
            })
        } else {
            Err(BidiError::IncompatibleSize)
        }
    }

    /// Clears the bidivec, removing all values.
    ///
    /// Note that this method has no effect on the allocated capacity
    /// of the bidivec.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiVec, bidivec};
    ///
    /// let mut bvec = bidivec![5; 3, 3];
    ///
    /// bvec.clear();
    ///
    /// assert!(bvec.is_empty());
    /// ```
    pub fn clear(&mut self) {
        check_consistent!(&self);
        self.data.clear();
        self.row_size = None;
    }

    /// Returns the number of items contained in the bidivec.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiVec, bidivec};
    ///
    /// let mut bvec = bidivec![5; 4, 3];
    ///
    /// assert_eq!(bvec.len(), 12);
    /// ```
    pub fn len(&self) -> usize {
        check_consistent!(&self);
        self.data.len()
    }

    /// Returns the width (that is, the size of a row) in the bidivec.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiVec, bidivec};
    ///
    /// let mut bvec = bidivec![5; 4, 3];
    ///
    /// assert_eq!(bvec.width(), 4);
    /// ```
    pub fn width(&self) -> usize {
        check_consistent!(&self);
        self.row_size.unwrap_or(0)
    }

    /// Returns the height (that is, the size of a column) in the bidivec.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiVec, bidivec};
    ///
    /// let mut bvec = bidivec![5; 4, 3];
    ///
    /// assert_eq!(bvec.height(), 3);
    /// ```
    pub fn height(&self) -> usize {
        check_consistent!(&self);
        match self.row_size {
            Some(w) => self.data.len() / w,
            None => 0,
        }
    }

    /// Returns true if the bidivec contains no elements (that
    /// implies that its width, height and len are all zero).
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiVec;
    ///
    /// let mut bvec = BidiVec::<i32>::new();
    ///
    /// assert!(bvec.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        check_consistent!(&self);
        self.data.is_empty()
    }

    /// Resizes the[`BidiVec`] in-place so that it has new width and
    /// height.
    ///
    /// Any new item that has to be created is created by cloning the
    /// supplied value.
    /// If the new size is smaller than before in a dimension, the
    ///[`BidiVec`] is truncated.
    ///
    /// This method requires `T` to implement [`Clone`],
    /// in order to be able to clone the passed value.
    /// If you need more flexibility (or want to rely on [`Default`] instead of
    /// [`Clone`]), use [`BidiVec::resize_with`].
    /// If you only need to resize to a smaller size, use [`BidiVec::truncate`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiVec;
    ///
    /// let mut bvec = BidiVec::new();
    /// bvec.resize(3, 3, 5);
    ///
    /// assert_eq!(bvec.len(), 9);
    /// assert_eq!(bvec.capacity(), 9);
    /// assert_eq!(bvec.width(), 3);
    /// assert_eq!(bvec.height(), 3);
    /// assert_eq!(bvec[(1, 2)], 5);
    /// ```
    pub fn resize(&mut self, new_width: usize, new_height: usize, value: T)
    where
        T: Clone,
    {
        if new_width == 0 || new_height == 0 {
            self.clear();
            return;
        }

        if self.row_size.is_some() {
            self.truncate(min(self.width(), new_width), min(self.height(), new_height))
                .unwrap();

            while self.width() < new_width {
                self.push_col(std::iter::repeat(value.clone()).take(self.height()))
                    .unwrap();
            }
        }

        self.data.resize(new_height * new_width, value);
        self.row_size = Some(new_width);
        check_consistent!(&self);
    }

    /// Resizes the[`BidiVec`] in-place so that it has new width and
    /// height, using the specified closure to generate new values.
    /// The order the clousre is called when producing a new value is
    /// not guaranteed. If the item produced is depending on the its
    /// coordinates, use the slower `BidiVec<T>::resize_with_xy`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiVec;
    ///
    /// let mut bvec = BidiVec::new();
    /// bvec.resize_with(3, 3, ||5);
    ///
    /// assert_eq!(bvec.len(), 9);
    /// assert_eq!(bvec.capacity(), 9);
    /// assert_eq!(bvec.width(), 3);
    /// assert_eq!(bvec.height(), 3);
    /// assert_eq!(bvec[(1, 2)], 5);
    /// ```
    pub fn resize_with<F>(&mut self, new_width: usize, new_height: usize, mut f: F)
    where
        F: FnMut() -> T,
    {
        if new_width == 0 || new_height == 0 {
            self.clear();
            return;
        }

        if self.row_size.is_some() {
            self.truncate(min(self.width(), new_width), min(self.height(), new_height))
                .unwrap();

            while self.width() < new_width {
                // avoid https://github.com/rust-lang/rust-clippy/issues/8098
                #[allow(clippy::redundant_closure)]
                self.push_col(std::iter::repeat_with(|| f()).take(self.height()))
                    .unwrap();
            }
        }

        self.data.resize_with(new_height * new_width, f);
        self.row_size = Some(new_width);
        check_consistent!(&self);
    }

    /// Resizes the[`BidiVec`] (mostly) in-place so that it has new width and
    /// height, using the specified closure to generate new values.
    /// The order the closure is called when producing a new value is
    /// not guaranteed, but the closure will receive the item coordinates
    /// as an input. If the coordinates are not needed, `BidiVec::resize_with`
    /// is faster and uses less temporary memory (this method uses up
    /// to a row or column size of temporary memory).
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiVec;
    ///
    /// let mut bvec = BidiVec::new();
    /// bvec.resize_with(3, 3, ||5);
    ///
    /// assert_eq!(bvec.len(), 9);
    /// assert_eq!(bvec.capacity(), 9);
    /// assert_eq!(bvec.width(), 3);
    /// assert_eq!(bvec.height(), 3);
    /// assert_eq!(bvec[(1, 2)], 5);
    /// ```
    pub fn resize_with_xy<F>(&mut self, new_width: usize, new_height: usize, mut f: F)
    where
        F: FnMut(usize, usize) -> T,
    {
        if new_width == 0 || new_height == 0 {
            self.clear();
            return;
        }

        if self.row_size.is_some() {
            self.truncate(min(self.width(), new_width), min(self.height(), new_height))
                .unwrap();

            while self.width() < new_width {
                let mut tmp = Vec::with_capacity(self.height());

                for y in 0..self.height() {
                    tmp.push(f(self.width(), y));
                }

                self.push_col(tmp).unwrap();
            }

            self.row_size = Some(new_width);
        }

        while self.height() < new_height {
            let y = self.height();
            for x in 0..new_width {
                self.data.push(f(x, y));
            }
            // if we just went from empty to filled, refresh row_size
            self.row_size = Some(new_width);
        }
        check_consistent!(&self);
    }

    /// Truncates the[`BidiVec`] so that it has new width and
    /// height that must be strictly lower or equal than the current.
    /// width and height, otherwise a [`BidiError::OutOfBounds`] error
    /// is produced.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiVec, bidivec};
    ///
    /// let mut bvec = bidivec![5; 150, 18];
    /// bvec.truncate(3, 4).unwrap();
    ///
    /// assert_eq!(bvec.len(), 12);
    /// assert_eq!(bvec.width(), 3);
    /// assert_eq!(bvec.height(), 4);
    /// assert_eq!(bvec[(1, 2)], 5);
    /// ```
    pub fn truncate(&mut self, new_width: usize, new_height: usize) -> Result<(), BidiError> {
        if new_width > self.width() || new_height > self.height() {
            return Err(BidiError::OutOfBounds);
        }

        if new_width == self.width() && new_height == self.height() {
            return Ok(());
        }

        if new_width == 0 || new_height == 0 {
            self.clear();
        } else {
            while self.width() > new_width {
                self.delete_last_col();
            }
            self.data.truncate(new_height * self.width());
        }

        check_consistent!(&self);
        Ok(())
    }

    /// Returns the number of elements the bidivec can hold without
    /// reallocating.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiVec;
    ///
    /// let bvec: BidiVec<i32> = BidiVec::with_capacity(10);
    /// assert_eq!(bvec.capacity(), 10);
    /// ```
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    /// Reserves capacity for at least `additional` more elements to be inserted
    /// in the given [`BidiVec<T>`]. The collection may reserve more space to avoid
    /// frequent reallocations. After calling [`BidiVec::reserve`], capacity will be
    /// greater than or equal to `self.len() + additional`. Does nothing if
    /// capacity is already sufficient.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds [`isize::MAX`] bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiVec, bidivec};
    ///
    /// let mut bvec = bidivec![111i32; 2, 2];
    /// bvec.reserve(10);
    /// assert!(bvec.capacity() >= 14);
    /// ```
    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional)
    }

    /// Reserves capacity for exactly `additional` more elements to be inserted
    /// in the given [`BidiVec<T>`]. The collection may reserve more space to avoid
    /// frequent reallocations. After calling [`BidiVec::reserve`], capacity will be
    /// greater than or equal to `self.len() + additional`. Does nothing if
    /// capacity is already sufficient.
    ///
    /// Note that the allocator may give the collection more space than it
    /// requests. Therefore, capacity can not be relied upon to be precisely
    /// minimal. Prefer [`BidiVec::reserve`] if future insertions are expected.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds [`isize::MAX`] bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiVec, bidivec};
    ///
    /// let mut bvec = bidivec![111i32; 2, 2];
    /// bvec.reserve_exact(10);
    /// assert!(bvec.capacity() >= 14);
    /// ```
    pub fn reserve_exact(&mut self, additional: usize) {
        self.data.reserve_exact(additional)
    }

    /// Tries to reserve capacity for at least `additional` more elements to be inserted
    /// in the given [`BidiVec<T>`]. The collection may reserve more space to avoid
    /// frequent reallocations. After calling `try_reserve`, capacity will be
    /// greater than or equal to `self.len() + additional`. Does nothing if
    /// capacity is already sufficient.
    ///
    /// Requires rustc 1.57 or later.
    #[rustversion::since(1.57)]
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.data.try_reserve(additional)
    }

    /// Tries to reserve the minimum capacity for exactly `additional`
    /// elements to be inserted in the given `Vec<T>`. After calling
    /// `try_reserve_exact`, capacity will be greater than or equal to
    /// `self.len() + additional` if it returns `Ok(())`.
    /// Does nothing if the capacity is already sufficient.
    ///
    /// Requires rustc 1.57 or later.
    #[rustversion::since(1.57)]
    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.data.try_reserve_exact(additional)
    }

    /// Shrinks the capacity of the bidivec as much as possible.
    ///
    /// See `Vec::shrink_to_fit` for more details
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiVec;
    ///
    /// let mut bvec = BidiVec::<i32>::with_capacity(10);
    /// bvec.push_row([1, 2, 3]).unwrap();
    /// assert_eq!(bvec.capacity(), 10);
    /// bvec.shrink_to_fit();
    /// assert!(bvec.capacity() >= 3);
    /// ```
    pub fn shrink_to_fit(&mut self) {
        self.data.shrink_to_fit()
    }

    /// Shrinks the capacity of the bidivec with a lower bound.
    ///
    /// The capacity will remain at least as large as both the length
    /// and the supplied value.
    ///
    /// If the current capacity is less than the lower limit, this is a no-op.
    ///
    /// Requires rustc 1.56 or later.
    #[rustversion::since(1.56)]
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.data.shrink_to(min_capacity)
    }

    /// Extracts a slice containing the specified range of bidivec contents,
    /// laid out linearly, by rows.
    pub fn as_slice<R: SliceIndex<[T]>>(&self, range: R) -> &R::Output {
        &self.data[range]
    }

    /// Extracts a slice containing the specified range of bidivec contents,
    /// laid out linearly, by rows.
    pub fn as_mut_slice<R: SliceIndex<[T]>>(&mut self, range: R) -> &mut R::Output {
        &mut self.data[range]
    }

    /// Returns a raw pointer to the bidivec's buffer.
    ///
    /// The caller must ensure that the bidivec outlives the pointer this
    /// function returns, or else it will end up pointing to garbage.
    /// Modifying the bidivec may cause its buffer to be reallocated,
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
    /// See [`Vec::as_ptr`] for warnings on using these pointers. The end pointer
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
    /// See [`Vec::as_mut_ptr`] and [`Vec::as_ptr`] for warnings on using these pointers.
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
        self.data.into_boxed_slice()
    }

    /// Converts the vector into a `Vec<T>` where items are linearly
    /// laid out by rows.
    pub fn into_vec(self) -> Vec<T> {
        self.data
    }

    /// Swaps two elements in the bidivec. If any of the coordinates are out
    /// of range, [`BidiError::OutOfBounds`] is returned
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiVec, bidivec};
    ///
    /// let mut bvec = bidivec!{
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9],
    /// };
    ///
    /// assert_eq!(bvec[(2, 0)], 3);
    /// assert_eq!(bvec[(1, 2)], 8);
    ///
    /// bvec.swap((2, 0), (1, 2)).unwrap();
    ///
    /// assert_eq!(bvec[(2, 0)], 8);
    /// assert_eq!(bvec[(1, 2)], 3);
    /// ```
    #[inline(always)]
    pub fn swap(&mut self, a: (usize, usize), b: (usize, usize)) -> Result<(), BidiError> {
        let idx_a = self.calc_index(a.0, a.1)?;
        let idx_b = self.calc_index(b.0, b.1)?;

        self.data.swap(idx_a, idx_b);
        Ok(())
    }

    /// Appends a new column to the bidivec.
    /// If the bidivec is not empty, the column to be appended must contain
    /// exactly `height()` elements, or [`BidiError::IncompatibleSize`] is
    /// returned.
    /// If the bidivec is not empty, this operation is also expensive
    /// as it requires O(column_size * bidivec_size) time; use[`BidiGrowVec`] for
    /// faster column pushes (at the loss of linear layout).
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiVec;
    ///
    /// let mut bvec = BidiVec::new();
    /// bvec.push_col([1, 2, 3]).unwrap();
    ///
    /// assert_eq!(bvec[(0, 0)], 1);
    /// assert_eq!(bvec[(0, 1)], 2);
    /// assert_eq!(bvec[(0, 2)], 3);
    /// ```
    pub fn push_col<I: IntoIterator<Item = T>>(&mut self, iter: I) -> Result<(), BidiError> {
        match self.row_size {
            None => {
                self.data.extend(iter);
                self.row_size = Some(1);
                check_consistent!(self);
                Ok(())
            }
            Some(row_size) => {
                let mut rows_changed: usize = 0;
                let mut force_rollback: bool = false;

                for (row, val) in iter.into_iter().enumerate() {
                    let insertion_point = (row + 1) * row_size + row;

                    match insertion_point.cmp(&self.data.len()) {
                        Ordering::Less => self.data.insert(insertion_point, val),
                        Ordering::Equal => self.data.push(val),
                        Ordering::Greater => {
                            force_rollback = true;
                            break;
                        }
                    }

                    rows_changed += 1;
                }

                if force_rollback
                    || ((self.data.len() % (row_size + 1)) != 0)
                    || (rows_changed == 0)
                {
                    for row in (0..rows_changed).rev() {
                        self.data.remove((row + 1) * row_size + row);
                    }
                    check_consistent!(self);
                    Err(BidiError::IncompatibleSize)
                } else {
                    self.row_size = Some(row_size + 1);
                    check_consistent!(self);
                    Ok(())
                }
            }
        }
    }

    /// Appends a new row to the bidivec.
    /// If the bidivec is not empty, the row to be appended must contain
    /// exactly `width()` elements, or [`BidiError::IncompatibleSize`] is
    /// returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiVec;
    ///
    /// let mut bvec = BidiVec::new();
    /// bvec.push_row([1, 2, 3]).unwrap();
    ///
    /// assert_eq!(bvec[(0, 0)], 1);
    /// assert_eq!(bvec[(1, 0)], 2);
    /// assert_eq!(bvec[(2, 0)], 3);
    /// ```
    pub fn push_row<I: IntoIterator<Item = T>>(&mut self, iter: I) -> Result<(), BidiError> {
        match self.row_size {
            None => {
                self.data.extend(iter);
                self.row_size = Some(self.data.len());
                check_consistent!(self);
                Ok(())
            }
            Some(row_size) => {
                let rollback_len = self.data.len();

                self.data.extend(iter);

                if self.data.len() != (row_size + rollback_len) {
                    self.data.truncate(rollback_len);
                    check_consistent!(self);
                    Err(BidiError::IncompatibleSize)
                } else {
                    check_consistent!(self);
                    Ok(())
                }
            }
        }
    }

    /// Inserts a new column in the middle of a bidivec.
    /// If the bidivec is not empty, the column to be inserted must contain
    /// exactly `height()` elements, or [`BidiError::IncompatibleSize`] is
    /// returned.
    ///
    /// If the bidivec is not empty, this operation is also expensive
    /// as it requires O(column_size * bidivec_size) time; use[`BidiGrowVec`] for
    /// faster column pushes (at the loss of linear layout).
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiVec;
    ///
    /// let mut bvec = BidiVec::new();
    /// bvec.push_row([1, 1, 1]).unwrap();
    /// bvec.push_row([1, 1, 1]).unwrap();
    /// bvec.push_row([1, 1, 1]).unwrap();
    /// bvec.insert_col(1, [5, 5, 5]).unwrap();
    ///
    /// assert_eq!(bvec[(0, 0)], 1);
    /// assert_eq!(bvec[(1, 1)], 5);
    /// assert_eq!(bvec[(2, 2)], 1);
    /// ```
    pub fn insert_col<I: IntoIterator<Item = T>>(
        &mut self,
        col: usize,
        iter: I,
    ) -> Result<(), BidiError> {
        match self.row_size {
            None if col == 0 => self.push_col(iter),
            None => Err(BidiError::OutOfBounds),
            Some(row_size) => match col.cmp(&row_size) {
                Ordering::Greater => Err(BidiError::OutOfBounds),
                Ordering::Equal => self.push_col(iter),
                Ordering::Less => {
                    let expected_len = self.data.len() + self.height();
                    let expected_count = self.height();
                    let mut insertion_count = 0;
                    let mut overflowed = false;

                    for v in iter.into_iter() {
                        let new_index = col + insertion_count * (row_size + 1);
                        insertion_count += 1;

                        if new_index >= self.data.len() {
                            overflowed = true;
                            break;
                        }

                        self.data.insert(new_index, v);

                        if insertion_count > expected_count {
                            break;
                        }
                    }

                    if overflowed || self.data.len() != expected_len {
                        for i in 0..insertion_count {
                            let del_index = col + i * row_size;

                            if del_index >= self.data.len() {
                                break;
                            }

                            self.data.remove(del_index);
                        }
                        //check_consistent!(self);
                        Err(BidiError::IncompatibleSize)
                    } else {
                        self.row_size = Some(row_size + 1);
                        check_consistent!(self);
                        Ok(())
                    }
                }
            },
        }
    }

    /// Inserts a new row in the middle of a bidivec.
    /// If the bidivec is not empty, the row to be inserted must contain
    /// exactly `width()` elements, or [`BidiError::IncompatibleSize`] is
    /// returned.
    ///
    /// This operation is O(bidivec_size).
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiVec;
    ///
    /// let mut bvec = BidiVec::new();
    /// bvec.push_row([1, 1, 1]).unwrap();
    /// bvec.push_row([1, 1, 1]).unwrap();
    /// bvec.push_row([1, 1, 1]).unwrap();
    /// bvec.insert_row(1, [5, 5, 5]).unwrap();
    ///
    /// assert_eq!(bvec.width(), 3);
    /// assert_eq!(bvec.height(), 4);
    /// assert_eq!(bvec[(1, 1)], 5);
    /// ```
    pub fn insert_row<I: IntoIterator<Item = T>>(
        &mut self,
        row: usize,
        iter: I,
    ) -> Result<(), BidiError> {
        match self.row_size {
            None if row == 0 => self.push_row(iter),
            None => Err(BidiError::OutOfBounds),
            Some(row_size) => match (row * row_size).cmp(&self.data.len()) {
                Ordering::Greater => Err(BidiError::OutOfBounds),
                Ordering::Equal => self.push_row(iter),
                Ordering::Less => {
                    let expected_len = self.data.len() + row_size;
                    let insertion_base = row * row_size;
                    let mut insertion_count = 0;

                    for v in iter.into_iter() {
                        self.data.insert(insertion_base + insertion_count, v);
                        insertion_count += 1;

                        if insertion_count > row_size {
                            break;
                        }
                    }

                    if self.data.len() != expected_len {
                        for _ in 0..insertion_count {
                            self.data.remove(insertion_base);
                        }
                        check_consistent!(self);
                        Err(BidiError::IncompatibleSize)
                    } else {
                        check_consistent!(self);
                        Ok(())
                    }
                }
            },
        }
    }

    /// Removes the specified column from the bidivec. If the column is
    /// outside of range, [`BidiError::OutOfBounds`] is returned.
    ///
    /// If the deleted data is not needed, `BidiVec::delete_col` provides
    /// better performances.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiVec, bidivec};
    ///
    /// let mut bvec = bidivec!{
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9],
    /// };
    ///
    /// assert_eq!(bvec.remove_col(1).unwrap(), vec![2, 5, 8]);
    ///
    /// assert_eq!(bvec.width(), 2);
    /// assert_eq!(bvec.height(), 3);
    /// assert_eq!(bvec[(1, 1)], 6);
    /// ```
    pub fn remove_col(&mut self, col: usize) -> Result<Vec<T>, BidiError> {
        if col >= self.width() {
            return Err(BidiError::OutOfBounds);
        }

        let row_size = self.row_size.unwrap();
        let mut result = Vec::with_capacity(self.height());

        for i in (0..self.height()).rev() {
            result.push(self.data.remove(i * row_size + col));
        }

        if self.data.is_empty() {
            self.row_size = None;
        } else {
            self.row_size = Some(row_size - 1);
        }

        result.reverse();

        Ok(result)
    }

    /// Removes the specified row from the bidivec. If the row is
    /// outside of range, [`BidiError::OutOfBounds`] is returned.
    ///
    /// If the deleted data is not needed, `BidiVec::delete_row` provides
    /// better performances.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiVec, bidivec};
    ///
    /// let mut bvec = bidivec!{
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9],
    /// };
    ///
    /// assert_eq!(bvec.remove_row(1).unwrap(), vec![4, 5, 6]);
    ///
    /// assert_eq!(bvec.width(), 3);
    /// assert_eq!(bvec.height(), 2);
    /// assert_eq!(bvec[(1, 1)], 8);
    /// ```
    pub fn remove_row(&mut self, row: usize) -> Result<Vec<T>, BidiError> {
        if row >= self.height() {
            return Err(BidiError::OutOfBounds);
        }

        let row_size = self.row_size.unwrap();
        let mut result = Vec::with_capacity(self.width());

        for _ in 0..self.width() {
            result.push(self.data.remove(row * row_size));
        }

        if self.data.is_empty() {
            self.row_size = None;
        }

        Ok(result)
    }

    /// Deletes the specified column from the bidivec. If the column is
    /// outside of range, [`BidiError::OutOfBounds`] is returned.
    ///
    /// If you need to access the deleted data is not needed,
    /// `BidiVec::remove_col` provides that data, at a performance cost.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiVec, bidivec};
    ///
    /// let mut bvec = bidivec!{
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9],
    /// };
    ///
    /// bvec.delete_col(1).unwrap();
    ///
    /// assert_eq!(bvec.width(), 2);
    /// assert_eq!(bvec.height(), 3);
    /// assert_eq!(bvec[(1, 1)], 6);
    /// ```
    pub fn delete_col(&mut self, col: usize) -> Result<(), BidiError> {
        if col >= self.width() {
            return Err(BidiError::OutOfBounds);
        }

        if let Some(row_size) = self.row_size {
            for i in (0..self.height()).rev() {
                self.data.remove(i * row_size + col);
            }

            if self.data.is_empty() {
                self.row_size = None;
            } else {
                self.row_size = Some(row_size - 1);
            }
        }

        Ok(())
    }

    /// Deletes the specified row from the bidivec. If the row is
    /// outside of range, [`BidiError::OutOfBounds`] is returned.
    ///
    /// If you need to access the deleted data is not needed,
    /// `BidiVec::remove_row` provides that data, at a performance cost.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiVec, bidivec};
    ///
    /// let mut bvec = bidivec!{
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9],
    /// };
    ///
    /// bvec.delete_row(1).unwrap();
    ///
    /// assert_eq!(bvec.width(), 3);
    /// assert_eq!(bvec.height(), 2);
    /// assert_eq!(bvec[(1, 1)], 8);
    /// ```
    pub fn delete_row(&mut self, row: usize) -> Result<(), BidiError> {
        if row >= self.height() {
            return Err(BidiError::OutOfBounds);
        }

        if let Some(row_size) = self.row_size {
            for _ in 0..self.width() {
                self.data.remove(row * row_size);
            }

            if self.data.is_empty() {
                self.row_size = None;
            }
        }

        Ok(())
    }

    /// Deletes the last column from the bidivec.
    ///
    /// If you need to access the deleted data is not needed,
    /// `BidiVec::pop_col` provides that data, at a performance cost.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiVec, bidivec};
    ///
    /// let mut bvec = bidivec!{
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9],
    /// };
    ///
    /// bvec.delete_last_col();
    ///
    /// assert_eq!(bvec.width(), 2);
    /// assert_eq!(bvec.height(), 3);
    /// ```
    pub fn delete_last_col(&mut self) {
        if let Some(row_size) = self.row_size {
            for i in (0..self.height()).rev() {
                self.data.remove((i + 1) * row_size - 1);
            }

            if self.data.is_empty() {
                self.row_size = None;
            } else {
                self.row_size = Some(row_size - 1);
            }

            check_consistent!(self);
        }
    }

    /// Deletes the last row from the bidivec.
    ///
    /// If you need to access the deleted data is not needed,
    /// `BidiVec::pop_row` provides that data, at a performance cost.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiVec, bidivec};
    ///
    /// let mut bvec = bidivec!{
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9],
    /// };
    ///
    /// bvec.delete_last_row();
    ///
    /// assert_eq!(bvec.width(), 3);
    /// assert_eq!(bvec.height(), 2);
    /// ```
    pub fn delete_last_row(&mut self) {
        if let Some(row_size) = self.row_size {
            self.data.truncate(self.data.len().saturating_sub(row_size));

            if self.data.is_empty() {
                self.row_size = None;
            }

            check_consistent!(self);
        }
    }

    /// Removes the last column from the bidivec, returning its data.
    ///
    /// If the removed data is not needed, `BidiVec::delete_last_col`
    /// provides better performances.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiVec, bidivec};
    ///
    /// let mut bvec = bidivec!{
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9],
    /// };
    ///
    /// assert_eq!(bvec.pop_col().unwrap(), vec![3, 6, 9]);
    ///
    /// assert_eq!(bvec.width(), 2);
    /// assert_eq!(bvec.height(), 3);
    /// ```
    #[must_use]
    pub fn pop_col(&mut self) -> Option<Vec<T>> {
        if let Some(row_size) = self.row_size {
            let mut result = Vec::with_capacity(self.height());

            for i in (0..self.height()).rev() {
                result.push(self.data.remove((i + 1) * row_size - 1));
            }

            if self.data.is_empty() {
                self.row_size = None;
            } else {
                self.row_size = Some(row_size - 1);
            }

            result.reverse();

            check_consistent!(self);
            Some(result)
        } else {
            None
        }
    }

    /// Removes the last row from the bidivec, returning its data.
    ///
    /// If the removed data is not needed, `BidiVec::delete_last_row`
    /// provides better performances.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiVec, bidivec};
    ///
    /// let mut bvec = bidivec!{
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9],
    /// };
    ///
    /// assert_eq!(bvec.pop_row().unwrap(), vec![7, 8, 9]);
    ///
    /// assert_eq!(bvec.width(), 3);
    /// assert_eq!(bvec.height(), 2);
    /// ```
    #[must_use]
    pub fn pop_row(&mut self) -> Option<Vec<T>> {
        if let Some(row_size) = self.row_size {
            let result = self
                .data
                .split_off(self.data.len().saturating_sub(row_size));

            if self.data.is_empty() {
                self.row_size = None;
            }

            check_consistent!(self);
            Some(result)
        } else {
            None
        }
    }

    /// Accesses an element in the BidiVec, using its cartesian coordinates.
    /// If coordinates are outside of range, [`None`] is returned.
    ///
    /// If the error is not going to be handled, direct indexing is an easier
    /// way to achieve the same results.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiVec, bidivec};
    ///
    /// let mut bvec = bidivec!{
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

    /// Mutably accesses an element in the BidiVec, using its cartesian coordinates.
    /// If coordinates are outside of range, [`None`] is returned.
    ///
    /// If the error is not going to be handled, direct indexing is an easier
    /// way to achieve the same results.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiVec, bidivec};
    ///
    /// let mut bvec = bidivec!{
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

    /// Checks if the specified coordinates are inside the bidivec bounds
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiVec, bidivec};
    ///
    /// let mut bvec = bidivec!{
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
        check_consistent!(&self);

        match self.row_size {
            Some(w) => {
                let idx = y * w + x;
                if x >= w || idx >= self.data.len() {
                    Err(BidiError::OutOfBounds)
                } else {
                    Ok(idx)
                }
            }
            None => Err(BidiError::OutOfBounds),
        }
    }

    /// Reverses the order of the items in the specified row.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiVec, bidivec};
    ///
    /// let mut bvec = bidivec!{
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
    /// use bidivec::{BidiVec, bidivec};
    ///
    /// let mut bvec = bidivec!{
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

    /// Transposes the bidivec, that is an operation that flips the bidivec
    /// over its diagonal (or, more simply, switches the meaning of columns and
    /// rows). As such, the result of a transposition is as wide as the original
    /// was tall, and as tall as the original was wide.
    ///
    /// While this is performed in-place, it still requires O(n) additional memory
    /// if the bidivec width and height are different (i.e. it's not a square).
    pub fn transpose(&mut self) {
        if let Some(row_size) = self.row_size {
            let mut slice = BidiMutSlice::new(&mut self.data, row_size).unwrap();
            slice.transpose();
            self.row_size = Some(self.height());
        }
    }

    /// Rotates the bidivec 90°, counter-clockwise (or, 270° clockwise).
    /// The result of such a rotation is as wide as the original
    /// was tall, and as tall as the original was wide.
    ///
    /// While this is performed in-place, it still requires O(n) additional memory
    /// if the bidivec width and height are different (i.e. it's not a square).
    pub fn rotate90ccw(&mut self) {
        self.transpose();
        self.reverse_columns();
    }

    /// Rotates the bidivec 180°.
    pub fn rotate180(&mut self) {
        self.data.reverse();
    }

    /// Rotates the bidivec 270°, counter-clockwise (or, 90° clockwise).
    /// The result of such a rotation is as wide as the original
    /// was tall, and as tall as the original was wide.
    ///
    /// While this is performed in-place, it still requires O(n) additional memory
    /// if the bidivec width and height are different (i.e. it's not a square).
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

    /// Crops the data structure to its new bounds by moving the origin to
    /// a new location, reducing the width and height and dropping excess
    /// data.
    pub fn crop(&mut self, rect: &BidiRect) -> Result<(), BidiError> {
        if rect.width == 0 || rect.height == 0 {
            self.clear();
            Ok(())
        } else if (rect.max_x() > self.width()) || (rect.max_y() > self.height()) {
            Err(BidiError::OutOfBounds)
        } else {
            self.truncate(rect.max_x(), rect.max_y()).unwrap();
            for _ in 0..rect.x {
                self.remove_col(0).unwrap();
            }
            for _ in 0..rect.y {
                self.remove_row(0).unwrap();
            }
            Ok(())
        }
    }

    /// Converts this instance into a [`BidiGrowVec<T>`]
    /// This operation is `O(width*height)` in the worst case.
    pub fn into_bidigrowvec(self) -> BidiGrowVec<T> {
        BidiGrowVec::<T>::from(self)
    }

    /// Converts this instance into a [`BidiArray<T>`]
    /// This operation is `O(1)` in the worst case.
    pub fn into_bidiarray(self) -> BidiArray<T> {
        BidiArray::<T>::from(self)
    }

    /// Converts this instance to an immutable [`BidiView`].
    pub fn as_bidiview(&self) -> &dyn BidiView<Output = T> {
        self
    }

    /// Converts this instance to a mutable [`BidiView`].
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

impl<T> BidiFrom<&dyn BidiView<Output = T>> for BidiVec<T>
where
    T: Clone,
{
    fn from_view(source: &dyn BidiView<Output = T>) -> Result<Self, BidiError> {
        Ok(BidiVec::<T>::with_size_func_xy(
            source.width(),
            source.height(),
            |x, y| source[(x, y)].clone(),
        ))
    }

    fn from_view_cut(source: &dyn BidiView<Output = T>, cut: &BidiRect) -> Result<Self, BidiError> {
        if cut.max_x() > source.height() || cut.max_y() > source.width() {
            return Err(BidiError::OutOfBounds);
        }

        Ok(BidiVec::<T>::with_size_func_xy(
            cut.width,
            cut.height,
            |x, y| source[(x + cut.x, y + cut.y)].clone(),
        ))
    }
}

impl<T> BidiFrom<BidiVec<T>> for BidiVec<T> {
    fn from_view(source: BidiVec<T>) -> Result<Self, BidiError> {
        Ok(source)
    }

    fn from_view_cut(mut source: BidiVec<T>, cut: &BidiRect) -> Result<Self, BidiError> {
        source.crop(cut)?;
        Ok(source)
    }
}

impl<T> BidiFrom<BidiGrowVec<T>> for BidiVec<T> {
    fn from_view(source: BidiGrowVec<T>) -> Result<Self, BidiError> {
        Ok(Self::from(source))
    }

    fn from_view_cut(mut source: BidiGrowVec<T>, cut: &BidiRect) -> Result<Self, BidiError> {
        source.crop(cut)?;
        Ok(Self::from(source))
    }
}

impl<T> BidiFrom<BidiArray<T>> for BidiVec<T> {
    fn from_view(source: BidiArray<T>) -> Result<Self, BidiError> {
        Ok(Self::from(source))
    }

    fn from_view_cut(source: BidiArray<T>, cut: &BidiRect) -> Result<Self, BidiError> {
        let mut this = Self::from(source);
        this.crop(cut)?;
        Ok(this)
    }
}

impl<T> Index<(usize, usize)> for BidiVec<T> {
    type Output = T;

    /// Accesses an element in the BidiVec, using its cartesian coordinates.
    /// If coordinates are outside of range, it panics.
    ///
    /// If you want a more graceful error handling, see [`BidiVec::get`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiVec, bidivec};
    ///
    /// let mut bvec = bidivec!{
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
                "Indexes out of bidivec bounds: ({},{}) out of {}x{}",
                index.0,
                index.1,
                self.width(),
                self.height()
            )
        });
        unsafe { self.data.get_unchecked(idx) }
    }
}

impl<T> IndexMut<(usize, usize)> for BidiVec<T> {
    /// Mutably accesses an element in the BidiVec, using its cartesian coordinates.
    /// If coordinates are outside of range, it panics.
    ///
    /// If you want a more graceful error handling, see [`BidiVec::get_mut`] and
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiVec, bidivec};
    ///
    /// let mut bvec = bidivec!{
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
                "Indexes out of bidivec bounds: ({},{}) out of {}x{}",
                index.0,
                index.1,
                self.width(),
                self.height()
            )
        });
        unsafe { self.data.get_unchecked_mut(idx) }
    }
}

impl<T> BidiView for BidiVec<T> {
    fn width(&self) -> usize {
        BidiVec::<T>::width(self)
    }
    fn height(&self) -> usize {
        BidiVec::<T>::height(self)
    }

    fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.get(x, y)
    }
}

impl<T> BidiViewMut for BidiVec<T> {
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.get_mut(x, y)
    }
}

unsafe impl<T> BidiViewMutIterable for BidiVec<T> {}

impl<T> From<BidiGrowVec<T>> for BidiVec<T> {
    /// Creates a new instance of [`BidiVec<T>`] from an existing [`BidiVec<T>`].
    /// This operation is `O(width*height)` in the worst case.
    fn from(other: BidiGrowVec<T>) -> Self {
        let width = other.width();
        let vec = other.into_vec();
        Self::from_vec(vec, width).unwrap()
    }
}

impl<T> From<BidiArray<T>> for BidiVec<T> {
    /// Creates a new instance of [`BidiVec<T>`] from an existing [`BidiVec<T>`].
    /// This operation is `O(1)` in the worst case.
    fn from(other: BidiArray<T>) -> Self {
        Self {
            data: other.data.into_vec(),
            row_size: if other.row_size == 0 {
                None
            } else {
                Some(other.row_size)
            },
        }
    }
}
