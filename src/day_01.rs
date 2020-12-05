/// Finds two values in list which sum to m in a list of length n.
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
        if exists[matching_value] {
            return Some([value, matching_value]);
        }

        exists[value] = true;
    }

    None
}

/// Finds three values in list which sum to m in a list of length n.
///
/// ## Runtime complexity
///
/// O(n) to create the exists array, plus O(m^2) for the search
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

    for potential_value_1 in 0..=m {
        if !exists[potential_value_1] {
            continue;
        }

        // Since we're reviewing values in increasing order, if a single value is more
        // than half of n we know no future value can create a solution, since all
        // future values will be larger than our current value.
        if potential_value_1 > (m / 2) {
            return None;
        }

        for potential_value_2 in (potential_value_1 + 1)..=m {
            if !exists[potential_value_2] {
                continue;
            }
            let current_sum = potential_value_1 + potential_value_2;

            // Once we've found two values that sum to more than n, we need to break
            // out of this inner loop since we won't find a solution here.
            if current_sum > m {
                break;
            }

            let potential_value_3 = m - current_sum;

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

        assert_eq!([262, 691, 1067], values);
        assert_eq!(193171814, multiplied_together);
    }
}
