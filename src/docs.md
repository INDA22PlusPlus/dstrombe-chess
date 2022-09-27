## Docs

## Pos
```rs
pub struct Pos {
	pub x : i8,
	pub y : i8
}
```
``Pos`` is used to represent a coordinate on the board. 
``x``: 0 is the leftmost position, 7 is the rightmost position.
``y`` : 0 is the *top*-most position and 7 is the *bottom*-most position. That is, whites paws start at y = 6.
You can add, subtract and compare Pos with the usual +, - and == ops

## Board
```rs
pub struct Board {
    pub turn : Color,
    pub fiftymove_counter : u32,
    pub en_passant_target_square : Option<Pos>,
    pub ks_castle_allowed_white : bool,
    pub qs_castle_allowed_white : bool,
    pub ks_castle_allowed_black : bool,
    pub qs_castle_allowed_black : bool,
    pub move_cnt : i32,
    pub board : ArrayVec<ArrayVec<Option<Box<dyn Piece>>, 8>, 8>
}```
The members of ``Board`` are not too interesting. To index the board, i.e. to get or set a piece at a given square,
you index the board as follows:
```rs
let mut board = Board::new();
let pos = Pos {2, 3}
let piece = board[pos];
```
Useful functions:
```rs
// this will return an array of squares that can be moved to.
pub fn get_possible_moves_at_square(&self, pos : Pos) -> Vec<Pos> 
// make a move. this function takes an Option<PieceType> if you want to promote. If not, pass None.
pub fn perform_move(&mut self, from : Pos, to : Pos,  promotion : Option<PieceType>) -> Result<(), &'static str>{
pub fn is_check_for(&self, color : Color) -> bool // if you want to eg. make the king red when checked

```


