use std::{time::{Duration, Instant}, fmt::Display};

use tabled::{Tabled,Table};

use crate::{position::Position, move_gen::AllPiecesMoveGen};

#[derive(Clone,Copy,Debug,PartialEq, Eq, Tabled)]
pub struct PerftDepthResult {
    tot: u64,
    captures: u64,
    en_passants: u64,
    castles: u64,
    promotions: u64,
    checks: u64,
    discovery_checks: u64,
    double_checks: u64,
    checkmates: u64
}

pub struct PerftResult {
    pub depth_results: Vec<PerftDepthResult>,
    pub tot_nodes: u64,
    pub time_elapsed: Duration,
    pub nodes_per_second: f64,
}

impl PerftDepthResult {
    pub fn new(
        tot: u64, 
        captures: u64, 
        en_passants: u64, 
        castles: u64, 
        promotions: u64, 
        checks: u64, 
        discovery_checks: u64, 
        double_checks: u64, 
        checkmates: u64
    ) -> Self {
        PerftDepthResult {
            tot,
            captures,
            en_passants,
            castles,
            promotions,
            checks,
            discovery_checks,
            double_checks,
            checkmates
        }

    }
    pub fn empty() -> PerftDepthResult {
        PerftDepthResult::new(0, 0, 0, 0, 0, 0, 0, 0, 0)
    }
}

impl Display for PerftResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "total nodes: {}", self.tot_nodes)?;
        writeln!(f, "time elapsed: {}", self.time_elapsed.as_secs_f32())?;
        writeln!(f, "nodes/s: {}", self.nodes_per_second)?;
        writeln!(f, "{}", Table::new(&self.depth_results).to_string())?;
        Ok(())
    }
}

pub fn perft(position: &Position, move_gen: &AllPiecesMoveGen, depth: usize) -> PerftResult {
    let mut depth_results = vec![PerftDepthResult::empty(); depth];

    let start = Instant::now();

    perft_helper(&mut depth_results, position, move_gen, depth, 0);

    let time_elapsed = start.elapsed();

    let tot_nodes = depth_results.iter()
        .fold(0, |tot, curr| tot + curr.tot);

    let nodes_per_second = tot_nodes as f64 / time_elapsed.as_secs_f64();


    PerftResult { 
        depth_results, 
        tot_nodes,
        time_elapsed,
        nodes_per_second
    }
}

fn perft_helper(depth_results: &mut Vec<PerftDepthResult>, position: &Position, move_gen: &AllPiecesMoveGen, max_depth: usize, curr_depth: usize) {
    if curr_depth == max_depth {
        return;
    }

    let curr_res = depth_results.get_mut(curr_depth).unwrap();

    let moves = move_gen.gen_moves(position);
    curr_res.tot += u64::try_from(moves.len()).unwrap();

    for mve in moves {
        let mut move_position = position.clone();
        move_position.make_move(mve).unwrap();

        perft_helper(depth_results, &move_position, move_gen, max_depth, curr_depth + 1);
    }
}