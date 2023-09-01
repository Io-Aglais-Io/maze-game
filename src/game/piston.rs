use super::maze_controller::CurrentData;
use glutin_window::GlutinWindow;
use opengl_graphics::{Filter, GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::{
    event_loop::{EventLoop, EventSettings, Events},
    input::RenderEvent,
    window::WindowSettings,
};

pub fn run() {
    // This initialises the basic window
    let settings = WindowSettings::new("4D maze", [900; 2]);
    let mut window: GlutinWindow = settings.build().expect("Window Creation failed");
    let mut events = Events::new(EventSettings::new().lazy(true));

    // Here we set up OpenGL
    let opengl = OpenGL::V3_2;
    let mut gl = GlGraphics::new(opengl);

    // Set up maze
    let mut current_data = CurrentData::new();

    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let glyphs = &mut GlyphCache::new("assets/Games.ttf", (), texture_settings)
        .expect("Could not load font");

    while let Some(e) = events.next(&mut window) {
        current_data.events(&e);
        current_data.check_win();

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::clear;

                clear([1.0; 4], g);
                current_data.draw(glyphs, &c, g)
            });
        }
    }
}
