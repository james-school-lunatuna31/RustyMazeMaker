mod image_creator;
mod maze;
fn main() {
    use std::fs;
    use std::io;

    println!("Do you want time mode (T) or fast gen mode (F)?");
    let mut mode = String::new();
    io::stdin()
        .read_line(&mut mode)
        .expect("Failed to read line");
    let mode = mode.trim().to_uppercase();

    if mode == "T" {
        println!("Enter the size for the maze:");
        let mut size = String::new();
        io::stdin()
            .read_line(&mut size)
            .expect("Failed to read line");
        let size: u16 = size.trim().parse().expect("Please type a number!");

        println!("Enter the duration (in minutes) for generating the maze:");
        let mut duration = String::new();
        io::stdin()
            .read_line(&mut duration)
            .expect("Failed to read line");
        let duration: u64 = duration.trim().parse().expect("Please type a number!");
        let duration = Duration::from_secs(duration * 60);

        println!("What do you want to name the maze?");
        let mut maze_name = String::new();
        io::stdin()
            .read_line(&mut maze_name)
            .expect("Failed to read line");
        let maze_name = maze_name.trim();

        let maze = generate_largest_maze_for_duration(size, duration);
        let dir_path = format!("./{}", maze_name);
        fs::create_dir_all(&dir_path).expect("Failed to create directory");
        image_creator::save_maze_images(&maze, &dir_path, 1, maze.get_solution_length());
        println!("Maze generated and saved to {}/maze_1.png", dir_path);
    } else if mode == "F" {
        println!("How many mazes do you want to generate?");
        let mut mazes_count = String::new();
        io::stdin()
            .read_line(&mut mazes_count)
            .expect("Failed to read line");
        let mazes_count: u32 = mazes_count.trim().parse().expect("Please type a number!");

        println!("What size should the mazes be?");
        let mut maze_size = String::new();
        io::stdin()
            .read_line(&mut maze_size)
            .expect("Failed to read line");
        let maze_size: u16 = maze_size.trim().parse().expect("Please type a number!");

        println!("What do you want to name the collection?");
        let mut collection_name = String::new();
        io::stdin()
            .read_line(&mut collection_name)
            .expect("Failed to read line");
        let collection_name = collection_name.trim();

        let dir_path = format!("./{}", collection_name);
        fs::create_dir_all(&dir_path).expect("Failed to create directory");

        let mut mazes = Vec::new();
        for _ in 0..mazes_count {
            let maze = maze::Maze::new(maze_size);
            mazes.push(maze);
        }

        // Sort mazes by their solution length
        mazes.sort_by_key(|m| m.get_solution_length());

        // Save sorted mazes
        for (i, maze) in mazes.iter().enumerate() {
            let file_path = format!("{}/maze_{}.png", dir_path, i + 1);
            image_creator::save_maze_images(maze, &dir_path, i + 1, maze.get_solution_length());
            println!("Maze {} generated and saved to {}", i + 1, file_path);
        }
    } else {
        println!("Invalid mode selected. Please restart and choose either T or F.");
    }
}

use std::io::Write;
use std::time::{Duration, Instant};

/// Generates the largest maze within a specified duration and size.
///
/// This function continuously generates mazes of a given size until the specified duration elapses.
/// It keeps track of the maze with the longest solution path generated during this time.
/// The progress of generation is displayed through a progress bar in the console.
///
/// # Arguments
///
/// * `size` - The size of the maze to generate (width and height are equal).
/// * `duration` - The maximum duration to spend on generating mazes.
///
/// # Returns
///
/// Returns the maze with the longest solution path that was generated within the given duration.

fn generate_largest_maze_for_duration(size: u16, duration: Duration) -> maze::Maze {
    let start_time = Instant::now();
    let mut largest_maze = maze::Maze::new(size);
    let mut largest_solution_size = largest_maze.get_solution_length();

    while Instant::now().duration_since(start_time) < duration {
        let maze = maze::Maze::new(size);
        let solution_size = maze.get_solution_length();
        let elapsed_time = Instant::now().duration_since(start_time);
        let total_duration_secs = duration.as_secs();
        let elapsed_secs = elapsed_time.as_secs();
        let progress_percentage = (elapsed_secs as f64 / total_duration_secs as f64) * 100.0;
        let progress_bar_length = 20; // Total length of the progress bar
        let filled_length =
            (progress_percentage / 100.0 * progress_bar_length as f64).round() as usize;
        let bar = "=".repeat(filled_length) + &" ".repeat(progress_bar_length - filled_length);
        print!(
            "\r[{}] {}/{} : {:.0}% Generating...",
            bar, elapsed_secs, total_duration_secs, progress_percentage
        );
        std::io::stdout().flush().unwrap();
        if solution_size > largest_solution_size {
            largest_maze = maze;
            largest_solution_size = solution_size;
        }
    }

    println!("\nTime's up!");
    largest_maze
}
