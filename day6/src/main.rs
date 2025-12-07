use std::ops::Range;
use std::str::FromStr;
use sdk::*;
use sdk::anyhow::{anyhow, bail};

fn main() -> anyhow::Result<()> {
    init();
    let output = go(include_str!("example.txt"), false)?;
    info!("Part 1 example output: {output}");
    let output = go(include_str!("input.txt"), false)?;
    info!("Part 1 output: {output}");
    let output = go(include_str!("example.txt"), true)?;
    info!("Part 2 example output: {output}");
    let output = go(include_str!("input.txt"), true)?;
    info!("Part 2 output: {output}");
    Ok(())
}

fn go(input: &str, cephalopodize: bool) -> anyhow::Result<u64> {
    let mut lines: Vec<String> = input.lines().map(|s| s.to_owned()).collect();
    let Some(max_line_length) = lines.iter().map(|l| l.len()).max() else {
        bail!("empty input");
    };
    // Ensure all lines have the same length. this allow us to index whitespace as 0 when cephalopoding
    for line in &mut lines {
        while line.len() < max_line_length {
            line.push(' ');
        }
    }

    // Remove the line with the operations
    let Some(operations) = lines.pop() else {
        bail!("empty input");
    };

    // [(index of start of digits, + or *)]
    let problems: Vec<(usize, char)> = operations.chars().enumerate().filter(|(_, c)| !c.is_whitespace())
        .collect();

    //[(start..end, + or *)]
    let mut problem_ranges: Vec<(Range<usize>, char)> = problems.as_slice().windows(2).map(|window| {
        let start = window[0].0;
        // subtract one to account for the whitespace between problems
        let end = window[1].0 - 1;
        (start..end, window[0].1)
    }).collect();
    // Last problem doesn't have partner in window, so we have to manually add it
    if let Some((i, c)) = problems.last() {
        problem_ranges.push(((*i)..max_line_length, *c))
    }


    let problem_inputs = if cephalopodize {
        let mut ceph_inputs = vec![Vec::new(); problems.len()];

        for (i, (range, _)) in problem_ranges.iter().enumerate() {
            ceph_inputs[i] = vec![String::new(); range.len()];
        }
        for (y, line) in lines.iter().enumerate() {
            // ones, tens, hundreds, etc
            let place = lines.len() as u32 - y as u32 - 1;
            for (problem_i, (range, _)) in problem_ranges.iter().enumerate() {
                let digits = &line[range.start..range.end];
                for (input_i, digit) in digits.chars().enumerate() {
                    if digit.is_numeric() {
                        ceph_inputs[problem_i][input_i].push(digit);
                    }
                }
            }
        }
        debug!("Cephalopod inputs: {ceph_inputs:?}");
        ceph_inputs
            .into_iter()
            .map(|s| {
                s.into_iter()
                    .map(|input| input.trim().parse::<u64>())
                    .collect::<Result<Vec<_>, _>>()
            }).collect::<Result<Vec<_>, _>>()?
    } else {
        let mut problem_inputs = vec![Vec::new(); problems.len()];
        for line in lines {
            for (x, (problem_range, _)) in problem_ranges.iter().enumerate() {
                let n: u64 = line[problem_range.start..problem_range.end].trim().parse()?;
                problem_inputs[x].push(n);
            }
        }
        problem_inputs
    };

    let mut total = 0;
    for ((_, op), stack) in problems.into_iter().zip(problem_inputs) {
        match op {
            '+' => total += stack.into_iter().sum::<u64>(),
            '*' => total += stack.into_iter().product::<u64>(),
            other => bail!("Unexpected op: {other}"),
        }
    }

    Ok(total)
}
