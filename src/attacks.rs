#![allow(dead_code)]

use std::time::{Duration, Instant};

use crate::bitboard::*;
use crate::codegen::get_square;
use crate::Side;

fn mask_pawn_attacks(square: i32, side: Side) -> u64 {
    let mut attacks = 0u64;
    let bitboard = as_bitboard(square);

    match side {
        Side::White => {
            if bitboard & NOT_A_FILE != 0 {
                attacks = set_bit(attacks, square - 9);
            }

            if bitboard & NOT_H_FILE != 0 {
                attacks = set_bit(attacks, square - 7);
            }
        }
        Side::Black => {
            if bitboard & NOT_A_FILE != 0 {
                attacks = set_bit(attacks, square + 7);
            }

            if bitboard & NOT_H_FILE != 0 {
                attacks = set_bit(attacks, square + 9);
            }
        }
    }

    attacks
}

fn mask_knight_attacks(square: i32) -> u64 {
    let mut attacks = 0u64;
    let bitboard = as_bitboard(square);

    if bitboard & NOT_A_FILE != 0 {
        attacks = set_bit(attacks, square - 17);
        attacks = set_bit(attacks, square + 15);
    }
    if bitboard & NOT_AB_FILE != 0 {
        attacks = set_bit(attacks, square - 10);
        attacks = set_bit(attacks, square + 6);
    }
    if bitboard & NOT_GH_FILE != 0 {
        attacks = set_bit(attacks, square - 6);
        attacks = set_bit(attacks, square + 10);
    }
    if bitboard & NOT_H_FILE != 0 {
        attacks = set_bit(attacks, square - 15);
        attacks = set_bit(attacks, square + 17);
    }

    attacks
}

fn mask_king_attacks(square: i32) -> u64 {
    let mut attacks = 0u64;
    let bitboard = as_bitboard(square);

    attacks = set_bit(attacks, square - 8);
    attacks = set_bit(attacks, square + 8);
    if bitboard & NOT_A_FILE != 0 {
        attacks = set_bit(attacks, square - 9);
        attacks = set_bit(attacks, square - 1);
        attacks = set_bit(attacks, square + 7);
    }
    if bitboard & NOT_H_FILE != 0 {
        attacks = set_bit(attacks, square + 9);
        attacks = set_bit(attacks, square + 1);
        attacks = set_bit(attacks, square - 7);
    }

    attacks
}

pub fn mask_bishop_attacks(square: i32) -> u64 {
    let mut attacks = 0u64;

    let rank = square / 8;
    let file = square % 8;

    for (i, j) in (1..rank).rev().zip((1..file).rev()) {
        attacks = set_bit(attacks, get_square(i, j));
    }

    for (i, j) in (1..rank).rev().zip(file + 1..7) {
        attacks = set_bit(attacks, get_square(i, j));
    }

    for (i, j) in (rank + 1..7).zip((1..file).rev()) {
        attacks = set_bit(attacks, get_square(i, j));
    }

    for (i, j) in (rank + 1..7).zip(file + 1..7) {
        attacks = set_bit(attacks, get_square(i, j));
    }

    attacks
}

fn mask_rook_attacks(square: i32) -> u64 {
    let mut attacks = 0u64;

    let rank = square / 8;
    let file = square % 8;

    for i in (1..rank).rev() {
        attacks = set_bit(attacks, get_square(i, file));
    }

    for i in (1..file).rev() {
        attacks = set_bit(attacks, get_square(rank, i));
    }

    for i in rank + 1..7 {
        attacks = set_bit(attacks, get_square(i, file));
    }

    for i in file + 1..7 {
        attacks = set_bit(attacks, get_square(rank, i));
    }

    attacks
}

fn bishop_attacks_on_the_fly(square: i32, blocker: u64) -> u64 {
    let mut attacks = 0u64;

    let rank = square / 8;
    let file = square % 8;

    for (i, j) in (0..rank).rev().zip((0..file).rev()) {
        let curr_square = get_square(i, j);
        attacks = set_bit(attacks, curr_square);
        if get_bit(blocker, curr_square) {
            break;
        }
    }

    for (i, j) in (0..rank).rev().zip(file + 1..8) {
        let curr_square = get_square(i, j);
        attacks = set_bit(attacks, curr_square);
        if get_bit(blocker, curr_square) {
            break;
        }
    }

    for (i, j) in (rank + 1..8).zip((0..file).rev()) {
        let curr_square = get_square(i, j);
        attacks = set_bit(attacks, curr_square);
        if get_bit(blocker, curr_square) {
            break;
        }
    }

    for (i, j) in (rank + 1..8).zip(file + 1..8) {
        let curr_square = get_square(i, j);
        attacks = set_bit(attacks, curr_square);
        if get_bit(blocker, curr_square) {
            break;
        }
    }

    attacks
}

