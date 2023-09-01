//! Contains the basic structures that the 4D game will use.
//! This is built in stage 1.

use crate::MAZE_SIZE;
use ndarray::{Array3, ArrayView2, Axis};
use std::fmt;

/// `Maze` is a newtype wrapper around `Array3`. This is done for 2 reasons:
///
/// * It allows me to implement my own methods not in `Array3`
/// * It lets me not type `u8` as the type over and over again
///
/// `Maze` contains 2 methods:
///
/// * `new_empty` equal to `ArrayBase::zeros()` filled with a size of
/// `MAZE_SIZE` * `view_2_axis` equal to `ArrayBase::subview()` using
/// `MazeAxis3` instead of `Axis`
#[derive(Default, Debug)]
pub struct Maze(pub(crate) Array3<MazeCell>);

/// `MazeSlice` is a newtype wrapper around `ArrayView2`. This is done for
/// 2 reasons:
///
/// * It allows me to implement my own methods not in `ArrayView2`
/// * It lets me not type `u8` as the type over and over again
///
/// `MazeSlice` contains 1 method:
///
/// * `cut_to_fit` equal to `ArrayBase::slice()` returning a section of the
/// array where from start to start + len
#[derive(Debug)]
pub struct MazeSlice<'slice>(pub(crate) ArrayView2<'slice, MazeCell>);

/// As ndarray has many dimensional methods it has an axis type that represents
/// many axis. It has a from implementation to transform it into an `Axis`
#[derive(Clone)]
pub enum MazeAxis3 {
    XY,
    XZ,
    YZ,
}

#[derive(Clone, PartialEq)]
pub enum MazeCell {
    Cell,
    Wall,
    OuterWall,
    Start,
    End,
    Unvisited,
    Test(u8),
}

impl Maze {
    /// Create a new empty maze with an `Array3` of zeros
    pub fn new_empty() -> Self {
        Self(Array3::default((MAZE_SIZE, MAZE_SIZE, MAZE_SIZE)))
    }

    #[cfg(test)]
    #[allow(clippy::cast_possible_truncation)]
    // Create a new filled maze for testing
    fn new_prefilled() -> Self {
        let mut mz = Self::new_empty();
        for (pos, cell) in mz.0.iter_mut().enumerate() {
            *cell = MazeCell::Test(pos as u8);
        }

        mz
    }

    /// Creates a `MazeSlice` containing a write-only view of 2 of the
    /// dimensions
    ///
    /// **Panics** if `pos` is greater than the size of the maze
    pub fn view_2_axis(&self, axis3: MazeAxis3, pos: usize) -> MazeSlice {
        MazeSlice(self.0.index_axis(axis3.into(), pos))
    }

    /// Returns the position of `MazeCell::End`
    ///
    /// **Panics** if `gen_maze()` has not been used (i.e. End may not be
    /// there)
    pub fn end_pos(&self) -> (usize, usize, usize) {
        let end_position = self
            .0
            .indexed_iter()
            .find_map(|(pos, data)| {
                if data == &MazeCell::End {
                    Some(pos)
                } else {
                    None
                }
            }) /* If gen_maze is used will always be Some */
            .unwrap();

        debug_assert_eq!(self.0.get(end_position), Some(&MazeCell::End));
        end_position
    }
}

impl<'slice> MazeSlice<'slice> {
    /// Returns a section of the `MazeSlice` so it can be rendered
    ///
    /// **Panics** if any piece of `start` or `len` add up to greater than the
    /// length of the maze is greater than the size of the maze
    #[allow(clippy::deref_addrof)]
    pub fn cut_to_fit(&self, start: (usize, usize), len: (usize, usize)) -> MazeSlice {
        MazeSlice(
            self.0
                .slice(s![start.0..=len.0 + start.0, start.1..=len.1 + start.1]),
        )
    }
}

impl MazeCell {
    pub fn is_traversable(&self) -> bool {
        match self {
            MazeCell::Cell | MazeCell::Start | MazeCell::End => true,
            _ => false,
        }
    }
}

impl MazeAxis3 {
    pub fn cycle(&mut self) {
        *self = match self {
            MazeAxis3::XY => MazeAxis3::XZ,
            MazeAxis3::XZ => MazeAxis3::YZ,
            MazeAxis3::YZ => MazeAxis3::XY,
        };
    }
}

