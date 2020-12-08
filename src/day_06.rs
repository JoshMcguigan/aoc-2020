struct Group {
    /// Number of people who answered each question
    answers: [usize; 26],
    /// Number of people in the group
    size: usize,
}

pub fn solve_part_1(input: &str) -> usize {
    parse_input(input)
        .iter()
        .map(|group| &group.answers)
        .flatten()
        .filter(|&&b| b > 0)
        .count()
}

pub fn solve_part_2(input: &str) -> usize {
    parse_input(input)
        .iter()
        .map(|group| {
            group
                .answers
                .iter()
                .filter(|&&responses| responses == group.size)
                .count()
        })
        .sum()
}

fn parse_input(input: &str) -> Vec<Group> {
    let mut groups = vec![];

    // Groups are separated in the input by an empty line.
    for group in input.split("\n\n") {
        let mut size = 0;
        let mut answers = [0; 26];

        for single_result in group.lines() {
            size += 1;

            for answer in single_result.bytes() {
                answers[byte_to_index(answer)] += 1;
            }
        }

        groups.push(Group { answers, size });
    }

    groups
}

/// Convert lowercase ascii to index from 0-25.
fn byte_to_index(byte: u8) -> usize {
    byte as usize - 97
}

#[cfg(test)]
mod tests {
    use super::{solve_part_1, solve_part_2};

    fn get_sample_input() -> String {
        std::fs::read_to_string("input/day_06_sample").unwrap()
    }

    fn get_test_input() -> String {
        std::fs::read_to_string("input/day_06").unwrap()
    }

    #[test]
    fn sample() {
        assert_eq!(11, solve_part_1(&get_sample_input()));
    }

    #[test]
    fn part1() {
        assert_eq!(6763, solve_part_1(&get_test_input()));
    }

    #[test]
    fn part2() {
        assert_eq!(3512, solve_part_2(&get_test_input()));
    }
}
