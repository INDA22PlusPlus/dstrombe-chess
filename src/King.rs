use crate::board::*;
use crate::util::*;
use crate::piece::*;

pub struct King {    
    color : Color,
    pos : Pos,
    piece_type : PieceType
}
impl King {
    pub fn new (color : Color, pos : Pos) -> Self {
        Self {color, pos, piece_type : PieceType::King}
    }
}

impl Piece for King {
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
    fn threat_map(&self, board : &Board) -> Vec<Pos> {
        // check all the squares adjacent to the king
        let mut legal_moves : Vec<Pos> = Vec::new();
        for i in 0..9 {
            let y = i / 3;
            let x = i % 3;
            let target = Pos {x : x as i8, y : y as i8};
            if board.is_within_board(target) {
                // FIXME: disallow moving to threatened squares
                match &board[target] {
                    Some(piece) => {
                        if piece.get_color() == self.color {
                            legal_moves.push(target);
                        }
                    }
                    None => {
                        legal_moves.push(target);
                    }
                } 
            }
        }
        legal_moves
    }
        
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn rook_test() {
        /*
		let rook = King::new(color::White, Pos{x : 0, y : 0})
        assert_eq!(true, b == );
		assert_eq!(Pos {x : 9, y : 4}, b + a);
		assert_eq!(Pos {x : -1, y : -2}, b - a);
	    */
    }
}
