#![feature(variant_count)]

use ascii_view::*;
use bitboard::*;
use board::*;
use move_generator::generate_moves;
use squares::*;

mod ascii_view;
mod attacks;
mod bitboard;
mod board;
mod codegen;
mod fens;
mod move_generator;
mod rand;
mod squares;

fn main() {
    let board = Board::from_fen(fens::STARTING_BOARD_FEN);

    let moves = generate_moves(&board);
    println!("{}", moves.len());
}
