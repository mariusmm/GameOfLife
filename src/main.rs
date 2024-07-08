use minifb::{Key, Window, WindowOptions};


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

        for x_idx in [-1, 0, 1].iter().cloned() {
            for y_idx in [-1, 0, 1].iter().cloned() {
                if x + x_idx < 0
                    || x + x_idx >= self.width as i32
                    || y + y_idx < 0
                    || y + y_idx >= self.height as i32
                    || (x_idx == 0 && y_idx == 0)
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
                2 | 3 => true,
                _ => false,
            }
        } else {
            num_neigh == 3
        }
    }

    fn next_generation(&self) -> Board {
        let mut new_board = Board::new(self.width, self.height);

        for y in 0..self.height as i32 {
            for x in 0..self.width as i32 {
                let alive = self.apply_rules(x, y);
                new_board.set(x as usize, y as usize, alive);
            }
        }

        new_board
    }

    fn print(&self) {
        println!("**************************************************************************");
        for y_idx in 0..self.height {
            for x_idx in 0..self.width {
                if self.get(x_idx, y_idx) {
                    print!("🟢");
                } else {
                    print!("⚪");
                }
            }
            println!();
        }
        println!("**************************************************************************");
    }
}

fn main() {
    let width = 50;
    let height = 50;
    let mut my_board = Board::new(width, height);

    // Initializing a simple pattern: a glider
    my_board.set(1, 0, true);
    my_board.set(2, 1, true);
    my_board.set(0, 2, true);
    my_board.set(1, 2, true);
    my_board.set(2, 2, true);

    let mut window = Window::new(
        "Conway's Game of Life",
        width * 10,
        height * 10,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut buffer: Vec<u32> = vec![0; width * height * 100];

        for y in 0..height {
            for x in 0..width {
                let color = if my_board.get(x, y) {
                    0x00FF00 // Green for alive cells
                } else {
                    0x000000 // Black for dead cells
                };

                for dy in 0..10 {
                    for dx in 0..10 {
                        buffer[(y * 10 + dy) * width * 10 + (x * 10 + dx)] = color;
                    }
                }
            }
        }

        window.update_with_buffer(&buffer, width * 10, height * 10).unwrap();

        my_board = my_board.next_generation();
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
