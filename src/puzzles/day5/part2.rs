use crate::utils::puzzle::Puzzle;

use itertools::Itertools;

use std::collections::VecDeque;
use std::error::Error;

pub struct Day5Part2();

impl Puzzle<String> for Day5Part2 {
  fn solve(lines: Vec<String>) -> Result<String, Box<dyn Error>> {
    let mut lines = lines.into_iter();
    let mut stacks = Vec::<VecDeque<char>>::new();

    for line in lines.by_ref() {
      if line.is_empty() {
        break;
      }

      for (index, cargo) in line.chars().skip(1).step_by(4).enumerate() {
        if index + 1 > stacks.len() {
          stacks.push(VecDeque::<char>::new());
        }

        if cargo != ' ' {
          stacks[index].push_front(cargo);
        }
      }
    }

    for line in lines {
      let (amount, from, to) = match line.split(' ').collect_tuple() {
        Some(("move", amount, "from", from, "to", to)) => (
          amount.parse::<usize>()?,
          from.parse::<usize>()?,
          to.parse::<usize>()?,
        ),
        _ => panic!("invalid input"),
      };

      let mut temp_stack = VecDeque::<char>::new();

      for _ in 0..amount {
        let cargo = stacks[from - 1]
          .pop_back()
          .expect("stack should contain cargo");

        temp_stack.push_front(cargo);
      }

      stacks[to - 1].append(&mut temp_stack);
    }

    let result = stacks
      .iter()
      .map(|s| s.back().expect("stack should contain cargo"))
      .collect::<String>();

    Ok(result)
  }
}

#[cfg(test)]
mod tests {
  use crate::utils::{input_reader, puzzle::Puzzle};

  use std::error::Error;

  #[test]
  fn solve_example() -> Result<(), Box<dyn Error>> {
    let lines = input_reader::example_for_day(5)?;
    let result = super::Day5Part2::solve(lines)?;

    assert_eq!(result, "MCD".to_string());

    Ok(())
  }

  #[test]
  fn solve() -> Result<(), Box<dyn Error>> {
    let lines = input_reader::for_day(5)?;
    let result = super::Day5Part2::solve(lines)?;

    assert_eq!(result, "GMPMLWNMG".to_string());

    Ok(())
  }
}