fn rook_attacks_on_the_fly(square: i32, blocker: u64) -> u64 {
    let mut attacks = 0u64;

    let rank = square / 8;
    let file = square % 8;

    for i in (0..rank).rev() {
        let curr_square = get_square(i, file);
        attacks = set_bit(attacks, curr_square);
        if get_bit(blocker, curr_square) {
            break;
        }
    }

    for i in (0..file).rev() {
        let curr_square = get_square(rank, i);
        attacks = set_bit(attacks, curr_square);
        if get_bit(blocker, curr_square) {
            break;
        }
    }

    for i in rank + 1..8 {
        let curr_square = get_square(i, file);
        attacks = set_bit(attacks, curr_square);
        if get_bit(blocker, curr_square) {
            break;
        }
    }

    for i in file + 1..8 {
        let curr_square = get_square(rank, i);
        attacks = set_bit(attacks, curr_square);
        if get_bit(blocker, curr_square) {
            break;
        }
    }

    attacks
}

fn generate_pawn_attacks() -> [[u64; 64]; 2] {
    let mut result = [[0u64; 64]; 2];

    for square in 0..64 {
        result[0][square] = mask_pawn_attacks(square as i32, Side::White);
        result[1][square] = mask_pawn_attacks(square as i32, Side::Black);
    }

    result
}

fn generate_knight_attacks() -> [u64; 64] {
    let mut result = [0u64; 64];

    for square in 0..64 {
        result[square] = mask_knight_attacks(square as i32);
    }

    result
}

fn generate_king_attacks() -> [u64; 64] {
    let mut result = [0u64; 64];

    for square in 0..64 {
        result[square] = mask_king_attacks(square as i32);
    }

    result
}

fn generate_bishop_occupancies() -> [usize; 64] {
    let mut result = [0usize; 64];

    for square in 0..64 {
        result[square] = bit_count(mask_bishop_attacks(square as i32));
    }

    result
}

fn generate_rook_occupancies() -> [usize; 64] {
    let mut result = [0usize; 64];

    for square in 0..64 {
        result[square] = bit_count(mask_rook_attacks(square as i32));
    }

    result
}

fn set_occupancy(index: usize, bits_in_mask: usize, attack_mask: u64) -> u64 {
    let mut occupancy = 0u64;
    let mut mask = attack_mask;

    for count in 0..bits_in_mask {
        let square = least_significant_bit_index(mask) as i32;
        mask = pop_bit(mask, square);

        if index & (1 << count) != 0 {
            occupancy = set_bit(occupancy, square);
        }
    }

    occupancy
}

fn find_magic_number(
    random_state: u32,
    square: i32,
    relevant_bits: usize,
    is_bishop: bool,
) -> (u64, u32) {
    use crate::rand::next_magic_candidate;

    // FIXME: I do not like that
    let mut occupancies = [0u64; 4096];
    let mut attacks = [0u64; 4096];
    let mut used_attacks: [u64; 4096]; // [0u64; 4096];
    let mut last_rand = random_state;

    let attack_mask = if is_bishop {
        mask_bishop_attacks(square)
    } else {
        mask_rook_attacks(square)
    };

    let occupancy_indices = 1 << relevant_bits;

    for index in 0..occupancy_indices {
        occupancies[index] = set_occupancy(index, relevant_bits, attack_mask);
        attacks[index] = if is_bishop {
            bishop_attacks_on_the_fly(square, occupancies[index])
        } else {
            rook_attacks_on_the_fly(square, occupancies[index])
        };
    }

    loop {
        let (magic, rnd) = next_magic_candidate(last_rand);
        last_rand = rnd;

        // Ignore inappropriate magics
        let mask = attack_mask.wrapping_mul(magic);
        if bit_count(mask & 0xFF00000000000000) < 6 {
            continue;
        }

        used_attacks = [0u64; 4096];
        let mut failed = false;

        for index in 0..occupancy_indices {
            let tested_magic = occupancies[index].wrapping_mul(magic);
            if let Some(magic_index) = tested_magic.checked_shr(64 - relevant_bits as u32) {
                let magic_index = magic_index as usize;
                if used_attacks[magic_index] == 0 {
                    used_attacks[magic_index] = attacks[index];
                } else if used_attacks[magic_index] != attacks[index] {
                    failed = true;
                }
            } else {
                failed = true;
            }

            if failed {
                break;
            }
        }

        if !failed {
            return (magic, last_rand);
        }
    }
}

fn generate_bishop_magic_numbers(occupancies: &[usize; 64]) -> [u64; 64] {
    let mut result = [0u64; 64];

    let mut random_state = 1804289383;

    for square in 0..64 {
        result[square] = {
            let (magic, rnd) =
                find_magic_number(random_state, square as i32, occupancies[square], true);
            random_state = rnd;
            magic
        };
    }

    result
}

fn generate_rook_magic_numbers(occupancies: &[usize; 64]) -> [u64; 64] {
    let mut result = [0u64; 64];

    let mut random_state = 1804289383;

    for square in 0..64 {
        result[square] = {
            let (magic, rnd) =
                find_magic_number(random_state, square as i32, occupancies[square], false);
            random_state = rnd;
            magic
        };
    }

    result
}

#[derive(Debug)]
struct MagicNumbers {
    bishop: [u64; 64],
    rook: [u64; 64],
}

