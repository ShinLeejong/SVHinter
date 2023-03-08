mod scenes;

use ggez::{Context, GameResult, event, GameError, graphics};
use scenes::scene_manager::SceneManager;

fn main() -> GameResult<()> {
    let (mut ctx, event_loop) = ggez::ContextBuilder::new("SVHinter", "Shin Leejong")
        .build()?;
    let scene_manager = SceneManager::new(&mut ctx)?;

    match config(&mut ctx) {
        Ok(_) => event::run(ctx, event_loop, scene_manager),
        Err(e) => panic!("{}", e)
    }
}

// configuration for game\diagnostic message [0]
fn config(ctx: &mut Context) -> GameResult<()> {
    graphics::set_window_title(&ctx, "Stardew Valley Hinter");

    Ok(())
}

pub struct MyEvent {
    // Add fields specific to your custom event
}

impl event::EventHandler<GameError> for MyEvent {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Handle event updates here
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }
}