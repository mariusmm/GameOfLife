use ggez::{
    graphics::{
        self,     
        Canvas,
        Sampler
    },
    Context,
    GameResult,
    GameError,
    input::mouse::MouseButton,

};

use mooeye::scene_manager;

use crate::board::Board;
use crate::ui::GUI;

use super::config;

pub struct Game {
    board: Board,
    running: bool,
    ui: GUI,
}

impl Game {
    pub fn new(ctx: &mut Context) -> GameResult<Game> {
        ctx.gfx.add_font(
            "Zepto",
            graphics::FontData::from_path(ctx, "/fonts/Zepto (8px).ttf")?,
        );

        let g = Game {
            board: Board::new(config::GRID_SIZE.0 as usize, config::GRID_SIZE.1 as usize, false),
            running: false,

            ui: GUI::new(ctx)?,
        };
        Ok(g)
    }

    fn parse_messages(&mut self, ctx: &mut Context) -> bool{
        match self.ui.get_messages(ctx) {
            1 => {println!("Pressed Play"); self.running = true; true}
            2 => {println!("Pressed Stop"); self.running = false; true}
            3 => {println!("Pressed Step"); self.board = self.board.next_generation(); true}
            _ => {false}
        }
    }
}

impl scene_manager::Scene for Game {
    fn update(&mut self, ctx: &mut Context) -> Result<scene_manager::SceneSwitch, GameError> {        
        if self.running {
            self.board = self.board.next_generation();
        }

        if self.parse_messages(ctx) == false && ctx.mouse.button_just_pressed(MouseButton::Left){
            self.board.input_cell(ctx);
        }       

        Ok(scene_manager::SceneSwitch::None)
    }

    fn draw(&mut self, ctx: &mut Context, mouse_listen: bool) -> Result<(), GameError> {

        let mut canvas = Canvas::from_frame(ctx, None);
        canvas.set_sampler(Sampler::nearest_clamp());

        self.board.draw_board(&mut canvas);

        canvas.finish(ctx)?;

        // Render UI
        self.ui.draw(ctx, mouse_listen)?;

        
        Ok(())
    }
}
