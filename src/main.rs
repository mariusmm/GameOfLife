mod board;

use board::{Board, CellState};
use std::cmp::PartialEq;
use rand::prelude::*;
use std::{thread, time::Duration};

fn main() {
    const WIDTH: usize = 10;
    const HEIGHT: usize = 10;
    let mut board = Board::new(WIDTH, HEIGHT);
    let mut rng = thread_rng();
    // board.set(rng.gen_range(0..WIDTH), rng.gen_range(0..HEIGHT), CellState::Alive);
    for _ in 0..(WIDTH * HEIGHT / 4) {
        board.set(rng.gen_range(0..WIDTH), rng.gen_range(0..HEIGHT), CellState::Alive);
    }


    loop {
        println!("Current Board:");
        board.print_board();

        let mut new_board = board.clone();
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                new_board.set(x, y, board.apply_rules(x as i32, y as i32));
            }
        }
        board = new_board;

        thread::sleep(Duration::from_millis(500));
        println!("\x1B[2J\x1B[1;1H");
    }
}
