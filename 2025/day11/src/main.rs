use std::{
    collections::{HashMap, HashSet},
    time::Instant,
    vec,
};

fn solution1(lines: &Vec<&str>) -> usize {
    let mut outs = HashSet::new();
    let mut map = HashMap::new();

    let mut starts = vec![];

    for line in lines.iter() {
        let (a, b) = line.split_once(":").unwrap();
        let (a, b) = (a.trim().to_string(), b.trim());
        let mut outputs = b.split(' ').map(|v| v.trim()).collect::<Vec<_>>();

        if a == "you" {
            starts.append(&mut outputs);
            continue;
        }

        if outputs[..] == ["out"] {
            outs.insert(a.clone());
            continue;
        }

        map.insert(a, outputs);
    }

    let mut paths = starts.iter().map(|s| s.to_string()).collect::<Vec<_>>();

    let mut total = 0_usize;

    while let Some(path) = paths.pop() {
        let next = map.get(&path).unwrap();

        for p in next.iter() {
            if outs.contains(*p) {
                total += 1;
            } else {
                paths.push(p.to_string());
            }
        }
    }

    total
}

fn solution2(lines: &Vec<&str>) -> usize {
    let mut map = HashMap::new();
    for line in lines.iter() {
        let (a, b) = line.split_once(":").unwrap();
        let (a, b) = (a.trim(), b.trim());
        let outputs = b.split(' ').map(|v| v.trim()).collect::<Vec<_>>();

        map.insert(a, outputs);
    }

    fn dfs<'a>(
        node: &'a str,
        map: &'a HashMap<&'a str, Vec<&'a str>>,
        memo: &'_ mut HashMap<(&'a str, bool, bool), usize>,
        state: (bool, bool),
    ) -> usize {
        let (is_fft, is_dac) = (state.0 || node == "fft", state.1 || node == "dac");

        if node == "out" {
            return (is_fft && is_dac) as usize;
        }

        if let Some(paths) = memo.get(&(node, is_fft, is_dac)) {
            return *paths;
        }

        let mut total = 0_usize;

        for neighbor in map
            .get(node)
            .expect(&format!("Node {} not found in map", node))
            .iter()
        {
            total += dfs(neighbor, map, memo, (is_fft, is_dac));
        }

        memo.insert((node, is_fft, is_dac), total);
        total
    }

    let mut memo = HashMap::new();

    dfs("svr", &map, &mut memo, (false, false))
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
