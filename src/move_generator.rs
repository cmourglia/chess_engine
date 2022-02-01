use crate::bitboard::*;
use crate::board::*;

/// Move encoding on an i32
///
/// 0000 0000 0000 0000 0000 0000 0011 1111 (    0x3F) -> Source square
/// 0000 0000 0000 0000 0000 1111 1100 0000 (   0xFC0) -> Destination square
/// 0000 0000 0000 0000 1111 0000 0000 0000 (  0xF000) -> Piece
/// 0000 0000 0000 1111 0000 0000 0000 0000 ( 0xF0000) -> Promotion resulting piece
/// 0000 0000 0001 0000 0000 0000 0000 0000 (0x100000) -> Capture flag
/// 0000 0000 0010 0000 0000 0000 0000 0000 (0x200000) -> Double push flag
/// 0000 0000 0100 0000 0000 0000 0000 0000 (0x400000) -> En-passant flag
/// 0000 0000 1000 0000 0000 0000 0000 0000 (0x800000) -> Castling flag

const SRC_SQUARE_MASK: i32 = 0x3F;
const DST_SQUARE_MASK: i32 = 0xFC0;
const PIECE_MASK: i32 = 0xF000;
const PROMOTION_PIECE_MASK: i32 = 0xF0000;
const CAPTURE_FLAG_MASK: i32 = 0x100000;
const DOUBLE_PUSH_FLAG_MASK: i32 = 0x200000;
const EN_PASSANT_FLAG_MASK: i32 = 0x400000;
const CASTLING_FLAG_MASK: i32 = 0x800000;

const SRC_SQUARE_BIT_OFFSET: i32 = 0;
const DST_SQUARE_BIT_OFFSET: i32 = 6;
const PIECE_BIT_OFFSET: i32 = 12;
const PROMOTION_PIECE_BIT_OFFSET: i32 = 16;
const CAPTURE_FLAG_BIT_OFFSET: i32 = 20;
const DOUBLE_PUSH_FLAG_BIT_OFFSET: i32 = 21;
const EN_PASSANT_FLAG_BIT_OFFSET: i32 = 22;
const CASTLING_FLAG_BIT_OFFSET: i32 = 23;

pub struct Move {}

impl Move {
    pub fn encode(piece: Piece, src_square: i32, dst_square: i32) -> i32 {
        let mut mv = 0i32;

        mv |= src_square << SRC_SQUARE_BIT_OFFSET;
        mv |= dst_square << DST_SQUARE_BIT_OFFSET;
        mv |= (piece as i32) << PIECE_BIT_OFFSET;

        mv
    }

    pub fn encode_capture(piece: Piece, src_square: i32, dst_square: i32) -> i32 {
        Move::encode(piece, src_square, dst_square) | 1 << CAPTURE_FLAG_BIT_OFFSET
    }

    pub fn encode_castling(piece: Piece, src_square: i32, dst_square: i32) -> i32 {
        Move::encode(piece, src_square, dst_square) | 1 << CASTLING_FLAG_BIT_OFFSET
    }

    pub fn encode_en_passant(piece: Piece, src_square: i32, dst_square: i32) -> i32 {
        Move::encode(piece, src_square, dst_square) | 1 << EN_PASSANT_FLAG_BIT_OFFSET
    }

    pub fn encode_promotion(
        piece: Piece,
        src_square: i32,
        dst_square: i32,
        with_capture: bool,
    ) -> i32 {
        let mut mv = Move::encode(piece, src_square, dst_square);

        mv |= 0xF << PROMOTION_PIECE_BIT_OFFSET;
        mv |= (with_capture as i32) << CAPTURE_FLAG_BIT_OFFSET;

        mv
    }

    pub fn encode_double_push(piece: Piece, src_square: i32, dst_square: i32) -> i32 {
        Move::encode(piece, src_square, dst_square) | 1 << DOUBLE_PUSH_FLAG_BIT_OFFSET
    }

    pub fn decode_src_square(mv: i32) -> i32 {
        (mv & SRC_SQUARE_MASK) >> SRC_SQUARE_BIT_OFFSET
    }

    pub fn decode_dst_square(mv: i32) -> i32 {
        (mv & DST_SQUARE_MASK) >> DST_SQUARE_BIT_OFFSET
    }

    pub fn decode_piece(mv: i32) -> Piece {
        Piece::from((mv & PIECE_MASK) >> PIECE_BIT_OFFSET)
    }

