
mod figure;
mod board;
mod interface;

use figure::{WHITE_FIGURE, WHITE_KING, BLACK_KING, Figure};


fn main() {
    let mut board = board::Board::new();
    let mut turn = WHITE_FIGURE;
    let king = (WHITE_KING as i32, BLACK_KING as i32);

    'figure_select: loop {
    	board.draw();

        let king = std::char::from_u32(((king.0+king.1)/2+turn*(king.0-(king.0+king.1)/2))as u32).expect("Unknown char");
    	print!("King {}'s turn.", king);
        
    	match interface::Interface::read_input() {
    		Ok(input_pos) => {
    			if figure::Figure::get_figure_color(board.get_field_content(input_pos)) != turn { 
    				println!("Can not move other players figures!");
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
