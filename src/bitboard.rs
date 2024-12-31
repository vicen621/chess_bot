use crate::{chess_move::Direction, defs::Square};

/// An empty bitboard.  It is sometimes useful to use !EMPTY to get the universe of squares.
///
/// ```
///     assert_eq!(EMPTY.count(), 0);
///
///     assert_eq!((!EMPTY).count(), 64);
///
pub type BitBoard = u64;
pub const EMPTY: BitBoard = 0;
pub const RANK_1: BitBoard = 0xFF;
pub const RANK_2: BitBoard = RANK_1 << 8;
pub const RANK_3: BitBoard = RANK_2 << 8;
pub const RANK_4: BitBoard = RANK_3 << 8;
pub const RANK_5: BitBoard = RANK_4 << 8;
pub const RANK_6: BitBoard = RANK_5 << 8;
pub const RANK_7: BitBoard = RANK_6 << 8;
pub const RANK_8: BitBoard = RANK_7 << 8;

pub const FILE_A: BitBoard = 0x0101_0101_0101_0101;
pub const FILE_B: BitBoard = FILE_A << 1;
pub const FILE_C: BitBoard = FILE_A << 2;
pub const FILE_D: BitBoard = FILE_A << 3;
pub const FILE_E: BitBoard = FILE_A << 4;
pub const FILE_F: BitBoard = FILE_A << 5;
pub const FILE_G: BitBoard = FILE_A << 6;
pub const FILE_H: BitBoard = FILE_A << 7;

pub trait BitBoardMethods {
    fn set(file: usize, rank: usize) -> BitBoard;
    fn from_square(sq: Square) -> Self;
    fn from_maybe_square(sq: Option<Square>) -> Option<BitBoard>;
    fn to_square(&self) -> Square;
    fn count(&self) -> u32;
    fn reverse_colors(&self) -> Self;
    fn to_size(&self, rightshift: u8) -> usize;
    fn format(&self) -> String;
    fn shift(&self, direction: i8) -> Self;
}

impl BitBoardMethods for BitBoard {
    /// Construct a new `BitBoard` with a particular `Square` set
    fn set(file: usize, rank: usize) -> BitBoard {
        BitBoard::from_square(Square::from_file_rank(file, rank))
    }

    /// Construct a new `BitBoard` with a particular `Square` set
    fn from_square(sq: Square) -> BitBoard {
        1u64 << sq.to_index()
    }

    /// Convert an `Option<Square>` to an `Option<BitBoard>`
    fn from_maybe_square(sq: Option<Square>) -> Option<BitBoard> {
        sq.map(|s| BitBoard::from_square(s))
    }

    /// Convert a `BitBoard` to a `Square`.  This grabs the least-significant `Square`
    fn to_square(&self) -> Square {
        Square::from_index(self.trailing_zeros() as usize)
    }

    /// Count the number of `Squares` set in this `BitBoard`
    fn count(&self) -> u32 {
        self.count_ones()
    }

    /// Reverse this `BitBoard`.  Look at it from the opponents perspective.
    fn reverse_colors(&self) -> BitBoard {
        self.swap_bytes()
    }

    /// Convert this `BitBoard` to a `usize` (for table lookups)
    fn to_size(&self, rightshift: u8) -> usize {
        (self >> rightshift) as usize
    }

    fn format(&self) -> String {
        let mut s: String = "".to_owned();

        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = Square::from_file_rank(file, rank);
                let mask = BitBoard::from_square(square);
                if self & mask != 0 {
                    s.push_str("1 ");
                } else {
                    s.push_str(". ");
                }
            }
            s.push_str("\n");
        }

        s
    }

    fn shift(&self, direction: i8) -> BitBoard {
        match direction {
            Direction::UP => self << 8,
            Direction::DOWN => self >> 8,
            Direction::LEFT => (self & !FILE_A) >> 1,
            Direction::RIGHT => (self & !FILE_H) << 1,
            Direction::UP_LEFT => (self & !FILE_A) << 7,
            Direction::UP_RIGHT => (self & !FILE_H) << 9,
            Direction::DOWN_LEFT => (self & !FILE_A) >> 9,
            Direction::DOWN_RIGHT => (self & !FILE_H) >> 7,
            i8::MIN..=0 => self >> -direction as u8,
            _ => self << direction as u8,
        }
    }
}

pub struct BitboardIterator {
    bb: BitBoard,
}

impl Iterator for BitboardIterator {
    type Item = (Square, BitBoard);

    fn next(&mut self) -> Option<(Square, BitBoard)> {
        if self.bb == 0 {
            return None;
        }

        let square = Square::from_index(self.bb.trailing_zeros() as usize);
        self.bb &= self.bb - 1;
        Some((square, self.bb))
    }
}

