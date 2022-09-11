struct Pawn {    
	pub color : Color,
	pub did_doublemove : bool,
}

impl Piece for Pawn {
	fn get_color(&self) -> Color {
		self.color
	}
	
   	fn legal_moves(&self, board : Board) -> Vec<Pos> {
   		let from : Pos;
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
   		
   		if from.y + direction >= 0 && from.y + direction <= 7 {
   			// check that the thing is not occupied lole
   			if(board[i]) 
   			legal_moves.push(Pos {from.x, from.y + direction})
   		}
   		
   		if from.y == starting_rank {
 			 		
   		}
   		a.push(Pos {x : 1, y : 2});
   		a
   	}
}
