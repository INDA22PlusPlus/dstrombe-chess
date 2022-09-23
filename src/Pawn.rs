use crate::board::*;
use crate::util::*;
use crate::piece::*;

// TODO: refactor, this was written before introducing many of the utils
pub struct Pawn {    
    color : Color,
    pos : Pos,
    piece_type : PieceType
}
impl Pawn {
    pub fn new (color : Color, pos : Pos) -> Self {
        Self {color, pos, piece_type : PieceType::Pawn}
    }
}
impl Piece for Pawn {
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
   		let mut legal_moves = Vec::new();
   		
        // white pawns move up the board
   		let direction = match self.get_color() {
   			Color::White => -1,
   			Color::Black => 1
   		};
   		
   		// the ranks where the pawns would nominally spawn,
   		// to
   		let starting_rank = match self.get_color() {
   			Color::White => 6,
   			Color::Black => 1
   		};
   		
        // check moves where no captures are involved (forward moves)
        // we bounds check before any potential OOB array access
   		if self.pos.y + direction >= 0 && self.pos.y + direction <= 7 {
   			// check to see that the square immediately ahead of the pawn is free
   			if board.board[self.pos.x as usize][(self.pos.y + direction) as usize].is_none() {
                legal_moves.push(Pos {x : self.pos.x, y : (self.pos.y + direction)});
                
                // checking eligibility for a double-stride pawn move
                if self.pos.y == starting_rank {
                    
                    if self.pos.y + (direction * 2) >= 0 && self.pos.y + (direction * 2) <= 7 {
                        if board.board[self.pos.x as usize][(self.pos.y + direction) as usize].is_none() {
                            legal_moves.push(Pos {x : self.pos.x, y : (self.pos.y + direction * 2)});
                        }
                    }
                }  
            }
   		}
        legal_moves.append(&mut match board.piece_blocks_check(self as &dyn Piece){
            true => {
               self.threat_map(&board)
            }
            false => Vec::new()
        });
   	    legal_moves	
   	}
    fn threat_map(&self, board : &Board) -> Vec<Pos> {
        
        let mut legal_moves = Vec::new();
   		
        // white pawns move up the board
   		let direction = match self.get_color() {
   			Color::White => -1,
   			Color::Black => 1
   		};
   		
   		// the ranks where the pawns would nominally spawn,
   		// to
   		let starting_rank = match self.get_color() {
   			Color::White => 6,
   			Color::Black => 1
   		};
        // check capture moves incl en passant
        // TODO: evaluate whether or not to reuse the code that is semi-duped within
        // bc doing so would probably make it less readable
        if (self.pos.y + direction >= 0 && self.pos.y + direction <= 7) {
            
            // capture to the right
            // lower bound check unnecessary
            let can_en_passant = board.en_passant_target_square.is_some();
            if self.pos.x + 1 <= 7 { 
                let attack = Pos {x : self.pos.x +1, y : (self.pos.y + direction)};
                // check if either the square is an enemy, or the en passant target square
                // the en passant target square can only be attacked if an enemy double-moved
                if board[attack].is_some() && (board[attack].as_ref().unwrap().get_color() != self.color || (can_en_passant && attack == board.en_passant_target_square.unwrap())) {
                    legal_moves.push(attack)
                }
            }
            // capture to the left
            // upper bound check unnecessary
            if self.pos.x - 1 >= 0 { 
                let attack = Pos {x : self.pos.x - 1, y : (self.pos.y + direction)};
                if board[attack].is_some() && (board[attack].as_ref().unwrap().get_color() != self.color ||  (can_en_passant && attack == board.en_passant_target_square.unwrap())) {
                    legal_moves.push(attack)
                }
            }
            
        }
        legal_moves
    }
}
