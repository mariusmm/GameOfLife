mod board;
mod graphics;

use board::Board;
use graphics::run_app;

fn main() {
    let width = 50;
    let height = 50;
    let cell_size = 10;
    let top_bar_height = 30;

    let mut my_board = Board::new(width, height);

    // Initializing a simple pattern: a glider
    my_board.set(1, 0, true);
    my_board.set(2, 1, true);
    my_board.set(0, 2, true);
    my_board.set(1, 2, true);
    my_board.set(2, 2, true);

    run_app(width, height, cell_size, top_bar_height, my_board);
}
