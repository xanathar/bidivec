use bidivec::pathfinding::*;
use bidivec::*;

const WIDTH: usize = 9;
const DATA: &[u8] = b"\
#########\
#..S....#\
#.#####.#\
#.......#\
#.#.###.#\
#...#.#.#\
#########\
";

pub fn main() -> Result<(), BidiError> {
    let map = BidiSlice::new(DATA, WIDTH)?;

    // Find the source point
    let start = map
        .iter()
        .with_coords()
        .find_map(|(x, y, t)| if *t == b'S' { Some((x, y)) } else { None })
        .unwrap();

    // Find the shortest path
    let res = pathfind_to_whole(&map, start, BidiNeighbours::Adjacent, |_, _, to, _| {
        if *to == b'#' {
            None
        } else {
            Some(1u32)
        }
    })
    .unwrap();

    // Print the result of the path
    const NORMAL: &str = "\x1b[0m";
    const VISITED: &str = "\x1b[1;32m";
    const UNREACHABLE: &str = "\x1b[1;31m";
    const START: &str = "\x1b[1;35m";

    for y in 0..map.height() {
        for x in 0..map.width() {
            match map[(x, y)] {
                b'.' => {
                    if let Some(cost) = res.tiles[(x, y)].cost {
                        let chr = (cost as u8 + b'0') as char;
                        print!("{}{}{}", VISITED, chr, NORMAL);
                    } else {
                        print!("{}!{}", UNREACHABLE, NORMAL);
                    }
                }
                b'S' => print!("{}S{}", START, NORMAL),
                c => print!("{}", c as char),
            }
        }
        println!();
    }

    Ok(())
}
