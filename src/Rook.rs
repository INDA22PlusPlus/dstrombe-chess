use crate::board::*;
use crate::util::*;
use crate::piece::*;

pub struct Rook {    
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
    fn set_pos(&mut self, pos : Pos) {
        self.pos = pos;
    }
    fn get_type(&self) -> PieceType {
        self.piece_type
    }

    // the plan is to check each of the 4 directions that the rook can move
    fn legal_moves(&self, board : &Board) -> Vec<Pos> {
        let legal_moves =  self.threat_map(&board).into_iter().filter(|m| !board.move_causes_self_check(self as &dyn Piece, (m.clone()), None));
        let z = legal_moves.collect::<Vec<Pos>>();
        z
    }
   	fn threat_map(&self, board : &Board) -> Vec<Pos> {
   		let mut legal_moves : Vec<Pos> = do_straight_move_check(&Box::new(self as &dyn Piece), &board);
        legal_moves
    }
        
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn rook_test() {
        /*
		let rook = Rook::new(color::White, Pos{x : 0, y : 0})
        assert_eq!(true, b == );
		assert_eq!(Pos {x : 9, y : 4}, b + a);
		assert_eq!(Pos {x : -1, y : -2}, b - a);
	    */
    }
}
