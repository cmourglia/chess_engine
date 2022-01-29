#![feature(variant_count)]
mod ascii_view;
mod attacks;
mod bitboard;
mod board;
mod codegen;
mod rand;
mod squares;

const EMPTY_BOARD_FEN: &str = "8/8/8/8/8/8/8/8 w - - -";
const STARTING_BOARD_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const SCOTCH_GAME_FEN: &str = "r1bqkbnr/pppp1ppp/2n5/4p3/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq d3 0 3";

fn main() {
    //find_magic_number(e4, relevant_bits, is_bishop)
    let attacks = crate::attacks::Attacks::new();

    println!("{:?}", attacks);
}
