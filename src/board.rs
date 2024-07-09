#[derive(Clone)]
struct Cell {
    pub alive: bool,
}

pub struct Board {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Board {
        Board {
            cells: vec![Cell { alive: false }; width * height],
            width,
            height,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        assert!(x < self.width && y < self.height);
        self.cells[y * self.width + x].alive
    }

    pub fn set(&mut self, x: usize, y: usize, alive: bool) {
        assert!(x < self.width && y < self.height);
        self.cells[y * self.width + x].alive = alive;
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_width(&self) -> usize {
        self.width
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

    pub fn apply_rules(&self, x: i32, y: i32) -> bool {
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

    pub fn evolve(&self, other: &mut Board) {
        for y in 0..self.height {
            for x in 0..self.width {
                let alive = self.apply_rules(x as i32, y as i32);
                other.set(x, y, alive);
            }
        }
    }

    pub fn random_init(&mut self) {
        for x_idx in 0..self.width {
            for y_idx in 0..self.height {
                if rand::random() {
                    self.set(x_idx, y_idx, true);
                } else {
                    self.set(x_idx, y_idx, false);
                }
            }
        }
    }

    pub fn set_glider(&mut self, x: usize, y: usize) {
        assert!(x >= 1 && y >= 1);
        self.set(x, y - 1, true);
        self.set(x + 1, y, true);
        self.set(x - 1, y + 1, true);
        self.set(x, y + 1, true);
        self.set(x + 1, y + 1, true);
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        println!("**************************************************************************");
        for y_idx in 0..self.height {
            for x_idx in 0..self.width {
                if self.get(x_idx, y_idx) {
                    print!("█");
                } else {
                    print!("░");
                }
            }
            println!();
        }
    }
}
