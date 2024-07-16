use mooeye::{sprite, ui, ui::UiContent, ui::UiContainer};
use ggez::{
    context::Context,
    graphics::{
        Color,
        Canvas, 
        Sampler
    },
    *,
};

use std::time::Duration;

pub struct GUI {
    gui: ui::UiElement<()>,
}

impl GUI {
    /// Creates a new FScene in the mooeye-idiomatic way.
    pub fn new(ctx: &Context) -> Result<Self, GameError> {
        // Reusing the visuals from E.

        let vis = ui::Visuals::new(
            Color::from_rgb(0, 0, 0),
            Color::from_rgb(12, 12, 12),
            1.,
            0.,
        );

        let hover_vis = ui::Visuals::new(
            Color::from_rgb(160, 160, 160),
            Color::from_rgb(12, 12, 12),
            3.,
            0.,
        );

        let cont_vis = ui::Visuals::new(
            Color::from_rgb(128, 128, 128),
            Color::from_rgb(0, 0, 0),
            1.,
            0.,
        );

        let mut top_bar = ui::containers::HorizontalBox::new();

        top_bar.spacing = 10.;
        for (i, s) in ["play", "stop", "step"].iter().enumerate(){
            let ui_sprite = sprite::Sprite::from_path(
                format!("/icons/{}.png", s),
                ctx,
                10,
                10,
                Duration::from_secs_f32(0.25),
            )?
            .to_element_builder((i + 1)  as u32, ctx)
            .with_visuals(vis)
            .with_hover_visuals(hover_vis)
            .scaled(2., 2.)
            .with_tooltip(
                graphics::Text::new(format!("{} Simulation.", s))
                    .set_scale(14.)
                    .to_owned()
                    .to_element_builder(0, ctx)
                    .with_visuals(cont_vis)
                    .build(),
            )
            .as_shrink()
            .build();

            top_bar.add(ui_sprite);
        }

        let top_bar = top_bar
            .to_element_builder(0, ctx)
            .with_visuals(cont_vis)
            .with_padding((2., 2., 2., 2.))
            .build();

        Ok(Self {
            gui: top_bar,
        })
    }

    pub fn draw(&mut self, ctx: &mut Context, mouse_listen: bool) -> Result<(), GameError> {
        // Once again, we first create a canvas and set a pixel sampler. Note that this time, we don't clear the background.

        let mut canvas = Canvas::from_frame(ctx, None);
        canvas.set_sampler(Sampler::nearest_clamp());

        let messages = self.gui.manage_messages(ctx, None);
        for i in 1..=5 {
            if messages.contains(&ui::UiMessage::Triggered(i)) {
                match i {
                    1 => println!("Play Pressed"),
                    2 => println!("Stop Pressed"),
                    3 => println!("Step Pressed"),
                    _ => {},
                };
            }
        }

        self.gui.draw_to_screen(ctx, &mut canvas, mouse_listen);

        canvas.finish(ctx)?;

        Ok(())
    }    

}
