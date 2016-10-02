use ::square::*;
use ::piece::*;

pub struct Move {
    moving_piece: Piece,
    from: Square,
    to: Square,
    captured_piece: Option<Piece>,
}

impl Move {
    pub fn new(moving_piece: Piece,
               from: Square,
               to: Square,
               captured_piece: Option<Piece>)
               -> Move {
        Move {
            moving_piece: moving_piece,
            from: from,
            to: to,
            captured_piece: captured_piece,
        }
    }
}
