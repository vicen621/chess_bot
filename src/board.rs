use std::str::FromStr;

use crate::{
    defs::{CastlingRights, Color, Move, Square, Squares}, fen_parser::{FenError, FenParser}, game_state::GameState
};

pub type BitBoard = u64;
const MAX_MOVE_COUNT: usize = 1024;

pub struct Board {
    pieces: [Square; 64],
    state: GameState,
    moves: Vec<Move>,

    black_ocupancy: BitBoard,
    white_ocupancy: BitBoard,
}

impl Board {
    pub fn new(pieces: [Square; 64], state: GameState, black_ocupancy: BitBoard, white_ocupancy: BitBoard) -> Board {
        Board { pieces, state, moves: Vec::with_capacity(MAX_MOVE_COUNT), black_ocupancy, white_ocupancy }
    }

    pub fn get_pieces(&self) -> &[Square; 64] {
        &self.pieces
    }

    pub fn get_side_to_move(&self) -> Color {
        self.state.side_to_move
    }

    pub fn get_castling_rights(&self) -> CastlingRights {
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

    pub fn print_board(&self) {
        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = Squares::from_file_rank(file, rank);
                let piece = self.pieces[square.to_index()];
                match piece {
                    Square::Empty => print!(". "),
                    Square::Occupied(piece) => print!("{} ", piece.to_char()),
                }
            }
            println!();
        }
    }

    pub fn make_move(&mut self, _from: Squares, _to: Squares) {
        // TODO
    }

    pub fn undo_move(&mut self) {
        // TODO
    }
}

/// Construct the initial position.
impl Default for Board {
    fn default() -> Board {
        Board::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
            .expect("Invalid FEN string")
    }
}

impl FromStr for Board {
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
    fn test_board() {
        let board = Board::default();

        assert_eq!(board.black_ocupancy, 0xFFFF << 48);
        assert_eq!(board.white_ocupancy, 0xFFFF);

        board.print_board();
        println!();
        print_bitboard(board.black_ocupancy);
        println!();
        print_bitboard(board.white_ocupancy);
    }
}
