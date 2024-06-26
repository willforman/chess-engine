use std::fmt::Display;
use std::io::Write;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};

use crate::evaluation::EvaluatePosition;
use crate::move_gen::GenerateMoves;
use crate::position::{Move, Position};

#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct SearchParams {
    pub search_moves: Option<Vec<Move>>,
    pub ponder: bool,
    pub white_time_msec: Option<u64>,
    pub black_time_msec: Option<u64>,
    pub white_inc_msec: Option<u64>,
    pub black_inc_msec: Option<u64>,
    pub moves_to_go: Option<u64>,
    pub max_depth: Option<u64>, // Done
    pub max_nodes: Option<u64>, // Done
    pub mate: Option<u64>,
    pub move_time_msec: Option<u64>, // Done
    pub infinite: bool,              // Done
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResultInfo {
    pub positions_processed: u64,
    pub time_elapsed: Duration,
}

enum SearchInfo {
    Depth {
        plies: u32,
    },
    SelDepth {
        plies: u32,
    },
    Time {
        msec: u64,
    },
    Nodes {
        nodes: u64,
    },
    Pv {
        moves: Vec<Move>,
    },
    MultiPv {
        num: u8,
    },
    Score {
        centipawns: f32,
        mate_moves: Option<i32>,
        lower_bound: Option<bool>,
        upper_bound: Option<bool>,
    },
    CurrMove {
        mve: Move,
    },
    CurrMoveNumber {
        move_num: u32,
    },
    HashFull {
        per_mill_full: u16,
    },
    NodesPerSecond {
        nodes_per_sec: f32,
    },
    TableHits {
        hits: u64,
    },
    ShredderHits {
        hits: u64,
    },
    CPULoad {
        cpu_usage_per_mill: u16,
    },
    String {
        str: String,
    },
    Refutation {
        moves: Vec<Move>,
    },
    CurrLine {
        moves: Vec<Move>,
        cpu_num: Option<u8>,
    },
}

fn moves_to_string(moves: &[Move]) -> String {
    moves
        .iter()
        .map(|mve| mve.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

impl Display for SearchInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let info_str = match self {
            SearchInfo::Depth { plies } => format!("depth {}", plies),
            SearchInfo::SelDepth { plies } => format!("seldepth {}", plies),
            SearchInfo::Time { msec } => format!("time {}", msec),
            SearchInfo::Nodes { nodes } => format!("nodes {}", nodes),
            SearchInfo::Pv { moves } => format!("pv {}", moves_to_string(moves)),
            SearchInfo::MultiPv { num } => format!("multipv {}", num),
            SearchInfo::Score {
                centipawns,
                mate_moves,
                lower_bound,
                upper_bound,
            } => {
                let mut score_str = format!("score cp {}", centipawns);

                if let Some(mate_moves) = mate_moves {
                    score_str.push_str(format!(" mate {}", mate_moves).as_str());
                };

                if lower_bound.is_some() {
                    score_str.push_str(" lowerbound");
                }
                if upper_bound.is_some() {
                    score_str.push_str(" upperbound");
                }

                score_str
            }
            SearchInfo::CurrMove { mve } => format!("currmove {}", mve),
            SearchInfo::CurrMoveNumber { move_num } => format!("currmovenumber {}", move_num),
            SearchInfo::HashFull { per_mill_full } => format!("hashfull {}", per_mill_full),
            SearchInfo::NodesPerSecond { nodes_per_sec } => format!("nps {}", nodes_per_sec),
            SearchInfo::TableHits { hits } => format!("tbhits {}", hits),
            SearchInfo::ShredderHits { hits } => format!("sbhits {}", hits),
            SearchInfo::CPULoad { cpu_usage_per_mill } => format!("cpuload {}", cpu_usage_per_mill),
            SearchInfo::String { str } => format!("string {}", str),
            SearchInfo::Refutation { moves } => format!("refutation {}", moves_to_string(moves)),
            SearchInfo::CurrLine { moves, cpu_num } => {
                if let Some(cpu_num) = cpu_num {
                    format!("currline {} {}", cpu_num, moves_to_string(moves))
                } else {
                    format!("currline {}", moves_to_string(moves))
                }
            }
        };
        write!(f, "{}", info_str)
    }
}

pub fn search(
    position: &Position,
    params: &SearchParams,
    move_gen: impl GenerateMoves + std::marker::Copy,
    position_eval: impl EvaluatePosition + std::marker::Copy,
    info_writer: Arc<Mutex<impl Write>>,
    terminate: Arc<AtomicBool>,
) -> (Option<Move>, SearchResultInfo) {
    let mut positions_processed: u64 = 0;
    let start = Instant::now();
    let (mut best_move, best_val) = search_helper(
        position,
        params,
        0,
        &mut positions_processed,
        &start,
        f64::MIN,
        f64::MAX,
        move_gen,
        position_eval,
        info_writer,
        Arc::clone(&terminate),
    );

    // If mate is passed, only return a move if it's a mate
    if params.mate.is_some() && (best_val == f64::MIN || best_val == f64::MAX) {
        best_move = None;
    }

    let search_info = SearchResultInfo {
        positions_processed,
        time_elapsed: start.elapsed(),
    };

    (best_move, search_info)
}