    pub fn decode_promotion_piece(mv: i32) -> Piece {
        Piece::from((mv & PROMOTION_PIECE_MASK) >> PROMOTION_PIECE_BIT_OFFSET)
    }

    pub fn is_promotion(mv: i32) -> bool {
        mv & PROMOTION_PIECE_MASK != 0
    }

    pub fn is_capture(mv: i32) -> bool {
        mv & CAPTURE_FLAG_MASK != 0
    }

    pub fn is_double_push(mv: i32) -> bool {
        mv & DOUBLE_PUSH_FLAG_MASK != 0
    }

    pub fn is_en_passant(mv: i32) -> bool {
        mv & EN_PASSANT_FLAG_MASK != 0
    }

    pub fn is_castling(mv: i32) -> bool {
        mv & CASTLING_FLAG_MASK != 0
    }
}

/// Check whether the given square is under attack.
/// In order to do this,instead of checking if any of the pieces is attacking the square,
/// we check if the square attacks any of the pieces as this piece.
/// e.g., if the e5 square is attacked by a black pawn on d6, it also means that,
/// as a white pawn, the square attacks d6, which is easier to check.
/// This also means we only do 6 attack lookups and 6 bitwise & instead of 16
/// (plus the need to find the pieces inside the pieces bitboards)
pub fn is_square_attacked(board: &Board, square: i32, attacking_side: Side) -> bool {
    let occupancy = board.occupancies[Side::Both as usize];

    let this_side = match attacking_side {
        Side::Black => Side::White,
        Side::White => Side::Black,
        Side::Both => unreachable!(),
    };

    let as_pawn = board.attacks.get_pawn_attacks(square, this_side);
    if bits_collide(as_pawn, board.bitboard(Piece::Pawn, attacking_side)) {
        return true;
    }

    let as_knight = board.attacks.get_knight_attacks(square);
    if bits_collide(as_knight, board.bitboard(Piece::Knight, attacking_side)) {
        return true;
    }

    let as_king = board.attacks.get_king_attacks(square);
    if bits_collide(as_king, board.bitboard(Piece::King, attacking_side)) {
        return true;
    }

    let as_bishop = board.attacks.get_bishop_attacks(square, occupancy);
    if bits_collide(as_bishop, board.bitboard(Piece::Bishop, attacking_side)) {
        return true;
    }

    let as_rook = board.attacks.get_rook_attacks(square, occupancy);
    if bits_collide(as_rook, board.bitboard(Piece::Rook, attacking_side)) {
        return true;
    }

    let as_queen = board.attacks.get_queen_attacks(square, occupancy);
    if bits_collide(as_queen, board.bitboard(Piece::Queen, attacking_side)) {
        return true;
    }

    false
}

pub struct Moves {
    moves: [i32; 256],
    move_count: usize,
}

impl Moves {
    pub fn new() -> Self {
        Self {
            moves: [0i32; 256],
            move_count: 0,
        }
    }

    pub fn push(&mut self, value: i32) {
        self.moves[self.move_count] = value;
        self.move_count += 1;
    }

    pub fn moves(&self) -> &[i32] {
        &self.moves[0..self.move_count]
    }

    pub fn len(&self) -> usize {
        self.move_count
    }
}

pub fn generate_moves(board: &Board) -> Moves {
    let mut moves = Moves::new();

    match board.side_to_move {
        Side::White => {
            generate_pawns(board, Side::White, &mut moves);
            generate_king_castles(board, Side::White, &mut moves);
        }
        Side::Black => {
            generate_pawns(board, Side::Black, &mut moves);
            generate_king_castles(board, Side::Black, &mut moves);
        }
        Side::Both => unreachable!(),
    }

    generate_knights(board, board.side_to_move, &mut moves);
    generate_bishops(board, board.side_to_move, &mut moves);
    generate_rooks(board, board.side_to_move, &mut moves);
    generate_queens(board, board.side_to_move, &mut moves);
    generate_kings(board, board.side_to_move, &mut moves);

    moves
}

