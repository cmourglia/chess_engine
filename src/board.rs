use phf::phf_map;

use crate::bitboard::{as_bitboard, get_bit, set_bit};
use crate::codegen::get_square;
use crate::squares::*;

#[derive(Clone, Copy)]
enum Piece {
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

#[derive(Debug)]
pub enum Side {
    White,
    Black,
    Both,
}

enum Castling {
    WhiteKing = 1 << 0,
    WhiteQueen = 1 << 1,
    BlackKing = 1 << 2,
    BlackQueen = 1 << 3,
}

const PIECE_TABLE: [char; 12] = ['♙', '♘', '♗', '♖', '♕', '♔', '♟', '♞', '♝', '♜', '♛', '♚'];

/// Map each ASCII character with a piece type
/// This is especially useful for FEN parsing
const ASCII_TO_PIECE: phf::Map<char, Piece> = phf_map! {
    'P' => Piece::WhitePawn,
    'N' => Piece::WhiteKnight,
    'B' => Piece::WhiteBishop,
    'R' => Piece::WhiteRook,
    'Q' => Piece::WhiteQueen,
    'K' => Piece::WhiteKing,
    'p' => Piece::BlackPawn,
    'n' => Piece::BlackKnight,
    'b' => Piece::BlackBishop,
    'r' => Piece::BlackRook,
    'q' => Piece::BlackQueen,
    'k' => Piece::BlackKing,
};

pub struct Board {
    /// One bitboard per piece type, which keeps track of every piece
    /// of this given type.
    /// e.g. white knights starting bitboard is given by B1 | G1.
    /// black rooks starting bitboard is A8 | H8, and so on.
    pieces: [u64; 12],

    /// Occupancies, index by Side enum
    /// 0: White pieces occupancy table
    /// 1: Black pieces occupancy table
    /// 2: All pieces occupancy table
    occupancies: [u64; 3],

    side_to_move: Side,

    /// A pawn has just made a two-square move, an en-passant square is then made available.
    /// Otherwise, this is set to NO_SQUARE(-1)
    en_passant_square: i32,

    /// Bitmask of available castlings (@Castling)
    /// 0001 -> White king-side castling
    /// 0010 -> White queen-side castling
    /// 0100 -> Black king-side castling
    /// 1000 -> Black queen-side castling.
    castling_rights: u8,
}

impl Board {
    pub fn new() -> Self {
        // TODO:
        // Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
        let white_pawns = 0u64
            | as_bitboard(A2)
            | as_bitboard(B2)
            | as_bitboard(C2)
            | as_bitboard(D2)
            | as_bitboard(E2)
            | as_bitboard(F2)
            | as_bitboard(G2)
            | as_bitboard(H2);

        let white_knights = 0u64 | as_bitboard(B1) | as_bitboard(G1);
        let white_bishops = 0u64 | as_bitboard(C1) | as_bitboard(F1);
        let white_rooks = 0u64 | as_bitboard(A1) | as_bitboard(H1);
        let white_queens = 0u64 | as_bitboard(D1);
        let white_king = 0u64 | as_bitboard(E1);

        let black_pawns = 0u64
            | as_bitboard(A7)
            | as_bitboard(B7)
            | as_bitboard(C7)
            | as_bitboard(D7)
            | as_bitboard(E7)
            | as_bitboard(F7)
            | as_bitboard(G7)
            | as_bitboard(H7);

        let black_knights = 0u64 | as_bitboard(B8) | as_bitboard(G8);
        let black_bishops = 0u64 | as_bitboard(C8) | as_bitboard(F8);
        let black_rooks = 0u64 | as_bitboard(A8) | as_bitboard(H8);
        let black_queens = 0u64 | as_bitboard(D8);
        let black_king = 0u64 | as_bitboard(E8);

        let pieces = [
            white_pawns,
            white_knights,
            white_bishops,
            white_rooks,
            white_queens,
            white_king,
            black_pawns,
            black_knights,
            black_bishops,
            black_rooks,
            black_queens,
            black_king,
        ];

        let occupancies = [
            Self::get_occupancy(&pieces, Side::White),
            Self::get_occupancy(&pieces, Side::Black),
            Self::get_occupancy(&pieces, Side::Both),
        ];

        Self {
            pieces,
            occupancies,
            side_to_move: Side::White,
            en_passant_square: NO_SQUARE,
            castling_rights: 15u8,
        }
    }

    pub fn print(&self) {
        println!("      ---------------");
        for rank in 0..8 {
            print!(" {}  |", 8 - rank);
            for file in 0..8 {
                let square = rank * 8 + file;

                let mut piece = -1i32;
                for (i, p) in self.pieces.iter().enumerate() {
                    if get_bit(*p, square) {
                        piece = i as i32;
                        break;
                    }
                }

                print!(
                    " {}",
                    if piece == -1 {
                        '·'
                    } else {
                        PIECE_TABLE[piece as usize]
                    }
                );
            }
            println!(" |");
        }

        println!("      ---------------");
        println!("      a b c d e f g h\n");

        println!(
            "Side to move: {:?} \nCastling: {:04b}\nEn-passant: {}\n",
            self.side_to_move,
            self.castling_rights,
            if self.en_passant_square == NO_SQUARE {
                "ø"
            } else {
                CELL_NAMES[self.en_passant_square as usize]
            }
        );
    }

    fn get_occupancy(pieces: &[u64; 12], side: Side) -> u64 {
        let indices = {
            match side {
                Side::White => 0..6,
                Side::Black => 6..12,
                Side::Both => 0..12,
            }
        };

        let mut result = 0u64;

        for i in indices {
            result |= pieces[i]
        }

        result
    }
}