#[allow(clippy::too_many_arguments)]
fn search_helper(
    position: &Position,
    params: &SearchParams,
    curr_depth: u64,
    positions_processed: &mut u64,
    start_time: &Instant,
    mut alpha: f64,
    beta: f64,
    move_gen: impl GenerateMoves + std::marker::Copy,
    position_eval: impl EvaluatePosition + std::marker::Copy,
    info_writer: Arc<Mutex<impl Write>>,
    terminate: Arc<AtomicBool>,
) -> (Option<Move>, f64) {
    // If this search has been terminated, return early
    if terminate.load(std::sync::atomic::Ordering::Relaxed) {
        return (None, 0.0);
    }
    // If this search is at the max number of nodes, return early
    if let Some(max_nodes) = params.max_nodes {
        debug_assert!(*positions_processed <= max_nodes);
        if *positions_processed == max_nodes {
            return (None, 0.0);
        }
    }
    // If search has exceeded total time, return early
    if let Some(move_time_msec) = params.move_time_msec {
        if start_time.elapsed().as_millis() >= u128::from(move_time_msec) {
            return (None, 0.0);
        }
    }

    if *positions_processed % 10000 == 0 {
        write_search_info(*positions_processed, start_time, Arc::clone(&info_writer));
    }

    let moves = move_gen.gen_moves(position);

    let mut best_val = f64::MIN;
    let mut best_move: Option<Move> = None;
    for mve in moves {
        let mut move_position = position.clone();
        move_position.make_move(&mve).unwrap();

        if let Some(max_depth) = params.max_depth {
            if curr_depth + 1 == max_depth {
                let val = position_eval.evaluate(&move_position);
                return (Some(mve), val);
            }
        }

        if let Some(mate) = params.mate {
            if curr_depth + 1 == mate {
                let val = position_eval.evaluate(&move_position);
                return (Some(mve), val);
            }
        }

        let (got_mve, got_val) = search_helper(
            &move_position,
            params,
            curr_depth + 1,
            positions_processed,
            start_time,
            -beta,
            -alpha,
            move_gen,
            position_eval,
            Arc::clone(&info_writer),
            Arc::clone(&terminate),
        );

        // Search has been terminated, return best move we found
        if got_mve.is_none() && got_val == 0.0 {
            break;
        }

        let got_val = -got_val;

        if got_val >= best_val {
            best_val = got_val;
            best_move = Some(mve);
        }

        best_val = f64::max(best_val, got_val);

        alpha = f64::max(alpha, got_val);

        if alpha >= beta {
            break;
        }
    }

    (best_move, best_val)
}

fn write_search_info(
    nodes_processed: u64,
    start_time: &Instant,
    info_writer: Arc<Mutex<impl Write>>,
) {
    // info depth 10 seldepth 6 multipv 1 score mate 3 nodes 971 nps 121375 hashfull 0 tbhits 0 time 8 pv f4g3 e6d6 d2d6 h1g1 d6d1
    let nps = nodes_processed as f32 / start_time.elapsed().as_secs_f32();
    let score_str = format!("mate 3");
    let info = format!("info depth {} seldepth {} multipv {} score {} nodes {} nps {:.0} hashfull {} tbhits {} time {} pv {}", 1, 1, 1, score_str, nodes_processed, nps, 0, 0, 0, "");
    info_writer
        .lock()
        .unwrap()
        .write_all(info.as_bytes())
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    use testresult::TestResult;

    use crate::bitboard::Square::*;
    use crate::evaluation::POSITION_EVALUATOR;
    use crate::move_gen::HYPERBOLA_QUINTESSENCE_MOVE_GEN;
    use crate::position::Move;

    #[derive(Clone, Copy)]
    struct DummyInfoWriter;

    impl Write for DummyInfoWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            let string = String::from_utf8(buf.to_vec()).unwrap();
            println!("{}", string);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    #[test_case(Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap(), 
    &[
        Move::new(D2, D4), Move::new(E7, E6),
        Move::new(C2, C4), Move::new(B8, C6),
        Move::new(B1, C3), Move::new(D8, H4),
        Move::new(G1, F3), Move::new(H4, G4),
        Move::new(H2, H3), Move::new(G4, G6),
        Move::new(E2, E4), Move::new(F8, B4),
        Move::new(F1, D3), Move::new(G6, G2),
        Move::new(A2, A3), Move::new(G2, H1),
        Move::new(E1, E2)
    ]; "random game that caused crash")]
    fn test_search(mut position: Position, moves: &[Move]) -> TestResult {
        for mve in moves {
            position.make_move(mve)?;
        }
        search(
            &position,
            &SearchParams {
                max_depth: Some(3),
                ..SearchParams::default()
            },
            HYPERBOLA_QUINTESSENCE_MOVE_GEN,
            POSITION_EVALUATOR,
            Arc::new(Mutex::new(DummyInfoWriter)),
            Arc::new(AtomicBool::new(false)),
        );
        Ok(())
    }

    #[test_case(Position::from_fen("k7/7R/6R1/8/8/8/8/K7 b - - 0 1").unwrap(), 2, Some(Move::new(A8, B8)))]
    #[test_case(Position::from_fen("k7/7R/8/8/8/8/8/K7 b - - 0 1").unwrap(), 2, None)]
    fn test_search_mate(
        position: Position,
        mate_moves: u64,
        move_want: Option<Move>,
    ) -> TestResult {
        let params = SearchParams {
            mate: Some(mate_moves),
            ..Default::default()
        };
        let (move_got, _) = search(
            &position,
            &params,
            HYPERBOLA_QUINTESSENCE_MOVE_GEN,
            POSITION_EVALUATOR,
            Arc::new(Mutex::new(DummyInfoWriter)),
            Arc::new(AtomicBool::new(false)),
        );
        assert_eq!(move_got, move_want);
        Ok(())
    }
}
