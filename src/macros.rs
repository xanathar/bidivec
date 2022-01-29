#[cfg(doc)]
use crate::*;

/// Creates a [`BidiVec`][crate::BidiVec]  containing the arguments.
///
/// `bidivec!` allows a [`BidiVec`] to be defined with a syntax somewhat
/// similar to array expressions and [`vec!`] macro invocations.
///
/// There are three forms of this macro:
///
/// - Create a [`BidiVec`][crate::BidiVec] from a series of comma
///   separated rows (from Rust 1.53 onward):
///
/// ```
/// # use bidivec::bidivec;
/// let v = bidivec!{
///     [1, 2],
///     [3, 4],
/// };
///
/// assert_eq!(v.width(), 2);
/// assert_eq!(v.height(), 2);
/// assert_eq!(v[(0, 0)], 1);
/// assert_eq!(v[(1, 0)], 2);
/// assert_eq!(v[(0, 1)], 3);
/// assert_eq!(v[(1, 1)], 4);
/// ```
///
/// - Create a [`BidiVec`][crate::BidiVec] containing a given list of elements, given a width:
///
/// ```
/// // Note that the final 2 is the width, not the total size
/// # use bidivec::bidivec;
/// let v = bidivec!(1, 2, 3, 4; 2);
///
/// assert_eq!(v.width(), 2);
/// assert_eq!(v.height(), 2);
/// assert_eq!(v[(0, 0)], 1);
/// assert_eq!(v[(1, 0)], 2);
/// assert_eq!(v[(0, 1)], 3);
/// assert_eq!(v[(1, 1)], 4);
/// ```
///
/// - Create a [`BidiVec`][crate::BidiVec] from a given element, width and height:
///
/// ```
/// # use bidivec::bidivec;
/// let v = bidivec![1; 2, 2];
///
/// assert_eq!(v.width(), 2);
/// assert_eq!(v.height(), 2);
/// assert_eq!(v[(0, 0)], 1);
/// assert_eq!(v[(1, 0)], 1);
/// assert_eq!(v[(0, 1)], 1);
/// assert_eq!(v[(1, 1)], 1);
/// ```
///
/// Note that unlike array expressions this syntax supports all elements
/// which implement [`Clone`] and the width and height don't have to be
/// constants.
///
/// This will use [`Clone`] to duplicate an expression, so one should be careful
/// using this with types having a nonstandard [`Clone`] implementation. For
/// example, `bidivec![Rc::new(1); 5, 5]` will create a vector of twentyfive references
/// to the same boxed integer value, not twentyfive references pointing to independently
/// boxed integers.
#[macro_export]
macro_rules! bidivec {
    () => (
        $crate::BidiVec::new()
    );
    ($elem:expr; $w:expr, $h:expr) => (
        $crate::BidiVec::with_elem($elem, $w, $h)
    );
    ($($x:expr),+ $(,)?; $w:expr) => (
        $crate::BidiVec::from_vec(vec![$($x),+], $w).unwrap()
    );
    ($([$($x:expr),+ $(,)?]),+ $(,)?) => ({
        let mut bv = $crate::BidiVec::new();
        $(
            bv.push_row(vec![$($x),+]).unwrap();
        )+
        bv
    });
}

