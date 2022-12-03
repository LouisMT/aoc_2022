use itertools::Itertools;

use crate::utils::puzzle::Puzzle;

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Lines};

pub struct Day2Part1();

impl Puzzle for Day2Part1 {
  fn solve(lines: Lines<BufReader<File>>) -> Result<i32, Box<dyn Error>> {
    let combinations = HashMap::from([
      (('X', 'C'), 6),
      (('Z', 'B'), 6),
      (('Y', 'A'), 6),
      (('A', 'X'), 3),
      (('X', 'A'), 3),
      (('B', 'Y'), 3),
      (('Y', 'B'), 3),
      (('C', 'Z'), 3),
      (('Z', 'C'), 3),
    ]);

    let scores = HashMap::from([('X', 1), ('Y', 2), ('Z', 3)]);

    let mut score = 0;

    for line in lines {
      match line?.chars().collect_tuple() {
        Some((theirs, _, mine)) => {
          score += combinations.get(&(mine, theirs)).unwrap_or(&0);
          score += scores.get(&mine).expect("turn should exist");
        }
        None => panic!("invalid input"),
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
    let result = super::Day2Part1::solve(lines)?;

    assert_eq!(result, 15);

    Ok(())
  }

  #[test]
  fn solve() -> Result<(), Box<dyn Error>> {
    let lines = input_reader::for_day(2)?;
    let result = super::Day2Part1::solve(lines)?;

    assert_eq!(result, 12794);

    Ok(())
  }
}
