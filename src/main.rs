

mod board;
mod application;
use board::*;


fn main() {
    println!("Hello, world!");
    let width = 20;
    let height = 20;

    let mut my_board = Board::new(width, height);

    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Hello, World")
        .build();
 

    while !rl.window_should_close() {
        let app_state = application::run_update(&mut my_board, &mut rl);
        application::run_render(app_state, &my_board, &mut rl, &thread);
    }
}