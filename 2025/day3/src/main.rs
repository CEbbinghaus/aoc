use std::ops::Range;


fn char_to_num(c: char) -> u32 {
    match c {
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => unreachable!(),
    }
}

fn find_largest_digit_index(range: &[u8]) -> usize {
    let mut idx = 0_usize;
    let mut max = 0;
    for (indx, c) in range.iter().enumerate() {
        let value = char_to_num(*c as char);
        if value > max {
            max = value;
            idx = indx;
        }
    };
    idx
}

fn solution1(ranges: &Vec<&str>) -> u32 {
    let mut total = 0_u32;

    for range in ranges {
        let range = range.as_bytes();
        let largest_digit_indx = find_largest_digit_index(&range[..range.len() - 1]);
        let second_largest_digit_indx = largest_digit_indx + 1 + find_largest_digit_index(&range[largest_digit_indx+1..]);
        
        let sum = (char_to_num(range[largest_digit_indx] as char) * 10) + char_to_num(range[second_largest_digit_indx] as char);
        total += sum;
    }

    total
}


fn get_largest_digit_index_in_range(data: &[u8], range: Range<usize>) -> usize {
    let data = &data[range.clone()];
    range.start + find_largest_digit_index(data)
}

fn solution2(ranges: &Vec<&str>) -> u64 {
    const TOTAL: usize = 12;
    
    let mut total = 0_u64;

    for range in ranges {
        let mut indexes = [0; TOTAL];
        
        for i in 0..TOTAL {
            let start = if i == 0 { 0 } else { indexes[i - 1] + 1 };
            let end = range.len() - (TOTAL - i - 1);

            indexes[i] = get_largest_digit_index_in_range(range.as_bytes(), start..end);
        }

        total += {
            let mut exponent = 1;
            let mut sum = 0_u64;
            for i in (0..TOTAL).rev() {
                sum += char_to_num(range.as_bytes()[indexes[i]] as char) as u64 * exponent;
                exponent *= 10;
            }
            sum
        };
    }

    total
}

fn main() {
    let file = std::env::args().nth(1).expect("no file given");
    println!("Reading file: {}", file);

    let contents = std::fs::read_to_string(file).expect("could not read file");
    let contents = contents.trim();
    let ranges = contents.split('\n').collect::<Vec<&str>>();

    println!("Solution 1 Total: {}", solution1(&ranges));
    println!("Solution 2 Total: {}", solution2(&ranges));
}
