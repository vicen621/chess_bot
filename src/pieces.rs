use crate::defs::Color;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piece_type() {
        assert_eq!(PieceType::from_char('K'), PieceType::King);
        assert_eq!(PieceType::from_char('Q'), PieceType::Queen);
        assert_eq!(PieceType::from_char('R'), PieceType::Rook);
        assert_eq!(PieceType::from_char('B'), PieceType::Bishop);
        assert_eq!(PieceType::from_char('N'), PieceType::Knight);
        assert_eq!(PieceType::from_char('P'), PieceType::Pawn);
        assert_eq!(PieceType::from_char('x'), PieceType::None);
    }#[test]
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
}
