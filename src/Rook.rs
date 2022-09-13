use crate::board::*;
use crate::util::*;
use crate::piece::*;

struct Rook {    
    color : Color,
    pos : Pos,
    piece_type : PieceType
}
impl Rook {
    pub fn new (color : Color, pos : Pos) -> Self {
        let str = "♖";
        println!("{}",str);
        Self {color, pos, piece_type : PieceType::Rook}
    }
}

impl Piece for Rook {
	fn get_color(&self) -> Color {
		self.color
	}
    fn get_pos(&self) -> Pos {
        self.pos
    }
    fn get_type(&self) -> PieceType {
        self.piece_type
    }
   	fn legal_moves(&self, board : Board) -> Vec<Pos> {
   		let mut legal_moves = Vec::new();
        
        // check left-right incl starting square
        for i in self.pos.x..7 {
            let piece_at_square = piece_at_square(Pos {x : i, y : self.pos.y}, &board); 
            match piece_at_square {
                Some(piece) => {
                    match piece.get_color() == self.color {
                        true => break,
                        false => {
                            legal_moves.push(Pos {x : i, y : self.pos.y});
                            break;
                        }
                    }
                }
                None => legal_moves.push(Pos {x : i, y : self.pos.y})
            };
        }
        // check right-left incl starting square
        if (self.pos.x > 0) {
            // order matters bc we break on encountering a piece, so .rev() it
            for i in (0..self.pos.x - 1).rev() {
                let piece_at_square = piece_at_square(Pos {x : i, y : self.pos.y}, &board); 
                match piece_at_square {
                    Some(piece) => {
                        match piece.get_color() == self.color {
                            true => break,
                            false => {
                                legal_moves.push(Pos {x : i, y : self.pos.y});
                                break;
                            }

                        }
                    }
                    None => legal_moves.push(Pos {x : i, y : self.pos.y})
                };
            }
        }
        legal_moves
    }
        
}
// cast a "ray" eminating from a point p that stops on encountering a piece,
// encompasses enemy pieces so that they may be marked for capture
// this is used so that the move availability for the rook may be implemented, i.e checks a row
// in one direction
// returns : squares that can be moved to given casting_piece and board
fn cast_straight_ray(range : std::ops::Range<i8>, casting_piece : &Box<dyn Piece>, board: &Board) -> Vec<Pos> {
    
    let mut legal_moves = Vec::new();
    // check left-right incl starting square
    let p_pos = casting_piece.get_pos();
    let p_col = casting_piece.get_color();
    for i in range {
        let piece_at_square = piece_at_square(Pos {x : i, y : p_pos.y}, &board); 
        match piece_at_square {
            Some(piece) => {
                match piece.get_color() == p_col {
                    true => break,
                    false => {
                        legal_moves.push(Pos {x : i, y : p_pos.y});
                        break;
                    }
                }
            }
            None => legal_moves.push(Pos {x : i, y : p_pos.y})
        };
    }
    legal_moves
}

