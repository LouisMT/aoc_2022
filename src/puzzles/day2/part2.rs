use crate::utils::puzzle::Puzzle;

use itertools::Itertools;

use std::collections::HashMap;
use std::error::Error;

pub struct Day2Part2();

impl Puzzle<i32> for Day2Part2 {
  fn solve(lines: Vec<String>) -> Result<i32, Box<dyn Error>> {
    let win_combinations = HashMap::from([('A', 'B'), ('B', 'C'), ('C', 'A')]);
    let lose_combinations = HashMap::from([('A', 'C'), ('B', 'A'), ('C', 'B')]);
    let scores = HashMap::from([('A', 1), ('B', 2), ('C', 3)]);

    let mut score = 0;

    for line in lines {
      score += match line.chars().collect_tuple() {
        Some((theirs, _, 'X')) => {
          let mine = lose_combinations.get(&theirs).expect("turn should exist");
          *scores.get(mine).expect("turn should exist")
        }
        Some((theirs, _, 'Y')) => scores.get(&theirs).expect("turn should exist") + 3,
        Some((theirs, _, 'Z')) => {
          let mine = win_combinations.get(&theirs).expect("turn should exist");
          scores.get(mine).expect("turn should exist") + 6
        }
        _ => panic!("invalid input"),
      }
    }

    Ok(score)
  }
}

#[cfg(test)]
mod tests {
  use crate::utils::{input_reader, puzzle::Puzzle};

  use std::error::Error;

  #[test]
  fn solve_example() -> Result<(), Box<dyn Error>> {
    let lines = input_reader::example_for_day(2)?;
    let result = super::Day2Part2::solve(lines)?;

    assert_eq!(result, 12);

    Ok(())
  }

  #[test]
  fn solve() -> Result<(), Box<dyn Error>> {
    let lines = input_reader::for_day(2)?;
    let result = super::Day2Part2::solve(lines)?;

    assert_eq!(result, 14979);

    Ok(())
  }
}
