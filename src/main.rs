extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

mod life;

const CELL_SIZE: i32 = 10;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 600, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let size : i32 = 64;
    let mut board1: Vec<u8> = life::gen_board(size);
    let mut board2: Vec<u8> = board1.clone();
    let mut current_board: usize = 0;

    let mut i: u32 = 0;
    'running: loop {
        i = (i + 12) % 255;
        canvas.set_draw_color(Color::RGB(i as u8, 64, (255 - i) as u8));
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

        // The rest of the game loop goes here...
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        let drawing_board= match current_board {
            0 => &board1,
            1 => &board2,
            _ => unreachable!()
        };
        for x in 0..size {
            for y in 0..size {
                if drawing_board[(y*size + x) as usize] == 0 {
                    let x = x as i32;
                    let y = y as i32;
                    let _ = canvas.fill_rect(sdl2::rect::Rect::new(x*CELL_SIZE,
                                                                   y*CELL_SIZE,
                                                                   CELL_SIZE as u32,
                                                                   CELL_SIZE as u32));
                }
            }
        }

        let next_board = (current_board + 1) % 2;
        if next_board == 1 {
            life::life(&board1, &mut board2, size);
        } else {
            life::life(&board2, &mut board1, size);
        }
        current_board = next_board;

        let _ = canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 10));
    }
}
