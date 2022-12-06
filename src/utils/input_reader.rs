use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;

use itertools::Itertools;

pub fn for_day(day: i32) -> Result<Vec<String>, Box<dyn Error>> {
  read_lines(format!("inputs/day_{}.txt", day))
}

#[cfg(test)]
pub fn example_for_day(day: i32) -> Result<Vec<String>, Box<dyn Error>> {
  read_lines(format!("inputs/day_{}_example.txt", day))
}

#[cfg(test)]
pub fn example_for_day_and_number(day: i32, number: i32) -> Result<Vec<String>, Box<dyn Error>> {
  read_lines(format!("inputs/day_{}_example_{}.txt", day, number))
}

fn read_lines<P>(path: P) -> Result<Vec<String>, Box<dyn Error>>
where
  P: AsRef<Path>,
{
  let lines = read_to_string(path)?
    .lines()
    .map(|l| l.to_string())
    .collect_vec();

  Ok(lines)
}
