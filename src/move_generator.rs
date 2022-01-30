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

pub fn generate_moves(board: &Board) -> Vec<Move> {
    let mut moves = vec![];

    match board.side_to_move {
        Side::White => {
            moves.append(&mut generate_pawn_moves(board, Side::White));
        }
        Side::Black => {
            moves.append(&mut generate_pawn_moves(board, Side::Black));
        }
        Side::Both => unreachable!(),
    }

    moves
}

fn generate_pawn_moves(board: &Board, side: Side) -> Vec<Move> {
    // Cache relevant data
    let all_occupancies = board.occupancies[Side::Both as usize];
    let my_occupancies = board.occupancies[side as usize];
    let opp_occupancies = board.occupancies[opponent_side(side) as usize];
    let en_passant_square = board.en_passant_square;

    // start_rank: This side's pawns start rank.
    //   This is given by the result of the integer division of the
    //   square index by 8.
    //   0 corresponds to rank 8,
    //   1 corresponds to rank 7 (black's start rank)
    //   ...
    //   6 corresponds to rank 2 (white's start rank)
    // back_rank: The opponents "back rank". This is used to detect promotion
    let (start_rank, back_rank) = match side {
        Side::White => (6, 0),
        Side::Black => (1, 7),
        Side::Both => unreachable!(),
    };

    let (one_square, two_squares) = match side {
        Side::White => (-8, -16),
        Side::Black => (8, 16),
        Side::Both => unreachable!(),
    };

    let mut bitboard = board.bitboard(Piece::Pawn, side);
    let mut moves = vec![];

    while bitboard != 0 {
        let source_square = least_significant_bit_index(bitboard) as i32;
        bitboard = pop_bit(bitboard, source_square);

        let target_square = source_square + one_square;
        if bitboard_from_square(target_square) & all_occupancies == 0 {
            let rank = source_square / 8;

            if rank == back_rank {
                moves.push(Move::promotion(source_square, target_square, false));
            } else {
                moves.push(Move::simple(source_square, target_square));
            }

            // The two squares move is only relevant if there is already no
            // blocker for the one square move.
            // We also need to make sure we are on the start rank.
            if rank == start_rank {
                let target_square = source_square + two_squares;
                if bitboard_from_square(target_square) & all_occupancies == 0 {
                    moves.push(Move::simple(source_square, target_square));
                }
            }
        }

        let mut attacks = board.attacks.get_pawn_attacks(source_square, side);
        while attacks != 0 {
            let target_square = least_significant_bit_index(attacks) as i32;
            attacks = pop_bit(attacks, target_square);

            if bitboard_from_square(target_square) & opp_occupancies == 1 {
                if source_square / 8 == back_rank {
                    moves.push(Move::promotion(source_square, target_square, true));
                } else {
                    moves.push(Move::capture(source_square, target_square));
                }
            } else if en_passant_square == target_square {
                // This test would be relevant only on 4th and 5th ranks, but it might
                // be more costly to perform a division than just test directly
                // with data already loaded...
                moves.push(Move::en_passant(source_square, target_square));
            }
        }
    }

    moves
}
