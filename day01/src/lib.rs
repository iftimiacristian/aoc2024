use common::{Day, DayContext, InputMode};

// Define a struct that will hold the DayContext and implement the Day trait
pub struct Day01Solution {
    context: DayContext,
}

impl Day01Solution {
    // Constructor for Day01Solution, taking a DayContext
    pub fn new(context: DayContext) -> Self {
        Day01Solution { context }
    }
}

// Implement the Day trait for Day01Solution
impl Day for Day01Solution {
    fn part1(&self) -> u64 {
        // Access the DayContext through self.context
        let input = self.context.load_input(InputMode::Example)
                               .expect("Failed to load input for Day 01 Part 1");

        // Your Part 1 logic will go here, using the 'input' string.
        // For now, let's print the input to confirm it's loaded.
        println!("Loaded input for Day {}: \n{}", self.context.day, input);

        0 // Placeholder return value
    }

    fn part2(&self) -> u64 {
        // Placeholder for Part 2 logic
        // You'll load input similarly:
        // let input = self.context.load_input(InputMode::Full).expect("Failed to load input for Day 01 Part 2");
        println!("Part 2 not yet implemented for Day {}", self.context.day);
        0
    }
}

pub fn build_day_01_solution(context: common::DayContext) -> Box<dyn common::Day> {
    Box::new(Day01Solution::new(context))
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::DayContext;

    #[test]
    fn part1_example() {
        let day_context = DayContext::new(1); // Create a DayContext for Day 1
        let solution = Day01Solution::new(day_context); // Instantiate your solution struct

        let expected_output = 0; // Replace 0 with the actual expected output for your example
        assert_eq!(solution.part1(), expected_output); // Call part1 on the solution instance
    }
}
