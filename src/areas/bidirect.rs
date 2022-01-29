use crate::BidiRectSigned;
use std::cmp::{max, min};
use std::ops::Range;

/// A simple data structure representing a bidimensional rectangle
/// used to express ranges, areas and rects in bidimensional data
/// structures. For an example, see [`BidiVec::crop()`][`crate::BidiVec::crop`].
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BidiRect {
    /// The left-most x coordinate of the rectangle (i.e. the minimum x value)
    pub x: usize,
    /// The top-most y coordinate of the rectangle (i.e. the minimum y value)
    pub y: usize,
    /// The width of the rectangle
    pub width: usize,
    /// The height of the rectangle
    pub height: usize,
}

impl BidiRect {
    /// Creates a new rectangle
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        BidiRect {
            x,
            y,
            width,
            height,
        }
    }

    /// Returns the coordinate of the top-left corner
    pub fn min_xy(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    /// Returns the coordinate of one more than the bottom-right corner
    /// (i.e. the bottom-right corner, non-inclusive)
    pub fn max_xy(&self) -> (usize, usize) {
        (self.x + self.width, self.y + self.height)
    }

    /// Returns the range of x coordinates (i.e. every `min_x <= x < max_x`)
    pub fn x_range(&self) -> Range<usize> {
        self.x..(self.x + self.width)
    }

    /// Returns the range of y coordinates (i.e. every `min_y <= y < max_y`)
    pub fn y_range(&self) -> Range<usize> {
        self.y..(self.y + self.height)
    }

    /// Returns the range of x coordinates clipped to a fence value
    /// (i.e. every `min_x <= x < min(max_x, fence_val)`)
    pub fn x_range_clip(&self, fence_val: usize) -> Range<usize> {
        self.x..min(fence_val, self.x + self.width)
    }

    /// Returns the range of y coordinates clipped to a fence value
    /// (i.e. every `min_y <= y < min(max_y, fence_val)`)
    pub fn y_range_clip(&self, fence_val: usize) -> Range<usize> {
        self.y..min(fence_val, self.y + self.height)
    }

    /// The left-most x coordinate of the rectangle (i.e. the minimum x value)
    pub fn min_x(&self) -> usize {
        self.x
    }

    /// The top-most y coordinate of the rectangle (i.e. the minimum y value)
    pub fn min_y(&self) -> usize {
        self.y
    }

    /// An x coordinate one to the right of the right-most coordinate of the
    /// rectangle
    pub fn max_x(&self) -> usize {
        self.x + self.width
    }

    /// A y coordinate one below of the bottom-most coordinate of the
    /// rectangle
    pub fn max_y(&self) -> usize {
        self.y + self.height
    }

    /// Returns true if the rect contains the specified point
    pub fn contains(&self, x: usize, y: usize) -> bool {
        x >= self.x && x < self.max_x() && y >= self.y && y < self.max_y()
    }

    /// Returns true if the rect contains the specified x coord
    pub fn contains_x(&self, x: usize) -> bool {
        x >= self.x && x < self.max_x()
    }

    /// Returns true if the rect contains the specified y coord
    pub fn contains_y(&self, y: usize) -> bool {
        y >= self.y && y < self.max_y()
    }

    /// Returns a new rectangle which is the intersection of the
    /// current rectangle and another rectangle.
    /// If the two rectangles do not overlap, `x` and `y` are
    /// undefined, but the `width` and `height` will be `0`.
    pub fn intersect(&self, other: &Self) -> Self {
        BidiRect {
            x: max(self.x, other.x),
            y: max(self.y, other.y),
            width: min(self.max_x(), other.max_x()).saturating_sub(max(self.x, other.x)),
            height: min(self.max_y(), other.max_y()).saturating_sub(max(self.y, other.y)),
        }
    }

    pub fn offset(&self, dx: isize, dy: isize) -> BidiRectSigned {
        BidiRectSigned {
            x: (self.x as isize) + dx,
            y: (self.y as isize) + dy,
            width: self.width,
            height: self.height,
        }
    }

    pub fn contains_signed(&self, x: isize, y: isize) -> bool {
        if x < 0 || y < 0 {
            false
        } else {
            self.contains(x as usize, y as usize)
        }
    }
}
