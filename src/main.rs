mod board;
use std::{sync::{Arc, Mutex, RwLock},thread, vec};

const WIDTH: usize = 50;
const HEIGHT: usize = 50;
fn main() {
    let mut my_board = board::Board::new(WIDTH, HEIGHT);
    let mut my_second_board = board::Board::new(WIDTH, HEIGHT);
    my_board.init_borad("random");
    
    let my_board_mutex = Arc::new(Mutex::new(my_board));
    for count in 0..100 {
        if count%2==0{
            
            let mut vec_rules = vec![];
            for x in 0..HEIGHT{
                for y in 0..WIDTH{
                    vec_rules.push(my_board.apply_rules(x as i32, y as i32));
                }
            }
            let vec_rules_mutex = Arc::new(RwLock::new(&vec_rules));
            let mut handles = vec![];
            for x in 0..HEIGHT{
                for y in 0..WIDTH{
                    let inner_mutex = Arc::clone(&my_board_mutex);
                    let handle = thread::spawn( move || {
                        let mut new_board = inner_mutex.lock().unwrap();
                        my_second_board.set(x, y, my_board_mutex.read().unwrap().apply_rules(x, y));
                    });
                    handles.push(handle);
                }
                for handle in handles{
                    handle.join().unwrap();
                }
            }

            my_second_board.print();
            
        } else {
            my_second_board.print();
        }



    }
}
