use tokio;

enum Color{
    White,
    Black
}

struct Board {
    board : Vec<Vec<Box<dyn Piece>>>
}

trait Piece {
    async fn is_legal_move(&self, from_x : i8, from_y : i8, to_x : i8, to_y : i8) -> bool;
    fn color(&self) -> Color;
}

struct Pawn {
    async fn is_legal_move(&self, board :  Board, from_x : i8, from_y : i8, to_x : i8, to_y : i8) -> bool {
        dir : bool = match self.color() {
            Color.White() => 1,
            Color.Black() => -1
        };
        let piece_at_square =piece_at_square = 
    }

    pub did_doublemove : bool,
}

struct King {

}

struct Queen {

}

struct Rook {

}

struct Bishop {

}

impl Piece for Pawn {
    async fn is_legal_move(&self, from)
}

