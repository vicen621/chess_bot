use crate::{bitboard::{BitBoard, BitBoardMethods, PieceItr, RANK_3, RANK_7}, chess_move::{Direction, Move, MoveType, PromotionType}, defs::{Color, Square}, pieces::PieceType, position::Position};

// TODO: Ver si se pueden pregenerar los movimientos y luego usarlos en el generador de movimientos


pub struct PawnDirections {
    up: i8,
    rank7: BitBoard,
    rank3: BitBoard,
}
impl PawnDirections {
    fn new(color: Color) -> Self {
        let (up, rank7, rank3) = match color {
            Color::White => (Direction::UP, RANK_7, RANK_3),
            Color::Black => (Direction::DOWN, RANK_7.reverse_colors(), RANK_3.reverse_colors()),
        };

        Self {
            up,
            rank7,
            rank3,
        }
    }
}

pub struct PawnMoves;
impl PawnMoves {
    pub fn generate_pseudo_legal_pawn_moves(position: &Position, moves: &mut Vec<Move>) {
        let dirs = PawnDirections::new(position.get_side_to_move());
        let pawns = position.get_pieces_color_type(position.get_side_to_move(), PieceType::Pawn);

        PawnMoves::generate_pawn_pushes(position, moves, &dirs, pawns);
        PawnMoves::generate_pawn_captures(position, moves, &dirs, pawns);
        PawnMoves::generate_pawn_en_passant(position, moves, &dirs, pawns);
        PawnMoves::generate_pawn_promotions(position, moves, &dirs, pawns);
    }

    /// Generate all quiet pushes, defined as single and double pushes,
    /// but excludes all promotions.
    fn generate_pawn_pushes(position: &Position, moves: &mut Vec<Move>, dirs: &PawnDirections, pawns: BitBoard) {
        let pawns = pawns & !dirs.rank7; // exluding promotions
        let empty_squares = !position.get_all_pieces();

        let single_pushes: BitBoard = pawns.shift(dirs.up) & empty_squares;
        let double_pawns = single_pushes & dirs.rank3;
        let double_pushes: BitBoard = double_pawns.shift(dirs.up) & empty_squares;

        PawnMoveExtractor::extract_pawn_moves(single_pushes, dirs.up, moves, MoveType::Quiet);
        PawnMoveExtractor::extract_pawn_moves(double_pushes, dirs.up + dirs.up, moves, MoveType::Quiet);
    }

    /// Generate all captures, excluding en passant captures and those which
    /// result in promotions and under-promotions.
    fn generate_pawn_captures(position: &Position, moves: &mut Vec<Move>, dirs: &PawnDirections, pawns: BitBoard) {
        let color = position.get_side_to_move();
        let pawns = pawns & !dirs.rank7; // excluding promotions
        let their_king = position.get_pieces_color_type(!color, PieceType::King);
        let captures = position.get_pieces_color(!color) & !their_king;

        let captures_left = pawns.shift(dirs.up + Direction::LEFT) & captures;
        let captures_right = pawns.shift(dirs.up + Direction::RIGHT) & captures;

        PawnMoveExtractor::extract_pawn_moves(captures_left, dirs.up + Direction::LEFT, moves, MoveType::Capture);
        PawnMoveExtractor::extract_pawn_moves(captures_right, dirs.up + Direction::RIGHT, moves, MoveType::Capture);
    }

    /// Generate all en passant captures.
    fn generate_pawn_en_passant(position: &Position, moves: &mut Vec<Move>, dirs: &PawnDirections, pawns: BitBoard) {
        let en_passant_bb = BitBoard::from_maybe_square(position.get_en_passant());

        if let Some(en_passant_bb) = en_passant_bb {
            let left_captures = pawns.shift(dirs.up + Direction::LEFT) & en_passant_bb;
            let right_captures = pawns.shift(dirs.up + Direction::RIGHT) & en_passant_bb;

            PawnMoveExtractor::extract_pawn_moves(left_captures, dirs.up + Direction::LEFT, moves, MoveType::EnPassantCapture);
            PawnMoveExtractor::extract_pawn_moves(right_captures, dirs.up + Direction::RIGHT, moves, MoveType::EnPassantCapture);
        }
    }

