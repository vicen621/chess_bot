use crate::{bitboard::{BitBoard, PieceItr}, chess_move::{Move, MoveExtractor, MoveType}, defs::Square, pieces::PieceType, position::Position};

pub struct SlidingLookup {
    rooks: [BitBoard; 64],
    bishops: [BitBoard; 64],
}

impl SlidingLookup {
    pub fn new() -> Self {
        let mut rooks = [0; 64];
        let mut bishops = [0; 64];

        for i in 0..64 {
            rooks[i] = Self::generate_rook_moves(i);
            bishops[i] = Self::generate_bishop_moves(i);
        }

        Self { rooks, bishops }
    }

    pub fn generate_pseudo_legal_rook_moves(&self, position: &Position, moves: &mut Vec<Move>) {
        let color = position.get_side_to_move();
        let rooks = position.get_pieces_color_type(color, PieceType::Rook);

        for (rook, _) in rooks.iter() {
            self.generate_rook_attacks(position, moves, rook);
            self.generate_rook_quiet_moves(position, moves, rook);
        }
    }

    pub fn generate_pseudo_legal_bishop_moves(&self, position: &Position, moves: &mut Vec<Move>) {
        let color = position.get_side_to_move();
        let bishops = position.get_pieces_color_type(color, PieceType::Bishop);

        for (bishop, _) in bishops.iter() {
            self.generate_bishop_attacks(position, moves, bishop);
            self.generate_bishop_quiet_moves(position, moves, bishop);
        }
    }

    pub fn generate_pseudo_legal_queen_moves(&self, position: &Position, moves: &mut Vec<Move>) {
        let color = position.get_side_to_move();
        let queens = position.get_pieces_color_type(color, PieceType::Queen);

        for (queen, _) in queens.iter() {
            self.generate_queen_attacks(position, moves, queen);
            self.generate_queen_quiet_moves(position, moves, queen);
        }
    }
}

// rooks
impl SlidingLookup {

    fn generate_rook_attacks(&self, position: &Position, moves: &mut Vec<Move>, rook: Square) {
        let color = position.get_side_to_move();
        let their_king = position.get_pieces_color_type(!color, PieceType::King);
        let captures = position.get_pieces_color(!color) & !their_king;

        let knight_captures = self.get_rook_moves(rook) & captures;

        MoveExtractor::extract_moves(rook, knight_captures, moves, MoveType::Capture);
    }

    fn generate_rook_quiet_moves(&self, position: &Position, moves: &mut Vec<Move>, rook: Square) {
        let empty_squares = !position.get_all_pieces();
        let rook_moves = self.get_rook_moves(rook) & empty_squares;

        MoveExtractor::extract_moves(rook, rook_moves, moves, MoveType::Quiet);
    }

    fn get_rook_moves(&self, square: Square) -> BitBoard {
        self.rooks[square.to_index()]
    }

    fn generate_rook_moves(square: usize) -> BitBoard {
        let mut attacks = 0;
        let (rank, file) = (square >> 3, square & 7);

        // Vertical attacks
        for r in 0..8 {
            if r != rank {
                attacks |= 1 << (r * 8 + file);
            }
        }

        // Horizontal attacks
        for f in 0..8 {
            if f != file {
                attacks |= 1 << (rank * 8 + f);
            }
        }

        attacks
    }
}

// Bishops
impl SlidingLookup {
    fn generate_bishop_attacks(&self, position: &Position, moves: &mut Vec<Move>, bishop: Square) {
        let color = position.get_side_to_move();
        let their_king = position.get_pieces_color_type(!color, PieceType::King);
        let captures = position.get_pieces_color(!color) & !their_king;

        let bishop_captures = self.get_bishop_moves(bishop) & captures;

        MoveExtractor::extract_moves(bishop, bishop_captures, moves, MoveType::Capture);
    }

    fn generate_bishop_quiet_moves(&self, position: &Position, moves: &mut Vec<Move>, bishop: Square) {
        let empty_squares = !position.get_all_pieces();
        let bishop_moves = self.get_bishop_moves(bishop) & empty_squares;

        MoveExtractor::extract_moves(bishop, bishop_moves, moves, MoveType::Quiet);
    }

    fn get_bishop_moves(&self, square: Square) -> BitBoard {
        self.bishops[square.to_index()]
    }

    fn generate_bishop_moves(square: usize) -> BitBoard {
        let mut attacks = 0;
        let (rank, file) = (square >> 3, square & 7);

        // Diagonal superior derecha
        let mut r = rank as isize + 1;
        let mut f = file as isize + 1;
        while r < 8 && f < 8 {
            attacks |= 1 << (r * 8 + f);
            r += 1;
            f += 1;
        }

        // Diagonal inferior izquierda
        r = rank as isize - 1;
        f = file as isize - 1;
        while r >= 0 && f >= 0 {
            attacks |= 1 << (r * 8 + f);
            r -= 1;
            f -= 1;
        }

        // Diagonal superior izquierda
        r = rank as isize + 1;
        f = file as isize - 1;
        while r < 8 && f >= 0 {
            attacks |= 1 << (r * 8 + f);
            r += 1;
            f -= 1;
        }

        // Diagonal inferior derecha
        r = rank as isize - 1;
        f = file as isize + 1;
        while r >= 0 && f < 8 {
            attacks |= 1 << (r * 8 + f);
            r -= 1;
            f += 1;
        }

        attacks
    }
}

// queens
impl SlidingLookup {
    fn generate_queen_attacks(&self, position: &Position, moves: &mut Vec<Move>, queen: Square) {
        let color = position.get_side_to_move();
        let their_king = position.get_pieces_color_type(!color, PieceType::King);
        let captures = position.get_pieces_color(!color) & !their_king;

        let queen_captures = self.get_queen_moves(queen) & captures;

        MoveExtractor::extract_moves(queen, queen_captures, moves, MoveType::Capture);
    }

    fn generate_queen_quiet_moves(&self, position: &Position, moves: &mut Vec<Move>, queen: Square) {
        let empty_squares = !position.get_all_pieces();
        let queen_moves = self.get_queen_moves(queen) & empty_squares;

        MoveExtractor::extract_moves(queen, queen_moves, moves, MoveType::Quiet);
    }

    fn get_queen_moves(&self, square: Square) -> BitBoard {
        self.rooks[square.to_index()] | self.bishops[square.to_index()]
    }
}
