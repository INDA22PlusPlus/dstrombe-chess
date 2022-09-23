use crate::util::*;
use crate::board::*;
use crate::pawn::*;
use crate::bishop::*;
use crate::knight::*;
use crate::rook::*;
use crate::queen::*;
use crate::king::*;
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
    fn legal_moves(&self, board : &Board) -> Vec<Pos>;
    
    // returns : squares threatened by this piece
    fn threat_map(&self, board : &Board) -> Vec<Pos>;

    // returns : piece color
    fn get_color(&self) -> Color;
    
    // returns : piece position
    fn get_pos(&self) -> Pos;

    // LIDL downcasting, since rust doesn't support it
    // could not think up any better solution
    fn get_type(&self) -> PieceType;
}
//ok mb we can clone piece using this trick
impl Clone for Box<dyn Piece> {
    fn clone(&self) -> Box<dyn Piece> {
        let s = self.as_ref();
        match s.get_type() {
            PieceType::Pawn => {
                Box::new(Pawn::new(s.get_color(), s.get_pos()) )
            },
            PieceType::Knight => {
                Box::new(Knight::new(s.get_color(), s.get_pos()) )
            },
            PieceType::Bishop => {
                Box::new(Bishop::new(s.get_color(), s.get_pos()) )
            },
            PieceType::Rook => {
                Box::new(Rook::new(s.get_color(), s.get_pos()) )
            },
            PieceType::Queen => {
                Box::new(Queen::new(s.get_color(), s.get_pos()) )
            },
            PieceType::King => {
                Box::new(King::new(s.get_color(), s.get_pos()) )
            },
        }
    }
}


// rust does not implicitly implement traits for their respective boxed variants.
// this is why we have to spec the implementation ourselves
impl Piece for Box<dyn Piece> {
     // board : current board state w.r.t. which we want to find legal moves
	// returns: a vector of positions that can be moved to
    fn legal_moves(&self, board : &Board) -> Vec<Pos> {
        self.as_ref().legal_moves(&board)
    }
    
    // returns : squares threatened by this piece
    fn threat_map(&self, board : &Board) -> Vec<Pos> {
        self.as_ref().threat_map(&board)
    }  


    // returns : piece color
    fn get_color(&self) -> Color {
        self.as_ref().get_color()
    }
    
    // returns : piece position
    fn get_pos(&self) -> Pos {
        self.as_ref().get_pos()
    }

    // LIDL downcasting, since rust doesn't support it
    // could not think up any better solution
    fn get_type(&self) -> PieceType {
        self.as_ref().get_type()
    }
}

