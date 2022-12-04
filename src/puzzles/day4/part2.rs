use crate::utils::puzzle::Puzzle;

use itertools::Itertools;

use std::error::Error;

pub struct Day4Part2();

impl Puzzle for Day4Part2 {
  fn solve(lines: Vec<String>) -> Result<i32, Box<dyn Error>> {
    let mut partially_overlapping_count = 0;

    for line in lines {
      let (start_a, end_a, start_b, end_b) = match line.split(['-', ',']).collect_tuple() {
        Some((start_a, end_a, start_b, end_b)) => (
          start_a.parse::<i32>()?,
          end_a.parse::<i32>()?,
          start_b.parse::<i32>()?,
          end_b.parse::<i32>()?,
        ),
        None => panic!("invalid input"),
      };

      if start_a <= end_b && end_a >= start_b {
        partially_overlapping_count += 1;
      }
    }

    Ok(partially_overlapping_count)
  }
}

#[cfg(test)]
mod tests {
  use crate::utils::{input_reader, puzzle::Puzzle};

  use std::error::Error;

  #[test]
  fn solve_example() -> Result<(), Box<dyn Error>> {
    let lines = input_reader::example_for_day(4)?;
    let result = super::Day4Part2::solve(lines)?;

    assert_eq!(result, 4);

    Ok(())
  }

  #[test]
  fn solve() -> Result<(), Box<dyn Error>> {
    let lines = input_reader::for_day(4)?;
    let result = super::Day4Part2::solve(lines)?;

    assert_eq!(result, 917);

    Ok(())
  }
}
