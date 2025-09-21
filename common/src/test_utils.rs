use super::{DayContext, InputMode, Part};
use std::io::Read;

pub fn load_example_input_for_day(day: u8, part: Part) -> Result<String, Box<dyn std::error::Error>> {
    let mut context = DayContext::new(day);
    context.set_part(part);
    let mut reader = context.load_input(InputMode::Example)?;
    let mut input_content = String::new();
    reader.read_to_string(&mut input_content)?;
    Ok(input_content)
}
