use itertools::{Itertools, MinMaxResult};
use priority_queue::PriorityQueue;
use std::{cmp::Reverse, time::Instant};

#[derive(Eq, PartialEq, Hash, Clone)]
struct NodeS1 {
    state: Vec<bool>,
    presses: usize,
}

fn find_min_button_presses_s1(pattern: &str, buttons: &Vec<Vec<usize>>) -> usize {
    let desired_state = vec![false; pattern.len()];

    let mut pq = PriorityQueue::new();

    pq.push(
        NodeS1 {
            state: pattern.chars().map(|c| c == '#').collect::<Vec<bool>>(),
            presses: 0,
        },
        Reverse(0_usize),
    );

    while let Some((node, _)) = pq.pop() {

        if node.state == desired_state {
            return node.presses;
        }

        for button in buttons.iter() {
            let mut new_state = node.state.clone();
            for &index in button.iter() {
                new_state[index] = !new_state[index];
            }

            pq.push(
                NodeS1 {
                    state: new_state,
                    presses: node.presses + 1,
                },
                Reverse(node.presses + 1),
            );
        }
    }

    panic!("Unable to find solution")
}

fn solution1(lines: &Vec<&str>) -> usize {
    let machines: Vec<_> = lines
        .iter()
        .map(|line| {
            // We can ignore the jolt requirements for this problem
            let end = line.chars().position(|c| c == '{').unwrap() - 1;

            let pattern_end = line.chars().position(|c| c == ']').unwrap();
            let pattern = &line[1..pattern_end];

            let buttons = line[pattern_end + 1..end]
                .trim()
                .split(' ')
                .map(|v| {
                    let v = &v[1..v.len() - 1];
                    v.split(',')
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>();

            (pattern, buttons)
        })
        .collect();

    machines
        .iter()
        .map(|(pattern, buttons)| find_min_button_presses_s1(pattern, buttons))
        .sum()
}

fn apply_button_to_state(state: &Vec<isize>, button: &Vec<usize>) -> Vec<isize> {
    let mut new_state = state.clone();
    for &v in button {
        new_state[v] -= 1;
    }
    new_state
}

struct NodeS2<'a> {
    state: Vec<isize>,
    presses: usize,
    available_buttons: &'a [Vec<usize>],
}

fn find_min_button_presses_s2(joltages: &Vec<usize>, buttons: &Vec<Vec<usize>>) -> usize {
    let mut best = usize::MAX;
    let initial_state: Vec<isize> = joltages.iter().map(|&x| x as isize).collect();

    let mut stack = vec![];

    stack.push(NodeS2 {
        state: initial_state,
        presses: 0,
        available_buttons: buttons.as_slice(),
    });

    'search: while let Some(NodeS2 {
        state,
        presses,
        available_buttons,
    }) = stack.pop()
    {
        let MinMaxResult::MinMax(&min, &max) = state.iter().minmax() else {
            panic!();
        };

        // We have gone over, end of the line
        if min < 0 {
            continue;
        }

        // Done, All joltages are zero
        if max == 0 {
            best = (best).min(presses);
            continue;
        }

        // We cannot beat the current best solution. So why even try
        if presses + (max as usize) >= best {
            continue;
        }

        // This optimization tries to focus on buttons that decrease high joltages while leaving low joltages alone
        // Its the only reason this thing runs in any reasonable amount of time
        for (i, j) in (0..state.len()).tuple_combinations::<(_, _)>() {
            if state[i] == state[j] {
                continue;
            }

            let (i, j) = if state[i] > state[j] { (i, j) } else { (j, i) };

            let helpful_buttons: Vec<usize> = available_buttons
                .iter()
                .enumerate()
                .filter_map(|(idx, b)| {
                    if b.contains(&i) && !b.contains(&j) {
                        Some(idx)
                    } else {
                        None
                    }
                })
                .collect();

            match helpful_buttons[..] {
                // No button will help us reach our goal. This means we are already
                // in a dead end since all possibilities from here on out
                // decrease both i and j together
                [] => continue 'search,
                // There is only one button that will help reach the goal
                // we MUST press it and can ignore everything else (for now)
                [button] => {
                    stack.push(NodeS2 {
                        state: apply_button_to_state(&state, &available_buttons[button]),
                        presses: presses + 1,
                        available_buttons,
                    });
                    continue 'search;
                }
                _ => {}
            }
        }

        for (i, button) in available_buttons.iter().enumerate().rev() {
            stack.push(NodeS2 {
                state: apply_button_to_state(&state, button),
                presses: presses + 1,
                available_buttons: &available_buttons[i..],
            });
        }
    }

    assert!(best != usize::MAX);

    best
}

fn solution2(lines: &Vec<&str>) -> usize {
    let machines: Vec<_> = lines
        .iter()
        .map(|line| {
            // We can ignore the jolt requirements for this problem
            let start = line.chars().position(|c| c == ']').unwrap() + 1;

            let joltage_start = line.chars().position(|c| c == '{').unwrap();

            let buttons = line[start..joltage_start]
                .trim()
                .split(' ')
                .map(|v| {
                    let v = &v[1..v.len() - 1];
                    v.split(',')
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>();

            let joltages = &line[joltage_start + 1..line.len() - 1];
            let joltages = joltages
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            (joltages, buttons)
        })
        .collect();

    machines
        .iter()
        .map(|(pattern, buttons)| find_min_button_presses_s2(pattern, buttons))
        .sum()
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
