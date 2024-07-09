use std::cmp::min;

use raylib::prelude::*;

use crate::board::Board;

pub struct AppState {
    square_hovered: [i32; 2],
    sq_width: f32,
}

pub fn run_update(board: &mut Board, rl: &mut RaylibHandle) -> AppState {
    let screen_height = rl.get_screen_height();
    let screen_width = rl.get_screen_width();
    let min_screen_size = min(screen_height, screen_width);

    let [width, height] = board.get_size();
    let small_size = min(width, height);
    let sq_width = min_screen_size as f32 / small_size as f32;

    let mouse_pos = rl.get_mouse_position();
    let square_hovered_vec = mouse_pos / sq_width;
    let square_hovered = [square_hovered_vec.x as i32, square_hovered_vec.y as i32];


    if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
        board.set(square_hovered[0] as usize, square_hovered[1] as usize, true);
    } else if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT) {
        board.set(square_hovered[0] as usize, square_hovered[1] as usize, false);
    } else if rl.is_key_released(KeyboardKey::KEY_G) {
        board.set_glider(square_hovered[0] as usize, square_hovered[1] as usize);
    }

    if rl.is_key_released(KeyboardKey::KEY_SPACE) || rl.is_key_down(KeyboardKey::KEY_R) {
        board.run_step();
    }

    return AppState {
        sq_width,
        square_hovered
    }
}

pub fn run_render(app_state: AppState, board: &Board, rl: &mut RaylibHandle, thread: &RaylibThread) {
    let [width, height] = board.get_size();

    let mut d = rl.begin_drawing(&thread);
    d.clear_background(Color::DARKGRAY);
    for x in 0..width {
        for y in 0..height {
            d.draw_rectangle_pro(Rectangle::new(
                x as f32 * app_state.sq_width,
                y as f32 * app_state.sq_width,
                app_state.sq_width,
                app_state.sq_width,
                
            ), Vector2::zero(), 0.0, if board.get(x, y) { color::Color::BLACK } else { color::Color::WHITE });
        }
    }
    d.draw_rectangle_pro(Rectangle::new(
        app_state.square_hovered[0] as f32 * app_state.sq_width,
        app_state.square_hovered[1] as f32 * app_state.sq_width,
        app_state.sq_width,
        app_state.sq_width
    ), Vector2::zero(), 0.0, color::rcolor(0, 0, 0, 150));
    d.draw_text("[LEFT_CLICK] to revive a tile", 10, 10, 10, color::Color::BLUE);
    d.draw_text("[RIGHT_CLICK] to kill a tile", 10, 30, 10, color::Color::BLUE);
    d.draw_text("[G] to generate a glider in the cursor position", 10, 50, 10, color::Color::BLUE);
    d.draw_text("[SPACE] to step the simulation", 10, 70, 10, color::Color::BLUE);
    d.draw_text("[R] to run the simulation continuously", 10, 90, 10, color::Color::BLUE);
}