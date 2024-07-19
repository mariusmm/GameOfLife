use std::env;
use std::time::Instant;
use std::thread;
use std::sync::{Arc, Mutex};

mod board;

fn main() {
    const WIDTH: usize = 20;
    const HEIGHT: usize = 20;
    const ITERATIONS: usize = 100;

    let args: Vec<String> = env::args().collect();

    println!("Starting game of life!");
    let (width, height, num_iterations) = if args.len() >= 4 {
        let w = args[1].parse::<usize>().unwrap();
        let h = args[2].parse::<usize>().unwrap();
        let i = args[3].parse::<usize>().unwrap();
        println!("Width: {} height: {} iterations: {}", w, h, i);
        (w, h, i)
    } else {
        println!("Width: {} height: {} iterations: {}", WIDTH, HEIGHT, ITERATIONS);
        (WIDTH, HEIGHT, ITERATIONS)
    };

    let board_a = Arc::new(Mutex::new(board::Board::new(width, height)));
    let board_b = Arc::new(Mutex::new(board::Board::new(width, height)));

    board_a.lock().unwrap().random_init();
    board_a.lock().unwrap().set_glider(5, 5);

    let start = Instant::now();

    let num_threads = thread::available_parallelism().map(|n| n.get()).unwrap_or(4);
    println!("Number of threads: {}", num_threads);

    for i in 0..num_iterations {
        let (src_board, dst_board) = if i % 2 == 0 {
            (Arc::clone(&board_a), Arc::clone(&board_b))
        } else {
            (Arc::clone(&board_b), Arc::clone(&board_a))
        };

        let chunk_size = (height + num_threads - 1) / num_threads;

        let handles: Vec<_> = (0..num_threads)
            .map(|t| {
                let src = Arc::clone(&src_board);
                let dst = Arc::clone(&dst_board);
                thread::spawn(move || {
                    let start_y = t * chunk_size;
                    let end_y = (start_y + chunk_size).min(height);
                    let src_guard = src.lock().unwrap();
                    let mut dst_guard = dst.lock().unwrap();

                    for y in start_y..end_y {
                        for x in 0..width {
                            let alive = src_guard.apply_rules(x as i32, y as i32);
                            dst_guard.set(x, y, alive);
                        }
                    }
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }

        // dst_board.lock().unwrap().print();
    }

    let duration = start.elapsed();

    if num_iterations % 2 == 0 {
        //board_a.lock().unwrap().print();
    } else {
        //board_b.lock().unwrap().print();
    }

    println!("Elapsed time: {:?}", duration);
}