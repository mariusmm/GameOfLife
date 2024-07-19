use ggez::graphics::{DrawParam, Canvas, Color, Rect};

use ggez::Context;
use rand::Rng;
use std::thread;
use std::sync::{Arc, Mutex};

use super::config::*;

const CHUNK_SIZE: usize = 100;

#[derive(Clone, Debug)]
pub struct Cell {
    pub alive: bool,
}

#[derive(Debug)]
pub struct Board {
    cells: Vec<Cell>,
    pub width: usize,
    pub height: usize,
    pub use_threading: bool,
}

impl Board {
    pub fn new(width: usize, height: usize, use_threading: bool) -> Board {
        let mut board = Board {
            cells: vec![Cell { alive: false }; width * height],
            width,
            height,
            use_threading,
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
        let mut new_board = Board::new(self.width, self.height, self.use_threading);
    
        if self.use_threading && self.width > CHUNK_SIZE && self.height > CHUNK_SIZE{
            let mut handles = vec![];
            let new_board_arcs: Vec<Vec<Arc<Mutex<Board>>>> = (0..self.height / CHUNK_SIZE)
                .map(|_| (0..self.width / CHUNK_SIZE).map(|_| Arc::new(Mutex::new(Board::new(CHUNK_SIZE, CHUNK_SIZE, false)))).collect())
                .collect();
    
            for chunk_x in (0..self.width).step_by(CHUNK_SIZE) {
                for chunk_y in (0..self.height).step_by(CHUNK_SIZE) {
                    let new_board_arc = Arc::clone(&new_board_arcs[chunk_y / CHUNK_SIZE][chunk_x / CHUNK_SIZE]);
                    let board_cells = self.cells.clone();
                    let width = self.width.clone();
                    let height = self.height;
    
                    let handle = thread::spawn(move || {
                        let board = Board { cells: board_cells, width, height, use_threading: false };
                        for y in chunk_y..(chunk_y + CHUNK_SIZE).min(height) {
                            for x in chunk_x..(chunk_x + CHUNK_SIZE).min(width) {
                                let alive = board.apply_rules(x as i32, y as i32);
                                let mut new_board_lock = new_board_arc.lock().unwrap();
                                new_board_lock.set(x - chunk_x, y - chunk_y, alive);
                            }
                        }
                    });
    
                    handles.push(handle);
                }
            }
    
            for handle in handles {
                handle.join().unwrap();
            }
    
            // Combine the chunks into the final new_board
            for chunk_x in (0..self.width).step_by(CHUNK_SIZE) {
                for chunk_y in (0..self.height).step_by(CHUNK_SIZE) {
                    let new_board_arc = Arc::clone(&new_board_arcs[chunk_y / CHUNK_SIZE][chunk_x / CHUNK_SIZE]);
                    let chunk_board = new_board_arc.lock().unwrap();
                    for y in 0..CHUNK_SIZE {
                        for x in 0..CHUNK_SIZE {
                            if chunk_y + y < self.height && chunk_x + x < self.width {
                                new_board.set(chunk_x + x, chunk_y + y, chunk_board.get(x, y));
                            }
                        }
                    }
                }
            }
        } else {
            for y in 0..self.height as i32 {
                for x in 0..self.width as i32 {
                    let alive = self.apply_rules(x, y);
                    new_board.set(x as usize, y as usize, alive);
                }
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

    pub fn input_cell(&mut self, ctx: &mut Context){
        let position = ctx.mouse.position();
    
        let cell_width = GRID_CELL_SIZE.0 as f32;
        let cell_height = GRID_CELL_SIZE.1 as f32;
        
        let x = ((position.x / cell_width) as usize).min(self.width - 1);
        let y = (((position.y - TOP_BAR) / cell_height) as usize).min(self.height - 1);
        
        let index = y * self.width + x;
        
        if index < self.cells.len() {
            self.cells[index].alive = !self.cells[index].alive;
        }
    }
}
