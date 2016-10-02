use ::square::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
#[derive(Copy, Clone, Debug)]
pub struct Piece {
    kind: PieceKind,
    color: Color,
}

impl Piece {
    pub fn new(kind: PieceKind, color: Color) -> Piece {
        Piece {
            kind: kind,
            color: color,
        }
    }
    pub fn get_kind(&self) -> PieceKind {
        self.kind
    }
    pub fn get_color(&self) -> Color {
        self.color
    }
}
