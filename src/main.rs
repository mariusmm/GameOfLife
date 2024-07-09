
use rand::Rng;  

#[derive(Clone)]
struct Cell {
    alive: bool,
}

struct Board {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Clone for Board {
    fn clone(&self) -> Board {
        Board {
            cells: self.cells.clone(),
            ..*self
        }
    }
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
        } else {
            num_neigh == 3
        }
    }

    fn print(&self) {
        println!("************************************************************");
        for y_idx in 0..self.height {
        for x_idx in 0..self.width {
                if self.get(x_idx, y_idx) {
                    //print!("\u{25A0}");
                    print!("*");
                } else {
                    //print!("\u{25A1}");
                    print!(" ");
                }
            }
            println!();
        }
    }

    fn inicializa(&mut self)
    {
        let mut rng = rand::thread_rng();  // Crea un generador de números aleatorios
        for y_idx in 0..self.height {
            for x_idx in 0..self.width {
                let random_number: u8 = rng.gen_range(0..=1);  // Genera un número aleatorio entre 0 y 1, inclusive
                self.set(x_idx,y_idx,random_number != 0);
            }
        }
    }

}


extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

pub fn main() {

    
    let mut my_board = Board::new(50, 50);
    my_board.inicializa();
    my_board.print();
    

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0,0,0));
        // The rest of the game loop goes here...
        let mut other_board = my_board.clone();

        for y_idx in 0..50 {
            for x_idx in 0..50 {
                other_board.set(x_idx, y_idx    , my_board.apply_rules(x_idx as i32, y_idx as i32) as bool);
                
                if other_board.get(x_idx,y_idx) == true {
                    canvas.fill_rect(sdl2::rect::Rect::new((x_idx*16) as i32, (y_idx*16) as i32, 16, 16));
                }
            }
        }


        my_board = other_board;
        
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 5));
    }
}
