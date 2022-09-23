use arrayvec::ArrayVec;
use crate::util::*;
use crate::piece::*;
use crate::king::*;
use crate::queen::*;
use crate::rook::*;
use crate::bishop::*;
use crate::knight::*;
use crate::pawn::*;
use std::ops::{Index, IndexMut};

// this is what we have to do since we can't implement Clone on traits
// we have to instead keep track of history in order to revert moves
pub struct HistoryItem {
    pub moved_to : Pos,
    pub moved_from : Pos,
    pub captured_piece : Option<Box<dyn Piece>>,
    pub promoted_at : Option<Pos>,
    pub delta_ep_target_square : (Option<Pos>, Option<Pos>),

    ///ughghh yes bad v bad
    pub turn : Color,
	pub fiftymove_counter : u32,
    pub en_passant_target_square : Option<Pos>,
	pub ks_castle_allowed_white : bool,
	pub qs_castle_allowed_white : bool,
	pub ks_castle_allowed_black : bool,
	pub qs_castle_allowed_black : bool,
    pub last_move_was_double_pawn_move : bool,
    pub move_cnt : i32,
}

pub struct GameOver {
    white_score : f32,
    black_score : f32,
    game_over_type : GameOverType

}

pub enum GameOverType {
    Checkmate,
    Resigns,
    Stalemate,
    Threefold,
    Fiftymoverule,
    DrawByAgreement
}

#[derive(Clone)]
pub struct Board {
	pub turn : Color,
	pub fiftymove_counter : u32,
    pub en_passant_target_square : Option<Pos>,
	pub ks_castle_allowed_white : bool,
	pub qs_castle_allowed_white : bool,
	pub ks_castle_allowed_black : bool,
	pub qs_castle_allowed_black : bool,
    pub last_move_was_double_pawn_move : bool,
    pub move_cnt : i32,
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
    fn create_starting_board() -> Self {
        let starting_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        return Board::from_fen(starting_fen.to_owned());
    }

    pub fn print(&self, highlight: Option<Vec<Pos>>) -> String {
        let mut built : String = "".to_string();
        built.push_str(&format!("Turn {}\n", self.move_cnt));
        
        for row in &self.board {
            for square in row {/* 
                if(is_some(highlight)) {
                    if highlight.unwrap().contains() {
                        built.push_str("o ");
                        continue;
                    }
                }*/
                match square {
                    
                    None => {built.push_str("  ");}, 
                    Some(piece) => {
                        match piece.get_color() {
                            Color::White => {
                                match piece.get_type() {
                                    PieceType::Pawn => {
                                        built.push_str("♙ ");
                                    }
                                    PieceType::Knight => {
                                        built.push_str("♘ ");
                                    }
                                    PieceType::Bishop => {
                                        built.push_str("♗ ");
                                    }
                                    PieceType::Rook => {
                                        built.push_str("♖ ");
                                    }
                                    PieceType::Queen => {
                                        built.push_str("♕ ");
                                    }
                                    PieceType::King => {
                                        built.push_str("♔ ");
                                    }
                                };
                            }
                            Color::Black => {
                                match piece.get_type() {
                                    PieceType::Pawn => {
                                        built.push_str("♟ ");
                                    }
                                    PieceType::Knight => {
                                        built.push_str("♞ ");
                                    }
                                    PieceType::Bishop => {
                                        built.push_str("♝ ");
                                    }
                                    PieceType::Rook => {
                                        built.push_str("♜ ");
                                    }
                                    PieceType::Queen => {
                                        built.push_str("♛ ");
                                    }
                                    PieceType::King => {
                                        built.push_str("♚ ");
                                    }
                                };
                            }
                        };
                    }
                };
            }
            built.push_str("\n");
        }
        built
    }

