use rand::Rng;

pub struct Maze {
    contents: Vec<Vec<Cell>>,
    solution_length: usize,
}

impl Maze {
    pub fn get_solution_length(&self) -> usize {
        self.solution_length
    }
    /// Constructs a new `Maze` with a given size.
    ///
    /// # Arguments
    ///
    /// * `size` - A u16 indicating the width and height of the maze.
    pub fn new(size: u16) -> Self {
        let mut maze = Maze {
            contents: vec![vec![Cell::new(); size as usize + 2]; size as usize + 2],
            solution_length: 0,
        };
        maze.generate();
        maze
    }

    pub fn generate(&mut self) {
        let maximum_pos = self.contents.len(); // this defines a working area for the maze.
        let (sol_start_x, sol_start_y, sol_end_x, sol_end_y) =
            (1, 0, maximum_pos - 2, maximum_pos - 1);
        let mut rng = rand::thread_rng();
        let start_x = sol_start_x;
        let start_y = sol_start_y; // random coords
        self.prepare_board(maximum_pos);
        self.contents[start_x][start_y].visited = true;
        let mut stack = vec![(start_x, start_y)];
        let mut sol_stack = vec![];
        while let Some(current_pos) = stack.last().cloned() {
            if (current_pos.0, current_pos.1) == (sol_end_x, sol_end_y) {
                sol_stack = stack.clone();
                self.solution_length = sol_stack.len() as usize;
            }
            while let Some(sol_cell) = sol_stack.pop() {
                self.contents[sol_cell.0][sol_cell.1].solution = true;
            }
            let valid_directions =
                self.get_valid_directions(current_pos, (maximum_pos, maximum_pos));

            if !valid_directions.is_empty() {
                let direction = valid_directions[rng.gen_range(0..valid_directions.len())];
                let (next_x, next_y) = match direction {
                    Direction::UP => (current_pos.0, current_pos.1 - 1),
                    Direction::DOWN => (current_pos.0, current_pos.1 + 1),
                    Direction::LEFT => (current_pos.0 - 1, current_pos.1),
                    Direction::RIGHT => (current_pos.0 + 1, current_pos.1),
                };

                match direction {
                    Direction::UP => {
                        self.contents[current_pos.0][current_pos.1].top = true;
                        self.contents[next_x][next_y].bottom = true;
                    }
                    Direction::DOWN => {
                        self.contents[current_pos.0][current_pos.1].bottom = true;
                        self.contents[next_x][next_y].top = true;
                    }
                    Direction::LEFT => {
                        self.contents[current_pos.0][current_pos.1].left = true;
                        self.contents[next_x][next_y].right = true;
                    }
                    Direction::RIGHT => {
                        self.contents[current_pos.0][current_pos.1].right = true;
                        self.contents[next_x][next_y].left = true;
                    }
                };
                self.contents[next_x][next_y].visited = true;
                stack.push((next_x, next_y));
            } else {
                stack.pop();
            }
        }
    }

    fn prepare_board(&mut self, maximum_pos: usize) {
        for x in 0..maximum_pos {
            for y in 0..maximum_pos {
                // Set all cells to have walls and not visited by default
                self.contents[x][y] = Cell {
                    top: false,
                    left: false,
                    right: false,
                    bottom: false,
                    visited: false,
                    border: false,
                    solution: false,
                };
            }
        }
        // Set the outer walls and mark them as visited
        for x in 0..maximum_pos {
            self.contents[x][0].top = true; // Top wall
            self.contents[x][0].right = true; // Right wall
            self.contents[x][0].left = true; // Left wall
            self.contents[x][0].visited = true; // Mark as visited
            self.contents[x][0].border = true; // Mark as border

            self.contents[x][maximum_pos - 1].left = true; // Left wall
            self.contents[x][maximum_pos - 1].right = true; // Right wall
            self.contents[x][maximum_pos - 1].bottom = true; // Bottom wall
            self.contents[x][maximum_pos - 1].visited = true; // Mark as visited
            self.contents[x][maximum_pos - 1].border = true; // Mark as border
        }
        for y in 0..maximum_pos {
            self.contents[0][y].top = true; // Top wall
            self.contents[0][y].left = true; // Right wall
            self.contents[0][y].bottom = true; // Bottom wall
            self.contents[0][y].visited = true; // Mark as visited
            self.contents[0][y].border = true; // Mark as border

            self.contents[maximum_pos - 1][y].top = true; // Top wall
            self.contents[maximum_pos - 1][y].right = true; // Left wall
            self.contents[maximum_pos - 1][y].bottom = true; // Bottom wall
            self.contents[maximum_pos - 1][y].visited = true; // Mark as visited
            self.contents[maximum_pos - 1][y].border = true; // Mark as border
        }

        self.contents[1][0].visited = false; // Top left corner
        self.contents[1][0].border = false;
        self.contents[maximum_pos - 2][maximum_pos - 1].visited = false; // Bottom right corner
        self.contents[maximum_pos - 2][maximum_pos - 1].border = false;
    }

    /// Returns a vector of valid directions that can be moved to from a given position.
    ///
    /// # Arguments
    ///
    /// * `current_pos` - A tuple (usize, usize) indicating the current position in the maze.
    /// * `size_constraints` - A tuple (usize, usize) indicating the maximum bounds of the maze.
    fn get_valid_directions(
        &self,
        current_pos: (usize, usize),
        size_constraints: (usize, usize),
    ) -> Vec<Direction> {
        let mut directions: Vec<Direction> = vec![];
        let (x, y) = current_pos;
        let (max_x, max_y) = size_constraints;

        // Up
        if y > 0 && !self.contents[x][y - 1].visited {
            directions.push(Direction::UP);
        }
        // Down
        if y < max_y - 1 && !self.contents[x][y + 1].visited {
            directions.push(Direction::DOWN);
        }
        // Left
        if x > 0 && !self.contents[x - 1][y].visited {
            directions.push(Direction::LEFT);
        }
        // Right
        if x < max_x - 1 && !self.contents[x + 1][y].visited {
            directions.push(Direction::RIGHT);
        }

        directions
    }

    /// Returns a reference to the maze contents.
    pub fn get_maze(&self) -> &Vec<Vec<Cell>> {
        &self.contents
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug)]
pub struct Cell {
    pub top: bool,
    pub left: bool,
    pub bottom: bool,
    pub right: bool,
    pub visited: bool,
    pub border: bool,
    pub solution: bool,
}

impl Cell {
    /// Constructs a new `Cell` with all walls intact and unvisited.
    /// true = no wall in that spot
    fn new() -> Self {
        Cell {
            top: false,
            left: false,
            bottom: false, // all fields start as "walls" or "false".
            right: false,
            visited: false,
            border: false,
            solution: false,
        }
    }
}
impl Clone for Cell {
    fn clone(&self) -> Self {
        Cell {
            top: self.top,
            left: self.left,
            bottom: self.bottom,
            right: self.right,
            visited: self.visited,
            border: self.border,
            solution: self.solution,
        }
    }
}
