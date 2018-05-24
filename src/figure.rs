use std::result::Result;
use board::Board;

pub type ValidMovement = Result<(i32, i32), String>;

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

pub const WHITE_FIGURE: i32 = 1; 
pub const BLACK_FIGURE: i32 = -WHITE_FIGURE; // black figure is negative WHITE_FIGURE, dependency in main
//           		   0  45  90  135 180 225 270 315
const DIRX: [i32; 8] = [0, -1, -1, -1,  0,  1, 1, 1];
const DIRY: [i32; 8] = [1,  1,  0, -1, -1, -1, 0, 1];


pub fn get_figure_color(icon: char) -> i32 {
	if 9812 <= icon as usize && icon as usize <= 9817{ WHITE_FIGURE }
	else if 9818 <= icon as usize && icon as usize <= 9823{ BLACK_FIGURE }
	else { 0 }
}

pub fn move_figure(board: &mut Board, movement_input: &Fn() -> ValidMovement, swap_figure_input: &Fn(i32) -> char, current_pos: (i32, i32)) -> ValidMovement {

	let icon = board.get_field_content(current_pos);
	println!("Selected: {}", icon);

	let requested_pos = movement_input()?;
	match icon {
		WHITE_QUEEN | BLACK_QUEEN => {
			check_lanes(&(current_pos, requested_pos), 0, 45, 99, &board)?;
		},
		WHITE_KING | BLACK_KING => {
			let color2rev = ((-get_figure_color(icon) +1)/2) as usize;
			//panic!("Should nto castle when fields are under attack, check not implemented!");
			//castling, this works for both colors because of the way castling_rook_flags work
			if requested_pos.0 == 1 + 7*color2rev as i32 && ((requested_pos.1 == 3 && board.castling_rook_flags[color2rev][0]) || 	
												  		 (requested_pos.1 == 7 && board.castling_rook_flags[color2rev][1])) {
				//moving rook
				board.move_char( (requested_pos.0, (requested_pos.1 / 4) * 7 + 1), 
								 (requested_pos.0, requested_pos.1 - ((requested_pos.1 / 5) * 2 -1)) );

			} else { check_lanes(&(current_pos, requested_pos), 0, 45, 1, &board)?; }	

			board.castling_rook_flags[color2rev][0] = false;
			board.castling_rook_flags[color2rev][1] = false;

		},
		WHITE_BISHOP | BLACK_BISHOP => {
			check_lanes(&(current_pos, requested_pos), 45, 45, 99, &board)?;
		},
		WHITE_ROOK | BLACK_ROOK => {
			check_lanes(&(current_pos, requested_pos), 0, 90, 99, &board)?;	

			if current_pos.1 == 1 || current_pos.1 == 8 { // castling is not allowed with rook that moved
				board.castling_rook_flags[(icon as usize / 9820) as usize][(current_pos.1 / 8) as usize] = false;
			}
		},
		WHITE_HUNTER | BLACK_HUNTER => {
			if(current_pos.1 - 2 == requested_pos.1 && (current_pos.0 - 1 == requested_pos.0 || current_pos.0 + 1 == requested_pos.0)) ||
			  (current_pos.1 - 1 == requested_pos.1 && (current_pos.0 - 2 == requested_pos.0 || current_pos.0 + 2 == requested_pos.0)) ||
			  (current_pos.1 + 1 == requested_pos.1 && (current_pos.0 - 2 == requested_pos.0 || current_pos.0 + 2 == requested_pos.0)) ||
			  (current_pos.1 + 2 == requested_pos.1 && (current_pos.0 - 1 == requested_pos.0 || current_pos.0 + 1 == requested_pos.0)) {

				if get_figure_color(icon) == get_figure_color(board.get_field_content(requested_pos)) {
					return Err("this figure can not sacrafice own figures!".to_string());
				}

				Board::check_bounds(requested_pos)?;
			}
		},
		WHITE_PAWN | BLACK_PAWN => {
			let max_moves = {
				if current_pos.0 == 2 && icon == WHITE_PAWN { (2 as i32) }
				else if current_pos.0 == 7 && icon == BLACK_PAWN { (2 as i32) }
				else { (1 as i32) }
			};
			let move_angle: i32 = { 
				if icon == WHITE_PAWN { 270 } else { 90 } 
			};
			//println!("Max moves: {}", max_moves);
			let can_move = {

				if current_pos.1 == requested_pos.1 { //pawn moving
					if !board.field_empty(requested_pos) { return Err("pawn can only eat other figures diagonally!".to_string()); }
					check_lane(current_pos, &requested_pos, move_angle as usize, max_moves, &board)? 
				}
				else { //pawn eating
					check_lane(current_pos, &requested_pos, (move_angle - 45 * (current_pos.1 - requested_pos.1)) as usize, 1, &board)? // todo
				}
			};
			
			if !can_move { return Err("can not move there!".to_string()); }

			//when pawn gets to the other side he can be replaced with another figure
			if (icon == WHITE_PAWN && requested_pos.0 == 8) || (icon == BLACK_PAWN && requested_pos.0 == 1) {
				let new_icon = swap_figure_input(get_figure_color(icon));
				println!("{}", new_icon);
				board.set_field_content(&current_pos, new_icon);
			}
		},
		_ => { panic!("moving unknown character!") }

	};


	board.move_char(current_pos, requested_pos); // it must be success because errors will handle all the other scenarios 
	Ok((requested_pos.0, requested_pos.1))
}


//helper functions, TODO: rewrite
fn check_lanes(pos: &((i32, i32), (i32, i32)), starting_angle: usize, angle_increment: usize, max_moves: i32, board: &Board) -> Result<(), String> {

	for i in 0..(360 / angle_increment) {
		//println!("i == {}", i);
		if check_lane(pos.0, &pos.1, starting_angle + angle_increment * i, max_moves, board)? { return Ok(()) }
	}

	Err("path was not found".to_string())
}

fn check_lane(mut pos: (i32, i32), requested_pos: &(i32, i32), move_angle: usize, mut max_moves: i32, board: &Board) -> Result<bool, String> {

	//println!("Checking at angle: {}", move_angle);
	let mut path_blocked = false;
	let color1 = get_figure_color(board.get_field_content(pos));
	loop {
		//println!("max moves: {}", max_moves);
		pos.0 += DIRX[move_angle / 45];
		pos.1 += DIRY[move_angle / 45];
		//println!("after change: {} {}", pos.0, pos.1);

		if Board::check_bounds(pos).is_err() { return Ok(false); }
		if pos.0 == requested_pos.0 && pos.1 == requested_pos.1 {
			if path_blocked { return Err("this figure can't skip over other figures!".to_string()); }
			if get_figure_color(board.get_field_content(pos)) == color1 { return Err("this figure can't sacrafice own figures!".to_string()) }
			if max_moves <= 0 { return Err("this figure can't move that far!".to_string()); }

			return Ok(true);
		}

		if !board.field_empty(pos) { path_blocked = true; }

		max_moves -= 1;
	}
}

fn check_chess() {

}
