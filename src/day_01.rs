/// Finds two values in list which sum to m in a list of length n.
///
/// Assumes each value in the list is unique.
///
/// ## Runtime complexity
///
/// O(n)
pub fn find_two_values_that_sum_to_m(list: &[usize], m: usize) -> Option<[usize; 2]> {
    // exists[i] is true if i is in list.
    let mut exists = vec![false; m + 1];

    for value in list.iter().copied() {
        // Any value larger than n cannot be part of a sum to n (our values are unsigned)
        // so we can ignore that value.
        if value > m {
            continue;
        }

        let matching_value = m - value;

        // If each value in the list is unique, we can't use the same value more
        // than once.
        if matching_value == value {
            continue;
        }

        if exists[matching_value] {
            return Some([value, matching_value]);
        }

        exists[value] = true;
    }

    None
}

/// Finds three values in list which sum to m in a list of length n.
///
/// Assumes each value in the list is unique.
///
/// ## Runtime complexity
///
/// O(n) to create the exists array, plus O(n^2) for the search => O(n^2)
pub fn find_three_values_that_sum_to_m(list: &[usize], m: usize) -> Option<[usize; 3]> {
    // exists[i] is true if i is in list.
    let mut exists = vec![false; m + 1];

    for value in list.iter().copied() {
        // Any value larger than n cannot be part of a sum to n (our values are unsigned)
        // so we can ignore that value.
        if value > m {
            continue;
        }

        exists[value] = true;
    }

    for potential_value_1 in list.iter().copied() {
        for potential_value_2 in list.iter().copied() {
            // If each value in the list is unique, we can't use the same value more
            // than once.
            if potential_value_1 == potential_value_2 {
                continue;
            }

            let current_sum = potential_value_1 + potential_value_2;

            if current_sum > m {
                continue;
            }

            let potential_value_3 = m - current_sum;

            // If each value in the list is unique, we can't use the same value more
            // than once.
            if potential_value_3 == potential_value_1 || potential_value_3 == potential_value_2 {
                continue;
            }

            if exists[potential_value_3] {
                return Some([potential_value_1, potential_value_2, potential_value_3]);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::{find_three_values_that_sum_to_m, find_two_values_that_sum_to_m};

    fn get_test_input() -> Vec<usize> {
        let input = std::fs::read_to_string("input/day_01").unwrap();
        let mut out: Vec<usize> = vec![];
        for value in input.lines() {
            out.push(value.parse().unwrap());
        }

        out
    }

    #[test]
    fn sample_input() {
        let sample_input = vec![1721, 979, 366, 299, 675, 1456];

        let values = find_two_values_that_sum_to_m(&sample_input, 2020).unwrap();

        assert_eq!([299, 1721], values);
    }

    #[test]
    fn part1() {
        let values = find_two_values_that_sum_to_m(&get_test_input(), 2020).unwrap();

        let multiplied_together = values[0] * values[1];

        assert_eq!([277, 1743], values);
        assert_eq!(482811, multiplied_together);
    }

    #[test]
    fn part2() {
        let values = find_three_values_that_sum_to_m(&get_test_input(), 2020).unwrap();

        let multiplied_together = values[0] * values[1] * values[2];

        assert_eq!([1067, 691, 262], values);
        assert_eq!(193171814, multiplied_together);
    }
}
