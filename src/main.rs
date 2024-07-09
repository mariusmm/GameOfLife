use std::thread;
use std::time::Duration;
use std::thread::sleep;
use std::sync::{Arc,Mutex};

#[derive(Clone)]
struct Cell {
    alive: bool,
}


struct Board {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Board {
    fn new(width: usize, height: usize) -> Board {
        Board {
            cells: vec![Cell { alive: false }; (width * height) as usize],
            width,
            height,
        }
    }


    fn get_cells(&self, x: usize, y: usize) -> bool {
        self.cells[y * self.width + x].alive
    }

    fn get_height(&self) -> usize {
        self.height
    }

    fn get_width(&self) -> usize{
        self.width
    }

    fn set(&mut self, x: usize, y: usize, alive: bool) {
        self.cells[y * self.width + x].alive = alive;
    }

    fn count_neighbors(&self, x: i32, y: i32) -> i32 {
        let mut count = 0;

        for x_idx in [-1, 0, 1] {
            for y_idx in [-1, 0, 1] {
                if x + x_idx < 0
                    || x + x_idx >= self.width as i32
                    || y + y_idx < 0
                    || y + y_idx >= self.height as i32
                    || x_idx == 0 && y_idx == 0
                {
                    continue;
                }
                if self.get_cells((x + x_idx) as usize, (y + y_idx) as usize) {
                    count += 1;
                }
            }
        }
        count
    }

    fn apply_rules(&self, x: i32, y: i32) -> bool {
        let num_neigh = self.count_neighbors(x, y);
        if self.get_cells(x as usize, y as usize) {
            match num_neigh {
                0 => false,
                1 => false,
                2 => true,
                3 => true,
                _ => false,
            }
        } else {
            num_neigh == 3
        }
    }


    fn init(&mut self, target_list:Vec<(usize,usize)>) {
        for i in target_list{
            self.set(i.0,i.1,true);
        }
    }

    fn print(&self) {
        println!("**************************************************************************");
        for y_idx in 0..self.height {
        for x_idx in 0..self.width {
                if self.get_cells(x_idx, y_idx) {
                    //print!("\u{25A0}");
                    print!("■ ");
                } else {
                    //print!("\u{25A1}");
                    print!("□ ");
                }
            }
            println!();
        }
    }
}

fn main() {

    let n_size:usize = 1000;
    let m_size:usize= 1000;

    let mut my_board =Board::new(m_size, n_size);
    let swap_board = Arc::new(Mutex::new(Board::new(m_size, n_size)));
    let n:usize= my_board.get_height();
    let m:usize = my_board.get_width();
    //Aqui estaria be treballar amb apuntadors, pero com que Rust ho determina unsafe, de momento ho evitare
    let mut target_list:Vec<(usize, usize)>= Vec::new();
    target_list.push((1,2));
    target_list.push((2,3));
    target_list.push((3,1));
    target_list.push((3,2));
    target_list.push((3,3));

    my_board.init(target_list);
    
    let my_board =Arc::new(Mutex::new(my_board));
    loop{
        let mut handles = vec![];
        for i in 0..m{
            let inner_my_board = Arc::clone(&my_board);
            let inner_swap_board = Arc::clone(&swap_board);
            let handle = thread::spawn(move || {
                for j in 0..n{
                    inner_swap_board.lock().unwrap().set(i, j, inner_my_board.lock().unwrap().apply_rules(i as i32, j as i32));
                }
            });
            handles.push(handle)
        }
        for handle in handles{
            handle.join().unwrap();
        }

        let mut handles = vec![];
        for i in 0..m{
            let inner_my_board = Arc::clone(&my_board);
            let inner_swap_board = Arc::clone(&swap_board);
            let handle = thread::spawn(move || {
                for j in 0..n{
                    inner_my_board.lock().unwrap().set(i,j,inner_swap_board.lock().unwrap().get_cells(i, j))
                }
            });
            handles.push(handle)
        }
        for handle in handles{
            handle.join().unwrap();
        }
        my_board.lock().unwrap().print();
    }
        
        
        
        
        
        
        
        
        
        

}



#[cfg(test)]
mod test_game_of_life{
    use super::*;

    #[test]
    fn init_test(){
        let mut test_board: Board = Board::new(10,10);
        let mut temp_vector:Vec<(usize,usize)> = Vec::new();
        temp_vector.push((1,1));

        assert_eq!(test_board.get_cells(1,1),false);
        test_board.init(temp_vector);
        assert_eq!(test_board.get_cells(1,1),true);
    }
}
