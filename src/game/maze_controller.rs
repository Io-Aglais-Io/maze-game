use crate::{gen_maze, Maze, MazeAxis3, MazeCell, MazeSlice};
use graphics::{
    character::CharacterCache, Context, Ellipse, Graphics, Image, Rectangle, Transformed,
};
use piston::input::{keyboard::Key, Button, GenericEvent};

/// Constants that define colours that the game uses
pub mod colours {
    use graphics::types::Color as Colour;

    pub const BLACK: Colour = [1.0, 1.0, 1.0, 1.0];
    pub const BLUE: Colour = [0.0, 0.0, 1.0, 1.0];
    pub const GREEN: Colour = [0.0, 1.0, 0.0, 1.0];
    pub const RED: Colour = [1.0, 0.0, 0.0, 1.0];
    pub const PINK: Colour = [1.0, 0.0, 1.0, 1.0];
    pub const YELLOW: Colour = [1.0, 1.0, 0.0, 1.0];
}

/// The scale up that everything uses
const SCALE_FACTOR: f64 = 65.0;

/// The offset from 0, 0 that everything uses
const OFFSET_FACTOR: f64 = 130.0;

/// `BaseData` is a struct that contains data that is meant to be stored for a
/// long time (i.e. the whole game). It contains 2 fields:
///
/// * `current_maze` which holds the current maze
/// * `end_pos` holds the position of `MazeCell::End`
/// * `score` which contains the user's score
pub struct BaseData {
    current_maze: Maze,
    end_pos: (usize, usize, usize),
    score: u64,
}

/// `CurrentData` is a struct that contains data that is meant to be stored for
/// a short time (i.e. the current level). It contains 4 fields:
///
/// * `player_pos` which holds the player's `player_pos`
/// * `cut_axis` which contains current axis the data is cut through
/// * `cut_pos` which contains `player_pos` the cut will be made in
/// * `base_data` which contains the `BaseData` for the game
///
/// `cut_axis` and `cut_pos` are used to construct the slice 'on the go'
pub struct CurrentData {
    player_pos: (usize, usize, usize),
    cut_axis: MazeAxis3,
    base_data: BaseData,
}

impl CurrentData {
    /// This creates a new `CurrentData` from a base data
    pub fn new() -> Self {
        Self {
            player_pos: (1, 1, 1),
            cut_axis: MazeAxis3::XY,
            base_data: BaseData::default(),
        }
    }