fn generate_pawns(board: &Board, side: Side, moves: &mut Moves) {
    // Cache relevant data
    let all_occupancies = board.occupancies[Side::Both as usize];
    let opp_occupancies = board.occupancies[opponent_side(side) as usize];
    let en_passant_square = board.en_passant_square;

    // start_rank: This side's pawns start rank.
    //   This is given by the result of the integer division of the
    //   square index by 8.
    //   0 corresponds to rank 8,
    //   1 corresponds to rank 7 (black's start rank)
    //   ...
    //   6 corresponds to rank 2 (white's start rank)
    // back_rank: The opponents "back rank". This is used to detect promotion
    let (start_rank, back_rank) = match side {
        Side::White => (6, 0),
        Side::Black => (1, 7),
        Side::Both => unreachable!(),
    };

    let (one_square, two_squares) = match side {
        Side::White => (-8, -16),
        Side::Black => (8, 16),
        Side::Both => unreachable!(),
    };

    let mut bitboard = board.bitboard(Piece::Pawn, side);

    while bitboard != 0 {
        let src_square = least_significant_bit_index(bitboard) as i32;
        bitboard = pop_bit(bitboard, src_square);

        let dst_square = src_square + one_square;
        if !bits_collide(bitboard_from_square(dst_square), all_occupancies) {
            let rank = src_square / 8;

            if rank == back_rank {
                moves.push(Move::encode_promotion(
                    Piece::Pawn,
                    src_square,
                    dst_square,
                    false,
                ));
            } else {
                moves.push(Move::encode(Piece::Pawn, src_square, dst_square));
            }

            // The two squares move is only relevant if there is already no
            // blocker for the one square move.
            // We also need to make sure we are on the start rank.
            if rank == start_rank {
                let dst_square = src_square + two_squares;
                if !bits_collide(bitboard_from_square(dst_square), all_occupancies) {
                    moves.push(Move::encode_double_push(
                        Piece::Pawn,
                        src_square,
                        dst_square,
                    ));
                }
            }
        }

        let mut attacks = board.attacks.get_pawn_attacks(src_square, side);
        if bits_collide(attacks, opp_occupancies) {
            while attacks != 0 {
                let dst_square = least_significant_bit_index(attacks) as i32;
                attacks = pop_bit(attacks, dst_square);

                if bits_collide(bitboard_from_square(dst_square), opp_occupancies) {
                    if src_square / 8 == back_rank {
                        moves.push(Move::encode_promotion(
                            Piece::Pawn,
                            src_square,
                            dst_square,
                            true,
                        ));
                    } else {
                        moves.push(Move::encode_capture(Piece::Pawn, src_square, dst_square));
                    }
                } else if en_passant_square == dst_square {
                    // This test would be relevant only on 4th and 5th ranks, but it might
                    // be more costly to perform a division than just test directly
                    // with data already loaded...
                    moves.push(Move::encode_en_passant(Piece::Pawn, src_square, dst_square));
                }
            }
        }
    }
}

fn generate_king_castles(board: &Board, side: Side, moves: &mut Moves) {
    let king_bitboard = board.bitboard(Piece::King, side);
    let king_square = least_significant_bit_index(king_bitboard) as i32;
    let opponent_side = opponent_side(side);

    // For some reason, we have no king...
    if king_bitboard == 0 {
        return;
    }

    // The king is under attack (checked), we cannot castle.
    if is_square_attacked(board, king_square, opponent_side) {
        return;
    }

    // Extract board info. Can we still castle king or queen side ?
    let (can_castle_king_side, can_castle_queen_side) = match side {
        Side::White => (
            board.castling_rights & Castling::WhiteKing as u8 != 0,
            board.castling_rights & Castling::WhiteQueen as u8 != 0,
        ),
        Side::Black => (
            board.castling_rights & Castling::BlackKing as u8 != 0,
            board.castling_rights & Castling::BlackQueen as u8 != 0,
        ),
        Side::Both => unreachable!(),
    };

    let all_occupancies = board.occupancies[Side::Both as usize];

    if can_castle_king_side {
        let squares = [king_square + 1, king_square + 2];

        if can_castle(board, &squares, all_occupancies, opponent_side) {
            moves.push(Move::encode_castling(
                Piece::King,
                king_square,
                king_square + 2,
            ));
        }
    }

    if can_castle_queen_side {
        let squares = [king_square - 1, king_square - 2, king_square - 3];

        if can_castle(board, &squares, all_occupancies, opponent_side) {
            moves.push(Move::encode_castling(
                Piece::King,
                king_square,
                king_square - 2,
            ));
        }
    }
}

fn can_castle(board: &Board, squares: &[i32], occupancy: u64, opponent_side: Side) -> bool {
    let mut ok = true;
    for square in squares {
        let bitboard = bitboard_from_square(*square);
        if bits_collide(bitboard, occupancy) {
            ok = false;
            break;
        }

        if is_square_attacked(board, *square, opponent_side) {
            ok = false;
            break;
        }
    }

    ok
}

