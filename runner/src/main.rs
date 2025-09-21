use std::env;
use common::{DayContext, DayRegistry};
use day01::build_day_01_solution;
use day02::build_day_02_solution;
use day03::build_day_03_solution;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut registry = DayRegistry::new();
    registry.register_builder(1, build_day_01_solution);
    registry.register_builder(2, build_day_02_solution);
    registry.register_builder(3, build_day_03_solution);

    let day_to_run: Option<u8> = args.get(1).and_then(|s| s.parse().ok());

    match day_to_run {
        Some(day) => {
            println!("Running solution for Day {day}...");
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
    use std::time::Instant;

    let day_context = DayContext::new(day);

    if let Some(builder) = registry.get_builder(day) {
        let solution = builder(day_context); // Use the builder to create the solution

        println!("Day {}: ", solution.day_number());

        let start = Instant::now();
        let result1 = solution.part1();
        let duration1 = start.elapsed();
        println!("  Part 1: {result1} (took {duration1:?})");

        let start = Instant::now();
        let result2 = solution.part2();
        let duration2 = start.elapsed();
        println!("  Part 2: {result2} (took {duration2:?})");

        println!(
            "  Memory estimate: {:.1} KB",
            solution.memory_usage_estimate() as f64 / 1024.0
        );
    } else {
        println!("Day {day} not implemented yet or not registered.");
    }
}
