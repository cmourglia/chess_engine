use std::fmt::Display;

use crate::bitboard::*;
use crate::board::*;
use crate::squares;

#[derive(Clone, Copy, Debug)]
pub struct Move {
    source_square: i32,
    target_square: i32,
    is_capture: bool,
    is_castle: bool,
    is_en_passant: bool,
    is_promotion: bool,
}

impl Move {
    fn simple(source_square: i32, target_square: i32) -> Self {
        Self {
            source_square,
            target_square,
            is_capture: false,
            is_castle: false,
            is_en_passant: false,
            is_promotion: false,
        }
    }

    fn capture(source_square: i32, target_square: i32) -> Self {
        Self {
            source_square,
            target_square,
            is_capture: true,
            is_castle: false,
            is_en_passant: false,
            is_promotion: false,
        }
    }

    fn castle(source_square: i32, target_square: i32) -> Self {
        Self {
            source_square,
            target_square,
            is_capture: false,
            is_castle: true,
            is_en_passant: false,
            is_promotion: false,
        }
    }

    fn en_passant(source_square: i32, target_square: i32) -> Self {
        Self {
            source_square,
            target_square,
            is_capture: true,
            is_castle: false,
            is_en_passant: true,
            is_promotion: false,
        }
    }

    fn promotion(source_square: i32, target_square: i32, is_capture: bool) -> Self {
        Self {
            source_square,
            target_square,
            is_capture,
            is_castle: false,
            is_en_passant: false,
            is_promotion: true,
        }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            squares::CELL_NAMES[self.source_square as usize],
            squares::CELL_NAMES[self.target_square as usize],
        )
    }
}

/// Check whether the given square is under attack.
/// In order to do this,instead of checking if any of the pieces is attacking the square,
/// we check if the square attacks any of the pieces as this piece.
/// e.g., if the e5 square is attacked by a black pawn on d6, it also means that,
/// as a white pawn, the square attacks d6, which is easier to check.
/// This also means we only do 6 attack lookups and 6 bitwise & instead of 16
/// (plus the need to find the pieces inside the pieces bitboards)
pub fn is_square_attacked(board: &Board, square: i32, attacking_side: Side) -> bool {
    let occupancy = board.occupancies[Side::Both as usize];

    let this_side = match attacking_side {
        Side::Black => Side::White,
        Side::White => Side::Black,
        Side::Both => unreachable!(),
    };

    let as_pawn = board.attacks.get_pawn_attacks(square, this_side);
    if bits_collide(as_pawn, board.bitboard(Piece::Pawn, attacking_side)) {
        return true;
    }

    let as_knight = board.attacks.get_knight_attacks(square);
    if bits_collide(as_knight, board.bitboard(Piece::Knight, attacking_side)) {
        return true;
    }

    let as_king = board.attacks.get_king_attacks(square);
    if bits_collide(as_king, board.bitboard(Piece::King, attacking_side)) {
        return true;
    }

    let as_bishop = board.attacks.get_bishop_attacks(square, occupancy);
    if bits_collide(as_bishop, board.bitboard(Piece::Bishop, attacking_side)) {
        return true;
    }

    let as_rook = board.attacks.get_rook_attacks(square, occupancy);
    if bits_collide(as_rook, board.bitboard(Piece::Rook, attacking_side)) {
        return true;
    }

    let as_queen = board.attacks.get_queen_attacks(square, occupancy);
    if bits_collide(as_queen, board.bitboard(Piece::Queen, attacking_side)) {
        return true;
    }

    false
}

