use std::error::Error;

pub trait Puzzle {
  fn solve(input: Vec<String>) -> Result<i32, Box<dyn Error>>;
}
