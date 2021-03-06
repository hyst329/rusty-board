use ::square::*;
use ::piece::*;

#[derive(Debug, Clone, Copy)]
pub struct Move {
    moving_piece: Piece,
    from: Square,
    to: Square,
    captured_piece: Option<Piece>,
    promoted_to: Option<PieceKind>,
    en_passant: bool,
    castling: bool,
}

impl Move {
    pub fn new(moving_piece: Piece,
               from: Square,
               to: Square,
               captured_piece: Option<Piece>,
               promoted_to: Option<PieceKind>,
               en_passant: bool,
               castling: bool)
               -> Move {
        Move {
            moving_piece: moving_piece,
            from: from,
            to: to,
            captured_piece: captured_piece,
            promoted_to: promoted_to,
            en_passant: en_passant,
            castling: castling,
        }
    }

    pub fn get_moving_piece(&self) -> Piece {
        self.moving_piece
    }
    pub fn get_square_from(&self) -> Square {
        self.from
    }
    pub fn get_square_to(&self) -> Square {
        self.to
    }
    pub fn get_captured_piece(&self) -> Option<Piece> {
        self.captured_piece
    }
    pub fn get_promoted_to(&self) -> Option<PieceKind> {
        self.promoted_to
    }
    pub fn is_en_passant(&self) -> bool {
        self.en_passant
    }
    pub fn is_castling(&self) -> bool {
        self.castling
    }
    pub fn is_normal_move(&self) -> bool {
        !(self.castling || self.en_passant)
    }
}
