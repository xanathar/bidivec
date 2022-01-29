use bidivec::pathfinding::*;
use bidivec::*;

const WIDTH: usize = 21;
const DATA: &[u8] = b"\
#####################\
#..S#.............#.#\
#.###.#.#.#.#####.#.#\
#...#.#.#.#.....#.#.#\
#.###.#.#.#.#####.#.#\
#...#.#.#.#...#.#...#\
#.#####.#######.#.#.#\
#.#.....#.#.......#.#\
#.#####.#.#####.#####\
#.....#.......#.#.#.#\
#.###.#.#.#######.#.#\
#.#.....#.#.#.#...#.#\
#.#.#######.#.#.###.#\
#.#.......#.........#\
#.###.###.#.#######.#\
#.#...#...#...#.....#\
#.###.###.#######.###\
#.#.#...#...#.#.....#\
#.#.#####.#.#.###.#.#\
#...#.....#.......#D#\
#####################\
";

pub fn main() -> Result<(), BidiError> {
    let map = BidiSlice::new(DATA, WIDTH)?;

    let start = map
        .iter()
        .with_coords()
        .find_map(|(x, y, t)| if *t == b'S' { Some((x, y)) } else { None })
        .unwrap();

    let dest = map
        .iter()
        .with_coords()
        .find_map(|(x, y, t)| if *t == b'D' { Some((x, y)) } else { None })
        .unwrap();

    println!("Without heuristics:");
    let res = pathfind_to_dest(&map, start, dest, BidiNeighbours::Adjacent, pathfind_cost).unwrap();
    print_pathfinding_results(&map, res);
    println!();

    println!("With heuristics:");
    let res = pathfind_to_dest_heuristic(
        &map,
        start,
        dest,
        BidiNeighbours::Adjacent,
        pathfind_cost,
        heuristic,
    )
    .unwrap();
    print_pathfinding_results(&map, res);
    println!();

    println!("Maze was generated using https://www.dcode.fr/maze-generator -- thanks");
    Ok(())
}

fn heuristic(from: (usize, usize), to: (usize, usize)) -> u32 {
    let (sx, sy) = (from.0 as i32, from.1 as i32);
    let (dx, dy) = (to.0 as i32, to.1 as i32);

    ((sx - dx).abs() + (sy - dy).abs()) as u32
}

fn pathfind_cost(_: &u8, _: (usize, usize), to_elem: &u8, _: (usize, usize)) -> Option<u32> {
    if *to_elem == b'#' {
        None
    } else {
        Some(1u32)
    }
}

fn print_pathfinding_results(map: &BidiSlice<u8>, res: PathFindData<u32>) {
    const NORMAL: &str = "\x1b[0m";
    const HIGHLIGHT: &str = "\x1b[1;33m";
    const VISITED: &str = "\x1b[1;31m";

    if let PathFindDataResult::ShortestPathFound(cost) = res.result {
        for y in 0..map.height() {
            print!("\t");
            for x in 0..map.width() {
                if res.tiles[(x, y)].in_shortest_path {
                    print!("{}{}{}", HIGHLIGHT, map[(x, y)] as char, NORMAL);
                } else if res.tiles[(x, y)].cost.is_some() {
                    print!("{}{}{}", VISITED, map[(x, y)] as char, NORMAL);
                } else {
                    print!("{}", map[(x, y)] as char)
                }
            }
            println!();
        }

        let visited = res.tiles.iter().filter(|n| n.cost.is_some()).count();
        println!("Path cost: {}", cost);
        println!("Visited nodes: {}", visited);
        println!("Efficiency: {}%", (cost as f32) / (visited as f32) * 100f32);
    } else {
        println!("Path not found 🤷");
    }
}
