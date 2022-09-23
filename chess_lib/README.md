#This chess library can#
* Keep track of all pieces
* Keep track of who's turn it is
* Manage all basic moves and kills
* Determine if a player is checked or checkmated
* Manage castling



##user interface##
call | does | example
------------- | ------------- | -------------
`create_game()`  | creates Game instance | `let mut game = create_game()`
`get_destinations(xy: (usize, usize))` | returns enum Destinations (Empty or Occupied(Vec<(usize, usize)>)) \n hej | \|