use crate::utils::puzzle::Puzzle;

use itertools::Itertools;

use std::collections::HashSet;
use std::error::Error;

pub struct Day9Part1();

impl Puzzle<usize> for Day9Part1 {
  fn solve(lines: Vec<String>) -> Result<usize, Box<dyn Error>> {
    let mut head = Point { x: 0, y: 0 };
    let mut tail = Point { x: 0, y: 0 };
    let mut visited = HashSet::<Point>::new();

    for line in lines {
      let (direction, amount) = match line.split(' ').collect_tuple() {
        Some((direction, amount)) => (direction, amount.parse::<usize>()?),
        None => panic!("invalid input"),
      };

      for _ in 0..amount {
        head = match direction {
          "U" => head.up(),
          "D" => head.down(),
          "L" => head.left(),
          "R" => head.right(),
          direction => panic!("unknown direction {}", direction),
        };

        tail = tail.follow(&head);
        visited.insert(tail);
      }
    }

    Ok(visited.len())
  }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
  x: isize,
  y: isize,
}

impl Point {
  fn left(&self) -> Point {
    Point {
      x: self.x - 1,
      y: self.y,
    }
  }

  fn right(&self) -> Point {
    Point {
      x: self.x + 1,
      y: self.y,
    }
  }

  fn up(&self) -> Point {
    Point {
      x: self.x,
      y: self.y + 1,
    }
  }

  fn down(&self) -> Point {
    Point {
      x: self.x,
      y: self.y - 1,
    }
  }

  fn follow(&self, head: &Point) -> Point {
    // 11T22
    // 1NNN2
    // LNHNR -> H = head
    // 3NNN4
    // 33B44

    match (self.x - head.x, self.y - head.y) {
      // Case L
      (-2, 0) => self.right(),
      // Case R
      (2, 0) => self.left(),
      // Case B
      (0, -2) => self.up(),
      // Case T
      (0, 2) => self.down(),
      // Case 1
      (-1, 2) | (-2, 1) | (-2, 2) => self.right().down(),
      // Case 2
      (1, 2) | (2, 1) | (2, 2) => self.left().down(),
      // Case 3
      (-1, -2) | (-2, -1) | (-2, -2) => self.right().up(),
      // Case 4
      (1, -2) | (2, -1) | (2, -2) => self.left().up(),
      // Case N or H
      _ => *self,
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::utils::{input_reader, puzzle::Puzzle};

  use std::error::Error;

  #[test]
  fn solve_example() -> Result<(), Box<dyn Error>> {
    let lines = input_reader::example_for_day_and_number(9, 1)?;
    let result = super::Day9Part1::solve(lines)?;

    assert_eq!(result, 13);

    Ok(())
  }

  #[test]
  fn solve() -> Result<(), Box<dyn Error>> {
    let lines = input_reader::for_day(9)?;
    let result = super::Day9Part1::solve(lines)?;

    assert_eq!(result, 6339);

    Ok(())
  }
}
