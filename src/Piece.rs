trait Piece {
   	fn legal_moves(&self, from : Pos, to : Pos) -> Vec<Pos>;
    fn get_color(&self) -> Color;
}

