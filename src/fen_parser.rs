use crate::board::{Board, CastlingRights, Color, PiecePositions, PieceType, Square};

pub struct FenParser;

impl FenParser {
    /// Parsea una posición FEN y devuelve un objeto `Board`.
    pub fn parse(fen: &str) -> Result<Board, FenError> {
        let parts: Vec<&str> = fen.split_whitespace().collect();

        if parts.len() != 6 {
            return Err(FenError::InvalidFormat);
        }

        let pieces = FenParser::parse_piece_positions(parts[0])?;
        let turn = FenParser::parse_turn(parts[1])?;
        let castling_rights = FenParser::parse_castling_rights(parts[2]);
        let en_passant = FenParser::parse_en_passant(parts[3])?;
        let halfmove_clock = parts[4]
            .parse::<u32>()
            .map_err(|_| FenError::InvalidFormat)?;
        let fullmove_counter = parts[5]
            .parse::<u32>()
            .map_err(|_| FenError::InvalidFormat)?;

        Ok(Board::new(
            pieces,
            turn,
            castling_rights,
            en_passant,
            halfmove_clock,
            fullmove_counter,
        ))
    }

    /// Parsea la posición de las piezas (primer campo de FEN).
    fn parse_piece_positions(pieces_fen: &str) -> Result<PiecePositions, FenError> {
        let mut positions = PiecePositions::new();
        let ranks: Vec<&str> = pieces_fen.split('/').collect();

        if ranks.len() != 8 {
            return Err(FenError::InvalidRankLength);
        }

        for (rank_index, rank) in ranks.iter().enumerate() {
            let rank_index = 7 - rank_index; // FEN ordena las filas de 8 a 1
            let mut file_index = 0;

            for c in rank.chars() {
                if c.is_digit(10) {
                    file_index += c.to_digit(10).unwrap();
                } else {
                    let piece_type = PieceType::from_char(c)?;
                    let color = Color::from_fen(c);
                    let square = Square::from_algebraic(&format!(
                        "{}{}",
                        ('a' as u8 + file_index as u8) as char,
                        (rank_index + 1) as u8
                    ))?; // +1 porque las filas empiezan en 1
                    positions.set_piece(&piece_type, &color, &square);
                    file_index += 1;
                }
            }

            if file_index != 8 {
                return Err(FenError::InvalidFileLength);
            }
        }

        Ok(positions)
    }

    /// Parsea el turno (segundo campo de FEN).
    fn parse_turn(turn_fen: &str) -> Result<Color, FenError> {
        if turn_fen.len() != 1 {
            return Err(FenError::InvalidTurn);
        }

        Color::from_char(turn_fen.chars().next().unwrap())
    }

    /// Parsea los derechos de enroque (tercer campo de FEN).
    fn parse_castling_rights(castling_fen: &str) -> CastlingRights {
        CastlingRights::from_str(castling_fen)
    }

    /// Parsea la casilla de peón al paso (cuarto campo de FEN).
    fn parse_en_passant(en_passant_fen: &str) -> Result<Option<Square>, FenError> {
        if en_passant_fen == "-" {
            Ok(None)
        } else {
            Square::from_algebraic(en_passant_fen).map(|s| Some(s))
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum FenError {
    InvalidFormat,
    InvalidPiece(char),
    InvalidRankLength,
    InvalidFileLength,
    InvalidTurn,
    InvalidCastlingRights,
    InvalidEnPassant,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_starting_position() {
        let board = FenParser::parse("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

        assert_eq!(board.get_pieces().get_pawns(&Color::White), 0xFF00);
        assert_eq!(board.get_pieces().get_knights(&Color::White), 0x42);
        assert_eq!(board.get_pieces().get_bishops(&Color::White), 0x24);
        assert_eq!(board.get_pieces().get_rooks(&Color::White), 0x81);
        assert_eq!(board.get_pieces().get_queens(&Color::White), 0x8);
        assert_eq!(board.get_pieces().get_king(&Color::White), 0x10);

        assert_eq!(board.get_turn(), &Color::White);
        assert_eq!(
            board.get_castling_rights(),
            &CastlingRights::new(true, true, true, true)
        );
        assert_eq!(board.get_en_passant(), &None);
        assert_eq!(board.get_halfmove_clock(), 0);
        assert_eq!(board.get_fullmove_counter(), 1);
    }

    #[test]
    fn test_invalid_fen_format() {
        let result = FenParser::parse("invalid fen");
        assert!(result.is_err());
    }
}
