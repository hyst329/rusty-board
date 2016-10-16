extern crate rusty_board;
use rusty_board::board::*;
use rusty_board::square::*;
use rusty_board::moves::*;
use rusty_board::piece::*;

#[test]
fn test_board() {
    let mut b = Board::new();
    for i in 0..64 {
        let s = Square::from_int(i);
        if let Some(p) = b.get_piece(s) {
            println!("{:} {:?}", s, p);
        }
    }
    let white_pawn = Piece::new(PieceKind::Pawn, Color::White);
    let e2 = Square::from_str("e2").unwrap();
    let e4 = Square::from_str("e4").unwrap();
    let m = Move::new(white_pawn, e2, e4, None, None, false, false);
    assert_eq!(b.do_move_inplace(m), Ok(()));
    assert_eq!(b.get_piece(e4), Some(white_pawn));
    let e3 = Square::from_str("e3").unwrap();
    assert_eq!(b.get_en_passant_square(), Some(e3));
}
