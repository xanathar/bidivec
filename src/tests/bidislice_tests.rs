use super::test_types::*;
use super::*;
use crate::run_test_on_types;

fn helper_build_4x5<T: Testable>() -> BidiArray<T> {
    bidiarray! {
        [T::new(11), T::new(12), T::new(13), T::new(14)],
        [T::new(21), T::new(22), T::new(23), T::new(24)],
        [T::new(31), T::new(32), T::new(33), T::new(34)],
        [T::new(41), T::new(42), T::new(43), T::new(44)],
        [T::new(51), T::new(52), T::new(53), T::new(54)],
    }
}

fn assert_layout<T: Testable>(v: BidiSlice<T>, width: usize, height: usize, items: Vec<i32>) {
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

    let v = v.into_slice();

    let len_differs = v.len() != items.len();

    let mut items_differ = false;
    for (i, j) in v.iter().zip(items.iter()) {
        items_differ |= i.id() != *j;
    }

    if items_differ || len_differs || height_differs || width_differs {
        println!("BidiSlices are different:");
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
}

fn assert_layout_bidivec<T: Testable>(
    v: BidiVec<T>,
    width: usize,
    height: usize,
    items: Vec<i32>,
) -> BidiVec<T> {
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
        println!("BidiVecs are different:");
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

    BidiVec::<T>::from_vec(v, vwidth).unwrap()
}

run_test_on_types!(bidislice_basics on all);
fn bidislice_basics<T: Testable>() {
    let v = bidiarray! {
        [T::new(11), T::new(12), T::new(13), T::new(14)],
        [T::new(21), T::new(22), T::new(23), T::new(24)],
        [T::new(31), T::new(32), T::new(33), T::new(34)],
        [T::new(41), T::new(42), T::new(43), T::new(44)],
        [T::new(51), T::new(52), T::new(53), T::new(54)],
    };

    let width = v.width();
    let slice = v.into_boxed_slice();
    let v = BidiSlice::new(&slice, width).unwrap();
    assert_layout::<T>(
        v,
        4,
        5,
        vec![
            11, 12, 13, 14, 21, 22, 23, 24, 31, 32, 33, 34, 41, 42, 43, 44, 51, 52, 53, 54,
        ],
    );
}

run_test_on_types!(bidislice_as_view on clonables);
fn bidislice_as_view<T: Testable + Clone>() {
    let v = bidiarray! {
        [T::new(11), T::new(12), T::new(13), T::new(14)],
        [T::new(21), T::new(22), T::new(23), T::new(24)],
        [T::new(31), T::new(32), T::new(33), T::new(34)],
        [T::new(41), T::new(42), T::new(43), T::new(44)],
        [T::new(51), T::new(52), T::new(53), T::new(54)],
    };

    let width = v.width();
    let slice = v.into_boxed_slice();
    let v = BidiSlice::new(&slice, width).unwrap();

    let bvec = BidiVec::from_view(v.as_bidiview()).unwrap();
    assert_layout_bidivec::<T>(
        bvec,
        4,
        5,
        vec![
            11, 12, 13, 14, 21, 22, 23, 24, 31, 32, 33, 34, 41, 42, 43, 44, 51, 52, 53, 54,
        ],
    );
}

// ==================================================
// Tests for iterators
// ==================================================

run_test_on_types!(iterator_basic on all);
fn iterator_basic<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let width = b.width();
    let slice = b.into_boxed_slice();
    let b = BidiSlice::new(&slice, width).unwrap();
    let v = b.iter().map(|t| t.id()).collect::<Vec<i32>>();

    assert_eq!(
        v,
        vec![11, 12, 13, 14, 21, 22, 23, 24, 31, 32, 33, 34, 41, 42, 43, 44, 51, 52, 53, 54]
    );
}

