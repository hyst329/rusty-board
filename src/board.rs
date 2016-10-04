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

/// The main data type for chessboard
pub struct Board {
    board: [Option<Piece>; 64],
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
            board: board,
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

    pub fn do_move_inplace(&mut self, m: Move) {}
}
