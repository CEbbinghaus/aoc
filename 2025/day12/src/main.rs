use std::time::Instant;

struct Shape {
    shape: [[bool; 3]; 3],
    surface: usize,
}

enum ShapeFlip {
    None,
    Flipped,
}

enum ShapeRotation {
    Deg0,
    Deg90,
    Deg180,
    Deg270,
}

struct ShapePermutation<'a>(ShapeFlip, ShapeRotation, &'a Shape);

impl<'a> ShapePermutation<'a> {
    fn get_permuted_grid(&self) -> [[bool; 3]; 3] {
        let mut grid = self.2.shape;

        match self.0 {
            ShapeFlip::None => {}
            ShapeFlip::Flipped => {
                for i in 0..3 {
                    for j in 0..3 {
                        grid[i][j] = self.2.shape[i][2 - j];
                    }
                }
            }
        }

        match self.1 {
            ShapeRotation::Deg0 => grid,
            ShapeRotation::Deg90 => {
                let mut new_grid = [[false; 3]; 3];
                for i in 0..3 {
                    for j in 0..3 {
                        new_grid[j][2 - i] = grid[i][j];
                    }
                }
                new_grid
            }
            ShapeRotation::Deg180 => {
                let mut new_grid = [[false; 3]; 3];
                for i in 0..3 {
                    for j in 0..3 {
                        new_grid[2 - i][2 - j] = grid[i][j];
                    }
                }
                new_grid
            }
            ShapeRotation::Deg270 => {
                let mut new_grid = [[false; 3]; 3];
                for i in 0..3 {
                    for j in 0..3 {
                        new_grid[2 - j][i] = grid[i][j];
                    }
                }
                new_grid
            }
        }
    }
}

fn apply_shape_permutation(
    grid: &mut Vec<Vec<bool>>,
    shape_perm: &ShapePermutation,
    top_left: (usize, usize),
) {
    let permuted_grid = shape_perm.get_permuted_grid();

    for i in 0..3 {
        for j in 0..3 {
            if permuted_grid[i][j] {
                grid[top_left.0 + i][top_left.1 + j] = true;
            }
        }
    }
}

fn parse_shape(data: &str) -> Shape {
    let (_, data) = data.split_once("\n").unwrap();

    let shape: [[bool; 3]; 3] = data
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c == '#')
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let surface = shape.iter().flatten().filter(|&&b| b).count();

    Shape { shape, surface }
}

fn solution1(shapes: &Vec<Shape>, trees: &Vec<&str>) -> usize {
    let trees = trees
        .iter()
        .map(|&tree| {
            let (size, shapes_required) = tree.split_once(": ").unwrap();

            let total_size = size
                .split("x")
                .map(|x| x.parse::<usize>().unwrap())
                .product::<usize>();

            let shape_indicies = shapes_required
                .split(" ")
                .map(|shape| shape.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            (total_size, shape_indicies)
        })
        .collect::<Vec<_>>();

    trees.iter().filter(|(total, presents)| *total >= presents.iter().sum::<usize>() * 9).count()
}

fn main() {
    let file = std::env::args().nth(1).expect("no file given");
    println!("Reading file: {}", file);

    let contents = std::fs::read_to_string(file).expect("could not read file");
    let contents = contents.trim_end_matches('\n');

    let mut blocks = contents.split("\n\n").collect::<Vec<_>>();

    let trees = blocks
        .pop()
        .unwrap()
        .split('\n')
        .map(|line| line.trim())
        .collect::<Vec<_>>();

    let shapes = blocks
        .iter()
        .map(|&data| parse_shape(data))
        .collect::<Vec<_>>();

    let start = Instant::now();
    println!(
        "Solution 1: {} in {:?}",
        solution1(&shapes, &trees),
        start.elapsed()
    );
}