pub trait PieceItr {
    fn iter(&self) -> BitboardIterator;
}

impl PieceItr for BitBoard {
    fn iter(&self) -> BitboardIterator {
        BitboardIterator { bb: *self }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_popcnt() {
        let bb = 0x0000_0000_0000_FF00;
        assert_eq!(bb.count(), 8);
    }

    #[test]
    fn test_to_square() {
        let bb = 0x0000_0000_0000_0001;
        assert_eq!(bb.to_square(), Square::A1);
    }

    #[test]
    fn test_from_square() {
        let bb = BitBoard::from_square(Square::A1);
        assert_eq!(bb, 0x0000_0000_0000_0001);
    }

    #[test]
    fn test_from_maybe_square() {
        let bb = BitBoard::from_maybe_square(Some(Square::A1));
        assert_eq!(bb, Some(0x0000_0000_0000_0001));
    }

    #[test]
    fn test_from_maybe_square_none() {
        let bb = BitBoard::from_maybe_square(None);
        assert_eq!(bb, None);
    }

    #[test]
    fn test_to_size() {
        let bb = 0x0000_0000_0000_0008;
        assert_eq!(bb.to_size(0), 8);
        assert_eq!(bb.to_size(1), 4);
    }

    #[test]
    fn test_reverse_colors() {
        let bb = 0x0000_0000_0000_FF00;
        let bb_reversed = bb.reverse_colors();
        assert_eq!(bb_reversed, 0x00FF_0000_0000_0000);
    }

    #[test]
    fn test_shift() {
        let bb = BitBoard::from_square(Square::B3);
        assert_eq!(bb.shift(Direction::UP), BitBoard::from_square(Square::B4));
        assert_eq!(bb.shift(Direction::DOWN), BitBoard::from_square(Square::B2));
        assert_eq!(bb.shift(Direction::LEFT), BitBoard::from_square(Square::A3));
        assert_eq!(bb.shift(Direction::RIGHT), BitBoard::from_square(Square::C3));
        assert_eq!(bb.shift(Direction::UP_LEFT), BitBoard::from_square(Square::A4));
        assert_eq!(bb.shift(Direction::UP_RIGHT), BitBoard::from_square(Square::C4));
        assert_eq!(bb.shift(Direction::DOWN_LEFT), BitBoard::from_square(Square::A2));
        assert_eq!(bb.shift(Direction::DOWN_RIGHT), BitBoard::from_square(Square::C2));
    }

    #[test]
    fn test_shift_borders() {
        let bb = BitBoard::from_square(Square::A8);
        assert_eq!(bb.shift(Direction::UP), EMPTY);
        assert_eq!(bb.shift(Direction::DOWN), BitBoard::from_square(Square::A7));
        assert_eq!(bb.shift(Direction::LEFT), EMPTY);
        assert_eq!(bb.shift(Direction::RIGHT), BitBoard::from_square(Square::B8));
        assert_eq!(bb.shift(Direction::UP_LEFT), EMPTY);
        assert_eq!(bb.shift(Direction::UP_RIGHT), EMPTY);
        assert_eq!(bb.shift(Direction::DOWN_LEFT), EMPTY);
        assert_eq!(bb.shift(Direction::DOWN_RIGHT), BitBoard::from_square(Square::B7));

    }

    #[test]
    fn test_files_bb() {
        assert_eq!(FILE_A, 0x0101_0101_0101_0101);
        assert_eq!(FILE_B, 0x0202_0202_0202_0202);
        assert_eq!(FILE_C, 0x0404_0404_0404_0404);
        assert_eq!(FILE_D, 0x0808_0808_0808_0808);
        assert_eq!(FILE_E, 0x1010_1010_1010_1010);
        assert_eq!(FILE_F, 0x2020_2020_2020_2020);
        assert_eq!(FILE_G, 0x4040_4040_4040_4040);
        assert_eq!(FILE_H, 0x8080_8080_8080_8080);
    }

    #[test]
    fn test_ranks_bb() {
        assert_eq!(RANK_1, 0x0000_0000_0000_00FF);
        assert_eq!(RANK_2, 0x0000_0000_0000_FF00);
        assert_eq!(RANK_3, 0x0000_0000_00FF_0000);
        assert_eq!(RANK_4, 0x0000_0000_FF00_0000);
        assert_eq!(RANK_5, 0x0000_00FF_0000_0000);
        assert_eq!(RANK_6, 0x0000_FF00_0000_0000);
        assert_eq!(RANK_7, 0x00FF_0000_0000_0000);
        assert_eq!(RANK_8, 0xFF00_0000_0000_0000);
    }
}
