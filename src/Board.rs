use arrayvec::ArrayVec;
use crate::util::*;
use crate::piece::Piece;
pub struct Board {
	pub turn : Color,
	pub fiftymove_counter : u32,
    pub last_move_was_double_pawn_move : bool,
	pub ks_castle_allowed_white : bool,
	pub qs_castle_allowed_white : bool,
	pub ks_castle_allowed_black : bool,
	pub qs_castle_allowed_black : bool,

    // quick reference I found that makes sense for those confused about Box<dyn Trait>
    // "Each trait object may have a different size, but all elements in Vec must have the same size. 
    // If one dyn Trait was 1 byte, another was 100 bytes, and another was 5 bytes, you couldn't address 
    // any trait object's data with just vec[n] in constant time, because there's no rule what data 
    // starts where — after all each size is dynamic known only at runtime, so the Vec would need to 
    // scan and measure all n-1 elements to know where the nth one starts."
    pub board : ArrayVec<ArrayVec<Option<Box<dyn Piece>>, 8>, 8>
}
impl Board {
	// FIXME implement
	//fn is_mate() -> bool {
	//	false
	//}
}
