//! Welcome to the code for the 4D maze game
//!
//! Project Organisation through the weeks
//! * [Week One](maze_lib/index.html)

#![warn(clippy::pedantic)]

#[macro_use]
extern crate ndarray;

pub mod game;
pub mod maze_lib;

pub use crate::maze_lib::{
    maze_base::{Maze, MazeAxis3, MazeCell, MazeSlice},
    maze_gen::gen_maze,
};

pub(crate) use crate::maze_lib::MAZE_SIZE;

fn main() {
    game::run();
}
