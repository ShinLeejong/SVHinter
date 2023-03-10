use ggez::{Context, GameResult, graphics::{self, Image, ImageGeneric, GlBackendSpec}, event, GameError};
use std::path::PathBuf;

pub struct SplashScene {
    // Add fields specific to this scene
    // splash_image: Result<ImageGeneric<GlBackendSpec>, GameError>
}

impl SplashScene {
    pub fn new(_ctx: &mut Context) -> GameResult<SplashScene> {
        Ok(SplashScene { 
            // splash_image: Image::new(_ctx, "/assets/splash.png")
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

        // let drawable = match &self.splash_image {
        //     Ok(image) => image,
        //     Err(error) => return Err(GameError::ResourceLoadError(format!("Error loading image : {}", error)))
        // };

        let mut path = PathBuf::new();
        path.push(env!("CARGO_MANIFEST_DIR"));
        path.push("src");
        path.push("assets");
        path.push("splash.png");

        let image_data = image::open(&path)?.to_rgba8();
        let (width, height) = image_data.dimensions();
        let image = Image::from_rgba8(ctx, width as u16, height as u16, &image_data)?;

        // Draw the splash image
        graphics::draw(
            ctx,
            &image,
            graphics::DrawParam::new().dest(ggez::mint::Point2 { x: 0.0, y: 0.0 }),
        )?;

        // Display the drawn image
        graphics::present(ctx)
    }
}