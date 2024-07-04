use std::time::Instant;

mod board;
fn main() {
    const WIDTH: usize = 1000;
    const HEIGHT: usize = 1000;

    println!("Starting game of life!");

    let mut my_board_a = board::Board::new(WIDTH, HEIGHT);
    let mut my_board_b = board::Board::new(WIDTH, HEIGHT);

    my_board_a.random_init();
    my_board_a.set_glider(5, 5);

    let num_iteracions = 10;

    let mut ab = true;

    let start = Instant::now();

    for _ in 1..num_iteracions {
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
        my_board_a.print();
    } else {
        my_board_b.print();
    }

    println!("Elapsed time: {:?}", duration);
}
