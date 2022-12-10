use crate::utils::puzzle::Puzzle;

use itertools::Itertools;

use std::error::Error;

pub struct Day10Part2();

impl Puzzle<String> for Day10Part2 {
  fn solve(lines: Vec<String>) -> Result<String, Box<dyn Error>> {
    let mut value = 1;
    let mut clock = 0;
    let mut crt_pixels = Vec::<char>::new();

    for line in lines {
      let cycles = match line.split(' ').collect_vec().as_slice() {
        ["noop"] => vec![0],
        ["addx", amount] => {
          let amount = amount.parse::<isize>()?;
          vec![0, amount]
        }
        _ => panic!("invalid input"),
      };

      for cycle in cycles {
        clock += 1;
        let pixel = (clock - 1) % 40;

        if value >= pixel - 1 && value <= pixel + 1 {
          crt_pixels.push('#');
        } else {
          crt_pixels.push('.');
        }

        value += cycle;
      }
    }

    Ok(crt_pixels.chunks(40).map(String::from_iter).join("\n"))
  }
}

#[cfg(test)]
mod tests {
  use crate::utils::{input_reader, puzzle::Puzzle};

  use std::error::Error;

  #[test]
  fn solve_example() -> Result<(), Box<dyn Error>> {
    let lines = input_reader::example_for_day(10)?;
    let result = super::Day10Part2::solve(lines)?;

    assert_eq!(
      result,
      vec![
        "##..##..##..##..##..##..##..##..##..##..",
        "###...###...###...###...###...###...###.",
        "####....####....####....####....####....",
        "#####.....#####.....#####.....#####.....",
        "######......######......######......####",
        "#######.......#######.......#######....."
      ]
      .join("\n")
    );

    Ok(())
  }

  #[test]
  fn solve() -> Result<(), Box<dyn Error>> {
    let lines = input_reader::for_day(10)?;
    let result = super::Day10Part2::solve(lines)?;

    assert_eq!(
      result,
      vec![
        "####...##..##..####.###...##..#....#..#.",
        "#.......#.#..#.#....#..#.#..#.#....#..#.",
        "###.....#.#....###..#..#.#....#....####.",
        "#.......#.#....#....###..#.##.#....#..#.",
        "#....#..#.#..#.#....#....#..#.#....#..#.",
        "####..##...##..#....#.....###.####.#..#."
      ]
      .join("\n")
    );

    Ok(())
  }
}
