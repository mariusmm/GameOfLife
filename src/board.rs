use std::cmp::PartialEq;
#[derive(Clone, PartialEq)]
pub enum CellState {
    Dead,
    Alive,
}

#[derive(Clone)]
struct Cell {
    alive: CellState,
}
#[derive(Clone)]
pub struct Board {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
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
#[cfg(test)]
mod tests {
    use std::cmp::PartialEq;
    use super::*;



    #[test]
    fn test_new_board() {
        let board = Board::new(3, 3);
        assert_eq!(board.width, 3);
        assert_eq!(board.height, 3);
        assert_eq!(board.cells.len(), 9);
        assert!(board.cells.iter().all(|cell| cell.alive == CellState::Dead));
    }

    #[test]
    fn test_set_and_get() {
        let mut board = Board::new(3, 3);
        board.set(1, 1, CellState::Alive);
        assert!(board.get(1, 1) == &CellState::Alive);
        board.set(1, 1, CellState::Dead);
        assert!(board.get(1, 1) == &CellState::Dead);
    }

    #[test]
    fn test_count_neighbors() {
        let mut board = Board::new(3, 3);
        board.set(0, 0, CellState::Alive);
        board.set(0, 1, CellState::Alive);
        board.set(1, 0, CellState::Alive);
        assert_eq!(board.count_alive_neighbors(1, 1), 3);
    }
}