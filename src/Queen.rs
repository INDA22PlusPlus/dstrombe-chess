use crate::board::*;
use crate::util::*;
use crate::piece::*;

pub struct Queen {    
    color : Color,
    pos : Pos,
    piece_type : PieceType
}
impl Queen {
    pub fn new (color : Color, pos : Pos) -> Self {
        Self {color, pos, piece_type : PieceType::Queen}
    }
}

impl Piece for Queen {
	fn get_color(&self) -> Color {
		self.color
	}
    fn get_pos(&self) -> Pos {
        self.pos
    }
    fn get_type(&self) -> PieceType {
        self.piece_type
    }
    fn legal_moves(&self, board : &Board) -> Vec<Pos> {
        let legal_moves =  self.threat_map(&board).into_iter().filter(|m| !board.move_causes_self_check(self as &dyn Piece, (m.clone()), None));
        legal_moves.collect::<Vec<Pos>>()
    }
    // we have to check both the straight and the diagonal avenues of movement to get the full sets
    // of moves for the queen
   	fn threat_map(&self, board : &Board) -> Vec<Pos> {
        let mut moves_diag = do_diag_move_check(&Box::new(self as &dyn Piece), &board); 
        let mut moves_straight = do_straight_move_check(&Box::new(self as &dyn Piece), &board);        
        moves_diag.append(&mut moves_straight);
        moves_diag
    }
        
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn rook_test() {
        /*
		let rook = Queen::new(color::White, Pos{x : 0, y : 0})
        assert_eq!(true, b == );
		assert_eq!(Pos {x : 9, y : 4}, b + a);
		assert_eq!(Pos {x : -1, y : -2}, b - a);
	    */
    }
}
