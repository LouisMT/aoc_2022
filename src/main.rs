mod puzzles;
mod utils;

use crate::utils::{input_reader, puzzle::Puzzle};

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  let input = input_reader::for_day(3)?;
  let result = puzzles::day3::part1::Day3Part1::solve(input)?;

  println!("Result: {}", result);

  Ok(())
}
