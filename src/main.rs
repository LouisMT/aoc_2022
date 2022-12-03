mod puzzles;
mod utils;

use crate::utils::{input_reader, puzzle::Puzzle};

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  let input = input_reader::for_day(1)?;
  let result = puzzles::day1::part1::Day1Part1::solve(input)?;

  println!("Result: {}", result);

  Ok(())
}
