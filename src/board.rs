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
    pub fn from_fen(fen: &str) -> Self {
        let mut pieces = [0u64; 12];

        let mut fen_iter = fen.split(' ');
        let position = fen_iter.next().unwrap_or("");

        let position_iter = position.split('/');
        for (rank, line) in position_iter.enumerate() {
            let mut file = 0;
            for c in line.chars() {
                if let Some(piece) = ASCII_TO_PIECE.get(&c) {
                    let piece_idx = *piece as usize;
                    pieces[piece_idx] = set_bit(pieces[piece_idx], get_square(rank as i32, file));
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
                _ => unreachable!(),
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
        }
    }

    pub fn new() -> Self {
        Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
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
