fn main() {
    println!("Hello, world!");
    let p0 = Piece::Pawn(Color::White);
    for m in moves(p0) {
        println!("{}", m);
    }
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


#[derive(Debug)]
enum Color {
    Black,
    White,
}
#[derive(Debug)]
enum Piece {
    Pawn(Color),
    Bishop(Color),
    // Knight(Color),
    // Rook(Color),
    // Queen(Color),
    // King(Color),
}

fn moves(piece: Piece) -> Vec<&'static str> {
    match piece {
        Piece::Pawn(_) => vec!["_f2", "f1", "*fr1", "*fl1"],
        Piece::Bishop(_) => vec!["fr-", "fl-", "br-", "bl-"],
    }

}