use std::ops;
use crate::board::Board;
use crate::piece::Piece;
use std::fmt;
#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub enum Color{
    White,
    Black
}


// partialeq is implemented because it allows us to overload ==
// partialeq does not provide an assurance of reflexivity (i.e. a == a is always true) 
#[derive(Debug, Clone, Copy)]
pub struct Pos {
	pub x : i8,
	pub y : i8
}

// This could be replaced with derive(PartialEq) but I did this for novelty
impl PartialEq for Pos {
	fn eq(&self, right : &Pos) -> bool {
		self.x == right.x && self.y == right.y
	}
}


// QoL operator overloading, to reduce fluff in our core logic -> c : Pos = a + b 
// i.e. no need for c : Pos = Pos {a.x + b.x, a.y + b.y}
impl ops::Add<Pos> for Pos {
	type Output = Pos;
	
	fn add(self, right : Pos) -> Pos {
		Pos {x : self.x + right.x, y : self.y + right.y}
	}
}

impl ops::Sub<Pos> for Pos {
	type Output = Pos;
	
	fn sub(self, right : Pos) -> Pos {
		Pos {x : self.x - right.x, y : self.y - right.y}
	}
}


// useful note about trait lifetimes I found 
// Each trait object may have a different size, but all elements in Vec must have the same size. 
// If one dyn Trait was 1 byte, another was 100 bytes, and another was 5 bytes, you couldn't 
// address any trait object's data with just vec[n] in constant time, because there's no rule 
// what data starts where — after all each size is dynamic known only at runtime, so the Vec 
// would need to scan and measure all n-1 elements to know where the nth one starts.

pub fn piece_at_square(pos : Pos, board : &Board) -> Option<&'_ Box<dyn Piece + '_>> {
    if pos.x >= 0 && pos.x <= 7 && pos.y >= 0 && pos.y <= 8 {
        return board.board[pos.x as usize][pos.y as usize].as_ref();
    }
    else {
        None
    }
} 

// utility that checks if a given piece can move in a direction and for how long it can move in
// said direction, allowing the caller to specify a max range for eg. implementing the king moving
// only one square
pub fn cast_ray(cast_dir : Pos, cast_range : i8, casting_piece : &Box<&dyn Piece>, board: &Board) -> Vec<Pos> {
    let mut legal_moves = Vec::new();
    let mut square_to_check = casting_piece.get_pos() + cast_dir; 
    loop {
        if !board.is_within_board(square_to_check) {
            break;
        }
        let check = &board[square_to_check];
        match check {
            Some(collided_piece) => {
                // we stop upon encountering a piece, but also add the encountered piece if it
                // belongs to the opposing color, so that it may be captured
                if collided_piece.get_color() != casting_piece.get_color() {
                    legal_moves.push(square_to_check);
                }
                break;
            }
            None => {legal_moves.push(square_to_check);}
        };
        square_to_check = square_to_check + cast_dir; 
        //println!("x {} y {}", cast_dir.x.to_string(), cast_dir.y.to_string() );
    }
    legal_moves
}

pub fn do_straight_move_check(piece : &Box<&dyn Piece>, board : &Board) -> Vec<Pos> {
    let mut legal_moves : Vec<Pos> = Vec::new();
    let mut moves_right = cast_ray(Pos {x : 1, y : 0}, i8::MAX, piece, &board); 
    let mut moves_left = cast_ray(Pos {x : -1, y : 0}, i8::MAX, piece, &board); 
    let mut moves_up = cast_ray(Pos {x : 0, y : 1}, i8::MAX, piece, &board); 
    let mut moves_down = cast_ray(Pos {x : 0, y : -1}, i8::MAX, piece, &board); 
    
    legal_moves.append(&mut moves_right);
    legal_moves.append(&mut moves_left);
    legal_moves.append(&mut moves_up);
    legal_moves.append(&mut moves_down);
    legal_moves
}

pub fn do_diag_move_check(piece : &Box<&dyn Piece>, board : &Board) -> Vec<Pos> {

    let mut legal_moves : Vec<Pos> = Vec::new();
    let mut moves_right = cast_ray(Pos {x : 1, y : 1}, i8::MAX, piece, &board); 
    let mut moves_left = cast_ray(Pos {x : -1, y : 1}, i8::MAX, piece, &board); 
    let mut moves_up = cast_ray(Pos {x : -1, y : -1}, i8::MAX, piece, &board); 
    let mut moves_down = cast_ray(Pos {x : 1, y : -1}, i8::MAX, piece, &board);
 
    legal_moves.append(&mut moves_right);
    legal_moves.append(&mut moves_left);
    legal_moves.append(&mut moves_up);
    legal_moves.append(&mut moves_down);
    legal_moves

}
#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn pos_ops_test() {
		let a = Pos {x : 5, y : 3};
		let b = Pos {x : 4, y : 1};
		let c = Pos {x : 4, y : 1};
		assert_eq!(true, b == c);
		assert_eq!(Pos {x : 9, y : 4}, b + a);
		assert_eq!(Pos {x : -1, y : -2}, b - a);
	}
}

