use bidivec::*;
use std::collections::VecDeque;
use std::iter::FromIterator;

// 🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄
// Solves the 11th advent-of-code 2021 challenge.
// See the exercise at https://adventofcode.com/2021/day/11
// 🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄

// Your data may vary; adjust WIDTH and DATA accordingly.
const WIDTH: usize = 10;
const DATA: &[u8] = b"\
4341347643\
5477728451\
2322733878\
5453762556\
2718123421\
4237886115\
5631617114\
2217667227\
4236581255\
4482627641\
";

pub fn main() -> Result<(), BidiError> {
    println!(
        "🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄"
    );
    println!("Solves the 11th advent-of-code 2021 challenge.");
    println!("See the exercise at https://adventofcode.com/2021/day/11");
    println!(
        "🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄"
    );
    println!();

    println!();
    println!("=== PART 1 ===");
    part1()?;

    println!();
    println!("=== PART 2 ===");
    part2()?;

    Ok(())
}

fn part1() -> Result<(), BidiError> {
    let map = BidiSlice::new(DATA, WIDTH)?;
    let mut map = BidiArray::from_view_map(&map, |v| (*v - b'0') as u32);

    let res: u32 = (1..=100).map(|_| step(&mut map)).sum();

    println!("{}", res);

    Ok(())
}

fn part2() -> Result<(), BidiError> {
    let map = BidiSlice::new(DATA, WIDTH)?;
    let mut map = BidiArray::from_view_map(&map, |v| (*v - b'0') as u32);

    let res: u64 = (1..std::u64::MAX).find(|_| step(&mut map) == 100).unwrap();

    println!("{}", res);

    Ok(())
}

pub fn step(map: &mut BidiArray<u32>) -> u32 {
    // First, the energy level of each octopus increases by 1.
    for octopus in map.iter_mut() {
        *octopus += 1;
    }

    // Then, any octopus with an energy level greater than 9 flashes.
    // This increases the energy level of all adjacent octopuses by 1,
    // including octopuses that are diagonally adjacent.
    // If this causes an octopus to have an energy level greater than 9,
    // it also flashes. This process continues as long as new octopuses
    // keep having their energy level increased beyond 9.
    let mut flashqueue = VecDeque::from_iter(
        map.iter()
            .with_coords()
            .filter(|(_, _, o)| **o == 10)
            .map(|(x, y, _)| (x, y)),
    );

    while let Some((x, y)) = flashqueue.pop_front() {
        for (nx, ny, octopus) in
            map.iter_mut()
                .with_coords()
                .on_neighbours(x, y, BidiNeighbours::Bordering)
        {
            *octopus += 1;
            if *octopus == 10 {
                flashqueue.push_back((nx, ny));
            }
        }
    }

    // Then, Finally, any octopus that flashed during this step has its energy level set to 0
    let mut flashcount = 0;
    for octopus in map.iter_mut().filter(|o| **o >= 10) {
        flashcount += 1;
        *octopus = 0;
    }

    flashcount
}
