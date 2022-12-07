use crate::utils::puzzle::Puzzle;

use itertools::Itertools;

use std::collections::HashMap;
use std::error::Error;

pub struct Day7Part1();

impl Puzzle<usize> for Day7Part1 {
  fn solve(lines: Vec<String>) -> Result<usize, Box<dyn Error>> {
    let mut dir_sizes = HashMap::<String, usize>::new();
    let mut current_dir = Vec::<String>::new();

    for line in lines {
      match line.split(' ').collect_vec().as_slice() {
        ["$", "cd", "/"] => {
          current_dir.clear();
          current_dir.push(String::from(""));
        }
        ["$", "cd", ".."] => {
          current_dir.pop();
        }
        ["$", "cd", dir] => current_dir.push(dir.to_string()),
        ["$", "ls"] => (),
        ["dir", _dir] => (),
        [size, _file] => {
          let size = size.parse::<usize>()?;

          for depth in 1..=current_dir.len() {
            let path = current_dir[0..depth].join("/");
            let total_size = dir_sizes.get(&path).unwrap_or(&0);
            dir_sizes.insert(path, total_size + size);
          }
        }
        _ => panic!("invalid input"),
      }
    }

    let size_sum = dir_sizes.values().filter(|&&s| s <= 100000).sum();

    Ok(size_sum)
  }
}

#[cfg(test)]
mod tests {
  use crate::utils::{input_reader, puzzle::Puzzle};

  use std::error::Error;

  #[test]
  fn solve_example() -> Result<(), Box<dyn Error>> {
    let lines = input_reader::example_for_day(7)?;
    let result = super::Day7Part1::solve(lines)?;

    assert_eq!(result, 95437);

    Ok(())
  }

  #[test]
  fn solve() -> Result<(), Box<dyn Error>> {
    let lines = input_reader::for_day(7)?;
    let result = super::Day7Part1::solve(lines)?;

    assert_eq!(result, 1444896);

    Ok(())
  }
}
