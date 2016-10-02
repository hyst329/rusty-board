extern crate rusty_board;
use rusty_board::square::*;

#[test]
fn test_square() {
    let e5: Square = Square::from_file_and_rank(File::E, Rank::Fifth);
    assert_eq!(format!("{}", e5), "e5");
}

#[test]
fn test_adjacent() {
    let e5: Square = Square::from_file_and_rank(File::E, Rank::Fifth);
    let e6: Square = Square::from_file_and_rank(File::E, Rank::Sixth);
    let e4: Square = Square::from_file_and_rank(File::E, Rank::Fourth);
    let d5: Square = Square::from_file_and_rank(File::D, Rank::Fifth);
    let f5: Square = Square::from_file_and_rank(File::F, Rank::Fifth);

    assert_eq!(e5.get_up(), Some(e6));
    assert_eq!(e5.get_down(), Some(e4));
    assert_eq!(e5.get_left(), Some(d5));
    assert_eq!(e5.get_right(), Some(f5));

    assert_eq!(e5.get_forward(Color::White), Some(e6));
    assert_eq!(e5.get_forward(Color::Black), Some(e4));
    assert_eq!(e5.get_back(Color::White), Some(e4));
    assert_eq!(e5.get_back(Color::Black), Some(e6));

    let h8: Square = Square::from_file_and_rank(File::H, Rank::Eighth);
    let g8: Square = Square::from_file_and_rank(File::G, Rank::Eighth);
    let h7: Square = Square::from_file_and_rank(File::H, Rank::Seventh);

    assert_eq!(h8.get_up(), None);
    assert_eq!(h8.get_down(), Some(h7));
    assert_eq!(h8.get_left(), Some(g8));
    assert_eq!(h8.get_right(), None);

    let a1: Square = Square::from_file_and_rank(File::A, Rank::First);
    let b1: Square = Square::from_file_and_rank(File::B, Rank::First);
    let a2: Square = Square::from_file_and_rank(File::A, Rank::Second);

    assert_eq!(a1.get_up(), Some(a2));
    assert_eq!(a1.get_down(), None);
    assert_eq!(a1.get_left(), None);
    assert_eq!(a1.get_right(), Some(b1));
}
