use super::test_types::*;
use super::*;
use crate::run_test_on_types;

fn helper_build_3x3<T: Testable>() -> BidiArray<T> {
    let mut v = BidiVec::<T>::new();
    v.push_row([T::new(0), T::new(1), T::new(2)])
        .expect("helper_build_3x3 - push 1");
    v.push_row([T::new(3), T::new(4), T::new(5)])
        .expect("helper_build_3x3 - push 2");
    v.push_row([T::new(6), T::new(7), T::new(8)])
        .expect("helper_build_3x3 - push 3");
    v.into_bidiarray()
}

fn helper_build_5x3<T: Testable>() -> BidiArray<T> {
    bidiarray! {
        [T::new(11), T::new(12), T::new(13), T::new(14), T::new(15)],
        [T::new(21), T::new(22), T::new(23), T::new(24), T::new(25)],
        [T::new(31), T::new(32), T::new(33), T::new(34), T::new(35)],
    }
}

fn helper_build_4x5<T: Testable>() -> BidiArray<T> {
    bidiarray! {
        [T::new(11), T::new(12), T::new(13), T::new(14)],
        [T::new(21), T::new(22), T::new(23), T::new(24)],
        [T::new(31), T::new(32), T::new(33), T::new(34)],
        [T::new(41), T::new(42), T::new(43), T::new(44)],
        [T::new(51), T::new(52), T::new(53), T::new(54)],
    }
}

fn helper_build_10x10<T: Testable>() -> BidiArray<T> {
    bidiarray! {
        [T::new(00), T::new(01), T::new(02), T::new(03), T::new(04), T::new(05), T::new(06), T::new(07), T::new(08), T::new(09)],
        [T::new(10), T::new(11), T::new(12), T::new(13), T::new(14), T::new(15), T::new(16), T::new(17), T::new(18), T::new(19)],
        [T::new(20), T::new(21), T::new(22), T::new(23), T::new(24), T::new(25), T::new(26), T::new(27), T::new(28), T::new(29)],
        [T::new(30), T::new(31), T::new(32), T::new(33), T::new(34), T::new(35), T::new(36), T::new(37), T::new(38), T::new(39)],
        [T::new(40), T::new(41), T::new(42), T::new(43), T::new(44), T::new(45), T::new(46), T::new(47), T::new(48), T::new(49)],
        [T::new(50), T::new(51), T::new(52), T::new(53), T::new(54), T::new(55), T::new(56), T::new(57), T::new(58), T::new(59)],
        [T::new(60), T::new(61), T::new(62), T::new(63), T::new(64), T::new(65), T::new(66), T::new(67), T::new(68), T::new(69)],
        [T::new(70), T::new(71), T::new(72), T::new(73), T::new(74), T::new(75), T::new(76), T::new(77), T::new(78), T::new(79)],
        [T::new(80), T::new(81), T::new(82), T::new(83), T::new(84), T::new(85), T::new(86), T::new(87), T::new(88), T::new(89)],
        [T::new(90), T::new(91), T::new(92), T::new(93), T::new(94), T::new(95), T::new(96), T::new(97), T::new(98), T::new(99)],
    }
}

fn helper_build_5x1<T: Testable>() -> BidiArray<T> {
    bidiarray! {
        [T::new(11), T::new(12), T::new(13), T::new(14), T::new(15)],
    }
}

fn helper_build_1x5<T: Testable>() -> BidiArray<T> {
    bidiarray! {
        [T::new(11)],
        [T::new(21)],
        [T::new(31)],
        [T::new(41)],
        [T::new(51)],
    }
}

fn assert_layout<T: Testable>(
    v: BidiArray<T>,
    width: usize,
    height: usize,
    items: Vec<i32>,
) -> BidiArray<T> {
    const COLOR_RESTORE: &str = "\x1b[0m";
    const COLOR_ALERT: &str = "\x1b[1;31m";

    fn alert_color(v: bool) -> &'static str {
        if v {
            COLOR_ALERT
        } else {
            COLOR_RESTORE
        }
    }

    let vwidth = v.width();
    let vheight = v.height();

    let width_differs = vwidth != width;
    let height_differs = vheight != height;

    let v = v.into_vec();

    let len_differs = v.len() != items.len();

    let mut items_differ = false;
    for (i, j) in v.iter().zip(items.iter()) {
        items_differ |= i.id() != *j;
    }

    if items_differ || len_differs || height_differs || width_differs {
        println!("BidiArrays are different:");
        println!(
            "{}           Width: found {} expected {}{}",
            alert_color(width_differs),
            vwidth,
            width,
            COLOR_RESTORE
        );
        println!(
            "{}          Height: found {} expected {}{}",
            alert_color(height_differs),
            vheight,
            height,
            COLOR_RESTORE
        );
        println!(
            "{}             Len: found {} expected {}{}",
            alert_color(len_differs),
            v.len(),
            items.len(),
            COLOR_RESTORE
        );
        println!(
            "{}     Found items: {:?}{}",
            alert_color(items_differ),
            v.iter().map(|v| v.id()).collect::<Vec<i32>>(),
            COLOR_RESTORE
        );
        println!(
            "{}  Expected items: {:?}{}",
            alert_color(items_differ),
            items,
            COLOR_RESTORE
        );
        println!();
        panic!();
    }

    BidiArray::<T>::from_vec(v, vwidth).unwrap()
}

// ==================================================
// Tests for slicing
// ==================================================