    /// Generate all promotions and under promotions, including pushes and captures on the eighth rank.
    fn generate_pawn_promotions(position: &Position, moves: &mut Vec<Move>, dirs: &PawnDirections, pawns: BitBoard) {
        let color = position.get_side_to_move();
        let pawns = pawns & dirs.rank7; // promotions only
        let empty_squares = !position.get_all_pieces();
        let their_king = position.get_pieces_color_type(!color, PieceType::King);
        let captures = position.get_pieces_color(!color) & !their_king;

        let pushes = pawns.shift(dirs.up) & empty_squares;
        let captures_left = pawns.shift(dirs.up + Direction::LEFT) & captures;
        let captures_right = pawns.shift(dirs.up + Direction::RIGHT) & captures;

        PawnMoveExtractor::extract_pawn_promotions(pushes, dirs.up, moves, PromotionType::Push);
        PawnMoveExtractor::extract_pawn_promotions(captures_left, dirs.up + Direction::LEFT, moves, PromotionType::Capture);
        PawnMoveExtractor::extract_pawn_promotions(captures_right, dirs.up + Direction::RIGHT, moves, PromotionType::Capture);
    }
}

struct PawnMoveExtractor;
impl PawnMoveExtractor {
    /// Given a resulting bitboard and a relevant offset, find all pawn moves using the given offset.
    pub fn extract_pawn_moves(bb: BitBoard, offset: i8, list: &mut Vec<Move>, kind: MoveType) {
        for (square, _) in bb.iter() {
            let m = Move {
                to: square,
                from: Square::from_index((square.to_index() as i8 - offset) as usize),
                kind,
            };
            list.push(m);
        }
    }

    /// Given a resulting bitboard, find and enumerate all possible promotions using the provided offset.
    pub fn extract_pawn_promotions(bb: BitBoard, offset: i8, list: &mut Vec<Move>, kind: PromotionType) {
        let itr = match kind {
            PromotionType::Push => MoveType::promotion_itr(),
            PromotionType::Capture => MoveType::promotion_capture_itr(),
        };

        for (square, _) in bb.iter() {
            let from = Square::from_index((square as i8 - offset) as usize);

            itr.clone().for_each(|promotion| -> () {
                let m = Move {
                    to: square,
                    from,
                    kind: *promotion,
                };
                list.push(m);
            });
        }
    }
}


#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;

    #[test]
    fn test_generate_pawn_moves() {
        let position = Position::from_str("8/P7/8/8/8/8/3PPP1P/8 w - - 0 1").ok().unwrap();

        let dirs = PawnDirections::new(position.get_side_to_move());
        let mut moves = vec![];
        let pawns = position.get_pieces_color_type(position.get_side_to_move(), PieceType::Pawn);

        PawnMoves::generate_pawn_pushes(&position, &mut moves, &dirs, pawns);
        moves.iter().for_each(|f| println!("{}", f.to_uci()));
        assert_eq!(moves.len(), 8);
    }

    #[test]
    fn test_generate_pawn_promotions() {
        let position = Position::from_str("8/P2PPP1P/8/8/8/8/8/2K2k2 w - - 0 1").ok().unwrap();

        let dirs = PawnDirections::new(position.get_side_to_move());
        let mut moves = vec![];
        let pawns = position.get_pieces_color_type(position.get_side_to_move(), PieceType::Pawn);

        PawnMoves::generate_pawn_promotions(&position, &mut moves, &dirs, pawns);
        moves.iter().for_each(|f| println!("{}", f.to_uci()));
        assert_eq!(moves.len(), 20);
    }

    #[test]
    fn test_generate_pawn_promotions_captures() {
        let position = Position::from_str("1b6/P7/8/8/6k1/2K5/8/8 w HAha - 0 1").ok().unwrap();

        let dirs = PawnDirections::new(position.get_side_to_move());
        let mut moves = vec![];
        let pawns = position.get_pieces_color_type(position.get_side_to_move(), PieceType::Pawn);

        PawnMoves::generate_pawn_promotions(&position, &mut moves, &dirs, pawns);
        moves.iter().for_each(|f| println!("{}", f.to_uci()));
        assert_eq!(moves.len(), 8);
    }

    #[test]
    fn test_generate_pawn_captures() {
        let position = Position::from_str("8/8/1R5r/P7/8/p1r5/1P6/8 w - - 0 1").ok().unwrap();

        let dirs = PawnDirections::new(position.get_side_to_move());
        let mut moves = vec![];
        let pawns = position.get_pieces_color_type(position.get_side_to_move(), PieceType::Pawn);

        PawnMoves::generate_pawn_captures(&position, &mut moves, &dirs, pawns);
        moves.iter().for_each(|f| println!("{}", f.to_uci()));
        assert_eq!(moves.len(), 2);
    }
}
