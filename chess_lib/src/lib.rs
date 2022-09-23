mod piece;
mod content;




pub use piece::*;
pub use content::*;


#[derive(Clone)]
pub struct Game {
    size: usize,
    board: Vec<Vec<Content>>,
    turn: Color,
    w_king: (usize, usize),
    b_king: (usize, usize),
    w_check: bool,
    b_check: bool,
}


impl Game {
   pub fn reset(&mut self) {
        self.board[0] = self.create_rank1(Color::Black);
        self.board[1] = self.create_rank2(Color::Black);

        self.board[self.size-2] = self.create_rank2(Color::White);
        self.board[self.size-1] = self.create_rank1(Color::White);

        self.w_king = (4, 7);
        self.b_king = (4, 0);
        self.w_check = false;
        self.b_check = false;
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

    pub fn can_castle(&self, color: Color) -> CanCastle {

        let first_rank = match color {
            Color::Black => self.board[0].clone(),
            Color::White => self.board[7].clone(),
        };
        //TRYING LEFT
        let left_side = vec![
            Content::Occupied(
                Piece {
                    color: color,
                    piece_type: PieceType::Rook,
                    times_moved: 0 
                }
            ),
            Content::Empty,
            Content::Empty,
            Content::Empty,
            Content::Occupied(
                Piece {
                    color: color,
                    piece_type: PieceType::King,
                    times_moved: 0 
                }
            )

        ];
        //TRYING RIGHT
        let right_side = vec![
            Content::Occupied(
                Piece {
                    color: color,
                    piece_type: PieceType::King,
                    times_moved: 0 
                }
            ),
            Content::Empty,
            Content::Empty,
            Content::Occupied(
                Piece {
                    color: color,
                    piece_type: PieceType::Rook,
                    times_moved: 0 
                }
            )

        ];
        return CanCastle {
            left: first_rank[0..=4] == left_side,
            right: first_rank[0..=7] == right_side
        };

        
        // let mut cd: Vec<(usize, usize)> = vec![];
        // let mut empty: Vec<(usize, usize)> = vec![];

        // //CHECKING RIGHT
        // let mut other_c = self.board[y][(x as i32 - 1) as usize];
        // match other_c {
        //     Content::Empty => (),
        //     _ => return empty
        // }
        // other_c = self.board[y][(x as i32 - 2) as usize];
        // match other_c {
        //     Content::Empty => (),
        //     _ => return empty
        // }
        // other_c = self.board[y][(x as i32 - 3) as usize];
        // match other_c {
        //     Content::Empty => (),
        //     _ => return empty
        // }
        // other_c = self.board[y][(x as i32 - 4) as usize];
        // match other_c {
        //     Content::Occupied(other_p) => {
        //         match other_p.piece_type {
        //             PieceType::Rook => {
        //                 if other_p.times_moved != 0 {
        //                     return empty;
        //                 } else {
        //                     cd.push((
        //                         (x as i32 - 2) as usize,
        //                         y
        //                     ));
        //                 }
        //             },
        //             _ => return empty
        //         }
        //     },
        //     Content::Empty => return empty
        // }


        // //CHECKING LEFT
        // other_c = self.board[y][(x as i32 + 1) as usize];
        // match other_c {
        //     Content::Empty => (),
        //     _ => return empty
        // }
        // other_c = self.board[y][(x as i32 + 2) as usize];
        // match other_c {
        //     Content::Empty => (),
        //     _ => return empty
        // }
        // other_c = self.board[y][(x as i32 + 3) as usize];
        // match other_c {
        //     Content::Occupied(other_p) => {
        //         match other_p.piece_type {
        //             PieceType::Rook => {
        //                 if other_p.times_moved != 0 {
        //                     return empty;
        //                 } else {
        //                     cd.push((
        //                         (x as i32 + 2) as usize,
        //                         y
        //                     ));
        //                 }
        //             },
        //             _ => return empty
        //         }
        //     },
        //     Content::Empty => return empty
        // }

        
        // return cd;
    }

    fn get_destinations_not_pawns(&self, this_p: Piece, x: usize, y: usize) -> Destinations {
        // k: factor 1 or -1 to rotate movement vectors 180 deg if white (bottom)
        let k = match this_p.color {
            Color::Black => 1,
            Color::White => -1,
        };
    
        let moves = this_p.get_moves();
        let mut destinations: Vec<(usize, usize)> = vec![];

        // ADDING POTENTIAL CASTLING DESTINATIONS 
        /* only need to add the kings destination,
        the rook will be able to make the same
        horisontal move whithout the castling */
        match this_p.piece_type {
            PieceType::King => {
                println!("checking for Castling, looking at king now");
                let can_c = self.can_castle(this_p.color);
                if can_c.left {
                    destinations.push((2, y))
                }
                if can_c.right {
                    destinations.push((6, y))
                }
            },
            _ => (),
        }

        // ADDING REGULAR DESTINATIONS
        match moves.move_type {
            MoveType::Once => {
                
                for move_vec in moves.move_vecs {
                    let other_x = (x as i32 + move_vec.1*k) as usize;
                    let other_y = (y as i32 + move_vec.0*k) as usize;
                    if !self.destination_outside_board(other_x as i32, other_y as i32) {

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
                }              
            },
            MoveType::Inf => {
                for move_vec in moves.move_vecs {
                    let mut other_x = x;
                    let mut other_y = y;
                    // let mut get_next = true;
                    loop {
                        other_x = (other_x as i32 + move_vec.1*k) as usize;
                        other_y = (other_y as i32 + move_vec.0*k) as usize;
                        if self.destination_outside_board(other_x as i32, other_y as i32) {
                            // get_next = false;
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
                                    // get_next = false;
                                    break;
                                } else {
                                    // get_next = false;
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
                            let other_p_2 = self.board[((y as i32) + 2*k) as usize][x];
                            match other_p_2 {
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
            let other_x = (x as i32 + move_vec.1*k) as usize;
            let other_y = (y as i32 + move_vec.0*k) as usize;
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

                            // double check that move is legal
                            if d.contains(&to) {
                                
                                // UPDATE TIMES MOVED IN CURRENT PIECE
                                this_p.times_moved += 1;
                                self.board[from.1][from.0] = Content::Occupied(this_p);

                                match this_p.piece_type {
                                    PieceType::King => {
                                        
                                        // update current king position
                                        match this_p.color {
                                            Color::White => self.w_king = to,
                                            Color::Black => self.b_king = to,
                                        }
                                        //CASTLING?
                                        let delta_x = to.0 as i32 - from.0 as i32;
                                        if delta_x == -2 {
                                            // castling left
                                            self.castle_left(this_p.color);
                                            self.next_turn();
                                            
                                        } else if delta_x == 2 {
                                            // castling right
                                            self.castle_right(this_p.color);
                                            self.next_turn();
                                        } else {
                                            // regular king move
                                            self.board[to.1][to.0] = self.board[from.1][from.0];
                                            self.board[from.1][from.0] = Content::Empty;
                                            self.next_turn();
                                        }
                                    },
                                    _ => {
                                        // regular move
                                        self.board[to.1][to.0] = self.board[from.1][from.0];
                                        self.board[from.1][from.0] = Content::Empty;
                                        self.next_turn();
                                    },
                                }

                            } else {
                                panic!("This move is not legal!");
                            }
                        },
                        Destinations::None => panic!("Can't move this piece!"),
                    }
                } else {
                    panic!("not your turn!");
                }
            }
        }
    }

    fn castle_left(&mut self, color: Color) {
        let y: usize = match color {
            Color::Black => 0,
            Color::White => 7,
        };
        self.board[y][0] = Content::Empty;
        self.board[y][2] = Content::Occupied(Piece {
            color: color,
            piece_type: PieceType::King,
            times_moved: 1
        });
        self.board[y][3] = Content::Occupied(Piece {
            color: color,
            piece_type: PieceType::Rook,
            times_moved: 1
        });
        self.board[y][4] = Content::Empty;
    }
    fn castle_right(&mut self, color: Color) {
        let y: usize = match color {
            Color::Black => 0,
            Color::White => 7,
        };
        self.board[y][4] = Content::Empty;
        self.board[y][5] = Content::Occupied(Piece {
            color: color,
            piece_type: PieceType::Rook,
            times_moved: 1
        });
        self.board[y][6] = Content::Occupied(Piece {
            color: color,
            piece_type: PieceType::King,
            times_moved: 1
        });
        self.board[y][7] = Content::Empty;
        
    }

    fn next_turn(&mut self) {
        self.check_check();
        match self.turn {
            Color::Black => self.turn = Color::White,
            Color::White => self.turn = Color::Black
        }
    }

    fn is_threatened(&self, xy: (usize, usize)) -> bool {
        match self.board[xy.1][xy.0] {
            Content::Occupied(this_p) => {
                
                let mut threatened = false;


                let mut x = 0;
                let mut y = 0;
                for row in & self.board {
                    for square in row {
                        match square {
                            Content::Occupied(other_p) => {
                                match self.get_destinations((x as usize, y as usize)) {
                                    Destinations::Exists(d) => {
                                        if this_p.color != other_p.color && d.contains(&xy) {
                                            threatened = true;
                                        }
                                    },
                                    Destinations::None => (),
                                }
                            },
                            Content::Empty => (),
                        }
                        x += 1;
                    }
                    x = 0;
                    y += 1;
                }
                return threatened;
            },
            Content::Empty => false
        }
    }

    // fn check_check(&self) {
    //     let mut x = 0;
    //     let mut y = 0;
    //     for row in & self.board {
    //         for square in row {
    //             match square {
    //                 Content::Occupied(this_p) => {
    //                     match self.get_destinations((x as usize, y as usize)) {
    //                         Destinations::Exists(d) => {
    //                             match this_p.color {
    //                                 Color::White => {
    //                                     if d.contains(&self.b_king) {
    //                                         println!("Check Black!");
    //                                     }
    //                                 },
    //                                 Color::Black => {
    //                                     if d.contains(&self.w_king) {
    //                                         println!("Check White!");
    //                                     }
    //                                 },
    //                             }
    //                         },
    //                         Destinations::None => (),
    //                     }
    //                 },
    //                 Content::Empty => (),
    //             }
    //             x += 1;
    //         }
    //         x = 0;
    //         y += 1;
    //     }
    // }
    fn check_check(&mut self) {
        self.w_check = self.is_threatened(self.w_king);
        self.b_check = self.is_threatened(self.b_king);
    }

    fn is_checked(&self, color: Color) -> bool {
        match color {
            Color::White => self.w_check,
            Color::Black => self.b_check,
        }
    }

    fn is_checkmated(&self, color: Color) -> bool {
        let king_xy = match color {
            Color::Black => self.b_king,
            Color::White => self.w_king,
        };

        
        let col = match color {
            Color::Black => "black",
            Color::White => "white",
        };
        // println!("checking if {} king is checkmated", col);

        // let mut can_save_king = false;

        let mut x = 0;
        let mut y = 0;
        for row in &self.board {
            for square in row {
                match square {
                    Content::Occupied(this_p) => {
                        if this_p.color == color {
                            
                            // println!("now checking potential moves at ({}, {})", x, y);
                            // dbg!(this_p.piece_type);

                            match self.get_destinations((x, y)) {
                                Destinations::Exists(dest_vecs) => {
                                    for dest_vec in dest_vecs {
                                        // println!("\ttrying moving ({}, {}) to ({}, {})", x, y, dest_vec.0, dest_vec.1);
                                        let mut tmp_game = self.clone();
                                        // OBS need to prevent tmp game from panicing "not your turn"
                                        // so the tmp_games turn has to be set to color of current piece
                                        tmp_game.turn = this_p.color;
                                        tmp_game.move_from_to((x, y), dest_vec);
                                        // tmp_game.check_check();
                                        if ! tmp_game.is_checked(color) {
                                            return false;
                                        }
                                    }
                                },
                                Destinations::None => (),
                            }
                        }
                    }
                    Content::Empty => (),
                }
                x = (x as i32 + 1) as usize;
            }
            x = 0;
            y = (y as i32 + 1) as usize;
        }
        return true;
        // match self.get_destinations(king_xy) {
        //     Destinations::Exists(king_dests) => {
        //         let mut safe_dests: Vec<(usize, usize)> = vec![];
        //         for king_dest in king_dests {
        //             if !self.is_threatened(king_dest) {
        //                 safe_dests.push(king_dest);
        //             }
        //         }
        //         return safe_dests.len() == 0;
        //     },
        //     Destinations::None => true,
        // }

    }

    pub fn coordinates_playable(&self, xy: (usize, usize)) -> bool {
        if !self.destination_outside_board(xy.0 as i32, xy.1 as i32) {
            match self.board[xy.1][xy.0] {
                Content::Occupied(this_p) => {
                    return this_p.color == self.turn;
                },
                Content::Empty => false,
            }
        } else {
            false
        }
    }
    pub fn get_turn(&self) -> Color {
        self.turn
    }

    pub fn get_gamestate(&self) -> GameState {
        if self.is_checked(Color::Black) {
            if self.is_checkmated(Color::Black) {
                return GameState::IsCheckMated(Color::Black);
            } else {
                return GameState::IsChecked(Color::Black);
            }
        } else if self.is_checked(Color::White) {
            if self.is_checkmated(Color::White) {
                return GameState::IsCheckMated(Color::White);
            } else {
                return GameState::IsChecked(Color::White);
            }
        } else {
            return GameState::NoThreats;
        }
    }
}




pub fn create_game() -> Game {
    let size: usize = 8;
    let mut game = Game {
        size: size,
        board: vec![vec![Content::Empty; size]; size],
        turn: Color::White,
        w_king: (4, 7),
        b_king: (4, 0),
        w_check: false,
        b_check: false,
    };
    game.reset();
    return game;
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

#[derive(Debug)]
pub enum GameState {
    NoThreats,
    IsChecked(Color),
    IsCheckMated(Color),

}

#[derive(Debug, PartialEq)]
pub struct CanCastle {
    left: bool,
    right: bool
}

#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;

    #[test]
    fn get_turn() {
        let mut game = create_game();
        assert_eq!(game.get_turn(), Color::White);
        game.move_from_to((1, 6), (1, 4));
        assert_eq!(game.get_turn(), Color::Black);
    }

    

    #[test]
    fn not_your_turn() { 
        let game = create_game();
        assert!( ! game.coordinates_playable((4, 1)));
    }


    #[test]
    fn pawn_moves() { 
        let mut game = create_game();
        game.move_from_to((1, 6), (1, 4));
        game.move_from_to((0, 1), (0, 3));

        game.draw();
        
        match game.get_destinations((1, 4)) {
            Destinations::Exists(mut d) => {
                assert_eq!(d.sort(), vec![(0, 3), (1, 3)].sort());
            },
            Destinations::None => panic!("should have moves!")
        }
    }

    #[test]
    fn check() {
        let mut game = create_game();
        game.move_from_to((3, 6), (3, 4));
        assert!( ! game.is_checked(Color::White));
        assert!( ! game.is_checked(Color::Black));
        game.move_from_to((4, 1), (4, 3));
        assert!( ! game.is_checked(Color::White));
        assert!( ! game.is_checked(Color::Black));
        game.move_from_to((7, 6), (7, 4));
        assert!( ! game.is_checked(Color::White));
        assert!( ! game.is_checked(Color::Black));
        game.move_from_to((5, 0), (1, 4));
        assert!(game.is_checked(Color::White));
        assert!( ! game.is_checked(Color::Black));

        game.move_from_to((2, 6), (2, 5));
        assert!( ! game.is_checked(Color::White));
        assert!( ! game.is_checked(Color::Black));

        assert!( ! game.is_checkmated(Color::White));
        assert!( ! game.is_checkmated(Color::Black));
    }

    #[test]
    fn fools_mate() {
        let mut game = create_game();
        game.move_from_to((5, 6), (5, 5));
        game.move_from_to((4, 1), (4, 3));
        game.move_from_to((6, 6), (6, 4));
        game.move_from_to((3, 0), (7, 4));
        game.draw();
        assert!(game.is_checkmated(Color::White));
    }

    #[test]
    fn scholars_mate() {
        let mut game = create_game();
        game.move_from_to((4, 6), (4, 4));
        game.move_from_to((4, 1), (4, 3));
        game.move_from_to((5, 7),(2, 4));
        game.move_from_to((1, 0),(2, 2));
        game.move_from_to((3, 7),(7, 3));
        game.move_from_to((6, 0),(5, 2));
        game.move_from_to((7, 3),(5, 1));
        game.draw();
        assert!(game.is_checked(Color::Black));
    }

    #[test]
    fn casteling() {
        let mut game = create_game();
        game.move_from_to((0, 6), (0, 5));
        game.move_from_to((1, 0), (0, 2));
        game.move_from_to((1, 6), (1, 5));
        game.move_from_to((1, 1), (1, 2));
        game.move_from_to((2, 6), (2, 5));
        game.move_from_to((2, 0), (1, 1));
        game.move_from_to((3, 6), (3, 5));
        game.move_from_to((3, 1), (3, 2));
        game.move_from_to((4, 6), (4, 5));
        game.move_from_to((3, 0), (3, 1));
        assert_eq!(
            game.can_castle(Color::Black),
            CanCastle {
                left: true,
                right: false
            }
        );
        assert_eq!(
            game.can_castle(Color::White),
            CanCastle {
                left: false,
                right: false
            }
        );
        game.move_from_to((6, 6), (6, 5));
        //performing the castling
        game.move_from_to((4, 0), (2, 0));
    }

}
