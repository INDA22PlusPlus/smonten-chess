



fn main() {

    let mut board = build_board();
    board.draw();
    let test = (3, 3);
    match board.get_possible_moves(test.1,test.0) {
        PossibleMoves::Moves(m) => {
            for i in &m {
                println!("{}, {}", i.1, i.0);
            }
            board.move_from_to(test, m[0]);
        },
        PossibleMoves::NoMoves => println!("no moves"),
    }
    // board.my_move();
    board.draw();
}


fn build_board() -> Board {
    let size: usize = 8;
    let mut tmp_board = Board {
        size: size as i32,
        board: vec![vec![Content::Empty; size]; size],
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
// use std::{fmt::Write, iter::Once, error::Chain};


#[derive(Clone)] // could not implement copy
struct Board {
    size: i32,
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
        self.board[1][1] = Content::Black(Piece::Knight);
        self.board[3][3] = Content::Black(Piece::Pawn);
    }
    fn draw(&self) {
        let mut i = 0;

        println!("   0 1 2 3 4 5 6 7");
        for row in &self.board {
            print!("{} ", i);
            for square in row {
                print!("|{}", square);
            }
            println!("|");
            i+=1;
        }
        // println!("   a b c d e f g h");
        println!("");
    }
    fn my_move(&mut self) {
        // swaps content in 4,4 with 2,2
        let tmp = self.board[4][4];
        self.board[4][4] = self.board[2][2];
        self.board[2][2] = tmp;
    }

    fn move_from_to(&mut self, from: (i32, i32), to: (i32, i32)) {
        match &self.board[to.0 as usize][to.1 as usize] {
            Content::Empty => {
                println!("want to move ({}, {}) to ({}, {})", from.1,from.0, to.1, to.0);
                let tmp = self.board[to.0 as usize][to.1 as usize];
                self.board[to.0 as usize][to.1 as usize] = self.board[from.0 as usize][from.1 as usize];
                self.board[from.0 as usize][from.1 as usize] = tmp;
            },
            _ => {
                println!("Killing not implemented yet");
            },
        }    
    }
    
    fn get_possible_moves(&self, row: i32, col: i32) -> PossibleMoves {
        // returns vector of tuples of coordinates to possible moves

        // let vec_of_legal_moves = self.board[row as usize][col as usize].get_moves();

        let this_content = &self.board[row as usize][col as usize];
        let vec_of_legal_moves = this_content.get_moves();
        match vec_of_legal_moves {
            None => {
                // println!("entered here");
                
                PossibleMoves::NoMoves
            },

            Some(move_type) => {
                match move_type {
                    // MoveType::Inf(x) => Some(x),
                    MoveType::Once(x) => {
                        let mut possible_destinations: Vec<(i32, i32)> = vec![];

                        for legal_move in x {
                            let dest_row = row + legal_move.0;
                            let dest_col = col + legal_move.1;

                            // println!("now checking {}, {}", dest_col, dest_row);

                            if self.coordinates_inside_board(dest_row, dest_col) {


                                let other_content = &self.board[dest_row as usize][dest_col as usize];
                                match other_content {
                                    Content::Black(_) => {
                                        match this_content {
                                            Content::White(_) => possible_destinations.push((dest_row, dest_col)),
                                            _ => (),
                                        }
                                    },
                                    Content::White(_) => {
                                        match this_content {
                                            Content::Black(_) => possible_destinations.push((dest_row, dest_col)),
                                            _ => (),
                                        }
                                    },
                                   Content::Empty => possible_destinations.push((dest_row, dest_col)),
                                }
                            }
                        }
                        if possible_destinations.len() == 0 {
                            PossibleMoves::NoMoves
                        } else {
                            PossibleMoves::Moves(possible_destinations)
                        }
                    },
                    MoveType::Inf(x) => {
                        let mut possible_destinations: Vec<(i32, i32)> = vec![];
                        for direction in x {
                            let mut get_next = true;
                            let mut dest_row = row;
                            let mut dest_col = col;
                            while get_next {
                                dest_row += direction.0;
                                dest_col += direction.1;

                                // println!("now checking {}, {}", dest_col, dest_row);

                                if self.coordinates_inside_board(dest_row, dest_col) {
                                    let other_content = &self.board[dest_row as usize][dest_col as usize];
                                    match other_content {
                                        Content::Black(_) => {
                                            match this_content {
                                                Content::White(_) => {
                                                    possible_destinations.push((dest_row, dest_col));
                                                    get_next = false;
                                                },
                                                _ => get_next = false,
                                            }
                                        },
                                        Content::White(_) => {
                                            match this_content {
                                                Content::Black(_) => {
                                                    possible_destinations.push((dest_row, dest_col));
                                                    get_next = false;
                                                },
                                                _ => get_next = false,
                                            }
                                        },
                                       Content::Empty => possible_destinations.push((dest_row, dest_col)),
                                    }
                                } else {
                                    get_next = false;
                                }
                            }

                        }
                        if possible_destinations.len() == 0 {
                            PossibleMoves::NoMoves
                        } else {
                            PossibleMoves::Moves(possible_destinations)
                        }
                    }
                }
            }
        }  
        
    }
    fn coordinates_inside_board(&self, col: i32, row: i32) -> bool {
        0 <= col && col < self.size && 0 <= row && col < self.size
    }
}

#[derive(Copy, Clone)]
enum Piece {
    Pawn,
    Bishop,
    Knight,
    // Rook,
    // Queen,
    // King,
}

enum PossibleMoves {
    Moves(Vec<(i32, i32)>),
    NoMoves,
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
                    Piece::Bishop => "♗",
                    Piece::Knight => "♘",
                    // Piece::Rook => "♖",
                    // Piece::Queen => "♕",
                    // Piece::King => "♔",
                }
            },
            Content::Black(piece) => {
                match piece {
                    Piece::Pawn => "♟",
                    Piece::Bishop => "♝",
                    Piece::Knight => "♞",
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
                        vec![(1, 0)]
                    )),
                    Piece::Knight => Some(MoveType::Once(
                        vec![(2, 1), (2, -1), (1, 2), (1, -2), (-2, 1), (-2, -1), (-1, 2), (-1, -2)]
                    )),
                    Piece::Bishop => Some(MoveType::Inf(
                        vec![(1, 1), (1, -1), (-1, 1), (-1, -1)]
                    )),
                }
            },
            Content::White(piece) => {
                match piece {
                    Piece::Pawn => Some(MoveType::Once(
                        vec![(-1, 0)]
                    )),
                    Piece::Knight => Some(MoveType::Once(
                        vec![(2, 1), (2, -1), (1, 2), (1, -2), (-2, 1), (-2, -1), (-1, 2), (-1, -2)]
                    )),
                    Piece::Bishop => Some(MoveType::Inf(
                        vec![(1, 1), (1, -1), (-1, 1), (-1, -1)]
                    )),
                }
            },
            Content::Empty => {
                // println!("entered Empty");
                None
            },
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