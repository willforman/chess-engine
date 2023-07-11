use strum::IntoEnumIterator;

use std::string::ToString;

use crate::bitboard::{BitBoard, Square, Direction};
use crate::position::{Piece,Side};

use super::{MoveGenError, GenerateLeapingMoves};

struct SquareToMoveDatabase([BitBoard; 64]);

impl SquareToMoveDatabase {
    fn get_bitboard(&self, square: Square) -> BitBoard {
        self.0[square as usize]
    }
}

struct ColoredSquareToMoveDatabase {
    white: SquareToMoveDatabase,
    black: SquareToMoveDatabase,
}

impl ColoredSquareToMoveDatabase {
    fn get_square_database(&self, side: Side) -> &SquareToMoveDatabase {
        match side {
            Side::White => &self.white,
            Side::Black => &self.black,
        }
    }
}

pub struct LeapingPiecesMoveGen {
    pawn_pushes: ColoredSquareToMoveDatabase,
    pawn_atks: ColoredSquareToMoveDatabase,
    knight_atks: SquareToMoveDatabase,
    king_atks: SquareToMoveDatabase,
}

impl LeapingPiecesMoveGen {
    fn new() -> Self {
        Self {
            pawn_pushes: calc_pawn_pushes(),
            pawn_atks: calc_pawn_atks(),
            knight_atks: calc_knight_atks(),
            king_atks: calc_king_atks(),
        }
    }
}

impl GenerateLeapingMoves for LeapingPiecesMoveGen {
    fn gen_moves(&self, piece_type: Piece, square: Square, side: Side) -> BitBoard {
        match piece_type {
            Piece::Pawn => Ok(self.pawn_pushes.get_square_database(side).get_bitboard(square) & self.pawn_atks.get_square_database(side).get_bitboard(square)),
            Piece::Knight => Ok(self.knight_atks.get_bitboard(square)),
            Piece::King => Ok(self.king_atks.get_bitboard(square)),
            _ => panic!("piece type: want [pawn, knight, king], got {}", piece_type.to_string())
        }
    }
}

fn calc_pawn_pushes() -> ColoredSquareToMoveDatabase {
    let white_single_push_dirs: Vec<Vec<Direction>> = vec![vec![Direction::North]]; 
    let white_double_push_dirs: Vec<Vec<Direction>> = vec![vec![Direction::North], vec![Direction::North, Direction::North]]; 
    let black_single_push_dirs: Vec<Vec<Direction>> = vec![vec![Direction::South]]; 
    let black_double_push_dirs: Vec<Vec<Direction>> = vec![vec![Direction::South], vec![Direction::South, Direction::South]]; 
    let edge_push_dirs: Vec<Vec<Direction>> = vec![]; 

    let white_bbs: [BitBoard; 64] = Square::iter()
        .map(|sq| {
            if sq >= Square::A8 || sq <= Square::H1 {
                BitBoard::from_square_shifts(sq, &edge_push_dirs)
            } else if sq <= Square::H2 {
                BitBoard::from_square_shifts(sq, &white_double_push_dirs)
            } else {
                BitBoard::from_square_shifts(sq, &white_single_push_dirs)
            }
        })
        .collect::<Vec<BitBoard>>()
        .try_into()
        .unwrap();

    let black_bbs: [BitBoard; 64] = Square::iter()
        .map(|sq| {
            if sq >= Square::A8 || sq <= Square::H1 {
                BitBoard::from_square_shifts(sq, &edge_push_dirs)
            } else if sq >= Square::A7 {
                BitBoard::from_square_shifts(sq, &black_double_push_dirs)
            } else {
                BitBoard::from_square_shifts(sq, &black_single_push_dirs)
            }
        })
        .collect::<Vec<BitBoard>>()
        .try_into()
        .unwrap();

    ColoredSquareToMoveDatabase {
        white: SquareToMoveDatabase(white_bbs),
        black: SquareToMoveDatabase(black_bbs),
    }
}

fn calc_pawn_atks() -> ColoredSquareToMoveDatabase {
    let white_atk_dirs: Vec<Vec<Direction>> = vec![vec![Direction::North, Direction::East], vec![Direction::North, Direction::West]]; 
    let black_atk_dirs: Vec<Vec<Direction>> = vec![vec![Direction::South, Direction::East], vec![Direction::South, Direction::West]]; 
    let edge_push_dirs: Vec<Vec<Direction>> = vec![]; 

    let white_bbs: [BitBoard; 64] = Square::iter()
        .map(|sq| {
            if sq >= Square::A8 || sq <= Square::H1 {
                BitBoard::from_square_shifts(sq, &edge_push_dirs)
            } else {
                BitBoard::from_square_shifts(sq, &white_atk_dirs)
            }
        })
        .collect::<Vec<BitBoard>>()
        .try_into()
        .unwrap();

    let black_bbs: [BitBoard; 64] = Square::iter()
        .map(|sq| {
            if sq >= Square::A8 || sq <= Square::H1 {
                BitBoard::from_square_shifts(sq, &edge_push_dirs)
            } else {
                BitBoard::from_square_shifts(sq, &black_atk_dirs)
            }
        })
        .collect::<Vec<BitBoard>>()
        .try_into()
        .unwrap();

    ColoredSquareToMoveDatabase {
        white: SquareToMoveDatabase(white_bbs),
        black: SquareToMoveDatabase(black_bbs),
    }
}

