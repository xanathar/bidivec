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

fn helper_bidiarray<T: Testable>() -> BidiArray<T> {
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

fn helper_bidigrowvec<T: Testable>() -> BidiGrowVec<T> {
    bidigrowvec! {
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

run_test_on_types!(from_conversions on all);
fn from_conversions<T: Testable>() {
    let vv = BidiVec::<T>::from(helper_bidivec());
    let vg = BidiGrowVec::<T>::from(helper_bidivec());
    let va = BidiArray::<T>::from(helper_bidivec());

    let gv = BidiVec::<T>::from(helper_bidigrowvec());
    let gg = BidiGrowVec::<T>::from(helper_bidigrowvec());
    let ga = BidiArray::<T>::from(helper_bidigrowvec());

    let av = BidiVec::<T>::from(helper_bidiarray());
    let aa = BidiGrowVec::<T>::from(helper_bidiarray());
    let ag = BidiArray::<T>::from(helper_bidiarray());

    let ha = helper_bidiarray::<T>();
    let hv = helper_bidivec::<T>();
    let hg = helper_bidigrowvec::<T>();

    let v: Vec<&dyn BidiView<Output = T>> =
        vec![&vv, &va, &vg, &gv, &ga, &gg, &av, &aa, &ag, &ha, &hv, &hg];

    for v1 in v.iter() {
        for v2 in v.iter() {
            assert_view_eq_views_dyn::<T>(*v1, *v2);
        }
    }
}

run_test_on_types!(into_conversions on all);
fn into_conversions<T: Testable>() {
    let vv = helper_bidivec::<T>();
    let va = helper_bidivec::<T>().into_bidigrowvec();
    let vg = helper_bidivec::<T>().into_bidiarray();

    let gv = helper_bidigrowvec::<T>().into_bidivec();
    let ga = helper_bidigrowvec::<T>();
    let gg = helper_bidigrowvec::<T>().into_bidiarray();

    let av = helper_bidiarray::<T>().into_bidivec();
    let aa = helper_bidiarray::<T>().into_bidigrowvec();
    let ag = helper_bidiarray::<T>();

    let v: Vec<&dyn BidiView<Output = T>> = vec![&vv, &va, &vg, &gv, &ga, &gg, &av, &aa, &ag];

    for v1 in v.iter() {
        for v2 in v.iter() {
            assert_view_eq_views_dyn::<T>(*v1, *v2);
        }
    }
}

run_test_on_types!(transformations_transpose on all);
fn transformations_transpose<T: Testable>() {
    let mut vv1 = helper_bidivec::<T>();
    let mut va1 = helper_bidivec::<T>().into_bidigrowvec();
    let mut vg1 = helper_bidivec::<T>().into_bidiarray();
    let mut gv1 = helper_bidigrowvec::<T>().into_bidivec();
    let mut ga1 = helper_bidigrowvec::<T>();
    let mut gg1 = helper_bidigrowvec::<T>().into_bidiarray();
    let mut av1 = helper_bidiarray::<T>().into_bidivec();
    let mut aa1 = helper_bidiarray::<T>().into_bidigrowvec();
    let mut ag1 = helper_bidiarray::<T>();

    let vv2 = helper_bidivec::<T>();
    let va2 = helper_bidivec::<T>().into_bidigrowvec();
    let vg2 = helper_bidivec::<T>().into_bidiarray();
    let gv2 = helper_bidigrowvec::<T>().into_bidivec();
    let ga2 = helper_bidigrowvec::<T>();
    let gg2 = helper_bidigrowvec::<T>().into_bidiarray();
    let av2 = helper_bidiarray::<T>().into_bidivec();
    let aa2 = helper_bidiarray::<T>().into_bidigrowvec();
    let ag2 = helper_bidiarray::<T>();

    vv1.transpose();
    va1.transpose();
    vg1.transpose();
    gv1.transpose();
    ga1.transpose();
    gg1.transpose();
    av1.transpose();
    aa1.transpose();
    ag1.transpose();

    let vv2 = vv2.to_transposed();
    let va2 = va2.to_transposed();
    let vg2 = vg2.to_transposed();
    let gv2 = gv2.to_transposed();
    let ga2 = ga2.to_transposed();
    let gg2 = gg2.to_transposed();
    let av2 = av2.to_transposed();
    let aa2 = aa2.to_transposed();
    let ag2 = ag2.to_transposed();

    let v: Vec<&dyn BidiView<Output = T>> = vec![
        &vv1, &va1, &vg1, &gv1, &ga1, &gg1, &av1, &aa1, &ag1, &vv2, &va2, &vg2, &gv2, &ga2, &gg2,
        &av2, &aa2, &ag2,
    ];

    for v1 in v.iter() {
        for v2 in v.iter() {
            assert_view_eq_views_dyn::<T>(*v1, *v2);
        }
    }
}

run_test_on_types!(transformations_rot90 on all);
fn transformations_rot90<T: Testable>() {
    let mut vv1 = helper_bidivec::<T>();
    let mut va1 = helper_bidivec::<T>().into_bidigrowvec();
    let mut vg1 = helper_bidivec::<T>().into_bidiarray();
    let mut gv1 = helper_bidigrowvec::<T>().into_bidivec();
    let mut ga1 = helper_bidigrowvec::<T>();
    let mut gg1 = helper_bidigrowvec::<T>().into_bidiarray();
    let mut av1 = helper_bidiarray::<T>().into_bidivec();
    let mut aa1 = helper_bidiarray::<T>().into_bidigrowvec();
    let mut ag1 = helper_bidiarray::<T>();

    let vv2 = helper_bidivec::<T>();
    let va2 = helper_bidivec::<T>().into_bidigrowvec();
    let vg2 = helper_bidivec::<T>().into_bidiarray();
    let gv2 = helper_bidigrowvec::<T>().into_bidivec();
    let ga2 = helper_bidigrowvec::<T>();
    let gg2 = helper_bidigrowvec::<T>().into_bidiarray();
    let av2 = helper_bidiarray::<T>().into_bidivec();
    let aa2 = helper_bidiarray::<T>().into_bidigrowvec();
    let ag2 = helper_bidiarray::<T>();

    vv1.rotate90ccw();
    va1.rotate90ccw();
    vg1.rotate90ccw();
    gv1.rotate90ccw();
    ga1.rotate90ccw();
    gg1.rotate90ccw();
    av1.rotate90ccw();
    aa1.rotate90ccw();
    ag1.rotate90ccw();

    let vv2 = vv2.to_rotated90ccw();
    let va2 = va2.to_rotated90ccw();
    let vg2 = vg2.to_rotated90ccw();
    let gv2 = gv2.to_rotated90ccw();
    let ga2 = ga2.to_rotated90ccw();
    let gg2 = gg2.to_rotated90ccw();
    let av2 = av2.to_rotated90ccw();
    let aa2 = aa2.to_rotated90ccw();
    let ag2 = ag2.to_rotated90ccw();

    let v: Vec<&dyn BidiView<Output = T>> = vec![
        &vv1, &va1, &vg1, &gv1, &ga1, &gg1, &av1, &aa1, &ag1, &vv2, &va2, &vg2, &gv2, &ga2, &gg2,
        &av2, &aa2, &ag2,
    ];

    for v1 in v.iter() {
        for v2 in v.iter() {
            assert_view_eq_views_dyn::<T>(*v1, *v2);
        }
    }
}

run_test_on_types!(transformations_rot270 on all);
fn transformations_rot270<T: Testable>() {
    let mut vv1 = helper_bidivec::<T>();
    let mut va1 = helper_bidivec::<T>().into_bidigrowvec();
    let mut vg1 = helper_bidivec::<T>().into_bidiarray();
    let mut gv1 = helper_bidigrowvec::<T>().into_bidivec();
    let mut ga1 = helper_bidigrowvec::<T>();
    let mut gg1 = helper_bidigrowvec::<T>().into_bidiarray();
    let mut av1 = helper_bidiarray::<T>().into_bidivec();
    let mut aa1 = helper_bidiarray::<T>().into_bidigrowvec();
    let mut ag1 = helper_bidiarray::<T>();

    let vv2 = helper_bidivec::<T>();
    let va2 = helper_bidivec::<T>().into_bidigrowvec();
    let vg2 = helper_bidivec::<T>().into_bidiarray();
    let gv2 = helper_bidigrowvec::<T>().into_bidivec();
    let ga2 = helper_bidigrowvec::<T>();
    let gg2 = helper_bidigrowvec::<T>().into_bidiarray();
    let av2 = helper_bidiarray::<T>().into_bidivec();
    let aa2 = helper_bidiarray::<T>().into_bidigrowvec();
    let ag2 = helper_bidiarray::<T>();

    vv1.rotate270ccw();
    va1.rotate270ccw();
    vg1.rotate270ccw();
    gv1.rotate270ccw();
    ga1.rotate270ccw();
    gg1.rotate270ccw();
    av1.rotate270ccw();
    aa1.rotate270ccw();
    ag1.rotate270ccw();

    let vv2 = vv2.to_rotated270ccw();
    let va2 = va2.to_rotated270ccw();
    let vg2 = vg2.to_rotated270ccw();
    let gv2 = gv2.to_rotated270ccw();
    let ga2 = ga2.to_rotated270ccw();
    let gg2 = gg2.to_rotated270ccw();
    let av2 = av2.to_rotated270ccw();
    let aa2 = aa2.to_rotated270ccw();
    let ag2 = ag2.to_rotated270ccw();

    let v: Vec<&dyn BidiView<Output = T>> = vec![
        &vv1, &va1, &vg1, &gv1, &ga1, &gg1, &av1, &aa1, &ag1, &vv2, &va2, &vg2, &gv2, &ga2, &gg2,
        &av2, &aa2, &ag2,
    ];

    for v1 in v.iter() {
        for v2 in v.iter() {
            assert_view_eq_views_dyn::<T>(*v1, *v2);
        }
    }
}

run_test_on_types!(transformations_rot180 on all);
fn transformations_rot180<T: Testable>() {
    let mut vv1 = helper_bidivec::<T>();
    let mut va1 = helper_bidivec::<T>().into_bidigrowvec();
    let mut vg1 = helper_bidivec::<T>().into_bidiarray();
    let mut gv1 = helper_bidigrowvec::<T>().into_bidivec();
    let mut ga1 = helper_bidigrowvec::<T>();
    let mut gg1 = helper_bidigrowvec::<T>().into_bidiarray();
    let mut av1 = helper_bidiarray::<T>().into_bidivec();
    let mut aa1 = helper_bidiarray::<T>().into_bidigrowvec();
    let mut ag1 = helper_bidiarray::<T>();

    let vv2 = helper_bidivec::<T>();
    let va2 = helper_bidivec::<T>().into_bidigrowvec();
    let vg2 = helper_bidivec::<T>().into_bidiarray();
    let gv2 = helper_bidigrowvec::<T>().into_bidivec();
    let ga2 = helper_bidigrowvec::<T>();
    let gg2 = helper_bidigrowvec::<T>().into_bidiarray();
    let av2 = helper_bidiarray::<T>().into_bidivec();
    let aa2 = helper_bidiarray::<T>().into_bidigrowvec();
    let ag2 = helper_bidiarray::<T>();

    vv1.rotate180();
    va1.rotate180();
    vg1.rotate180();
    gv1.rotate180();
    ga1.rotate180();
    gg1.rotate180();
    av1.rotate180();
    aa1.rotate180();
    ag1.rotate180();

    let vv2 = vv2.to_rotated180();
    let va2 = va2.to_rotated180();
    let vg2 = vg2.to_rotated180();
    let gv2 = gv2.to_rotated180();
    let ga2 = ga2.to_rotated180();
    let gg2 = gg2.to_rotated180();
    let av2 = av2.to_rotated180();
    let aa2 = aa2.to_rotated180();
    let ag2 = ag2.to_rotated180();

    let v: Vec<&dyn BidiView<Output = T>> = vec![
        &vv1, &va1, &vg1, &gv1, &ga1, &gg1, &av1, &aa1, &ag1, &vv2, &va2, &vg2, &gv2, &ga2, &gg2,
        &av2, &aa2, &ag2,
    ];

    for v1 in v.iter() {
        for v2 in v.iter() {
            assert_view_eq_views_dyn::<T>(*v1, *v2);
        }
    }
}

run_test_on_types!(transformations_reverse_columns on all);
fn transformations_reverse_columns<T: Testable>() {
    let mut vv1 = helper_bidivec::<T>();
    let mut va1 = helper_bidivec::<T>().into_bidigrowvec();
    let mut vg1 = helper_bidivec::<T>().into_bidiarray();
    let mut gv1 = helper_bidigrowvec::<T>().into_bidivec();
    let mut ga1 = helper_bidigrowvec::<T>();
    let mut gg1 = helper_bidigrowvec::<T>().into_bidiarray();
    let mut av1 = helper_bidiarray::<T>().into_bidivec();
    let mut aa1 = helper_bidiarray::<T>().into_bidigrowvec();
    let mut ag1 = helper_bidiarray::<T>();

    let vv2 = helper_bidivec::<T>();
    let va2 = helper_bidivec::<T>().into_bidigrowvec();
    let vg2 = helper_bidivec::<T>().into_bidiarray();
    let gv2 = helper_bidigrowvec::<T>().into_bidivec();
    let ga2 = helper_bidigrowvec::<T>();
    let gg2 = helper_bidigrowvec::<T>().into_bidiarray();
    let av2 = helper_bidiarray::<T>().into_bidivec();
    let aa2 = helper_bidiarray::<T>().into_bidigrowvec();
    let ag2 = helper_bidiarray::<T>();

    vv1.reverse_columns();
    va1.reverse_columns();
    vg1.reverse_columns();
    gv1.reverse_columns();
    ga1.reverse_columns();
    gg1.reverse_columns();
    av1.reverse_columns();
    aa1.reverse_columns();
    ag1.reverse_columns();

    let vv2 = vv2.to_reversed_columns();
    let va2 = va2.to_reversed_columns();
    let vg2 = vg2.to_reversed_columns();
    let gv2 = gv2.to_reversed_columns();
    let ga2 = ga2.to_reversed_columns();
    let gg2 = gg2.to_reversed_columns();
    let av2 = av2.to_reversed_columns();
    let aa2 = aa2.to_reversed_columns();
    let ag2 = ag2.to_reversed_columns();

    let v: Vec<&dyn BidiView<Output = T>> = vec![
        &vv1, &va1, &vg1, &gv1, &ga1, &gg1, &av1, &aa1, &ag1, &vv2, &va2, &vg2, &gv2, &ga2, &gg2,
        &av2, &aa2, &ag2,
    ];

    for v1 in v.iter() {
        for v2 in v.iter() {
            assert_view_eq_views_dyn::<T>(*v1, *v2);
        }
    }
}

run_test_on_types!(transformations_reverse_rows on all);
fn transformations_reverse_rows<T: Testable>() {
    let mut vv1 = helper_bidivec::<T>();
    let mut va1 = helper_bidivec::<T>().into_bidigrowvec();
    let mut vg1 = helper_bidivec::<T>().into_bidiarray();
    let mut gv1 = helper_bidigrowvec::<T>().into_bidivec();
    let mut ga1 = helper_bidigrowvec::<T>();
    let mut gg1 = helper_bidigrowvec::<T>().into_bidiarray();
    let mut av1 = helper_bidiarray::<T>().into_bidivec();
    let mut aa1 = helper_bidiarray::<T>().into_bidigrowvec();
    let mut ag1 = helper_bidiarray::<T>();

    let vv2 = helper_bidivec::<T>();
    let va2 = helper_bidivec::<T>().into_bidigrowvec();
    let vg2 = helper_bidivec::<T>().into_bidiarray();
    let gv2 = helper_bidigrowvec::<T>().into_bidivec();
    let ga2 = helper_bidigrowvec::<T>();
    let gg2 = helper_bidigrowvec::<T>().into_bidiarray();
    let av2 = helper_bidiarray::<T>().into_bidivec();
    let aa2 = helper_bidiarray::<T>().into_bidigrowvec();
    let ag2 = helper_bidiarray::<T>();

    vv1.reverse_rows();
    va1.reverse_rows();
    vg1.reverse_rows();
    gv1.reverse_rows();
    ga1.reverse_rows();
    gg1.reverse_rows();
    av1.reverse_rows();
    aa1.reverse_rows();
    ag1.reverse_rows();

    let vv2 = vv2.to_reversed_rows();
    let va2 = va2.to_reversed_rows();
    let vg2 = vg2.to_reversed_rows();
    let gv2 = gv2.to_reversed_rows();
    let ga2 = ga2.to_reversed_rows();
    let gg2 = gg2.to_reversed_rows();
    let av2 = av2.to_reversed_rows();
    let aa2 = aa2.to_reversed_rows();
    let ag2 = ag2.to_reversed_rows();

    let v: Vec<&dyn BidiView<Output = T>> = vec![
        &vv1, &va1, &vg1, &gv1, &ga1, &gg1, &av1, &aa1, &ag1, &vv2, &va2, &vg2, &gv2, &ga2, &gg2,
        &av2, &aa2, &ag2,
    ];

    for v1 in v.iter() {
        for v2 in v.iter() {
            assert_view_eq_views_dyn::<T>(*v1, *v2);
        }
    }
}

run_test_on_types!(transformations_crop on all);
fn transformations_crop<T: Testable>() {
    let rect = BidiRect::new(3, 3, 6, 6);

    let mut vv1 = helper_bidivec::<T>();
    let mut va1 = helper_bidivec::<T>().into_bidigrowvec();
    let mut gv1 = helper_bidigrowvec::<T>().into_bidivec();
    let mut ga1 = helper_bidigrowvec::<T>();
    let mut av1 = helper_bidiarray::<T>().into_bidivec();
    let mut aa1 = helper_bidiarray::<T>().into_bidigrowvec();

    let vv2 = helper_bidivec::<T>();
    let va2 = helper_bidivec::<T>().into_bidigrowvec();
    let vg2 = helper_bidivec::<T>().into_bidiarray();
    let gv2 = helper_bidigrowvec::<T>().into_bidivec();
    let ga2 = helper_bidigrowvec::<T>();
    let gg2 = helper_bidigrowvec::<T>().into_bidiarray();
    let av2 = helper_bidiarray::<T>().into_bidivec();
    let aa2 = helper_bidiarray::<T>().into_bidigrowvec();
    let ag2 = helper_bidiarray::<T>();

    vv1.crop(&rect).unwrap();
    va1.crop(&rect).unwrap();
    gv1.crop(&rect).unwrap();
    ga1.crop(&rect).unwrap();
    av1.crop(&rect).unwrap();
    aa1.crop(&rect).unwrap();

    let vv2 = vv2.to_cropped(&rect).unwrap();
    let va2 = va2.to_cropped(&rect).unwrap();
    let vg2 = vg2.to_cropped(&rect).unwrap();
    let gv2 = gv2.to_cropped(&rect).unwrap();
    let ga2 = ga2.to_cropped(&rect).unwrap();
    let gg2 = gg2.to_cropped(&rect).unwrap();
    let av2 = av2.to_cropped(&rect).unwrap();
    let aa2 = aa2.to_cropped(&rect).unwrap();
    let ag2 = ag2.to_cropped(&rect).unwrap();

    let v: Vec<&dyn BidiView<Output = T>> = vec![
        &vv1, &va1, &gv1, &ga1, &av1, &aa1, &vv2, &va2, &vg2, &gv2, &ga2, &gg2, &av2, &aa2, &ag2,
    ];

    for v1 in v.iter() {
        for v2 in v.iter() {
            assert_view_eq_views_dyn::<T>(*v1, *v2);
        }
    }
}
