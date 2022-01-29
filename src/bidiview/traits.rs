use super::transforming::*;
use crate::bidiiter::*;
use crate::{BidiError, BidiRect};
use std::ops::{Index, IndexMut};

/// An object-safe trait providing a bidimensional view over a data structure.
///
/// This trait is used internally to abstract many operations from the
/// underlying data structures, but can be implemented by external types
/// to re-use the algorithms with other bidimensional data structures.
pub trait BidiView: Index<(usize, usize)> {
    /// Returns the width of the bidimensional view
    ///
    /// # Examples
    /// ```
    /// # use bidivec::{bidiarray, BidiView};
    ///
    /// let v = bidiarray!{
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    /// };
    ///
    /// let r = v.size();
    ///
    /// assert_eq!(3, v.width());
    /// assert_eq!(2, v.height());
    /// ```
    fn width(&self) -> usize;

    /// Returns the height of the bidimensional view
    ///
    /// # Examples
    /// ```
    /// # use bidivec::{bidiarray, BidiView};
    ///
    /// let v = bidiarray!{
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    /// };
    ///
    /// assert_eq!(3, v.width());
    /// assert_eq!(2, v.height());
    /// ```
    fn height(&self) -> usize;

    /// Returns the size of the bidimensional view (i.e. a tuple of `(width, height)`).
    ///
    /// # Examples
    /// ```
    /// # use bidivec::{bidiarray, BidiView};
    ///
    /// let v = bidiarray!{
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    /// };
    ///
    /// let r = v.size();
    ///
    /// assert_eq!(3, r.0);
    /// assert_eq!(2, r.1);
    /// ```
    fn size(&self) -> (usize, usize) {
        (self.width(), self.height())
    }

    /// Returns true if two bidimensional views are equivalent (that is they
    /// have the same width, height and equal elements
    ///
    /// # Examples
    /// ```
    /// # use bidivec::{bidivec, bidiarray, BidiView};
    ///
    /// let v = bidivec!{
    ///     [1, 2],
    ///     [3, 4],
    /// };
    ///
    /// let a = bidiarray!{
    ///     [1, 2],
    ///     [3, 4],
    /// };
    ///
    /// assert!(v.equivalent(&a));
    /// ```
    fn equivalent<V>(&self, other: &V) -> bool
    where
        Self: Sized,
        Self::Output: PartialEq + Sized + std::fmt::Debug,
        V: BidiView<Output = Self::Output>,
    {
        for y in 0..self.height() {
            for x in 0..self.width() {
                print!("{:?}", self[(x, y)]);
            }
            println!();
        }

        if other.width() != self.width() || other.height() != self.height() {
            false
        } else {
            self.iter().eq(other.iter())
        }
    }

    /// Returns the item at (x, y) coordinates, or [`None`] if the
    /// coordinates are out of range.
    ///
    /// # Examples
    /// ```
    /// # use bidivec::{bidivec, bidiarray, BidiView};
    ///
    /// let a = bidiarray!{
    ///     [1, 2],
    ///     [3, 4],
    /// };
    ///
    /// assert_eq!(a.get(0, 0).unwrap(), &1);
    /// assert!(a.get(3, 0).is_none());
    /// ```
    fn get(&self, x: usize, y: usize) -> Option<&Self::Output>;

    /// Returns the item at (x, y) coordinates (using signed coordinates),
    /// or [`None`] if the coordinates are out of range.
    ///
    /// # Examples
    /// ```
    /// # use bidivec::{bidivec, bidiarray, BidiView};
    ///
    /// let a = bidiarray!{
    ///     [1, 2],
    ///     [3, 4],
    /// };
    ///
    /// assert_eq!(a.get_signed(0, 0).unwrap(), &1);
    /// assert!(a.get_signed(3, 0).is_none());
    /// assert!(a.get_signed(-1, 0).is_none());
    /// ```
    fn get_signed(&self, x: isize, y: isize) -> Option<&Self::Output> {
        if x < 0 || y < 0 {
            None
        } else {
            self.get(x as usize, y as usize)
        }
    }

    /// Returns the bounding rect of the view
    ///
    /// # Examples
    /// ```
    /// # use bidivec::{bidiarray, BidiView};
    ///
    /// let v = bidiarray!{
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    /// };
    ///
    /// let r = v.bounding_rect();
    ///
    /// assert_eq!(0, r.x);
    /// assert_eq!(0, r.y);
    /// assert_eq!(3, r.width);
    /// assert_eq!(2, r.height);
    /// ```
    fn bounding_rect(&self) -> BidiRect {
        BidiRect::new(0, 0, self.width(), self.height())
    }

