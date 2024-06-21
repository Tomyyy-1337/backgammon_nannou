use nannou::color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    White,
    Black,
}

#[derive(Debug, Clone, Copy)]
pub struct Backgammon {
    board: [i8; 24],
    white_bar: u8,
    black_bar: u8,
    white_off: u8,
    black_off: u8,
    current_player: Player,
}

impl Backgammon {
    pub fn new() -> Self {
        Backgammon {
            board: [2,0,0,0,0,-5,0,-3,0,0,0,5,-5,0,0,0,3,0,5,0,0,0,0,-2],
            white_bar: 0,
            black_bar: 0,
            white_off: 0,
            black_off: 0,
            current_player: Player::White,
        }
    }

    pub fn invert(&self) -> Self {
        Backgammon {
            board: self.board.iter().rev().map(|&x| -x).collect::<Vec<_>>().try_into().unwrap(),
            white_bar: self.black_bar,
            black_bar: self.white_bar,
            white_off: self.black_off,
            black_off: self.white_off,
            current_player: match self.current_player {
                Player::White => Player::Black,
                Player::Black => Player::White,
            },
        }
    }
   
}