#[test]
fn slice_indexing_full() {
    let v = BidiArray::with_size_func_xy(3, 3, |x, y| x + y * 3);
    assert_eq!(v.as_slice(..), &[0usize, 1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn slice_indexing_partial() {
    let v = BidiArray::with_size_func_xy(3, 3, |x, y| x + y * 3);
    assert_eq!(v.as_slice(1..8), &[1usize, 2, 3, 4, 5, 6, 7]);
}

// ==================================================
// Tests for transpose
// ==================================================

run_test_on_types!(transpose_3x3 on all);
fn transpose_3x3<T: Testable>() {
    let mut v = helper_build_3x3();
    v.transpose();
    assert_layout::<T>(v, 3, 3, vec![0, 3, 6, 1, 4, 7, 2, 5, 8]);
}

run_test_on_types!(transpose_5x3 on all);
fn transpose_5x3<T: Testable>() {
    let mut v = helper_build_5x3();
    v.transpose();
    assert_layout::<T>(
        v,
        3,
        5,
        vec![11, 21, 31, 12, 22, 32, 13, 23, 33, 14, 24, 34, 15, 25, 35],
    );
}

run_test_on_types!(transpose_4x5 on all);
fn transpose_4x5<T: Testable>() {
    let mut v = helper_build_4x5();
    v.transpose();
    assert_layout::<T>(
        v,
        5,
        4,
        vec![
            11, 21, 31, 41, 51, 12, 22, 32, 42, 52, 13, 23, 33, 43, 53, 14, 24, 34, 44, 54,
        ],
    );
}

run_test_on_types!(transpose_5x1 on all);
fn transpose_5x1<T: Testable>() {
    let mut v = helper_build_5x1();
    v.transpose();
    assert_layout::<T>(v, 1, 5, vec![11, 12, 13, 14, 15]);
}

run_test_on_types!(transpose_1x5 on all);
fn transpose_1x5<T: Testable>() {
    let mut v = helper_build_1x5();
    v.transpose();
    assert_layout::<T>(v, 5, 1, vec![11, 21, 31, 41, 51]);
}

// ==================================================
// Tests for rotate90ccw
// ==================================================

run_test_on_types!(rotate90ccw_3x3 on all);
fn rotate90ccw_3x3<T: Testable>() {
    let mut v = helper_build_3x3();
    v.rotate90ccw();
    assert_layout::<T>(v, 3, 3, vec![2, 5, 8, 1, 4, 7, 0, 3, 6]);
}

run_test_on_types!(rotate90ccw_5x3 on all);
fn rotate90ccw_5x3<T: Testable>() {
    let mut v = helper_build_5x3();
    v.rotate90ccw();
    assert_layout::<T>(
        v,
        3,
        5,
        vec![15, 25, 35, 14, 24, 34, 13, 23, 33, 12, 22, 32, 11, 21, 31],
    );
}

run_test_on_types!(rotate90ccw_4x5 on all);
fn rotate90ccw_4x5<T: Testable>() {
    let mut v = helper_build_4x5();
    v.rotate90ccw();
    assert_layout::<T>(
        v,
        5,
        4,
        vec![
            14, 24, 34, 44, 54, 13, 23, 33, 43, 53, 12, 22, 32, 42, 52, 11, 21, 31, 41, 51,
        ],
    );
}

run_test_on_types!(rotate90ccw_5x1 on all);
fn rotate90ccw_5x1<T: Testable>() {
    let mut v = helper_build_5x1();
    v.rotate90ccw();
    assert_layout::<T>(v, 1, 5, vec![15, 14, 13, 12, 11]);
}

run_test_on_types!(rotate90ccw_1x5 on all);
fn rotate90ccw_1x5<T: Testable>() {
    let mut v = helper_build_1x5();
    v.rotate90ccw();
    assert_layout::<T>(v, 5, 1, vec![11, 21, 31, 41, 51]);
}

// ==================================================
// Tests for rotate270ccw
// ==================================================

run_test_on_types!(rotate270ccw_3x3 on all);
fn rotate270ccw_3x3<T: Testable>() {
    let mut v = helper_build_3x3();
    v.rotate270ccw();
    assert_layout::<T>(v, 3, 3, vec![6, 3, 0, 7, 4, 1, 8, 5, 2]);
}

run_test_on_types!(rotate270ccw_5x3 on all);
fn rotate270ccw_5x3<T: Testable>() {
    let mut v = helper_build_5x3();
    v.rotate270ccw();
    assert_layout::<T>(
        v,
        3,
        5,
        vec![31, 21, 11, 32, 22, 12, 33, 23, 13, 34, 24, 14, 35, 25, 15],
    );
}

run_test_on_types!(rotate270ccw_4x5 on all);
fn rotate270ccw_4x5<T: Testable>() {
    let mut v = helper_build_4x5();
    v.rotate270ccw();
    assert_layout::<T>(
        v,
        5,
        4,
        vec![
            51, 41, 31, 21, 11, 52, 42, 32, 22, 12, 53, 43, 33, 23, 13, 54, 44, 34, 24, 14,
        ],
    );
}

run_test_on_types!(rotate270ccw_5x1 on all);
fn rotate270ccw_5x1<T: Testable>() {
    let mut v = helper_build_5x1();
    v.rotate270ccw();
    assert_layout::<T>(v, 1, 5, vec![11, 12, 13, 14, 15]);
}

run_test_on_types!(rotate270ccw_1x5 on all);
fn rotate270ccw_1x5<T: Testable>() {
    let mut v = helper_build_1x5();
    v.rotate270ccw();
    assert_layout::<T>(v, 5, 1, vec![51, 41, 31, 21, 11]);
}

// ==================================================
// Tests for rotate180
// ==================================================

run_test_on_types!(rotate180_3x3 on all);
fn rotate180_3x3<T: Testable>() {
    let mut v = helper_build_3x3();
    v.rotate180();
    assert_layout::<T>(v, 3, 3, vec![8, 7, 6, 5, 4, 3, 2, 1, 0]);
}

run_test_on_types!(rotate180_5x3 on all);
fn rotate180_5x3<T: Testable>() {
    let mut v = helper_build_5x3();
    v.rotate180();
    assert_layout::<T>(
        v,
        5,
        3,
        vec![35, 34, 33, 32, 31, 25, 24, 23, 22, 21, 15, 14, 13, 12, 11],
    );
}

run_test_on_types!(rotate180_4x5 on all);
fn rotate180_4x5<T: Testable>() {
    let mut v = helper_build_4x5();
    v.rotate180();
    assert_layout::<T>(
        v,
        4,
        5,
        vec![
            54, 53, 52, 51, 44, 43, 42, 41, 34, 33, 32, 31, 24, 23, 22, 21, 14, 13, 12, 11,
        ],
    );
}

run_test_on_types!(rotate180_5x1 on all);
fn rotate180_5x1<T: Testable>() {
    let mut v = helper_build_5x1();
    v.rotate180();
    assert_layout::<T>(v, 5, 1, vec![15, 14, 13, 12, 11]);
}

run_test_on_types!(rotate180_1x5 on all);
fn rotate180_1x5<T: Testable>() {
    let mut v = helper_build_1x5();
    v.rotate180();
    assert_layout::<T>(v, 1, 5, vec![51, 41, 31, 21, 11]);
}

// ==================================================
// Tests for reverse_columns
// ==================================================

run_test_on_types!(reverse_columns_3x3 on all);
fn reverse_columns_3x3<T: Testable>() {
    let mut v = helper_build_3x3();
    v.reverse_columns();
    assert_layout::<T>(v, 3, 3, vec![6, 7, 8, 3, 4, 5, 0, 1, 2]);
}

run_test_on_types!(reverse_columns_5x3 on all);
fn reverse_columns_5x3<T: Testable>() {
    let mut v = helper_build_5x3();
    v.reverse_columns();
    assert_layout::<T>(
        v,
        5,
        3,
        vec![31, 32, 33, 34, 35, 21, 22, 23, 24, 25, 11, 12, 13, 14, 15],
    );
}

run_test_on_types!(reverse_columns_4x5 on all);
fn reverse_columns_4x5<T: Testable>() {
    let mut v = helper_build_4x5();
    v.reverse_columns();
    assert_layout::<T>(
        v,
        4,
        5,
        vec![
            51, 52, 53, 54, 41, 42, 43, 44, 31, 32, 33, 34, 21, 22, 23, 24, 11, 12, 13, 14,
        ],
    );
}

run_test_on_types!(reverse_columns_5x1 on all);
fn reverse_columns_5x1<T: Testable>() {
    let mut v = helper_build_5x1();
    v.reverse_columns();
    assert_layout::<T>(v, 5, 1, vec![11, 12, 13, 14, 15]);
}

run_test_on_types!(reverse_columns_1x5 on all);
fn reverse_columns_1x5<T: Testable>() {
    let mut v = helper_build_1x5();
    v.reverse_columns();
    assert_layout::<T>(v, 1, 5, vec![51, 41, 31, 21, 11]);
}

// ==================================================
// Tests for reverse_rows
// ==================================================

run_test_on_types!(reverse_rows_3x3 on all);
fn reverse_rows_3x3<T: Testable>() {
    let mut v = helper_build_3x3();
    v.reverse_rows();
    assert_layout::<T>(v, 3, 3, vec![2, 1, 0, 5, 4, 3, 8, 7, 6]);
}

run_test_on_types!(reverse_rows_5x3 on all);
fn reverse_rows_5x3<T: Testable>() {
    let mut v = helper_build_5x3();
    v.reverse_rows();
    assert_layout::<T>(
        v,
        5,
        3,
        vec![15, 14, 13, 12, 11, 25, 24, 23, 22, 21, 35, 34, 33, 32, 31],
    );
}

run_test_on_types!(reverse_rows_4x5 on all);
fn reverse_rows_4x5<T: Testable>() {
    let mut v = helper_build_4x5();
    v.reverse_rows();
    assert_layout::<T>(
        v,
        4,
        5,
        vec![
            14, 13, 12, 11, 24, 23, 22, 21, 34, 33, 32, 31, 44, 43, 42, 41, 54, 53, 52, 51,
        ],
    );
}

run_test_on_types!(reverse_rows_5x1 on all);
fn reverse_rows_5x1<T: Testable>() {
    let mut v = helper_build_5x1();
    v.reverse_rows();
    assert_layout::<T>(v, 5, 1, vec![15, 14, 13, 12, 11]);
}

run_test_on_types!(reverse_rows_1x5 on all);
fn reverse_rows_1x5<T: Testable>() {
    let mut v = helper_build_1x5();
    v.reverse_rows();
    assert_layout::<T>(v, 1, 5, vec![11, 21, 31, 41, 51]);
}

// ==================================================
// Tests for bidiview interop
// ==================================================

run_test_on_types!(bidiview_interop_basics on all);
fn bidiview_interop_basics<T: Testable>() {
    let v1 = helper_build_10x10::<T>();
    let v2 = helper_build_10x10::<T>();

    assert_view_eq_views(&v1, &v2);

    let v3 = BidiArray::from_view(v1).unwrap();
    assert_view_eq_views(&v2, &v3);
}

run_test_on_types!(bidiview_interop_clone on clonables);
fn bidiview_interop_clone<T: Testable + Clone>() {
    let v1 = helper_build_10x10::<T>();
    let v2 = BidiArray::from_view(&v1 as &dyn BidiView<Output = T>).unwrap();
    assert_view_eq_views(&v1, &v2);
}

run_test_on_types!(bidiview_interop_cut1 on clonables);
fn bidiview_interop_cut1<T: Testable + Clone>() {
    let v1 = helper_build_10x10::<T>();
    let cut = BidiRect::new(0, 0, 3, 3);
    let v2 = BidiArray::from_view_cut(&v1 as &dyn BidiView<Output = T>, &cut).unwrap();
    assert_view_eq_views(
        &bidiarray! {
            [0, 1, 2],
            [10, 11, 12],
            [20, 21, 22],
        },
        &v2,
    );
}

run_test_on_types!(bidiview_interop_cut2 on all);
fn bidiview_interop_cut2<T: Testable>() {
    let v1 = helper_build_10x10::<T>();
    let cut = BidiRect::new(0, 0, 3, 3);
    let v2 = BidiArray::from_view_cut(v1, &cut).unwrap();
    assert_view_eq_views(
        &bidiarray! {
            [0, 1, 2],
            [10, 11, 12],
            [20, 21, 22],
        },
        &v2,
    );
}

// ==================================================
// Tests for iterators
// ==================================================

run_test_on_types!(iterator_basic on all);
fn iterator_basic<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let v = b.iter().map(|t| t.id()).collect::<Vec<i32>>();

    assert_eq!(
        v,
        vec![11, 12, 13, 14, 21, 22, 23, 24, 31, 32, 33, 34, 41, 42, 43, 44, 51, 52, 53, 54]
    );
}

run_test_on_types!(iterator_basic_by_col on all);
fn iterator_basic_by_col<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let v = b.iter().by_column().map(|t| t.id()).collect::<Vec<i32>>();

    assert_eq!(
        v,
        vec![11, 21, 31, 41, 51, 12, 22, 32, 42, 52, 13, 23, 33, 43, 53, 14, 24, 34, 44, 54,]
    );
}

run_test_on_types!(iterator_basic_on_row on all);
fn iterator_basic_on_row<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let v = b.iter().on_row(3).map(|t| t.id()).collect::<Vec<i32>>();

    assert_eq!(v, vec![41, 42, 43, 44,]);
}

