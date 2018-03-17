
mod figure;
mod board;
mod interface;

use figure::{WHITE_FIGURE, Figure};


fn main() {
    let mut board = board::Board::new();
    let mut turn = WHITE_FIGURE;

    'figure_select: loop {
    	board.draw();
    	print!("{} players turn. Please", turn);
    	match interface::Interface::read_input() {
    		Ok(input_pos) => {
    			if figure::Figure::get_figure_color(board.get_field_content(input_pos)) != turn { 
    				println!("it is not that players turn!");
    				continue; 
    			}

    			'move_figure: loop { 
    				match Figure::move_figure(input_pos, &mut board) {
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
	}

    
}
