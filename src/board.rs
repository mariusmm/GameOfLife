#[derive(Clone)]
pub enum CellState {
    Dead,
    Alive,
}
#[derive(Clone)]
struct Cell {
    alive: crate::CellState,
}
#[derive(Clone)]
pub struct Board {
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
    pub fn new(width: usize, height: usize) -> Board {
        Board {
            cells: vec![Cell { alive: CellState::Dead }; width * height],
            width,
            height,
        }
    }
    fn get(&self, x: usize, y: usize) -> &CellState {
        &self.cells[y * self.width + x].alive
    }
    pub fn set(&mut self, x: usize, y: usize, state: CellState) {
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

    pub fn apply_rules(&self, x: i32, y: i32) -> CellState {
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

    pub fn print_board(&self){
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