pub struct Game {
    size: usize,
    board: Vec<Vec<Content>>,
    turn: Color,
    score_w: i32,
    score_b: i32,
}

impl Game {
   pub fn reset(&mut self) {
        self.board[0] = self.create_rank1(Color::Black);
        self.board[1] = self.create_rank2(Color::Black);


        //test
        self.board[3][0] = Content::Occupied(Piece {
            color: Color::White,
            piece_type: PieceType:: Pawn,
            times_moved: 0
        });

        self.board[self.size-2] = self.create_rank2(Color::White);
        self.board[self.size-1] = self.create_rank1(Color::White);
    }
    fn create_rank1(&self, color: Color) -> Vec<Content> {
        vec![
            Content::Occupied(Piece {
                color: color,
                piece_type: PieceType:: Bishop,
                times_moved: 0
            });
            self.size   
        ]
    }
    fn create_rank2(&self, color: Color) -> Vec<Content> {
        vec![
            Content::Occupied(Piece {
                color: color,
                piece_type: PieceType:: Pawn,
                times_moved: 0
            });
            self.size
        ]
    }
    pub fn draw(&self) {
        for row in &self.board {
            for square in row {
                print!("|{}", square.get_symbol());
            }
            println!("|");
        }
        println!("");
    }

    fn destination_outside_board(&self, x: i32, y: i32) -> bool {
        0 > x || x >= self.size as i32 || 0 > y || y >= self.size as i32
    }

    pub fn get_destinations(&self, x: usize, y: usize) -> Destinations {
        let this_content = self.board[y][x];
        match this_content {
            Content::Empty => Destinations::None,
            Content::Occupied(this_p) => {
                let moves = this_p.get_moves();
                let mut destinations: Vec<Destination> = vec![];

                match moves.move_type {
                    MoveType::Once => {
                        
                        for move_vec in moves.move_vecs {
                            let other_x = (x as i32 + move_vec.1) as usize;
                            let other_y = (y as i32 + move_vec.0) as usize;
                            if self.destination_outside_board(other_x as i32, other_y as i32) {
                                break;
                            }
                            let other_content = self.board[other_y][other_x];
                            match other_content {
                                Content::Empty => {
                                    destinations.push(Destination::Empty((other_x, other_y)))
                                },
                                Content::Occupied(other_p) => {
                                    if this_p.color != other_p.color {
                                        destinations.push(Destination::Kill((other_x, other_y)));
                                    }
                                }
                            }
                        }                        
                        
                    },
                    MoveType::Inf => {
                        for move_vec in moves.move_vecs {
                            let mut other_x = x;
                            let mut other_y = y;
                            let mut get_next = true;
                            while get_next {
                                other_x = (other_x as i32 + move_vec.1) as usize;
                                other_y = (other_y as i32 + move_vec.0) as usize;
                                if self.destination_outside_board(other_x as i32, other_y as i32) {
                                    get_next = false;
                                    break;
                                }
                                let other_content = self.board[other_y][other_x];
                                match other_content {
                                    Content::Empty => {
                                        destinations.push(Destination::Empty((other_x, other_y)))
                                    },
                                    Content::Occupied(other_p) => {
                                        if this_p.color != other_p.color {
                                            destinations.push(Destination::Kill((other_x, other_y)));
                                            get_next = false;
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                // return:
                Destinations::Exists(destinations)
            }
        }
    }
}

pub fn create_game() -> Game {
    let size: usize = 8;
    let mut game = Game {
        size: size,
        board: vec![vec![Content::Empty; size]; size],
        turn: Color::White,
        score_w: 0,
        score_b: 0,
    };
    game.reset();
    game
}







#[derive(Copy, Clone)]
enum Content {
    Occupied(Piece),
    Empty
}
impl Content {
    fn get_symbol(&self) -> &'static str {
        match self {
            Content::Empty => "_",
            Content::Occupied(p) => {
                match p.color {
                    Color::White => {
                        match p.piece_type {
                            PieceType::Pawn => "♙",
                            PieceType::Bishop => "♗",
                        }
                    },
                    Color::Black => {
                        match p.piece_type {
                            PieceType::Pawn => "♟",
                            PieceType::Bishop => "♝",
                        }
                    }
                }
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Color {
    Black,
    White
}

#[derive(Copy, Clone)]
enum PieceType {
    Pawn,
    Bishop
}



#[derive(Copy, Clone)]
pub struct Piece {
    color: Color,
    piece_type: PieceType,
    times_moved: u32,
}

impl Piece {
    fn get_moves(&self) -> Move {
        match self.piece_type {
            PieceType::Pawn => {
                let mut m = vec![(1, 0)];
                if self.times_moved == 0 {
                    m.push((2, 0));
                }
                Move {
                    move_vecs: m,
                    move_type: MoveType::Once
                }
            },
            PieceType::Bishop => {
                Move {
                    move_vecs: vec![(1, 1), (1, -1), (-1, 1), (-1, -1)],
                    move_type: MoveType::Inf
                }
            }
        }
    }
}

struct Move {
    move_vecs: Vec<(i32, i32)>,
    move_type: MoveType
}
enum MoveType {
    Once,
    Inf
}

pub enum Destinations {
    Exists(Vec<Destination>),
    None
}

pub enum Destination {
    Empty((usize, usize)),
    Kill((usize, usize))
}


pub fn add(left: usize, right: usize) -> usize {

    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }


}
