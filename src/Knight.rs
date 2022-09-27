use crate::board::*;
use crate::util::*;
use crate::piece::*;

pub struct Knight {    
    color : Color,
    pos : Pos,
    piece_type : PieceType
}
impl Knight {
    pub fn new (color : Color, pos : Pos) -> Self {
        Self {color, pos, piece_type : PieceType::Knight}   
    }
}

impl Piece for Knight {
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
        let mut legal_moves : Vec<Pos> = Vec::new();
        // yes, a table of relative knight moves. Smarter solutions are available for piece-centric
        // engines
        let table = [Pos {x : 1, y : 2}, Pos {x : 2, y : 1}, Pos {x : -1, y : 2}, Pos {x : 1, y : -2}, Pos {x : -2, y : 1}, Pos {x : 2, y : -1}, Pos {x : -1, y : -2}, Pos {x : -2, y : -1}];
        for entry in table {
            let target_pos = self.pos + entry;
            if board.is_within_board(target_pos) {
                match &board[target_pos] {
                   Some(piece) => {
                       if piece.get_color() != self.color {
                           legal_moves.push(target_pos);
                       } 
                   }
                   None => {
                       legal_moves.push(target_pos);
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
		let rook = Knight::new(color::White, Pos{x : 0, y : 0})
        assert_eq!(true, b == );
		assert_eq!(Pos {x : 9, y : 4}, b + a);
		assert_eq!(Pos {x : -1, y : -2}, b - a);
	    */
    }
}
