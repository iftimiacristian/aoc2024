// common/src/lib.rs

mod enums;
mod day_trait;
mod day_context;
mod registry;

pub use enums::{Part, InputMode};
pub use day_trait::Day;
pub use day_context::DayContext;
pub use registry::{DaySolutionBuilder, DayRegistry};