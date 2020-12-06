use std::collections::HashMap;

pub fn count_valid_passports(input: &str) -> usize {
    // The cid field is optional, and thus not included here.
    let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    passport_iter(input)
        .filter(|passport_data| {
            required_fields
                .iter()
                .copied()
                .all(|req_field| passport_data.contains_key(req_field))
        })
        .count()
}

type FieldValidator = fn(&str) -> bool;

pub fn count_valid_passports_with_field_validation(input: &str) -> usize {
    let field_validators: &[(&str, FieldValidator)] = &[
        ("byr", validate_byr),
        ("iyr", validate_iyr),
        ("eyr", validate_eyr),
        ("hgt", validate_hgt),
        ("hcl", validate_hcl),
        ("ecl", validate_ecl),
        ("pid", validate_pid),
    ];

    passport_iter(input)
        .filter(|passport_data| {
            field_validators
                .iter()
                .copied()
                .all(|(key, validator)| match passport_data.get(key) {
                    Some(val) => validator(val),
                    None => false,
                })
        })
        .count()
}

fn validate_byr(input: &str) -> bool {
    validate_year(input, 1920, 2020)
}

fn validate_iyr(input: &str) -> bool {
    validate_year(input, 2010, 2020)
}

fn validate_eyr(input: &str) -> bool {
    validate_year(input, 2020, 2030)
}

fn validate_year(input: &str, min: u32, max: u32) -> bool {
    input
        .parse::<u32>()
        .map(|yr| yr >= min && yr <= max)
        .unwrap_or(false)
}

fn validate_hgt(input: &str) -> bool {
    if input.ends_with("in") {
        input
            .replace("in", "")
            .parse::<u32>()
            .map(|height_in| height_in >= 59 && height_in <= 76)
            .unwrap_or(false)
    } else if input.ends_with("cm") {
        input
            .replace("cm", "")
            .parse::<u32>()
            .map(|height_cm| height_cm >= 150 && height_cm <= 193)
            .unwrap_or(false)
    } else {
        false
    }
}

fn validate_hcl(input: &str) -> bool {
    if input.starts_with('#') {
        u32::from_str_radix(&input.replacen('#', "", 1), 16).is_ok()
    } else {
        false
    }
}

fn validate_ecl(input: &str) -> bool {
    let allowed_values = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

    allowed_values.contains(&input)
}

fn validate_pid(input: &str) -> bool {
    input.len() == 9 && input.parse::<u32>().is_ok()
}

fn passport_iter(input: &str) -> impl Iterator<Item = HashMap<&str, &str>> {
    // Passports are separated in the input file by blank lines, which we
    // find by looking for a newline from the line above followed by the
    // newline for the blank line.
    input.split("\n\n").map(|passport_text| {
        let mut passport_data = HashMap::new();

        for passport_field in passport_text.split_whitespace() {
            let mut splitter = passport_field.split(':');
            let key = splitter.next().unwrap();
            let value = splitter.next().unwrap();

            passport_data.insert(key, value);
        }

        passport_data
    })
}

#[cfg(test)]
mod tests {
    use super::{count_valid_passports, count_valid_passports_with_field_validation};

    fn get_sample_input() -> String {
        std::fs::read_to_string("input/day_04_sample").unwrap()
    }

    fn get_test_input() -> String {
        std::fs::read_to_string("input/day_04").unwrap()
    }

    #[test]
    fn sample() {
        assert_eq!(2, count_valid_passports(&get_sample_input()));
    }

    #[test]
    fn part1() {
        assert_eq!(192, count_valid_passports(&get_test_input()));
    }

    #[test]
    fn part2() {
        assert_eq!(
            101,
            count_valid_passports_with_field_validation(&get_test_input())
        );
    }
}
