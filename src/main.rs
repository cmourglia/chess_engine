#![feature(variant_count)]

use std::time::Instant;

use ascii_view::*;
use attacks::*;
use board::*;
use move_generator::generate_moves;

mod ascii_view;
mod attacks;
mod bitboard;
mod board;
mod codegen;
mod fens;
mod move_generator;
mod rand;
mod squares;

fn run_depth(board: &Board, depth: i32, results: &mut [usize; 10]) {
    if depth == 0 {
        return;
    }
    // This will be easier to play moves from multiple threads
    let mut my_board = board.clone();

    let moves = generate_moves(&my_board);

    results[(depth - 1) as usize] += moves.len();

    for mv in moves {
        let old_board = my_board.play_move(mv);
        run_depth(&my_board, depth - 1, results);
        my_board = old_board
    }
}

fn main() {
    let attacks = Attacks::new();
    let board = Board::from_fen(fens::STARTING_BOARD_FEN, &attacks);

    let mut result = [0usize; 10];
    let start_time = Instant::now();
    run_depth(&board, 5, &mut result);
    let dt = start_time.elapsed();

    println!("{:?} - {}", result, dt.as_millis());
}