impl From<MazeAxis3> for Axis {
    fn from(maze_axis: MazeAxis3) -> Self {
        match maze_axis {
            MazeAxis3::XY => Self(0),
            MazeAxis3::XZ => Self(1),
            MazeAxis3::YZ => Self(2),
        }
    }
}

impl fmt::Debug for MazeAxis3 {
    #[allow(unused_must_use)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "The {}-axis",
            match self {
                MazeAxis3::XY => "X",
                MazeAxis3::XZ => "Y",
                MazeAxis3::YZ => "Z",
            }
        );

        Ok(())
    }
}

impl fmt::Debug for MazeCell {
    #[allow(unused_must_use)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}",
            match self {
                MazeCell::Wall => "2".to_owned(),
                MazeCell::Cell => "1".to_owned(),
                MazeCell::OuterWall => "0".to_owned(),
                MazeCell::Start => "3".to_owned(),
                MazeCell::End => "5".to_owned(),
                MazeCell::Unvisited => "4".to_owned(),
                MazeCell::Test(t) => format!("{:?}", t),
            }
        );

        Ok(())
    }
}

impl Default for MazeCell {
    fn default() -> Self {
        MazeCell::Unvisited
    }
}

#[cfg(test)]
#[allow(clippy::cast_possible_truncation)]
mod tests {
    use super::*;

    const MAZE_SIZE_2: usize = MAZE_SIZE * MAZE_SIZE;

    #[test]
    fn test_empty() {
        assert_eq!(
            Maze::new_empty().0,
            Array3::<MazeCell>::default((MAZE_SIZE, MAZE_SIZE, MAZE_SIZE))
        )
    }

    #[test]
    fn test_make_slice_no_mut() {
        let mz = Maze::new_prefilled();
        let mz_view = mz.view_2_axis(MazeAxis3::XY, 1);

        for (mz_num, num) in mz_view.0.iter().zip(MAZE_SIZE_2..=MAZE_SIZE_2 * 2) {
            assert_eq!(*mz_num, MazeCell::Test(num as u8));
        }
    }

    #[test]
    fn test_cut_to_fit_nopanic() {
        let mz = Maze::new_prefilled();
        let mz_view = mz.view_2_axis(MazeAxis3::XY, 1);

        for (mz_num, num) in mz_view.cut_to_fit((0, 0), (1, 1)).0.iter().zip(
            [
                MazeCell::Test(MAZE_SIZE_2 as u8),
                MazeCell::Test((MAZE_SIZE_2 + 1) as u8),
                MazeCell::Test((MAZE_SIZE_2 + MAZE_SIZE) as u8),
                MazeCell::Test((MAZE_SIZE_2 + MAZE_SIZE + 1) as u8),
            ]
            .iter(),
        ) {
            assert_eq!(*mz_num, *num)
        }

        for (mz_num, num) in mz_view.cut_to_fit((1, 1), (1, 1)).0.iter().zip(
            [
                MazeCell::Test((MAZE_SIZE_2 + MAZE_SIZE + 1) as u8),
                MazeCell::Test((MAZE_SIZE_2 + MAZE_SIZE + 2) as u8),
                MazeCell::Test((MAZE_SIZE_2 + MAZE_SIZE * 2 + 1) as u8),
                MazeCell::Test((MAZE_SIZE_2 + MAZE_SIZE * 2 + 2) as u8),
            ]
            .iter(),
        ) {
            assert_eq!(*mz_num, *num)
        }

        for (mz_num, num) in mz_view
            .cut_to_fit((0, 0), (0, 0))
            .0
            .iter()
            .zip([MazeCell::Test(MAZE_SIZE_2 as u8)].iter())
        {
            assert_eq!(*mz_num, *num)
        }

        assert_eq!(
            mz_view.cut_to_fit((0, 0), (MAZE_SIZE - 1, MAZE_SIZE - 1)).0,
            mz_view.0
        )
    }

    #[should_panic]
    #[test]
    fn test_cut_to_fit_panic() {
        let mz = Maze::new_prefilled();
        let mz_view = mz.view_2_axis(MazeAxis3::XY, 1);

        assert_eq!(
            mz_view.cut_to_fit((0, 0), (MAZE_SIZE, MAZE_SIZE)).0,
            mz_view.0
        )
    }
}