run_test_on_types!(iterator_basic_on_row_out_of_range on all);
fn iterator_basic_on_row_out_of_range<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let v = b.iter().on_row(5).map(|t| t.id()).collect::<Vec<i32>>();

    assert_eq!(v, vec![]);
}

run_test_on_types!(iterator_basic_on_col_out_of_range on all);
fn iterator_basic_on_col_out_of_range<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let v = b.iter().on_column(4).map(|t| t.id()).collect::<Vec<i32>>();

    assert_eq!(v, vec![]);
}

run_test_on_types!(iterator_basic_on_col on all);
fn iterator_basic_on_col<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let v = b.iter().on_column(1).map(|t| t.id()).collect::<Vec<i32>>();

    assert_eq!(v, vec![12, 22, 32, 42, 52,]);
}

run_test_on_types!(iterator_basic_on_rect on all);
fn iterator_basic_on_rect<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let v = b
        .iter()
        .on_rect(&BidiRect::new(1, 1, 3, 3))
        .map(|t| t.id())
        .collect::<Vec<i32>>();

    assert_eq!(v, vec![22, 23, 24, 32, 33, 34, 42, 43, 44,]);
}

run_test_on_types!(iterator_basic_on_rect_reversed on all);
fn iterator_basic_on_rect_reversed<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let v = b
        .iter()
        .by_column()
        .on_rect(&BidiRect::new(1, 1, 3, 3))
        .map(|t| t.id())
        .collect::<Vec<i32>>();

    assert_eq!(v, vec![22, 32, 42, 23, 33, 43, 24, 34, 44,]);
}

