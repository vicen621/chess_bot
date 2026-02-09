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
            castling_rights: CastlingRights::default(), // Default, lo sobreescribiremos leyendo el FEN
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

        board.castling_rights = CastlingRights::from_fen(parts[2]);

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

        // Usamos el helper para obtener el color enemigo
        let opponent = self.turn.opposite();

        if self.castling_rights.can_castle(self.turn, true) {
            let f_sq = index + 1; // f1 o f8
            let g_sq = index + 2; // g1 o g8

            // 1. Vacías
            if self.squares[f_sq].is_none() && self.squares[g_sq].is_none() {
                // 2. No estoy en jaque ahora
                if !self.is_square_attacked(index, opponent) {
                    // 3. La casilla de paso (f) no está atacada
                    if !self.is_square_attacked(f_sq, opponent) {
                        // Nota: La casilla destino (g) la verificará el filtro general después
                        moves.push(Move::new(index, g_sq));
                    }
                }
            }
        }

        if self.castling_rights.can_castle(self.turn, false) {
            let d_sq = index - 1; // d1 o d8
            let c_sq = index - 2; // c1 o c8
            let b_sq = index - 3; // b1 o b8 (debe estar vacío también)

            if self.squares[d_sq].is_none() && self.squares[c_sq].is_none() && self.squares[b_sq].is_none() {
                if !self.is_square_attacked(index, opponent) && !self.is_square_attacked(d_sq, opponent) {
                    moves.push(Move::new(index, c_sq));
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

        // Si movemos el rey, perdemos ambos derechos de enroque
        if PieceType::King == piece.piece_type {
            self.castling_rights.remove_castling_rights(self.turn, true);
            self.castling_rights.remove_castling_rights(self.turn, false);
            let (to_rank, to_file) = self.index_to_coord(mv.to);

            if to_file == 6 {
                // Enroque corto
                let rook_from = self.coord_to_index(to_rank, 7);
                let rook_to = self.coord_to_index(to_rank, 5);
                self.squares[rook_to] = self.squares[rook_from].take();
            } else if to_file == 2 {
                // Enroque largo
                let rook_from = self.coord_to_index(to_rank, 0);
                let rook_to = self.coord_to_index(to_rank, 3);
                self.squares[rook_to] = self.squares[rook_from].take();
            }
        }

        // Si movemos una torre desde su posición inicial, perdemos el derecho de enroque correspondiente
        if PieceType::Rook == piece.piece_type {
            let (_, from_file) = self.index_to_coord(mv.from);
            if from_file == 0 {
                // Torre de la columna 'a'
                self.castling_rights.remove_castling_rights(self.turn, false);
            } else if from_file == 7 {
                // Torre de la columna 'h'
                self.castling_rights.remove_castling_rights(self.turn, true);
            }
        }

        // Si capturamos una torre en su posición inicial, el oponente pierde el derecho de enroque correspondiente
        if let Some(captured_piece) = self.squares[mv.to] {
            if captured_piece.piece_type == PieceType::Rook {
                let (_, to_file) = self.index_to_coord(mv.to);
                if to_file == 0 {
                    // Torre de la columna 'a'
                    self.castling_rights.remove_castling_rights(captured_piece.color, false);
                } else if to_file == 7 {
                    // Torre de la columna 'h'
                    self.castling_rights.remove_castling_rights(captured_piece.color, true);
                }
            }
        }

        // Mover la pieza (y manejar promoción si aplica)
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

    fn is_king_attacked(&self, color: Color) -> bool {
        let king_pos = match self.find_king(color) {
            Some(p) => p,
            None => return false,
        };

        // Usamos la nueva función que NO genera movimientos
        let attacker = match color {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };

        self.is_square_attacked(king_pos, attacker)
    }

    // Verifica si una casilla específica está siendo atacada por un color dado
    pub fn is_square_attacked(&self, square: Square, attacker: Color) -> bool {
        let (rank, file) = self.index_to_coord(square);

        // 1. Verificamos PEONES (Cuidado: miramos "hacia atrás" desde la perspectiva del atacante)
        let pawn_attack_rank = match attacker {
            Color::White => rank as isize - 1, // Si ataca el blanco desde abajo, el peón está abajo
            Color::Black => rank as isize + 1, // Si ataca el negro desde arriba, el peón está arriba
        };

        for &delta_file in &[-1, 1] {
            let attack_file = file as isize + delta_file;
            if pawn_attack_rank >= 0 && pawn_attack_rank < 8 && attack_file >= 0 && attack_file < 8 {
                let idx = self.coord_to_index(pawn_attack_rank as usize, attack_file as usize);
                if let Some(piece) = self.squares[idx] {
                    if piece.color == attacker && piece.piece_type == PieceType::Pawn {
                        return true;
                    }
                }
            }
        }

        // 2. Verificamos CABALLOS
        let knight_jumps = [(-2, -1), (-2, 1), (-1, -2), (-1, 2), (1, -2), (1, 2), (2, -1), (2, 1)];
        for (delta_rank, delta_file) in knight_jumps {
            let tr = rank as isize + delta_rank;
            let tf = file as isize + delta_file;
            if tr >= 0 && tr < 8 && tf >= 0 && tf < 8 {
                let idx = self.coord_to_index(tr as usize, tf as usize);
                if let Some(piece) = self.squares[idx] {
                    if piece.color == attacker && piece.piece_type == PieceType::Knight {
                        return true;
                    }
                }
            }
        }

        // 3. Verificamos REY (adyacente)
        let king_moves = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
        for (delta_rank, delta_file) in king_moves {
             let tr = rank as isize + delta_rank;
             let tf = file as isize + delta_file;
             if tr >= 0 && tr < 8 && tf >= 0 && tf < 8 {
                 let idx = self.coord_to_index(tr as usize, tf as usize);
                 if let Some(piece) = self.squares[idx] {
                     if piece.color == attacker && piece.piece_type == PieceType::King {
                         return true;
                     }
                 }
             }
        }

        // 4. Verificamos PIEZAS DESLIZANTES (Torre/Reina y Alfil/Reina)
        // Ortogonales (Torre/Reina)
        let straight_dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (delta_rank, delta_file) in straight_dirs {
            let mut tr = rank as isize + delta_rank;
            let mut tf = file as isize + delta_file;
            while tr >= 0 && tr < 8 && tf >= 0 && tf < 8 {
                let idx = self.coord_to_index(tr as usize, tf as usize);
                match self.squares[idx] {
                    Some(piece) => {
                        if piece.color == attacker && (piece.piece_type == PieceType::Rook || piece.piece_type == PieceType::Queen) {
                            return true;
                        }
                        break; // Bloqueado por cualquier pieza
                    },
                    None => {} // Sigue buscando
                }
                tr += delta_rank;
                tf += delta_file;
            }
        }

        // Diagonales (Alfil/Reina)
        let diag_dirs = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
        for (delta_rank, delta_file) in diag_dirs {
            let mut tr = rank as isize + delta_rank;
            let mut tf = file as isize + delta_file;
            while tr >= 0 && tr < 8 && tf >= 0 && tf < 8 {
                let idx = self.coord_to_index(tr as usize, tf as usize);
                match self.squares[idx] {
                    Some(piece) => {
                        if piece.color == attacker && (piece.piece_type == PieceType::Bishop || piece.piece_type == PieceType::Queen) {
                            return true;
                        }
                        break;
                    },
                    None => {}
                }
                tr += delta_rank;
                tf += delta_file;
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
