#![allow(dead_code)]

use crate::bitboard::*;
use crate::codegen::get_square;
use crate::Side;

const BISHOP_OCCUPANCY_COUNTS: [usize; 64] = generate_bishop_occupancies();
const ROOK_OCCUPANCY_COUNTS: [usize; 64] = generate_rook_occupancies();

const fn mask_pawn_attacks(square: i32, side: Side) -> u64 {
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

const fn mask_knight_attacks(square: i32) -> u64 {
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

const fn mask_king_attacks(square: i32) -> u64 {
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

const fn mask_bishop_attacks(square: i32) -> u64 {
    let mut attacks = 0u64;

    let rank = square / 8;
    let file = square % 8;

    let mut i = rank - 1;
    let mut j = file - 1;
    while i >= 1 && j >= 1 {
        attacks = set_bit(attacks, get_square(i, j));
        i -= 1;
        j -= 1;
    }

    let mut i = rank - 1;
    let mut j = file + 1;
    while i >= 1 && j < 7 {
        attacks = set_bit(attacks, get_square(i, j));
        i -= 1;
        j += 1;
    }

    let mut i = rank + 1;
    let mut j = file - 1;
    while i < 7 && j >= 1 {
        attacks = set_bit(attacks, get_square(i, j));
        i += 1;
        j -= 1;
    }

    let mut i = rank + 1;
    let mut j = file + 1;
    while i < 7 && j < 7 {
        attacks = set_bit(attacks, get_square(i, j));
        i += 1;
        j += 1;
    }

    attacks
}

pub const fn mask_rook_attacks(square: i32) -> u64 {
    let mut attacks = 0u64;

    let rank = square / 8;
    let file = square % 8;

    let mut i = rank - 1;
    while i >= 1 {
        attacks = set_bit(attacks, get_square(i, file));
        i -= 1;
    }

    let mut i = file - 1;
    while i >= 1 {
        attacks = set_bit(attacks, get_square(rank, i));
        i -= 1;
    }

    let mut i = rank + 1;
    while i < 7 {
        attacks = set_bit(attacks, get_square(i, file));
        i += 1;
    }

    let mut i = file + 1;
    while i < 7 {
        attacks = set_bit(attacks, get_square(rank, i));
        i += 1;
    }

    attacks
}

pub const fn bishop_attacks_on_the_fly(square: i32, blocker: u64) -> u64 {
    let mut attacks = 0u64;

    let rank = square / 8;
    let file = square % 8;

    let mut i = rank - 1;
    let mut j = file - 1;
    while i >= 0 && j >= 0 {
        let curr_square = get_square(i, j);
        attacks = set_bit(attacks, curr_square);
        if get_bit(blocker, curr_square) {
            break;
        }

        i -= 1;
        j -= 1;
    }

    let mut i = rank - 1;
    let mut j = file + 1;
    while i >= 0 && j <= 7 {
        let curr_square = get_square(i, j);
        attacks = set_bit(attacks, curr_square);
        if get_bit(blocker, curr_square) {
            break;
        }

        i -= 1;
        j += 1;
    }

    let mut i = rank + 1;
    let mut j = file - 1;
    while i <= 7 && j >= 0 {
        let curr_square = get_square(i, j);
        attacks = set_bit(attacks, curr_square);
        if get_bit(blocker, curr_square) {
            break;
        }

        i += 1;
        j -= 1;
    }

    let mut i = rank + 1;
    let mut j = file + 1;
    while i <= 7 && j <= 7 {
        let curr_square = get_square(i, j);
        attacks = set_bit(attacks, curr_square);
        if get_bit(blocker, curr_square) {
            break;
        }

        i += 1;
        j += 1;
    }

    attacks
}

pub const fn rook_attacks_on_the_fly(square: i32, blocker: u64) -> u64 {
    let mut attacks = 0u64;

    let rank = square / 8;
    let file = square % 8;

    let mut i = rank - 1;
    while i >= 0 {
        let curr_square = get_square(i, file);
        attacks = set_bit(attacks, curr_square);
        if get_bit(blocker, curr_square) {
            break;
        }
        i -= 1;
    }

    let mut i = file - 1;
    while i >= 0 {
        let curr_square = get_square(rank, i);
        attacks = set_bit(attacks, curr_square);
        if get_bit(blocker, curr_square) {
            break;
        }
        i -= 1;
    }

    let mut i = rank + 1;
    while i <= 7 {
        let curr_square = get_square(i, file);
        attacks = set_bit(attacks, curr_square);
        if get_bit(blocker, curr_square) {
            break;
        }
        i += 1;
    }

    let mut i = file + 1;
    while i <= 7 {
        let curr_square = get_square(rank, i);
        attacks = set_bit(attacks, curr_square);
        if get_bit(blocker, curr_square) {
            break;
        }
        i += 1;
    }

    attacks
}

const fn generate_pawn_attacks() -> [[u64; 64]; 2] {
    let mut result = [[0u64; 64]; 2];

    let mut square = 0;
    while square < 64 {
        result[0][square] = mask_pawn_attacks(square as i32, Side::White);
        result[1][square] = mask_pawn_attacks(square as i32, Side::Black);
        square += 1;
    }

    result
}

const fn generate_knight_attacks() -> [u64; 64] {
    let mut result = [0u64; 64];

    let mut square = 0;
    while square < 64 {
        result[square] = mask_knight_attacks(square as i32);
        square += 1;
    }

    result
}

const fn generate_king_attacks() -> [u64; 64] {
    let mut result = [0u64; 64];

    let mut square = 0;
    while square < 64 {
        result[square] = mask_king_attacks(square as i32);
        square += 1;
    }

    result
}

const fn generate_bishop_occupancies() -> [usize; 64] {
    let mut result = [0usize; 64];

    let mut square = 0;
    while square < 64 {
        result[square] = bit_count(mask_bishop_attacks(square as i32));
        square += 1;
    }

    result
}

const fn generate_rook_occupancies() -> [usize; 64] {
    let mut result = [0usize; 64];

    let mut square = 0;
    while square < 64 {
        result[square] = bit_count(mask_rook_attacks(square as i32));
        square += 1;
    }

    result
}

const fn set_occupancy(index: usize, bits_in_mask: usize, attack_mask: u64) -> u64 {
    let mut occupancy = 0u64;
    let mut mask = attack_mask;

    // Loop over the range of bits within attack mask
    let mut count = 0;
    while count < bits_in_mask {
        let square = least_significant_bit_index(mask) as i32;
        mask = pop_bit(mask, square);

        if index & (1 << count) != 0 {
            occupancy = set_bit(occupancy, square);
        }
        count += 1;
    }

    occupancy
}

const fn get_random_u32_number(random_state: u32) -> u32 {
    let mut number = random_state;

    // xorshift32
    number ^= number << 13;
    number ^= number >> 15;
    number ^= number << 5;

    number
}

const fn get_random_u64_number(random_state: u32) -> (u64, u32) {
    let r1 = get_random_u32_number(random_state);
    let r2 = get_random_u32_number(r1);
    let r3 = get_random_u32_number(r2);
    let r4 = get_random_u32_number(r3);

    let n1 = r1 as u64 & 0xFFFFu64;
    let n2 = r2 as u64 & 0xFFFFu64;
    let n3 = r3 as u64 & 0xFFFFu64;
    let n4 = r4 as u64 & 0xFFFFu64;

    let number = n1 | (n2 << 16) | (n3 << 32) | (n4 << 48);

    (number, r4)
}

const fn next_magic_candidate(random_state: u32) -> (u64, u32) {
    let (n1, r1) = get_random_u64_number(random_state);
    let (n2, r2) = get_random_u64_number(r1);
    let (n3, r3) = get_random_u64_number(r2);

    (n1 & n2 & n3, r3)
}

const fn find_magic_number(
    random_state: u32,
    square: i32,
    relevant_bits: usize,
    is_bishop: bool,
) -> (u64, u32) {
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

    let mut index = 0;
    while index < occupancy_indices {
        occupancies[index] = set_occupancy(index, relevant_bits, attack_mask);
        attacks[index] = if is_bishop {
            bishop_attacks_on_the_fly(square, occupancies[index])
        } else {
            rook_attacks_on_the_fly(square, occupancies[index])
        };

        index += 1;
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

        let mut index = 0;
        while index < occupancy_indices {
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

            index += 1;
        }

        if !failed {
            return (magic, last_rand);
        }
    }
}

fn generate_bishop_magic_numbers() -> [u64; 64] {
    let mut result = [0u64; 64];
    let mut square = 0;

    let mut random_state = 1804289383;

    while square < 64 {
        result[square] = {
            let (magic, rnd) = find_magic_number(
                random_state,
                square as i32,
                BISHOP_OCCUPANCY_COUNTS[square],
                true,
            );
            random_state = rnd;
            magic
        };

        square += 1;
    }

    result
}

fn generate_rook_magic_numbers() -> [u64; 64] {
    let mut result = [0u64; 64];
    let mut square = 0;

    let mut random_state = 1804289383;

    while square < 64 {
        result[square] = {
            let (magic, rnd) = find_magic_number(
                random_state,
                square as i32,
                ROOK_OCCUPANCY_COUNTS[square],
                false,
            );
            random_state = rnd;
            magic
        };

        square += 1;
    }

    result
}

#[derive(Debug)]
struct MagicNumbers {
    bishop: [u64; 64],
    rook: [u64; 64],
}

impl MagicNumbers {
    pub fn new() -> Self {
        let mut bishop = [0u64; 64];
        let mut random_state = 1804289383;

        for square in 0..64 {
            bishop[square] = {
                let (magic, rnd) = find_magic_number(
                    random_state,
                    square as i32,
                    BISHOP_OCCUPANCY_COUNTS[square],
                    true,
                );
                random_state = rnd;
                magic
            };
        }

        let mut rook = [0u64; 64];

        for square in 0..64 {
            rook[square] = {
                let (magic, rnd) = find_magic_number(
                    random_state,
                    square as i32,
                    ROOK_OCCUPANCY_COUNTS[square],
                    false,
                );
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
pub struct Attacks {
    pub pawn: [[u64; 64]; 2],
    pub knight: [u64; 64],
    pub king: [u64; 64],
    pub rook: [[u64; 4096]; 64],
    pub bishop: [[u64; 512]; 64],

    magic_numbers: MagicNumbers,
    sliding_masks: SlidingMasks,
}

impl Attacks {
    pub fn new() -> Self {
        let magic_numbers = MagicNumbers::new();
        let sliding_masks = SlidingMasks::new();

        let mut bishop = [[0u64; 512]; 64];
        let mut rook = [[0u64; 4096]; 64];

        for square in 0..64 {
            let bishop_attack_mask = sliding_masks.bishop[square];
            let bishop_relevant_bit_count = bit_count(bishop_attack_mask);
            let bishop_occupancy_indices = 1 << bishop_relevant_bit_count;

            for index in 0..bishop_occupancy_indices {
                let occupancy = set_occupancy(index, bishop_relevant_bit_count, bishop_attack_mask);
                let magic_index = occupancy.wrapping_mul(magic_numbers.bishop[square])
                    >> (64 - BISHOP_OCCUPANCY_COUNTS[square]);

                bishop[square][magic_index as usize] =
                    bishop_attacks_on_the_fly(square as i32, occupancy);
            }

            let rook_attack_mask = sliding_masks.rook[square];
            let rook_relevant_bit_count = bit_count(rook_attack_mask);
            let rook_occupancy_indices = 1 << rook_relevant_bit_count;

            for index in 0..rook_occupancy_indices {
                let occupancy = set_occupancy(index, rook_relevant_bit_count, rook_attack_mask);
                let magic_index = occupancy.wrapping_mul(magic_numbers.rook[square])
                    >> (64 - ROOK_OCCUPANCY_COUNTS[square]);

                rook[square][magic_index as usize] =
                    rook_attacks_on_the_fly(square as i32, occupancy);
            }
        }

        Self {
            pawn: generate_pawn_attacks(),
            knight: generate_knight_attacks(),
            king: generate_king_attacks(),
            bishop,
            rook,
            magic_numbers,
            sliding_masks,
        }
    }
}