run_test_on_types!(iterator_basic_on_reversed_rect on all);
fn iterator_basic_on_reversed_rect<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let v = b
        .iter()
        .on_rect(&BidiRect::new(1, 1, 3, 3))
        .by_column()
        .map(|t| t.id())
        .collect::<Vec<i32>>();

    assert_eq!(v, vec![22, 32, 42, 23, 33, 43, 24, 34, 44,]);
}

run_test_on_types!(iterator_basic_with_xy_before on all);
fn iterator_basic_with_xy_before<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let v = b
        .iter()
        .with_coords()
        .map(|(x, y, t)| t.id() + ((x + 1) as i32) * 100 + ((y + 1) as i32) * 1000)
        .collect::<Vec<i32>>();

    assert_eq!(
        v,
        vec![
            1111, 1212, 1313, 1414, 2121, 2222, 2323, 2424, 3131, 3232, 3333, 3434, 4141, 4242,
            4343, 4444, 5151, 5252, 5353, 5454,
        ]
    );
}

run_test_on_types!(iterator_basic_by_col_with_xy_before on all);
fn iterator_basic_by_col_with_xy_before<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let v = b
        .iter()
        .with_coords()
        .by_column()
        .map(|(x, y, t)| t.id() + ((x + 1) as i32) * 100 + ((y + 1) as i32) * 1000)
        .collect::<Vec<i32>>();

    assert_eq!(
        v,
        vec![
            1111, 2121, 3131, 4141, 5151, 1212, 2222, 3232, 4242, 5252, 1313, 2323, 3333, 4343,
            5353, 1414, 2424, 3434, 4444, 5454,
        ]
    );
}

run_test_on_types!(iterator_basic_on_row_with_xy_before on all);
fn iterator_basic_on_row_with_xy_before<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let v = b
        .iter()
        .with_coords()
        .on_row(3)
        .map(|(x, y, t)| t.id() + ((x + 1) as i32) * 100 + ((y + 1) as i32) * 1000)
        .collect::<Vec<i32>>();

    assert_eq!(v, vec![4141, 4242, 4343, 4444,]);
}

