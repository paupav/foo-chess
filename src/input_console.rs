use ValidMovement;
use std::io::{self, Write};
use std::char;

pub fn movement_input() -> ValidMovement {
    let mut input = String::new();
    print!("Enter one number and one letter: ");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut input).expect("failed to read from standard input 1");

    let mut parts = input.split_whitespace();

    match (parts.next(), parts.next()) {
        (Some(number), Some(letter)) =>
        {
            //this is the correct order and users should listen!
            let row = { // ? isn't implemented
                if let Ok(num) = number.parse::<i32>() { num } else { return Err("First input should be number!".to_string()); }
            };

            let column = {
                if let Ok(lett) = letter.parse::<char>(){
                    if (lett.to_ascii_uppercase() as u8) < 64 { return Err("Second input should be letter".to_string()); }
                    (lett.to_ascii_uppercase() as u8 - 64) as i32
                } else { return Err("Second input should be letter".to_string()); }
            };

            if row < 1 || column < 1 || row > 8 || column > 8 { return Err("Input our of bounds".to_string()); } // replace with check bonds
            return Ok( (row, column) );
        },

        _ => { return Err("Wrong input, enter one number and one letter!".to_string()); }
    };
}


pub fn swap_figure_input(color: i32) -> char {
    let starting_figure = (9813 + (-color + 1)/2 * 6) as u32;
    println!("That move allows you to replace figure. Enter number of desired figure:");
    for i in 1..6 {
        println!("{}: {}", i, char::from_u32(starting_figure + i -1).expect("Unable to turn number to char in swap_figure #1"));
    }
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Could not read line in swap figures");
    let trimmed = input.trim();

    match trimmed.parse::<u32>() {
        Ok(selection_number) => {
            if selection_number < 1 || selection_number > 6 { 
                return char::from_u32(starting_figure).expect("Unable to turn number to char in swap_figure #2"); // default to queen
            }
            return char::from_u32(selection_number -1 + starting_figure).expect("Unable to turn number to char in swap_figure #3")
        },
        Err(_dummy) => return char::from_u32(starting_figure).expect("Unable to turn number to char in swap_figure #4"), // default to queen
    }
}
