use std::collections::{HashMap, HashSet};

struct Bags {
    /// Map of bag to bags which are contained by that bag
    contains: HashMap<String, Vec<(String, usize)>>,
    /// Map of bag to bags which contain that bag
    contained_by: HashMap<String, Vec<String>>,
}

pub fn solve_part_1(input: &str) -> usize {
    let bags = parse_input(input);

    let mut bags_containing_shiny_gold = HashSet::new();

    let mut bags_to_visit = vec!["shiny gold"];

    while let Some(bag) = bags_to_visit.pop() {
        let bags = match bags.contained_by.get(bag) {
            Some(b) => b,
            None => continue,
        };
        for bag in bags {
            bags_containing_shiny_gold.insert(bag);
            bags_to_visit.push(bag);
        }
    }

    bags_containing_shiny_gold.len()
}

pub fn solve_part_2(input: &str) -> usize {
    let bags = parse_input(input);

    bag_count(&bags, "shiny gold")
}

fn bag_count(bags: &Bags, color: &str) -> usize {
    let mut count = 0;

    for (bag, quantity) in bags.contains.get(color).unwrap_or(&vec![]) {
        count += quantity + quantity * bag_count(bags, bag);
    }

    count
}

fn parse_input(input: &str) -> Bags {
    let mut contains: HashMap<String, Vec<(String, usize)>> = HashMap::new();
    let mut contained_by: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.lines() {
        if line.contains("contain no other bags") {
            continue;
        }

        let mut parts = line.split(" bags contain ");
        let container = parts.next().unwrap();
        let contained_bags = parts.next().unwrap().trim_end_matches('.');

        let mut contained = vec![];

        for contained_bag in contained_bags.split(", ") {
            // contained_bag is something like "5 dark olive bags"
            // Handle optoinal plural by stripping 's', then stripping singular "bag".
            let contained_bag = contained_bag.trim_end_matches('s').trim_end_matches(" bag");
            let mut parts = contained_bag.splitn(2, ' ');
            let quantity = parts.next().unwrap().parse().unwrap();
            let contained_bag = parts.next().unwrap();

            contained.push((contained_bag.to_string(), quantity));
            contained_by
                .entry(contained_bag.to_string())
                .or_default()
                .push(container.to_string());
        }

        contains.insert(container.to_string(), contained);
    }

    Bags {
        contains,
        contained_by,
    }
}

#[cfg(test)]
mod tests {
    use super::{solve_part_1, solve_part_2};

    fn get_sample_input() -> String {
        std::fs::read_to_string("input/day_07_sample").unwrap()
    }

    fn get_test_input() -> String {
        std::fs::read_to_string("input/day_07").unwrap()
    }

    #[test]
    fn sample() {
        assert_eq!(4, solve_part_1(&get_sample_input()));
    }

    #[test]
    fn sample_part_2() {
        assert_eq!(32, solve_part_2(&get_sample_input()));
    }

    #[test]
    fn part1() {
        assert_eq!(238, solve_part_1(&get_test_input()));
    }

    #[test]
    fn part2() {
        assert_eq!(82930, solve_part_2(&get_test_input()));
    }
}
