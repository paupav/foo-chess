use std::io::{self, Write};

pub type ValidMovement = Result<(i32, i32), String>;
pub struct Interface;

impl Interface {
	pub fn read_input() -> ValidMovement {
		let mut input = String::new();
		print!(" Enter one number and one letter: ");
		let _ = io::stdout().flush();
		io::stdin().read_line(&mut input).expect("failed to read from standard input 1");

		let mut parts = input.split_whitespace();

		match (parts.next(), parts.next()) {

			(Some(number), Some(letter)) =>
			{
				let row = { // ? isn't implemented
					if let Ok(num) = number.parse::<i32>() {
						num
					} else {
						return Err("First input should be number!".to_string());
					}
				};

				let column = {

					if let Ok(lett) = letter.parse::<char>(){

						if (lett.to_ascii_uppercase() as u8) < 64 { return Err("Second input should be letter".to_string()); }

						(lett.to_ascii_uppercase() as u8 - 64) as i32

					} else { return Err("Second input should be letter".to_string()); }
				};

				if row < 1 || column < 1 || row > 8 || column > 8 { return Err("Input our of bounds".to_string()); }

				return Ok( (row, column) );
			},

			_ => { return Err("Wrong input, enter one number and one letter!".to_string()); }
		};
	}
}