use std::vec::Vec;
use figure::{Vector, Figure, self};

const WHITE_FIELD: char = '◻';
const BLACK_FIELD: char = '◼';


pub struct Board {
	figures: Vec<Figure>,
	board: [[char; 9]; 9],
}

impl Board {
	pub fn new() -> Board {
		let mut figures = Vec::new();
		figures.push(Figure::new(figure::WHITE_QUEEN, 1, 4));

		figures.get(0).expect("fefvw").move_figure(figure::BLACK);


		Board {
			figures,
			board: [
				['8','♜','♞','♝','♕','♔','♝','♞','♜'],
				['7','♟','♟','♟','♟','♟','♟','♟','♟'],
				['6','◻','◼','◻','◼','◻','◼','◻','◼'],
				['5','◼','◻','◼','◻','◼','◻','◼','◻'],
				['4','◻','◼','◻','◼','◻','◼','◻','◼'],
				['3','◼','◻','◼','◻','◼','◻','◼','◻'],
				['2','♗','♗','♗','♗','♗','♗','♗','♗'],
				['1','♖','♘','♗','♕','♔','♗','♘','♖'],
				['*','A','B', 'C','D','E','F', 'G','H'],
			]
		}
	}

	pub fn draw(&self) {
		for row in self.board.into_iter() {
			for character in row {
				print!("{} ", character);
			}
			println!("");
		}
	}

	pub fn update(&self, old_pos: Vector<u32>, new_pos: Vector<u32>) {

	}
}
