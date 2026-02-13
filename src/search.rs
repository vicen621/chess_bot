use crate::{
    evaluation::evaluate,
    types::{Board, Color, Move},
};

const INFINITY: i32 = 50000;
const MATE_SCORE: i32 = 49000;

pub fn search_best_move(board: &Board, depth: u32) -> Option<Move> {
    let moves = board.generate_moves();
    let mut alpha = -INFINITY;
    let beta = INFINITY;
    let mut best_move = None;

    for mv in moves {
        let mut new_board = board.clone();
        new_board.make_move(&mv);
        let score = -negamax(&new_board, depth - 1, -beta, -alpha);

        if score > alpha {
            alpha = score;
            best_move = Some(mv);
        }
    }

    best_move
}

fn negamax(board: &Board, depth: u32, mut alpha: i32, beta: i32) -> i32 {
    let moves = board.generate_moves();
    if moves.is_empty() {
        if board.is_king_attacked(board.turn) {
            return -MATE_SCORE - (depth as i32); // Penaliza más cuanto más profundo estemos
        } else {
            return 0; // Tablas por ahogado
        }
    }

    if depth == 0 {
        let score = evaluate(board);
        return if board.turn == Color::White {
            score
        } else {
            -score
        };
    }

    for mv in moves {
        let mut new_board = board.clone();
        new_board.make_move(&mv);
        let eval = -negamax(&new_board, depth - 1, -beta, -alpha);
        if eval >= beta {
            return beta;
        }
        alpha = alpha.max(eval);
    }

    alpha
}
