#![allow(dead_code)]
#![allow(unused_imports)]

use std::io::{self, Write};
use std::result::Result;
use std::str::{SplitWhitespace, FromStr};
use std::thread;

type ValidMovement = Result<(), String>;

pub const WHITE_KING: char = '♔';
pub const WHITE_QUEEN: char = '♕';
pub const WHITE_BISHOP: char = '♗';
pub const WHITE_HUNTER: char = '♘';
pub const WHITE_ROOK: char = '♖';
pub const WHITE_PAWN: char = '♙';

pub const BLACK_KING: char = '♚';
pub const BLACK_QUEEN: char = '♛';
pub const BLACK_BISHOP: char = '♝';
pub const BLACK_HUNTER: char = '♞';
pub const BLACK_ROOK: char = '♜';
pub const BLACK_PAWN: char = '♟';
//add hash map later

pub const BLACK: i32 = 1;
pub const WHITE: i32 = 0;

//           		   0  45  90  135 180 225 270 315
const DIRX: [i32; 8] = [0, -1, -1, -1,  0,  1, 1, 1];
const DIRY: [i32; 8] = [1,  1,  0, -1, -1, -1, 0, 1];

pub struct Vector<T> {
	x: T,
	y: T,
}

pub struct Figure {
	icon: char,
	pos: Vector<i32>,
}

impl Figure {

	pub fn new(icon: char, row: i32, column: i32) -> Figure {
		Figure{
			icon: icon,
			pos: Vector{x: row, y: column}
		}
	}

	pub fn move_figure(&self, turn: i32) {

		loop {
			let _ = io::stdout().flush();
			let mut input = String::new();
			io::stdin()
				.read_line(&mut input)
				.expect("failed to read from standard input 1");

			if let Err(result) = self.valid_movement(&mut input) {
				print!("{}", result);
			} else { break; }

		}

	}

	fn valid_movement(&self, input: &mut str) -> ValidMovement {

		let mut parts = input.split_whitespace();

		match (parts.next(), parts.next()) {

			(Some(number), Some(letter)) =>
			{

				let row = {
					if let Ok(num) = number.parse::<i32>() {
						num
					} else {
						return Err("First input should be number!\n".to_string());
					}
				};

				let column = {

					if let Ok(lett) = letter.parse::<char>(){

						if (lett.to_ascii_uppercase() as u8) < 64 {
							return Err("Second input should be letter\n".to_string());
						}

						lett.to_ascii_uppercase() as u8 - 64
					} else {
						return Err("Second input should be letter\n".to_string());
					}
				};


				println!("{} {}", row, column);

				if row < 1 || column < 1 || row > 8 || column > 8 {
					return Err("Input out of bounds\n".to_string());
				}

				return self.move_queen((row, column as i32));

			},

			_ => { 
				return Err("Wrong input, enter one number and one letter!\n".to_string());
			}
		}

		//Ok(())

	}

	fn move_queen(&self, requested_pos: (i32, i32)) -> ValidMovement{
		Err("queen still can't move\n".to_string())
	}

	fn check_lanes(&self, requested_pos: (i32, i32), starting_angle: usize, angle_increment: usize) -> ValidMovement {
		let num_of_threads = (315 - starting_angle) / angle_increment -1;
		let mut thread_handles: Vec<thread::JoinHandle<_>> = Vec::new();

		for i in 0..num_of_threads {
			thread::spawn(|| {
				Figure::check_lane((self.pos.x, self.pos.y), requested_pos, starting_angle + angle_increment * i, 99);
			});
		} 	

		Ok(())
	}

	fn check_lane(mut pos: (i32, i32), requested_pos: (i32, i32), move_angle: usize, max_moves: i32) -> ValidMovement {
		loop {
			pos.0 += DIRX[move_angle / 45];
			pos.1 += DIRX[move_angle / 45];

			if pos.0 < 1 || pos.1 < 1 || pos.1 > 8 || pos.0 > 8 {
				return Err("".to_string());
			}
			if pos.0 == requested_pos.0 && pos.1 == requested_pos.1 {
				return Ok(());
			}
		}
	}
}