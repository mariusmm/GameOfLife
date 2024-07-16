use std::env;
use std::time::Instant;
#[warn(non_snake_case)]
use std::io::{self, Write};

mod board;
fn main() {
    const WIDTH: usize = 10;
    const HEIGHT: usize = 10;
    const ITERATIONS: usize = 8;

    let args: Vec<String> = env::args().collect();

    println!("Starting game of life!");
    let mut my_board_a;
    let mut my_board_b;
    let num_iteracions;
    let interactive_mode : bool;

    if args.len() >= 5 {
        let width = args[1].parse::<usize>().unwrap();
        let height = args[2].parse::<usize>().unwrap();
        let iterations = args[3].parse::<usize>().unwrap();
        interactive_mode = args[4].parse::<bool>().unwrap_or(true);
        println!(
            "Width: {} height: {} iterations: {} interactive: {}",
            width, height, iterations, interactive_mode
        );

        my_board_a = board::Board::new(width, height);
        my_board_b = board::Board::new(width, height);
        num_iteracions = iterations;
        // check if interactive mode arg is provided
        /*if args.len() >= 5 {
            interactive_mode = args[4].parse::<bool>().unwrap();
        }*/
    } else if args.len() >= 4 {
        let _width = args[1].parse::<usize>().unwrap();
        let _height = args[2].parse::<usize>().unwrap();
        let _iterations = args[3].parse::<usize>().unwrap();
        interactive_mode = true; //default to true if the fourth arg is not provided
        println!(
            "Width: {} height: {} iterations: {} interactive: {} ",
            WIDTH, HEIGHT, ITERATIONS, interactive_mode
        );
        my_board_a = board::Board::new(WIDTH, HEIGHT);
        my_board_b = board::Board::new(WIDTH, HEIGHT);
        num_iteracions = ITERATIONS;
    } else {
        my_board_a = board::Board::new(WIDTH, HEIGHT);
        my_board_b = board::Board::new(WIDTH, HEIGHT);
        interactive_mode = true; //Default to true
        println!(
            "Width: {} height: {} iterations: {} interactive: {}",
            WIDTH, HEIGHT, ITERATIONS, interactive_mode
        );
        num_iteracions = ITERATIONS;
    }

    //my_board_a.random_init();
    //my_board_a.set_glider(5, 5);

    let pattern = if args.len() > 5 { &args[5] } else { "random" };
    match pattern {
        "glider" => my_board_a.set_glider(5, 5),
        "blinker" => my_board_a.set_blinker(5, 5),
        "block" => my_board_a.set_block(5, 5),
        _ => my_board_a.random_init(),
    }

    //let separator = "--------............--------";
    let mut ab = true;
    let start = Instant::now();

    for _ in 0..num_iteracions {
        if interactive_mode {
            print!("\n");
            print!("\nPress 'n' for next iteration, 'r' to reset, 'q' to quit: ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            if input == "q" {
                break;
            } else if input == "r" {
                my_board_a.random_init();
                println!("Board reset.");
            } else if input != "n" {
                println!("Invalid input. Please enter 'n', 'r', or 'q'.");
                continue;
            }
        } else {
            println!("\n");
        }

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

    /*if ab {
        //my_board_a.print();
    } else {
        //my_board_b.print();
    }*/

    println!("Elapsed time: {:?}", duration);
}
