use crate::utils::puzzle::Puzzle;

use std::cmp::max;
use std::error::Error;

pub struct Day1Part1();

impl Puzzle<i32> for Day1Part1 {
  fn solve(lines: Vec<String>) -> Result<i32, Box<dyn Error>> {
    let mut current_calories = 0;
    let mut max_calories = 0;

    for line in lines {
      if line.is_empty() {
        max_calories = max(current_calories, max_calories);
        current_calories = 0;
      } else {
        current_calories += line.parse::<i32>()?;
      }
    }

    max_calories = max(current_calories, max_calories);

    Ok(max_calories)
  }
}

#[cfg(test)]
mod tests {
  use crate::utils::{input_reader, puzzle::Puzzle};

  use std::error::Error;

  #[test]
  fn solve_example() -> Result<(), Box<dyn Error>> {
    let lines = input_reader::example_for_day(1)?;
    let result = super::Day1Part1::solve(lines)?;

    assert_eq!(result, 24000);

    Ok(())
  }

  #[test]
  fn solve() -> Result<(), Box<dyn Error>> {
    let lines = input_reader::for_day(1)?;
    let result = super::Day1Part1::solve(lines)?;

    assert_eq!(result, 74711);

    Ok(())
  }
}
