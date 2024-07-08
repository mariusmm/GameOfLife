use std::thread::sleep;
use std::time::Duration;
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn una_iter()
    {
        let mut board1 = Board::new(10, 10);
        let mut board2 = Board::new(10, 10);
        
        board1.ejemplo_planador(5, 5);
    
        for y_idx in 0..board1.height {
            for x_idx in 0..board2.width {
                let alive: bool = board1.apply_rules(x_idx as i32, y_idx as i32);
                    board2.set(x_idx, y_idx, alive);
                }
                
        }
    
        let mut compr = Board::new(10, 10);
        
        
        compr.set(6, 4, true);
        compr.set(5, 5, true);
        compr.set(6, 5, true);
        compr.set(5, 6, true);
        compr.set(7, 6, true);




        let mut compr_b:bool = true;

        for y_idx in 0..board2.height {
            for x_idx in 0..board2.width{
                    compr_b &= board2.get(x_idx, y_idx) == compr.get(x_idx, y_idx);
                }
        }
        board2.print();
        compr.print();
        assert_eq!(compr_b, true);

    }
}


#[derive(Clone)]
struct Cell {
    alive: bool,
}
#[derive(Clone)]
struct Board {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Board {
    fn new(width: usize, height: usize) -> Board {
        Board {
            cells: vec![Cell { alive: false }; width * height],
            width,
            height,
        }
    }

    fn get(&self, x: usize, y: usize) -> bool {
        self.cells[y * self.width + x].alive
    }

    fn set(&mut self, x: usize, y: usize, alive: bool) {
        self.cells[y * self.width + x].alive = alive;
    }

    fn count_neighbors(&self, x: i32, y: i32) -> usize {
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
                if self.get((x + x_idx) as usize, (y + y_idx) as usize) {
                    count += 1;
                }
            }
        }
        count
    }

    fn apply_rules(&self, x: i32, y: i32) -> bool {
        let num_neigh = self.count_neighbors(x, y);
        if self.get(x as usize, y as usize) {
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

    fn print(&self) {
        println!("**************************************************************************");
        for y_idx in 0..self.height {
            for x_idx in 0..self.width {
                    if self.get(x_idx, y_idx) {
                        print!("\u{25A0}");
                        //print!("*");
                    } else {
                        print!("\u{25A1}");
                        //print!(" ");
                    }
                    print!(" ");
                }
                println!();
        }
    }

    fn ejemplo_planador(&mut self, x: i32, y: i32)
    {
        self.set(x as usize, y as usize, true);
        self.set((x+1) as usize, y as usize, true);
        self.set((x+2) as usize, y as usize, true);
        self.set(x as usize, (y+1) as usize, true);
        self.set((x+2) as usize, (y+2) as usize, true);
    }
}

fn main() {
    println!("Hello, world!");

    let mut board1 = Board::new(50, 50);
    let mut board2 = Board::new(50, 50);
    
    board1.ejemplo_planador(30, 30);
    
    board1.print();
  

    loop {
        for y_idx in 0..board1.height {
            for x_idx in 0..board1.width {
                let alive: bool = board1.apply_rules(x_idx as i32, y_idx as i32);
                    board2.set(x_idx, y_idx, alive);
                }
                
        }
    
        board1 = board2.clone();
        sleep(Duration::from_millis(500));
        board1.print();
    }
}

