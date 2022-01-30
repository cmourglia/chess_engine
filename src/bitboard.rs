#![allow(dead_code)]

use crate::codegen;

pub const NOT_A_FILE: u64 = codegen::not_a_file();
pub const NOT_H_FILE: u64 = codegen::not_h_file();
pub const NOT_AB_FILE: u64 = codegen::not_ab_file();
pub const NOT_GH_FILE: u64 = codegen::not_gh_file();

pub const fn bitboard_from_square(square: i32) -> u64 {
    if let Some(v) = (1 as u64).checked_shl(square as u32) {
        v
    } else {
        0
    }
}

pub fn make_blocker(squares: Vec<i32>) -> u64 {
    let mut result = 0u64;

    for square in squares {
        result |= bitboard_from_square(square)
    }

    result
}

pub const fn get_bit(bitboard: u64, square: i32) -> bool {
    bitboard & bitboard_from_square(square) != 0
}

pub const fn set_bit(bitboard: u64, square: i32) -> u64 {
    bitboard | bitboard_from_square(square)
}

pub const fn toggle_bit(bitboard: u64, square: i32) -> u64 {
    bitboard ^ bitboard_from_square(square)
}

pub const fn pop_bit(bitboard: u64, square: i32) -> u64 {
    bitboard & !bitboard_from_square(square)
}

pub const fn bit_count(bitboard: u64) -> usize {
    let mut bits = bitboard;
    let mut bit_counter = 0;
    while bits != 0 {
        bits &= bits - 1;
        bit_counter += 1;
    }
    bit_counter
}

pub const fn bits_collide(bitboard_a: u64, bitboard_b: u64) -> bool {
    bitboard_a & bitboard_b != 0
}

pub const fn least_significant_bit_index(bitboard: u64) -> usize {
    if bitboard == 0 {
        usize::MAX
    } else {
        let bits = bitboard as i64;
        let bits = bits & bits.wrapping_neg();
        bit_count(bits as u64 - 1)
    }
}
