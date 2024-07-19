use std::env;
use std::time::Instant;
use std::sync::{Arc, Mutex};
mod board;
fn main() {
    const WIDTH: usize = 20;
    const HEIGHT: usize = 20;
    const ITERATIONS: usize = 100;

    let args: Vec<String> = env::args().collect();

    println!("Starting game of life!");
    let my_board_a;
    let my_board_b;
    let num_iteracions;

    if args.len() >= 4 {
        let width = args[1].parse::<usize>().unwrap();
        let height = args[2].parse::<usize>().unwrap();
        let iterations = args[3].parse::<usize>().unwrap();
        println!(
            "Width: {} height: {} iterations: {} ",
            width, height, iterations
        );
        my_board_a = Arc::new(Mutex::new(board::Board::new(width, height)));
        my_board_b = Arc::new(Mutex::new(board::Board::new(width, height)));
        num_iteracions = iterations;
    } else {
        my_board_a = Arc::new(Mutex::new(board::Board::new(WIDTH, HEIGHT)));
        my_board_b = Arc::new(Mutex::new(board::Board::new(WIDTH, HEIGHT)));
        num_iteracions = ITERATIONS;
        println!(
            "Width: {} height: {} iterations: {} ",
            WIDTH, HEIGHT, ITERATIONS
        );
    }
    {
        let mut board_a = my_board_a.lock().unwrap();
        board_a.random_init();
        board_a.set_glider(5, 5);
    }

    let mut ab = true;

    let start = Instant::now();

    for _ in 0..num_iteracions {
        if ab {
            let board_a = my_board_a.lock().unwrap();
            let mut board_b = my_board_b.lock().unwrap();
            for x_idx in 0..board_a.get_width() {
                for y_idx in 0..board_a.get_height() {
                    let alive = board_a.apply_rules(x_idx as i32, y_idx as i32);
                    board_b.set(x_idx, y_idx, alive);
                }
            }
            board_b.print();
            println!(" ");
            println!(" ");
        } else {
            let mut board_a = my_board_a.lock().unwrap();
            let board_b = my_board_b.lock().unwrap();
            for x_idx in 0..board_b.get_width() {
                for y_idx in 0..board_b.get_height() {
                    let alive = board_b.apply_rules(x_idx as i32, y_idx as i32);
                    board_a.set(x_idx, y_idx, alive);
                }
            }
            board_a.print();
            println!(" ");
            println!(" ");
        }
        ab = !ab;
    }

    let duration = start.elapsed();


    println!("Elapsed time: {:?}", duration);
}
