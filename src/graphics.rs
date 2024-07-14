use ggez;
use ggez::graphics;
use ggez::input::keyboard; 
use ggez::input::mouse;

use crate::board::Board;
use ggez::event::EventHandler;

pub struct Game {
    board: Board,
    running: bool,

    frames: usize,
    fps: String,
}

impl Game {
    pub fn new(ctx: &mut ggez::Context) -> ggez::GameResult<Game> {
        ctx.gfx.add_font(
            "LiberationMono",
            graphics::FontData::from_path(ctx, "/LiberationMono-Regular.ttf")?,
        );

        let g = Game {
            board: Board::new(50, 50),
            running: false,

            frames: 0,
            fps: format!("FPS: {}", ctx.time.fps() as i64),

        };
        Ok(g)
    }

    fn draw_ui(&mut self, ctx: &mut ggez::Context, canvas: &mut graphics::Canvas) -> ggez::GameResult {
        
        // let play_button = graphics::Text::new("Play");
        // let stop_button = graphics::Text::new("Stop");
        // let step_button = graphics::Text::new("Step");

        // graphics::draw(ctx, &play_button, (ggez::mint::Point2 { x: 10.0, y: 10.0 },))?;
        // graphics::draw(ctx, &stop_button, (ggez::mint::Point2 { x: 10.0, y: 30.0 },))?;
        // graphics::draw(ctx, &step_button, (ggez::mint::Point2 { x: 10.0, y: 50.0 },))?;

        self.frames += 1;
        if (self.frames % 100) == 0 {
            self.fps = format!("FPS: {}", ctx.time.fps() as i64);
        }

        canvas.draw(
            graphics::Text::new(&self.fps)
                .set_font("LiberationMono")
                .set_scale(24.),
                ggez::glam::Vec2::new(10.0, 10.0),
        );


        Ok(())
    }
}

impl EventHandler<ggez::GameError> for Game {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        if self.running {
            self.board = self.board.next_generation();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        let mut canvas =
        graphics::Canvas::from_frame(ctx, graphics::Color::from([0.0;4]));


        // Render UI
        self.draw_ui(ctx, &mut canvas)?;
        

        canvas.finish(ctx)?;
        Ok(())
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut ggez::Context, button: mouse::MouseButton, x: f32, y: f32) -> Result<(), ggez::GameError> {
        if button == mouse::MouseButton::Left {
            let x = (x / 10.0) as usize;
            let y = (y / 10.0) as usize;
            if x < self.board.width && y < self.board.height {
                let current = self.board.get(x, y);
                self.board.set(x, y, !current);
            }
        }
        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut ggez::Context, input: keyboard::KeyInput) -> Result<(), ggez::GameError> {
        match input.keycode {
            Some(keyboard::KeyCode::P) => self.running = true,
            Some(keyboard::KeyCode::S) => self.running = false,
            Some(keyboard::KeyCode::Space) => self.board = self.board.next_generation(),
            _ => {}
        }
        Ok(())
    }
}
