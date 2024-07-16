use std::env;
use std::time::Instant;
use std::io::{self, Write};

#[warn(non_snake_case)]
#[warn(non_upper_case_globals)]
#[warn(dead_code)]

const colors: [&str; 9] = ["🟥", "🟧", "🟨", "🟩", "🟦", "🟪", "⬛", "⬜", "🟫"];

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
    
    let chose_color = true;
    let interactive_mode = prompt_for_interactive_mode();

    if chose_color{
        choose_colors();
    }

    if args.len() >= 5 {
        let width = args[1].parse::<usize>().unwrap();
        let height = args[2].parse::<usize>().unwrap();
        let iterations = args[3].parse::<usize>().unwrap();
        //interactive_mode = args[4].parse::<bool>().unwrap_or(true);
        println!(
            "Width: {} height: {} iterations: {} interactive: {}",
            width, height, iterations, interactive_mode
        );

        my_board_a = board::Board::new(width, height);
        my_board_b = board::Board::new(width, height);
        num_iteracions = iterations;
    } else {
        my_board_a = board::Board::new(WIDTH, HEIGHT);
        my_board_b = board::Board::new(WIDTH, HEIGHT);
        //interactive_mode = true; //Default to true
        println!(
            "Width: {} height: {} iterations: {} interactive: {}",
            WIDTH, HEIGHT, ITERATIONS, interactive_mode
        );
        num_iteracions = ITERATIONS;
    }


    let pattern = if args.len() > 5 { &args[5] } else { "random" };
    match pattern {
        "glider" => my_board_a.set_glider(5, 5),
        "blinker" => my_board_a.set_blinker(5, 5),
        "block" => my_board_a.set_block(5, 5),
        _ => my_board_a.random_init(),
    }

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

    println!("Elapsed time: {:?}", duration);
}


fn choose_colors() {
    println!("Choose the colors for Alive and Dead:");

    
    for (i, &color) in colors.iter().enumerate() {
        println!("{}. {}", i + 1, color);
    }

    let alive_color = get_color_choice("Alive");
    let dead_color = get_color_choice("Dead");

    board::set_colors(alive_color, dead_color);
}

fn get_color_choice(name: &str) -> String {
    loop {
        print!("Enter the number for {} color: ", name);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<usize>() {
            Ok(num) if num >= 1 && num <= 9 => return colors[num - 1].to_string(),
            _ => println!("Invalid input, please enter a number between 1 and 9."),
        }
    }
}

fn prompt_for_interactive_mode() -> bool {
    loop {
        print!("Do you want to interact with it? (y/n): ");
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "n" => return false, // No means interactive_mode is false
            "y" => return true,  // Yes means interactive_mode is true
            _ => println!("Invalid input, please enter 'y' or 'n'."),
        }
    }
}