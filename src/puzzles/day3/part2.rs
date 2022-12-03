use crate::utils::puzzle::Puzzle;

use itertools::Itertools;

use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Lines};

pub struct Day3Part2();

impl Puzzle for Day3Part2 {
  fn solve(lines: Lines<BufReader<File>>) -> Result<i32, Box<dyn Error>> {
    let mut total_priority = 0;

    for lines_chunk in &lines.chunks(3) {
      match lines_chunk
        .map(|l| HashSet::<char>::from_iter(l.unwrap().chars()))
        .reduce(|acc, items| {
          HashSet::<char>::from_iter(acc.intersection(&items).copied())
        }) {
          Some(items) => total_priority += items.into_iter().map(|i| compute_priority(i)).sum::<i32>(),
          None => continue
        }
    }

    Ok(total_priority)
  }
}

fn compute_priority(item: char) -> i32 {
  match item as u32 {
    item if item >= 97 => item - 96,
    item => item - 38,
  }.try_into().unwrap()
}
