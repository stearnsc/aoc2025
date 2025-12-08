use std::collections::{HashMap, HashSet};
use sdk::*;
use sdk::anyhow::anyhow;
use sdk::winnow::combinator::separated;
use sdk::winnow::token::take_while;

fn main() -> anyhow::Result<()> {
    init();
    let output = go(include_str!("example.txt"))?;
    info!("Example output: {} splits, {} paths", output.split_count, output.path_count);
    let output = go(include_str!("input.txt"))?;
    info!("Output: {} splits, {} paths", output.split_count, output.path_count);
    Ok(())
}

struct PathCounts {
    split_count: u64,
    path_count: u64,
}

fn go(mut input: &str) -> anyhow::Result<PathCounts> {
    let manifold = parse_manifold(&mut input)
        .map_err(|e| anyhow!("{e}"))?;

    let mut beams = HashSet::new();
    let mut active_paths = HashMap::new();
    let mut emitters = HashSet::new();
    let mut split_count = 0;
    fn in_beam_path(x: usize, y: usize, beams: &HashSet<(usize, usize)>, emitters: &HashSet<(usize, usize)>) -> bool {
        if y == 0 {
            return false;
        }
        beams.contains(&(x, y - 1)) || emitters.contains(&(x, y - 1))
    }
    fn split_beam(
        x: usize,
        y: usize,
        beams: &mut HashSet<(usize, usize)>,
        manifold: &Manifold,
        active_paths: &mut HashMap<(usize, usize), u64>,
        split_count: &mut u64
    ) {
        let active_path_count = active_paths.get(&(x, y - 1)).cloned().expect("Missing path");
        *split_count += 1;
        if x > 0 {
            beams.insert((x - 1, y));
            *active_paths.entry((x - 1, y)).or_default() += active_path_count;
        }
        if (x + 1) < manifold.width {
            beams.insert((x + 1, y));
            *active_paths.entry((x + 1, y)).or_default() += active_path_count;
        }
    }

    for y in 0..manifold.height {
        debug!("Split count: {split_count}");
        debug!("row: {:?}", manifold.inner[y]);
        for x in 0..manifold.width {
            let in_path = in_beam_path(x, y, &beams, &emitters);
            match manifold.get(x, y) {
                None if in_path => {
                    beams.insert((x, y));
                    let inbound_path_count = active_paths.get(&(x, y - 1)).copied();
                    *active_paths.entry((x, y)).or_default() += inbound_path_count.unwrap_or(1);
                }
                Some(Optic::Emitter) => {
                    emitters.insert((x, y));
                }
                Some(Optic::Splitter) if in_path => {
                    split_beam(x, y, &mut beams, &manifold, &mut active_paths, &mut split_count);
                }
                _ => {
                    // do nothing
                }
            }
        }
    }

    info!("Beam map: \n{}", print_beams(&manifold, &beams));

    let path_count = active_paths.iter()
        .filter(|((_, y), _)| *y == manifold.height - 1)
        .map(|(_, count)| *count)
        .sum();
    Ok(PathCounts { split_count, path_count })
}

struct Manifold {
    inner: Vec<Vec<Option<Optic>>>,
    height: usize,
    width: usize,
}

impl Manifold {
    fn new(rows: Vec<Vec<Option<Optic>>>) -> Result<Self, TextError> {
        let height = rows.len();
        let width = rows.first().map(|row| row.len()).unwrap_or(0);
        if !rows.iter().all(|row| row.len() == width) {
            return Err(TextError("Not all rows have equal width".into()));
        }
        Ok(Manifold {
            inner: rows,
            height,
            width,
        })
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn get(&self, x: usize, y: usize) -> Option<Optic> {
        self.inner[y][x]
    }
}

fn parse_manifold(input: &mut &str) -> winnow::Result<Manifold> {
    separated(
        1..,
        take_while(1.., ['.', '^' , 'S']).try_map(|row: &str| {
            row.chars().map(|c| match c {
                '.' => Ok(None),
                'S' => Ok(Some(Optic::Emitter)),
                '^' => Ok(Some(Optic::Splitter)),
                other => Err(TextError(format!("Unexpected input: {other}")))
            }).collect::<Result<Vec<_>, _>>()
        }),
        '\n',
    ).try_map(|rows| Manifold::new(rows))
        .parse_next(input)
}

fn print_beams(manifold: &Manifold, beams: &HashSet<(usize, usize)>) -> String {
    let mut output = String::new();
    for y in 0..manifold.height {
        for x in 0..manifold.width {
            match manifold.get(x, y) {
                Some(Optic::Emitter) => {
                    output.push('S');
                }
                Some(Optic::Splitter) => {
                    output.push('^');
                }
                None if beams.contains(&(x, y)) => {
                    output.push('|');
                }
                None => {
                    output.push('.');
                }
            }
        }
        output.push('\n');
    }
    output
}

#[derive(Debug, Copy, Clone)]
enum Optic {
    Emitter,
    Splitter,
}