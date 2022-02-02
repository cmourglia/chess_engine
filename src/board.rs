use num_enum::FromPrimitive;
use phf::phf_map;

use crate::attacks::Attacks;
use crate::bitboard::*;
use crate::codegen::get_square;
use crate::move_generator::*;
use crate::squares::*;

#[derive(Clone, Copy, Eq, PartialEq, FromPrimitive)]
#[repr(i32)]
pub enum Piece {
    #[num_enum(default)]
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy)]
pub enum SidedPiece {
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,
}

#[derive(Debug, Clone, Copy)]
pub enum Side {
    White,
    Black,
    Both,
}

pub enum Castling {
    WhiteKing = 1 << 0,
    WhiteQueen = 1 << 1,
    BlackKing = 1 << 2,
    BlackQueen = 1 << 3,
}

/// Map each ASCII character with a piece type
/// This is especially useful for FEN parsing
const ASCII_TO_PIECE: phf::Map<char, SidedPiece> = phf_map! {
    'P' => SidedPiece::WhitePawn,
    'N' => SidedPiece::WhiteKnight,
    'B' => SidedPiece::WhiteBishop,
    'R' => SidedPiece::WhiteRook,
    'Q' => SidedPiece::WhiteQueen,
    'K' => SidedPiece::WhiteKing,
    'p' => SidedPiece::BlackPawn,
    'n' => SidedPiece::BlackKnight,
    'b' => SidedPiece::BlackBishop,
    'r' => SidedPiece::BlackRook,
    'q' => SidedPiece::BlackQueen,
    'k' => SidedPiece::BlackKing,
};

pub fn opponent_side(side: Side) -> Side {
    match side {
        Side::White => Side::Black,
        Side::Black => Side::White,
        Side::Both => unreachable!(),
    }
}

#[derive(Clone, Copy)]
pub struct Board<'a> {
    /// One bitboard per piece type, which keeps track of every piece
    /// of this given type.
    /// e.g. white knights starting bitboard is given by B1 | G1.
    /// black rooks starting bitboard is A8 | H8, and so on.
    pub pieces: [u64; 12],

    /// Occupancies, index by Side enum
    /// 0: White pieces occupancy table
    /// 1: Black pieces occupancy table
    /// 2: All pieces occupancy table
    pub occupancies: [u64; 3],

    pub side_to_move: Side,

    /// A pawn has just made a two-square move, an en-passant square is then made available.
    /// Otherwise, this is set to NO_SQUARE(-1)
    pub en_passant_square: i32,

    /// Bitmask of available castlings (@Castling)
    /// 0001 -> White king-side castling
    /// 0010 -> White queen-side castling
    /// 0100 -> Black king-side castling
    /// 1000 -> Black queen-side castling.
    pub castling_rights: u8,

    /// Attack maps
    pub attacks: &'a Attacks,
}

impl<'a> Board<'a> {
    pub fn from_fen(fen: &str, attacks: &'a Attacks) -> Self {
        let mut pieces = [0u64; 12];

        let mut fen_iter = fen.split(' ');
        let position = fen_iter.next().unwrap_or("");

        let position_iter = position.split('/');
        for (rank, line) in position_iter.enumerate() {
            let mut file = 0;
            for c in line.chars() {
                if let Some(piece) = ASCII_TO_PIECE.get(&c) {
                    let piece_idx = *piece as usize;
                    pieces[piece_idx] |= bitboard_from_square(get_square(rank as i32, file));
                    file += 1;
                } else {
                    if c.is_numeric() {
                        file += (c as u8 - '0' as u8) as i32;
                    } else {
                        unreachable!();
                    }
                }
            }
        }

        let occupancies = [
            Self::get_occupancy(&pieces, Side::White),
            Self::get_occupancy(&pieces, Side::Black),
            Self::get_occupancy(&pieces, Side::Both),
        ];

        let side_to_move = match fen_iter.next().unwrap() {
            "w" => Side::White,
            "b" => Side::Black,
            _ => unreachable!(),
        };

        let castling_str = fen_iter.next().unwrap();
        let mut castling_rights = 0u8;
        for c in castling_str.chars() {
            match c {
                'K' => castling_rights |= Castling::WhiteKing as u8,
                'Q' => castling_rights |= Castling::WhiteQueen as u8,
                'k' => castling_rights |= Castling::BlackKing as u8,
                'q' => castling_rights |= Castling::BlackQueen as u8,
                _ => {}
            }
        }

        let mut en_passant_square = NO_SQUARE;
        let en_passant_str = fen_iter.next().unwrap();
        if let Some(en_passant) = CELL_TO_SQUARE.get(en_passant_str) {
            en_passant_square = *en_passant;
        }

        // TODO: Handle 50 moves rule parsing

        Self {
            pieces,
            occupancies,
            side_to_move,
            en_passant_square,
            castling_rights,
            attacks,
        }
    }

    pub fn bitboard(&self, piece: Piece, side: Side) -> u64 {
        let index = piece as usize + side as usize * std::mem::variant_count::<Piece>();
        self.pieces[index]
    }

    pub fn mut_bitboard(&mut self, piece: Piece, side: Side) -> &mut u64 {
        let index = piece as usize + side as usize * std::mem::variant_count::<Piece>();
        &mut self.pieces[index]
    }

    pub fn get_occupancy(pieces: &[u64; 12], side: Side) -> u64 {
        let mut result = 0u64;

        let indices = match side {
            Side::White => 0..6,
            Side::Black => 6..12,
            Side::Both => 0..12,
        };

        for i in indices {
            result |= pieces[i]
        }

        result
    }

    pub fn play_move(&mut self, mv: i32) -> Self {
        // TODO: Check the perf of this call.
        let current_state = self.clone();

        // Reset state
        self.en_passant_square = NO_SQUARE;

        let piece = Move::decode_piece(mv);

        let src_square = Move::decode_src_square(mv);
        let src_bitbard = bitboard_from_square(src_square);

        let dst_square = Move::decode_dst_square(mv);

        let mut bitboard = self.mut_bitboard(piece, self.side_to_move);
        Board::apply_move_to_bitboard(bitboard, src_square, dst_square);

        let mut my_occupancy = &mut self.occupancies[self.side_to_move as usize];
        Board::apply_move_to_bitboard(my_occupancy, src_square, dst_square);

        let mut all_occupancy = &mut self.occupancies[Side::Both as usize];
        Board::apply_move_to_bitboard(all_occupancy, src_square, dst_square);

        if Move::is_double_push(mv) {
            self.en_passant_square = match self.side_to_move {
                Side::White => dst_square + 8,
                Side::Black => dst_square - 8,
                Side::Both => unreachable!(),
            };
        }

        // Castling rights
        // Is it a rook move ?

        // Is it a king move ?

        self.side_to_move = opponent_side(self.side_to_move);

        current_state
    }

    fn apply_move_to_bitboard(bitboard: &mut u64, src_square: i32, dst_square: i32) {
        *bitboard = pop_bit(*bitboard, src_square);
        *bitboard |= bitboard_from_square(dst_square);
    }
}