    /// Returns an iterator over the items of the view
    ///
    /// # Examples
    /// ```
    /// # use bidivec::{bidiarray, BidiView};
    ///
    /// fn get_max<V>(v: &V) -> Option<i32>
    /// where V: BidiView<Output=i32>
    /// {
    ///     v.iter().copied().max()
    /// };
    ///
    /// let v = bidiarray!{
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    /// };
    ///
    /// assert_eq!(Some(6), get_max(&v));
    /// ```
    fn iter(&self) -> Iter<Self::Output, Self>
    where
        Self::Output: Sized,
        Self: Sized,
    {
        Iter::new(self)
    }

    /// Returns a bidiview that represents data in this bidiview as
    /// transposed (that is, flipped over its diagonal).
    ///
    /// # Examples
    /// ```
    /// # use bidivec::{bidiarray, BidiView};
    ///
    /// let v = bidiarray!{
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    /// };
    ///
    /// let v = v.to_transposed();
    ///
    /// assert!(v.equivalent(&bidiarray!{
    ///     [1, 4],
    ///     [2, 5],
    ///     [3, 6],
    /// }));
    /// ```
    fn to_transposed(self) -> TransposingBidiView<Self>
    where
        Self: Sized,
    {
        TransposingBidiView::new(self)
    }

    /// Returns a bidiview that represents data in this bidiview as
    /// rotated by 180°.
    ///
    /// # Examples
    /// ```
    /// # use bidivec::{bidiarray, BidiView};
    ///
    /// let v = bidiarray!{
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    /// };
    ///
    /// let v = v.to_rotated180();
    ///
    /// assert!(v.equivalent(&bidiarray!{
    ///     [6, 5, 4],
    ///     [3, 2, 1],
    /// }));
    /// ```
    fn to_rotated180(self) -> Rotating180BidiView<Self>
    where
        Self: Sized,
    {
        Rotating180BidiView::new(self)
    }

    /// Returns a bidiview that represents data in this bidiview as
    /// rotated by 90° counter-clockwise
    ///
    /// # Examples
    /// ```
    /// # use bidivec::{bidiarray, BidiView};
    ///
    /// let v = bidiarray!{
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    /// };
    ///
    /// let v = v.to_rotated90ccw();
    ///
    /// assert!(v.equivalent(&bidiarray!{
    ///     [3, 6],
    ///     [2, 5],
    ///     [1, 4],
    /// }));
    /// ```
    fn to_rotated90ccw(self) -> Rotating90BidiView<Self>
    where
        Self: Sized,
    {
        Rotating90BidiView::new(self)
    }

    /// Returns a bidiview that represents data in this bidiview as
    /// rotated by 270° counter-clockwise
    ///
    /// # Examples
    /// ```
    /// # use bidivec::{bidiarray, BidiView};
    ///
    /// let v = bidiarray!{
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    /// };
    ///
    /// let v = v.to_rotated270ccw();
    ///
    /// assert!(v.equivalent(&bidiarray!{
    ///     [4, 1],
    ///     [5, 2],
    ///     [6, 3],
    /// }));
    /// ```
    fn to_rotated270ccw(self) -> Rotating270BidiView<Self>
    where
        Self: Sized,
    {
        Rotating270BidiView::new(self)
    }

    /// Returns a bidiview that represents data in this bidiview as
    /// if its columns were all reversed (as if flipped over an horizontal
    /// axis).
    ///
    /// # Examples
    /// ```
    /// # use bidivec::{bidiarray, BidiView};
    ///
    /// let v = bidiarray!{
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    /// };
    ///
    /// let v = v.to_reversed_columns();
    ///
    /// assert!(v.equivalent(&bidiarray!{
    ///     [4, 5, 6],
    ///     [1, 2, 3],
    /// }));
    /// ```
    fn to_reversed_columns(self) -> ReversingColumnsBidiView<Self>
    where
        Self: Sized,
    {
        ReversingColumnsBidiView::new(self)
    }

    /// Returns a bidiview that represents data in this bidiview as
    /// if its rows were all reversed (as if flipped over a vertical
    /// axis).
    ///
    /// # Examples
    /// ```
    /// # use bidivec::{bidiarray, BidiView};
    ///
    /// let v = bidiarray!{
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    /// };
    ///
    /// let v = v.to_reversed_rows();
    ///
    /// assert!(v.equivalent(&bidiarray!{
    ///     [3, 2, 1],
    ///     [6, 5, 4],
    /// }));
    /// ```
    fn to_reversed_rows(self) -> ReversingRowsBidiView<Self>
    where
        Self: Sized,
    {
        ReversingRowsBidiView::new(self)
    }

