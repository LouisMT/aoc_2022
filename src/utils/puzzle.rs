use std::error::Error;

pub trait Puzzle<A> {
  fn solve(input: Vec<String>) -> Result<A, Box<dyn Error>>;
}
