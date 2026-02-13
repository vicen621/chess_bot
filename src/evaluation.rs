use crate::types::{Board, Color, PieceType};

const PAWN_VALUE: i32 = 100;
const KNIGHT_VALUE: i32 = 320;
const BISHOP_VALUE: i32 = 330;
const ROOK_VALUE: i32 = 500;
const QUEEN_VALUE: i32 = 900;

pub fn evaluate(board: &Board) -> i32 {
    let mut score = 0;

    for opt_piece in board.squares.iter() {
        if let Some(piece) = opt_piece {
            let piece_value = match piece.piece_type {
                PieceType::Pawn => PAWN_VALUE,
                PieceType::Knight => KNIGHT_VALUE,
                PieceType::Bishop => BISHOP_VALUE,
                PieceType::Rook => ROOK_VALUE,
                PieceType::Queen => QUEEN_VALUE,
                _ => 0, // El rey no tiene valor material
            };

            if piece.color == Color::White {
                score += piece_value;
            } else {
                score -= piece_value;
            }
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Board;

    #[test]
    fn test_eval_start_position() {
        // La posición inicial debe ser igualdad absoluta (0)
        let board = Board::initial_position();
        let score = evaluate(&board);
        assert_eq!(score, 0, "La posición inicial debería tener score 0");
    }

    #[test]
    fn test_eval_white_pawn_advantage() {
        // Blancas tienen un peón extra en e4
        // FEN: Reyes y un peón blanco
        let fen = "8/8/8/8/4P3/8/8/k3K3 w - - 0 1";
        let board = Board::from_fen(fen).expect("FEN inválido");
        let score = evaluate(&board);

        assert_eq!(score, PAWN_VALUE, "Blancas deberían ganar por un peón");
    }

    #[test]
    fn test_eval_black_rook_advantage() {
        // Negras tienen una torre extra
        // FEN: Reyes y una torre negra en a8
        let fen = "r7/8/8/8/8/8/8/k3K3 w - - 0 1";
        let board = Board::from_fen(fen).expect("FEN inválido");
        let score = evaluate(&board);

        assert_eq!(
            score, -ROOK_VALUE,
            "Negras deberían ganar por una torre (score negativo)"
        );
    }

    #[test]
    fn test_eval_material_imbalance() {
        // Blanco: Rey + Dama. Negro: Rey + Torre + Alfil.
        let fen = "8/8/8/2b5/2r5/8/3Q4/k3K3 w - - 0 1";

        let board = Board::from_fen(fen).expect("FEN inválido");
        let score = evaluate(&board);

        let expected_white = QUEEN_VALUE;
        let expected_black = ROOK_VALUE + BISHOP_VALUE;
        let expected_diff = expected_white - expected_black;

        assert_eq!(
            score, expected_diff,
            "El cálculo de desequilibrio material falló"
        );
    }

    #[test]
    fn test_eval_symmetric_position() {
        let fen = "8/8/8/4p3/4P3/8/8/k3K3 w - - 0 1";
        let board = Board::from_fen(fen).expect("FEN inválido");
        let score = evaluate(&board);

        assert_eq!(score, 0, "Material igual y simétrico debería ser 0");
    }

    #[test]
    fn test_eval_complex_capture() {
        // Blancas faltan: 1 Peón.
        // Negras faltan: 1 Caballo.
        // Ventaja blanca: Caballo - Peón = 320 - 100 = +220.

        // FEN startpos sin peón blanco e2 y sin caballo negro g8
        let fen = "rnbqkb1r/pppppppp/8/8/8/8/PPPP1PPP/RNBQKBNR w KQkq - 0 1";

        let board = Board::from_fen(fen).expect("FEN inválido");
        let score = evaluate(&board);

        assert_eq!(score, KNIGHT_VALUE - PAWN_VALUE, "Fallo en conteo complejo");
    }
}
