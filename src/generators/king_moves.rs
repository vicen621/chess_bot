use crate::{bitboard::{BitBoard, BitBoardMethods, EMPTY}, chess_move::{Direction, Move, MoveExtractor, MoveType}, defs::Square, pieces::PieceType, position::Position};

struct KingLookup {
    moves: [BitBoard; 64],
}

impl KingLookup {
    pub fn new() -> Self {
        let mut moves = [EMPTY; 64];

        for i in 0..64 {
            let king = BitBoard::from_square(Square::from_index(i));
            moves[i] = Self::generate_king_moves(king);
        }

        Self { moves }
    }

    pub fn generate_pseudo_legal_king_moves(&self, position: &Position, moves: &mut Vec<Move>) {
        let king = position.get_pieces_color_type(position.get_side_to_move(), PieceType::King);

        self.generate_king_attacks(position, moves, king);
        self.generate_king_quiet_moves(position, moves, king);
    }
}

impl KingLookup {
    fn generate_king_moves(bb: BitBoard) -> BitBoard {
        let mut attacks = EMPTY;

        Direction::king_itr().for_each(|offset| -> () {
            let target = bb.shift(*offset);
            attacks |= target;
        });

        attacks
    }

    fn generate_king_quiet_moves(&self, position: &Position, moves: &mut Vec<Move>, king: BitBoard) {
        let empty_squares = !position.get_all_pieces();

        let quiet_moves = self.get_king_moves_bb(king) & empty_squares;

        MoveExtractor::extract_moves(king.to_square(), quiet_moves, moves, MoveType::Quiet);
    }

    fn generate_king_attacks(&self, position: &Position, moves: &mut Vec<Move>, king: BitBoard) {
        let color = position.get_side_to_move();
        let their_king = position.get_pieces_color_type(!color, PieceType::King);
        let captures = position.get_pieces_color(!color) & !their_king;

        let king_captures = self.get_king_moves_bb(king) & captures;

        MoveExtractor::extract_moves(king.to_square(), king_captures, moves, MoveType::Quiet);
    }

    fn get_king_moves_bb(&self, bb: BitBoard) -> BitBoard {
        self.moves[bb.trailing_zeros() as usize]
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_generate_king_moves() {
        let bb = BitBoard::from_square(Square::A8);
        let attacks = KingLookup::generate_king_moves(bb);
        let expected = BitBoard::from_square(Square::B7) | BitBoard::from_square(Square::A7) | BitBoard::from_square(Square::B8);

        assert_eq!(attacks, expected);
    }

    #[test]
    fn test_king_lookup() {
        let lookup = KingLookup::new();
        let bb = BitBoard::from_square(Square::A8);
        let moves = lookup.get_king_moves_bb(bb);
        let expected = BitBoard::from_square(Square::B7) | BitBoard::from_square(Square::A7) | BitBoard::from_square(Square::B8);

        assert_eq!(moves, expected);
    }

    #[test]
    fn test_generate_pseudo_legal_king_moves() {
        let mut moves = vec![];
        let position = Position::from_str("8/8/8/8/4R3/4K3/4r3/8 w - - 0 1").unwrap();
        let lookup = KingLookup::new();

        lookup.generate_pseudo_legal_king_moves(&position, &mut moves);

        moves.iter().for_each(|m| -> () {
            println!("{}", m.to_uci());
        });

        assert_eq!(moves.len(), 7);
    }
}
