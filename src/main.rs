
mod figure;
mod board;

fn main() {
    let board = board::Board::new();
    board.draw();
}
