# This chess library can #
* Keep track of all pieces
* Keep track of who's turn it is
* Manage all basic moves and kills
* Determine if a player is checked or checkmated
* Manage castling

# This chess library CAN NOT 💀#
* Manage Promotion
* Manage En Passant


## user interface ##
call | does | example
------------- | ------------- | -------------
`create_game()`  | creates Game instance | `let mut game = create_game()`
`get_destinations(xy: (usize, usize))` | returns enum `Destinations` (`Empty` or `Occupied(Vec<(usize, usize)>))` for the square at coordinates xy | `let destinations = game.get_destinations((0, 1))`
`move_from_to(from: (usize, usize), to: (usize, usize))` | moves piece | `game.move_from_to((4, 0), (2, 0))`
`draw()` | prints current game board to terminal | `game.draw()`
`get_turn()` | returns enum `Color` (`Balack` or `White`), color of players who's turn it is | `cur_turn_col = game.get_turn()`
`is_checked(color: Color)` | returns true/false | `is_white_checked = is_checked(Color::White)`
`is_checkmated(color: Color)` | returns true/false | `is_white_checkmated = is_checked(Color::Black)`
