mod board;

const WIDTH: usize = 50;
const HEIGHT: usize = 50;
fn main() {
    let mut my_board = board::Board::new(WIDTH, HEIGHT);
    my_board.init_borad("random");
    //let counter = 0;
    for counter in 0..100 {
        my_board.print();
        my_board.update();
    }
}
