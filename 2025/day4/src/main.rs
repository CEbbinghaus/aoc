use ndarray::Array2;

fn get_cell_value(char: char) -> usize {
    match char {
        '@' => 1,
        '.' => 0,
        _ => panic!("Invalid character"),
    }
}

fn count_neighbours(table: &Array2<char>, x: usize, y: usize, width: usize, height: usize) -> usize {
    let mut count = 0;

    // let mut debug = [[0u8; 3]; 3];
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                let char = table[[ny as usize, nx as usize]];
                // debug[(dy + 1) as usize][(dx + 1) as usize] = char as u8;
                count += get_cell_value(char);
            }
        }
    }
    // println!("Neighbours for ({}, {}) ({} total):", x, y, count);
    // for row in &debug {
    //     println!("{:?}", row);
    // }
    count
}

fn solution1(table: &Array2<char>) -> usize {
    let height = table.nrows();
    let width = table.ncols();

    let mut total = 0;

    for y in 0..height {
        for x in 0..width {
            if table[[y, x]] != '@' {
                continue;
            }

            if count_neighbours(&table, x, y, width, height) < 4 {
                total += 1;
            }
        }
    }
    total
}

fn find_indicies_to_remove(table: &Array2<char>, width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut to_remove = Vec::new();
    for y in 0..height {
        for x in 0..width {
            if table[[y, x]] != '@' {
                continue;
            }

            if count_neighbours(&table, x, y, width, height) < 4 {
                to_remove.push((x, y));
            }
        }
    }
    to_remove
}

#[cfg(debug_assertions)]
fn render_table(table: &Array2<char>, inidicies: &Vec<(usize, usize)>) {
    let height = table.nrows();
    let width = table.ncols();

    for y in 0..height {
        for x in 0..width {
            if inidicies.contains(&(x, y)) {
                print!("x");
            } else {
                print!("{}", table[[y, x]]);
            }
        }
        println!();
    }
    println!();
}

fn solution2(mut table: Array2<char>) -> usize {
    let height = table.nrows();
    let width = table.ncols();

    let mut total_removed = 0;
    let mut indicies = find_indicies_to_remove(&table, width, height);
    while !indicies.is_empty() {
        for (x, y) in &indicies {
            table[[*y, *x]] = '.';
            total_removed += 1;
        }

        #[cfg(debug_assertions)]
        render_table(&table, &indicies);

        indicies = find_indicies_to_remove(&table, width, height);

    }

    total_removed
}

fn main() {
    let file = std::env::args().nth(1).expect("no file given");
    println!("Reading file: {}", file);

    let contents = std::fs::read_to_string(file).expect("could not read file");
    let contents = contents.trim();

    let mut width = 0;

    let data: Vec<char> = contents
        .split('\n')
        .enumerate()
        .flat_map(|(i, line)| {
            let row: Vec<char> = line.chars()
                .collect();

            if i == 0 { width = row.len() }

            row
        })
        .collect();


    let arr = Array2::from_shape_vec((data.len() / width, width), data).unwrap();

    println!("Solution 1: {}", solution1(&arr));
    println!("Solution 2: {}", solution2(arr));
}
