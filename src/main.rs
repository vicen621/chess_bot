#[allow(dead_code)]
mod board;
#[allow(dead_code)]
mod evaluation;
#[allow(dead_code)]
mod search;
#[allow(dead_code)]
mod types;

use crate::{search::search_best_move, types::Board};
use std::{
    fs::OpenOptions,
    io::{self, BufRead, Write},
};

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
                                        let error_msg = format!(
                                            "ERROR CRÍTICO: Falló al parsear el movimiento en position startpos'{}' (turno {}). FEN actual: {}",
                                            mv,
                                            i + 1,
                                            board.to_fen()
                                        );
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
                                log_to_file(&format!(
                                    "ERROR CRÍTICO AL CARGAR POSITION FEN: {}",
                                    e
                                ));
                                log_to_file(&format!("FEN INTENTADO: {}", fen_str));
                            }
                        }
                    }
                    _ => {}
                }
            }
            "go" => {
                let best_move = search_best_move(&board, 4); // Profundidad de búsqueda fija

                let best_move_str = match best_move {
                    Some(mv) => mv.to_string(),
                    None => "0000".to_string(), // Movimiento nulo si no se encuentra ninguno
                };

                println!("bestmove {}", best_move_str);
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
    mod search_tests;
    mod types_tests;
}
