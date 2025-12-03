fn solution1(ranges: &Vec<&str>) -> u64 {
    let mut total: u64 = 0;
    for range in ranges {
        let bounds = range
            .split_once('-')
            .expect("invalid range");
        let start: u64 = bounds.0.parse().expect("invalid start");
        let end: u64 = bounds.1.parse().expect("invalid end");
        
        for n in start..=end {
            let str = n.to_string();
            let halflen = str.len() / 2;
            
            if str[..halflen] == str[halflen..] {
                println!("Found pattern in number: {}", n);
                total += n as u64;
            }
        }
    }
    total
}

fn find_pattern(str: &str) -> bool {
    let halflen = str.len() / 2;

    for i in 1..halflen+1 {
        let pattern = &str[..i];
        if str.split(pattern).all(|v| v.is_empty()) {
            return true;
        }
    }
    false
}

fn solution2(ranges: &Vec<&str>) -> u64 {
    let mut total: u64 = 0;
    for range in ranges {
        let bounds = range
            .split_once('-')
            .expect("invalid range");
        let start: u64 = bounds.0.parse().expect("invalid start");
        let end: u64 = bounds.1.parse().expect("invalid end");
        
        for n in start..=end {
            if find_pattern(&n.to_string()) {
                println!("Found pattern in number: {}", n);
                total += n as u64;
            }
        }
    }
    total
}

// Just pipe into standard in `echo input | cargo run`
fn main() {
    let file = std::env::args().nth(1).expect("no file given");
    println!("Reading file: {}", file);

    let contents = std::fs::read_to_string(file).expect("could not read file");
    let contents = contents.trim();
    
    let ranges = contents.split(',').collect::<Vec<&str>>();

    println!("Solution 1 Total: {}", solution1(&ranges));
    println!("Solution 2 Total: {}", solution2(&ranges));
}
