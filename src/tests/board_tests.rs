use crate::types::*;

// --- HELPERS PARA TESTEAR CÓMODAMENTE ---

// Convierte notación algebraica ("a1", "h8") a índice (0..63)
// Esto hace que los tests sean mucho más fáciles de leer.
fn square(square: &str) -> usize {
    let col = square.chars().nth(0).unwrap() as usize - 'a' as usize;
    let row = square.chars().nth(1).unwrap() as usize - '1' as usize;
    row * 8 + col
}

// Verifica si un movimiento existe en la lista generada
fn contains_move(moves: &Vec<Move>, from: &str, to: &str) -> bool {
    let f = square(from);
    let t = square(to);
    moves.iter().any(|m| m.from == f && m.to == t)
}

// --- TESTS DE FEN Y PARSING ---

#[test]
fn test_fen_parsing_initial() {
    let board = Board::initial_position();
    // Verificar esquinas y reyes
    assert_eq!(
        board.get_at_square(square("e1")).unwrap().piece_type,
        PieceType::King
    );
    assert_eq!(
        board.get_at_square(square("a1")).unwrap().piece_type,
        PieceType::Rook
    );
    assert_eq!(
        board.get_at_square(square("h8")).unwrap().piece_type,
        PieceType::Rook
    );
    assert_eq!(board.turn, Color::White);
}

#[test]
fn test_fen_parsing_custom() {
    // Tablero vacío excepto torre blanca en e4
    let fen = "8/8/8/8/4R3/8/8/8 w - - 0 1";
    let board = Board::from_fen(fen).unwrap();
    assert_eq!(
        board.get_at_square(square("e4")).unwrap().piece_type,
        PieceType::Rook
    );
    assert_eq!(board.get_at_square(square("e4")).unwrap().color, Color::White);
    assert!(board.get_at_square(square("a1")).is_none());
}

// --- TESTS DE MOVIMIENTO ---

#[test]
fn test_knight_moves_center() {
    // Caballo en d4 (centro), debe tener 8 saltos
    let board = Board::from_fen("8/8/8/8/3N4/8/8/8 w - - 0 1").unwrap();
    let moves = board.generate_moves();

    assert_eq!(moves.len(), 8);
    assert!(contains_move(&moves, "d4", "e6"));
    assert!(contains_move(&moves, "d4", "c6"));
    assert!(contains_move(&moves, "d4", "f5"));
    assert!(contains_move(&moves, "d4", "b5"));
    // ... verificar el resto si se desea
}

#[test]
fn test_knight_moves_corner() {
    // Caballo en a1 (esquina), solo 2 saltos
    let board = Board::from_fen("8/8/8/8/8/8/8/N7 w - - 0 1").unwrap();
    let moves = board.generate_moves();

    assert_eq!(moves.len(), 2);
    assert!(contains_move(&moves, "a1", "b3"));
    assert!(contains_move(&moves, "a1", "c2"));
}

#[test]
fn test_king_moves() {
    // Rey en e4, rodeado de vacío. 8 movimientos.
    let board = Board::from_fen("8/8/8/8/4K3/8/8/8 w - - 0 1").unwrap();
    let moves = board.generate_moves();
    assert_eq!(moves.len(), 8);
    assert!(contains_move(&moves, "e4", "e5"));
    assert!(contains_move(&moves, "e4", "d3"));
}

#[test]
fn test_rook_sliding_and_capture() {
    // Torre blanca en d4.
    // Peón negro en d7 (capturable).
    // Peón blanco en d2 (bloqueo amigo).
    // Paredes a los lados.
    // FEN: 8/3p4/8/8/3R4/8/3P4/8 w - - 0 1
    let board = Board::from_fen("8/3p4/8/8/3R4/8/3P4/8 w - - 0 1").unwrap();
    let moves = board.generate_moves();

    // Arriba: d5, d6, d7 (captura). NO d8 (detrás del enemigo)
    assert!(contains_move(&moves, "d4", "d5"));
    assert!(contains_move(&moves, "d4", "d6"));
    assert!(contains_move(&moves, "d4", "d7")); // Captura
    assert!(!contains_move(&moves, "d4", "d8")); // No puede saltar

    // Abajo: d3. NO d2 (amigo), NO d1
    assert!(contains_move(&moves, "d4", "d3"));
    assert!(!contains_move(&moves, "d4", "d2"));

    // Lados: a4..h4 (7 movimientos horizontales)
    assert!(contains_move(&moves, "d4", "a4"));
    assert!(contains_move(&moves, "d4", "h4"));
}

