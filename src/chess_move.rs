use std::slice::Iter;

use crate::{bitboard::{BitBoard, PieceItr}, defs::Square, pieces::PieceType};

pub struct Move {
    pub from: Square,
    pub to: Square,
    pub kind: MoveType,
}

impl Move {
    pub fn new(from: Square, to: Square, kind: MoveType) -> Self {
        Self {
            from,
            to,
            kind,
        }
    }

    pub fn to_uci(&self) -> String {
        let mut s = format!("{}{}", self.from.to_algebraic(), self.to.to_algebraic());
        if self.is_promotion() || self.is_promotion_capture() {
            s.push_str(&self.promoted_piece().unwrap().to_char().to_string());
        }

        s
    }

    pub fn is_double_pawn_push(&self) -> bool {
        ((self.to.to_index()) - (self.from.to_index())) == 16
    }

    pub fn is_promotion_capture(&self) -> bool {
        self.kind == MoveType::KnightPromotionCapture
            || self.kind == MoveType::BishopPromotionCapture
            || self.kind == MoveType::RookPromotionCapture
            || self.kind == MoveType::QueenPromotionCapture
    }

    pub fn is_promotion(&self) -> bool {
        self.kind == MoveType::KnightPromotion
            || self.kind == MoveType::BishopPromotion
            || self.kind == MoveType::RookPromotion
            || self.kind == MoveType::QueenPromotion
    }

    pub fn is_en_passant_capture(&self) -> bool {
        self.kind == MoveType::EnPassantCapture
    }

    pub fn is_castle(&self) -> bool {
        self.kind == MoveType::CastleKing || self.kind == MoveType::CastleQueen
    }

    pub fn is_capture(&self) -> bool {
        self.kind == MoveType::Capture
            || self.kind == MoveType::EnPassantCapture
            || self.is_promotion_capture()
    }

    pub fn promoted_piece(&self) -> Option<PieceType> {
        match self.kind {
            MoveType::RookPromotionCapture | MoveType::RookPromotion => Some(PieceType::Rook),
            MoveType::KnightPromotionCapture | MoveType::KnightPromotion => Some(PieceType::Knight),
            MoveType::BishopPromotionCapture | MoveType::BishopPromotion => Some(PieceType::Bishop),
            MoveType::QueenPromotionCapture | MoveType::QueenPromotion => Some(PieceType::Queen),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum MoveType {
    Capture,
    EnPassantCapture,
    KnightPromotion,
    BishopPromotion,
    RookPromotion,
    QueenPromotion,
    KnightPromotionCapture,
    BishopPromotionCapture,
    RookPromotionCapture,
    QueenPromotionCapture,
    Quiet,
    CastleKing,
    CastleQueen,
    Null,
}

impl MoveType {
    pub fn promotion_itr() -> Iter<'static, MoveType> {
        static PROMOTIONS: [MoveType; 4] = [
            MoveType::KnightPromotion,
            MoveType::BishopPromotion,
            MoveType::RookPromotion,
            MoveType::QueenPromotion,
        ];
        PROMOTIONS.iter()
    }

    pub fn promotion_capture_itr() -> Iter<'static, MoveType> {
        static PROMOTIONS: [MoveType; 4] = [
            MoveType::KnightPromotionCapture,
            MoveType::BishopPromotionCapture,
            MoveType::RookPromotionCapture,
            MoveType::QueenPromotionCapture,
        ];
        PROMOTIONS.iter()
    }
}

#[derive(Clone, Copy)]
pub enum PromotionType {
    Push,
    Capture,
}

pub struct Direction;
impl Direction {
    pub const UP: i8 = 8;
    pub const DOWN: i8 = -8;
    pub const RIGHT: i8 = 1;
    pub const LEFT: i8 = -1;
    pub const UP_RIGHT: i8 = Self::UP + Self::RIGHT;
    pub const UP_LEFT: i8 = Self::UP + Self::LEFT;
    pub const DOWN_RIGHT: i8 = Self::DOWN + Self::RIGHT;
    pub const DOWN_LEFT: i8 = Self::DOWN + Self::LEFT;

    pub fn king_itr() -> Iter<'static, i8> {
        static KING_MOVES: [i8; 8] = [
            Direction::UP,
            Direction::DOWN,
            Direction::RIGHT,
            Direction::LEFT,
            Direction::UP_RIGHT,
            Direction::UP_LEFT,
            Direction::DOWN_RIGHT,
            Direction::DOWN_LEFT,
        ];
        KING_MOVES.iter()
    }
}

pub struct MoveExtractor;
impl MoveExtractor {
    pub fn extract_moves(from: Square, bb: BitBoard, list: &mut Vec<Move>, kind: MoveType) {
        for (square, _) in bb.iter() {
            let m = Move {
                to: square,
                from,
                kind,
            };
            list.push(m);
        }
    }
}