    /// This function draws the graphics every time it is called
    // The allow stops clippy from complaining that I doing casts that may
    // result in truncation of data
    #[allow(clippy::cast_possible_truncation)]
    pub fn draw<G: Graphics, C>(&self, glyphs: &mut C, c: &Context, g: &mut G)
    where
        C: CharacterCache<Texture = G::Texture>,
    {
        // Create a 2D slice
        let mz = self.create_slice();
        // Iterate through the maze
        for ((x_pos, y_pos), cell) in mz.0.indexed_iter() {
            let x_pos = f64::from(x_pos as u32) * SCALE_FACTOR + OFFSET_FACTOR;
            let y_pos = f64::from(y_pos as u32) * SCALE_FACTOR + OFFSET_FACTOR;

            // In the first argument of .draw() consists of a 4-element float array with the
            // elements of the array controlling: 1 - Left/right player_pos
            // 2 - Up/down direction
            // 3 - Left/right width
            // 4 - Up/down width
            Rectangle::new(match cell {
                MazeCell::Cell => colours::BLACK,
                MazeCell::Wall => colours::BLUE,
                MazeCell::OuterWall => colours::GREEN,
                MazeCell::Start => colours::PINK,
                MazeCell::End => colours::RED,
                _ => unreachable!(),
            })
            .draw(
                [
                    x_pos,
                    y_pos,
                    /* = 1 * SCALE_FACTOR */ SCALE_FACTOR,
                    SCALE_FACTOR,
                ],
                &c.draw_state,
                c.transform,
                g,
            )
        }

        self.draw_player(&c, g);
        self.draw_text(glyphs, c, g);
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn draw_player<G: Graphics>(&self, c: &Context, g: &mut G) {
        let (pos_x_like, pos_y_like) = match self.cut_axis {
            MazeAxis3::XY => (
                f64::from(self.player_pos.0 as u32) * SCALE_FACTOR + OFFSET_FACTOR,
                f64::from(self.player_pos.1 as u32) * SCALE_FACTOR + OFFSET_FACTOR,
            ),
            MazeAxis3::XZ => (
                f64::from(self.player_pos.0 as u32) * SCALE_FACTOR + OFFSET_FACTOR,
                f64::from(self.player_pos.2 as u32) * SCALE_FACTOR + OFFSET_FACTOR,
            ),
            MazeAxis3::YZ => (
                f64::from(self.player_pos.1 as u32) * SCALE_FACTOR + OFFSET_FACTOR,
                f64::from(self.player_pos.2 as u32) * SCALE_FACTOR + OFFSET_FACTOR,
            ),
        };

        Ellipse::new(colours::YELLOW).draw(
            [
                pos_x_like,
                pos_y_like,
                /* = 1 * SCALE_FACTOR */ SCALE_FACTOR,
                SCALE_FACTOR,
            ],
            &c.draw_state,
            c.transform,
            g,
        );
    }

    pub fn events<E: GenericEvent>(&mut self, e: &E) {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            // Create a 2D slice
            let mz = self.create_slice();

            // Create value of x, y and z from player pos
            let (x, y, z) = self.player_pos;

            match key {
                // Movement
                Key::Left => match self.cut_axis {
                    MazeAxis3::XY if mz.0.get((x - 1, y)).unwrap().is_traversable() => {
                        self.player_pos.0 -= 1
                    }
                    MazeAxis3::XZ if mz.0.get((x - 1, z)).unwrap().is_traversable() => {
                        self.player_pos.0 -= 1
                    }
                    MazeAxis3::YZ if mz.0.get((y - 1, z)).unwrap().is_traversable() => {
                        self.player_pos.1 -= 1
                    }
                    _ => {}
                },
                Key::Right => match self.cut_axis {
                    MazeAxis3::XY if mz.0.get((x + 1, y)).unwrap().is_traversable() => {
                        self.player_pos.0 += 1
                    }
                    MazeAxis3::XZ if mz.0.get((x + 1, z)).unwrap().is_traversable() => {
                        self.player_pos.0 += 1
                    }
                    MazeAxis3::YZ if mz.0.get((y + 1, z)).unwrap().is_traversable() => {
                        self.player_pos.1 += 1
                    }
                    _ => {}
                },
                Key::Up => match self.cut_axis {
                    MazeAxis3::XY if mz.0.get((x, y - 1)).unwrap().is_traversable() => {
                        self.player_pos.1 -= 1
                    }
                    MazeAxis3::XZ if mz.0.get((x, z - 1)).unwrap().is_traversable() => {
                        self.player_pos.2 -= 1
                    }
                    MazeAxis3::YZ if mz.0.get((y, z - 1)).unwrap().is_traversable() => {
                        self.player_pos.2 -= 1
                    }
                    _ => {}
                },
                Key::Down => match self.cut_axis {
                    MazeAxis3::XY if mz.0.get((x, y + 1)).unwrap().is_traversable() => {
                        self.player_pos.1 += 1
                    }
                    MazeAxis3::XZ if mz.0.get((x, z + 1)).unwrap().is_traversable() => {
                        self.player_pos.2 += 1
                    }
                    MazeAxis3::YZ if mz.0.get((y, z + 1)).unwrap().is_traversable() => {
                        self.player_pos.2 += 1
                    }
                    _ => {}
                },
                // Rotation
                Key::D => self.cut_axis.cycle(),
                _ => {}
            }
        };
    }

    pub fn check_win(&mut self) {
        if self.base_data.current_maze.0.get(self.player_pos) == Some(&MazeCell::End) {
            // `base_data` changes
            self.base_data.score += 1;
            self.base_data.current_maze = gen_maze();
            self.base_data.end_pos = self.base_data.current_maze.end_pos();

            // Reset `self`
            self.cut_axis = MazeAxis3::XY;
            self.player_pos = (1, 1, 1);
        };
    }

