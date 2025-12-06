use sdk::anyhow::anyhow;
use sdk::winnow::ascii::{dec_int, dec_uint};
use sdk::winnow::combinator::separated;
use sdk::winnow::error::{ContextError, InputError};
use sdk::winnow::token::one_of;
use sdk::*;

fn main() -> anyhow::Result<()> {
    init();
    let output = go(include_str!("example.txt"))?;
    info!("Example password: {output}");
    let output = go(include_str!("input.txt"))?;
    info!("Real password: {output}");
    Ok(())
}

fn go(mut input: &str) -> anyhow::Result<i32> {
    const START: i32 = 50;
    const DIAL_SIZE: i32 = 100;
    let turns: Vec<Turn> = separated(0.., parse_turn, '\n')
        .parse_next(&mut input)
        .map_err(|e| anyhow!("{e}"))?;
    let (zeroes, _) = turns.into_iter().fold((0, START), |(zeroes, position), t| {
        let output = match t {
            Turn::Left(n) => (position - n) % DIAL_SIZE,
            Turn::Right(n) => (position + n) % DIAL_SIZE,
        };
        let output = if output.is_negative() { output + DIAL_SIZE } else { output };
        let zeroes = if output == 0 { zeroes + 1 } else { zeroes };
        debug!("Input: {position}, turn: {t:?}, output: {output}, zero count: {zeroes}");
        (zeroes, output)
    });
    Ok(zeroes)
}

fn parse_turn(input: &mut &str) -> winnow::Result<Turn> {
    (one_of(['L', 'R']), dec_int)
        .try_map(|(dir, n)| match dir {
            'L' => Ok(Turn::Left(n)),
            'R' => Ok(Turn::Right(n)),
            other => Err(InputError::at(other)),
        })
        .parse_next(input)
}

#[derive(Debug, Copy, Clone)]
enum Turn {
    Left(i32),
    Right(i32),
}