#[test]
fn test_bishop_blocked() {
    // Alfil en c1, peón propio en b2. Está atrapado excepto hacia la derecha.
    // FEN: 8/8/8/8/8/8/1P6/2B5 w - - 0 1
    let board = Board::from_fen("8/8/8/8/8/8/1P6/2B5 w - - 0 1").unwrap();
    let moves = board.generate_moves();

    // No puede ir a a3 ni b2
    assert!(!contains_move(&moves, "c1", "b2"));
    assert!(!contains_move(&moves, "c1", "a3"));

    // Puede ir a d2, e3...
    assert!(contains_move(&moves, "c1", "d2"));
}

#[test]
fn test_queen_moves() {
    // Reina en d4. Debería tener movimientos de torre + alfil.
    // Tablero vacío.
    let board = Board::from_fen("8/8/8/8/3Q4/8/8/8 w - - 0 1").unwrap();
    let moves = board.generate_moves();

    // 7 verticales + 7 horizontales + 7 diagonal principal + 6 diagonal secundaria = 27 movimientos
    assert_eq!(moves.len(), 27);
}

// --- TESTS DE PEONES ---

#[test]
fn test_pawn_white_basic_movement() {
    // Peón blanco en e2 (posición inicial).
    // Debe poder mover a e3 (1 paso) y e4 (2 pasos).
    let board = Board::from_fen("8/8/8/8/8/8/4P3/8 w - - 0 1").unwrap();
    let moves = board.generate_moves();

    assert!(contains_move(&moves, "e2", "e3"));
    assert!(contains_move(&moves, "e2", "e4"));
    // No debe poder ir a los lados ni capturar a la nada
    assert!(!contains_move(&moves, "e2", "d3"));
    assert!(!contains_move(&moves, "e2", "f3"));
}

#[test]
fn test_pawn_black_basic_movement() {
    // Peón negro en e7 (posición inicial).
    // Debe poder mover a e6 (1 paso) y e5 (2 pasos).
    // Turno: b (negras)
    let board = Board::from_fen("8/4p3/8/8/8/8/8/8 b - - 0 1").unwrap();
    let moves = board.generate_moves();

    assert!(contains_move(&moves, "e7", "e6"));
    assert!(contains_move(&moves, "e7", "e5"));
}

#[test]
fn test_pawn_no_double_jump_if_moved() {
    // Peón blanco en e3 (ya se movió).
    // Solo puede ir a e4. NO a e5.
    let board = Board::from_fen("8/8/8/8/8/4P3/8/8 w - - 0 1").unwrap();
    let moves = board.generate_moves();

    assert!(contains_move(&moves, "e3", "e4"));
    assert!(!contains_move(&moves, "e3", "e5"));
}

#[test]
fn test_pawn_blocked() {
    // Peón blanco en e2, peón negro en e3.
    // El peón blanco está totalmente bloqueado.
    let board = Board::from_fen("8/8/8/8/8/4p3/4P3/8 w - - 0 1").unwrap();
    let moves = board.generate_moves();

    // No puede avanzar 1 paso
    assert!(!contains_move(&moves, "e2", "e3"));
    // No puede avanzar 2 pasos (saltar por encima)
    assert!(!contains_move(&moves, "e2", "e4"));
}

#[test]
fn test_pawn_capture() {
    // Peón blanco en e4.
    // Peones negros en d5 y f5 (capturables).
    // Peón negro en e5 (bloqueo frontal).
    let board = Board::from_fen("8/8/8/3ppp2/4P3/8/8/8 w - - 0 1").unwrap();
    let moves = board.generate_moves();

    // Capturas diagonales
    assert!(contains_move(&moves, "e4", "d5"));
    assert!(contains_move(&moves, "e4", "f5"));

    // Bloqueado al frente
    assert!(!contains_move(&moves, "e4", "e5"));
}

#[test]
fn test_pawn_capture_border() {
    // Peón blanco en h2.
    // Solo puede capturar hacia la izquierda (g3).
    // Si intenta capturar a la "derecha" (columna i inexistente/a3 pacman), debe fallar.
    let board = Board::from_fen("8/8/8/8/8/6p1/7P/8 w - - 0 1").unwrap();
    let moves = board.generate_moves();

    // Asumimos que hay algo en g3 para comer
    assert!(contains_move(&moves, "h2", "g3"));

    // Verificar que no genera basura fuera del tablero
    // (Esto depende de tu implementación, pero no debería crashear ni generar a3)
    let invalid_capture = moves.iter().any(|m| m.to == square("a3"));
    assert!(!invalid_capture, "El peón hizo Pacman desde h2 a a3!");
}

