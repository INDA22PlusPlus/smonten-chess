fn main() {

    let size: usize = 8;

    let mut board = Board {
        board: vec![vec![Content::Empty; size]; size],
    };
    board.board[0] = vec![
        Content::Black(Piece::Rook),
        Content::Black(Piece::Knight),
        Content::Black(Piece::Bishop),
        Content::Black(Piece::Queen),
        Content::Black(Piece::King),
        Content::Black(Piece::Bishop),
        Content::Black(Piece::Knight),
        Content::Black(Piece::Rook),
    ];
    board.board[1] = vec![Content::Black(Piece::Pawn); size];
    board.board[6] = vec![Content::White(Piece::Pawn); size];

    board.board[7] = vec![
        Content::White(Piece::Rook),
        Content::White(Piece::Knight),
        Content::White(Piece::Bishop),
        Content::White(Piece::Queen),
        Content::White(Piece::King),
        Content::White(Piece::Bishop),
        Content::White(Piece::Knight),
        Content::White(Piece::Rook),
    ];


    board.draw();
    // board.my_move();
    // board.draw();
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
use std::fmt::Write;


#[derive(Clone)] // could not implement copy
struct Board {
    board: Vec<Vec<Content>>,
}



impl Board {
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
        let tmp = self.board[4][4];
        self.board[4][4] = self.board[2][2];
        self.board[2][2] = tmp;
    }
    fn get_possible_moves(&self, row: i32, col: i32) -> Vec<Vec<i32>> {
        vec![vec![0, 0], vec![row, col]]
    }
}
#[derive(Copy, Clone)]
enum Piece {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
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
                    Piece::Rook => "♖",
                    Piece::Queen => "♕",
                    Piece::King => "♔",
                }
            },
            Content::Black(piece) => {
                match piece {
                    Piece::Pawn => "♟",
                    Piece::Bishop => "♝",
                    Piece::Knight => "♞",
                    Piece::Rook => "♜",
                    Piece::Queen => "♛",
                    Piece::King => "♚",
                }
            },
            Content::Empty => "_",
        }
    }
}
impl std::fmt::Display for Content {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.get_symbol())
    }
}

