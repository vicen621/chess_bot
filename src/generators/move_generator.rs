use crate::{chess_move::Move, pieces::PieceType, position::Position};

use super::{king_moves::KingLookup, knight_moves::KnightLookup, pawn_moves::PawnMoves, sliding_moves::SlidingLookup};

const MAX_MOVES: usize = 256;

pub struct MoveGenerator {
    pub knight_lookup: KnightLookup,
    pub king_lookup: KingLookup,
    pub sliding_lookup: SlidingLookup,
}

impl MoveGenerator {
    pub fn new() -> Self {
        Self {
            knight_lookup: KnightLookup::new(),
            king_lookup: KingLookup::new(),
            sliding_lookup: SlidingLookup::new(),
        }
    }

    pub fn generate_all_pseudo_legal_moves(&self, position: &Position) -> Vec<Move> {
        let mut moves = Vec::with_capacity(MAX_MOVES);

        //TODO: generate castling moves
        self.generate_pseudo_legal_moves(position, &mut moves, PieceType::Pawn);
        self.generate_pseudo_legal_moves(position, &mut moves, PieceType::Knight);
        self.generate_pseudo_legal_moves(position, &mut moves, PieceType::King);
        self.generate_pseudo_legal_moves(position, &mut moves, PieceType::Queen);
        self.generate_pseudo_legal_moves(position, &mut moves, PieceType::Rook);

        moves
    }

    pub fn generate_pseudo_legal_moves(&self, position: &Position, moves: &mut Vec<Move>, piece_type: PieceType) {
        match piece_type {
            PieceType::Pawn => {
                PawnMoves::generate_pseudo_legal_pawn_moves(position, moves);
            }
            PieceType::Knight => {
                self.knight_lookup.generate_pseudo_legal_knight_moves(position, moves);
            }
            PieceType::King => {
                self.king_lookup.generate_pseudo_legal_king_moves(position, moves);
            }
            PieceType::Queen => {
                self.sliding_lookup.generate_pseudo_legal_queen_moves(position, moves);
            }
            PieceType::Rook => {
                self.sliding_lookup.generate_pseudo_legal_rook_moves(position, moves);
            }
            PieceType::Bishop => {
                self.sliding_lookup.generate_pseudo_legal_bishop_moves(position, moves);
            }
            _ => {}
        }
    }
}
