use crate::board::*;
use crate::util::*;
use crate::piece::*;

pub struct Bishop {    
    color : Color,
    pos : Pos,
    piece_type : PieceType
}

impl Bishop {
    pub fn new (color : Color, pos : Pos) -> Self {
        Self {color, pos, piece_type : PieceType::Bishop}
    }
}

impl Piece for Bishop {
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

   	fn legal_moves(&self, board : &Board) -> Vec<Pos> {
        let legal_moves =  self.threat_map(&board).into_iter().filter(|m| !board.move_causes_self_check(self as &dyn Piece, (m.clone()), None));
        legal_moves.collect::<Vec<Pos>>()
    }

    fn threat_map(&self, board : &Board) -> Vec<Pos> {
        do_diag_move_check(&Box::new(self as &dyn Piece), &board)
    }
        
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn rook_test() {
        /*
		let rook = Bishop::new(color::White, Pos{x : 0, y : 0})
        assert_eq!(true, b == );
		assert_eq!(Pos {x : 9, y : 4}, b + a);
		assert_eq!(Pos {x : -1, y : -2}, b - a);
	    */
    }
}