    pub fn create_slice(&self) -> MazeSlice {
        self.base_data.current_maze.view_2_axis(
            self.cut_axis.clone(),
            pick_axis(&self.cut_axis, self.player_pos),
        )
    }

    pub fn draw_text<G: Graphics, C>(&self, glyphs: &mut C, c: &Context, g: &mut G)
    where
        C: CharacterCache<Texture = G::Texture>,
    {
        let current_pos_text = (
            format!("Current position: {:?}", self.player_pos),
            [10.0, 25.0],
        );
        let start_pos_text = ("Start position: (1, 1, 1)".to_string(), [260.0, 50.0]);
        let end_pos_text = (
            format!("End position: {:?}", self.base_data.end_pos),
            [10.0, 75.0],
        );
        let current_axis_text = (
            format!(
                "Current Axis: {}",
                match self.cut_axis {
                    MazeAxis3::XY => "XY",
                    MazeAxis3::XZ => "XZ",
                    MazeAxis3::YZ => "YZ",
                }
                .to_string()
            ),
            [260.0, 100.0],
        );
        let score_text = (format!("Score: {}", self.base_data.score), [10.0, 125.0]);

        let texts = [
            current_pos_text,
            start_pos_text,
            end_pos_text,
            current_axis_text,
            score_text,
        ];

        let text_image = Image::new_color(colours::RED);

        for (text, screen_pos) in &texts {
            for (ch, text_pos) in text.chars().zip((0_u32..).map(f64::from)) {
                if let Ok(glyph_char) = glyphs.character(20, ch) {
                    text_image.draw(
                        glyph_char.texture,
                        &c.draw_state,
                        c.transform.trans(
                            screen_pos[0] + glyph_char.left() + text_pos * 20.0,
                            /* The y-axis in font co-ordinates points up instead of down */
                            screen_pos[1] - glyph_char.top(),
                        ),
                        g,
                    )
                }
            }
        }
    }
}

impl Default for BaseData {
    #[inline]
    fn default() -> Self {
        let current_maze = gen_maze();
        let end_pos = current_maze.end_pos();
        Self {
            current_maze,
            end_pos,
            score: 0,
        }
    }
}

impl Default for CurrentData {
    #[inline]
    fn default() -> Self {
        let base_data = BaseData::default();

        Self {
            player_pos: (1, 1, 1),
            cut_axis: MazeAxis3::XY,
            base_data,
        }
    }
}

pub fn pick_axis(axis: &MazeAxis3, pos: (usize, usize, usize)) -> usize {
    match axis {
        MazeAxis3::XY => pos.2,
        MazeAxis3::XZ => pos.1,
        MazeAxis3::YZ => pos.0,
    }
}