// --- TESTS DE LEGALIDAD DE MOVIMIENTOS ---

#[test]
fn test_absolute_pin() {
    // Situación: Rey blanco en e1, Torre blanca en e2, Torre negra en e8.
    // La Torre blanca está CLAVADA. No puede moverse a los lados (d2, f2),
    // solo puede moverse verticalmente (e3...e8) manteniendo la protección.
    // FEN: 4r3/8/8/8/8/8/4R3/4K3 w - - 0 1
    let board = Board::from_fen("4r3/8/8/8/8/8/4R3/4K3 w - - 0 1").unwrap();
    let moves = board.generate_moves();

    // Movimientos legales de la torre e2:
    assert!(contains_move(&moves, "e2", "e3")); // Vertical OK
    assert!(contains_move(&moves, "e2", "e8")); // Capturar al atacante OK

    // Movimientos ILEGALES (exponen al rey):
    assert!(!contains_move(&moves, "e2", "d2")); // Horizontal ILEGAL
    assert!(!contains_move(&moves, "e2", "f2")); // Horizontal ILEGAL
}

#[test]
fn test_king_cannot_move_into_check() {
    // Rey blanco en e1. Torre negra en a2.
    // El rey NO puede subir a la fila 2 porque está controlada por la torre.
    // FEN: 8/8/8/8/8/8/r7/4K3 w - - 0 1
    let board = Board::from_fen("8/8/8/8/8/8/r7/4K3 w - - 0 1").unwrap();
    let moves = board.generate_moves();

    // Puede moverse por la fila 1
    assert!(contains_move(&moves, "e1", "f1"));
    assert!(contains_move(&moves, "e1", "d1"));

    // NO puede subir a la fila 2 (Suicidio)
    assert!(!contains_move(&moves, "e1", "e2"));
    assert!(!contains_move(&moves, "e1", "d2"));
    assert!(!contains_move(&moves, "e1", "f2"));
}

#[test]
fn test_check_evasion() {
    // Rey blanco en e1 está en JAQUE por Torre negra en e8.
    // El rey tiene que escapar o bloquear. No hay piezas para bloquear.
    // Solo puede mover a d1, f1, o subir a d2, f2 (e2 sigue atacado).
    // FEN: 4r3/8/8/8/8/8/8/4K3 w - - 0 1
    let board = Board::from_fen("4r3/8/8/8/8/8/8/4K3 w - - 0 1").unwrap();
    let moves = board.generate_moves();

    // Escapes válidos
    assert!(contains_move(&moves, "e1", "d1"));
    assert!(contains_move(&moves, "e1", "f1"));
    assert!(contains_move(&moves, "e1", "d2")); // La torre solo ataca columna E

    // Movimientos inválidos (sigue en jaque)
    assert!(!contains_move(&moves, "e1", "e2")); // Sigue en columna E
}

#[test]
fn test_checkmate_no_moves() {
    // Jaque mate del pasillo.
    // Rey blanco en a1, peones en a2, b2, c2 (encerrado).
    // Torre negra en d1 dando jaque.
    // Nadie puede comer a la torre, el rey no puede escapar, los peones no ayudan.
    // Resultado: 0 movimientos legales.
    // FEN: 6k1/8/8/8/8/8/PPP5/K2r4 w - - 0 1
    let board = Board::from_fen("6k1/8/8/8/8/8/PPP5/K2r4 w - - 0 1").unwrap();
    let moves = board.generate_moves();

    assert_eq!(moves.len(), 0, "Debería haber 0 movimientos en Jaque Mate");
}

#[test]
fn test_stalemate() {
    // Rey ahogado.
    // Rey blanco en a1. Reina negra en c2.
    // El rey NO está en jaque, pero no tiene a dónde ir.
    // FEN: 8/8/8/8/8/8/2q5/K7 w - - 0 1
    let board = Board::from_fen("8/8/8/8/8/8/2q5/K7 w - - 0 1").unwrap();
    let moves = board.generate_moves();

    // is_in_check debería ser false
    assert_eq!(moves.len(), 0, "Debería haber 0 movimientos en Rey Ahogado");
}

