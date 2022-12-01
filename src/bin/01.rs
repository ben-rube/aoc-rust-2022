pub fn part_one(input: &str) -> Option<usize> {
    let result = core_processing(input);
    Some(result[0])
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut result = core_processing(input);
    result.sort_by(|a, b| b.cmp(a));
    Some(result.iter().take(3).sum())
}

fn core_processing(input: &str) -> Vec<usize> {
    let mut all_calories: Vec<usize> = Vec::new();
    let split_lines: Vec<&str> = input.split("\n\n").collect();

    for elf in split_lines {
        let calories: Vec<&str> = elf.split("\n").collect();

        all_calories.push(
            calories
                .iter()
                .map(|v| v.parse::<usize>().unwrap_or_else(|_| 0))
                .collect::<Vec<_>>()
                .iter()
                .sum(),
        )
    }

    all_calories.sort_by(|a, b| b.cmp(a));
    return all_calories;
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
