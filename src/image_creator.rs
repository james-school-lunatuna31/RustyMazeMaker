use crate::maze::Maze; // Ensure this path is correct and include Cell
use image::{ImageBuffer, Rgb};
use std::fs;

const CELL_SIZE: u32 = 20;
const WALL_THICKNESS: u32 = 1;
const WALL_COLOR: [u8; 3] = [0, 0, 0]; // Black
const FINAL_COLOR: [u8; 3] = [255, 255, 255]; // White
const PATH_COLOR: [u8; 3] = [0, 0, 255]; // Blue for visited cells
const SOLUTION_COLOR: [u8; 3] = [255, 255, 145]; // Light Yellow
const BORDER_COLOR: [u8; 3] = [169, 169, 169]; // Darker Grey
pub fn create_maze_image(show_solution: bool, maze: &Maze) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let cells = maze.get_maze();
    let size = cells.len() as u32;
    let img_size = CELL_SIZE * size;
    let img = ImageBuffer::from_fn(img_size, img_size, |x, y| {
        let cell_x = x / CELL_SIZE;
        let cell_y = y / CELL_SIZE;
        let in_cell_x = x % CELL_SIZE;
        let in_cell_y = y % CELL_SIZE;
        if (in_cell_x < WALL_THICKNESS && !cells[cell_x as usize][cell_y as usize].left)
            || (in_cell_y < WALL_THICKNESS && !cells[cell_x as usize][cell_y as usize].top)
            || (in_cell_x >= CELL_SIZE - WALL_THICKNESS
                && !cells[cell_x as usize][cell_y as usize].right)
            || (in_cell_y >= CELL_SIZE - WALL_THICKNESS
                && !cells[cell_x as usize][cell_y as usize].bottom)
        {
            Rgb(WALL_COLOR)
        } else {
            if cells[cell_x as usize][cell_y as usize].border {
                Rgb(BORDER_COLOR)
            } else if cells[cell_x as usize][cell_y as usize].solution && show_solution {
                Rgb(SOLUTION_COLOR)
            } else if cells[cell_x as usize][cell_y as usize].visited {
                Rgb(FINAL_COLOR)
            } else {
                Rgb(PATH_COLOR)
            }
        }
    });
    img
}
// Modified to save two versions of the maze image
pub fn save_maze_images(maze: &Maze, collection_name: &str, maze_number: usize, maze_size: usize) {
    let base_path = format!("{}/", collection_name);
    let problems_path = format!("{}problems/", &base_path);
    let solutions_path = format!("{}solutions/", &base_path);

    // Create directories if they don't exist
    fs::create_dir_all(&problems_path).expect("Failed to create problems directory");
    fs::create_dir_all(&solutions_path).expect("Failed to create solutions directory");

    let problem_image = create_maze_image(false, maze);
    let solution_image = create_maze_image(true, maze);

    let problem_file_path = format!("{}maze_{}.png", problems_path, maze_number);
    let solution_file_path = format!(
        "{}maze_{}_{}_sol.png",
        solutions_path, maze_number, maze_size
    );

    match problem_image.save(&problem_file_path) {
        Ok(_) => println!("Maze problem image saved to {}", problem_file_path),
        Err(e) => println!("Failed to save maze problem image: {}", e),
    }

    match solution_image.save(&solution_file_path) {
        Ok(_) => println!("Maze solution image saved to {}", solution_file_path),
        Err(e) => println!("Failed to save maze solution image: {}", e),
    }
}
