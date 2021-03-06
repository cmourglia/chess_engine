#![allow(non_upper_case_globals, dead_code)]

use phf::phf_map;

pub const a8: i32 = 0;
pub const b8: i32 = 1;
pub const c8: i32 = 2;
pub const d8: i32 = 3;
pub const e8: i32 = 4;
pub const f8: i32 = 5;
pub const g8: i32 = 6;
pub const h8: i32 = 7;
pub const a7: i32 = 8;
pub const b7: i32 = 9;
pub const c7: i32 = 10;
pub const d7: i32 = 11;
pub const e7: i32 = 12;
pub const f7: i32 = 13;
pub const g7: i32 = 14;
pub const h7: i32 = 15;
pub const a6: i32 = 16;
pub const b6: i32 = 17;
pub const c6: i32 = 18;
pub const d6: i32 = 19;
pub const e6: i32 = 20;
pub const f6: i32 = 21;
pub const g6: i32 = 22;
pub const h6: i32 = 23;
pub const a5: i32 = 24;
pub const b5: i32 = 25;
pub const c5: i32 = 26;
pub const d5: i32 = 27;
pub const e5: i32 = 28;
pub const f5: i32 = 29;
pub const g5: i32 = 30;
pub const h5: i32 = 31;
pub const a4: i32 = 32;
pub const b4: i32 = 33;
pub const c4: i32 = 34;
pub const d4: i32 = 35;
pub const e4: i32 = 36;
pub const f4: i32 = 37;
pub const g4: i32 = 38;
pub const h4: i32 = 39;
pub const a3: i32 = 40;
pub const b3: i32 = 41;
pub const c3: i32 = 42;
pub const d3: i32 = 43;
pub const e3: i32 = 44;
pub const f3: i32 = 45;
pub const g3: i32 = 46;
pub const h3: i32 = 47;
pub const a2: i32 = 48;
pub const b2: i32 = 49;
pub const c2: i32 = 50;
pub const d2: i32 = 51;
pub const e2: i32 = 52;
pub const f2: i32 = 53;
pub const g2: i32 = 54;
pub const h2: i32 = 55;
pub const a1: i32 = 56;
pub const b1: i32 = 57;
pub const c1: i32 = 58;
pub const d1: i32 = 59;
pub const e1: i32 = 60;
pub const f1: i32 = 61;
pub const g1: i32 = 62;
pub const h1: i32 = 63;
pub const NO_SQUARE: i32 = -1;

pub const CELL_NAMES: [&str; 64] = [
    "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8", "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7",
    "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6", "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5",
    "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4", "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3",
    "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2", "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1",
];

pub const CELL_TO_SQUARE: phf::Map<&'static str, i32> = phf_map! {
    "a8" => 0,
    "b8" => 1,
    "c8" => 2,
    "d8" => 3,
    "e8" => 4,
    "f8" => 5,
    "g8" => 6,
    "h8" => 7,
    "a7" => 8,
    "b7" => 9,
    "c7" => 10,
    "d7" => 11,
    "e7" => 12,
    "f7" => 13,
    "g7" => 14,
    "h7" => 15,
    "a6" => 16,
    "b6" => 17,
    "c6" => 18,
    "d6" => 19,
    "e6" => 20,
    "f6" => 21,
    "g6" => 22,
    "h6" => 23,
    "a5" => 24,
    "b5" => 25,
    "c5" => 26,
    "d5" => 27,
    "e5" => 28,
    "f5" => 29,
    "g5" => 30,
    "h5" => 31,
    "a4" => 32,
    "b4" => 33,
    "c4" => 34,
    "d4" => 35,
    "e4" => 36,
    "f4" => 37,
    "g4" => 38,
    "h4" => 39,
    "a3" => 40,
    "b3" => 41,
    "c3" => 42,
    "d3" => 43,
    "e3" => 44,
    "f3" => 45,
    "g3" => 46,
    "h3" => 47,
    "a2" => 48,
    "b2" => 49,
    "c2" => 50,
    "d2" => 51,
    "e2" => 52,
    "f2" => 53,
    "g2" => 54,
    "h2" => 55,
    "a1" => 56,
    "b1" => 57,
    "c1" => 58,
    "d1" => 59,
    "e1" => 60,
    "f1" => 61,
    "g1" => 62,
    "h1" => 63,
};
