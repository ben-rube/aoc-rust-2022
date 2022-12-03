pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|line| search_matching(create_groups(line)))
            .map(determine_priority)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .collect::<Vec<_>>()
            .chunks_exact(3)
            .filter_map(search_matching_in_group)
            .map(determine_priority)
            .sum(),
    )
}

fn create_groups(input: &str) -> (&str, &str) {
    input.split_at(input.len() / 2)
}

fn search_matching(input: (&str, &str)) -> Option<char> {
    input.0.chars().find(|x| input.1.contains(*x))
}

fn search_matching_in_group(input: &[&str]) -> Option<char> {
    input
        .first()?
        .chars()
        .find(|c| input[1..].iter().all(|x| x.contains(*c)))
}

fn determine_priority(input: char) -> u32 {
    if input.is_lowercase() {
        ((input as u32) - (b'a' as u32)) + 1
    } else {
        ((input as u32) - (b'A' as u32)) + 27
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
