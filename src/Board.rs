use ArrayVec::*;

struct Board {
	turn : Color,
	fiftymove_counter : u32,
	ks_castle_allowed_white : bool,
	qs_castle_allowed_white : bool,
	ks_castle_allowed_black : bool,
	qs_castle_allowed_black : bool,
    board : ArrayVec<ArrayVec<Box<dyn Piece>, 8>, 8>
}
impl Board {
	// FIXME implement
	//fn is_mate() -> bool {
	//	false
	//}
}
