use crate::fen_parser::{FenError, FenParser};

const FEN_STARTING_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Debug, PartialEq)]
pub struct Piece {
    piece_type: PieceType,
    color: Color
}

impl Piece {
    pub fn new(piece_type: PieceType, color: Color) -> Piece {
        Piece {
            piece_type,
            color
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PieceType {
    Empty,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

impl PieceType {
    pub fn from_char(c: char) -> Result<PieceType, FenError> {
        let c = c.to_uppercase().next().unwrap();
        match c {
            'P' => Ok(PieceType::Pawn),
            'N' => Ok(PieceType::Knight),
            'B' => Ok(PieceType::Bishop),
            'R' => Ok(PieceType::Rook),
            'Q' => Ok(PieceType::Queen),
            'K' => Ok(PieceType::King),
            _ => Err(FenError::InvalidPiece(c))
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Color {
    White,
    Black
}

impl Color {
    pub fn opposite(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White
        }
    }

    pub fn from_char(s: char) -> Result<Color, FenError> {
        match s {
            'w' => Ok(Color::White),
            'b' => Ok(Color::Black),
            _ => Err(FenError::InvalidFormat)
        }
    }

    pub fn from_fen(c: char) -> Color {
        if c.is_uppercase() {
            Color::White
        } else {
            Color::Black
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct PiecePositions {
    white: BitBoards,
    black: BitBoards
}

impl PiecePositions {
    pub fn new() -> PiecePositions {
        PiecePositions {
            white: BitBoards::empty(),
            black: BitBoards::empty()
        }
    }

    pub fn set_piece(&mut self, piece_type: &PieceType, color: &Color, square: &Square) {
        match color {
            Color::White => self.white.set_piece(piece_type, square),
            Color::Black => self.black.set_piece(piece_type, square)
        }
    }

    pub fn get_white(&self) -> &BitBoards {
        &self.white
    }

    pub fn get_black(&self) -> &BitBoards {
        &self.black
    }
}

#[derive(Debug, PartialEq)]
pub struct BitBoards {
    pawns: u64,
    knights: u64,
    bishops: u64,
    rooks: u64,
    queens: u64,
    king: u64
}

impl BitBoards {
    fn empty() -> BitBoards {
        BitBoards {
            pawns: 0,
            knights: 0,
            bishops: 0,
            rooks: 0,
            queens: 0,
            king: 0
        }
    }

    pub fn set_piece(&mut self, piece_type: &PieceType, square: &Square) {
        let index = square.0;
        match piece_type {
            PieceType::Pawn => self.pawns |= 1 << index,
            PieceType::Knight => self.knights |= 1 << index,
            PieceType::Bishop => self.bishops |= 1 << index,
            PieceType::Rook => self.rooks |= 1 << index,
            PieceType::Queen => self.queens |= 1 << index,
            PieceType::King => self.king |= 1 << index,
            _ => {}
        }
    }

    pub fn get_pawns(&self) -> u64 {
        self.pawns
    }

    pub fn get_knights(&self) -> u64 {
        self.knights
    }

    pub fn get_bishops(&self) -> u64 {
        self.bishops
    }

    pub fn get_rooks(&self) -> u64 {
        self.rooks
    }

    pub fn get_queens(&self) -> u64 {
        self.queens
    }

    pub fn get_king(&self) -> u64 {
        self.king
    }
}

#[derive(Debug, PartialEq)]
pub struct CastlingRights {
    white_king_side: bool,
    white_queen_side: bool,
    black_king_side: bool,
    black_queen_side: bool
}

impl CastlingRights {
    pub fn new(white_king_side: bool, white_queen_side: bool, black_king_side: bool, black_queen_side: bool) -> CastlingRights {
        CastlingRights {
            white_king_side,
            white_queen_side,
            black_king_side,
            black_queen_side
        }
    }

    pub fn from_str(s: &str) -> CastlingRights {
        let white_king_side = s.contains("K");
        let white_queen_side = s.contains("Q");
        let black_king_side = s.contains("k");
        let black_queen_side = s.contains("q");
        CastlingRights::new(white_king_side, white_queen_side, black_king_side, black_queen_side)
    }
}

#[derive(Debug, PartialEq)]
pub struct Square(pub u8);

impl Square {
    pub fn from_algebraic(s: &str) -> Result<Square, FenError> {
        let mut chars = s.chars();
        let file = chars.next().unwrap();
        let rank = chars.next().unwrap().to_digit(10).unwrap() as u8;

        if file < 'a' || file > 'h' || rank < 1 || rank > 8 {
            return Err(FenError::InvalidFormat);
        }

        let square_index = (rank - 1) * 8 + (file as u8 - 'a' as u8);

        Ok(Square(square_index))
    }

    pub fn to_algebraic(&self) -> String {
        let file = self.0 % 8;
        let rank = self.0 / 8;
        format!("{}{}", (b'a' + file) as char, rank + 1)
    }
}

#[derive(Debug, PartialEq)]
pub struct Board {
    pieces: PiecePositions,
    pieces_types: [PieceType; 64],
    turn: Color,
    castling_rights: CastlingRights,
    en_passant: Option<Square>,
    halfmove_clock: u32,
    fullmove_counter: u32
}

impl Board {
    pub fn new(pieces: PiecePositions, pieces_types: [PieceType; 64], turn: Color, castling_rights: CastlingRights, en_passant: Option<Square>, halfmove_clock: u32, fullmove_counter: u32) -> Board {
        Board {
            pieces,
            pieces_types,
            turn,
            castling_rights,
            en_passant,
            halfmove_clock,
            fullmove_counter
        }
    }

    pub fn from_fen(fen: &str) -> Result<Board, FenError> {
        FenParser::parse(fen)
    }

    pub fn starting_position() -> Board {
        Board::from_fen(FEN_STARTING_POSITION).unwrap()
    }

    pub fn get_pieces(&self) -> &PiecePositions {
        &self.pieces
    }

    pub fn get_piece_types(&self) -> &[PieceType; 64] {
        &self.pieces_types
    }

    pub fn get_turn(&self) -> &Color {
        &self.turn
    }

    pub fn get_castling_rights(&self) -> &CastlingRights {
        &self.castling_rights
    }

    pub fn get_en_passant(&self) -> &Option<Square> {
        &self.en_passant
    }

    pub fn get_halfmove_clock(&self) -> u32 {
        self.halfmove_clock
    }

    pub fn get_fullmove_counter(&self) -> u32 {
        self.fullmove_counter
    }

    /// Sets a piece in the given square.
    pub fn set_piece(&mut self, piece_type: &PieceType, color: &Color, square: &Square) {
        self.pieces.set_piece(piece_type, color, square);
    }

    pub fn get_legal_moves(&self, moves: Vec<Move>) -> Vec<Move> {
        todo!();
    }
}

struct Move {
    from: Square,
    to: Square,
    promotion: Option<PieceType>
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piece_type_from_char() {
        assert_eq!(PieceType::from_char('P').unwrap(), PieceType::Pawn);
        assert_eq!(PieceType::from_char('N').unwrap(), PieceType::Knight);
        assert_eq!(PieceType::from_char('R').unwrap(), PieceType::Rook);
        assert_eq!(PieceType::from_char('Q').unwrap(), PieceType::Queen);
        assert_eq!(PieceType::from_char('B').unwrap(), PieceType::Bishop);
        assert_eq!(PieceType::from_char('K').unwrap(), PieceType::King);
        assert!(PieceType::from_char('a').is_err());
    }

    #[test]
    fn test_color_from_char() {
        assert_eq!(Color::from_char('w').unwrap(), Color::White);
        assert_eq!(Color::from_char('b').unwrap(), Color::Black);
        assert!(Color::from_char('a').is_err());
    }

    #[test]
    fn test_color_from_fen() {
        assert_eq!(Color::from_fen('P'), Color::White);
        assert_eq!(Color::from_fen('p'), Color::Black);
    }

    #[test]
    fn test_color_opposite() {
        assert_eq!(Color::White.opposite(), Color::Black);
        assert_eq!(Color::Black.opposite(), Color::White);
    }

    #[test]
    fn test_square_from_algebraic() {
        assert_eq!(Square::from_algebraic("a1"), Ok(Square(0)));
        assert_eq!(Square::from_algebraic("h8"), Ok(Square(63)));
        assert!(Square::from_algebraic("i1").is_err());
    }

    #[test]
    fn test_square_to_algebraic() {
        assert_eq!(Square(0).to_algebraic(), "a1");
        assert_eq!(Square(63).to_algebraic(), "h8");
    }

    #[test]
    fn test_castling_rights_from_str() {
        let castling_rights = CastlingRights::from_str("KQkq");
        assert_eq!(castling_rights.white_king_side, true);
        assert_eq!(castling_rights.white_queen_side, true);
        assert_eq!(castling_rights.black_king_side, true);
        assert_eq!(castling_rights.black_queen_side, true);
    }
}
