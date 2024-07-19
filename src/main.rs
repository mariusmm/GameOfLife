/*#[derive(Clone)]
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
                    //print!("\u{25A0}");
                    print!("*");
                } else {
                    //print!("\u{25A1}");
                    print!(" ");
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
*/

#[derive(Clone)]
struct Cell {
    alive: bool,
}

#[derive(Clone)] // Derive Clone for Board
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

        for x_idx in [-1, 0, 1].iter().cloned() {
            for y_idx in [-1, 0, 1].iter().cloned() {
                if x_idx == 0 && y_idx == 0 {
                    continue;
                }
                let nx = x + x_idx;
                let ny = y + y_idx;
                if nx >= 0 && nx < self.width as i32 && ny >= 0 && ny < self.height as i32 {
                    if self.get(nx as usize, ny as usize) {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    fn apply_rules(&self, x: i32, y: i32) -> bool {
        let num_neighbors = self.count_neighbors(x, y);
        if self.get(x as usize, y as usize) {
            num_neighbors == 2 || num_neighbors == 3
        } else {
            num_neighbors == 3
        }
    }

    fn next_generation(&self) -> Board {
        let mut new_board = self.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                let new_state = self.apply_rules(x as i32, y as i32);
                new_board.set(x, y, new_state);
            }
        }
        new_board
    }

    fn print(&self) {
        println!("**************************************************************************");
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get(x, y) {
                    print!("*");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }

    fn set_initial_pattern(&mut self) {
        // Example: Glider pattern
        self.set(1, 0, true);
        self.set(2, 1, true);
        self.set(0, 2, true);
        self.set(1, 2, true);
        self.set(2, 2, true);
    }
}

fn main() {
    let mut my_board = Board::new(50, 50);
    my_board.set_initial_pattern();

    for _ in 0..100 {  // Evolve for 100 generations
        my_board.print();
        my_board = my_board.next_generation();
        std::thread::sleep(std::time::Duration::from_millis(100)); // Pause for a moment
        print!("\x1B[2J\x1B[1;1H"); // Clear the screen
    }
}