#[allow(clippy::doc_markdown)]
pub mod stage_three {
    //! # Development Plan Stage 3
    //!
    //! In this phase I will:
    //!
    //! * Create the code that will create a window the game will be played in
    //! * Create a basic representation of the maze
    //!
    //! ## Tests:
    //! | Test Number | Description                                            | Test Data | Expected Result                  | Actual Result     |
    //! |-------------|--------------------------------------------------------|-----------|----------------------------------|-------------------|
    //! | 4.1         | With empty draw command a empty window appears         | -         | An empty window                  |					  |
	//!	| 4.2         | With empty draw command a empty window named "4D Maze" | -         | An empty window named "4D Maze"  |					  |
	//!	| 4.3		  | Draws a randomly generated maze                        | -         | A randomly generate maze appears |			          |
    //!
    //! ## Pseudocode (for Amethyst)
    //! ```text
    //! function run() {
    //!     GameData gameData = new GameData.default().with_sprite_sheet_processor()
    //!     Game game = new Application(Pong, gameData)
    //!     game.run()
    //! }
    //!
    //! Object MazeWall extends Component {
    //!     x_pos: int
    //!     y_pos: int
    //!
    //!     function new(x_pos: int, y_pos: int) {
    //!         x_pos = x_pos
    //!         y_pos = y_pos
    //!     }
    //! }
    //!
    //! Object Player extends Component {
    //!     x_pos: int
    //!     y_pos: int
    //!
    //!     function new() {
    //!         x_pos = 1
    //!         y_pos = 1
    //!     }
    //! }
    //!
    //! Object Pong extends SimpleState {
    //!     function on_start(data: StateData) {
    //!         World world = data.world
    //!         
    //!         initalise_camera(world)
    //!         world.register(MazeWall)
    //!         world.register(Pong)
    //!     }
    //! }
    //!
    //! function initalise_camera(world: World) {
    //!     return Projection.Orthographic(world)
    //! }
    //! ```
    //!
    //! ## What happened:
    //!
    //! First I tried to create the actual game with the [Amethyst library](https://github.com/amethyst/amethyst).
    //! I had multiple problems:
    //!
    //! * On my Linux computer, it would not compile.
    //! * On a Windows computer, the game would not be able any files.
    //!
    //! After a few weeks of trying, and failing to get it to load anywhere I decided to move onto using the [Piston library](https://github.com/PistonDevelopers/piston). I have had better success using this library and this is what the game is currently using.
    //! This is the reason why I have created psuedocode using Piston and
    //! Amethyst
    //!
    //! ## Pseudocode (for Piston)
    //!
    //! ```text
    //! function run {
    //!     gl = OpenGL.initalise()
    //!     Window window = new Window.name("4D maze").openGL(gl)
    //!
    //!     MazeData mazeData = new MazeData
    //!
    //!     loop {
    //!         mazeData.draw()
    //!     }
    //! }
    //!
    //! Object MazeData {
    //!     player_pos: (x, y, z)
    //!     maze_view: MazeSlice
    //!     current_maze: Maze
    //!     score: int
    //!
    //!     function new() {
    //!         self.player_pos = (1, 1, 1)
    //!         self.current_maze = Maze.maze_gen()
    //!         self.maze_view = current_maze.view_2_axis(Axis.XY, 1)
    //!         self.score = 0
    //!     }
    //!
    //!     function draw (g: Graphics) {
    //!         for (x_pos, y_pos, cell) in self.maze_view {
    //!             Color cellColor;
    //!             
    //!             switch cell{
    //!                 case Cell:
    //!                     cellColour = Black
    //!                 case Wall:
    //!                     cellColour = Blue
    //!                 case OuterWall:
    //!                     cellColour = Green
    //!                 case Start
    //!                     cellColour = Red
    //!                 case End
    //!                     cellColour = Red
    //!                 default:
    //!                     cellColour = new Color(0, 0, 0, 0)
    //!             }
    //!             Rectangle.new(cellColour, [x_pos, y_pos, 10, 10], g.draw_state, g.transform).draw()
    //!         }
    //!     }
    //! }
    //! ```
    //!
    //! ## Design structures
    //!
    //! ```rust
    //! pub struct CurrentData {
    //!     player_pos: (int, usize, usize),
    //!     maze_view: MazeAxis3,
    //!     current_maze: usize,
    //!     score: int,
    //! }
    //! ```
    //!
    //! ## Code changes
    //!
    //! The major change I have made (from the Piston pseudocode) is to change
    //! the definitions of `CurrentData` currently looks like
    //! this:
    //!
    //! ```rust
    //! pub struct CurrentData {
    //!     player_pos: (usize, usize, usize),
    //!     cut_axis: MazeAxis3,
    //!     cut_pos: usize,
    //!     base_data: BaseData,
    //! }
    //! ```
    //!
    //! The base data refers to
    //!
    //! ```rust
    //! pub struct BaseData {
    //!     current_maze: Maze,
    //!     score: u64,
    //! }
    //! ```
    //!
    //! I changed the design because this way is more flexible and allows me to
    //! keep the `BaseData` throughout the whole game.
    //!
    //! The other change is that `maze_view` is split up into `cut_axis` and
    //! `cut_pos` because I was having issues with lifetimes (e.g.
    //! ```text
    //! ```
    //! )
	//!
	//!	Another change I made was to make the start cell a different colour(I called it pink but
	//!	it is really magenta). I did this because sometimes I had trouble telling the start and
	//! the end apart.
    //!
    //! ## Code Explanation
    //!
    //! ### `piston.rs`:
    //!
    //! ```rust
    //! use super::maze_controller::*;
    //! use glutin_window::GlutinWindow;
    //! use opengl_graphics::{GlGraphics, OpenGL};
    //! use piston::{
    //!     event_loop::{EventLoop, EventSettings, Events},
    //!     input::RenderEvent,
    //!     window::WindowSettings,
    //! };
    //!
    //! pub fn run() {
    //! ```
    //!
    //! This code initialises the basic window which is called "4D maze" with a
    //! size 900x900 and lazily loaded (it reruns the loop further down when
    //! something happens)
    //!
    //! ```rust
    //! let settings = WindowSettings::new("4D maze", [900; 2]);
    //! let mut window: GlutinWindow = settings.build().expect("Window Creation failed");
    //! let mut events = Events::new(EventSettings::new().lazy(true));
    //! ```
    //!
    //! The sets up OpenGL and creates a variable that will allow data to be
    //! drawn onto it
    //!
    //! ```rust
    //! let opengl = OpenGL::V3_2;
    //! let mut gl = GlGraphics::new(opengl);
    //! ```
    //!
    //! We call methods to set up the actual game data
    //!
    //! ```rust
    //! let current_data = CurrentData::new();
    //! ```
    //!
    //! This is run every time the game needs to process
    //!
    //! ```rust
    //!     while let Some(e) = events.next(&mut window) {
    //!         if let Some(args) = e.render_args() {
    //! ```
    //!
    //! Each time the drawing code is run the canvas is cleaned and
    //! `CurrentData` draws the maze onto the screen
    //!
    //! ```rust
    //!             gl.draw(args.viewport(), |c, g| {
    //!                 use graphics::clear;
    //!
    //!                 clear([0.0; 4], g);
    //!                 current_data.draw(&c, g)
    //!             });
    //!         }
    //!     }
    //! }
    //! ```
    //!
    //! ### `maze_controller.rs`
    //!
    //! Here the `Color` type is renamed to `Colour` which is easier to
    //! remember, leading to less bugs
    //!
    //! ```rust
    //! use crate::maze_lib::{maze_base::*, maze_gen::gen_maze, MAZE_SIZE};
    //! use graphics::{types::Color as Colour, Context, Graphics, Rectangle};
    //! ```
    //!
    //! `BaseData` is a struct that contains data that is meant to be stored for
    //! a longer time (i.e. the whole game). It contains 2 fields:
    //! * `current_maze` which holds the current maze
    //! * `score` which contains the user's score
    //!
    //! ```rust
    //! pub struct BaseData {
    //!     pub current_maze: Maze,
    //!     pub score: u64,
    //! }
    //! ```
    //!
    //! `CurrentData` is a struct that contains data that is meant to be stored
    //! for short time (i.e. the current level). It contains 4 fields:
    //! * `player_pos` which holds the player's position
    //! * `cut_axis` which contains current axis the data is cut through
    //! * `cut_pos` which contains `player_pos` the cut will be made in
    //! * `base_data` which contains the `BaseData` for the game
    //!
    //! `cut_axis` and `cut_pos` are used to construct the slice 'on the go'
    //! (mentioned above)
    //!
    //! ```rust
    //! pub struct CurrentData {
    //!     player_pos: (usize, usize, usize),
    //!     cut_axis: MazeAxis3,
    //!     cut_pos: usize,
    //!     base_data: BaseData,
    //! }
    //! ```
    //!
    //! These constants are used to define 4 colours [R, G, B, A] (0 - 1) that
    //! allow me to quickly change colours in the maze Colour is just a type
    //! definition `[f64; 4]` which means no constructor is allowed
    //!
    //! ```rust
    //! /// Constants that define colours that the game uses
    //! pub mod colours {
    //!     use graphics::types::Color as Colour;
    //!
    //!     pub const BLACK: Colour = [1.0, 1.0, 1.0, 1.0];
    //!     pub const BLUE: Colour = [0.0, 0.0, 1.0, 1.0];
    //!     pub const GREEN: Colour = [0.0, 1.0, 0.0, 1.0];
    //!     pub const RED: Colour = [1.0, 0.0, 0.0, 1.0];
    //!     pub const PINK: Colour = [1.0, 0.0, 1.0, 1.0];
    //!     pub const YELLOW: Colour = [1.0, 1.0, 0.0, 1.0];
    //! }
    //! ```
    //!
    //! ```rust
    //! impl CurrentData {
    //!     /// This creates a new `CurrentData` from a base data
    //!     pub fn new() -> Self {
    //!         Self {
    //!             player_pos: (1, 1, 1),
    //!             cut_axis: MazeAxis3::XY,
    //!             cut_pos: 1,
    //!             base_data: BaseData::default(),
    //!         }
    //!     }
    //! ```
    //!
    //! This is the actual graphics drawing code using the
    //! `piston2d-opengl_graphics` library
    //!
    //! ```rust
    //!     // The allow stops clippy from complaining that I doing casts that may
    //!     // result in truncation of data
    //!     #[allow(clippy::cast_possible_truncation)]
    //!     pub fn draw<G: Graphics>(&self, c: &Context, g: &mut G) {
    //! ```
    //!
    //! Now we construct the slice. `base_data` can now not be used for the rest
    //! of the function
    //!
    //! ```
    //! // Create a 3D slice
    //! let mz = self
    //!     .base_data
    //!     .current_maze
    //!     .view_2_axis(self.cut_axis.clone(), self.cut_pos);
    //! ```
    //!
    //! This iterates through the `MazeSlice`. `.indexed_iter()` allows returns
    //! a tuple of the sizes of the dimensions which is unpacked.
    //!
    //! ```
    //!         // Iterate through the maze
    //!         for ((x_pos, y_pos), cell) in mz.0.indexed_iter() {
    //! ```
    //!
    //!
    //! These scale the positions up to a suitable scale (after converting it to
    //! a `f64` first)
    //!
    //! ```rust
    //! let x_pos = f64::from(x_pos as u32) * SCALE_FACTOR + OFFSET_FACTOR;
    //! let y_pos = f64::from(y_pos as u32) * SCALE_FACTOR + OFFSET_FACTOR;
    //! ```
    //!
    //! In the first argument of .draw() consists of a 4-element float array
    //! with the elements of the array controlling:
    //! * 0 - Left/right position
    //! * 1 - Up/down direction
    //! * 2 - Left/right width
    //! * 3 - Up/down width
    //!
    //! After this the rectangle is actually drawn to the screen
    //! You may notice the size in different than the psuedocode. This is
    //! because I had to discover the right scale by trial and error
    //!
    //! ```rust
    //!             Rectangle::new(match cell {
    //!                 &MazeCell::Cell => BLACK,
    //!                 &MazeCell::Wall => BLUE,
    //!                 &MazeCell::OuterWall => GREEN,
    //!                 &MazeCell::Start => PINK,
    //!                 &MazeCell::End => RED,
    //!                 _ => unreachable!(),
    //!             })
    //!             .draw(
    //!                 [
    //!                     x_pos,
    //!                     y_pos,
    //!                     /* = 1 * SCALE_FACTOR */ SCALE_FACTOR,
    //!                     SCALE_FACTOR,
    //!                 ],
    //!                 &c.draw_state,
    //!                 c.transform,
    //!                 g,
    //!            )
    //!         }
    //!     }
    //! }
    //! ```
    //!
    //! These defaults allow quick creation on the structs
    //!
    //! ```
    //! impl Default for BaseData {
    //!     #[inline]
    //!     fn default() -> Self {
    //!         Self {
    //!             current_maze: gen_maze(),
    //!             score: 0,
    //!         }
    //!     }
    //! }
    //!
    //! impl Default for CurrentData {
    //!     #[inline]
    //!     fn default() -> Self {
    //!         let base_data = BaseData::default();
    //!
    //!         Self {
    //!             player_pos: (1, 1, 1),
    //!             cut_axis: MazeAxis3::XY,
    //!             cut_pos: 1,
    //!             base_data,
    //!         }
    //!     }
    //! }
    //! ```
}

