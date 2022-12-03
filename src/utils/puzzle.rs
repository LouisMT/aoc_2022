use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Lines};

pub trait Puzzle {
  fn solve(input: Lines<BufReader<File>>) -> Result<i32, Box<dyn Error>>;
}
