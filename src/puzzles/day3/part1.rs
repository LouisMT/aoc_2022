use crate::utils::puzzle::Puzzle;

use itertools::Itertools;

use std::collections::HashSet;
use std::error::Error;

pub struct Day3Part1();

impl Puzzle for Day3Part1 {
  fn solve(lines: Vec<String>) -> Result<i32, Box<dyn Error>> {
    let mut total_priority = 0;

    for line in lines {
      let items = line.chars().collect_vec();
      let (left, right) = items.split_at(items.len() / 2);
      let left_set = HashSet::<&char>::from_iter(left);
      let right_set = HashSet::<&char>::from_iter(right);
      let same_items = left_set.intersection(&right_set);

      for same_item in same_items {
        total_priority += compute_priority(**same_item);
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
    let result = super::Day3Part1::solve(lines)?;

    assert_eq!(result, 157);

    Ok(())
  }

  #[test]
  fn solve() -> Result<(), Box<dyn Error>> {
    let lines = input_reader::for_day(3)?;
    let result = super::Day3Part1::solve(lines)?;

    assert_eq!(result, 8039);

    Ok(())
  }
}
