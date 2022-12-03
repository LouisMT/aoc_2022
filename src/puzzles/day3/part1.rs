use crate::utils::puzzle::Puzzle;

use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Lines};

pub struct Day3Part1();

impl Puzzle for Day3Part1 {
  fn solve(lines: Lines<BufReader<File>>) -> Result<i32, Box<dyn Error>> {
    let mut total_priority = 0;

    for line_res in lines {
      let items: Vec<char> = line_res?.chars().collect();
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
