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

// This is a very rough bounds check since the plotted data
// all falls within one big circle, so we can eliminate
// all boxes that would extend outside that circle
fn is_in_bounds(tile: &Tile) -> bool {
    const CENTER_X: i64 = 50_000;
    const CENTER_Y: i64 = 50_000;
    const RADIUS: i64 = 50_000;

    (tile.x as i64 - CENTER_X).pow(2) + (tile.y as i64 - CENTER_Y).pow(2) <= RADIUS.pow(2)
}

fn solution2(tiles: &Vec<Tile>) -> usize {
    assert!(tiles.iter().all(is_in_bounds));

    let lines: Vec<_> = tiles
        .iter()
        .circular_tuple_windows::<(_, _)>()
        .map(|(a, b)| Line::from_tiles(a, b))
        .collect();

    let blocks: Vec<_> = tiles
        .iter()
        .tuple_combinations::<(_, _)>()
        .filter(|(a, b)| {
            // By applying this filter, We guarantee that only "realistic" blocks are considered
            is_in_bounds(&Tile { x: a.x, y: b.y }) && is_in_bounds(&Tile { x: b.x, y: a.y })
        })
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