fn calc_knight_atks() -> SquareToMoveDatabase {
    let dirs: Vec<Vec<Direction>> = vec![
        vec![Direction::North, Direction::North, Direction::East], 
        vec![Direction::North, Direction::North, Direction::West], 
        vec![Direction::South, Direction::South, Direction::East], 
        vec![Direction::South, Direction::South, Direction::West], 
        vec![Direction::North, Direction::East, Direction::East], 
        vec![Direction::North, Direction::West, Direction::West], 
        vec![Direction::South, Direction::East, Direction::East], 
        vec![Direction::South, Direction::West, Direction::West], 
    ];

    let bbs: [BitBoard; 64] = Square::iter()
        .map(|sq| BitBoard::from_square_shifts(sq, &dirs))
        .collect::<Vec<BitBoard>>()
        .try_into()
        .unwrap();
    SquareToMoveDatabase(bbs)
}

fn calc_king_atks() -> SquareToMoveDatabase {
    let dirs: Vec<Vec<Direction>> = vec![
        vec![Direction::North], 
        vec![Direction::East], 
        vec![Direction::West], 
        vec![Direction::South], 
        vec![Direction::North, Direction::East], 
        vec![Direction::North, Direction::West], 
        vec![Direction::South, Direction::East], 
        vec![Direction::South, Direction::West], 
    ];

    let bbs: [BitBoard; 64] = Square::iter()
        .map(|sq| BitBoard::from_square_shifts(sq, &dirs))
        .collect::<Vec<BitBoard>>()
        .try_into()
        .unwrap();
    SquareToMoveDatabase(bbs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Square::*;
    use test_case::test_case;
    
    #[test_case(D4, BitBoard::from_squares(&[B5, C6, E6, F5, B3, C2, E2, F3]) ; "center")]
    #[test_case(A8, BitBoard::from_squares(&[B6, C7]) ; "corner")]
    #[test_case(A4, BitBoard::from_squares(&[B6, C5, C3, B2]) ; "edge")]
    fn test_calc_knight_atks(square: Square, want: BitBoard) {
        let got = calc_knight_atks();
        let sq_got = got.get_bitboard(square);
        assert_eq!(sq_got, want);
    }

    #[test_case(D4, BitBoard::from_squares(&[C5, D5, E5, C4, E4, C3, D3, E3]) ; "center")]
    #[test_case(A8, BitBoard::from_squares(&[A7, B7, B8]) ; "corner")]
    #[test_case(C1, BitBoard::from_squares(&[B1, B2, C2, D2, D1]) ; "edge")]
    fn test_calc_king_atks(square: Square, want: BitBoard) {
        let got = calc_king_atks();
        let sq_got = got.get_bitboard(square);
        assert_eq!(sq_got, want);
    }

    #[test_case(D2, Side::White, BitBoard::from_squares(&[D3, D4]) ; "white double")]
    #[test_case(B3, Side::White, BitBoard::from_squares(&[B4]) ; "white single")]
    #[test_case(G7, Side::White, BitBoard::from_squares(&[G8]) ; "white single edge")]
    #[test_case(G8, Side::White, BitBoard::from_squares(&[]) ; "white edge")]
    #[test_case(D7, Side::Black, BitBoard::from_squares(&[D6, D5]) ; "black double")]
    #[test_case(B6, Side::Black, BitBoard::from_squares(&[B5]) ; "black single")]
    #[test_case(G2, Side::Black, BitBoard::from_squares(&[G1]) ; "black single edge")]
    #[test_case(G1, Side::Black, BitBoard::from_squares(&[]) ; "black edge")]
    fn test_calc_pawn_pushes(square: Square, side: Side, want: BitBoard) {
        let got = calc_pawn_pushes();
        let sq_got = got.get_square_database(side).get_bitboard(square);
        assert_eq!(sq_got, want);
    }

    #[test_case(D2, Side::White, BitBoard::from_squares(&[C3, E3]) ; "white")]
    #[test_case(A7, Side::White, BitBoard::from_squares(&[B8]) ; "white edge")]
    #[test_case(D7, Side::Black, BitBoard::from_squares(&[C6, E6]) ; "black")]
    #[test_case(A2, Side::Black, BitBoard::from_squares(&[B1]) ; "black edge")]
    fn test_calc_pawn_atks(square: Square, side: Side, want: BitBoard) {
        let got = calc_pawn_atks();
        let sq_got = got.get_square_database(side).get_bitboard(square);
        assert_eq!(sq_got, want);
    }
}