#[test]
fn test_perft_initial_position() {
    let board = Board::initial_position();

    // Profundidad 1: 20 movimientos posibles (16 peones + 4 caballos)
    board.perft_divide(1);
    assert_eq!(board.perft(1), 20);

    // Profundidad 2: 400 movimientos posibles
    board.perft_divide(2);
    assert_eq!(board.perft(2), 400);

    // Profundidad 3: 8,902 movimientos posibles
    // Aquí es donde suelen aparecer errores de clavadas o jaques sutiles
    board.perft_divide(3);
    assert_eq!(board.perft(3), 8902);

    // Profundidad 4: 197,281. (Puede tardar un par de segundos en modo debug)
    board.perft_divide(4);
    assert_eq!(board.perft(4), 197281);
}

// --- TESTS DE PROMOCIÓN ---

#[test]
fn test_pawn_promotion_generation() {
    // Peón blanco en a7 a punto de coronar.
    // FEN: 8/P7/8/8/8/8/8/8 w - - 0 1
    let board = Board::from_fen("8/P7/8/8/8/8/8/8 w - - 0 1").unwrap();
    let moves = board.generate_moves();

    // Debería haber 4 movimientos posibles desde a7 a a8
    // Uno para cada tipo de promoción (Q, R, B, N)
    let promotions: Vec<&Move> = moves.iter()
        .filter(|m| m.from == square("a7") && m.to == square("a8"))
        .collect();

    assert_eq!(promotions.len(), 4, "Debería haber 4 opciones de coronación");

    // Verificamos que existan los tipos específicos (si implementaste el campo promotion)
    // Nota: Ajusta esto según cómo hayas llamado al campo en tu struct Move
    /*
    assert!(promotions.iter().any(|m| m.promotion == Some(PieceType::Queen)));
    assert!(promotions.iter().any(|m| m.promotion == Some(PieceType::Knight)));
    */
}

#[test]
fn test_pawn_promotion_capture() {
    // Peón blanco en b7, torre negra en a8.
    // Puede coronar comiendo (b7xa8) o avanzando (b7-b8). Total 8 movimientos.
    // FEN: r7/1P6/8/8/8/8/8/8 w - - 0 1
    let board = Board::from_fen("r7/1P6/8/8/8/8/8/8 w - - 0 1").unwrap();
    let moves = board.generate_moves();

    let captures: Vec<&Move> = moves.iter()
        .filter(|m| m.from == square("b7") && m.to == square("a8"))
        .collect();

    assert_eq!(captures.len(), 4, "Debería haber 4 formas de capturar coronando");
}

#[test]
fn test_make_move_promotion() {
    // Ejecutar una promoción a Reina
    let mut board = Board::from_fen("8/P7/8/8/8/8/8/K7 w - - 0 1").unwrap();

    let m = Move::with_promotion(square("a7"), square("a8"), PieceType::Queen);

    board.make_move(&m);

    let piece = board.get_at_square(square("a8")).unwrap();
    assert_eq!(piece.piece_type, PieceType::Queen, "El peón debería ser ahora una Reina");
    assert_eq!(piece.color, Color::White);
    assert!(board.get_at_square(square("a7")).is_none(), "La casilla original debe estar vacía");
}

// --- TESTS DE EN PASSANT ---

#[test]
fn test_en_passant_opportunity() {
    // Situación: Peón blanco en e5. Peón negro acaba de mover d7-d5.
    // El FEN marca "d6" como casilla objetivo de en passant.
    // FEN: rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 1
    let board = Board::from_fen("rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 1").unwrap();
    let moves = board.generate_moves();

    // Debe existir el movimiento e5 -> d6 (captura al paso)
    assert!(contains_move(&moves, "e5", "d6"), "El generador no detectó la captura al paso");
}

#[test]
fn test_make_move_en_passant_removes_pawn() {
    // Ejecutamos la captura al paso y verificamos que el peón enemigo desaparezca.
    // Blanco en e5, Negro en d5. Objetivo ep: d6.
    let mut board = Board::from_fen("8/8/8/3pP3/8/8/8/8 w - - 0 1").unwrap();

    // Asignamos manualmente el target por si el FEN no lo parseó (aunque debería)
    board.en_passant_target = Some(square("d6"));

    let ep_move = Move::new(square("e5"), square("d6"));
    board.make_move(&ep_move);

    // 1. El peón blanco debe estar en d6
    assert_eq!(board.get_at_square(square("d6")).unwrap().piece_type, PieceType::Pawn);

    // 2. La casilla e5 debe estar vacía
    assert!(board.get_at_square(square("e5")).is_none());

    // 3. CRÍTICO: El peón negro en d5 (el capturado) debe haber DESAPARECIDO
    assert!(board.get_at_square(square("d5")).is_none(), "El peón capturado al paso NO fue eliminado del tablero");
}

