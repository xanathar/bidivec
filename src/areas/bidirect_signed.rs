use std::cmp::min;
use std::ops::Range;

/// A simple data structure representing a bidimensional rectangle
/// with a signed coordinate, used to express ranges, areas and
/// rects in bidimensional data structures with an offset applied.
/// For an example, see [`Iter::on_border()`][`crate::bidiiter::Iter::on_border`].
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BidiRectSigned {
    /// The left-most x coordinate of the rectangle (i.e. the minimum x value)
    pub x: isize,
    /// The top-most y coordinate of the rectangle (i.e. the minimum y value)
    pub y: isize,
    /// The width of the rectangle
    pub width: usize,
    /// The height of the rectangle
    pub height: usize,
}

impl BidiRectSigned {
    /// Creates a new rectangle
    pub fn new(x: isize, y: isize, width: usize, height: usize) -> Self {
        BidiRectSigned {
            x,
            y,
            width,
            height,
        }
    }

    /// Returns the coordinate of the top-left corner
    pub fn min_xy(&self) -> (isize, isize) {
        (self.x, self.y)
    }

    /// Returns the coordinate of one more than the bottom-right corner
    /// (i.e. the bottom-right corner, non-inclusive)
    pub fn max_xy(&self) -> (isize, isize) {
        (
            self.x + (self.width as isize),
            self.y + (self.height as isize),
        )
    }

    /// Returns the range of x coordinates (i.e. every `min_x <= x < max_x`)
    pub fn x_range(&self) -> Range<isize> {
        self.x..(self.x + (self.width as isize))
    }

    /// Returns the range of y coordinates (i.e. every `min_y <= y < max_y`)
    pub fn y_range(&self) -> Range<isize> {
        self.y..(self.y + (self.height as isize))
    }

    /// Returns the range of x coordinates clipped to a fence value
    /// (i.e. every `min_x <= x < min(max_x, fence_val)`)
    pub fn x_range_clip(&self, fence_val: isize) -> Range<isize> {
        self.x..min(fence_val, self.x + (self.width as isize))
    }

    /// Returns the range of y coordinates clipped to a fence value
    /// (i.e. every `min_y <= y < min(max_y, fence_val)`)
    pub fn y_range_clip(&self, fence_val: isize) -> Range<isize> {
        self.y..min(fence_val, self.y + (self.height as isize))
    }

    /// The left-most x coordinate of the rectangle (i.e. the minimum x value)
    pub fn min_x(&self) -> isize {
        self.x
    }

    /// The top-most y coordinate of the rectangle (i.e. the minimum y value)
    pub fn min_y(&self) -> isize {
        self.y
    }

    /// An x coordinate one to the right of the right-most coordinate of the
    /// rectangle
    pub fn max_x(&self) -> isize {
        self.x + (self.width as isize)
    }

    /// A y coordinate one below of the bottom-most coordinate of the
    /// rectangle
    pub fn max_y(&self) -> isize {
        self.y + (self.height as isize)
    }

    /// Returns true if the rect contains the specified point
    pub fn contains(&self, x: isize, y: isize) -> bool {
        x >= self.x && x < self.max_x() && y >= self.y && y < self.max_y()
    }

    /// Returns true if the rect contains the specified x coord
    pub fn contains_x(&self, x: isize) -> bool {
        x >= self.x && x < self.max_x()
    }

    /// Returns true if the rect contains the specified y coord
    pub fn contains_y(&self, y: isize) -> bool {
        y >= self.y && y < self.max_y()
    }
}
