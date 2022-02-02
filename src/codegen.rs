#![allow(dead_code)]

use crate::bitboard::*;

pub const fn get_square(rank: i32, file: i32) -> i32 {
    rank * 8 + file
}

fn for_each_square<T>(cb: &mut T)
where
    T: FnMut(i32, i32, i32),
{
    for rank in 0..8 {
        for file in 0..8 {
            let square = get_square(file, rank);
            cb(rank, file, square);
        }
    }
}

pub fn generate_squares() {
    for_each_square(&mut |file, rank, square| {
        let letter: char = ('A' as u8 + file as u8).into();
        println!("pub const {}{}: i32 = {};", letter, 8 - rank, square);
    });
}

pub const fn not_a_file() -> u64 {
    let mut bitboard = 0u64;
    let mut rank = 0;
    while rank < 8 {
        let mut file = 1;
        while file < 8 {
            bitboard |= bitboard_from_square(get_square(rank, file));
            file += 1;
        }
        rank += 1;
    }

    bitboard
}

pub const fn not_h_file() -> u64 {
    let mut bitboard = 0u64;
    let mut rank = 0;
    while rank < 8 {
        let mut file = 0;
        while file < 7 {
            bitboard |= bitboard_from_square(get_square(rank, file));
            file += 1;
        }
        rank += 1;
    }

    bitboard
}

pub const fn not_ab_file() -> u64 {
    let mut bitboard = 0u64;
    let mut rank = 0;
    while rank < 8 {
        let mut file = 2;
        while file < 8 {
            bitboard |= bitboard_from_square(get_square(rank, file));
            file += 1;
        }
        rank += 1;
    }

    bitboard
}

pub const fn not_gh_file() -> u64 {
    let mut bitboard = 0u64;
    let mut rank = 0;
    while rank < 8 {
        let mut file = 0;
        while file < 6 {
            bitboard |= bitboard_from_square(get_square(rank, file));
            file += 1;
        }
        rank += 1;
    }

    bitboard
}
