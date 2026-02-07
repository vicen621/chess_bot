pub type Square = usize; // 0-63 representing squares on the chessboard

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opposite(&self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Piece {
    pub color: Color,
    pub piece_type: PieceType,
}

impl Piece {
    pub fn new(color: Color, piece_type: PieceType) -> Self {
        Piece { color, piece_type }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    pub squares: [Option<Piece>; 64],
    pub turn: Color,
    pub castling_rights: CastlingRights,
    pub en_passant_target: Option<Square>,
}

pub struct Move {
    pub from: Square,
    pub to: Square,
    pub promotion: Option<PieceType>,
}

impl Move {
    pub fn new(from: Square, to: Square) -> Self {
        Move { from, to, promotion: None }
    }

    pub fn with_promotion(from: Square, to: Square, promotion: PieceType) -> Self {
        Move {
            from,
            to,
            promotion: Some(promotion),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct CastlingRights {
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool,
}

impl CastlingRights {
    pub fn new(
        white_kingside: bool,
        white_queenside: bool,
        black_kingside: bool,
        black_queenside: bool,
    ) -> Self {
        CastlingRights {
            white_kingside,
            white_queenside,
            black_kingside,
            black_queenside,
        }
    }

    pub fn from_fen(fen: &str) -> Self {
        CastlingRights {
            white_kingside: fen.contains('K'),
            white_queenside: fen.contains('Q'),
            black_kingside: fen.contains('k'),
            black_queenside: fen.contains('q'),
        }
    }

    pub fn can_castle(&self, color: Color, kingside: bool) -> bool {
        match (color, kingside) {
            (Color::White, true) => self.white_kingside,
            (Color::White, false) => self.white_queenside,
            (Color::Black, true) => self.black_kingside,
            (Color::Black, false) => self.black_queenside,
        }
    }

    pub fn remove_castling_rights(&mut self, color: Color, kingside: bool) {
        match (color, kingside) {
            (Color::White, true) => self.white_kingside = false,
            (Color::White, false) => self.white_queenside = false,
            (Color::Black, true) => self.black_kingside = false,
            (Color::Black, false) => self.black_queenside = false,
        }
    }
}
