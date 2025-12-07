use std::cmp::min;
use sdk::*;
use sdk::anyhow::{anyhow};
use sdk::winnow::combinator::separated;
use sdk::winnow::token::{take_while};

fn main() -> anyhow::Result<()> {
    init();
    let output = go(include_str!("example.txt"), false)?;
    info!("Part 1 Example output: {output}");
    let output = go(include_str!("input.txt"), false)?;
    info!("Part 1 Output: {output}");
    let output = go(include_str!("example.txt"), true)?;
    info!("Part 2 Example output: {output}");
    let output = go(include_str!("input.txt"), true)?;
    info!("Part 2 Output: {output}");
    Ok(())
}

fn go(mut input: &str, take: bool) -> anyhow::Result<u64> {
    let mut grid = parse_grid(&mut input).map_err(|e| anyhow!("{e}"))?;
    let mut count = 0;
    loop {
        let mut to_take = Vec::new();
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                let neighbors = grid.neighbors(x, y);
                if grid.get(x, y) && neighbors.iter().filter(|x| **x).count() < 4 {
                    count += 1;
                    if take {
                        to_take.push((x, y));
                    }
                }
            }
        }
        if to_take.is_empty() {
            break;
        }
        for (x, y) in to_take {
            grid.set(x, y, false);
        }
    }

    Ok(count)
}

#[derive(Debug, Clone)]
struct Grid {
    inner: Vec<Vec<bool>>,
    height: usize,
    width: usize
}

impl Grid {
    fn new(input: Vec<Vec<bool>>) -> Result<Self, ParseError> {
        if input.is_empty() {
            return Err(ParseError("Empty grid".to_owned()));
        }
        let height = input.len();
        let width = input[0].len();
        if !input.iter().all(|v| v.len() == width) {
            return Err(ParseError("Not all rows are equal length".to_owned()));
        }
        Ok(Grid {
            inner: input,
            height,
            width
        })
    }

    fn height(&self) -> usize {
        self.height
    }

    fn width(&self) -> usize {
        self.width
    }

    fn get(&self, x: usize, y: usize) -> bool {
        self.inner[y][x]
    }

    fn set(&mut self, x: usize, y: usize, value: bool) {
        self.inner[y][x] = value;
    }

    fn neighbors(&self, x: usize, y: usize) -> Vec<bool> {
        let mut neighbors = Vec::new();
        let min_y = y.checked_sub(1).unwrap_or(y);
        let max_y = min(y + 1, self.height - 1);
        let min_x = x.checked_sub(1).unwrap_or(x);
        let max_x = min(x + 1, self.width - 1);
        for check_y in min_y..=max_y {
            for check_x in min_x..=max_x {
                if check_x == x && check_y == y {
                    continue;
                }
                neighbors.push(self.get(check_x, check_y));
            }
        }
        neighbors
    }
}

fn parse_grid(input: &mut &str) -> winnow::Result<Grid> {
    fn parse_row(input: &mut &str) -> winnow::Result<Vec<bool>> {
        take_while(1.., ['.', '@'])
            .map(|row: &str| row.chars().map(|c| c == '@').collect())
            .parse_next(input)
    }

    separated(0.., parse_row, '\n')
        .try_map(Grid::new)
        .parse_next(input)
}