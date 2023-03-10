use ggez::{Context, GameResult, graphics::{self, ImageGeneric, GlBackendSpec}, event, GameError};

pub struct SplashScene {
    // Add fields specific to this scene
    splash_image: Result<ImageGeneric<GlBackendSpec>, GameError>
}

impl SplashScene {
    pub fn new(_ctx: &mut Context) -> GameResult<SplashScene> {
        Ok(SplashScene { 
            splash_image: graphics::Image::new(_ctx, "/splash.png")
        })
    }
}

impl event::EventHandler<GameError> for SplashScene {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Clear the screen
        graphics::clear(ctx, graphics::Color::BLACK);

        let drawable = match &self.splash_image {
            Ok(image) => image,
            Err(error) => return Err(GameError::ResourceLoadError(format!("Error loading image : {}", error)))
        };

        // Draw the splash image
        graphics::draw(
            ctx,
            drawable,
            graphics::DrawParam::new().dest(ggez::mint::Point2 { x: 0.0, y: 0.0 }),
        )?;

        // Display the drawn image
        graphics::present(ctx)
    }
}