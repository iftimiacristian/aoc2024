// common/src/registry.rs

use super::day_context::DayContext;
use super::day_trait::Day;
use std::collections::HashMap;

pub type DaySolutionBuilder = fn(DayContext) -> Box<dyn Day>;

pub struct DayRegistry {
    builders: HashMap<u8, DaySolutionBuilder>,
}

impl Default for DayRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl DayRegistry {
    pub fn new() -> Self {
        DayRegistry {
            builders: HashMap::new(),
        }
    }

    pub fn register_builder(&mut self, day_num: u8, builder: DaySolutionBuilder) {
        self.builders.insert(day_num, builder);
    }

    pub fn get_builder(&self, day_num: u8) -> Option<&DaySolutionBuilder> {
        self.builders.get(&day_num)
    }

    pub fn get_all_days(&self) -> Vec<u8> {
        let mut days: Vec<u8> = self.builders.keys().cloned().collect();
        days.sort_unstable();
        days
    }
}
