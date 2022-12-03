use crate::utils::puzzle::Puzzle;

use std::cmp::max;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Lines};

pub struct Day1Part1();

impl Puzzle for Day1Part1 {
  fn solve(lines: Lines<BufReader<File>>) -> Result<i32, Box<dyn Error>> {
    let mut current_calories = 0;
    let mut max_calories = 0;

    for line_res in lines {
      match line_res? {
        line if line.is_empty() => {
          max_calories = max(current_calories, max_calories);
          current_calories = 0;
        }

        line => {
          let number = line.parse::<i32>()?;
          current_calories += number
        }
      }
    }

    Ok(max_calories)
  }
}
