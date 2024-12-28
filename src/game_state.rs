use crate::defs::{Castling, CastlingRights, Color, Square};

// The game state is separated from the board because it is easier to serialize and deserialize.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GameState {
    pub side_to_move: Color,
    pub castling: CastlingRights,
    pub halfmove_clock: u8,
    pub en_passant: Option<usize>,
    pub fullmove_number: u16,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            side_to_move: Color::White,
            castling: Castling::ANY_CASTLING,
            halfmove_clock: 0,
            en_passant: None,
            fullmove_number: 1,
        }
    }
}

impl GameState {
    pub fn new(side_to_move: Color, castling: CastlingRights, halfmove_clock: u8, en_passant: Option<usize>, fullmove_number: u16) -> Self {
        GameState {
            side_to_move,
            castling,
            halfmove_clock,
            en_passant,
            fullmove_number,
        }
    }
}
