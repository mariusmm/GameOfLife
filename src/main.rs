use ggez::{ContextBuilder, GameResult};
mod board;
mod graphics;
mod ui;
mod config;
use graphics::Game;
use std::env;
use std::path;
use mooeye::scene_manager::SceneManager;

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
        .window_mode(ggez::conf::WindowMode::default().dimensions(config::SCREEN_SIZE.0, config::SCREEN_SIZE.1))
        .build()?;

	let game = Game::new(&mut ctx)?;
	SceneManager::new_and_run(event_loop, ctx, game);
}