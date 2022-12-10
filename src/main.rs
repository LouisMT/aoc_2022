mod puzzles;
mod utils;

use crate::utils::{input_reader, puzzle::Puzzle};

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  let input = input_reader::for_day(10)?;
  let result = puzzles::day10::part2::Day10Part2::solve(input)?;

  println!("{}", result);

  Ok(())
}
