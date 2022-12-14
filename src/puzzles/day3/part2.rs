use crate::utils::puzzle::Puzzle;

use itertools::Itertools;

use std::collections::HashSet;
use std::error::Error;

pub struct Day3Part2();

impl Puzzle<i32> for Day3Part2 {
  fn solve(lines: Vec<String>) -> Result<i32, Box<dyn Error>> {
    let mut total_priority = 0;

    for lines in &lines.into_iter().chunks(3) {
      total_priority += match lines
        .into_iter()
        .map(|l| HashSet::<char>::from_iter(l.chars()))
        .reduce(|acc, items| HashSet::<char>::from_iter(acc.intersection(&items).copied()))
      {
        Some(items) => items.into_iter().map(compute_priority).sum(),
        None => 0,
      }
    }

    Ok(total_priority)
  }
}

fn compute_priority(item: char) -> i32 {
  match item as i32 {
    item if item >= 97 => item - 96,
    item => item - 38,
  }
}

#[cfg(test)]
mod tests {
  use crate::utils::{input_reader, puzzle::Puzzle};

  use std::error::Error;

  #[test]
  fn solve_example() -> Result<(), Box<dyn Error>> {
    let lines = input_reader::example_for_day(3)?;
    let result = super::Day3Part2::solve(lines)?;

    assert_eq!(result, 70);

    Ok(())
  }

  #[test]
  fn solve() -> Result<(), Box<dyn Error>> {
    let lines = input_reader::for_day(3)?;
    let result = super::Day3Part2::solve(lines)?;

    assert_eq!(result, 2510);

    Ok(())
  }
}
