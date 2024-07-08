use std::cmp::PartialEq;
use rand::prelude::*;
use std::{thread, time::Duration};

#[derive(Clone)]
enum CellState {
    Dead,
    Alive,
}
#[derive(Clone)]
struct Cell {
    alive: CellState,
}
#[derive(Clone)]
struct Board {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}

impl PartialEq for &CellState {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (CellState::Dead, CellState::Dead) => true,
            (CellState::Alive, CellState::Alive) => true,
            _ => false,
        }
    }
}

impl Board {
    fn new(width: usize, height: usize) -> Board {
        Board {
            cells: vec![Cell { alive: CellState::Dead }; width * height],
            width,
            height,
        }
    }
    fn get(&self, x: usize, y: usize) -> &CellState {
        &self.cells[y * self.width + x].alive
    }
    fn set(&mut self, x: usize, y: usize, state: CellState) {
        self.cells[y * self.width + x].alive = state;
    }
    fn count_alive_neighbors(&self, x: i32, y: i32) -> usize {
        let mut count = 0;
        for x_idx in [-1,0,1]{
            for y_idx in [-1,0,1]{
                if x + x_idx < 0
                    || x + x_idx >= self.width as i32
                    || y + y_idx < 0
                    || y + y_idx >= self.height as i32{
                    continue;
                }
                if self.get((x + x_idx) as usize, (y + y_idx) as usize) == &CellState::Alive{
                    count += 1;
                }
            }
        }
        count
    }

    fn apply_rules(&self, x: i32, y: i32) -> CellState {
        let alive_neighbors = self.count_alive_neighbors(x, y);

        if self.get(x as usize, y as usize) == &CellState::Alive{
            match alive_neighbors {
                2 | 3 => CellState::Alive,
                _ => CellState::Dead,
            }
        } else {
            match alive_neighbors {
                3 => CellState::Alive,
                _ => CellState::Dead,
            }
        }
    }

    fn print_board(&self){
        for y in 0..self.height{
            for x in 0..self.width{

                match self.get(x, y){
                    CellState::Alive => print!("🙋‍ \t"),
                    CellState::Dead => print!("😵 \t"),
                }
            }
            println!();
        }
    }
}


fn main() {
    const WIDTH: usize = 10;
    const HEIGHT: usize = 10;
    let mut board = Board::new(WIDTH, HEIGHT);
    let mut rng = thread_rng();
    // board.set(rng.gen_range(0..WIDTH), rng.gen_range(0..HEIGHT), CellState::Alive);
    for _ in 0..(WIDTH * HEIGHT / 4) {
        board.set(rng.gen_range(0..WIDTH), rng.gen_range(0..HEIGHT), CellState::Alive);
    }


    loop {
        println!("Current Board:");
        board.print_board();

        let mut new_board = board.clone();
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                new_board.set(x, y, board.apply_rules(x as i32, y as i32));
            }
        }
        board = new_board;

        thread::sleep(Duration::from_millis(500));
        println!("\x1B[2J\x1B[1;1H");
    }
}
