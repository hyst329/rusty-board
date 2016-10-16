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
                    PieceKind::Pawn => {
                        let t1 = s.get_forward(p.get_color()).unwrap();
                        let t2 = t1.get_forward(p.get_color());
                        if t2.is_some() {
                            let t2 = t2.unwrap();
                            // no need to promotion
                            if let None = board.get_piece(t1) {
                                let m = Move::new(p, s, t1, None, None, false, false);
                                res.push(m);
                                if (s.get_rank() == Rank::Second &&
                                    p.get_color() == Color::White) ||
                                    (s.get_rank() == Rank::Seventh &&
                                        p.get_color() == Color::Black) &&
                                        board.get_piece(t2).is_none() {
                                    // move 2 squares from starting position
                                    let m = Move::new(p, s, t2, None, None, false, false);
                                    res.push(m);
                                }
                            }
                            for d in [Direction::Left, Direction::Right].iter() {
                                if let Some(t) = t1.get_by_dir(*d) {
                                    match board.get_piece(t) {
                                        Some(piece) if piece.get_color() != p.get_color() => {
                                            let m = Move::new(p,
                                                              s,
                                                              t,
                                                              Some(piece),
                                                              None,
                                                              false,
                                                              false);
                                            res.push(m);
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        } else {
                            for pr in [PieceKind::Knight,
                                PieceKind::Bishop,
                                PieceKind::Rook,
                                PieceKind::Queen]
                                .iter() {
                                if let None = board.get_piece(t1) {
                                    let m = Move::new(p, s, t1, None, Some(*pr), false, false);
                                    res.push(m);
                                }
                                for d in [Direction::Left, Direction::Right].iter() {
                                    if let Some(t) = t1.get_by_dir(*d) {
                                        match board.get_piece(t) {
                                            Some(piece) if piece.get_color() != p.get_color() => {
                                                let m = Move::new(p,
                                                                  s,
                                                                  t,
                                                                  Some(piece),
                                                                  Some(*pr),
                                                                  false,
                                                                  false);
                                                res.push(m);
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                        }
                    }
                    PieceKind::Knight => {
                        for t in s.get_knight_moves() {
                            if let Some(piece) = board.get_piece(t) {
                                if piece.get_color() != p.get_color() {
                                    let m = Move::new(p, s, t, Some(piece), None, false, false);
                                    res.push(m);
                                }
                            } else {
                                let m = Move::new(p, s, t, None, None, false, false);
                                res.push(m);
                            }
                        }
                    }
                    PieceKind::Bishop | PieceKind::Rook | PieceKind::Queen => {
                        let dirs: &[Direction] = match p.get_kind() {
                            PieceKind::Bishop => &bishop_dirs,
                            PieceKind::Rook => &rook_dirs,
                            PieceKind::Queen => &queen_dirs,
                            _ => unreachable!(),
                        };
                        let mut t;
                        for d in dirs {
                            t = s;
                            while let Some(u) = t.get_by_dir(*d) {
                                t = u;
                                if let Some(piece) = board.get_piece(t) {
                                    if piece.get_color() != p.get_color() {
                                        let m = Move::new(p, s, t, Some(piece), None, false, false);
                                        res.push(m);
                                    }
                                    break;
                                } else {
                                    let m = Move::new(p, s, t, None, None, false, false);
                                    res.push(m);
                                }
                            }
                        }
                    }
                    PieceKind::King => {
                        for d in &queen_dirs {
                            if let Some(t) = s.get_by_dir(*d) {
                                if let Some(piece) = board.get_piece(t) {
                                    if piece.get_color() != p.get_color() {
                                        let m = Move::new(p, s, t, Some(piece), None, false, false);
                                        res.push(m);
                                    }
                                } else {
                                    let m = Move::new(p, s, t, None, None, false, false);
                                    res.push(m);
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
    // TODO: Castling and en passant
    res
}
