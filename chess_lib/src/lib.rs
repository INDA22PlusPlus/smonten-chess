mod piece;
mod content;

use piece::*;
use content::*;



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

        self.board[self.size-2] = self.create_rank2(Color::White);
        self.board[self.size-1] = self.create_rank1(Color::White);
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
    fn create_rank1(&self, color: Color) -> Vec<Content> {
        vec![
            Content::Occupied(Piece {
                color: color,
                piece_type: PieceType:: Rook,
                times_moved: 0
            }),
            Content::Occupied(Piece {
                color: color,
                piece_type: PieceType:: Knight,
                times_moved: 0
            }),
            Content::Occupied(Piece {
                color: color,
                piece_type: PieceType:: Bishop,
                times_moved: 0
            }),
            Content::Occupied(Piece {
                color: color,
                piece_type: PieceType:: Queen,
                times_moved: 0
            }),
            Content::Occupied(Piece {
                color: color,
                piece_type: PieceType:: King,
                times_moved: 0
            }),
            Content::Occupied(Piece {
                color: color,
                piece_type: PieceType:: Bishop,
                times_moved: 0
            }),
            Content::Occupied(Piece {
                color: color,
                piece_type: PieceType:: Knight,
                times_moved: 0
            }),
            Content::Occupied(Piece {
                color: color,
                piece_type: PieceType:: Rook,
                times_moved: 0
            })
        ]
    }
    pub fn draw(&self) {
        println!("   0 1 2 3 4 5 6 7");
        let mut i = 0;
        for row in &self.board {
            print!("{} ", i);
            for square in row {
                print!("|{}", square.get_symbol());
            }
            println!("|");
            i += 1;
        }
        println!("");
    }

    fn destination_outside_board(&self, x: i32, y: i32) -> bool {
        0 > x || x >= self.size as i32 || 0 > y || y >= self.size as i32
    }

    pub fn get_destinations(&self, xy: (usize, usize)) -> Destinations {
        let x = xy.0;
        let y = xy.1;
        let this_content = self.board[y][x];
        match this_content {
            Content::Empty => Destinations::None,
            Content::Occupied(this_p) => {
                match this_p.piece_type {
                    PieceType::Pawn => self.get_destinations_pawns(this_p, x, y),
                    _ => self.get_destinations_not_pawns(this_p, x, y)
                }
                
            }
        }
    }

    fn get_destinations_not_pawns(&self, this_p: Piece, x: usize, y: usize) -> Destinations {
        // k factor 1 or -1 to rotate movement vectors if white (bottom)
        let k = match this_p.color {
            Color::Black => 1,
            Color::White => -1,
        };
    
        let moves = this_p.get_moves();
        // let mut destinations: Vec<Destination> = vec![];
        let mut destinations: Vec<(usize, usize)> = vec![];
    
        match moves.move_type {
            MoveType::Once => {
                
                for move_vec in moves.move_vecs {
                    let other_x = (x as i32 + move_vec.1*k) as usize;
                    let other_y = (y as i32 + move_vec.0*k) as usize;
                    if self.destination_outside_board(other_x as i32, other_y as i32) {
                        break;
                    }
                    let other_content = self.board[other_y][other_x];
                    match other_content {
                        Content::Empty => {
                            // destinations.push(Destination::Empty((other_x, other_y)))
                            destinations.push((other_x, other_y));
                        },
                        Content::Occupied(other_p) => {
                            if this_p.color != other_p.color {
                            // destinations.push(Destination::Empty((other_x, other_y)))
                            destinations.push((other_x, other_y));
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
                        other_x = (other_x as i32 + move_vec.1*k) as usize;
                        other_y = (other_y as i32 + move_vec.0*k) as usize;
                        if self.destination_outside_board(other_x as i32, other_y as i32) {
                            get_next = false;
                            break;
                        }
                        let other_content = self.board[other_y][other_x];
                        match other_content {
                            Content::Empty => {
                            // destinations.push(Destination::Empty((other_x, other_y)))
                            destinations.push((other_x, other_y));
                            },
                            Content::Occupied(other_p) => {
                                if this_p.color != other_p.color {
                                    // destinations.push(Destination::Empty((other_x, other_y)))
                                    destinations.push((other_x, other_y));
                                    get_next = false;
                                    break;
                                } else {
                                    get_next = false;
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
        // RETURN, return enum None if no destinations pushed to "destinations"
        if destinations.len() != 0 {
            Destinations::Exists(destinations)
        } else {
            Destinations::None
        }
    
    }

    fn get_destinations_pawns(&self, this_p: Piece, x: usize, y: usize) -> Destinations {
        let k = match this_p.color {
            Color::Black => 1,
            Color::White => -1,
        };
        // let mut destinations: Vec<Destination> = vec![];
        let mut destinations: Vec<(usize, usize)> = vec![];
    
    
        if !self.destination_outside_board(x as i32, y as i32 +k) {
            let other_p = self.board[((y as i32) + k) as usize][x];
            match other_p {
                Content::Empty => {
                    // destinations.push(Destination::Empty((x, (y as i32 + k) as usize)));
                    destinations.push((x, (y as i32 + k) as usize));
                    if this_p.times_moved == 0 {
                        if !self.destination_outside_board(x as i32, y as i32 +2*k) {
                            let other_p = self.board[((y as i32) + k) as usize][x];
                            match other_p {
                                // Content::Empty => destinations.push(Destination::Empty((x, (y as i32 + 2*k) as usize))),
                                Content::Empty => destinations.push((x, (y as i32 + 2*k) as usize)),
                                _ => (),
                            }
                        }
                    }
                },
                _ => (),
            }
        }
    
        for move_vec in vec![(1, 1), (1, -1)] {
            let other_x = (x as i32 + move_vec.1) as usize;
            let other_y = (y as i32 + move_vec.0) as usize;
            if !self.destination_outside_board(other_x as i32, other_y as i32) {
                match self.board[other_y][other_x] {
                    Content::Occupied(other_p) => {
                        if this_p.color != other_p.color {
                            // destinations.push(Destination::Kill((other_x, other_y)));
                            destinations.push((other_x, other_y));
                        }
                    },
                    _ => (),
                }
            }
        }
        // RETURN, return enum None if no destinations pushed to "destinations"
        if destinations.len() != 0 {
            Destinations::Exists(destinations)
        } else {
            Destinations::None
        }
    
        
    }
    

    pub fn move_from_to(&mut self, from: (usize, usize), to: (usize, usize)) {
        match self.board[from.1][from.0] {
            Content::Empty => panic!("Tried to move empty!"),
            Content::Occupied(mut this_p) => {
                if this_p.color == self.turn {
                    match self.get_destinations(from) {
                        Destinations::Exists(d) => {
                            if d.contains(&to) {
                                
                                this_p.times_moved += 1;
                                self.next_turn();


                                self.board[to.1][to.0] = self.board[from.1][from.0];
                                self.board[from.1][from.0] = Content::Empty;


                            } else {
                                panic!("This move is not legal!");
                            }
                        },
                        Destinations::None => panic!("Can't move this piece!"),
                    }
                } else {
                    println!("not your turn!");
                }
            }
        }
    }

    fn next_turn(&mut self) {
        match self.turn {
            Color::Black => self.turn = Color::White,
            Color::White => self.turn = Color::Black
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



#[derive(Debug)]
pub enum Destinations {
    // Exists(Vec<Destination>),
    Exists(Vec<(usize, usize)>),
    None
}

#[derive(Debug)]
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
