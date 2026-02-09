use crate::types::*;

#[test]
fn test_castling_rights_from_fen() {
    let fen = "-";
    let rights = CastlingRights::from_fen(fen);
    assert!(!rights.white_kingside);
    assert!(!rights.white_queenside);
    assert!(!rights.black_kingside);
    assert!(!rights.black_queenside);
}

#[test]
fn test_castling_rights_from_fen_white_king_side_black_queen_side() {
    let fen = "Kq";
    let rights = CastlingRights::from_fen(fen);
    assert!(rights.white_kingside);
    assert!(!rights.white_queenside);
    assert!(!rights.black_kingside);
    assert!(rights.black_queenside);
}
