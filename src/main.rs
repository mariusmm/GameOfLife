use std::{env, thread};
use std::sync::{Arc, Mutex}; 
use std::time::Instant;

mod board;

fn main() {
    const WIDTH: usize = 50; 
    const HEIGHT: usize = 50; 
    const ITERATIONS: usize = 1000;
    const FORM: &str = "glider"; 
    const NUM_THREADS: usize = 4; 

    let args: Vec<String> = env::args().collect(); 

    println!("Starting game of life!"); 
    let mut my_board_a; 
    let my_board_b; 
    let num_iterations; 
    let initial_form: &str; 

    if args.len() >= 5 {
        let width: usize = args[1].parse::<usize>().unwrap(); 
        let height: usize = args[2].parse::<usize>().unwrap(); 
        let iterations: usize = args[3].parse::<usize>().unwrap(); 
        let form: &str = &args[4]; 

        println!(
            "Width: {} height: {} iterations: {} form {} ",
            width, height, iterations, form
        ); 
        my_board_a = board::Board::new(width, height); 
        my_board_b = board::Board::new(width, height); 
        num_iterations = iterations; 
        initial_form = form; 
    } else {
        my_board_a = board::Board::new(WIDTH, HEIGHT); 
        my_board_b = board::Board::new(WIDTH, HEIGHT); 
        num_iterations = ITERATIONS; 
        initial_form = FORM; 
        println!(
            "Width: {} height: {} iterations: {} form: {}",
            WIDTH, HEIGHT, ITERATIONS, FORM
        );
    }

    match initial_form {
        "block" => my_board_a.set_block(5, 5),
        "beehive" => my_board_a.set_beehive(5, 5),
        "loaf" => my_board_a.set_loaf(5, 5),
        "blinker" => my_board_a.set_blinker(5, 5),
        "toad" => my_board_a.set_toad(5, 5),
        "beacon" => my_board_a.set_beacon(5, 5),
        "glider" => my_board_a.set_glider(5, 5),
        _ => println!("Unknown form"),
    }

    let mut ab: bool = true; 

    let start = Instant::now(); 

    let my_board_a = Arc::new(Mutex::new(my_board_a)); 
    let my_board_b = Arc::new(Mutex::new(my_board_b)); 

    for _ in 0..num_iterations { 
        let handles: Vec<_> = (0..NUM_THREADS).map(|thread_index| { 
            let my_board_a = Arc::clone(&my_board_a); 
            let my_board_b = Arc::clone(&my_board_b); 
            thread::spawn(move || { 
                let (start_row, end_row) = divide_work(thread_index, NUM_THREADS, WIDTH); 
                if ab { 
                    let board_a = my_board_a.lock().unwrap(); 
                    let mut board_b = my_board_b.lock().unwrap();
                    for x_idx in start_row..end_row { 
                        for y_idx in 0..board_a.get_height() { 
                            let alive = board_a.apply_rules(x_idx as i32, y_idx as i32); 
                            board_b.set(x_idx, y_idx, alive); 
                        }
                    }
                } else {
                    let board_b = my_board_b.lock().unwrap(); 
                    let mut board_a = my_board_a.lock().unwrap(); 
                    for x_idx in start_row..end_row { 
                        for y_idx in 0..board_b.get_height() { 
                            let alive = board_b.apply_rules(x_idx as i32, y_idx as i32); 
                            board_a.set(x_idx, y_idx, alive);
                        }
                    }
                }
            })
        }).collect(); 

        for handle in handles { 
            handle.join().unwrap();
        }

        if ab { 
            let board_b = my_board_b.lock().unwrap(); 
            board_b.print(); 
        } else {
            let board_a = my_board_a.lock().unwrap(); 
            board_a.print(); 
        }

        ab = !ab; 
    }

    let duration = start.elapsed(); 
    println!("Elapsed time: {:?}", duration); 
}

fn divide_work(thread_index: usize, num_threads: usize, width: usize) -> (usize, usize) {
    let rows_per_thread = width / num_threads; 
    let start_row = thread_index * rows_per_thread; 
    let mut end_row = start_row + rows_per_thread; 
    if thread_index == num_threads - 1 { 
        end_row = width;
    }
    (start_row, end_row)
}
