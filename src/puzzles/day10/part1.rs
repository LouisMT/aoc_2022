use crate::utils::puzzle::Puzzle;

use itertools::Itertools;

use std::error::Error;

pub struct Day10Part1();

impl Puzzle<isize> for Day10Part1 {
  fn solve(lines: Vec<String>) -> Result<isize, Box<dyn Error>> {
    let mut value = 1;
    let mut clock = 0;
    let mut next_clock = 20;
    let mut signal_strength_sum = 0;

    for line in lines {
      let cycles = match line.split(' ').collect_vec().as_slice() {
        ["noop"] => vec![0],
        ["addx", amount] => {
          let amount = amount.parse::<isize>()?;
          vec![0, amount]
        }
        _ => panic!("invalid input"),
      };

      for cycle in cycles {
        clock += 1;

        if clock == next_clock {
          signal_strength_sum += clock * value;
          next_clock += 40;
        }

        value += cycle;
      }
    }

    Ok(signal_strength_sum)
  }
}

#[cfg(test)]
mod tests {
  use crate::utils::{input_reader, puzzle::Puzzle};

  use std::error::Error;

  #[test]
  fn solve_example() -> Result<(), Box<dyn Error>> {
    let lines = input_reader::example_for_day(10)?;
    let result = super::Day10Part1::solve(lines)?;

    assert_eq!(result, 13140);

    Ok(())
  }

  #[test]
  fn solve() -> Result<(), Box<dyn Error>> {
    let lines = input_reader::for_day(10)?;
    let result = super::Day10Part1::solve(lines)?;

    assert_eq!(result, 11960);

    Ok(())
  }
}