    pub fn new() -> Self {
        Self::create_starting_board()
    }
    //
    pub fn piece_blocks_check(&self, piece : &dyn Piece) -> bool {
        let mut copy = self.clone();
        copy[piece.get_pos()] = None;
        copy.is_check_for(piece.get_color())
    }
    pub fn is_check_for(&self, color : Color) -> bool {
        let king = self.get_king(color);
        for row in &self.board {
            for square in row {
                match square {
                    None => {}, 
                    Some(piece) => {
                        if piece.get_color() != color {
                            if piece.threat_map(self).contains(&king.get_pos()){
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }
    pub fn is_game_over() -> Option<GameOver>{
        None
    }
    //fn is_mate(&self) -> Color {
//
  //  } 
    pub fn get_possible_moves_at_square(&self, pos : Pos) -> Vec<Pos> {
        match &self[pos] {
            None => Vec::new(),
            Some(piece) => piece.legal_moves(&self)
        }
    }
    pub fn perform_move(&mut self, from : Pos, to : Pos) {
        let mut from_piece = self[from].as_ref();
        let mut captured_piece = self[to].clone();
        
        // check if we are promoting
        let promoted = (to.y == 7 || to.x == 0) && from_piece.expect("from is None, impossible board state").get_type() == PieceType::Pawn;
        let promoted_at = match promoted {
            true => Some(from),
            false => None
        };
        // keep track of en-passant
        let ep_square = match from_piece.expect("from is None, impossible board state").get_type() == PieceType::Pawn {
            true => {
                if (to.y - from.y) == 2 {
                    Some(to)
                } else {
                    None
                }
            }
            false => {
                None
            }
        };

        let delta_ep_target_square = (self.en_passant_target_square.clone(), ep_square);
        let generated_history = HistoryItem {
            moved_to : to,
            moved_from : from,
            captured_piece : captured_piece,
            promoted_at : promoted_at,
            delta_ep_target_square : delta_ep_target_square,

            ///ughghh yes bad v bad
            turn : self.turn,
            fiftymove_counter : self.fiftymove_counter,
            en_passant_target_square : self.en_passant_target_square,
            ks_castle_allowed_white : self.ks_castle_allowed_white,
            qs_castle_allowed_white : self.qs_castle_allowed_white,
            ks_castle_allowed_black : self.ks_castle_allowed_black,
            qs_castle_allowed_black : self.qs_castle_allowed_black,
            last_move_was_double_pawn_move : self.last_move_was_double_pawn_move,
            move_cnt : self.move_cnt,
        };

        self.move_cnt += 1;
        self.fiftymove_counter += 1; // FIXME
        self.turn = match self.turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
        let from_piece = self[from].clone();
        self[to] = from_piece.clone();
        self[from] = None;

        //generated_history
    }
    
    fn restore_board_from_history(&mut self, history : HistoryItem) {
        self[history.moved_from] = self[history.moved_to].clone();
        
        //FIXME: revert promotion and ep_target
        self.turn = history.turn;
        self.fiftymove_counter = history.fiftymove_counter;
        self.en_passant_target_square = history.en_passant_target_square;
        self.ks_castle_allowed_white = history.ks_castle_allowed_white;
        self.qs_castle_allowed_white = history.qs_castle_allowed_white;
        self.ks_castle_allowed_black = history.ks_castle_allowed_black;
        self.qs_castle_allowed_black = history.qs_castle_allowed_black;
        self.last_move_was_double_pawn_move = history.last_move_was_double_pawn_move;
        self.move_cnt = history.move_cnt;
        self[history.moved_to] = history.captured_piece;
    }

    fn get_king(&self, color : Color) -> Box<dyn Piece> {
        for row in &self.board {
            for square in row {
                match square {
                    None => {},
                    Some(piece) => {
                        if piece.get_color() == color {
                            if piece.get_type() == PieceType::King {
                                return piece.clone();
                            }
                        }
                    }
                };
            
            }
        }
        panic!("there is no king!");
    }

    fn is_mate_for_color(&self, color : Color) -> bool {
        let mut found_defense = false;
        for row in &self.board {
            for square in row {
                match square {
                    None => {}, 
                    Some(piece) => {
                        
                        // only check defending moves for our pieces
                        if piece.get_color() != color {
                            continue;
                        }
                        // check if every piece is able to defend against the check
                        // if not then it's mate
                        for attempted_move in piece.legal_moves(&self) {
                            let mut board_clone = self.clone();
                            board_clone.perform_move(piece.get_pos(), attempted_move);
                            if !board_clone.is_check_for(color) {
                                found_defense = true;
                                break;
                            }
                        }
                    }
                }
            }
        } 
        found_defense
    }
    fn is_stalemate(&self) -> bool {
        false
    }

    // This function allows the import of board positions from a given FEN
    // which also has some utility within our program, eg for easily generating
    // the default starting position
    // https://en.wikipedia.org/wiki/Forsyth–Edwards_Notation 
    // TODO: break into sub-functions and return a Result instead of panicking 
    fn from_fen(fen : String) -> Self {
        let mut params = fen.split(" ").collect::<Vec<&str>>();
        let mut ranks = params[0].split("/").collect::<Vec<&str>>();
        let mut curr_square : Pos = Pos{x : 0, y : 0};
        let mut board : ArrayVec<ArrayVec<Option<Box<dyn Piece>>, 8>, 8> = ArrayVec::<_, 8>::new();
        let turn;
        let fiftymove_counter;
        let en_passant_target_square;
        let mut ks_castle_allowed_white = false;
        let mut ks_castle_allowed_black = false;
        let mut qs_castle_allowed_black = false;
        let mut qs_castle_allowed_white = false;
        let last_move_was_double_pawn_move = false;
        let mut move_cnt = 1; // it starts at 1
        for (i, rank) in ranks.iter().enumerate() {
            let mut rank_to_add = ArrayVec::<_, 8>::new();
            for c in rank.chars() {
                let square : Option<Box<dyn Piece>> = match(c) {
                    'p' =>  Some(Box::new(Pawn::new(Color::Black, curr_square))),
                    'n' =>  Some(Box::new(Knight::new(Color::Black, curr_square))),
                    'b' =>  Some(Box::new(Bishop::new(Color::Black, curr_square))),
                    'r' =>  Some(Box::new(Rook::new(Color::Black, curr_square))),
                    'q' =>  Some(Box::new(Queen::new(Color::Black, curr_square))),
                    'k' =>  Some(Box::new(King::new(Color::Black, curr_square))),
                    'P' =>  Some(Box::new(Pawn::new(Color::White, curr_square))),
                    'N' =>  Some(Box::new(Knight::new(Color::White, curr_square))),
                    'B' =>  Some(Box::new(Bishop::new(Color::White, curr_square))),
                    'R' =>  Some(Box::new(Rook::new(Color::White, curr_square))),
                    'Q' =>  Some(Box::new(Queen::new(Color::White, curr_square))),
                    'K' =>  Some(Box::new(King::new(Color::White, curr_square))),
                    '0'..='9' => {
                        //FIXME ugly hack remove this
                        // we are skipping empty squares, numbers denote empty squares
                        // in FEN
                        curr_square.x -= 1;
                        let skip = (c as i32 - 49) as i8; // ascii ints range from 48-57
                        curr_square.x += skip;
                        for z in 0..skip {
                            rank_to_add.push(None);
                        }
                        None
                    }
                    _ => panic!("invalid FEN board configuration"),
                };
                rank_to_add.push(square);
                curr_square.x += 1;
            }
            board.push(rank_to_add);
            curr_square.x = 0;
            curr_square.y += 1
        }
        
        turn = match params[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => panic!("invalid color during FEN parsing")
        };
        
        let castling_availability_chars = params[2].chars();
        for c in castling_availability_chars {
            match c {
                'K' => ks_castle_allowed_white = true,
                'Q' => qs_castle_allowed_white = true,
                'k' => ks_castle_allowed_black = true,
                'q' => qs_castle_allowed_black = true,
                '-' => break,
                _ => panic!("invalid FEN when parsing castling availability")
            };
        }

        let en_passant_target_str = params[3];
        en_passant_target_square = match en_passant_target_str.len() {
            1 => None, // - and other invalid strings, we could panic on chars that are not "-"
            2 => {
                let chars = en_passant_target_str.chars().collect::<Vec<char>>();
                let x = match chars[0] {
                    'a'..='h' => {
                        (chars[0] as i32 - 97) as i8 // 97 is a in ascii
                    } 
                    _ => panic!("invalid en passant square when parsing FEN")
                };
                let y = match chars[1] {
                    '1'..='8' => {
                        7 - (chars[1] as i32 - 49) as i8 // 49 is 1 in ascii and idx 0 is the
                                                             // 8th rank
                    } 
                    _ => panic!("invalid en passant square when parsing FEN")
                };
                Some(Pos {x, y})
            }
            _ => panic!("invalid en passant square when parsing FEN")
        };
        
        let fiftymove_param = params[4];
        fiftymove_counter = fiftymove_param.parse().unwrap();

        let move_cnt_param = params[5];
        move_cnt = move_cnt_param.parse().unwrap();
        
        Board {
            turn, 
            fiftymove_counter,
            en_passant_target_square,
            ks_castle_allowed_white,
            ks_castle_allowed_black,
            qs_castle_allowed_white,
            qs_castle_allowed_black,
            last_move_was_double_pawn_move,
            move_cnt,
            board
        }
    }
    
    // TODO: change return type to i8 or sth? Surely we will always play on a square board
    fn get_size(&self) -> Pos {
        return Pos { x: 8, y : 8};
    }
    pub fn is_within_board(&self, pos : Pos) -> bool {
        let size = self.get_size();
        return pos.x >= 0 && pos.y >= 0 && pos.x <= size.x && pos.y <= size.y
    }
    
}

// allow us to iterate over the pieces of the board easier
/* 
impl Iterator for Board {
    type Item = Option<Box<dyn Piece>>;

    fn next(&mut self) -> Self::Item {
        self.count
    }
}
*/
// allow indexing the board using a Pos directly instead of going
// board[pos.x][pos.y]. Allows board[pos]
impl Index<Pos> for Board {
    type Output = Option<Box<dyn Piece>>;
    
    fn index(&self, pos : Pos) -> &Self::Output {
        &self.board[pos.x as usize][pos.y as usize]
    }
}

impl IndexMut<Pos> for Board {
    fn index_mut(&mut self, pos : Pos) -> &mut Self::Output {
        &mut self.board[pos.x as usize][pos.y as usize]
    }
}