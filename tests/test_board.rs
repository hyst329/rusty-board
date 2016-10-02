extern crate rusty_board;
use rusty_board::board::*;
use rusty_board::square::*;

#[test]
fn test_board() {
    let b = Board::new();
    for i in 0..64 {
        let s = Square::from_int(i);
        if let Some(p) = b.get_piece(s) {
            println!("{:} {:?}", s, p);
        }
    }
}
