use std::collections::HashMap;

pub struct Day01 {
    left: Vec<i32>,
    right: Vec<i32>,
}

impl Day01 {
    pub fn input_size(&self) -> usize {
        self.left.len()
    }
}

impl Day01 {
    pub fn new(mut left: Vec<i32>, mut right: Vec<i32>) -> Self {
        left.sort_unstable();
        right.sort_unstable();
        Self { left, right }
    }

    pub fn part1(&self) -> i32 {
        self.left
            .iter()
            .zip(self.right.iter())
            .map(|(a, b)| (a - b).abs())
            .sum()
    }

    pub fn part2(&self) -> i32 {
        let right_counts: HashMap<i32, i32> =
            self.right.iter().fold(HashMap::new(), |mut acc, &num| {
                *acc.entry(num).or_insert(0) += 1;
                acc
            });

        self.left
            .iter()
            .map(|&num| num * right_counts.get(&num).unwrap_or(&0))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{load_example_input_for_day, Part};

    #[test]
    fn part1_example() {
        let input_content = load_example_input_for_day(1, Part::One).expect("Failed to load example input");
        let (left, right) = crate::input::parse_input(&input_content).expect("Failed to parse input");
        let solution = Day01::new(left, right);
        assert_eq!(solution.part1(), 11);
    }

    #[test]
    fn part2_example() {
        let input_content = load_example_input_for_day(1, Part::Two).expect("Failed to load example input");
        let (left, right) = crate::input::parse_input(&input_content).expect("Failed to parse input");
        let solution = Day01::new(left, right);
        assert_eq!(solution.part2(), 31);
    }
}