#[test]
fn test_double_push_sets_en_passant_target() {
    // Verificar que si muevo un peón 2 pasos, se setea la bandera para el siguiente turno
    let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

    // Movemos e2 -> e4
    let m = Move::new(square("e2"), square("e4"));
    board.make_move(&m);

    // El target debe ser e3
    assert_eq!(board.en_passant_target, Some(square("e3")), "Mover e2-e4 debería activar e3 como target");
}

#[test]
fn test_en_passant_rights_expire() {
    // Si hago una jugada que no es capturar al paso, el derecho debe desaparecer.
    let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

    // 1. Blanco mueve e2-e4 (Activa target e3)
    board.make_move(&Move::new(square("e2"), square("e4")));
    assert_eq!(board.en_passant_target, Some(square("e3")));

    // 2. Negro hace una jugada cualquiera (h7-h6)
    // Al terminar este turno, el target e3 debe limpiarse porque nadie lo aprovechó.
    board.make_move(&Move::new(square("h7"), square("h6")));

    assert_eq!(board.en_passant_target, None, "El derecho de en passant debe expirar tras un turno");
}

// --- TESTS DE ENROQUE (CASTLING) ---

#[test]
fn test_castling_white_short_legal() {
    // FEN: Blancas pueden enrocar corto.
    // Rey en e1, Torre en h1. f1 y g1 vacíos.
    let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQK2R w KQkq - 0 1").unwrap();
    let moves = board.generate_moves();

    // Debe existir el movimiento e1 -> g1
    assert!(contains_move(&moves, "e1", "g1"), "El enroque corto blanco debería ser legal");
}

#[test]
fn test_castling_black_long_legal() {
    // FEN: Negras pueden enrocar largo.
    // Rey en e8, Torre en a8. b8, c8, d8 vacíos.
    let board = Board::from_fen("r3kbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1").unwrap();
    let moves = board.generate_moves();

    // Debe existir el movimiento e8 -> c8
    assert!(contains_move(&moves, "e8", "c8"), "El enroque largo negro debería ser legal");
}

#[test]
fn test_castling_blocked() {
    // FEN: Alfil blanco en f1 bloquea el enroque corto.
    let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKB1R w KQkq - 0 1").unwrap();
    let moves = board.generate_moves();

    assert!(!contains_move(&moves, "e1", "g1"), "No se puede enrocar si hay piezas en el medio");
}

#[test]
fn test_castling_cant_escape_check() {
    // FEN: El rey blanco está en JAQUE por una torre negra en e8.
    // Regla: No puedes enrocar para salir de jaque.
    let board = Board::from_fen("4r3/8/8/8/8/8/8/4K2R w K - 0 1").unwrap();
    let moves = board.generate_moves();

    assert!(!contains_move(&moves, "e1", "g1"), "No se puede enrocar estando en jaque");
}

#[test]
fn test_castling_through_check() {
    // FEN: Torre negra en f8 ataca la casilla f1 (casilla de paso).
    // El rey no está en jaque en e1, ni lo estaría en g1, pero f1 es "lava".
    // Regla: El rey no puede pasar por una casilla atacada.
    let board = Board::from_fen("5r2/8/8/8/8/8/8/4K2R w K - 0 1").unwrap();
    let moves = board.generate_moves();

    assert!(!contains_move(&moves, "e1", "g1"), "No se puede enrocar pasando por una casilla atacada (f1)");
}

#[test]
fn test_castling_rights_lost_after_king_move() {
    // 1. Tenemos derechos.
    let mut board = Board::from_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1").unwrap();
    assert!(contains_move(&board.generate_moves(), "e1", "g1"));

    // 2. Movemos el rey (e1 -> e2) y perdemos los derechos.
    // Nota: Esto asume que tu make_move actualiza castling_rights (si no lo hace, este test fallará y te recordará implementarlo).
    board.make_move(&Move::new(square("e1"), square("e2")));

    // 3. Movemos el rey de vuelta (e2 -> e1).
    board.make_move(&Move::new(square("e2"), square("e1")));

    // 4. Intentamos enrocar de nuevo. No debería dejarme.
    let moves = board.generate_moves();
    assert!(!contains_move(&moves, "e1", "g1"), "Perdiste el derecho al mover el rey");
    assert!(!contains_move(&moves, "e1", "c1"), "Perdiste el derecho al mover el rey");
}

