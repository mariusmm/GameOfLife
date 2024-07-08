use std::env;
use std::time::Instant;

mod board;
fn main() {
    const WIDTH: usize = 1000;
    const HEIGHT: usize = 1000;
    const ITERATIONS: usize = 100;

    let args: Vec<String> = env::args().collect();

    println!("Starting game of life!");
    let mut my_board_a;
    let mut my_board_b;
    let num_iteracions;

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
        num_iteracions = iterations;
    } else {
        my_board_a = board::Board::new(WIDTH, HEIGHT);
        my_board_b = board::Board::new(WIDTH, HEIGHT);
        num_iteracions = ITERATIONS;
        println!(
            "Width: {} height: {} iterations: {} ",
            WIDTH, HEIGHT, ITERATIONS
        );
    }

    my_board_a.random_init();
    my_board_a.set_glider(5, 5);

    let mut ab = true;

    let start = Instant::now();

    for _ in 0..num_iteracions {
        if ab {
            for x_idx in 0..my_board_a.get_width() {
                for y_idx in 0..my_board_a.get_height() {
                    let alive = my_board_a.apply_rules(x_idx as i32, y_idx as i32);
                    my_board_b.set(x_idx, y_idx, alive);
                }
            }
            //my_board_b.print();
        } else {
            for x_idx in 0..my_board_a.get_width() {
                for y_idx in 0..my_board_a.get_height() {
                    let alive = my_board_b.apply_rules(x_idx as i32, y_idx as i32);
                    my_board_a.set(x_idx, y_idx, alive);
                }
            }
            //my_board_a.print();
        }
        ab = !ab;
    }

    let duration = start.elapsed();

    if ab {
        //my_board_a.print();
    } else {
        //my_board_b.print();
    }

    println!("Elapsed time: {:?}", duration);
}
