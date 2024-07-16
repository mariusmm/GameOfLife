use ggez::{
    graphics,
    Context,
    GameResult,
    GameError,
};

use mooeye::scene_manager;

use crate::board::Board;
use crate::ui::GUI;

pub struct Game {
    board: Board,
    running: bool,

    frames: usize,
    fps: String,

    ui: GUI,
}

impl Game {
    pub fn new(ctx: &mut Context) -> GameResult<Game> {
        ctx.gfx.add_font(
            "LiberationMono",
            graphics::FontData::from_path(ctx, "/fonts/Zepto (8px).ttf")?,
        );

        let g = Game {
            board: Board::new(50, 50),
            running: false,

            frames: 0,
            fps: format!("FPS: {}", ctx.time.fps() as i64),

            ui: GUI::new(ctx)?,
        };
        Ok(g)
    }
}

impl scene_manager::Scene for Game {
    fn update(&mut self, _ctx: &mut Context) -> Result<scene_manager::SceneSwitch, GameError> {
        if self.running {
            self.board = self.board.next_generation();
        }
        Ok(scene_manager::SceneSwitch::None)
    }

    fn draw(&mut self, ctx: &mut Context, mouse_listen: bool) -> Result<(), GameError> {

        // Render UI
        self.ui.draw(ctx, mouse_listen)?;
        
        Ok(())
    }
}
