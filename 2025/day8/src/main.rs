use std::{collections::{HashMap, HashSet}, time::Instant};

use itertools::Itertools;

fn get_euclidean_distance(a: &[usize; 3], b: &[usize; 3]) -> f64 {
    let dx = (a[0] as isize - b[0] as isize) as f64;
    let dy = (a[1] as isize - b[1] as isize) as f64;
    let dz = (a[2] as isize - b[2] as isize) as f64;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

fn solution1(boxes: &Vec<[usize; 3]>, num_checks: usize, num_results: usize) -> usize {
    let mut existing: HashSet<(usize, usize)> = HashSet::new();
    let mut circuits = HashMap::new();
    let mut id = 0_u16;

    for _ in 0..num_checks {
        let mut best = (f64::MAX, (0_usize, 0_usize));

        for i in 0..boxes.len() {
            for j in (i + 1)..boxes.len() {
                if existing.contains(&(i, j)) {
                    continue;
                }

                let distance = get_euclidean_distance(&boxes[i], &boxes[j]);
                if distance < best.0 {
                    best = (distance, (i, j));
                }
            }
        }

        existing.insert(best.1);

        match (circuits.get(&best.1.0), (circuits.get(&best.1.1))) {
            (None, None) => {
                circuits.insert(best.1.0, id);
                circuits.insert(best.1.1, id);
                id += 1;
            }
            (Some(&circuit_id), None) | (None, Some(&circuit_id)) => {
                // println!("Adding to existing circuit {}", id);
                circuits.insert(best.1.0, circuit_id);
                circuits.insert(best.1.1, circuit_id);
            }
            (Some(&circuit_id_a), Some(&circuit_id_b)) => {
                // They are part of the same circuit already, we can skip
                if circuit_id_a == circuit_id_b {
                    continue;
                }

                // We now have 2 big circuits we need to merge
                let final_circuit_id = circuit_id_a.min(circuit_id_b);

                for (_, v) in circuits.iter_mut() {
                    if *v == circuit_id_a || *v == circuit_id_b {
                        *v = final_circuit_id;
                    }
                }
            }
        };

        // println!("Current circuits: {:?}", circuits);
    }

    circuits
        .iter()
        .into_group_map_by(|&(_, v)| v)
        .iter()
        .map(|(_, value)| value.len())
        .sorted_by(|a, b| b.cmp(&a))
        .take(num_results)
        .reduce(|a, b| a * b)
        .unwrap()
}

fn solution2(boxes: &Vec<[usize; 3]>) -> usize {
    let mut set = HashSet::new();

    for i in 0..boxes.len() {
        for j in (i + 1)..boxes.len() {
            set.insert((i, j));
        }
    }

    let mut circuits = HashMap::new();
    let mut id = 0_u16;

    let mut last = (0_usize, 0_usize);

    while set.len() > 0 {
        let mut best = (f64::MAX, (0_usize, 0_usize));

        for (i, j) in set.iter() {
            let distance = get_euclidean_distance(&boxes[*i], &boxes[*j]);
            if distance < best.0 {
                best = (distance, (*i, *j));
            }
        }

        set.remove(&best.1);

        last = best.1;

        match (circuits.get(&best.1.0), (circuits.get(&best.1.1))) {
            (None, None) => {
                circuits.insert(best.1.0, id);
                circuits.insert(best.1.1, id);
                id += 1;
            }
            (Some(&circuit_id), None) | (None, Some(&circuit_id)) => {
                // println!("Adding to existing circuit {}", id);
                circuits.insert(best.1.0, circuit_id);
                circuits.insert(best.1.1, circuit_id);

                circuits
                    .iter()
                    .filter_map(|(k, v)| if circuit_id == *v { Some(*k) } else { None })
                    .for_each(|id| {
                        set.remove(&(best.1.0, id));
                        set.remove(&(id, best.1.0));
                        set.remove(&(best.1.1, id));
                        set.remove(&(id, best.1.1));
                    });
            }
            (Some(&circuit_id_a), Some(&circuit_id_b)) => {
                // They are part of the same circuit already, we can skip
                if circuit_id_a == circuit_id_b {
                    continue;
                }

                // We first remove all potential connections between the two circuits from the search space
                // Without this 99% of our runtime is spent checking items against other items within the same circuit
                for id_a in circuits
                    .iter()
                    .filter_map(|(k, v)| if circuit_id_a == *v { Some(*k) } else { None })
                {
                    for id_b in circuits
                        .iter()
                        .filter_map(|(k, v)| if circuit_id_b == *v { Some(*k) } else { None })
                    {
                        set.remove(&(id_a, id_b));
                        set.remove(&(id_b, id_a));
                    }
                }

                // We now have 2 big circuits we need to merge
                let final_circuit_id = circuit_id_a.min(circuit_id_b);
                for (_, v) in circuits.iter_mut() {
                    if *v == circuit_id_a || *v == circuit_id_b {
                        *v = final_circuit_id;
                    }
                }
            }
        };
    }

    boxes[last.0][0] * boxes[last.1][0]
}

fn main() {
    let file = std::env::args().nth(1).expect("no file given");
    println!("Reading file: {}", file);

    let contents = std::fs::read_to_string(file).expect("could not read file");
    let contents = contents.trim_end_matches('\n');
    let lines = contents
        .split('\n')
        .map(|line| {
            line.split(',')
                .map(|part| part.parse().unwrap())
                .collect::<Vec<usize>>()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<[usize; 3]>>();


    // time each solution:

    let start = Instant::now();
    println!("Solution 1: {} in {:?}", solution1(&lines, 1000, 3), start.elapsed());
    let start = Instant::now();
    println!("Solution 2: {} in {:?}", solution2(&lines), start.elapsed());
}