#[test]
fn test_make_move_castling_moves_rook() {
    // Enroque corto blanco
    let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQK2R w KQkq - 0 1").unwrap();

    // Ejecutamos e1 -> g1
    let castling_move = Move::new(square("e1"), square("g1"));
    board.make_move(&castling_move);

    // 1. El rey debe estar en g1
    assert_eq!(board.get_at_square(square("g1")).unwrap().piece_type, PieceType::King);

    // 2. LA TORRE debe haberse teletransportado de h1 a f1
    assert!(board.get_at_square(square("h1")).is_none(), "La torre debe salir de h1");
    assert_eq!(board.get_at_square(square("f1")).unwrap().piece_type, PieceType::Rook, "La torre debe aparecer en f1");
}

// --- TESTS DE ACTUALIZACIÓN DE DERECHOS (CASTLING RIGHTS) ---

#[test]
fn test_king_move_removes_both_rights() {
    // 1. Empezamos con todos los derechos ("KQkq")
    // FEN inicial estándar
    let mut board = Board::initial_position();
    assert_eq!(board.castling_rights, CastlingRights::new(true, true, true, true));

    // 2. Movemos el Rey Blanco (e1 -> e2)
    let move_king = Move::new(square("e1"), square("e2"));
    board.make_move(&move_king);

    // 3. El blanco debería haber perdido K y Q. El negro conserva k y q.
    assert_eq!(board.castling_rights, CastlingRights::new(false, false, true, true), "Mover el rey blanco debe eliminar 'K' y 'Q'");

    // 4. Movemos el Rey Negro (e8 -> e7)
    let move_black_king = Move::new(square("e8"), square("e7"));
    board.make_move(&move_black_king);

    // 5. Ahora nadie debería tener derechos..
    assert_eq!(board.castling_rights, CastlingRights::default(), "Mover el rey negro debe eliminar 'k' y 'q'");
}

#[test]
fn test_rook_move_removes_specific_right() {
    // 1. Empezamos con "KQkq"
    let mut board = Board::initial_position();

    // 2. Movemos la Torre Blanca del lado de Rey (h1 -> h2)
    // Esto debería matar SOLO la 'K', manteniendo 'Q', 'k', 'q'.
    let move_rook_h1 = Move::new(square("h1"), square("h2"));
    board.make_move(&move_rook_h1);

    assert_eq!(board.castling_rights, CastlingRights::new(false, true, true, true), "Mover torre h1 debe eliminar solo 'K'");

    // 3. Movemos la Torre Negra del lado de Dama (a8 -> a7)
    // Turno negro (se hace un movimiento dummy blanco para cambiar turno o forzamos el turno si es necesario,
    // pero make_move cambia el turno automático, así que:
    // Turno 1 (Blanco): h1->h2.
    // Turno 2 (Negro): a8->a7.
    let move_rook_a8 = Move::new(square("a8"), square("a7"));
    board.make_move(&move_rook_a8);

    // Debería quedar "Qk" (Se fue 'K' antes, ahora se va 'q').
    assert_eq!(board.castling_rights, CastlingRights::new(false, true, true, false), "Mover torre a8 debe eliminar solo 'q'");
}

#[test]
fn test_rook_capture_removes_opponent_right() {
    // ESTE ES EL CASO DIFÍCIL
    // Situación: Torre negra en h2 lista para comerse a la torre blanca de h1.
    // Derechos iniciales: "KQkq".
    // FEN: r3k2r/7r/8/8/8/8/7P/R3K2R b KQkq - 0 1
    // (Juegan negras)
    let mut board = Board::from_fen("r3k2r/7r/8/8/8/8/7P/R3K2R b KQkq - 0 1").unwrap();

    // Verificamos que el blanco tiene derecho 'K'
    assert!(board.castling_rights.white_kingside);

    // Las negras capturan la torre de h1 (h2 -> h1)
    let capture_move = Move::new(square("h2"), square("h1"));
    board.make_move(&capture_move);

    // EL BLANCO DEBE PERDER EL DERECHO 'K' AUNQUE NO HAYA MOVIDO SU TORRE
    // (Porque ya no tiene torre en h1 para enrocar)
    assert!(!board.castling_rights.white_kingside, "Si te comen la torre de h1, pierdes el derecho 'K'");

    // Los otros derechos deben seguir intactos
    assert!(board.castling_rights.white_queenside);
    assert!(board.castling_rights.black_kingside);
    assert!(board.castling_rights.black_queenside);
}
