use crate::bitboard::*;
use crate::board::*;
use crate::move_generator::*;
use crate::squares::*;

const PIECE_TABLE: [char; 12] = ['♙', '♘', '♗', '♖', '♕', '♔', '♟', '♞', '♝', '♜', '♛', '♚'];

pub fn print_bitboard(bitboard: u64) {
    println!("\n  -*- {:#016X} -*-", bitboard);

    println!("      ---------------");
    for rank in 0..8 {
        print!(" {}  |", 8 - rank);
        for file in 0..8 {
            let square = rank * 8 + file;

            print!(
                " {}",
                if get_bit(bitboard, square) {
                    '×'
                } else {
                    '·'
                }
            );
        }
        println!(" |");
    }

    println!("      ---------------");
    println!("      a b c d e f g h\n");
}

pub fn print_board(board: &Board) {
    println!("      ---------------");
    for rank in 0..8 {
        print!(" {}  |", 8 - rank);
        for file in 0..8 {
            let square = rank * 8 + file;

            let mut piece = -1i32;
            for (i, p) in board.pieces.iter().enumerate() {
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
        board.side_to_move,
        board.castling_rights,
        if board.en_passant_square == NO_SQUARE {
            "ø"
        } else {
            CELL_NAMES[board.en_passant_square as usize]
        }
    );
}

pub fn print_attacked_squares(board: &Board, side: Side) {
    println!("      ---------------");
    for rank in 0..8 {
        print!(" {}  |", 8 - rank);
        for file in 0..8 {
            let square = rank * 8 + file;

            print!(
                " {}",
                if is_square_attacked(board, square, side) {
                    '×'
                } else {
                    '·'
                }
            );
        }
        println!(" |");
    }

    println!("      ---------------");
    println!("      a b c d e f g h\n");
}

pub fn print_move(mv: i32) {
    println!(
        "{}{} {} (capture: {}, en-passant: {}, castles: {}, double push: {}, promotion: {} ({}))",
        CELL_NAMES[Move::decode_src_square(mv) as usize],
        CELL_NAMES[Move::decode_dst_square(mv) as usize],
        PIECE_TABLE[Move::decode_piece(mv) as usize],
        Move::is_capture(mv),
        Move::is_en_passant(mv),
        Move::is_castling(mv),
        Move::is_double_push(mv),
        Move::is_promotion(mv),
        PIECE_TABLE[Move::decode_promotion_piece(mv) as usize],
    );
}
