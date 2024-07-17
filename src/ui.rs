use mooeye::{sprite, ui::{self, UiContainer, UiContent}};
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

//TODO: Use TOP_BAR for top bar drawing
use super::config;

pub struct GUI {
    pub gui: ui::UiElement<()>,
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
        let ui_buttons = ["play", "stop", "step"];

        top_bar.spacing = 10.;
        for (i, s) in ui_buttons.iter().enumerate(){
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
            .with_alignment(ui::Alignment::Min, ui::Alignment::Min)
            .with_tooltip(
                graphics::Text::new(format!("{} simulation.", s))
                    .set_scale(14.)
                    .set_font("Zepto")
                    .to_owned()
                    .to_element_builder(0, ctx)
                    .with_visuals(cont_vis)
                    .build(),
            )
            .as_shrink()
            .build();

            top_bar.add(ui_sprite);
        }

        let spacer_width:f32 = ctx.gfx.size().0 - (ui_buttons.len()*20) as f32;

        let top_bar = top_bar
            .to_element_builder(0, ctx)
            .with_visuals(cont_vis)
            .with_padding((4., spacer_width , 4., 4.))
            .with_alignment(ui::Alignment::Min, ui::Alignment::Min)
            .build();

        Ok(Self {
            gui: top_bar,
        })
    }

    pub fn draw(&mut self, ctx: &mut Context, mouse_listen: bool) -> Result<(), GameError> {
        // Once again, we first create a canvas and set a pixel sampler. Note that this time, we don't clear the background.

        let mut canvas = Canvas::from_frame(ctx, None);
        canvas.set_sampler(Sampler::nearest_clamp());

        self.gui.draw_to_screen(ctx, &mut canvas, mouse_listen);

        canvas.finish(ctx)?;

        Ok(())
    }

    pub fn get_messages(&mut self, ctx: &mut Context) -> u32 {
        let messages = self.gui.manage_messages(ctx, None);
        for i in 1..=5 {
            if messages.contains(&ui::UiMessage::Triggered(i)) {
                return i;
            }
        }
        0
    }   

}