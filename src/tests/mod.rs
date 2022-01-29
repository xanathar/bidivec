#![cfg(test)]
use crate::*;
use test_types::Testable;

mod bidiarray_tests;
mod bidigrowvec_tests;
mod bidimutslice_tests;
mod bidislice_tests;
mod bidivec_tests;
mod conversions;
mod copies;
mod test_types;

fn assert_err<T>(expected_err: BidiError, r: Result<T, BidiError>) {
    match r {
        Err(e) if e == expected_err => (),
        Err(e) => {
            panic!("Expected '{:?}' got '{:?}'.", expected_err, e)
        }
        Ok(_) => {
            panic!("Expected '{:?}' got an ok.", expected_err)
        }
    }
}

fn assert_view_eq_views_dyn<T: Testable>(
    v1: &dyn BidiView<Output = T>,
    v2: &dyn BidiView<Output = T>,
) {
    const COLOR_RESTORE: &str = "\x1b[0m";
    const COLOR_ALERT: &str = "\x1b[1;31m";
    const COLOR_OK: &str = "\x1b[1;32m";
    const MAX_SIZE: usize = 100;

    fn alert_color(v: bool) -> &'static str {
        if v {
            COLOR_ALERT
        } else {
            COLOR_RESTORE
        }
    }

    let width_differs = v1.width() != v2.width();
    let height_differs = v1.height() != v2.height();

    let mut diff = [false; MAX_SIZE * MAX_SIZE];
    let mut any_diff = false;

    for y in 0..std::cmp::min(v1.height(), v2.height()) {
        for x in 0..std::cmp::min(v1.width(), v2.width()) {
            let i2 = &v2[(x, y)];
            let i1 = &v1[(x, y)];
            diff[y * MAX_SIZE + x] = i1.id() != i2.id();
            any_diff |= diff[y * MAX_SIZE + x];
        }
    }

    if any_diff || height_differs || width_differs {
        println!("BidiVecs are different:");
        println!(
            "Width: {}found {} expected {}{}",
            alert_color(width_differs),
            v1.width(),
            v2.width(),
            COLOR_RESTORE
        );
        println!(
            "Height: {}found {} expected {}{}",
            alert_color(height_differs),
            v1.height(),
            v2.height(),
            COLOR_RESTORE
        );
        println!("Found items:");
        for y in 0..v1.height() {
            print!("\t");
            for x in 0..v1.width() {
                if diff[y * MAX_SIZE + x] {
                    print!("{}{:?}{}, ", COLOR_ALERT, v1[(x, y)].id(), COLOR_RESTORE);
                } else {
                    print!("{:?}, ", v1[(x, y)].id());
                }
            }
            println!();
        }
        println!();
        println!("Expected items:");
        for y in 0..v2.height() {
            print!("\t");
            for x in 0..v2.width() {
                if diff[y * MAX_SIZE + x] {
                    print!("{}{:?}{}, ", COLOR_OK, v2[(x, y)].id(), COLOR_RESTORE);
                } else {
                    print!("{:?}, ", v2[(x, y)].id());
                }
            }
            println!();
        }
        panic!();
    }
}

fn assert_view_eq_views<V1, V2>(v1: &V1, v2: &V2)
where
    V1: BidiView,
    V1::Output: test_types::Testable,
    V2: BidiView,
    V2::Output: test_types::Testable,
{
    const COLOR_RESTORE: &str = "\x1b[0m";
    const COLOR_ALERT: &str = "\x1b[1;31m";
    const COLOR_OK: &str = "\x1b[1;32m";
    const MAX_SIZE: usize = 100;

    fn alert_color(v: bool) -> &'static str {
        if v {
            COLOR_ALERT
        } else {
            COLOR_RESTORE
        }
    }

    let width_differs = v1.width() != v2.width();
    let height_differs = v1.height() != v2.height();

    let mut diff = [false; MAX_SIZE * MAX_SIZE];
    let mut any_diff = false;

    for y in 0..std::cmp::min(v1.height(), v2.height()) {
        for x in 0..std::cmp::min(v1.width(), v2.width()) {
            let i2 = &v2[(x, y)];
            let i1 = &v1[(x, y)];
            diff[y * MAX_SIZE + x] = i1.id() != i2.id();
            any_diff |= diff[y * MAX_SIZE + x];
        }
    }

    if any_diff || height_differs || width_differs {
        println!("BidiVecs are different:");
        println!(
            "Width: {}found {} expected {}{}",
            alert_color(width_differs),
            v1.width(),
            v2.width(),
            COLOR_RESTORE
        );
        println!(
            "Height: {}found {} expected {}{}",
            alert_color(height_differs),
            v1.height(),
            v2.height(),
            COLOR_RESTORE
        );
        println!("Found items:");
        for y in 0..v1.height() {
            print!("\t");
            for x in 0..v1.width() {
                if diff[y * MAX_SIZE + x] {
                    print!("{}{:?}{}, ", COLOR_ALERT, v1[(x, y)].id(), COLOR_RESTORE);
                } else {
                    print!("{:?}, ", v1[(x, y)].id());
                }
            }
            println!();
        }
        println!();
        println!("Expected items:");
        for y in 0..v2.height() {
            print!("\t");
            for x in 0..v2.width() {
                if diff[y * MAX_SIZE + x] {
                    print!("{}{:?}{}, ", COLOR_OK, v2[(x, y)].id(), COLOR_RESTORE);
                } else {
                    print!("{:?}, ", v2[(x, y)].id());
                }
            }
            println!();
        }
        panic!();
    }
}
