use ::board::*;
use ::moves::*;
use ::piece::*;
use ::square::*;

pub fn generate_pseudo_legal_moves(board: Board) -> Vec<Move> {
    let mut res = Vec::new();
    let rook_dirs = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];
    let bishop_dirs = [Direction::UpLeft,
                       Direction::UpRight,
                       Direction::DownLeft,
                       Direction::DownRight];
    let queen_dirs = [Direction::Up,
                      Direction::Down,
                      Direction::Left,
                      Direction::Right,
                      Direction::UpLeft,
                      Direction::UpRight,
                      Direction::DownLeft,
                      Direction::DownRight];
    for i in 0..64 {
        let s = Square::from_int(i);
        match board.get_piece(s) {
            Some(p) if p.get_color() == board.get_side_to_move() => {
                match p.get_kind() {
                    PieceKind::Pawn => {}
                    PieceKind::Knight => {}
                    PieceKind::Bishop => {
                        let mut t;
                        for d in &bishop_dirs {
                            t = s;
                            while let Some(u) = t.get_by_dir(*d) {
                                t = u;
                                if let Some(piece) = board.get_piece(t) {
                                    if piece.get_color() != p.get_color() {
                                        let m = Move::new(p, s, t, Some(piece));
                                        res.push(m);
                                    }
                                    break;
                                }
                                let m = Move::new(p, s, t, None);
                                res.push(m);
                            }
                        }
                    }
                    PieceKind::Rook => {
                        let mut t;
                        for d in &rook_dirs {
                            t = s;
                            while let Some(u) = t.get_by_dir(*d) {
                                t = u;
                                if let Some(piece) = board.get_piece(t) {
                                    if piece.get_color() != p.get_color() {
                                        let m = Move::new(p, s, t, Some(piece));
                                        res.push(m);
                                    }
                                    break;
                                }
                                let m = Move::new(p, s, t, None);
                                res.push(m);
                            }
                        }
                    }
                    PieceKind::Queen => {
                        let mut t;
                        for d in &queen_dirs {
                            t = s;
                            while let Some(u) = t.get_by_dir(*d) {
                                t = u;
                                if let Some(piece) = board.get_piece(t) {
                                    if piece.get_color() != p.get_color() {
                                        let m = Move::new(p, s, t, Some(piece));
                                        res.push(m);
                                    }
                                    break;
                                }
                                let m = Move::new(p, s, t, None);
                                res.push(m);
                            }
                        }
                    }
                    PieceKind::King => {}
                }
            }
            _ => {}
        }
    }
    res
}
