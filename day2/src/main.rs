use std::error::Error;
use std::str::FromStr;
use sdk::*;
use sdk::anyhow::{anyhow, bail};
use sdk::winnow::combinator::separated;
use sdk::winnow::token::take_while;

fn main() -> anyhow::Result<()> {
    init();
    let result = go(include_str!("example.txt"), is_invalid_part_1)?;
    info!("Part 1 Example result: {result}");
    let result = go(include_str!("input.txt"), is_invalid_part_1)?;
    info!("Part 1 result: {result}");
    let result = go(include_str!("example.txt"), is_invalid_part_2)?;
    info!("Part 2 Example result: {result}");
    let result = go(include_str!("input.txt"), is_invalid_part_2)?;
    info!("Part 2 result: {result}");
    Ok(())
}

fn go(mut input: &str, is_invalid: impl Fn(u64) -> bool) -> anyhow::Result<u64> {
    let ranges: Vec<IdRange> = separated(0.., parse_range, ',')
        .parse_next(&mut input)
        .map_err(|e| anyhow!("{e}"))?;

    let sum = ranges.into_iter()
        .flat_map(|r| r.start..=r.end)
        .filter(|id| is_invalid(*id))
        .sum();

    Ok(sum)
}

fn parse_range(input: &mut &str) -> winnow::Result<IdRange> {
    (
        take_while(1.., |c: char| c.is_numeric()),
        '-',
        take_while(1.., |c: char| c.is_numeric()),
    ).try_map(|(start, _, end): (&str, char, &str)| -> Result<IdRange, <u64 as FromStr>::Err> {
        let range = IdRange {
            start: start.parse()?,
            end: end.parse()?,
        };
        Ok(range)
    }).parse_next(input)
}

fn is_invalid_part_1(id: u64) -> bool {
    let mut digits: u32 = 0;
    let mut x = id;
    while x > 0 {
        digits += 1;
        x = x / 10;
    }
    if digits % 2 == 1 {
        return false;
    }
    let pivot = 10_u64.pow(digits / 2);
    let front = id / pivot;
    let back = id % pivot;
    // debug!("id: {id}, front: {front}, back: {back}");
    front == back
}

fn is_invalid_part_2(id: u64) -> bool {
    let id = id.to_string();
    let len = id.len();
    let mut test = String::with_capacity(len);
    let pivot = id.len() / 2;
    for i in 1..=pivot {
        if len % i == 0 {
            (0..(len / i)).for_each(|_| test.push_str(&id[..i]));
            if id == test {
                return true;
            } else {
                test.clear()
            }
        }
    }
    false
}

#[derive(Debug, Clone)]
struct IdRange {
    start: u64,
    end: u64,
}
