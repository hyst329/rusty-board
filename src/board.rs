use ::square::*;
use ::piece::*;
use ::moves::*;

/// Represents different castlings for different sides
pub enum CastlingRights {
    /// White can castle kingside
    WhiteKingside,
    /// White can castle queenside
    WhiteQueenside,
    /// Black can castle kingside
    BlackKingside,
    /// Black can castle queenside
    BlackQueenside,
}

#[derive(Clone)]
/// The main data type for chessboard
pub struct Board {
    board: Vec<Option<Piece>>,
    side_to_move: Color,
    move_number: u16,
    halfmove_count: u16,
    en_passant_square: Option<Square>,
    white_king: Square,
    black_king: Square,
    castling: [bool; 4],
    initial_king_file: File,
    initial_queens_rook_file: File,
    initial_kings_rook_file: File,
    move_list: Vec<Move>,
}

impl Board {
    /// Constructs a new chessboard with starting position.
    /// Equivalent to `Board::new_chess960(518)`.
    pub fn new() -> Board {
        Board::new_chess960(518)
    }
    /// Constructs a new chessboard with Chess960 starting position number `position_number`.
    /// Position numbers are treated modulo 960
    /// (i. e. #960, #961, #962 are equivalent to #0, #1, #2 etc.)
    pub fn new_chess960(position_number: u32) -> Board {
        let n1 = position_number % 960;
        let mut piece_order = [PieceKind::Pawn; 8];
        let mut indices: Vec<usize> = (0..8).collect();
        let n2 = n1 / 4;
        let b1 = n1 % 4;
        let n3 = n2 / 4;
        let b2 = n2 % 4;
        let b1idx = (2 * b1 + 1) as usize;
        let b2idx = (2 * b2) as usize;
        piece_order[b1idx] = PieceKind::Bishop;
        piece_order[b2idx] = PieceKind::Bishop;
        let b1pos = indices.iter().position(|&x| x == b1idx).unwrap();
        indices.remove(b1pos);
        let b2pos = indices.iter().position(|&x| x == b2idx).unwrap();
        indices.remove(b2pos);
        let n4 = n3 / 6;
        let q = n3 % 6;
        let qidx = indices[q as usize];
        indices.remove(q as usize);
        piece_order[qidx] = PieceKind::Queen;
        let kts = match n4 {
            0 => (0, 1),
            1 => (0, 2),
            2 => (0, 3),
            3 => (0, 4),
            4 => (1, 2),
            5 => (1, 3),
            6 => (1, 4),
            7 => (2, 3),
            8 => (2, 4),
            9 => (3, 4),
            _ => unreachable!(),
        };
        piece_order[indices[kts.0]] = PieceKind::Knight;
        piece_order[indices[kts.1]] = PieceKind::Knight;
        indices.remove(kts.0);
        indices.remove(kts.1 - 1); // vector gets shorter, so we must subtract one
        piece_order[indices[0]] = PieceKind::Rook;
        piece_order[indices[1]] = PieceKind::King;
        piece_order[indices[2]] = PieceKind::Rook;
        let mut board: [Option<Piece>; 64] = [None; 64];
        for i in 0..8 {
            board[i] = Some(Piece::new(piece_order[i], Color::White));
            board[8 + i] = Some(Piece::new(PieceKind::Pawn, Color::White));
            board[48 + i] = Some(Piece::new(PieceKind::Pawn, Color::Black));
            board[56 + i] = Some(Piece::new(piece_order[i], Color::Black));
        }
        let qr_file = match indices[0] {
            0 => File::A,
            1 => File::B,
            2 => File::C,
            3 => File::D,
            4 => File::E,
            5 => File::F,
            6 => File::G,
            7 => File::H,
            _ => unreachable!(),
        };
        let k_file = match indices[1] {
            0 => File::A,
            1 => File::B,
            2 => File::C,
            3 => File::D,
            4 => File::E,
            5 => File::F,
            6 => File::G,
            7 => File::H,
            _ => unreachable!(),
        };
        let kr_file = match indices[2] {
            0 => File::A,
            1 => File::B,
            2 => File::C,
            3 => File::D,
            4 => File::E,
            5 => File::F,
            6 => File::G,
            7 => File::H,
            _ => unreachable!(),
        };
        Board {
            board: board.to_vec(),
            side_to_move: Color::White,
            move_number: 1,
            halfmove_count: 0,
            en_passant_square: None,
            white_king: Square::from_file_and_rank(k_file, Rank::First),
            black_king: Square::from_file_and_rank(k_file, Rank::Eighth),
            castling: [true; 4],
            initial_king_file: k_file,
            initial_queens_rook_file: qr_file,
            initial_kings_rook_file: kr_file,
            move_list: Vec::new(),
        }
    }

