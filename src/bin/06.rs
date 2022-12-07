use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    Some(calculate_sequence(input, 4))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(calculate_sequence(input, 14))
}

fn calculate_sequence(input: &str, offset: usize) -> u32 {
    let packet_markers = input.chars().collect::<Vec<char>>();
    for (i, c) in packet_markers.windows(offset).enumerate() {
        if c.iter().collect::<HashSet<_>>().len() == offset {
            return (i + offset) as u32;
        }
    }
    0
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(6));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(23));
    }
}
