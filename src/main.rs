use std::env;
use std::time::Instant;
mod board;

fn main() {
    const WIDTH: usize = 50;
    const HEIGHT: usize = 50;
    const ITERATIONS: usize = 100;

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
        if ab {
            my_board_a.evolve(&mut my_board_b);
        } else {
            my_board_b.evolve(&mut my_board_a);
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
