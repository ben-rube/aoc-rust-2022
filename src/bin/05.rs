use itertools::Itertools;

pub fn part_one(input: &str) -> Option<String> {
    let (boxes, places) = input.split_once("\n\n").unwrap();
    let mut col = vec![vec![]; 9];
    for l in boxes.lines().rev().skip(1).map(str::as_bytes) {
        for i in 0..col.len() {
            let c = l[i * 4 + 1];
            if c.is_ascii_alphabetic() {
                col[i].push(c as char);
            }
        }
    }
    let rules = places
        .lines()
        .map(|l| {
            l.split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok())
                .collect_tuple()
                .unwrap()
        })
        .collect::<Vec<_>>();
    Some(calculate_stacks(&rules, col))
}

pub fn part_two(input: &str) -> Option<String> {
    let (boxes, places) = input.split_once("\n\n").unwrap();
    let mut col = vec![vec![]; 9];
    for l in boxes.lines().rev().skip(1).map(str::as_bytes) {
        for i in 0..col.len() {
            let c = l[i * 4 + 1];
            if c.is_ascii_alphabetic() {
                col[i].push(c as char);
            }
        }
    }
    let rules = places
        .lines()
        .map(|l| {
            l.split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok())
                .collect_tuple()
                .unwrap()
        })
        .collect::<Vec<_>>();
    Some(calculate_stacks_2(&rules, col))
}

fn calculate_stacks(instructions: &[(usize, usize, usize)], mut stacks: Vec<Vec<char>>) -> String {
    for &(times, from, to) in instructions {
        for _ in 0..times {
            let item = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(item);
        }
    }
    stacks.iter().map(|s| s.last().unwrap()).join("")
}

fn calculate_stacks_2(
    instructions: &[(usize, usize, usize)],
    mut stacks: Vec<Vec<char>>,
) -> String {
    for &(times, from, to) in instructions {
        let len = stacks[to - 1].len() + times;
        stacks[to - 1].resize(len, 'x');
        for i in 0..times {
            let item = stacks[from - 1].pop().unwrap();
            stacks[to - 1][len - 1 - i] = item;
        }
    }
    stacks.iter().map(|s| s.last().unwrap()).join("")
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}
