# This chess library can: #
* Keep track of all pieces
* Keep track of who's turn it is
* Manage all basic moves and kills
* Determine if a player is checked or checkmated
* Manage castling
* Manage Promotion

# This chess library CAN NOT 💀: #
* Manage En Passant


## user interface ##
call | does | example
------------- | ------------- | -------------
`create_game()`  | creates Game instance | `let mut game = create_game()`
`get_destinations(xy: (usize, usize))` | returns enum `Destinations` (`Empty` or `Occupied(Vec<(usize, usize)>))` for the square at coordinates xy | `let destinations = game.get_destinations((0, 1))`
`move_from_to(from: (usize, usize), to: (usize, usize))` | moves piece | `game.move_from_to((4, 0), (2, 0))`
`draw()` | prints current game board to terminal, for debugging | `game.draw()`
`get_turn()` | returns enum `Color` (`Balack` or `White`), color of players who's turn it is | `cur_turn_col = game.get_turn()`
`get_gamestate()` | returns enum `GameState` (`NoThreats`or `IsChecked(Color)` or `IsCheckMated(Color)`) | `let cur_state = game.get_gamestate()`
`coordinates_playable(xy: (usize, usize))` | returns true/false if quare at xy if current player has a piece there | `let is_playable = game.coordinates_playable((4, 3))`
`get_promotion_state()` | returns enum `Promotion`( `MustPromote(Color, (usize, usize))` or `None` ) | `let promstate = game.get_promotion_state()`
`promote(new_piece_type: PieceType)` | replaces the piece that must be promoted with `new_piece_type`in correct color | `game.promote(PieceType::Queen)`

This is what I thought most important but more methods for checking states and possibilites are available in the library

## important notes ##
* promotion works like this: after a player has moved one of their pawns to last rank, the library expects the next action to be a call of `promote()`, any attempt to make a move will cause a `panic!`. Thus the user of this library need to check `get_promotion_state()` regularly.

## useful imports when using the library (how I made it work) ##
```
extern crate chess_lib;
use chess_lib::{Game, create_game, Destination, Color, PieceType};
```

 ___

> *When I Wrote this, Only God and I understood it; Now God Alone Knows*