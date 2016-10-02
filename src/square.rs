use std::fmt;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Color {
    White,
    Black,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rank {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", *self as u8 + 1)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Square(u8);

impl Square {
    pub fn from_int(num: u32) -> Square {
        Square((num % 64) as u8)
    }

    pub fn from_file_and_rank(file: File, rank: Rank) -> Square {
        Square(file as u8 + 8 * rank as u8)
    }

    pub fn get_file(&self) -> File {
        match self.0 % 8 {
            0 => File::A,
            1 => File::B,
            2 => File::C,
            3 => File::D,
            4 => File::E,
            5 => File::F,
            6 => File::G,
            7 => File::H,
            _ => unreachable!(),
        }
    }
    pub fn get_rank(&self) -> Rank {
        match self.0 / 8 {
            0 => Rank::First,
            1 => Rank::Second,
            2 => Rank::Third,
            3 => Rank::Fourth,
            4 => Rank::Fifth,
            5 => Rank::Sixth,
            6 => Rank::Seventh,
            7 => Rank::Eighth,
            _ => unreachable!(),
        }
    }
    pub fn get_up(&self) -> Option<Square> {
        if self.get_rank() != Rank::Eighth {
            Some(Square(self.0 + 8))
        } else {
            None
        }
    }

    pub fn get_down(&self) -> Option<Square> {
        if self.get_rank() != Rank::First {
            Some(Square(self.0 - 8))
        } else {
            None
        }
    }
    pub fn get_left(&self) -> Option<Square> {
        if self.get_file() != File::A {
            Some(Square(self.0 - 1))
        } else {
            None
        }
    }

    pub fn get_right(&self) -> Option<Square> {
        if self.get_file() != File::H {
            Some(Square(self.0 + 1))
        } else {
            None
        }
    }
    pub fn get_up_left(&self) -> Option<Square> {
        if self.get_rank() != Rank::Eighth && self.get_file() != File::A {
            Some(Square(self.0 + 7))
        } else {
            None
        }
    }

    pub fn get_up_right(&self) -> Option<Square> {
        if self.get_rank() != Rank::Eighth && self.get_file() != File::H {
            Some(Square(self.0 + 9))
        } else {
            None
        }
    }
    pub fn get_down_left(&self) -> Option<Square> {
        if self.get_rank() != Rank::First && self.get_file() != File::A {
            Some(Square(self.0 - 9))
        } else {
            None
        }
    }

    pub fn get_down_right(&self) -> Option<Square> {
        if self.get_rank() != Rank::First && self.get_file() != File::H {
            Some(Square(self.0 - 7))
        } else {
            None
        }
    }

    pub fn get_forward(&self, color: Color) -> Option<Square> {
        match color {
            Color::White => self.get_up(),
            Color::Black => self.get_down(),
        }
    }

    pub fn get_back(&self, color: Color) -> Option<Square> {
        match color {
            Color::White => self.get_down(),
            Color::Black => self.get_up(),
        }
    }

    pub fn get_knight_moves(&self) -> Vec<Square> {
        let mut res = Vec::new();
        if self.get_rank() <= Rank::Sixth && self.get_file() != File::H {
            res.push(Square(self.0 + 17));
        }
        if self.get_rank() <= Rank::Sixth && self.get_file() != File::A {
            res.push(Square(self.0 + 15));
        }
        if self.get_rank() >= Rank::Third && self.get_file() != File::H {
            res.push(Square(self.0 - 15));
        }
        if self.get_rank() >= Rank::Third && self.get_file() != File::A {
            res.push(Square(self.0 - 17));
        }
        if self.get_rank() != Rank::Eighth && self.get_file() <= File::F {
            res.push(Square(self.0 + 10));
        }
        if self.get_rank() != Rank::Eighth && self.get_file() >= File::C {
            res.push(Square(self.0 + 6));
        }
        if self.get_rank() != Rank::First && self.get_file() <= File::F {
            res.push(Square(self.0 - 6));
        }
        if self.get_rank() != Rank::First && self.get_file() >= File::C {
            res.push(Square(self.0 - 10));
        }
        res
    }

    pub fn get_by_dir(&self, dir: Direction) -> Option<Square> {
        match dir {
            Direction::Up => self.get_up(),
            Direction::Down => self.get_down(),
            Direction::Left => self.get_left(),
            Direction::Right => self.get_right(),
            Direction::UpLeft => self.get_up_left(),
            Direction::UpRight => self.get_up_right(),
            Direction::DownLeft => self.get_down_left(),
            Direction::DownRight => self.get_down_right(),
        }
    }

    pub fn as_index(&self) -> usize {
        self.0 as usize
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.get_file(), self.get_rank())
    }
}
