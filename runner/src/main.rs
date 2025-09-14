use std::env;
use common::{Day, DayContext, DayRegistry}; // Added DayRegistry, DaySolutionBuilder
use day01::build_day_01_solution; // Import the builder function

fn main() {
    let args: Vec<String> = env::args().collect();

    // Initialize the DayRegistry
    let mut registry = DayRegistry::new();
    // Register Day 01's solution builder
    registry.register_builder(1, build_day_01_solution);

    let day_to_run: Option<u8> = args.get(1).and_then(|s| s.parse().ok());

    match day_to_run {
        Some(day) => {
            println!("Running solution for Day {}...", day);
            run_day(&registry, day);
        }
        None => {
            println!("No day specified. Running all available days...");
            let all_days = registry.get_all_days();
            for day in all_days {
                run_day(&registry, day);
            }
        }
    }
}

fn run_day(registry: &DayRegistry, day: u8) {
    // Create a DayContext for the current day
    let day_context = DayContext::new(day);

    if let Some(builder) = registry.get_builder(day) {
        let solution = builder(day_context); // Use the builder to create the solution

        // Run Part 1
        let result_part1 = solution.part1();
        println!("Day {} Part 1 Result: {}", day, result_part1);

        // Run Part 2
        let result_part2 = solution.part2();
        println!("Day {} Part 2 Result: {}", day, result_part2);
    } else {
        println!("Day {} not implemented yet or not registered.", day);
    }
}
