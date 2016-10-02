extern crate rusty_board;
use rusty_board::board::*;
use rusty_board::square::*;
use rusty_board::generator::*;

#[test]
fn test_board() {
    let b = Board::new();
    let moves = generate_pseudo_legal_moves(b);
    println!("{:?}", moves);
    assert_eq!(moves.len(), 20);
}
