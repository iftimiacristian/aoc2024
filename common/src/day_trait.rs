// common/src/day_trait.rs

pub trait Day {
    fn part1(&self) -> String;

    fn part2(&self) -> String;

    fn day_number(&self) -> u8;

    /// Optional: Estimated memory usage in bytes
    /// Default implementation returns 1MB
    fn memory_usage_estimate(&self) -> usize {
        1024 * 1024
    }
}
