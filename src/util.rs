use std::ops;


// partialeq is implemented because it allows us to overload ==
// partialeq does not provide an assurance of reflexivity (i.e. a == a is always true) 
#[derive(Debug, Clone, Copy)]
struct Pos {
	pub x : i8,
	pub y : i8
}

// This could be replaced with derive(PartialEq) but I did this for novelty
impl PartialEq for Pos {
	fn eq(&self, right : &Pos) -> bool {
		self.x == right.x && self.y == right.y
	}
}


// QoL operator overloading, to reduce fluff in our core logic -> c : Pos = a + b 
// i.e. no need for c : Pos = Pos {a.x + b.x, a.y + b.y}
impl ops::Add<Pos> for Pos {
	type Output = Pos;
	
	fn add(self, right : Pos) -> Pos {
		Pos {x : self.x + right.x, y : self.y + right.y}
	}
}

impl ops::Sub<Pos> for Pos {
	type Output = Pos;
	
	fn sub(self, right : Pos) -> Pos {
		Pos {x : self.x - right.x, y : self.y - right.y}
	}
}


#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn add_test() {
		let a = Pos {x : 5, y : 3};
		let b = Pos {x : 4, y : 1};
		let c = Pos {x : 4, y : 1};
		assert_eq!(true, b == c);
		assert_eq!(Pos {x : 9, y : 4}, b + a);
		assert_eq!(Pos {x : -1, y : -2}, b - a);
	}
}

