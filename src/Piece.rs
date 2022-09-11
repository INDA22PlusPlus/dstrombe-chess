

trait Piece {
	// board : current board state w.r.t. which we want to find legal moves
	// returns: a vector of positions that can be moved to
   	fn legal_moves(&self, board : Board) -> Vec<Pos>;
    fn get_color(&self) -> Color;
}



