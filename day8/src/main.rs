use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::mem;
use sdk::*;
use sdk::anyhow::{anyhow, bail};
use sdk::winnow::ascii::dec_uint;
use sdk::winnow::combinator::separated;

fn main() -> anyhow::Result<()> {
    init();
    let output = go(include_str!("example.txt"))?;
    info!("Part 1 example output: {output}");
    Ok(())
}

fn go(mut input: &str) -> anyhow::Result<usize> {
    let boxes = parse_boxes(&mut input).map_err(|e| anyhow!("{e}"))?;
    let mut circuits: Vec<HashSet<JBox>> = Vec::new();
    // JBox -> index of circuit in `circuits`
    let mut circuit_membership: HashMap<JBox, usize> = HashMap::new();

    let mut nearest_neighbors: Vec<_> = boxes.iter()
        .filter_map(|jbox| nearest_neighbor(jbox, &boxes).map(|neighbor| {
            (jbox.distance_to(&neighbor), *jbox, neighbor)
        }))
        .collect();
    nearest_neighbors.sort_by(|(dist_a, _, _), (dist_b, _, _)| f64::total_cmp(dist_a, dist_b));

    for (_, a, b) in nearest_neighbors.iter().take(10) {
        match (circuit_membership.get(a), circuit_membership.get(b)) {
            (None, None) => {
                let mut new_circuit = HashSet::new();
                new_circuit.insert(*a);
                new_circuit.insert(*b);
                circuits.push(new_circuit);
                let circuit_id = circuits.len() - 1;
                circuit_membership.insert(*a, circuit_id);
                circuit_membership.insert(*b, circuit_id);
            }
            (None, Some(circuit)) => {
                circuits[*circuit].insert(*a);
                circuit_membership.insert(*a, *circuit);
            }
            (Some(circuit), None) => {
                circuits[*circuit].insert(*b);
                circuit_membership.insert(*b, *circuit);
            }
            (Some(a_circuit), Some(b_circuit)) => {
                let keep_id = min(*a_circuit, *b_circuit);
                let merge_id = max(*a_circuit, *b_circuit);
                let to_migrate = mem::take(&mut circuits[merge_id]);
                for jbox in &to_migrate {
                    circuit_membership.insert(*jbox, keep_id);
                }
                circuits[keep_id].extend(to_migrate);
            }
        }
    }
    debug!("circuits: {circuits:?}");
    let mut circuit_sizes: Vec<_> = circuits.iter().map(|boxes| boxes.len())
        .collect();
    circuit_sizes.sort_by_key(|len| usize::MAX - *len);
    let result = circuit_sizes.into_iter().take(5).product();
    Ok(result)
}

fn nearest_neighbor(jbox: &JBox, boxes: &[JBox]) -> Option<JBox> {
    boxes.iter().fold(None, |closest, next| {
        if *next == *jbox {
            return closest;
        }
        let Some(closest) = closest else {
            return Some(*next);
        };
        if jbox.distance_to(next) < jbox.distance_to(&closest) {
            Some(*next)
        } else {
            Some(closest)
        }
    })
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct JBox {
    x: usize,
    y: usize,
    z: usize,
}

impl JBox {
    fn distance_to(&self, other: &JBox) -> f64 {
        (
            (self.x as f64 - other.x as f64).powi(2) +
                (self.y as f64 - other.y as f64).powi(2) +
                (self.z as f64 - other.z as f64).powi(2)
        ).sqrt()
    }
}

fn parse_jbox(input: &mut &str) -> winnow::Result<JBox> {
    separated(3, dec_uint::<_, usize, _>, ',')
        .map(|n: Vec<usize>| JBox { x: n[0], y: n[1], z: n[2] })
        .parse_next(input)
}

fn parse_boxes(input: &mut &str) -> winnow::Result<Vec<JBox>> {
    separated(1.., parse_jbox, '\n').parse_next(input)
}
