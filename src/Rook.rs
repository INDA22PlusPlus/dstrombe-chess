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
        }
        // check right-left incl starting square
        if (self.pos.x > 0) {
            for i in (0..self.pos.x - 1) {
                let piece_at_square = piece_at_square(Pos {x : i, y : self.pos.y}, &board); 
            }
        }
        legal_moves
    }
    
}