run_test_on_types!(iterator_basic_on_row_out_of_range_with_xy_before on all);
fn iterator_basic_on_row_out_of_range_with_xy_before<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let v = b
        .iter()
        .with_coords()
        .on_row(5)
        .map(|(x, y, t)| t.id() + ((x + 1) as i32) * 100 + ((y + 1) as i32) * 1000)
        .collect::<Vec<i32>>();

    assert_eq!(v, vec![]);
}

run_test_on_types!(iterator_basic_on_col_out_of_range_with_xy_before on all);
fn iterator_basic_on_col_out_of_range_with_xy_before<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let v = b
        .iter()
        .with_coords()
        .on_column(4)
        .map(|(x, y, t)| t.id() + ((x + 1) as i32) * 100 + ((y + 1) as i32) * 1000)
        .collect::<Vec<i32>>();

    assert_eq!(v, vec![]);
}

run_test_on_types!(iterator_basic_on_col_with_xy_before on all);
fn iterator_basic_on_col_with_xy_before<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let v = b
        .iter()
        .with_coords()
        .on_column(1)
        .map(|(x, y, t)| t.id() + ((x + 1) as i32) * 100 + ((y + 1) as i32) * 1000)
        .collect::<Vec<i32>>();

    assert_eq!(v, vec![1212, 2222, 3232, 4242, 5252,]);
}

run_test_on_types!(iterator_basic_on_rect_with_xy_before on all);
fn iterator_basic_on_rect_with_xy_before<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let v = b
        .iter()
        .with_coords()
        .on_rect(&BidiRect::new(1, 1, 3, 3))
        .map(|(x, y, t)| t.id() + ((x + 1) as i32) * 100 + ((y + 1) as i32) * 1000)
        .collect::<Vec<i32>>();

    assert_eq!(
        v,
        vec![2222, 2323, 2424, 3232, 3333, 3434, 4242, 4343, 4444,]
    );
}

run_test_on_types!(iterator_basic_on_rect_reversed_with_xy_before on all);
fn iterator_basic_on_rect_reversed_with_xy_before<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let v = b
        .iter()
        .with_coords()
        .by_column()
        .on_rect(&BidiRect::new(1, 1, 3, 3))
        .map(|(x, y, t)| t.id() + ((x + 1) as i32) * 100 + ((y + 1) as i32) * 1000)
        .collect::<Vec<i32>>();

    assert_eq!(
        v,
        vec![2222, 3232, 4242, 2323, 3333, 4343, 2424, 3434, 4444,]
    );
}

run_test_on_types!(iterator_basic_on_reversed_rect_with_xy_before on all);
fn iterator_basic_on_reversed_rect_with_xy_before<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let v = b
        .iter()
        .with_coords()
        .on_rect(&BidiRect::new(1, 1, 3, 3))
        .by_column()
        .map(|(x, y, t)| t.id() + ((x + 1) as i32) * 100 + ((y + 1) as i32) * 1000)
        .collect::<Vec<i32>>();

    assert_eq!(
        v,
        vec![2222, 3232, 4242, 2323, 3333, 4343, 2424, 3434, 4444,]
    );
}

run_test_on_types!(iterator_basic_by_col_with_xy_after on all);
fn iterator_basic_by_col_with_xy_after<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let v = b
        .iter()
        .by_column()
        .with_coords()
        .map(|(x, y, t)| t.id() + ((x + 1) as i32) * 100 + ((y + 1) as i32) * 1000)
        .collect::<Vec<i32>>();

    assert_eq!(
        v,
        vec![
            1111, 2121, 3131, 4141, 5151, 1212, 2222, 3232, 4242, 5252, 1313, 2323, 3333, 4343,
            5353, 1414, 2424, 3434, 4444, 5454,
        ]
    );
}

run_test_on_types!(iterator_basic_on_row_with_xy_after on all);
fn iterator_basic_on_row_with_xy_after<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let v = b
        .iter()
        .on_row(3)
        .with_coords()
        .map(|(x, y, t)| t.id() + ((x + 1) as i32) * 100 + ((y + 1) as i32) * 1000)
        .collect::<Vec<i32>>();

    assert_eq!(v, vec![4141, 4242, 4343, 4444,]);
}

run_test_on_types!(iterator_basic_on_row_out_of_range_with_xy_after on all);
fn iterator_basic_on_row_out_of_range_with_xy_after<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let v = b
        .iter()
        .on_row(5)
        .with_coords()
        .map(|(x, y, t)| t.id() + ((x + 1) as i32) * 100 + ((y + 1) as i32) * 1000)
        .collect::<Vec<i32>>();

    assert_eq!(v, vec![]);
}

run_test_on_types!(iterator_basic_on_col_out_of_range_with_xy_after on all);
fn iterator_basic_on_col_out_of_range_with_xy_after<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let v = b
        .iter()
        .on_column(4)
        .with_coords()
        .map(|(x, y, t)| t.id() + ((x + 1) as i32) * 100 + ((y + 1) as i32) * 1000)
        .collect::<Vec<i32>>();

    assert_eq!(v, vec![]);
}

run_test_on_types!(iterator_basic_on_col_with_xy_after on all);
fn iterator_basic_on_col_with_xy_after<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let v = b
        .iter()
        .on_column(1)
        .with_coords()
        .map(|(x, y, t)| t.id() + ((x + 1) as i32) * 100 + ((y + 1) as i32) * 1000)
        .collect::<Vec<i32>>();

    assert_eq!(v, vec![1212, 2222, 3232, 4242, 5252,]);
}

