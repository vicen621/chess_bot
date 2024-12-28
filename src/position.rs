use std::str::FromStr;

use crate::{
    defs::{Color, Piece, PieceType, Squares},
    fen_parser::{FenError, FenParser},
    game_state::GameState,
};

pub type BitBoard = u64;
const MAX_MOVE_COUNT: usize = 1024;

pub struct Position {
    board: [Piece; 64],
    by_type_bitboards: [BitBoard; 6],
    by_color_bitboards: [BitBoard; 2],
    all_pieces_bitboard: BitBoard,
    state: GameState,
}

impl Position {
    pub fn new(state: GameState) -> Position {
        Position {
            board: [Piece::EMPTY; 64],
            by_type_bitboards: [0; 6],
            by_color_bitboards: [0; 2],
            all_pieces_bitboard: 0,
            state,
        }
    }

    pub fn add_piece(&mut self, piece: Piece, square: Squares) {
        let index = square.to_index();
        let mask = 1 << index;

        self.by_type_bitboards[piece.get_piece_type().to_index()] |= mask;
        self.by_color_bitboards[piece.get_color().to_index()] |= mask;
        self.all_pieces_bitboard |= mask;
        self.board[index] = piece;
    }

    pub fn remove_piece(&mut self, square: Squares) {
        let index = square.to_index();
        let mask = !(1 << index);
        let piece = self.board[index];

        self.by_type_bitboards[piece.get_piece_type().to_index()] ^= mask;
        self.by_color_bitboards[piece.get_color().to_index()] ^= mask;
        self.all_pieces_bitboard ^= mask;
        self.board[index] = Piece::EMPTY;
    }

    pub fn move_piece(&mut self, from: Squares, to: Squares) {
        let piece = self.board[from.to_index()];
        let from_to: BitBoard = 1 << from.to_index() | 1 << to.to_index();

        self.by_type_bitboards[piece.get_piece_type().to_index()] ^= from_to;
        self.by_color_bitboards[piece.get_color().to_index()] ^= from_to;
        self.all_pieces_bitboard ^= from_to;
        self.board[from.to_index()] = Piece::EMPTY;
        self.board[to.to_index()] = piece;
    }

    pub fn do_move(&mut self) {
        todo!();
    }

    pub fn undo_move(&mut self) {
        todo!();
    }
}

// getters
impl Position {
    pub fn get_side_to_move(&self) -> Color {
        self.state.side_to_move
    }

    pub fn get_castling(&self) -> u8 {
        self.state.castling
    }

    pub fn get_halfmove_clock(&self) -> u8 {
        self.state.halfmove_clock
    }

    pub fn get_en_passant(&self) -> Option<usize> {
        self.state.en_passant
    }

    pub fn get_fullmove_number(&self) -> u16 {
        self.state.fullmove_number
    }

    pub fn get_pieces_type(&self, piece_type: PieceType) -> BitBoard {
        self.by_type_bitboards[piece_type.to_index()]
    }

    pub fn get_pieces_color(&self, color: Color) -> BitBoard {
        self.by_color_bitboards[color.to_index()]
    }

    pub fn get_all_pieces(&self) -> BitBoard {
        self.all_pieces_bitboard
    }

    pub fn get_pieces_color_type(&self, color: Color, piece_type: PieceType) -> BitBoard {
        self.by_color_bitboards[color.to_index()] & self.by_type_bitboards[piece_type.to_index()]
    }
}

/// Construct the initial position.
impl Default for Position {
    fn default() -> Position {
        Position::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
            .expect("Invalid FEN string")
    }
}

impl FromStr for Position {
    type Err = FenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        FenParser::parse_fen(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn print_bitboard(bitboard: BitBoard) {
        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = Squares::from_file_rank(file, rank);
                let mask = 1 << square.to_index();
                if bitboard & mask != 0 {
                    print!("1 ");
                } else {
                    print!(". ");
                }
            }
            println!();
        }
    }

    #[test]
    fn test_position() {
        let position = Position::default();
        print_bitboard(position.get_all_pieces());
        println!();
        print_bitboard(position.get_pieces_color(Color::White));
        println!();
        print_bitboard(position.get_pieces_color(Color::Black));
        println!();
        print_bitboard(position.get_pieces_type(PieceType::Pawn));
        println!();
        print_bitboard(position.get_pieces_color_type(Color::White, PieceType::Pawn));
        println!();
        print_bitboard(position.get_pieces_color_type(Color::Black, PieceType::Pawn));
        println!();
    }
}
