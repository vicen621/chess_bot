mod board;
mod types;

use std::{fs::OpenOptions, io::{self, BufRead, Write}};

use crate::types::Board;

fn main() {
    let stdin = io::stdin();
    let mut board = Board::initial_position();

    for line in stdin.lock().lines() {
        let input = line.unwrap();

        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "uci" => {
                println!("id name ChessBot621");
                println!("id author Vicente Garcia Marti");
                println!("uciok");
            }
            "isready" => {
                println!("readyok");
            }
            "ucinewgame" => {
                board = Board::initial_position();
            }
            "position" => {
                if parts.len() < 2 {
                    continue;
                }

                match parts[1] {
                    "startpos" => {
                        board = Board::initial_position();
                        if parts.len() > 2 && parts[2] == "moves" {
                            for (i, mv) in parts[3..].iter().enumerate() {
                                match board.parse_move(mv) {
                                    Some(chess_move) => {
                                        board.make_move(&chess_move);
                                    }
                                    None => {
                                        let error_msg = format!("ERROR CRÍTICO: Falló al parsear el movimiento en position startpos'{}' (turno {}). FEN actual: {}", mv, i+1, board.to_fen());
                                        log_to_file(&error_msg);
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    "fen" => {
                        let str = parts[2..].join(" ");
                        let fen_parts: Vec<&str> = str.split(" moves ").collect();
                        let fen_str = fen_parts[0];

                        match Board::from_fen(fen_str) {
                            Ok(new_board) => {
                                board = new_board;
                                // Aplicar movimientos si los hay
                                if fen_parts.len() > 1 {
                                    let moves: Vec<&str> =
                                        fen_parts[1].split_whitespace().collect();
                                    for mv in moves {
                                        if let Some(chess_move) = board.parse_move(mv) {
                                            board.make_move(&chess_move);
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                log_to_file(&format!("ERROR CRÍTICO AL CARGAR POSITION FEN: {}", e)) ;
                                log_to_file(&format!("FEN INTENTADO: {}", fen_str));
                            }
                        }
                    }
                    _ => {}
                }
            }
            "go" => {
                // Aca va la logica para calcular el mejor movimiento.
                // Por ahora, devuelvo un movimiento  aleatorio.
                let best_move = get_best_move_placeholder(&board);
                println!("bestmove {}", best_move);
            }
            "quit" => {
                break;
            }
            "setoption" => {
                // Lichess manda configuraciones (como el Move Overhead).
                // En el futuro, se parsea: name <x> value <y>
            }
            _ => {}
        }
    }
}

// Función temporal hasta que implemente un motor real
fn get_best_move_placeholder(board: &Board) -> String {
    let moves = board.generate_moves();
    if let Some(m) = moves.first() {
        format!(
            "{}{}{}",
            Board::index_to_coord_algebraic(m.from),
            Board::index_to_coord_algebraic(m.to),
            match m.promotion {
                Some(crate::types::PieceType::Queen) => "q",
                _ => "",
            }
        )
    } else {
        "0000".to_string() // Null move si no hay movimientos (Mate/Ahogado)
    }
}

fn log_to_file(msg: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("debug.log")
        .unwrap(); // Si falla el log, que explote todo (para avisarte)

    if let Err(e) = writeln!(file, "{}", msg) {
        eprintln!("No se pudo escribir en el log: {}", e);
    }
}

#[cfg(test)]
mod tests {
    mod board_tests;
}
