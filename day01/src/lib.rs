use common::{Day, DayContext, InputMode};
use std::fs::File;
use std::io::{BufRead, BufReader};

// Define a struct that will hold the DayContext and implement the Day trait
pub struct Day01Solution {
    context: DayContext,
    column1_data: Vec<i32>,
    column2_data: Vec<i32>,
}

impl Day01Solution {
    // Constructor for Day01Solution, taking a DayContext
    pub fn new(context: DayContext, mode: InputMode) -> Self {
        let mut solution = Day01Solution {
            context,
            column1_data: Vec::new(),
            column2_data: Vec::new(),
        };
        let reader = solution.context.load_input(mode)
            .expect("Failed to load input for Day 01");

        solution.load_columns(reader);

        solution.column1_data.sort();
        solution.column2_data.sort();

        solution
    }
}

// Implement the Day trait for Day01Solution
impl Day for Day01Solution {
    fn part1(&self) {
        println!("{}", self.get_distance_sum());
    }

    fn part2(&self) {
        println!("{}", self.get_total_similarity_score());
    }
}

impl Day01Solution {
    fn absolute_difference(a: i32, b: i32) -> i32 {
        (a - b).abs()
    }

    fn extract_two_numbers(line: &str) -> Result<(i32, i32), String> {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() != 2 {
            return Err(format!("Failed to extract two numbers from line: {}", line));
        }

        let number1 = parts[0].parse::<i32>().map_err(|e| e.to_string())?;
        let number2 = parts[1].parse::<i32>().map_err(|e| e.to_string())?;

        Ok((number1, number2))
    }

    fn load_columns(&mut self, reader: BufReader<File>) {
        for line in reader.lines() {
            let line = line.expect("Failed to read line from file");

            match Day01Solution::extract_two_numbers(&line) {
                Ok((number1, number2)) => {
                    self.column1_data.push(number1);
                    self.column2_data.push(number2);
                }
                Err(e) => {
                    eprintln!("Error extracting numbers: {}", e);
                }
            }
        }
    }

    pub fn get_distance_sum(&self) -> i32 {
        let mut total = 0;
        for (val1, val2) in self.column1_data.iter().zip(self.column2_data.iter()) {
           total +=  Self::absolute_difference(*val1, *val2);
        }
        total
    }

    pub fn get_total_similarity_score(&self) -> i32 {
        let mut total = 0;

        for val1 in self.column1_data.iter() {
            let occurences = self.column2_data.iter()
                .filter(|&val2| val1 == val2)
                .count() as i32;

            total += occurences * val1;
        }

        total
    }

}

pub fn build_day_01_solution(context: common::DayContext) -> Box<dyn common::Day> {
    Box::new(Day01Solution::new(context, InputMode::Full))
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::DayContext;

    #[test]
    fn part1_example() {
        let day_context = DayContext::new(1); // Create a DayContext for Day 1
        let solution = Day01Solution::new(day_context, InputMode::Example); // Instantiate your solution struct

        let expected_output = 11; // Replace 0 with the actual expected output for your example
        assert_eq!(solution.get_distance_sum(), expected_output); // Call part1 on the solution instance
    }

    #[test]
    fn part2_example() {
        let mut day_context = DayContext::new(1);
        day_context.next_part();
        let solution = Day01Solution::new(day_context, InputMode::Example);
        let expected_output = 31;
        assert_eq!(solution.get_total_similarity_score(), expected_output);
    }
}
