extern crate amethyst;
use amethyst::prelude::*;
use amethyst::renderer::{DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage};
use amethyst::core::transform::TransformBundle;
use amethyst::input::InputBundle;
use amethyst::ui::{DrawUi, UiBundle};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let path = "./resources/display_config.ron";
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new())
            .with_pass(DrawUi::new()),
    );

    let input_bundle = InputBundle::<String, String>::new()
        .with_bindings_from_file("./resources/bindings.ron")?;

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<String, String>::new())?
        .with(PaddleSystem, "paddle_system", &["input_system"])
        .with(MoveBallSystem, "ball_system", &[])
        .with(BounceSystem, "collision_system", &["paddle_system", "ball_system"])
        .with(WinnerSystem, "winner_system", &["ball_system"]);

    let assets_dir = format!("{}/examples/breakout/resources", env!("CARGO_MANIFEST_DIR"));
    let mut game = Application::new(assets_dir, Example::default(), game_data)?;
    game.run();

    Ok(())
}