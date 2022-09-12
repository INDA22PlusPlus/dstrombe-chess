use crate::util::*;
use crate::board::*;

// This is needed for downcasting which rust does not support
#[derive(Copy, Clone, PartialEq)]
pub enum PieceType {
    Pawn,
    Rook,
    Queen,
    Knight,
    King,
    Bishop
}

pub trait Piece {
	// board : current board state w.r.t. which we want to find legal moves
	// returns: a vector of positions that can be moved to
    fn legal_moves(&self, board : Board) -> Vec<Pos>;
    
    // returns : piece color
    fn get_color(&self) -> Color;
    
    // returns : piece position
    fn get_pos(&self) -> Pos;

    // LIDL downcasting, since rust doesn't support it
    // could not think up any better solution
    fn get_type(&self) -> PieceType;
}



