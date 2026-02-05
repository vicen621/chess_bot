use std::fmt::Display;

pub type Square = usize; // 0-63 representing squares on the chessboard

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Piece {
    pub color: Color,
    pub piece_type: PieceType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    pub squares: [Option<Piece>; 64],
    pub turn: Color,
    pub castling_rights: String,
    pub en_passant_target: Option<Square>,
}

pub struct Move {
    pub from: Square,
    pub to: Square,
    pub promotion: Option<PieceType>,
}

impl Move {
    pub fn new(from: Square, to: Square) -> Self {
        Move { from, to, promotion: None }
    }

    pub fn with_promotion(from: Square, to: Square, promotion: PieceType) -> Self {
        Move {
            from,
            to,
            promotion: Some(promotion),
        }
    }
}

impl Board {
    pub fn initial_position() -> Self {
        Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    }

    pub fn from_fen(fen: &str) -> Result<Self, String> {
        let mut board = Board {
            squares: [None; 64],
            turn: Color::White, // Default, lo sobreescribiremos leyendo el FEN
            castling_rights: String::new(),
            en_passant_target: None,
        };
        // rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1

        // El FEN tiene 6 partes separadas por espacios:
        // 1. Piezas (rnbqk...)
        // 2. Turno (w/b)
        // 3. Enroques (KQkq)
        // 4. Peón al paso (-)
        // 5. Reloj 50 movidas
        // 6. Número de jugada
        let parts: Vec<&str> = fen.split_whitespace().collect();
        if parts.len() < 2 {
            return Err("FEN inválido: menos de 2 partes".to_string());
        }

        let pieces = parts[0];
        let mut rank = 7;
        let mut file = 0;
        for c in pieces.chars() {
            if c == '/' {
                rank -= 1;
                file = 0;
            } else if c.is_digit(10) {
                file += c.to_digit(10).unwrap() as usize;
            } else {
                let color = if c.is_uppercase() {
                    Color::White
                } else {
                    Color::Black
                };
                let piece_type = match c.to_ascii_lowercase() {
                    'p' => PieceType::Pawn,
                    'n' => PieceType::Knight,
                    'b' => PieceType::Bishop,
                    'r' => PieceType::Rook,
                    'q' => PieceType::Queen,
                    'k' => PieceType::King,
                    _ => return Err(format!("FEN inválido: pieza desconocida '{}'", c)),
                };
                let index = rank * 8 + file;
                board.squares[index] = Some(Piece { color, piece_type });
                file += 1;
            }
        }

        board.turn = match parts[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return Err("FEN inválido: turno desconocido".to_string()),
        };

        board.castling_rights = parts[2].to_string();

        if parts[3] != "-" {
            let file_char = parts[3].chars().next().unwrap();
            let rank_char = parts[3].chars().nth(1).unwrap();
            let file = (file_char as u8 - b'a') as usize;
            let rank = (rank_char as u8 - b'1') as usize;
            board.en_passant_target = Some(rank * 8 + file);
        }

        Ok(board)
    }

    pub fn generate_moves(&self) -> Vec<Move> {
        let pseudo_moves = self.generate_pseudo_moves();
        pseudo_moves
            .into_iter()
            .filter(|mv| {
                let mut temp_board = self.clone();
                temp_board.make_move(mv);
                // Después de hacer el movimiento, el rey del turno actual NO debe estar en jaque
                !temp_board.is_king_attacked(self.turn)
            })
            .collect()
    }

    fn generate_pseudo_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();

