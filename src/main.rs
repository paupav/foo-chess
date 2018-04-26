extern crate ncurses;

mod figure;
mod board;

use figure::{WHITE_FIGURE, WHITE_KING, BLACK_KING, Figure, ValidMovement};
use std::io::{self, Write};
use std::ascii::AsciiExt;
use ncurses::*;

fn read_input() -> ValidMovement {
    let mut input = String::new();
    print!(" Enter one number and one letter: ");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut input).expect("failed to read from standard input 1");

    let mut parts = input.split_whitespace();

    match (parts.next(), parts.next()) {
        (Some(number), Some(letter)) =>
        {
            let row = { // ? isn't implemented
                if let Ok(num) = number.parse::<i32>() { num } else { return Err("First input should be number!".to_string()); }
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


fn main() {
    let mut board = board::Board::new();
    let mut turn = WHITE_FIGURE;
    let king = (WHITE_KING as i32, BLACK_KING as i32);



    'figure_select: loop {
    	board.draw();
        //if 1 == 1 { break; } // to avoid warnings
        let king = std::char::from_u32(((king.0+king.1)/2+turn*(king.0-(king.0+king.1)/2))as u32).expect("Unknown char");
    	print!("King {}'s turn.", king);
        
    	match read_input() {
    		Ok(input_pos) => {
    			if figure::Figure::get_figure_color(board.get_field_content(input_pos)) != turn { 
    				println!("Can not move other players figures!");
    				continue; 
    			}

    			'move_figure: loop {     
    				match Figure::move_figure(&mut board, &read_input, input_pos) {
    					Ok(pos) => { 
    						println!("Figure moved to: {} {}", pos.0, pos.1); 
    						turn = -turn;
    					}
    					Err(msg) => { println!("Figure didn't move: {}", msg); }
   					}
   					break; // tmp break
   				}
   				//break; // break when king is eaten
    		}
    		Err(msg) => { println!("{}", msg); }
    	}
	} //EOF

    /*initscr();

   // mv(max_y - 10, 10);

    attron(A_BOLD() | A_BLINK());
    printw("Hello!");
    attroff(A_BOLD() | A_BLINK());

    getch();
    endwin();*/

    
}
