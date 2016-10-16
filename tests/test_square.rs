extern crate rusty_board;
use rusty_board::square::*;

#[test]
fn test_square() {
    let e5: Square = Square::from_str("e5").unwrap();
    assert_eq!(format!("{}", e5), "e5");
}

#[test]
fn test_adjacent() {
    let e5: Square = Square::from_str("e5").unwrap();
    let e6: Square = Square::from_str("e6").unwrap();
    let e4: Square = Square::from_str("e4").unwrap();
    let d5: Square = Square::from_str("d5").unwrap();
    let f5: Square = Square::from_str("f5").unwrap();

    assert_eq!(e5.get_up(), Some(e6));
    assert_eq!(e5.get_down(), Some(e4));
    assert_eq!(e5.get_left(), Some(d5));
    assert_eq!(e5.get_right(), Some(f5));

    assert_eq!(e5.get_forward(Color::White), Some(e6));
    assert_eq!(e5.get_forward(Color::Black), Some(e4));
    assert_eq!(e5.get_back(Color::White), Some(e4));
    assert_eq!(e5.get_back(Color::Black), Some(e6));

    let h8: Square = Square::from_str("h8").unwrap();
    let g8: Square = Square::from_str("g8").unwrap();
    let h7: Square = Square::from_str("h7").unwrap();

    assert_eq!(h8.get_up(), None);
    assert_eq!(h8.get_down(), Some(h7));
    assert_eq!(h8.get_left(), Some(g8));
    assert_eq!(h8.get_right(), None);

    let a1: Square = Square::from_str("a1").unwrap();
    let b1: Square = Square::from_str("b1").unwrap();
    let a2: Square = Square::from_str("a2").unwrap();

    assert_eq!(a1.get_up(), Some(a2));
    assert_eq!(a1.get_down(), None);
    assert_eq!(a1.get_left(), None);
    assert_eq!(a1.get_right(), Some(b1));

    assert_eq!(e5.get_knight_moves().len(), 8);
    assert_eq!(h8.get_knight_moves().len(), 2);
    assert_eq!(b1.get_knight_moves().len(), 3);
}
