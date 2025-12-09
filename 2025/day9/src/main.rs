use std::time::Instant;

use itertools::Itertools;

#[derive(Debug)]
struct Tile {
    x: usize,
    y: usize,
}

impl Tile {
    fn new(x: usize, y: usize) -> Self {
        Tile { x, y }
    }

    fn get_size(tile_a: &Tile, tile_b: &Tile) -> usize {
        let dx = (tile_a.x as isize - tile_b.x as isize).abs() as usize + 1;
        let dy = (tile_a.y as isize - tile_b.y as isize).abs() as usize + 1;
        dx * dy
    }

    fn contains(tile_a: &Tile, tile_b: &Tile, tile_c: &Tile) -> bool {
        let min_x = tile_a.x.min(tile_b.x);
        let max_x = tile_a.x.max(tile_b.x);
        let min_y = tile_a.y.min(tile_b.y);
        let max_y = tile_a.y.max(tile_b.y);

        tile_c.x > min_x && tile_c.x < max_x && tile_c.y > min_y && tile_c.y < max_y
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl From<&str> for Tile {
    fn from(s: &str) -> Self {
        let (x, y) = s.split_once(',').unwrap();
        Tile {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}

fn solution1(tiles: &Vec<Tile>) -> usize {
    let mut max = (usize::MIN, (0_usize, 0_usize));

    for (i, j) in tiles.iter().tuple_combinations::<(_, _)>() {
        let size = Tile::get_size(i, j);
        if size > max.0 {
            max = (size, (i.x, i.y));
        }
    }

    max.0
}

#[derive(Debug)]
enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Debug)]
struct Line<'a> {
    start: &'a Tile,
    end: &'a Tile,
    orientation: Orientation,
    edge: usize,
}

impl<'a> Line<'a> {
    fn from_tiles(a: &'a Tile, b: &'a Tile) -> Self {
        let (orientation, edge) = if a.x == b.x {
            (Orientation::Vertical, a.x)
        } else {
            (Orientation::Horizontal, a.y)
        };

        Line {
            start: a,
            end: b,
            orientation,
            edge,
        }
    }

    fn intersect(line_a: &Line, line_b: &Line) -> bool {
        fn between(v: usize, a: usize, b: usize) -> bool {
            v > a.min(b) && v < a.max(b)
        }

        match (&line_a.orientation, &line_b.orientation) {
            (Orientation::Horizontal, Orientation::Vertical) => {
                let x = line_b.edge;
                let y = line_a.edge;

                between(x, line_a.start.x, line_a.end.x) && between(y, line_b.start.y, line_b.end.y)
            }
            (Orientation::Vertical, Orientation::Horizontal) => Line::intersect(line_b, line_a),
            _ => false,
        }
    }
}

// This is NOT a correct equality check
// Instead we check if either the start or end of one line
// matches either the start or end of the other line
// So they could be neighboring OR identical
impl PartialEq for Line<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start
            || self.start == other.end
            || self.end == other.start
            || self.end == other.end
    }
}

// Just a helper to sort blocks by size
fn cmp_block(a: &(&Tile, &Tile), b: &(&Tile, &Tile)) -> std::cmp::Ordering {
    let size_a = Tile::get_size(a.0, a.1);
    let size_b = Tile::get_size(b.0, b.1);
    size_a.cmp(&size_b)
}

fn solution2(tiles: &Vec<Tile>) -> usize {
    let lines: Vec<_> = tiles
        .iter()
        .circular_tuple_windows::<(_, _)>()
        .map(|(a, b)| Line::from_tiles(a, b))
        .collect();

    let blocks: Vec<_> = tiles
        .iter()
        .tuple_combinations::<(_, _)>()
        .sorted_by(|a, b| cmp_block(a, b))
        .rev()
        .collect();

    'search: for (i, j) in blocks {
        let imaginary_tiles = vec![Tile::new(i.x, j.y), Tile::new(j.x, i.y)];
        let block_lines = vec![
            Line::from_tiles(i, &imaginary_tiles[0]),
            Line::from_tiles(&imaginary_tiles[0], j),
            Line::from_tiles(j, &imaginary_tiles[1]),
            Line::from_tiles(&imaginary_tiles[1], i),
        ];

        for line in &lines {
            if block_lines.contains(line) {
                continue;
            }

            if Tile::contains(i, j, line.start) || Tile::contains(i, j, line.end) {
                continue 'search;
            }

            let intersects = block_lines
                .iter()
                .any(|block_line| Line::intersect(block_line, line));

            if intersects {
                continue 'search;
            }
        }

        return Tile::get_size(i, j);
    }

    panic!("No block found");
}

fn main() {
    let file = std::env::args().nth(1).expect("no file given");
    println!("Reading file: {}", file);

    let contents = std::fs::read_to_string(file).expect("could not read file");
    let contents = contents.trim_end_matches('\n');
    let lines = contents
        .split('\n')
        .map(|line| line.into())
        .collect::<Vec<Tile>>();

    let start = Instant::now();
    println!("Solution 1: {} in {:?}", solution1(&lines), start.elapsed());
    let start = Instant::now();
    println!("Solution 2: {} in {:?}", solution2(&lines), start.elapsed());
}
