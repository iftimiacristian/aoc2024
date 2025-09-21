// common/src/lib.rs

mod day_context;
mod day_trait;
mod enums;
pub mod error;
mod registry;
mod test_utils;

pub use day_context::DayContext;
pub use day_trait::Day;
pub use enums::{InputMode, Part};
pub use error::ParseError;
pub use registry::{DayRegistry, DaySolutionBuilder};
pub use test_utils::load_example_input_for_day;

pub use std::time::Duration;
