use crate::bidiiter::{Iter, IterMut};
use std::cmp::{min, Ordering};
use std::default::Default;
use std::iter::Iterator;
use std::ops::{Index, IndexMut};

use crate::*;

/// A growable bidimensional array type with heap-allocated contents,
/// which trades off linear layout (and memory locality) for faster
/// performances when inserting and deleting items.
///
/// If you don't need to insert items (other than appending rows),
/// [`BidiGrowVec<T>`] offers better memory locality and a linear layout
/// which is friendly for unsafe code.
///
/// BidiGrowVecs have `O(1)` indexing, amortized `O(1)` push of rows (to the end) and
/// `O(row_length)` pops (from the end).
///
/// Most methods will not implicitly panic if out-of-bounds accesses are attempted,
/// but they will return a [`BidiError`] for graceful error handling.
///
/// # Examples
///
/// You can explicitly create a [`BidiGrowVec`] with [`BidiGrowVec::new`]:
///
/// ```
/// # use bidivec::BidiGrowVec;
/// let v: BidiGrowVec<i32> = BidiGrowVec::new();
/// ```
///
/// or by using the [`bidigrowvec!`] macro:
///
/// ```
/// # use bidivec::{BidiGrowVec, bidigrowvec};
/// // this creates an empty bidigrowvec
/// let v: BidiGrowVec<i32> = bidigrowvec![];
///
/// // this creates a 3x2 bidigrowvec, with all items equal to 1
/// let v = bidigrowvec![1; 3, 2];
///
/// // this creates a 3x2 bidigrowvec from items; the final '3' is the
/// // bidigrowvec's width
/// let v = bidigrowvec![1, 2, 3, 4, 5, 6; 3];
///
/// // this creates a 3x2 bidigrowvec, by listing the rows separately
/// let v = bidigrowvec!{
///     [1, 2, 3],
///     [4, 5, 6],
/// };
/// ```
///
/// You can push rows onto the end of a bidigrowvec (which will grow the bidigrowvec
/// as needed):
///
/// ```
/// # use bidivec::{BidiGrowVec, bidigrowvec};
/// let mut v = bidigrowvec![1, 2; 1];
///
/// v.push_row([3, 4]);
/// ```
///
/// Popping rows works in much the same way:
///
/// ```
/// # use bidivec::{BidiGrowVec, bidigrowvec};
/// let mut v = bidigrowvec![1, 2; 1];
///
/// let one_and_two = v.pop_row();
/// ```
///
/// BidiGrowVecs support indexing with cartesian coordinates (through the [`Index`] and [`IndexMut`] traits);
/// note that coordinates out of range will cause the code to panic.
/// The [`BidiGrowVec::get`] and [`BidiGrowVec::get_mut`] methods offer a safer way to access the bidigrowvec contents,
/// by returning an `Option`, in the same vein of `Vec<T>`.
///
/// ```
/// # use bidivec::{BidiGrowVec, bidigrowvec};
/// let mut v = bidigrowvec!{[1, 2, 3], [4, 5, 6]};
/// let four = v[(0, 1)];
/// v[(1, 1)] = v[(1, 0)] + v[(2, 0)];
/// ```
#[derive(Clone, Default, Debug, PartialEq)]
pub struct BidiGrowVec<T> {
    pub(crate) data: Vec<Vec<T>>,
}

