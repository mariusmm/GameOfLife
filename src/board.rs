#[derive(Clone)]
pub struct Cell {
    pub alive: bool,
}

pub struct Board {
    cells: Vec<Cell>,
    pub width: usize,
    pub height: usize,
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
        self.cells[y * self.width + x].alive
    }

    pub fn set(&mut self, x: usize, y: usize, alive: bool) {
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

    pub fn next_generation(&self) -> Board {
        let mut new_board = Board::new(self.width, self.height);

        for y in 0..self.height as i32 {
            for x in 0..self.width as i32 {
                let alive = self.apply_rules(x, y);
                new_board.set(x as usize, y as usize, alive);
            }
        }

        new_board
    }

    // pub fn draw_board(&self, ctx: &mut Context, ) -> GameResult{
    //     // Render game board
    //     for y in 0..self.board.height {
    //         for x in 0..self.board.width {
    //             let color = if self.board.get(x, y) {
    //                 graphics::Color::GREEN
    //             } else {
    //                 graphics::Color::WHITE
    //             };
    //             let rect = graphics::Rect::new_i32(x as i32 * 10, y as i32 * 10, 10, 10);
    //             let square = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, color)?;
    //             graphics::draw(ctx, &square, graphics::DrawParam::default())?;
    //         }
    //     }

    //     Ok(())
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_board() {
        let board = Board::new(3, 3);
        assert_eq!(board.width, 3);
        assert_eq!(board.height, 3);
        assert_eq!(board.cells.len(), 9);
        assert!(board.cells.iter().all(|cell| !cell.alive));
    }

    #[test]
    fn test_set_and_get() {
        let mut board = Board::new(3, 3);
        board.set(1, 1, true);
        assert!(board.get(1, 1));
        board.set(1, 1, false);
        assert!(!board.get(1, 1));
    }

    #[test]
    fn test_count_neighbors() {
        let mut board = Board::new(3, 3);
        board.set(0, 0, true);
        board.set(0, 1, true);
        board.set(1, 0, true);
        assert_eq!(board.count_neighbors(1, 1), 3);
    }
}
