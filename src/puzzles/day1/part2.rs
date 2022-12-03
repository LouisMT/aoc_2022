use itertools::Itertools;

use crate::utils::puzzle::Puzzle;

use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Lines};

pub struct Day1Part2();

impl Puzzle for Day1Part2 {
  fn solve(lines: Lines<BufReader<File>>) -> Result<i32, Box<dyn Error>> {
    let mut current_calories = 0;
    let mut max_calories = Vec::<i32>::new();

    for line in lines {
      match line? {
        line if line.is_empty() => {
          max_calories.push(current_calories);
          current_calories = 0;
        }

        line => current_calories += line.parse::<i32>()?,
      }
    }

    max_calories.push(current_calories);

    let top_3_sum = max_calories
      .into_iter()
      .sorted_by(|a, b| b.cmp(a))
      .take(3)
      .sum();

    Ok(top_3_sum)
  }
}

#[cfg(test)]
mod tests {
  use crate::utils::{input_reader, puzzle::Puzzle};

  use std::error::Error;

  #[test]
  fn solve_example() -> Result<(), Box<dyn Error>> {
    let lines = input_reader::example_for_day(1)?;
    let result = super::Day1Part2::solve(lines)?;

    assert_eq!(result, 45000);

    Ok(())
  }

  #[test]
  fn solve() -> Result<(), Box<dyn Error>> {
    let lines = input_reader::for_day(1)?;
    let result = super::Day1Part2::solve(lines)?;

    assert_eq!(result, 209481);

    Ok(())
  }
}