    /// Returns a bidiview that represents data in this bidiview as if was
    /// cropped, by starting at a different origin, and having different
    /// height and width.
    ///
    /// # Examples
    /// ```
    /// # use bidivec::{bidiarray, BidiView, BidiRect};
    ///
    /// let v = bidiarray!{
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    /// };
    ///
    /// let v = v.to_cropped(&BidiRect::new(0, 0, 2, 2))?;
    ///
    /// assert!(v.equivalent(&bidiarray!{
    ///     [1, 2],
    ///     [4, 5],
    /// }));
    /// # Ok::<(), bidivec::BidiError>(())
    /// ```
    fn to_cropped(self, rect: &BidiRect) -> Result<CroppingBidiView<Self>, BidiError>
    where
        Self: Sized,
    {
        CroppingBidiView::new(self, rect)
    }
}

/// An object-safe trait providing a mutable bidimensional view over a data structure.
pub trait BidiViewMut: BidiView + IndexMut<(usize, usize)> {
    /// Mutably returns the item at (x, y) coordinates, or [`None`] if the
    /// coordinates are out of range.
    ///
    /// # Examples
    /// ```
    /// # use bidivec::{bidivec, bidiarray, BidiView, BidiViewMut};
    ///
    /// let mut a = bidiarray!{
    ///     [1, 2],
    ///     [3, 4],
    /// };
    ///
    /// *a.get_mut(0, 0).unwrap() = 8;
    ///
    /// assert_eq!(a[(0, 0)], 8);
    /// assert!(a.get_mut(3, 0).is_none());
    /// ```
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Self::Output>;

    /// Mutably returns the item at (x, y) coordinates (using signed coordinates),
    /// or [`None`] if the coordinates are out of range.
    ///
    /// # Examples
    /// ```
    /// # use bidivec::{bidivec, bidiarray, BidiView, BidiViewMut};
    ///
    /// let mut a = bidiarray!{
    ///     [1, 2],
    ///     [3, 4],
    /// };
    ///
    /// *a.get_mut_signed(0, 0).unwrap() = 8;
    ///
    /// assert_eq!(a[(0, 0)], 8);
    /// assert!(a.get_mut_signed(-1, 0).is_none());
    fn get_mut_signed(&mut self, x: isize, y: isize) -> Option<&mut Self::Output> {
        if x < 0 || y < 0 {
            None
        } else {
            self.get_mut(x as usize, y as usize)
        }
    }
}

/// An unsafe trait for views which can have a [`BidiViewMut`] mutable iterator.
/// This is `unsafe` because additional constraints must be guaranteed by a [`BidiViewMut`]
/// to be safely mutably iterable.
///
/// # Safety
///
/// Types implementing this trait must absolutely guarantee that a given item is accessed
/// uniquely through a given `(x, y)` pair of coordinates, or, more explicitely, that
/// given two set of coordinates `(x, y)` and `(x', y')`, they refer to the same item in
/// memory if and only if `x == x'` and `y == y'`.
///
/// If that isn't true, mutable aliasing may occur and that violates the borrow-checker
/// invariants.
pub unsafe trait BidiViewMutIterable: BidiViewMut {
    /// Returns a mutable iterator over the items of the view
    ///
    /// # Examples
    /// ```
    /// # use bidivec::{bidiarray, BidiView, BidiViewMutIterable};
    ///
    /// fn zeroize<V>(v: &mut V)
    /// where V: BidiViewMutIterable<Output=i32>
    /// {
    ///     for n in v.iter_mut() {
    ///         *n = 0;
    ///     }
    /// };
    ///
    /// let mut v = bidiarray!{
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    /// };
    ///
    /// zeroize(&mut v);
    ///
    /// assert!(v.equivalent(&bidiarray!{
    ///     [0, 0, 0],
    ///     [0, 0, 0],
    /// }));
    /// ```
    fn iter_mut(&mut self) -> IterMut<Self::Output, Self>
    where
        Self::Output: Sized,
        Self: Sized,
    {
        IterMut::new(self)
    }
}

/// An object-safe trait that bidimensional data structures can implement to
/// provide construction from other existing bidimensional data structures.
pub trait BidiFrom<S>: Sized {
    /// Constructs a new instance of the type implementing this trait
    /// using another BidiView as the source of data.
    fn from_view(source: S) -> Result<Self, BidiError>;
    /// Constructs a new instance of the type implementing this trait
    /// using the specified region of another BidiView as the source of data.
    fn from_view_cut(source: S, cut: &BidiRect) -> Result<Self, BidiError>;
}

#[allow(dead_code)]
fn canary_trait_object_safe_bidiview(_: &dyn BidiView<Output = ()>) -> ! {
    // This exists only to make compilation fail if for whatever reason
    // BidiView ceases to be object-safe.
    unimplemented!()
}
#[allow(dead_code)]
fn canary_trait_object_safe_bidiviewmut(_: &dyn BidiViewMut<Output = ()>) -> ! {
    // This exists only to make compilation fail if for whatever reason
    // BidiViewMut ceases to be object-safe.
    unimplemented!()
}
