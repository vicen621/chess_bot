use crate::types::*;
use std::fmt::Display;

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
                board.squares[index] = Some(Piece::new(color, piece_type));
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
            let promo_piece = Piece::new(piece.color, mv.promotion.unwrap());
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
