fn main() {
    let mut board = Board {
        board: vec![
            vec![Content::Empty, Content::White(Piece::Pawn)],
            vec![Content::Black(Piece::Pawn), Content::Empty]
        ],
    };
    board.draw();
    board.my_move();
    board.draw();
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

enum Coin {
    Penny,
    Nickel,
}

#[derive(Clone)] // could not implement copy
struct Board {
    board: Vec<Vec<Content>>,
}



impl Board {
    fn draw(&self) {
        for row in &self.board {
            for square in row {
                print!("|{}", square);
            }
            println!("|");
        }
    }
    fn my_move(&mut self) {
        let tmp = self.board[1][0];
        self.board[1][0] = self.board[0][0];
        self.board[0][0] = tmp;
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
            Content::White(Piece::Pawn) => "♟",
            Content::Black(Piece::Pawn) => "♙",
            Content::Empty => "_",
        }
    }
}
impl std::fmt::Display for Content {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.get_symbol())
    }
}

