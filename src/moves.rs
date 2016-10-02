use ::square::*;
use ::piece::*;

#[derive(Debug)]
pub struct Move {
    moving_piece: Piece,
    from: Square,
    to: Square,
    captured_piece: Option<Piece>,
    promoted_to: Option<PieceKind>,
}

impl Move {
    pub fn new(moving_piece: Piece,
               from: Square,
               to: Square,
               captured_piece: Option<Piece>,
               promoted_to: Option<PieceKind>)
               -> Move {
        Move {
            moving_piece: moving_piece,
            from: from,
            to: to,
            captured_piece: captured_piece,
            promoted_to: promoted_to,
        }
    }
}
