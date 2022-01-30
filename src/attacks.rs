#![allow(dead_code)]

use std::time::{Duration, Instant};

use crate::bitboard::*;
use crate::board::Side;
use crate::codegen::get_square;

const BISHOP_MAGIC_NUMBERS: [u64; 64] = [
    22698322406146064,
    1205099108012032,
    9228087301064687616,
    73187894424010752,
    1130444285345906,
    4612275373907992576,
    2305988214307962881,
    2306019103008638978,
    288866237490987264,
    72216138562421252,
    1162214595146940544,
    37163512463630369,
    2315977277682221568,
    9368050759537001484,
    4611691533434038336,
    38484651935748,
    594475288664083602,
    9052289030553760,
    327074215130433544,
    7298083213874892800,
    147492889954385920,
    72093336789393409,
    10376383993509711872,
    2341908098781939841,
    9239486531352396033,
    2312604190083450881,
    1130332548499472,
    4612816316921872896,
    18159534086299648,
    148636654801256960,
    1162211278356744200,
    70922811770880,
    1157434484545553041,
    290539367752177667,
    17334400050057448448,
    9259403035103723648,
    4515153089667584,
    186354585114313280,
    581395369348858368,
    720934383325946884,
    1585550262873362440,
    330174554376773637,
    1171780878892830720,
    2378079007612618752,
    211141968036416,
    299273329573952,
    577025935659768325,
    5910134490650908032,
    564067182052352,
    4857645661978624,
    86695402290546696,
    4828157972997996608,
    144124122148962304,
    4508857473761281,
    2310771089116012544,
    6918656061436134410,
    288250176238258186,
    5774531719424258058,
    1152921783790252048,
    2325469252946049,
    432381023788073472,
    3476780081401086209,
    28237116734505473,
    9223961384147108352,
];

const ROOK_MAGIC_NUMBERS: [u64; 64] = [
    36029347315843096,
    2395915482942144836,
    4683761243343822848,
    5800645116214378624,
    144117387167277064,
    72061992218788096,
    4935947390663263488,
    144120840269693010,
    586734589561537152,
    310889386656923784,
    144678277649272896,
    13835339598980451584,
    141287378387968,
    216876478145757312,
    39406501303190056,
    9386908999430996224,
    2323866753710456865,
    4508272553886784,
    1442564753737138193,
    4611829504963250176,
    9223672203663441924,
    2309361996245500416,
    9238030728695972368,
    9277804459638423812,
    36029077265653760,
    1152956698642894848,
    1198476265293856,
    2612378338414690576,
    1227232002265580944,
    4612249870324477956,
    7116056435917058,
    2305861159746216004,
    450430333637099553,
    1342107942052167744,
    81099979820634120,
    281509890101248,
    9232519990786131968,
    577024595912038404,
    4909064335625552384,
    12686640717271924801,
    11529250781270147073,
    4503875042164736,
    864972740875059216,
    2251868541583369,
    4647719213627113600,
    9251519543116071040,
    11402004316553224,
    6918093079257940004,
    282171843543808,
    11558204743909632,
    589976018488426624,
    153162004377436416,
    11533727443936608384,
    18295890666586880,
    144258130091484160,
    594485269764262400,
    9223392120675533314,
    1729418542945669638,
    635517997811746,
    4902449807076901963,
    4756364173907068930,
    302304468669302826,
    565153280561156,
    277027554306,
];

fn mask_pawn_attacks(square: i32, side: Side) -> u64 {
    let mut attacks = 0u64;
    let bitboard = bitboard_from_square(square);

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
        _ => {}
    }

    attacks
}

fn mask_knight_attacks(square: i32) -> u64 {
    let mut attacks = 0u64;
    let bitboard = bitboard_from_square(square);

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
    let bitboard = bitboard_from_square(square);

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
    pawn: [[u64; 64]; 2],
    knight: [u64; 64],
    king: [u64; 64],
    rook: Vec<u64>,
    bishop: Vec<u64>,

    sliding_masks: SlidingMasks,
    occupancies: Occupancies,
}

