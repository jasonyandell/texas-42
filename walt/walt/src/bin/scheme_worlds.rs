//! Batch membership audit of Scheme queries on supplied opening worlds.
//! Input rows: id declaration viewer hand0-bits hand1-bits hand2-bits hand3-bits.
//! No outcome labels enter this process. No support enumeration or player call.
use std::io::{self, Read};
use walt::kernel::{Hidden, Kernel};
use walt::rules::{ContextSet, Domino, DominoSet, Seat};
use walt::scheme::{Budget, Fix, Frame, Registry};

fn main() {
    if let Err(e) = run() {
        eprintln!("scheme_worlds: {e}");
        std::process::exit(1);
    }
}

fn parse_world(row: &str) -> Result<(u64, Frame, walt::kernel::World), String> {
    let fields = row
        .split_whitespace()
        .map(str::parse::<u64>)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    if fields.len() != 7 || ![0, 1, 2, 3, 4, 5, 6, 7, 9].contains(&fields[1]) || fields[2] > 3 {
        return Err("expected id decl viewer and four opening hand masks".into());
    }
    let viewer = Seat::from_index(fields[2] as usize).unwrap();
    let mut hands = [DominoSet::EMPTY; 4];
    let mut seen = 0u64;
    for (i, bits) in fields[3..].iter().copied().enumerate() {
        if bits >= (1 << 28) || bits.count_ones() != 7 || bits & seen != 0 {
            return Err("opening hands must partition all 28 tiles into seven per seat".into());
        }
        seen |= bits;
        hands[i] = Domino::ALL
            .into_iter()
            .filter(|d| bits & (1 << d.index()) != 0)
            .collect();
    }
    let others: Vec<_> = Seat::ALL.into_iter().filter(|s| *s != viewer).collect();
    let pool = hands
        .iter()
        .enumerate()
        .filter(|(i, _)| *i != fields[2] as usize)
        .flat_map(|(_, h)| h.iter())
        .collect();
    let hidden = std::array::from_fn(|i| Hidden {
        seat: others[i],
        capacity: 7,
        voids: ContextSet::EMPTY,
    });
    let kernel = Kernel::new(
        walt::solver::decl_of(fields[1] as usize),
        viewer,
        hands[fields[2] as usize],
        pool,
        hidden,
    )
    .map_err(|e| format!("{e:?}"))?;
    let world = kernel.world(std::array::from_fn(|i| hands[others[i].index()]));
    let frame = Frame::new(kernel, viewer, vec![], DominoSet::EMPTY).map_err(|e| e.to_string())?;
    Ok((fields[0], frame, world))
}

fn run() -> Result<(), String> {
    let paths = std::env::args().skip(1).collect::<Vec<_>>();
    if paths.is_empty() || paths.len() > 64 {
        return Err("pass 1..64 Scheme paths; opening worlds arrive on stdin".into());
    }
    let registry = Registry::standard();
    let queries = paths
        .iter()
        .map(|p| {
            std::fs::read_to_string(p)
                .map_err(|e| e.to_string())?
                .parse::<Fix>()
                .map_err(|e| e.to_string())?
                .compile(&registry)
                .map_err(|e| e.to_string())
        })
        .collect::<Result<Vec<_>, String>>()?;
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .map_err(|e| e.to_string())?;
    let mut output = String::new();
    let mut ids = std::collections::BTreeSet::new();
    for row in input.lines() {
        let (id, frame, world) = parse_world(row)?;
        if !ids.insert(id) {
            return Err("duplicate input id".into());
        }
        let mut mask = 0u64;
        for (i, query) in queries.iter().enumerate() {
            let answers = query
                .evaluate(&frame, &world, &mut Budget::new(1_000_000))
                .map_err(|e| e.to_string())?;
            if !answers.is_empty() {
                mask |= 1 << i;
            }
        }
        output.push_str(&format!("{id}\t{mask}\n"));
    }
    print!("{output}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn refuses_invalid_worlds_instead_of_testing_impossible_ownership() {
        assert!(parse_world("1 6 0 127 16256 2080768 266338304").is_ok());
        assert!(parse_world("1 6 0 127 127 2080768 266338304").is_err());
        assert!(parse_world("1 8 0 127 16256 2080768 266338304").is_err());
        assert!(parse_world("1 6 4 127 16256 2080768 266338304").is_err());
    }
}
