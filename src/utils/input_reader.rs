use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

pub fn for_day(day: i32) -> Result<Lines<BufReader<File>>, Box<dyn Error>> {
  read_input(format!("inputs/day_{}.txt", day))
}

#[cfg(test)]
pub fn example_for_day(day: i32) -> Result<Lines<BufReader<File>>, Box<dyn Error>> {
  read_input(format!("inputs/day_{}_example.txt", day))
}

fn read_input<P>(path: P) -> Result<Lines<BufReader<File>>, Box<dyn Error>>
where
  P: AsRef<Path>,
{
  let input = File::open(path)?;
  let reader = BufReader::new(input);

  Ok(reader.lines())
}
