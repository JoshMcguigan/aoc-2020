use text_io::scan;

pub fn count_valid_passwords_part1(input: &str) -> usize {
    let mut valid_count = 0;

    for line in input.lines() {
        let (min, max, character, password): (usize, usize, char, String);
        scan!(line.bytes() => "{}-{} {}: {}", min, max, character, password);

        let mut count = 0;
        for pw_character in password.bytes() {
            if character as u8 == pw_character {
                count += 1;
            }
        }

        if count >= min && count <= max {
            valid_count += 1;
        }
    }

    valid_count
}

pub fn count_valid_passwords_part2(input: &str) -> usize {
    let mut valid_count = 0;

    for line in input.lines() {
        let (index_1, index_2, character, password): (usize, usize, char, String);
        scan!(line.bytes() => "{}-{} {}: {}", index_1, index_2, character, password);

        let password_bytes = password.as_bytes();

        // Convert the indexes to zero-index.
        let exists_at_index_1 = password_bytes[index_1 - 1] == character as u8;
        let exists_at_index_2 = password_bytes[index_2 - 1] == character as u8;

        if exists_at_index_1 ^ exists_at_index_2 {
            valid_count += 1;
        }
    }

    valid_count
}

#[cfg(test)]
mod tests {
    use super::{count_valid_passwords_part1, count_valid_passwords_part2};

    fn get_sample_input() -> &'static str {
        "1-3 a: abcde\
         \n1-3 b: cdefg\
         \n2-9 c: ccccccccc"
    }

    fn get_test_input() -> String {
        std::fs::read_to_string("input/day_02").unwrap()
    }

    #[test]
    fn sample_input() {
        assert_eq!(2, count_valid_passwords_part1(get_sample_input()));
    }

    #[test]
    fn part1() {
        assert_eq!(396, count_valid_passwords_part1(&get_test_input()));
    }

    #[test]
    fn part2() {
        assert_eq!(428, count_valid_passwords_part2(&get_test_input()));
    }
}
