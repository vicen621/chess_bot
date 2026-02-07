mod board;
mod types;

use crate::types::Board;

fn main() {
    let board = Board::from_fen("7K/8/8/8/8/8/8/8 w - - 0 1").unwrap();
    println!("{}", board);

    let moves = board.generate_moves();
    for mv in moves {
        println!(
            "{:?} -> {:?}",
            Board::index_to_coord_algebraic(mv.from),
            Board::index_to_coord_algebraic(mv.to)
        );
    }
}

#[cfg(test)]
mod tests {
    mod board_tests;
}
