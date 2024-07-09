use std::env;
use std::time::Instant;

mod board;

fn main() {
    const WIDTH: usize = 50;
    const HEIGHT: usize = 50;
    const ITERATIONS: usize = 1000;

    let args: Vec<String> = env::args().collect();

    println!("Starting game of life!");
    let mut my_board_a;
    let mut my_board_b;
    let num_iteracions;
    let initial_form: &str;

    if args.len() >= 5 {
        let width = args[1].parse::<usize>().unwrap();
        let height = args[2].parse::<usize>().unwrap();
        let iterations = args[3].parse::<usize>().unwrap();
        let form: &str = &args[4];

        println!(
            "Width: {} height: {} iterations: {} ",
            width, height, iterations
        );
        my_board_a = board::Board::new(width, height);
        my_board_b = board::Board::new(width, height);
        num_iteracions = iterations;
        initial_form = form;
    } else {
        my_board_a = board::Board::new(WIDTH, HEIGHT);
        my_board_b = board::Board::new(WIDTH, HEIGHT);
        num_iteracions = ITERATIONS;
        initial_form = "glider";
        println!(
            "Width: {} height: {} iterations: {} ",
            WIDTH, HEIGHT, ITERATIONS
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

    for _ in 0..num_iteracions {
        if ab {
            for x_idx in 0..my_board_a.get_width() {
                for y_idx in 0..my_board_a.get_height() {
                    let alive = my_board_a.apply_rules(x_idx as i32, y_idx as i32);
                    my_board_b.set(x_idx, y_idx, alive);
                }
            }
            my_board_b.print();
        } else {
            for x_idx in 0..my_board_a.get_width() {
                for y_idx in 0..my_board_a.get_height() {
                    let alive = my_board_b.apply_rules(x_idx as i32, y_idx as i32);
                    my_board_a.set(x_idx, y_idx, alive);
                }
            }
            my_board_a.print();
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
