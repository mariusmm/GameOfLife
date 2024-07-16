use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;
mod board;

fn main() {
    const WIDTH: usize = 50;
    const HEIGHT: usize = 50;
    const ITERATIONS: usize = 100;
    const THREADS: usize = 4; // Número de hilos a usar

    let args: Vec<String> = env::args().collect();

    println!("Starting game of life!");
    let mut my_board_a;
    let mut my_board_b;
    let num_iterations;

    if args.len() >= 4 {
        let width = args[1].parse::<usize>().unwrap();
        let height = args[2].parse::<usize>().unwrap();
        let iterations = args[3].parse::<usize>().unwrap();
        println!(
            "Width: {} height: {} iterations: {} ",
            width, height, iterations
        );
        my_board_a = board::Board::new(width, height);
        my_board_b = board::Board::new(width, height);
        num_iterations = iterations;
    } else {
        my_board_a = board::Board::new(WIDTH, HEIGHT);
        my_board_b = board::Board::new(WIDTH, HEIGHT);
        num_iterations = ITERATIONS;
        println!(
            "Width: {} height: {} iterations: {} ",
            WIDTH, HEIGHT, ITERATIONS
        );
    }

    my_board_a.random_init();
    my_board_a.set_glider(5, 5);

    let mut ab = true;

    let start = Instant::now();

    for _ in 0..num_iterations {
        let (current_board, next_board) = if ab {
            (&my_board_a, &mut my_board_b)
        } else {
            (&my_board_b, &mut my_board_a)
        };

        let current_board = Arc::new(current_board.clone());
        let next_board = Arc::new(Mutex::new(next_board.clone()));

        let mut handles = vec![];

        for t in 0..THREADS {
            let current_board = Arc::clone(&current_board);
            let next_board = Arc::clone(&next_board);
            let handle = thread::spawn(move || {
                let height = current_board.get_height();
                let rows_per_thread = height / THREADS;
                let start_row = t * rows_per_thread;
                let end_row = if t == THREADS - 1 {
                    height
                } else {
                    start_row + rows_per_thread
                };
                for y in start_row..end_row {
                    for x in 0..current_board.get_width() {
                        let alive = current_board.apply_rules(x as i32, y as i32);
                        let mut next_board = next_board.lock().unwrap();
                        next_board.set(x, y, alive);
                    }
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        ab = !ab;
    }

    let duration = start.elapsed();

    if ab {
        my_board_a.print();
    } else {
        my_board_b.print();
    }

    println!("Elapsed time: {:?}", duration);
}
