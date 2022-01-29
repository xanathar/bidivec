use thiserror::Error;

/// The error type for operations on bidimensional data structures
/// in the crate.
#[derive(Error, Copy, Clone, Debug, PartialEq)]
pub enum BidiError {
    /// The size of an argument is not compatible with the current
    /// width or height of the data structure.
    ///
    /// For example, this happens when trying to add a row which is
    /// longer than the current width of the data structure.
    #[error("incompatible argument size")]
    IncompatibleSize,
    /// The argument would access the data structure outside of its
    /// own boundaries.
    ///
    /// For example, this happens when trying to insert a row at an
    /// index which is greater than the current height of the data
    /// structure.
    #[error("coordinates out of bounds")]
    OutOfBounds,
}