run_test_on_types!(iterator_basic_on_rect_with_xy_after on all);
fn iterator_basic_on_rect_with_xy_after<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let v = b
        .iter()
        .on_rect(&BidiRect::new(1, 1, 3, 3))
        .with_coords()
        .map(|(x, y, t)| t.id() + ((x + 1) as i32) * 100 + ((y + 1) as i32) * 1000)
        .collect::<Vec<i32>>();

    assert_eq!(
        v,
        vec![2222, 2323, 2424, 3232, 3333, 3434, 4242, 4343, 4444,]
    );
}

run_test_on_types!(iterator_basic_on_rect_reversed_with_xy_after on all);
fn iterator_basic_on_rect_reversed_with_xy_after<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let v = b
        .iter()
        .by_column()
        .on_rect(&BidiRect::new(1, 1, 3, 3))
        .with_coords()
        .map(|(x, y, t)| t.id() + ((x + 1) as i32) * 100 + ((y + 1) as i32) * 1000)
        .collect::<Vec<i32>>();

    assert_eq!(
        v,
        vec![2222, 3232, 4242, 2323, 3333, 4343, 2424, 3434, 4444,]
    );
}

run_test_on_types!(iterator_basic_on_reversed_rect_with_xy_after on all);
fn iterator_basic_on_reversed_rect_with_xy_after<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let v = b
        .iter()
        .on_rect(&BidiRect::new(1, 1, 3, 3))
        .by_column()
        .with_coords()
        .map(|(x, y, t)| t.id() + ((x + 1) as i32) * 100 + ((y + 1) as i32) * 1000)
        .collect::<Vec<i32>>();

    assert_eq!(
        v,
        vec![2222, 3232, 4242, 2323, 3333, 4343, 2424, 3434, 4444,]
    );
}

run_test_on_types!(iterator_basic_on_rect_reversed_with_xy_middle on all);
fn iterator_basic_on_rect_reversed_with_xy_middle<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let v = b
        .iter()
        .by_column()
        .with_coords()
        .on_rect(&BidiRect::new(1, 1, 3, 3))
        .map(|(x, y, t)| t.id() + ((x + 1) as i32) * 100 + ((y + 1) as i32) * 1000)
        .collect::<Vec<i32>>();

    assert_eq!(
        v,
        vec![2222, 3232, 4242, 2323, 3333, 4343, 2424, 3434, 4444,]
    );
}

run_test_on_types!(iterator_basic_on_reversed_rect_with_xy_middle on all);
fn iterator_basic_on_reversed_rect_with_xy_middle<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let v = b
        .iter()
        .on_rect(&BidiRect::new(1, 1, 3, 3))
        .with_coords()
        .by_column()
        .map(|(x, y, t)| t.id() + ((x + 1) as i32) * 100 + ((y + 1) as i32) * 1000)
        .collect::<Vec<i32>>();

    assert_eq!(
        v,
        vec![2222, 3232, 4242, 2323, 3333, 4343, 2424, 3434, 4444,]
    );
}

run_test_on_types!(iterator_mutate_rect_xy_middle on all);
fn iterator_mutate_rect_xy_middle<T: Testable>() {
    let mut b = helper_build_4x5::<T>();
    let iter = b
        .iter_mut()
        .on_rect(&BidiRect::new(1, 1, 3, 3))
        .with_coords()
        .by_column();

    for (x, y, t) in iter {
        *t = T::new(-(((x + 1) as i32) * 10 + ((y + 1) as i32)));
    }

    let v = b.iter().by_column().map(|t| t.id()).collect::<Vec<i32>>();

    assert_eq!(
        v,
        vec![
            11, 21, 31, 41, 51, 12, -22, -23, -24, 52, 13, -32, -33, -34, 53, 14, -42, -43, -44,
            54,
        ]
    );
}

run_test_on_types!(iterator_mutate_rect_xy_before on all);
fn iterator_mutate_rect_xy_before<T: Testable>() {
    let mut b = helper_build_4x5::<T>();
    let iter = b
        .iter_mut()
        .with_coords()
        .on_rect(&BidiRect::new(1, 1, 3, 3))
        .by_column();

    for (x, y, t) in iter {
        *t = T::new(-(((x + 1) as i32) * 10 + ((y + 1) as i32)));
    }

    let v = b.iter().by_column().map(|t| t.id()).collect::<Vec<i32>>();

    assert_eq!(
        v,
        vec![
            11, 21, 31, 41, 51, 12, -22, -23, -24, 52, 13, -32, -33, -34, 53, 14, -42, -43, -44,
            54,
        ]
    );
}

run_test_on_types!(iterator_mutate_rect_xy_after on all);
fn iterator_mutate_rect_xy_after<T: Testable>() {
    let mut b = helper_build_4x5::<T>();
    let iter = b
        .iter_mut()
        .on_rect(&BidiRect::new(1, 1, 3, 3))
        .by_column()
        .with_coords();

    for (x, y, t) in iter {
        *t = T::new(-(((x + 1) as i32) * 10 + ((y + 1) as i32)));
    }

    let v = b.iter().by_column().map(|t| t.id()).collect::<Vec<i32>>();

    assert_eq!(
        v,
        vec![
            11, 21, 31, 41, 51, 12, -22, -23, -24, 52, 13, -32, -33, -34, 53, 14, -42, -43, -44,
            54,
        ]
    );
}

