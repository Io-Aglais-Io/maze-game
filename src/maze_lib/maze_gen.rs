//! Generates the maze
use crate::{Maze, MazeCell, MAZE_SIZE};
use rand::{distributions::Uniform, thread_rng, Rng};

pub fn gen_maze() -> Maze {
    const MAZE_SIZE_MINUS_ONE: usize = MAZE_SIZE - 1;

    let mut maze = Maze::new_empty();
    let mut mz = maze.0.view_mut();

    let mut visited: Vec<(usize, usize, usize)> = Vec::with_capacity(MAZE_SIZE.pow(3) / 2);
    let mut tot_visited: Vec<(usize, usize, usize)> = Vec::with_capacity(MAZE_SIZE.pow(3) / 2);
    let mut pos = (1, 1, 1);

    for (row_outer_pos, mut row_outer) in mz.outer_iter_mut().enumerate() {
        match row_outer_pos {
            0 | MAZE_SIZE_MINUS_ONE => {
                for cell in row_outer.iter_mut() {
                    *cell = MazeCell::OuterWall;
                }
            }
            _ if row_outer_pos % 2 == 0 => {
                for (row_inner_pos, mut row_inner) in row_outer.outer_iter_mut().enumerate() {
                    match row_inner_pos {
                        0 | MAZE_SIZE_MINUS_ONE => {
                            for cell in row_inner.iter_mut() {
                                *cell = MazeCell::OuterWall;
                            }
                        }
                        _ => {
                            for (cell_pos, cell) in row_inner.iter_mut().enumerate() {
                                match cell_pos {
                                    0 | MAZE_SIZE_MINUS_ONE => {
                                        *cell = MazeCell::OuterWall;
                                    }
                                    _ => {
                                        *cell = MazeCell::Wall;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => {
                for (row_inner_pos, mut row_inner) in row_outer.outer_iter_mut().enumerate() {
                    match row_inner_pos {
                        0 | MAZE_SIZE_MINUS_ONE => {
                            for cell in row_inner.iter_mut() {
                                *cell = MazeCell::OuterWall;
                            }
                        }
                        _ if row_inner_pos % 2 == 0 => {
                            for (cell_pos, cell) in row_inner.iter_mut().enumerate() {
                                match cell_pos {
                                    0 | MAZE_SIZE_MINUS_ONE => {
                                        *cell = MazeCell::OuterWall;
                                    }
                                    _ => {
                                        *cell = MazeCell::Wall;
                                    }
                                }
                            }
                        }
                        _ => {
                            for (cell_pos, cell) in row_inner.iter_mut().enumerate() {
                                match cell_pos {
                                    0 | MAZE_SIZE_MINUS_ONE => {
                                        *cell = MazeCell::OuterWall;
                                    }
                                    _ if cell_pos % 2 == 0 => {
                                        *cell = MazeCell::Wall;
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    *mz.get_mut(pos).unwrap() = MazeCell::Start;
    visited.push(pos);
    tot_visited.push(pos);

    let mut rng = thread_rng();
    let range = Uniform::new(0, 6);

    loop {
        enum AxisDir {
            XPos,
            XNeg,
            YPos,
            YNeg,
            ZPos,
            ZNeg,
        }

        let (x, y, z) = pos;

        let x1 = x.checked_sub(2).is_none();
        let y1 = y.checked_sub(2).is_none();
        let z1 = z.checked_sub(2).is_none();

        if visited.is_empty() {
            break;
        } else if (x1 || mz.get((x - 2, y, z)) != Some(&MazeCell::Unvisited))
            && mz.get((x + 2, y, z)) != Some(&MazeCell::Unvisited)
            && (y1 || mz.get((x, y - 2, z)) != Some(&MazeCell::Unvisited))
            && mz.get((x, y + 2, z)) != Some(&MazeCell::Unvisited)
            && (z1 || mz.get((x, y, z - 2)) != Some(&MazeCell::Unvisited))
            && mz.get((x, y, z + 2)) != Some(&MazeCell::Unvisited)
        {
            pos = visited.pop().unwrap();
        } else {
            let mut axis: AxisDir;

            loop {
                pos = match rng.sample(range) {
                    0 => {
                        axis = AxisDir::XPos;
                        (x + 2, y, z)
                    }
                    1 => {
                        axis = AxisDir::YPos;
                        (x, y + 2, z)
                    }
                    2 => {
                        axis = AxisDir::ZPos;
                        (x, y, z + 2)
                    }
                    3 if !x1 => {
                        axis = AxisDir::XNeg;
                        (x - 2, y, z)
                    }
                    4 if !y1 => {
                        axis = AxisDir::YNeg;
                        (x, y - 2, z)
                    }
                    5 if !z1 => {
                        axis = AxisDir::ZNeg;
                        (x, y, z - 2)
                    }
                    _ => continue,
                };

                let cell_inner = mz.get(pos);
                if cell_inner == Some(&MazeCell::Unvisited) {
                    break;
                }
            }

            *mz.get_mut(pos).unwrap() = MazeCell::Cell;
            *mz.get_mut(match axis {
                AxisDir::XPos => (x + 1, y, z),
                AxisDir::YPos => (x, y + 1, z),
                AxisDir::ZPos => (x, y, z + 1),
                AxisDir::XNeg => (x - 1, y, z),
                AxisDir::YNeg => (x, y - 1, z),
                AxisDir::ZNeg => (x, y, z - 1),
            })
            .unwrap() = MazeCell::Cell;

            visited.push(pos);
            tot_visited.push(pos);
        }
    }

    *mz.get_mut(tot_visited.pop().unwrap()).unwrap() = MazeCell::End;

    maze
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_4s() {
        let mz = gen_maze();
        for cell in mz.0.iter() {
            if cell == &MazeCell::Unvisited {
                panic!("Unvisited cell found");
            }
        }
    }
}

pub mod week_two {
    //! # Development Plan Stage 2
    //! In this phrase I will:
    //!
    //! * Create the algorithm to randomly generate the maze using `Recursive
    //!   Backtracking`
    //!
    //! ## Tests:
    //!
    //! | Test Number | Description
    //! | Test Data | Expected Result                             | Actual
    //! Result               |
    //! |-------------|----------------------------------------------------------------------------------------------------------------------|-----------|---------------------------------------------|-----------------------------|
    //! | 2.1         | Test that the maze is generated properly by checking no
    //! `MazeCell::Unfilled` exists which show incomplete solutions  | -
    //! | No instances of `MazeCell unfilled`         |`no_4s`
    //!
    //! ## Psuedocode
    //!
    //! ```text
    //! fn r_backtrack() -> Maze {
    //!     Maze mz = Maze.new()
    //!     List visited = List.new()
    //!     List visited_tot = List.new()
    //!     (int, int, int) pos = (1, 1, 1)
    //!     int column = 1
    //!     int row = 1
    //!     int outer_row = 1
    //!
    //!     // X Axis
    //!     for i = 0 to i = MAZE_SIZE ** 2
    //!         mz((row, column, 1)) = MazeCell::OuterWall
    //!         if column == MAZE_SIZE - 1 {
    //!             set column = 1
    //!             set row = row + 1
    //!         }
    //!
    //!         column = column + 1
    //!     }
    //!
    //!     column = 1
    //!     row = 1
    //!     outer_row = 1
    //!
    //!     for i = 0 to i = MAZE_SIZE ** 2
    //!         mz((column, row, MAZE_SIZE)) = MazeCell::OuterWall
    //!         if column == MAZE_SIZE - 1 {
    //!             set column = 1
    //!             set row = row + 1
    //!         }
    //!
    //!         column = column + 1
    //!     }
    //!
    //!     // Y axis
    //!     column = 1
    //!     row = 1
    //!     outer_row = 1
    //!
    //!     for i = 0 to i = MAZE_SIZE ** 2
    //!         mz((column, 1, row_outer)) = MazeCell::OuterWall
    //!         if column == MAZE_SIZE - 1 {
    //!             set column = 1
    //!             set outer_row = outer_row + 1
    //!         }
    //!
    //!         column = column + 1
    //!     }
    //!
    //!     column = 1
    //!     row = 1
    //!     outer_row = 1
    //!
    //!     for i = 0 to i = MAZE_SIZE ** 2
    //!         mz((column, MAZE_SIZE, row_outer)) = MazeCell::OuterWall
    //!         if column == MAZE_SIZE - 1 {
    //!             set column = 1
    //!             set outer_row = outer_row + 1
    //!         }
    //!
    //!         column = column + 1
    //!     }
    //!
    //!     // Z axis
    //!     column = 1
    //!     row = 1
    //!     outer_row = 1
    //!
    //!     for i = 0 to i = MAZE_SIZE ** 2
    //!         mz((1, row, row_outer)) = MazeCell::OuterWall
    //!         if row == MAZE_SIZE - 1 {
    //!             set row = 1
    //!             set outer_row = outer_row + 1
    //!         }
    //!
    //!         row = row + 1
    //!     }
    //!
    //!     column = 1
    //!     row = 1
    //!     outer_row = 1
    //!
    //!     for i = 0 to i = MAZE_SIZE ** 2
    //!         mz((MAZE_SIZE, row, row_outer)) = MazeCell::OuterWall
    //!         if row == MAZE_SIZE - 1 {
    //!             set column = 1
    //!             set outer_row = outer_row + 1
    //!         }
    //!
    //!         row = row + 1
    //!     }
    //!
    //!     // Select the start
    //!     mz((1, 1, 1)) = MazeCell::Start
    //!
    //!     // Step through and add walls
    //!     column = 1
    //!     row = 1
    //!     outer_row = 1
    //!
    //!     while i = 0 to i = (MAZE_SIZE - 2) ** 2 / 2 {
    //!         if (column % 2 == 0) or (row % 2 == 0) or (row_outer % 2 == 0) then {
    //!             mz((column, row, outer_row)) = MazeCell::Wall
    //!         }
    //!
    //!         if column == MAZE_SIZE - 1 {
    //!             set column = 1
    //!             set row = row + 1
    //!         }
    //!
    //!         if row == MAZE_SIZE - 1 {
    //!             set row = 1
    //!             set row_outer = row_outer + 1
    //!         }
    //!
    //!         column = column + 1
    //!     }
    //!
    //!     loop {
    //!         (int, int, int) (x, y, z) = pos
    //!         if visited.is_empty() then {
    //!             break
    //!         }
    //!         else if
    //!             mz((x - 2, y, z)) == Some(MazeCell::Unvisited)
    //!             or mz((x, y - 2, z)) == Some(MazeCell::Unvisited)
    //!             or mz((x, y, z - 2)) == Some(MazeCell::Unvisited)
    //!             or mz((x + 2, y, z)) == Some(MazeCell::Unvisited)
    //!             or mz((x, y + 2, z)) == Some(MazeCell::Unvisited)
    //!             or mz((x, y, z + 2)) == Some(MazeCell::Unvisited)
    //!             then
    //!         {
    //!             pos = visited.pop()
    //!         }
    //!         else {
    //!             enum Dir {
    //!                 XPos
    //!                 XNeg
    //!                 YPos
    //!                 YNeg
    //!                 ZPos
    //!                 ZNeg
    //!             }
    //!
    //!             Dir axis
    //!
    //!             loop {
    //!                 num = thread_rng().sample(Distribution::Uniform(0, 6))
    //!                 switch num {
    //!                     case 0: {
    //!                         pos = (x + 2, y, z)
    //!                         axis = XPos
    //!                     }
    //!                     case 1: {
    //!                         pos = (x, y + 2, z)
    //!                         axis = YPos
    //!                     }
    //!                     case 2: {
    //!                         pos = (x, y, z + 2)
    //!                         axis = ZPos
    //!                     }
    //!                     case 3: {
    //!                         pos = (x - 2, y, z)
    //!                         axis = XNeg
    //!                     }
    //!                     case 4: {
    //!                         pos = (x, y - 2, z)
    //!                         axis = YNeg
    //!                     }
    //!                     case 5: {
    //!                         pos = (x, y, z - 2)
    //!                         axis = ZNeg
    //!                     }
    //!                 }
    //!
    //!                 if mz(pos) == Some(MazeCell::Unvisited) then {
    //!                     break;
    //!                 }
    //!             }
    //!
    //!             mz(pos) = MazeCell::Cell
    //!
    //!             mz(switch axis {
    //!                 case XPos: (x + 1, y, z)
    //!                 case XNeg: (x - 1, y, z)
    //!                 case YPos: (x, y + 1, z)
    //!                 case YNeg: (x, y - 1, z)
    //!                 case ZPos: (x, y, z + 1)
    //!                 case ZNeg: (x, y, z - 1)
    //!             }) = MazeCell::Cell
    //!
    //!             visited.push(pos)
    //!             visited_tot.push(pos)
    //!         }
    //!     }
    //!
    //!     mz(visited_tot.pop()) = MazeCell::End
    //! }
    //! ```
    //!
    //! ## Changes
    //!
    //! * Setting the cells as `MazeCell::OuterWall` is done alongside adding
    //!   walls
    //!
    //! ## Code explanation
    //!
    //! ### First we create the main data structures this program uses:
    //!
    //! A constant that I used as Rust does not allow constant operations in
    //! match arms
    //!
    //! ```rust
    //! pub fn gen_maze() -> Maze {
    //!     const MAZE_SIZE_MINUS_ONE: usize = MAZE_SIZE - 1;
    //! ```
    //!
    //! This is the version of maze that will be used for the whole level and we
    //! use a mutable reference to the inner array throughout the program
    //!
    //! ```rust
    //! let mut maze = Maze::new_empty();
    //! let mut mz = maze.0.view_mut();
    //! ```
    //!
    //! Create the vectors with preinitialised sizes of the half the size of the
    //! array (the most possible as even this is overkill)
    //!
    //! ```rust
    //! let mut visited: Vec<(usize, usize, usize)> = Vec::with_capacity(MAZE_SIZE.pow(3) / 2);
    //! let mut tot_visited: Vec<(usize, usize, usize)> = Vec::with_capacity(MAZE_SIZE.pow(3) / 2);
    //! ```
    //!
    //! This stores the current position of the backtracking pointer
    //!
    //! ```rust
    //! let mut pos = (1, 1, 1);
    //! ```
    //!
    //! This mix of nested iterators and match statements create both the outer
    //! walls and the normal walls. It works by:
    //! * If the current subview of the maze is at position 0 or the last
    //!   position
    //!     * Loop through the inner `MazeCell` and set them to `OuterWall`
    //! * If the current subview of the maze is at a position that is divisible
    //!   by 2
    //!     * Apply a similar check to the top level but without the divisible
    //!       by 2 check instead if the data should not be `OuterWall` it is
    //!       `Wall`
    //! * Otherwise apply the top level algorithm to the top view of the array
    //!     * When you get to last level instead do nothing if otherwise
    //! ```rust
    //! for (row_outer_pos, mut row_outer) in mz.outer_iter_mut().enumerate() {
    //!     match row_outer_pos {
    //!         0 | MAZE_SIZE_MINUS_ONE => {
    //!             for cell in row_outer.iter_mut() {
    //!                 *cell = MazeCell::OuterWall;
    //!             }
    //!         }
    //!         _ if row_outer_pos % 2 == 0 => {
    //!             for (row_inner_pos, mut row_inner) in row_outer.outer_iter_mut().enumerate() {
    //!                 match row_inner_pos {
    //!                     0 | MAZE_SIZE_MINUS_ONE => {
    //!                         for cell in row_inner.iter_mut() {
    //!                             *cell = MazeCell::OuterWall;
    //!                         }
    //!                     }
    //!                     _ => {
    //!                         for (cell_pos, cell) in row_inner.iter_mut().enumerate() {
    //!                             match cell_pos {
    //!                                 0 | MAZE_SIZE_MINUS_ONE => {
    //!                                     *cell = MazeCell::OuterWall;
    //!                                 }
    //!                                 _ => {
    //!                                     *cell = MazeCell::Wall;
    //!                                 }
    //!                             }
    //!                         }
    //!                     }
    //!                 }
    //!             }
    //!         }
    //!         _ => {
    //!             for (row_inner_pos, mut row_inner) in row_outer.outer_iter_mut().enumerate() {
    //!                 match row_inner_pos {
    //!                     0 | MAZE_SIZE_MINUS_ONE => {
    //!                         for cell in row_inner.iter_mut() {
    //!                             *cell = MazeCell::OuterWall;
    //!                         }
    //!                     }
    //!                     _ if row_inner_pos % 2 == 0 => {
    //!                         for (cell_pos, cell) in row_inner.iter_mut().enumerate() {
    //!                             match cell_pos {
    //!                                 0 | MAZE_SIZE_MINUS_ONE => {
    //!                                     *cell = MazeCell::OuterWall;
    //!                                 }
    //!                                 _ => {
    //!                                     *cell = MazeCell::Wall;
    //!                                 }
    //!                             }
    //!                         }
    //!                     }
    //!                     _ => {
    //!                         for (cell_pos, cell) in row_inner.iter_mut().enumerate() {
    //!                             match cell_pos {
    //!                                 0 | MAZE_SIZE_MINUS_ONE => {
    //!                                     *cell = MazeCell::OuterWall;
    //!                                 }
    //!                                 _ if cell_pos % 2 == 0 => {
    //!                                     *cell = MazeCell::Wall;
    //!                                 }
    //!                                 _ => {}
    //!                             }
    //!                         }
    //!                     }
    //!                 }
    //!             }
    //!         }
    //!     }
    //! }
    //! ```
    //!
    //! Set (1, 1, 1) as the start and add it as the first element on the 2
    //! arrays
    //!
    //! ```rust
    //! *mz.get_mut(pos).unwrap() = MazeCell::Start;
    //! visited.push(pos);
    //! tot_visited.push(pos);
    //! ```
    //!
    //! Preinitialise a random number generator
    //!
    //! ```rust
    //! let mut rng = thread_rng();
    //! let range = Uniform::new(0, 6);
    //! ```
    //!
    //! Create an enum to store the various directions the maze could go
    //!
    //!
    //! ```rust
    //!     loop {
    //!         enum AxisDir {
    //!             XPos,
    //!             XNeg,
    //!             YPos,
    //!             YNeg,
    //!             ZPos,
    //!             ZNeg,
    //!         }
    //! ```
    //!
    //! Unpack pos into x, y and z
    //! Then try to subtract 2 from them (if x, y or z == 1 then they will be
    //! false)
    //!
    //! ```rust
    //! let (x, y, z) = pos;
    //!
    //! let x1 = x.checked_sub(2).is_none();
    //! let y1 = y.checked_sub(2).is_none();
    //! let z1 = z.checked_sub(2).is_none();
    //! ```
    //!
    //! If the list is empty: break; you have reached the end of the maze
    //!
    //! ```rust
    //! if visited.is_empty() {
    //!     break;
    //! }
    //! ```
    //!
    //! Otherwise check to see if there are any unvisited cells around (|| is
    //! short circuiting so there will be out of bounds panics). If no cells are
    //! found pop from visited, and try again.
    //!
    //! ```rust
    //!          else if (x1 || mz.get((x - 2, y, z)) != Some(&MazeCell::Unvisited))
    //!             &&
    //! mz.get((x + 2, y, z)) != Some(&MazeCell::Unvisited)
    //!             && (y1 || mz.get((x, y - 2, z)) != Some(&MazeCell::Unvisited))
    //!             && mz.get((x, y + 2, z)) != Some(&MazeCell::Unvisited)
    //!             && (z1 || mz.get((x, y, z - 2)) != Some(&MazeCell::Unvisited))
    //!             && mz.get((x, y, z + 2)) != Some(&MazeCell::Unvisited)
    //!         {
    //!             pos = visited.pop().unwrap();
    //!         }
    //! ```
    //!
    //! Otherwise a random number is generated and matched upon with the result
    //! being stored in `pos` (the if-guards on the matches prevent them) from
    //! causing panics when the numbers go out of bounds
    //!
    //! ```rust
    //! else {
    //!             let mut axis: AxisDir;
    //!
    //!             loop {
    //!                 pos = match rng.sample(range) {
    //!                     0 => {
    //!                         axis = AxisDir::XPos;
    //!                         (x + 2, y, z)
    //!                     }
    //!                     1 => {
    //!                         axis = AxisDir::YPos;
    //!                         (x, y + 2, z)
    //!                     }
    //!                     2 => {
    //!                         axis = AxisDir::ZPos;
    //!                         (x, y, z + 2)
    //!                     }
    //!                     3 if !x1 => {
    //!                         axis = AxisDir::XNeg;
    //!                         (x - 2, y, z)
    //!                     }
    //!                     4 if !y1 => {
    //!                         axis = AxisDir::YNeg;
    //!                         (x, y - 2, z)
    //!                     }
    //!                     5 if !z1 => {
    //!                         axis = AxisDir::ZNeg;
    //!                         (x, y, z - 2)
    //!                     }
    //!                     _ => continue,
    //!                 };
    //! ```
    //!
    //! Check to see if the cell that generated was unvisited; if it is not try
    //! again
    //!
    //! ```rust
    //!                 let cell_inner = mz.get(pos);
    //!                 if cell_inner == Some(&MazeCell::Unvisited) {
    //!                     break;
    //!                 }
    //!             }
    //! ```
    //!
    //! Make the chosen cell and the wall between it and the old cell a
    //! `MazeCell::Cell`
    //!
    //! ```rust
    //! 
    //!             *mz.get_mut(pos).unwrap() = MazeCell::Cell;
    //!             *mz.get_mut(match axis {
    //!                 AxisDir::XPos => (x + 1, y, z),
    //!                 AxisDir::YPos => (x, y + 1, z),
    //!                 AxisDir::ZPos => (x, y, z + 1),
    //!                 AxisDir::XNeg => (x - 1, y, z),
    //!                 AxisDir::YNeg => (x, y - 1, z),
    //!                 AxisDir::ZNeg => (x, y, z - 1),
    //!             })
    //!             .unwrap() = MazeCell::Cell;
    //!
    //!             visited.push(pos);
    //!             tot_visited.push(pos)
    //!         }
    //!     }
    //! ```
    //!
    //! Make the last position generated the end node
    //!
    //!
    //! ```rust
    //! 
    //!     *mz.get_mut(tot_visited.pop().unwrap()).unwrap() = MazeCell::End;
    //!
    //!     maze
    //! }
    //! ```
    //!
    //! ## Development problems:
    //!
    //! ### Problem 1:
    //!
    //! At one point I tried use the vector to store a pointer to the elements
    //! in the array as they were moved around. But, Rust's borrow checker did
    //! not allow me to write code created mutable references to data that
    //! already had an immutable reference pointed at it. Instead I used a
    //! vector that stores a 3-tuple of co-ordinates When I replace the
    //! definition of visited with (actual type can be inferred)
    //!
    //! ```rust
    //! // Declaration
    //! let mut visited: Vec<&MazeCell> = Vec::with_capacity(MAZE_SIZE.pow(3) / 2);
    //!
    //! // Use
    //! visited.push(mz.get(pos).unwrap());
    //! ```
    //!
    //! Rust throws this error
    //!
    //! ```text
    //! error[E0502]: cannot borrow mz as mutable because it is also borrowed as immutable
    //!    --> src/maze_lib/maze_gen.rs:160:14
    //!     |
    //! 91  |     visited.push(mz.get(pos).unwrap());
    //!     |            -- immutable borrow occurs here
    //! ...
    //! 160 |             *mz.get_mut(pos).unwrap() = MazeCell::Cell;
    //!     |              ^^^^^^^^^^^^^^^ mutable borrow occurs here
    //! ...
    //! 173 |             visited.push(mz.get(pos).unwrap());
    //!     |             - immutable borrow used here, in later iteration of loop
    //!
    //! error[E0502]: cannot borrow mz as mutable because it is also borrowed as immutable
    //!    --> src/maze_lib/maze_gen.rs:161:14
    //!     |
    //! 91  |       visited.push(mz.get(pos).unwrap());
    //!     |              -- immutable borrow occurs here
    //! ...
    //! 161 |               *mz.get_mut(match axis {
    //!     |  ______________^
    //! 162 | |                 AxisDir::XPos => (x + 1, y, z),
    //! 163 | |                 AxisDir::YPos => (x, y + 1, z),
    //! 164 | |                 AxisDir::ZPos => (x, y, z + 1),
    //! ...   |
    //! 167 | |                 AxisDir::ZNeg => (x, y, z - 1),
    //! 168 | |             })
    //!     | |______________^ mutable borrow occurs here
    //! ...
    //! 173 |               visited.push(mz.get(pos).unwrap());
    //!     |               - immutable borrow used here, in later iteration of loop
    //! ```
    //!
    //! So I have instead replaced it with a visited definition of:
    //!
    //! ```rust
    //! // Declaration
    //! let mut visited: Vec<(usize, usize, usize)> = Vec::with_capacity(MAZE_SIZE.pow(3) / 2);
    //!
    //! // Use
    //! visited.push(pos);
    //! ```
    //!
    //! Which does not use references so does not get stopped by the borrow
    //! checker
    //!
    //! ### Problem 2:
    //!
    //! I also had a problem with the maze creation step where the program would
    //! panic. The line numbers pointed me to the else if part of the if
    //! statement:
    //!
    //! ```rust
    //!     mz.get((x - 2, y, z)) != Some(&MazeCell::Unvisited) || ...
    //! ```
    //!
    //! Was crashing.
    //!
    //! I quickly realised that this was because the operator was underflowing as
    //! usize is unsigned. So I implemented a check for underflowing subtraction
    //! and, as Rust has short circuiting operators, the program does not crash
    //! at overflow. I now looks like:
    //!
    //! ```rust
    //! // Check for underflow
    //! let z1 = z.checked_sub(2).is_none();
    //!
    //! // Use short-circuiting to guarantee they do not overflow
    //! (x1 || mz.get((x - 2, y, z)) != Some(&MazeCell::Unvisited))
    //!
    //! // I had a similar problem in the random direction picker.
    //! // I solved this by using that value as an if-guard
    //! 3 if !x1 =>
    //! ```
    // This allows any structures I quote to be used
}