run_test_on_types!(iterator_basic_by_col on all);
fn iterator_basic_by_col<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let width = b.width();
    let slice = b.into_boxed_slice();
    let b = BidiSlice::new(&slice, width).unwrap();
    let v = b.iter().by_column().map(|t| t.id()).collect::<Vec<i32>>();

    assert_eq!(
        v,
        vec![11, 21, 31, 41, 51, 12, 22, 32, 42, 52, 13, 23, 33, 43, 53, 14, 24, 34, 44, 54,]
    );
}

run_test_on_types!(iterator_basic_on_row on all);
fn iterator_basic_on_row<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let width = b.width();
    let slice = b.into_boxed_slice();
    let b = BidiSlice::new(&slice, width).unwrap();
    let v = b.iter().on_row(3).map(|t| t.id()).collect::<Vec<i32>>();

    assert_eq!(v, vec![41, 42, 43, 44,]);
}

run_test_on_types!(iterator_basic_on_row_out_of_range on all);
fn iterator_basic_on_row_out_of_range<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let width = b.width();
    let slice = b.into_boxed_slice();
    let b = BidiSlice::new(&slice, width).unwrap();
    let v = b.iter().on_row(5).map(|t| t.id()).collect::<Vec<i32>>();

    assert_eq!(v, vec![]);
}

run_test_on_types!(iterator_basic_on_col_out_of_range on all);
fn iterator_basic_on_col_out_of_range<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let width = b.width();
    let slice = b.into_boxed_slice();
    let b = BidiSlice::new(&slice, width).unwrap();
    let v = b.iter().on_column(4).map(|t| t.id()).collect::<Vec<i32>>();

    assert_eq!(v, vec![]);
}

run_test_on_types!(iterator_basic_on_col on all);
fn iterator_basic_on_col<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let width = b.width();
    let slice = b.into_boxed_slice();
    let b = BidiSlice::new(&slice, width).unwrap();
    let v = b.iter().on_column(1).map(|t| t.id()).collect::<Vec<i32>>();

    assert_eq!(v, vec![12, 22, 32, 42, 52,]);
}

run_test_on_types!(iterator_basic_on_rect on all);
fn iterator_basic_on_rect<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let width = b.width();
    let slice = b.into_boxed_slice();
    let b = BidiSlice::new(&slice, width).unwrap();
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
    let width = b.width();
    let slice = b.into_boxed_slice();
    let b = BidiSlice::new(&slice, width).unwrap();
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
    let width = b.width();
    let slice = b.into_boxed_slice();
    let b = BidiSlice::new(&slice, width).unwrap();
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
    let width = b.width();
    let slice = b.into_boxed_slice();
    let b = BidiSlice::new(&slice, width).unwrap();
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
    let width = b.width();
    let slice = b.into_boxed_slice();
    let b = BidiSlice::new(&slice, width).unwrap();
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
    let width = b.width();
    let slice = b.into_boxed_slice();
    let b = BidiSlice::new(&slice, width).unwrap();
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
    let width = b.width();
    let slice = b.into_boxed_slice();
    let b = BidiSlice::new(&slice, width).unwrap();
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
    let width = b.width();
    let slice = b.into_boxed_slice();
    let b = BidiSlice::new(&slice, width).unwrap();
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
    let width = b.width();
    let slice = b.into_boxed_slice();
    let b = BidiSlice::new(&slice, width).unwrap();
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
    let width = b.width();
    let slice = b.into_boxed_slice();
    let b = BidiSlice::new(&slice, width).unwrap();
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
    let width = b.width();
    let slice = b.into_boxed_slice();
    let b = BidiSlice::new(&slice, width).unwrap();
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
    let width = b.width();
    let slice = b.into_boxed_slice();
    let b = BidiSlice::new(&slice, width).unwrap();
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
    let width = b.width();
    let slice = b.into_boxed_slice();
    let b = BidiSlice::new(&slice, width).unwrap();
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
    let width = b.width();
    let slice = b.into_boxed_slice();
    let b = BidiSlice::new(&slice, width).unwrap();
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
    let width = b.width();
    let slice = b.into_boxed_slice();
    let b = BidiSlice::new(&slice, width).unwrap();
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
    let width = b.width();
    let slice = b.into_boxed_slice();
    let b = BidiSlice::new(&slice, width).unwrap();
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
    let width = b.width();
    let slice = b.into_boxed_slice();
    let b = BidiSlice::new(&slice, width).unwrap();
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
    let width = b.width();
    let slice = b.into_boxed_slice();
    let b = BidiSlice::new(&slice, width).unwrap();
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
    let width = b.width();
    let slice = b.into_boxed_slice();
    let b = BidiSlice::new(&slice, width).unwrap();
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
    let width = b.width();
    let slice = b.into_boxed_slice();
    let b = BidiSlice::new(&slice, width).unwrap();
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
    let width = b.width();
    let slice = b.into_boxed_slice();
    let b = BidiSlice::new(&slice, width).unwrap();
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
    let width = b.width();
    let slice = b.into_boxed_slice();
    let b = BidiSlice::new(&slice, width).unwrap();
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