/// Creates a [`BidiGrowVec`][crate::BidiGrowVec] containing the arguments.
///
/// `bidigrowvec!` allows a [`BidiGrowVec`] to be defined with a syntax somewhat
/// similar to array expressions and [`vec!`] macro invocations.
///
/// There are three forms of this macro:
///
/// - Create a [`BidiGrowVec`][crate::BidiGrowVec] from a series of comma
///   separated rows (from Rust 1.53 onward):
///
/// ```
/// # use bidivec::bidigrowvec;
/// let v = bidigrowvec!{
///     [1, 2],
///     [3, 4]
/// };
///
/// assert_eq!(v.width(), 2);
/// assert_eq!(v.height(), 2);
/// assert_eq!(v[(0, 0)], 1);
/// assert_eq!(v[(1, 0)], 2);
/// assert_eq!(v[(0, 1)], 3);
/// assert_eq!(v[(1, 1)], 4);
/// ```
///
/// - Create a [`BidiGrowVec`][crate::BidiGrowVec] containing a given list of elements, given a width:
///
/// ```
/// // Note that the final 2 is the width, not the total size
/// # use bidivec::bidigrowvec;
/// let v = bidigrowvec!(1, 2, 3, 4; 2);
///
/// assert_eq!(v.width(), 2);
/// assert_eq!(v.height(), 2);
/// assert_eq!(v[(0, 0)], 1);
/// assert_eq!(v[(1, 0)], 2);
/// assert_eq!(v[(0, 1)], 3);
/// assert_eq!(v[(1, 1)], 4);
/// ```
///
/// - Create a [`BidiGrowVec`][crate::BidiGrowVec] from a given element, width and height:
///
/// ```
/// # use bidivec::bidigrowvec;
/// let v = bidigrowvec![1; 2, 2];
///
/// assert_eq!(v.width(), 2);
/// assert_eq!(v.height(), 2);
/// assert_eq!(v[(0, 0)], 1);
/// assert_eq!(v[(1, 0)], 1);
/// assert_eq!(v[(0, 1)], 1);
/// assert_eq!(v[(1, 1)], 1);
/// ```
///
/// Note that unlike array expressions this syntax supports all elements
/// which implement [`Clone`] and the width and height don't have to be
/// constants.
///
/// This will use [`Clone`] to duplicate an expression, so one should be careful
/// using this with types having a nonstandard [`Clone`] implementation. For
/// example, `bidigrowvec![Rc::new(1); 5, 5]` will create a vector of twentyfive references
/// to the same boxed integer value, not twentyfive references pointing to independently
/// boxed integers.
#[macro_export]
macro_rules! bidigrowvec {
    () => (
        $crate::BidiGrowVec::new()
    );
    ($elem:expr; $w:expr, $h:expr) => (
        $crate::BidiGrowVec::with_elem($elem, $w, $h)
    );
    ($($x:expr),+ $(,)?; $w:expr) => (
        $crate::BidiGrowVec::from_vec(vec![$($x),+], $w).unwrap()
    );
    ($([$($x:expr),+ $(,)?]),+ $(,)?) => ({
        let mut bv = $crate::BidiGrowVec::new();
        $(
            bv.push_row(vec![$($x),+]).unwrap();
        )+
        bv
    });
}

/// Creates a [`BidiArray`][crate::BidiArray] containing the arguments.
///
/// `bidiarray!` allows a [`BidiArray`] to be defined with a syntax somewhat
/// similar to array expressions and [`vec!`] macro invocations.
///
/// There are three forms of this macro:
///
/// - Create a [`BidiArray`][crate::BidiArray] from a series of comma
///   separated rows (from Rust 1.53 onward):
///
/// ```
/// # use bidivec::bidiarray;
/// let v = bidiarray!{
///     [1, 2],
///     [3, 4]
/// };
///
/// assert_eq!(v.width(), 2);
/// assert_eq!(v.height(), 2);
/// assert_eq!(v[(0, 0)], 1);
/// assert_eq!(v[(1, 0)], 2);
/// assert_eq!(v[(0, 1)], 3);
/// assert_eq!(v[(1, 1)], 4);
/// ```
///
/// - Create a [`BidiArray`][crate::BidiArray] containing a given list of elements, given a width:
///
/// ```
/// // Note that the final 2 is the width, not the total size
/// # use bidivec::bidiarray;
/// let v = bidiarray!(1, 2, 3, 4; 2);
///
/// assert_eq!(v.width(), 2);
/// assert_eq!(v.height(), 2);
/// assert_eq!(v[(0, 0)], 1);
/// assert_eq!(v[(1, 0)], 2);
/// assert_eq!(v[(0, 1)], 3);
/// assert_eq!(v[(1, 1)], 4);
/// ```
///
/// - Create a [`BidiArray`][crate::BidiArray] from a given element, width and height:
///
/// ```
/// # use bidivec::bidiarray;
/// let v = bidiarray![1; 2, 2];
///
/// assert_eq!(v.width(), 2);
/// assert_eq!(v.height(), 2);
/// assert_eq!(v[(0, 0)], 1);
/// assert_eq!(v[(1, 0)], 1);
/// assert_eq!(v[(0, 1)], 1);
/// assert_eq!(v[(1, 1)], 1);
/// ```
///
/// Note that unlike array expressions this syntax supports all elements
/// which implement [`Clone`] and the width and height don't have to be
/// constants.
///
/// This will use [`Clone`] to duplicate an expression, so one should be careful
/// using this with types having a nonstandard [`Clone`] implementation. For
/// example, `bidiarray![Rc::new(1); 5, 5]` will create a vector of twentyfive references
/// to the same boxed integer value, not twentyfive references pointing to independently
/// boxed integers.
#[macro_export]
macro_rules! bidiarray {
    () => (
        $crate::BidiArray::new()
    );
    ($elem:expr; $w:expr, $h:expr) => (
        $crate::BidiArray::with_elem($elem, $w, $h)
    );
    ($($x:expr),+ $(,)?; $w:expr) => (
        $crate::BidiArray::from_vec(vec![$($x),+], $w).unwrap()
    );
    ($([$($x:expr),+ $(,)?]),+ $(,)?) => ({
        let mut bv = $crate::BidiVec::new();
        $(
            bv.push_row(vec![$($x),+]).unwrap();
        )+
        bv.into_bidiarray()
    });
}