fn handle_attacks(
    piece: Piece,
    initial_attacks: u64,
    initial_square: i32,
    my_occupancy: u64,
    opponent_occupancy: u64,
    moves: &mut Moves,
) {
    let mut attacks = initial_attacks;

    while attacks != 0 {
        let attacked_square = least_significant_bit_index(attacks) as i32;
        attacks = pop_bit(attacks, attacked_square);

        let attacked_bitboard = bitboard_from_square(attacked_square);

        if bits_collide(attacked_bitboard, opponent_occupancy) {
            moves.push(Move::encode_capture(piece, initial_square, attacked_square));
        } else if !bits_collide(attacked_bitboard, my_occupancy) {
            moves.push(Move::encode(piece, initial_square, attacked_square));
        }
    }
}

fn generate_knights(board: &Board, side: Side, moves: &mut Moves) {
    let my_occupancy = board.occupancies[side as usize];
    let opponent_occupancy = board.occupancies[opponent_side(side) as usize];

    let mut knights = board.bitboard(Piece::Knight, side);

    while knights != 0 {
        let square = least_significant_bit_index(knights) as i32;
        knights = pop_bit(knights, square);

        let attacks = board.attacks.get_knight_attacks(square);
        handle_attacks(
            Piece::Knight,
            attacks,
            square,
            my_occupancy,
            opponent_occupancy,
            moves,
        );
    }
}

fn generate_bishops(board: &Board, side: Side, moves: &mut Moves) {
    let occupancy = board.occupancies[Side::Both as usize];
    let my_occupancy = board.occupancies[side as usize];
    let opponent_occupancy = board.occupancies[opponent_side(side) as usize];

    let mut bishops = board.bitboard(Piece::Bishop, side);

    while bishops != 0 {
        let square = least_significant_bit_index(bishops) as i32;
        bishops = pop_bit(bishops, square);

        let attacks = board.attacks.get_bishop_attacks(square, occupancy);
        handle_attacks(
            Piece::Bishop,
            attacks,
            square,
            my_occupancy,
            opponent_occupancy,
            moves,
        );
    }
}

fn generate_rooks(board: &Board, side: Side, moves: &mut Moves) {
    let occupancy = board.occupancies[Side::Both as usize];
    let my_occupancy = board.occupancies[side as usize];
    let opponent_occupancy = board.occupancies[opponent_side(side) as usize];

    let mut rooks = board.bitboard(Piece::Rook, side);

    while rooks != 0 {
        let square = least_significant_bit_index(rooks) as i32;
        rooks = pop_bit(rooks, square);

        let attacks = board.attacks.get_rook_attacks(square, occupancy);
        handle_attacks(
            Piece::Rook,
            attacks,
            square,
            my_occupancy,
            opponent_occupancy,
            moves,
        );
    }
}

fn generate_queens(board: &Board, side: Side, moves: &mut Moves) {
    let occupancy = board.occupancies[Side::Both as usize];
    let my_occupancy = board.occupancies[side as usize];
    let opponent_occupancy = board.occupancies[opponent_side(side) as usize];

    let mut queens = board.bitboard(Piece::Queen, side);

    while queens != 0 {
        let square = least_significant_bit_index(queens) as i32;
        queens = pop_bit(queens, square);

        let attacks = board.attacks.get_queen_attacks(square, occupancy);
        handle_attacks(
            Piece::Queen,
            attacks,
            square,
            my_occupancy,
            opponent_occupancy,
            moves,
        );
    }
}

fn generate_kings(board: &Board, side: Side, moves: &mut Moves) {
    let my_occupancy = board.occupancies[side as usize];
    let opponent_occupancy = board.occupancies[opponent_side(side) as usize];

    let king = board.bitboard(Piece::King, side);
    let square = least_significant_bit_index(king) as i32;

    let mut attacks = board.attacks.get_king_attacks(square);

    while attacks != 0 {
        let attacked_square = least_significant_bit_index(attacks) as i32;
        attacks = pop_bit(attacks, attacked_square);

        let attacked_bitboard = bitboard_from_square(attacked_square);

        // We need to make sure we do not put ourselves in check
        if !is_square_attacked(board, attacked_square, opponent_side(side)) {
            if bits_collide(attacked_bitboard, opponent_occupancy) {
                moves.push(Move::encode_capture(Piece::King, square, attacked_square));
            } else if !bits_collide(attacked_bitboard, my_occupancy) {
                moves.push(Move::encode(Piece::King, square, attacked_square));
            }
        }
    }
}
