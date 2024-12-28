pub type CastlingRights = u8;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
    None,
}
impl PieceType {
    pub fn from_char(c: char) -> PieceType {
        let c = c.to_uppercase().next().unwrap();
        match c {
            'K' => Self::King,
            'Q' => Self::Queen,
            'R' => Self::Rook,
            'B' => Self::Bishop,
            'N' => Self::Knight,
            'P' => Self::Pawn,
            _ => Self::None,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Self::King => 'k',
            Self::Queen => 'q',
            Self::Rook => 'r',
            Self::Bishop => 'b',
            Self::Knight => 'n',
            Self::Pawn => 'p',
            Self::None => '.',
        }
    }

    pub fn to_index(&self) -> usize {
        *self as usize
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Piece {
    piece_type: PieceType,
    color: Color,
}

impl Piece {
    pub const EMPTY: Piece = Piece { piece_type: PieceType::None, color: Color::White };
    pub const WHITE_PAWN: Piece = Piece { piece_type: PieceType::Pawn, color: Color::White };
    pub const WHITE_KNIGHT: Piece = Piece { piece_type: PieceType::Knight, color: Color::White };
    pub const WHITE_BISHOP: Piece = Piece { piece_type: PieceType::Bishop, color: Color::White };
    pub const WHITE_ROOK: Piece = Piece { piece_type: PieceType::Rook, color: Color::White };
    pub const WHITE_QUEEN: Piece = Piece { piece_type: PieceType::Queen, color: Color::White };
    pub const WHITE_KING: Piece = Piece { piece_type: PieceType::King, color: Color::White };
    pub const BLACK_PAWN: Piece = Piece { piece_type: PieceType::Pawn, color: Color::Black };
    pub const BLACK_KNIGHT: Piece = Piece { piece_type: PieceType::Knight, color: Color::Black };
    pub const BLACK_BISHOP: Piece = Piece { piece_type: PieceType::Bishop, color: Color::Black };
    pub const BLACK_ROOK: Piece = Piece { piece_type: PieceType::Rook, color: Color::Black };
    pub const BLACK_QUEEN: Piece = Piece { piece_type: PieceType::Queen, color: Color::Black };
    pub const BLACK_KING: Piece = Piece { piece_type: PieceType::King, color: Color::Black };

    pub fn new(piece_type: PieceType, color: Color) -> Self {
        Self {
            piece_type,
            color,
        }
    }

    pub fn from_char(c: char) -> Self {
        let piece_type = PieceType::from_char(c);
        let color = if c.is_uppercase() { Color::White } else { Color::Black };
        Self::new(piece_type, color)
    }

    pub fn to_char(&self) -> char {
        let c = self.piece_type.to_char();
        if self.color == Color::White {
            c.to_uppercase().next().unwrap()
        } else {
            c
        }
    }

    pub fn get_piece_type(&self) -> &PieceType {
        &self.piece_type
    }

    pub fn get_color(&self) -> &Color {
        &self.color
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black
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

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Square {
    Empty,
    Occupied(Piece)
}

#[repr(usize)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Squares {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8
}
impl Squares {
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
    pub from: Squares,
    pub to: Squares,
    pub promotion: Option<PieceType>,
}

impl Move {
    pub fn new(from: Squares, to: Squares, promotion: Option<PieceType>) -> Self {
        Self {
            from,
            to,
            promotion,
        }
    }

    pub fn from_uci(uci: &str) -> Self {
        let from = Squares::from_algebraic(&uci[0..2]);
        let to = Squares::from_algebraic(&uci[2..4]);
        let promotion = if uci.len() == 5 {
            Some(PieceType::from_char(uci.chars().nth(4).unwrap()))
        } else {
            None
        };
        Self::new(from, to, promotion)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_squares() {
        assert_eq!(Squares::from_algebraic("a1"), Squares::A1);
        assert_eq!(Squares::from_algebraic("h8"), Squares::H8);
        assert_eq!(Squares::from_algebraic("e4"), Squares::E4);
        assert_eq!(Squares::from_algebraic("d5"), Squares::D5);
        assert_eq!(Squares::from_algebraic("c6"), Squares::C6);
        assert_eq!(Squares::from_algebraic("b7"), Squares::B7);
        assert_eq!(Squares::from_algebraic("g2"), Squares::G2);
        assert_eq!(Squares::from_algebraic("f3"), Squares::F3);
    }

    #[test]
    fn test_piece_type() {
        assert_eq!(PieceType::from_char('K'), PieceType::King);
        assert_eq!(PieceType::from_char('Q'), PieceType::Queen);
        assert_eq!(PieceType::from_char('R'), PieceType::Rook);
        assert_eq!(PieceType::from_char('B'), PieceType::Bishop);
        assert_eq!(PieceType::from_char('N'), PieceType::Knight);
        assert_eq!(PieceType::from_char('P'), PieceType::Pawn);
        assert_eq!(PieceType::from_char('x'), PieceType::None);
    }

    #[test]
    fn test_piece() {
        let piece = Piece::from_char('K');
        assert_eq!(piece.piece_type, PieceType::King);
        assert_eq!(piece.color, Color::White);

        let piece = Piece::from_char('k');
        assert_eq!(piece.piece_type, PieceType::King);
        assert_eq!(piece.color, Color::Black);

        let piece = Piece::from_char('Q');
        assert_eq!(piece.piece_type, PieceType::Queen);
        assert_eq!(piece.color, Color::White);

        let piece = Piece::from_char('q');
        assert_eq!(piece.piece_type, PieceType::Queen);
        assert_eq!(piece.color, Color::Black);

        let piece = Piece::from_char('R');
        assert_eq!(piece.piece_type, PieceType::Rook);
        assert_eq!(piece.color, Color::White);

        let piece = Piece::from_char('r');
        assert_eq!(piece.piece_type, PieceType::Rook);
        assert_eq!(piece.color, Color::Black);

        let piece = Piece::from_char('B');
        assert_eq!(piece.piece_type, PieceType::Bishop);
        assert_eq!(piece.color, Color::White);

        let piece = Piece::from_char('b');
        assert_eq!(piece.piece_type, PieceType::Bishop);
        assert_eq!(piece.color, Color::Black);

        let piece = Piece::from_char('N');
        assert_eq!(piece.piece_type, PieceType::Knight);
        assert_eq!(piece.color, Color::White);

        let piece = Piece::from_char('n');
        assert_eq!(piece.piece_type, PieceType::Knight);
        assert_eq!(piece.color, Color::Black);

        let piece = Piece::from_char('P');
        assert_eq!(piece.piece_type, PieceType::Pawn);
        assert_eq!(piece.color, Color::White);

        let piece = Piece::from_char('p');
        assert_eq!(piece.piece_type, PieceType::Pawn);
        assert_eq!(piece.color, Color::Black);
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
        assert_eq!(Squares::A1.to_index(), 0);
        assert_eq!(Squares::H8.to_index(), 63);
        assert_eq!(Squares::E4.to_index(), 28);
    }
}