impl MagicNumbers {
    pub fn new(occupancies: &Occupancies) -> Self {
        let mut bishop = [0u64; 64];
        let mut random_state = 1804289383;

        for square in 0..64 {
            bishop[square] = {
                let (magic, rnd) = find_magic_number(
                    random_state,
                    square as i32,
                    occupancies.bishop[square],
                    true,
                );
                random_state = rnd;
                magic
            };
        }

        let mut rook = [0u64; 64];

        for square in 0..64 {
            rook[square] = {
                let (magic, rnd) =
                    find_magic_number(random_state, square as i32, occupancies.rook[square], false);
                random_state = rnd;
                magic
            };
        }

        Self { bishop, rook }
    }
}

#[derive(Debug)]
struct SlidingMasks {
    bishop: [u64; 64],
    rook: [u64; 64],
}

impl SlidingMasks {
    fn new() -> Self {
        let mut bishop = [0u64; 64];
        let mut rook = [0u64; 64];

        for square in 0..64 {
            bishop[square] = mask_bishop_attacks(square as i32);
            rook[square] = mask_rook_attacks(square as i32);
        }

        Self { bishop, rook }
    }
}

#[derive(Debug)]
struct Occupancies {
    bishop: [usize; 64],
    rook: [usize; 64],
}

impl Occupancies {
    fn new() -> Self {
        Self {
            bishop: generate_bishop_occupancies(),
            rook: generate_rook_occupancies(),
        }
    }
}

pub struct Attacks {
    pub pawn: [[u64; 64]; 2],
    pub knight: [u64; 64],
    pub king: [u64; 64],
    pub rook: [[u64; 4096]; 64],
    pub bishop: [[u64; 512]; 64],

    magic_numbers: MagicNumbers,
    sliding_masks: SlidingMasks,
    occupancies: Occupancies,
}

fn time_as_ms(d: Duration) -> f64 {
    d.as_micros() as f64 * 1e-3
}

impl Attacks {
    pub fn new() -> Self {
        println!("Start generation... ");
        let timer = Instant::now();

        let occupancies = Occupancies::new();
        let occupancies_time = timer.elapsed();
        println!("  Occupancies ok ({}ms)... ", time_as_ms(occupancies_time));

        let magic_numbers = MagicNumbers::new(&occupancies);
        let magic_numbers_time = timer.elapsed();
        println!(
            "  Magic numbers ok ({}ms)... ",
            time_as_ms(magic_numbers_time - occupancies_time)
        );

        let sliding_masks = SlidingMasks::new();
        let sliding_masks_time = timer.elapsed();
        println!(
            "  Sliding masks ok ({}ms)... ",
            time_as_ms(sliding_masks_time - magic_numbers_time)
        );

        let pawn = generate_pawn_attacks();
        let pawn_time = timer.elapsed();
        println!(
            "  Pawn attacks ok ({}ms)... ",
            time_as_ms(pawn_time - sliding_masks_time)
        );

        let knight = generate_knight_attacks();
        let knight_time = timer.elapsed();
        println!(
            "  Knight attacks ok ({}ms)...",
            time_as_ms(knight_time - pawn_time)
        );

        let king = generate_king_attacks();
        let king_time = timer.elapsed();
        println!(
            "  King attacks ok ({}ms)...",
            time_as_ms(king_time - knight_time)
        );

        let mut bishop = [[0u64; 512]; 64];
        for square in 0..64 {
            let bishop_attack_mask = sliding_masks.bishop[square];
            let bishop_relevant_bit_count = bit_count(bishop_attack_mask);
            let bishop_occupancy_indices = 1 << bishop_relevant_bit_count;

            for index in 0..bishop_occupancy_indices {
                let occupancy = set_occupancy(index, bishop_relevant_bit_count, bishop_attack_mask);
                let magic_index = occupancy.wrapping_mul(magic_numbers.bishop[square])
                    >> (64 - occupancies.bishop[square]);

                bishop[square][magic_index as usize] =
                    bishop_attacks_on_the_fly(square as i32, occupancy);
            }
        }
        let bishop_time = timer.elapsed();
        println!(
            "  Bishop attacks ok ({}ms)...",
            time_as_ms(bishop_time - king_time)
        );

        let mut rook = [[0u64; 4096]; 64];
        for square in 0..64 {
            let rook_attack_mask = sliding_masks.rook[square];
            let rook_relevant_bit_count = bit_count(rook_attack_mask);
            let rook_occupancy_indices = 1 << rook_relevant_bit_count;

            for index in 0..rook_occupancy_indices {
                let occupancy = set_occupancy(index, rook_relevant_bit_count, rook_attack_mask);
                let magic_index = occupancy.wrapping_mul(magic_numbers.rook[square])
                    >> (64 - occupancies.rook[square]);

                rook[square][magic_index as usize] =
                    rook_attacks_on_the_fly(square as i32, occupancy);
            }
        }
        let rook_time = timer.elapsed();
        println!(
            "  Rook attacks ok ({}ms)...",
            time_as_ms(rook_time - bishop_time)
        );

        Self {
            pawn,
            knight,
            king,
            bishop,
            rook,
            magic_numbers,
            sliding_masks,
            occupancies,
        }
    }
}
