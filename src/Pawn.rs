struct Pawn {    
	pub color : Color,
	pub did_doublemove : bool,
}

impl Piece for Pawn {
	fn get_color(&self) -> Color {
		self.color
	}
   	fn legal_moves(&self, from : Pos, to : Pos) -> Vec<Pos> {
   		let mut a = Vec::new();
   		a.push(Pos {x : 1, y : 2});
   		a
   	}
}