run_test_on_types!(iterator_border_inside on all);
fn iterator_border_inside<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let width = b.width();
    let slice = b.into_boxed_slice();
    let b = BidiSlice::new(&slice, width).unwrap();
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
    let width = b.width();
    let slice = b.into_boxed_slice();
    let b = BidiSlice::new(&slice, width).unwrap();
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
    let width = b.width();
    let slice = b.into_boxed_slice();
    let b = BidiSlice::new(&slice, width).unwrap();
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
    let width = b.width();
    let slice = b.into_boxed_slice();
    let b = BidiSlice::new(&slice, width).unwrap();
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
    let width = b.width();
    let slice = b.into_boxed_slice();
    let b = BidiSlice::new(&slice, width).unwrap();
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
    let width = b.width();
    let slice = b.into_boxed_slice();
    let b = BidiSlice::new(&slice, width).unwrap();
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
    let width = b.width();
    let slice = b.into_boxed_slice();
    let b = BidiSlice::new(&slice, width).unwrap();
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
    let width = b.width();
    let slice = b.into_boxed_slice();
    let b = BidiSlice::new(&slice, width).unwrap();
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
    let width = b.width();
    let slice = b.into_boxed_slice();
    let b = BidiSlice::new(&slice, width).unwrap();
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
    let width = b.width();
    let slice = b.into_boxed_slice();
    let b = BidiSlice::new(&slice, width).unwrap();
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
    let width = b.width();
    let slice = b.into_boxed_slice();
    let b = BidiSlice::new(&slice, width).unwrap();
    let v = b
        .iter()
        .on_border(&BidiRectSigned::new(1, 0, 2, 2))
        .map(|t| t.id())
        .collect::<Vec<i32>>();

    assert_eq!(v, vec![12, 13, 23, 22]);
}

run_test_on_types!(iterator_neighbours_bordering on all);
fn iterator_neighbours_bordering<T: Testable>() {
    let b = helper_build_4x5::<T>();
    let width = b.width();
    let slice = b.into_boxed_slice();
    let b = BidiSlice::new(&slice, width).unwrap();
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
    let width = b.width();
    let slice = b.into_boxed_slice();
    let b = BidiSlice::new(&slice, width).unwrap();
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
    let width = b.width();
    let slice = b.into_boxed_slice();
    let b = BidiSlice::new(&slice, width).unwrap();
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
    let width = b.width();
    let slice = b.into_boxed_slice();
    let b = BidiSlice::new(&slice, width).unwrap();
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
    let width = b.width();
    let slice = b.into_boxed_slice();
    let b = BidiSlice::new(&slice, width).unwrap();
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
    let width = b.width();
    let slice = b.into_boxed_slice();
    let b = BidiSlice::new(&slice, width).unwrap();
    let v = b
        .iter()
        .on_neighbours(3, 4, BidiNeighbours::Adjacent)
        .map(|t| t.id())
        .collect::<Vec<i32>>();

    assert_eq!(v, vec![44, 53]);
}
