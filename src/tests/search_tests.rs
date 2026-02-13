use crate::{
    search::search_best_move,
    types::{Board, Square},
};

fn get_best_move_coords(fen: &str, depth: u32) -> (Square, Square) {
    let board = Board::from_fen(fen).expect("FEN inválido");
    let best_move = search_best_move(&board, depth).expect("Debe encontrar un movimiento");
    (best_move.from, best_move.to)
}

#[test]
fn test_mate_in_one() {
    // Posición: Mate del Pastor (Dama blanca en f3, Alfil en c4, peón negro en f7 indefenso)
    // FEN: r1bqk1nr/pppp1ppp/2n5/2b1p3/2B1P3/5Q2/PPPP1PPP/RNB1K1NR w KQkq - 0 1
    // Mejor jugada: Qxf7# (f3 -> f7)

    let fen = "r1bqk1nr/pppp1ppp/2n5/2b1p3/2B1P3/5Q2/PPPP1PPP/RNB1K1NR w KQkq - 0 1";
    let (from, to) = get_best_move_coords(fen, 2);

    // f3 es índice 21, f7 es índice 53
    let f3 = 21;
    let f7 = 53;

    assert_eq!(from, f3, "La dama debería moverse desde f3");
    assert_eq!(to, f7, "La dama debería atacar f7 para mate");
}

#[test]
fn test_dont_eat_protected_piece() {
    // Situación: Dama Blanca en d1. Peón Negro en d4. Torre Negra en d8 defendiendo al peón.
    // Depth 1: "Como el peón (+100 puntos)". ERROR -> Luego pierdo la Dama (-900).
    // Depth 3: "Si como el peón, me comen la dama. Mejor hago otra cosa".

    let fen = "3r4/8/8/8/3p4/8/8/3Q4 w - - 0 1";

    // --- PRUEBA CON DEPTH 1 ---
    // Con depth 1, solo ve la captura inmediata.
    let (from_d1, to_d1) = get_best_move_coords(fen, 1);
    // d1=3, d4=27
    assert_eq!(from_d1, 3);
    assert_eq!(
        to_d1, 27,
        "A profundidad 1, el bot debería ser codicioso y comer el peón suicida"
    );

    // --- PRUEBA CON DEPTH 3 ---
    // Con depth 3, ve la respuesta del oponente.
    let (from_d3, to_d3) = get_best_move_coords(fen, 3);

    // No debe mover a d4 (27). Cualquier otro sitio es mejor.
    assert_ne!(
        to_d3, 27,
        "A profundidad 3, el bot NO debería comer el peón protegido"
    );
}

#[test]
fn test_mate_in_two_sacrifice() {
    // Posición clásica de mate de pasillo con sacrificio.
    // Qb8+?? sería respondido con Rxb8 (perdiendo la dama).
    // La única ganadora es 1. Qe8+! Rxe8 2. Rxe8#

    let fen = "r5k1/5ppp/8/4Q3/8/8/8/4R1K1 w - - 0 1";

    let (from, to) = get_best_move_coords(fen, 3);

    // e5=36, e8=60
    println!("Best move: from {} to {}", from, to);
    assert_eq!(from, 36, "La dama debe moverse desde e5");
    assert_eq!(
        to, 60,
        "El bot debería sacrificar la dama en e8 (única forma de mate)"
    );
}

#[test]
fn test_defend_mate() {
    let fen = "8/8/8/8/8/8/1q6/k1K5 w - - 0 1"; // Rey blanco c1, Dama negra b2 (Jaque)
    // Única legal: c1d1

    let (from, to) = get_best_move_coords(fen, 2);

    // c1=2, d1=3
    assert_eq!(from, 2);
    assert_eq!(
        to, 3,
        "Debe encontrar el único movimiento legal para salir de jaque"
    );
}
