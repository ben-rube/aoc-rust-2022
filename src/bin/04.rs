pub fn part_one(input: &str) -> Option<u32> {
    let converted_zones = convert_input(input);
    Some(find_complete_overlap(converted_zones))
}

pub fn part_two(input: &str) -> Option<u32> {
    let converted_zones = convert_input(input);
    Some(find_any_overlap(converted_zones))
}

fn convert_input(input: &str) -> Vec<u32> {
    let parts = input
        .trim()
        .split(&[',', '-', '\n'][..])
        .collect::<Vec<&str>>();

    parts.iter().map(|x| x.parse::<u32>().unwrap()).collect()
}

fn find_complete_overlap(zones: Vec<u32>) -> u32 {
    let mut conflicts: u32 = 0;
    for i in zones.chunks(4) {
        let group_a1 = i[0];
        let group_a2 = i[1];
        let group_b1 = i[2];
        let group_b2 = i[3];
        if (group_a1 <= group_b1 && group_a2 >= group_b2)
            || (group_b1 <= group_a1 && group_b2 >= group_a2)
        {
            conflicts += 1;
        }
    }
    conflicts
}

fn find_any_overlap(zones: Vec<u32>) -> u32 {
    let mut conflicts: u32 = 0;
    for i in zones.chunks(4) {
        let group_a1 = i[0];
        let group_a2 = i[1];
        let group_b1 = i[2];
        let group_b2 = i[3];
        if (group_a1 <= group_b1 && group_b1 <= group_a2)
            || (group_b1 <= group_a1 && group_a1 <= group_b2)
        {
            conflicts += 1;
        }
    }
    conflicts
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