pub mod stage_four {
    //! # Development Plan Stage 4
    //! In this phase I will:
    //!
    //! * Show the player's location in the maze a coloured dot
    //! * Implement rotation of the maze
    //!
    //! ## Tests:
    //!
    //! | Test Number | Description                  | Test Data | Expected Result                           | Actual Result |
    //! |-------------|------------------------------|-----------|-------------------------------------------|---------------|
    //! | 6.1         | Make sure the player appears | -         | The player is drawn at the start location |               |
    //!
    //! ## Pseudocode:
    //!
    //! ```text
    //! # From last pseudocode in MazeData
    //! Object MazeData {
    //!     player_pos: (x, y, z)
    //!     cut_axis: MazeAxis3,
    //!     cut_pos: usize,
    //!     base_data: BaseData
    //!
    //!     ...
    //!
    //!     function draw_player(g: Graphics) {
    //!         int pos_x_like = 0
    //!         int pos_y_like = 0
    //!         
    //!         switch self.cut_axis {
    //!             case XY:
    //!                 pos_x_like = self.player_pos.0
    //!                 pos_y_like = self.player_pos.1
    //!             case XZ:
    //!                 pos_x_like = self.player_pos.0
    //!                 pos_y_like = self.player_pos.2
    //!             case YZ:
    //!                 pos_x_like = self.player_pos.1
    //!                 pos_y_like = self.player_pos.2
    //!             #No default needed
    //!         }
    //!
    //!         # Exact values TBD
    //!         Ellipse.new(Color.RED, [pos_x_like, pos_y_like, 10, 10], g.draw_state, g.transform).draw()
    //!     }
    //! ```
    //!
    //! ## Problems:
    //!
    //! * I accidentally spelt `Ellipse` as `Eclipse` in the pseudocode (which I
    //!   have corrected) and the actual code which made the code throw errors.
    //!
    //! * There were no circles drawn I fixed this because I realised that I was
    //!   not calling the function anywhere (I was not warned by the compiler
    //!   because the function was public and `unused_code` warning was off).
    //!
    //! ## Code:
    //!
    //! This is located in `MazeData` within `maze_controller.rs`
    //!
    //! The code begins with the same function signature as `draw()`
    //!
    //! ```rust
    //! pub fn draw_player<G: Graphics>(&self, c: &Context, g: &mut G) {
    //!     const YELLOW: Colour = [1.0, 1.0, 0.0, 1.0];
    //! ```
    //!
    //! This matches the player's position and axis to a drawing position
    //!
    //! ```rust
    //! let (pos_x_like, pos_y_like) = match self.cut_axis {
    //!     MazeAxis3::XY => (
    //!         f64::from(self.player_pos.0 as u32) * SCALE_FACTOR + OFFSET_FACTOR,
    //!         f64::from(self.player_pos.1 as u32) * SCALE_FACTOR + OFFSET_FACTOR,
    //!     ),
    //!     MazeAxis3::XZ => (
    //!         f64::from(self.player_pos.0 as u32) * SCALE_FACTOR + OFFSET_FACTOR,
    //!         f64::from(self.player_pos.2 as u32) * SCALE_FACTOR + OFFSET_FACTOR,
    //!     ),
    //!     MazeAxis3::YZ => (
    //!         f64::from(self.player_pos.1 as u32) * SCALE_FACTOR + OFFSET_FACTOR,
    //!         f64::from(self.player_pos.2 as u32) * SCALE_FACTOR + OFFSET_FACTOR,
    //!     ),
    //! };
    //! ```
    //!
    //! This draws the ellipse
    //!
    //! ```rust
    //!     Ellipse::new(YELLOW).draw(
    //!                 [
    //!                     pos_x_like,
    //!                     pos_y_like,
    //!                     /* = 1 * SCALE_FACTOR */ SCALE_FACTOR,
    //!                     SCALE_FACTOR,
    //!                 ],
    //!                 &c.draw_state,
    //!                 c.transform,
    //!                 g,
    //!            )
    //!     }
    //! ```
}
