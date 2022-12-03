use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

pub fn for_day(day: i32) -> Result<Lines<BufReader<File>>, Box<dyn Error>> {
  let filename = format!("inputs/day_{}.txt", day);
  let input = File::open(filename)?;
  let reader = BufReader::new(input);

  Ok(reader.lines())
}