run_test_on_types!(iterator_mutate_rect on all);
fn iterator_mutate_rect<T: Testable>() {
    let mut b = helper_build_4x5::<T>();
    let iter = b.iter_mut().on_rect(&BidiRect::new(1, 1, 3, 3)).by_column();

    for t in iter {
        *t = T::new(-t.id());
    }

    let v = b.iter().by_column().map(|t| t.id()).collect::<Vec<i32>>();

    assert_eq!(
        v,
        vec![
            11, 21, 31, 41, 51, 12, -22, -32, -42, 52, 13, -23, -33, -43, 53, 14, -24, -34, -44,
            54,
        ]
    );
}

run_test_on_types!(iterator_mutate_all on all);
fn iterator_mutate_all<T: Testable>() {
    let mut b = helper_build_4x5::<T>();

    for t in b.iter_mut() {
        *t = T::new(-t.id());
    }

    let v = b.iter().by_column().map(|t| t.id()).collect::<Vec<i32>>();

    assert_eq!(
        v,
        vec![
            -11, -21, -31, -41, -51, -12, -22, -32, -42, -52, -13, -23, -33, -43, -53, -14, -24,
            -34, -44, -54,
        ]
    );
}

run_test_on_types!(iterator_border_inside on all);
fn iterator_border_inside<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let v = b
        .iter()
        .on_border(&BidiRectSigned::new(1, 0, 3, 3))
        .map(|t| t.id())
        .collect::<Vec<i32>>();

    assert_eq!(v, vec![12, 13, 14, 24, 34, 33, 32, 22]);
}

run_test_on_types!(iterator_border_outside_1 on all);
fn iterator_border_outside_1<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let v = b
        .iter()
        .on_border(&BidiRectSigned::new(1, 2, 4, 4))
        .map(|t| t.id())
        .collect::<Vec<i32>>();

    assert_eq!(v, vec![32, 33, 34, 52, 42]);
}

run_test_on_types!(iterator_border_outside_2 on all);
fn iterator_border_outside_2<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let v = b
        .iter()
        .on_border(&BidiRectSigned::new(-1, -1, 4, 4))
        .map(|t| t.id())
        .collect::<Vec<i32>>();

    assert_eq!(v, vec![13, 23, 33, 32, 31]);
}

run_test_on_types!(iterator_border_outside_3 on all);
fn iterator_border_outside_3<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let v = b
        .iter()
        .on_border(&BidiRectSigned::new(-2, -2, 3, 3))
        .map(|t| t.id())
        .collect::<Vec<i32>>();

    assert_eq!(v, vec![11]);
}

run_test_on_types!(iterator_border_1x1 on all);
fn iterator_border_1x1<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let v = b
        .iter()
        .on_border(&BidiRectSigned::new(0, 0, 1, 1))
        .map(|t| t.id())
        .collect::<Vec<i32>>();

    assert_eq!(v, vec![11]);
}

run_test_on_types!(iterator_border_0x0 on all);
fn iterator_border_0x0<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let v = b
        .iter()
        .on_border(&BidiRectSigned::new(0, 0, 0, 0))
        .map(|t| t.id())
        .collect::<Vec<i32>>();

    assert_eq!(v, vec![]);
}

run_test_on_types!(iterator_border_inside_3x1 on all);
fn iterator_border_inside_3x1<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let v = b
        .iter()
        .on_border(&BidiRectSigned::new(1, 0, 3, 1))
        .map(|t| t.id())
        .collect::<Vec<i32>>();

    assert_eq!(v, vec![12, 13, 14]);
}

run_test_on_types!(iterator_border_inside_1x3 on all);
fn iterator_border_inside_1x3<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let v = b
        .iter()
        .on_border(&BidiRectSigned::new(1, 0, 1, 3))
        .map(|t| t.id())
        .collect::<Vec<i32>>();

    assert_eq!(v, vec![12, 22, 32]);
}

run_test_on_types!(iterator_border_inside_2x3 on all);
fn iterator_border_inside_2x3<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let v = b
        .iter()
        .on_border(&BidiRectSigned::new(1, 0, 2, 3))
        .map(|t| t.id())
        .collect::<Vec<i32>>();

    assert_eq!(v, vec![12, 13, 23, 33, 32, 22]);
}

run_test_on_types!(iterator_border_inside_3x2 on all);
fn iterator_border_inside_3x2<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let v = b
        .iter()
        .on_border(&BidiRectSigned::new(1, 0, 3, 2))
        .map(|t| t.id())
        .collect::<Vec<i32>>();

    assert_eq!(v, vec![12, 13, 14, 24, 23, 22]);
}

run_test_on_types!(iterator_border_inside_2x2 on all);
fn iterator_border_inside_2x2<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let v = b
        .iter()
        .on_border(&BidiRectSigned::new(1, 0, 2, 2))
        .map(|t| t.id())
        .collect::<Vec<i32>>();

    assert_eq!(v, vec![12, 13, 23, 22]);
}

run_test_on_types!(iterator_border_inside_mut on all);
fn iterator_border_inside_mut<T: Testable>() {
    let mut b = helper_build_4x5::<T>();

    for v in b.iter_mut().on_border(&BidiRectSigned::new(1, 0, 3, 3)) {
        *v = T::new(-v.id());
    }

    let v = b.iter().map(|t| t.id()).collect::<Vec<i32>>();

    assert_eq!(
        v,
        vec![
            11, -12, -13, -14, 21, -22, 23, -24, 31, -32, -33, -34, 41, 42, 43, 44, 51, 52, 53, 54,
        ]
    );
}

