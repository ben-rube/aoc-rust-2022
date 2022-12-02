pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let converted = line.as_bytes();
                let op = (converted[0] - b'A') as i8;
                let me = (converted[2] - b'X') as i8;
                let op_rule = op;
                let result = me;
                let result = (result - op_rule + 1).rem_euclid(3);

                let head_result = result + 1;
                let final_score = 3 * result;
                (head_result + final_score) as u32
            })
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let converted = line.as_bytes();
                let op = (converted[0] - b'A') as i8;
                let me = (converted[2] - b'X') as i8;
                let op_rule = op;
                let result = me;
                let my_pick = (op_rule - 1 + result).rem_euclid(3);
                let head_result = my_pick + 1;
                let final_score = 3 * result;
                (head_result + final_score) as u32
            })
            .sum::<u32>(),
    )
}
fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
