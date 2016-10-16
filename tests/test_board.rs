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
    let e2 = Square::from_file_and_rank(File::E, Rank::Second);
    let e4 = Square::from_file_and_rank(File::E, Rank::Fourth);
    let m = Move::new(white_pawn, e2, e4, None, None, false, false);
    assert_eq!(b.do_move_inplace(m), Ok(()));
    assert_eq!(b.get_piece(e4), Some(white_pawn));
}
