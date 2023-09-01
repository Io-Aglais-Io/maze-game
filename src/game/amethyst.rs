use amethyst::{
    assets::{AssetStorage, Loader},
    core::{
        cgmath::{Matrix4, Vector3},
        transform::{GlobalTransform, Transform},
    },
    ecs::prelude::{Component, VecStorage},
    input::{is_close_requested, is_key_down},
    prelude::*,
    renderer::{
        Camera, DisplayConfig, DrawFlat, Event, Pipeline, PngFormat, PosTex, Projection,
        RenderBundle, Sprite, Stage, Texture, TextureHandle, VirtualKeyCode, WithSpriteRender,
    },
};

const ARENA_HEIGHT: f32 = 100.0;
const ARENA_WIDTH: f32 = 100.0;

pub struct FourMaze;

pub struct Wall {
    x_pos: u64,
    y_pos: u64,
    orient: Orientation,
}

pub enum Orientation {
    Horizontal,
    Vertical,
}

impl<'a, 'b> State<GameData<'a, 'b>> for FourMaze {
    fn handle_event(&mut self, _: StateData<GameData>, event: Event) -> Trans<GameData<'a, 'b>> {
        if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
            Trans::Quit
        } else {
            Trans::None
        }
    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        data.data.update(&data.world);
        Trans::None
    }

    fn on_start(&mut self, _data: StateData<GameData>) {}
}

impl Component for Wall {
    type Storage = VecStorage<Self>;
}

fn initialise_camera(world: &mut World) {
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            ARENA_WIDTH,
            ARENA_HEIGHT,
            0.0,
        )))
        .with(GlobalTransform(
            Matrix4::from_translation(Vector3::new(0.0, 0.0, 1.0)).into(),
        ))
        .build();
}

pub fn run() -> amethyst::Result<()> {
    #[allow(clippy::default_trait_access)]
    amethyst::start_logger(Default::default());

    let path = "./resources/display_config.ron";

    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat::<PosTex>::new()),
    );

    let game_data =
        GameDataBuilder::default().with_bundle(RenderBundle::new(pipe, Some(config)))?;
    let mut game = Application::new("./", FourMaze, game_data)?;
    game.run();

    Ok(())
}
