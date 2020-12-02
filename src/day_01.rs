/// Finds two values in list which sum to n.
///
/// ## Runtime complexity
///
/// n * lg(n) for the sort, plus n * lg(n) for the search => O(n * lg(n))
pub fn find_two_values_that_sum_to_n(mut list: Vec<u64>, n: u64) -> Option<[u64; 2]> {
    // Our algorithm below relies on having a sorted list.
    list.sort_unstable();

    for (i, potential_value) in list.iter().copied().enumerate() {
        // We are assuming unsigned values, so if our potential value is
        // greater than n it cannot be a solution. Further, since we are
        // iterating over the values in sorted order, once we reach a value
        // which is greater than n we know there cannot be any solution, so
        // we return None.
        //
        // Since we've sorted the values, if a single value is more than half of n we
        // know no future value can create a solution, since all future values will
        // be larger than our current value.
        if potential_value > (n / 2) {
            return None;
        }

        let other_value = n - potential_value;
        if list[i + 1..].binary_search(&other_value).is_ok() {
            return Some([potential_value, other_value]);
        }
    }

    None
}

/// Finds three values in list which sum to n.
///
/// ## Runtime complexity
///
/// n * lg(n) for the sort, plus n^2 * lg(n) for the search => O(n^2 * lg(n))
///
/// This worst case complexity is pretty bad, but most of the items in the input data
/// happen to be near n, which means the sum of the first two values will quickly become
/// greater than n allowing us to stop checking a particular value.
pub fn find_three_values_that_sum_to_n(mut list: Vec<u64>, n: u64) -> Option<[u64; 3]> {
    // Our algorithm below relies on having a sorted list.
    list.sort_unstable();

    for potential_value_index_1 in 0..list.len() {
        let val1 = list[potential_value_index_1];

        // Since we've sorted the values, if a single value is more than half of n we
        // know no future value can create a solution, since all future values will
        // be larger than our current value.
        if val1 > (n / 2) {
            return None;
        }

        for potential_value_index_2 in (potential_value_index_1 + 1)..list.len() {
            let val2 = list[potential_value_index_2];
            let current_sum = val1 + val2;

            // Once we've found two values that sum to more than n, we need to break
            // out of this inner loop since we won't find a solution here.
            if current_sum > n {
                break;
            }

            let val3 = n - current_sum;

            // Since our list is sorted, we are not going to find any value 3
            // which is smaller than value 2, so we can skip the binary search.
            if val3 < val2 {
                break;
            }

            if list[potential_value_index_2 + 1..]
                .binary_search(&val3)
                .is_ok()
            {
                return Some([val1, val2, val3]);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::{find_three_values_that_sum_to_n, find_two_values_that_sum_to_n};

    fn get_test_input() -> Vec<u64> {
        let input = std::fs::read_to_string("input/day_01").unwrap();
        let mut out: Vec<u64> = vec![];
        for value in input.lines() {
            out.push(value.parse().unwrap());
        }

        out
    }

    #[test]
    fn sample_input() {
        let sample_input = vec![1721, 979, 366, 299, 675, 1456];

        let values = find_two_values_that_sum_to_n(sample_input, 2020).unwrap();

        assert_eq!([299, 1721], values);
    }

    #[test]
    fn part1() {
        let values = find_two_values_that_sum_to_n(get_test_input(), 2020).unwrap();

        let multiplied_together = values[0] * values[1];

        assert_eq!([277, 1743], values);
        assert_eq!(482811, multiplied_together);
    }

    #[test]
    fn part2() {
        let values = find_three_values_that_sum_to_n(get_test_input(), 2020).unwrap();

        let multiplied_together = values[0] * values[1] * values[2];

        assert_eq!([262, 691, 1067], values);
        assert_eq!(193171814, multiplied_together);
    }
}
