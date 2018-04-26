pub const WHITE_FIELD: char = '◻';
pub const BLACK_FIELD: char = '◼';

pub struct Board {
	board: [[char; 9]; 9],
}

impl Board {
	pub fn new() -> Board {
		Board {
			board: [
				['*','A','B','C','D','E','F', 'G','H'],
				['1','♖','♘','♗','♕','♔','♗','♘','♖'],
				['2','♙','♙','♙','♙','♙','♙','♙','♙'],
				['3','◻','◼','◻','◼','◻','◼','◻','◼'],
				['4','◼','◻','◼','◻','◼','◻','◼','◻'],
				['5','◻','◼','◻','◼','◻','◼','◻','◼'],
				['6','◼','◻','◼','◻','◼','◻','◼','◻'],
				['7','♟','♙','♟','♟','♟','♟','♟','♟'],
				['8','♜','♞','♝','♛','♚','♝','♞','♜'],
			]
		}
	}

	pub fn draw(&self) {
		for row in self.board.into_iter().rev() { 
			for character in row { 
				print!("{} ", character); 
			} 
			println!(""); 
		}
	}

	pub fn check_bounds(pos: (i32, i32)) -> Result<(), String> {
		if pos.0 < 1 || pos.1 < 1 || pos.0 > 8 || pos.1 > 8 { return Err("input out of bounds!".to_string()); }
		Ok(())
	}

	pub fn get_field_content(&self, pos: (i32, i32)) -> char {
		self.board[pos.0 as usize][pos.1 as usize]
	}

	pub fn field_empty(&self, pos: (i32, i32)) -> bool { 
		(self.get_field_content(pos) == WHITE_FIELD || self.get_field_content(pos) == BLACK_FIELD)
	}

	pub fn move_char(&mut self, old_pos: (i32, i32), new_pos: (i32, i32)) {
		let old_pos = (old_pos.0 as usize, old_pos.1 as usize);
		let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
		self.board[new_pos.0][new_pos.1] = self.board[old_pos.0][old_pos.1];
		self.board[old_pos.0][old_pos.1] = {
			let val = old_pos.0 + 8 * old_pos.1;
			if (val % 2) == 0 { '◻' } else { '◼' }
		};
	}
}
