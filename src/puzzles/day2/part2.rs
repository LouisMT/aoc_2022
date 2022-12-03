use itertools::Itertools;

use crate::utils::puzzle::Puzzle;

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Lines};

pub struct Day2Part2();

impl Puzzle for Day2Part2 {
  fn solve(lines: Lines<BufReader<File>>) -> Result<i32, Box<dyn Error>> {
    let win_combinations = HashMap::from([('A', 'B'), ('B', 'C'), ('C', 'A')]);
    let lose_combinations = HashMap::from([('A', 'C'), ('B', 'A'), ('C', 'B')]);
    let scores = HashMap::from([('A', 1), ('B', 2), ('C', 3)]);

    let mut score = 0;

    for line in lines {
      score += match line?.chars().collect_tuple() {
        Some((theirs, _, 'X')) => {
          let mine = lose_combinations.get(&theirs).unwrap();
          scores.get(&mine).unwrap_or(&0).to_owned()
        }
        Some((theirs, _, 'Y')) => 3 + scores.get(&theirs).unwrap_or(&0),
        Some((theirs, _, 'Z')) => {
          let mine = win_combinations.get(&theirs).unwrap();
          6 + scores.get(&mine).unwrap_or(&0)
        }
        _ => panic!("Unexpected number of chars"),
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
