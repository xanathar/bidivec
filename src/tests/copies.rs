#![cfg(test)]
use super::*;
use test_types::*;

fn helper_bidivec<T: Testable>() -> BidiVec<T> {
    bidivec! {
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

fn helper_blended<T: Testable>() -> BidiVec<T> {
    bidivec! {
        [T::new(00), T::new(01), T::new(02), T::new(03), T::new(04), T::new(05), T::new(6106), T::new(6207), T::new(6308), T::new(09)],
        [T::new(10), T::new(11), T::new(12), T::new(13), T::new(14), T::new(15), T::new(7116), T::new(7217), T::new(7318), T::new(19)],
        [T::new(20), T::new(21), T::new(22), T::new(23), T::new(24), T::new(25), T::new(8126), T::new(8227), T::new(8328), T::new(29)],
        [T::new(30), T::new(31), T::new(32), T::new(33), T::new(34), T::new(35), T::new(9136), T::new(9237), T::new(9338), T::new(39)],
        [T::new(40), T::new(41), T::new(42), T::new(43), T::new(44), T::new(45), T::new(46), T::new(47), T::new(48), T::new(49)],
        [T::new(50), T::new(51), T::new(52), T::new(53), T::new(54), T::new(55), T::new(56), T::new(57), T::new(58), T::new(59)],
        [T::new(60), T::new(61), T::new(62), T::new(63), T::new(64), T::new(65), T::new(66), T::new(67), T::new(68), T::new(69)],
        [T::new(70), T::new(71), T::new(72), T::new(73), T::new(74), T::new(75), T::new(76), T::new(77), T::new(78), T::new(79)],
        [T::new(80), T::new(81), T::new(82), T::new(83), T::new(84), T::new(85), T::new(86), T::new(87), T::new(88), T::new(89)],
        [T::new(90), T::new(91), T::new(92), T::new(93), T::new(94), T::new(95), T::new(96), T::new(97), T::new(98), T::new(99)],
    }
}

fn helper_copied<T: Testable>() -> BidiVec<T> {
    bidivec! {
        [T::new(56), T::new(57), T::new(58), T::new(59), T::new(04), T::new(05), T::new(06), T::new(07), T::new(08), T::new(09)],
        [T::new(66), T::new(67), T::new(68), T::new(69), T::new(14), T::new(15), T::new(16), T::new(17), T::new(18), T::new(19)],
        [T::new(76), T::new(77), T::new(78), T::new(79), T::new(24), T::new(25), T::new(26), T::new(27), T::new(28), T::new(29)],
        [T::new(86), T::new(87), T::new(88), T::new(89), T::new(34), T::new(35), T::new(36), T::new(37), T::new(38), T::new(39)],
        [T::new(96), T::new(97), T::new(98), T::new(99), T::new(44), T::new(45), T::new(46), T::new(47), T::new(48), T::new(49)],
        [T::new(50), T::new(51), T::new(52), T::new(53), T::new(54), T::new(55), T::new(56), T::new(57), T::new(58), T::new(59)],
        [T::new(60), T::new(61), T::new(62), T::new(63), T::new(64), T::new(65), T::new(66), T::new(67), T::new(68), T::new(69)],
        [T::new(70), T::new(71), T::new(72), T::new(73), T::new(74), T::new(75), T::new(76), T::new(77), T::new(78), T::new(79)],
        [T::new(80), T::new(81), T::new(82), T::new(83), T::new(84), T::new(85), T::new(86), T::new(87), T::new(88), T::new(89)],
        [T::new(90), T::new(91), T::new(92), T::new(93), T::new(94), T::new(95), T::new(96), T::new(97), T::new(98), T::new(99)],
    }
}

run_test_on_types!(flat_copy on copyables);
fn flat_copy<T: Testable + Copy>() {
    let mut bv1 = helper_bidivec::<T>();
    let bv2 = helper_bidivec::<T>();
    let exp = helper_copied::<T>();

    crate::editing::copy(&bv2, &mut bv1, &BidiRect::new(6, 5, 4, 5), (0, 0)).unwrap();

    assert_view_eq_views(&bv1, &exp);
}

run_test_on_types!(flat_clone_over on clonables);
fn flat_clone_over<T: Testable + Clone>() {
    let mut bv1 = helper_bidivec::<T>();
    let bv2 = helper_bidivec::<T>();
    let exp = helper_copied::<T>();

    crate::editing::clone_over(&bv2, &mut bv1, &BidiRect::new(6, 5, 4, 5), (0, 0)).unwrap();

    assert_view_eq_views(&bv1, &exp);
}

run_test_on_types!(flat_blender on all);
fn flat_blender<T: Testable>() {
    let mut bv1 = helper_bidivec::<T>();
    let bv2 = helper_bidivec::<T>();
    let exp = helper_blended::<T>();

    crate::editing::blend(
        &bv2,
        &mut bv1,
        &BidiRect::new(1, 6, 3, 4),
        (6, 0),
        |s, d| *d = T::new(s.id() * 100 + d.id()),
    )
    .unwrap();

    assert_view_eq_views(&bv1, &exp);
}

#[test]
fn copy_example() {
    let v1 = bidivec! {
        [0, 1, 2, 3],
        [4, 5, 6, 7],
        [8, 9, 10, 11],
        [12, 13, 14, 15],
    };
    let mut v2 = bidivec![-1; 4, 4];

    editing::copy(&v1, &mut v2, &BidiRect::new(0, 0, 2, 2), (0, 0)).unwrap();

    assert_eq!(v2[(0, 0)], 0);
    assert_eq!(v2[(1, 0)], 1);
    assert_eq!(v2[(0, 1)], 4);
    assert_eq!(v2[(1, 1)], 5);
    assert_eq!(v2[(2, 0)], -1);
    assert_eq!(v2[(0, 2)], -1);
}

#[test]
fn clone_over_example() {
    #[derive(Clone)]
    struct Cloneable(i32);

    let v1 = bidivec! {
        [Cloneable(0), Cloneable(1), Cloneable(2), Cloneable(3)],
        [Cloneable(4), Cloneable(5), Cloneable(6), Cloneable(7)],
        [Cloneable(8), Cloneable(9), Cloneable(10), Cloneable(11)],
        [Cloneable(12), Cloneable(13), Cloneable(14), Cloneable(15)],
    };
    let mut v2 = bidivec![Cloneable(-1); 4, 4];

    editing::clone_over(&v1, &mut v2, &BidiRect::new(0, 0, 2, 2), (0, 0)).unwrap();

    assert_eq!(v2[(0, 0)].0, 0);
    assert_eq!(v2[(1, 0)].0, 1);
    assert_eq!(v2[(0, 1)].0, 4);
    assert_eq!(v2[(1, 1)].0, 5);
    assert_eq!(v2[(2, 0)].0, -1);
    assert_eq!(v2[(0, 2)].0, -1);
}

#[test]
fn blend_example() {
    let v1 = bidivec! {
        [0, 1, 2, 3],
        [4, 5, 6, 7],
        [8, 9, 10, 11],
        [12, 13, 14, 15],
    };
    let mut v2 = bidivec![100; 4, 4];

    editing::blend(
        &v1,
        &mut v2,
        &BidiRect::new(0, 0, 2, 2),
        (0, 0),
        |src, dst| *dst = src + 2 * (*dst),
    )
    .unwrap();

    assert_eq!(v2[(0, 0)], 200);
    assert_eq!(v2[(1, 0)], 201);
    assert_eq!(v2[(0, 1)], 204);
    assert_eq!(v2[(1, 1)], 205);
    assert_eq!(v2[(2, 0)], 100);
    assert_eq!(v2[(0, 2)], 100);
}
