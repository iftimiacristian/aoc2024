// common/src/lib.rs

mod day_context;
mod day_trait;
mod enums;
mod registry;

pub use day_context::DayContext;
pub use day_trait::Day;
pub use enums::{InputMode, Part};
pub use registry::{DayRegistry, DaySolutionBuilder};

pub use std::time::Duration;
