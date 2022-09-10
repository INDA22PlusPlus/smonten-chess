fn main() {
    println!("Hello, world!");
    let p0 = Piece {
        kind: PieceKind::Pawn,
        color: Color::White,
    };
    dbg!(p0);
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
enum PieceKind {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

#[derive(Debug)]
struct Piece {
    kind: PieceKind,
    color: Color,
}
