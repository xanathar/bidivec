//! A module containing adapters that alter the behavior of
//! a given [`BidiView`][crate::BidiView] (or [`BidiViewMut`][crate::BidiViewMut])
//! to apply transformations like flips, rotations and crops.
//!
//! Types in this module are supposed to be used through methods in
//! [`BidiView`][crate::BidiView] and [`BidiViewMut`][crate::BidiViewMut], rather than directly.

use super::*;
use crate::{BidiError, BidiRect};
use std::cmp::min;
use std::ops::{Index, IndexMut};

macro_rules! impl_transform_type {
    ($t:ty, $src:tt) => {
        impl<S: BidiView> Index<(usize, usize)> for $t {
            type Output = S::Output;

            fn index(&self, index: (usize, usize)) -> &Self::Output {
                let pos = self._pos(index.0, index.1);
                &self.$src[pos]
            }
        }

        impl<S: BidiView + BidiViewMut> IndexMut<(usize, usize)> for $t {
            fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
                let pos = self._pos(index.0, index.1);
                &mut self.$src[pos]
            }
        }

        impl<S: BidiView> BidiView for $t {
            fn width(&self) -> usize {
                self._width()
            }

            fn height(&self) -> usize {
                self._height()
            }

            fn get(&self, x: usize, y: usize) -> Option<&Self::Output> {
                let pos = self._pos(x, y);
                self.$src.get(pos.0, pos.1)
            }
        }

        impl<S: BidiView + BidiViewMut> BidiViewMut for $t {
            fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Self::Output> {
                let pos = self._pos(x, y);
                self.$src.get_mut(pos.0, pos.1)
            }
        }
    };
}

/// Used as an output type by [`BidiView::to_transposed()`].
#[derive(Debug)]
pub struct TransposingBidiView<S: BidiView>(S);
impl_transform_type!(TransposingBidiView<S>, 0);
impl<S: BidiView> TransposingBidiView<S> {
    pub fn new(source: S) -> Self {
        Self(source)
    }
    fn _pos(&self, x: usize, y: usize) -> (usize, usize) {
        (y, x)
    }
    fn _width(&self) -> usize {
        self.0.height()
    }
    fn _height(&self) -> usize {
        self.0.width()
    }
}

/// Used as an output type by [`BidiView::to_reversed_columns()`].
#[derive(Debug)]
pub struct ReversingColumnsBidiView<S: BidiView>(S);
impl_transform_type!(ReversingColumnsBidiView<S>, 0);
impl<S: BidiView> ReversingColumnsBidiView<S> {
    pub fn new(source: S) -> Self {
        Self(source)
    }
    fn _pos(&self, x: usize, y: usize) -> (usize, usize) {
        (x, self._height() - y - 1)
    }
    fn _width(&self) -> usize {
        self.0.width()
    }
    fn _height(&self) -> usize {
        self.0.height()
    }
}

/// Used as an output type by [`BidiView::to_reversed_rows()`].
#[derive(Debug)]
pub struct ReversingRowsBidiView<S: BidiView>(S);
impl_transform_type!(ReversingRowsBidiView<S>, 0);
impl<S: BidiView> ReversingRowsBidiView<S> {
    pub fn new(source: S) -> Self {
        Self(source)
    }
    fn _pos(&self, x: usize, y: usize) -> (usize, usize) {
        (self._width() - x - 1, y)
    }
    fn _width(&self) -> usize {
        self.0.width()
    }
    fn _height(&self) -> usize {
        self.0.height()
    }
}

/// Used as an output type by [`BidiView::to_rotated180()`].
#[derive(Debug)]
pub struct Rotating180BidiView<S: BidiView>(S);
impl_transform_type!(Rotating180BidiView<S>, 0);
impl<S: BidiView> Rotating180BidiView<S> {
    pub fn new(source: S) -> Self {
        Self(source)
    }
    fn _pos(&self, x: usize, y: usize) -> (usize, usize) {
        (self._width() - x - 1, self._height() - y - 1)
    }
    fn _width(&self) -> usize {
        self.0.width()
    }
    fn _height(&self) -> usize {
        self.0.height()
    }
}

/// Used as an output type by [`BidiView::to_rotated270ccw()`].
#[derive(Debug)]
pub struct Rotating270BidiView<S: BidiView>(S);
impl_transform_type!(Rotating270BidiView<S>, 0);
impl<S: BidiView> Rotating270BidiView<S> {
    pub fn new(source: S) -> Self {
        Self(source)
    }
    fn _pos(&self, x: usize, y: usize) -> (usize, usize) {
        (y, self.0.height() - x - 1)
    }
    fn _width(&self) -> usize {
        self.0.height()
    }
    fn _height(&self) -> usize {
        self.0.width()
    }
}

/// Used as an output type by [`BidiView::to_rotated90ccw()`].
#[derive(Debug)]
pub struct Rotating90BidiView<S: BidiView>(S);
impl_transform_type!(Rotating90BidiView<S>, 0);
impl<S: BidiView> Rotating90BidiView<S> {
    pub fn new(source: S) -> Self {
        Self(source)
    }
    fn _pos(&self, x: usize, y: usize) -> (usize, usize) {
        (self.0.width() - y - 1, x)
    }
    fn _width(&self) -> usize {
        self.0.height()
    }
    fn _height(&self) -> usize {
        self.0.width()
    }
}

/// Used as an output type by [`BidiView::to_cropped()`].
#[derive(Debug)]
pub struct CroppingBidiView<S: BidiView> {
    source: S,
    rect: BidiRect,
}
impl_transform_type!(CroppingBidiView<S>, source);
impl<S: BidiView> CroppingBidiView<S> {
    pub fn new(source: S, r: &BidiRect) -> Result<Self, BidiError> {
        if (r.width + r.x > source.width()) || (r.height + r.y > source.height()) {
            Err(BidiError::OutOfBounds)
        } else {
            Ok(Self {
                source,
                rect: r.clone(),
            })
        }
    }

    fn _pos(&self, x: usize, y: usize) -> (usize, usize) {
        (x + self.rect.x, y + self.rect.y)
    }
    fn _width(&self) -> usize {
        min(self.rect.width, self.source.width())
    }
    fn _height(&self) -> usize {
        min(self.rect.height, self.source.height())
    }
}
