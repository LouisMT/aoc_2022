mod puzzles;
mod utils;

use crate::utils::{input_reader, puzzle::Puzzle};

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  let input = input_reader::for_day(3)?;
  let result = puzzles::day3::part2::Day3Part2::solve(input)?;

  println!("Result: {}", result);

  Ok(())
}
