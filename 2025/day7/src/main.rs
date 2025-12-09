use std::time::Instant;

fn solution1(lines: &Vec<&str>) -> usize {
    let height = lines.len();
    let width = lines[0].len();

    assert!(lines.iter().all(|line| line.len() == width));

    // Get the start index
    let start = lines[0].chars().position(|c| c == 'S').unwrap();

    let mut beams = vec![0_usize; width];
    beams[start] = 1;

    let mut total = 0_usize;
    for line_num in 1..height {
        let line = lines[line_num];

        for (i, char) in line.chars().enumerate() {
            match char {
                '^' => {
                    if beams[i] != 0 {
                        total += 1;
                    }
                    beams[i - 1] += 1;
                    beams[i + 1] += 1;
                    beams[i] = 0;
                }
                '.' => {}
                _ => panic!("Unexpected character"),
            }
        }
    }

    total
}

fn solution2(lines: &Vec<&str>) -> usize {
    let height = lines.len();
    let width = lines[0].len();

    assert!(lines.iter().all(|line| line.len() == width));

    // Get the start index
    let start = lines[0].chars().position(|c| c == 'S').unwrap();

    // beams[i] = number of ways to reach column i
    let mut beams = vec![0_usize; width];
    // we insert the starting possibility
    beams[start] = 1;

    for line_num in 1..height {
        let line = lines[line_num];

        for (i, char) in line.chars().enumerate() {
            match char {
                '^' => {
                    let count = beams[i];
                    beams[i - 1] += count;
                    beams[i + 1] += count;
                    beams[i] = 0;
                }
                '.' => {}
                _ => panic!("Unexpected character"),
            }
        }
    }

    beams.iter().sum()
}

fn main() {
    let file = std::env::args().nth(1).expect("no file given");
    println!("Reading file: {}", file);

    let contents = std::fs::read_to_string(file).expect("could not read file");
    let contents = contents.trim_end_matches('\n');

    let lines = contents.split('\n').collect::<Vec<&str>>();

    let start = Instant::now();
    println!("Solution 1: {} in {:?}", solution1(&lines), start.elapsed());
    let start = Instant::now();
    println!("Solution 2: {} in {:?}", solution2(&lines), start.elapsed());
}