run_test_on_types!(iterator_border_outside_1_mut on all);
fn iterator_border_outside_1_mut<T: Testable>() {
    let mut b = helper_build_4x5::<T>();

    for v in b.iter_mut().on_border(&BidiRectSigned::new(1, 2, 4, 4)) {
        *v = T::new(-v.id());
    }

    let v = b.iter().map(|t| t.id()).collect::<Vec<i32>>();

    assert_eq!(
        v,
        vec![11, 12, 13, 14, 21, 22, 23, 24, 31, -32, -33, -34, 41, -42, 43, 44, 51, -52, 53, 54,]
    );
}

run_test_on_types!(iterator_border_outside_2_mut on all);
fn iterator_border_outside_2_mut<T: Testable>() {
    let mut b = helper_build_4x5::<T>();

    for v in b.iter_mut().on_border(&BidiRectSigned::new(-1, -1, 4, 4)) {
        *v = T::new(-v.id());
    }

    let v = b.iter().map(|t| t.id()).collect::<Vec<i32>>();

    assert_eq!(
        v,
        vec![11, 12, -13, 14, 21, 22, -23, 24, -31, -32, -33, 34, 41, 42, 43, 44, 51, 52, 53, 54,]
    );
}

run_test_on_types!(iterator_neighbours_bordering on all);
fn iterator_neighbours_bordering<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let v = b
        .iter()
        .on_neighbours(1, 1, BidiNeighbours::Bordering)
        .map(|t| t.id())
        .collect::<Vec<i32>>();

    assert_eq!(v, vec![12, 13, 23, 33, 32, 31, 21, 11]);
}

run_test_on_types!(iterator_neighbours_adjacent on all);
fn iterator_neighbours_adjacent<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let v = b
        .iter()
        .on_neighbours(1, 1, BidiNeighbours::Adjacent)
        .map(|t| t.id())
        .collect::<Vec<i32>>();

    assert_eq!(v, vec![12, 23, 32, 21]);
}

run_test_on_types!(iterator_neighbours_bordering_clipped on all);
fn iterator_neighbours_bordering_clipped<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let v = b
        .iter()
        .on_neighbours(0, 0, BidiNeighbours::Bordering)
        .map(|t| t.id())
        .collect::<Vec<i32>>();

    assert_eq!(v, vec![12, 22, 21]);
}

run_test_on_types!(iterator_neighbours_adjacent_clipped on all);
fn iterator_neighbours_adjacent_clipped<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let v = b
        .iter()
        .on_neighbours(0, 0, BidiNeighbours::Adjacent)
        .map(|t| t.id())
        .collect::<Vec<i32>>();

    assert_eq!(v, vec![12, 21]);
}

run_test_on_types!(iterator_neighbours_bordering_clipped_2 on all);
fn iterator_neighbours_bordering_clipped_2<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let v = b
        .iter()
        .on_neighbours(3, 4, BidiNeighbours::Bordering)
        .map(|t| t.id())
        .collect::<Vec<i32>>();

    assert_eq!(v, vec![44, 53, 43]);
}

run_test_on_types!(iterator_neighbours_adjacent_clipped_2 on all);
fn iterator_neighbours_adjacent_clipped_2<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let v = b
        .iter()
        .on_neighbours(3, 4, BidiNeighbours::Adjacent)
        .map(|t| t.id())
        .collect::<Vec<i32>>();

    assert_eq!(v, vec![44, 53]);
}

run_test_on_types!(iterator_neighbours_bordering_mut on all);
fn iterator_neighbours_bordering_mut<T: Testable>() {
    let mut b = helper_build_4x5::<T>();

    for v in b.iter_mut().on_neighbours(1, 1, BidiNeighbours::Bordering) {
        *v = T::new(-v.id());
    }

    let v = b.iter().map(|t| t.id()).collect::<Vec<i32>>();

    assert_eq!(
        v,
        vec![
            -11, -12, -13, 14, -21, 22, -23, 24, -31, -32, -33, 34, 41, 42, 43, 44, 51, 52, 53, 54,
        ]
    );
}

run_test_on_types!(iterator_neighbours_adjacent_mut on all);
fn iterator_neighbours_adjacent_mut<T: Testable>() {
    let mut b = helper_build_4x5::<T>();

    for v in b.iter_mut().on_neighbours(1, 1, BidiNeighbours::Adjacent) {
        *v = T::new(-v.id());
    }

    let v = b.iter().map(|t| t.id()).collect::<Vec<i32>>();

    assert_eq!(
        v,
        vec![11, -12, 13, 14, -21, 22, -23, 24, 31, -32, 33, 34, 41, 42, 43, 44, 51, 52, 53, 54,]
    );
}

run_test_on_types!(iterator_neighbours_bordering_mut_clipped on all);
fn iterator_neighbours_bordering_mut_clipped<T: Testable>() {
    let mut b = helper_build_4x5::<T>();

    for v in b.iter_mut().on_neighbours(0, 0, BidiNeighbours::Bordering) {
        *v = T::new(-v.id());
    }

    let v = b.iter().map(|t| t.id()).collect::<Vec<i32>>();

    assert_eq!(
        v,
        vec![11, -12, 13, 14, -21, -22, 23, 24, 31, 32, 33, 34, 41, 42, 43, 44, 51, 52, 53, 54,]
    );
}

run_test_on_types!(iterator_neighbours_adjacent_mut_clipped on all);
fn iterator_neighbours_adjacent_mut_clipped<T: Testable>() {
    let mut b = helper_build_4x5::<T>();

    for v in b.iter_mut().on_neighbours(3, 4, BidiNeighbours::Adjacent) {
        *v = T::new(-v.id());
    }

    let v = b.iter().map(|t| t.id()).collect::<Vec<i32>>();

    assert_eq!(
        v,
        vec![11, 12, 13, 14, 21, 22, 23, 24, 31, 32, 33, 34, 41, 42, 43, -44, 51, 52, -53, 54,]
    );
}
