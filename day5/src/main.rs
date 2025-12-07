use std::cmp::max;
use std::mem;
use std::ops::{RangeInclusive};
use sdk::*;
use sdk::anyhow::anyhow;
use sdk::winnow::ascii::dec_uint;
use sdk::winnow::combinator::separated;

fn main() -> anyhow::Result<()> {
    init();
    let output = go(include_str!("example.txt"), true)?;
    info!("Part 1 example output: {output}");
    let output = go(include_str!("input.txt"), true)?;
    info!("Part 1 output: {output}");
    let output = go(include_str!("example.txt"), false)?;
    info!("Part 2 example output: {output}");
    let output = go(include_str!("input.txt"), false)?;
    info!("Part 2 output: {output}");
    Ok(())
}

fn go(mut input: &str, only_included_ingredients: bool) -> anyhow::Result<u64> {
    let (mut fresh_ranges, ids): (Vec<RangeInclusive<u64>>, Vec<u64>) = (
        separated(1.., parse_range, '\n'),
        "\n\n",
        separated(1.., dec_uint::<&str, u64, _>, '\n')
    )
        .map(|(id_ranges, _, ids): (Vec<RangeInclusive<u64>>, _, Vec<u64>)| {
            (id_ranges, ids)
        })
        .parse_next(&mut input)
        .map_err(|e| anyhow!("{e}"))?;

    merge_ranges(&mut fresh_ranges);

    let fresh = if only_included_ingredients {
        ids.into_iter().filter(|id| fresh_ranges.iter().any(|r| r.contains(id))).count() as u64
    } else {
        fresh_ranges.iter().map(|r| (r.end() + 1) - r.start()).sum()
    };

    Ok(fresh)
}

fn parse_range(input: &mut &str) -> winnow::Result<RangeInclusive<u64>> {
    (dec_uint, '-', dec_uint).map(|(from, _, to)| from..=to).parse_next(input)
}

fn merge_ranges(ranges: &mut Vec<RangeInclusive<u64>>) {
    ranges.sort_by_key(|r| *r.start());
    *ranges = mem::take(ranges).into_iter().fold(Vec::new(), |mut ranges, next| {
        if let Some(prev) = ranges.pop() {
            if prev.end() >= next.start() {
                let max_end = *max(prev.end(), next.end());
                let merged = (*prev.start())..=max_end;
                // debug!("Merging ranges {prev:?} & {next:?} into {merged:?}");
                ranges.push(merged);
            } else {
                ranges.push(prev);
                ranges.push(next);
            }
        } else {
            ranges.push(next);
        }
        ranges
    });
}