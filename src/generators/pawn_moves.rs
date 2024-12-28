type BitBoard = u64;
struct PawnMoves {
    pub white_forward_moves: [BitBoard; 64],
    pub white_capture_moves: [BitBoard; 64],
}

impl PawnMoves {
    pub fn initialize() {
        let mut forward_moves = [Vec::new(); 64];
        let mut capture_moves = [Vec::new(); 64];
        let mut double_moves = [Vec::new(); 64];

        for square in 0..64 {
            let rank = square / 8;
            let file = square % 8;

            if rank == 0 || rank == 7 {
                continue;
            }

            if rank < 7 {
                forward_moves[square].push(square + 8); // forward move

                if rank == 1 {
                    forward_moves[square].push(square + 16); // double move
                }

                if file > 0 {
                    capture_moves[square].push(square + 7); // capture left
                }
                if file < 7 {
                    capture_moves[square].push(square + 9); // capture right
                }
            }

            if rank > 0 {
                forward_moves[square].push(square - 8); // forward move

                if rank == 6 {
                    forward_moves[square].push(square - 16); // double move
                }

                if file > 0 {
                    capture_moves[square].push(square - 9); // capture left
                }
                if file < 7 {
                    capture_moves[square].push(square - 7); // capture right
                }
            }
        }
    }
}
