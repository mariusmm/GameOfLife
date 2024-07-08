#[derive(Clone)]
pub struct Cell {
    alive: bool,
}
pub struct Board {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}
fn decide_random(x:usize, y: usize, count :usize) -> bool{
    let rnd = x*3 + y + count;
    if rnd%2 == 0{
        true
    } else{
        false
    }
}

impl Board {
    pub fn new(width: usize, height: usize) -> Board {
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
        } else if num_neigh == 3{
            true
        }else{
            false
        }
    }
    

    pub fn init_borad(&mut self, game_type:&str){
        match game_type{
            "random" => self.init_random(),
           // "ship" => self.insert_ship(5, 5),
            _ => (),
        }
    }

    /*fn insert_ship(&self, x:i32, y:i32){


        //TODO

    }*/



    fn init_random(&mut self){
        let mut count: usize = 0;
        for x in 0..self.height{
            for y in 0..self.width{
                let rnd=decide_random(x,y, count);
                self.set(x, y, rnd);
                count += 1;
            }
        }
    }
    pub fn update(&mut self){
        let mut new_board = Board::new(self.width, self.height);
        for x in 0..self.height{
            for y in 0..self.width{
                new_board.set(x, y, self.apply_rules(x as i32, y as i32));
            }
        }
        for x in 0..self.height{
            for y in 0..self.width{
                self.set(x, y, new_board.get(x, y));
            }
        }
    }

    pub fn print(&self) {
        println!("**************************************************************************");
        for y_idx in 0..self.height {
            for x_idx in 0..self.width {
                    if self.get(x_idx, y_idx) {
                        print!("\u{25A0}");
                        //print!("*");
                    } else {
                        print!("\u{25A1}");
                        //print!(" ");
                    }
                }
            println!();
        }
    }
}


#[cfg(test)]
mod test{
    use super::*;
    #[test]
    pub fn test_decide_random(){
        let x = 2;
        let y = 3;
        let count = 5;
        let first = decide_random(x, y, count);
        let second = decide_random(x, y, count+1);
        assert_ne!(first, second);

    }

    #[test]
    pub fn test_new(){
        let height = 3;
        let width = 4;
        let test_board = Board::new(width, height);

        assert_eq!(height,test_board.height);
    }

    #[test]
    pub fn test_veins(){
        let height = 15;
        let width = 15;
        let mut test_board = Board::new(width, height);
        test_board.init_borad("random");
        let veins = test_board.count_neighbors(5, 5);
        assert_eq!(6, veins); //if should be 8
    }
}

