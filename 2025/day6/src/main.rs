use std::time::Instant;

// returns the numeric value of a character '0' to '9'
fn get_value_from_char(char: u8) -> u8 {
    return char - 48;
}

fn parse_number_from_span(span: &[u8]) -> usize {
    let mut num = 0_usize;

    for i in 0..span.len() {
        let cell = span[i];
        // skip all spaces before and after the number
        if cell == b' ' {
            continue;
        }
        let value = get_value_from_char(cell);
        num = num * 10 + value as usize;
    }

    num
}

fn solution1(nums: &Vec<&[u8]>) -> usize {
    let height = nums.len();
    let width = nums[0].len();
    
    assert!(nums.iter().all(|row| row.len() == width), "All rows must have the same length");

    let mut total = 0_usize;

    let mut i = 0;
    
    while i < width {
        let operator = nums[height - 1][i];

        assert!(operator != b' ', "Operator cannot be a space at column {}", i);

        let span_width = nums[height - 1][(i + 1)..]
            .iter()
            .position(|&c| c != b' ')
            .unwrap_or(width - i);

        let mut num = match operator {
            b'+' => 0_usize,
            b'*' => 1_usize,
            _ => panic!("Unknown operator: {}", operator as char),
        };

        for j in 0..height - 1 {
            let span = &nums[j][i..(i + span_width)];

            let value = parse_number_from_span(span);
            
            match operator {
                b'+' => num += value,
                b'*' => num *= value,
                _ => panic!("Unknown operator: {}", operator),
            }
        }
        total += num;
        i += span_width + 1;
    }

    total
}

fn solution2(lines: &Vec<&[u8]>) -> usize {
    let height = lines.len();
    let width = lines[0].len();

    assert!(lines.iter().all(|line| line.len() == width), "All lines must have the same length");

    let mut total = 0_usize;

    let mut nums = Vec::new();

    for i in (0..width).rev() {
        if lines[height - 1][usize::min(i + 1, width - 1)] != b' ' {
            continue;
        }

        let mut num = 0_usize;
        for j in 0..(height - 1) {
            let cell = lines[j][i];

            if cell == b' ' {
                continue;
            }

            let value = cell - 48;
            num = num * 10 + value as usize;
        }

        nums.push(num);
        
        let operator = lines[height - 1][i];

        if operator == b' ' {
            continue;
        }

        // println!("Processing column {}: operator = {}, nums = {:?}", i, operator as char, nums);

        total += nums.into_iter().reduce(|acc, x| {
            match operator {
                b'+' => acc + x,
                b'*' => acc * x,
                _ => panic!("Unknown operator: {}", operator as char),
            }
        }).unwrap();

        nums = Vec::new();
    }

    total
}

fn main() {
    let file = std::env::args().nth(1).expect("no file given");
    println!("Reading file: {}", file);

    let contents = std::fs::read_to_string(file).expect("could not read file");
    let contents = contents.trim_end_matches('\n');

    let lines = contents.split('\n').map(|line| line.as_bytes()).collect::<Vec<&[u8]>>();

    let start = Instant::now();
    println!("Solution 1: {} in {:?}", solution1(&lines), start.elapsed());
    let start = Instant::now();
    println!("Solution 2: {} in {:?}", solution2(&lines), start.elapsed());
}
