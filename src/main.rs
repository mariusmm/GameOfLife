use ggez::{ContextBuilder, GameResult};
use ggez::event;
mod board;
mod graphics;
use graphics::Game;
use std::env;
use std::path;

const GRID_SIZE: (i16, i16) = (30, 20);
const GRID_CELL_SIZE: (i16, i16) = (32, 32);

const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
);

fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("assets");
        path
    } else {
        path::PathBuf::from("./assets")
    };


    let ( mut ctx,  event_loop) = ContextBuilder::new("conways_game_of_life", "Jan Gras").add_resource_path(resource_dir)
        .window_setup(ggez::conf::WindowSetup::default().title("conways_game_of_life!"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()?;
    let game = Game::new(&mut ctx)?;

    event::run(ctx, event_loop, game)
}