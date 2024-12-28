use crate::{bitboard::{BitBoardMethods, RANK_3, RANK_7}, defs::{Color, Direction, Move}, pieces::PieceType, position::Position};

pub fn generate_pawn_moves(position: &Position) -> Vec<Move> {
    let mut moves = Vec::new();
    let color = position.get_side_to_move();
    let (rank_3, rank_7) = if position.get_side_to_move() == Color::White {
        (RANK_3, RANK_7)
    } else {
        (RANK_3.reverse_colors(), RANK_7.reverse_colors())
    };

    let pawns = position.get_pieces_color_type(color, PieceType::Pawn);
    let empty_squares = !position.get_all_pieces();

    let single_pushes = pawns.shift(Direction::UP) & empty_squares;
    let double_pawns = single_pushes & rank_3;
    let double_pushes = double_pawns.shift(Direction::UP) & empty_squares;

    //TODO: Terminar

    moves
}