impl<T> BidiGrowVec<T> {
    /// Constructs a new, empty [`BidiGrowVec<T>`].
    ///
    /// # Examples
    ///
    /// ```
    /// # #![allow(unused_mut)]
    /// use bidivec::BidiGrowVec;
    ///
    /// let mut bvec: BidiGrowVec<i32> = BidiGrowVec::new();
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// Constructs a new [`BidiGrowVec<T>`] with the specified size,
    /// cloning the specified item in every position.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds [`isize::MAX`] bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiGrowVec;
    ///
    /// let mut bvec = BidiGrowVec::with_elem(5, 3, 3);
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
        let mut this = Self {
            data: Vec::with_capacity(height),
        };
        this.resize(width, height, value);
        this
    }

    /// Constructs a new [`BidiGrowVec<T>`] with the specified size,
    /// using the default value in every position.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds [`isize::MAX`] bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiGrowVec;
    ///
    /// let mut bvec = BidiGrowVec::<i32>::with_size_default(3, 3);
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

    /// Constructs a new [`BidiGrowVec<T>`] with the specified size,
    /// using the specified closure to produce values.
    /// The order the closure is called when producing a new value is
    /// not guaranteed. If the item produced is depending on the its
    /// coordinates, use the slower `BidiGrowVec<T>::with_size_func_xy`.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds [`isize::MAX`] bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiGrowVec;
    ///
    /// let mut bvec = BidiGrowVec::with_size_func(3, 3, ||137);
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
        let mut this = Self {
            data: Vec::with_capacity(height),
        };
        this.resize_with(width, height, f);
        this
    }

    /// Constructs a new [`BidiGrowVec<T>`] with the specified size,
    /// using the specified closure to produce values.
    /// The order the closure is called when producing a new value is
    /// not guaranteed, but the closure will receive the item coordinates
    /// as an input. If the coordinates are not needed, `BidiGrowVec::with_size_func`
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
    /// use bidivec::BidiGrowVec;
    ///
    /// let mut bvec = BidiGrowVec::with_size_func_xy(3, 3, |x,y| x+y);
    ///
    /// assert_eq!(bvec.len(), 9);
    /// assert_eq!(bvec.width(), 3);
    /// assert_eq!(bvec.height(), 3);
    /// assert_eq!(bvec[(1, 2)], 3);
    /// ```
    pub fn with_size_func_xy<F>(width: usize, height: usize, f: F) -> Self
    where
        F: FnMut(usize, usize) -> T,
    {
        let mut this = Self {
            data: Vec::with_capacity(height),
        };
        this.resize_with_xy(width, height, f);
        this
    }

    /// Creates a bidigrowvec from a draining iterator, using the specified
    /// `row_size`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiGrowVec, bidigrowvec};
    ///
    /// let mut vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
    /// let mut bvec = BidiGrowVec::from_iterator(vec.drain(..), 3).unwrap();
    ///
    /// let expected = bidigrowvec!{
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

    /// Creates a bidigrowvec from another view iterator, using the specified
    /// mapping function to create and/or transform elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiGrowVec, bidigrowvec};
    ///
    /// let from = bidigrowvec!{
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9],
    /// };
    ///
    /// let to = BidiGrowVec::from_view_map(&from, |n| n - 1);
    ///
    /// assert_eq!(to, bidigrowvec!{
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

    /// Creates a [`BidiGrowVec<T>`] from a `Vec<T>` and a specified row size.
    /// Note that, unlike [`BidiVec<T>`] where the operation is O(1), this
    /// constructor is O(n).
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
            Ok(Self { data: Vec::new() })
        } else if row_size != 0 && (vec.len() % row_size) == 0 {
            let height = vec.len() / row_size;
            let mut this = Self {
                data: Vec::with_capacity(height),
            };
            let mut vec = vec;

            for _ in 0..height {
                let rest = vec.split_off(row_size);
                let row = vec;
                vec = rest;
                this.push_row(row)?;
            }

            Ok(this)
        } else {
            Err(BidiError::IncompatibleSize)
        }
    }

    /// Clears the bidigrowvec, removing all values.
    ///
    /// Note that this method has no effect on the allocated capacity
    /// of the bidigrowvec.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiGrowVec, bidigrowvec};
    ///
    /// let mut bvec = bidigrowvec![5; 3, 3];
    ///
    /// bvec.clear();
    ///
    /// assert!(bvec.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Returns the number of items contained in the bidigrowvec.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiGrowVec, bidigrowvec};
    ///
    /// let mut bvec = bidigrowvec![5; 4, 3];
    ///
    /// assert_eq!(bvec.len(), 12);
    /// ```
    pub fn len(&self) -> usize {
        self.width() * self.height()
    }

    /// Returns the width (that is, the size of a row) in the bidigrowvec.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiGrowVec, bidigrowvec};
    ///
    /// let mut bvec = bidigrowvec![5; 4, 3];
    ///
    /// assert_eq!(bvec.width(), 4);
    /// ```
    pub fn width(&self) -> usize {
        if self.data.is_empty() {
            0
        } else {
            self.data[0].len()
        }
    }

    /// Returns the height (that is, the size of a column) in the bidigrowvec.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiGrowVec, bidigrowvec};
    ///
    /// let mut bvec = bidigrowvec![5; 4, 3];
    ///
    /// assert_eq!(bvec.height(), 3);
    /// ```
    pub fn height(&self) -> usize {
        self.data.len()
    }

    /// Returns true if the bidigrowvec contains no elements (that
    /// implies that its width, height and len are all zero).
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiGrowVec;
    ///
    /// let mut bvec = BidiGrowVec::<i32>::new();
    ///
    /// assert!(bvec.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Resizes the[`BidiGrowVec`] in-place so that it has new width and
    /// height.
    ///
    /// Any new item that has to be created is created by cloning the
    /// supplied value.
    /// If the new size is smaller than before in a dimension, the
    ///[`BidiGrowVec`] is truncated.
    ///
    /// This method requires `T` to implement [`Clone`],
    /// in order to be able to clone the passed value.
    /// If you need more flexibility (or want to rely on [`Default`] instead of
    /// [`Clone`]), use [`BidiGrowVec::resize_with`].
    /// If you only need to resize to a smaller size, use [`BidiGrowVec::truncate`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiGrowVec;
    ///
    /// let mut bvec = BidiGrowVec::new();
    /// bvec.resize(3, 3, 5);
    ///
    /// assert_eq!(bvec.len(), 9);
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

        self.truncate(min(self.width(), new_width), min(self.height(), new_height))
            .unwrap();

        for _ in self.width()..new_width {
            self.push_col(std::iter::repeat(value.clone()).take(self.height()))
                .unwrap();
        }

        for _ in self.height()..new_height {
            self.push_row(std::iter::repeat(value.clone()).take(new_width))
                .unwrap();
        }
    }

    /// Resizes the[`BidiGrowVec`] in-place so that it has new width and
    /// height, using the specified closure to generate new values.
    /// The order the clousre is called when producing a new value is
    /// not guaranteed. If the item produced is depending on the its
    /// coordinates, use the slower `BidiGrowVec<T>::resize_with_xy`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiGrowVec;
    ///
    /// let mut bvec = BidiGrowVec::new();
    /// bvec.resize_with(3, 3, ||5);
    ///
    /// assert_eq!(bvec.len(), 9);
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

        self.truncate(min(self.width(), new_width), min(self.height(), new_height))
            .unwrap();

        for _ in self.width()..new_width {
            // avoid https://github.com/rust-lang/rust-clippy/issues/8098
            #[allow(clippy::redundant_closure)]
            self.push_col(std::iter::repeat_with(|| f()).take(self.height()))
                .unwrap();
        }

        for _ in self.height()..new_height {
            // avoid https://github.com/rust-lang/rust-clippy/issues/8098
            #[allow(clippy::redundant_closure)]
            self.push_row(std::iter::repeat_with(|| f()).take(new_width))
                .unwrap();
        }
    }

    /// Resizes the[`BidiGrowVec`] (mostly) in-place so that it has new width and
    /// height, using the specified closure to generate new values.
    /// The order the closure is called when producing a new value is
    /// not guaranteed, but the closure will receive the item coordinates
    /// as an input. If the coordinates are not needed, `BidiGrowVec::resize_with`
    /// is faster and uses less temporary memory (this method uses up
    /// to a row or column size of temporary memory).
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiGrowVec;
    ///
    /// let mut bvec = BidiGrowVec::new();
    /// bvec.resize_with(3, 3, ||5);
    ///
    /// assert_eq!(bvec.len(), 9);
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

        self.truncate(min(self.width(), new_width), min(self.height(), new_height))
            .unwrap();

        for x in self.width()..new_width {
            let mut tmp = Vec::with_capacity(self.height());

            for y in 0..self.height() {
                tmp.push(f(x, y));
            }

            self.push_col(tmp).unwrap();
        }

        for y in self.height()..new_height {
            let mut tmp = Vec::with_capacity(new_width);

            for x in 0..new_width {
                tmp.push(f(x, y));
            }

            self.push_row(tmp).unwrap();
        }
    }

    /// Truncates the[`BidiGrowVec`] so that it has new width and
    /// height that must be strictly lower or equal than the current.
    /// width and height, otherwise a [`BidiError::OutOfBounds`] error
    /// is produced.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiGrowVec, bidigrowvec};
    ///
    /// let mut bvec = bidigrowvec![5; 150, 18];
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
            self.data.truncate(new_height);

            for y in 0..new_height {
                self.data[y].truncate(new_width);
            }
        }

        Ok(())
    }

    /// Shrinks the capacity of the bidigrowvec as much as possible.
    pub fn shrink_to_fit(&mut self) {
        for v in self.data.iter_mut() {
            v.shrink_to_fit();
        }

        self.data.shrink_to_fit()
    }

    /// Converts the vector into a `Vec<T>` where items are linearly
    /// laid out by rows.
    /// This is an O(n) operation; if you need faster conversion to and
    /// from `Vec<T>` consider using [`BidiVec<T>`].
    pub fn into_vec(self) -> Vec<T> {
        let mut result = Vec::with_capacity(self.height() * self.width());
        let mut data = self.data;

        for mut row in data.drain(..) {
            result.append(&mut row);
        }

        result
    }

    /// Converts the vector into a `Vec<Vec<T>>` where items are arranged
    /// as in a vec of rows, where each row is itself a vec of items.
    /// This is O(1).
    pub fn into_vec_of_vec(self) -> Vec<Vec<T>> {
        self.data
    }

    /// Swaps two elements in the bidigrowvec. If any of the coordinates are out
    /// of range, [`BidiError::OutOfBounds`] is returned
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiGrowVec, bidigrowvec};
    ///
    /// let mut bvec = bidigrowvec!{
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
        if self.valid_coords(a.0, a.1) && self.valid_coords(b.0, b.1) {
            if a.1 == b.1 {
                self.data[a.1].swap(a.0, b.0);
            } else {
                let ptr1: *mut T = &mut self.data[a.1][a.0];
                let ptr2: *mut T = &mut self.data[b.1][b.0];
                unsafe {
                    std::ptr::swap(ptr1, ptr2);
                }
            }
            Ok(())
        } else {
            Err(BidiError::OutOfBounds)
        }
    }

    /// Appends a new column to the bidigrowvec.
    /// If the bidigrowvec is not empty, the column to be appended must contain
    /// exactly `height()` elements, or [`BidiError::IncompatibleSize`] is
    /// returned.
    /// If the bidigrowvec is not empty, this operation is also expensive
    /// as it requires O(column_size * bidigrowvec_size) time; use[`BidiGrowVec`] for
    /// faster column pushes (at the loss of linear layout).
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiGrowVec;
    ///
    /// let mut bvec = BidiGrowVec::new();
    /// bvec.push_col([1, 2, 3]).unwrap();
    ///
    /// assert_eq!(bvec[(0, 0)], 1);
    /// assert_eq!(bvec[(0, 1)], 2);
    /// assert_eq!(bvec[(0, 2)], 3);
    /// ```
    pub fn push_col<I: IntoIterator<Item = T>>(&mut self, iter: I) -> Result<(), BidiError> {
        if self.data.is_empty() {
            for val in iter.into_iter() {
                self.data.push(vec![val]);
            }
            Ok(())
        } else {
            let saved_width = self.width();
            let mut rollback = false;
            let mut rows_changed: usize = 0;

            for (row, val) in iter.into_iter().enumerate() {
                rows_changed += 1;
                if row < self.data.len() {
                    self.data[row].push(val);
                } else {
                    rollback = true;
                    break;
                }
            }

            if rollback || rows_changed != self.height() {
                for v in self.data.iter_mut() {
                    v.truncate(saved_width);
                }
                Err(BidiError::IncompatibleSize)
            } else {
                Ok(())
            }
        }
    }

    /// Appends a new row to the bidigrowvec.
    /// If the bidigrowvec is not empty, the row to be appended must contain
    /// exactly `width()` elements, or [`BidiError::IncompatibleSize`] is
    /// returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiGrowVec;
    ///
    /// let mut bvec = BidiGrowVec::new();
    /// bvec.push_row([1, 2, 3]).unwrap();
    ///
    /// assert_eq!(bvec[(0, 0)], 1);
    /// assert_eq!(bvec[(1, 0)], 2);
    /// assert_eq!(bvec[(2, 0)], 3);
    /// ```
    pub fn push_row<I: IntoIterator<Item = T>>(&mut self, iter: I) -> Result<(), BidiError> {
        if self.data.is_empty() {
            self.data.push(iter.into_iter().collect::<Vec<T>>());
            Ok(())
        } else {
            let width = self.width();
            self.data.push(iter.into_iter().collect::<Vec<T>>());

            if self.data[self.data.len() - 1].len() != width {
                self.data.truncate(self.data.len() - 1);
                Err(BidiError::IncompatibleSize)
            } else {
                Ok(())
            }
        }
    }

    /// Inserts a new column in the middle of a bidigrowvec.
    /// If the bidigrowvec is not empty, the column to be inserted must contain
    /// exactly `height()` elements, or [`BidiError::IncompatibleSize`] is
    /// returned.
    ///
    /// If the bidigrowvec is not empty, this operation requires
    /// O(column_size * row_size) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiGrowVec;
    ///
    /// let mut bvec = BidiGrowVec::new();
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
        match col.cmp(&self.width()) {
            Ordering::Greater => Err(BidiError::OutOfBounds),
            Ordering::Equal => self.push_col(iter),
            Ordering::Less => {
                let saved_width = self.width();
                let mut rollback = false;
                let mut rows_changed: usize = 0;

                for (row, val) in iter.into_iter().enumerate() {
                    rows_changed += 1;
                    if row < self.data.len() {
                        self.data[row].insert(col, val);
                    } else {
                        rollback = true;
                        break;
                    }
                }

                if rollback || rows_changed != self.height() {
                    for v in self.data.iter_mut() {
                        if v.len() > saved_width {
                            v.remove(col);
                        }
                    }
                    Err(BidiError::IncompatibleSize)
                } else {
                    Ok(())
                }
            }
        }
    }

    /// Inserts a new row in the middle of a bidigrowvec.
    /// If the bidigrowvec is not empty, the row to be inserted must contain
    /// exactly `width()` elements, or [`BidiError::IncompatibleSize`] is
    /// returned.
    ///
    /// This operation is O(height).
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::BidiGrowVec;
    ///
    /// let mut bvec = BidiGrowVec::new();
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
        match (row).cmp(&self.data.len()) {
            Ordering::Greater => Err(BidiError::OutOfBounds),
            Ordering::Equal => self.push_row(iter),
            Ordering::Less => {
                let row_data = iter.into_iter().collect::<Vec<T>>();

                if row_data.len() != self.width() {
                    Err(BidiError::IncompatibleSize)
                } else {
                    self.data.insert(row, row_data);
                    Ok(())
                }
            }
        }
    }

    /// Removes the specified column from the bidigrowvec. If the column is
    /// outside of range, [`BidiError::OutOfBounds`] is returned.
    ///
    /// If the deleted data is not needed, `BidiGrowVec::delete_col` provides
    /// better performances.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiGrowVec, bidigrowvec};
    ///
    /// let mut bvec = bidigrowvec!{
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
            Err(BidiError::OutOfBounds)
        } else {
            let mut result = Vec::with_capacity(self.height());

            for v in self.data.iter_mut() {
                result.push(v.remove(col));
            }

            self.collapse();

            Ok(result)
        }
    }

    /// Removes the specified row from the bidigrowvec. If the row is
    /// outside of range, [`BidiError::OutOfBounds`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiGrowVec, bidigrowvec};
    ///
    /// let mut bvec = bidigrowvec!{
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
            Err(BidiError::OutOfBounds)
        } else {
            Ok(self.data.remove(row))
        }
    }

    /// Deletes the specified column from the bidigrowvec. If the column is
    /// outside of range, [`BidiError::OutOfBounds`] is returned.
    ///
    /// If you need to access the deleted data is not needed,
    /// `BidiGrowVec::remove_col` provides that data, at a performance cost.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiGrowVec, bidigrowvec};
    ///
    /// let mut bvec = bidigrowvec!{
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
            Err(BidiError::OutOfBounds)
        } else {
            for v in self.data.iter_mut() {
                v.remove(col);
            }
            self.collapse();
            Ok(())
        }
    }

    /// Deletes the specified row from the bidigrowvec. If the row is
    /// outside of range, [`BidiError::OutOfBounds`] is returned.
    ///
    /// This method is no faster than `remove_row` and exists only
    /// for symmetry with `delete_col` and other data structures
    /// where `delete_row` offers a performance advantage.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiGrowVec, bidigrowvec};
    ///
    /// let mut bvec = bidigrowvec!{
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
        self.remove_row(row)?;
        Ok(())
    }

    /// Deletes the last column from the bidigrowvec.
    ///
    /// If you need to access the deleted data is not needed,
    /// `BidiGrowVec::pop_col` provides that data, at a performance cost.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiGrowVec, bidigrowvec};
    ///
    /// let mut bvec = bidigrowvec!{
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
        if self.width() > 0 {
            self.delete_col(self.width() - 1).unwrap();
        }
    }

    /// Deletes the last row from the bidigrowvec.
    ///
    /// If you need to access the deleted data is not needed,
    /// `BidiGrowVec::pop_row` provides that data, at a performance cost.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiGrowVec, bidigrowvec};
    ///
    /// let mut bvec = bidigrowvec!{
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
        self.data.pop();
    }

    /// Removes the last column from the bidigrowvec, returning its data.
    ///
    /// If the removed data is not needed, `BidiGrowVec::delete_last_col`
    /// provides better performances.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiGrowVec, bidigrowvec};
    ///
    /// let mut bvec = bidigrowvec!{
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
        if self.is_empty() {
            None
        } else {
            Some(self.remove_col(self.width() - 1).unwrap())
        }
    }

    /// Removes the last row from the bidigrowvec, returning its data.
    ///
    /// If the removed data is not needed, `BidiGrowVec::delete_last_row`
    /// provides better performances.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiGrowVec, bidigrowvec};
    ///
    /// let mut bvec = bidigrowvec!{
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
        self.data.pop()
    }

    /// Accesses an element in the BidiGrowVec, using its cartesian coordinates.
    /// If coordinates are outside of range, [`None`] is returned.
    ///
    /// If the error is not going to be handled, direct indexing is an easier
    /// way to achieve the same results.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiGrowVec, bidigrowvec};
    ///
    /// let mut bvec = bidigrowvec!{
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
        if self.valid_coords(x, y) {
            Some(&self.data[y][x])
        } else {
            None
        }
    }

    /// Mutably accesses an element in the BidiGrowVec, using its cartesian coordinates.
    /// If coordinates are outside of range, [`None`] is returned.
    ///
    /// If the error is not going to be handled, direct indexing is an easier
    /// way to achieve the same results.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiGrowVec, bidigrowvec};
    ///
    /// let mut bvec = bidigrowvec!{
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
        if self.valid_coords(x, y) {
            Some(&mut self.data[y][x])
        } else {
            None
        }
    }

    /// Checks if the specified coordinates are inside the bidigrowvec bounds
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiGrowVec, bidigrowvec};
    ///
    /// let mut bvec = bidigrowvec!{
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
        x < self.width() && y < self.height()
    }

    /// Reverses the order of the items in the specified row.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiGrowVec, bidigrowvec};
    ///
    /// let mut bvec = bidigrowvec!{
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
            Err(BidiError::OutOfBounds)
        } else {
            self.data[row].reverse();
            Ok(())
        }
    }

    /// Reverses the order of the items in the specified column.
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiGrowVec, bidigrowvec};
    ///
    /// let mut bvec = bidigrowvec!{
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
            Err(BidiError::OutOfBounds)
        } else {
            let height = self.height();

            for y in 0..(height / 2) {
                self.swap((col, y), (col, height - 1 - y)).unwrap();
            }

            Ok(())
        }
    }

    /// Transposes the bidigrowvec, that is an operation that flips the bidigrowvec
    /// over its diagonal (or, more simply, switches the meaning of columns and
    /// rows). As such, the result of a transposition is as wide as the original
    /// was tall, and as tall as the original was wide.
    pub fn transpose(&mut self) {
        let width = self.width();
        let height = self.height();
        let min_dimension = std::cmp::min(width, height);

        for x in 0..min_dimension {
            for y in (x + 1)..min_dimension {
                self.swap((x, y), (y, x)).unwrap();
            }
        }

        if width > min_dimension {
            let mut excess_cols = Vec::new();

            while self.data[0].len() > min_dimension {
                let colv = self.remove_col(min_dimension).unwrap();
                excess_cols.push(colv);
            }

            for col in excess_cols.drain(..) {
                self.push_row(col).unwrap();
            }
        } else if height > min_dimension {
            let mut excess_rows = Vec::new();

            while self.data.len() > min_dimension {
                let rowv = self.remove_row(min_dimension).unwrap();
                excess_rows.push(rowv);
            }

            for row in excess_rows.drain(..) {
                self.push_col(row).unwrap();
            }
        }
    }

    /// Rotates the bidigrowvec 90°, counter-clockwise (or, 270° clockwise).
    /// The result of such a rotation is as wide as the original
    /// was tall, and as tall as the original was wide.
    ///
    /// While this is performed in-place, it still requires O(n) additional memory
    /// if the bidigrowvec width and height are different (i.e. it's not a square).
    pub fn rotate90ccw(&mut self) {
        self.transpose();
        self.reverse_columns();
    }

    /// Rotates the bidigrowvec 180°.
    pub fn rotate180(&mut self) {
        self.reverse_rows();
        self.reverse_columns();
    }

    /// Rotates the bidigrowvec 270°, counter-clockwise (or, 90° clockwise).
    /// The result of such a rotation is as wide as the original
    /// was tall, and as tall as the original was wide.
    ///
    /// While this is performed in-place, it still requires O(n) additional memory
    /// if the bidigrowvec width and height are different (i.e. it's not a square).
    pub fn rotate270ccw(&mut self) {
        self.transpose();
        self.reverse_rows();
    }

    /// Reverse the order of items in all columns. This is equivalent to flipping
    /// the data structure over its horizontal axis.
    pub fn reverse_columns(&mut self) {
        self.data.reverse();
    }

    /// Reverse the order of items in all rows. This is equivalent to flipping
    /// the data structure over its vertical axis.
    pub fn reverse_rows(&mut self) {
        for r in self.data.iter_mut() {
            r.reverse();
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

    fn collapse(&mut self) {
        if !self.data.is_empty() && self.data[0].is_empty() {
            self.data.clear();
        }
    }

    /// Converts this instance into a [`BidiVec<T>`]
    /// This operation is `O(width*height)` in the worst case.
    pub fn into_bidivec(self) -> BidiVec<T> {
        BidiVec::<T>::from(self)
    }

    /// Converts this instance into a [`BidiArray<T>`]
    /// This operation is `O(width*height)` in the worst case.
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

impl<T> BidiFrom<&dyn BidiView<Output = T>> for BidiGrowVec<T>
where
    T: Clone,
{
    fn from_view(source: &dyn BidiView<Output = T>) -> Result<Self, BidiError> {
        Ok(BidiGrowVec::<T>::with_size_func_xy(
            source.width(),
            source.height(),
            |x, y| source[(x, y)].clone(),
        ))
    }

    fn from_view_cut(source: &dyn BidiView<Output = T>, cut: &BidiRect) -> Result<Self, BidiError> {
        if cut.max_x() > source.height() || cut.max_y() > source.width() {
            return Err(BidiError::OutOfBounds);
        }

        Ok(BidiGrowVec::<T>::with_size_func_xy(
            cut.width,
            cut.height,
            |x, y| source[(x + cut.x, y + cut.y)].clone(),
        ))
    }
}

impl<T> BidiFrom<BidiGrowVec<T>> for BidiGrowVec<T> {
    fn from_view(source: BidiGrowVec<T>) -> Result<Self, BidiError> {
        Ok(source)
    }

    fn from_view_cut(mut source: BidiGrowVec<T>, cut: &BidiRect) -> Result<Self, BidiError> {
        source.crop(cut)?;
        Ok(source)
    }
}

impl<T> BidiFrom<BidiVec<T>> for BidiGrowVec<T> {
    fn from_view(source: BidiVec<T>) -> Result<Self, BidiError> {
        Ok(Self::from(source))
    }

    fn from_view_cut(mut source: BidiVec<T>, cut: &BidiRect) -> Result<Self, BidiError> {
        source.crop(cut)?;
        Ok(Self::from(source))
    }
}

impl<T> BidiFrom<BidiArray<T>> for BidiGrowVec<T> {
    fn from_view(source: BidiArray<T>) -> Result<Self, BidiError> {
        Ok(Self::from(source))
    }

    fn from_view_cut(source: BidiArray<T>, cut: &BidiRect) -> Result<Self, BidiError> {
        let mut this = Self::from(source);
        this.crop(cut)?;
        Ok(this)
    }
}

impl<T> Index<(usize, usize)> for BidiGrowVec<T> {
    type Output = T;

    /// Accesses an element in the BidiGrowVec, using its cartesian coordinates.
    /// If coordinates are outside of range, it panics.
    ///
    /// If you want a more graceful error handling, see [`BidiGrowVec::get`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiGrowVec, bidigrowvec};
    ///
    /// let mut bvec = bidigrowvec!{
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9],
    /// };
    ///
    /// assert_eq!(bvec[(1, 1)], 5);
    /// ```
    #[inline(always)]
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.1][index.0]
    }
}

