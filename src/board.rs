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

    pub fn print(&self) {
        println!("**************************************************************************");
        for y_idx in 0..self.height {
            for x_idx in 0..self.width {
                if self.get(x_idx, y_idx) {
                    print!("\u{2B1C}");
                } else {
                    print!("\u{2B1B}");
                }
            }
            println!();
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
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_x_y() {
        let mut board = Board::new(5, 5);
        board.set(0, 4, true);
        board.set(0, 0, true);
        board.set(1, 2, true);
        board.set(2, 3, true);
        board.print();
    }

    #[test]
    fn test_board() {
        let mut board = Board::new(5, 5);
        board.set_glider(1, 1);
        board.print();
    }

    #[test]
    fn test_count_neighbors() {
        let mut board = Board::new(5, 5);
        board.set_glider(1, 1);
        let count = board.count_neighbors(1, 1);
        assert_eq!(count, 5);
        let count = board.count_neighbors(0, 0);
        assert_eq!(count, 1);
        let count = board.count_neighbors(0, 1);
        assert_eq!(count, 3);
        let count = board.count_neighbors(1, 3);
        assert_eq!(count, 3);
        let count = board.count_neighbors(0, 3);
        assert_eq!(count, 2);
    }

    #[test]
    fn test_apply_rules() {
        let mut board = Board::new(5, 5);
        board.set_glider(1, 1);
        board.print();
        let rules = board.apply_rules(1, 1);
        assert!(!rules);
        let rules = board.apply_rules(1, 2);
        assert!(rules);
        let rules = board.apply_rules(1, 3);
        assert!(rules);
    }
}
