pub type Square = usize; // 0-63 representing squares on the chessboard

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
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
    pub castling_rights: String,
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
