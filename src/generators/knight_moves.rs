use crate::{bitboard::{BitBoard, BitBoardMethods, EMPTY, FILE_A, FILE_B, FILE_G, FILE_H}, chess_move::{Move, MoveExtractor, MoveType}, defs::Square, position::Position};

// TODO: Ver como usar los movimientos pregenerados con todos los caballos a la vez
// cuando a la posición le pido los caballos de color blanco me devuelve todos los caballos blancos en el tablero
// pudiendo ser 1 o más caballos. Entonces necesito los movimientos de todos los caballos blancos a la vez.
// READ: https://josherv.in/2021/03/19/chess-1/#:~:text=Sliding%20Piece%20Generation%3A%20Classical%20Approach
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

    pub fn get_knight_moves(&self, square: Square) -> BitBoard {
        self.moves[square.to_index()]
    }

    pub fn get_knight_moves_bb(&self, bb: BitBoard) -> BitBoard {
        self.moves[bb.trailing_zeros() as usize]
    }
}

impl KnightLookup {
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
    use super::*;

    #[test]
    fn test_generate_knight_moves() {
        let bb = BitBoard::from_square(Square::A8);
        let attacks = KnightLookup::generate_knight_moves(bb);
        let expected = BitBoard::from_square(Square::B6) | BitBoard::from_square(Square::C7);

        assert_eq!(attacks, expected);
    }

    #[test]
    fn test_knight_lookup() {
        let lookup = KnightLookup::new();
        let bb = BitBoard::from_square(Square::A8);
        let moves = lookup.get_knight_moves_bb(bb);
        let expected = BitBoard::from_square(Square::B6) | BitBoard::from_square(Square::C7);

        assert_eq!(moves, expected);
    }
}
