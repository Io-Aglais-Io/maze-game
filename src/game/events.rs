pub mod stage_five {
    //! # Development Plan Stage 5
    //!
    //! In this stage I will:
    //! * Implement movement with the arrow keys
    //! * Implement rotation with keys
    //!
    //! ## Tests:
    //!
    //! | Test Number | Description                                   | Test Data                            | Expected Result								 | Actual Result |
    //! |-------------|-----------------------------------------------|--------------------------------------|-----------------------------------------------|---------------|
    //! | 7.1         | Make sure the player moves right              | The maze must allow me to move right | The player moves right						 |               |
	//!	| 7.2         | Make sure the player moves left				  | Move right then left                 | The player moves left						 |               |
	//!	| 7.3         | Make sure the player moves down				  | The maze must allow me to move down  | The player moves down						 |               |
	//!	| 7.4         | Make sure the player moves up				  | Move down then up                    | The player moves up  						 |               |
	//!	| 7.5         | Rotation three times is the same as beginning | -                                    | The maze looks the same after three rotations |               |
	//!	| 7.6         |	Movement is possible after rotation           | -									 | Try tests 7.1 to 7.4 after 1 rotation         |               |
    //!
	//!	Watch this video for the tests
    //!
	//! ## Pseudocode:
    //!
    //! In `piston.rs` in render loop
    //!
    //! ```text
    //!     ...
    //!     loop {
    //!         maze_controller.event()
    //!         ...
    //! ```
    //!
    //! In `maze_controller.rs`
    //!
    //! ```text
    //! object MazeController {
    //!     ...
    //!     
    //!     function events (e: Event) {
    //!         if Button::Keyboard(Key::Left) == e.press_args() {
    //!             switch self.cut_axis {
    //!                 case XY or XZ if self.player_pos.0 >= 1:
    //!                     self.player_pos.0 -= 1
    //!                 case YZ if self.player_pos.1 >= 1:
    //!                     self.player_pos.1 -= 1
    //!                 default:
    //!                     break;
    //!             }
    //!         } else if Button::Keyboard(Key::Right) == e.press_args {
    //!             switch self.cut_axis {
    //!                 case XY or XZ if self.player_pos.0 <= MAZE_SIZE - 1:
    //!                     self.player_pos.0 += 1
    //!                 case YZ if self.player_pos.1 <= MAZE_SIZE - 1:
    //!                     self.player_pos.1 += 1
    //!                 default:
    //!                     break;
    //!             }
    //!         } else if Button::Keyboard(Key::Up) == e.press_args {
    //!             switch self.cut_axis {
    //!                 case XY if self.player_pos.1 >= 1:
    //!                     self.player_pos.1 -= 1
    //!                 case YZ or XZ if self.player_pos.2 >= 1:
    //!                     self.player_pos.2 -= 1
    //!                 default:
    //!                     break;
    //!             }
    //!         } else if Button::Keyboard(Key::Up) == e.press_args {
    //!             switch self.cut_axis {
    //!                 case XY if self.player_pos.1 <= MAZE_SIZE - 1:
    //!                     self.player_pos.1 += 1
    //!                 case YZ or XZ if self.player_pos.2 <= MAZE_SIZE - 1:
    //!                     self.player_pos.2 += 1
    //!                 default:
    //!                     break;
    //!             }
    //!         } else if Button::Keyboard(Key::D) == e.press_args {
    //!             switch self.cut_axis {
    //!                 case XY:
    //!                     self.cut_axis = MazeAxis3::XZ
    //!                 case XZ:
    //!                     self.cut_axis = MazeAxis3::YZ
    //!                 case YZ:
    //!                     self.cut_axis = MazeAxis3::XY
    //!             }
    //!         }
    //!     }        
    //! }
    //! ```
    //!
    //! # Development
    //! * At first I got the wrong equalities. Each part of the switch statement
    //!   should read. e.g
    //!
    //! ```text
    //!     case XY or XZ if self.player_pos.2 < MAZE_SLICE - 2
    //!     # and
    //!     case XY or XZ if self.player_pos.2 > 1
    //! ```
    //!
    //! * Next, I realised this method would not allow control where the player
    //!   moved to. So, I designed a new algorithm that checks whether the target
	//!	  location is allowed to be moved to:
    //!
    //! In `maze_base.rs`:
    //!
    //! ```text
    //! object MazeAxis3 {
    //!     function boolean is_traversable() {
    //!         switch self {
    //!             case Cell or Start or End:
    //!                 return true;
    //!             default:
    //!                 return false;
    //!         }
    //!     }
    //! }
    //! ```
    //!
    //! In `maze_controller.rs`:
    //!
    //! ```text
    //! object MazeController {
    //!     ...
    //!     
    //!     function events (e: Event) {
    //!         (usize, usize, usize) (x, y, z) = self.player_pos
    //!
    //!         if Button::Keyboard(Key::Left) == e.press_args() {
    //!             switch self.cut_axis {
    //!                 case XY if self.maze[x - 1][y].is_traversable():
    //!                     self.player_pos.0 -= 1
    //!                 case XZ if self.maze[x - 1][z].is_traversable():
    //!                     self.player_pos.0 -= 1
    //!                 case YZ if self.maze[y - 1][z].is_traversable():
    //!                     self.player_pos.1 -= 1
    //!                 default:
    //!                     break;
    //!             }
    //!         } else if Button::Keyboard(Key::Right) == e.press_args {
    //!             switch self.cut_axis {
    //!                 case XY if self.maze[x + 1][y].is_traversable():
    //!                     self.player_pos.0 += 1
    //!                 case XZ if self.maze[x + 1][z].is_traversable():
    //!                     self.player_pos.0 += 1
    //!                 case YZ if self.maze[y + 1][z].is_traversable():
    //!                     self.player_pos.1 += 1
    //!                 default:
    //!                     break;
    //!             }
    //!         } else if Button::Keyboard(Key::Up) == e.press_args {
    //!             switch self.cut_axis {
    //!                 case XY if self.maze[x][y - 1].is_traversable():
    //!                     self.player_pos.1 -= 1
    //!                 case XZ if self.maze[x][z - 1].is_traversable():
    //!                     self.player_pos.2 -= 1
    //!                 case YZ if self.maze[y][z - 1].is_traversable():
    //!                     self.player_pos.2 -= 1
    //!                 default:
    //!                     break;
    //!             }
    //!         } else {
    //!             switch self.cut_axis {
    //!                 case XY if self.maze[x][y + 1].is_traversable():
    //!                     self.player_pos.1 += 1
    //!                 case XZ if self.maze[x][z + 1].is_traversable():
    //!                     self.player_pos.2 += 1
    //!                 case YZ if self.maze[y][z + 1].is_traversable():
    //!                     self.player_pos.2 += 1
    //!                 default:
    //!                     break;
    //!             }
    //!         }
    //!     }
    //! }
    //! ```
    //!
    //! * After I moved onto the next stage I realised occasionally movement did
    //!   not work. I found this was because when I had copied the code for the
    //!   up/down keys from the left/right keys I had not changed the +/- target
    //!   to the right axis. So I changed them.
    //!
    //! ## Code
    //!
    //! ```rust
    //!     pub fn events<E: GenericEvent>(&mut self, e: &E) {
    //! 	        if let Some(Button::Keyboard(key)) = e.press_args() {
    //! ```
    //!
    //! Create a 2D slice using this function which makes it quicker to produce
    //! a `MazeSlice`. The function calls `pick_axis()` which looks like this:
    //!
    //! ```rust
    //! let mz = self.create_slice();
    //! ```
    //!
    //! ```rust
    //! pub fn pick_axis(axis: &MazeAxis3, pos: (usize, usize, usize)) -> usize {
    //!     match axis {
    //!         MazeAxis3::XY => pos.2,
    //!         MazeAxis3::XZ => pos.1,
    //!         MazeAxis3::YZ => pos.0,
    //!     }
    //! }
    //! ```
    //!
    //! These function allows:
    //!
    //! * Quicker creation of slices when coding
    //! * Removal of `cut_pos` from `current_data` saving 8 bytes of memory
    //!   space
    //!
    //! A method of `current_data`
    //!
    //! ```rust
    //! pub fn create_slice(&self) -> MazeSlice {
    //!         self.base_data.current_maze.view_2_axis(
    //!             self.cut_axis.clone(),
    //!             pick_axis(&self.cut_axis, self.player_pos),
    //!         )
    //!     }
    //! ```
    //!
    //! Back to `events()`
    //! Create value of x, y and z from `self.player_pos` by pattern-matching
    //!
    //! ```rust
    //! let (x, y, z) = self.player_pos;
    //! ```
    //!
    //! Each of these deal with the a key by matching against it
    //! They work in the same way as the pseudocode
    //!
    //! ```rust
    //! match key {
    //!     Key::Left => match self.cut_axis {
    //!         MazeAxis3::XY if mz.0.get((x - 1, y)).unwrap().is_traversable() => {
    //!             self.player_pos.0 -= 1
    //!         }
    //!         MazeAxis3::XZ if mz.0.get((x - 1, z)).unwrap().is_traversable() => {
    //!             self.player_pos.0 -= 1
    //!         }
    //!         MazeAxis3::YZ if mz.0.get((y - 1, z)).unwrap().is_traversable() => {
    //!             self.player_pos.1 -= 1
    //!         }
    //!         _ => {}
    //!     },
    //!     Key::Right => match self.cut_axis {
    //!         MazeAxis3::XY if mz.0.get((x + 1, y)).unwrap().is_traversable() => {
    //!             self.player_pos.0 += 1
    //!         }
    //!         MazeAxis3::XZ if mz.0.get((x + 1, z)).unwrap().is_traversable() => {
    //!             self.player_pos.0 += 1
    //!         }
    //!         MazeAxis3::YZ if mz.0.get((y + 1, z)).unwrap().is_traversable() => {
    //!             self.player_pos.1 += 1
    //!         }
    //!         _ => {}
    //!     },
    //!     Key::Up => match self.cut_axis {
    //!         MazeAxis3::XY if mz.0.get((x, y - 1)).unwrap().is_traversable() => {
    //!             self.player_pos.1 -= 1
    //!         }
    //!         MazeAxis3::XZ if mz.0.get((x, z - 1)).unwrap().is_traversable() => {
    //!             self.player_pos.1 -= 1
    //!         }
    //!         MazeAxis3::YZ if mz.0.get((y, z - 1)).unwrap().is_traversable() => {
    //!             self.player_pos.2 -= 1
    //!         }
    //!         _ => {}
    //!     },
    //!     Key::Down => match self.cut_axis {
    //!         MazeAxis3::XY if mz.0.get((x, y + 1)).unwrap().is_traversable() => {
    //!             self.player_pos.1 += 1
    //!         }
    //!         MazeAxis3::XZ if mz.0.get((x, z + 1)).unwrap().is_traversable() => {
    //!             self.player_pos.1 += 1
    //!         }
    //!         MazeAxis3::YZ if mz.0.get((y, z + 1)).unwrap().is_traversable() => {
    //!             self.player_pos.2 += 1
    //!         }
    //!     },
    //! }
    //! ```
    //!
    //! This handles rotation by calling `cycle()` on the `MazeAxis3`
    //!
    //! ```rust  
    //!                 Key::D => self.cut_axis.cycle(),
    //! ```
    //!
    //! `cycle()` looks like this (in `maze_base.rs`):
    //!
    //! ```rust
    //! pub fn cycle(&mut self) {
    //!         *self = match self {
    //!             MazeAxis3::XY => MazeAxis3::XZ,
    //!             MazeAxis3::XZ => MazeAxis3::YZ,
    //!             MazeAxis3::YZ => MazeAxis3::XY,
    //!         };
    //!     }
    //! ```
}

