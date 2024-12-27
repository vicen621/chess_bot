use crate::fen_parser::{FenError, FenParser};

const FEN_STARTING_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Debug, PartialEq)]
pub struct Piece {
    piece_type: PieceType,
    color: Color,
}

impl Piece {
    pub fn new(piece_type: PieceType, color: Color) -> Piece {
        Piece { piece_type, color }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PieceType {
    None,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
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
            _ => Err(FenError::InvalidPiece(c)),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opposite(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    pub fn from_char(s: char) -> Result<Color, FenError> {
        match s {
            'w' => Ok(Color::White),
            'b' => Ok(Color::Black),
            _ => Err(FenError::InvalidFormat),
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
    white: Bitboards,
    black: Bitboards,
}

impl PiecePositions {
    pub fn new() -> PiecePositions {
        PiecePositions {
            white: Bitboards::empty(),
            black: Bitboards::empty(),
        }
    }

    pub fn set_piece(&mut self, piece_type: &PieceType, color: &Color, square: &Square) {
        match color {
            Color::White => self.white.set_piece(piece_type, square),
            Color::Black => self.black.set_piece(piece_type, square),
        }
    }

    fn make_move(&mut self, piece_type: &PieceType, color: &Color, mv: &Move) {
        match color {
            Color::White => self.white.make_move(piece_type, mv),
            Color::Black => self.black.make_move(piece_type, mv),
        }
    }

    pub fn get_pawns(&self, color: &Color) -> u64 {
        match color {
            Color::White => self.white.get_pawns(),
            Color::Black => self.black.get_pawns(),
        }
    }

    pub fn get_knights(&self, color: &Color) -> u64 {
        match color {
            Color::White => self.white.get_knights(),
            Color::Black => self.black.get_knights(),
        }
    }

    pub fn get_bishops(&self, color: &Color) -> u64 {
        match color {
            Color::White => self.white.get_bishops(),
            Color::Black => self.black.get_bishops(),
        }
    }

    pub fn get_rooks(&self, color: &Color) -> u64 {
        match color {
            Color::White => self.white.get_rooks(),
            Color::Black => self.black.get_rooks(),
        }
    }

    pub fn get_queens(&self, color: &Color) -> u64 {
        match color {
            Color::White => self.white.get_queens(),
            Color::Black => self.black.get_queens(),
        }
    }

    pub fn get_king(&self, color: &Color) -> u64 {
        match color {
            Color::White => self.white.get_king(),
            Color::Black => self.black.get_king(),
        }
    }

    pub fn get_all(&self, color: &Color) -> u64 {
        match color {
            Color::White => self.white.get_all(),
            Color::Black => self.black.get_all(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Bitboards {
    pawns: u64,
    knights: u64,
    bishops: u64,
    rooks: u64,
    queens: u64,
    king: u64,
    all: u64,
}

impl Bitboards {
    fn empty() -> Bitboards {
        Bitboards {
            pawns: 0,
            knights: 0,
            bishops: 0,
            rooks: 0,
            queens: 0,
            king: 0,
            all: 0,
        }
    }

    fn set_piece(&mut self, piece_type: &PieceType, square: &Square) {
        let index = square.0;
        self.all |= 1 << index;
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

    fn make_move(&mut self, piece_type: &PieceType, mv: &Move) {
        let from_index = mv.from.0;
        let to_index = mv.to.0;
        let from_bitboard = 1 << from_index;
        let to_bitboard = 1 << to_index;
        let from_to_bitboard = from_bitboard ^ to_bitboard;

        self.all ^= from_to_bitboard;
        match piece_type {
            PieceType::Pawn => self.pawns ^= from_to_bitboard,
            PieceType::Knight => self.knights ^= from_to_bitboard,
            PieceType::Bishop => self.bishops ^= from_to_bitboard,
            PieceType::Rook => self.rooks ^= from_to_bitboard,
            PieceType::Queen => self.queens ^= from_to_bitboard,
            PieceType::King => self.king ^= from_to_bitboard,
            _ => {}
        }
    }

    fn get_pawns(&self) -> u64 {
        self.pawns
    }

    fn get_knights(&self) -> u64 {
        self.knights
    }

    fn get_bishops(&self) -> u64 {
        self.bishops
    }

    fn get_rooks(&self) -> u64 {
        self.rooks
    }

    fn get_queens(&self) -> u64 {
        self.queens
    }

    fn get_king(&self) -> u64 {
        self.king
    }

    fn get_all(&self) -> u64 {
        self.all
    }
}

#[derive(Debug, PartialEq)]
pub struct CastlingRights {
    white_king_side: bool,
    white_queen_side: bool,
    black_king_side: bool,
    black_queen_side: bool,
}

impl CastlingRights {
    pub fn new(
        white_king_side: bool,
        white_queen_side: bool,
        black_king_side: bool,
        black_queen_side: bool,
    ) -> CastlingRights {
        CastlingRights {
            white_king_side,
            white_queen_side,
            black_king_side,
            black_queen_side,
        }
    }

    pub fn from_str(s: &str) -> CastlingRights {
        let white_king_side = s.contains("K");
        let white_queen_side = s.contains("Q");
        let black_king_side = s.contains("k");
        let black_queen_side = s.contains("q");
        CastlingRights::new(
            white_king_side,
            white_queen_side,
            black_king_side,
            black_queen_side,
        )
    }
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub promotion: Option<PieceType>,
}

impl Move {
    pub fn new(from: Square, to: Square, promotion: Option<PieceType>) -> Move {
        Move { from, to, promotion }
    }
}

impl Move {
    fn from_uci(uci: &str) -> Result<Move, FenError> {
        let from = Square::from_algebraic(&uci[0..2])?;
        let to = Square::from_algebraic(&uci[2..4])?;
        let promotion = if uci.len() == 5 {
            Some(PieceType::from_char(uci.chars().nth(4).unwrap())?)
        } else {
            None
        };

        Ok(Move::new(from, to, promotion))
    }
}

#[derive(Debug, PartialEq)]
pub struct Board {
    pieces: PiecePositions,
    moves: Vec<Move>,
    turn: Color,
    castling_rights: CastlingRights,
    en_passant: Option<Square>,
    halfmove_clock: u32,
    fullmove_counter: u32,
}

impl Board {
    pub fn new(
        pieces: PiecePositions,
        turn: Color,
        castling_rights: CastlingRights,
        en_passant: Option<Square>,
        halfmove_clock: u32,
        fullmove_counter: u32,
    ) -> Board {
        Board {
            pieces,
            moves: Vec::new(),
            turn,
            castling_rights,
            en_passant,
            halfmove_clock,
            fullmove_counter,
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

    pub fn uci_make_move(&mut self, uci_move: &str) {
        let mv = Move::from_uci(uci_move).unwrap();
        let piece = self.piece_at(&mv.from);
        self.make_move(&piece, &mv);
    }

    pub fn get_legal_moves(&self, mut _moves: Vec<Move>) -> Vec<Move> {
        todo!();
    }
}


// private methods for board
impl Board {
    fn piece_at(&self, square: &Square) -> PieceType {
        let square_bb = 1 << square.0;

        if self.get_pieces().get_pawns(&self.turn) & square_bb != 0 {
            PieceType::Pawn
        } else if self.get_pieces().get_knights(&self.turn) & square_bb != 0 {
            PieceType::Knight
        } else if self.get_pieces().get_bishops(&self.turn) & square_bb != 0 {
            PieceType::Bishop
        } else if self.get_pieces().get_rooks(&self.turn) & square_bb != 0 {
            PieceType::Rook
        } else if self.get_pieces().get_queens(&self.turn) & square_bb != 0 {
            PieceType::Queen
        } else if self.get_pieces().get_king(&self.turn) & square_bb != 0 {
            PieceType::King
        } else {
            PieceType::None
        }
    }

    fn make_move(&mut self, piece: &PieceType, mv: &Move) {
        self.pieces.make_move(piece, &self.turn, mv);
        self.moves.push(mv.clone());
        self.turn = self.turn.opposite();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn print_bitboard(bitboard: u64) {
        for rank in (0..8).rev() { // Los ranks van de 7 a 0 (fila superior a fila inferior)
            for file in 0..8 {    // Los files van de 0 a 7 (columna izquierda a derecha)
                let square = rank * 8 + file; // Calcular el índice del bit
                let bit = (bitboard >> square) & 1; // Extraer el bit correspondiente
                if bit == 1 {
                    print!("1 ");
                } else {
                    print!(". ");
                }
            }
            println!(); // Nueva línea al final de cada fila
        }
        println!(); // Espaciado entre bitboards si imprimes varios
    }


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

    #[test]
    fn test_move_from_uci() {
        let mv = Move::from_uci("e2e4").unwrap();
        assert_eq!(mv.from, Square(12));
        assert_eq!(mv.to, Square(28));
        assert_eq!(mv.promotion, None);
    }

    #[test]
    fn test_move_from_uci_with_promotion() {
        let mv = Move::from_uci("e7e8q").unwrap();
        assert_eq!(mv.from, Square(52));
        assert_eq!(mv.to, Square(60));
        assert_eq!(mv.promotion, Some(PieceType::Queen));
    }

    #[test]
    fn test_piece_positions_set_piece() {
        let mut piece_positions = PiecePositions::new();
        piece_positions.set_piece(&PieceType::Pawn, &Color::White, &Square(0));
        piece_positions.set_piece(&PieceType::Knight, &Color::White, &Square(1));
        piece_positions.set_piece(&PieceType::Bishop, &Color::White, &Square(2));
        piece_positions.set_piece(&PieceType::Rook, &Color::White, &Square(3));
        piece_positions.set_piece(&PieceType::Queen, &Color::White, &Square(4));
        piece_positions.set_piece(&PieceType::King, &Color::White, &Square(5));

        assert_eq!(piece_positions.get_pawns(&Color::White), 1);
        assert_eq!(piece_positions.get_knights(&Color::White), 2);
        assert_eq!(piece_positions.get_bishops(&Color::White), 4);
        assert_eq!(piece_positions.get_rooks(&Color::White), 8);
        assert_eq!(piece_positions.get_queens(&Color::White), 16);
        assert_eq!(piece_positions.get_king(&Color::White), 32);
    }

    #[test]
    fn test_piece_positions_make_move() {
        let mut piece_positions = PiecePositions::new();
        piece_positions.set_piece(&PieceType::Pawn, &Color::White, &Square(8));
        piece_positions.make_move(&PieceType::Pawn, &Color::White, &Move::new(Square(8), Square(16), None));

        assert_eq!(piece_positions.get_pawns(&Color::White), 1 << 16);
    }

    #[test]
    fn test_piece_positions_get_all() {
        let mut piece_positions = PiecePositions::new();
        piece_positions.set_piece(&PieceType::Pawn, &Color::White, &Square(8));
        piece_positions.set_piece(&PieceType::Knight, &Color::White, &Square(9));
        piece_positions.set_piece(&PieceType::Bishop, &Color::White, &Square(10));
        piece_positions.set_piece(&PieceType::Rook, &Color::White, &Square(11));
        piece_positions.set_piece(&PieceType::Queen, &Color::White, &Square(12));
        piece_positions.set_piece(&PieceType::King, &Color::White, &Square(13));

        assert_eq!(piece_positions.get_all(&Color::White), 0x3F00);
    }

    #[test]
    fn test_board_make_move() {
        let mut board = Board::starting_position();
        board.uci_make_move("e2e4");
        let square = Square::from_algebraic("e4").unwrap().0;
        print_bitboard(board.get_pieces().get_pawns(&Color::White));
        assert_eq!(board.get_pieces().get_pawns(&Color::White) & 1 << square, 1 << square);
    }
}