impl<T> IndexMut<(usize, usize)> for BidiGrowVec<T> {
    /// Mutably accesses an element in the BidiGrowVec, using its cartesian coordinates.
    /// If coordinates are outside of range, it panics.
    ///
    /// If you want a more graceful error handling, see [`BidiGrowVec::get_mut`] and
    ///
    /// # Examples
    ///
    /// ```
    /// use bidivec::{BidiGrowVec, bidigrowvec};
    ///
    /// let mut bvec = bidigrowvec!{
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
        &mut self.data[index.1][index.0]
    }
}

impl<T> BidiView for BidiGrowVec<T> {
    fn width(&self) -> usize {
        BidiGrowVec::<T>::width(self)
    }
    fn height(&self) -> usize {
        BidiGrowVec::<T>::height(self)
    }

    fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.get(x, y)
    }
}

impl<T> BidiViewMut for BidiGrowVec<T> {
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.get_mut(x, y)
    }
}

unsafe impl<T> BidiViewMutIterable for BidiGrowVec<T> {}

impl<T> From<BidiVec<T>> for BidiGrowVec<T> {
    /// Creates a new instance of [`BidiGrowVec<T>`] from an existing [`BidiVec<T>`].
    /// This operation is `O(width*height)` in the worst case.
    fn from(other: BidiVec<T>) -> Self {
        let row_size = other.width();
        Self::from_vec(other.data, row_size).unwrap()
    }
}

impl<T> From<BidiArray<T>> for BidiGrowVec<T> {
    /// Creates a new instance of [`BidiGrowVec<T>`] from an existing [`BidiVec<T>`].
    /// This operation is `O(width*height)` in the worst case.
    fn from(other: BidiArray<T>) -> Self {
        let row_size = other.width();
        Self::from_vec(other.data.into_vec(), row_size).unwrap()
    }
}
