fn main() {

    let board = build_board();
    board.draw();
    dbg!(board.get_possible_moves(1, 1));
    // board.my_move();
    // board.draw();
}





fn build_board() -> Board {
    let mut tmp_board = Board {
        board: vec![vec![Content::Empty; 8]; 8],
    };
    tmp_board.reset();
    tmp_board
}
/*
Gcode:
f2l1 = forward 2, left 1
f-   = forward until other piece or edge
fl-   = diag forward/left until other piece or edge
_f2l1 = forward 2, left 1 ONLY IF occupied by other player
*f2l1 = forward 2, left 1 ONLY IF first move


EXAMPLE: Pawn Gcode:
f1
*f2
_fl1
_fr1

EXAMPLE: Bishop Gcode:
fl-
fr-
bl-
br-

*/
use std::{fmt::Write, iter::Once};


#[derive(Clone)] // could not implement copy
struct Board {
    board: Vec<Vec<Content>>,
}



impl Board {
    fn reset(&mut self) {
        // self.board[0] = vec![
        //     Content::Black(Piece::Rook),
        //     Content::Black(Piece::Knight),
        //     Content::Black(Piece::Bishop),
        //     Content::Black(Piece::Queen),
        //     Content::Black(Piece::King),
        //     Content::Black(Piece::Bishop),
        //     Content::Black(Piece::Knight),
        //     Content::Black(Piece::Rook),
        // ];
        // self.board[1] = vec![Content::Black(Piece::Pawn); 8];
        // self.board[6] = vec![Content::White(Piece::Pawn); 8];

        // self.board[7] = vec![
        //     Content::White(Piece::Rook),
        //     Content::White(Piece::Knight),
        //     Content::White(Piece::Bishop),
        //     Content::White(Piece::Queen),
        //     Content::White(Piece::King),
        //     Content::White(Piece::Bishop),
        //     Content::White(Piece::Knight),
        //     Content::White(Piece::Rook),
        // ];
        self.board[1][1] = Content::Black(Piece::Pawn);
    }
    fn draw(&self) {
        let mut i = 8;
        for row in &self.board {
            print!("{} ", i);
            for square in row {
                print!("|{}", square);
            }
            println!("|");
            i-=1;
        }
        println!("   a b c d e f g h");
        println!("");
    }
    fn my_move(&mut self) {
        // swaps content in 4,4 with 2,2
        let tmp = self.board[4][4];
        self.board[4][4] = self.board[2][2];
        self.board[2][2] = tmp;
    }
    fn get_possible_moves(&self, row: i32, col: i32) -> Option<Vec<(i32, i32)>> {
        // returns vector of tuples of coordinates to possible moves
        // println!("content at {}, {} is {}", row, col, self.board[row as usize][col as usize]);
        // 
        // all_moves = self.board[row as usize][col as usize].moves();
        // dbg!(self.board[row as usize][col as usize].get_moves());
        let all_moves = self.board[row as usize][col as usize].get_moves();
        match all_moves {
            None => None,
            Some(move_type) => {
                match move_type {
                    MoveType::Inf(x) => Some(x),
                    MoveType::Once(x) => {
                        let destinations: Vec<(i32, i32)> = vec![];
                        for move in x {
                            destinations.push((row+move[0], col+move[1]));
                        }
                        destinations
                    },
                }
            }
        }
    }
}
#[derive(Copy, Clone)]
enum Piece {
    Pawn,
    // Bishop,
    // Knight,
    // Rook,
    // Queen,
    // King,
}


#[derive(Copy, Clone)]
enum Content {
    White(Piece),
    Black(Piece),
    Empty,
}
impl Content {
    fn get_symbol(&self) -> &'static str {
        match self {
            Content::White(piece) => {
                match piece {
                    Piece::Pawn => "♙",
                    // Piece::Bishop => "♗",
                    // Piece::Knight => "♘",
                    // Piece::Rook => "♖",
                    // Piece::Queen => "♕",
                    // Piece::King => "♔",
                }
            },
            Content::Black(piece) => {
                match piece {
                    Piece::Pawn => "♟",
                    // Piece::Bishop => "♝",
                    // Piece::Knight => "♞",
                    // Piece::Rook => "♜",
                    // Piece::Queen => "♛",
                    // Piece::King => "♚",
                }
            },
            Content::Empty => "_",
        }
    }
    fn get_moves(&self) -> Option<MoveType> {
        match self {
            Content::Black(piece) => {
                match piece {
                    Piece::Pawn => Some(MoveType::Once(
                        vec![(-1, 0)]
                    ))
                }
            },
            Content::White(piece) => {
                match piece {
                    Piece::Pawn => Some(MoveType::Once(
                        vec![(1, 0)]
                    ))
                }
            },
            Content::Empty => None
        }
    }
}
impl std::fmt::Display for Content {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.get_symbol())
    }
}


enum MoveType {
    Inf(Vec<(i32, i32)>),
    Once(Vec<(i32, i32)>),
}