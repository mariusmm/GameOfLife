use ggez::{
    graphics::{DrawParam, Canvas, Color, Rect},
    //input::mouse
};
use rand::Rng;
use super::config::*;

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
        let mut board = Board {
            cells: vec![Cell { alive: false }; width * height],
            width,
            height,
        };
        board.initialize();
        board

    }

    pub fn initialize(&mut self) {
        let total_cells = (self.width * self.height) as usize;
        let mut rng = rand::thread_rng();
        
        self.cells = (0..total_cells)
            .map(|_| Cell { alive: rng.gen_bool(0.5) })
            .collect();
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

    pub fn draw_board(&self, canvas: &mut Canvas) {
        // Board will adapt to the screen based on the cell size configured
        let cell_width = GRID_CELL_SIZE.0 as f32;
        let cell_height = GRID_CELL_SIZE.1 as f32;

        for (i, cell) in self.cells.iter().enumerate() {
            if cell.alive {
                let x = (i as i16 % GRID_SIZE.0) as f32 * cell_width;
                let y = (i as i16 / GRID_SIZE.0) as f32 * cell_height + TOP_BAR;

                let rect = Rect::new(x, y, cell_width, cell_height);

                canvas.draw(
                    &ggez::graphics::Quad,
                    DrawParam::new()
                        .dest_rect(rect.into())
                        .color(Color::WHITE),
                );
            }
        }
    }

}