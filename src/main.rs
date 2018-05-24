extern crate ncurses;

mod figure;
mod board;
mod input_console;

use figure::{WHITE_FIGURE, WHITE_KING, BLACK_KING, ValidMovement};
use input_console::{movement_input, swap_figure_input};

fn main() {
    let mut board = board::Board::new();
    let mut turn = WHITE_FIGURE;
    let king = (WHITE_KING as i32, BLACK_KING as i32);



    'figure_select: loop {
    	board.draw();
        //if 1 == 1 { break; } // to avoid warnings
        let king = std::char::from_u32(((king.0+king.1)/2+turn*(king.0-(king.0+king.1)/2))as u32).expect("Unknown char");
    	println!("Turn: {}", king);
        
    	match movement_input() {
    		Ok(input_pos) => {
    			if figure::get_figure_color(board.get_field_content(input_pos)) != turn { 
    				println!("Can not move other players figures!");
    				continue; 
    			}

    			'move_figure: loop {     
    				match figure::move_figure(&mut board, &movement_input, &swap_figure_input, input_pos) {
    					Ok(pos) => { 
    						println!("Figure moved to: {} {}", pos.0, pos.1); 
    						turn = -turn;
    					}
    					Err(msg) => { println!("Figure didn't move: {}", msg); }
   					}
   					break;
   				}
   				//break; // break when checkmate
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
