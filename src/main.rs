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

        for x_idx in -1..1 {
            for y_idx in -1..1 {
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
            if num_neigh == 3 {
                true
            } else {
                false
            }
        }
    }

    fn print(&self) {
        for x_idx in 0..self.width {
            for y_idx in 0..self.height {
                if self.get(x_idx, y_idx) {
                    print!("\u{25A0}");
                } else {
                    print!("\u{25A1}");
                }
            }
            println!();
        }
    }
}

fn main() {
    println!("Hello, world!");

    let my_board = Board::new(50, 50);
    my_board.print();
}
