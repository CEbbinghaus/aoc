use std::io::{self, BufRead};

// Just pipe into executable `echo input | cargo run`
fn main() {
    let stdin = io::stdin();

    let mut occurences = 0;
    let mut location = 50;

    for line in stdin.lock().lines().filter_map(|v| v.ok()) {
        if line.is_empty() {
            continue;
        }

        let line = line.trim_start();

        let offset = match line.chars().nth(0).unwrap() {
            'R' => str::parse::<i32>(&line[1..]).expect("valid digits"),
            'L' => -str::parse::<i32>(&line[1..]).expect("valid digits"),
            char => panic!("invalid direction \"{line}\" starting with {char:?}"),
        };

        // This has got to be the most stupid thing I have ever done, But it runs in a second and works so fuck me.
        // Not wasting any more time on this.
        let op = offset.signum();
        for _ in 0..offset.abs() {
            location += op;

            match location {
                0 => occurences += 1,
                -1 => location = 99,
                100 => {
                    occurences += 1;
                    location = 0;
                },
                1..100 => {},
                _ => unreachable!("location {} was out of bounds, due to {}", location, op),
            }
        }
    }

    println!("{}", occurences);
}

