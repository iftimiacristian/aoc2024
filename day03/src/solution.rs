pub struct Day03 {
    input: Vec<(i32, i32)>,
}

impl Day03 {
    pub fn new(input: Vec<(i32, i32)>) -> Self {
        Self { input }
    }

    pub fn input_size(&self) -> usize {
        // self.input.len()
        0
    }

    pub fn part1(&self) -> i32 {
        self.input.iter().map(|(a, b)| a * b).sum()
    }

    pub fn part2(&self) -> i32 {
        // TODO: Implement Part 2
        0
    }

}

#[cfg(test)]
mod tests {
    use common::{load_example_input_for_day, Part};
    use crate::input;
    use super::*;

    #[test]
    fn part1_example() {
        let input_content = load_example_input_for_day(3, Part::One).expect("Failed to load example input");
        let input = input::parse_input(&input_content).expect("Failed to parse example input");

        let solution = Day03::new(input);
        assert_eq!(solution.part1(), 161);
    }

    #[test]
    fn part2_example() {
        let input = vec![(1, 2), (3, 4)]; // Example pairs
        let solution = Day03::new(input);
        assert_eq!(solution.part2(), 0);
    }
}
