use ggez::{Context, GameResult, event, GameError};
use crate::scenes::intro_scene::intro_scene::IntroScene;
use crate::scenes::game_scene::game_scene::GameScene;

pub struct SceneManager {
    current_scene: Box<dyn event::EventHandler<GameError>>,
}

impl SceneManager {
    pub fn new(ctx: &mut ggez::Context) -> GameResult<SceneManager> {
        let intro_scene = IntroScene::new(ctx)?;
        Ok(SceneManager { current_scene: Box::new(intro_scene) })
    }

    pub fn switch_to_intro_scene(&mut self, ctx: &mut Context) -> GameResult<()> {
        let intro_scene = IntroScene::new(ctx)?;
        self.current_scene = Box::new(intro_scene);
        Ok(())
    }

    pub fn switch_to_game_scene(&mut self, ctx: &mut Context) -> GameResult<()> {
        let game_scene = GameScene::new(ctx)?;
        self.current_scene = Box::new(game_scene);
        Ok(())
    }
}

impl event::EventHandler<GameError> for SceneManager {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.current_scene.update(ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.current_scene.draw(ctx);
        Ok(())
    }

    fn mouse_button_down_event(
            &mut self,
            _ctx: &mut Context,
            _button: event::MouseButton,
            _x: f32,
            _y: f32,
        ) {
        self.switch_to_game_scene(_ctx);
    }

    fn mouse_button_up_event(
            &mut self,
            _ctx: &mut Context,
            _button: event::MouseButton,
            _x: f32,
            _y: f32,
        ) {
        self.switch_to_intro_scene(_ctx);
    }
}