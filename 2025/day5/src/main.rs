use std::ops::{RangeInclusive};

fn parse_ranges(fresh_ranges: &Vec<&str>) -> Vec<RangeInclusive<usize>> {
    let mut ranges = Vec::new();

    for range in fresh_ranges {
        let parts: Vec<&str> = range.split('-').collect();
        if parts.len() == 2 {
            if let (Ok(start), Ok(end)) = (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
                ranges.push(start..=end);
            }
        }
    }

    ranges.sort_by(|a, b| a.start().cmp(&b.start()));
    ranges
}

fn solution1(fresh_ingredients: &Vec<&str>, ingredients: &Vec<&str>) -> usize {
    let ranges = parse_ranges(fresh_ingredients);

    let mut total = 0_usize;
    for ingredient in ingredients {
        let ingredient = ingredient.parse::<usize>().unwrap();

        let mut iter = ranges.iter();
        'inner: while let Some(range) = iter.next() && *range.start() <= ingredient
        {
            if  range.contains(&ingredient) {
                total += 1;
                break 'inner;
            }
        }
    }
    total
}

fn solution2(fresh_ingredients: &Vec<&str>) -> usize {
    let ranges = parse_ranges(fresh_ingredients);

    // Merge overlapping ranges
    let mut merged_ranges: Vec<RangeInclusive<usize>> = Vec::new();
    for range in ranges {
        // TIL you can get a mutable reference to the last element of a vector
        if let Some(last) = merged_ranges.last_mut() {
            if *last.end() >= *range.start() {
                let new_end = std::cmp::max(*last.end(), *range.end());
                *last = *last.start()..=new_end;
                continue;
            }
        }
        merged_ranges.push(range);
    }

    merged_ranges.iter().map(|r| r.end() - (r.start() - 1)).sum()
}

fn main() {
    let file = std::env::args().nth(1).expect("no file given");
    println!("Reading file: {}", file);

    let contents = std::fs::read_to_string(file).expect("could not read file");
    let contents = contents.trim();

    let (fresh_ingredients, ingredients) =
        contents.split_once("\n\n").expect("could not split input");

    let fresh_ingredients: Vec<&str> = fresh_ingredients.lines().collect();
    let ingredients: Vec<&str> = ingredients.lines().collect();

    println!(
        "Solution 1: {}",
        solution1(&fresh_ingredients, &ingredients)
    );

    println!(
        "Solution 2: {}",
        solution2(&fresh_ingredients)
    );
}
