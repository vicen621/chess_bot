use crate::{bitboard::{BitBoard, BitBoardMethods, EMPTY}, chess_move::Direction, defs::Square};

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

    pub fn get_king_moves(&self, square: Square) -> BitBoard {
        self.moves[square.to_index()]
    }

    pub fn get_king_moves_bb(&self, bb: BitBoard) -> BitBoard {
        self.moves[bb.trailing_zeros() as usize]
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
}

#[cfg(test)]
mod tests {
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
    fn test_king_lookup_square() {
        let lookup = KingLookup::new();
        let square = Square::A8;
        let moves = lookup.get_king_moves(square);
        let expected = BitBoard::from_square(Square::B7) | BitBoard::from_square(Square::A7) | BitBoard::from_square(Square::B8);

        assert_eq!(moves, expected);
    }
}
