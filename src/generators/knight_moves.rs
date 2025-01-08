use crate::{bitboard::{BitBoard, BitBoardMethods, PieceItr, EMPTY, FILE_A, FILE_B, FILE_G, FILE_H}, chess_move::{Move, MoveExtractor, MoveType}, defs::Square, pieces::PieceType, position::Position};

struct KnightLookup {
    moves: [BitBoard; 64],
}

impl KnightLookup {
    pub fn new() -> Self {
        let mut moves = [EMPTY; 64];

        for i in 0..64 {
            let knight = BitBoard::from_square(Square::from_index(i));
            moves[i] = Self::generate_knight_moves(knight);
        }

        Self { moves }
    }

    pub fn generate_pseudo_legal_knight_moves(&self, position: &Position, moves: &mut Vec<Move>) {
        let knights = position.get_pieces_color_type(position.get_side_to_move(), PieceType::Knight);

        for (knight,_) in knights.iter() {
            self.generate_knight_attacks(position, moves, knight);
            self.generate_knight_quiet_moves(position, moves, knight);
        }
    }
}

impl KnightLookup {
    fn generate_knight_attacks(&self, position: &Position, moves: &mut Vec<Move>, knight: Square) {
        let color = position.get_side_to_move();
        let their_king = position.get_pieces_color_type(!color, PieceType::King);
        let captures = position.get_pieces_color(!color) & !their_king;

        let knight_captures = self.get_knight_moves(knight) & captures;

        MoveExtractor::extract_moves(knight, knight_captures, moves, MoveType::Capture);
    }

    fn generate_knight_quiet_moves(&self, position: &Position, moves: &mut Vec<Move>, knight: Square) {
        let empty_squares = !position.get_all_pieces();

        let quiet_moves = self.get_knight_moves(knight) & empty_squares;

        MoveExtractor::extract_moves(knight, quiet_moves, moves, MoveType::Quiet);
    }

    fn get_knight_moves(&self, square: Square) -> BitBoard {
        self.moves[square.to_index()]
    }

    fn upUpRight(bb: BitBoard) -> BitBoard {
        (bb & !FILE_H) << 17
    }

    fn upUpLeft(bb: BitBoard) -> BitBoard {
        (bb & !FILE_A) << 15
    }

    fn downDownRight(bb: BitBoard) -> BitBoard {
        (bb & !FILE_H) >> 15
    }

    fn downDownLeft(bb: BitBoard) -> BitBoard {
        (bb & !FILE_A) >> 17
    }

    fn upLeftLeft(bb: BitBoard) -> BitBoard {
        (bb & !FILE_A & !FILE_B) << 6
    }

    fn downLeftLeft(bb: BitBoard) -> BitBoard {
        (bb & !FILE_A & !FILE_B) >> 10
    }

    fn upRightRight(bb: BitBoard) -> BitBoard {
        (bb & !FILE_H & !FILE_G) << 10
    }

    fn downRightRight(bb: BitBoard) -> BitBoard {
        (bb & !FILE_H & !FILE_G) >> 6
    }

    fn generate_knight_moves(bb: BitBoard) -> BitBoard {
        Self::upUpRight(bb) | Self::upRightRight(bb) | Self::downRightRight(bb) | Self::downDownRight(bb) | Self::downDownLeft(bb) | Self::downLeftLeft(bb) | Self::upLeftLeft(bb) | Self::upUpLeft(bb)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_generate_knight_moves() {
        let bb = BitBoard::from_square(Square::A8);
        let attacks = KnightLookup::generate_knight_moves(bb);
        let expected = BitBoard::from_square(Square::B6) | BitBoard::from_square(Square::C7);

        assert_eq!(attacks, expected);
    }

    #[test]
    fn test_generate_pseudo_legal_knight_moves() {
        let position = Position::from_str("8/8/8/8/8/8/3N4/8 w - - 0 1").ok().unwrap();
        let mut moves = vec![];

        let knight_lookup = KnightLookup::new();
        knight_lookup.generate_pseudo_legal_knight_moves(&position, &mut moves);

        moves.iter().for_each(|f| println!("{}", f.to_uci()));
        assert_eq!(moves.len(), 6);
    }
}
