use crate::{
    defs::{Castling, CastlingRights, Color, Square}, game_state::GameState, pieces::Piece, position::Position
};

pub struct FenParser;
impl FenParser {
    pub fn parse_fen(fen: &str) -> Result<Position, FenError> {
        let parts: Vec<&str> = fen.split_whitespace().collect();

        if parts.len() != 6 {
            return Err(FenError::InvalidFormat);
        }

        let side_to_move = FenParser::parse_turn(parts[1])?;
        let castling_rights = FenParser::parse_castling_rights(parts[2]);
        let en_passant = FenParser::parse_en_passant(parts[3]);
        let halfmove_clock = parts[4]
            .parse::<u8>()
            .map_err(|_| FenError::InvalidFormat)?;
        let fullmove_counter = parts[5]
            .parse::<u16>()
            .map_err(|_| FenError::InvalidFormat)?;

        let state = GameState::new(
            side_to_move,
            castling_rights,
            halfmove_clock,
            en_passant,
            fullmove_counter,
        );
        let mut position = Position::new(state);
        FenParser::parse_piece_positions(parts[0], &mut position)?;

        Ok(position)
    }
}

// private methods
impl FenParser {
    /// Parsea la posición de las piezas (primer campo de FEN).
    fn parse_piece_positions(pieces_fen: &str, position: &mut Position) -> Result<(), FenError> {
        let ranks: Vec<&str> = pieces_fen.split('/').collect();

        if ranks.len() != 8 {
            return Err(FenError::InvalidRankLength);
        }

        for (rank_index, rank) in ranks.iter().enumerate() {
            let rank_index = 7 - rank_index; // FEN ordena las filas de 8 a 1
            let mut file_index = 0;

            for c in rank.chars() {
                if c.is_digit(10) {
                    file_index += c.to_digit(10).unwrap() as usize;
                } else {
                    let piece = Piece::from_char(c);
                    let square = Square::from_file_rank(file_index, rank_index);
                    position.add_piece(piece, square);
                    file_index += 1;
                }
            }

            if file_index != 8 {
                return Err(FenError::InvalidFileLength);
            }
        }

        Ok(())
    }

    /// Parsea el turno (segundo campo de FEN).
    fn parse_turn(turn_fen: &str) -> Result<Color, FenError> {
        if turn_fen.len() != 1 {
            return Err(FenError::InvalidTurn);
        }

        if turn_fen == "w" {
            Ok(Color::White)
        } else if turn_fen == "b" {
            Ok(Color::Black)
        } else {
            Err(FenError::InvalidTurn)
        }
    }

    /// Parsea los derechos de enroque (tercer campo de FEN).
    fn parse_castling_rights(castling_fen: &str) -> CastlingRights {
        let mut castling_rights = Castling::NO_CASTLING;

        for c in castling_fen.chars() {
            match c {
                'K' => castling_rights |= Castling::WHITE_KING_SIDE,
                'Q' => castling_rights |= Castling::WHITE_QUEEN_SIDE,
                'k' => castling_rights |= Castling::BLACK_KING_SIDE,
                'q' => castling_rights |= Castling::BLACK_QUEEN_SIDE,
                '-' => return Castling::NO_CASTLING,
                _ => return Castling::NO_CASTLING,
            }
        }

        castling_rights
    }

    /// Parsea la casilla de peón al paso (cuarto campo de FEN).
    fn parse_en_passant(en_passant_fen: &str) -> Option<usize> {
        if en_passant_fen == "-" {
            None
        } else {
            Some(Square::from_algebraic(en_passant_fen).to_index())
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum FenError {
    InvalidFormat,
    InvalidRankLength,
    InvalidFileLength,
    InvalidTurn,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_fen() {
        let position =
            FenParser::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
                .unwrap();

        assert_eq!(position.get_all_pieces(), 0xFFFF_0000_0000_FFFF);
        assert_eq!(position.get_side_to_move(), Color::White);
        assert_eq!(position.get_castling(), Castling::ANY_CASTLING);
        assert_eq!(position.get_halfmove_clock(), 0);
        assert_eq!(position.get_en_passant(), None);
        assert_eq!(position.get_fullmove_number(), 1);
    }
}