pub mod stage_six {
    //! # Development Plan Stage 6
    //!
    //! In this stage I will:
    //!
    //! * Create a GUI
    //! * Create a new maze when the old one is finished and show score
    //!
    //! ## Tests:
    //!
    //! | Test Number | Description                                       | Test Data        | Expected Result                            | Actual Result |
    //! |-------------|---------------------------------------------------|------------------|--------------------------------------------|---------------|
    //!	| 8.1         | Make sure the current location shows that         |	Movement in maze | Current location shows that                |				  |
	//!	| 8.2         | Make sure the end location shows the end location | End location     | End location is where the End cell type is |				  |
	//!	| 8.3         | Make sure the start location shows 1,1,1		  | -                | Start location is (1,1,1)                  |				  |
	//!	| 8.4         | Make sure the current axis shows the current axis | `D` 3 times      | XY, XZ then YZ                             |				  |
    //!
	//! Watch this video for the tests
	//!
    //! ## Pseudocode
    //!
    //! In `maze_controller.rs`
    //!
    //! ```text
    //! object MazeController {
    //!     ...
    //!
    //!     function check_win() {
    //!         if self.base_data.current_maze.0.get(self.player_pos) {
    //!             self.base_data.score++
    //!             self.base_data.current_maze = gen_maze()
    //!             
    //!             self.cut_axis = MazeAxis3::XY
    //!             self.player_pos = (1, 1, 1)
    //!         }
    //!     }
    //!     
    //!     function draw_text(glyphs: CharacterCache, g: Graphics) {
    //!         # (string, pos). Pos is uncertain. When it is coded the positions will be tested
    //!         CURRENT_POS_TEXT = ("Current position:" + self.player_pos, [10,1])
    //!         START_POS_TEXT = ("Start position: (1, 1, 1)", [550,1])
    //!         END_POS_TEXT = ("End position:" + self.base_data.end_pos, [250,1])
    //!         CURRENT_AXIS_TEXT = ("Current Axis:" + switch cut_axis {
    //!             case XY:
    //!                 "XY"
    //!             case XZ:
    //!                 "XZ"
    //!             case YZ:
    //!                 "YZ"
    //!         }, [10, 25])
    //!         SCORE_TEXT = ("Score" + score, [550,25])
    //!
    //!         AXIS_TEXTS = ["XY", "YZ", "XZ"]
    //!
    //!         TEXTS = [
    //!             CURRENT_POS_TEXT,
    //!             START_POS_TEXT,
    //!             END_POS_TEXT,
    //!             CURRENT_AXIS_TEXT,
    //!             SCORE_TEXT
    //!         ]
    //!
    //!         text_image = Image.new_color(BLACK)
    //!
    //!         for (text, text_pos) in TEXTS {
    //!             for char in text.chars() {
    //!                 glyph_char = glyph(10, char)
    //!                 if glyph_char.is_ok() {
    //!                     text_image.draw(glyph_char, text_pos, g.draw_state, g.transform)
    //!                 }
    //!             }
    //!         }
    //!     }
    //! ...
    //!  
    //! # Add field in self.base_data to store the end position
    //! object self.base_data {
    //!     ...
    //!     end_pos: (usize, usize, usize)
    //!
    //!     function new() {
    //!         ...
    //!         end_pos = current_maze.end_pos()
    //!     }
    //! }
    //! ```
    //!
    //! In `maze_base.rs`
    //! ```text
    //! object Maze {
    //!     ...
    //!     function (usize, usize, usize) end_pos() {
    //!         return 0.find(MazeCell::End)
    //!     }
    //! }
    //! ```
    //!
    //! ## Development
    //! * The first problem was that the text was all placed in one position. I
    //!   solved this by calling `.enumerate()` on the `.char()` iterator. But
    //!   that did not work (as `pos` is a `f64` and `.enumerate()` produces
    //!   `usize`). Next, I tried calling `.zip(0_f64..)` which pairs it up with
    //!   an infinite list of floats but `f64` does not implement the necessary
    //!   trait so `.zip((0_u32..).map(f64::from))` which produces an infinite
    //!   list of `u32` and converts them to `f64`. They are then added to
    //!   `text_pos` (scaled by the text size).
    //! * The second problem was that the letters were going diagonally. This is
    //!   fixed by only adding the `text_pos` to the x-axis only
    //!
    //! ## Code
    //!
    //! In `maze_controller.rs`
    //!
    //! The C: bound makes sure that the texture the graphics backend uses is
    //! also used when drawing the font
    //!
    //! ```rust
    //! pub fn draw_text<G: Graphics, C>(&self, glyphs: &mut C, c: &Context, g: &mut G)
    //!     where
    //!         C: CharacterCache<Texture = G::Texture>,
    //!     {
    //! ```
    //!
    //! These all create a `String` with the sentence and the data. The data
    //! type is `(String, [f64; 2])`
    //!
    //! ```rust
    //! let current_pos_text = (
    //!     format!("Current position: {:?}", self.player_pos),
    //!     [10.0, 25.0],
    //! );
    //! let start_pos_text = ("Start position: (1, 1, 1)".to_string(), [260.0, 50.0]);
    //! let end_pos_text = (
    //!     format!("End position: {:?}", self.base_data.end_pos),
    //!     [10.0, 75.0],
    //! );
    //! let current_axis_text = (
    //!     format!(
    //!         "Current Axis: {}",
    //!         match self.cut_axis {
    //!             MazeAxis3::XY => "XY",
    //!             MazeAxis3::XZ => "XZ",
    //!             MazeAxis3::YZ => "YZ",
    //!         }
    //!         .to_string()
    //!     ),
    //!     [260.0, 100.0],
    //! );
    //! let score_text = (format!("Score: {}", self.base_data.score), [10.0, 125.0]);
    //! ```
    //!
    //! Next, all of the texts are grouped into an array
    //!
    //! ```rust
    //! let texts = [
    //!     current_pos_text,
    //!     start_pos_text,
    //!     end_pos_text,
    //!     current_axis_text,
    //!     score_text,
    //! ];
    //! ```
    //!
    //! Set the text colour to red
    //!
    //! ```rust
    //! let text_image = Image::new_color(colours::RED);
    //! ```
    //!
    //! Iterate through the text array, unpacking each element into the `String`
    //! and the position
    //!
    //! ```rust
    //! for (text, screen_pos) in &texts {
    //! ```
    //!
    //! Iterate through each `String`. Before that label the position of
    //! everything in the array (this is mentioned in the development)
    //!
    //! ```rust
    //!             for (ch, text_pos) in text.chars().zip((0_u32..).map(f64::from)) {
    //! ```
    //!
    //! Convert each letter into a size 20 glyph (the `if let` allows just
    //! ignores it if it has an error)
    //!
    //! ```rust
    //!                 if let Ok(glyph_char) = glyphs.character(20, ch) {
    //! ```
    //!
    //! Finally, draw the character
    //!
    //! ```rust
    //!                     text_image.draw(
    //!                         glyph_char.texture,
    //!                         &c.draw_state,
    //!                         c.transform.trans(
    //!                             screen_pos[0] + glyph_char.left() + text_pos * 20.0,
    //!                             /* The y-axis in font co-ordinates points up instead of down */
    //!                             screen_pos[1] - glyph_char.top(),
    //!                         ),
    //!                         g,
    //!                     )
    //!                 }
    //!             }
    //!         }
    //!     }
    //! ```
}
