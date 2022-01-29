use bidivec::pathfinding::*;
use bidivec::*;
use std::time::Instant;

const ITERATIONS: usize = 5;

pub fn create_pathfind_map(width: usize, height: usize) -> BidiVec<u8> {
    BidiVec::with_size_func(width, height, || rand::random::<u8>())
}

pub fn create_uniform_map(width: usize, height: usize, value: u8) -> BidiVec<u8> {
    BidiVec::with_elem(value, width, height)
}

pub fn create_checkerboard_map(width: usize, height: usize, value1: u8, value2: u8) -> BidiVec<u8> {
    BidiVec::with_size_func_xy(width, height, |x, y| {
        if (x + y) % 2 == 1 {
            value1
        } else {
            value2
        }
    })
}

fn benchmark(testname: &str, func: fn(BidiVec<u8>) -> (), vec: BidiVec<u8>) -> u128 {
    let mut total_duration = 0u128;

    for it in 1..=ITERATIONS {
        let v = vec.clone();
        println!(
            "\tBenchmarking {}, iteration {}/{}...",
            testname, it, ITERATIONS
        );
        let start = Instant::now();
        func(v);
        let duration = start.elapsed().as_millis();
        total_duration += duration;
        println!("\t\t{}ms", duration);
    }

    total_duration / (ITERATIONS as u128)
}

fn pathfind_djikstra(map: BidiVec<u8>) {
    pathfind_to_dest(
        &map,
        (0, 0),
        (map.width() - 1, map.height() - 1),
        BidiNeighbours::Adjacent,
        |_, _, to, _| Some(*to as u32),
    )
    .unwrap();
}

fn flood_fill(mut map: BidiVec<u8>) {
    let x = map.width() / 2;
    let y = map.height() / 2;

    editing::flood_fill(
        &mut map,
        (x, y),
        BidiNeighbours::Bordering,
        |_, a, b| a == b,
        |b, _| {
            *b = 255;
        },
    )
    .unwrap();
}

pub fn main() -> Result<(), BidiError> {
    let mut results = Vec::new();

    let sizes = [50, 100, 200, 500, 1000, 2000, 5000];

    for size in sizes {
        let testname = format!("pathfind_djikstra_{}x{}", size, size);
        results.push((
            testname.clone(),
            benchmark(
                &testname,
                pathfind_djikstra,
                create_pathfind_map(size, size),
            ),
        ));

        let testname = format!("flood_fill_whole_{}x{}", size, size);
        results.push((
            testname.clone(),
            benchmark(&testname, flood_fill, create_uniform_map(size, size, 0u8)),
        ));

        let testname = format!("flood_fill_checkerboard_{}x{}", size, size);
        results.push((
            testname.clone(),
            benchmark(
                &testname,
                flood_fill,
                create_checkerboard_map(size, size, 0u8, 255u8),
            ),
        ));
    }

    println!();
    println!();
    println!("=== RESULTS ===");
    println!();
    for res in results.iter() {
        println!("{} = {}ms", res.0, res.1)
    }

    Ok(())
}