fn time_as_ms(d: Duration) -> f64 {
    d.as_micros() as f64 * 1e-3
}

impl Attacks {
    pub fn print_magic_numbers() {
        let occupancies = Occupancies::new();
        let magic_numbers = MagicNumbers::new(&occupancies);

        println!("Bishop magics: {:?}", magic_numbers.bishop);
        println!("Rook magics: {:?}", magic_numbers.rook);
    }

    pub fn new() -> Self {
        println!("Start generation... ");
        let timer = Instant::now();

        let occupancies = Occupancies::new();
        let occupancies_time = timer.elapsed();
        println!("  Occupancies ok ({}ms)... ", time_as_ms(occupancies_time));

        let sliding_masks = SlidingMasks::new();
        let sliding_masks_time = timer.elapsed();
        println!(
            "  Sliding masks ok ({}ms)... ",
            time_as_ms(sliding_masks_time - occupancies_time)
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

        let mut bishop = Vec::new();
        bishop.resize(512 * 64, 0u64);

        for square in 0..64 {
            let bishop_attack_mask = sliding_masks.bishop[square];
            let bishop_relevant_bit_count = bit_count(bishop_attack_mask);
            let bishop_occupancy_indices = 1 << bishop_relevant_bit_count;

            for index in 0..bishop_occupancy_indices {
                let occupancy = set_occupancy(index, bishop_relevant_bit_count, bishop_attack_mask);
                let magic_index = occupancy.wrapping_mul(BISHOP_MAGIC_NUMBERS[square])
                    >> (64 - occupancies.bishop[square]);

                bishop[square * 512 + magic_index as usize] =
                    bishop_attacks_on_the_fly(square as i32, occupancy);
            }
        }
        let bishop_time = timer.elapsed();
        println!(
            "  Bishop attacks ok ({}ms)...",
            time_as_ms(bishop_time - king_time)
        );

        let mut rook = Vec::new();
        rook.resize(4096 * 64, 0u64);

        for square in 0..64 {
            let rook_attack_mask = sliding_masks.rook[square];
            let rook_relevant_bit_count = bit_count(rook_attack_mask);
            let rook_occupancy_indices = 1 << rook_relevant_bit_count;

            for index in 0..rook_occupancy_indices {
                let occupancy = set_occupancy(index, rook_relevant_bit_count, rook_attack_mask);
                let magic_index = occupancy.wrapping_mul(ROOK_MAGIC_NUMBERS[square])
                    >> (64 - occupancies.rook[square]);

                rook[square * 4096 + magic_index as usize] =
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
            sliding_masks,
            occupancies,
        }
    }

    pub fn get_pawn_attacks(&self, square: i32, side: Side) -> u64 {
        self.pawn[side as usize][square as usize]
    }

    pub fn get_knight_attacks(&self, square: i32) -> u64 {
        self.knight[square as usize]
    }

    pub fn get_king_attacks(&self, square: i32) -> u64 {
        self.king[square as usize]
    }

    pub fn get_bishop_attacks(&self, square: i32, occupancy: u64) -> u64 {
        let mut occupancy_idx = occupancy;

        let index = square as usize;

        occupancy_idx &= self.sliding_masks.bishop[index];
        occupancy_idx = occupancy_idx.wrapping_mul(BISHOP_MAGIC_NUMBERS[index]);
        occupancy_idx >>= 64 - self.occupancies.bishop[index];

        self.bishop[index * 512 + occupancy_idx as usize]
    }

    pub fn get_rook_attacks(&self, square: i32, occupancy: u64) -> u64 {
        let mut occupancy_idx = occupancy;

        let index = square as usize;

        occupancy_idx &= self.sliding_masks.rook[index];
        occupancy_idx = occupancy_idx.wrapping_mul(ROOK_MAGIC_NUMBERS[index]);
        occupancy_idx >>= 64 - self.occupancies.rook[index];

        self.rook[index * 4096 + occupancy_idx as usize]
    }

    pub fn get_queen_attacks(&self, square: i32, occupancy: u64) -> u64 {
        self.get_bishop_attacks(square, occupancy) | self.get_rook_attacks(square, occupancy)
    }
}
