use crate::utils::puzzle::Puzzle;

use itertools::Itertools;

use std::cmp::max;
use std::error::Error;

pub struct Day8Part2();

impl Puzzle<usize> for Day8Part2 {
  fn solve(lines: Vec<String>) -> Result<usize, Box<dyn Error>> {
    let grid = lines
      .into_iter()
      .map(|row| {
        row
          .chars()
          .map(|col| col.to_digit(10).expect("size should be a digit"))
          .collect_vec()
      })
      .collect_vec();

    let mut max_scenic_score = 0;

    for (x, row) in grid.iter().enumerate() {
      let max_point = Point {
        x: grid.len() - 1,
        y: row.len() - 1,
      };

      for (y, col) in row.iter().enumerate() {
        let point = Point { x, y };

        let scenic_score = [Point::left, Point::right, Point::up, Point::down]
          .into_iter()
          .map(|step| compute_distance(&grid, *col, &max_point, &point, step, 0))
          .reduce(|scenic_score, distance| scenic_score * distance)
          .expect("there should be at least one distance");

        max_scenic_score = max(scenic_score, max_scenic_score);
      }
    }

    Ok(max_scenic_score)
  }
}

struct Point {
  x: usize,
  y: usize,
}

impl Point {
  fn left(&self, _: &Point) -> Option<Point> {
    if self.x == 0 {
      None
    } else {
      Some(Point {
        x: self.x - 1,
        y: self.y,
      })
    }
  }

  fn right(&self, max: &Point) -> Option<Point> {
    if self.x == max.x {
      None
    } else {
      Some(Point {
        x: self.x + 1,
        y: self.y,
      })
    }
  }

  fn up(&self, _: &Point) -> Option<Point> {
    if self.y == 0 {
      None
    } else {
      Some(Point {
        x: self.x,
        y: self.y - 1,
      })
    }
  }

  fn down(&self, max: &Point) -> Option<Point> {
    if self.y == max.y {
      None
    } else {
      Some(Point {
        x: self.x,
        y: self.y + 1,
      })
    }
  }
}

fn compute_distance(
  grid: &Vec<Vec<u32>>,
  height: u32,
  max: &Point,
  current: &Point,
  step: fn(&Point, &Point) -> Option<Point>,
  distance: usize,
) -> usize {
  if let Some(next) = step(current, max) {
    if grid[next.x][next.y] >= height {
      distance + 1
    } else {
      compute_distance(grid, height, max, &next, step, distance + 1)
    }
  } else {
    distance
  }
}

#[cfg(test)]
mod tests {
  use crate::utils::{input_reader, puzzle::Puzzle};

  use std::error::Error;

  #[test]
  fn solve_example() -> Result<(), Box<dyn Error>> {
    let lines = input_reader::example_for_day(8)?;
    let result = super::Day8Part2::solve(lines)?;

    assert_eq!(result, 8);

    Ok(())
  }

  #[test]
  fn solve() -> Result<(), Box<dyn Error>> {
    let lines = input_reader::for_day(8)?;
    let result = super::Day8Part2::solve(lines)?;

    assert_eq!(result, 263670);

    Ok(())
  }
}
