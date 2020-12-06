pub fn count_trees(input: &str, slope_right: usize, slope_down: usize) -> usize {
    let map = parse_input(input);

    let mut tree_collisions = 0;
    let mut position = 0;

    // Skip the first row, because we know we are starting in a safe place.
    for row in map.iter().step_by(slope_down).skip(1) {
        position += slope_right;
        position %= row.len();

        if row[position] == Space::Tree {
            tree_collisions += 1;
        }
    }

    tree_collisions
}

#[derive(PartialEq)]
pub enum Space {
    Open,
    Tree,
}

fn parse_input(input: &str) -> Vec<Vec<Space>> {
    let mut out = vec![];
    let width = input.lines().next().unwrap().len();

    for raw_row in input.lines() {
        let mut row = Vec::with_capacity(width);
        for byte in raw_row.bytes() {
            let space = match byte {
                b'.' => Space::Open,
                b'#' => Space::Tree,
                _ => {
                    continue;
                }
            };

            row.push(space);
        }

        out.push(row);
    }

    out
}

#[cfg(test)]
mod tests {
    use super::count_trees;

    fn get_sample_input() -> String {
        std::fs::read_to_string("input/day_03_sample").unwrap()
    }

    fn get_test_input() -> String {
        std::fs::read_to_string("input/day_03").unwrap()
    }

    #[test]
    fn sample() {
        assert_eq!(7, count_trees(&get_sample_input(), 3, 1));
    }

    #[test]
    fn part1() {
        assert_eq!(280, count_trees(&get_test_input(), 3, 1));
    }

    #[test]
    fn part2() {
        let test_input = get_test_input();

        let mut multiplied_total = 1;
        for &(slope_right, slope_down) in &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
            multiplied_total *= count_trees(&test_input, slope_right, slope_down);
        }

        assert_eq!(4355551200, multiplied_total);
    }
}