    /// Returns piece on a given square or `None` if the square is empty.
    pub fn get_piece(&self, sq: Square) -> Option<Piece> {
        self.board[sq.as_index()]
    }

    /// Returns which side is to move now.
    pub fn get_side_to_move(&self) -> Color {
        self.side_to_move
    }

    pub fn get_en_passant_square(&self) -> Option<Square> {
        self.en_passant_square
    }

    /// Makes a move on the board. Returns `Ok(())` if the move succeeds and `Err` otherwise.
    /// This method _modifies_ the original board.
    pub fn do_move_inplace(&mut self, m: Move) -> Result<(), &'static str> {
        let from = m.get_square_from();
        let to = m.get_square_to();
        if let None = self.get_piece(from) {
            return Err("Empty starting square");
        }
        let p = self.get_piece(from).unwrap();
        if p != m.get_moving_piece() {
            return Err("Wrong moving piece");
        }
        if self.get_piece(to) != m.get_captured_piece() && !m.is_en_passant() {
            return Err("Wrong captured piece");
        }
        if m.is_normal_move() {
            self.board[from.as_index()] = None;
            self.board[to.as_index()] = if m.get_promoted_to().is_some() {
                Some(Piece::new(m.get_promoted_to().unwrap(), p.get_color()))
            } else {
                Some(p)
            };
            if m.get_moving_piece().get_kind() == PieceKind::Pawn &&
                (from.as_index() as i16 - to.as_index() as i16).abs() == 16 {
                let square_index = (from.as_index() + to.as_index()) / 2;
                self.en_passant_square = Some(Square::from_int(square_index as u32));
            }
        }
        if m.is_castling() {
            //TODO: Castling move
        }
        if m.is_en_passant() {
            //TODO: En passant move
        }
        self.move_list.push(m);
        Ok(())
    }


    /// Makes a move on the board, like `do_move_inplace()`, but returns _new_ (cloned) board.
    pub fn do_move(&self, m: Move) -> Result<Board, &'static str> {
        let mut c = self.clone();
        try!(c.do_move_inplace(m));
        Ok(c)
    }

    /// Undoes the last move made. Returns `Ok(())` on success and `Err` otherwise
    /// (for example, when it is nothing to undo or the move list is somehow corrupted).
    /// This method _modifies_ the original board.
    pub fn undo_move_inplace(&mut self) -> Result<(), &'static str> {
        if let Some(m) = self.move_list.pop() {
            let from = m.get_square_from();
            let to = m.get_square_to();
            self.board[to.as_index()] = m.get_captured_piece();
            let p = m.get_moving_piece();
            if self.board[to.as_index()] != Some(p) {
                return Err("Square occupied with wrong piece!");
            }
            if let Some(_) = self.board[from.as_index()] {
                return Err("Starting square somehow occupied");
            }
            self.board[from.as_index()] = if m.get_promoted_to().is_some() {
                Some(Piece::new(PieceKind::Pawn, p.get_color()))
            } else {
                Some(p)
            };
            Ok(())
        } else {
            Err("Nothing to undo")
        }
    }

    /// Undoes the last move made, like `undo_move_inplace()`, but returns _new_ (cloned) board.
    pub fn undo_move(&self) -> Result<Board, &'static str> {
        let mut c = self.clone();
        try!(c.undo_move_inplace());
        Ok(c)
    }
}
