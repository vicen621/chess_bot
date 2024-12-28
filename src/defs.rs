use crate::pieces::PieceType;

pub type CastlingRights = u8;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opposite(&self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }

    pub fn to_index(&self) -> usize {
        *self as usize
    }
}

pub struct Castling;
impl Castling {
    pub const NO_CASTLING: CastlingRights = 0;
    pub const WHITE_KING_SIDE: CastlingRights = 1;
    pub const WHITE_QUEEN_SIDE: CastlingRights = 2;
    pub const BLACK_KING_SIDE: CastlingRights = 4;
    pub const BLACK_QUEEN_SIDE: CastlingRights = 8;

    pub const KING_SIDE: CastlingRights = Self::BLACK_KING_SIDE | Self::WHITE_KING_SIDE;
    pub const QUEEN_SIDE: CastlingRights = Self::BLACK_QUEEN_SIDE | Self::WHITE_QUEEN_SIDE;
    pub const WHITE_CASTLING: CastlingRights = Self::WHITE_KING_SIDE | Self::WHITE_QUEEN_SIDE;
    pub const BLACK_CASTLING: CastlingRights = Self::BLACK_KING_SIDE | Self::BLACK_QUEEN_SIDE;
    pub const ANY_CASTLING: CastlingRights = Self::BLACK_CASTLING | Self::WHITE_CASTLING;
}

#[repr(usize)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Square {
    A1,
    B1,
    C1,
    D1,
    E1,
    F1,
    G1,
    H1,
    A2,
    B2,
    C2,
    D2,
    E2,
    F2,
    G2,
    H2,
    A3,
    B3,
    C3,
    D3,
    E3,
    F3,
    G3,
    H3,
    A4,
    B4,
    C4,
    D4,
    E4,
    F4,
    G4,
    H4,
    A5,
    B5,
    C5,
    D5,
    E5,
    F5,
    G5,
    H5,
    A6,
    B6,
    C6,
    D6,
    E6,
    F6,
    G6,
    H6,
    A7,
    B7,
    C7,
    D7,
    E7,
    F7,
    G7,
    H7,
    A8,
    B8,
    C8,
    D8,
    E8,
    F8,
    G8,
    H8,
}

impl Square {
    pub fn from_algebraic(algebraic: &str) -> Self {
        let file = algebraic.chars().nth(0).unwrap() as u8 - b'a';
        let rank = algebraic.chars().nth(1).unwrap().to_digit(10).unwrap() as u8 - 1;
        Self::from_file_rank(file as usize, rank as usize)
    }

    pub fn from_file_rank(file: usize, rank: usize) -> Self {
        let index = rank * 8 + file;
        Self::from_index(index)
    }

    pub fn from_index(index: usize) -> Self {
        unsafe { std::mem::transmute(index) }
    }

    pub fn to_algebraic(&self) -> String {
        let index = *self as u8;
        let rank = index / 8 + 1;
        let file = index % 8;
        let rank = (rank + b'1') as char;
        let file = (file + b'A') as char;
        format!("{}{}", file, rank)
    }

    pub fn to_index(&self) -> usize {
        *self as usize
    }
}

pub struct Move {
    pub from: Square,
    pub to: Square,
    pub promotion: Option<PieceType>,
}

impl Move {
    pub fn new(from: Square, to: Square, promotion: Option<PieceType>) -> Self {
        Self {
            from,
            to,
            promotion,
        }
    }

    pub fn from_uci(uci: &str) -> Self {
        let from = Square::from_algebraic(&uci[0..2]);
        let to = Square::from_algebraic(&uci[2..4]);
        let promotion = if uci.len() == 5 {
            Some(PieceType::from_char(uci.chars().nth(4).unwrap()))
        } else {
            None
        };
        Self::new(from, to, promotion)
    }
}

pub struct Direction;
impl Direction {
    pub const UP: i8 = 8;
    pub const DOWN: i8 = -8;
    pub const RIGHT: i8 = 1;
    pub const LEFT: i8 = -1;
    pub const UP_RIGHT: i8 = Self::UP + Self::RIGHT;
    pub const UP_LEFT: i8 = Self::UP + Self::LEFT;
    pub const DOWN_RIGHT: i8 = Self::DOWN + Self::RIGHT;
    pub const DOWN_LEFT: i8 = Self::DOWN + Self::LEFT;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_squares() {
        assert_eq!(Square::from_algebraic("a1"), Square::A1);
        assert_eq!(Square::from_algebraic("h8"), Square::H8);
        assert_eq!(Square::from_algebraic("e4"), Square::E4);
        assert_eq!(Square::from_algebraic("d5"), Square::D5);
        assert_eq!(Square::from_algebraic("c6"), Square::C6);
        assert_eq!(Square::from_algebraic("b7"), Square::B7);
        assert_eq!(Square::from_algebraic("g2"), Square::G2);
        assert_eq!(Square::from_algebraic("f3"), Square::F3);
    }

    #[test]
    fn test_castling() {
        assert_eq!(Castling::WHITE_KING_SIDE, 1);
        assert_eq!(Castling::WHITE_QUEEN_SIDE, 2);
        assert_eq!(Castling::BLACK_KING_SIDE, 4);
        assert_eq!(Castling::BLACK_QUEEN_SIDE, 8);
        assert_eq!(Castling::NO_CASTLING, 0);
        assert_eq!(Castling::KING_SIDE, 5);
        assert_eq!(Castling::QUEEN_SIDE, 10);
        assert_eq!(Castling::WHITE_CASTLING, 3);
        assert_eq!(Castling::BLACK_CASTLING, 12);
        assert_eq!(Castling::ANY_CASTLING, 15);
    }

    #[test]
    fn test_square_to_index() {
        assert_eq!(Square::A1.to_index(), 0);
        assert_eq!(Square::H8.to_index(), 63);
        assert_eq!(Square::E4.to_index(), 28);
    }
}
