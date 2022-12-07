use crate::utils::puzzle::Puzzle;

use itertools::Itertools;

use std::collections::HashSet;
use std::error::Error;

pub struct Day6Part2();

impl Puzzle<usize> for Day6Part2 {
  fn solve(lines: Vec<String>) -> Result<usize, Box<dyn Error>> {
    for line in lines {
      let chars = line.chars().collect_vec();

      for x in 13..chars.len() {
        let mut unique = HashSet::<char>::new();

        if (0..14).all(|y| unique.insert(chars[x - y])) {
          return Ok(x + 1);
        }
      }
    }

    panic!("marker not found");
  }
}

#[cfg(test)]
mod tests {
  use crate::utils::{input_reader, puzzle::Puzzle};

  use std::error::Error;

  #[test]
  fn solve_example_1() -> Result<(), Box<dyn Error>> {
    let lines = input_reader::example_for_day_and_number(6, 1)?;
    let result = super::Day6Part2::solve(lines)?;

    assert_eq!(result, 19);

    Ok(())
  }

  #[test]
  fn solve_example_2() -> Result<(), Box<dyn Error>> {
    let lines = input_reader::example_for_day_and_number(6, 2)?;
    let result = super::Day6Part2::solve(lines)?;

    assert_eq!(result, 23);

    Ok(())
  }

  #[test]
  fn solve_example_3() -> Result<(), Box<dyn Error>> {
    let lines = input_reader::example_for_day_and_number(6, 3)?;
    let result = super::Day6Part2::solve(lines)?;

    assert_eq!(result, 23);

    Ok(())
  }

  #[test]
  fn solve_example_4() -> Result<(), Box<dyn Error>> {
    let lines = input_reader::example_for_day_and_number(6, 4)?;
    let result = super::Day6Part2::solve(lines)?;

    assert_eq!(result, 29);

    Ok(())
  }

  #[test]
  fn solve_example_5() -> Result<(), Box<dyn Error>> {
    let lines = input_reader::example_for_day_and_number(6, 5)?;
    let result = super::Day6Part2::solve(lines)?;

    assert_eq!(result, 26);

    Ok(())
  }

  #[test]
  fn solve() -> Result<(), Box<dyn Error>> {
    let lines = input_reader::for_day(6)?;
    let result = super::Day6Part2::solve(lines)?;

    assert_eq!(result, 3716);

    Ok(())
  }
}