        for (index, square) in self.squares.iter().enumerate() {
            if let Some(piece) = square {
                if piece.color == self.turn {
                    match piece.piece_type {
                        PieceType::Pawn => {
                            self.gen_pawn_moves(index, &mut moves);
                        }
                        PieceType::Knight => {
                            self.gen_knight_moves(index, &mut moves);
                        }
                        PieceType::Bishop => {
                            let directions = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
                            self.gen_sliding_moves(index, &mut moves, &directions);
                        }
                        PieceType::Rook => {
                            let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                            self.gen_sliding_moves(index, &mut moves, &directions);
                        }
                        PieceType::Queen => {
                            let directions = [
                                (-1, -1),
                                (-1, 0),
                                (-1, 1),
                                (0, -1),
                                (0, 1),
                                (1, -1),
                                (1, 0),
                                (1, 1),
                            ];
                            self.gen_sliding_moves(index, &mut moves, &directions);
                        }
                        PieceType::King => {
                            self.gen_king_moves(index, &mut moves);
                        }
                    }
                }
            }
        }
        moves
    }

    fn gen_knight_moves(&self, index: Square, moves: &mut Vec<Move>) {
        let (rank, file) = self.index_to_coord(index);
        let jumps = [
            (-2, -1),
            (-2, 1),
            (-1, -2),
            (-1, 2),
            (1, -2),
            (1, 2),
            (2, -1),
            (2, 1),
        ];

        for (delta_rank, delta_file) in jumps.iter() {
            let to_rank = rank as isize + delta_rank;
            let to_file = file as isize + delta_file;

            if to_rank >= 0 && to_rank < 8 && to_file >= 0 && to_file < 8 {
                let to_index = self.coord_to_index(to_rank as Square, to_file as Square);
                let target_square = self.squares[to_index];

                match target_square {
                    Some(piece) => {
                        if piece.color != self.turn {
                            moves.push(Move::new(index, to_index))
                        }
                    } // No puede capturar sus propias piezas
                    _ => {
                        moves.push(Move::new(index, to_index));
                    }
                }
            }
        }
    }

    pub fn gen_king_moves(&self, index: Square, moves: &mut Vec<Move>) {
        let (rank, file) = self.index_to_coord(index);
        let directions = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        for (delta_rank, delta_file) in directions.iter() {
            let to_rank = rank as isize + delta_rank;
            let to_file = file as isize + delta_file;

            if to_rank >= 0 && to_rank < 8 && to_file >= 0 && to_file < 8 {
                let to_index = self.coord_to_index(to_rank as Square, to_file as Square);
                let target_square = self.squares[to_index];

                match target_square {
                    Some(piece) => {
                        if piece.color != self.turn {
                            moves.push(Move::new(index, to_index))
                        }
                    } // No puede capturar sus propias piezas
                    _ => {
                        moves.push(Move::new(index, to_index));
                    }
                }
            }
        }
    }

    fn gen_sliding_moves(
        &self,
        index: Square,
        moves: &mut Vec<Move>,
        directions: &[(isize, isize)],
    ) {
        let (rank, file) = self.index_to_coord(index);

        for (delta_rank, delta_file) in directions.iter() {
            let mut to_rank = rank as isize + delta_rank;
            let mut to_file = file as isize + delta_file;

            while to_rank >= 0 && to_rank < 8 && to_file >= 0 && to_file < 8 {
                let to_index = self.coord_to_index(to_rank as Square, to_file as Square);
                let target_square = self.squares[to_index];

                match target_square {
                    Some(piece) => {
                        if piece.color != self.turn {
                            moves.push(Move::new(index, to_index))
                        }
                        break; // No puede saltar piezas
                    }
                    None => {
                        moves.push(Move::new(index, to_index));
                    }
                }

                to_rank += delta_rank;
                to_file += delta_file;
            }
        }
    }

    fn gen_pawn_moves(&self, index: Square, moves: &mut Vec<Move>) {
        if let Some(piece) = self.squares[index] {
            let (rank, file) = self.index_to_coord(index);
            let (direction, start_rank, promotion_rank) = match piece.color {
                Color::White => (1, 1, 7),
                Color::Black => (-1, 6, 0),
            };

            // Movimiento hacia adelante
            let forward_rank = rank as isize + direction;
            if forward_rank >= 0 && forward_rank < 8 {
                let forward_index = self.coord_to_index(forward_rank as Square, file);
                if self.squares[forward_index].is_none() {
                    if forward_rank as Square == promotion_rank {
                        // Promoción sin captura
                        for &promo_piece in &[PieceType::Queen, PieceType::Rook, PieceType::Bishop, PieceType::Knight] {
                            moves.push(Move::with_promotion(index, forward_index, promo_piece));
                        }
                    } else {
                        moves.push(Move::new(index, forward_index));
                    }
                }
            }

            // Movimiento doble desde la posición inicial
            if rank == start_rank {
                let double_forward_rank = rank as isize + 2 * direction;
                let double_forward_index =
                    self.coord_to_index(double_forward_rank as Square, file);
                if self.squares[double_forward_index].is_none() && self.squares[self.coord_to_index(forward_rank as Square, file)].is_none() {
                    moves.push(Move::new(index, double_forward_index));
                }
            }

            // Capturas diagonales
            for &delta_file in &[-1, 1] {
                let capture_file = file as isize + delta_file;
                if capture_file >= 0 && capture_file < 8 {
                    let capture_rank = rank as isize + direction;
                    if capture_rank >= 0 && capture_rank < 8 {
                        let capture_index = self.coord_to_index(capture_rank as Square, capture_file as Square);
                        if let Some(target_piece) = self.squares[capture_index] {
                            if target_piece.color != piece.color {
                                if capture_rank as Square == promotion_rank {
                                    // Promoción con captura
                                    for &promo_piece in &[PieceType::Queen, PieceType::Rook, PieceType::Bishop, PieceType::Knight] {
                                        moves.push(Move::with_promotion(index, capture_index, promo_piece));
                                    }
                                } else {
                                    moves.push(Move::new(index, capture_index));
                                }
                            }
                        } else if let Some(ep_target) = self.en_passant_target {
                            // Captura al paso (en passant)
                            if ep_target == capture_index {
                                moves.push(Move::new(index, capture_index));
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn make_move(&mut self, mv: &Move) {
        let piece = self.squares[mv.from].take().unwrap();

        // Captura al paso
        if piece.piece_type == PieceType::Pawn && mv.to == self.en_passant_target.unwrap_or(64) {
            let (to_rank, to_file) = self.index_to_coord(mv.to);
            let captured_pawn_rank = match self.turn {
                Color::White => to_rank - 1,
                Color::Black => to_rank + 1,
            };
            let captured_pawn_index = self.coord_to_index(captured_pawn_rank, to_file);
            self.squares[captured_pawn_index] = None; // Remover el peón capturado
        }

        self.en_passant_target = None; // Resetear objetivo al paso

        // si el movimiento es un doble avance de peón, establecer el objetivo al paso
        if piece.piece_type == PieceType::Pawn {
            let (from_rank, _) = self.index_to_coord(mv.from);
            let (to_rank, _) = self.index_to_coord(mv.to);
            if (piece.color == Color::White && from_rank == 1 && to_rank == 3)
                || (piece.color == Color::Black && from_rank == 6 && to_rank == 4)
            {
                // Movimiento doble de peón
                let ep_rank = (from_rank + to_rank) / 2;
                let ep_file = mv.to % 8;
                self.en_passant_target = Some(self.coord_to_index(ep_rank, ep_file));
            }
        }

        // si el movimiento es una promoción, colocar la pieza promovida, sino mover la pieza normalmente
        if mv.promotion.is_some() {
            let promo_piece = Piece {
                color: piece.color,
                piece_type: mv.promotion.unwrap(),
            };
            self.squares[mv.to] = Some(promo_piece);
        } else {
            self.squares[mv.to] = Some(piece);
        }

        // Cambiar el turno
        self.turn = match self.turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
    }

    fn find_king(&self, color: Color) -> Option<Square> {
        for (index, square) in self.squares.iter().enumerate() {
            if let Some(piece) = square {
                if piece.piece_type == PieceType::King && piece.color == color {
                    return Some(index);
                }
            }
        }
        None
    }

    /// verifica si el rey del turno actual está en jaque
    fn is_in_check(&self) -> bool {
        let king_index = match self.find_king(self.turn) {
            Some(index) => index,
            None => return false, // No hay rey, no puede estar en jaque
        };

        let mut temp_board = self.clone();
        temp_board.turn = match self.turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };

        let opponent_moves = temp_board.generate_pseudo_moves();

        for mv in opponent_moves {
            if mv.to == king_index {
                return true;
            }
        }

        false
    }

    fn is_king_attacked(&self, color: Color) -> bool {
        let king_pos = match self.find_king(color) {
            Some(p) => p,
            None => return false,
        };

        // Generamos movimientos del turno actual (que es el enemigo del 'color')
        let moves = self.generate_pseudo_moves();
        for m in moves {
            if m.to == king_pos {
                return true;
            }
        }
        false
    }

    pub fn coord_to_index(&self, rank: Square, file: Square) -> Square {
        rank * 8 + file
    }

    pub fn get_at_square(&self, index: Square) -> Option<Piece> {
        self.squares[index]
    }

    pub fn get_at(&self, rank: Square, file: Square) -> Option<Piece> {
        let index = self.coord_to_index(rank, file);
        self.squares[index]
    }

    pub fn index_to_coord(&self, index: Square) -> (Square, Square) {
        (index / 8, index % 8) // rank, file
    }

    pub fn index_to_coord_algebraic(index: Square) -> String {
        let (rank, file) = (index / 8, index % 8);
        let file_char = (b'a' + file as u8) as char;
        let rank_char = (b'1' + rank as u8) as char;
        format!("{}{}", file_char, rank_char)
    }

    // Cuenta cuántos nodos hoja existen a una profundidad dada
    pub fn perft(&self, depth: u32) -> u64 {
        if depth == 0 {
            return 1;
        }

        let moves = self.generate_moves();
        let mut nodes = 0;

        for m in moves {
            let mut board_copy = self.clone();
            board_copy.make_move(&m);
            nodes += board_copy.perft(depth - 1);
        }
        nodes
    }

    // Imprime el conteo de nodos para cada movimiento raíz
    pub fn perft_divide(&self, depth: u32) {
        let moves = self.generate_moves();
        let mut total_nodes = 0;

        for m in moves {
            let mut board_copy = self.clone();
            board_copy.make_move(&m);
            let nodes = board_copy.perft(depth - 1);

            // Imprimimos en formato "e2e4: 20"
            println!("{}: {}",
                format!("{}{}",
                    Board::index_to_coord_algebraic(m.from),
                    Board::index_to_coord_algebraic(m.to)
                ),
                nodes
            );
            total_nodes += nodes;
        }
        println!("\nTotal Nodes: {}", total_nodes);
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rank in (0..8).rev() {
            write!(f, "{}  ", rank + 1)?;
            for file in 0..8 {
                let piece = self.get_at(rank, file);
                let piece_str = match piece {
                    Some(piece) => match piece.piece_type {
                        PieceType::Pawn => "P",
                        PieceType::Knight => "N",
                        PieceType::Bishop => "B",
                        PieceType::Rook => "R",
                        PieceType::Queen => "Q",
                        PieceType::King => "K",
                    },
                    None => ".",
                };
                let piece_str = match piece {
                    Some(p) if p.color == Color::White => piece_str.to_uppercase(),
                    Some(_) => piece_str.to_lowercase(),
                    None => piece_str.to_string(),
                };
                write!(f, "{} ", piece_str)?;
            }
            writeln!(f)?;
        }
        writeln!(f)?;
        write!(f, "   a b c d e f g h")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- HELPERS PARA TESTEAR CÓMODAMENTE ---

    // Convierte notación algebraica ("a1", "h8") a índice (0..63)
    // Esto hace que los tests sean mucho más fáciles de leer.
    fn s(square: &str) -> usize {
        let col = square.chars().nth(0).unwrap() as usize - 'a' as usize;
        let row = square.chars().nth(1).unwrap() as usize - '1' as usize;
        row * 8 + col
    }

    // Verifica si un movimiento existe en la lista generada
    fn contains_move(moves: &Vec<Move>, from: &str, to: &str) -> bool {
        let f = s(from);
        let t = s(to);
        moves.iter().any(|m| m.from == f && m.to == t)
    }

    // --- TESTS DE FEN Y PARSING ---

    #[test]
    fn test_fen_parsing_initial() {
        let board = Board::initial_position();
        // Verificar esquinas y reyes
        assert_eq!(
            board.get_at_square(s("e1")).unwrap().piece_type,
            PieceType::King
        );
        assert_eq!(
            board.get_at_square(s("a1")).unwrap().piece_type,
            PieceType::Rook
        );
        assert_eq!(
            board.get_at_square(s("h8")).unwrap().piece_type,
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
            board.get_at_square(s("e4")).unwrap().piece_type,
            PieceType::Rook
        );
        assert_eq!(board.get_at_square(s("e4")).unwrap().color, Color::White);
        assert!(board.get_at_square(s("a1")).is_none());
    }

    // --- TESTS DE MOVIMIENTO ---

    #[test]
    fn test_knight_moves_center() {
        // Caballo en d4 (centro), debe tener 8 saltos
        let board = Board::from_fen("8/8/8/8/3N4/8/8/8 w - - 0 1").unwrap();
        let moves = board.generate_pseudo_moves();

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
        let moves = board.generate_pseudo_moves();

        assert_eq!(moves.len(), 2);
        assert!(contains_move(&moves, "a1", "b3"));
        assert!(contains_move(&moves, "a1", "c2"));
    }

    #[test]
    fn test_king_moves() {
        // Rey en e4, rodeado de vacío. 8 movimientos.
        let board = Board::from_fen("8/8/8/8/4K3/8/8/8 w - - 0 1").unwrap();
        let moves = board.generate_pseudo_moves();
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
        let moves = board.generate_pseudo_moves();

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
        let moves = board.generate_pseudo_moves();

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
        let moves = board.generate_pseudo_moves();

        // 7 verticales + 7 horizontales + 7 diagonal principal + 6 diagonal secundaria = 27 movimientos
        assert_eq!(moves.len(), 27);
    }

    // --- TESTS DE PEONES ---

    #[test]
    fn test_pawn_white_basic_movement() {
        // Peón blanco en e2 (posición inicial).
        // Debe poder mover a e3 (1 paso) y e4 (2 pasos).
        let board = Board::from_fen("8/8/8/8/8/8/4P3/8 w - - 0 1").unwrap();
        let moves = board.generate_pseudo_moves();

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
        let moves = board.generate_pseudo_moves();

        assert!(contains_move(&moves, "e7", "e6"));
        assert!(contains_move(&moves, "e7", "e5"));
    }

    #[test]
    fn test_pawn_no_double_jump_if_moved() {
        // Peón blanco en e3 (ya se movió).
        // Solo puede ir a e4. NO a e5.
        let board = Board::from_fen("8/8/8/8/8/4P3/8/8 w - - 0 1").unwrap();
        let moves = board.generate_pseudo_moves();

        assert!(contains_move(&moves, "e3", "e4"));
        assert!(!contains_move(&moves, "e3", "e5"));
    }

    #[test]
    fn test_pawn_blocked() {
        // Peón blanco en e2, peón negro en e3.
        // El peón blanco está totalmente bloqueado.
        let board = Board::from_fen("8/8/8/8/8/4p3/4P3/8 w - - 0 1").unwrap();
        let moves = board.generate_pseudo_moves();

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
        let moves = board.generate_pseudo_moves();

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
        let moves = board.generate_pseudo_moves();

        // Asumimos que hay algo en g3 para comer
        assert!(contains_move(&moves, "h2", "g3"));

        // Verificar que no genera basura fuera del tablero
        // (Esto depende de tu implementación, pero no debería crashear ni generar a3)
        let invalid_capture = moves.iter().any(|m| m.to == s("a3"));
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
            .filter(|m| m.from == s("a7") && m.to == s("a8"))
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
            .filter(|m| m.from == s("b7") && m.to == s("a8"))
            .collect();

        assert_eq!(captures.len(), 4, "Debería haber 4 formas de capturar coronando");
    }

    #[test]
    fn test_make_move_promotion() {
        // Ejecutar una promoción a Reina
        let mut board = Board::from_fen("8/P7/8/8/8/8/8/K7 w - - 0 1").unwrap();

        // Creamos el movimiento manual (ajusta según tu constructor)
        // Asumiendo que agregaste un campo `promotion` a tu Move
        // let m = Move { from: s("a7"), to: s("a8"), promotion: Some(PieceType::Queen) };

        // SI AÚN NO TIENES EL CAMPO EN EL CONSTRUCTOR, COMENTA ESTE TEST HASTA TENERLO
        /*
        board.make_move(m);

        let piece = board.get_at_square(s("a8")).unwrap();
        assert_eq!(piece.piece_type, PieceType::Queen, "El peón debería ser ahora una Reina");
        assert_eq!(piece.color, Color::White);
        assert!(board.get_at_square(s("a7")).is_none(), "La casilla original debe estar vacía");
        */
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
        board.en_passant_target = Some(s("d6"));

        let ep_move = Move::new(s("e5"), s("d6"));
        board.make_move(&ep_move);

        // 1. El peón blanco debe estar en d6
        assert_eq!(board.get_at_square(s("d6")).unwrap().piece_type, PieceType::Pawn);

        // 2. La casilla e5 debe estar vacía
        assert!(board.get_at_square(s("e5")).is_none());

        // 3. CRÍTICO: El peón negro en d5 (el capturado) debe haber DESAPARECIDO
        assert!(board.get_at_square(s("d5")).is_none(), "El peón capturado al paso NO fue eliminado del tablero");
    }

    #[test]
    fn test_double_push_sets_en_passant_target() {
        // Verificar que si muevo un peón 2 pasos, se setea la bandera para el siguiente turno
        let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

        // Movemos e2 -> e4
        let m = Move::new(s("e2"), s("e4"));
        board.make_move(&m);

        // El target debe ser e3
        assert_eq!(board.en_passant_target, Some(s("e3")), "Mover e2-e4 debería activar e3 como target");
    }

    #[test]
    fn test_en_passant_rights_expire() {
        // Si hago una jugada que no es capturar al paso, el derecho debe desaparecer.
        let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

        // 1. Blanco mueve e2-e4 (Activa target e3)
        board.make_move(&Move::new(s("e2"), s("e4")));
        assert_eq!(board.en_passant_target, Some(s("e3")));

        // 2. Negro hace una jugada cualquiera (h7-h6)
        // Al terminar este turno, el target e3 debe limpiarse porque nadie lo aprovechó.
        board.make_move(&Move::new(s("h7"), s("h6")));

        assert_eq!(board.en_passant_target, None, "El derecho de en passant debe expirar tras un turno");
    }
}
