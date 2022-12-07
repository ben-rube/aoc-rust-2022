use std::{collections::HashMap, path::PathBuf, str::FromStr};

enum FileStructure {
    Dir(PathBuf),
    File(usize),
}

pub fn part_one(input: &str) -> Option<usize> {
    let state = parse_input(input);
    Some(
        state
            .iter()
            .map(|e| calculate_size(&state, e.0))
            .filter(|size| *size < 100000)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let state = parse_input(input);
    Some(delete_space(state))
}

fn calculate_size(tree: &HashMap<PathBuf, Vec<FileStructure>>, path: &PathBuf) -> usize {
    tree[path]
        .iter()
        .map(|e| match e {
            FileStructure::Dir(path) => calculate_size(tree, path),
            FileStructure::File(size) => *size,
        })
        .sum()
}

fn delete_space(state: HashMap<PathBuf, Vec<FileStructure>>) -> usize {
    let total_space = 70000000 - calculate_size(&state, &PathBuf::from_str("/").unwrap());
    state
        .iter()
        .map(|p| calculate_size(&state, p.0))
        .filter(|file_size| *file_size >= 30000000 - total_space)
        .min()
        .unwrap()
}

fn parse_input(input: &str) -> HashMap<PathBuf, Vec<FileStructure>> {
    let mut state: HashMap<PathBuf, Vec<FileStructure>> = HashMap::new();
    let mut in_dir = PathBuf::from_str("/").unwrap();

    for line in input.trim().lines() {
        if line.starts_with("$ cd ..") {
            in_dir.pop();
            continue;
        }

        if line.starts_with("$ ls") {
            continue;
        }

        if line.starts_with("$ cd") {
            // Get value after "$ cd "
            in_dir.push(line[5..].to_string());
            state.entry(in_dir.clone()).or_default();
        }

        if line.starts_with("dir") {
            // Gets vales after "dir "
            let name = &line[4..];
            let dir = in_dir.join(name);

            state.entry(dir.clone()).or_default();
            state
                .get_mut(&in_dir)
                .unwrap()
                .push(FileStructure::Dir(dir))
        } else {
            let file = line
                .split_once(' ')
                .map(|(size, _)| size.parse::<usize>().unwrap_or(0))
                .unwrap();
            state
                .get_mut(&in_dir)
                .unwrap()
                .push(FileStructure::File(file));
        }
    }
    state
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
