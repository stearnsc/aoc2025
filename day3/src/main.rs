use sdk::anyhow::anyhow;
use sdk::winnow::combinator::separated;
use sdk::winnow::token::take_while;
use sdk::*;

fn main() -> anyhow::Result<()> {
    init();
    let output = go(include_str!("example.txt"), 2)?;
    info!("Part 1 Example output: {output}");
    let output = go(include_str!("input.txt"), 2)?;
    info!("Part 1 output: {output}");
    let output = go(include_str!("example.txt"), 12)?;
    info!("Part 2 Example output: {output}");
    let output = go(include_str!("input.txt"), 12)?;
    info!("Part 2 output: {output}");
    Ok(())
}

fn go(mut input: &str, cell_count: usize) -> anyhow::Result<u64> {
    let batteries: Vec<_> = separated(0.., parse_battery, '\n')
        .parse_next(&mut input)
        .map_err(|e| anyhow!("{e}"))?;

    let sum = batteries.into_iter().map(|b| b.max_joltage(cell_count)).sum();

    Ok(sum)
}

struct Battery(Vec<u8>);

impl Battery {
    fn max_joltage(&self, cell_count: usize) -> u64 {
        let mut digits = vec![0_u64; cell_count];
        let len = self.0.len();
        for (battery_i, &cell) in self.0.iter().enumerate() {
            let cell = cell as u64;
            // remaining cells available to use after the current cell
            let remaining_cells = len - battery_i - 1;

            for digits_i in 0..cell_count {
                // remaining digits we must set
                let remaining_digits = cell_count - digits_i - 1;
                if cell > digits[digits_i] && remaining_cells >= remaining_digits {
                    digits[digits_i] = cell;
                    // Clear out remaining digits
                    for i in (digits_i + 1)..cell_count {
                        digits[i] = 0;
                    }
                    break;
                }
            }
        }
        debug!("Battery: {:?}, digits: {digits:?}", self.0);
        digits
            .iter()
            .enumerate()
            .map(|(n, d)| d * 10_u64.pow(cell_count as u32 - n as u32 - 1))
            .sum()
    }
}

fn parse_battery(input: &mut &str) -> winnow::Result<Battery> {
    take_while(1.., |c: char| c.is_numeric())
        .map(|s: &str| {
            Battery(
                s.chars()
                    .map(|c| c.to_digit(10).expect("{c} was not a digit") as u8)
                    .collect(),
            )
        })
        .parse_next(input)
}