pub mod week_one {
    //! # Development Plan Stage 1
    //! In this phrase I will:
    //!
    //! * Create data structures that will store the basic data in the array:
    //!   * A wrapped array
    //!   * A view of the array that is composed of references to it
    //!   * An enum to show the choice of axis
    //! * Develop functions that:
    //!   * Create a view from an array
    //!   * Cut a section of the view of the array slice up so that can be
    //! shown on screen
    //!
    //! This is the start of my project so this week will involve setting up
    //! the fundamental data types and setting the project up ready to use.
    //!
    //! I will not need to do much testing this phase, except from testing
    //! every function works as normal, because Rust will guarantee that there
    //! will be no pointer errors (e.g. use-after-free) which can be a problem
    //! when using pointers to a type like the maze viewer trait will be.
    //!
    //! | Test Number | Description
    //! | Test Data                                        | Expected Result
    //! | Actual Result               |
    //! |-------------|---------------------------------------------------------------------------------------------------------|--------------------------------------------------|-----------------------------------------------------------|-----------------------------|
    //! | 1.1         | `Maze`: Test that `new_empty` works correctly
    //! | -                                                | 3D array of all
    //! zeros is returned                         | `test_empty`
    //! | | 1.2         | `Maze`: Test that `view_2_axis` correctly returns
    //! a view to only 2 axes of the array                    | Prefilled
    //! Maze, X axis and the 2nd pos           | Correct part of `Maze`
    //! is returned as a `MazeSlice`       | `test_make_slice_no_mut`    | |
    //! 1.
    //! 3
    //! |
    //! `MazeSlice`:
    //! Test
    //! that
    //! `cut_to_slice`
    //! correctly
    //! returns
    //! a
    //! section
    //! of
    //! itself
    //! cut
    //! to
    //! the
    //! right
    //! dimensions
    //! |
    //! Result
    //! from
    //! `1.
    //! 2`
    //! and
    //! (1,
    //! 1)
    //! along
    //! (1,
    //! 1)
    //! elements
    //! |
    //! Correct
    //! part
    //! of
    //! `MazeSlice`
    //! is
    //! returned
    //! (a
    //! 1
    //! x
    //! 1
    //! section)
    //! |
    //! `test_cut_to_fit_nopanic`
    //! | | 1.4         | `MazeSlice`: Test that `cut_to_slice` correctly
    //! returns a section of itself cut to the right dimensions | Result
    //! from `1.2` and (8,8) along (1,1) elements | Panic()!
    //! | `test_cut_to_fits_panic`   |
    //!
    //! ## What happened
    //!
    //! When I first tried to program this task I got stuck with taking
    //! higher-dimensional slice of the maze so I made the decision to port the
    //! code to `ndarray`. I then created wrapper types around
    //! the base array types in `ndarray` and I am implementing my own methods
    //! on top of those.
    //!
    //! ## Design Structures
    //!
    //! `Maze`:
    //!
    //! ```rust
    //! // Default - Provides a default value when initialised as long as all fields implement debug
    //! // Debug - Provides a printable debug printout as long as all fields implement debug
    //! #[derive(Default, Debug)]
    //! pub struct Maze(Array3<MazeCell>)
    //!
    //! impl Maze {
    //!     // Create a new empty maze with an `Array3` of zeros
    //!     // MAZE_SLICE is a constant declared in the upper mod.rs
    //!     pub fn new_empty() -> Self {
    //!         Maze(Array3::default((MAZE_SIZE, MAZE_SIZE, MAZE_SIZE)))
    //!     }
    //!
    //!     //Only runs in test configuration
    //!     #[cfg(test)]
    //!     #[allow(clippy::cast_possible_truncation)]
    //!     // Create a new filled maze for testing
    //!     fn new_prefilled() -> Self {
    //!         let mut mz = Self::new_empty();
    //!         for (pos, cell) in mz.0.iter_mut().enumerate() {
    //!             *cell = MazeCell::Test(pos as u8);
    //!         }
    //!
    //!         mz
    //!     }
    //!
    //!     // Creates a `MazeSlice` containing a write-only view of 2 of the dimensions
    //!     pub fn view_2_axis(&self, axis3: MazeAxis3, pos: usize) -> MazeSlice {
    //!         MazeSlice(self.0.subview(axis3.into(), pos))
    //!     }
    //! }
    //! ```
    //!
    //! `MazeAxis3`:
    //!
    //! ```rust
    //! pub enum MazeAxis3 {
    //!     XY,
    //!     XZ,
    //!     YZ,
    //! }
    //!
    //! impl From<MazeAxis3> for Axis {
    //!     fn from(maze_axis: MazeAxis3) -> Self {
    //!         // Converts the variants of an `Axis`
    //!         match maze_axis {
    //!             MazeAxis3::XY => Axis(0),
    //!             MazeAxis3::XZ => Axis(1),
    //!             MazeAxis3::YZ => Axis(2),
    //!         }
    //!     }
    //! }
    //!
    //! impl fmt::Debug for MazeAxis3 {
    //!     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    //!         // Print out the name of the variant
    //!         write!(
    //!             f,
    //!             "The {}-axis",
    //!             match self {
    //!                 MazeAxis3::XY => "X",
    //!                 MazeAxis3::XZ => "Y",
    //!                 MazeAxis3::YZ => "Z",
    //!             }
    //!         );
    //!
    //!         Ok(())
    //!     }
    //! }
    //! ```
    //!
    //! `MazeSlice`:
    //!
    //! ```rust
    //! pub struct MazeSlice<'slice>(ArrayView2<'slice, u8>)
    //! // As MazeSlice is a reference to data it needs a lifetime
    //!
    //! impl<'slice> MazeSlice<'slice> {
    //!     pub fn cut_to_fit(&self, start: (usize, usize), len: (usize, usize)) -> MazeSlice {
    //!         MazeSlice(
    //!             self.0
    //!                 .slice(s![start.0..=len.0 + start.0, start.1..=len.1 + start.1]),
    //!         )
    //!         // Cuts a slice through the reference between the start and the length
    //!     }
    //! }
    //! ```
    //!
    //! `MazeCell`
    //!
    //! ```rust
    //! // derive implementations for Clone (as data copying is cheap) and a
    //! // PartialEq (this means that variants can be copied)
    //! #[derive(Clone, PartialEq)]
    //! //pub(crate) makes the data visable to the crate
    //! pub(crate) enum MazeCell {
    //!     Cell,
    //!     Wall,
    //!     OuterWall,
    //!     Start,
    //!     End,
    //!     Unvisited,
    //!     Test(u8),
    //! }
    //!
    //! impl fmt::Debug for MazeCell {
    //!     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    //!         //Print out a number corresponding to the enum varient (so all printouts are aligned)
    //!         write!(
    //!             f,
    //!             "{:?}",
    //!             match self {
    //!                 MazeCell::Wall => "2".to_owned(),
    //!                 MazeCell::Cell => "1".to_owned(),
    //!                 MazeCell::OuterWall => "0".to_owned(),
    //!                 MazeCell::Start => "3".to_owned(),
    //!                 MazeCell::End => "5".to_owned(),
    //!                 MazeCell::Unvisited => "4".to_owned(),
    //!                 MazeCell::Test(t) => format!("{:?}", t),
    //!             }
    //!         );
    //!
    //!         Ok(())
    //!     }
    //! }
    //!
    //! impl Default for MazeCell {
    //!     fn default() -> Self {
    //!         // When initialised MazeCell will have variant Unvisited
    //!         MazeCell::Unvisited
    //!     }
    //! }
    //! ```
    //!
    //!  
    //! ## Psuedocode
    //!
    //! ```ignore
    //! struct Maze (3D array of MazeCell using ndArray)
    //! struct MazeSlice (3D array of references to MazeCell using ndArray)
    //!
    //! enum Axis{
    //!     xy axis
    //!     xz axis
    //!     yz axis
    //! }
    //!
    //! fn take_axis_slice (Self: Maze, axis: Maze3Axis, pos: int) -> MazeSlice{
    //!     return switch(axis)
    //!         case: XY {
    //!             # All data from the other dimensions with X left out at position pos
    //!             Self[][][pos]
    //!         }
    //!         case: XZ {
    //!             # All data from the other dimensions with Y left out at position pos
    //!             Self[][pos][]
    //!         }
    //!         case: YZ {
    //!             # All data from the other dimensions with Z left out at position pos
    //!             Self[pos][][]
    //!         }
    //! }
    //!
    //! fn cut_to_fit(Self: MazeSlice, start: (int, int), len: (int, int)) -> Maze{
    //!     return MazeSlice(
    //!         Self.0[start.0 to len.0 + start.0, start.1 to len.1 + start.1]
    //!     )
    //! }
    //! ```

    // This allows any structures I quote to be used
}
