//use std::{cell::LazyCell, sync::LazyLock};
//
//use arrayvec::ArrayVec;
//use strum::IntoEnumIterator;
//
//use crate::{bitboard::BitBoard, GenerateMoves, Piece, Square, HYPERBOLA_QUINTESSENCE_MOVE_GEN};
//
//use super::{masks::MASKS_LIST, traits::GenerateSlidingMoves};
//
//#[derive(Default)]
//pub struct MagicBitboard;
//
//static MAGIC_BITBOARD: LazyLock<MagicBitboard> = LazyLock::new(|| MagicBitboard::default());
//
//fn gen_rook_moves(move_gen: &impl GenerateSlidingMoves) -> [BitBoard; 64 * 2_usize.pow(14)] {
//    let mut rook_moves = [BitBoard::empty(); 64 * 2_usize.pow(14)];
//
//    for sq in Square::iter() {
//        let blocker_boards = gen_blocker_boards(sq);
//        for (board_idx, &board) in blocker_boards.iter().enumerate() {
//            let moves = move_gen.gen_moves(Piece::Rook, sq, board);
//            let move_idx = (sq as usize) * 2_usize.pow(14) + board_idx;
//            rook_moves[move_idx] = moves;
//        }
//    }
//    rook_moves
//}
//
//struct MagicEntry {
//    mask: BitBoard,
//    magic: u64,
//    shift: u8,
//    offset: u32,
//}
//
//// Magics from Ethereal: https://github.com/AndyGrant/Ethereal
//
//#[rustfmt::skip]
//const ROOK_MAGICS: [MagicEntry; 64] = [
//    0xA180022080400230, 0x0040100040022000, 0x0080088020001002, 0x0080080280841000,
//    0x4200042010460008, 0x04800A0003040080, 0x0400110082041008, 0x008000A041000880,
//    0x10138001A080C010, 0x0000804008200480, 0x00010011012000C0, 0x0022004128102200,
//    0x000200081201200C, 0x202A001048460004, 0x0081000100420004, 0x4000800380004500,
//    0x0000208002904001, 0x0090004040026008, 0x0208808010002001, 0x2002020020704940,
//    0x8048010008110005, 0x6820808004002200, 0x0A80040008023011, 0x00B1460000811044,
//    0x4204400080008EA0, 0xB002400180200184, 0x2020200080100380, 0x0010080080100080,
//    0x2204080080800400, 0x0000A40080360080, 0x02040604002810B1, 0x008C218600004104,
//    0x8180004000402000, 0x488C402000401001, 0x4018A00080801004, 0x1230002105001008,
//    0x8904800800800400, 0x0042000C42003810, 0x008408110400B012, 0x0018086182000401,
//    0x2240088020C28000, 0x001001201040C004, 0x0A02008010420020, 0x0010003009010060,
//    0x0004008008008014, 0x0080020004008080, 0x0282020001008080, 0x50000181204A0004,
//    0x48FFFE99FECFAA00, 0x48FFFE99FECFAA00, 0x497FFFADFF9C2E00, 0x613FFFDDFFCE9200,
//    0xFFFFFFE9FFE7CE00, 0xFFFFFFF5FFF3E600, 0x0010301802830400, 0x510FFFF5F63C96A0,
//    0xEBFFFFB9FF9FC526, 0x61FFFEDDFEEDAEAE, 0x53BFFFEDFFDEB1A2, 0x127FFFB9FFDFB5F6,
//    0x411FFFDDFFDBF4D6, 0x0801000804000603, 0x0003FFEF27EEBE74, 0x7645FFFECBFEA79E,
//];
//
//#[rustfmt::skip]
//const BISHOP_MAGICS: [MagicEntry; 64] = [
//    0xFFEDF9FD7CFCFFFF, 0xFC0962854A77F576, 0x5822022042000000, 0x2CA804A100200020,
//    0x0204042200000900, 0x2002121024000002, 0xFC0A66C64A7EF576, 0x7FFDFDFCBD79FFFF,
//    0xFC0846A64A34FFF6, 0xFC087A874A3CF7F6, 0x1001080204002100, 0x1810080489021800,
//    0x0062040420010A00, 0x5028043004300020, 0xFC0864AE59B4FF76, 0x3C0860AF4B35FF76,
//    0x73C01AF56CF4CFFB, 0x41A01CFAD64AAFFC, 0x040C0422080A0598, 0x4228020082004050,
//    0x0200800400E00100, 0x020B001230021040, 0x7C0C028F5B34FF76, 0xFC0A028E5AB4DF76,
//    0x0020208050A42180, 0x001004804B280200, 0x2048020024040010, 0x0102C04004010200,
//    0x020408204C002010, 0x02411100020080C1, 0x102A008084042100, 0x0941030000A09846,
//    0x0244100800400200, 0x4000901010080696, 0x0000280404180020, 0x0800042008240100,
//    0x0220008400088020, 0x04020182000904C9, 0x0023010400020600, 0x0041040020110302,
//    0xDCEFD9B54BFCC09F, 0xF95FFA765AFD602B, 0x1401210240484800, 0x0022244208010080,
//    0x1105040104000210, 0x2040088800C40081, 0x43FF9A5CF4CA0C01, 0x4BFFCD8E7C587601,
//    0xFC0FF2865334F576, 0xFC0BF6CE5924F576, 0x80000B0401040402, 0x0020004821880A00,
//    0x8200002022440100, 0x0009431801010068, 0xC3FFB7DC36CA8C89, 0xC3FF8A54F4CA2C89,
//    0xFFFFFCFCFD79EDFF, 0xFC0863FCCB147576, 0x040C000022013020, 0x2000104000420600,
//    0x0400000260142410, 0x0800633408100500, 0xFC087E8E4BB2F736, 0x43FF9E4EF4CA2C89,
//];
//
//const ROOK_MOVES: &[BitBoard] = todo!();
//const BISHOP_MOVES: &[BitBoard] = todo!();
//
//const fn magic_index(entry: &MagicEntry, blockers: BitBoard) -> usize {
//    let blockers = blockers & entry.mask;
//    let hash = blockers.to_val().wrapping_mul(entry.magic);
//    let index = (hash >> entry.shift) as usize;
//    entry.offset as usize + index
//}
//
//const fn get_rook_moves(square: Square, blockers: BitBoard) -> BitBoard {
//    let magic = &ROOK_MAGICS[square as usize];
//    ROOK_MOVES[magic_index(magic, blockers)]
//}
//
//const fn get_bishop_moves(square: Square, blockers: BitBoard) -> BitBoard {
//    let magic = &BISHOP_MAGICS[square as usize];
//    BISHOP_MOVES[magic_index(magic, blockers)]
//}
//
//impl GenerateSlidingMoves for MagicBitboard {
//    fn gen_moves(&self, piece: Piece, square: Square, occupancy: BitBoard) -> BitBoard {
//        BitBoard::empty()
//    }
//}
